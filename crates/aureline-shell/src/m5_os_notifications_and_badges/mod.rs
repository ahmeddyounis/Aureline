//! OS-level attention parity for the M5 durable job families: privacy-safe
//! notification summaries, durable badge classes, named taskbar/dock progress,
//! quiet-hours / admin-suppression parity, and exact-target reopen.
//!
//! Aureline's in-product activity center already treats attention as durable
//! truth: every badge, progress indicator, and reopen action is derived from a
//! durable job object, never from a transient toast. The OS surfaces —
//! lock-screen and notification-center text, the dock/taskbar badge, the
//! dock/taskbar progress affordance, and the companion mirror — are easy to
//! ship as their own desktop-only states that drift from that truth: a
//! notification that leaks code or a secret onto a lock screen, a badge counter
//! that reflects raw event fanout, a generic progress spinner that maps to no
//! named job, a quiet-hours decision that diverges from the in-app surface, or
//! an action that reopens nothing. This module carries the in-product promise
//! out to the OS for the M5 durable job families: every OS attention surface
//! MUST derive from the same durable job object (reusing the canonical
//! [`DurableAttentionJobFamily`], [`DurableJobRowStateClass`], and
//! [`AggregateCountClass`] vocabularies rather than synthesizing a desktop-only
//! state), MUST keep its lock-screen and notification-center copy summary-first
//! with an explicit source-object label, client scope, and one safe reopen
//! action, MUST derive its badge count from a durable count class, MUST map its
//! taskbar/dock progress to a named durable job class (never a generic spinner),
//! MUST apply quiet-hours / do-not-disturb / admin suppression identically to
//! the in-app and companion surfaces with a visible suppression audit, and MUST
//! land its action on the exact durable object or a truthful placeholder with
//! source and freshness intact.
//!
//! The audit projects, for each registered OS attention surface, the canonical
//! surface descriptor and its typed [`M5OsNotificationEnvelope`] against the
//! qualification result the surface certifies for each of the five OS-attention
//! parity guarantees the M5 lanes must pass:
//!
//! - `privacy_safe_summary`
//! - `badge_durable_class`
//! - `progress_named_job_class`
//! - `suppression_parity`
//! - `exact_reopen_parity`
//!
//! The resulting [`M5OsAttentionReport`] is the canonical truth object for the
//! M5 OS-attention parity lane. It is consumed by:
//!
//! - the live shell notification router / dock-badge / taskbar-progress / About
//!   surfaces (so the in-product surfaces quote the same per-surface findings
//!   the CLI prints);
//! - the headless inspector (`aureline_shell_m5_os_notifications`), which is the
//!   only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/ux/m5_os_notifications_and_badges/`;
//! - the support-export wrapper that lets a reviewer pivot from a support case
//!   to the surface that leaked, mis-counted, or diverged; and
//! - the markdown audit under
//!   `artifacts/ux/m5/os-notification-and-reopen.md` and the companion doc under
//!   `docs/m5/os-notifications-badges-and-progress.md` (rendered from the same
//!   seed), and the CI gate `tools/ci/m5/os_notifications_and_badges_check.py`.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered OS attention surface declares a binding for each of the
//!    five parity guarantees.
//! 2. Every surface carries a canonical exact-target reopen anchor, a durable
//!    job ref, a non-empty support note, a declared privacy class, an explicit
//!    source-object label, one safe reopen action label, and a flag asserting it
//!    derives from a durable object rather than a synthesized desktop-only
//!    state; a missing field or a desktop-only synthesized surface is a blocker.
//! 3. A qualified guarantee carries the captured evidence the guarantee requires
//!    — an envelope ref, a declared privacy class, and an evidence-freshness
//!    stamp for every guarantee; a lock-screen and payload disclosure for the
//!    privacy guarantee; a badge basis and count class for the badge guarantee;
//!    a progress basis for the progress guarantee; a suppression parity result,
//!    decision, and visible audit for the suppression guarantee; and a reopen
//!    outcome for the reopen guarantee. A red result (a lock-screen leak, a
//!    protected payload body, a raw-event badge counter, a generic progress
//!    spinner, a diverging suppression decision, a missing suppression audit, or
//!    a lost reopen target) is a blocker.
//! 4. A surface that paints an OS affordance from a synthesized desktop-only
//!    state (`unqualified_desktop_only_state`) and a marketed guarantee claimed
//!    with no captured evidence (`missing_evidence`) are blockers.
//! 5. Stale durable evidence on a marketed guarantee is a blocker, so release
//!    tooling can narrow a marketed surface instead of shipping it as implicitly
//!    stable.
//! 6. At least one surface qualifies each of the five guarantees so the audit
//!    cannot regress into a single happy-path surface.
//!
//! All identifiers, refs, and label strings are deterministic so the checked-in
//! fixtures under `fixtures/ux/m5_os_notifications_and_badges/` are bit-for-bit
//! equal to the seeded report returned by [`seeded_m5_os_attention_report`].

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::badge_aggregate_stable::AggregateCountClass;
use crate::durable_attention_beta::{
    DurableAttentionJobFamily, DurableJobRowStateClass, QuietHoursDecisionClass,
};

const GENERATED_AT: &str = "2026-06-16T00:00:00Z";

/// Schema version exported with every OS-attention record.
pub const M5_OS_ATTENTION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by UI, CLI, docs, and support export.
pub const M5_OS_ATTENTION_SHARED_CONTRACT_REF: &str = "shell:m5_os_notifications_and_badges:v1";

/// Stable record kind for the audit report payload.
pub const M5_OS_ATTENTION_REPORT_RECORD_KIND: &str = "shell_m5_os_attention_report_record";

/// Stable record kind for one per-surface qualification row.
pub const M5_OS_ATTENTION_ROW_RECORD_KIND: &str = "shell_m5_os_attention_row_record";

/// Stable record kind for one typed OS notification envelope.
pub const M5_OS_NOTIFICATION_ENVELOPE_RECORD_KIND: &str =
    "shell_m5_os_notification_envelope_record";

/// Stable record kind for the support-export wrapper.
pub const M5_OS_ATTENTION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_os_attention_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_OS_ATTENTION_REPORT_ID: &str = "shell:m5_os_notifications_and_badges:report:v1";

/// Stable support-export id.
pub const M5_OS_ATTENTION_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-os-notifications-and-badges:001";

/// Boundary schema ref for the canonical envelope contract.
pub const M5_OS_ATTENTION_SOURCE_SCHEMA_REF: &str =
    "schemas/ux/m5-os-notification-envelope.schema.json";

/// Markdown publication ref this audit is rendered to.
pub const M5_OS_ATTENTION_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/os-notification-and-reopen.md";

/// Companion doc publication ref.
pub const M5_OS_ATTENTION_PUBLISHED_DOC_REF: &str =
    "docs/m5/os-notifications-badges-and-progress.md";

/// One OS-attention parity guarantee a surface certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsAttentionGuarantee {
    /// Lock-screen and notification-center copy is summary-first, names the
    /// source object, client scope, and one safe reopen action, and leaks no
    /// code, secret, AI-prompt, or high-risk-mutation detail.
    PrivacySafeSummary,
    /// The badge count derives from a durable count class, never raw event
    /// fanout.
    BadgeDurableClass,
    /// The taskbar/dock progress maps to a named durable job class, never a
    /// generic activity spinner.
    ProgressNamedJobClass,
    /// Quiet-hours, do-not-disturb, and admin suppression apply identically to
    /// the in-app, OS, and companion surfaces, with a visible suppression audit.
    SuppressionParity,
    /// The OS action lands on the exact durable object or a truthful placeholder
    /// with source and freshness intact.
    ExactReopenParity,
}

impl M5OsAttentionGuarantee {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrivacySafeSummary => "privacy_safe_summary",
            Self::BadgeDurableClass => "badge_durable_class",
            Self::ProgressNamedJobClass => "progress_named_job_class",
            Self::SuppressionParity => "suppression_parity",
            Self::ExactReopenParity => "exact_reopen_parity",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::PrivacySafeSummary => "Privacy-safe summary",
            Self::BadgeDurableClass => "Badge durable class",
            Self::ProgressNamedJobClass => "Progress named job class",
            Self::SuppressionParity => "Suppression parity",
            Self::ExactReopenParity => "Exact reopen parity",
        }
    }

    /// The five parity guarantees, in canonical order.
    pub const fn required_guarantees() -> [Self; 5] {
        [
            Self::PrivacySafeSummary,
            Self::BadgeDurableClass,
            Self::ProgressNamedJobClass,
            Self::SuppressionParity,
            Self::ExactReopenParity,
        ]
    }

    /// `true` when a qualified binding must carry lock-screen and payload
    /// disclosures.
    pub const fn requires_privacy_disclosures(self) -> bool {
        matches!(self, Self::PrivacySafeSummary)
    }

    /// `true` when a qualified binding must carry a badge basis and count class.
    pub const fn requires_badge(self) -> bool {
        matches!(self, Self::BadgeDurableClass)
    }

    /// `true` when a qualified binding must carry a progress basis.
    pub const fn requires_progress(self) -> bool {
        matches!(self, Self::ProgressNamedJobClass)
    }

    /// `true` when a qualified binding must carry a suppression parity result.
    pub const fn requires_suppression(self) -> bool {
        matches!(self, Self::SuppressionParity)
    }

    /// `true` when a qualified binding must carry a reopen outcome.
    pub const fn requires_reopen_outcome(self) -> bool {
        matches!(self, Self::ExactReopenParity)
    }
}

/// OS-attention privacy class assigned to a surface.
///
/// `security_critical` and `managed_sensitive` are high-stakes: their OS
/// surfaces must always carry an exact-target reopen affordance and a non-empty
/// suppression-control set, so the audit requires a present reopen outcome on
/// every qualified guarantee for these classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsPrivacyClass {
    /// Summary-safe; carries no workspace, code, or secret detail.
    SummarySafe,
    /// Workspace-sensitive; may reference workspace content by reference only.
    WorkspaceSensitive,
    /// Security-critical; concerns credentials, approvals, or high-risk action.
    SecurityCritical,
    /// Managed-sensitive; governed by admin policy and managed-depth rules.
    ManagedSensitive,
}

impl M5OsPrivacyClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummarySafe => "summary_safe",
            Self::WorkspaceSensitive => "workspace_sensitive",
            Self::SecurityCritical => "security_critical",
            Self::ManagedSensitive => "managed_sensitive",
        }
    }

    /// `true` for the classes whose surface is high-stakes for the audit.
    pub const fn is_high_stakes(self) -> bool {
        matches!(self, Self::SecurityCritical | Self::ManagedSensitive)
    }
}

/// Client scope the OS surface is bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsClientScope {
    /// The local desktop product.
    DesktopProduct,
    /// A managed (admin/policy) desktop deployment.
    ManagedDesktop,
    /// A companion surface mirroring the desktop object.
    CompanionMirror,
}

