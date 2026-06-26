//! Docs-pack manager rows/cards and manifest parity with import/export
//! continuity across the claimed M5 docs/help/onboarding profiles.
//!
//! The canonical [`DocsPackManifest`] already pins a docs pack's pack id,
//! signer, source channel, version range, refresh state, mirror source, pin
//! state, and schema version. This lane materializes the *manager* layer on top
//! of that manifest: the rows/cards a person uses to pin, refresh, remove,
//! change the mirror source, and toggle offline availability, plus the
//! pack-size/count, last-successful-refresh, and import/export continuity each
//! row carries. The manager keeps docs packs as versioned, mirrorable,
//! exportable, policy-aware product artifacts rather than hidden caches.
//!
//! Each [`DocsPackManagerRow`] embeds the canonical [`DocsPackManifest`] —
//! proving the manager reuses one manifest truth rather than re-deriving
//! signer/channel/mirror/version state — and adds:
//!
//! * a [`DocsPackLifecycleFlow`] (`local_only`, `mirrored`, `managed`,
//!   `air_gapped`) that keeps mirror and offline flows first-class;
//! * a set of [`DocsPackManagerActionState`] affordances (pin/unpin, refresh,
//!   remove, change-mirror-source, set-offline-availability, export) whose
//!   enablement is attributable — a disabled action always names why;
//! * pack size, document count, last successful refresh, and last refresh
//!   attempt, surfaced rather than hidden when a payload is unavailable;
//! * a [`DocsPackImportExportContinuity`] block that preserves docs-pack
//!   identity and lifecycle state across import/export so a managed or
//!   air-gapped pack never flattens into generic documentation cache metadata.
//!
//! A [`DocsPackManagerProfileProjection`] records, per claimed M5 manager
//! surface (docs-browser manager, help-pane manager, onboarding manager,
//! settings docs-packs manager, air-gapped console, support export), that the
//! signer/channel/mirror source, pin/offline/refresh posture, version range,
//! import/export continuity, and lifecycle state stay visible without re-minting
//! truth locally.
//!
//! The [`DocsPackManagerPacket`] validates the cross-cutting invariants: every
//! row keeps signer/channel/mirror source, refresh state, pin/offline posture,
//! and version range visible; an unavailable payload or signature state is
//! disclosed rather than hidden; mirror/offline/air-gapped rows never degrade
//! into opaque cache or browser-only fallback wording; every required lifecycle
//! flow and manager action is represented; and import/export continuity survives
//! export. The packet reuses the canonical docs-pack manifest, signer, channel,
//! refresh, mirror, pin, and availability vocabularies owned by
//! [`crate::docs_pack_truth_packet`] rather than minting parallel tokens. Raw
//! document bodies, raw URLs, raw provider payloads, and credentials never cross
//! this boundary.
//!
//! The boundary schema is
//! [`schemas/docs/ship-docs-pack-manager-rows-and-manifest-parity-with-import-export-continuity.schema.json`](../../../../schemas/docs/ship-docs-pack-manager-rows-and-manifest-parity-with-import-export-continuity.schema.json).
//! The contract doc is
//! [`docs/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity.md`](../../../../docs/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity/`](../../../../fixtures/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    DocsPackChannel, DocsPackLocalAvailability, DocsPackManifest, DocsPackPinState,
    DocsPackRefreshState,
};

/// Stable record-kind tag carried by [`DocsPackManagerPacket`].
pub const DOCS_PACK_MANAGER_RECORD_KIND: &str = "docs_pack_manager_packet";

/// Stable record-kind tag carried by [`DocsPackManagerSupportExport`].
pub const DOCS_PACK_MANAGER_SUPPORT_EXPORT_RECORD_KIND: &str = "docs_pack_manager_support_export";

/// Schema version for docs-pack manager records.
pub const DOCS_PACK_MANAGER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_PACK_MANAGER_SCHEMA_REF: &str =
    "schemas/docs/ship-docs-pack-manager-rows-and-manifest-parity-with-import-export-continuity.schema.json";

/// Repo-relative path of the contract doc.
pub const DOCS_PACK_MANAGER_DOC_REF: &str =
    "docs/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity.md";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_PACK_MANAGER_ARTIFACT_REF: &str =
    "artifacts/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_PACK_MANAGER_SUMMARY_REF: &str =
    "artifacts/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_PACK_MANAGER_FIXTURE_DIR: &str =
    "fixtures/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity";

/// Closed lifecycle-flow taxonomy for a managed docs pack.
///
/// Keeps mirror and offline flows first-class rather than collapsing every
/// non-local pack into one "remote" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackLifecycleFlow {
    /// Pack ships and resolves entirely from the local workspace / binary.
    LocalOnly,
    /// Pack mirrors a signed upstream and may refresh against it.
    Mirrored,
    /// Pack is distributed and pinned by an operator / managed deployment.
    Managed,
    /// Pack reached the instance through an air-gapped, out-of-band import.
    AirGapped,
}

impl DocsPackLifecycleFlow {
    /// Every required lifecycle flow a stable packet must exercise.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalOnly,
        Self::Mirrored,
        Self::Managed,
        Self::AirGapped,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Mirrored => "mirrored",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
        }
    }

    /// True when the flow keeps mirror / offline posture first-class and must
    /// never degrade into opaque-cache or browser-only fallback wording.
    pub const fn is_mirror_or_offline(self) -> bool {
        matches!(self, Self::Mirrored | Self::AirGapped)
    }

    /// True when `origin` is an admissible import provenance for this flow.
    pub fn allows_origin(self, origin: DocsPackImportOrigin) -> bool {
        match self {
            Self::LocalOnly => matches!(origin, DocsPackImportOrigin::FreshlyInstalled),
            Self::Mirrored => matches!(
                origin,
                DocsPackImportOrigin::MirroredSync
                    | DocsPackImportOrigin::ImportedBundle
                    | DocsPackImportOrigin::OperatorManaged
            ),
            Self::Managed => matches!(
                origin,
                DocsPackImportOrigin::OperatorManaged | DocsPackImportOrigin::ImportedBundle
            ),
            Self::AirGapped => matches!(
                origin,
                DocsPackImportOrigin::AirGappedSideload | DocsPackImportOrigin::OperatorManaged
            ),
        }
    }
}

/// Claimed M5 manager surface that must read the same manager truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackManagerProfile {
    /// Docs-browser docs-pack manager pane.
    DocsBrowserManager,
    /// Help-pane docs-pack manager.
    HelpPaneManager,
    /// Onboarding / learning docs-pack manager step.
    OnboardingManager,
    /// Settings docs-packs manager surface.
    SettingsDocsPacksManager,
    /// Air-gapped / managed console.
    AirGappedConsole,
    /// Support export bundle.
    SupportExport,
}

