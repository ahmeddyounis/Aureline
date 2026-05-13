//! Portable profile export and optional-sync review primitives.
//!
//! This module is the first workspace-owned consumer for the alpha profile
//! export, device-registry, and conflict-review contracts. It keeps file
//! export/import as the local-first baseline, while giving settings, keymaps,
//! and saved views the same scope, portability, attribution, and non-widening
//! checks before a later UI or CLI surface renders them.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for alpha portable profile export projections.
pub const PORTABLE_PROFILE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Schema version for alpha device-registry projections.
pub const DEVICE_REGISTRY_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Schema version for alpha conflict-review packets.
pub const CONFLICT_REVIEW_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Artifact class carried or explained by a portable profile export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableArtifactClass {
    /// User or profile setting rows.
    Setting,
    /// Keymap bindings and resolver metadata.
    Keymap,
    /// Saved collection views or equivalent reusable collection state.
    SavedView,
    /// Snippet libraries.
    Snippet,
    /// Theme, appearance, or design-token choices.
    Theme,
    /// Extension selections or recommendations.
    ExtensionSelection,
    /// Layout defaults that are safe to move across machines.
    LayoutPreset,
    /// Non-sensitive AI preset references.
    AiPreset,
    /// Terminal preference rows that contain no live session authority.
    TerminalPreference,
}

/// Owner scope for profile-carried artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactOwnerScope {
    /// User-owned profile state.
    User,
    /// Workspace-owned state that must travel through a workspace export.
    Workspace,
    /// Shared state with an explicit non-user owner.
    Shared,
    /// Admin or policy-pinned state that the user profile may reference only.
    PolicyPinned,
    /// Provider-owned state that cannot be rewritten as user-authored state.
    ProviderOwned,
}

/// Privacy class for a profile-carried artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactPrivacyClass {
    /// Safe metadata and review labels only.
    MetadataSafeDefault,
    /// Visible to the local operator but not exported broadly by default.
    OperatorOnlyRestricted,
    /// Restricted to internal support metadata.
    InternalSupportRestricted,
    /// Contains or references secret-bearing parameters and must be excluded.
    SecretBearingExcluded,
}

/// Round-trip posture for a portable profile artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactPortabilityLabel {
    /// The artifact can round-trip unchanged through the profile body.
    Portable,
    /// The artifact stays on the current device and is listed for honesty.
    LocalOnly,
    /// The artifact is carried with visible fallback or loss of fidelity.
    Downgraded,
    /// The artifact is not carried and must name an exclusion reason.
    Excluded,
}

impl ArtifactPortabilityLabel {
    /// Returns `true` when the artifact may be applied without downgrade copy.
    pub const fn can_round_trip(self) -> bool {
        matches!(self, Self::Portable)
    }
}

/// Source posture for an artifact or conflict revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateSourcePosture {
    /// The state exists only on the current device.
    LocalOnly,
    /// The state came from an opted-in sync lane.
    Synced,
    /// The state came from an explicit import package.
    Imported,
    /// The state is pinned by policy and not user-owned.
    PolicyPinned,
    /// The state is owned by an upstream provider.
    ProviderOwned,
}

impl StateSourcePosture {
    /// Returns `true` when a profile apply cannot claim this owner silently.
    pub const fn is_managed_authority(self) -> bool {
        matches!(self, Self::PolicyPinned | Self::ProviderOwned)
    }
}

/// Reason a state class or artifact does not travel unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonPortableExclusionReason {
    /// Raw secret material or a value equivalent to a raw secret.
    SecretMaterial,
    /// Delegated credentials or approval authority.
    DelegatedCredential,
    /// A binding unique to this machine.
    MachineLocalBinding,
    /// Workspace trust grants.
    TrustGrant,
    /// Admin or organization policy payloads.
    AdminPolicy,
    /// Transient collection selection state.
    TransientSelection,
    /// Provider cursor state that may be stale on another device.
    StaleProviderCursor,
    /// Filter or query parameters that carry secret-bearing values.
    SecretBearingParameter,
    /// Required capability is missing on the target.
    MissingCapability,
    /// Provider-owned state cannot be rewritten into the profile body.
    ProviderOwnedUnsupported,
    /// Policy locks the artifact or field.
    PolicyLocked,
}

