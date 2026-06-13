//! Resolve every new network-capable surface action through one shared
//! transport-trust model before any request leaves the current boundary.
//!
//! The sibling [`crate::networked_surface_transport_matrix`] freezes which
//! trust material *class* a surface is allowed to use, and
//! [`crate::networked_surface_transport_decision`] records one decision per
//! action. This module makes the **trust inputs and host proof themselves** a
//! first-class governed object: for every covered surface it freezes the
//! trust-store descriptor, the organization CA bundle / pin-set review state,
//! the SSH/TLS host-proof state and history depth, the client-certificate
//! binding posture, and the trust-root freshness and rotation cue — so a
//! missing or stale trust input surfaces as a typed `deny_trust` reason or a
//! typed host-proof state rather than generic offline copy, and no helper,
//! client, or extension can ship a direct CA override or silently downgrade
//! trust outside this vocabulary.
//!
//! Each [`TrustEvaluationRecord`] answers, per surface and in one inspectable
//! record:
//!
//! - **where the trust anchors come from** ([`CaBundleDescriptor`]) — the
//!   trust-store source ([`TrustStoreSourceClass`]), the CA bundle / pin-set
//!   review state ([`CaBundleReviewClass`]), and the pin count, by opaque
//!   bundle handle only (never raw CA bytes),
//! - **what the host proved** ([`HostProofDescriptor`]) — the typed host-proof
//!   state ([`HostProofStateClass`]) and history depth, by opaque proof handle
//!   (never a raw host key),
//! - **how the client authenticated to the endpoint** ([`ClientCertDescriptor`])
//!   — the client-certificate posture ([`ClientCertPostureClass`]) by opaque
//!   binding handle (never raw certificate or private-key bytes),
//! - **whether the trust root is fresh** ([`TrustRootDescriptor`]) — the
//!   freshness ([`TrustRootFreshnessClass`]) and the rotation cue
//!   ([`RotationCueClass`]) that tells a surface a trust-root rotation is due,
//!   in progress, or required,
//! - **what trust decided** ([`TrustOutcomeClass`]) — trusted, trusted with a
//!   rotation cue, host-proof pending, denied, or not-applicable loopback —
//!   with a typed [`TrustDenialClass`] (`deny_trust`) when refused, and
//! - **that trust did not bypass governance** — no direct CA override
//!   ([`TrustEvaluationRecord::no_direct_ca_override`]) and no silent trust
//!   downgrade ([`TrustEvaluationRecord::no_silent_trust_downgrade`]).
//!
//! These records aggregate into a stable proof packet ([`TransportTrustPage`])
//! consumed by product UI, CLI/headless output, diagnostics, support exports,
//! and admin/audit surfaces. A missing or unverifiable trust input produces a
//! labeled **denied** record with a typed `deny_trust` reason rather than a
//! hidden fallback to an untrusted root.
//!
//! The stable claim holds when **all** of the following conditions are verified
//! simultaneously for every covered surface record:
//!
//! 1. All required surfaces have a trust-evaluation record.
//! 2. No raw trust material (raw CA bytes, raw certificate bytes) is present on
//!    any record (`raw_trust_material_excluded`).
//! 3. No raw private-key / SSH-private material is present
//!    (`private_key_material_excluded`).
//! 4. No record ships a direct CA override (`no_direct_ca_override`).
//! 5. No record silently downgrades trust (`no_silent_trust_downgrade`).
//! 6. Every record preserves local-core continuity.
//! 7. Every denied record carries a typed `deny_trust` reason.
//! 8. Every record exposes a typed host-proof state and a complete set of trust
//!    inputs.
//! 9. Every record whose trust root needs rotation carries an active rotation
//!    cue.
//! 10. Every record whose egress class requires a policy epoch carries a
//!     last-known-good policy epoch ref.
//!
//! Four conditions force [`TrustQualificationClass::Withdrawn`] immediately and
//! cannot be overridden: raw trust material exposed, raw private-key material
//! exposed, a shipped direct CA override, or a silent trust downgrade. A missing
//! required surface narrows to [`TrustQualificationClass::Preview`]; the
//! remaining gaps narrow to `Beta`, which lets release and support tooling
//! automatically narrow stale or under-qualified rows before publication.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque handles/refs only. Raw CA bundles, raw certificate bytes, raw private
//! keys, raw SSH host or private keys, raw credentials, and raw bearer/session
//! tokens stay outside the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/network/networked-surface-transport-trust.md`
//! - Artifact: `artifacts/network/networked-surface-transport-trust.md`
//! - Schema: `schemas/network/networked_surface_transport_trust.schema.json`
//! - Contract ref: [`TRANSPORT_TRUST_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::networked_surface_transport_matrix::{
    EgressClass, MirrorOfflineBehaviorClass, OriginScopeClass, SurfaceClass, REQUIRED_SURFACES,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const TRANSPORT_TRUST_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const TRANSPORT_TRUST_SHARED_CONTRACT_REF: &str = "remote:networked_surface_transport_trust:v1";

/// Record-kind tag for [`TransportTrustPage`] payloads.
pub const TRANSPORT_TRUST_PAGE_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_page_record";

/// Record-kind tag for [`CaBundleDescriptor`] payloads.
pub const TRANSPORT_TRUST_CA_BUNDLE_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_ca_bundle_record";

/// Record-kind tag for [`HostProofDescriptor`] payloads.
pub const TRANSPORT_TRUST_HOST_PROOF_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_host_proof_record";

/// Record-kind tag for [`ClientCertDescriptor`] payloads.
pub const TRANSPORT_TRUST_CLIENT_CERT_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_client_cert_record";

/// Record-kind tag for [`TrustRootDescriptor`] payloads.
pub const TRANSPORT_TRUST_ROOT_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_root_record";

/// Record-kind tag for [`TrustEvaluationRecord`] payloads.
pub const TRANSPORT_TRUST_EVALUATION_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_evaluation_record";

/// Record-kind tag for [`TrustRow`] payloads.
pub const TRANSPORT_TRUST_ROW_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_row_record";

/// Record-kind tag for [`TrustDefect`] payloads.
pub const TRANSPORT_TRUST_DEFECT_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_defect_record";

/// Record-kind tag for [`TrustSummary`] payloads.
pub const TRANSPORT_TRUST_SUMMARY_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_summary_record";

/// Record-kind tag for [`TrustSupportExport`] payloads.
pub const TRANSPORT_TRUST_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_networked_surface_transport_trust_support_export_record";

/// Repo-relative path of the stable doc for this transport-trust lane.
pub const TRANSPORT_TRUST_DOC_REF: &str = "docs/network/networked-surface-transport-trust.md";

/// Repo-relative path of the artifact summary for this transport-trust lane.
pub const TRANSPORT_TRUST_ARTIFACT_REF: &str =
    "artifacts/network/networked-surface-transport-trust.md";

/// Repo-relative ref to the canonical evidence index this lane binds into for
/// the closeout certification lane.
pub const TRANSPORT_TRUST_EVIDENCE_INDEX_REF: &str = "artifacts/release/m5/xt12-evidence-index.md";

// ---------------------------------------------------------------------------
// Trust-store source vocabulary
// ---------------------------------------------------------------------------

/// Where a surface's trust anchors are sourced from.
///
/// This names the *trust-store descriptor* behind a networked surface so the
/// origin of trust is inspectable rather than an implicit transport internal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStoreSourceClass {
    /// The platform/OS system trust store.
    SystemTrustStore,
    /// A surface-specific pinned CA / pin set.
    PinnedCaSet,
    /// An organization-reviewed managed CA bundle.
    ManagedOrgBundle,
    /// The signed-mirror trust root.
    MirrorRoot,
    /// An SSH `known_hosts`-style host-proof store.
    SshKnownHosts,
    /// A loopback surface that carries no TLS trust material.
    NoTlsLoopback,
}

