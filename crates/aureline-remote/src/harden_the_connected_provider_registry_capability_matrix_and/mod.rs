//! Harden the connected-provider registry, capability matrix, and snapshot
//! degradation across stable provider families.
//!
//! This module produces a stable proof packet that makes every claimed
//! connected-provider family — code-host, issue/planning, and CI/checks —
//! inspectable through one typed vocabulary across all four deployment
//! profiles (public SaaS, self-hosted, managed, and air-gapped mirror).
//!
//! Each provider descriptor row carries:
//! - the provider family (code_host, issue_tracker, ci_checks),
//! - the acting identity class (human_account, installation_grant,
//!   delegated_credential, or local_only_no_account),
//! - the callback/ingress path class (public_saas, mirrored_ingress, or
//!   customer_controlled),
//! - the supported object kinds with inspect-vs-mutate posture,
//! - the supported publish modes (local_draft, publish_now,
//!   open_in_provider, publish_later_queue, inspect_only),
//! - the snapshot freshness class,
//! - an explicit local-core continuity declaration,
//! - the dependency class (local_only, network, managed, air_gapped), and
//! - a mirror/self-host posture note.
//!
//! The stable claim holds when **all six** of the following conditions are
//! verified simultaneously:
//!
//! 1. All nine required (provider_family × actor_identity) pairs are covered
//!    (three families × three actor classes: human_account, installation_grant,
//!    and delegated_credential).
//! 2. No raw private material is exposed on any descriptor record.
//! 3. Every descriptor explicitly declares its local-core continuity posture.
//! 4. Every descriptor carries an explicit dependency class.
//! 5. Every descriptor names its callback/ingress path class.
//! 6. Every descriptor covers at least one object kind with an explicit
//!    inspect-vs-mutate posture.
//!
//! One condition forces `Withdrawn` immediately and cannot be overridden:
//!
//! - Any descriptor record carries `raw_private_material_excluded: false`
//!   (narrow reason: [`ProviderRegistryNarrowReasonClass::RawPrivateMaterialExposed`]).
//!
//! A missing required row narrows to `Preview` rather than `Beta` because
//! the coverage gap prevents any verifiable claim for that (family, actor)
//! pair.
//!
//! Snapshot degradation is automatic: unsupported or policy-blocked mutation
//! classes narrow the affected descriptor below Stable in product copy, docs,
//! and release packets instead of inheriting adjacent green rows. Stale,
//! expired, or revoked descriptors carry an explicit freshness class and
//! degraded reason so provider settings and support packets can explain the
//! state without private maintainer knowledge.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque refs only. Raw provider URLs, raw tokens, raw callback bodies,
//! raw private keys, and raw policy bundle bodies never appear on any record.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/harden-the-connected-provider-registry-capability-matrix-and.md`
//! - Artifact: `artifacts/enterprise/m4/harden-the-connected-provider-registry-capability-matrix-and.md`
//! - Contract ref: [`PROVIDER_REGISTRY_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const PROVIDER_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const PROVIDER_REGISTRY_SHARED_CONTRACT_REF: &str =
    "remote:provider_registry_harden:v1";

/// Record-kind tag for [`ProviderRegistryPage`] payloads.
pub const PROVIDER_REGISTRY_PAGE_RECORD_KIND: &str =
    "remote_provider_registry_page_record";

/// Record-kind tag for [`ProviderDescriptorRecord`] payloads.
pub const PROVIDER_DESCRIPTOR_RECORD_KIND: &str =
    "remote_provider_descriptor_record";

/// Record-kind tag for [`ProviderRegistryRow`] payloads.
pub const PROVIDER_REGISTRY_ROW_RECORD_KIND: &str =
    "remote_provider_registry_row_record";

/// Record-kind tag for [`ProviderRegistryDefect`] payloads.
pub const PROVIDER_REGISTRY_DEFECT_RECORD_KIND: &str =
    "remote_provider_registry_defect_record";

/// Record-kind tag for [`ProviderRegistrySummary`] payloads.
pub const PROVIDER_REGISTRY_SUMMARY_RECORD_KIND: &str =
    "remote_provider_registry_summary_record";

/// Record-kind tag for [`ProviderRegistrySupportExport`] payloads.
pub const PROVIDER_REGISTRY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_provider_registry_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const PROVIDER_REGISTRY_DOC_REF: &str =
    "docs/enterprise/m4/harden-the-connected-provider-registry-capability-matrix-and.md";

/// Repo-relative path of the artifact summary for this lane.
pub const PROVIDER_REGISTRY_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/harden-the-connected-provider-registry-capability-matrix-and.md";

/// All nine required (provider_family, actor_identity) pairs in canonical order.
///
/// Three families × three actor identity classes = nine required rows.
pub const REQUIRED_DESCRIPTOR_PAIRS: [(ProviderFamilyClass, ActorIdentityClass); 9] = [
    (ProviderFamilyClass::CodeHost, ActorIdentityClass::HumanAccount),
    (ProviderFamilyClass::CodeHost, ActorIdentityClass::InstallationGrant),
    (ProviderFamilyClass::CodeHost, ActorIdentityClass::DelegatedCredential),
    (ProviderFamilyClass::IssueTracker, ActorIdentityClass::HumanAccount),
    (ProviderFamilyClass::IssueTracker, ActorIdentityClass::InstallationGrant),
    (ProviderFamilyClass::IssueTracker, ActorIdentityClass::DelegatedCredential),
    (ProviderFamilyClass::CiChecks, ActorIdentityClass::HumanAccount),
    (ProviderFamilyClass::CiChecks, ActorIdentityClass::InstallationGrant),
    (ProviderFamilyClass::CiChecks, ActorIdentityClass::DelegatedCredential),
];