/// Capability dependency attached to a profile artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDependency {
    /// Stable capability id.
    pub capability_id: String,
    /// Whether the dependency is required for unchanged apply.
    pub required: bool,
    /// Whether the dependency is present on the current target.
    pub present: bool,
    /// Portability posture used when the capability is missing.
    pub fallback_portability: ArtifactPortabilityLabel,
    /// Redaction-aware fallback note.
    pub fallback_note: String,
}

/// One artifact row inside a portable profile export review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableProfileArtifact {
    /// Stable artifact id used by review, diagnostics, and support projections.
    pub artifact_id: String,
    /// Artifact class.
    pub artifact_class: PortableArtifactClass,
    /// Owner scope for the artifact.
    pub owner_scope: ArtifactOwnerScope,
    /// Privacy class for the artifact.
    pub privacy_class: ArtifactPrivacyClass,
    /// Portability posture.
    pub portability_label: ArtifactPortabilityLabel,
    /// Where this artifact came from.
    pub source_posture: StateSourcePosture,
    /// Source device when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_device_ref: Option<String>,
    /// Source artifact, provider, policy, or import ref.
    pub source_ref: String,
    /// Source revision ref.
    pub source_revision_ref: String,
    /// Capability dependencies that affect round-trip fidelity.
    #[serde(default)]
    pub capability_dependencies: Vec<CapabilityDependency>,
    /// Reasons this artifact is local-only, downgraded, or excluded.
    #[serde(default)]
    pub exclusion_reasons: Vec<NonPortableExclusionReason>,
    /// Whether the row would capture transient collection selection.
    #[serde(default)]
    pub captures_transient_selection: bool,
    /// Whether the row would capture a stale provider cursor.
    #[serde(default)]
    pub captures_stale_provider_cursor: bool,
    /// Whether the row would capture secret-bearing query or filter parameters.
    #[serde(default)]
    pub captures_secret_bearing_parameters: bool,
    /// Redaction-aware note explaining the portability posture.
    pub portability_note: String,
}

/// Portable profile export review body consumed by workspace entry surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableProfileExport {
    /// Schema version for this alpha export projection.
    pub schema_version: u32,
    /// Stable profile id.
    pub profile_id: String,
    /// Explicit export scope selected by the user or policy.
    pub profile_scope: String,
    /// Profile revision the export captures.
    pub profile_revision_ref: String,
    /// Source device that produced the export.
    pub source_device_ref: String,
    /// Artifact rows carried or explained by the export.
    pub artifacts: Vec<PortableProfileArtifact>,
    /// Non-portable classes named by the export manifest.
    pub non_portable_exclusions: Vec<NonPortableExclusionReason>,
}

