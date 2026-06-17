//! Per-desktop-profile qualification family for native-desktop integration,
//! handler ownership, reopen fidelity, OS-notification privacy, and
//! external-path recovery.
//!
//! The native-desktop matrix in [`crate::m5_native_desktop`] names every claimed
//! system-entry, handler, reopen, notification, and external-path *surface*. A
//! green matrix proves the contract exists; it does not, on its own, prove that
//! the contract is currently exercised on each desktop profile Aureline ships.
//! This module turns native-desktop behavior into a claimable, auto-narrowing
//! qualification family: one machine-readable row per claimed desktop profile
//! (platform × delivery channel) that certifies the canonical qualification
//! dimensions against the platform-conformance drill corpus, and a claim packet
//! that narrows the published claim the moment a row goes stale, missing, or
//! red.
//!
//! Track invariant the family enforces: OS-level entry and reopen never bypass
//! trust, profile, tenant, or policy evaluation; channel/build ownership is
//! inspectable; notification/badge/progress signals derive from durable
//! objects; and missing roots, locked stores, or topology drift preserve user
//! context through truthful placeholders and recovery actions — and every one
//! of those promises is qualified *per claimed desktop profile* rather than
//! implied by a single "OS integration supported" claim or by another platform
//! or channel passing nearby.
//!
//! Each claimed profile declares a binding for each of the canonical
//! qualification dimensions:
//!
//! - `channel_build_ownership`
//! - `protocol_handler_ownership`
//! - `file_association_ownership`
//! - `reopen_fidelity`
//! - `notification_privacy`
//! - `external_root_recovery`
//! - `store_lock_recovery`
//!
//! and each dimension binds the platform-conformance drill it is qualified by:
//!
//! - `channel_ownership_audit`
//! - `handler_conflict`
//! - `wrong_target_reopen`
//! - `lock_screen_privacy`
//! - `missing_root_recovery`
//! - `store_lock`
//!
//! The resulting [`NativeDesktopQualificationReport`] is the canonical truth
//! object for the native-desktop qualification lane. From it the
//! [`NativeDesktopClaimPacket`] derives the auto-narrowing claim scope that
//! Help/About, install/update, docs, support packets, evaluation materials, and
//! the shiproom and release-center surfaces ingest instead of maintaining
//! parallel summaries.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every claimed profile declares a binding for each of the seven canonical
//!    dimensions, and every required dimension is qualified on at least one
//!    claimed profile.
//! 2. Every profile carries a platform, a delivery channel, a channel/build
//!    owner, a trust checkpoint, a non-empty continuity note, a downgrade rule,
//!    and a flag asserting it rides the governed qualification harness; a
//!    missing field, or a profile that drives its own qualification off the
//!    harness, is a blocker.
//! 3. A qualified dimension carries the proof it requires — the drill it is
//!    qualified by, an evidence pack, and an evidence pack that names *this*
//!    profile so a row can never borrow another profile's or channel's proof.
//!    A failed dimension is a blocker, and each failure stays a distinct class:
//!    an unprovable channel owner, a protocol-handler conflict, a
//!    file-association conflict, a wrong-target reopen, a lock-screen leak, a
//!    silent loss on a missing root, and a store-lock dead end are never
//!    collapsed into one generic finding.
//! 4. Stale evidence on a marketed profile is a blocker, so release tooling can
//!    narrow the profile instead of publishing it as implicitly stable.
//! 5. The published claim for each profile is *derived* from its dimension
//!    qualification — a profile is published only when every marketed dimension
//!    is qualified with fresh evidence, narrowed when some dimensions are
//!    narrowed or red, and withheld when none qualify — so the claim can never
//!    be greener than the proof.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under
//! `fixtures/platform/m5-native-desktop-qualification/` are bit-for-bit equal to
//! the seeded report returned by [`seeded_native_desktop_qualification`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every qualification record.
pub const QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every qualification record.
pub const QUALIFICATION_SHARED_CONTRACT_REF: &str = "shell:m5_native_desktop_qualification:v1";

/// Stable record kind for [`NativeDesktopQualificationReport`] payloads.
pub const QUALIFICATION_REPORT_RECORD_KIND: &str =
    "shell_m5_native_desktop_qualification_report_record";

/// Stable record kind for [`NativeDesktopProfileRow`] payloads.
pub const QUALIFICATION_PROFILE_RECORD_KIND: &str =
    "shell_m5_native_desktop_qualification_profile_record";

/// Stable record kind for [`NativeDesktopQualificationSupportExport`] payloads.
pub const QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_native_desktop_qualification_support_export_record";

/// Stable record kind for [`NativeDesktopClaimPacket`] payloads.
pub const QUALIFICATION_CLAIM_PACKET_RECORD_KIND: &str =
    "shell_m5_native_desktop_qualification_claim_packet_record";

/// Stable report id quoted across surfaces.
pub const QUALIFICATION_REPORT_ID: &str = "shell:m5_native_desktop_qualification:v1";

/// Stable support-export id quoted in the published wrapper.
pub const QUALIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-native-desktop-qualification:001";

/// Stable claim-packet id quoted in the shiproom packet.
pub const QUALIFICATION_CLAIM_PACKET_ID: &str = "shiproom:m5-native-desktop-claim-packet:001";

/// Source schema ref for the canonical qualification contract.
pub const QUALIFICATION_SOURCE_SCHEMA_REF: &str =
    "schemas/platform/m5-native-desktop-qualification.schema.json";

/// Path of the published markdown qualification matrix artifact.
pub const QUALIFICATION_PUBLISHED_REPORT_REF: &str =
    "artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md";

/// Path of the published companion doc.
pub const QUALIFICATION_PUBLISHED_DOC_REF: &str = "docs/m5/native-desktop-qualification.md";

/// Path of the published shiproom claim packet.
pub const QUALIFICATION_CLAIM_PACKET_REF: &str =
    "artifacts/shiproom/m5-native-desktop-claim-packet/m5_native_desktop_claim_packet.md";

/// Cross-link to the native-desktop matrix this family certifies.
pub const QUALIFICATION_MATRIX_REPORT_REF: &str = "artifacts/platform/m5-native-desktop-matrix.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-16T00:00:00Z";

/// A desktop platform a profile is scoped to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesktopPlatform {
    /// macOS desktop platform.
    Macos,
    /// Windows desktop platform.
    Windows,
    /// Linux desktop platform.
    Linux,
}

impl DesktopPlatform {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }
}

/// The delivery channel a profile certifies. A claimed profile is a
/// `(platform, channel)` pair: each channel needs its own current proof and
/// cannot inherit another channel's qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryChannel {
    /// General-availability stable channel.
    Stable,
    /// Pre-release beta channel.
    Beta,
    /// Centrally managed fleet deployment.
    ManagedFleet,
    /// Portable build that does not register OS-level handlers.
    Portable,
}

impl DeliveryChannel {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::ManagedFleet => "managed_fleet",
            Self::Portable => "portable",
        }
    }
}

/// How the channel/build owns the OS-level registration for a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipKind {
    /// Each channel owns its own registration; side-by-side installs do not
    /// collide.
    ChannelScopedOwner,
    /// A shared default is arbitrated by explicit user or admin choice.
    SharedDefaultArbitrated,
    /// A managed fleet deployment owns the registration centrally.
    ManagedFleetOwned,
    /// A portable build does not register an OS-level handler.
    PortableNonRegistering,
}

impl OwnershipKind {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelScopedOwner => "channel_scoped_owner",
            Self::SharedDefaultArbitrated => "shared_default_arbitrated",
            Self::ManagedFleetOwned => "managed_fleet_owned",
            Self::PortableNonRegistering => "portable_non_registering",
        }
    }
}

/// One of the seven canonical native-desktop qualification dimensions every
/// claimed profile must certify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDimension {
    /// The channel/build that owns the OS registration is inspectable; no
    /// handler can be silently taken over.
    ChannelBuildOwnership,
    /// The protocol/deep-link scheme handler is owned without conflict.
    ProtocolHandlerOwnership,
    /// The file-type association is owned without conflict.
    FileAssociationOwnership,
    /// A reopen path lands on the exact target, and a wrong target recovers.
    ReopenFidelity,
    /// OS notifications, badges, and progress stay privacy-safe on shared and
    /// lock surfaces.
    NotificationPrivacy,
    /// A removable volume or network share that disappears preserves context
    /// through a truthful placeholder and recovery action.
    ExternalRootRecovery,
    /// A locked credential or trust store preserves the pending action and
    /// recovers truthfully.
    StoreLockRecovery,
}