// ---------------------------------------------------------------------------
// Provider family vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the three claimed provider families.
///
/// Each variant corresponds to a distinct subsystem whose outbound requests,
/// callback events, and object mutations must be governed through the
/// provider registry proof packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFamilyClass {
    /// VCS host (e.g., cloud or self-hosted code repository service).
    CodeHost,
    /// Issue tracker or planning tool.
    IssueTracker,
    /// Continuous integration or check-run provider.
    CiChecks,
}

impl ProviderFamilyClass {
    /// Stable closed-vocabulary token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeHost => "code_host",
            Self::IssueTracker => "issue_tracker",
            Self::CiChecks => "ci_checks",
        }
    }

    /// Human-readable label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CodeHost => "Code host",
            Self::IssueTracker => "Issue tracker",
            Self::CiChecks => "CI / checks",
        }
    }
}

// ---------------------------------------------------------------------------
// Actor identity vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the acting identity class on a provider descriptor.
///
/// Using a typed token instead of generic `connected` language ensures that
/// acting authority is always explicit on every descriptor row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorIdentityClass {
    /// A human user's personal connected account; auth via system browser or
    /// OIDC session.
    HumanAccount,
    /// An installation grant or app install (e.g., a platform app granted
    /// access to one or more repositories); auth via installation token.
    InstallationGrant,
    /// A delegated user token or service credential narrowed below the
    /// human-account scope.
    DelegatedCredential,
    /// No connected account; local-only authority with no provider callbacks.
    LocalOnlyNoAccount,
}

impl ActorIdentityClass {
    /// Stable closed-vocabulary token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanAccount => "human_account",
            Self::InstallationGrant => "installation_grant",
            Self::DelegatedCredential => "delegated_credential",
            Self::LocalOnlyNoAccount => "local_only_no_account",
        }
    }

    /// Human-readable label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::HumanAccount => "Human account",
            Self::InstallationGrant => "Installation grant",
            Self::DelegatedCredential => "Delegated credential",
            Self::LocalOnlyNoAccount => "Local only (no account)",
        }
    }
}

// ---------------------------------------------------------------------------
// Callback / ingress path vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the callback/ingress path that delivers provider
/// events to the IDE.
///
/// Making the path explicit ensures that deployment and sovereignty claims
/// remain auditable: a managed or self-hosted row may not silently inherit
/// the public SaaS posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackPathClass {
    /// Events arrive through the provider's public SaaS endpoint.
    PublicSaas,
    /// Events arrive through a declared mirrored ingress endpoint operated
    /// by the organization.
    MirroredIngress,
    /// Events arrive through a customer-controlled endpoint (self-hosted or
    /// enterprise-managed ingress).
    CustomerControlled,
    /// No inbound events; polling or import-only.
    PollingOrImportOnly,
}

impl CallbackPathClass {
    /// Stable closed-vocabulary token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicSaas => "public_saas",
            Self::MirroredIngress => "mirrored_ingress",
            Self::CustomerControlled => "customer_controlled",
            Self::PollingOrImportOnly => "polling_or_import_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Object kind vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for provider object kinds covered by the support matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectKindClass {
    /// Pull request or merge request.
    PullRequest,
    /// Branch or ref.
    Branch,
    /// Issue or planning work item.
    IssueOrWorkItem,
    /// Check run or status check.
    CheckRun,
    /// CI pipeline run.
    PipelineRun,
}

impl ObjectKindClass {
    /// Stable closed-vocabulary token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PullRequest => "pull_request",
            Self::Branch => "branch",
            Self::IssueOrWorkItem => "issue_or_work_item",
            Self::CheckRun => "check_run",
            Self::PipelineRun => "pipeline_run",
        }
    }
}

// ---------------------------------------------------------------------------
// Mutation posture vocabulary
// ---------------------------------------------------------------------------

/// Inspect-vs-mutate posture for a supported object kind.
///
/// Policy-blocked or unsupported mutation classes narrow automatically instead
/// of inheriting adjacent green rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationPostureClass {
    /// Read-only; no mutations allowed or supported.
    InspectOnly,
    /// Both inspect and mutate are supported.
    MutateAllowed,
    /// Mutations require a browser handoff; no in-editor direct mutation.
    BrowserOnlyMutate,
    /// Mutations require a publish-later queue; deferred, not immediate.
    PublishLaterOnly,
}

impl MutationPostureClass {
    /// Stable closed-vocabulary token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::MutateAllowed => "mutate_allowed",
            Self::BrowserOnlyMutate => "browser_only_mutate",
            Self::PublishLaterOnly => "publish_later_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Publish mode vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the supported surface publish modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishModeClass {
    /// Local draft only; not published to provider.
    LocalDraft,
    /// Publish immediately to provider.
    PublishNow,
    /// Open mutation in provider browser; no in-editor publish.
    OpenInProvider,
    /// Queue for deferred publish.
    PublishLaterQueue,
    /// Read-only; no publish supported.
    InspectOnly,
}

