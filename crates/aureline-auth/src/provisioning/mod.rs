//! Provisioning hooks, policy-bundle history, and admin-audit export baseline.
//!
//! This module owns the beta projection that turns SCIM and signed-file
//! provisioning hooks, policy-bundle history transitions, and entitlement
//! changes into one inspectable admin-audit export. It gives admin, support,
//! shell, headless, and reviewer surfaces one record per event that names:
//!
//! - the provisioning source (SCIM managed endpoint, SCIM self-hosted
//!   endpoint, signed-file import, signed mirror import, air-gapped signed
//!   import, local advisory file, or runtime preload origin) and its
//!   provenance bundle (signer, signed-at, fetched-at, valid-until, signature
//!   blob ref);
//! - the typed lifecycle posture (active / suspended / disabled / deleted) and
//!   freshness class (live / cached / stale / expired / missing);
//! - the policy-bundle history transition (applied / staged / replaced /
//!   rolled-back / revoked) tying a pack id and version to its predecessor; and
//! - the entitlement change (granted, revoked, scope narrowed, scope widened,
//!   seat added, seat removed, quota changed, expiry shortened, expiry
//!   extended) with before and after authority tokens.
//!
//! Surfaces (admin console, support export, settings center, shell trust
//! center, enterprise docs, fixture replay) consume the seeded page from
//! [`seeded_admin_audit_export_beta_page`] rather than re-deriving local
//! "is_provisioned" or "history_recorded" checks. The export wrapper preserves
//! provenance and signature fields verbatim and excludes raw private material
//! so enterprise pilots can replay one auditable model across connected,
//! mirror-only, offline, and enterprise-managed beta profiles.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::trust::CapabilityAuthorityClass;

/// Beta schema version exported with every provisioning and admin-audit record.
pub const ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every provisioning and admin-audit
/// record.
pub const ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF: &str =
    "security:admin_audit_export_beta:v1";

/// Source matrix ref consumed by this beta projection.
pub const ADMIN_AUDIT_EXPORT_BETA_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/admin_audit_exports/admin_audit_matrix.yaml";

/// Stable record kind for [`AdminAuditExportBetaPage`] payloads.
pub const ADMIN_AUDIT_EXPORT_BETA_PAGE_RECORD_KIND: &str =
    "security_admin_audit_export_beta_page_record";

/// Stable record kind for [`ProvisioningEvent`] payloads.
pub const PROVISIONING_EVENT_RECORD_KIND: &str =
    "security_admin_audit_export_beta_provisioning_event_record";

/// Stable record kind for [`PolicyBundleHistoryEvent`] payloads.
pub const POLICY_BUNDLE_HISTORY_EVENT_RECORD_KIND: &str =
    "security_admin_audit_export_beta_policy_bundle_history_event_record";

/// Stable record kind for [`EntitlementChangeEvent`] payloads.
pub const ENTITLEMENT_CHANGE_EVENT_RECORD_KIND: &str =
    "security_admin_audit_export_beta_entitlement_change_event_record";

/// Stable record kind for [`AdminAuditExportBetaDefect`] payloads.
pub const ADMIN_AUDIT_EXPORT_BETA_DEFECT_RECORD_KIND: &str =
    "security_admin_audit_export_beta_defect_record";

/// Stable record kind for [`AdminAuditExportBetaSummary`] payloads.
pub const ADMIN_AUDIT_EXPORT_BETA_SUMMARY_RECORD_KIND: &str =
    "security_admin_audit_export_beta_summary_record";

/// Stable record kind for [`AdminAuditExportBetaSupportExport`] payloads.
pub const ADMIN_AUDIT_EXPORT_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_admin_audit_export_beta_support_export_record";

/// Profile under which a provisioning event, history transition, or
/// entitlement change is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminAuditExportBetaProfileClass {
    /// Connected beta profile with live provisioning and entitlement paths.
    Connected,
    /// Mirror-only profile served from a declared signed mirror.
    MirrorOnly,
    /// Offline profile served from a last-known-good or air-gapped snapshot.
    Offline,
    /// Enterprise-managed profile applying signed managed narrowing.
    EnterpriseManaged,
}

impl AdminAuditExportBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Origin class for a provisioning hook event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningSourceClass {
    /// Vendor-managed SCIM endpoint pulled over a managed transport.
    ScimManagedEndpoint,
    /// Customer self-hosted SCIM endpoint pulled over a customer transport.
    ScimSelfHostedEndpoint,
    /// Signed file imported manually by an admin.
    SignedFileImport,
    /// Signed mirror import served from a declared mirror.
    SignedMirrorImport,
    /// Air-gapped signed transfer of a provisioning snapshot.
    AirGappedSignedImport,
    /// Local advisory file; never sufficient for managed authority.
    LocalAdvisoryFile,
    /// Build preload or first-run seed.
    RuntimePreloadOrigin,
}

impl ProvisioningSourceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScimManagedEndpoint => "scim_managed_endpoint",
            Self::ScimSelfHostedEndpoint => "scim_self_hosted_endpoint",
            Self::SignedFileImport => "signed_file_import",
            Self::SignedMirrorImport => "signed_mirror_import",
            Self::AirGappedSignedImport => "air_gapped_signed_import",
            Self::LocalAdvisoryFile => "local_advisory_file",
            Self::RuntimePreloadOrigin => "runtime_preload_origin",
        }
    }

    /// True when this origin requires a verified signature before authority
    /// widens beyond local advisory.
    pub const fn requires_signature(self) -> bool {
        !matches!(self, Self::LocalAdvisoryFile | Self::RuntimePreloadOrigin)
    }

    /// True when this origin can carry managed authority for enterprise
    /// pilots.
    pub const fn is_managed_authority(self) -> bool {
        matches!(
            self,
            Self::ScimManagedEndpoint
                | Self::ScimSelfHostedEndpoint
                | Self::SignedFileImport
                | Self::SignedMirrorImport
                | Self::AirGappedSignedImport
        )
    }
}

/// Event class for a provisioning hook event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningEventClass {
    /// A user identity was created.
    UserCreated,
    /// A user identity was updated.
    UserUpdated,
    /// A user identity was suspended.
    UserSuspended,
    /// A user identity was reactivated.
    UserReactivated,
    /// A user identity was deleted or hard-deprovisioned.
    UserDeleted,
    /// A group was created.
    GroupCreated,
    /// A group was updated.
    GroupUpdated,
    /// A group was deleted.
    GroupDeleted,
    /// A membership edge was added.
    MembershipAdded,
    /// A membership edge was removed.
    MembershipRemoved,
}

