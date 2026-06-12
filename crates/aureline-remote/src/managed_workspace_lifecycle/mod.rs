//! Managed-workspace lifecycle truth for remote, preview, companion, incident,
//! and support surfaces.
//!
//! This module produces a stable lifecycle proof packet that makes the
//! managed-workspace lifecycle a first-class reviewed concept on the remote and
//! companion lanes that provision, warm, suspend, resume, reconnect, rebuild,
//! recreate, expire, or hand off to a browser/companion surface.
//!
//! The packet embeds one [`LifecycleRecord`] per required lifecycle state and
//! derives a per-record disposition and an overall disposition. Each record
//! carries:
//!
//! - the lifecycle state and the prior state it transitioned from,
//! - the typed transition reason,
//! - the persistence class (and whether it changed materially),
//! - template and image provenance (and whether provenance changed),
//! - a stable target-identity ref and whether identity was preserved,
//! - the claimed continuity class relative to the prior runtime,
//! - the recovery options offered before the user loses context,
//! - expiry class and an opaque expiry-timing ref,
//! - whether local-safe continuation is available, and
//! - the typed caveat history that survives resume and reprovision.
//!
//! The lifecycle truth is projected onto every consuming surface (desktop,
//! preview route, companion handoff, incident packet, support export) so a
//! control-plane outage or expiry event degrades to an attributable local-safe
//! continuation rather than a generic loss-of-context failure.
//!
//! A record qualifies [`LifecycleDispositionClass::Truthful`] only when **all**
//! of the following hold:
//!
//! 1. No raw private material is exposed on the record.
//! 2. A stable target-identity ref is declared.
//! 3. The record reaches every required consuming surface.
//! 4. A claimed `exact_continuity` is not contradicted by a material change in
//!    persistence class, template/image provenance, or target identity.
//! 5. Any material change carries a non-empty caveat history.
//! 6. Outage and expiry states offer local-safe continuation and at least one
//!    recovery option.
//!
//! One condition forces [`LifecycleDispositionClass::Withheld`] immediately and
//! cannot be overridden: a record carries `raw_private_material_excluded:
//! false`.
//!
//! This packet implements the M5 guardrail directly: a managed resume path may
//! never imply exact continuity when the backing image, template, persistence
//! class, or target identity changed materially. It reuses the canonical
//! managed-workspace lifecycle vocabulary frozen at
//! `artifacts/runtime/managed_workspace_lifecycle.yaml` rather than inventing a
//! second continuity language for companion or preview surfaces.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque refs only. Raw credentials, raw private keys, raw image bytes, raw
//! endpoint URLs, and raw PII never appear on any record.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/remote/managed_workspace_lifecycle.md`
//! - Artifact: `artifacts/remote/managed_workspace_lifecycle.md`
//! - Schema: `schemas/remote/managed_workspace_lifecycle.schema.json`
//! - Contract ref: [`MANAGED_WORKSPACE_LIFECYCLE_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use aureline_auth::{
    secret_boundary_use_audit_result_for_health, seeded_secret_boundary_active_repair_state,
    seeded_secret_boundary_profile_parity_rows, seeded_secret_boundary_repairable_states,
    SecretBoundaryActingIdentityClass, SecretBoundaryConsumerIdentityClass,
    SecretBoundaryConsumerIdentityReceipt, SecretBoundaryCredentialMode,
    SecretBoundaryCredentialStateRow, SecretBoundaryDeclinePath,
    SecretBoundaryDelegatedCredentialRow, SecretBoundaryDelegatedUseClass,
    SecretBoundaryExportSafetyBanner, SecretBoundaryHealthStateClass,
    SecretBoundaryProjectionControl, SecretBoundaryProjectionControlClass,
    SecretBoundaryProjectionMode, SecretBoundaryProjectionModeAudit,
    SecretBoundaryRepairOwnerClass, SecretBoundarySecretAccessPrompt, SecretBoundarySecretClass,
    SecretBoundaryStorageClass, SecretBoundarySurfaceState, SecretBoundaryWorkflowDependency,
    M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const MANAGED_WORKSPACE_LIFECYCLE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const MANAGED_WORKSPACE_LIFECYCLE_SHARED_CONTRACT_REF: &str =
    "remote:managed_workspace_lifecycle:v1";

/// Record-kind tag for [`ManagedWorkspaceLifecyclePage`] payloads.
pub const MANAGED_WORKSPACE_LIFECYCLE_PAGE_RECORD_KIND: &str =
    "remote_managed_workspace_lifecycle_page_record";

/// Record-kind tag for [`LifecycleRecord`] payloads.
pub const MANAGED_WORKSPACE_LIFECYCLE_RECORD_KIND: &str =
    "remote_managed_workspace_lifecycle_record";

/// Record-kind tag for [`LifecycleMatrixRow`] payloads.
pub const MANAGED_WORKSPACE_LIFECYCLE_ROW_RECORD_KIND: &str =
    "remote_managed_workspace_lifecycle_row_record";

/// Record-kind tag for [`LifecycleDefect`] payloads.
pub const MANAGED_WORKSPACE_LIFECYCLE_DEFECT_RECORD_KIND: &str =
    "remote_managed_workspace_lifecycle_defect_record";

/// Record-kind tag for [`LifecycleSummary`] payloads.
pub const MANAGED_WORKSPACE_LIFECYCLE_SUMMARY_RECORD_KIND: &str =
    "remote_managed_workspace_lifecycle_summary_record";

/// Record-kind tag for [`LifecycleSupportExport`] payloads.
pub const MANAGED_WORKSPACE_LIFECYCLE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_managed_workspace_lifecycle_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const MANAGED_WORKSPACE_LIFECYCLE_DOC_REF: &str = "docs/remote/managed_workspace_lifecycle.md";

/// Repo-relative path of the artifact summary for this lane.
pub const MANAGED_WORKSPACE_LIFECYCLE_ARTIFACT_REF: &str =
    "artifacts/remote/managed_workspace_lifecycle.md";

/// All required lifecycle-state tokens in canonical order.
///
/// The packet must carry exactly one [`LifecycleRecord`] per token for a
/// fully reviewed lifecycle claim.
pub const REQUIRED_LIFECYCLE_STATES: [&str; 10] = [
    "provision",
    "warm",
    "ready",
    "suspended",
    "resumed",
    "reconnecting",
    "rebuild_required",
    "recreate_required",
    "expired",
    "local_safe_continuation",
];

/// All required consuming surfaces a lifecycle record must reach.
pub const REQUIRED_SURFACES: [&str; 5] = [
    "desktop",
    "preview_route",
    "companion_handoff",
    "incident_packet",
    "support_export",
];

/// Total required record count (one per lifecycle state).
pub const REQUIRED_RECORD_COUNT: usize = REQUIRED_LIFECYCLE_STATES.len();

// ---------------------------------------------------------------------------
// Lifecycle state vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the managed-workspace lifecycle states surfaced on the
/// M5 remote and companion lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStateClass {
    /// The control plane is allocating the workspace from a template or image;
    /// no runtime is reachable yet.
    Provision,
    /// The workspace is booting and warming caches; the runtime exists but is
    /// not yet ready for interactive work.
    Warm,
    /// The workspace is reachable and ready for interactive work.
    Ready,
    /// The workspace was paused by an idle window or explicit request; the
    /// persistent volume survives but the runtime is not executing.
    Suspended,
    /// The workspace was resumed from a suspended state; whether it implies
    /// exact continuity depends on whether the runtime changed materially.
    Resumed,
    /// The connection to a reachable workspace dropped and is being
    /// re-established; local-safe continuation applies during the gap.
    Reconnecting,
    /// The workspace must be rebuilt because a successor image, capsule drift,
    /// or volume drift makes the prior runtime non-resumable in place.
    RebuildRequired,
    /// The workspace must be recreated from scratch because its identity or
    /// backing volume can no longer be recovered.
    RecreateRequired,
    /// The workspace expired after an idle, hibernation, or hard-deadline
    /// window; the backing runtime is gone.
    Expired,
    /// The control plane is unreachable or the workspace is gone; work
    /// continues against a local-safe mirror with explicit caveats.
    LocalSafeContinuation,
}