impl PublishModeClass {
    /// Stable closed-vocabulary token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraft => "local_draft",
            Self::PublishNow => "publish_now",
            Self::OpenInProvider => "open_in_provider",
            Self::PublishLaterQueue => "publish_later_queue",
            Self::InspectOnly => "inspect_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Snapshot freshness vocabulary
// ---------------------------------------------------------------------------

/// Freshness class for a provider descriptor snapshot.
///
/// Stale, expired, or revoked descriptors carry an explicit freshness class
/// and a degraded reason so provider settings and support packets can explain
/// the state without private maintainer knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotFreshnessClass {
    /// Descriptor is current and within its freshness window.
    Fresh,
    /// Descriptor is stale but within an acceptable grace window.
    StaleWithinWindow,
    /// Descriptor has expired beyond its freshness window.
    ExpiredBeyondWindow,
    /// Provider grant has been revoked or the connection is disconnected.
    RevokedOrDisconnected,
}

impl SnapshotFreshnessClass {
    /// Stable closed-vocabulary token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::StaleWithinWindow => "stale_within_window",
            Self::ExpiredBeyondWindow => "expired_beyond_window",
            Self::RevokedOrDisconnected => "revoked_or_disconnected",
        }
    }

    /// Returns `true` when this freshness class represents a usable state.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Fresh | Self::StaleWithinWindow)
    }
}

// ---------------------------------------------------------------------------
// Dependency class vocabulary
// ---------------------------------------------------------------------------

/// Ownership tier for a provider descriptor's external dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    /// No external network dependency; local-only with no account.
    LocalOnly,
    /// Requires a live network connection to a public or hosted endpoint.
    Network,
    /// Requires connectivity to a managed service controlled by an enterprise
    /// admin; local work continues without managed capabilities.
    Managed,
    /// Operates against a declared signed mirror or air-gapped media only.
    AirGapped,
}

impl DependencyClass {
    /// Stable closed-vocabulary token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Network => "network",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and individual rows.
///
/// The tier is derived, not asserted: it is set by comparing the audit defect
/// list against the six stability conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRegistryQualificationClass {
    /// All six stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required (family, actor) pair has no descriptor; the coverage gap
    /// prevents a beta claim for the missing pair.
    Preview,
    /// Raw private material was exposed on a descriptor record; the packet
    /// is withdrawn immediately and cannot be overridden.
    Withdrawn,
}

impl ProviderRegistryQualificationClass {
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
/// [`ProviderRegistryQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRegistryNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// A descriptor record carries `raw_private_material_excluded: false`;
    /// withdraws the packet immediately.
    RawPrivateMaterialExposed,
    /// A required (family, actor) pair has no descriptor record; narrows to
    /// preview.
    RequiredRowMissing,
    /// A descriptor does not declare its local-core continuity posture
    /// explicitly.
    LocalCoreContinuityUndeclared,
    /// A descriptor does not carry an explicit dependency class.
    DependencyClassUndeclared,
    /// A descriptor does not name its callback/ingress path class.
    CallbackPathUndeclared,
    /// A descriptor covers no object kind with an explicit inspect-vs-mutate
    /// posture.
    ObjectSupportUndeclared,
}

impl ProviderRegistryNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::RequiredRowMissing => "required_row_missing",
            Self::LocalCoreContinuityUndeclared => "local_core_continuity_undeclared",
            Self::DependencyClassUndeclared => "dependency_class_undeclared",
            Self::CallbackPathUndeclared => "callback_path_undeclared",
            Self::ObjectSupportUndeclared => "object_support_undeclared",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws the
    /// packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::RawPrivateMaterialExposed)
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredRowMissing)
    }
}

// ---------------------------------------------------------------------------
// Object support entry (per-object-kind row inside a descriptor)
// ---------------------------------------------------------------------------

/// Per-object-kind support entry inside a provider descriptor.
///
/// Each entry records the object kind, the inspect-vs-mutate posture, and
/// whether the mutation requires a browser handoff or publish-later queue.
/// No raw object IDs, raw URLs, or raw credential material may appear here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectSupportEntry {
    /// The object kind this entry covers.
    pub object_kind: ObjectKindClass,
    /// Stable token for [`Self::object_kind`].
    pub object_kind_token: String,
    /// Inspect-vs-mutate posture for this object kind.
    pub mutation_posture: MutationPostureClass,
    /// Stable token for [`Self::mutation_posture`].
    pub mutation_posture_token: String,
    /// Publish modes supported for this object kind.
    pub supported_publish_modes: Vec<PublishModeClass>,
    /// Stable tokens for [`Self::supported_publish_modes`].
    pub supported_publish_mode_tokens: Vec<String>,
    /// `true` when mutations require a browser handoff.
    pub browser_handoff_required: bool,
    /// `true` when offline publish-later is supported.
    pub offline_publish_later_supported: bool,
}

impl ObjectSupportEntry {
    /// Construct an object support entry, filling in all token fields.
    pub fn new(
        object_kind: ObjectKindClass,
        mutation_posture: MutationPostureClass,
        supported_publish_modes: Vec<PublishModeClass>,
        browser_handoff_required: bool,
        offline_publish_later_supported: bool,
    ) -> Self {
        let supported_publish_mode_tokens = supported_publish_modes
            .iter()
            .map(|m| m.as_str().to_owned())
            .collect();
        Self {
            object_kind,
            object_kind_token: object_kind.as_str().to_owned(),
            mutation_posture,
            mutation_posture_token: mutation_posture.as_str().to_owned(),
            supported_publish_modes,
            supported_publish_mode_tokens,
            browser_handoff_required,
            offline_publish_later_supported,
        }
    }
}