impl M5OsClientScope {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopProduct => "desktop_product",
            Self::ManagedDesktop => "managed_desktop",
            Self::CompanionMirror => "companion_mirror",
        }
    }
}

/// Whether the lock-screen / notification-center copy stays summary-first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsLockScreenDisclosure {
    /// The copy is a bounded summary that names the source object, client scope,
    /// and one safe reopen action only.
    SummaryWithSourceAndScope,
    /// The copy leaks code, secret, AI-prompt, or high-risk-mutation detail.
    /// Always a blocker.
    LeaksProtectedDetail,
}

impl M5OsLockScreenDisclosure {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummaryWithSourceAndScope => "summary_with_source_and_scope",
            Self::LeaksProtectedDetail => "leaks_protected_detail",
        }
    }
}

/// Whether the OS notification packet keeps payloads minimized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsPayloadDisclosure {
    /// The packet carries stable class enums and durable refs only.
    EnumsAndRefsOnly,
    /// The packet carries code, secret, AI-prompt, or raw provider detail by
    /// default. Always a blocker.
    CarriesProtectedBody,
}

impl M5OsPayloadDisclosure {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnumsAndRefsOnly => "enums_and_refs_only",
            Self::CarriesProtectedBody => "carries_protected_body",
        }
    }
}

/// Whether the badge count derives from a durable count class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsBadgeBasis {
    /// The badge count derives from a durable [`AggregateCountClass`] and stays
    /// correct after retries and partial delivery.
    DurableCountClass,
    /// The badge count reflects raw event fanout and drifts after retries.
    /// Always a blocker.
    RawEventFanout,
}

impl M5OsBadgeBasis {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableCountClass => "durable_count_class",
            Self::RawEventFanout => "raw_event_fanout",
        }
    }
}

/// Whether the taskbar/dock progress maps to a named durable job class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsProgressBasis {
    /// Progress derives from a named durable job class and its envelope progress.
    NamedDurableJobClass,
    /// Progress is a generic activity spinner mapped to no named job. Always a
    /// blocker.
    GenericSpinner,
}

impl M5OsProgressBasis {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedDurableJobClass => "named_durable_job_class",
            Self::GenericSpinner => "generic_spinner",
        }
    }
}

/// Whether the quiet-hours / DND / admin-suppression decision matches across
/// surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsSuppressionParity {
    /// The suppression decision is identical across the in-app, OS, and
    /// companion surfaces.
    ParityAcrossSurfaces,
    /// The OS surface diverges from the in-app suppression decision. Always a
    /// blocker.
    DivergesFromInApp,
}

impl M5OsSuppressionParity {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityAcrossSurfaces => "parity_across_surfaces",
            Self::DivergesFromInApp => "diverges_from_in_app",
        }
    }
}

/// Whether the OS action resolves the exact durable object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsReopenOutcome {
    /// The action reopens the exact durable object through the in-product
    /// surface.
    ExactDurableObject,
    /// The action lands on a truthful placeholder that names the source and
    /// freshness of the missing target.
    TruthfulPlaceholder,
    /// The action fails to resolve its target. Always a blocker.
    TargetLost,
}

impl M5OsReopenOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactDurableObject => "exact_durable_object",
            Self::TruthfulPlaceholder => "truthful_placeholder",
            Self::TargetLost => "target_lost",
        }
    }
}

/// Freshness of the captured evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsEvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed guarantee.
    Stale,
}

impl M5OsEvidenceFreshness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// A suppression / interruptibility control a surface exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsSuppressionControl {
    /// Honour the user's quiet-hours window.
    QuietHours,
    /// Honour the user's do-not-disturb / focus mode.
    DoNotDisturb,
    /// Honour admin suppression policy.
    AdminSuppress,
    /// Mute future OS notifications for the object.
    Mute,
    /// Snooze the notification until later.
    Snooze,
    /// Show only a bounded lock-screen summary.
    LockScreenSummary,
}

impl M5OsSuppressionControl {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuietHours => "quiet_hours",
            Self::DoNotDisturb => "do_not_disturb",
            Self::AdminSuppress => "admin_suppress",
            Self::Mute => "mute",
            Self::Snooze => "snooze",
            Self::LockScreenSummary => "lock_screen_summary",
        }
    }
}

/// Lifecycle label retained on the canonical surface descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsSurfaceLifecycle {
    /// Generally available.
    Stable,
    /// Beta lane; visibility and narrowing can change.
    Beta,
    /// Deprecated; surfaces must point at the replacement.
    Deprecated,
}

impl M5OsSurfaceLifecycle {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Qualification status a surface reports for one parity guarantee.
///
/// Only `Qualified` rows project captured evidence and are drift/red checked.
/// `ExplicitlyNarrowed`, `NotApplicable`, and `PlatformOmitted` rows are
/// accepted as long as they carry a `narrowing_reason`.
/// `UnqualifiedDesktopOnlyState` (an OS affordance painted from a synthesized
/// desktop-only state instead of a durable object) and `MissingEvidence` are
/// blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OsQualificationStatus {
    /// The guarantee is qualified with captured evidence.
    Qualified,
    /// The surface narrows this guarantee; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// The guarantee does not apply to this surface; a reason MUST be set.
    NotApplicable,
    /// The guarantee is not surfaced on this platform; a reason MUST be set.
    PlatformOmitted,
    /// The surface paints an OS affordance from a synthesized desktop-only state
    /// instead of a durable object. Always a blocker.
    UnqualifiedDesktopOnlyState,
    /// A marketed guarantee is claimed with no captured evidence. Always a
    /// blocker.
    MissingEvidence,
}

impl M5OsQualificationStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::NotApplicable => "not_applicable",
            Self::PlatformOmitted => "platform_omitted",
            Self::UnqualifiedDesktopOnlyState => "unqualified_desktop_only_state",
            Self::MissingEvidence => "missing_evidence",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyNarrowed | Self::NotApplicable | Self::PlatformOmitted
        )
    }

    /// `true` for the status that projects captured evidence.
    pub const fn projects_evidence(self) -> bool {
        matches!(self, Self::Qualified)
    }
}

/// Exact-target reopen linkage retained on the OS notification envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsReopenLinkage {
    /// Outcome the action resolves to.
    pub reopen_outcome: M5OsReopenOutcome,
    /// Canonical exact-target reopen anchor ref the action reopens from.
    pub reopen_anchor_ref: String,
    /// In-product command the action routes through. MUST be present.
    pub command_id_ref: String,
    /// `true` when the action must resolve through an in-product surface and
    /// never a privileged OS shortcut.
    pub must_resolve_through_in_product_surface: bool,
    /// `true` when the action preserves the source object identity.
    pub preserves_source: bool,
    /// `true` when the action preserves the freshness of the target.
    pub preserves_freshness: bool,
}

/// Typed OS notification envelope — the privacy-safe truth packet the OS
/// surfaces of one durable job family read.
///
/// This is the boundary object validated by
/// `schemas/ux/m5-os-notification-envelope.schema.json`. It carries no
/// credential bodies or raw provider payloads: only stable class enums, durable
/// refs, and label refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsNotificationEnvelope {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the envelope.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable envelope id (e.g. `os-envelope:task_run`).
    pub envelope_id: String,
    /// Durable job family the surface derives from.
    pub job_family: DurableAttentionJobFamily,
    /// Durable job-row state class the envelope was minted from.
    pub job_state_class: DurableJobRowStateClass,
    /// Durable job id ref the envelope derives from. MUST be non-empty.
    pub durable_job_id_ref: String,
    /// Canonical event id ref shared with the in-app activity row.
    pub canonical_event_id_ref: String,
    /// Declared privacy class.
    pub privacy_class: M5OsPrivacyClass,
    /// Client scope the surface is bound to.
    pub client_scope: M5OsClientScope,
    /// Explicit label ref naming the source object on the OS surface.
    pub source_object_label_ref: String,
    /// Label ref for the one safe reopen action exposed on the OS surface.
    pub safe_reopen_action_label_ref: String,
    /// Lock-screen / notification-center disclosure class.
    pub lock_screen_disclosure: M5OsLockScreenDisclosure,
    /// Payload-minimization class.
    pub payload_disclosure: M5OsPayloadDisclosure,
    /// Badge count class the badge derives from.
    pub badge_count_class: AggregateCountClass,
    /// Whether the badge derives from a durable count class.
    pub badge_basis: M5OsBadgeBasis,
    /// Whether taskbar/dock progress maps to a named durable job class, when the
    /// surface exposes progress.
    pub progress_basis: Option<M5OsProgressBasis>,
    /// Suppression decision applied across surfaces.
    pub suppression_decision: QuietHoursDecisionClass,
    /// Whether the OS surface matches the in-app suppression decision.
    pub suppression_parity: M5OsSuppressionParity,
    /// `true` when the suppression decision keeps a visible audit.
    pub suppression_audit_visible: bool,
    /// Exact-target reopen linkage.
    pub reopen_linkage: M5OsReopenLinkage,
}

/// Canonical descriptor for one OS attention surface's parity contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsSurfaceDescriptor {
    /// Stable surface id (e.g. `os:task_run`).
    pub surface_id: String,
    /// Durable job family the surface derives from.
    pub job_family: DurableAttentionJobFamily,
    /// Durable job-row state class the surface was minted from.
    pub job_state_class: DurableJobRowStateClass,
    /// Durable job id ref the surface derives from. MUST be non-empty.
    pub durable_job_id_ref: String,
    /// Canonical event id ref shared with the in-app activity row.
    pub canonical_event_id_ref: String,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Support note retained on the descriptor. MUST be non-empty.
    pub support_note: String,
    /// Declared privacy class.
    pub privacy_class: M5OsPrivacyClass,
    /// Client scope the surface is bound to.
    pub client_scope: M5OsClientScope,
    /// Pinned surface lifecycle label.
    pub lifecycle_label: M5OsSurfaceLifecycle,
    /// Explicit source-object label ref. MUST be non-empty.
    pub source_object_label_ref: String,
    /// One safe reopen action label ref. MUST be non-empty.
    pub safe_reopen_action_label_ref: String,
    /// Canonical exact-target reopen anchor ref. MUST be non-empty.
    pub reopen_anchor_ref: String,
    /// Suppression / interruptibility controls the surface exposes, in canonical
    /// order.
    pub suppression_controls: Vec<M5OsSuppressionControl>,
    /// `true` when the surface derives from a durable object rather than a
    /// synthesized desktop-only state. MUST be `true`.
    pub derived_from_durable_object: bool,
    /// `true` when the surface is marketed on desktop and therefore must pass the
    /// claimed matrix or narrow accordingly.
    pub marketed_on_desktop: bool,
}