impl PortableProfileExport {
    /// Validates the alpha export invariants for keymaps, saved views, and exclusions.
    pub fn validate(&self) -> Result<(), ProfileAlphaValidationError> {
        if self.schema_version != PORTABLE_PROFILE_ALPHA_SCHEMA_VERSION {
            return Err(ProfileAlphaValidationError::WrongSchemaVersion {
                expected: PORTABLE_PROFILE_ALPHA_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.profile_scope.trim().is_empty() {
            return Err(ProfileAlphaValidationError::MissingProfileScope);
        }
        if self.profile_revision_ref.trim().is_empty() || self.source_device_ref.trim().is_empty() {
            return Err(ProfileAlphaValidationError::MissingProfileAttribution);
        }

        for required in [
            PortableArtifactClass::Keymap,
            PortableArtifactClass::SavedView,
        ] {
            if !self
                .artifacts
                .iter()
                .any(|artifact| artifact.artifact_class == required)
            {
                return Err(ProfileAlphaValidationError::MissingRequiredArtifactClass(
                    required,
                ));
            }
        }

        for required in [
            NonPortableExclusionReason::SecretMaterial,
            NonPortableExclusionReason::DelegatedCredential,
        ] {
            if !self.non_portable_exclusions.contains(&required) {
                return Err(ProfileAlphaValidationError::MissingNonPortableExclusion(
                    required,
                ));
            }
        }

        for artifact in &self.artifacts {
            artifact.validate()?;
        }

        Ok(())
    }

    /// Returns a stable portability projection suitable for support or review UI.
    pub fn portability_projection(&self) -> Vec<ArtifactPortabilityProjection> {
        self.artifacts
            .iter()
            .map(|artifact| ArtifactPortabilityProjection {
                artifact_id: artifact.artifact_id.clone(),
                artifact_class: artifact.artifact_class,
                owner_scope: artifact.owner_scope,
                privacy_class: artifact.privacy_class,
                portability_label: artifact.portability_label,
                explanation: artifact.portability_note.clone(),
            })
            .collect()
    }
}

impl PortableProfileArtifact {
    /// Validates one profile artifact row.
    pub fn validate(&self) -> Result<(), ProfileAlphaValidationError> {
        if self.artifact_id.trim().is_empty()
            || self.source_ref.trim().is_empty()
            || self.source_revision_ref.trim().is_empty()
        {
            return Err(ProfileAlphaValidationError::ArtifactMissingAttribution {
                artifact_id: self.artifact_id.clone(),
            });
        }

        if !self.portability_label.can_round_trip() && self.exclusion_reasons.is_empty() {
            return Err(
                ProfileAlphaValidationError::ArtifactMissingExclusionReason {
                    artifact_id: self.artifact_id.clone(),
                },
            );
        }

        if self.artifact_class == PortableArtifactClass::SavedView {
            if self.captures_transient_selection {
                return Err(
                    ProfileAlphaValidationError::SavedViewCarriesForbiddenState {
                        artifact_id: self.artifact_id.clone(),
                        reason: NonPortableExclusionReason::TransientSelection,
                    },
                );
            }
            if self.captures_stale_provider_cursor {
                return Err(
                    ProfileAlphaValidationError::SavedViewCarriesForbiddenState {
                        artifact_id: self.artifact_id.clone(),
                        reason: NonPortableExclusionReason::StaleProviderCursor,
                    },
                );
            }
            if self.captures_secret_bearing_parameters {
                return Err(
                    ProfileAlphaValidationError::SavedViewCarriesForbiddenState {
                        artifact_id: self.artifact_id.clone(),
                        reason: NonPortableExclusionReason::SecretBearingParameter,
                    },
                );
            }
        }

        if self.privacy_class == ArtifactPrivacyClass::SecretBearingExcluded
            && self.portability_label != ArtifactPortabilityLabel::Excluded
        {
            return Err(ProfileAlphaValidationError::SecretBearingArtifactPortable {
                artifact_id: self.artifact_id.clone(),
            });
        }

        for dependency in &self.capability_dependencies {
            if dependency.required
                && !dependency.present
                && dependency.fallback_portability == ArtifactPortabilityLabel::Portable
            {
                return Err(
                    ProfileAlphaValidationError::CapabilityDependencyMissingFallback {
                        artifact_id: self.artifact_id.clone(),
                        capability_id: dependency.capability_id.clone(),
                    },
                );
            }
        }

        Ok(())
    }
}

/// Export-safe row that explains an artifact's portability posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactPortabilityProjection {
    /// Stable artifact id.
    pub artifact_id: String,
    /// Artifact class.
    pub artifact_class: PortableArtifactClass,
    /// Owner scope.
    pub owner_scope: ArtifactOwnerScope,
    /// Privacy class.
    pub privacy_class: ArtifactPrivacyClass,
    /// Portability label.
    pub portability_label: ArtifactPortabilityLabel,
    /// Redaction-aware explanation.
    pub explanation: String,
}

/// Transport state shown by a device-registry surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncTransportState {
    /// Sync transport is reachable.
    Available,
    /// Sync transport is reachable but not healthy enough to apply blindly.
    Degraded,
    /// Sync transport is unavailable.
    Unavailable,
    /// Sync transport or policy refused the session.
    Refused,
}

/// Local fallback posture shown when sync cannot be trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalFallbackPosture {
    /// Device is synced and no local-only fallback is currently active.
    Synced,
    /// Local durable state remains authoritative.
    LocalOnlyAuthoritative,
    /// User is carrying continuity through manual export/import.
    ManualContinuity,
    /// Session is refused and local durable state remains untouched.
    Refused,
}