impl ProvisioningEventClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserCreated => "user_created",
            Self::UserUpdated => "user_updated",
            Self::UserSuspended => "user_suspended",
            Self::UserReactivated => "user_reactivated",
            Self::UserDeleted => "user_deleted",
            Self::GroupCreated => "group_created",
            Self::GroupUpdated => "group_updated",
            Self::GroupDeleted => "group_deleted",
            Self::MembershipAdded => "membership_added",
            Self::MembershipRemoved => "membership_removed",
        }
    }

    /// Subject kind implied by [`Self`].
    pub const fn subject_kind(self) -> ProvisioningSubjectKindClass {
        match self {
            Self::UserCreated
            | Self::UserUpdated
            | Self::UserSuspended
            | Self::UserReactivated
            | Self::UserDeleted => ProvisioningSubjectKindClass::User,
            Self::GroupCreated | Self::GroupUpdated | Self::GroupDeleted => {
                ProvisioningSubjectKindClass::Group
            }
            Self::MembershipAdded | Self::MembershipRemoved => {
                ProvisioningSubjectKindClass::Membership
            }
        }
    }
}

/// Subject kind for a provisioning event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningSubjectKindClass {
    /// User identity.
    User,
    /// Group identity.
    Group,
    /// Membership edge tying a user to a group.
    Membership,
}

impl ProvisioningSubjectKindClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Group => "group",
            Self::Membership => "membership",
        }
    }
}

/// Lifecycle posture observed on the subject after the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningLifecycleStateClass {
    /// Subject is active.
    Active,
    /// Subject is suspended; authority is held closed.
    Suspended,
    /// Subject is disabled; reactivation requires admin action.
    Disabled,
    /// Subject is deleted or hard-deprovisioned.
    Deleted,
}

impl ProvisioningLifecycleStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Disabled => "disabled",
            Self::Deleted => "deleted",
        }
    }

    /// True when the lifecycle state holds managed authority closed.
    pub const fn fails_closed(self) -> bool {
        !matches!(self, Self::Active)
    }
}

/// Freshness posture of the provisioning event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningFreshnessClass {
    /// Pulled from the source within the live freshness window.
    Live,
    /// Pulled from the source within an acceptable cache window.
    Cached,
    /// Pulled but past the acceptable cache window; managed authority is
    /// narrowed.
    Stale,
    /// The bundle's `valid_until` has passed; managed authority is closed.
    Expired,
    /// The bundle was never received or has been revoked.
    Missing,
}

impl ProvisioningFreshnessClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }

    /// True when the freshness posture forbids widening managed authority.
    pub const fn fails_closed(self) -> bool {
        matches!(self, Self::Stale | Self::Expired | Self::Missing)
    }
}

/// Provenance bundle attached to a provisioning event or history transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningProvenance {
    /// Authoritative origin URL or filesystem reference.
    pub source_ref: String,
    /// Signer identity recorded on the signed bundle.
    pub signer_id: String,
    /// Signed-at timestamp on the bundle.
    pub signed_at: String,
    /// Fetched-at timestamp on this profile.
    pub fetched_at: String,
    /// Valid-until timestamp on the bundle.
    pub valid_until: String,
    /// Mirror or import path label, if any.
    pub transport_label: String,
    /// Stable ref to the preserved signature blob in the artifact store.
    pub signature_blob_ref: String,
}

/// One provisioning hook event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub event_id: String,
    /// Source class.
    pub source_class: ProvisioningSourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_token: String,
    /// Event class.
    pub event_class: ProvisioningEventClass,
    /// Stable token for [`Self::event_class`].
    pub event_token: String,
    /// Subject kind.
    pub subject_kind: ProvisioningSubjectKindClass,
    /// Stable token for [`Self::subject_kind`].
    pub subject_kind_token: String,
    /// Stable subject id (synthetic; never carries raw private material).
    pub subject_id: String,
    /// Lifecycle posture observed after the event.
    pub lifecycle_state: ProvisioningLifecycleStateClass,
    /// Stable token for [`Self::lifecycle_state`].
    pub lifecycle_state_token: String,
    /// Freshness posture of the event.
    pub freshness: ProvisioningFreshnessClass,
    /// Stable token for [`Self::freshness`].
    pub freshness_token: String,
    /// Applied-at timestamp.
    pub applied_at: String,
    /// Profile token under which this event was inspected.
    pub profile_token: String,
    /// Reviewable provenance.
    pub provenance: ProvisioningProvenance,
    /// True when no undeclared public endpoint fallback is permitted on this
    /// hook.
    pub no_public_endpoint_fallback: bool,
    /// True when raw private/secret material is excluded from the record.
    pub raw_private_material_excluded: bool,
}

/// History transition class for a policy bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyBundleTransitionClass {
    /// Bundle was applied for the first time.
    Applied,
    /// Bundle was staged and is awaiting admin review or rollout.
    Staged,
    /// Bundle replaced a predecessor on this profile.
    ReplacedBySuccessor,
    /// Bundle was rolled back to its predecessor.
    RolledBack,
    /// Bundle was revoked.
    Revoked,
}

impl PolicyBundleTransitionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::Staged => "staged",
            Self::ReplacedBySuccessor => "replaced_by_successor",
            Self::RolledBack => "rolled_back",
            Self::Revoked => "revoked",
        }
    }

    /// True when the transition expects a predecessor reference.
    pub const fn requires_predecessor(self) -> bool {
        matches!(self, Self::ReplacedBySuccessor | Self::RolledBack)
    }
}

/// One policy-bundle history event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyBundleHistoryEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub event_id: String,
    /// Pack id this event covers.
    pub pack_id: String,
    /// Pack version string.
    pub pack_version: String,
    /// Predecessor pack id, or empty when none.
    pub replaces_pack_id: String,
    /// Transition class.
    pub transition: PolicyBundleTransitionClass,
    /// Stable token for [`Self::transition`].
    pub transition_token: String,
    /// Source token mirroring the policy-pack source vocabulary.
    pub source_token: String,
    /// Signature-state token mirroring the policy-pack signature vocabulary.
    pub signature_state_token: String,
    /// Apply-state token mirroring the policy-pack apply vocabulary.
    pub apply_state_token: String,
    /// Signer identity recorded on the bundle.
    pub signer_id: String,
    /// Signed-at timestamp on the bundle.
    pub signed_at: String,
    /// Applied-at timestamp on this profile.
    pub applied_at: String,
    /// Profile token under which this transition was inspected.
    pub profile_token: String,
    /// Authoritative origin ref preserved verbatim from the upstream pack.
    pub provenance_origin_ref: String,
    /// Preserved signature blob ref.
    pub provenance_signature_blob_ref: String,
}