impl QualificationDimension {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelBuildOwnership => "channel_build_ownership",
            Self::ProtocolHandlerOwnership => "protocol_handler_ownership",
            Self::FileAssociationOwnership => "file_association_ownership",
            Self::ReopenFidelity => "reopen_fidelity",
            Self::NotificationPrivacy => "notification_privacy",
            Self::ExternalRootRecovery => "external_root_recovery",
            Self::StoreLockRecovery => "store_lock_recovery",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ChannelBuildOwnership => "Channel / build ownership",
            Self::ProtocolHandlerOwnership => "Protocol-handler ownership",
            Self::FileAssociationOwnership => "File-association ownership",
            Self::ReopenFidelity => "Reopen fidelity",
            Self::NotificationPrivacy => "Notification privacy",
            Self::ExternalRootRecovery => "External-root recovery",
            Self::StoreLockRecovery => "Store-lock recovery",
        }
    }

    /// Returns the seven required dimensions in canonical order.
    pub const fn required_dimensions() -> [Self; 7] {
        [
            Self::ChannelBuildOwnership,
            Self::ProtocolHandlerOwnership,
            Self::FileAssociationOwnership,
            Self::ReopenFidelity,
            Self::NotificationPrivacy,
            Self::ExternalRootRecovery,
            Self::StoreLockRecovery,
        ]
    }

    /// The platform-conformance drill this dimension is qualified by.
    pub const fn required_drill(self) -> QualificationDrill {
        match self {
            Self::ChannelBuildOwnership => QualificationDrill::ChannelOwnershipAudit,
            Self::ProtocolHandlerOwnership => QualificationDrill::HandlerConflict,
            Self::FileAssociationOwnership => QualificationDrill::HandlerConflict,
            Self::ReopenFidelity => QualificationDrill::WrongTargetReopen,
            Self::NotificationPrivacy => QualificationDrill::LockScreenPrivacy,
            Self::ExternalRootRecovery => QualificationDrill::MissingRootRecovery,
            Self::StoreLockRecovery => QualificationDrill::StoreLock,
        }
    }

    /// The distinct failure class this dimension fails into.
    pub const fn canonical_failure_mode(self) -> QualificationFailureMode {
        match self {
            Self::ChannelBuildOwnership => QualificationFailureMode::OwnershipUnprovable,
            Self::ProtocolHandlerOwnership => QualificationFailureMode::ProtocolHandlerConflict,
            Self::FileAssociationOwnership => QualificationFailureMode::FileAssociationConflict,
            Self::ReopenFidelity => QualificationFailureMode::WrongTargetReopen,
            Self::NotificationPrivacy => QualificationFailureMode::LockScreenLeak,
            Self::ExternalRootRecovery => QualificationFailureMode::MissingRootSilentLoss,
            Self::StoreLockRecovery => QualificationFailureMode::StoreLockDeadEnd,
        }
    }
}

/// One platform-conformance drill a dimension is qualified by.
///
/// The drill corpus binds the wrong-target reopen, handler conflict,
/// lock-screen privacy, missing-root recovery, and store-lock drills into the
/// qualification family per the platform-conformance requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDrill {
    /// Channel/build ownership audit drill.
    ChannelOwnershipAudit,
    /// Handler-conflict (protocol/file-association takeover) drill.
    HandlerConflict,
    /// Wrong-target reopen drill.
    WrongTargetReopen,
    /// Lock-screen / shared-surface notification privacy drill.
    LockScreenPrivacy,
    /// Missing removable/network root recovery drill.
    MissingRootRecovery,
    /// Credential/trust store-lock recovery drill.
    StoreLock,
}

impl QualificationDrill {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelOwnershipAudit => "channel_ownership_audit",
            Self::HandlerConflict => "handler_conflict",
            Self::WrongTargetReopen => "wrong_target_reopen",
            Self::LockScreenPrivacy => "lock_screen_privacy",
            Self::MissingRootRecovery => "missing_root_recovery",
            Self::StoreLock => "store_lock",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ChannelOwnershipAudit => "Channel-ownership audit",
            Self::HandlerConflict => "Handler conflict",
            Self::WrongTargetReopen => "Wrong-target reopen",
            Self::LockScreenPrivacy => "Lock-screen privacy",
            Self::MissingRootRecovery => "Missing-root recovery",
            Self::StoreLock => "Store lock",
        }
    }

    /// Returns the required drills in canonical order.
    pub const fn required_drills() -> [Self; 6] {
        [
            Self::ChannelOwnershipAudit,
            Self::HandlerConflict,
            Self::WrongTargetReopen,
            Self::LockScreenPrivacy,
            Self::MissingRootRecovery,
            Self::StoreLock,
        ]
    }
}

/// A distinct native-desktop qualification failure class.
///
/// Each class names a materially different way a desktop profile can fail
/// qualification. They are never collapsed: an unprovable channel owner, a
/// protocol-handler conflict, a wrong-target reopen, and a lock-screen leak are
/// separate findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationFailureMode {
    /// The channel/build owner could not be proven; a handler could be taken
    /// over silently.
    OwnershipUnprovable,
    /// A protocol/deep-link handler conflicts with another registrant.
    ProtocolHandlerConflict,
    /// A file-type association conflicts with another registrant.
    FileAssociationConflict,
    /// A reopen path landed on the wrong target.
    WrongTargetReopen,
    /// A notification leaked private content on a lock or shared surface.
    LockScreenLeak,
    /// A missing removable/network root silently lost user context.
    MissingRootSilentLoss,
    /// A locked store dead-ended the pending action.
    StoreLockDeadEnd,
}

impl QualificationFailureMode {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnershipUnprovable => "ownership_unprovable",
            Self::ProtocolHandlerConflict => "protocol_handler_conflict",
            Self::FileAssociationConflict => "file_association_conflict",
            Self::WrongTargetReopen => "wrong_target_reopen",
            Self::LockScreenLeak => "lock_screen_leak",
            Self::MissingRootSilentLoss => "missing_root_silent_loss",
            Self::StoreLockDeadEnd => "store_lock_dead_end",
        }
    }
}

/// Status a profile reports for one qualification dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationStatus {
    /// The dimension is qualified with captured drill evidence.
    Qualified,
    /// The dimension does not apply to this profile; a reason MUST be set.
    NotApplicable,
    /// The profile narrows this dimension; a reason MUST be set.
    ExplicitlyNarrowed,
    /// The dimension is claimed but unproven. Always a blocker.
    Unqualified,
    /// The dimension's drill failed. Always a blocker.
    Failed,
}

impl QualificationStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::NotApplicable => "not_applicable",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::Unqualified => "unqualified",
            Self::Failed => "failed",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ExplicitlyNarrowed)
    }

    /// `true` for the status that projects captured drill evidence.
    pub const fn projects_evidence(self) -> bool {
        matches!(self, Self::Qualified)
    }
}

/// Freshness of the captured qualification evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed profile.
    Stale,
}

impl EvidenceFreshness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// The published claim state derived for one profile. The claim is computed
/// from the dimension qualification, so it can never be greener than the proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimState {
    /// Every marketed dimension is qualified with fresh evidence; the
    /// native-desktop claim is published for this profile.
    Published,
    /// Some dimensions are narrowed or red; the claim is published with an
    /// explicit narrowed scope.
    Narrowed,
    /// No marketed dimension qualifies; the claim is withheld.
    Withheld,
}

impl ClaimState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Narrowed => "narrowed",
            Self::Withheld => "withheld",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Published => "Published",
            Self::Narrowed => "Narrowed",
            Self::Withheld => "Withheld",
        }
    }
}

/// Cross-links to the canonical upstream packets the qualification family
/// depends on so the matrix, handler-ownership, reopen, notification, and
/// recovery proofs cannot drift independently of the claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationCrossLinks {
    /// Native-desktop matrix this family certifies.
    pub native_desktop_matrix_ref: String,
    /// Channel-ownership audit ledger.
    pub channel_ownership_ref: String,
    /// Protocol-handler ownership matrix.
    pub protocol_handler_ownership_ref: String,
    /// File-association ownership matrix.
    pub file_association_ownership_ref: String,
    /// Exact-target reopen corpus.
    pub reopen_corpus_ref: String,
    /// Lock-screen / notification-privacy rows.
    pub notification_privacy_ref: String,
    /// Store-lock and external-root recovery packet.
    pub external_root_recovery_ref: String,
    /// Install-topology / portability governance packet.
    pub install_topology_ref: String,
}

impl QualificationCrossLinks {
    /// Returns the cross-link fields as `(label, ref)` pairs in canonical
    /// order.
    pub fn as_pairs(&self) -> [(&'static str, &str); 8] {
        [
            ("native_desktop_matrix_ref", &self.native_desktop_matrix_ref),
            ("channel_ownership_ref", &self.channel_ownership_ref),
            (
                "protocol_handler_ownership_ref",
                &self.protocol_handler_ownership_ref,
            ),
            (
                "file_association_ownership_ref",
                &self.file_association_ownership_ref,
            ),
            ("reopen_corpus_ref", &self.reopen_corpus_ref),
            ("notification_privacy_ref", &self.notification_privacy_ref),
            (
                "external_root_recovery_ref",
                &self.external_root_recovery_ref,
            ),
            ("install_topology_ref", &self.install_topology_ref),
        ]
    }

    /// The canonical cross-link set every report carries.
    pub fn canonical() -> Self {
        Self {
            native_desktop_matrix_ref: QUALIFICATION_MATRIX_REPORT_REF.to_owned(),
            channel_ownership_ref: "artifacts/release/channel_ownership_audit.yaml".to_owned(),
            protocol_handler_ownership_ref:
                "artifacts/platform/protocol_handler_ownership_matrix.yaml".to_owned(),
            file_association_ownership_ref:
                "artifacts/platform/file_association_ownership_matrix.yaml".to_owned(),
            reopen_corpus_ref: "fixtures/platform/exact_target_reopen_cases".to_owned(),
            notification_privacy_ref: "artifacts/platform/lock_screen_privacy_rows.yaml".to_owned(),
            external_root_recovery_ref:
                "artifacts/platform/m5-store-lock-and-external-root-recovery.md".to_owned(),
            install_topology_ref: "artifacts/install/m5/m5-install-and-portability-governance.md"
                .to_owned(),
        }
    }
}

/// Canonical descriptor for one claimed desktop profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationProfileDescriptor {
    /// Stable profile id (e.g. `profile:macos.stable`).
    pub profile_id: String,
    /// Platform the profile is scoped to.
    pub platform: DesktopPlatform,
    /// Delivery channel the profile certifies.
    pub channel: DeliveryChannel,
    /// Descriptor revision the qualification was produced against.
    pub descriptor_revision_ref: String,
    /// Reviewer-facing display label ref.
    pub display_label_ref: String,
    /// Channel/build owner of the OS-level registration. MUST be non-empty.
    pub channel_build_owner_ref: String,
    /// How the channel/build owns the registration.
    pub ownership_kind: OwnershipKind,
    /// Trust / profile / tenant / policy checkpoint qualification routes
    /// through. MUST be non-empty.
    pub trust_checkpoint_ref: String,
    /// Continuity note retained on the descriptor. MUST be non-empty.
    pub continuity_note: String,
    /// Freshness of the captured evidence.
    pub evidence_freshness: EvidenceFreshness,
    /// Timestamp the evidence was captured.
    pub evidence_captured_at: String,
    /// Rule downstream surfaces follow when evidence goes stale. MUST be
    /// non-empty.
    pub downgrade_rule_ref: String,
    /// `true` when the profile is marketed and must qualify or narrow.
    pub marketed: bool,
    /// `true` once the profile rides the governed qualification harness and
    /// does not drive its own qualification path. MUST be `true`.
    pub registered_on_qualification_harness: bool,
}