/// One alpha device-registry record for review and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDeviceRegistryAlphaRecord {
    /// Schema version for this alpha device record.
    pub schema_version: u32,
    /// Opaque device id.
    pub device_id: String,
    /// Redaction-aware label chosen by the user.
    pub device_label: String,
    /// Device revision or lineage cursor shown in review.
    pub device_revision_ref: String,
    /// Current transport state.
    pub transport_state: SyncTransportState,
    /// Local fallback posture.
    pub local_fallback_posture: LocalFallbackPosture,
    /// Identity mode at registration.
    pub identity_mode: String,
}

impl SyncDeviceRegistryAlphaRecord {
    /// Validates that the device row can render identity, revision, transport, and fallback.
    pub fn validate(&self) -> Result<(), ProfileAlphaValidationError> {
        if self.schema_version != DEVICE_REGISTRY_ALPHA_SCHEMA_VERSION {
            return Err(ProfileAlphaValidationError::WrongSchemaVersion {
                expected: DEVICE_REGISTRY_ALPHA_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.device_id.trim().is_empty()
            || self.device_label.trim().is_empty()
            || self.device_revision_ref.trim().is_empty()
            || self.identity_mode.trim().is_empty()
        {
            return Err(ProfileAlphaValidationError::DeviceIdentityIncomplete {
                device_id: self.device_id.clone(),
            });
        }
        if self.transport_state != SyncTransportState::Available
            && self.local_fallback_posture == LocalFallbackPosture::Synced
        {
            return Err(ProfileAlphaValidationError::DeviceMissingLocalFallback {
                device_id: self.device_id.clone(),
            });
        }
        Ok(())
    }
}

/// Export-safe device row shown by a device-registry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceRegistrySurfaceRow {
    /// Opaque device id.
    pub device_id: String,
    /// Redaction-aware device label.
    pub device_label: String,
    /// Revision or lineage cursor.
    pub device_revision_ref: String,
    /// Transport state.
    pub transport_state: SyncTransportState,
    /// Local fallback posture.
    pub local_fallback_posture: LocalFallbackPosture,
}

/// Projects device registry records into the minimum surface row.
pub fn project_device_registry_surface(
    devices: &[SyncDeviceRegistryAlphaRecord],
) -> Result<Vec<DeviceRegistrySurfaceRow>, ProfileAlphaValidationError> {
    let mut rows = Vec::with_capacity(devices.len());
    for device in devices {
        device.validate()?;
        rows.push(DeviceRegistrySurfaceRow {
            device_id: device.device_id.clone(),
            device_label: device.device_label.clone(),
            device_revision_ref: device.device_revision_ref.clone(),
            transport_state: device.transport_state,
            local_fallback_posture: device.local_fallback_posture,
        });
    }
    Ok(rows)
}

/// Artifact class that can appear in a conflict-review packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictArtifactClass {
    /// Setting value conflict.
    Setting,
    /// Keymap binding conflict.
    Keymap,
    /// Saved-view conflict.
    SavedView,
}

/// Conflict classes required by the alpha review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncConflictClassAlpha {
    /// Same artifact key diverged across sources.
    SameKeyDivergence,
    /// Policy locks the incoming or local value.
    PolicyLocked,
    /// A required capability is missing.
    MissingCapability,
    /// One side deleted while the other side modified.
    DeleteVsModify,
    /// Incoming remote revision is stale.
    StaleRemote,
}