impl DocsPackManagerProfile {
    /// Every required manager profile, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::DocsBrowserManager,
        Self::HelpPaneManager,
        Self::OnboardingManager,
        Self::SettingsDocsPacksManager,
        Self::AirGappedConsole,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserManager => "docs_browser_manager",
            Self::HelpPaneManager => "help_pane_manager",
            Self::OnboardingManager => "onboarding_manager",
            Self::SettingsDocsPacksManager => "settings_docs_packs_manager",
            Self::AirGappedConsole => "air_gapped_console",
            Self::SupportExport => "support_export",
        }
    }
}

/// Closed manager-action vocabulary for a docs-pack row/card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackManagerAction {
    /// Pin the pack to its current revision.
    Pin,
    /// Release a pin so the pack tracks its channel head.
    Unpin,
    /// Refresh the pack against its source channel / mirror.
    Refresh,
    /// Remove the pack from this instance.
    Remove,
    /// Change the mirror source the pack resolves through.
    ChangeMirrorSource,
    /// Toggle offline availability (pin for offline / release the offline pin).
    SetOfflineAvailability,
    /// Export the pack as a signed, identity-preserving bundle.
    ExportPack,
}

impl DocsPackManagerAction {
    /// Manager actions that must be represented on every row regardless of pin
    /// posture. Pin/unpin is governed separately because exactly one applies.
    pub const REQUIRED_NON_PIN: [Self; 5] = [
        Self::Refresh,
        Self::Remove,
        Self::ChangeMirrorSource,
        Self::SetOfflineAvailability,
        Self::ExportPack,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pin => "pin",
            Self::Unpin => "unpin",
            Self::Refresh => "refresh",
            Self::Remove => "remove",
            Self::ChangeMirrorSource => "change_mirror_source",
            Self::SetOfflineAvailability => "set_offline_availability",
            Self::ExportPack => "export_pack",
        }
    }
}

/// Closed availability taxonomy for one manager action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackManagerActionAvailability {
    /// Action is available and ready to invoke.
    Available,
    /// Action is available but requires an explicit confirmation step.
    RequiresConfirmation,
    /// Action is disabled by policy; the reason must be disclosed.
    DisabledByPolicy,
    /// Action is disabled because the pack payload is unavailable.
    DisabledUnavailable,
    /// Action does not apply to this pack (e.g. change-mirror on a local pack).
    NotApplicable,
}

impl DocsPackManagerActionAvailability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::RequiresConfirmation => "requires_confirmation",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::DisabledUnavailable => "disabled_unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this availability must name a disclosed reason.
    pub const fn requires_reason(self) -> bool {
        matches!(
            self,
            Self::DisabledByPolicy | Self::DisabledUnavailable | Self::NotApplicable
        )
    }
}

/// One manager action affordance on a docs-pack row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackManagerActionState {
    /// Closed manager action.
    pub action: DocsPackManagerAction,
    /// Closed availability for the action.
    pub availability: DocsPackManagerActionAvailability,
    /// Disclosed reason when the action is disabled or not applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

impl DocsPackManagerActionState {
    fn has_required_reason(&self) -> bool {
        if !self.availability.requires_reason() {
            return true;
        }
        self.disabled_reason
            .as_deref()
            .map(|reason| !reason.trim().is_empty())
            .unwrap_or(false)
    }
}

/// Closed import-provenance taxonomy preserved across import/export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackImportOrigin {
    /// Pack was freshly installed in this workspace / binary.
    FreshlyInstalled,
    /// Pack arrived as a signed import bundle.
    ImportedBundle,
    /// Pack arrived through a mirror sync.
    MirroredSync,
    /// Pack was sideloaded from air-gapped, out-of-band media.
    AirGappedSideload,
    /// Pack was distributed by an operator / managed deployment.
    OperatorManaged,
}

impl DocsPackImportOrigin {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshlyInstalled => "freshly_installed",
            Self::ImportedBundle => "imported_bundle",
            Self::MirroredSync => "mirrored_sync",
            Self::AirGappedSideload => "air_gapped_sideload",
            Self::OperatorManaged => "operator_managed",
        }
    }
}

/// Import/export continuity block. Preserves docs-pack identity and lifecycle
/// state across import and export so a managed or air-gapped pack never
/// flattens into generic documentation cache metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackImportExportContinuity {
    /// Stable continuity id.
    pub continuity_id: String,
    /// Closed import provenance for the current revision.
    pub import_origin: DocsPackImportOrigin,
    /// Opaque ref to the import bundle this revision arrived in, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_bundle_ref: Option<String>,
    /// Time the current revision was imported, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<String>,
    /// Opaque ref to the export bundle most recently produced, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_bundle_ref: Option<String>,
    /// Stable continuity token carried verbatim across import and export so
    /// pack identity is never re-minted on round-trip.
    pub continuity_token: String,
    /// True when export preserves the pack identity (id, signer, channel).
    pub preserves_identity_on_export: bool,
    /// True when export preserves the lifecycle state (pin/offline/mirror).
    pub preserves_lifecycle_state_on_export: bool,
}

impl DocsPackImportExportContinuity {
    fn is_continuous(&self) -> bool {
        !self.continuity_id.trim().is_empty()
            && !self.continuity_token.trim().is_empty()
            && self.preserves_identity_on_export
            && self.preserves_lifecycle_state_on_export
    }
}

/// One manager row/card over a docs pack. Embeds the canonical
/// [`DocsPackManifest`] so the manager reuses one manifest truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackManagerRow {
    /// Stable row id.
    pub row_id: String,
    /// Canonical docs-pack manifest the row manages.
    pub manifest: DocsPackManifest,
    /// Closed lifecycle flow for the pack.
    pub lifecycle_flow: DocsPackLifecycleFlow,
    /// Pack size in bytes; absent when the payload is unavailable locally.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_size_bytes: Option<u64>,
    /// Document count; absent when the payload is unavailable locally.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_count: Option<u32>,
    /// Last successful refresh ISO timestamp; absent when never refreshed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_refresh_at: Option<String>,
    /// Last refresh attempt ISO timestamp; absent when never attempted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refresh_attempt_at: Option<String>,
    /// Manager action affordances exposed on the row.
    pub actions: Vec<DocsPackManagerActionState>,
    /// Import/export continuity block.
    pub import_export_continuity: DocsPackImportExportContinuity,
    /// True when the row shows the signer identity.
    pub shows_signer: bool,
    /// True when the row shows the source channel.
    pub shows_channel: bool,
    /// True when the row shows the mirror source.
    pub shows_mirror_source: bool,
    /// True when the row shows the version range.
    pub shows_version_range: bool,
    /// True when the row shows the refresh state.
    pub shows_refresh_state: bool,
    /// True when the row shows the pin / offline posture.
    pub shows_pin_offline_posture: bool,
    /// True when the row keeps the signature state visible (never hidden).
    pub signature_state_visible: bool,
    /// True when an unavailable payload is disclosed rather than hidden.
    pub unavailable_payload_disclosed: bool,
    /// True when the row collapsed into an opaque cache badge; must be false.
    pub degraded_to_opaque_cache: bool,
    /// True when the row degraded to browser-only fallback wording; must be
    /// false for mirror / offline / air-gapped rows.
    pub browser_only_fallback_wording: bool,
    /// Disclosure note for unavailable / mirror / offline posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
    /// True when raw URLs, raw bodies, secrets, and provider payloads are
    /// excluded from this row projection.
    pub raw_boundary_material_excluded: bool,
}