// ---------------------------------------------------------------------------
// Provider descriptor record (per-(family, actor) row)
// ---------------------------------------------------------------------------

/// Per-(provider_family, actor_identity) descriptor record.
///
/// Each record captures the provider family, acting identity class, callback
/// path, supported object kinds with inspect-vs-mutate posture, snapshot
/// freshness, dependency class, local-core continuity posture, and
/// mirror/self-host notes for one (family, actor) pair. Together the nine
/// required rows form the [`ProviderDescriptorSnapshot`] that the registry
/// proof packet embeds as evidence.
///
/// No raw provider URLs, raw tokens, raw callback bodies, raw private keys,
/// raw policy bundle bodies, or raw PII may appear on this record. Only
/// closed-vocabulary tokens, opaque refs, and plain-language summary
/// sentences cross the export boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderDescriptorRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Provider family for this descriptor.
    pub provider_family: ProviderFamilyClass,
    /// Stable token for [`Self::provider_family`].
    pub provider_family_token: String,
    /// Acting identity class for this descriptor.
    pub actor_identity: ActorIdentityClass,
    /// Stable token for [`Self::actor_identity`].
    pub actor_identity_token: String,
    /// Opaque ref identifying the canonical host.
    pub canonical_host_ref: String,
    /// Opaque ref identifying the tenant or org scope.
    pub tenant_or_org_scope_ref: Option<String>,
    /// Callback/ingress path class.
    pub callback_path: CallbackPathClass,
    /// Stable token for [`Self::callback_path`].
    pub callback_path_token: String,
    /// Object kinds covered by this descriptor with inspect-vs-mutate posture.
    pub object_support: Vec<ObjectSupportEntry>,
    /// Snapshot freshness class.
    pub snapshot_freshness: SnapshotFreshnessClass,
    /// Stable token for [`Self::snapshot_freshness`].
    pub snapshot_freshness_token: String,
    /// Opaque ref to the freshness epoch for this descriptor.
    pub freshness_epoch_ref: Option<String>,
    /// Degraded reason when freshness class is not fresh; export-safe.
    pub degraded_reason: Option<String>,
    /// `true` when the local-core editing floor is preserved regardless of
    /// whether this provider's managed or network capabilities are available.
    pub local_core_continuity_allowed: bool,
    /// Dependency class for this descriptor.
    pub dependency_class: DependencyClass,
    /// Stable token for [`Self::dependency_class`].
    pub dependency_class_token: String,
    /// Mirror/self-host posture note; export-safe.
    pub mirror_self_host_note: Option<String>,
    /// `true` when no raw provider URL, raw token, raw callback body, raw
    /// private key, or raw PII is present on this record.
    pub raw_private_material_excluded: bool,
    /// Plain-language summary safe for UI, support export, and diagnostics.
    pub summary: String,
}

impl ProviderDescriptorRecord {
    /// Construct a provider descriptor record, filling in all token fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        provider_family: ProviderFamilyClass,
        actor_identity: ActorIdentityClass,
        canonical_host_ref: impl Into<String>,
        tenant_or_org_scope_ref: Option<impl Into<String>>,
        callback_path: CallbackPathClass,
        object_support: Vec<ObjectSupportEntry>,
        snapshot_freshness: SnapshotFreshnessClass,
        freshness_epoch_ref: Option<impl Into<String>>,
        degraded_reason: Option<impl Into<String>>,
        local_core_continuity_allowed: bool,
        dependency_class: DependencyClass,
        mirror_self_host_note: Option<impl Into<String>>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: PROVIDER_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_REGISTRY_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_REGISTRY_SHARED_CONTRACT_REF.to_owned(),
            provider_family,
            provider_family_token: provider_family.as_str().to_owned(),
            actor_identity,
            actor_identity_token: actor_identity.as_str().to_owned(),
            canonical_host_ref: canonical_host_ref.into(),
            tenant_or_org_scope_ref: tenant_or_org_scope_ref.map(Into::into),
            callback_path,
            callback_path_token: callback_path.as_str().to_owned(),
            object_support,
            snapshot_freshness,
            snapshot_freshness_token: snapshot_freshness.as_str().to_owned(),
            freshness_epoch_ref: freshness_epoch_ref.map(Into::into),
            degraded_reason: degraded_reason.map(Into::into),
            local_core_continuity_allowed,
            dependency_class,
            dependency_class_token: dependency_class.as_str().to_owned(),
            mirror_self_host_note: mirror_self_host_note.map(Into::into),
            raw_private_material_excluded: true,
            summary: summary.into(),
        }
    }

    /// Returns `true` when this descriptor covers at least one object kind
    /// with an explicit inspect-vs-mutate posture.
    pub fn has_object_support(&self) -> bool {
        !self.object_support.is_empty()
    }

    /// Returns the row key `(provider_family_token, actor_identity_token)`.
    pub fn row_key(&self) -> (String, String) {
        (
            self.provider_family_token.clone(),
            self.actor_identity_token.clone(),
        )
    }
}