/// Review actions that every alpha conflict packet names explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictAction {
    /// Retain the local revision.
    KeepLocal,
    /// Adopt the synced or incoming revision after review.
    KeepSynced,
    /// Open a structured comparison.
    Compare,
}

impl ConflictAction {
    /// Returns the user-facing action label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::KeepLocal => "Keep local",
            Self::KeepSynced => "Keep synced",
            Self::Compare => "Compare",
        }
    }
}

/// One action offer inside a conflict packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictActionOffer {
    /// Action being offered.
    pub action: ConflictAction,
    /// Whether the action can currently run.
    pub available: bool,
    /// Redaction-aware disabled reason when unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

/// Field-aware diff row inside a conflict packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictFieldDiff {
    /// Dotted or pointer-like field path.
    pub field_path: String,
    /// Diff kind.
    pub change_kind: String,
    /// Redaction-aware summary.
    pub summary: String,
}

/// Revision candidate inside a conflict packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictRevision {
    /// Revision ref.
    pub revision_ref: String,
    /// Source posture for this revision.
    pub source_posture: StateSourcePosture,
    /// Source ref.
    pub source_ref: String,
    /// Source device when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_device_ref: Option<String>,
}

impl ConflictRevision {
    /// Returns `true` when enough attribution is present for support review.
    pub fn has_attribution(&self) -> bool {
        !self.revision_ref.trim().is_empty() && !self.source_ref.trim().is_empty()
    }
}

/// Widening vector denied by profile import or sync apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WideningVector {
    /// No widening vector.
    None,
    /// Workspace trust would widen.
    WorkspaceTrust,
    /// Extension permissions would widen.
    ExtensionPermission,
    /// Managed entitlement would widen.
    ManagedEntitlement,
    /// AI egress would widen.
    AiEgress,
    /// Network egress would widen.
    NetworkEgress,
    /// Credential exposure would widen.
    CredentialExposure,
    /// Managed ownership would widen.
    ManagedOwnership,
}

impl WideningVector {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::WorkspaceTrust => "workspace_trust",
            Self::ExtensionPermission => "extension_permission",
            Self::ManagedEntitlement => "managed_entitlement",
            Self::AiEgress => "ai_egress",
            Self::NetworkEgress => "network_egress",
            Self::CredentialExposure => "credential_exposure",
            Self::ManagedOwnership => "managed_ownership",
        }
    }
}

/// Non-widening verdict attached to conflict and import review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonWideningVerdict {
    /// Whether adopting the incoming revision would widen authority.
    pub would_widen: bool,
    /// Blocked vectors.
    #[serde(default)]
    pub blocked_vectors: Vec<WideningVector>,
    /// Whether applying would claim managed/provider ownership silently.
    #[serde(default)]
    pub managed_ownership_would_widen: bool,
    /// Redaction-aware note.
    pub note: String,
}

impl NonWideningVerdict {
    /// Returns `true` when the verdict allows apply through the profile lane.
    pub fn allows_profile_apply(&self) -> bool {
        !self.would_widen
            && !self.managed_ownership_would_widen
            && self
                .blocked_vectors
                .iter()
                .all(|vector| *vector == WideningVector::None)
    }
}

/// Conflict-review packet for alpha profile, keymap, and saved-view sync.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictReviewPacketAlpha {
    /// Schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Artifact class.
    pub artifact_class: ConflictArtifactClass,
    /// Artifact id.
    pub artifact_id: String,
    /// Owner scope.
    pub owner_scope: ArtifactOwnerScope,
    /// Privacy class.
    pub privacy_class: ArtifactPrivacyClass,
    /// Portability label.
    pub portability_label: ArtifactPortabilityLabel,
    /// Conflict class.
    pub conflict_class: SyncConflictClassAlpha,
    /// Local candidate revision.
    pub local_revision: ConflictRevision,
    /// Incoming synced/imported candidate revision.
    pub incoming_revision: ConflictRevision,
    /// Winning revision after review, if already chosen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_revision: Option<ConflictRevision>,
    /// Field-aware diffs.
    pub field_diffs: Vec<ConflictFieldDiff>,
    /// Offered actions.
    pub action_offers: Vec<ConflictActionOffer>,
    /// Non-widening verdict.
    pub non_widening_verdict: NonWideningVerdict,
    /// Missing capability ids for missing-capability packets.
    #[serde(default)]
    pub missing_capability_ids: Vec<String>,
}