impl TrustStoreSourceClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemTrustStore => "system_trust_store",
            Self::PinnedCaSet => "pinned_ca_set",
            Self::ManagedOrgBundle => "managed_org_bundle",
            Self::MirrorRoot => "mirror_root",
            Self::SshKnownHosts => "ssh_known_hosts",
            Self::NoTlsLoopback => "no_tls_loopback",
        }
    }

    /// Human-readable label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SystemTrustStore => "System trust store",
            Self::PinnedCaSet => "Pinned CA set",
            Self::ManagedOrgBundle => "Managed organization CA bundle",
            Self::MirrorRoot => "Signed-mirror trust root",
            Self::SshKnownHosts => "SSH known-hosts store",
            Self::NoTlsLoopback => "Loopback (no TLS)",
        }
    }
}

// ---------------------------------------------------------------------------
// CA bundle review vocabulary
// ---------------------------------------------------------------------------

/// Review state of the CA bundle / pin set backing a surface.
///
/// An organization CA bundle that has been reviewed
/// ([`CaBundleReviewClass::OrgReviewed`]) is distinguishable from a bare system
/// default so admins can reason about which surfaces depend on a vetted bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaBundleReviewClass {
    /// The platform system default trust set; no org review applies.
    SystemDefault,
    /// An organization-reviewed managed bundle.
    OrgReviewed,
    /// A surface-pinned CA / pin set.
    PinnedSet,
    /// The signed-mirror trust root.
    MirrorRoot,
    /// No CA bundle applies (loopback / no TLS).
    NotApplicable,
}

impl CaBundleReviewClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemDefault => "system_default",
            Self::OrgReviewed => "org_reviewed",
            Self::PinnedSet => "pinned_set",
            Self::MirrorRoot => "mirror_root",
            Self::NotApplicable => "not_applicable",
        }
    }
}

// ---------------------------------------------------------------------------
// Host-proof state vocabulary
// ---------------------------------------------------------------------------

/// Typed state of the SSH/TLS host proof for a surface.
///
/// A missing or changed host proof surfaces as a typed state (and, downstream,
/// a typed `deny_trust` reason) rather than generic offline copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostProofStateClass {
    /// The host proof matches the pinned proof.
    PinnedMatch,
    /// A trust-on-first-use record exists and the host proof matches it.
    KnownTofu,
    /// No prior host proof exists; a first-use decision is pending and the
    /// surface holds rather than trusting silently.
    FirstUsePending,
    /// The host proof changed from the recorded proof (mismatch).
    ChangedMismatch,
    /// The host proof was revoked.
    Revoked,
    /// No host proof applies (loopback / no TLS).
    NotApplicable,
}

impl HostProofStateClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedMatch => "pinned_match",
            Self::KnownTofu => "known_tofu",
            Self::FirstUsePending => "first_use_pending",
            Self::ChangedMismatch => "changed_mismatch",
            Self::Revoked => "revoked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns `true` when this state is a verified, trust-bearing match.
    pub const fn is_trusting(self) -> bool {
        matches!(self, Self::PinnedMatch | Self::KnownTofu)
    }

    /// Returns `true` when this state must deny trust.
    pub const fn is_deny_state(self) -> bool {
        matches!(self, Self::ChangedMismatch | Self::Revoked)
    }
}

// ---------------------------------------------------------------------------
// Client-certificate posture vocabulary
// ---------------------------------------------------------------------------

/// Client-certificate binding posture for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientCertPostureClass {
    /// No client certificate is required.
    NotRequired,
    /// A client certificate is optional and one was presented.
    OptionalPresented,
    /// A client certificate is required and a binding was presented.
    RequiredPresented,
    /// A managed client certificate was provisioned for the surface.
    ManagedProvisioned,
    /// A client certificate is required but no binding is present.
    RequiredAbsent,
}

impl ClientCertPostureClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::OptionalPresented => "optional_presented",
            Self::RequiredPresented => "required_presented",
            Self::ManagedProvisioned => "managed_provisioned",
            Self::RequiredAbsent => "required_absent",
        }
    }

    /// Returns `true` when a required client certificate is missing.
    pub const fn is_required_absent(self) -> bool {
        matches!(self, Self::RequiredAbsent)
    }

    /// Returns `true` when a binding handle is expected for this posture.
    pub const fn expects_binding(self) -> bool {
        matches!(
            self,
            Self::OptionalPresented | Self::RequiredPresented | Self::ManagedProvisioned
        )
    }
}

// ---------------------------------------------------------------------------
// Trust-root freshness vocabulary
// ---------------------------------------------------------------------------

/// Freshness of the trust root behind a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRootFreshnessClass {
    /// The trust root is fresh and well within its validity window.
    Fresh,
    /// The trust root is valid but rotation is due soon.
    RotationDue,
    /// A trust-root rotation is in progress (overlapping roots are trusted).
    RotationInProgress,
    /// The trust root is expired beyond its validity window.
    Expired,
    /// A static pinned root that is not expected to rotate.
    PinnedStatic,
}

impl TrustRootFreshnessClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::RotationDue => "rotation_due",
            Self::RotationInProgress => "rotation_in_progress",
            Self::Expired => "expired",
            Self::PinnedStatic => "pinned_static",
        }
    }

    /// Returns `true` when the trust root is past its validity window.
    pub const fn is_expired(self) -> bool {
        matches!(self, Self::Expired)
    }

    /// Returns `true` when this freshness state requires an active rotation cue.
    pub const fn needs_rotation_cue(self) -> bool {
        matches!(
            self,
            Self::RotationDue | Self::RotationInProgress | Self::Expired
        )
    }
}

// ---------------------------------------------------------------------------
// Rotation cue vocabulary
// ---------------------------------------------------------------------------

/// The trust-root rotation cue surfaced to product and admin surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RotationCueClass {
    /// No rotation cue; the trust root is fresh.
    None,
    /// Rotation is recommended soon.
    RotateSoon,
    /// Rotation is currently in progress.
    Rotating,
    /// Rotation is required now (the root is expired).
    RotateNow,
    /// The root is pinned static and no rotation is expected.
    PinnedNoRotation,
}

impl RotationCueClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RotateSoon => "rotate_soon",
            Self::Rotating => "rotating",
            Self::RotateNow => "rotate_now",
            Self::PinnedNoRotation => "pinned_no_rotation",
        }
    }

    /// Returns `true` when this cue is an active rotation prompt.
    pub const fn is_active_cue(self) -> bool {
        matches!(self, Self::RotateSoon | Self::Rotating | Self::RotateNow)
    }
}

// ---------------------------------------------------------------------------
// Trust outcome vocabulary
// ---------------------------------------------------------------------------

/// What the shared transport-trust layer decided for one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustOutcomeClass {
    /// All trust inputs verified; the surface is trusted.
    Trusted,
    /// The surface is trusted, but a trust-root rotation cue is active.
    TrustedRotationDue,
    /// The host proof is pending a first-use decision; the surface holds.
    HostProofPending,
    /// Trust was refused; a typed [`TrustDenialClass`] is recorded.
    DenyTrust,
    /// No trust material applies (loopback / no TLS); local-core continues.
    NotApplicableLoopback,
}

impl TrustOutcomeClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::TrustedRotationDue => "trusted_rotation_due",
            Self::HostProofPending => "host_proof_pending",
            Self::DenyTrust => "deny_trust",
            Self::NotApplicableLoopback => "not_applicable_loopback",
        }
    }

    /// Returns `true` when this outcome must carry a typed denial reason.
    pub const fn requires_denial_reason(self) -> bool {
        matches!(self, Self::DenyTrust)
    }

    /// Returns `true` when this outcome grants trust to the surface.
    pub const fn is_trusted(self) -> bool {
        matches!(self, Self::Trusted | Self::TrustedRotationDue)
    }
}

// ---------------------------------------------------------------------------
// deny_trust vocabulary
// ---------------------------------------------------------------------------

/// Closed `deny_trust` vocabulary: the typed reasons trust may be refused.
///
/// A missing or unverifiable trust input is surfaced as one of these typed
/// reasons rather than generic offline copy or a silent fallback to an
/// untrusted root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustDenialClass {
    /// The trust store could not be loaded for this surface.
    TrustStoreUnavailable,
    /// The CA bundle is missing.
    CaBundleMissing,
    /// The CA bundle is stale beyond its refresh window.
    CaBundleStale,
    /// A managed organization bundle has not been verified.
    ManagedBundleUnverified,
    /// No host proof is recorded and one is required.
    HostProofMissing,
    /// The host proof changed from the recorded proof.
    HostProofChanged,
    /// The host proof was revoked.
    HostProofRevoked,
    /// A required client certificate is absent.
    ClientCertRequiredAbsent,
    /// The trust root is expired beyond its validity window.
    TrustRootExpired,
    /// The presented pin set does not match the recorded pin set.
    PinSetMismatch,
    /// The signed-mirror trust root does not match the recorded root.
    MirrorRootMismatch,
}