/// Change class for an entitlement event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementChangeClass {
    /// Entitlement was granted.
    Granted,
    /// Entitlement was revoked.
    Revoked,
    /// Entitlement scope narrowed.
    ScopeNarrowed,
    /// Entitlement scope widened.
    ScopeWidened,
    /// Seat count increased.
    SeatAdded,
    /// Seat count decreased.
    SeatRemoved,
    /// Quota or limit changed.
    QuotaChanged,
    /// Expiry shortened.
    ExpiryShortened,
    /// Expiry extended.
    ExpiryExtended,
}

impl EntitlementChangeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Granted => "granted",
            Self::Revoked => "revoked",
            Self::ScopeNarrowed => "scope_narrowed",
            Self::ScopeWidened => "scope_widened",
            Self::SeatAdded => "seat_added",
            Self::SeatRemoved => "seat_removed",
            Self::QuotaChanged => "quota_changed",
            Self::ExpiryShortened => "expiry_shortened",
            Self::ExpiryExtended => "expiry_extended",
        }
    }

    /// True when the change narrows authority and a managed source must be
    /// signature-verified.
    pub const fn narrows_authority(self) -> bool {
        matches!(
            self,
            Self::Revoked
                | Self::ScopeNarrowed
                | Self::SeatRemoved
                | Self::ExpiryShortened
                | Self::QuotaChanged
        )
    }
}

/// One entitlement-change event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementChangeEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub event_id: String,
    /// Change class.
    pub change_class: EntitlementChangeClass,
    /// Stable token for [`Self::change_class`].
    pub change_token: String,
    /// Stable entitlement id (synthetic; never carries raw private material).
    pub entitlement_id: String,
    /// Stable subject id (synthetic; never carries raw private material).
    pub subject_id: String,
    /// Before-state label.
    pub before_state: String,
    /// After-state label.
    pub after_state: String,
    /// Authority token in effect before the change.
    pub before_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::before_authority`].
    pub before_authority_token: String,
    /// Authority token in effect after the change.
    pub after_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::after_authority`].
    pub after_authority_token: String,
    /// Applied-at timestamp.
    pub applied_at: String,
    /// Source token mirroring the policy-pack source vocabulary.
    pub source_token: String,
    /// Signer identity recorded on the underlying signed bundle.
    pub signer_id: String,
    /// Profile token under which this change was inspected.
    pub profile_token: String,
    /// Plain-language explanation rendered by inspectors and exports.
    pub explanation: String,
}

/// Typed validator defect kind for the admin-audit export beta page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminAuditExportBetaDefectKind {
    /// A record's source token does not match its source class.
    SourceTokenDrift,
    /// A provisioning event's event token does not match its event class.
    EventClassTokenDrift,
    /// A provisioning event's subject kind token does not match its subject kind.
    SubjectKindTokenDrift,
    /// A provisioning event's lifecycle-state token does not match its class.
    LifecycleStateTokenDrift,
    /// A provisioning event's freshness token does not match its class.
    FreshnessTokenDrift,
    /// A history event's transition token does not match its class.
    BundleTransitionTokenDrift,
    /// An entitlement event's change token does not match its class.
    EntitlementChangeTokenDrift,
    /// An entitlement event's authority token does not match its class.
    EntitlementAuthorityTokenDrift,
    /// A provisioning event from a managed source lacks a verified signature
    /// blob.
    ManagedSourceMissingSignature,
    /// A provisioning event's subject kind does not match its event class.
    SubjectKindMismatch,
    /// A history transition that requires a predecessor lacks one.
    HistoryTransitionMissingPredecessor,
    /// A history event from a managed source lacks a verified signature.
    HistoryManagedSourceMissingSignature,
    /// An entitlement narrowing event was applied without a managed-authority
    /// source.
    EntitlementNarrowingWithoutManagedSource,
    /// A record permits an undeclared public endpoint fallback.
    HiddenPublicEndpointFallback,
    /// A record would expose raw private or secret material.
    RawPrivateMaterialExposed,
    /// A required beta profile is missing from the page coverage.
    ProfileCoverageMissing,
}

impl AdminAuditExportBetaDefectKind {
    /// Stable token recorded on defect rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceTokenDrift => "source_token_drift",
            Self::EventClassTokenDrift => "event_class_token_drift",
            Self::SubjectKindTokenDrift => "subject_kind_token_drift",
            Self::LifecycleStateTokenDrift => "lifecycle_state_token_drift",
            Self::FreshnessTokenDrift => "freshness_token_drift",
            Self::BundleTransitionTokenDrift => "bundle_transition_token_drift",
            Self::EntitlementChangeTokenDrift => "entitlement_change_token_drift",
            Self::EntitlementAuthorityTokenDrift => "entitlement_authority_token_drift",
            Self::ManagedSourceMissingSignature => "managed_source_missing_signature",
            Self::SubjectKindMismatch => "subject_kind_mismatch",
            Self::HistoryTransitionMissingPredecessor => {
                "history_transition_missing_predecessor"
            }
            Self::HistoryManagedSourceMissingSignature => {
                "history_managed_source_missing_signature"
            }
            Self::EntitlementNarrowingWithoutManagedSource => {
                "entitlement_narrowing_without_managed_source"
            }
            Self::HiddenPublicEndpointFallback => "hidden_public_endpoint_fallback",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
        }
    }
}