impl LifecycleStateClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provision => "provision",
            Self::Warm => "warm",
            Self::Ready => "ready",
            Self::Suspended => "suspended",
            Self::Resumed => "resumed",
            Self::Reconnecting => "reconnecting",
            Self::RebuildRequired => "rebuild_required",
            Self::RecreateRequired => "recreate_required",
            Self::Expired => "expired",
            Self::LocalSafeContinuation => "local_safe_continuation",
        }
    }

    /// Human-readable state label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Provision => "Provisioning",
            Self::Warm => "Warming",
            Self::Ready => "Ready",
            Self::Suspended => "Suspended",
            Self::Resumed => "Resumed",
            Self::Reconnecting => "Reconnecting",
            Self::RebuildRequired => "Rebuild required",
            Self::RecreateRequired => "Recreate required",
            Self::Expired => "Expired",
            Self::LocalSafeContinuation => "Local-safe continuation",
        }
    }

    /// Returns `true` when this state represents a control-plane outage, an
    /// expiry, or a recovery transition that must offer local-safe
    /// continuation and at least one recovery option before the user loses
    /// context.
    pub const fn requires_local_safe_continuation(self) -> bool {
        matches!(
            self,
            Self::Reconnecting
                | Self::RebuildRequired
                | Self::RecreateRequired
                | Self::Expired
                | Self::LocalSafeContinuation
        )
    }
}

// ---------------------------------------------------------------------------
// Persistence class vocabulary
// ---------------------------------------------------------------------------

/// Persistence class describing how a workspace's state is backed.
///
/// Making the persistence class explicit lets a resumed or reprovisioned
/// surface state plainly whether the prior filesystem survived, was restored
/// from a snapshot, or was discarded for a fresh build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistenceClass {
    /// A durable volume survives suspend, resume, and reconnect.
    PersistentVolume,
    /// Scratch storage that is discarded on suspend or expiry.
    EphemeralScratch,
    /// State restored from a point-in-time snapshot; not guaranteed identical
    /// to the live runtime at suspend time.
    SnapshotRestored,
    /// A fresh build on a successor image; prior scratch state is gone.
    RebuiltFresh,
    /// A new workspace identity with no carried-over backing state.
    RecreatedNew,
    /// A local mirror of the last validated state; managed runtime is absent.
    LocalMirror,
}

impl PersistenceClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistentVolume => "persistent_volume",
            Self::EphemeralScratch => "ephemeral_scratch",
            Self::SnapshotRestored => "snapshot_restored",
            Self::RebuiltFresh => "rebuilt_fresh",
            Self::RecreatedNew => "recreated_new",
            Self::LocalMirror => "local_mirror",
        }
    }
}

// ---------------------------------------------------------------------------
// Provenance vocabulary
// ---------------------------------------------------------------------------

/// Provenance posture for the template or image backing a workspace.
///
/// Template and image provenance is exposed before a user loses context so a
/// resume that lands on a successor image or a drifted template is never
/// presented as exact continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClass {
    /// Pinned to an exact content digest; reproducible.
    PinnedDigest,
    /// Pinned to a mutable tag; reproducible only while the tag is stable.
    PinnedTag,
    /// A newer successor image or template superseded the prior one.
    SuccessorImage,
    /// The template or image drifted from its declared pin.
    DriftedUnpinned,
    /// Provenance cannot be determined (e.g. local-safe continuation).
    Unknown,
}

impl ProvenanceClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedDigest => "pinned_digest",
            Self::PinnedTag => "pinned_tag",
            Self::SuccessorImage => "successor_image",
            Self::DriftedUnpinned => "drifted_unpinned",
            Self::Unknown => "unknown",
        }
    }
}

// ---------------------------------------------------------------------------
// Continuity class vocabulary
// ---------------------------------------------------------------------------

/// Claimed relationship between a resumed/reprovisioned runtime and the prior
/// one.
///
/// A surface may claim [`Self::ExactContinuity`] only when no material change
/// occurred. The audit enforces this: claiming exact continuity over a changed
/// persistence class, changed provenance, or changed target identity is a
/// guardrail violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityClass {
    /// The runtime is the same instance with the same backing state; the user
    /// loses no context.
    ExactContinuity,
    /// The runtime changed materially (image, template, persistence class, or
    /// identity); continuity is partial and carries caveats.
    MaterialChange,
    /// A fresh runtime with no carried-over state; there is no continuity.
    FreshNoContinuity,
    /// Only a local-safe mirror remains; managed continuity is unavailable.
    LocalSafeOnly,
}

impl ContinuityClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactContinuity => "exact_continuity",
            Self::MaterialChange => "material_change",
            Self::FreshNoContinuity => "fresh_no_continuity",
            Self::LocalSafeOnly => "local_safe_only",
        }
    }

    /// Returns `true` when this claim asserts exact continuity with the prior
    /// runtime.
    pub const fn claims_exact_continuity(self) -> bool {
        matches!(self, Self::ExactContinuity)
    }
}

// ---------------------------------------------------------------------------
// Transition reason vocabulary
// ---------------------------------------------------------------------------

/// Typed reason a workspace transitioned into its current lifecycle state.
///
/// Reuses the canonical reason vocabulary frozen at
/// `artifacts/runtime/managed_workspace_lifecycle.yaml`; silent transitions are
/// non-conforming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionReasonClass {
    /// A user requested workspace creation.
    UserRequestedCreate,
    /// A user requested a resume from a suspended state.
    UserRequestedResume,
    /// A user requested a suspend/pause.
    UserRequestedSuspend,
    /// An idle window elapsed and the control plane suspended the workspace.
    IdleWindowElapsed,
    /// A hibernation window elapsed and the workspace expired.
    HibernationWindowElapsed,
    /// A successor image became available; the workspace must rebuild.
    SuccessorImageAvailable,
    /// Capsule or volume drift was detected; the runtime is non-resumable.
    CapsuleDriftDetected,
    /// The control plane became unreachable.
    ControlPlaneFailure,
    /// The control plane recovered and the connection is being re-established.
    ControlPlaneRecovered,
    /// A hard expiry deadline was reached.
    ExpiryDeadlineReached,
}

impl TransitionReasonClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserRequestedCreate => "user_requested_create",
            Self::UserRequestedResume => "user_requested_resume",
            Self::UserRequestedSuspend => "user_requested_suspend",
            Self::IdleWindowElapsed => "idle_window_elapsed",
            Self::HibernationWindowElapsed => "hibernation_window_elapsed",
            Self::SuccessorImageAvailable => "successor_image_available",
            Self::CapsuleDriftDetected => "capsule_drift_detected",
            Self::ControlPlaneFailure => "control_plane_failure",
            Self::ControlPlaneRecovered => "control_plane_recovered",
            Self::ExpiryDeadlineReached => "expiry_deadline_reached",
        }
    }
}

// ---------------------------------------------------------------------------
// Recovery option vocabulary
// ---------------------------------------------------------------------------