impl M5OsSurfaceDescriptor {
    /// `true` when this surface's privacy class makes it high-stakes.
    pub const fn is_high_stakes(&self) -> bool {
        self.privacy_class.is_high_stakes()
    }
}

/// Per-guarantee binding a surface reports for one parity guarantee.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsAttentionBinding {
    /// Guarantee this binding covers.
    pub guarantee: M5OsAttentionGuarantee,
    /// Qualification status the surface reports.
    pub qualification_status: M5OsQualificationStatus,
    /// `true` when the surface is marketed on this guarantee.
    pub marketed_on_guarantee: bool,
    /// Captured envelope ref (`None` for non-qualified rows).
    pub projected_envelope_ref: Option<String>,
    /// Captured privacy class (`None` for non-qualified rows).
    pub projected_privacy_class: Option<M5OsPrivacyClass>,
    /// Captured lock-screen disclosure (`None` unless the guarantee requires it).
    pub projected_lock_screen: Option<M5OsLockScreenDisclosure>,
    /// Captured payload disclosure (`None` unless the guarantee requires it).
    pub projected_payload_disclosure: Option<M5OsPayloadDisclosure>,
    /// Captured badge basis (`None` unless the guarantee requires it).
    pub projected_badge_basis: Option<M5OsBadgeBasis>,
    /// Captured badge count class (`None` unless the guarantee requires it).
    pub projected_badge_count_class: Option<AggregateCountClass>,
    /// Captured progress basis (`None` unless the guarantee requires it).
    pub projected_progress_basis: Option<M5OsProgressBasis>,
    /// Captured suppression parity (`None` unless the guarantee requires it).
    pub projected_suppression_parity: Option<M5OsSuppressionParity>,
    /// Captured suppression decision (`None` unless the guarantee requires it).
    pub projected_suppression_decision: Option<QuietHoursDecisionClass>,
    /// Captured suppression-audit visibility (`None` unless the guarantee
    /// requires it).
    pub projected_suppression_audit_visible: Option<bool>,
    /// Captured reopen outcome (`None` unless the guarantee requires it or the
    /// surface is high-stakes).
    pub projected_reopen_outcome: Option<M5OsReopenOutcome>,
    /// Freshness of the captured evidence (`None` for non-qualified rows).
    pub evidence_freshness: Option<M5OsEvidenceFreshness>,
    /// Timestamp the evidence was captured (`None` for non-qualified rows).
    pub evidence_captured_at: Option<String>,
    /// Narrowing reason set when `qualification_status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the row.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum M5OsAttentionBlockingFinding {
    /// A surface paints an OS affordance from a synthesized desktop-only state.
    UnqualifiedDesktopOnlyState {
        /// Surface that exposes the gap.
        surface_id: String,
        /// Guarantee that exposes the gap.
        guarantee: M5OsAttentionGuarantee,
    },
    /// A marketed guarantee is claimed with no captured evidence.
    MissingEvidence {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A qualified guarantee is missing its captured envelope ref.
    MissingEnvelopeRef {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A guarantee leaks code, secret, AI-prompt, or high-risk-mutation detail
    /// onto a lock screen / notification center.
    LockScreenLeak {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A guarantee carries a protected payload body by default.
    ProtectedPayloadBody {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A guarantee paints a badge from raw event fanout instead of a durable
    /// count class.
    BadgeRawEventFanout {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A guarantee maps taskbar/dock progress to a generic spinner instead of a
    /// named durable job class.
    ProgressGenericSpinner {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A guarantee's OS suppression decision diverges from the in-app decision.
    SuppressionDivergence {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A suppressed guarantee keeps no visible suppression audit.
    SuppressionAuditMissing {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A guarantee loses the exact-target reopen affordance.
    ReopenTargetLost {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A marketed guarantee carries stale evidence.
    StaleEvidenceOnMarketedRow {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
    },
    /// A non-qualified row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
        qualification_status: M5OsQualificationStatus,
    },
    /// A qualified row is missing a captured-evidence field it requires.
    MissingProjection {
        surface_id: String,
        guarantee: M5OsAttentionGuarantee,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical exact-target reopen anchor.
    DescriptorMissingReopenAnchor { surface_id: String },
    /// The descriptor carries no durable job ref.
    MissingDurableJobRef { surface_id: String },
    /// The descriptor carries no support note.
    MissingSupportNote { surface_id: String },
    /// The descriptor carries no source-object label.
    MissingSourceObjectLabel { surface_id: String },
    /// The descriptor carries no safe reopen action label.
    MissingSafeReopenAction { surface_id: String },
    /// The surface paints an OS affordance from a synthesized desktop-only state
    /// (descriptor flag).
    SurfaceNotDerivedFromDurableObject { surface_id: String },
    /// A high-stakes surface exposes no suppression controls.
    MissingSuppressionControls { surface_id: String },
    /// The envelope's durable job ref or reopen anchor disagrees with the
    /// descriptor.
    EnvelopeDescriptorMismatch {
        surface_id: String,
        /// Name of the mismatched field.
        field: String,
    },
}

impl M5OsAttentionBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnqualifiedDesktopOnlyState { .. } => "unqualified_desktop_only_state",
            Self::MissingEvidence { .. } => "missing_evidence",
            Self::MissingEnvelopeRef { .. } => "missing_envelope_ref",
            Self::LockScreenLeak { .. } => "lock_screen_leak",
            Self::ProtectedPayloadBody { .. } => "protected_payload_body",
            Self::BadgeRawEventFanout { .. } => "badge_raw_event_fanout",
            Self::ProgressGenericSpinner { .. } => "progress_generic_spinner",
            Self::SuppressionDivergence { .. } => "suppression_divergence",
            Self::SuppressionAuditMissing { .. } => "suppression_audit_missing",
            Self::ReopenTargetLost { .. } => "reopen_target_lost",
            Self::StaleEvidenceOnMarketedRow { .. } => "stale_evidence_on_marketed_row",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingReopenAnchor { .. } => "descriptor_missing_reopen_anchor",
            Self::MissingDurableJobRef { .. } => "missing_durable_job_ref",
            Self::MissingSupportNote { .. } => "missing_support_note",
            Self::MissingSourceObjectLabel { .. } => "missing_source_object_label",
            Self::MissingSafeReopenAction { .. } => "missing_safe_reopen_action",
            Self::SurfaceNotDerivedFromDurableObject { .. } => {
                "surface_not_derived_from_durable_object"
            }
            Self::MissingSuppressionControls { .. } => "missing_suppression_controls",
            Self::EnvelopeDescriptorMismatch { .. } => "envelope_descriptor_mismatch",
        }
    }

    /// Returns the surface id this finding is attached to.
    pub fn surface_id(&self) -> &str {
        match self {
            Self::UnqualifiedDesktopOnlyState { surface_id, .. }
            | Self::MissingEvidence { surface_id, .. }
            | Self::MissingEnvelopeRef { surface_id, .. }
            | Self::LockScreenLeak { surface_id, .. }
            | Self::ProtectedPayloadBody { surface_id, .. }
            | Self::BadgeRawEventFanout { surface_id, .. }
            | Self::ProgressGenericSpinner { surface_id, .. }
            | Self::SuppressionDivergence { surface_id, .. }
            | Self::SuppressionAuditMissing { surface_id, .. }
            | Self::ReopenTargetLost { surface_id, .. }
            | Self::StaleEvidenceOnMarketedRow { surface_id, .. }
            | Self::MissingNarrowingReason { surface_id, .. }
            | Self::MissingProjection { surface_id, .. }
            | Self::DescriptorMissingReopenAnchor { surface_id }
            | Self::MissingDurableJobRef { surface_id }
            | Self::MissingSupportNote { surface_id }
            | Self::MissingSourceObjectLabel { surface_id }
            | Self::MissingSafeReopenAction { surface_id }
            | Self::SurfaceNotDerivedFromDurableObject { surface_id }
            | Self::MissingSuppressionControls { surface_id }
            | Self::EnvelopeDescriptorMismatch { surface_id, .. } => surface_id,
        }
    }

    /// Returns the guarantee this finding is attached to, when guarantee-scoped.
    pub fn guarantee(&self) -> Option<M5OsAttentionGuarantee> {
        match self {
            Self::UnqualifiedDesktopOnlyState { guarantee, .. }
            | Self::MissingEvidence { guarantee, .. }
            | Self::MissingEnvelopeRef { guarantee, .. }
            | Self::LockScreenLeak { guarantee, .. }
            | Self::ProtectedPayloadBody { guarantee, .. }
            | Self::BadgeRawEventFanout { guarantee, .. }
            | Self::ProgressGenericSpinner { guarantee, .. }
            | Self::SuppressionDivergence { guarantee, .. }
            | Self::SuppressionAuditMissing { guarantee, .. }
            | Self::ReopenTargetLost { guarantee, .. }
            | Self::StaleEvidenceOnMarketedRow { guarantee, .. }
            | Self::MissingNarrowingReason { guarantee, .. }
            | Self::MissingProjection { guarantee, .. } => Some(*guarantee),
            Self::DescriptorMissingReopenAnchor { .. }
            | Self::MissingDurableJobRef { .. }
            | Self::MissingSupportNote { .. }
            | Self::MissingSourceObjectLabel { .. }
            | Self::MissingSafeReopenAction { .. }
            | Self::SurfaceNotDerivedFromDurableObject { .. }
            | Self::MissingSuppressionControls { .. }
            | Self::EnvelopeDescriptorMismatch { .. } => None,
        }
    }
}

/// One per-surface OS-attention qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsAttentionRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the surface.
    pub descriptor: M5OsSurfaceDescriptor,
    /// Typed OS notification envelope the surface reads.
    pub envelope: M5OsNotificationEnvelope,
    /// Guarantee-by-guarantee qualification bindings, in canonical order.
    pub bindings: Vec<M5OsAttentionBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5OsAttentionBlockingFinding>,
    /// `true` when the surface's privacy class classifies it as high-stakes.
    pub high_stakes: bool,
    /// `true` when the surface is marketed on desktop.
    pub marketed: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsAttentionFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unqualified_desktop_only_state` findings.
    pub unqualified_desktop_only_state: usize,
    /// Number of `missing_evidence` findings.
    pub missing_evidence: usize,
    /// Number of `missing_envelope_ref` findings.
    pub missing_envelope_ref: usize,
    /// Number of `lock_screen_leak` findings.
    pub lock_screen_leak: usize,
    /// Number of `protected_payload_body` findings.
    pub protected_payload_body: usize,
    /// Number of `badge_raw_event_fanout` findings.
    pub badge_raw_event_fanout: usize,
    /// Number of `progress_generic_spinner` findings.
    pub progress_generic_spinner: usize,
    /// Number of `suppression_divergence` findings.
    pub suppression_divergence: usize,
    /// Number of `suppression_audit_missing` findings.
    pub suppression_audit_missing: usize,
    /// Number of `reopen_target_lost` findings.
    pub reopen_target_lost: usize,
    /// Number of `stale_evidence_on_marketed_row` findings.
    pub stale_evidence_on_marketed_row: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
    /// Number of `descriptor_missing_reopen_anchor` findings.
    pub descriptor_missing_reopen_anchor: usize,
    /// Number of `missing_durable_job_ref` findings.
    pub missing_durable_job_ref: usize,
    /// Number of `missing_support_note` findings.
    pub missing_support_note: usize,
    /// Number of `missing_source_object_label` findings.
    pub missing_source_object_label: usize,
    /// Number of `missing_safe_reopen_action` findings.
    pub missing_safe_reopen_action: usize,
    /// Number of `surface_not_derived_from_durable_object` findings.
    pub surface_not_derived_from_durable_object: usize,
    /// Number of `missing_suppression_controls` findings.
    pub missing_suppression_controls: usize,
    /// Number of `envelope_descriptor_mismatch` findings.
    pub envelope_descriptor_mismatch: usize,
}

impl M5OsAttentionFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unqualified_desktop_only_state: 0,
            missing_evidence: 0,
            missing_envelope_ref: 0,
            lock_screen_leak: 0,
            protected_payload_body: 0,
            badge_raw_event_fanout: 0,
            progress_generic_spinner: 0,
            suppression_divergence: 0,
            suppression_audit_missing: 0,
            reopen_target_lost: 0,
            stale_evidence_on_marketed_row: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_reopen_anchor: 0,
            missing_durable_job_ref: 0,
            missing_support_note: 0,
            missing_source_object_label: 0,
            missing_safe_reopen_action: 0,
            surface_not_derived_from_durable_object: 0,
            missing_suppression_controls: 0,
            envelope_descriptor_mismatch: 0,
        }
    }

    fn record(&mut self, finding: &M5OsAttentionBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5OsAttentionBlockingFinding::UnqualifiedDesktopOnlyState { .. } => {
                self.unqualified_desktop_only_state += 1
            }
            M5OsAttentionBlockingFinding::MissingEvidence { .. } => self.missing_evidence += 1,
            M5OsAttentionBlockingFinding::MissingEnvelopeRef { .. } => {
                self.missing_envelope_ref += 1
            }
            M5OsAttentionBlockingFinding::LockScreenLeak { .. } => self.lock_screen_leak += 1,
            M5OsAttentionBlockingFinding::ProtectedPayloadBody { .. } => {
                self.protected_payload_body += 1
            }
            M5OsAttentionBlockingFinding::BadgeRawEventFanout { .. } => {
                self.badge_raw_event_fanout += 1
            }
            M5OsAttentionBlockingFinding::ProgressGenericSpinner { .. } => {
                self.progress_generic_spinner += 1
            }
            M5OsAttentionBlockingFinding::SuppressionDivergence { .. } => {
                self.suppression_divergence += 1
            }
            M5OsAttentionBlockingFinding::SuppressionAuditMissing { .. } => {
                self.suppression_audit_missing += 1
            }
            M5OsAttentionBlockingFinding::ReopenTargetLost { .. } => self.reopen_target_lost += 1,
            M5OsAttentionBlockingFinding::StaleEvidenceOnMarketedRow { .. } => {
                self.stale_evidence_on_marketed_row += 1
            }
            M5OsAttentionBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5OsAttentionBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5OsAttentionBlockingFinding::DescriptorMissingReopenAnchor { .. } => {
                self.descriptor_missing_reopen_anchor += 1
            }
            M5OsAttentionBlockingFinding::MissingDurableJobRef { .. } => {
                self.missing_durable_job_ref += 1
            }
            M5OsAttentionBlockingFinding::MissingSupportNote { .. } => {
                self.missing_support_note += 1
            }
            M5OsAttentionBlockingFinding::MissingSourceObjectLabel { .. } => {
                self.missing_source_object_label += 1
            }
            M5OsAttentionBlockingFinding::MissingSafeReopenAction { .. } => {
                self.missing_safe_reopen_action += 1
            }
            M5OsAttentionBlockingFinding::SurfaceNotDerivedFromDurableObject { .. } => {
                self.surface_not_derived_from_durable_object += 1
            }
            M5OsAttentionBlockingFinding::MissingSuppressionControls { .. } => {
                self.missing_suppression_controls += 1
            }
            M5OsAttentionBlockingFinding::EnvelopeDescriptorMismatch { .. } => {
                self.envelope_descriptor_mismatch += 1
            }
        }
    }
}

/// Per-guarantee coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsAttentionCoverageSummary {
    /// Guarantee this summary covers.
    pub guarantee: M5OsAttentionGuarantee,
    /// Number of `qualified` rows on this guarantee.
    pub qualified_rows: usize,
    /// Number of `explicitly_narrowed` rows on this guarantee.
    pub explicitly_narrowed_rows: usize,
    /// Number of `not_applicable` rows on this guarantee.
    pub not_applicable_rows: usize,
    /// Number of `platform_omitted` rows on this guarantee.
    pub platform_omitted_rows: usize,
    /// Number of `unqualified_desktop_only_state` rows on this guarantee.
    pub unqualified_desktop_only_state_rows: usize,
    /// Number of `missing_evidence` rows on this guarantee.
    pub missing_evidence_rows: usize,
}

impl M5OsAttentionCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows + self.not_applicable_rows + self.platform_omitted_rows
    }
}

/// A single reopen-anchor index entry the audit publishes so the OS surfaces,
/// docs, and release surfaces can reopen each surface by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsReopenAnchorEntry {
    /// Durable job family the anchor belongs to.
    pub job_family: DurableAttentionJobFamily,
    /// Surface id the anchor reopens.
    pub surface_id: String,
    /// Durable job id ref the anchor resolves to.
    pub durable_job_id_ref: String,
    /// Canonical exact-target reopen anchor ref.
    pub reopen_anchor_ref: String,
}

/// One marketed guarantee release tooling should narrow because its evidence is
/// stale or red.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsNarrowableRow {
    /// Surface id that must narrow.
    pub surface_id: String,
    /// Guarantee that must narrow.
    pub guarantee: M5OsAttentionGuarantee,
    /// Stable reason the row is narrowable.
    pub reason: String,
}

/// M5 OS-attention parity qualification audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsAttentionReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Boundary schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required parity guarantees, in canonical order.
    pub required_guarantees: Vec<M5OsAttentionGuarantee>,
    /// Per-surface qualification rows, sorted by `descriptor.surface_id`.
    pub rows: Vec<M5OsAttentionRow>,
    /// Per-guarantee coverage summary, in canonical order.
    pub guarantee_coverage: Vec<M5OsAttentionCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5OsAttentionFindingSummary,
    /// Canonical reopen-anchor index, sorted by surface id.
    pub reopen_anchor_index: Vec<M5OsReopenAnchorEntry>,
    /// Number of registered OS surfaces present.
    pub registered_surface_count: usize,
    /// Number of high-stakes surfaces present.
    pub high_stakes_surface_count: usize,
    /// Number of surfaces marketed on desktop.
    pub marketed_surface_count: usize,
    /// Total parity guarantees checked.
    pub parity_guarantees_checked: usize,
    /// Marketed rows release tooling should narrow because their evidence is
    /// stale or red.
    pub narrowable_marketed_rows: Vec<M5OsNarrowableRow>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl M5OsAttentionReport {
    /// Returns `true` when every required guarantee is qualified by at least one
    /// surface.
    pub fn every_required_guarantee_qualified(&self) -> bool {
        for guarantee in M5OsAttentionGuarantee::required_guarantees() {
            let any_qualified = self.rows.iter().any(|surface| {
                surface.bindings.iter().any(|binding| {
                    binding.guarantee == guarantee
                        && binding.qualification_status == M5OsQualificationStatus::Qualified
                })
            });
            if !any_qualified {
                return false;
            }
        }
        true
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: surfaces={}, high_stakes={}, marketed={}, guarantees={}, blocking={}, clean={}",
            self.registered_surface_count,
            self.high_stakes_surface_count,
            self.marketed_surface_count,
            self.parity_guarantees_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.guarantee_coverage {
            lines.push(format!(
                "{}: qualified={}, narrowed={}, desktop_only={}, missing_evidence={}",
                coverage.guarantee.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_desktop_only_state_rows,
                coverage.missing_evidence_rows,
            ));
        }
        for surface in &self.rows {
            for finding in &surface.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.surface_id(),
                    finding
                        .guarantee()
                        .map(M5OsAttentionGuarantee::as_str)
                        .unwrap_or("surface"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_rows {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.surface_id,
                narrowable.guarantee.as_str(),
                narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 OS notification, badge, progress, and reopen parity audit\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_os_notifications_and_badges`](../../../../crates/aureline-shell/src/m5_os_notifications_and_badges/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- report-md > \\\n  artifacts/ux/m5/os-notification-and-reopen.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Registered OS surfaces: `{}`\n",
            self.registered_surface_count
        ));
        out.push_str(&format!(
            "- High-stakes surfaces: `{}`\n",
            self.high_stakes_surface_count
        ));
        out.push_str(&format!(
            "- Marketed surfaces: `{}`\n",
            self.marketed_surface_count
        ));
        out.push_str(&format!(
            "- Parity guarantees checked: `{}`\n",
            self.parity_guarantees_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed rows: `{}`\n",
            self.narrowable_marketed_rows.len()
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

        out.push_str("## Per-guarantee coverage\n\n");
        out.push_str(
            "| Parity guarantee | Qualified | Narrowed | Desktop-only | Missing evidence |\n\
             | ---------------- | --------: | -------: | -----------: | ---------------: |\n",
        );
        for coverage in &self.guarantee_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                coverage.guarantee.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_desktop_only_state_rows,
                coverage.missing_evidence_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unqualified_desktop_only_state` | {} |\n",
            self.findings_summary.unqualified_desktop_only_state
        ));
        out.push_str(&format!(
            "| `missing_evidence` | {} |\n",
            self.findings_summary.missing_evidence
        ));
        out.push_str(&format!(
            "| `missing_envelope_ref` | {} |\n",
            self.findings_summary.missing_envelope_ref
        ));
        out.push_str(&format!(
            "| `lock_screen_leak` | {} |\n",
            self.findings_summary.lock_screen_leak
        ));
        out.push_str(&format!(
            "| `protected_payload_body` | {} |\n",
            self.findings_summary.protected_payload_body
        ));
        out.push_str(&format!(
            "| `badge_raw_event_fanout` | {} |\n",
            self.findings_summary.badge_raw_event_fanout
        ));
        out.push_str(&format!(
            "| `progress_generic_spinner` | {} |\n",
            self.findings_summary.progress_generic_spinner
        ));
        out.push_str(&format!(
            "| `suppression_divergence` | {} |\n",
            self.findings_summary.suppression_divergence
        ));
        out.push_str(&format!(
            "| `suppression_audit_missing` | {} |\n",
            self.findings_summary.suppression_audit_missing
        ));
        out.push_str(&format!(
            "| `reopen_target_lost` | {} |\n",
            self.findings_summary.reopen_target_lost
        ));
        out.push_str(&format!(
            "| `stale_evidence_on_marketed_row` | {} |\n",
            self.findings_summary.stale_evidence_on_marketed_row
        ));
        out.push_str(&format!(
            "| `missing_narrowing_reason` | {} |\n",
            self.findings_summary.missing_narrowing_reason
        ));
        out.push_str(&format!(
            "| `missing_projection` | {} |\n",
            self.findings_summary.missing_projection
        ));
        out.push_str(&format!(
            "| `descriptor_missing_reopen_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_reopen_anchor
        ));
        out.push_str(&format!(
            "| `missing_durable_job_ref` | {} |\n",
            self.findings_summary.missing_durable_job_ref
        ));
        out.push_str(&format!(
            "| `missing_support_note` | {} |\n",
            self.findings_summary.missing_support_note
        ));
        out.push_str(&format!(
            "| `missing_source_object_label` | {} |\n",
            self.findings_summary.missing_source_object_label
        ));
        out.push_str(&format!(
            "| `missing_safe_reopen_action` | {} |\n",
            self.findings_summary.missing_safe_reopen_action
        ));
        out.push_str(&format!(
            "| `surface_not_derived_from_durable_object` | {} |\n",
            self.findings_summary
                .surface_not_derived_from_durable_object
        ));
        out.push_str(&format!(
            "| `missing_suppression_controls` | {} |\n",
            self.findings_summary.missing_suppression_controls
        ));
        out.push_str(&format!(
            "| `envelope_descriptor_mismatch` | {} |\n\n",
            self.findings_summary.envelope_descriptor_mismatch
        ));

        out.push_str("## Reopen anchor index\n\n");
        out.push_str(
            "| Durable job family | Surface id | Durable job ref | Reopen anchor |\n| ------------------ | ---------- | --------------- | ------------- |\n",
        );
        for entry in &self.reopen_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` |\n",
                entry.job_family.as_str(),
                entry.surface_id,
                entry.durable_job_id_ref,
                entry.reopen_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface rows\n\n");
        for surface in &self.rows {
            out.push_str(&format!(
                "### `{}` ({}, {}, {})\n\n",
                surface.descriptor.surface_id,
                surface.descriptor.job_family.as_str(),
                surface.descriptor.privacy_class.as_str(),
                surface.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Durable job ref: `{}`\n",
                surface.descriptor.durable_job_id_ref
            ));
            out.push_str(&format!(
                "- Job state class: `{}`\n",
                surface.descriptor.job_state_class.as_str()
            ));
            out.push_str(&format!(
                "- Client scope: `{}`\n",
                surface.descriptor.client_scope.as_str()
            ));
            out.push_str(&format!(
                "- Badge count class: `{}`\n",
                surface.envelope.badge_count_class.as_str()
            ));
            out.push_str(&format!(
                "- Reopen anchor: `{}`\n",
                surface.descriptor.reopen_anchor_ref
            ));
            out.push_str(&format!(
                "- Suppression controls: {}\n",
                if surface.descriptor.suppression_controls.is_empty() {
                    "none".to_owned()
                } else {
                    surface
                        .descriptor
                        .suppression_controls
                        .iter()
                        .map(|control| format!("`{}`", control.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
            out.push_str(&format!(
                "- Marketed on desktop: `{}`\n",
                if surface.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- High-stakes: `{}`\n\n",
                if surface.high_stakes { "yes" } else { "no" }
            ));

            out.push_str(
                "| Parity guarantee | Status | Lock screen | Payload | Badge | Progress | Suppression | Reopen | Freshness | Narrowing reason |\n\
                 | ---------------- | ------ | ----------- | ------- | ----- | -------- | ----------- | ------ | --------- | ---------------- |\n",
            );
            for binding in &surface.bindings {
                let lock_screen = binding
                    .projected_lock_screen
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let payload = binding
                    .projected_payload_disclosure
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let badge = binding
                    .projected_badge_basis
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let progress = binding
                    .projected_progress_basis
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let suppression = binding
                    .projected_suppression_parity
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let reopen = binding
                    .projected_reopen_outcome
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let freshness = binding
                    .evidence_freshness
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let narrowing = binding.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.guarantee.display_label(),
                    binding.qualification_status.as_str(),
                    lock_screen,
                    payload,
                    badge,
                    progress,
                    suppression,
                    reopen,
                    freshness,
                    narrowing,
                ));
            }
            out.push('\n');

            if surface.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &surface.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .guarantee()
                            .map(M5OsAttentionGuarantee::as_str)
                            .unwrap_or("surface"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_os_notifications_and_badges_fixtures\n",
        );
        out.push_str("python3 tools/ci/m5/os_notifications_and_badges_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 OS-attention audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OsAttentionSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5OsAttentionReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5OsAttentionSupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(support_export_id: impl Into<String>, report: M5OsAttentionReport) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for surface in &report.rows {
            case_ids.push(surface.descriptor.surface_id.clone());
            case_ids.push(surface.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_OS_ATTENTION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_OS_ATTENTION_SCHEMA_VERSION,
            shared_contract_ref: M5_OS_ATTENTION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-surface blocking findings from a descriptor, its envelope,
/// and its guarantee bindings.
fn compute_surface_findings(
    descriptor: &M5OsSurfaceDescriptor,
    envelope: &M5OsNotificationEnvelope,
    bindings: &[M5OsAttentionBinding],
    high_stakes: bool,
) -> Vec<M5OsAttentionBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (surface-scoped) findings.
    if descriptor.reopen_anchor_ref.trim().is_empty() {
        findings.push(
            M5OsAttentionBlockingFinding::DescriptorMissingReopenAnchor {
                surface_id: descriptor.surface_id.clone(),
            },
        );
    }
    if descriptor.durable_job_id_ref.trim().is_empty() {
        findings.push(M5OsAttentionBlockingFinding::MissingDurableJobRef {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.support_note.trim().is_empty() {
        findings.push(M5OsAttentionBlockingFinding::MissingSupportNote {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.source_object_label_ref.trim().is_empty() {
        findings.push(M5OsAttentionBlockingFinding::MissingSourceObjectLabel {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.safe_reopen_action_label_ref.trim().is_empty() {
        findings.push(M5OsAttentionBlockingFinding::MissingSafeReopenAction {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if !descriptor.derived_from_durable_object {
        findings.push(
            M5OsAttentionBlockingFinding::SurfaceNotDerivedFromDurableObject {
                surface_id: descriptor.surface_id.clone(),
            },
        );
    }
    if high_stakes && descriptor.suppression_controls.is_empty() {
        findings.push(M5OsAttentionBlockingFinding::MissingSuppressionControls {
            surface_id: descriptor.surface_id.clone(),
        });
    }

    // Envelope/descriptor consistency: the envelope must derive from the same
    // durable object and reopen anchor the descriptor names.
    if envelope.durable_job_id_ref != descriptor.durable_job_id_ref {
        findings.push(M5OsAttentionBlockingFinding::EnvelopeDescriptorMismatch {
            surface_id: descriptor.surface_id.clone(),
            field: "durable_job_id_ref".to_owned(),
        });
    }
    if envelope.reopen_linkage.reopen_anchor_ref != descriptor.reopen_anchor_ref {
        findings.push(M5OsAttentionBlockingFinding::EnvelopeDescriptorMismatch {
            surface_id: descriptor.surface_id.clone(),
            field: "reopen_anchor_ref".to_owned(),
        });
    }

    for binding in bindings {
        let guarantee = binding.guarantee;
        let surface_id = descriptor.surface_id.clone();

        match binding.qualification_status {
            M5OsQualificationStatus::UnqualifiedDesktopOnlyState => {
                findings.push(M5OsAttentionBlockingFinding::UnqualifiedDesktopOnlyState {
                    surface_id: surface_id.clone(),
                    guarantee,
                });
            }
            M5OsQualificationStatus::MissingEvidence => {
                findings.push(M5OsAttentionBlockingFinding::MissingEvidence {
                    surface_id: surface_id.clone(),
                    guarantee,
                });
            }
            M5OsQualificationStatus::Qualified => {
                compute_qualified_findings(binding, high_stakes, &surface_id, &mut findings);
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = binding
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(M5OsAttentionBlockingFinding::MissingNarrowingReason {
                        surface_id: surface_id.clone(),
                        guarantee,
                        qualification_status: status,
                    });
                }
            }
            _ => {}
        }
    }
    findings
}

/// Computes the blocking findings for one qualified parity binding.
fn compute_qualified_findings(
    binding: &M5OsAttentionBinding,
    high_stakes: bool,
    surface_id: &str,
    findings: &mut Vec<M5OsAttentionBlockingFinding>,
) {
    let guarantee = binding.guarantee;

    // Required captured-evidence projections (universal for qualified rows).
    if binding.projected_envelope_ref.is_none() {
        findings.push(M5OsAttentionBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_envelope_ref".to_owned(),
        });
    }
    if binding.projected_privacy_class.is_none() {
        findings.push(M5OsAttentionBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_privacy_class".to_owned(),
        });
    }
    if binding.evidence_freshness.is_none() {
        findings.push(M5OsAttentionBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "evidence_freshness".to_owned(),
        });
    }

    // Guarantee-specific required projections.
    if guarantee.requires_privacy_disclosures() {
        if binding.projected_lock_screen.is_none() {
            findings.push(M5OsAttentionBlockingFinding::MissingProjection {
                surface_id: surface_id.to_owned(),
                guarantee,
                field: "projected_lock_screen".to_owned(),
            });
        }
        if binding.projected_payload_disclosure.is_none() {
            findings.push(M5OsAttentionBlockingFinding::MissingProjection {
                surface_id: surface_id.to_owned(),
                guarantee,
                field: "projected_payload_disclosure".to_owned(),
            });
        }
    }
    if guarantee.requires_badge() {
        if binding.projected_badge_basis.is_none() {
            findings.push(M5OsAttentionBlockingFinding::MissingProjection {
                surface_id: surface_id.to_owned(),
                guarantee,
                field: "projected_badge_basis".to_owned(),
            });
        }
        if binding.projected_badge_count_class.is_none() {
            findings.push(M5OsAttentionBlockingFinding::MissingProjection {
                surface_id: surface_id.to_owned(),
                guarantee,
                field: "projected_badge_count_class".to_owned(),
            });
        }
    }
    if guarantee.requires_progress() && binding.projected_progress_basis.is_none() {
        findings.push(M5OsAttentionBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_progress_basis".to_owned(),
        });
    }
    if guarantee.requires_suppression() {
        if binding.projected_suppression_parity.is_none() {
            findings.push(M5OsAttentionBlockingFinding::MissingProjection {
                surface_id: surface_id.to_owned(),
                guarantee,
                field: "projected_suppression_parity".to_owned(),
            });
        }
        if binding.projected_suppression_decision.is_none() {
            findings.push(M5OsAttentionBlockingFinding::MissingProjection {
                surface_id: surface_id.to_owned(),
                guarantee,
                field: "projected_suppression_decision".to_owned(),
            });
        }
        if binding.projected_suppression_audit_visible.is_none() {
            findings.push(M5OsAttentionBlockingFinding::MissingProjection {
                surface_id: surface_id.to_owned(),
                guarantee,
                field: "projected_suppression_audit_visible".to_owned(),
            });
        }
    }
    if guarantee.requires_reopen_outcome() && binding.projected_reopen_outcome.is_none() {
        findings.push(M5OsAttentionBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_reopen_outcome".to_owned(),
        });
    }
    if high_stakes && binding.projected_reopen_outcome.is_none() {
        findings.push(M5OsAttentionBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            guarantee,
            field: "projected_reopen_outcome".to_owned(),
        });
    }

    // Red captured results.
    if binding.projected_envelope_ref.is_none() {
        findings.push(M5OsAttentionBlockingFinding::MissingEnvelopeRef {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_lock_screen == Some(M5OsLockScreenDisclosure::LeaksProtectedDetail) {
        findings.push(M5OsAttentionBlockingFinding::LockScreenLeak {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_payload_disclosure == Some(M5OsPayloadDisclosure::CarriesProtectedBody) {
        findings.push(M5OsAttentionBlockingFinding::ProtectedPayloadBody {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_badge_basis == Some(M5OsBadgeBasis::RawEventFanout) {
        findings.push(M5OsAttentionBlockingFinding::BadgeRawEventFanout {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_progress_basis == Some(M5OsProgressBasis::GenericSpinner) {
        findings.push(M5OsAttentionBlockingFinding::ProgressGenericSpinner {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_suppression_parity == Some(M5OsSuppressionParity::DivergesFromInApp) {
        findings.push(M5OsAttentionBlockingFinding::SuppressionDivergence {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if guarantee.requires_suppression()
        && binding.projected_suppression_audit_visible == Some(false)
    {
        findings.push(M5OsAttentionBlockingFinding::SuppressionAuditMissing {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_reopen_outcome == Some(M5OsReopenOutcome::TargetLost) {
        findings.push(M5OsAttentionBlockingFinding::ReopenTargetLost {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
    if binding.marketed_on_guarantee
        && binding.evidence_freshness == Some(M5OsEvidenceFreshness::Stale)
    {
        findings.push(M5OsAttentionBlockingFinding::StaleEvidenceOnMarketedRow {
            surface_id: surface_id.to_owned(),
            guarantee,
        });
    }
}

/// Computes the per-guarantee coverage and per-class finding summary.
fn summarize_report(
    surfaces: &[M5OsAttentionRow],
) -> (
    Vec<M5OsAttentionCoverageSummary>,
    M5OsAttentionFindingSummary,
) {
    let mut coverage: Vec<M5OsAttentionCoverageSummary> =
        M5OsAttentionGuarantee::required_guarantees()
            .iter()
            .map(|guarantee| M5OsAttentionCoverageSummary {
                guarantee: *guarantee,
                qualified_rows: 0,
                explicitly_narrowed_rows: 0,
                not_applicable_rows: 0,
                platform_omitted_rows: 0,
                unqualified_desktop_only_state_rows: 0,
                missing_evidence_rows: 0,
            })
            .collect();
    let mut summary = M5OsAttentionFindingSummary::empty();

    for surface in surfaces {
        for binding in &surface.bindings {
            if let Some(coverage_row) = coverage
                .iter_mut()
                .find(|row| row.guarantee == binding.guarantee)
            {
                match binding.qualification_status {
                    M5OsQualificationStatus::Qualified => coverage_row.qualified_rows += 1,
                    M5OsQualificationStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5OsQualificationStatus::NotApplicable => coverage_row.not_applicable_rows += 1,
                    M5OsQualificationStatus::PlatformOmitted => {
                        coverage_row.platform_omitted_rows += 1
                    }
                    M5OsQualificationStatus::UnqualifiedDesktopOnlyState => {
                        coverage_row.unqualified_desktop_only_state_rows += 1
                    }
                    M5OsQualificationStatus::MissingEvidence => {
                        coverage_row.missing_evidence_rows += 1
                    }
                }
            }
        }
        for finding in &surface.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Computes the marketed rows release tooling should narrow because their
/// evidence is stale or red.
fn compute_narrowable_rows(surfaces: &[M5OsAttentionRow]) -> Vec<M5OsNarrowableRow> {
    let mut narrowable = Vec::new();
    for surface in surfaces {
        if !surface.marketed {
            continue;
        }
        for finding in &surface.blocking_findings {
            if let Some(guarantee) = finding.guarantee() {
                narrowable.push(M5OsNarrowableRow {
                    surface_id: surface.descriptor.surface_id.clone(),
                    guarantee,
                    reason: format!("blocking_finding:{}", finding.class_token()),
                });
            }
        }
    }
    narrowable
}

/// Builds an [`M5OsAttentionRow`] from a descriptor, envelope, and bindings,
/// computing the per-surface blocking findings.
pub fn build_m5_os_attention_row(
    descriptor: M5OsSurfaceDescriptor,
    envelope: M5OsNotificationEnvelope,
    bindings: Vec<M5OsAttentionBinding>,
) -> M5OsAttentionRow {
    let high_stakes = descriptor.is_high_stakes();
    let marketed = descriptor.marketed_on_desktop;
    let blocking_findings =
        compute_surface_findings(&descriptor, &envelope, &bindings, high_stakes);

    M5OsAttentionRow {
        record_kind: M5_OS_ATTENTION_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_OS_ATTENTION_SCHEMA_VERSION,
        shared_contract_ref: M5_OS_ATTENTION_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        envelope,
        bindings,
        blocking_findings,
        high_stakes,
        marketed,
    }
}

/// Builds a full [`M5OsAttentionReport`] from per-surface rows.
pub fn build_m5_os_attention_report(surfaces: Vec<M5OsAttentionRow>) -> M5OsAttentionReport {
    let mut surfaces = surfaces;
    surfaces.sort_by(|left, right| left.descriptor.surface_id.cmp(&right.descriptor.surface_id));

    let registered_surface_count = surfaces.len();
    let high_stakes_surface_count = surfaces.iter().filter(|row| row.high_stakes).count();
    let marketed_surface_count = surfaces.iter().filter(|row| row.marketed).count();
    let parity_guarantees_checked = surfaces.iter().map(|row| row.bindings.len()).sum::<usize>();

    let (guarantee_coverage, findings_summary) = summarize_report(&surfaces);
    let narrowable_marketed_rows = compute_narrowable_rows(&surfaces);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut reopen_anchor_index: Vec<M5OsReopenAnchorEntry> = surfaces
        .iter()
        .map(|surface| M5OsReopenAnchorEntry {
            job_family: surface.descriptor.job_family,
            surface_id: surface.descriptor.surface_id.clone(),
            durable_job_id_ref: surface.descriptor.durable_job_id_ref.clone(),
            reopen_anchor_ref: surface.descriptor.reopen_anchor_ref.clone(),
        })
        .collect();
    reopen_anchor_index.sort_by(|left, right| left.surface_id.cmp(&right.surface_id));

    M5OsAttentionReport {
        record_kind: M5_OS_ATTENTION_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_OS_ATTENTION_SCHEMA_VERSION,
        shared_contract_ref: M5_OS_ATTENTION_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_OS_ATTENTION_REPORT_ID.to_owned(),
        source_schema_ref: M5_OS_ATTENTION_SOURCE_SCHEMA_REF.to_owned(),
        required_guarantees: M5OsAttentionGuarantee::required_guarantees().to_vec(),
        rows: surfaces,
        guarantee_coverage,
        findings_summary,
        reopen_anchor_index,
        registered_surface_count,
        high_stakes_surface_count,
        marketed_surface_count,
        parity_guarantees_checked,
        narrowable_marketed_rows,
        report_clean,
        published_report_ref: M5_OS_ATTENTION_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_OS_ATTENTION_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_OS_ATTENTION_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/notification-privacy-and-badges.md".to_owned(),
            "docs/m5/durable-progress-and-reopen.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-os-notifications-and-badges".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_os_attention_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5OsAttentionValidationError {
    /// The audit has no registered surfaces.
    NoRegisteredSurfaces,
    /// A required parity guarantee has no qualified surface.
    RequiredGuaranteeNotQualified { guarantee: String },
    /// A surface is missing a required guarantee from its binding set.
    MissingRequiredGuarantee {
        surface_id: String,
        guarantee: String,
    },
    /// A blocking finding remains on the surface.
    BlockingFindingPresent {
        surface_id: String,
        guarantee: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A surface's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { surface_id: String },
}

/// Validates an audit report against the M5 OS-attention acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_os_attention_report(
    report: &M5OsAttentionReport,
) -> Result<(), Vec<M5OsAttentionValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5OsAttentionValidationError::NoRegisteredSurfaces);
    }

    for guarantee in M5OsAttentionGuarantee::required_guarantees() {
        let any_qualified = report.rows.iter().any(|surface| {
            surface.bindings.iter().any(|binding| {
                binding.guarantee == guarantee
                    && binding.qualification_status == M5OsQualificationStatus::Qualified
            })
        });
        if !any_qualified {
            errors.push(
                M5OsAttentionValidationError::RequiredGuaranteeNotQualified {
                    guarantee: guarantee.as_str().to_owned(),
                },
            );
        }
    }

    for surface in &report.rows {
        for guarantee in M5OsAttentionGuarantee::required_guarantees() {
            if !surface
                .bindings
                .iter()
                .any(|binding| binding.guarantee == guarantee)
            {
                errors.push(M5OsAttentionValidationError::MissingRequiredGuarantee {
                    surface_id: surface.descriptor.surface_id.clone(),
                    guarantee: guarantee.as_str().to_owned(),
                });
            }
        }
        if surface.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(M5OsAttentionValidationError::MissingDescriptorRevisionRef {
                surface_id: surface.descriptor.surface_id.clone(),
            });
        }
        for finding in &surface.blocking_findings {
            errors.push(M5OsAttentionValidationError::BlockingFindingPresent {
                surface_id: finding.surface_id().to_owned(),
                guarantee: finding
                    .guarantee()
                    .map(|guarantee| guarantee.as_str().to_owned())
                    .unwrap_or_else(|| "surface".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(M5OsAttentionValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5OsAttentionValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_os_attention_report`].
struct SurfaceSeed {
    surface_id: &'static str,
    job_family: DurableAttentionJobFamily,
    job_state_class: DurableJobRowStateClass,
    durable_job_id_ref: &'static str,
    canonical_event_id_ref: &'static str,
    descriptor_revision_ref: &'static str,
    support_note: &'static str,
    privacy_class: M5OsPrivacyClass,
    client_scope: M5OsClientScope,
    lifecycle_label: M5OsSurfaceLifecycle,
    source_object_label_ref: &'static str,
    safe_reopen_action_label_ref: &'static str,
    reopen_anchor_ref: &'static str,
    command_id_ref: &'static str,
    suppression_controls: &'static [M5OsSuppressionControl],
    badge_count_class: AggregateCountClass,
    progress_basis: Option<M5OsProgressBasis>,
    suppression_decision: QuietHoursDecisionClass,
    reopen_outcome: M5OsReopenOutcome,
    bindings: &'static [BindingSeed],
}

struct BindingSeed {
    guarantee: M5OsAttentionGuarantee,
    qualification_status: M5OsQualificationStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
}

/// Helper: a qualified guarantee with captured evidence.
const fn qualified(guarantee: M5OsAttentionGuarantee) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5OsQualificationStatus::Qualified,
        narrowing_reason: None,
        note: None,
    }
}