impl DocsPackManagerRow {
    fn is_well_formed(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.manifest.pack_id.trim().is_empty()
            && !self.manifest.pack_revision_ref.trim().is_empty()
            && !self.manifest.display_label.trim().is_empty()
            && !self
                .manifest
                .version_range
                .min_inclusive_ref
                .trim()
                .is_empty()
            && !self
                .manifest
                .version_range
                .max_inclusive_ref
                .trim()
                .is_empty()
            && !self
                .manifest
                .signing
                .signing_authority_ref
                .trim()
                .is_empty()
            && self.manifest.manifest_schema_version >= 1
    }

    /// Whether the row keeps the manifest truth required by acceptance #1
    /// visible: signer, channel, mirror source, version range, refresh state,
    /// and pin / offline posture.
    fn shows_manifest_truth(&self) -> bool {
        self.shows_signer
            && self.shows_channel
            && self.shows_mirror_source
            && self.shows_version_range
            && self.shows_refresh_state
            && self.shows_pin_offline_posture
    }

    fn content_unavailable_locally(&self) -> bool {
        self.manifest
            .local_availability
            .content_unavailable_locally()
    }

    /// Whether an unavailable payload keeps its payload / signature state
    /// disclosed rather than hidden.
    fn unavailable_state_disclosed(&self) -> bool {
        if !self.content_unavailable_locally() {
            return true;
        }
        self.unavailable_payload_disclosed && self.signature_state_visible
    }

    /// Whether the row keeps mirror / offline posture first-class.
    fn is_mirror_or_offline(&self) -> bool {
        self.lifecycle_flow.is_mirror_or_offline()
            || self.manifest.pin_state == DocsPackPinState::PinnedOffline
            || self.manifest.local_availability == DocsPackLocalAvailability::MirrorOfflinePinned
            || self.manifest.source_class.is_mirror_class()
    }

    /// Whether mirror / offline rows avoid opaque-cache / browser-only wording.
    fn mirror_offline_first_class(&self) -> bool {
        if !self.is_mirror_or_offline() {
            return true;
        }
        !self.degraded_to_opaque_cache
            && !self.browser_only_fallback_wording
            && self.shows_mirror_source
    }

    fn actions_present(&self) -> BTreeSet<DocsPackManagerAction> {
        self.actions.iter().map(|state| state.action).collect()
    }

    fn pin_action_consistent(&self) -> bool {
        let actions = self.actions_present();
        let pinned = matches!(
            self.manifest.pin_state,
            DocsPackPinState::Pinned
                | DocsPackPinState::PinnedOffline
                | DocsPackPinState::PinnedCompatWindow
        );
        if pinned {
            actions.contains(&DocsPackManagerAction::Unpin)
                && !actions.contains(&DocsPackManagerAction::Pin)
        } else {
            actions.contains(&DocsPackManagerAction::Pin)
                && !actions.contains(&DocsPackManagerAction::Unpin)
        }
    }

    fn has_required_actions(&self) -> bool {
        let actions = self.actions_present();
        DocsPackManagerAction::REQUIRED_NON_PIN
            .iter()
            .all(|action| actions.contains(action))
            && self.pin_action_consistent()
    }
}

/// Per-profile projection proving a manager surface reuses the same packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackManagerProfileProjection {
    /// Manager profile.
    pub profile: DocsPackManagerProfile,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub packet_id_ref: String,
    /// Render timestamp.
    pub rendered_at: String,
    /// True when the surface preserves row identity verbatim.
    pub preserves_row_identity: bool,
    /// True when the surface shows signer / channel / mirror source.
    pub shows_signer_channel_mirror: bool,
    /// True when the surface shows pin / offline / refresh posture.
    pub shows_pin_offline_refresh: bool,
    /// True when the surface shows the version range.
    pub shows_version_range: bool,
    /// True when the surface preserves the import/export continuity block.
    pub preserves_import_export_continuity: bool,
    /// True when the surface preserves the lifecycle state.
    pub preserves_lifecycle_state: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl DocsPackManagerProfileProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.preserves_row_identity
            && self.shows_signer_channel_mirror
            && self.shows_pin_offline_refresh
            && self.shows_version_range
            && self.preserves_import_export_continuity
            && self.preserves_lifecycle_state
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Promotion state derived from the packet's validation findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackManagerPromotionState {
    /// Packet certifies the stable claim.
    Stable,
    /// Packet must remain narrowed below stable.
    NarrowedBelowStable,
    /// Packet blocks stable publication.
    BlocksStable,
}