/// Recovery option offered to the user before context is lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryOptionClass {
    /// Resume the suspended workspace in place.
    Resume,
    /// Re-establish the dropped connection.
    Reconnect,
    /// Rebuild the workspace on the current or successor image.
    Rebuild,
    /// Recreate the workspace from scratch under a new identity.
    Recreate,
    /// Continue locally against the last validated mirror.
    LocalSafeContinue,
    /// Escalate to the operator who owns the control plane.
    ContactOperator,
}

impl RecoveryOptionClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resume => "resume",
            Self::Reconnect => "reconnect",
            Self::Rebuild => "rebuild",
            Self::Recreate => "recreate",
            Self::LocalSafeContinue => "local_safe_continue",
            Self::ContactOperator => "contact_operator",
        }
    }
}

// ---------------------------------------------------------------------------
// Expiry class vocabulary
// ---------------------------------------------------------------------------

/// Typed expiry posture describing what window, if any, governs the workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryClass {
    /// No expiry window applies to this state.
    None,
    /// An idle window governs suspension.
    IdleWindow,
    /// A hibernation window governs expiry after suspension.
    HibernationWindow,
    /// A hard deadline governs expiry regardless of activity.
    HardDeadline,
    /// A control-plane outage clock governs the local-safe grace window.
    ControlPlaneOutage,
}

impl ExpiryClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::IdleWindow => "idle_window",
            Self::HibernationWindow => "hibernation_window",
            Self::HardDeadline => "hard_deadline",
            Self::ControlPlaneOutage => "control_plane_outage",
        }
    }
}

// ---------------------------------------------------------------------------
// Caveat vocabulary
// ---------------------------------------------------------------------------

/// Typed caveat that survives a resume or reprovision so the user is never
/// shown implied exact continuity over a changed runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaveatClass {
    /// The persistence class changed relative to the prior runtime.
    PersistenceClassChanged,
    /// The backing template changed relative to the prior runtime.
    TemplateChanged,
    /// The backing image changed relative to the prior runtime.
    ImageChanged,
    /// The target identity changed relative to the prior runtime.
    TargetIdentityChanged,
    /// A session or credential re-authentication is required before mutation.
    SessionReauthRequired,
    /// Scratch state was discarded and is not recoverable.
    ScratchStateDiscarded,
    /// Only a local-safe mirror remains; managed capabilities are unavailable.
    LocalSafeOnly,
}

impl CaveatClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistenceClassChanged => "persistence_class_changed",
            Self::TemplateChanged => "template_changed",
            Self::ImageChanged => "image_changed",
            Self::TargetIdentityChanged => "target_identity_changed",
            Self::SessionReauthRequired => "session_reauth_required",
            Self::ScratchStateDiscarded => "scratch_state_discarded",
            Self::LocalSafeOnly => "local_safe_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Surface vocabulary
// ---------------------------------------------------------------------------

/// Consuming surface that must render the lifecycle truth consistently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// The desktop shell where the user originates the lifecycle action.
    Desktop,
    /// A preview route reachable through the managed workspace.
    PreviewRoute,
    /// A companion (browser/mobile) handoff surface.
    CompanionHandoff,
    /// An incident packet emitted when the control plane degrades.
    IncidentPacket,
    /// A support export consumed by diagnostics and release evidence.
    SupportExport,
}

impl SurfaceClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::PreviewRoute => "preview_route",
            Self::CompanionHandoff => "companion_handoff",
            Self::IncidentPacket => "incident_packet",
            Self::SupportExport => "support_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Disposition vocabulary
// ---------------------------------------------------------------------------

/// Published disposition for the overall packet and individual records.
///
/// The disposition is derived, not asserted: it is set by comparing the audit
/// defect list against the truthfulness conditions. A caller may never bump a
/// record to [`Self::Truthful`] without a clean audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleDispositionClass {
    /// All truthfulness conditions hold; the lifecycle claim is published.
    Truthful,
    /// A continuity, caveat, or local-safe-continuation condition narrowed the
    /// claim; the lifecycle truth is published with the narrowing made
    /// explicit.
    Narrowed,
    /// A coverage gap (missing required state or surface) flags the record for
    /// review.
    Flagged,
    /// Raw private material was exposed or target identity was undeclared; the
    /// claim is withheld.
    Withheld,
}

impl LifecycleDispositionClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Truthful => "truthful",
            Self::Narrowed => "narrowed",
            Self::Flagged => "flagged",
            Self::Withheld => "withheld",
        }
    }

    /// Returns `true` when this disposition publishes the unqualified
    /// lifecycle claim.
    pub const fn is_truthful(self) -> bool {
        matches!(self, Self::Truthful)
    }
}

// ---------------------------------------------------------------------------
// Narrow reason vocabulary
// ---------------------------------------------------------------------------

/// Typed reason a record or the packet was narrowed below
/// [`LifecycleDispositionClass::Truthful`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowReasonClass {
    /// No narrowing — the record is truthful.
    NotNarrowed,
    /// A record carries `raw_private_material_excluded: false`; the claim is
    /// withheld immediately.
    RawPrivateMaterialExposed,
    /// A record does not declare a stable target-identity ref.
    TargetIdentityUndeclared,
    /// A required lifecycle state is absent from the snapshot.
    RequiredStateMissing,
    /// A record does not reach every required consuming surface.
    SurfaceCoverageIncomplete,
    /// A record claims `exact_continuity` over a material change.
    ContinuityOverclaim,
    /// A material change carries no caveat history.
    CaveatHistoryMissing,
    /// An outage or expiry state offers no local-safe continuation or no
    /// recovery option.
    LocalSafeContinuationUnavailable,
}

impl NarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::TargetIdentityUndeclared => "target_identity_undeclared",
            Self::RequiredStateMissing => "required_state_missing",
            Self::SurfaceCoverageIncomplete => "surface_coverage_incomplete",
            Self::ContinuityOverclaim => "continuity_overclaim",
            Self::CaveatHistoryMissing => "caveat_history_missing",
            Self::LocalSafeContinuationUnavailable => "local_safe_continuation_unavailable",
        }
    }

    /// Returns the disposition this reason forces.
    pub const fn disposition(self) -> LifecycleDispositionClass {
        match self {
            Self::NotNarrowed => LifecycleDispositionClass::Truthful,
            Self::RawPrivateMaterialExposed | Self::TargetIdentityUndeclared => {
                LifecycleDispositionClass::Withheld
            }
            Self::RequiredStateMissing | Self::SurfaceCoverageIncomplete => {
                LifecycleDispositionClass::Flagged
            }
            Self::ContinuityOverclaim
            | Self::CaveatHistoryMissing
            | Self::LocalSafeContinuationUnavailable => LifecycleDispositionClass::Narrowed,
        }
    }

    /// Returns `true` when this reason forces an immediate withhold.
    pub const fn is_withhold_reason(self) -> bool {
        matches!(self.disposition(), LifecycleDispositionClass::Withheld)
    }
}

// ---------------------------------------------------------------------------
// Lifecycle record (per lifecycle state)
// ---------------------------------------------------------------------------