/// Helper: a not-applicable guarantee with a documented reason.
const fn not_applicable(guarantee: M5OsAttentionGuarantee, reason: &'static str) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5OsQualificationStatus::NotApplicable,
        narrowing_reason: Some(reason),
        note: None,
    }
}

use M5OsAttentionGuarantee::{
    BadgeDurableClass, ExactReopenParity, PrivacySafeSummary, ProgressNamedJobClass,
    SuppressionParity,
};
use M5OsSuppressionControl::{
    AdminSuppress, DoNotDisturb, LockScreenSummary, Mute, QuietHours, Snooze,
};

const FULL_SUPPRESSION: &[M5OsSuppressionControl] = &[
    QuietHours,
    DoNotDisturb,
    AdminSuppress,
    Mute,
    Snooze,
    LockScreenSummary,
];

const BASIC_SUPPRESSION: &[M5OsSuppressionControl] =
    &[QuietHours, DoNotDisturb, Mute, Snooze, LockScreenSummary];

/// Bindings for a surface that exposes taskbar/dock progress (a running job).
const PROGRESS_BINDINGS: &[BindingSeed] = &[
    qualified(PrivacySafeSummary),
    qualified(BadgeDurableClass),
    qualified(ProgressNamedJobClass),
    qualified(SuppressionParity),
    qualified(ExactReopenParity),
];