impl TrustDenialClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustStoreUnavailable => "trust_store_unavailable",
            Self::CaBundleMissing => "ca_bundle_missing",
            Self::CaBundleStale => "ca_bundle_stale",
            Self::ManagedBundleUnverified => "managed_bundle_unverified",
            Self::HostProofMissing => "host_proof_missing",
            Self::HostProofChanged => "host_proof_changed",
            Self::HostProofRevoked => "host_proof_revoked",
            Self::ClientCertRequiredAbsent => "client_cert_required_absent",
            Self::TrustRootExpired => "trust_root_expired",
            Self::PinSetMismatch => "pin_set_mismatch",
            Self::MirrorRootMismatch => "mirror_root_mismatch",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual
/// trust-evaluation rows.
///
/// The tier is derived, not asserted: it is computed by comparing the audit
/// defect list against the stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete surface coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustQualificationClass {
    /// All stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required surface has no record; the coverage gap prevents a beta claim.
    Preview,
    /// A hard guardrail was violated; the packet is withdrawn immediately and
    /// cannot be overridden.
    Withdrawn,
}

impl TrustQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Returns `true` when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns `true` when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`TrustQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustNarrowReasonClass {
    /// No narrowing — the record qualifies stable.
    NotNarrowed,
    /// A record carries `raw_trust_material_excluded: false`; withdraws the
    /// packet immediately.
    RawTrustMaterialExposed,
    /// A record carries `private_key_material_excluded: false`; withdraws the
    /// packet immediately.
    PrivateKeyMaterialExposed,
    /// A record ships a direct CA override; withdraws the packet immediately.
    DirectCaOverrideShipped,
    /// A record silently downgrades trust; withdraws the packet immediately.
    SilentTrustDowngrade,
    /// A required surface has no record; narrows to preview.
    RequiredSurfaceMissing,
    /// A denied record carries no typed `deny_trust` reason.
    DenyReasonMissing,
    /// A record's trust inputs or host-proof state are missing or inconsistent.
    TrustInputClassificationIncomplete,
    /// A record whose trust root needs rotation carries no active rotation cue.
    RotationCueMissing,
    /// A record does not preserve local-core continuity.
    LocalCoreContinuityNotPreserved,
    /// A record whose egress class requires a policy epoch is missing the
    /// last-known-good policy epoch ref.
    PolicyEpochRefMissing,
}

impl TrustNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawTrustMaterialExposed => "raw_trust_material_exposed",
            Self::PrivateKeyMaterialExposed => "private_key_material_exposed",
            Self::DirectCaOverrideShipped => "direct_ca_override_shipped",
            Self::SilentTrustDowngrade => "silent_trust_downgrade",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DenyReasonMissing => "deny_reason_missing",
            Self::TrustInputClassificationIncomplete => "trust_input_classification_incomplete",
            Self::RotationCueMissing => "rotation_cue_missing",
            Self::LocalCoreContinuityNotPreserved => "local_core_continuity_not_preserved",
            Self::PolicyEpochRefMissing => "policy_epoch_ref_missing",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws the
    /// packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawTrustMaterialExposed
                | Self::PrivateKeyMaterialExposed
                | Self::DirectCaOverrideShipped
                | Self::SilentTrustDowngrade
        )
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredSurfaceMissing)
    }
}

// ---------------------------------------------------------------------------
// Trust-input descriptors
// ---------------------------------------------------------------------------

/// Trust-store and CA bundle / pin-set descriptor for a surface.
///
/// The bundle is named by opaque handle only; raw CA bytes never appear.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaBundleDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Trust-store source for this surface.
    pub trust_store_source: TrustStoreSourceClass,
    /// Stable token for [`Self::trust_store_source`].
    pub trust_store_source_token: String,
    /// CA bundle / pin-set review state.
    pub review_state: CaBundleReviewClass,
    /// Stable token for [`Self::review_state`].
    pub review_state_token: String,
    /// Number of pins / CA anchors in the bundle (0 for loopback / no TLS).
    pub pin_count: u32,
    /// Opaque handle identifying the CA bundle. Never raw CA bytes.
    pub bundle_handle: String,
    /// `true` when this bundle is a forbidden direct CA override. Must be
    /// `false` on every clean record.
    pub is_direct_ca_override: bool,
    /// Export-safe note describing the bundle.
    pub note: String,
}

impl CaBundleDescriptor {
    /// Construct a governed (non-override) CA bundle descriptor.
    pub fn new(
        trust_store_source: TrustStoreSourceClass,
        review_state: CaBundleReviewClass,
        pin_count: u32,
        bundle_handle: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TRANSPORT_TRUST_CA_BUNDLE_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
            trust_store_source,
            trust_store_source_token: trust_store_source.as_str().to_owned(),
            review_state,
            review_state_token: review_state.as_str().to_owned(),
            pin_count,
            bundle_handle: bundle_handle.into(),
            is_direct_ca_override: false,
            note: note.into(),
        }
    }
}

/// SSH/TLS host-proof descriptor for a surface.
///
/// The host proof is named by opaque handle only; raw host keys never appear.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostProofDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Typed host-proof state.
    pub state: HostProofStateClass,
    /// Stable token for [`Self::state`].
    pub state_token: String,
    /// Opaque handle identifying the host proof. Never a raw host key.
    pub proof_handle: String,
    /// Number of recorded host-proof history entries.
    pub history_depth: u32,
    /// Export-safe note describing the host proof.
    pub note: String,
}

impl HostProofDescriptor {
    /// Construct a host-proof descriptor.
    pub fn new(
        state: HostProofStateClass,
        proof_handle: impl Into<String>,
        history_depth: u32,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TRANSPORT_TRUST_HOST_PROOF_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
            state,
            state_token: state.as_str().to_owned(),
            proof_handle: proof_handle.into(),
            history_depth,
            note: note.into(),
        }
    }
}

/// Client-certificate binding descriptor for a surface.
///
/// The binding is named by opaque handle only; raw certificate or private-key
/// bytes never appear.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientCertDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Client-certificate posture.
    pub posture: ClientCertPostureClass,
    /// Stable token for [`Self::posture`].
    pub posture_token: String,
    /// Opaque handle identifying the client-certificate binding; empty when no
    /// binding applies. Never raw certificate or private-key bytes.
    pub binding_handle: String,
    /// Export-safe note describing the binding.
    pub note: String,
}

impl ClientCertDescriptor {
    /// Construct a client-certificate descriptor.
    pub fn new(
        posture: ClientCertPostureClass,
        binding_handle: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TRANSPORT_TRUST_CLIENT_CERT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
            posture,
            posture_token: posture.as_str().to_owned(),
            binding_handle: binding_handle.into(),
            note: note.into(),
        }
    }
}

/// Trust-root freshness and rotation-cue descriptor for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRootDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Trust-root freshness state.
    pub freshness: TrustRootFreshnessClass,
    /// Stable token for [`Self::freshness`].
    pub freshness_token: String,
    /// Trust-root rotation cue.
    pub rotation_cue: RotationCueClass,
    /// Stable token for [`Self::rotation_cue`].
    pub rotation_cue_token: String,
    /// Opaque ref to the trust-root rotation record; `None` when no rotation is
    /// tracked. Never raw root bytes.
    pub rotation_ref: Option<String>,
    /// Export-safe note describing the trust-root state.
    pub note: String,
}

impl TrustRootDescriptor {
    /// Construct a trust-root descriptor.
    pub fn new(
        freshness: TrustRootFreshnessClass,
        rotation_cue: RotationCueClass,
        rotation_ref: Option<impl Into<String>>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TRANSPORT_TRUST_ROOT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
            freshness,
            freshness_token: freshness.as_str().to_owned(),
            rotation_cue,
            rotation_cue_token: rotation_cue.as_str().to_owned(),
            rotation_ref: rotation_ref.map(Into::into),
            note: note.into(),
        }
    }

    /// Returns `true` when this descriptor's freshness/cue pair is consistent —
    /// a root that needs rotation carries an active cue.
    pub fn rotation_cue_consistent(&self) -> bool {
        if self.freshness.needs_rotation_cue() {
            self.rotation_cue.is_active_cue()
        } else {
            true
        }
    }
}

// ---------------------------------------------------------------------------
// Trust-evaluation record (per surface)
// ---------------------------------------------------------------------------

/// One inspectable trust-evaluation record emitted before a network-capable
/// surface action leaves the current boundary.
///
/// The record freezes the trust-store descriptor, the CA bundle / pin-set
/// review state, the host-proof state and history depth, the client-certificate
/// posture, the trust-root freshness and rotation cue, the typed outcome, the
/// typed `deny_trust` reason when refused, and the guardrail flags that prove
/// trust resolution did not ship a direct CA override or silently downgrade
/// trust.
///
/// No raw CA bundles, raw certificate bytes, raw private keys, raw SSH host or
/// private keys, raw credentials, or raw bearer/session tokens may appear on
/// this record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustEvaluationRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque id for this record.
    pub record_id: String,
    /// UTC instant trust was evaluated.
    pub evaluated_at: String,
    /// Surface this record belongs to.
    pub surface: SurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Origin ownership scope for the endpoint being reached.
    pub origin_scope: OriginScopeClass,
    /// Stable token for [`Self::origin_scope`].
    pub origin_scope_token: String,
    /// Egress class enforced for this surface.
    pub egress_class: EgressClass,
    /// Stable token for [`Self::egress_class`].
    pub egress_class_token: String,
    /// Trust-store / CA bundle descriptor.
    pub ca_bundle: CaBundleDescriptor,
    /// Host-proof descriptor.
    pub host_proof: HostProofDescriptor,
    /// Client-certificate descriptor.
    pub client_cert: ClientCertDescriptor,
    /// Trust-root freshness / rotation descriptor.
    pub trust_root: TrustRootDescriptor,
    /// Typed outcome of trust evaluation.
    pub outcome: TrustOutcomeClass,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Typed `deny_trust` reason when refused; `None` otherwise.
    pub denial_reason: Option<TrustDenialClass>,
    /// Stable token for [`Self::denial_reason`]; empty when `None`.
    pub denial_reason_token: String,
    /// Mirror/offline behavior when the primary route is unavailable.
    pub mirror_offline_behavior: MirrorOfflineBehaviorClass,
    /// Stable token for [`Self::mirror_offline_behavior`].
    pub mirror_offline_behavior_token: String,
    /// Opaque ref to the last-known-good policy epoch governing this surface.
    /// Present for egress classes that require it; `None` otherwise.
    pub policy_epoch_ref: Option<String>,
    /// `true` when trust resolution did not override the certificate authority.
    pub no_direct_ca_override: bool,
    /// `true` when trust resolution did not silently downgrade to an untrusted
    /// root or accept a host proof without a recorded decision.
    pub no_silent_trust_downgrade: bool,
    /// `true` when local-core editing continues regardless of this surface's
    /// availability.
    pub local_core_continuity_preserved: bool,
    /// Plain-language summary safe for UI, support export, and diagnostics.
    pub summary: String,
    /// `true` when no raw trust material (CA bytes, certificate bytes) is
    /// present on this record.
    pub raw_trust_material_excluded: bool,
    /// `true` when no raw private-key / SSH-private material is present.
    pub private_key_material_excluded: bool,
}

impl TrustEvaluationRecord {
    /// Construct a trust-evaluation record, filling in token fields from the
    /// typed enum values and descriptors.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        record_id: impl Into<String>,
        evaluated_at: impl Into<String>,
        surface: SurfaceClass,
        origin_scope: OriginScopeClass,
        egress_class: EgressClass,
        ca_bundle: CaBundleDescriptor,
        host_proof: HostProofDescriptor,
        client_cert: ClientCertDescriptor,
        trust_root: TrustRootDescriptor,
        outcome: TrustOutcomeClass,
        denial_reason: Option<TrustDenialClass>,
        mirror_offline_behavior: MirrorOfflineBehaviorClass,
        policy_epoch_ref: Option<impl Into<String>>,
        local_core_continuity_preserved: bool,
        summary: impl Into<String>,
    ) -> Self {
        let denial_reason_token = denial_reason
            .map(|d| d.as_str().to_owned())
            .unwrap_or_default();
        let no_direct_ca_override = !ca_bundle.is_direct_ca_override;
        Self {
            record_kind: TRANSPORT_TRUST_EVALUATION_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
            record_id: record_id.into(),
            evaluated_at: evaluated_at.into(),
            surface,
            surface_token: surface.as_str().to_owned(),
            origin_scope,
            origin_scope_token: origin_scope.as_str().to_owned(),
            egress_class,
            egress_class_token: egress_class.as_str().to_owned(),
            ca_bundle,
            host_proof,
            client_cert,
            trust_root,
            outcome,
            outcome_token: outcome.as_str().to_owned(),
            denial_reason,
            denial_reason_token,
            mirror_offline_behavior,
            mirror_offline_behavior_token: mirror_offline_behavior.as_str().to_owned(),
            policy_epoch_ref: policy_epoch_ref.map(Into::into),
            no_direct_ca_override,
            no_silent_trust_downgrade: true,
            local_core_continuity_preserved,
            summary: summary.into(),
            raw_trust_material_excluded: true,
            private_key_material_excluded: true,
        }
    }

    /// Returns `true` when every classification token and the trust-input
    /// descriptors are present and internally consistent.
    pub fn is_fully_classified(&self) -> bool {
        if self.surface_token.is_empty()
            || self.origin_scope_token.is_empty()
            || self.egress_class_token.is_empty()
            || self.outcome_token.is_empty()
            || self.ca_bundle.trust_store_source_token.is_empty()
            || self.ca_bundle.review_state_token.is_empty()
            || self.host_proof.state_token.is_empty()
            || self.client_cert.posture_token.is_empty()
            || self.trust_root.freshness_token.is_empty()
            || self.trust_root.rotation_cue_token.is_empty()
        {
            return false;
        }
        // A trusted outcome must not sit on top of a host proof that denies
        // trust, and a host-proof-pending outcome must carry the pending state.
        match self.outcome {
            TrustOutcomeClass::Trusted | TrustOutcomeClass::TrustedRotationDue => {
                if self.host_proof.state.is_deny_state() {
                    return false;
                }
            }
            TrustOutcomeClass::HostProofPending => {
                if self.host_proof.state != HostProofStateClass::FirstUsePending {
                    return false;
                }
            }
            TrustOutcomeClass::NotApplicableLoopback => {
                if self.host_proof.state != HostProofStateClass::NotApplicable {
                    return false;
                }
            }
            TrustOutcomeClass::DenyTrust => {}
        }
        true
    }

    /// Returns `true` when this record exposes a typed host-proof state.
    pub fn exposes_host_proof_state(&self) -> bool {
        !self.host_proof.state_token.is_empty()
    }

    /// Returns `true` when the trust-root freshness/cue pair is consistent.
    pub fn rotation_cue_consistent(&self) -> bool {
        self.trust_root.rotation_cue_consistent()
    }

    /// Returns `true` when no raw trust or private-key material is present.
    pub fn raw_material_excluded(&self) -> bool {
        self.raw_trust_material_excluded && self.private_key_material_excluded
    }

    /// Returns `true` when trust resolution honored every no-bypass guardrail.
    pub fn no_bypass(&self) -> bool {
        self.no_direct_ca_override
            && self.no_silent_trust_downgrade
            && !self.ca_bundle.is_direct_ca_override
    }
}

// ---------------------------------------------------------------------------
// Trust snapshot (aggregate of all records)
// ---------------------------------------------------------------------------

/// Aggregate of all trust-evaluation records for the covered surfaces.
///
/// The snapshot carries one [`TrustEvaluationRecord`] per network-capable
/// surface. A snapshot missing any required surface causes the page to narrow
/// to `Preview`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustSnapshot {
    /// All trust-evaluation records in the snapshot.
    pub records: Vec<TrustEvaluationRecord>,
}

impl TrustSnapshot {
    /// Returns the record for the given surface, if present.
    pub fn record_for_surface(&self, surface: SurfaceClass) -> Option<&TrustEvaluationRecord> {
        self.records.iter().find(|r| r.surface == surface)
    }

    /// Returns the set of surface tokens covered by this snapshot.
    pub fn covered_surface_tokens(&self) -> BTreeSet<&str> {
        self.records
            .iter()
            .map(|r| r.surface_token.as_str())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Trust row (per-record stability row)
// ---------------------------------------------------------------------------

/// Stability qualification for one trust-evaluation record in the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Record id for this row.
    pub record_id: String,
    /// Surface token for this row.
    pub surface_token: String,
    /// Origin scope token from the record.
    pub origin_scope_token: String,
    /// Egress class token from the record.
    pub egress_class_token: String,
    /// Trust-store source token from the record.
    pub trust_store_source_token: String,
    /// CA bundle review state token from the record.
    pub ca_review_state_token: String,
    /// Pin count from the record.
    pub ca_pin_count: u32,
    /// Host-proof state token from the record.
    pub host_proof_state_token: String,
    /// Client-certificate posture token from the record.
    pub client_cert_posture_token: String,
    /// Trust-root freshness token from the record.
    pub trust_root_freshness_token: String,
    /// Trust-root rotation cue token from the record.
    pub rotation_cue_token: String,
    /// Outcome token from the record.
    pub outcome_token: String,
    /// Denial reason token from the record; empty when not denied.
    pub denial_reason_token: String,
    /// Mirror/offline behavior token from the record.
    pub mirror_offline_behavior_token: String,
    /// `true` when no direct CA override is present.
    pub no_direct_ca_override: bool,
    /// `true` when no silent trust downgrade is present.
    pub no_silent_trust_downgrade: bool,
    /// `true` when local-core continuity is preserved.
    pub local_core_continuity_preserved: bool,
    /// `true` when a policy epoch ref is present.
    pub policy_epoch_present: bool,
    /// `true` when raw trust material is excluded.
    pub raw_trust_material_excluded: bool,
    /// `true` when raw private-key material is excluded.
    pub private_key_material_excluded: bool,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the trust page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TrustSummary {
    /// Total row count (one row per record in the snapshot).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Surface tokens covered by the snapshot.
    pub surfaces_covered: Vec<String>,
    /// Number of records with no direct CA override.
    pub no_direct_ca_override_count: usize,
    /// Number of records with no silent trust downgrade.
    pub no_silent_trust_downgrade_count: usize,
    /// Number of records that preserve local-core continuity.
    pub local_core_continuity_preserved_count: usize,
    /// Number of records that expose a typed host-proof state.
    pub host_proof_state_exposed_count: usize,
    /// Record counts by trust-store source token.
    pub trust_store_source_counts: BTreeMap<String, usize>,
    /// Record counts by host-proof state token.
    pub host_proof_state_counts: BTreeMap<String, usize>,
    /// Record counts by outcome token.
    pub outcome_counts: BTreeMap<String, usize>,
    /// Record counts by rotation cue token.
    pub rotation_cue_counts: BTreeMap<String, usize>,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl TrustSummary {
    fn from_rows(rows: &[TrustRow], snapshot: &TrustSnapshot, defects: &[TrustDefect]) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        let has_withdrawal = defects
            .iter()
            .any(|d| d.narrow_reason.is_withdrawal_reason());
        let has_preview = defects.iter().any(|d| d.narrow_reason.is_preview_reason());
        let overall = if has_withdrawal || withdrawn > 0 {
            TrustQualificationClass::Withdrawn
        } else if has_preview || preview > 0 {
            TrustQualificationClass::Preview
        } else if !defects.is_empty() || beta > 0 {
            TrustQualificationClass::Beta
        } else {
            TrustQualificationClass::Stable
        };
        let surfaces_covered: Vec<String> = snapshot
            .records
            .iter()
            .map(|r| r.surface_token.clone())
            .collect();
        let no_direct_ca_override_count = snapshot
            .records
            .iter()
            .filter(|r| r.no_direct_ca_override)
            .count();
        let no_silent_trust_downgrade_count = snapshot
            .records
            .iter()
            .filter(|r| r.no_silent_trust_downgrade)
            .count();
        let local_core_continuity_preserved_count = snapshot
            .records
            .iter()
            .filter(|r| r.local_core_continuity_preserved)
            .count();
        let host_proof_state_exposed_count = snapshot
            .records
            .iter()
            .filter(|r| r.exposes_host_proof_state())
            .count();
        let mut trust_store_source_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut host_proof_state_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut outcome_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut rotation_cue_counts: BTreeMap<String, usize> = BTreeMap::new();
        for record in &snapshot.records {
            *trust_store_source_counts
                .entry(record.ca_bundle.trust_store_source_token.clone())
                .or_insert(0) += 1;
            *host_proof_state_counts
                .entry(record.host_proof.state_token.clone())
                .or_insert(0) += 1;
            *outcome_counts
                .entry(record.outcome_token.clone())
                .or_insert(0) += 1;
            *rotation_cue_counts
                .entry(record.trust_root.rotation_cue_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            surfaces_covered,
            no_direct_ca_override_count,
            no_silent_trust_downgrade_count,
            local_core_continuity_preserved_count,
            host_proof_state_exposed_count,
            trust_store_source_counts,
            host_proof_state_counts,
            outcome_counts,
            rotation_cue_counts,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the trust page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: TrustNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (surface token or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl TrustDefect {
    fn new(
        narrow_reason: TrustNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: TRANSPORT_TRUST_DEFECT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:networked-surface-transport-trust:{}:{}",
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
// Transport-trust page (proof packet)
// ---------------------------------------------------------------------------

/// Stable transport-trust proof packet for the network-capable surfaces.
///
/// The packet is the single inspectable record that proves every claimed M5
/// network-capable surface exposes its trust-store source, CA bundle / pin-set
/// review state, host-proof state, client-certificate posture, and trust-root
/// freshness / rotation cue through one shared model with a typed `deny_trust`
/// vocabulary and no direct CA override or silent trust downgrade. Dashboards,
/// docs, Help/About surfaces, CLI/headless output, support exports, release
/// tooling, and diagnostics should ingest this packet rather than reconstructing
/// trust from raw certificate material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportTrustPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Repo-relative ref to the canonical evidence index this packet binds into.
    pub evidence_index_ref: String,
    /// Aggregate summary derived from all rows.
    pub summary: TrustSummary,
    /// Per-record stability rows.
    pub rows: Vec<TrustRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<TrustDefect>,
    /// The trust snapshot embedded as evidence.
    pub trust_snapshot: TrustSnapshot,
}

impl TransportTrustPage {
    /// Build the transport-trust page from a trust snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        trust_snapshot: TrustSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&trust_snapshot);
        let rows = derive_rows(&trust_snapshot, &defects);
        let summary = TrustSummary::from_rows(&rows, &trust_snapshot, &defects);
        Self {
            record_kind: TRANSPORT_TRUST_PAGE_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            evidence_index_ref: TRANSPORT_TRUST_EVIDENCE_INDEX_REF.to_owned(),
            summary,
            rows,
            defects,
            trust_snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token == TrustQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all required surfaces have a record.
    pub fn covers_all_required_surfaces(&self) -> bool {
        let covered = self.trust_snapshot.covered_surface_tokens();
        REQUIRED_SURFACES
            .iter()
            .all(|surface| covered.contains(surface.as_str()))
    }

    /// Returns `true` when no record ships a direct CA override.
    pub fn no_record_ships_direct_ca_override(&self) -> bool {
        self.trust_snapshot
            .records
            .iter()
            .all(|r| r.no_direct_ca_override && !r.ca_bundle.is_direct_ca_override)
    }

    /// Returns `true` when no record silently downgrades trust.
    pub fn no_record_allows_silent_trust_downgrade(&self) -> bool {
        self.trust_snapshot
            .records
            .iter()
            .all(|r| r.no_silent_trust_downgrade)
    }

    /// Returns `true` when every record exposes a typed host-proof state.
    pub fn all_records_expose_host_proof_state(&self) -> bool {
        self.trust_snapshot
            .records
            .iter()
            .all(|r| r.exposes_host_proof_state())
    }

    /// Returns `true` when every record exposes a complete set of trust inputs.
    pub fn all_records_expose_trust_inputs(&self) -> bool {
        self.trust_snapshot
            .records
            .iter()
            .all(|r| r.is_fully_classified())
    }

    /// Returns `true` when every record whose trust root needs rotation carries
    /// an active rotation cue.
    pub fn rotation_cues_consistent(&self) -> bool {
        self.trust_snapshot
            .records
            .iter()
            .all(|r| r.rotation_cue_consistent())
    }

    /// Returns `true` when every denied record carries a typed denial reason.
    pub fn denied_records_carry_reasons(&self) -> bool {
        self.trust_snapshot
            .records
            .iter()
            .all(|r| !r.outcome.requires_denial_reason() || r.denial_reason.is_some())
    }

    /// Returns `true` when every egress class that requires a policy epoch ref
    /// carries one.
    pub fn egress_classes_have_policy_epoch_refs(&self) -> bool {
        self.trust_snapshot.records.iter().all(|r| {
            if r.egress_class.requires_policy_epoch_ref() {
                r.policy_epoch_ref.is_some()
            } else {
                true
            }
        })
    }

    /// Render a stable CLI/headless view of the packet so terminal, diagnostics,
    /// and support surfaces quote identical trust tokens and outcome codes.
    pub fn render_cli_view(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "TRANSPORT TRUST — {}", self.page_label);
        let _ = writeln!(
            out,
            "overall: {}  rows: {}  stable: {}  beta: {}  preview: {}  withdrawn: {}",
            self.summary.overall_qualification_token,
            self.summary.row_count,
            self.summary.stable_row_count,
            self.summary.beta_row_count,
            self.summary.preview_row_count,
            self.summary.withdrawn_row_count,
        );
        let _ = writeln!(
            out,
            "guardrails: no_direct_ca_override={} no_silent_trust_downgrade={} host_proof_state_exposed={}",
            self.summary.no_direct_ca_override_count,
            self.summary.no_silent_trust_downgrade_count,
            self.summary.host_proof_state_exposed_count,
        );
        for row in &self.rows {
            let denial = if row.denial_reason_token.is_empty() {
                "-"
            } else {
                row.denial_reason_token.as_str()
            };
            let _ = writeln!(
                out,
                "  {:<26} store={:<18} host_proof={:<18} client_cert={:<20} root={:<20} cue={:<18} outcome={:<22} deny={:<26} {}",
                row.surface_token,
                row.trust_store_source_token,
                row.host_proof_state_token,
                row.client_cert_posture_token,
                row.trust_root_freshness_token,
                row.rotation_cue_token,
                row.outcome_token,
                denial,
                row.qualification_token,
            );
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the transport-trust page plus a
/// metadata-safe defect roll-up.
///
/// No raw CA bundles, raw certificate bytes, raw private keys, raw SSH host or
/// private keys, raw credentials, raw cookies, or raw bearer/session tokens may
/// appear in this export. Only closed-vocabulary tokens, opaque refs, counts,
/// and plain-language summary sentences cross the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The transport-trust page embedded as evidence.
    pub page: TransportTrustPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<TrustNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw trust material is excluded from this export.
    pub raw_trust_material_excluded: bool,
    /// `true` when raw private-key material is excluded from this export.
    pub private_key_material_excluded: bool,
}

impl TrustSupportExport {
    /// Wrap a transport-trust page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: TransportTrustPage,
    ) -> Self {
        let mut reasons: Vec<TrustNarrowReasonClass> = Vec::new();
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
            record_kind: TRANSPORT_TRUST_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_trust_material_excluded: true,
            private_key_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions (public API)
// ---------------------------------------------------------------------------

/// Re-run the trust audit over the snapshot embedded in a page.
pub fn audit_transport_trust_page(page: &TransportTrustPage) -> Vec<TrustDefect> {
    audit_snapshot(&page.trust_snapshot)
}

/// Validate a transport-trust page; returns `Ok` when the audit is clean.
pub fn validate_transport_trust_page(page: &TransportTrustPage) -> Result<(), Vec<TrustDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &TrustSnapshot) -> Vec<TrustDefect> {
    let mut defects: Vec<TrustDefect> = Vec::new();

    // Hard guardrails first — any one of these withdraws the packet and makes no
    // further check meaningful.
    for record in &snapshot.records {
        if !record.raw_trust_material_excluded {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::RawTrustMaterialExposed,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' exposes raw trust material; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
        if !record.private_key_material_excluded {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::PrivateKeyMaterialExposed,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' exposes raw private-key material; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
        if !record.no_direct_ca_override || record.ca_bundle.is_direct_ca_override {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::DirectCaOverrideShipped,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' ships a direct CA override; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
        if !record.no_silent_trust_downgrade {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::SilentTrustDowngrade,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' silently downgrades trust; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
    }

    let covered: BTreeSet<&str> = snapshot
        .records
        .iter()
        .map(|r| r.surface_token.as_str())
        .collect();

    // Coverage check: all required surfaces must have a record.
    for required_surface in &REQUIRED_SURFACES {
        if !covered.contains(required_surface.as_str()) {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::RequiredSurfaceMissing,
                required_surface.as_str(),
                format!(
                    "required surface '{}' has no trust-evaluation record; packet is narrowed to preview",
                    required_surface.as_str()
                ),
            ));
        }
    }

    // Per-record checks.
    for record in &snapshot.records {
        if record.outcome.requires_denial_reason() && record.denial_reason.is_none() {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::DenyReasonMissing,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' is denied but carries no typed deny_trust reason",
                    record.record_id, record.surface_token
                ),
            ));
        }

        if !record.is_fully_classified() {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::TrustInputClassificationIncomplete,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' is missing or inconsistent in its trust-input/host-proof classification",
                    record.record_id, record.surface_token
                ),
            ));
        } else if !record.rotation_cue_consistent() {
            // Only meaningful when the descriptors are well-formed.
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::RotationCueMissing,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' has trust-root freshness '{}' but no active rotation cue",
                    record.record_id,
                    record.surface_token,
                    record.trust_root.freshness_token
                ),
            ));
        }

        if !record.local_core_continuity_preserved {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::LocalCoreContinuityNotPreserved,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' does not preserve local-core continuity; local work may be blocked",
                    record.record_id, record.surface_token
                ),
            ));
        }

        if record.egress_class.requires_policy_epoch_ref() && record.policy_epoch_ref.is_none() {
            defects.push(TrustDefect::new(
                TrustNarrowReasonClass::PolicyEpochRefMissing,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' ({}) has no policy_epoch_ref; the governing trust policy must be traceable",
                    record.record_id, record.surface_token, record.egress_class_token
                ),
            ));
        }
    }

    defects
}

fn derive_rows(snapshot: &TrustSnapshot, page_defects: &[TrustDefect]) -> Vec<TrustRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_preview_reason());

    let overall_narrow_reason = if has_withdrawal {
        page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(TrustNarrowReasonClass::RawTrustMaterialExposed)
    } else if has_preview {
        TrustNarrowReasonClass::RequiredSurfaceMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        TrustNarrowReasonClass::NotNarrowed
    };

    snapshot
        .records
        .iter()
        .map(|record| {
            let row_narrow = find_row_narrow_reason(record, page_defects, overall_narrow_reason);
            let row_qual = qualification_for_reason(row_narrow);
            let summary = build_row_summary(&record.surface_token, &row_qual, row_narrow);
            TrustRow {
                record_kind: TRANSPORT_TRUST_ROW_RECORD_KIND.to_owned(),
                schema_version: TRANSPORT_TRUST_SCHEMA_VERSION,
                shared_contract_ref: TRANSPORT_TRUST_SHARED_CONTRACT_REF.to_owned(),
                record_id: record.record_id.clone(),
                surface_token: record.surface_token.clone(),
                origin_scope_token: record.origin_scope_token.clone(),
                egress_class_token: record.egress_class_token.clone(),
                trust_store_source_token: record.ca_bundle.trust_store_source_token.clone(),
                ca_review_state_token: record.ca_bundle.review_state_token.clone(),
                ca_pin_count: record.ca_bundle.pin_count,
                host_proof_state_token: record.host_proof.state_token.clone(),
                client_cert_posture_token: record.client_cert.posture_token.clone(),
                trust_root_freshness_token: record.trust_root.freshness_token.clone(),
                rotation_cue_token: record.trust_root.rotation_cue_token.clone(),
                outcome_token: record.outcome_token.clone(),
                denial_reason_token: record.denial_reason_token.clone(),
                mirror_offline_behavior_token: record.mirror_offline_behavior_token.clone(),
                no_direct_ca_override: record.no_direct_ca_override,
                no_silent_trust_downgrade: record.no_silent_trust_downgrade,
                local_core_continuity_preserved: record.local_core_continuity_preserved,
                policy_epoch_present: record.policy_epoch_ref.is_some(),
                raw_trust_material_excluded: record.raw_trust_material_excluded,
                private_key_material_excluded: record.private_key_material_excluded,
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn qualification_for_reason(reason: TrustNarrowReasonClass) -> TrustQualificationClass {
    if reason.is_withdrawal_reason() {
        TrustQualificationClass::Withdrawn
    } else if reason.is_preview_reason() {
        TrustQualificationClass::Preview
    } else if reason != TrustNarrowReasonClass::NotNarrowed {
        TrustQualificationClass::Beta
    } else {
        TrustQualificationClass::Stable
    }
}

fn find_row_narrow_reason(
    record: &TrustEvaluationRecord,
    page_defects: &[TrustDefect],
    overall_narrow_reason: TrustNarrowReasonClass,
) -> TrustNarrowReasonClass {
    if overall_narrow_reason.is_withdrawal_reason() {
        return overall_narrow_reason;
    }
    if let Some(defect) = page_defects
        .iter()
        .find(|d| d.source == record.surface_token)
    {
        return defect.narrow_reason;
    }
    TrustNarrowReasonClass::NotNarrowed
}

fn build_row_summary(
    surface_token: &str,
    qual: &TrustQualificationClass,
    narrow_reason: TrustNarrowReasonClass,
) -> String {
    match qual {
        TrustQualificationClass::Stable => format!(
            "Surface '{}' trust evaluation qualifies stable: the trust-store source, CA bundle \
             review state, host-proof state, client-certificate posture, and trust-root freshness \
             / rotation cue are typed; no direct CA override or silent trust downgrade is present; \
             and local editing continues regardless of the route.",
            surface_token
        ),
        _ => format!(
            "Surface '{}' trust evaluation narrowed to {} ({}): see defect list for details.",
            surface_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable transport-trust page consumed by the headless
/// example, the integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: all required surfaces have a record,
/// no raw trust or private-key material is present, no record ships a direct CA
/// override or silent trust downgrade, every record exposes a typed host-proof
/// state and a complete set of trust inputs, every denied record carries a typed
/// reason, every record whose trust root needs rotation carries an active cue,
/// and every record carries a policy epoch ref where required.
pub fn seeded_transport_trust_page() -> TransportTrustPage {
    TransportTrustPage::new(
        "remote:networked_surface_transport_trust:default",
        "Networked-surface transport-trust governance — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_transport_trust_snapshot(),
    )
}

/// Build the seeded transport-trust snapshot used by the seeded page.
///
/// Each required surface is represented with a fully-typed, clean record that
/// passes all stability conditions. The records together exercise every
/// trust-store source, host-proof state (pinned, TOFU, not-applicable, and a
/// changed-mismatch deny), client-certificate posture, and trust-root freshness
/// / rotation cue, plus a typed `deny_trust` outcome.
pub fn seeded_transport_trust_snapshot() -> TrustSnapshot {
    let at = "2026-06-01T00:00:00Z";
    TrustSnapshot {
        records: vec![
            // AI gateway — managed org bundle, pinned host proof, mutual TLS, fresh root.
            TrustEvaluationRecord::new(
                "remote:transport_trust:ai_gateway:0001",
                at,
                SurfaceClass::AiGateway,
                OriginScopeClass::ManagedTenant,
                EgressClass::ManagedEndpoint,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::ManagedOrgBundle,
                    CaBundleReviewClass::OrgReviewed,
                    3,
                    "trust_bundle:ai_gateway:org:0001",
                    "Organization-reviewed managed CA bundle pins the gateway roots.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::PinnedMatch,
                    "host_proof:ai_gateway:0001",
                    4,
                    "Gateway host proof matches the pinned proof; four prior entries recorded.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::RequiredPresented,
                    "client_cert:ai_gateway:0001",
                    "Mutual TLS client certificate is bound and presented to the gateway.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::Fresh,
                    RotationCueClass::None,
                    Some("rotation:ai_gateway:2026-06-01"),
                    "Trust root is fresh; no rotation cue is active.",
                ),
                TrustOutcomeClass::Trusted,
                None,
                MirrorOfflineBehaviorClass::LocalCoreOnly,
                Some("epoch:ai_gateway:2026-06-01"),
                true,
                "AI gateway trust resolved against the organization-reviewed managed bundle with a \
                 pinned host proof and a presented mutual-TLS client certificate; the trust root \
                 is fresh; local editing continues without the gateway.",
            ),
            // Docs / browser fetcher — system trust store, TOFU, rotation due (cue).
            TrustEvaluationRecord::new(
                "remote:transport_trust:docs_browser_fetcher:0001",
                at,
                SurfaceClass::DocsBrowserFetcher,
                OriginScopeClass::ThirdParty,
                EgressClass::PublicInternet,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::SystemTrustStore,
                    CaBundleReviewClass::SystemDefault,
                    0,
                    "trust_bundle:docs_browser_fetcher:system:0001",
                    "Platform system trust store backs documentation fetches.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::KnownTofu,
                    "host_proof:docs_browser_fetcher:0001",
                    2,
                    "Documentation origin host proof matches the trust-on-first-use record.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::NotRequired,
                    "",
                    "No client certificate is required for documentation fetches.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::RotationDue,
                    RotationCueClass::RotateSoon,
                    Some("rotation:docs_browser_fetcher:2026-09-01"),
                    "Trust root rotation is due soon; a rotate-soon cue is surfaced.",
                ),
                TrustOutcomeClass::TrustedRotationDue,
                None,
                MirrorOfflineBehaviorClass::CachedOffline,
                Some("epoch:docs_browser_fetcher:2026-06-01"),
                true,
                "Documentation fetch trust resolved against the system trust store with a \
                 trust-on-first-use host proof; the trust root is valid but rotation is due soon, \
                 so a rotate-soon cue is surfaced; cached content stays available offline.",
            ),
            // Request / API client — pinned CA set, pinned host proof, fresh root.
            TrustEvaluationRecord::new(
                "remote:transport_trust:request_api_client:0001",
                at,
                SurfaceClass::RequestApiClient,
                OriginScopeClass::UserConfigured,
                EgressClass::PublicInternet,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::PinnedCaSet,
                    CaBundleReviewClass::PinnedSet,
                    2,
                    "trust_bundle:request_api_client:pinned:0001",
                    "User-configured pinned CA set backs the API client.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::PinnedMatch,
                    "host_proof:request_api_client:0001",
                    3,
                    "API host proof matches the pinned proof.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::NotRequired,
                    "",
                    "No client certificate is required for this API client.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::Fresh,
                    RotationCueClass::None,
                    Some("rotation:request_api_client:2026-06-01"),
                    "Trust root is fresh; no rotation cue is active.",
                ),
                TrustOutcomeClass::Trusted,
                None,
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:request_api_client:2026-06-01"),
                true,
                "API client trust resolved against a user-configured pinned CA set with a pinned \
                 host proof; the trust root is fresh; the surface denies all when offline and \
                 local work continues.",
            ),
            // Database / cloud connector — pinned CA set, pinned host proof, mutual TLS.
            TrustEvaluationRecord::new(
                "remote:transport_trust:database_cloud_connector:0001",
                at,
                SurfaceClass::DatabaseCloudConnector,
                OriginScopeClass::UserConfigured,
                EgressClass::PublicInternet,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::PinnedCaSet,
                    CaBundleReviewClass::PinnedSet,
                    1,
                    "trust_bundle:database_cloud_connector:pinned:0001",
                    "Pinned CA set backs the data-store connector.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::PinnedMatch,
                    "host_proof:database_cloud_connector:0001",
                    5,
                    "Data-store host proof matches the pinned proof.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::RequiredPresented,
                    "client_cert:database_cloud_connector:0001",
                    "Mutual TLS client certificate is bound and presented to the data store.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::Fresh,
                    RotationCueClass::None,
                    Some("rotation:database_cloud_connector:2026-06-01"),
                    "Trust root is fresh; no rotation cue is active.",
                ),
                TrustOutcomeClass::Trusted,
                None,
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:database_cloud_connector:2026-06-01"),
                true,
                "Data-store trust resolved against a pinned CA set with a pinned host proof and a \
                 presented mutual-TLS client certificate; the trust root is fresh; the connector \
                 denies all when offline.",
            ),
            // Registry read — mirror root, pinned host proof, pinned-static root.
            TrustEvaluationRecord::new(
                "remote:transport_trust:registry_read:0001",
                at,
                SurfaceClass::RegistryRead,
                OriginScopeClass::FirstParty,
                EgressClass::MirrorOnly,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::MirrorRoot,
                    CaBundleReviewClass::MirrorRoot,
                    1,
                    "trust_bundle:registry_read:mirror:0001",
                    "Signed-mirror trust root backs registry reads.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::PinnedMatch,
                    "host_proof:registry_read:0001",
                    6,
                    "Mirror host proof matches the pinned mirror proof.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::NotRequired,
                    "",
                    "No client certificate is required for mirror reads.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::PinnedStatic,
                    RotationCueClass::PinnedNoRotation,
                    None::<String>,
                    "Mirror trust root is pinned static; no rotation is expected.",
                ),
                TrustOutcomeClass::Trusted,
                None,
                MirrorOfflineBehaviorClass::MirrorFirstThenDeny,
                Some("epoch:registry_read:2026-06-01"),
                true,
                "Registry read trust resolved against the signed-mirror root with a pinned host \
                 proof; the mirror root is pinned static and not expected to rotate; the route \
                 denies rather than falling through to an untrusted root.",
            ),
            // Companion handoff — loopback, no TLS, not-applicable trust.
            TrustEvaluationRecord::new(
                "remote:transport_trust:companion_handoff:0001",
                at,
                SurfaceClass::CompanionHandoff,
                OriginScopeClass::LoopbackLocal,
                EgressClass::LoopbackOnly,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::NoTlsLoopback,
                    CaBundleReviewClass::NotApplicable,
                    0,
                    "trust_bundle:companion_handoff:loopback:0001",
                    "Loopback handoff carries no TLS trust material.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::NotApplicable,
                    "host_proof:companion_handoff:0001",
                    0,
                    "No host proof applies on the on-device loopback boundary.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::NotRequired,
                    "",
                    "No client certificate is required on the loopback boundary.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::PinnedStatic,
                    RotationCueClass::PinnedNoRotation,
                    None::<String>,
                    "No trust root applies on the loopback boundary.",
                ),
                TrustOutcomeClass::NotApplicableLoopback,
                None,
                MirrorOfflineBehaviorClass::LocalCoreOnly,
                None::<String>,
                true,
                "Companion handoff trust is not applicable on the on-device loopback boundary; no \
                 TLS trust material, host proof, or client certificate participates; the desktop \
                 continues without the companion.",
            ),
            // Provider mutation — managed org bundle, pinned host proof, rotation in progress.
            TrustEvaluationRecord::new(
                "remote:transport_trust:provider_mutation:0001",
                at,
                SurfaceClass::ProviderMutation,
                OriginScopeClass::ManagedTenant,
                EgressClass::ManagedEndpoint,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::ManagedOrgBundle,
                    CaBundleReviewClass::OrgReviewed,
                    4,
                    "trust_bundle:provider_mutation:org:0001",
                    "Organization-reviewed managed CA bundle backs provider mutation.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::PinnedMatch,
                    "host_proof:provider_mutation:0001",
                    7,
                    "Provider host proof matches the pinned proof.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::RequiredPresented,
                    "client_cert:provider_mutation:0001",
                    "Mutual TLS client certificate is bound and presented to the provider.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::RotationInProgress,
                    RotationCueClass::Rotating,
                    Some("rotation:provider_mutation:2026-06-10"),
                    "Trust-root rotation is in progress; overlapping roots are trusted.",
                ),
                TrustOutcomeClass::Trusted,
                None,
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:provider_mutation:2026-06-01"),
                true,
                "Provider mutation trust resolved against the organization-reviewed managed bundle \
                 with a pinned host proof and a presented mutual-TLS client certificate; a \
                 trust-root rotation is in progress with overlapping roots trusted; the surface \
                 denies all when offline.",
            ),
            // Sync / offboarding — managed org bundle, TOFU, managed client cert, fresh.
            TrustEvaluationRecord::new(
                "remote:transport_trust:sync_offboarding:0001",
                at,
                SurfaceClass::SyncOffboarding,
                OriginScopeClass::ManagedTenant,
                EgressClass::ManagedEndpoint,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::ManagedOrgBundle,
                    CaBundleReviewClass::OrgReviewed,
                    3,
                    "trust_bundle:sync_offboarding:org:0001",
                    "Organization-reviewed managed CA bundle backs sync traffic.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::KnownTofu,
                    "host_proof:sync_offboarding:0001",
                    3,
                    "Sync host proof matches the trust-on-first-use record.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::ManagedProvisioned,
                    "client_cert:sync_offboarding:0001",
                    "A managed client certificate was provisioned for sync traffic.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::Fresh,
                    RotationCueClass::None,
                    Some("rotation:sync_offboarding:2026-06-01"),
                    "Trust root is fresh; no rotation cue is active.",
                ),
                TrustOutcomeClass::Trusted,
                None,
                MirrorOfflineBehaviorClass::OfflineGrace,
                Some("epoch:sync_offboarding:2026-06-01"),
                true,
                "Sync trust resolved against the organization-reviewed managed bundle with a \
                 trust-on-first-use host proof and a managed-provisioned client certificate; the \
                 trust root is fresh; the surface holds within its offline-grace window and local \
                 data is retained.",
            ),
            // Remote preview route — host proof changed -> typed deny_trust.
            TrustEvaluationRecord::new(
                "remote:transport_trust:remote_preview_route:0001",
                at,
                SurfaceClass::RemotePreviewRoute,
                OriginScopeClass::FirstParty,
                EgressClass::ManagedEndpoint,
                CaBundleDescriptor::new(
                    TrustStoreSourceClass::PinnedCaSet,
                    CaBundleReviewClass::PinnedSet,
                    2,
                    "trust_bundle:remote_preview_route:pinned:0001",
                    "Pinned CA set backs the remote preview route.",
                ),
                HostProofDescriptor::new(
                    HostProofStateClass::ChangedMismatch,
                    "host_proof:remote_preview_route:0001",
                    4,
                    "Preview host proof changed from the recorded proof; trust is refused.",
                ),
                ClientCertDescriptor::new(
                    ClientCertPostureClass::NotRequired,
                    "",
                    "No client certificate is required for the preview route.",
                ),
                TrustRootDescriptor::new(
                    TrustRootFreshnessClass::Fresh,
                    RotationCueClass::None,
                    Some("rotation:remote_preview_route:2026-06-01"),
                    "Trust root is fresh, but the host proof changed.",
                ),
                TrustOutcomeClass::DenyTrust,
                Some(TrustDenialClass::HostProofChanged),
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:remote_preview_route:2026-06-01"),
                true,
                "Remote preview trust found a changed host proof (the recorded proof no longer \
                 matches); trust is denied with a typed deny_trust reason rather than silently \
                 accepting the new proof; the local workspace continues without the preview.",
            ),
        ],
    }
}