impl DocsPackManagerPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackManagerFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`DocsPackManagerPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackManagerFindingKind {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity is incomplete.
    MissingPacketIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Packet declared no manager rows.
    MissingManagerRows,
    /// A manager row is incomplete.
    ManagerRowIncomplete,
    /// A manager row hides signer / channel / mirror / version / refresh / pin.
    ManagerRowHidesManifestTruth,
    /// An unavailable payload or signature state is hidden rather than disclosed.
    UnavailablePayloadHidden,
    /// A mirror / offline / air-gapped row degraded into opaque cache or
    /// browser-only fallback wording.
    MirrorOfflineDegraded,
    /// A required manager action is missing or pin/unpin is inconsistent.
    RequiredManagerActionMissing,
    /// A disabled / not-applicable action dropped its disclosed reason.
    ManagerActionReasonMissing,
    /// Import/export continuity is incomplete or not preserved on export.
    ImportExportContinuityLost,
    /// A lifecycle flow disagrees with its import provenance.
    LifecycleFlowOriginMismatch,
    /// Required lifecycle-flow coverage for a stable claim is missing.
    RequiredLifecycleFlowCoverageMissing,
    /// A required manager profile has no projection.
    MissingProfileProjection,
    /// A profile projection references an unknown packet or drops truth.
    ProfileProjectionDrift,
    /// Raw boundary material is present in the export.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl DocsPackManagerFindingKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingManagerRows => "missing_manager_rows",
            Self::ManagerRowIncomplete => "manager_row_incomplete",
            Self::ManagerRowHidesManifestTruth => "manager_row_hides_manifest_truth",
            Self::UnavailablePayloadHidden => "unavailable_payload_hidden",
            Self::MirrorOfflineDegraded => "mirror_offline_degraded",
            Self::RequiredManagerActionMissing => "required_manager_action_missing",
            Self::ManagerActionReasonMissing => "manager_action_reason_missing",
            Self::ImportExportContinuityLost => "import_export_continuity_lost",
            Self::LifecycleFlowOriginMismatch => "lifecycle_flow_origin_mismatch",
            Self::RequiredLifecycleFlowCoverageMissing => {
                "required_lifecycle_flow_coverage_missing"
            }
            Self::MissingProfileProjection => "missing_profile_projection",
            Self::ProfileProjectionDrift => "profile_projection_drift",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the docs-pack manager validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackManagerValidationFinding {
    /// Closed finding kind.
    pub finding_kind: DocsPackManagerFindingKind,
    /// Finding severity.
    pub severity: DocsPackManagerFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl DocsPackManagerValidationFinding {
    fn blocker(finding_kind: DocsPackManagerFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: DocsPackManagerFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`DocsPackManagerPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackManagerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Manager rows.
    pub rows: Vec<DocsPackManagerRow>,
    /// Per-profile projections.
    pub profile_projections: Vec<DocsPackManagerProfileProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

/// Export-safe docs-pack manager packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackManagerPacket {
    /// Record kind; must equal [`DOCS_PACK_MANAGER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_PACK_MANAGER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Manager rows.
    pub rows: Vec<DocsPackManagerRow>,
    /// Per-profile projections.
    pub profile_projections: Vec<DocsPackManagerProfileProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Derived promotion state.
    pub promotion_state: DocsPackManagerPromotionState,
    /// Validation findings.
    #[serde(default)]
    pub validation_findings: Vec<DocsPackManagerValidationFinding>,
}

impl DocsPackManagerPacket {
    /// Materializes the packet and records its derived findings and promotion
    /// state.
    pub fn materialize(input: DocsPackManagerPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DOCS_PACK_MANAGER_RECORD_KIND.to_owned(),
            schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            generated_at: input.generated_at,
            rows: input.rows,
            profile_projections: input.profile_projections,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            promotion_state: DocsPackManagerPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet's invariants, including the stored promotion
    /// state.
    pub fn validate(&self) -> Vec<DocsPackManagerValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker findings exist.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == DocsPackManagerFindingSeverity::Blocker)
    }

    /// Returns true when at least one projection preserves this packet for
    /// `profile`.
    pub fn has_projection_for(&self, profile: DocsPackManagerProfile) -> bool {
        self.profile_projections.iter().any(|projection| {
            projection.profile == profile && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique lifecycle-flow tokens carried across rows.
    pub fn lifecycle_flow_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lifecycle_flow);
        }
        set.into_iter().map(DocsPackLifecycleFlow::as_str).collect()
    }

    /// Wraps the packet in an export-safe support export.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DocsPackManagerSupportExport {
        DocsPackManagerSupportExport {
            record_kind: DOCS_PACK_MANAGER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs pack manager packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Docs-Pack Manager Rows And Manifest Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Promotion: `{}`\n",
            self.promotion_state.as_str()
        ));
        out.push_str(&format!(
            "- Rows: {} / Profiles: {}\n",
            self.rows.len(),
            self.profile_projections.len()
        ));
        out.push_str("\n## Managed packs\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** (`{}`): {} / {} / pin `{}` / refresh `{}`\n",
                row.manifest.display_label,
                row.lifecycle_flow.as_str(),
                row.manifest.source_class.as_str(),
                row.manifest.source_channel.as_str(),
                row.manifest.pin_state.as_str(),
                row.manifest.refresh_state.as_str(),
            ));
        }
        out.push_str("\n## Profiles\n\n");
        for profile in DocsPackManagerProfile::REQUIRED {
            let count = self
                .profile_projections
                .iter()
                .filter(|projection| projection.profile == profile)
                .count();
            out.push_str(&format!(
                "- `{}`: {} projection(s)\n",
                profile.as_str(),
                count
            ));
        }
        out
    }

    fn derived_findings(&self, check_promotion: bool) -> Vec<DocsPackManagerValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != DOCS_PACK_MANAGER_RECORD_KIND {
            findings.push(DocsPackManagerValidationFinding::blocker(
                DocsPackManagerFindingKind::WrongRecordKind,
                "record kind does not match the docs-pack manager contract",
            ));
        }
        if self.schema_version != DOCS_PACK_MANAGER_SCHEMA_VERSION {
            findings.push(DocsPackManagerValidationFinding::blocker(
                DocsPackManagerFindingKind::WrongSchemaVersion,
                "schema version does not match the docs-pack manager contract",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            findings.push(DocsPackManagerValidationFinding::blocker(
                DocsPackManagerFindingKind::MissingPacketIdentity,
                "packet identity is incomplete",
            ));
        }

        validate_source_contracts(self, &mut findings);

        if self.rows.is_empty() {
            findings.push(DocsPackManagerValidationFinding::blocker(
                DocsPackManagerFindingKind::MissingManagerRows,
                "packet must declare at least one manager row",
            ));
        }

        self.validate_rows(&mut findings);
        self.validate_lifecycle_coverage(&mut findings);
        self.validate_projections(&mut findings);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("docs pack manager packet serializes"),
        ) {
            findings.push(DocsPackManagerValidationFinding::blocker(
                DocsPackManagerFindingKind::RawBoundaryMaterialPresent,
                "export contains forbidden raw boundary material",
            ));
        }

        if check_promotion {
            let derived = promotion_state_for_findings(&findings);
            if self.promotion_state != derived {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::PromotionStateMismatch,
                    "stored promotion state disagrees with derived findings",
                ));
            }
        }

        findings
    }

    fn validate_rows(&self, findings: &mut Vec<DocsPackManagerValidationFinding>) {
        for row in &self.rows {
            if !row.is_well_formed() {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::ManagerRowIncomplete,
                    format!("manager row {} drops a required identity field", row.row_id),
                ));
            }
            if !row.raw_boundary_material_excluded {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::RawBoundaryMaterialPresent,
                    format!("manager row {} retains raw boundary material", row.row_id),
                ));
            }
            if !row.shows_manifest_truth() {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::ManagerRowHidesManifestTruth,
                    format!(
                        "manager row {} hides signer/channel/mirror/version/refresh/pin truth",
                        row.row_id
                    ),
                ));
            }
            if !row.unavailable_state_disclosed() {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::UnavailablePayloadHidden,
                    format!(
                        "manager row {} hides an unavailable payload or signature state",
                        row.row_id
                    ),
                ));
            }
            if !row.mirror_offline_first_class() {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::MirrorOfflineDegraded,
                    format!(
                        "manager row {} degraded a mirror/offline pack into opaque cache or browser-only wording",
                        row.row_id
                    ),
                ));
            }
            if !row.has_required_actions() {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::RequiredManagerActionMissing,
                    format!(
                        "manager row {} omits a required action or its pin/unpin posture is inconsistent",
                        row.row_id
                    ),
                ));
            }
            for action in &row.actions {
                if !action.has_required_reason() {
                    findings.push(DocsPackManagerValidationFinding::blocker(
                        DocsPackManagerFindingKind::ManagerActionReasonMissing,
                        format!(
                            "manager row {} action {} is disabled without a disclosed reason",
                            row.row_id,
                            action.action.as_str()
                        ),
                    ));
                }
            }
            if !row.import_export_continuity.is_continuous() {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::ImportExportContinuityLost,
                    format!(
                        "manager row {} loses docs-pack identity or lifecycle state across import/export",
                        row.row_id
                    ),
                ));
            }
            if !row
                .lifecycle_flow
                .allows_origin(row.import_export_continuity.import_origin)
            {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::LifecycleFlowOriginMismatch,
                    format!(
                        "manager row {} lifecycle flow {} disagrees with import origin {}",
                        row.row_id,
                        row.lifecycle_flow.as_str(),
                        row.import_export_continuity.import_origin.as_str()
                    ),
                ));
            }
        }
    }

    fn validate_lifecycle_coverage(&self, findings: &mut Vec<DocsPackManagerValidationFinding>) {
        let present: HashSet<DocsPackLifecycleFlow> =
            self.rows.iter().map(|row| row.lifecycle_flow).collect();
        for required in DocsPackLifecycleFlow::REQUIRED {
            if !present.contains(&required) {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::RequiredLifecycleFlowCoverageMissing,
                    format!(
                        "no managed pack exercises the {} lifecycle flow",
                        required.as_str()
                    ),
                ));
                return;
            }
        }
    }

    fn validate_projections(&self, findings: &mut Vec<DocsPackManagerValidationFinding>) {
        let present: BTreeSet<DocsPackManagerProfile> = self
            .profile_projections
            .iter()
            .map(|projection| projection.profile)
            .collect();
        for required in DocsPackManagerProfile::REQUIRED {
            if !present.contains(&required) {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::MissingProfileProjection,
                    format!(
                        "no projection reuses the manager packet on the {} profile",
                        required.as_str()
                    ),
                ));
                break;
            }
        }
        for projection in &self.profile_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(DocsPackManagerValidationFinding::blocker(
                    DocsPackManagerFindingKind::ProfileProjectionDrift,
                    format!(
                        "projection {} on {} dropped shared manager truth",
                        projection.projection_ref,
                        projection.profile.as_str()
                    ),
                ));
            }
        }
    }
}