/// Bindings for a surface with no progress affordance (approval / advisory
/// state): the progress guarantee is narrowed honestly.
const NO_PROGRESS_BINDINGS: &[BindingSeed] = &[
    qualified(PrivacySafeSummary),
    qualified(BadgeDurableClass),
    not_applicable(
        ProgressNamedJobClass,
        "approval_or_advisory_state_exposes_no_taskbar_or_dock_progress_affordance",
    ),
    qualified(SuppressionParity),
    qualified(ExactReopenParity),
];

const SURFACE_SEEDS: &[SurfaceSeed] = &[
    // Background indexing / search-readiness. Summary-safe; running with
    // progress; counts as durable-running work.
    SurfaceSeed {
        surface_id: "os:indexing",
        job_family: DurableAttentionJobFamily::Indexing,
        job_state_class: DurableJobRowStateClass::Running,
        durable_job_id_ref: "obj:durable-job:indexing:2026.06.16-01",
        canonical_event_id_ref: "ux:event:indexing:2026.06.16-01",
        descriptor_revision_ref: "os-rev:indexing:2026.06.16-01",
        support_note: "Indexing OS attention derives from the durable indexing job: the dock badge counts durable-running work, the taskbar progress maps to the named indexing job class, and the notification reopens the exact indexing job.",
        privacy_class: M5OsPrivacyClass::SummarySafe,
        client_scope: M5OsClientScope::DesktopProduct,
        lifecycle_label: M5OsSurfaceLifecycle::Beta,
        source_object_label_ref: "label:os.indexing:source_object",
        safe_reopen_action_label_ref: "label:os.indexing:open_durable_job",
        reopen_anchor_ref: "os:reopen:indexing",
        command_id_ref: "cmd:activity.open_durable_job_row",
        suppression_controls: BASIC_SUPPRESSION,
        badge_count_class: AggregateCountClass::DurableRunning,
        progress_basis: Some(M5OsProgressBasis::NamedDurableJobClass),
        suppression_decision: QuietHoursDecisionClass::NotSuppressed,
        reopen_outcome: M5OsReopenOutcome::ExactDurableObject,
        bindings: PROGRESS_BINDINGS,
    },
    // Install / update / download. Summary-safe; running with progress.
    SurfaceSeed {
        surface_id: "os:install_update",
        job_family: DurableAttentionJobFamily::InstallUpdateDownload,
        job_state_class: DurableJobRowStateClass::Running,
        durable_job_id_ref: "obj:durable-job:install_update:2026.06.16-01",
        canonical_event_id_ref: "ux:event:install_update:2026.06.16-01",
        descriptor_revision_ref: "os-rev:install_update:2026.06.16-01",
        support_note: "Install/update OS attention derives from the durable install job: the taskbar progress maps to the named install job class, the lock-screen copy stays a bounded summary, and the notification reopens the exact install job.",
        privacy_class: M5OsPrivacyClass::SummarySafe,
        client_scope: M5OsClientScope::DesktopProduct,
        lifecycle_label: M5OsSurfaceLifecycle::Beta,
        source_object_label_ref: "label:os.install_update:source_object",
        safe_reopen_action_label_ref: "label:os.install_update:open_durable_job",
        reopen_anchor_ref: "os:reopen:install_update",
        command_id_ref: "cmd:activity.open_durable_job_row",
        suppression_controls: BASIC_SUPPRESSION,
        badge_count_class: AggregateCountClass::DurableRunning,
        progress_basis: Some(M5OsProgressBasis::NamedDurableJobClass),
        suppression_decision: QuietHoursDecisionClass::HeldQuietHours,
        reopen_outcome: M5OsReopenOutcome::ExactDurableObject,
        bindings: PROGRESS_BINDINGS,
    },
    // Notebook / task run. Workspace-sensitive; running with progress.
    SurfaceSeed {
        surface_id: "os:task_run",
        job_family: DurableAttentionJobFamily::TaskRun,
        job_state_class: DurableJobRowStateClass::Running,
        durable_job_id_ref: "obj:durable-job:task_run:2026.06.16-01",
        canonical_event_id_ref: "ux:event:task_run:2026.06.16-01",
        descriptor_revision_ref: "os-rev:task_run:2026.06.16-01",
        support_note: "Task-run OS attention derives from the durable task job: the lock-screen copy references the workspace by name only, the badge counts durable-running work, the taskbar progress maps to the named task class, and the notification reopens the exact task run.",
        privacy_class: M5OsPrivacyClass::WorkspaceSensitive,
        client_scope: M5OsClientScope::DesktopProduct,
        lifecycle_label: M5OsSurfaceLifecycle::Beta,
        source_object_label_ref: "label:os.task_run:source_object",
        safe_reopen_action_label_ref: "label:os.task_run:open_durable_job",
        reopen_anchor_ref: "os:reopen:task_run",
        command_id_ref: "cmd:activity.open_durable_job_row",
        suppression_controls: BASIC_SUPPRESSION,
        badge_count_class: AggregateCountClass::DurableRunning,
        progress_basis: Some(M5OsProgressBasis::NamedDurableJobClass),
        suppression_decision: QuietHoursDecisionClass::NotSuppressed,
        reopen_outcome: M5OsReopenOutcome::ExactDurableObject,
        bindings: PROGRESS_BINDINGS,
    },
    // Test run. Workspace-sensitive; running with progress; failed runs count.
    SurfaceSeed {
        surface_id: "os:test_run",
        job_family: DurableAttentionJobFamily::TestRun,
        job_state_class: DurableJobRowStateClass::Failed,
        durable_job_id_ref: "obj:durable-job:test_run:2026.06.16-01",
        canonical_event_id_ref: "ux:event:test_run:2026.06.16-01",
        descriptor_revision_ref: "os-rev:test_run:2026.06.16-01",
        support_note: "Test-run OS attention derives from the durable test job: the badge counts failed runs as a durable class, the taskbar progress maps to the named test class while running, and the notification reopens the exact failing test run.",
        privacy_class: M5OsPrivacyClass::WorkspaceSensitive,
        client_scope: M5OsClientScope::DesktopProduct,
        lifecycle_label: M5OsSurfaceLifecycle::Beta,
        source_object_label_ref: "label:os.test_run:source_object",
        safe_reopen_action_label_ref: "label:os.test_run:open_durable_job",
        reopen_anchor_ref: "os:reopen:test_run",
        command_id_ref: "cmd:activity.open_durable_job_row",
        suppression_controls: BASIC_SUPPRESSION,
        badge_count_class: AggregateCountClass::FailedRuns,
        progress_basis: Some(M5OsProgressBasis::NamedDurableJobClass),
        suppression_decision: QuietHoursDecisionClass::NotSuppressed,
        reopen_outcome: M5OsReopenOutcome::ExactDurableObject,
        bindings: PROGRESS_BINDINGS,
    },
    // Remote / data-API reconnect. Security-critical; provider-auth attention.
    SurfaceSeed {
        surface_id: "os:remote_reconnect",
        job_family: DurableAttentionJobFamily::RemoteReconnect,
        job_state_class: DurableJobRowStateClass::Running,
        durable_job_id_ref: "obj:durable-job:remote_reconnect:2026.06.16-01",
        canonical_event_id_ref: "ux:event:remote_reconnect:2026.06.16-01",
        descriptor_revision_ref: "os-rev:remote_reconnect:2026.06.16-01",
        support_note: "Remote-reconnect OS attention is security-critical: the lock-screen copy never exposes credentials or endpoints, the badge counts provider-auth attention as a durable class, and the notification reopens the exact reconnect job through the in-product surface.",
        privacy_class: M5OsPrivacyClass::SecurityCritical,
        client_scope: M5OsClientScope::DesktopProduct,
        lifecycle_label: M5OsSurfaceLifecycle::Beta,
        source_object_label_ref: "label:os.remote_reconnect:source_object",
        safe_reopen_action_label_ref: "label:os.remote_reconnect:open_durable_job",
        reopen_anchor_ref: "os:reopen:remote_reconnect",
        command_id_ref: "cmd:activity.open_durable_job_row",
        suppression_controls: FULL_SUPPRESSION,
        badge_count_class: AggregateCountClass::ProviderAuthAttention,
        progress_basis: Some(M5OsProgressBasis::NamedDurableJobClass),
        suppression_decision: QuietHoursDecisionClass::CriticalBypass,
        reopen_outcome: M5OsReopenOutcome::ExactDurableObject,
        bindings: PROGRESS_BINDINGS,
    },
    // AI review / apply. Security-critical; needs approval; no progress bar.
    SurfaceSeed {
        surface_id: "os:ai_review",
        job_family: DurableAttentionJobFamily::AiReview,
        job_state_class: DurableJobRowStateClass::NeedsApproval,
        durable_job_id_ref: "obj:durable-job:ai_review:2026.06.16-01",
        canonical_event_id_ref: "ux:event:ai_review:2026.06.16-01",
        descriptor_revision_ref: "os-rev:ai_review:2026.06.16-01",
        support_note: "AI-review OS attention is security-critical: the lock-screen copy never exposes prompt text, diffs, or generated code, the badge counts pending review/approval as a durable class, and the notification reopens the exact review for an in-product decision.",
        privacy_class: M5OsPrivacyClass::SecurityCritical,
        client_scope: M5OsClientScope::DesktopProduct,
        lifecycle_label: M5OsSurfaceLifecycle::Beta,
        source_object_label_ref: "label:os.ai_review:source_object",
        safe_reopen_action_label_ref: "label:os.ai_review:open_review",
        reopen_anchor_ref: "os:reopen:ai_review",
        command_id_ref: "cmd:activity.open_durable_job_row",
        suppression_controls: FULL_SUPPRESSION,
        badge_count_class: AggregateCountClass::PendingReviewApproval,
        progress_basis: None,
        suppression_decision: QuietHoursDecisionClass::NotSuppressed,
        reopen_outcome: M5OsReopenOutcome::ExactDurableObject,
        bindings: NO_PROGRESS_BINDINGS,
    },
    // Git / hosted review. Workspace-sensitive; needs approval; no progress bar.
    SurfaceSeed {
        surface_id: "os:git_review",
        job_family: DurableAttentionJobFamily::GitReview,
        job_state_class: DurableJobRowStateClass::NeedsApproval,
        durable_job_id_ref: "obj:durable-job:git_review:2026.06.16-01",
        canonical_event_id_ref: "ux:event:git_review:2026.06.16-01",
        descriptor_revision_ref: "os-rev:git_review:2026.06.16-01",
        support_note: "Git-review OS attention derives from the durable review job: the badge counts pending review/approval as a durable class and the notification reopens the exact review; review states expose no taskbar or dock progress.",
        privacy_class: M5OsPrivacyClass::WorkspaceSensitive,
        client_scope: M5OsClientScope::DesktopProduct,
        lifecycle_label: M5OsSurfaceLifecycle::Beta,
        source_object_label_ref: "label:os.git_review:source_object",
        safe_reopen_action_label_ref: "label:os.git_review:open_review",
        reopen_anchor_ref: "os:reopen:git_review",
        command_id_ref: "cmd:activity.open_durable_job_row",
        suppression_controls: BASIC_SUPPRESSION,
        badge_count_class: AggregateCountClass::PendingReviewApproval,
        progress_basis: None,
        suppression_decision: QuietHoursDecisionClass::HeldQuietHours,
        reopen_outcome: M5OsReopenOutcome::ExactDurableObject,
        bindings: NO_PROGRESS_BINDINGS,
    },
    // Admin / policy advisory (managed sync). Managed-sensitive; admin
    // suppression; no progress bar.
    SurfaceSeed {
        surface_id: "os:admin_policy",
        job_family: DurableAttentionJobFamily::AdminPolicy,
        job_state_class: DurableJobRowStateClass::NeedsApproval,
        durable_job_id_ref: "obj:durable-job:admin_policy:2026.06.16-01",
        canonical_event_id_ref: "ux:event:admin_policy:2026.06.16-01",
        descriptor_revision_ref: "os-rev:admin_policy:2026.06.16-01",
        support_note: "Admin-policy OS attention is managed-sensitive: admin suppression is honoured identically on every surface with a visible audit, the badge counts managed advisories as a durable class, and the notification reopens the exact advisory through the in-product surface.",
        privacy_class: M5OsPrivacyClass::ManagedSensitive,
        client_scope: M5OsClientScope::ManagedDesktop,
        lifecycle_label: M5OsSurfaceLifecycle::Beta,
        source_object_label_ref: "label:os.admin_policy:source_object",
        safe_reopen_action_label_ref: "label:os.admin_policy:open_advisory",
        reopen_anchor_ref: "os:reopen:admin_policy",
        command_id_ref: "cmd:activity.open_durable_job_row",
        suppression_controls: FULL_SUPPRESSION,
        badge_count_class: AggregateCountClass::ManagedAdvisories,
        progress_basis: None,
        suppression_decision: QuietHoursDecisionClass::AdminSuppressed,
        reopen_outcome: M5OsReopenOutcome::ExactDurableObject,
        bindings: NO_PROGRESS_BINDINGS,
    },
];