/// Per-state managed-workspace lifecycle record.
///
/// Each record represents one lifecycle state and the transition that produced
/// it, plus the persistence, provenance, continuity, recovery, expiry, and
/// caveat truth that every consuming surface must render identically.
///
/// No raw credentials, raw private keys, raw image bytes, raw endpoint URLs, or
/// raw PII may appear on this record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Lifecycle state for this record.
    pub lifecycle_state: LifecycleStateClass,
    /// Stable token for [`Self::lifecycle_state`]; also the record's row key.
    pub lifecycle_state_token: String,
    /// Prior lifecycle state, when this state was reached by transition.
    pub prior_lifecycle_state: Option<LifecycleStateClass>,
    /// Stable token for [`Self::prior_lifecycle_state`]; `None` when absent.
    pub prior_lifecycle_state_token: Option<String>,
    /// Typed reason for the transition into this state.
    pub transition_reason: TransitionReasonClass,
    /// Stable token for [`Self::transition_reason`].
    pub transition_reason_token: String,
    /// Persistence class backing the workspace in this state.
    pub persistence_class: PersistenceClass,
    /// Stable token for [`Self::persistence_class`].
    pub persistence_class_token: String,
    /// `true` when the persistence class changed materially from the prior
    /// runtime.
    pub persistence_changed: bool,
    /// Provenance of the backing template.
    pub template_provenance: ProvenanceClass,
    /// Stable token for [`Self::template_provenance`].
    pub template_provenance_token: String,
    /// Provenance of the backing image.
    pub image_provenance: ProvenanceClass,
    /// Stable token for [`Self::image_provenance`].
    pub image_provenance_token: String,
    /// `true` when the template or image provenance changed materially.
    pub provenance_changed: bool,
    /// Opaque, stable ref for the workspace target identity. Never a raw URL,
    /// hostname, or credential.
    pub target_identity_ref: String,
    /// `true` when the target identity was preserved across the transition.
    pub target_identity_preserved: bool,
    /// Claimed continuity class relative to the prior runtime.
    pub continuity_class: ContinuityClass,
    /// Stable token for [`Self::continuity_class`].
    pub continuity_class_token: String,
    /// Recovery options offered to the user before context is lost.
    pub recovery_options: Vec<RecoveryOptionClass>,
    /// Typed expiry posture for this state.
    pub expiry_class: ExpiryClass,
    /// Stable token for [`Self::expiry_class`].
    pub expiry_class_token: String,
    /// Opaque ref for the governing expiry window or timer. `None` when no
    /// expiry window applies.
    pub expiry_timing_ref: Option<String>,
    /// `true` when local-safe continuation is available in this state.
    pub local_safe_continuation_available: bool,
    /// Typed caveat history that survives resume and reprovision.
    pub caveat_history: Vec<CaveatClass>,
    /// Consuming surfaces that render this lifecycle truth.
    pub surfaces_present: Vec<SurfaceClass>,
    /// `true` when no raw credentials, raw private keys, or raw PII are present
    /// on this record. Must be `true` for the truthful claim to hold.
    pub raw_private_material_excluded: bool,
    /// Plain-language summary safe for UI, support exports, and diagnostics.
    pub summary: String,
}

impl LifecycleRecord {
    /// Construct a lifecycle record, populating all derived token fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lifecycle_state: LifecycleStateClass,
        prior_lifecycle_state: Option<LifecycleStateClass>,
        transition_reason: TransitionReasonClass,
        persistence_class: PersistenceClass,
        persistence_changed: bool,
        template_provenance: ProvenanceClass,
        image_provenance: ProvenanceClass,
        provenance_changed: bool,
        target_identity_ref: impl Into<String>,
        target_identity_preserved: bool,
        continuity_class: ContinuityClass,
        recovery_options: Vec<RecoveryOptionClass>,
        expiry_class: ExpiryClass,
        expiry_timing_ref: Option<impl Into<String>>,
        local_safe_continuation_available: bool,
        caveat_history: Vec<CaveatClass>,
        surfaces_present: Vec<SurfaceClass>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: MANAGED_WORKSPACE_LIFECYCLE_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_LIFECYCLE_SCHEMA_VERSION,
            shared_contract_ref: MANAGED_WORKSPACE_LIFECYCLE_SHARED_CONTRACT_REF.to_owned(),
            lifecycle_state,
            lifecycle_state_token: lifecycle_state.as_str().to_owned(),
            prior_lifecycle_state,
            prior_lifecycle_state_token: prior_lifecycle_state.map(|s| s.as_str().to_owned()),
            transition_reason,
            transition_reason_token: transition_reason.as_str().to_owned(),
            persistence_class,
            persistence_class_token: persistence_class.as_str().to_owned(),
            persistence_changed,
            template_provenance,
            template_provenance_token: template_provenance.as_str().to_owned(),
            image_provenance,
            image_provenance_token: image_provenance.as_str().to_owned(),
            provenance_changed,
            target_identity_ref: target_identity_ref.into(),
            target_identity_preserved,
            continuity_class,
            continuity_class_token: continuity_class.as_str().to_owned(),
            recovery_options,
            expiry_class,
            expiry_class_token: expiry_class.as_str().to_owned(),
            expiry_timing_ref: expiry_timing_ref.map(Into::into),
            local_safe_continuation_available,
            caveat_history,
            surfaces_present,
            raw_private_material_excluded: true,
            summary: summary.into(),
        }
    }

    /// Returns `true` when this record reflects a material change in the
    /// backing runtime relative to the prior one.
    pub fn has_material_change(&self) -> bool {
        self.persistence_changed || self.provenance_changed || !self.target_identity_preserved
    }
}

// ---------------------------------------------------------------------------
// Lifecycle snapshot (aggregate of all records)
// ---------------------------------------------------------------------------

/// Aggregate of all lifecycle records in the packet.
///
/// The snapshot must contain one record per required lifecycle state to satisfy
/// the truthful claim. A missing required state flags the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleSnapshot {
    /// All lifecycle records in this snapshot.
    pub records: Vec<LifecycleRecord>,
}

impl LifecycleSnapshot {
    /// Returns the record for the given lifecycle-state token, if present.
    pub fn record_for_state(&self, state_token: &str) -> Option<&LifecycleRecord> {
        self.records
            .iter()
            .find(|r| r.lifecycle_state_token == state_token)
    }