/// Support-export wrapper preserving the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackManagerSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Exported packet id.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Exact packet preserved by the export.
    pub export_packet: DocsPackManagerPacket,
}

impl DocsPackManagerSupportExport {
    /// Returns true when the export preserves the same packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DOCS_PACK_MANAGER_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DOCS_PACK_MANAGER_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in docs-pack manager export.
#[derive(Debug)]
pub enum DocsPackManagerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export is not export-safe or its packet failed validation.
    Validation(Vec<DocsPackManagerValidationFinding>),
    /// Support export wrapper is not export-safe.
    NotExportSafe,
}

impl fmt::Display for DocsPackManagerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "docs pack manager export parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs pack manager export failed validation: {tokens}"
                )
            }
            Self::NotExportSafe => {
                write!(
                    formatter,
                    "docs pack manager export wrapper is not export-safe"
                )
            }
        }
    }
}

impl Error for DocsPackManagerArtifactError {}

/// Returns the seeded stable docs-pack manager packet input.
pub fn seeded_stable_docs_pack_manager_input() -> DocsPackManagerPacketInput {
    seed::seeded_input()
}

/// Materializes the checked-in stable docs-pack manager packet.
///
/// # Errors
///
/// Returns an error when the seeded packet fails its own stable invariants.
pub fn current_stable_docs_pack_manager_packet(
) -> Result<DocsPackManagerPacket, DocsPackManagerArtifactError> {
    let packet = DocsPackManagerPacket::materialize(seeded_stable_docs_pack_manager_input());
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DocsPackManagerArtifactError::Validation(findings))
    }
}

/// Reads and validates the checked-in stable support export.
///
/// # Errors
///
/// Returns an error when the checked artifact fails to parse, is not
/// export-safe, or its packet fails validation.
pub fn current_stable_docs_pack_manager_export(
) -> Result<DocsPackManagerSupportExport, DocsPackManagerArtifactError> {
    let export: DocsPackManagerSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity/support_export.json"
    )))
    .map_err(DocsPackManagerArtifactError::SupportExport)?;
    let findings = export.export_packet.validate();
    if !findings.is_empty() {
        return Err(DocsPackManagerArtifactError::Validation(findings));
    }
    if !export.is_export_safe() {
        return Err(DocsPackManagerArtifactError::NotExportSafe);
    }
    Ok(export)
}

fn validate_source_contracts(
    packet: &DocsPackManagerPacket,
    findings: &mut Vec<DocsPackManagerValidationFinding>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    if !refs.contains(DOCS_PACK_MANAGER_SCHEMA_REF) || !refs.contains(DOCS_PACK_MANAGER_DOC_REF) {
        findings.push(DocsPackManagerValidationFinding::blocker(
            DocsPackManagerFindingKind::MissingSourceContracts,
            "source contract refs omit the schema or contract doc",
        ));
    }
}