fn build_binding_from_seed(seed: &SurfaceSeed, binding_seed: &BindingSeed) -> M5OsAttentionBinding {
    let guarantee = binding_seed.guarantee;
    let qualified = binding_seed.qualification_status.projects_evidence();
    let high_stakes = seed.privacy_class.is_high_stakes();
    let marketed_on_guarantee = !matches!(
        binding_seed.qualification_status,
        M5OsQualificationStatus::NotApplicable | M5OsQualificationStatus::PlatformOmitted
    );

    M5OsAttentionBinding {
        guarantee,
        qualification_status: binding_seed.qualification_status,
        marketed_on_guarantee,
        projected_envelope_ref: qualified
            .then(|| format!("os-envelope:{}:{}", seed.surface_id, guarantee.as_str())),
        projected_privacy_class: qualified.then_some(seed.privacy_class),
        projected_lock_screen: (qualified && guarantee.requires_privacy_disclosures())
            .then_some(M5OsLockScreenDisclosure::SummaryWithSourceAndScope),
        projected_payload_disclosure: (qualified && guarantee.requires_privacy_disclosures())
            .then_some(M5OsPayloadDisclosure::EnumsAndRefsOnly),
        projected_badge_basis: (qualified && guarantee.requires_badge())
            .then_some(M5OsBadgeBasis::DurableCountClass),
        projected_badge_count_class: (qualified && guarantee.requires_badge())
            .then_some(seed.badge_count_class),
        projected_progress_basis: (qualified && guarantee.requires_progress()).then_some(
            seed.progress_basis
                .unwrap_or(M5OsProgressBasis::NamedDurableJobClass),
        ),
        projected_suppression_parity: (qualified && guarantee.requires_suppression())
            .then_some(M5OsSuppressionParity::ParityAcrossSurfaces),
        projected_suppression_decision: (qualified && guarantee.requires_suppression())
            .then_some(seed.suppression_decision),
        projected_suppression_audit_visible: (qualified && guarantee.requires_suppression())
            .then_some(true),
        projected_reopen_outcome: (qualified
            && (guarantee.requires_reopen_outcome() || high_stakes))
            .then_some(seed.reopen_outcome),
        evidence_freshness: qualified.then_some(M5OsEvidenceFreshness::Fresh),
        evidence_captured_at: qualified.then(|| GENERATED_AT.to_owned()),
        narrowing_reason: binding_seed.narrowing_reason.map(str::to_owned),
        note: binding_seed.note.map(str::to_owned),
    }
}