    /// Returns the set of lifecycle-state tokens covered by this snapshot.
    pub fn covered_states(&self) -> BTreeSet<&str> {
        self.records
            .iter()
            .map(|r| r.lifecycle_state_token.as_str())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Lifecycle matrix row (derived disposition row)
// ---------------------------------------------------------------------------

/// Derived disposition row for one lifecycle record in the proof packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleMatrixRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Lifecycle-state token for this row.
    pub lifecycle_state_token: String,
    /// Prior lifecycle-state token, when present.
    pub prior_lifecycle_state_token: Option<String>,
    /// Transition-reason token from the record.
    pub transition_reason_token: String,
    /// Persistence-class token from the record.
    pub persistence_class_token: String,
    /// Continuity-class token from the record.
    pub continuity_class_token: String,
    /// `true` when the record reflects a material runtime change.
    pub material_change: bool,
    /// `true` when local-safe continuation is available.
    pub local_safe_continuation_available: bool,
    /// `true` when raw private material is excluded from the record.
    pub raw_private_material_excluded: bool,
    /// Derived disposition token.
    pub disposition_token: String,
    /// Why the row was narrowed (or `not_narrowed` when truthful).
    pub narrow_reason_token: String,
    /// Plain-language summary of the disposition for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the lifecycle page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LifecycleSummary {
    /// Total record count.
    pub record_count: usize,
    /// Records published as truthful.
    pub truthful_count: usize,
    /// Records narrowed.
    pub narrowed_count: usize,
    /// Records flagged.
    pub flagged_count: usize,
    /// Records withheld.
    pub withheld_count: usize,
    /// Lifecycle-state tokens covered by the snapshot.
    pub states_covered: Vec<String>,
    /// Number of records reflecting a material runtime change.
    pub material_change_count: usize,
    /// Number of records offering local-safe continuation.
    pub local_safe_continuation_count: usize,
    /// Overall disposition token derived from all rows.
    pub overall_disposition_token: String,
}

impl LifecycleSummary {
    fn from_rows(
        rows: &[LifecycleMatrixRow],
        snapshot: &LifecycleSnapshot,
        defects: &[LifecycleDefect],
    ) -> Self {
        let mut truthful = 0usize;
        let mut narrowed = 0usize;
        let mut flagged = 0usize;
        let mut withheld = 0usize;
        for row in rows {
            match row.disposition_token.as_str() {
                "truthful" => truthful += 1,
                "narrowed" => narrowed += 1,
                "flagged" => flagged += 1,
                "withheld" => withheld += 1,
                _ => {}
            }
        }
        // The overall disposition is the most severe of the per-row dispositions
        // and any page-level defect (e.g. a missing required state, which
        // produces no row but must still narrow the packet).
        let mut overall = if withheld > 0 {
            LifecycleDispositionClass::Withheld
        } else if flagged > 0 {
            LifecycleDispositionClass::Flagged
        } else if narrowed > 0 {
            LifecycleDispositionClass::Narrowed
        } else {
            LifecycleDispositionClass::Truthful
        };
        for defect in defects {
            let defect_disposition = defect.narrow_reason.disposition();
            if defect_disposition > overall {
                overall = defect_disposition;
            }
        }
        let states_covered: Vec<String> = snapshot
            .records
            .iter()
            .map(|r| r.lifecycle_state_token.clone())
            .collect();
        let material_change_count = snapshot
            .records
            .iter()
            .filter(|r| r.has_material_change())
            .count();
        let local_safe_continuation_count = snapshot
            .records
            .iter()
            .filter(|r| r.local_safe_continuation_available)
            .count();
        Self {
            record_count: rows.len(),
            truthful_count: truthful,
            narrowed_count: narrowed,
            flagged_count: flagged,
            withheld_count: withheld,
            states_covered,
            material_change_count,
            local_safe_continuation_count,
            overall_disposition_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the lifecycle audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: NarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject lifecycle-state token or `page`.
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl LifecycleDefect {
    fn new(
        narrow_reason: NarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: MANAGED_WORKSPACE_LIFECYCLE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_LIFECYCLE_SCHEMA_VERSION,
            shared_contract_ref: MANAGED_WORKSPACE_LIFECYCLE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:managed-workspace-lifecycle:{}:{}",
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
// Lifecycle page (proof packet)
// ---------------------------------------------------------------------------

/// Stable proof packet for the managed-workspace lifecycle lane.
///
/// The page is the single inspectable record that proves lifecycle truth for
/// the M5 remote and companion flows. Desktop, preview, companion, incident,
/// and support/export surfaces should ingest this packet rather than
/// re-describing lifecycle state with subsystem-specific status strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecyclePage {
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
    pub summary: LifecycleSummary,
    /// Per-record disposition entries.
    pub rows: Vec<LifecycleMatrixRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<LifecycleDefect>,
    /// The lifecycle snapshot embedded as evidence.
    pub snapshot: LifecycleSnapshot,
}

impl ManagedWorkspaceLifecyclePage {
    /// Build the lifecycle page from a snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        snapshot: LifecycleSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&snapshot);
        let rows = derive_rows(&snapshot, &defects);
        let summary = LifecycleSummary::from_rows(&rows, &snapshot, &defects);
        Self {
            record_kind: MANAGED_WORKSPACE_LIFECYCLE_PAGE_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_LIFECYCLE_SCHEMA_VERSION,
            shared_contract_ref: MANAGED_WORKSPACE_LIFECYCLE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            snapshot,
        }
    }

    /// Returns `true` when the overall disposition is truthful.
    pub fn publishes_truthful(&self) -> bool {
        self.summary.overall_disposition_token == LifecycleDispositionClass::Truthful.as_str()
    }

    /// Returns `true` when no withheld rows are present.
    pub fn no_withheld_rows(&self) -> bool {
        self.summary.withheld_count == 0
    }

    /// Returns `true` when all required lifecycle states are covered.
    pub fn covers_all_required_states(&self) -> bool {
        let covered = self.snapshot.covered_states();
        REQUIRED_LIFECYCLE_STATES
            .iter()
            .all(|s| covered.contains(s))
    }

    /// Returns `true` when every record reaches every required surface.
    pub fn all_records_reach_required_surfaces(&self) -> bool {
        self.snapshot
            .records
            .iter()
            .all(record_reaches_all_surfaces)
    }

    /// Returns `true` when no record claims exact continuity over a material
    /// change.
    pub fn no_continuity_overclaim(&self) -> bool {
        self.snapshot
            .records
            .iter()
            .all(|r| !(r.continuity_class.claims_exact_continuity() && r.has_material_change()))
    }

    /// Returns `true` when every material-change record carries a caveat
    /// history.
    pub fn all_material_changes_carry_caveats(&self) -> bool {
        self.snapshot
            .records
            .iter()
            .all(|r| !r.has_material_change() || !r.caveat_history.is_empty())
    }

    /// Returns `true` when every outage/expiry state offers local-safe
    /// continuation and at least one recovery option.
    pub fn all_outage_states_offer_local_safe_continuation(&self) -> bool {
        self.snapshot.records.iter().all(|r| {
            if r.lifecycle_state.requires_local_safe_continuation() {
                r.local_safe_continuation_available && !r.recovery_options.is_empty()
            } else {
                true
            }
        })
    }

    /// Projects the shared M5 secret-boundary state for the managed runtime lane.
    pub fn secret_boundary_states(&self) -> Vec<SecretBoundarySurfaceState> {
        let Some(record) = self
            .snapshot
            .record_for_state(LifecycleStateClass::Ready.as_str())
        else {
            return Vec::new();
        };

        let target_label = format!("Managed workspace {}", record.target_identity_ref);
        let decline_path = SecretBoundaryDeclinePath {
            decline_label: "Continue with local-safe mirror".to_owned(),
            still_works_summary:
                "Declining keeps local editing and lifecycle review available while managed runtime actions stay closed."
                    .to_owned(),
        };
        let workflows = vec![
            managed_runtime_workflow("workflow:managed.runtime.resume", "Resume managed runtime"),
            managed_runtime_workflow(
                "workflow:managed.runtime.repair",
                "Repair managed runtime auth or host proof",
            ),
        ];
        let health_state = managed_runtime_health_state(record);
        let projection_controls = managed_runtime_projection_controls();
        let audit_result = secret_boundary_use_audit_result_for_health(health_state);

        vec![SecretBoundarySurfaceState {
            matrix_row_id: "m5.secret.managed.workspace_runtime".to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            secret_access_prompt: SecretBoundarySecretAccessPrompt {
                matrix_row_id: "m5.secret.managed.workspace_runtime".to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                requester_label: "Managed workspace runtime".to_owned(),
                secret_class: SecretBoundarySecretClass::CloudDelegatedIdentity,
                target_workflow_label: target_label.clone(),
                storage_class: SecretBoundaryStorageClass::RemoteVault,
                credential_mode: SecretBoundaryCredentialMode::RemoteVaultFetch,
                projection_mode: SecretBoundaryProjectionMode::RemoteVaultFetch,
                lifetime_label: "Managed runtime credential lease".to_owned(),
                expires_at: record.expiry_timing_ref.clone(),
                dependent_workflows: workflows.clone(),
                decline_path: decline_path.clone(),
            },
            credential_state_row: SecretBoundaryCredentialStateRow {
                matrix_row_id: "m5.secret.managed.workspace_runtime".to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                display_label: "Managed runtime credential state".to_owned(),
                secret_class: SecretBoundarySecretClass::CloudDelegatedIdentity,
                source_class: SecretBoundaryCredentialMode::RemoteVaultFetch,
                target_boundary_label: target_label.clone(),
                storage_class: SecretBoundaryStorageClass::RemoteVault,
                projection_mode: SecretBoundaryProjectionMode::RemoteVaultFetch,
                health_state,
                expires_at: record.expiry_timing_ref.clone(),
                rotate_action_label: "Rotate managed runtime lease".to_owned(),
                revoke_action_label: "Revoke managed runtime credential".to_owned(),
                test_action_label: "Validate managed runtime trust".to_owned(),
                dependent_workflows: workflows,
                decline_path,
            },
            vault_picker: None,
            delegated_credential_row: Some(SecretBoundaryDelegatedCredentialRow {
                matrix_row_id: "m5.secret.managed.workspace_runtime".to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                delegated_use_class: SecretBoundaryDelegatedUseClass::RemoteVaultFetch,
                target_host_or_workspace_label: target_label,
                expires_at: record.expiry_timing_ref.clone(),
                policy_owner_label: "Managed workspace control plane".to_owned(),
                projection_controls: projection_controls.clone(),
            }),
            consumer_identity_receipt: SecretBoundaryConsumerIdentityReceipt::new(
                "m5.secret.managed.workspace_runtime:consumer-receipt",
                "m5.secret.managed.workspace_runtime",
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                SecretBoundaryConsumerIdentityClass::RemoteHelper,
                "Managed workspace control plane",
                format!("Managed workspace {}", record.target_identity_ref),
                SecretBoundaryCredentialMode::RemoteVaultFetch,
                SecretBoundaryProjectionMode::RemoteVaultFetch,
                SecretBoundaryStorageClass::RemoteVault,
                audit_result,
            ),
            projection_mode_audit: SecretBoundaryProjectionModeAudit::new(
                "m5.secret.managed.workspace_runtime:projection-audit",
                "m5.secret.managed.workspace_runtime",
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                SecretBoundaryConsumerIdentityClass::RemoteHelper,
                "Managed workspace control plane",
                format!("Managed workspace {}", record.target_identity_ref),
                SecretBoundaryProjectionMode::RemoteVaultFetch,
                audit_result,
                SecretBoundaryRepairOwnerClass::RemoteOperator,
                projection_controls
                    .iter()
                    .map(|control| control.control_class)
                    .collect(),
            ),
            repairable_states: seeded_secret_boundary_repairable_states(
                "m5.secret.managed.workspace_runtime",
            ),
            active_repair_state: seeded_secret_boundary_active_repair_state(
                "m5.secret.managed.workspace_runtime",
                health_state,
            ),
            profile_parity_rows: seeded_secret_boundary_profile_parity_rows(
                "m5.secret.managed.workspace_runtime",
            ),
            export_safety_banner: SecretBoundaryExportSafetyBanner::standard(
                "m5.secret.managed.workspace_runtime",
                "Raw managed-runtime credentials, vault material, and host proofs stay excluded from support bundles and lifecycle exports.",
            ),
        }]
    }
}

fn managed_runtime_projection_controls() -> Vec<SecretBoundaryProjectionControl> {
    let local_safe_note =
        "Local editing and bounded lifecycle review remain available while managed auth is closed.";
    vec![
        SecretBoundaryProjectionControl::new(
            "m5.secret.managed.workspace_runtime",
            SecretBoundaryProjectionControlClass::PauseForwarding,
            "Pause managed runtime auth",
            local_safe_note,
        ),
        SecretBoundaryProjectionControl::new(
            "m5.secret.managed.workspace_runtime",
            SecretBoundaryProjectionControlClass::StopUsingSecret,
            "Stop using managed runtime secret",
            local_safe_note,
        ),
        SecretBoundaryProjectionControl::new(
            "m5.secret.managed.workspace_runtime",
            SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            "Drop managed runtime delegate",
            local_safe_note,
        ),
    ]
}

fn managed_runtime_workflow(
    workflow_ref: impl Into<String>,
    workflow_label: impl Into<String>,
) -> SecretBoundaryWorkflowDependency {
    SecretBoundaryWorkflowDependency {
        workflow_ref: workflow_ref.into(),
        workflow_label: workflow_label.into(),
    }
}

fn managed_runtime_health_state(record: &LifecycleRecord) -> SecretBoundaryHealthStateClass {
    match record.lifecycle_state {
        LifecycleStateClass::Expired => SecretBoundaryHealthStateClass::Expired,
        LifecycleStateClass::Reconnecting | LifecycleStateClass::LocalSafeContinuation => {
            SecretBoundaryHealthStateClass::ForwardingPaused
        }
        LifecycleStateClass::RebuildRequired | LifecycleStateClass::RecreateRequired => {
            SecretBoundaryHealthStateClass::RemoteVaultUnavailable
        }
        _ => SecretBoundaryHealthStateClass::Healthy,
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the lifecycle page plus a metadata-safe
/// defect roll-up.
///
/// No raw credentials, raw private keys, raw image bytes, or raw PII may appear
/// in this export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleSupportExport {
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
    /// The lifecycle page embedded as evidence.
    pub page: ManagedWorkspaceLifecyclePage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<NarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl LifecycleSupportExport {
    /// Wrap a lifecycle page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: ManagedWorkspaceLifecyclePage,
    ) -> Self {
        let mut reasons: Vec<NarrowReasonClass> = Vec::new();
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
            record_kind: MANAGED_WORKSPACE_LIFECYCLE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_LIFECYCLE_SCHEMA_VERSION,
            shared_contract_ref: MANAGED_WORKSPACE_LIFECYCLE_SHARED_CONTRACT_REF.to_owned(),
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
// Public API functions
// ---------------------------------------------------------------------------

/// Re-run the lifecycle audit over the snapshot embedded in a page.
pub fn audit_lifecycle_page(page: &ManagedWorkspaceLifecyclePage) -> Vec<LifecycleDefect> {
    audit_snapshot(&page.snapshot)
}

/// Validate a lifecycle page; returns `Ok` when the audit is clean.
///
/// # Errors
///
/// Returns the defect list when one or more truthfulness conditions are
/// violated.
pub fn validate_lifecycle_page(
    page: &ManagedWorkspaceLifecyclePage,
) -> Result<(), Vec<LifecycleDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn record_reaches_all_surfaces(record: &LifecycleRecord) -> bool {
    let present: BTreeSet<&str> = record.surfaces_present.iter().map(|s| s.as_str()).collect();
    REQUIRED_SURFACES.iter().all(|s| present.contains(s))
}

fn audit_snapshot(snapshot: &LifecycleSnapshot) -> Vec<LifecycleDefect> {
    let mut defects: Vec<LifecycleDefect> = Vec::new();

    // Hard guardrail: raw private material exposed — withhold immediately.
    for record in &snapshot.records {
        if !record.raw_private_material_excluded {
            defects.push(LifecycleDefect::new(
                NarrowReasonClass::RawPrivateMaterialExposed,
                record.lifecycle_state_token.clone(),
                format!(
                    "state '{}' has raw_private_material_excluded: false; the lifecycle \
                     claim is withheld",
                    record.lifecycle_state_token
                ),
            ));
            return defects;
        }
    }

    let covered = snapshot.covered_states();

    // Coverage check: all required lifecycle states must be present.
    for state in &REQUIRED_LIFECYCLE_STATES {
        if !covered.contains(state) {
            defects.push(LifecycleDefect::new(
                NarrowReasonClass::RequiredStateMissing,
                (*state).to_owned(),
                format!(
                    "required lifecycle state '{state}' has no record; the packet is \
                     flagged for review"
                ),
            ));
        }
    }

    // Per-record checks.
    for record in &snapshot.records {
        let key = &record.lifecycle_state_token;

        if record.target_identity_ref.is_empty() {
            defects.push(LifecycleDefect::new(
                NarrowReasonClass::TargetIdentityUndeclared,
                key.clone(),
                format!(
                    "state '{key}' does not declare a target_identity_ref; identity must \
                     stay stable across desktop, preview, companion, and support surfaces"
                ),
            ));
        }

        if !record_reaches_all_surfaces(record) {
            defects.push(LifecycleDefect::new(
                NarrowReasonClass::SurfaceCoverageIncomplete,
                key.clone(),
                format!(
                    "state '{key}' does not reach every required consuming surface \
                     (desktop, preview_route, companion_handoff, incident_packet, \
                     support_export); lifecycle truth must be consistent across surfaces"
                ),
            ));
        }

        if record.continuity_class.claims_exact_continuity() && record.has_material_change() {
            defects.push(LifecycleDefect::new(
                NarrowReasonClass::ContinuityOverclaim,
                key.clone(),
                format!(
                    "state '{key}' claims exact_continuity while the persistence class, \
                     template/image provenance, or target identity changed materially; a \
                     resume may never imply exact continuity over a changed runtime"
                ),
            ));
        }

        if record.has_material_change() && record.caveat_history.is_empty() {
            defects.push(LifecycleDefect::new(
                NarrowReasonClass::CaveatHistoryMissing,
                key.clone(),
                format!(
                    "state '{key}' reflects a material runtime change but carries no caveat \
                     history; changed persistence class and caveats must survive resume and \
                     reprovision"
                ),
            ));
        }

        if record.lifecycle_state.requires_local_safe_continuation()
            && (!record.local_safe_continuation_available || record.recovery_options.is_empty())
        {
            defects.push(LifecycleDefect::new(
                NarrowReasonClass::LocalSafeContinuationUnavailable,
                key.clone(),
                format!(
                    "state '{key}' is an outage or expiry state but offers no local-safe \
                     continuation or no recovery option; control-plane outages must degrade \
                     to attributable local-safe continuation rather than generic failure"
                ),
            ));
        }
    }

    defects
}

fn derive_rows(
    snapshot: &LifecycleSnapshot,
    page_defects: &[LifecycleDefect],
) -> Vec<LifecycleMatrixRow> {
    let overall_narrow_reason = overall_narrow_reason(page_defects);

    snapshot
        .records
        .iter()
        .map(|record| {
            let row_narrow = find_row_narrow_reason(record, page_defects, overall_narrow_reason);
            let disposition = row_narrow.disposition();
            let summary = build_row_summary(&record.lifecycle_state_token, disposition, row_narrow);
            LifecycleMatrixRow {
                record_kind: MANAGED_WORKSPACE_LIFECYCLE_ROW_RECORD_KIND.to_owned(),
                schema_version: MANAGED_WORKSPACE_LIFECYCLE_SCHEMA_VERSION,
                shared_contract_ref: MANAGED_WORKSPACE_LIFECYCLE_SHARED_CONTRACT_REF.to_owned(),
                lifecycle_state_token: record.lifecycle_state_token.clone(),
                prior_lifecycle_state_token: record.prior_lifecycle_state_token.clone(),
                transition_reason_token: record.transition_reason_token.clone(),
                persistence_class_token: record.persistence_class_token.clone(),
                continuity_class_token: record.continuity_class_token.clone(),
                material_change: record.has_material_change(),
                local_safe_continuation_available: record.local_safe_continuation_available,
                raw_private_material_excluded: record.raw_private_material_excluded,
                disposition_token: disposition.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

/// Choose the most severe page-wide narrow reason: a withhold reason dominates,
/// then a flag reason, then any other.
fn overall_narrow_reason(page_defects: &[LifecycleDefect]) -> NarrowReasonClass {
    if let Some(d) = page_defects
        .iter()
        .find(|d| d.narrow_reason.is_withhold_reason())
    {
        return d.narrow_reason;
    }
    if let Some(d) = page_defects
        .iter()
        .find(|d| d.narrow_reason.disposition() == LifecycleDispositionClass::Flagged)
    {
        return d.narrow_reason;
    }
    page_defects
        .first()
        .map(|d| d.narrow_reason)
        .unwrap_or(NarrowReasonClass::NotNarrowed)
}

fn find_row_narrow_reason(
    record: &LifecycleRecord,
    page_defects: &[LifecycleDefect],
    overall_narrow_reason: NarrowReasonClass,
) -> NarrowReasonClass {
    // Prefer the most severe defect that names this row directly.
    let mut row_reason: Option<NarrowReasonClass> = None;
    for defect in page_defects
        .iter()
        .filter(|d| d.source == record.lifecycle_state_token)
    {
        row_reason = Some(match row_reason {
            None => defect.narrow_reason,
            Some(existing) if defect.narrow_reason.disposition() > existing.disposition() => {
                defect.narrow_reason
            }
            Some(existing) => existing,
        });
    }
    if let Some(reason) = row_reason {
        return reason;
    }
    // A page-level coverage gap (missing required state) flags the whole packet
    // but does not narrow individually-clean rows below truthful.
    if overall_narrow_reason == NarrowReasonClass::RequiredStateMissing {
        return NarrowReasonClass::NotNarrowed;
    }
    NarrowReasonClass::NotNarrowed
}

fn build_row_summary(
    state_token: &str,
    disposition: LifecycleDispositionClass,
    narrow_reason: NarrowReasonClass,
) -> String {
    match disposition {
        LifecycleDispositionClass::Truthful => format!(
            "State '{state_token}' publishes truthfully: identity is stable, continuity is \
             not overclaimed, material changes carry caveats, and outage states offer \
             local-safe continuation."
        ),
        _ => format!(
            "State '{state_token}' {} ({}): see defect list for details.",
            disposition.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded truthful packet consumed by integration tests and the
/// fixture generator.
///
/// The seeded page produces zero defects: all ten required lifecycle states are
/// covered, no raw private material is exposed, every record declares a stable
/// target identity, reaches every required surface, never overclaims
/// continuity, carries caveats for every material change, and offers local-safe
/// continuation for every outage and expiry state.
pub fn seeded_managed_workspace_lifecycle_page() -> ManagedWorkspaceLifecyclePage {
    ManagedWorkspaceLifecyclePage::new(
        "remote:managed_workspace_lifecycle:default",
        "Managed-workspace lifecycle — provision, warm, ready, suspend, resume, reconnect, \
         rebuild, recreate, expiry, and local-safe continuation — truthful packet",
        "2026-06-11T00:00:00Z",
        seeded_lifecycle_snapshot(),
    )
}

/// Build the seeded lifecycle snapshot used by the seeded page.
///
/// Each of the ten required lifecycle states is represented with a fully-typed,
/// clean record that passes every truthfulness condition.
pub fn seeded_lifecycle_snapshot() -> LifecycleSnapshot {
    let all_surfaces = || {
        vec![
            SurfaceClass::Desktop,
            SurfaceClass::PreviewRoute,
            SurfaceClass::CompanionHandoff,
            SurfaceClass::IncidentPacket,
            SurfaceClass::SupportExport,
        ]
    };
    let identity = "mw:identity:alpha";

    LifecycleSnapshot {
        records: vec![
            LifecycleRecord::new(
                LifecycleStateClass::Provision,
                None,
                TransitionReasonClass::UserRequestedCreate,
                PersistenceClass::EphemeralScratch,
                false,
                ProvenanceClass::PinnedDigest,
                ProvenanceClass::PinnedDigest,
                false,
                identity,
                true,
                ContinuityClass::FreshNoContinuity,
                vec![RecoveryOptionClass::ContactOperator],
                ExpiryClass::None,
                None::<String>,
                true,
                vec![],
                all_surfaces(),
                "Provisioning: the control plane is allocating the workspace from a pinned \
                 template and image; no runtime is reachable yet and no prior context exists.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::Warm,
                Some(LifecycleStateClass::Provision),
                TransitionReasonClass::UserRequestedCreate,
                PersistenceClass::PersistentVolume,
                false,
                ProvenanceClass::PinnedDigest,
                ProvenanceClass::PinnedDigest,
                false,
                identity,
                true,
                ContinuityClass::ExactContinuity,
                vec![],
                ExpiryClass::None,
                None::<String>,
                true,
                vec![],
                all_surfaces(),
                "Warming: the workspace is booting and warming caches on the same pinned \
                 image; the persistent volume is attached and identity is stable.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::Ready,
                Some(LifecycleStateClass::Warm),
                TransitionReasonClass::UserRequestedCreate,
                PersistenceClass::PersistentVolume,
                false,
                ProvenanceClass::PinnedDigest,
                ProvenanceClass::PinnedDigest,
                false,
                identity,
                true,
                ContinuityClass::ExactContinuity,
                vec![],
                ExpiryClass::None,
                None::<String>,
                true,
                vec![],
                all_surfaces(),
                "Ready: the workspace is reachable and ready for interactive work on the \
                 pinned image with its persistent volume attached.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::Suspended,
                Some(LifecycleStateClass::Ready),
                TransitionReasonClass::IdleWindowElapsed,
                PersistenceClass::PersistentVolume,
                false,
                ProvenanceClass::PinnedDigest,
                ProvenanceClass::PinnedDigest,
                false,
                identity,
                true,
                ContinuityClass::ExactContinuity,
                vec![RecoveryOptionClass::Resume],
                ExpiryClass::IdleWindow,
                Some("window:idle:30m"),
                true,
                vec![],
                all_surfaces(),
                "Suspended: an idle window paused the workspace; the persistent volume \
                 survives and resume restores the same runtime in place.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::Resumed,
                Some(LifecycleStateClass::Suspended),
                TransitionReasonClass::UserRequestedResume,
                PersistenceClass::PersistentVolume,
                false,
                ProvenanceClass::PinnedDigest,
                ProvenanceClass::PinnedDigest,
                false,
                identity,
                true,
                ContinuityClass::ExactContinuity,
                vec![RecoveryOptionClass::Reconnect],
                ExpiryClass::None,
                None::<String>,
                true,
                vec![CaveatClass::SessionReauthRequired],
                all_surfaces(),
                "Resumed: the workspace resumed from suspension on the same volume and image; \
                 continuity is exact, but a session re-authentication is required before \
                 mutations commit.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::Reconnecting,
                Some(LifecycleStateClass::Ready),
                TransitionReasonClass::ControlPlaneFailure,
                PersistenceClass::PersistentVolume,
                false,
                ProvenanceClass::PinnedDigest,
                ProvenanceClass::PinnedDigest,
                false,
                identity,
                true,
                ContinuityClass::LocalSafeOnly,
                vec![
                    RecoveryOptionClass::Reconnect,
                    RecoveryOptionClass::LocalSafeContinue,
                ],
                ExpiryClass::ControlPlaneOutage,
                Some("window:reconnect:grace"),
                true,
                vec![CaveatClass::LocalSafeOnly],
                all_surfaces(),
                "Reconnecting: the control-plane connection dropped and is being \
                 re-established; work continues against a local-safe mirror during the grace \
                 window and resumes the same runtime once reconnected.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::RebuildRequired,
                Some(LifecycleStateClass::Ready),
                TransitionReasonClass::SuccessorImageAvailable,
                PersistenceClass::RebuiltFresh,
                true,
                ProvenanceClass::PinnedDigest,
                ProvenanceClass::SuccessorImage,
                true,
                identity,
                true,
                ContinuityClass::MaterialChange,
                vec![
                    RecoveryOptionClass::Rebuild,
                    RecoveryOptionClass::LocalSafeContinue,
                ],
                ExpiryClass::None,
                None::<String>,
                true,
                vec![
                    CaveatClass::ImageChanged,
                    CaveatClass::PersistenceClassChanged,
                    CaveatClass::ScratchStateDiscarded,
                ],
                all_surfaces(),
                "Rebuild required: a successor image superseded the prior one; rebuilding \
                 keeps the workspace identity but lands on a fresh filesystem, so scratch \
                 state is discarded and continuity is not exact.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::RecreateRequired,
                Some(LifecycleStateClass::Expired),
                TransitionReasonClass::CapsuleDriftDetected,
                PersistenceClass::RecreatedNew,
                true,
                ProvenanceClass::DriftedUnpinned,
                ProvenanceClass::DriftedUnpinned,
                true,
                "mw:identity:alpha-successor",
                false,
                ContinuityClass::FreshNoContinuity,
                vec![
                    RecoveryOptionClass::Recreate,
                    RecoveryOptionClass::LocalSafeContinue,
                    RecoveryOptionClass::ContactOperator,
                ],
                ExpiryClass::None,
                None::<String>,
                true,
                vec![
                    CaveatClass::TargetIdentityChanged,
                    CaveatClass::ImageChanged,
                    CaveatClass::PersistenceClassChanged,
                    CaveatClass::ScratchStateDiscarded,
                ],
                all_surfaces(),
                "Recreate required: capsule drift makes the prior workspace non-recoverable; \
                 recreating mints a new workspace identity with no carried-over state, so \
                 there is no continuity with the prior runtime.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::Expired,
                Some(LifecycleStateClass::Suspended),
                TransitionReasonClass::HibernationWindowElapsed,
                PersistenceClass::LocalMirror,
                true,
                ProvenanceClass::Unknown,
                ProvenanceClass::Unknown,
                true,
                identity,
                true,
                ContinuityClass::LocalSafeOnly,
                vec![
                    RecoveryOptionClass::Recreate,
                    RecoveryOptionClass::LocalSafeContinue,
                    RecoveryOptionClass::ContactOperator,
                ],
                ExpiryClass::HibernationWindow,
                Some("window:hibernation:7d"),
                true,
                vec![
                    CaveatClass::ScratchStateDiscarded,
                    CaveatClass::PersistenceClassChanged,
                    CaveatClass::LocalSafeOnly,
                ],
                all_surfaces(),
                "Expired: the hibernation window elapsed and the managed runtime is gone; the \
                 last validated state is available as a local-safe mirror and recreate restores \
                 a managed runtime under the same identity.",
            ),
            LifecycleRecord::new(
                LifecycleStateClass::LocalSafeContinuation,
                Some(LifecycleStateClass::Reconnecting),
                TransitionReasonClass::ControlPlaneFailure,
                PersistenceClass::LocalMirror,
                true,
                ProvenanceClass::Unknown,
                ProvenanceClass::Unknown,
                true,
                identity,
                true,
                ContinuityClass::LocalSafeOnly,
                vec![
                    RecoveryOptionClass::Reconnect,
                    RecoveryOptionClass::LocalSafeContinue,
                    RecoveryOptionClass::ContactOperator,
                ],
                ExpiryClass::ControlPlaneOutage,
                Some("window:outage:open"),
                true,
                vec![
                    CaveatClass::LocalSafeOnly,
                    CaveatClass::PersistenceClassChanged,
                ],
                all_surfaces(),
                "Local-safe continuation: the control plane is unreachable; editing continues \
                 against the last validated local mirror with managed capabilities suspended, \
                 and reconnect restores the managed runtime when the outage clears.",
            ),
        ],
    }
}