// ---------------------------------------------------------------------------
// Provider descriptor snapshot (aggregate of all descriptor records)
// ---------------------------------------------------------------------------

/// Aggregate of all provider descriptor records.
///
/// The snapshot carries one [`ProviderDescriptorRecord`] per (provider_family,
/// actor_identity) pair. The nine required pairs are the three families ×
/// human_account, installation_grant, and delegated_credential. A snapshot
/// missing any required pair causes the registry proof packet to narrow to
/// `Preview`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderDescriptorSnapshot {
    /// All descriptor records in the snapshot.
    pub records: Vec<ProviderDescriptorRecord>,
}

impl ProviderDescriptorSnapshot {
    /// Returns the record for the given (family, actor) pair, if present.
    pub fn record_for_pair(
        &self,
        family: ProviderFamilyClass,
        actor: ActorIdentityClass,
    ) -> Option<&ProviderDescriptorRecord> {
        self.records
            .iter()
            .find(|r| r.provider_family == family && r.actor_identity == actor)
    }

    /// Returns the set of `(family_token, actor_token)` pairs covered by
    /// this snapshot.
    pub fn covered_pairs(&self) -> BTreeSet<(String, String)> {
        self.records.iter().map(|r| r.row_key()).collect()
    }
}

// ---------------------------------------------------------------------------
// Registry row (per-descriptor qualification row)
// ---------------------------------------------------------------------------

/// Qualification row for one (provider_family, actor_identity) descriptor.
///
/// Each row is derived from a single [`ProviderDescriptorRecord`] in the
/// snapshot. The qualification is computed against the six stability conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRegistryRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Provider family token.
    pub provider_family_token: String,
    /// Actor identity token.
    pub actor_identity_token: String,
    /// Callback path token.
    pub callback_path_token: String,
    /// Dependency class token.
    pub dependency_class_token: String,
    /// Snapshot freshness token.
    pub snapshot_freshness_token: String,
    /// `true` when the local-core continuity posture is explicitly declared.
    pub local_core_continuity_declared: bool,
    /// `true` when at least one object kind has an explicit posture.
    pub object_support_declared: bool,
    /// `true` when raw private material is excluded.
    pub raw_private_material_excluded: bool,
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