/// Typed validation defect for the admin-audit export beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAuditExportBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: AdminAuditExportBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (event id, history id, change id, or "page").
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl AdminAuditExportBetaDefect {
    fn new(
        defect_kind: AdminAuditExportBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: ADMIN_AUDIT_EXPORT_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION,
            shared_contract_ref: ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the admin-audit export beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAuditExportBetaSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Stable record kind for the summary itself.
    pub record_kind: String,
    /// Number of provisioning events.
    pub provisioning_event_count: usize,
    /// Number of policy-bundle history events.
    pub policy_bundle_event_count: usize,
    /// Number of entitlement-change events.
    pub entitlement_change_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Provisioning source tokens present across the page.
    pub provisioning_sources_present: Vec<String>,
    /// Provisioning event-class counts by token.
    pub provisioning_event_counts_by_class: BTreeMap<String, usize>,
    /// Lifecycle-state tokens present across the page.
    pub lifecycle_states_present: Vec<String>,
    /// Freshness tokens present across the page.
    pub freshness_states_present: Vec<String>,
    /// Policy-bundle transition tokens present across the page.
    pub policy_bundle_transitions_present: Vec<String>,
    /// Entitlement-change counts by token.
    pub entitlement_changes_by_class: BTreeMap<String, usize>,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl AdminAuditExportBetaSummary {
    /// Builds a summary over provisioning events, history events, entitlement
    /// changes, and defects.
    pub fn from_records(
        provisioning_events: &[ProvisioningEvent],
        policy_bundle_history: &[PolicyBundleHistoryEvent],
        entitlement_changes: &[EntitlementChangeEvent],
        defects: &[AdminAuditExportBetaDefect],
    ) -> Self {
        let mut profiles_present: BTreeSet<String> = BTreeSet::new();
        for event in provisioning_events {
            profiles_present.insert(event.profile_token.clone());
        }
        for event in policy_bundle_history {
            profiles_present.insert(event.profile_token.clone());
        }
        for event in entitlement_changes {
            profiles_present.insert(event.profile_token.clone());
        }

        let provisioning_sources_present: BTreeSet<String> = provisioning_events
            .iter()
            .map(|event| event.source_token.clone())
            .collect();
        let lifecycle_states_present: BTreeSet<String> = provisioning_events
            .iter()
            .map(|event| event.lifecycle_state_token.clone())
            .collect();
        let freshness_states_present: BTreeSet<String> = provisioning_events
            .iter()
            .map(|event| event.freshness_token.clone())
            .collect();
        let policy_bundle_transitions_present: BTreeSet<String> = policy_bundle_history
            .iter()
            .map(|event| event.transition_token.clone())
            .collect();

        let mut provisioning_event_counts_by_class: BTreeMap<String, usize> = BTreeMap::new();
        for event in provisioning_events {
            *provisioning_event_counts_by_class
                .entry(event.event_token.clone())
                .or_insert(0) += 1;
        }
        let mut entitlement_changes_by_class: BTreeMap<String, usize> = BTreeMap::new();
        for event in entitlement_changes {
            *entitlement_changes_by_class
                .entry(event.change_token.clone())
                .or_insert(0) += 1;
        }

        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: ADMIN_AUDIT_EXPORT_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: ADMIN_AUDIT_EXPORT_BETA_SUMMARY_RECORD_KIND.to_owned(),
            provisioning_event_count: provisioning_events.len(),
            policy_bundle_event_count: policy_bundle_history.len(),
            entitlement_change_count: entitlement_changes.len(),
            profiles_present: profiles_present.into_iter().collect(),
            provisioning_sources_present: provisioning_sources_present.into_iter().collect(),
            provisioning_event_counts_by_class,
            lifecycle_states_present: lifecycle_states_present.into_iter().collect(),
            freshness_states_present: freshness_states_present.into_iter().collect(),
            policy_bundle_transitions_present: policy_bundle_transitions_present
                .into_iter()
                .collect(),
            entitlement_changes_by_class,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by admin, support, shell, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAuditExportBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Provisioning hook events.
    pub provisioning_events: Vec<ProvisioningEvent>,
    /// Policy-bundle history transitions.
    pub policy_bundle_history: Vec<PolicyBundleHistoryEvent>,
    /// Entitlement-change events.
    pub entitlement_changes: Vec<EntitlementChangeEvent>,
    /// Typed validation defects.
    pub defects: Vec<AdminAuditExportBetaDefect>,
    /// Aggregate summary.
    pub summary: AdminAuditExportBetaSummary,
}

/// Support-export wrapper for the admin-audit export beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAuditExportBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: AdminAuditExportBetaPage,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private/secret material is excluded.
    pub raw_private_material_excluded: bool,
}

impl AdminAuditExportBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: AdminAuditExportBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: ADMIN_AUDIT_EXPORT_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION,
            shared_contract_ref: ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_private_material_excluded: true,
        }
    }
}