/// Per-dimension binding a profile reports for one qualification dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationDimensionBinding {
    /// Dimension this binding covers.
    pub dimension: QualificationDimension,
    /// Drill the dimension is qualified by.
    pub required_drill: QualificationDrill,
    /// Status the profile reports.
    pub status: QualificationStatus,
    /// Failure mode (`Some` only when `status` is `failed`).
    pub failure_mode: Option<QualificationFailureMode>,
    /// Bound platform-conformance drill case (required when qualified or
    /// failed).
    pub drill_ref: Option<String>,
    /// Captured evidence-pack ref (required when qualified).
    pub evidence_pack_ref: Option<String>,
    /// Narrowing reason set when `status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the binding.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum QualificationBlockingFinding {
    /// The channel/build owner could not be proven.
    OwnershipUnprovable {
        /// Profile that exposes the gap.
        profile_id: String,
        /// Dimension that exposes the gap.
        dimension: QualificationDimension,
    },
    /// A protocol/deep-link handler conflicts with another registrant.
    ProtocolHandlerConflict {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A file-type association conflicts with another registrant.
    FileAssociationConflict {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A reopen path landed on the wrong target.
    WrongTargetReopen {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A notification leaked private content on a lock or shared surface.
    LockScreenLeak {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A missing removable/network root silently lost user context.
    MissingRootSilentLoss {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A locked store dead-ended the pending action.
    StoreLockDeadEnd {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A marketed dimension is claimed but unproven.
    UnqualifiedMarketedDimension {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A binding's declared failure mode disagrees with its dimension's
    /// canonical failure mode, or a failure mode is set without a failed
    /// status.
    FailureModeDrift {
        profile_id: String,
        dimension: QualificationDimension,
        /// Declared failure mode, when present.
        declared_failure_mode: Option<QualificationFailureMode>,
    },
    /// A binding's declared required drill disagrees with the dimension's
    /// canonical drill.
    DrillKindDrift {
        profile_id: String,
        dimension: QualificationDimension,
        /// Declared drill that drifted.
        declared_drill: QualificationDrill,
    },
    /// A qualified or failed dimension is missing its drill ref.
    MissingDrillRef {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A qualified dimension is missing its captured evidence pack.
    MissingEvidencePack {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A qualified dimension's evidence pack does not name this profile, so the
    /// row borrows another profile's or channel's proof.
    BorrowedProofAcrossProfile {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A narrowed dimension is missing the `narrowing_reason`.
    MissingNarrowingReason {
        profile_id: String,
        dimension: QualificationDimension,
        status: QualificationStatus,
    },
    /// The profile is missing a required dimension binding.
    MissingRequiredDimension {
        profile_id: String,
        dimension: QualificationDimension,
    },
    /// A marketed profile carries stale evidence.
    StaleEvidenceOnMarketedProfile { profile_id: String },
    /// The descriptor carries no channel/build owner.
    MissingChannelBuildOwner { profile_id: String },
    /// The descriptor carries no trust checkpoint.
    MissingTrustCheckpoint { profile_id: String },
    /// The descriptor carries no continuity note.
    MissingContinuityNote { profile_id: String },
    /// The descriptor carries no downgrade rule.
    MissingDowngradeRule { profile_id: String },
    /// The profile drives its own qualification off the governed harness.
    ProfileNotOnHarness { profile_id: String },
}

impl QualificationBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::OwnershipUnprovable { .. } => "ownership_unprovable",
            Self::ProtocolHandlerConflict { .. } => "protocol_handler_conflict",
            Self::FileAssociationConflict { .. } => "file_association_conflict",
            Self::WrongTargetReopen { .. } => "wrong_target_reopen",
            Self::LockScreenLeak { .. } => "lock_screen_leak",
            Self::MissingRootSilentLoss { .. } => "missing_root_silent_loss",
            Self::StoreLockDeadEnd { .. } => "store_lock_dead_end",
            Self::UnqualifiedMarketedDimension { .. } => "unqualified_marketed_dimension",
            Self::FailureModeDrift { .. } => "failure_mode_drift",
            Self::DrillKindDrift { .. } => "drill_kind_drift",
            Self::MissingDrillRef { .. } => "missing_drill_ref",
            Self::MissingEvidencePack { .. } => "missing_evidence_pack",
            Self::BorrowedProofAcrossProfile { .. } => "borrowed_proof_across_profile",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingRequiredDimension { .. } => "missing_required_dimension",
            Self::StaleEvidenceOnMarketedProfile { .. } => "stale_evidence_on_marketed_profile",
            Self::MissingChannelBuildOwner { .. } => "missing_channel_build_owner",
            Self::MissingTrustCheckpoint { .. } => "missing_trust_checkpoint",
            Self::MissingContinuityNote { .. } => "missing_continuity_note",
            Self::MissingDowngradeRule { .. } => "missing_downgrade_rule",
            Self::ProfileNotOnHarness { .. } => "profile_not_on_harness",
        }
    }

    /// Returns the profile id this finding is attached to.
    pub fn profile_id(&self) -> &str {
        match self {
            Self::OwnershipUnprovable { profile_id, .. }
            | Self::ProtocolHandlerConflict { profile_id, .. }
            | Self::FileAssociationConflict { profile_id, .. }
            | Self::WrongTargetReopen { profile_id, .. }
            | Self::LockScreenLeak { profile_id, .. }
            | Self::MissingRootSilentLoss { profile_id, .. }
            | Self::StoreLockDeadEnd { profile_id, .. }
            | Self::UnqualifiedMarketedDimension { profile_id, .. }
            | Self::FailureModeDrift { profile_id, .. }
            | Self::DrillKindDrift { profile_id, .. }
            | Self::MissingDrillRef { profile_id, .. }
            | Self::MissingEvidencePack { profile_id, .. }
            | Self::BorrowedProofAcrossProfile { profile_id, .. }
            | Self::MissingNarrowingReason { profile_id, .. }
            | Self::MissingRequiredDimension { profile_id, .. }
            | Self::StaleEvidenceOnMarketedProfile { profile_id }
            | Self::MissingChannelBuildOwner { profile_id }
            | Self::MissingTrustCheckpoint { profile_id }
            | Self::MissingContinuityNote { profile_id }
            | Self::MissingDowngradeRule { profile_id }
            | Self::ProfileNotOnHarness { profile_id } => profile_id,
        }
    }

    /// Returns the dimension this finding is attached to, when dimension-scoped.
    pub fn dimension(&self) -> Option<QualificationDimension> {
        match self {
            Self::OwnershipUnprovable { dimension, .. }
            | Self::ProtocolHandlerConflict { dimension, .. }
            | Self::FileAssociationConflict { dimension, .. }
            | Self::WrongTargetReopen { dimension, .. }
            | Self::LockScreenLeak { dimension, .. }
            | Self::MissingRootSilentLoss { dimension, .. }
            | Self::StoreLockDeadEnd { dimension, .. }
            | Self::UnqualifiedMarketedDimension { dimension, .. }
            | Self::FailureModeDrift { dimension, .. }
            | Self::DrillKindDrift { dimension, .. }
            | Self::MissingDrillRef { dimension, .. }
            | Self::MissingEvidencePack { dimension, .. }
            | Self::BorrowedProofAcrossProfile { dimension, .. }
            | Self::MissingNarrowingReason { dimension, .. }
            | Self::MissingRequiredDimension { dimension, .. } => Some(*dimension),
            _ => None,
        }
    }
}

/// One per-profile qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopProfileRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the profile.
    pub descriptor: QualificationProfileDescriptor,
    /// Per-dimension bindings, in canonical dimension order.
    pub bindings: Vec<QualificationDimensionBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<QualificationBlockingFinding>,
    /// Derived claim state for this profile.
    pub claim_state: ClaimState,
    /// `true` when the profile is marketed.
    pub marketed: bool,
}

impl NativeDesktopProfileRow {
    /// Dimensions this profile qualifies, in canonical order.
    pub fn qualified_dimensions(&self) -> Vec<QualificationDimension> {
        self.bindings
            .iter()
            .filter(|binding| binding.status == QualificationStatus::Qualified)
            .map(|binding| binding.dimension)
            .collect()
    }

    /// Dimensions this profile narrows or marks not applicable, in canonical
    /// order.
    pub fn narrowed_dimensions(&self) -> Vec<QualificationDimension> {
        self.bindings
            .iter()
            .filter(|binding| {
                matches!(
                    binding.status,
                    QualificationStatus::ExplicitlyNarrowed | QualificationStatus::NotApplicable
                )
            })
            .map(|binding| binding.dimension)
            .collect()
    }

    /// Dimensions this profile fails or leaves unqualified, in canonical order.
    pub fn blocked_dimensions(&self) -> Vec<QualificationDimension> {
        self.bindings
            .iter()
            .filter(|binding| {
                matches!(
                    binding.status,
                    QualificationStatus::Failed | QualificationStatus::Unqualified
                )
            })
            .map(|binding| binding.dimension)
            .collect()
    }
}

/// One `(class, count)` blocking-finding tally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationFindingCount {
    /// Finding class token.
    pub class: String,
    /// Number of findings in this class.
    pub count: usize,
}

/// Per-class blocking-finding summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationFindingSummary {
    /// Total blocking findings across the report.
    pub total_blocking_findings: usize,
    /// Per-class tallies, sorted by class token.
    pub by_class: Vec<QualificationFindingCount>,
}

/// Per-dimension coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationDimensionCoverage {
    /// Dimension this summary covers.
    pub dimension: QualificationDimension,
    /// Number of profiles that qualify the dimension.
    pub qualified: usize,
    /// Number of profiles that mark the dimension not applicable.
    pub not_applicable: usize,
    /// Number of profiles that explicitly narrow the dimension.
    pub explicitly_narrowed: usize,
    /// Number of profiles that leave the dimension unqualified.
    pub unqualified: usize,
    /// Number of profiles that fail the dimension.
    pub failed: usize,
}

/// The auto-narrowing published claim for one profile. Help/About, install,
/// docs, support, and evaluation materials ingest this entry directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileClaim {
    /// Profile id this claim covers.
    pub profile_id: String,
    /// Platform the profile is scoped to.
    pub platform: DesktopPlatform,
    /// Delivery channel the profile certifies.
    pub channel: DeliveryChannel,
    /// Derived claim state.
    pub claim_state: ClaimState,
    /// Dimensions qualified for this profile, in canonical order.
    pub qualified_dimensions: Vec<QualificationDimension>,
    /// Dimensions narrowed or marked not applicable, in canonical order.
    pub narrowed_dimensions: Vec<QualificationDimension>,
    /// Dimensions failed or left unqualified, in canonical order.
    pub blocked_dimensions: Vec<QualificationDimension>,
    /// Evidence freshness of the underlying profile descriptor.
    pub evidence_freshness: EvidenceFreshness,
    /// Downgrade rule downstream surfaces follow when this claim narrows.
    pub downgrade_rule_ref: String,
    /// Stable reason describing why the claim is published, narrowed, or
    /// withheld.
    pub reason: String,
}

/// One marketed profile release tooling should narrow because its evidence is
/// stale or a dimension is red/unproven.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowableProfile {
    /// Profile id that must narrow.
    pub profile_id: String,
    /// Dimension that must narrow, when dimension-scoped.
    pub dimension: Option<QualificationDimension>,
    /// Stable reason the profile is narrowable.
    pub reason: String,
}