impl ConflictReviewPacketAlpha {
    /// Validates field awareness, attribution, and the required action set.
    pub fn validate(&self) -> Result<(), ProfileAlphaValidationError> {
        if self.schema_version != CONFLICT_REVIEW_ALPHA_SCHEMA_VERSION {
            return Err(ProfileAlphaValidationError::WrongSchemaVersion {
                expected: CONFLICT_REVIEW_ALPHA_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty() || self.artifact_id.trim().is_empty() {
            return Err(ProfileAlphaValidationError::ConflictMissingAttribution {
                packet_id: self.packet_id.clone(),
            });
        }
        if !self.local_revision.has_attribution() || !self.incoming_revision.has_attribution() {
            return Err(ProfileAlphaValidationError::ConflictMissingAttribution {
                packet_id: self.packet_id.clone(),
            });
        }
        if self.field_diffs.is_empty()
            || self
                .field_diffs
                .iter()
                .any(|diff| diff.field_path.trim().is_empty())
        {
            return Err(ProfileAlphaValidationError::ConflictMissingFieldDiff {
                packet_id: self.packet_id.clone(),
            });
        }

        let offered: BTreeSet<_> = self
            .action_offers
            .iter()
            .map(|offer| offer.action)
            .collect();
        for action in [
            ConflictAction::KeepLocal,
            ConflictAction::KeepSynced,
            ConflictAction::Compare,
        ] {
            if !offered.contains(&action) {
                return Err(ProfileAlphaValidationError::ConflictActionMissing {
                    packet_id: self.packet_id.clone(),
                    action,
                });
            }
        }

        if !self.non_widening_verdict.allows_profile_apply()
            && self.action_available(ConflictAction::KeepSynced)
        {
            return Err(ProfileAlphaValidationError::ScopeWideningActionAllowed {
                packet_id: self.packet_id.clone(),
            });
        }

        if matches!(
            self.conflict_class,
            SyncConflictClassAlpha::PolicyLocked | SyncConflictClassAlpha::MissingCapability
        ) && self.action_available(ConflictAction::KeepSynced)
        {
            return Err(
                ProfileAlphaValidationError::BlockedConflictKeepSyncedAvailable {
                    packet_id: self.packet_id.clone(),
                    conflict_class: self.conflict_class,
                },
            );
        }

        if self.conflict_class == SyncConflictClassAlpha::MissingCapability
            && self.missing_capability_ids.is_empty()
        {
            return Err(ProfileAlphaValidationError::ConflictMissingCapabilityId {
                packet_id: self.packet_id.clone(),
            });
        }

        if let Some(winning_revision) = &self.winning_revision {
            if !winning_revision.has_attribution() {
                return Err(ProfileAlphaValidationError::ConflictMissingAttribution {
                    packet_id: self.packet_id.clone(),
                });
            }
        }

        Ok(())
    }

    /// Returns whether an action is currently available.
    pub fn action_available(&self, action: ConflictAction) -> bool {
        self.action_offers
            .iter()
            .any(|offer| offer.action == action && offer.available)
    }
}

/// Decision produced by non-widening import review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportApplyDecision {
    /// The apply may proceed.
    ApplyAllowed,
    /// The apply may proceed only because it narrows behavior.
    NarrowingOnly,
    /// The apply is blocked.
    Blocked,
}

/// Scope-explicit import or sync apply request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportApplyRequest {
    /// Source artifact ref.
    pub source_artifact_ref: String,
    /// Explicit target scope selected by the user or caller.
    pub target_scope: String,
    /// Current owner posture at the target.
    pub current_owner: StateSourcePosture,
    /// Incoming owner posture.
    pub incoming_owner: StateSourcePosture,
    /// Widening vectors detected before apply.
    #[serde(default)]
    pub widening_vectors: Vec<WideningVector>,
    /// Whether the proposed apply only narrows behavior.
    pub narrowing_only: bool,
}

/// Result of reviewing an import or sync apply request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportApplyReview {
    /// Decision reached by the review.
    pub decision: ImportApplyDecision,
    /// Target scope copied from the request.
    pub target_scope: String,
    /// Whether the request may apply through the profile lane.
    pub allowed: bool,
    /// Redaction-aware reasons shown by review/support surfaces.
    pub reasons: Vec<String>,
}