/// Validates the admin-audit export page and returns typed defects on failure.
pub fn validate_admin_audit_export_beta_page(
    page: &AdminAuditExportBetaPage,
) -> Result<(), Vec<AdminAuditExportBetaDefect>> {
    let defects = audit_admin_audit_export_beta_page(
        &page.provisioning_events,
        &page.policy_bundle_history,
        &page.entitlement_changes,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for an admin-audit export beta page.
pub fn audit_admin_audit_export_beta_page(
    provisioning_events: &[ProvisioningEvent],
    policy_bundle_history: &[PolicyBundleHistoryEvent],
    entitlement_changes: &[EntitlementChangeEvent],
) -> Vec<AdminAuditExportBetaDefect> {
    let mut defects = Vec::new();

    for event in provisioning_events {
        if event.source_token != event.source_class.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::SourceTokenDrift,
                event.event_id.clone(),
                "source_token",
                "source_token must match source_class",
            ));
        }
        if event.event_token != event.event_class.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::EventClassTokenDrift,
                event.event_id.clone(),
                "event_token",
                "event_token must match event_class",
            ));
        }
        if event.subject_kind_token != event.subject_kind.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::SubjectKindTokenDrift,
                event.event_id.clone(),
                "subject_kind_token",
                "subject_kind_token must match subject_kind",
            ));
        }
        if event.subject_kind != event.event_class.subject_kind() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::SubjectKindMismatch,
                event.event_id.clone(),
                "subject_kind",
                "subject_kind must match event_class",
            ));
        }
        if event.lifecycle_state_token != event.lifecycle_state.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::LifecycleStateTokenDrift,
                event.event_id.clone(),
                "lifecycle_state_token",
                "lifecycle_state_token must match lifecycle_state",
            ));
        }
        if event.freshness_token != event.freshness.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::FreshnessTokenDrift,
                event.event_id.clone(),
                "freshness_token",
                "freshness_token must match freshness",
            ));
        }
        if !event.no_public_endpoint_fallback {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::HiddenPublicEndpointFallback,
                event.event_id.clone(),
                "no_public_endpoint_fallback",
                "provisioning event permits undeclared public endpoint fallback",
            ));
        }
        if !event.raw_private_material_excluded {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::RawPrivateMaterialExposed,
                event.event_id.clone(),
                "raw_private_material_excluded",
                "provisioning event records must be export-safe metadata",
            ));
        }
        if event.source_class.is_managed_authority()
            && event.provenance.signature_blob_ref.is_empty()
        {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::ManagedSourceMissingSignature,
                event.event_id.clone(),
                "provenance.signature_blob_ref",
                "managed-authority provisioning source must preserve signature blob ref",
            ));
        }
    }

    let pack_ids: BTreeSet<&str> = policy_bundle_history
        .iter()
        .map(|event| event.pack_id.as_str())
        .collect();

    for event in policy_bundle_history {
        if event.transition_token != event.transition.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::BundleTransitionTokenDrift,
                event.event_id.clone(),
                "transition_token",
                "transition_token must match transition",
            ));
        }
        if event.transition.requires_predecessor() && event.replaces_pack_id.is_empty() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::HistoryTransitionMissingPredecessor,
                event.event_id.clone(),
                "replaces_pack_id",
                "history transition requires a predecessor pack id",
            ));
        }
        if event.transition.requires_predecessor()
            && !event.replaces_pack_id.is_empty()
            && !pack_ids.contains(event.replaces_pack_id.as_str())
        {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::HistoryTransitionMissingPredecessor,
                event.event_id.clone(),
                "replaces_pack_id",
                "predecessor pack id is not present in the history",
            ));
        }
        let managed = matches!(
            event.source_token.as_str(),
            "vendor_managed_origin"
                | "customer_self_hosted_origin"
                | "signed_mirror_origin"
                | "manual_signed_file_import"
                | "air_gapped_signed_transfer"
        );
        let verified = matches!(
            event.signature_state_token.as_str(),
            "verified_live"
                | "verified_mirror"
                | "verified_manual_import"
                | "verified_air_gapped"
                | "not_required_local_origin"
        );
        if managed && !verified {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::HistoryManagedSourceMissingSignature,
                event.event_id.clone(),
                "signature_state_token",
                "managed-source history transition must have a verified signature state",
            ));
        }
        if event.provenance_signature_blob_ref.is_empty() && managed {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::HistoryManagedSourceMissingSignature,
                event.event_id.clone(),
                "provenance_signature_blob_ref",
                "managed-source history transition must preserve signature blob ref",
            ));
        }
    }

    for event in entitlement_changes {
        if event.change_token != event.change_class.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::EntitlementChangeTokenDrift,
                event.event_id.clone(),
                "change_token",
                "change_token must match change_class",
            ));
        }
        if event.before_authority_token != event.before_authority.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::EntitlementAuthorityTokenDrift,
                event.event_id.clone(),
                "before_authority_token",
                "before_authority_token must match before_authority",
            ));
        }
        if event.after_authority_token != event.after_authority.as_str() {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::EntitlementAuthorityTokenDrift,
                event.event_id.clone(),
                "after_authority_token",
                "after_authority_token must match after_authority",
            ));
        }
        let managed = matches!(
            event.source_token.as_str(),
            "vendor_managed_origin"
                | "customer_self_hosted_origin"
                | "signed_mirror_origin"
                | "manual_signed_file_import"
                | "air_gapped_signed_transfer"
        );
        if event.change_class.narrows_authority() && !managed {
            defects.push(AdminAuditExportBetaDefect::new(
                AdminAuditExportBetaDefectKind::EntitlementNarrowingWithoutManagedSource,
                event.event_id.clone(),
                "source_token",
                "entitlement narrowing requires a managed-authority source",
            ));
        }
    }

    let required_profiles: BTreeSet<&str> = AdminAuditExportBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    let observed_profiles: BTreeSet<&str> = provisioning_events
        .iter()
        .map(|event| event.profile_token.as_str())
        .chain(
            policy_bundle_history
                .iter()
                .map(|event| event.profile_token.as_str()),
        )
        .chain(
            entitlement_changes
                .iter()
                .map(|event| event.profile_token.as_str()),
        )
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(AdminAuditExportBetaDefect::new(
            AdminAuditExportBetaDefectKind::ProfileCoverageMissing,
            "page",
            "profiles",
            format!("missing {} profile coverage", missing),
        ));
    }

    defects
}

/// Builds the seeded admin-audit export beta page covering connected, mirror,
/// offline, and enterprise-managed profiles with provisioning events,
/// policy-bundle history, and entitlement changes.
pub fn seeded_admin_audit_export_beta_page() -> AdminAuditExportBetaPage {
    let provisioning_events = seed_provisioning_events();
    let policy_bundle_history = seed_policy_bundle_history();
    let entitlement_changes = seed_entitlement_changes();

    let defects = audit_admin_audit_export_beta_page(
        &provisioning_events,
        &policy_bundle_history,
        &entitlement_changes,
    );
    let summary = AdminAuditExportBetaSummary::from_records(
        &provisioning_events,
        &policy_bundle_history,
        &entitlement_changes,
        &defects,
    );

    AdminAuditExportBetaPage {
        record_kind: ADMIN_AUDIT_EXPORT_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION,
        shared_contract_ref: ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: ADMIN_AUDIT_EXPORT_BETA_SOURCE_MATRIX_REF.to_owned(),
        provisioning_events,
        policy_bundle_history,
        entitlement_changes,
        defects,
        summary,
    }
}