/// Aggregate banner emitted with the registry page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProviderRegistrySummary {
    /// Total row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// `(family_token, actor_token)` pairs covered by the snapshot.
    pub pairs_covered: Vec<(String, String)>,
    /// Number of rows with explicit local-core continuity declaration.
    pub local_core_continuity_declared_count: usize,
    /// Number of rows with at least one object kind declared.
    pub object_support_declared_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl ProviderRegistrySummary {
    fn from_rows(rows: &[ProviderRegistryRow], snapshot: &ProviderDescriptorSnapshot) -> Self {
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
        let overall = if withdrawn > 0 {
            ProviderRegistryQualificationClass::Withdrawn
        } else if preview > 0 {
            ProviderRegistryQualificationClass::Preview
        } else if beta > 0 {
            ProviderRegistryQualificationClass::Beta
        } else {
            ProviderRegistryQualificationClass::Stable
        };
        let pairs_covered: Vec<(String, String)> = snapshot
            .records
            .iter()
            .map(|r| (r.provider_family_token.clone(), r.actor_identity_token.clone()))
            .collect();
        let local_core_continuity_declared_count =
            rows.iter().filter(|r| r.local_core_continuity_declared).count();
        let object_support_declared_count =
            rows.iter().filter(|r| r.object_support_declared).count();
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            pairs_covered,
            local_core_continuity_declared_count,
            object_support_declared_count,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the registry page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRegistryDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: ProviderRegistryNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (`family:actor` pair token or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl ProviderRegistryDefect {
    fn new(
        narrow_reason: ProviderRegistryNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: PROVIDER_REGISTRY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_REGISTRY_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_REGISTRY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:provider-registry:{}:{}",
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
// Registry page (proof packet)
// ---------------------------------------------------------------------------

/// Stable proof packet for the connected-provider registry hardening lane.
///
/// The packet is the single inspectable record that proves the stable claim
/// for the provider registry across all claimed provider families and actor
/// identity classes. Dashboards, docs, Help/About surfaces, support exports,
/// and diagnostics should ingest this packet rather than cloning
/// subsystem-specific status strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRegistryPage {
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
    /// Aggregate summary derived from all rows.
    pub summary: ProviderRegistrySummary,
    /// Per-(family, actor) stability rows.
    pub rows: Vec<ProviderRegistryRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<ProviderRegistryDefect>,
    /// The descriptor snapshot embedded as evidence.
    pub descriptor_snapshot: ProviderDescriptorSnapshot,
}

impl ProviderRegistryPage {
    /// Build the registry page from a provider descriptor snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        descriptor_snapshot: ProviderDescriptorSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&descriptor_snapshot);
        let rows = derive_registry_rows(&descriptor_snapshot, &defects);
        let summary = ProviderRegistrySummary::from_rows(&rows, &descriptor_snapshot);
        Self {
            record_kind: PROVIDER_REGISTRY_PAGE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_REGISTRY_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_REGISTRY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            descriptor_snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ProviderRegistryQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all nine required (family, actor) pairs are covered.
    pub fn covers_all_required_pairs(&self) -> bool {
        let covered = self.descriptor_snapshot.covered_pairs();
        REQUIRED_DESCRIPTOR_PAIRS.iter().all(|(family, actor)| {
            covered.contains(&(family.as_str().to_owned(), actor.as_str().to_owned()))
        })
    }

    /// Returns `true` when every descriptor explicitly declares local-core
    /// continuity.
    pub fn all_descriptors_declare_local_core_continuity(&self) -> bool {
        self.descriptor_snapshot
            .records
            .iter()
            .all(|r| r.local_core_continuity_allowed)
    }

    /// Returns `true` when every descriptor names a callback path class.
    pub fn all_descriptors_declare_callback_path(&self) -> bool {
        self.descriptor_snapshot
            .records
            .iter()
            .all(|r| !r.callback_path_token.is_empty())
    }

    /// Returns `true` when every descriptor covers at least one object kind.
    pub fn all_descriptors_declare_object_support(&self) -> bool {
        self.descriptor_snapshot
            .records
            .iter()
            .all(|r| r.has_object_support())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the registry page plus a metadata-safe
/// defect roll-up.
///
/// No raw provider URLs, raw tokens, raw callback bodies, or raw private key
/// material may appear in this export. Only closed-vocabulary tokens, opaque
/// refs, counts, and plain-language summary sentences cross the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRegistrySupportExport {
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
    /// The registry page embedded as evidence.
    pub page: ProviderRegistryPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<ProviderRegistryNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl ProviderRegistrySupportExport {
    /// Wrap a registry page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: ProviderRegistryPage,
    ) -> Self {
        let mut reasons: Vec<ProviderRegistryNarrowReasonClass> = Vec::new();
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
            record_kind: PROVIDER_REGISTRY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_REGISTRY_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_REGISTRY_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions (public API)
// ---------------------------------------------------------------------------

/// Re-run the registry audit over the snapshot embedded in a page.
pub fn audit_provider_registry_page(
    page: &ProviderRegistryPage,
) -> Vec<ProviderRegistryDefect> {
    audit_snapshot(&page.descriptor_snapshot)
}

/// Validate a registry page; returns `Ok` when the audit is clean.
pub fn validate_provider_registry_page(
    page: &ProviderRegistryPage,
) -> Result<(), Vec<ProviderRegistryDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &ProviderDescriptorSnapshot) -> Vec<ProviderRegistryDefect> {
    let mut defects: Vec<ProviderRegistryDefect> = Vec::new();

    // Hard guardrail: raw private material exposed — withdraw immediately.
    for record in &snapshot.records {
        if !record.raw_private_material_excluded {
            defects.push(ProviderRegistryDefect::new(
                ProviderRegistryNarrowReasonClass::RawPrivateMaterialExposed,
                format!(
                    "{}:{}",
                    record.provider_family_token, record.actor_identity_token
                ),
                format!(
                    "descriptor '{}:{}' has raw_private_material_excluded: false; packet is withdrawn",
                    record.provider_family_token, record.actor_identity_token
                ),
            ));
            return defects;
        }
    }

    let covered = snapshot.covered_pairs();

    // Coverage check: all nine required (family, actor) pairs must be present.
    for (family, actor) in &REQUIRED_DESCRIPTOR_PAIRS {
        let key = (family.as_str().to_owned(), actor.as_str().to_owned());
        if !covered.contains(&key) {
            defects.push(ProviderRegistryDefect::new(
                ProviderRegistryNarrowReasonClass::RequiredRowMissing,
                format!("{}:{}", family.as_str(), actor.as_str()),
                format!(
                    "required pair '{}:{}' has no descriptor record; packet is narrowed to preview",
                    family.as_str(),
                    actor.as_str()
                ),
            ));
        }
    }

    // Per-descriptor checks.
    for record in &snapshot.records {
        let source = format!(
            "{}:{}",
            record.provider_family_token, record.actor_identity_token
        );

        if !record.local_core_continuity_allowed {
            defects.push(ProviderRegistryDefect::new(
                ProviderRegistryNarrowReasonClass::LocalCoreContinuityUndeclared,
                source.clone(),
                format!(
                    "descriptor '{}' does not declare local-core continuity; \
                     local work may be blocked by managed provider capabilities",
                    source
                ),
            ));
        }

        if record.dependency_class_token.is_empty() {
            defects.push(ProviderRegistryDefect::new(
                ProviderRegistryNarrowReasonClass::DependencyClassUndeclared,
                source.clone(),
                format!(
                    "descriptor '{}' has an empty dependency_class_token; \
                     dependency class must be explicit",
                    source
                ),
            ));
        }

        if record.callback_path_token.is_empty() {
            defects.push(ProviderRegistryDefect::new(
                ProviderRegistryNarrowReasonClass::CallbackPathUndeclared,
                source.clone(),
                format!(
                    "descriptor '{}' has an empty callback_path_token; \
                     callback/ingress path class must be named for sovereignty audits",
                    source
                ),
            ));
        }

        if !record.has_object_support() {
            defects.push(ProviderRegistryDefect::new(
                ProviderRegistryNarrowReasonClass::ObjectSupportUndeclared,
                source.clone(),
                format!(
                    "descriptor '{}' covers no object kinds; at least one object kind \
                     with an explicit inspect-vs-mutate posture is required",
                    source
                ),
            ));
        }
    }

    defects
}

fn derive_registry_rows(
    snapshot: &ProviderDescriptorSnapshot,
    page_defects: &[ProviderRegistryDefect],
) -> Vec<ProviderRegistryRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_preview_reason());

    let overall_narrow_reason = if has_withdrawal {
        ProviderRegistryNarrowReasonClass::RawPrivateMaterialExposed
    } else if has_preview {
        ProviderRegistryNarrowReasonClass::RequiredRowMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        ProviderRegistryNarrowReasonClass::NotNarrowed
    };

    snapshot
        .records
        .iter()
        .map(|record| {
            let source = format!(
                "{}:{}",
                record.provider_family_token, record.actor_identity_token
            );
            let row_narrow =
                find_row_narrow_reason(&source, page_defects, overall_narrow_reason);
            let row_qual = if row_narrow.is_withdrawal_reason() {
                ProviderRegistryQualificationClass::Withdrawn
            } else if row_narrow.is_preview_reason() {
                ProviderRegistryQualificationClass::Preview
            } else if row_narrow != ProviderRegistryNarrowReasonClass::NotNarrowed {
                ProviderRegistryQualificationClass::Beta
            } else {
                ProviderRegistryQualificationClass::Stable
            };
            let summary = build_row_summary(&source, &row_qual, row_narrow);
            ProviderRegistryRow {
                record_kind: PROVIDER_REGISTRY_ROW_RECORD_KIND.to_owned(),
                schema_version: PROVIDER_REGISTRY_SCHEMA_VERSION,
                shared_contract_ref: PROVIDER_REGISTRY_SHARED_CONTRACT_REF.to_owned(),
                provider_family_token: record.provider_family_token.clone(),
                actor_identity_token: record.actor_identity_token.clone(),
                callback_path_token: record.callback_path_token.clone(),
                dependency_class_token: record.dependency_class_token.clone(),
                snapshot_freshness_token: record.snapshot_freshness_token.clone(),
                local_core_continuity_declared: record.local_core_continuity_allowed,
                object_support_declared: record.has_object_support(),
                raw_private_material_excluded: record.raw_private_material_excluded,
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn find_row_narrow_reason(
    source: &str,
    page_defects: &[ProviderRegistryDefect],
    overall_narrow_reason: ProviderRegistryNarrowReasonClass,
) -> ProviderRegistryNarrowReasonClass {
    if let Some(defect) = page_defects.iter().find(|d| d.source == source) {
        return defect.narrow_reason;
    }
    overall_narrow_reason
}

fn build_row_summary(
    source: &str,
    qual: &ProviderRegistryQualificationClass,
    narrow_reason: ProviderRegistryNarrowReasonClass,
) -> String {
    match qual {
        ProviderRegistryQualificationClass::Stable => format!(
            "Descriptor '{}' qualifies stable: all six stability conditions hold, \
             acting identity is explicit, callback path is named, object support is \
             declared, local-core continuity is preserved, and dependency class is explicit.",
            source
        ),
        _ => format!(
            "Descriptor '{}' narrowed to {} ({}): see defect list for details.",
            source,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet consumed by the headless example, the
/// integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: all nine required (family, actor)
/// pairs are covered, no raw private material is exposed, every descriptor
/// declares local-core continuity, all dependency classes are explicit, all
/// callback path classes are named, and all descriptors cover at least one
/// object kind with an explicit posture.
pub fn seeded_provider_registry_page() -> ProviderRegistryPage {
    ProviderRegistryPage::new(
        "remote:provider_registry:default",
        "Connected-provider registry hardening — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_provider_descriptor_snapshot(),
    )
}

/// Build the seeded provider descriptor snapshot used by the seeded page.
///
/// Each of the nine required (family, actor) pairs is represented with a
/// fully-typed, clean record that passes all six stability conditions.
pub fn seeded_provider_descriptor_snapshot() -> ProviderDescriptorSnapshot {
    ProviderDescriptorSnapshot {
        records: vec![
            // ---- code_host × human_account ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::CodeHost,
                ActorIdentityClass::HumanAccount,
                "host:code-host:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![
                    ObjectSupportEntry::new(
                        ObjectKindClass::PullRequest,
                        MutationPostureClass::MutateAllowed,
                        vec![PublishModeClass::PublishNow, PublishModeClass::LocalDraft],
                        false,
                        false,
                    ),
                    ObjectSupportEntry::new(
                        ObjectKindClass::Branch,
                        MutationPostureClass::MutateAllowed,
                        vec![PublishModeClass::PublishNow],
                        false,
                        false,
                    ),
                ],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:code-host:human:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "Code host (human account): public SaaS callback path; \
                 pull requests and branches are mutable; local editing continues \
                 without provider connectivity.",
            ),
            // ---- code_host × installation_grant ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::CodeHost,
                ActorIdentityClass::InstallationGrant,
                "host:code-host:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![
                    ObjectSupportEntry::new(
                        ObjectKindClass::PullRequest,
                        MutationPostureClass::MutateAllowed,
                        vec![
                            PublishModeClass::PublishNow,
                            PublishModeClass::PublishLaterQueue,
                        ],
                        false,
                        true,
                    ),
                    ObjectSupportEntry::new(
                        ObjectKindClass::Branch,
                        MutationPostureClass::InspectOnly,
                        vec![PublishModeClass::InspectOnly],
                        false,
                        false,
                    ),
                ],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:code-host:install:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "Code host (installation grant): public SaaS callback path; \
                 pull requests mutable with publish-later support; branch inspect-only; \
                 local editing continues without provider connectivity.",
            ),
            // ---- code_host × delegated_credential ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::CodeHost,
                ActorIdentityClass::DelegatedCredential,
                "host:code-host:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![
                    ObjectSupportEntry::new(
                        ObjectKindClass::PullRequest,
                        MutationPostureClass::InspectOnly,
                        vec![PublishModeClass::InspectOnly],
                        false,
                        false,
                    ),
                ],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:code-host:delegated:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "Code host (delegated credential): public SaaS callback path; \
                 inspect-only access to pull requests; local editing continues \
                 without provider connectivity.",
            ),
            // ---- issue_tracker × human_account ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::IssueTracker,
                ActorIdentityClass::HumanAccount,
                "host:issue-tracker:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![ObjectSupportEntry::new(
                    ObjectKindClass::IssueOrWorkItem,
                    MutationPostureClass::MutateAllowed,
                    vec![PublishModeClass::PublishNow, PublishModeClass::LocalDraft],
                    false,
                    false,
                )],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:issue-tracker:human:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "Issue tracker (human account): public SaaS callback path; \
                 issues and work items are mutable; local editing continues \
                 without provider connectivity.",
            ),
            // ---- issue_tracker × installation_grant ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::IssueTracker,
                ActorIdentityClass::InstallationGrant,
                "host:issue-tracker:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![ObjectSupportEntry::new(
                    ObjectKindClass::IssueOrWorkItem,
                    MutationPostureClass::MutateAllowed,
                    vec![
                        PublishModeClass::PublishNow,
                        PublishModeClass::PublishLaterQueue,
                    ],
                    false,
                    true,
                )],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:issue-tracker:install:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "Issue tracker (installation grant): public SaaS callback path; \
                 issues and work items mutable with publish-later support; \
                 local editing continues without provider connectivity.",
            ),
            // ---- issue_tracker × delegated_credential ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::IssueTracker,
                ActorIdentityClass::DelegatedCredential,
                "host:issue-tracker:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![ObjectSupportEntry::new(
                    ObjectKindClass::IssueOrWorkItem,
                    MutationPostureClass::InspectOnly,
                    vec![PublishModeClass::InspectOnly],
                    false,
                    false,
                )],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:issue-tracker:delegated:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "Issue tracker (delegated credential): public SaaS callback path; \
                 inspect-only access; local editing continues without provider connectivity.",
            ),
            // ---- ci_checks × human_account ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::CiChecks,
                ActorIdentityClass::HumanAccount,
                "host:ci-checks:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![
                    ObjectSupportEntry::new(
                        ObjectKindClass::CheckRun,
                        MutationPostureClass::MutateAllowed,
                        vec![PublishModeClass::OpenInProvider, PublishModeClass::InspectOnly],
                        true,
                        false,
                    ),
                    ObjectSupportEntry::new(
                        ObjectKindClass::PipelineRun,
                        MutationPostureClass::MutateAllowed,
                        vec![PublishModeClass::OpenInProvider],
                        true,
                        false,
                    ),
                ],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:ci-checks:human:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "CI/checks (human account): public SaaS callback path; check runs and \
                 pipeline runs mutated via browser handoff; local editing continues \
                 without provider connectivity.",
            ),
            // ---- ci_checks × installation_grant ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::CiChecks,
                ActorIdentityClass::InstallationGrant,
                "host:ci-checks:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![
                    ObjectSupportEntry::new(
                        ObjectKindClass::CheckRun,
                        MutationPostureClass::MutateAllowed,
                        vec![PublishModeClass::PublishNow, PublishModeClass::PublishLaterQueue],
                        false,
                        true,
                    ),
                    ObjectSupportEntry::new(
                        ObjectKindClass::PipelineRun,
                        MutationPostureClass::MutateAllowed,
                        vec![
                            PublishModeClass::PublishNow,
                            PublishModeClass::PublishLaterQueue,
                        ],
                        false,
                        true,
                    ),
                ],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:ci-checks:install:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "CI/checks (installation grant): public SaaS callback path; check runs \
                 and pipeline runs mutable with publish-later support; local editing \
                 continues without provider connectivity.",
            ),
            // ---- ci_checks × delegated_credential ----
            ProviderDescriptorRecord::new(
                ProviderFamilyClass::CiChecks,
                ActorIdentityClass::DelegatedCredential,
                "host:ci-checks:saas:v1",
                Some("scope:org:default"),
                CallbackPathClass::PublicSaas,
                vec![
                    ObjectSupportEntry::new(
                        ObjectKindClass::CheckRun,
                        MutationPostureClass::InspectOnly,
                        vec![PublishModeClass::InspectOnly],
                        false,
                        false,
                    ),
                    ObjectSupportEntry::new(
                        ObjectKindClass::PipelineRun,
                        MutationPostureClass::InspectOnly,
                        vec![PublishModeClass::InspectOnly],
                        false,
                        false,
                    ),
                ],
                SnapshotFreshnessClass::Fresh,
                Some("epoch:ci-checks:delegated:2026-06-01"),
                None::<String>,
                true,
                DependencyClass::Network,
                None::<String>,
                "CI/checks (delegated credential): public SaaS callback path; \
                 inspect-only access to check runs and pipeline runs; local editing \
                 continues without provider connectivity.",
            ),
        ],
    }
}