/// Reviews an import or sync apply request against non-widening rules.
pub fn review_non_widening_import(request: &ImportApplyRequest) -> ImportApplyReview {
    let mut reasons = Vec::new();

    if request.target_scope.trim().is_empty() {
        reasons.push("target scope is required before profile apply".to_string());
    }
    if request.source_artifact_ref.trim().is_empty() {
        reasons.push("source artifact ref is required before profile apply".to_string());
    }
    for vector in &request.widening_vectors {
        if *vector != WideningVector::None {
            reasons.push(format!("blocked widening vector: {}", vector.as_str()));
        }
    }
    if request.incoming_owner.is_managed_authority()
        && request.current_owner != request.incoming_owner
    {
        reasons.push("incoming artifact would claim policy or provider ownership".to_string());
    }
    if request.current_owner.is_managed_authority()
        && request.current_owner != request.incoming_owner
    {
        reasons.push(
            "current policy or provider owner cannot be replaced by profile apply".to_string(),
        );
    }

    if !reasons.is_empty() {
        return ImportApplyReview {
            decision: ImportApplyDecision::Blocked,
            target_scope: request.target_scope.clone(),
            allowed: false,
            reasons,
        };
    }

    let decision = if request.narrowing_only {
        ImportApplyDecision::NarrowingOnly
    } else {
        ImportApplyDecision::ApplyAllowed
    };

    ImportApplyReview {
        decision,
        target_scope: request.target_scope.clone(),
        allowed: true,
        reasons: Vec::new(),
    }
}

/// Validation errors for alpha profile and sync review objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileAlphaValidationError {
    /// The payload used an unsupported schema version.
    WrongSchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// The profile export did not name an explicit scope.
    MissingProfileScope,
    /// The profile export did not name revision or source-device attribution.
    MissingProfileAttribution,
    /// The profile export did not include a required artifact class.
    MissingRequiredArtifactClass(PortableArtifactClass),
    /// The profile export did not list a required exclusion class.
    MissingNonPortableExclusion(NonPortableExclusionReason),
    /// An artifact did not carry source or revision attribution.
    ArtifactMissingAttribution {
        /// Artifact id.
        artifact_id: String,
    },
    /// A local-only, downgraded, or excluded artifact did not explain why.
    ArtifactMissingExclusionReason {
        /// Artifact id.
        artifact_id: String,
    },
    /// A saved view tried to carry transient or unsafe provider state.
    SavedViewCarriesForbiddenState {
        /// Artifact id.
        artifact_id: String,
        /// Forbidden state reason.
        reason: NonPortableExclusionReason,
    },
    /// A secret-bearing artifact was not excluded.
    SecretBearingArtifactPortable {
        /// Artifact id.
        artifact_id: String,
    },
    /// A missing required capability did not declare a downgrade or exclusion.
    CapabilityDependencyMissingFallback {
        /// Artifact id.
        artifact_id: String,
        /// Capability id.
        capability_id: String,
    },
    /// A device row did not expose identity, label, revision, or identity mode.
    DeviceIdentityIncomplete {
        /// Device id.
        device_id: String,
    },
    /// A degraded or refused device row did not expose local-only fallback.
    DeviceMissingLocalFallback {
        /// Device id.
        device_id: String,
    },
    /// A conflict packet did not include a required action.
    ConflictActionMissing {
        /// Packet id.
        packet_id: String,
        /// Missing action.
        action: ConflictAction,
    },
    /// A conflict packet lacked source, device, or revision attribution.
    ConflictMissingAttribution {
        /// Packet id.
        packet_id: String,
    },
    /// A conflict packet lacked field-aware diffs.
    ConflictMissingFieldDiff {
        /// Packet id.
        packet_id: String,
    },
    /// A widening conflict left Keep synced available.
    ScopeWideningActionAllowed {
        /// Packet id.
        packet_id: String,
    },
    /// A blocked conflict class left Keep synced available.
    BlockedConflictKeepSyncedAvailable {
        /// Packet id.
        packet_id: String,
        /// Conflict class.
        conflict_class: SyncConflictClassAlpha,
    },
    /// A missing-capability packet did not name the capability.
    ConflictMissingCapabilityId {
        /// Packet id.
        packet_id: String,
    },
}