fn seed_provisioning_events() -> Vec<ProvisioningEvent> {
    vec![
        provisioning_event(
            "admin-audit:provisioning:001",
            ProvisioningSourceClass::ScimManagedEndpoint,
            ProvisioningEventClass::UserCreated,
            "user:0001",
            ProvisioningLifecycleStateClass::Active,
            ProvisioningFreshnessClass::Live,
            "2026-05-15T01:00:00Z",
            AdminAuditExportBetaProfileClass::Connected,
            ProvisioningProvenance {
                source_ref:
                    "https://scim.aureline.example/v2/Users?fetched=2026-05-15T01:00:00Z"
                        .to_owned(),
                signer_id: "scim-signer:aureline-managed".to_owned(),
                signed_at: "2026-05-15T00:55:00Z".to_owned(),
                fetched_at: "2026-05-15T01:00:00Z".to_owned(),
                valid_until: "2026-05-22T00:55:00Z".to_owned(),
                transport_label: "https-scim-managed".to_owned(),
                signature_blob_ref:
                    "artifacts/security/m3/admin_audit_exports/scim_managed_001.sig".to_owned(),
            },
        ),
        provisioning_event(
            "admin-audit:provisioning:002",
            ProvisioningSourceClass::ScimSelfHostedEndpoint,
            ProvisioningEventClass::GroupCreated,
            "group:reviewers",
            ProvisioningLifecycleStateClass::Active,
            ProvisioningFreshnessClass::Live,
            "2026-05-15T01:05:00Z",
            AdminAuditExportBetaProfileClass::Connected,
            ProvisioningProvenance {
                source_ref:
                    "https://scim.customer.example/v2/Groups?fetched=2026-05-15T01:05:00Z"
                        .to_owned(),
                signer_id: "scim-signer:customer-selfhosted".to_owned(),
                signed_at: "2026-05-15T01:00:00Z".to_owned(),
                fetched_at: "2026-05-15T01:05:00Z".to_owned(),
                valid_until: "2026-05-22T01:00:00Z".to_owned(),
                transport_label: "https-scim-self-hosted".to_owned(),
                signature_blob_ref:
                    "artifacts/security/m3/admin_audit_exports/scim_self_hosted_002.sig"
                        .to_owned(),
            },
        ),
        provisioning_event(
            "admin-audit:provisioning:003",
            ProvisioningSourceClass::SignedMirrorImport,
            ProvisioningEventClass::MembershipAdded,
            "membership:user-0001-in-group-reviewers",
            ProvisioningLifecycleStateClass::Active,
            ProvisioningFreshnessClass::Cached,
            "2026-05-15T01:10:00Z",
            AdminAuditExportBetaProfileClass::MirrorOnly,
            ProvisioningProvenance {
                source_ref: "mirror://signed-mirror/aureline/provisioning/2026-05-15".to_owned(),
                signer_id: "scim-signer:aureline-managed".to_owned(),
                signed_at: "2026-05-15T00:55:00Z".to_owned(),
                fetched_at: "2026-05-15T01:10:00Z".to_owned(),
                valid_until: "2026-05-22T00:55:00Z".to_owned(),
                transport_label: "signed-mirror".to_owned(),
                signature_blob_ref:
                    "artifacts/security/m3/admin_audit_exports/signed_mirror_003.sig".to_owned(),
            },
        ),
        provisioning_event(
            "admin-audit:provisioning:004",
            ProvisioningSourceClass::SignedFileImport,
            ProvisioningEventClass::UserSuspended,
            "user:0042",
            ProvisioningLifecycleStateClass::Suspended,
            ProvisioningFreshnessClass::Live,
            "2026-05-15T02:00:00Z",
            AdminAuditExportBetaProfileClass::EnterpriseManaged,
            ProvisioningProvenance {
                source_ref: "file:///etc/aureline/provisioning/manual_2026-05-15.scim.signed"
                    .to_owned(),
                signer_id: "scim-signer:aureline-managed".to_owned(),
                signed_at: "2026-05-15T01:55:00Z".to_owned(),
                fetched_at: "2026-05-15T02:00:00Z".to_owned(),
                valid_until: "2026-05-22T01:55:00Z".to_owned(),
                transport_label: "manual-signed-file-import".to_owned(),
                signature_blob_ref:
                    "artifacts/security/m3/admin_audit_exports/manual_signed_004.sig".to_owned(),
            },
        ),
        provisioning_event(
            "admin-audit:provisioning:005",
            ProvisioningSourceClass::AirGappedSignedImport,
            ProvisioningEventClass::UserDeleted,
            "user:0099",
            ProvisioningLifecycleStateClass::Deleted,
            ProvisioningFreshnessClass::Cached,
            "2026-05-15T03:00:00Z",
            AdminAuditExportBetaProfileClass::Offline,
            ProvisioningProvenance {
                source_ref: "airgap://courier-2026-05-15/aureline/provisioning".to_owned(),
                signer_id: "scim-signer:aureline-managed".to_owned(),
                signed_at: "2026-05-14T22:00:00Z".to_owned(),
                fetched_at: "2026-05-15T03:00:00Z".to_owned(),
                valid_until: "2026-05-21T22:00:00Z".to_owned(),
                transport_label: "air-gapped-signed-transfer".to_owned(),
                signature_blob_ref:
                    "artifacts/security/m3/admin_audit_exports/airgap_005.sig".to_owned(),
            },
        ),
        provisioning_event(
            "admin-audit:provisioning:006",
            ProvisioningSourceClass::ScimManagedEndpoint,
            ProvisioningEventClass::UserReactivated,
            "user:0042",
            ProvisioningLifecycleStateClass::Active,
            ProvisioningFreshnessClass::Stale,
            "2026-05-15T04:00:00Z",
            AdminAuditExportBetaProfileClass::Connected,
            ProvisioningProvenance {
                source_ref:
                    "https://scim.aureline.example/v2/Users/0042?fetched=2026-05-15T04:00:00Z"
                        .to_owned(),
                signer_id: "scim-signer:aureline-managed".to_owned(),
                signed_at: "2026-05-08T00:55:00Z".to_owned(),
                fetched_at: "2026-05-15T04:00:00Z".to_owned(),
                valid_until: "2026-05-15T00:55:00Z".to_owned(),
                transport_label: "https-scim-managed".to_owned(),
                signature_blob_ref:
                    "artifacts/security/m3/admin_audit_exports/scim_managed_006.sig".to_owned(),
            },
        ),
    ]
}