fn build_envelope_from_seed(seed: &SurfaceSeed) -> M5OsNotificationEnvelope {
    M5OsNotificationEnvelope {
        record_kind: M5_OS_NOTIFICATION_ENVELOPE_RECORD_KIND.to_owned(),
        schema_version: M5_OS_ATTENTION_SCHEMA_VERSION,
        shared_contract_ref: M5_OS_ATTENTION_SHARED_CONTRACT_REF.to_owned(),
        envelope_id: format!("os-envelope:{}", seed.surface_id),
        job_family: seed.job_family,
        job_state_class: seed.job_state_class,
        durable_job_id_ref: seed.durable_job_id_ref.to_owned(),
        canonical_event_id_ref: seed.canonical_event_id_ref.to_owned(),
        privacy_class: seed.privacy_class,
        client_scope: seed.client_scope,
        source_object_label_ref: seed.source_object_label_ref.to_owned(),
        safe_reopen_action_label_ref: seed.safe_reopen_action_label_ref.to_owned(),
        lock_screen_disclosure: M5OsLockScreenDisclosure::SummaryWithSourceAndScope,
        payload_disclosure: M5OsPayloadDisclosure::EnumsAndRefsOnly,
        badge_count_class: seed.badge_count_class,
        badge_basis: M5OsBadgeBasis::DurableCountClass,
        progress_basis: seed.progress_basis,
        suppression_decision: seed.suppression_decision,
        suppression_parity: M5OsSuppressionParity::ParityAcrossSurfaces,
        suppression_audit_visible: true,
        reopen_linkage: M5OsReopenLinkage {
            reopen_outcome: seed.reopen_outcome,
            reopen_anchor_ref: seed.reopen_anchor_ref.to_owned(),
            command_id_ref: seed.command_id_ref.to_owned(),
            must_resolve_through_in_product_surface: true,
            preserves_source: true,
            preserves_freshness: true,
        },
    }
}