impl fmt::Display for ProfileAlphaValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongSchemaVersion { expected, actual } => {
                write!(f, "expected schema version {expected}, got {actual}")
            }
            Self::MissingProfileScope => write!(f, "profile export must name a scope"),
            Self::MissingProfileAttribution => {
                write!(
                    f,
                    "profile export must name revision and source-device refs"
                )
            }
            Self::MissingRequiredArtifactClass(class) => {
                write!(
                    f,
                    "profile export is missing required artifact class {class:?}"
                )
            }
            Self::MissingNonPortableExclusion(reason) => {
                write!(
                    f,
                    "profile export is missing non-portable exclusion {reason:?}"
                )
            }
            Self::ArtifactMissingAttribution { artifact_id } => {
                write!(f, "artifact {artifact_id} is missing attribution")
            }
            Self::ArtifactMissingExclusionReason { artifact_id } => {
                write!(f, "artifact {artifact_id} is missing an exclusion reason")
            }
            Self::SavedViewCarriesForbiddenState {
                artifact_id,
                reason,
            } => write!(
                f,
                "saved view artifact {artifact_id} carries forbidden state {reason:?}"
            ),
            Self::SecretBearingArtifactPortable { artifact_id } => {
                write!(f, "secret-bearing artifact {artifact_id} must be excluded")
            }
            Self::CapabilityDependencyMissingFallback {
                artifact_id,
                capability_id,
            } => write!(
                f,
                "artifact {artifact_id} missing fallback for capability {capability_id}"
            ),
            Self::DeviceIdentityIncomplete { device_id } => {
                write!(
                    f,
                    "device {device_id} is missing displayable identity fields"
                )
            }
            Self::DeviceMissingLocalFallback { device_id } => {
                write!(f, "device {device_id} is degraded without local fallback")
            }
            Self::ConflictActionMissing { packet_id, action } => {
                write!(f, "conflict packet {packet_id} missing action {action:?}")
            }
            Self::ConflictMissingAttribution { packet_id } => {
                write!(f, "conflict packet {packet_id} missing attribution")
            }
            Self::ConflictMissingFieldDiff { packet_id } => {
                write!(f, "conflict packet {packet_id} missing field diff")
            }
            Self::ScopeWideningActionAllowed { packet_id } => {
                write!(f, "conflict packet {packet_id} allows widening apply")
            }
            Self::BlockedConflictKeepSyncedAvailable {
                packet_id,
                conflict_class,
            } => write!(
                f,
                "conflict packet {packet_id} leaves Keep synced available for {conflict_class:?}"
            ),
            Self::ConflictMissingCapabilityId { packet_id } => {
                write!(f, "conflict packet {packet_id} missing capability id")
            }
        }
    }
}

impl std::error::Error for ProfileAlphaValidationError {}