fn seed_policy_bundle_history() -> Vec<PolicyBundleHistoryEvent> {
    vec![
        policy_bundle_history_event(
            "admin-audit:bundle-history:001",
            "policy-pack-beta:baseline:2026.04.0",
            "2026.04.0",
            "",
            PolicyBundleTransitionClass::Applied,
            "vendor_managed_origin",
            "verified_live",
            "effective",
            "policy-signer:aureline-baseline",
            "2026-04-01T00:00:00Z",
            "2026-04-01T00:05:00Z",
            AdminAuditExportBetaProfileClass::Connected,
            "https://policy.aureline.example/packs/baseline/2026.04.0",
            "artifacts/security/policy_packs/baseline-2026.04.0.sig",
        ),
        policy_bundle_history_event(
            "admin-audit:bundle-history:002",
            "policy-pack-beta:baseline:2026.05.0",
            "2026.05.0",
            "policy-pack-beta:baseline:2026.04.0",
            PolicyBundleTransitionClass::ReplacedBySuccessor,
            "vendor_managed_origin",
            "verified_live",
            "effective",
            "policy-signer:aureline-baseline",
            "2026-05-01T00:00:00Z",
            "2026-05-15T00:05:00Z",
            AdminAuditExportBetaProfileClass::Connected,
            "https://policy.aureline.example/packs/baseline/2026.05.0",
            "artifacts/security/policy_packs/baseline-2026.05.0.sig",
        ),
        policy_bundle_history_event(
            "admin-audit:bundle-history:003",
            "policy-pack-beta:mirror:2026.05.0",
            "2026.05.0",
            "policy-pack-beta:baseline:2026.04.0",
            PolicyBundleTransitionClass::ReplacedBySuccessor,
            "signed_mirror_origin",
            "verified_mirror",
            "effective",
            "policy-signer:aureline-baseline",
            "2026-05-01T00:00:00Z",
            "2026-05-15T00:10:00Z",
            AdminAuditExportBetaProfileClass::MirrorOnly,
            "mirror://signed-mirror/aureline/policy/2026-05-15",
            "artifacts/security/policy_packs/baseline-2026.05.0.sig",
        ),
        policy_bundle_history_event(
            "admin-audit:bundle-history:004",
            "policy-pack-beta:manual:2026.05.0",
            "2026.05.0",
            "policy-pack-beta:baseline:2026.04.0",
            PolicyBundleTransitionClass::ReplacedBySuccessor,
            "manual_signed_file_import",
            "verified_manual_import",
            "effective",
            "policy-signer:aureline-baseline",
            "2026-05-01T00:00:00Z",
            "2026-05-15T00:20:00Z",
            AdminAuditExportBetaProfileClass::EnterpriseManaged,
            "file:///etc/aureline/policy/2026-05-15.manual.signed",
            "artifacts/security/policy_packs/baseline-2026.05.0.sig",
        ),
        policy_bundle_history_event(
            "admin-audit:bundle-history:005",
            "policy-pack-beta:airgapped:2026.05.0",
            "2026.05.0",
            "policy-pack-beta:baseline:2026.04.0",
            PolicyBundleTransitionClass::ReplacedBySuccessor,
            "air_gapped_signed_transfer",
            "verified_air_gapped",
            "effective",
            "policy-signer:aureline-baseline",
            "2026-05-01T00:00:00Z",
            "2026-05-15T00:30:00Z",
            AdminAuditExportBetaProfileClass::Offline,
            "airgap://courier-2026-05-15/aureline/policy",
            "artifacts/security/policy_packs/baseline-2026.05.0.sig",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn policy_bundle_history_event(
    event_id: &str,
    pack_id: &str,
    pack_version: &str,
    replaces_pack_id: &str,
    transition: PolicyBundleTransitionClass,
    source_token: &str,
    signature_state_token: &str,
    apply_state_token: &str,
    signer_id: &str,
    signed_at: &str,
    applied_at: &str,
    profile: AdminAuditExportBetaProfileClass,
    origin_ref: &str,
    signature_blob_ref: &str,
) -> PolicyBundleHistoryEvent {
    PolicyBundleHistoryEvent {
        record_kind: POLICY_BUNDLE_HISTORY_EVENT_RECORD_KIND.to_owned(),
        schema_version: ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION,
        shared_contract_ref: ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF.to_owned(),
        event_id: event_id.to_owned(),
        pack_id: pack_id.to_owned(),
        pack_version: pack_version.to_owned(),
        replaces_pack_id: replaces_pack_id.to_owned(),
        transition,
        transition_token: transition.as_str().to_owned(),
        source_token: source_token.to_owned(),
        signature_state_token: signature_state_token.to_owned(),
        apply_state_token: apply_state_token.to_owned(),
        signer_id: signer_id.to_owned(),
        signed_at: signed_at.to_owned(),
        applied_at: applied_at.to_owned(),
        profile_token: profile.as_str().to_owned(),
        provenance_origin_ref: origin_ref.to_owned(),
        provenance_signature_blob_ref: signature_blob_ref.to_owned(),
    }
}

fn seed_entitlement_changes() -> Vec<EntitlementChangeEvent> {
    vec![
        entitlement_change_event(
            "admin-audit:entitlement-change:001",
            EntitlementChangeClass::Granted,
            "entitlement:provider-tool-call",
            "user:0001",
            "not_present",
            "allowed_with_approval",
            CapabilityAuthorityClass::NotApplicable,
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "2026-05-15T01:00:00Z",
            "vendor_managed_origin",
            "policy-signer:aureline-baseline",
            AdminAuditExportBetaProfileClass::Connected,
            "Provider tool calls granted under the 2026.05 baseline pack.",
        ),
        entitlement_change_event(
            "admin-audit:entitlement-change:002",
            EntitlementChangeClass::ScopeNarrowed,
            "entitlement:remote-attach",
            "user:0001",
            "denied_baseline",
            "allowed_with_approval",
            CapabilityAuthorityClass::PolicyDenied,
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "2026-05-15T01:05:00Z",
            "vendor_managed_origin",
            "policy-signer:aureline-baseline",
            AdminAuditExportBetaProfileClass::Connected,
            "Remote attach scope narrowed to reviewed targets only.",
        ),
        entitlement_change_event(
            "admin-audit:entitlement-change:003",
            EntitlementChangeClass::Revoked,
            "entitlement:ai-tool-call-mutating",
            "user:0001",
            "allowed",
            "policy_denied",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::PolicyDenied,
            "2026-05-15T01:10:00Z",
            "signed_mirror_origin",
            "policy-signer:aureline-baseline",
            AdminAuditExportBetaProfileClass::MirrorOnly,
            "AI mutating tool calls revoked for this workspace under the mirrored pack.",
        ),
        entitlement_change_event(
            "admin-audit:entitlement-change:004",
            EntitlementChangeClass::SeatRemoved,
            "entitlement:debugger-host-beta",
            "user:0099",
            "seat_allocated",
            "seat_revoked",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::NotApplicable,
            "2026-05-15T03:00:00Z",
            "air_gapped_signed_transfer",
            "policy-signer:aureline-baseline",
            AdminAuditExportBetaProfileClass::Offline,
            "Debugger-host seat removed under the air-gapped offboarding transfer.",
        ),
        entitlement_change_event(
            "admin-audit:entitlement-change:005",
            EntitlementChangeClass::ExpiryShortened,
            "entitlement:offline-bundle",
            "subject:enterprise-pilot",
            "valid_until_2026-05-22T00:55:00Z",
            "valid_until_2026-05-15T00:55:00Z",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::DegradedPreviewOnly,
            "2026-05-15T02:30:00Z",
            "manual_signed_file_import",
            "policy-signer:aureline-baseline",
            AdminAuditExportBetaProfileClass::EnterpriseManaged,
            "Offline bundle expiry shortened by managed signed-file import.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn entitlement_change_event(
    event_id: &str,
    change_class: EntitlementChangeClass,
    entitlement_id: &str,
    subject_id: &str,
    before_state: &str,
    after_state: &str,
    before_authority: CapabilityAuthorityClass,
    after_authority: CapabilityAuthorityClass,
    applied_at: &str,
    source_token: &str,
    signer_id: &str,
    profile: AdminAuditExportBetaProfileClass,
    explanation: &str,
) -> EntitlementChangeEvent {
    EntitlementChangeEvent {
        record_kind: ENTITLEMENT_CHANGE_EVENT_RECORD_KIND.to_owned(),
        schema_version: ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION,
        shared_contract_ref: ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF.to_owned(),
        event_id: event_id.to_owned(),
        change_class,
        change_token: change_class.as_str().to_owned(),
        entitlement_id: entitlement_id.to_owned(),
        subject_id: subject_id.to_owned(),
        before_state: before_state.to_owned(),
        after_state: after_state.to_owned(),
        before_authority,
        before_authority_token: before_authority.as_str().to_owned(),
        after_authority,
        after_authority_token: after_authority.as_str().to_owned(),
        applied_at: applied_at.to_owned(),
        source_token: source_token.to_owned(),
        signer_id: signer_id.to_owned(),
        profile_token: profile.as_str().to_owned(),
        explanation: explanation.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn provisioning_event(
    event_id: &str,
    source_class: ProvisioningSourceClass,
    event_class: ProvisioningEventClass,
    subject_id: &str,
    lifecycle_state: ProvisioningLifecycleStateClass,
    freshness: ProvisioningFreshnessClass,
    applied_at: &str,
    profile: AdminAuditExportBetaProfileClass,
    provenance: ProvisioningProvenance,
) -> ProvisioningEvent {
    let subject_kind = event_class.subject_kind();
    ProvisioningEvent {
        record_kind: PROVISIONING_EVENT_RECORD_KIND.to_owned(),
        schema_version: ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION,
        shared_contract_ref: ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF.to_owned(),
        event_id: event_id.to_owned(),
        source_class,
        source_token: source_class.as_str().to_owned(),
        event_class,
        event_token: event_class.as_str().to_owned(),
        subject_kind,
        subject_kind_token: subject_kind.as_str().to_owned(),
        subject_id: subject_id.to_owned(),
        lifecycle_state,
        lifecycle_state_token: lifecycle_state.as_str().to_owned(),
        freshness,
        freshness_token: freshness.as_str().to_owned(),
        applied_at: applied_at.to_owned(),
        profile_token: profile.as_str().to_owned(),
        provenance,
        no_public_endpoint_fallback: true,
        raw_private_material_excluded: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_admin_audit_export_beta_page();
        validate_admin_audit_export_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        assert!(page.provisioning_events.len() >= 4);
        assert!(page.policy_bundle_history.len() >= 4);
        assert!(page.entitlement_changes.len() >= 4);
        for profile in AdminAuditExportBetaProfileClass::ALL {
            assert!(page
                .summary
                .profiles_present
                .iter()
                .any(|token| token == profile.as_str()));
        }
    }

    #[test]
    fn provisioning_events_include_scim_and_signed_alternatives() {
        let page = seeded_admin_audit_export_beta_page();
        let sources: BTreeSet<&str> = page
            .provisioning_events
            .iter()
            .map(|event| event.source_token.as_str())
            .collect();
        assert!(sources.contains("scim_managed_endpoint"));
        assert!(sources.contains("scim_self_hosted_endpoint"));
        assert!(sources.contains("signed_file_import"));
        assert!(sources.contains("signed_mirror_import"));
        assert!(sources.contains("air_gapped_signed_import"));
    }

    #[test]
    fn policy_bundle_history_records_replacement_with_predecessor() {
        let page = seeded_admin_audit_export_beta_page();
        let replaced: Vec<&PolicyBundleHistoryEvent> = page
            .policy_bundle_history
            .iter()
            .filter(|event| {
                event.transition == PolicyBundleTransitionClass::ReplacedBySuccessor
            })
            .collect();
        assert!(!replaced.is_empty());
        for event in replaced {
            assert!(!event.replaces_pack_id.is_empty());
        }
    }

    #[test]
    fn entitlement_changes_carry_authority_transitions() {
        let page = seeded_admin_audit_export_beta_page();
        assert!(page.entitlement_changes.iter().any(|event| {
            event.change_class == EntitlementChangeClass::Granted
                && event.before_authority == CapabilityAuthorityClass::NotApplicable
                && event.after_authority == CapabilityAuthorityClass::ApprovalRequiredPerInvocation
        }));
        assert!(page.entitlement_changes.iter().any(|event| {
            event.change_class == EntitlementChangeClass::Revoked
                && event.after_authority == CapabilityAuthorityClass::PolicyDenied
        }));
    }

    #[test]
    fn validator_flags_managed_source_missing_signature() {
        let mut page = seeded_admin_audit_export_beta_page();
        page.provisioning_events[0]
            .provenance
            .signature_blob_ref
            .clear();
        let defects = audit_admin_audit_export_beta_page(
            &page.provisioning_events,
            &page.policy_bundle_history,
            &page.entitlement_changes,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AdminAuditExportBetaDefectKind::ManagedSourceMissingSignature));
    }

    #[test]
    fn validator_flags_history_transition_missing_predecessor() {
        let mut page = seeded_admin_audit_export_beta_page();
        let event = page
            .policy_bundle_history
            .iter_mut()
            .find(|event| {
                event.transition == PolicyBundleTransitionClass::ReplacedBySuccessor
            })
            .expect("seeded replacement event");
        event.replaces_pack_id.clear();
        let defects = audit_admin_audit_export_beta_page(
            &page.provisioning_events,
            &page.policy_bundle_history,
            &page.entitlement_changes,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AdminAuditExportBetaDefectKind::HistoryTransitionMissingPredecessor));
    }

    #[test]
    fn validator_flags_entitlement_narrowing_without_managed_source() {
        let mut page = seeded_admin_audit_export_beta_page();
        let event = page
            .entitlement_changes
            .iter_mut()
            .find(|event| event.change_class == EntitlementChangeClass::Revoked)
            .expect("seeded revoked entitlement");
        event.source_token = "local_advisory_file".to_owned();
        let defects = audit_admin_audit_export_beta_page(
            &page.provisioning_events,
            &page.policy_bundle_history,
            &page.entitlement_changes,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AdminAuditExportBetaDefectKind::EntitlementNarrowingWithoutManagedSource));
    }

    #[test]
    fn validator_flags_public_fallback_on_provisioning() {
        let mut page = seeded_admin_audit_export_beta_page();
        page.provisioning_events[0].no_public_endpoint_fallback = false;
        let defects = audit_admin_audit_export_beta_page(
            &page.provisioning_events,
            &page.policy_bundle_history,
            &page.entitlement_changes,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AdminAuditExportBetaDefectKind::HiddenPublicEndpointFallback));
    }

    #[test]
    fn support_export_round_trip_is_metadata_safe() {
        let page = seeded_admin_audit_export_beta_page();
        let export = AdminAuditExportBetaSupportExport::from_page(
            "admin-audit:support-export:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.defect_counts_by_kind.is_empty());
    }

    #[test]
    fn summary_counts_match_records() {
        let page = seeded_admin_audit_export_beta_page();
        assert_eq!(
            page.summary.provisioning_event_count,
            page.provisioning_events.len()
        );
        assert_eq!(
            page.summary.policy_bundle_event_count,
            page.policy_bundle_history.len()
        );
        assert_eq!(
            page.summary.entitlement_change_count,
            page.entitlement_changes.len()
        );
        assert_eq!(page.summary.defect_count, 0);
    }
}