fn promotion_state_for_findings(
    findings: &[DocsPackManagerValidationFinding],
) -> DocsPackManagerPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == DocsPackManagerFindingSeverity::Blocker)
    {
        DocsPackManagerPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == DocsPackManagerFindingSeverity::Warning)
    {
        DocsPackManagerPromotionState::NarrowedBelowStable
    } else {
        DocsPackManagerPromotionState::Stable
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

mod seed {
    use super::*;
    use crate::{
        DocsPackMirrorLineage, DocsPackMirrorState, DocsPackPublishableState,
        DocsPackSignatureStatus, DocsPackSignerClass, DocsPackSigningBlock, DocsPackSourceClass,
        DocsPackVersionRange,
    };

    const PACKET_ID: &str = "packet:docs_pack_manager:001";

    pub(super) fn seeded_input() -> DocsPackManagerPacketInput {
        DocsPackManagerPacketInput {
            packet_id: PACKET_ID.to_owned(),
            surface_label: "workflow:docs_pack_manager_rows_and_import_export_continuity:stable"
                .to_owned(),
            generated_at: "2026-06-26T00:00:00Z".to_owned(),
            rows: rows(),
            profile_projections: projections(),
            source_contract_refs: vec![
                DOCS_PACK_MANAGER_SCHEMA_REF.to_owned(),
                DOCS_PACK_MANAGER_DOC_REF.to_owned(),
                DOCS_PACK_MANAGER_ARTIFACT_REF.to_owned(),
                DOCS_PACK_MANAGER_SUMMARY_REF.to_owned(),
            ],
            redaction_class_token: "metadata_safe_default".to_owned(),
        }
    }

    fn signing(
        status: DocsPackSignatureStatus,
        signer: DocsPackSignerClass,
        authority: &str,
        digest: Option<&str>,
    ) -> DocsPackSigningBlock {
        DocsPackSigningBlock {
            signature_status: status,
            signer_class: signer,
            signing_authority_ref: authority.to_owned(),
            signing_chain_digest: digest.map(str::to_owned),
        }
    }

    fn version_range(min: &str, max: &str) -> DocsPackVersionRange {
        DocsPackVersionRange {
            min_inclusive_ref: min.to_owned(),
            max_inclusive_ref: max.to_owned(),
        }
    }

    fn not_applicable_lineage() -> DocsPackMirrorLineage {
        DocsPackMirrorLineage {
            mirror_state: DocsPackMirrorState::NotApplicable,
            mirror_of_pack_id: None,
            upstream_revision_ref: None,
            predecessor_revision_ref: None,
            air_gapped_origin_label: None,
            offline_expiration_at: None,
        }
    }

    fn required_actions(pinned: bool) -> Vec<DocsPackManagerActionState> {
        let pin_state = if pinned {
            action(
                DocsPackManagerAction::Unpin,
                DocsPackManagerActionAvailability::Available,
                None,
            )
        } else {
            action(
                DocsPackManagerAction::Pin,
                DocsPackManagerActionAvailability::Available,
                None,
            )
        };
        vec![
            pin_state,
            action(
                DocsPackManagerAction::Refresh,
                DocsPackManagerActionAvailability::Available,
                None,
            ),
            action(
                DocsPackManagerAction::Remove,
                DocsPackManagerActionAvailability::RequiresConfirmation,
                None,
            ),
            action(
                DocsPackManagerAction::ChangeMirrorSource,
                DocsPackManagerActionAvailability::Available,
                None,
            ),
            action(
                DocsPackManagerAction::SetOfflineAvailability,
                DocsPackManagerActionAvailability::Available,
                None,
            ),
            action(
                DocsPackManagerAction::ExportPack,
                DocsPackManagerActionAvailability::Available,
                None,
            ),
        ]
    }

    fn action(
        action: DocsPackManagerAction,
        availability: DocsPackManagerActionAvailability,
        reason: Option<&str>,
    ) -> DocsPackManagerActionState {
        DocsPackManagerActionState {
            action,
            availability,
            disabled_reason: reason.map(str::to_owned),
        }
    }

    fn continuity(
        id: &str,
        origin: DocsPackImportOrigin,
        import_bundle: Option<&str>,
        imported_at: Option<&str>,
        export_bundle: Option<&str>,
        token: &str,
    ) -> DocsPackImportExportContinuity {
        DocsPackImportExportContinuity {
            continuity_id: id.to_owned(),
            import_origin: origin,
            import_bundle_ref: import_bundle.map(str::to_owned),
            imported_at: imported_at.map(str::to_owned),
            export_bundle_ref: export_bundle.map(str::to_owned),
            continuity_token: token.to_owned(),
            preserves_identity_on_export: true,
            preserves_lifecycle_state_on_export: true,
        }
    }

    fn rows() -> Vec<DocsPackManagerRow> {
        vec![
            local_only_row(),
            mirrored_row(),
            managed_row(),
            air_gapped_row(),
            unavailable_row(),
        ]
    }

    fn local_only_row() -> DocsPackManagerRow {
        DocsPackManagerRow {
            row_id: "manager-row:project-docs".to_owned(),
            manifest: DocsPackManifest {
                pack_id: "pack:workspace-project-docs".to_owned(),
                pack_revision_ref: "rev:project-docs@workspace-head".to_owned(),
                display_label: "Workspace project docs".to_owned(),
                source_class: DocsPackSourceClass::ProjectDocs,
                source_channel: DocsPackChannel::Stable,
                signing: signing(
                    DocsPackSignatureStatus::SignedAndVerified,
                    DocsPackSignerClass::FirstPartyProject,
                    "authority:first-party-project",
                    Some("digest:project-docs-chain"),
                ),
                version_range: version_range("rev:project-docs@1", "rev:project-docs@head"),
                refresh_state: DocsPackRefreshState::AuthoritativeLive,
                last_refresh_at: Some("2026-06-26T00:00:00Z".to_owned()),
                mirror_lineage: not_applicable_lineage(),
                pin_state: DocsPackPinState::Unpinned,
                local_availability: DocsPackLocalAvailability::AvailableLocal,
                publishable_state: DocsPackPublishableState::Publishable,
                publishable_blocking_reasons: Vec::new(),
                manifest_schema_version: 1,
                disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
            lifecycle_flow: DocsPackLifecycleFlow::LocalOnly,
            pack_size_bytes: Some(1_048_576),
            document_count: Some(214),
            last_successful_refresh_at: Some("2026-06-26T00:00:00Z".to_owned()),
            last_refresh_attempt_at: Some("2026-06-26T00:00:00Z".to_owned()),
            actions: required_actions(false),
            import_export_continuity: continuity(
                "continuity:project-docs",
                DocsPackImportOrigin::FreshlyInstalled,
                None,
                None,
                Some("export:project-docs-bundle"),
                "token:project-docs",
            ),
            shows_signer: true,
            shows_channel: true,
            shows_mirror_source: true,
            shows_version_range: true,
            shows_refresh_state: true,
            shows_pin_offline_posture: true,
            signature_state_visible: true,
            unavailable_payload_disclosed: true,
            degraded_to_opaque_cache: false,
            browser_only_fallback_wording: false,
            disclosure_note: None,
            raw_boundary_material_excluded: true,
        }
    }

    fn mirrored_row() -> DocsPackManagerRow {
        DocsPackManagerRow {
            row_id: "manager-row:std-mirror".to_owned(),
            manifest: DocsPackManifest {
                pack_id: "pack:std-mirror".to_owned(),
                pack_revision_ref: "rev:std-mirror@1.84.0".to_owned(),
                display_label: "Standard library mirror".to_owned(),
                source_class: DocsPackSourceClass::MirroredOfficialDocs,
                source_channel: DocsPackChannel::Stable,
                signing: signing(
                    DocsPackSignatureStatus::SignedAndVerified,
                    DocsPackSignerClass::OfficialUpstreamMirror,
                    "authority:official-upstream-mirror",
                    Some("digest:std-mirror-chain"),
                ),
                version_range: version_range("rev:std-mirror@1.83.0", "rev:std-mirror@1.84.0"),
                refresh_state: DocsPackRefreshState::WarmCached,
                last_refresh_at: Some("2026-06-25T00:00:00Z".to_owned()),
                mirror_lineage: DocsPackMirrorLineage {
                    mirror_state: DocsPackMirrorState::Continuous,
                    mirror_of_pack_id: Some("upstream:std-docs".to_owned()),
                    upstream_revision_ref: Some("upstream-rev:std@1.84.0".to_owned()),
                    predecessor_revision_ref: Some("rev:std-mirror@1.83.0".to_owned()),
                    air_gapped_origin_label: None,
                    offline_expiration_at: Some("2026-07-26T00:00:00Z".to_owned()),
                },
                pin_state: DocsPackPinState::PinnedOffline,
                local_availability: DocsPackLocalAvailability::MirrorOfflinePinned,
                publishable_state: DocsPackPublishableState::Publishable,
                publishable_blocking_reasons: Vec::new(),
                manifest_schema_version: 1,
                disclosure_note: Some(
                    "Mirror is pinned for offline use; warm cache within its window.".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            lifecycle_flow: DocsPackLifecycleFlow::Mirrored,
            pack_size_bytes: Some(8_388_608),
            document_count: Some(1_902),
            last_successful_refresh_at: Some("2026-06-25T00:00:00Z".to_owned()),
            last_refresh_attempt_at: Some("2026-06-25T00:00:00Z".to_owned()),
            actions: required_actions(true),
            import_export_continuity: continuity(
                "continuity:std-mirror",
                DocsPackImportOrigin::MirroredSync,
                Some("import:std-mirror-sync"),
                Some("2026-06-25T00:00:00Z"),
                Some("export:std-mirror-bundle"),
                "token:std-mirror",
            ),
            shows_signer: true,
            shows_channel: true,
            shows_mirror_source: true,
            shows_version_range: true,
            shows_refresh_state: true,
            shows_pin_offline_posture: true,
            signature_state_visible: true,
            unavailable_payload_disclosed: true,
            degraded_to_opaque_cache: false,
            browser_only_fallback_wording: false,
            disclosure_note: Some(
                "Offline-pinned mirror stays attributable and refreshable.".to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn managed_row() -> DocsPackManagerRow {
        DocsPackManagerRow {
            row_id: "manager-row:enterprise-cookbook".to_owned(),
            manifest: DocsPackManifest {
                pack_id: "pack:enterprise-cookbook".to_owned(),
                pack_revision_ref: "rev:enterprise-cookbook@2.3.1".to_owned(),
                display_label: "Enterprise cookbook".to_owned(),
                source_class: DocsPackSourceClass::CuratedKnowledgePack,
                source_channel: DocsPackChannel::Enterprise,
                signing: signing(
                    DocsPackSignatureStatus::SignedAndVerified,
                    DocsPackSignerClass::OperatorCurated,
                    "authority:operator-curated",
                    Some("digest:enterprise-cookbook-chain"),
                ),
                version_range: version_range(
                    "rev:enterprise-cookbook@2.0.0",
                    "rev:enterprise-cookbook@2.3.1",
                ),
                refresh_state: DocsPackRefreshState::WarmCached,
                last_refresh_at: Some("2026-06-20T00:00:00Z".to_owned()),
                mirror_lineage: not_applicable_lineage(),
                pin_state: DocsPackPinState::PinnedCompatWindow,
                local_availability: DocsPackLocalAvailability::AvailableLocal,
                publishable_state: DocsPackPublishableState::Publishable,
                publishable_blocking_reasons: Vec::new(),
                manifest_schema_version: 1,
                disclosure_note: Some(
                    "Managed pack pinned within its declared compatibility window.".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            lifecycle_flow: DocsPackLifecycleFlow::Managed,
            pack_size_bytes: Some(2_097_152),
            document_count: Some(431),
            last_successful_refresh_at: Some("2026-06-20T00:00:00Z".to_owned()),
            last_refresh_attempt_at: Some("2026-06-20T00:00:00Z".to_owned()),
            actions: required_actions(true),
            import_export_continuity: continuity(
                "continuity:enterprise-cookbook",
                DocsPackImportOrigin::OperatorManaged,
                Some("import:enterprise-distribution-2026q2"),
                Some("2026-06-20T00:00:00Z"),
                Some("export:enterprise-cookbook-bundle"),
                "token:enterprise-cookbook",
            ),
            shows_signer: true,
            shows_channel: true,
            shows_mirror_source: true,
            shows_version_range: true,
            shows_refresh_state: true,
            shows_pin_offline_posture: true,
            signature_state_visible: true,
            unavailable_payload_disclosed: true,
            degraded_to_opaque_cache: false,
            browser_only_fallback_wording: false,
            disclosure_note: None,
            raw_boundary_material_excluded: true,
        }
    }

    fn air_gapped_row() -> DocsPackManagerRow {
        DocsPackManagerRow {
            row_id: "manager-row:support-runbook".to_owned(),
            manifest: DocsPackManifest {
                pack_id: "pack:support-runbook".to_owned(),
                pack_revision_ref: "rev:support-runbook@2026-q2".to_owned(),
                display_label: "Support runbook pack".to_owned(),
                source_class: DocsPackSourceClass::SupportRunbook,
                source_channel: DocsPackChannel::Enterprise,
                signing: signing(
                    DocsPackSignatureStatus::SignedAndVerified,
                    DocsPackSignerClass::SupportPipeline,
                    "authority:support-pipeline",
                    Some("digest:support-runbook-chain"),
                ),
                version_range: version_range(
                    "rev:support-runbook@2026-q1",
                    "rev:support-runbook@2026-q2",
                ),
                refresh_state: DocsPackRefreshState::DegradedCached,
                last_refresh_at: Some("2026-04-01T00:00:00Z".to_owned()),
                mirror_lineage: DocsPackMirrorLineage {
                    mirror_state: DocsPackMirrorState::NotApplicable,
                    mirror_of_pack_id: None,
                    upstream_revision_ref: None,
                    predecessor_revision_ref: None,
                    air_gapped_origin_label: Some("enterprise distribution 2026 Q2".to_owned()),
                    offline_expiration_at: Some("2026-10-01T00:00:00Z".to_owned()),
                },
                pin_state: DocsPackPinState::PinnedOffline,
                local_availability: DocsPackLocalAvailability::MirrorOfflinePinned,
                publishable_state: DocsPackPublishableState::Publishable,
                publishable_blocking_reasons: Vec::new(),
                manifest_schema_version: 1,
                disclosure_note: Some(
                    "Air-gapped import; degraded cache disclosed and still attributable."
                        .to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            lifecycle_flow: DocsPackLifecycleFlow::AirGapped,
            pack_size_bytes: Some(524_288),
            document_count: Some(88),
            last_successful_refresh_at: Some("2026-04-01T00:00:00Z".to_owned()),
            last_refresh_attempt_at: Some("2026-06-01T00:00:00Z".to_owned()),
            actions: vec![
                action(
                    DocsPackManagerAction::Unpin,
                    DocsPackManagerActionAvailability::Available,
                    None,
                ),
                action(
                    DocsPackManagerAction::Refresh,
                    DocsPackManagerActionAvailability::DisabledUnavailable,
                    Some("Air-gapped pack cannot refresh online; re-import to update."),
                ),
                action(
                    DocsPackManagerAction::Remove,
                    DocsPackManagerActionAvailability::RequiresConfirmation,
                    None,
                ),
                action(
                    DocsPackManagerAction::ChangeMirrorSource,
                    DocsPackManagerActionAvailability::DisabledByPolicy,
                    Some("Mirror source is fixed by the air-gapped distribution policy."),
                ),
                action(
                    DocsPackManagerAction::SetOfflineAvailability,
                    DocsPackManagerActionAvailability::Available,
                    None,
                ),
                action(
                    DocsPackManagerAction::ExportPack,
                    DocsPackManagerActionAvailability::Available,
                    None,
                ),
            ],
            import_export_continuity: continuity(
                "continuity:support-runbook",
                DocsPackImportOrigin::AirGappedSideload,
                Some("import:support-runbook-media-2026q2"),
                Some("2026-04-01T00:00:00Z"),
                Some("export:support-runbook-bundle"),
                "token:support-runbook",
            ),
            shows_signer: true,
            shows_channel: true,
            shows_mirror_source: true,
            shows_version_range: true,
            shows_refresh_state: true,
            shows_pin_offline_posture: true,
            signature_state_visible: true,
            unavailable_payload_disclosed: true,
            degraded_to_opaque_cache: false,
            browser_only_fallback_wording: false,
            disclosure_note: Some(
                "Air-gapped pack stays first-class: attributable, inspectable, revocable."
                    .to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn unavailable_row() -> DocsPackManagerRow {
        DocsPackManagerRow {
            row_id: "manager-row:extension-pack-unavailable".to_owned(),
            manifest: DocsPackManifest {
                pack_id: "pack:extension-docs".to_owned(),
                pack_revision_ref: "rev:extension-docs@1.4.0".to_owned(),
                display_label: "Extension docs pack".to_owned(),
                source_class: DocsPackSourceClass::ExtensionDocsPack,
                source_channel: DocsPackChannel::Beta,
                signing: signing(
                    DocsPackSignatureStatus::SignedButUnverified,
                    DocsPackSignerClass::PermittedPublisher,
                    "authority:permitted-publisher",
                    Some("digest:extension-docs-chain"),
                ),
                version_range: version_range(
                    "rev:extension-docs@1.0.0",
                    "rev:extension-docs@1.4.0",
                ),
                refresh_state: DocsPackRefreshState::Unverified,
                last_refresh_at: None,
                mirror_lineage: not_applicable_lineage(),
                pin_state: DocsPackPinState::Pinned,
                local_availability: DocsPackLocalAvailability::UnavailableDisclosed,
                publishable_state: DocsPackPublishableState::Blocked,
                publishable_blocking_reasons: vec!["signature_unverified".to_owned()],
                manifest_schema_version: 1,
                disclosure_note: Some(
                    "Payload unavailable locally; signature unverified and disclosed.".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            lifecycle_flow: DocsPackLifecycleFlow::Managed,
            pack_size_bytes: None,
            document_count: None,
            last_successful_refresh_at: None,
            last_refresh_attempt_at: Some("2026-06-10T00:00:00Z".to_owned()),
            actions: vec![
                action(
                    DocsPackManagerAction::Unpin,
                    DocsPackManagerActionAvailability::Available,
                    None,
                ),
                action(
                    DocsPackManagerAction::Refresh,
                    DocsPackManagerActionAvailability::DisabledUnavailable,
                    Some("Pack payload is unavailable locally; re-acquire before refresh."),
                ),
                action(
                    DocsPackManagerAction::Remove,
                    DocsPackManagerActionAvailability::Available,
                    None,
                ),
                action(
                    DocsPackManagerAction::ChangeMirrorSource,
                    DocsPackManagerActionAvailability::NotApplicable,
                    Some("Extension pack does not resolve through a mirror source."),
                ),
                action(
                    DocsPackManagerAction::SetOfflineAvailability,
                    DocsPackManagerActionAvailability::DisabledUnavailable,
                    Some("Cannot pin an unavailable payload for offline use."),
                ),
                action(
                    DocsPackManagerAction::ExportPack,
                    DocsPackManagerActionAvailability::DisabledUnavailable,
                    Some("Export records identity only; payload bytes are unavailable."),
                ),
            ],
            import_export_continuity: continuity(
                "continuity:extension-docs",
                DocsPackImportOrigin::ImportedBundle,
                Some("import:extension-docs-bundle"),
                Some("2026-05-01T00:00:00Z"),
                Some("export:extension-docs-identity"),
                "token:extension-docs",
            ),
            shows_signer: true,
            shows_channel: true,
            shows_mirror_source: true,
            shows_version_range: true,
            shows_refresh_state: true,
            shows_pin_offline_posture: true,
            signature_state_visible: true,
            unavailable_payload_disclosed: true,
            degraded_to_opaque_cache: false,
            browser_only_fallback_wording: false,
            disclosure_note: Some(
                "Unavailable payload and unverified signature stay visible, not hidden.".to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn projections() -> Vec<DocsPackManagerProfileProjection> {
        DocsPackManagerProfile::REQUIRED
            .into_iter()
            .map(projection)
            .collect()
    }

    fn projection(profile: DocsPackManagerProfile) -> DocsPackManagerProfileProjection {
        DocsPackManagerProfileProjection {
            profile,
            projection_ref: format!("projection:{}", profile.as_str()),
            packet_id_ref: PACKET_ID.to_owned(),
            rendered_at: "2026-06-26T00:00:00Z".to_owned(),
            preserves_row_identity: true,
            shows_signer_channel_mirror: true,
            shows_pin_offline_refresh: true,
            shows_version_range: true,
            preserves_import_export_continuity: true,
            preserves_lifecycle_state: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }
}