fn build_surface_from_seed(seed: &SurfaceSeed) -> M5OsAttentionRow {
    let descriptor = M5OsSurfaceDescriptor {
        surface_id: seed.surface_id.to_owned(),
        job_family: seed.job_family,
        job_state_class: seed.job_state_class,
        durable_job_id_ref: seed.durable_job_id_ref.to_owned(),
        canonical_event_id_ref: seed.canonical_event_id_ref.to_owned(),
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        support_note: seed.support_note.to_owned(),
        privacy_class: seed.privacy_class,
        client_scope: seed.client_scope,
        lifecycle_label: seed.lifecycle_label,
        source_object_label_ref: seed.source_object_label_ref.to_owned(),
        safe_reopen_action_label_ref: seed.safe_reopen_action_label_ref.to_owned(),
        reopen_anchor_ref: seed.reopen_anchor_ref.to_owned(),
        suppression_controls: seed.suppression_controls.to_vec(),
        derived_from_durable_object: true,
        marketed_on_desktop: true,
    };
    let envelope = build_envelope_from_seed(seed);
    let bindings: Vec<M5OsAttentionBinding> = seed
        .bindings
        .iter()
        .map(|binding_seed| build_binding_from_seed(seed, binding_seed))
        .collect();
    build_m5_os_attention_row(descriptor, envelope, bindings)
}

/// Seeded audit builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/ux/m5_os_notifications_and_badges/`.
pub fn seeded_m5_os_attention_report() -> M5OsAttentionReport {
    let surfaces = SURFACE_SEEDS.iter().map(build_surface_from_seed).collect();
    build_m5_os_attention_report(surfaces)
}