/// Native-desktop per-desktop-profile qualification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopQualificationReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required dimensions, in canonical order.
    pub required_dimensions: Vec<QualificationDimension>,
    /// Required drills, in canonical order.
    pub required_drills: Vec<QualificationDrill>,
    /// Union of claimed platforms across all profiles, sorted.
    pub claimed_platforms: Vec<DesktopPlatform>,
    /// Cross-links to upstream packets.
    pub cross_links: QualificationCrossLinks,
    /// Per-profile rows, sorted by `descriptor.profile_id`.
    pub profiles: Vec<NativeDesktopProfileRow>,
    /// Per-dimension coverage summary, in canonical dimension order.
    pub dimension_coverage: Vec<QualificationDimensionCoverage>,
    /// Auto-narrowing claim scope, sorted by profile id.
    pub claim_scope: Vec<ProfileClaim>,
    /// Per-class blocking-finding summary.
    pub findings_summary: QualificationFindingSummary,
    /// Number of registered profiles present.
    pub registered_profile_count: usize,
    /// Number of profiles marketed.
    pub marketed_profile_count: usize,
    /// Total dimension bindings checked.
    pub dimensions_checked: usize,
    /// Number of profiles whose claim is published.
    pub published_claim_count: usize,
    /// Number of profiles whose claim is narrowed.
    pub narrowed_claim_count: usize,
    /// Number of profiles whose claim is withheld.
    pub withheld_claim_count: usize,
    /// Marketed profiles release tooling should narrow.
    pub narrowable_marketed_profiles: Vec<NarrowableProfile>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this report is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Shiproom claim-packet publication ref.
    pub claim_packet_ref: String,
    /// Native-desktop matrix ref this family certifies.
    pub matrix_report_ref: String,
    /// Docs/help refs the report can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the report can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the report was generated.
    pub generated_at: String,
}

impl NativeDesktopQualificationReport {
    /// Returns `true` when every required dimension is qualified by at least one
    /// profile.
    pub fn every_dimension_qualified(&self) -> bool {
        QualificationDimension::required_dimensions()
            .into_iter()
            .all(|dimension| {
                self.profiles.iter().any(|profile| {
                    profile.bindings.iter().any(|binding| {
                        binding.dimension == dimension
                            && binding.status == QualificationStatus::Qualified
                    })
                })
            })
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "qualification: profiles={}, marketed={}, dimensions={}, published={}, narrowed={}, withheld={}, blocking={}, clean={}",
            self.registered_profile_count,
            self.marketed_profile_count,
            self.dimensions_checked,
            self.published_claim_count,
            self.narrowed_claim_count,
            self.withheld_claim_count,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.dimension_coverage {
            lines.push(format!(
                "{}: qualified={}, not_applicable={}, narrowed={}, unqualified={}, failed={}",
                coverage.dimension.as_str(),
                coverage.qualified,
                coverage.not_applicable,
                coverage.explicitly_narrowed,
                coverage.unqualified,
                coverage.failed,
            ));
        }
        for claim in &self.claim_scope {
            lines.push(format!(
                "claim: {} -- {} -- {}",
                claim.profile_id,
                claim.claim_state.as_str(),
                claim.reason,
            ));
        }
        for profile in &self.profiles {
            for finding in &profile.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.profile_id(),
                    finding
                        .dimension()
                        .map(QualificationDimension::as_str)
                        .unwrap_or("profile"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_profiles {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.profile_id,
                narrowable
                    .dimension
                    .map(QualificationDimension::as_str)
                    .unwrap_or("profile"),
                narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown qualification matrix artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 native-desktop qualification matrix\n\n");
        out.push_str(
            "Generated from the seeded qualification family in\n\
             [`crate::m5_native_desktop_qualification`](../../../crates/aureline-shell/src/m5_native_desktop_qualification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- report-md > \\\n  artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix: `{}`\n",
            self.matrix_report_ref
        ));
        out.push_str(&format!(
            "- Claimed platforms: {}\n",
            self.claimed_platforms
                .iter()
                .map(|platform| format!("`{}`", platform.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Registered profiles: `{}`\n",
            self.registered_profile_count
        ));
        out.push_str(&format!(
            "- Marketed profiles: `{}`\n",
            self.marketed_profile_count
        ));
        out.push_str(&format!(
            "- Dimensions checked: `{}`\n",
            self.dimensions_checked
        ));
        out.push_str(&format!(
            "- Claim scope: published `{}`, narrowed `{}`, withheld `{}`\n",
            self.published_claim_count, self.narrowed_claim_count, self.withheld_claim_count,
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed profiles: `{}`\n",
            self.narrowable_marketed_profiles.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Cross-links\n\n");
        out.push_str("| Upstream packet | Ref |\n| --------------- | --- |\n");
        for (label, value) in self.cross_links.as_pairs() {
            out.push_str(&format!("| `{label}` | `{value}` |\n"));
        }
        out.push('\n');

        out.push_str("## Per-dimension coverage\n\n");
        out.push_str(
            "| Dimension | Drill | Qualified | Not applicable | Narrowed | Unqualified | Failed |\n\
             | --------- | ----- | --------: | -------------: | -------: | ----------: | -----: |\n",
        );
        for coverage in &self.dimension_coverage {
            out.push_str(&format!(
                "| {} | `{}` | {} | {} | {} | {} | {} |\n",
                coverage.dimension.display_label(),
                coverage.dimension.required_drill().as_str(),
                coverage.qualified,
                coverage.not_applicable,
                coverage.explicitly_narrowed,
                coverage.unqualified,
                coverage.failed,
            ));
        }
        out.push('\n');

        out.push_str("## Claim scope\n\n");
        out.push_str(
            "| Profile | Platform | Channel | Claim | Reason |\n\
             | ------- | -------- | ------- | ----- | ------ |\n",
        );
        for claim in &self.claim_scope {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | **{}** | {} |\n",
                claim.profile_id,
                claim.platform.as_str(),
                claim.channel.as_str(),
                claim.claim_state.display_label(),
                claim.reason,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        for tally in &self.findings_summary.by_class {
            out.push_str(&format!("| `{}` | {} |\n", tally.class, tally.count));
        }
        if self.findings_summary.by_class.is_empty() {
            out.push_str("| _(none)_ | 0 |\n");
        }
        out.push('\n');

        out.push_str("## Per-profile rows\n\n");
        for profile in &self.profiles {
            out.push_str(&format!(
                "### `{}` ({} / {})\n\n",
                profile.descriptor.profile_id,
                profile.descriptor.platform.as_str(),
                profile.descriptor.channel.as_str(),
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                profile.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Channel/build owner: `{}` (`{}`)\n",
                profile.descriptor.channel_build_owner_ref,
                profile.descriptor.ownership_kind.as_str(),
            ));
            out.push_str(&format!(
                "- Trust checkpoint: `{}`\n",
                profile.descriptor.trust_checkpoint_ref
            ));
            out.push_str(&format!(
                "- Evidence freshness: `{}` (captured `{}`)\n",
                profile.descriptor.evidence_freshness.as_str(),
                profile.descriptor.evidence_captured_at,
            ));
            out.push_str(&format!(
                "- Downgrade rule: `{}`\n",
                profile.descriptor.downgrade_rule_ref
            ));
            out.push_str(&format!(
                "- Claim state: **{}**\n",
                profile.claim_state.display_label()
            ));
            out.push_str(&format!(
                "- Marketed: `{}`\n",
                if profile.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- Continuity note: {}\n\n",
                profile.descriptor.continuity_note
            ));

            out.push_str(
                "| Dimension | Drill | Status | Failure | Drill ref | Narrowing reason |\n\
                 | --------- | ----- | ------ | ------- | --------- | ---------------- |\n",
            );
            for binding in &profile.bindings {
                let failure = binding
                    .failure_mode
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let drill = binding.drill_ref.as_deref().unwrap_or("-");
                let narrowing = binding.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.dimension.display_label(),
                    binding.required_drill.as_str(),
                    binding.status.as_str(),
                    failure,
                    drill,
                    narrowing,
                ));
            }
            out.push('\n');

            if profile.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &profile.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .dimension()
                            .map(QualificationDimension::as_str)
                            .unwrap_or("profile"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_native_desktop_qualification_fixtures\n",
        );
        out.push_str("python3 tools/ci/m5/native_desktop_qualification_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the qualification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopQualificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Qualification report quoted in full.
    pub report: NativeDesktopQualificationReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl NativeDesktopQualificationSupportExport {
    /// Builds the support-export wrapper for a qualification report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: NativeDesktopQualificationReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for profile in &report.profiles {
            case_ids.push(profile.descriptor.profile_id.clone());
            case_ids.push(profile.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: QUALIFICATION_SCHEMA_VERSION,
            shared_contract_ref: QUALIFICATION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// One downgrade-rule binding the claim packet exposes so shiproom and
/// release-center surfaces can see how a profile narrows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimDowngradeRule {
    /// Profile id the rule applies to.
    pub profile_id: String,
    /// Current claim state.
    pub claim_state: ClaimState,
    /// Downgrade rule downstream surfaces follow.
    pub downgrade_rule_ref: String,
}

/// Shiproom-facing claim packet. It derives the auto-narrowing claim scope from
/// the qualification report so Help/About, install/update, docs, support, and
/// evaluation materials reuse one source of truth instead of parallel
/// summaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopClaimPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by shiproom and release center.
    pub shared_contract_ref: String,
    /// Stable claim-packet id.
    pub claim_packet_id: String,
    /// Qualification report quoted in full.
    pub report: NativeDesktopQualificationReport,
    /// Profile ids whose claim is published, sorted.
    pub publishable_profiles: Vec<String>,
    /// Profile ids whose claim is narrowed, sorted.
    pub narrowed_profiles: Vec<String>,
    /// Profile ids whose claim is withheld, sorted.
    pub withheld_profiles: Vec<String>,
    /// Per-profile downgrade rules, sorted by profile id.
    pub downgrade_rules: Vec<ClaimDowngradeRule>,
    /// `true` when no profile is withheld and the report is clean.
    pub claim_publishable: bool,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl NativeDesktopClaimPacket {
    /// Builds the shiproom claim packet for a qualification report.
    pub fn from_report(
        claim_packet_id: impl Into<String>,
        report: NativeDesktopQualificationReport,
    ) -> Self {
        let mut publishable_profiles = Vec::new();
        let mut narrowed_profiles = Vec::new();
        let mut withheld_profiles = Vec::new();
        let mut downgrade_rules = Vec::new();
        let mut case_ids = vec![report.report_id.clone()];

        for claim in &report.claim_scope {
            match claim.claim_state {
                ClaimState::Published => publishable_profiles.push(claim.profile_id.clone()),
                ClaimState::Narrowed => narrowed_profiles.push(claim.profile_id.clone()),
                ClaimState::Withheld => withheld_profiles.push(claim.profile_id.clone()),
            }
            downgrade_rules.push(ClaimDowngradeRule {
                profile_id: claim.profile_id.clone(),
                claim_state: claim.claim_state,
                downgrade_rule_ref: claim.downgrade_rule_ref.clone(),
            });
            case_ids.push(claim.profile_id.clone());
        }
        publishable_profiles.sort();
        narrowed_profiles.sort();
        withheld_profiles.sort();
        downgrade_rules.sort_by(|left, right| left.profile_id.cmp(&right.profile_id));

        let claim_publishable = report.report_clean && withheld_profiles.is_empty();

        Self {
            record_kind: QUALIFICATION_CLAIM_PACKET_RECORD_KIND.to_owned(),
            schema_version: QUALIFICATION_SCHEMA_VERSION,
            shared_contract_ref: QUALIFICATION_SHARED_CONTRACT_REF.to_owned(),
            claim_packet_id: claim_packet_id.into(),
            report,
            publishable_profiles,
            narrowed_profiles,
            withheld_profiles,
            downgrade_rules,
            claim_publishable,
            case_ids,
        }
    }

    /// Renders the shiproom claim-packet markdown artifact.
    pub fn render_markdown(&self) -> String {
        let report = &self.report;
        let mut out = String::new();
        out.push_str("# Shiproom claim packet — M5 native-desktop qualification\n\n");
        out.push_str(
            "This packet is the shiproom- and release-center-facing view of the\n\
             native-desktop qualification family. It does not maintain its own\n\
             summary: the claim scope below is derived from the canonical\n\
             qualification report and narrows automatically when a profile row goes\n\
             stale, missing, or red.\n\n",
        );

        out.push_str("## Canonical inputs\n\n");
        out.push_str(&format!(
            "- Qualification matrix: `{}`\n",
            report.published_report_ref
        ));
        out.push_str(&format!(
            "- Report fixture: `{}`\n",
            "fixtures/platform/m5-native-desktop-qualification/report.json"
        ));
        out.push_str(&format!(
            "- Boundary schema: `{}`\n",
            report.source_schema_ref
        ));
        out.push_str(&format!(
            "- Companion doc: `{}`\n",
            report.published_doc_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix: `{}`\n",
            report.matrix_report_ref
        ));
        out.push_str("- CI gate: `tools/ci/m5/native_desktop_qualification_check.py`\n\n");

        out.push_str(&format!(
            "- Claim publishable: **{}**\n",
            if self.claim_publishable { "yes" } else { "no" }
        ));
        out.push_str(&format!(
            "- Published profiles: `{}`\n",
            self.publishable_profiles.len()
        ));
        out.push_str(&format!(
            "- Narrowed profiles: `{}`\n",
            self.narrowed_profiles.len()
        ));
        out.push_str(&format!(
            "- Withheld profiles: `{}`\n\n",
            self.withheld_profiles.len()
        ));

        out.push_str("## Claim scope\n\n");
        out.push_str(
            "| Profile | Platform | Channel | Claim | Downgrade rule | Reason |\n\
             | ------- | -------- | ------- | ----- | -------------- | ------ |\n",
        );
        for claim in &report.claim_scope {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | **{}** | `{}` | {} |\n",
                claim.profile_id,
                claim.platform.as_str(),
                claim.channel.as_str(),
                claim.claim_state.display_label(),
                claim.downgrade_rule_ref,
                claim.reason,
            ));
        }
        out.push('\n');

        out.push_str("## Sign-off gate\n\n");
        out.push_str(
            "Promotion of the native-desktop claim holds unless all of the following\n\
             are true on the current qualification report:\n\n",
        );
        out.push_str(
            "1. The report is clean: every claimed profile binds all seven\n   \
             dimensions and no profile carries a blocking finding\n   \
             (`report.report_clean == true`).\n",
        );
        out.push_str(
            "2. No distinct qualification failure is open — `ownership_unprovable`,\n   \
             `protocol_handler_conflict`, `file_association_conflict`,\n   \
             `wrong_target_reopen`, `lock_screen_leak`, `missing_root_silent_loss`,\n   \
             or `store_lock_dead_end`.\n",
        );
        out.push_str(
            "3. No profile borrows another profile's or channel's proof\n   \
             (`borrowed_proof_across_profile`); each claimed row carries current\n   \
             proof of its own.\n",
        );
        out.push_str(
            "4. No marketed profile carries stale evidence\n   \
             (`narrowable_marketed_profiles` is empty).\n",
        );
        out.push_str(
            "5. No profile claim is withheld, and every narrowed claim names the\n   \
             dimensions it dropped.\n\n",
        );

        out.push_str("## Regenerating this packet\n\n");
        out.push_str(
            "This packet is checked in alongside the report it derives from. When the\n\
             qualification contract changes, regenerate the packet and re-run the gate\n\
             before re-reviewing:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- claim-packet-md > \\\n  artifacts/shiproom/m5-native-desktop-claim-packet/m5_native_desktop_claim_packet.md\n",
        );
        out.push_str("python3 tools/ci/m5/native_desktop_qualification_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Computes the per-profile blocking findings from a descriptor and its
/// dimension bindings.
fn compute_profile_findings(
    descriptor: &QualificationProfileDescriptor,
    bindings: &[QualificationDimensionBinding],
) -> Vec<QualificationBlockingFinding> {
    let mut findings = Vec::new();
    let profile_id = descriptor.profile_id.clone();

    // Descriptor-level (profile-scoped) findings.
    if descriptor.channel_build_owner_ref.trim().is_empty() {
        findings.push(QualificationBlockingFinding::MissingChannelBuildOwner {
            profile_id: profile_id.clone(),
        });
    }
    if descriptor.trust_checkpoint_ref.trim().is_empty() {
        findings.push(QualificationBlockingFinding::MissingTrustCheckpoint {
            profile_id: profile_id.clone(),
        });
    }
    if descriptor.continuity_note.trim().is_empty() {
        findings.push(QualificationBlockingFinding::MissingContinuityNote {
            profile_id: profile_id.clone(),
        });
    }
    if descriptor.downgrade_rule_ref.trim().is_empty() {
        findings.push(QualificationBlockingFinding::MissingDowngradeRule {
            profile_id: profile_id.clone(),
        });
    }
    if !descriptor.registered_on_qualification_harness {
        findings.push(QualificationBlockingFinding::ProfileNotOnHarness {
            profile_id: profile_id.clone(),
        });
    }
    if descriptor.marketed && descriptor.evidence_freshness == EvidenceFreshness::Stale {
        findings.push(
            QualificationBlockingFinding::StaleEvidenceOnMarketedProfile {
                profile_id: profile_id.clone(),
            },
        );
    }

    // Every required dimension must be bound.
    let present: Vec<QualificationDimension> =
        bindings.iter().map(|binding| binding.dimension).collect();
    for dimension in QualificationDimension::required_dimensions() {
        if !present.contains(&dimension) {
            findings.push(QualificationBlockingFinding::MissingRequiredDimension {
                profile_id: profile_id.clone(),
                dimension,
            });
        }
    }

    for binding in bindings {
        compute_binding_findings(&profile_id, descriptor.marketed, binding, &mut findings);
    }

    findings
}

/// Computes the blocking findings for one dimension binding.
fn compute_binding_findings(
    profile_id: &str,
    marketed: bool,
    binding: &QualificationDimensionBinding,
    findings: &mut Vec<QualificationBlockingFinding>,
) {
    let dimension = binding.dimension;

    // A binding that declares the wrong drill for its dimension is drift.
    if binding.required_drill != dimension.required_drill() {
        findings.push(QualificationBlockingFinding::DrillKindDrift {
            profile_id: profile_id.to_owned(),
            dimension,
            declared_drill: binding.required_drill,
        });
    }

    match binding.status {
        QualificationStatus::Failed => {
            findings.push(failure_finding(dimension, profile_id));
            if binding.failure_mode != Some(dimension.canonical_failure_mode()) {
                findings.push(QualificationBlockingFinding::FailureModeDrift {
                    profile_id: profile_id.to_owned(),
                    dimension,
                    declared_failure_mode: binding.failure_mode,
                });
            }
            if binding.drill_ref.is_none() {
                findings.push(QualificationBlockingFinding::MissingDrillRef {
                    profile_id: profile_id.to_owned(),
                    dimension,
                });
            }
        }
        QualificationStatus::Unqualified => {
            if marketed {
                findings.push(QualificationBlockingFinding::UnqualifiedMarketedDimension {
                    profile_id: profile_id.to_owned(),
                    dimension,
                });
            }
            if binding.failure_mode.is_some() {
                findings.push(QualificationBlockingFinding::FailureModeDrift {
                    profile_id: profile_id.to_owned(),
                    dimension,
                    declared_failure_mode: binding.failure_mode,
                });
            }
        }
        QualificationStatus::Qualified => {
            if binding.failure_mode.is_some() {
                findings.push(QualificationBlockingFinding::FailureModeDrift {
                    profile_id: profile_id.to_owned(),
                    dimension,
                    declared_failure_mode: binding.failure_mode,
                });
            }
            if binding.drill_ref.is_none() {
                findings.push(QualificationBlockingFinding::MissingDrillRef {
                    profile_id: profile_id.to_owned(),
                    dimension,
                });
            }
            match &binding.evidence_pack_ref {
                None => findings.push(QualificationBlockingFinding::MissingEvidencePack {
                    profile_id: profile_id.to_owned(),
                    dimension,
                }),
                Some(evidence) if !evidence.contains(profile_id) => {
                    findings.push(QualificationBlockingFinding::BorrowedProofAcrossProfile {
                        profile_id: profile_id.to_owned(),
                        dimension,
                    });
                }
                Some(_) => {}
            }
        }
        status if status.requires_narrowing_reason() => {
            if binding.failure_mode.is_some() {
                findings.push(QualificationBlockingFinding::FailureModeDrift {
                    profile_id: profile_id.to_owned(),
                    dimension,
                    declared_failure_mode: binding.failure_mode,
                });
            }
            let reason_ok = binding
                .narrowing_reason
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                == Some(false);
            if !reason_ok {
                findings.push(QualificationBlockingFinding::MissingNarrowingReason {
                    profile_id: profile_id.to_owned(),
                    dimension,
                    status,
                });
            }
        }
        _ => {}
    }
}

/// Maps a failed dimension to its distinct blocking finding.
fn failure_finding(
    dimension: QualificationDimension,
    profile_id: &str,
) -> QualificationBlockingFinding {
    let profile_id = profile_id.to_owned();
    match dimension.canonical_failure_mode() {
        QualificationFailureMode::OwnershipUnprovable => {
            QualificationBlockingFinding::OwnershipUnprovable {
                profile_id,
                dimension,
            }
        }
        QualificationFailureMode::ProtocolHandlerConflict => {
            QualificationBlockingFinding::ProtocolHandlerConflict {
                profile_id,
                dimension,
            }
        }
        QualificationFailureMode::FileAssociationConflict => {
            QualificationBlockingFinding::FileAssociationConflict {
                profile_id,
                dimension,
            }
        }
        QualificationFailureMode::WrongTargetReopen => {
            QualificationBlockingFinding::WrongTargetReopen {
                profile_id,
                dimension,
            }
        }
        QualificationFailureMode::LockScreenLeak => QualificationBlockingFinding::LockScreenLeak {
            profile_id,
            dimension,
        },
        QualificationFailureMode::MissingRootSilentLoss => {
            QualificationBlockingFinding::MissingRootSilentLoss {
                profile_id,
                dimension,
            }
        }
        QualificationFailureMode::StoreLockDeadEnd => {
            QualificationBlockingFinding::StoreLockDeadEnd {
                profile_id,
                dimension,
            }
        }
    }
}

/// Derives the published claim state and reason for a profile row from its
/// dimension qualification, so the claim is never greener than the proof.
fn derive_claim_state(
    descriptor: &QualificationProfileDescriptor,
    bindings: &[QualificationDimensionBinding],
    has_blockers: bool,
) -> (ClaimState, String) {
    let qualified = bindings
        .iter()
        .filter(|binding| binding.status == QualificationStatus::Qualified)
        .count();
    let narrowed: Vec<&str> = bindings
        .iter()
        .filter(|binding| {
            matches!(
                binding.status,
                QualificationStatus::ExplicitlyNarrowed | QualificationStatus::NotApplicable
            )
        })
        .map(|binding| binding.dimension.as_str())
        .collect();
    let blocked: Vec<&str> = bindings
        .iter()
        .filter(|binding| {
            matches!(
                binding.status,
                QualificationStatus::Failed | QualificationStatus::Unqualified
            )
        })
        .map(|binding| binding.dimension.as_str())
        .collect();

    if !descriptor.marketed {
        return (ClaimState::Withheld, "profile_not_marketed".to_owned());
    }

    if qualified == 0 {
        return (
            ClaimState::Withheld,
            "no_marketed_dimension_qualified".to_owned(),
        );
    }

    let stale = descriptor.evidence_freshness == EvidenceFreshness::Stale;
    if !blocked.is_empty() || has_blockers || stale {
        let reason = if !blocked.is_empty() {
            format!("blocked_dimensions:{}", blocked.join(","))
        } else if stale {
            "stale_evidence".to_owned()
        } else {
            "blocking_findings_present".to_owned()
        };
        return (ClaimState::Narrowed, reason);
    }

    if !narrowed.is_empty() {
        return (
            ClaimState::Narrowed,
            format!("narrowed_dimensions:{}", narrowed.join(",")),
        );
    }

    (
        ClaimState::Published,
        "all_marketed_dimensions_qualified_with_fresh_evidence".to_owned(),
    )
}

/// Computes the per-dimension coverage summary and per-class finding summary.
fn summarize_report(
    profiles: &[NativeDesktopProfileRow],
) -> (
    Vec<QualificationDimensionCoverage>,
    QualificationFindingSummary,
) {
    let mut dimension_coverage: Vec<QualificationDimensionCoverage> =
        QualificationDimension::required_dimensions()
            .into_iter()
            .map(|dimension| QualificationDimensionCoverage {
                dimension,
                qualified: 0,
                not_applicable: 0,
                explicitly_narrowed: 0,
                unqualified: 0,
                failed: 0,
            })
            .collect();

    let mut class_counts: Vec<QualificationFindingCount> = Vec::new();
    let mut total = 0usize;

    for profile in profiles {
        for binding in &profile.bindings {
            if let Some(coverage) = dimension_coverage
                .iter_mut()
                .find(|row| row.dimension == binding.dimension)
            {
                match binding.status {
                    QualificationStatus::Qualified => coverage.qualified += 1,
                    QualificationStatus::NotApplicable => coverage.not_applicable += 1,
                    QualificationStatus::ExplicitlyNarrowed => coverage.explicitly_narrowed += 1,
                    QualificationStatus::Unqualified => coverage.unqualified += 1,
                    QualificationStatus::Failed => coverage.failed += 1,
                }
            }
        }
        for finding in &profile.blocking_findings {
            total += 1;
            let class = finding.class_token();
            if let Some(tally) = class_counts.iter_mut().find(|tally| tally.class == class) {
                tally.count += 1;
            } else {
                class_counts.push(QualificationFindingCount {
                    class: class.to_owned(),
                    count: 1,
                });
            }
        }
    }

    class_counts.sort_by(|left, right| left.class.cmp(&right.class));
    (
        dimension_coverage,
        QualificationFindingSummary {
            total_blocking_findings: total,
            by_class: class_counts,
        },
    )
}

/// Computes the marketed profiles release tooling should narrow because their
/// evidence is stale or a dimension is red/unproven.
fn compute_narrowable_profiles(profiles: &[NativeDesktopProfileRow]) -> Vec<NarrowableProfile> {
    let mut narrowable = Vec::new();
    for profile in profiles {
        if !profile.marketed {
            continue;
        }
        for finding in &profile.blocking_findings {
            narrowable.push(NarrowableProfile {
                profile_id: profile.descriptor.profile_id.clone(),
                dimension: finding.dimension(),
                reason: format!("blocking_finding:{}", finding.class_token()),
            });
        }
    }
    narrowable
}

/// Builds a [`NativeDesktopProfileRow`] from a descriptor and its bindings,
/// computing the per-profile blocking findings and derived claim state.
pub fn build_profile_row(
    descriptor: QualificationProfileDescriptor,
    bindings: Vec<QualificationDimensionBinding>,
) -> NativeDesktopProfileRow {
    let marketed = descriptor.marketed;
    let blocking_findings = compute_profile_findings(&descriptor, &bindings);
    let (claim_state, _reason) =
        derive_claim_state(&descriptor, &bindings, !blocking_findings.is_empty());

    NativeDesktopProfileRow {
        record_kind: QUALIFICATION_PROFILE_RECORD_KIND.to_owned(),
        schema_version: QUALIFICATION_SCHEMA_VERSION,
        shared_contract_ref: QUALIFICATION_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        claim_state,
        marketed,
    }
}

/// Builds the auto-narrowing claim scope from the finished profile rows.
fn compute_claim_scope(profiles: &[NativeDesktopProfileRow]) -> Vec<ProfileClaim> {
    profiles
        .iter()
        .map(|profile| {
            let (claim_state, reason) = derive_claim_state(
                &profile.descriptor,
                &profile.bindings,
                !profile.blocking_findings.is_empty(),
            );
            ProfileClaim {
                profile_id: profile.descriptor.profile_id.clone(),
                platform: profile.descriptor.platform,
                channel: profile.descriptor.channel,
                claim_state,
                qualified_dimensions: profile.qualified_dimensions(),
                narrowed_dimensions: profile.narrowed_dimensions(),
                blocked_dimensions: profile.blocked_dimensions(),
                evidence_freshness: profile.descriptor.evidence_freshness,
                downgrade_rule_ref: profile.descriptor.downgrade_rule_ref.clone(),
                reason,
            }
        })
        .collect()
}

/// Builds a full [`NativeDesktopQualificationReport`] from per-profile rows.
pub fn build_qualification_report(
    profiles: Vec<NativeDesktopProfileRow>,
) -> NativeDesktopQualificationReport {
    let mut profiles = profiles;
    profiles.sort_by(|left, right| left.descriptor.profile_id.cmp(&right.descriptor.profile_id));

    let registered_profile_count = profiles.len();
    let marketed_profile_count = profiles.iter().filter(|profile| profile.marketed).count();
    let dimensions_checked = profiles
        .iter()
        .map(|profile| profile.bindings.len())
        .sum::<usize>();

    let (dimension_coverage, findings_summary) = summarize_report(&profiles);
    let claim_scope = compute_claim_scope(&profiles);
    let narrowable_marketed_profiles = compute_narrowable_profiles(&profiles);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let published_claim_count = claim_scope
        .iter()
        .filter(|claim| claim.claim_state == ClaimState::Published)
        .count();
    let narrowed_claim_count = claim_scope
        .iter()
        .filter(|claim| claim.claim_state == ClaimState::Narrowed)
        .count();
    let withheld_claim_count = claim_scope
        .iter()
        .filter(|claim| claim.claim_state == ClaimState::Withheld)
        .count();

    let mut platform_set: Vec<DesktopPlatform> = Vec::new();
    for profile in &profiles {
        let platform = profile.descriptor.platform;
        if !platform_set.contains(&platform) {
            platform_set.push(platform);
        }
    }
    platform_set.sort();

    NativeDesktopQualificationReport {
        record_kind: QUALIFICATION_REPORT_RECORD_KIND.to_owned(),
        schema_version: QUALIFICATION_SCHEMA_VERSION,
        shared_contract_ref: QUALIFICATION_SHARED_CONTRACT_REF.to_owned(),
        report_id: QUALIFICATION_REPORT_ID.to_owned(),
        source_schema_ref: QUALIFICATION_SOURCE_SCHEMA_REF.to_owned(),
        required_dimensions: QualificationDimension::required_dimensions().to_vec(),
        required_drills: QualificationDrill::required_drills().to_vec(),
        claimed_platforms: platform_set,
        cross_links: QualificationCrossLinks::canonical(),
        profiles,
        dimension_coverage,
        claim_scope,
        findings_summary,
        registered_profile_count,
        marketed_profile_count,
        dimensions_checked,
        published_claim_count,
        narrowed_claim_count,
        withheld_claim_count,
        narrowable_marketed_profiles,
        report_clean,
        published_report_ref: QUALIFICATION_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: QUALIFICATION_PUBLISHED_DOC_REF.to_owned(),
        claim_packet_ref: QUALIFICATION_CLAIM_PACKET_REF.to_owned(),
        matrix_report_ref: QUALIFICATION_MATRIX_REPORT_REF.to_owned(),
        docs_help_refs: vec![
            QUALIFICATION_PUBLISHED_DOC_REF.to_owned(),
            "docs/help/native_desktop_integration.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-native-desktop-qualification".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_qualification_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum QualificationValidationError {
    /// The report has no registered profiles.
    NoRegisteredProfiles,
    /// A required dimension has no qualified profile.
    RequiredDimensionNotQualified { dimension: String },
    /// A profile is missing a required dimension from its binding set.
    MissingRequiredDimension {
        profile_id: String,
        dimension: String,
    },
    /// A blocking finding remains on the profile.
    BlockingFindingPresent {
        profile_id: String,
        dimension: String,
        class: String,
    },
    /// A cross-link ref is empty.
    CrossLinkMissing { field: String },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// The shiproom claim-packet ref is empty.
    ClaimPacketRefMissing,
    /// The matrix cross-link ref is empty.
    MatrixReportRefMissing,
    /// A profile's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { profile_id: String },
    /// A profile's derived claim state disagrees with its dimension
    /// qualification.
    ClaimStateDrift {
        profile_id: String,
        declared: String,
        derived: String,
    },
}

/// Validates a qualification report against the acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_qualification_report(
    report: &NativeDesktopQualificationReport,
) -> Result<(), Vec<QualificationValidationError>> {
    let mut errors = Vec::new();

    if report.profiles.is_empty() {
        errors.push(QualificationValidationError::NoRegisteredProfiles);
    }

    for dimension in QualificationDimension::required_dimensions() {
        let any_qualified = report.profiles.iter().any(|profile| {
            profile.bindings.iter().any(|binding| {
                binding.dimension == dimension && binding.status == QualificationStatus::Qualified
            })
        });
        if !any_qualified {
            errors.push(
                QualificationValidationError::RequiredDimensionNotQualified {
                    dimension: dimension.as_str().to_owned(),
                },
            );
        }
    }

    for profile in &report.profiles {
        for dimension in QualificationDimension::required_dimensions() {
            if !profile
                .bindings
                .iter()
                .any(|binding| binding.dimension == dimension)
            {
                errors.push(QualificationValidationError::MissingRequiredDimension {
                    profile_id: profile.descriptor.profile_id.clone(),
                    dimension: dimension.as_str().to_owned(),
                });
            }
        }
        if profile.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(QualificationValidationError::MissingDescriptorRevisionRef {
                profile_id: profile.descriptor.profile_id.clone(),
            });
        }
        // The claim state must equal the freshly derived state, so a row can
        // never publish a claim greener than its proof.
        let (derived, _reason) = derive_claim_state(
            &profile.descriptor,
            &profile.bindings,
            !profile.blocking_findings.is_empty(),
        );
        if derived != profile.claim_state {
            errors.push(QualificationValidationError::ClaimStateDrift {
                profile_id: profile.descriptor.profile_id.clone(),
                declared: profile.claim_state.as_str().to_owned(),
                derived: derived.as_str().to_owned(),
            });
        }
        for finding in &profile.blocking_findings {
            errors.push(QualificationValidationError::BlockingFindingPresent {
                profile_id: finding.profile_id().to_owned(),
                dimension: finding
                    .dimension()
                    .map(|dimension| dimension.as_str().to_owned())
                    .unwrap_or_else(|| "profile".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    for (field, value) in report.cross_links.as_pairs() {
        if value.trim().is_empty() {
            errors.push(QualificationValidationError::CrossLinkMissing {
                field: field.to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(QualificationValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(QualificationValidationError::PublishedDocRefMissing);
    }
    if report.claim_packet_ref.trim().is_empty() {
        errors.push(QualificationValidationError::ClaimPacketRefMissing);
    }
    if report.matrix_report_ref.trim().is_empty() {
        errors.push(QualificationValidationError::MatrixReportRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_native_desktop_qualification`].
struct ProfileSeed {
    profile_id: &'static str,
    platform: DesktopPlatform,
    channel: DeliveryChannel,
    descriptor_revision_ref: &'static str,
    display_label_ref: &'static str,
    channel_build_owner_ref: &'static str,
    ownership_kind: OwnershipKind,
    trust_checkpoint_ref: &'static str,
    continuity_note: &'static str,
    /// Dimensions this profile marks not applicable (everything else is
    /// qualified), with the honest reason.
    not_applicable_dimensions: &'static [(QualificationDimension, &'static str)],
}

/// Builds the dimension bindings for a seed: every required dimension is
/// qualified with a profile-scoped evidence pack unless the seed marks it not
/// applicable with a documented reason.
fn build_bindings_from_seed(seed: &ProfileSeed) -> Vec<QualificationDimensionBinding> {
    QualificationDimension::required_dimensions()
        .into_iter()
        .map(|dimension| {
            if let Some((_, reason)) = seed
                .not_applicable_dimensions
                .iter()
                .find(|(narrowed, _)| *narrowed == dimension)
            {
                QualificationDimensionBinding {
                    dimension,
                    required_drill: dimension.required_drill(),
                    status: QualificationStatus::NotApplicable,
                    failure_mode: None,
                    drill_ref: None,
                    evidence_pack_ref: None,
                    narrowing_reason: Some((*reason).to_owned()),
                    note: None,
                }
            } else {
                QualificationDimensionBinding {
                    dimension,
                    required_drill: dimension.required_drill(),
                    status: QualificationStatus::Qualified,
                    failure_mode: None,
                    drill_ref: Some(format!(
                        "drill:{}:{}",
                        seed.profile_id,
                        dimension.required_drill().as_str()
                    )),
                    evidence_pack_ref: Some(format!(
                        "evidence:{}:{}",
                        seed.profile_id,
                        dimension.as_str()
                    )),
                    narrowing_reason: None,
                    note: None,
                }
            }
        })
        .collect()
}

fn build_profile_from_seed(seed: &ProfileSeed) -> NativeDesktopProfileRow {
    let descriptor = QualificationProfileDescriptor {
        profile_id: seed.profile_id.to_owned(),
        platform: seed.platform,
        channel: seed.channel,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        display_label_ref: seed.display_label_ref.to_owned(),
        channel_build_owner_ref: seed.channel_build_owner_ref.to_owned(),
        ownership_kind: seed.ownership_kind,
        trust_checkpoint_ref: seed.trust_checkpoint_ref.to_owned(),
        continuity_note: seed.continuity_note.to_owned(),
        evidence_freshness: EvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:native_desktop_qualification:narrow_on_stale_or_red"
            .to_owned(),
        marketed: true,
        registered_on_qualification_harness: true,
    };
    let bindings = build_bindings_from_seed(seed);
    build_profile_row(descriptor, bindings)
}

// A portable build does not register OS-level handlers, so it narrows the two
// handler-ownership dimensions; its native-desktop claim is published with an
// explicit narrowed scope.
const PORTABLE_NON_REGISTERING: &[(QualificationDimension, &str)] = &[
    (
        QualificationDimension::ProtocolHandlerOwnership,
        "portable_build_registers_no_protocol_handler_so_protocol_handler_ownership_is_not_applicable",
    ),
    (
        QualificationDimension::FileAssociationOwnership,
        "portable_build_registers_no_file_association_so_file_association_ownership_is_not_applicable",
    ),
];

const PROFILE_SEEDS: &[ProfileSeed] = &[
    ProfileSeed {
        profile_id: "profile:macos.stable",
        platform: DesktopPlatform::Macos,
        channel: DeliveryChannel::Stable,
        descriptor_revision_ref: "profile-rev:macos.stable:2026.06.01-01",
        display_label_ref: "label:profile.macos.stable:primary",
        channel_build_owner_ref: "channel-owner:macos.stable.signed_app_bundle",
        ownership_kind: OwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:macos.stable.profile_tenant_policy",
        continuity_note: "The macOS stable channel owns its own protocol and file-association registrations, lands every reopen on the exact target, keeps lock-screen notifications summary-first, and recovers truthfully from a missing volume or a locked keychain.",
        not_applicable_dimensions: &[],
    },
    ProfileSeed {
        profile_id: "profile:windows.stable",
        platform: DesktopPlatform::Windows,
        channel: DeliveryChannel::Stable,
        descriptor_revision_ref: "profile-rev:windows.stable:2026.06.01-01",
        display_label_ref: "label:profile.windows.stable:primary",
        channel_build_owner_ref: "channel-owner:windows.stable.per_user_install",
        ownership_kind: OwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:windows.stable.profile_tenant_policy",
        continuity_note: "The Windows stable channel registers protocol and file associations per install with an inspectable owner, reopens jump-list and taskbar targets exactly, hides private notification detail on the lock screen, and recovers from a disconnected share or a locked credential vault.",
        not_applicable_dimensions: &[],
    },
    ProfileSeed {
        profile_id: "profile:linux.stable",
        platform: DesktopPlatform::Linux,
        channel: DeliveryChannel::Stable,
        descriptor_revision_ref: "profile-rev:linux.stable:2026.06.01-01",
        display_label_ref: "label:profile.linux.stable:primary",
        channel_build_owner_ref: "channel-owner:linux.stable.desktop_entry",
        ownership_kind: OwnershipKind::SharedDefaultArbitrated,
        trust_checkpoint_ref: "trust:linux.stable.profile_tenant_policy",
        continuity_note: "The Linux stable channel arbitrates the shared default desktop entry explicitly, lands recent-item reopens on the exact target, keeps notification content privacy-safe, and recovers truthfully from a missing mount or a locked secret service.",
        not_applicable_dimensions: &[],
    },
    ProfileSeed {
        profile_id: "profile:macos.beta",
        platform: DesktopPlatform::Macos,
        channel: DeliveryChannel::Beta,
        descriptor_revision_ref: "profile-rev:macos.beta:2026.06.01-01",
        display_label_ref: "label:profile.macos.beta:primary",
        channel_build_owner_ref: "channel-owner:macos.beta.signed_app_bundle",
        ownership_kind: OwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:macos.beta.profile_tenant_policy",
        continuity_note: "The macOS beta channel owns a side-by-side registration that cannot collide with stable, reopens the exact target through policy, keeps lock-screen copy summary-first, and recovers from a missing volume or a locked keychain with its own current proof.",
        not_applicable_dimensions: &[],
    },
    ProfileSeed {
        profile_id: "profile:windows.managed_fleet",
        platform: DesktopPlatform::Windows,
        channel: DeliveryChannel::ManagedFleet,
        descriptor_revision_ref: "profile-rev:windows.managed_fleet:2026.06.01-01",
        display_label_ref: "label:profile.windows.managed_fleet:primary",
        channel_build_owner_ref: "channel-owner:windows.managed_fleet.central_deployment",
        ownership_kind: OwnershipKind::ManagedFleetOwned,
        trust_checkpoint_ref: "trust:windows.managed_fleet.profile_tenant_policy",
        continuity_note: "The managed Windows fleet owns protocol and file-association registrations centrally with an inspectable owner, reopens fleet targets exactly under policy, suppresses private notification detail by admin policy, and recovers from a disconnected network home or a locked managed vault.",
        not_applicable_dimensions: &[],
    },
    ProfileSeed {
        profile_id: "profile:linux.portable",
        platform: DesktopPlatform::Linux,
        channel: DeliveryChannel::Portable,
        descriptor_revision_ref: "profile-rev:linux.portable:2026.06.01-01",
        display_label_ref: "label:profile.linux.portable:primary",
        channel_build_owner_ref: "channel-owner:linux.portable.appimage",
        ownership_kind: OwnershipKind::PortableNonRegistering,
        trust_checkpoint_ref: "trust:linux.portable.profile_tenant_policy",
        continuity_note: "The Linux portable build registers no OS-level handler, so protocol and file-association ownership are explicitly not claimed; it still lands recent-item reopens exactly, keeps notification content privacy-safe, and recovers from a missing mount or a locked secret service.",
        not_applicable_dimensions: PORTABLE_NON_REGISTERING,
    },
];

/// Seeded report builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/platform/m5-native-desktop-qualification/`.
pub fn seeded_native_desktop_qualification() -> NativeDesktopQualificationReport {
    let profiles = PROFILE_SEEDS.iter().map(build_profile_from_seed).collect();
    build_qualification_report(profiles)
}

/// Builds the seeded support-export wrapper.
pub fn seeded_qualification_support_export() -> NativeDesktopQualificationSupportExport {
    NativeDesktopQualificationSupportExport::from_report(
        QUALIFICATION_SUPPORT_EXPORT_ID,
        seeded_native_desktop_qualification(),
    )
}

/// Builds the seeded shiproom claim packet.
pub fn seeded_qualification_claim_packet() -> NativeDesktopClaimPacket {
    NativeDesktopClaimPacket::from_report(
        QUALIFICATION_CLAIM_PACKET_ID,
        seeded_native_desktop_qualification(),
    )
}
