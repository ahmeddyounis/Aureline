//! Frozen M5 security-advisory, emergency-notice, affected-install, and
//! disclosure-link component matrix.
//!
//! This module locks Aureline's user-facing security-advisory component model
//! into one export-safe packet. Every advisory primitive family M5 claims that
//! still drifts too easily into website copy or a generic update banner —
//! security-advisory cards, emergency notices, affected-install panels,
//! disclosure/history blocks, advisory activity rows, and native-notification
//! handoff — is named once here, bound to a canonical shell zone, responsive
//! class, and window class, and constrained by the same required-anatomy,
//! severity, continuity, dismissal, disclosure, and export rules regardless of
//! the surface family that renders it.
//!
//! The shell topology this matrix binds against — the eight canonical shell
//! zones, the compact/standard/expanded responsive classes, the window classes,
//! and the ten claimed M5 surface families — is the one already frozen by
//! [`crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix`];
//! this matrix re-exports that vocabulary rather than minting parallel terms.
//! The advisory record, surface projection, affected-install assessment, and
//! severity vocabulary this matrix aligns to are the ones already frozen in
//! [`schemas/security/advisory_card.schema.json`](../../../../schemas/security/advisory_card.schema.json),
//! [`schemas/security/affected_install_assessment.schema.json`](../../../../schemas/security/affected_install_assessment.schema.json),
//! and [`docs/security/severity_matrix.md`](../../../../docs/security/severity_matrix.md);
//! this matrix references those contracts as source-of-truth rather than
//! inventing a second advisory dialect.
//!
//! What this matrix adds is the stable vocabulary for the advisory *components*
//! themselves: the advisory component families, the required advisory anatomy,
//! the compact severity classes, the action states and required actions, the
//! emergency dismissal states, the local-continuity claims, the delivery
//! profiles and mirror-freshness states, the disclosure fields, the native
//! notification behaviors, the export fields, the projection surfaces, the
//! non-visual accessibility routes, and the mandatory labels every advisory
//! component must be able to show.
//!
//! The matrix is the single source of truth for whether a claimed M5 advisory
//! component may publish an advisory-card, emergency-notice, affected-install,
//! or disclosure claim. Update, marketplace, Help/About, support bundles,
//! native notifications, and mirror/offline drills all consume this packet so
//! one advisory model identifies the affected object, severity, current
//! exposure, fixed version or mitigation, signer/source state, and primary
//! actions without hiding local continuity; one emergency-notice model stays
//! explicit about blast radius, acknowledge/snooze/dismiss rules, and
//! forced-disable scope; and mirror lag, unsigned distribution, or a stale
//! notice state auto-narrows the claim instead of silently staying green. No M5
//! lane invents a generic advisory banner or hides continuity rules off this
//! matrix.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5AdvisoryComponentVocabularySet`] rather than minted per surface. Raw
//! reporter identities, raw exploit payloads, raw signatures, raw hostnames, raw
//! paths, private registry URLs, credentials, and raw evidence bodies stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/security/m5-advisory-component-matrix.schema.json`](../../../../schemas/security/m5-advisory-component-matrix.schema.json)
//! and the contract doc is
//! [`docs/security/m5_advisory_component_matrix_contract.md`](../../../../docs/security/m5_advisory_component_matrix_contract.md).
//! The protected fixture directory is
//! [`fixtures/security/m5-advisory-scenarios/`](../../../../fixtures/security/m5-advisory-scenarios/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_advisory_component_matrix,
    seeded_m5_advisory_component_matrix_affected_install_panel_preview_narrowed,
    seeded_m5_advisory_component_matrix_emergency_notice_beta_narrowed,
    M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID,
};

// The canonical shell topology — zones, responsive classes, window classes,
// consumer surfaces, and the ten claimed M5 surface families — is frozen once,
// in the shell-zone matrix. This matrix reuses it verbatim so no advisory
// component invents a parallel slot, layout class, window class, or surface
// family.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellSurfaceFamily, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AdvisoryComponentMatrixPacket`].
pub const M5_ADVISORY_COMPONENTS_MATRIX_RECORD_KIND: &str =
    "freeze_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix";

/// Schema version for M5 advisory-component-matrix records.
pub const M5_ADVISORY_COMPONENTS_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the advisory-component boundary schema.
pub const M5_ADVISORY_COMPONENTS_SCHEMA_REF: &str =
    "schemas/security/m5-advisory-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ADVISORY_COMPONENTS_DOC_REF: &str =
    "docs/security/m5_advisory_component_matrix_contract.md";

/// Repo-relative path of the frozen shell-zone schema this matrix binds against.
pub const M5_ADVISORY_COMPONENTS_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen advisory-surface contract this matrix aligns
/// its advisory-card, emergency-banner, and disclosure-link vocabulary to.
pub const M5_ADVISORY_COMPONENTS_ADVISORY_CARD_CONTRACT_REF: &str =
    "schemas/security/advisory_card.schema.json";

/// Repo-relative path of the frozen affected-install assessment contract this
/// matrix aligns its affected-install-panel vocabulary to.
pub const M5_ADVISORY_COMPONENTS_AFFECTED_INSTALL_CONTRACT_REF: &str =
    "schemas/security/affected_install_assessment.schema.json";

/// Repo-relative path of the frozen severity matrix this matrix's severity
/// vocabulary projects from.
pub const M5_ADVISORY_COMPONENTS_SEVERITY_MATRIX_REF: &str = "docs/security/severity_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ADVISORY_COMPONENTS_FIXTURE_DIR: &str = "fixtures/security/m5-advisory-scenarios";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ADVISORY_COMPONENTS_ARTIFACT_REF: &str =
    "artifacts/release/m5-advisory-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_ADVISORY_COMPONENTS_CSV_REF: &str = "artifacts/release/m5-advisory-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_ADVISORY_COMPONENTS_REPORT_REF: &str =
    "artifacts/security/m5-advisory-component-matrix.md";

/// One of the six governed advisory component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryComponentFamily {
    /// A security-advisory card naming the affected object, severity, exposure,
    /// fix/mitigation, signer/source state, and primary actions.
    AdvisoryCard,
    /// An emergency notice declaring blast radius, dismissal rules, and
    /// forced-disable scope.
    EmergencyNotice,
    /// An affected-install panel assessing which install lanes are affected and
    /// what still works locally.
    AffectedInstallPanel,
    /// A disclosure/history block carrying copy-safe ids, disclosure timing, and
    /// resolved-versus-active history.
    DisclosureBlock,
    /// A single advisory activity/history row projected into the activity center
    /// and support export.
    AdvisoryActivityRow,
    /// Native OS-notification handoff for an advisory or emergency notice.
    NativeNotificationHandoff,
}

impl M5AdvisoryComponentFamily {
    /// Every governed advisory component family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AdvisoryCard,
        Self::EmergencyNotice,
        Self::AffectedInstallPanel,
        Self::DisclosureBlock,
        Self::AdvisoryActivityRow,
        Self::NativeNotificationHandoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryCard => "advisory_card",
            Self::EmergencyNotice => "emergency_notice",
            Self::AffectedInstallPanel => "affected_install_panel",
            Self::DisclosureBlock => "disclosure_block",
            Self::AdvisoryActivityRow => "advisory_activity_row",
            Self::NativeNotificationHandoff => "native_notification_handoff",
        }
    }

    /// `true` when this family is a security-advisory card and must therefore
    /// declare its required anatomy.
    pub const fn is_advisory_card(self) -> bool {
        matches!(self, Self::AdvisoryCard)
    }

    /// `true` when this family is an emergency notice and must therefore declare
    /// its dismissal states.
    pub const fn is_emergency_notice(self) -> bool {
        matches!(self, Self::EmergencyNotice)
    }

    /// `true` when this family carries user/admin action and must therefore
    /// declare its action states and required actions.
    pub const fn carries_action(self) -> bool {
        matches!(self, Self::AdvisoryCard | Self::EmergencyNotice)
    }

    /// `true` when this family assesses an install and must therefore declare its
    /// continuity claims, delivery profiles, and freshness states.
    pub const fn assesses_install(self) -> bool {
        matches!(self, Self::AffectedInstallPanel)
    }

    /// `true` when this family discloses history and must therefore declare its
    /// disclosure fields.
    pub const fn discloses_history(self) -> bool {
        matches!(self, Self::DisclosureBlock)
    }

    /// `true` when this family is an advisory activity row and must therefore
    /// declare its export fields.
    pub const fn is_activity_row(self) -> bool {
        matches!(self, Self::AdvisoryActivityRow)
    }

    /// `true` when this family hands off to a native notification and must
    /// therefore declare its notification behaviors.
    pub const fn hands_off_native(self) -> bool {
        matches!(self, Self::NativeNotificationHandoff)
    }
}

/// Controlled compact severity class. Aligns field-for-field with the
/// `surface_severity_class` frozen in `schemas/security/advisory_card.schema.json`
/// and the closed severity vocabulary in `docs/security/severity_matrix.md`. An
/// advisory component may not surface a bare "warning" without one of these named
/// severities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisorySeverityClass {
    /// Informational notice; no exposure.
    Informational,
    /// Low severity.
    Low,
    /// Moderate severity (maps from `security_severity.medium`).
    Moderate,
    /// High severity.
    High,
    /// Critical severity.
    Critical,
    /// Operational emergency; stays distinct from critical.
    OperationalEmergency,
}

impl M5AdvisorySeverityClass {
    /// Every severity class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Informational,
        Self::Low,
        Self::Moderate,
        Self::High,
        Self::Critical,
        Self::OperationalEmergency,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Critical => "critical",
            Self::OperationalEmergency => "operational_emergency",
        }
    }
}

/// Required advisory anatomy — the named parts every advisory card must be able
/// to show so it identifies the affected object, its exposure, the fix, the
/// signer/source state, the primary actions, and local continuity rather than
/// reading as marketing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryAnatomyField {
    /// The affected object / subject the advisory names.
    AffectedObject,
    /// The severity of the advisory.
    Severity,
    /// The current exposure / whether the install is affected right now.
    CurrentExposure,
    /// The fixed version or the mitigation available.
    FixedVersionOrMitigation,
    /// The signer / source continuity state of the distribution.
    SignerSourceState,
    /// The primary actions the user or admin can take.
    PrimaryActions,
    /// What still works locally / local-continuity claim.
    LocalContinuity,
}

impl M5AdvisoryAnatomyField {
    /// Every anatomy field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AffectedObject,
        Self::Severity,
        Self::CurrentExposure,
        Self::FixedVersionOrMitigation,
        Self::SignerSourceState,
        Self::PrimaryActions,
        Self::LocalContinuity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AffectedObject => "affected_object",
            Self::Severity => "severity",
            Self::CurrentExposure => "current_exposure",
            Self::FixedVersionOrMitigation => "fixed_version_or_mitigation",
            Self::SignerSourceState => "signer_source_state",
            Self::PrimaryActions => "primary_actions",
            Self::LocalContinuity => "local_continuity",
        }
    }
}

/// Controlled action state. Aligns with the `action_state_class` frozen in the
/// advisory-surface contract; distinguishes an informational notice from a
/// blocking or immediate-remediation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryActionState {
    /// Informational; no action required.
    Informational,
    /// A review is recommended.
    ReviewRecommended,
    /// An action is required.
    ActionRequired,
    /// The advisory blocks continued use until acted on.
    Blocking,
    /// Immediate remediation is required.
    ImmediateRemediation,
    /// Mitigation is complete; state is retained as history.
    MitigationComplete,
}

impl M5AdvisoryActionState {
    /// Every action state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Informational,
        Self::ReviewRecommended,
        Self::ActionRequired,
        Self::Blocking,
        Self::ImmediateRemediation,
        Self::MitigationComplete,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::ReviewRecommended => "review_recommended",
            Self::ActionRequired => "action_required",
            Self::Blocking => "blocking",
            Self::ImmediateRemediation => "immediate_remediation",
            Self::MitigationComplete => "mitigation_complete",
        }
    }
}

/// Controlled primary next action. Aligns with the `required_action_class` frozen
/// in the advisory-surface contract so an advisory names one canonical next
/// action rather than free-text guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryRequiredAction {
    /// No action.
    None,
    /// Review the notice.
    ReviewNotice,
    /// Update to the fixed version.
    UpdateToFixedVersion,
    /// Roll back or repin.
    RollbackOrRepin,
    /// Disable or remove the affected object.
    DisableOrRemove,
    /// Import a signed snapshot.
    ImportSignedSnapshot,
    /// Rotate the trust root.
    RotateTrustRoot,
    /// Export a support packet.
    ExportSupportPacket,
    /// Contact an administrator.
    ContactAdmin,
    /// Wait for a superseding action.
    WaitForSupersedingAction,
}

impl M5AdvisoryRequiredAction {
    /// Every required action, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::None,
        Self::ReviewNotice,
        Self::UpdateToFixedVersion,
        Self::RollbackOrRepin,
        Self::DisableOrRemove,
        Self::ImportSignedSnapshot,
        Self::RotateTrustRoot,
        Self::ExportSupportPacket,
        Self::ContactAdmin,
        Self::WaitForSupersedingAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ReviewNotice => "review_notice",
            Self::UpdateToFixedVersion => "update_to_fixed_version",
            Self::RollbackOrRepin => "rollback_or_repin",
            Self::DisableOrRemove => "disable_or_remove",
            Self::ImportSignedSnapshot => "import_signed_snapshot",
            Self::RotateTrustRoot => "rotate_trust_root",
            Self::ExportSupportPacket => "export_support_packet",
            Self::ContactAdmin => "contact_admin",
            Self::WaitForSupersedingAction => "wait_for_superseding_action",
        }
    }
}

/// Controlled dismissal state for an emergency notice. Aligns with the
/// `acknowledgement_state_class` frozen in the advisory-surface contract so
/// acknowledge/snooze/dismiss rules are explicit and a forced-disable notice
/// cannot be dismissed away.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryDismissalState {
    /// The notice is not acknowledgeable / not dismissable.
    NotAcknowledgeable,
    /// The notice is unacknowledged.
    Unacknowledged,
    /// The notice was acknowledged (acknowledgement is not mitigation).
    Acknowledged,
    /// The notice is snoozed until a scheduled review.
    SnoozedUntilReview,
    /// The notice is blocked until the exposure is remediated.
    BlockedUntilRemediated,
}

impl M5AdvisoryDismissalState {
    /// Every dismissal state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotAcknowledgeable,
        Self::Unacknowledged,
        Self::Acknowledged,
        Self::SnoozedUntilReview,
        Self::BlockedUntilRemediated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAcknowledgeable => "not_acknowledgeable",
            Self::Unacknowledged => "unacknowledged",
            Self::Acknowledged => "acknowledged",
            Self::SnoozedUntilReview => "snoozed_until_review",
            Self::BlockedUntilRemediated => "blocked_until_remediated",
        }
    }
}

/// Controlled local-continuity claim — what still works locally when the affected
/// install-profile is disabled or when the mirror is behind, so an advisory never
/// hides safe local continuity behind a blocking banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryContinuityClaim {
    /// Local use is unaffected.
    LocalUseUnaffected,
    /// Local use continues in a degraded mode.
    DegradedLocalMode,
    /// Continuity requires disabling the affected profile.
    RequiresDisablingAffectedProfile,
    /// The offline mirror is behind; the lag is disclosed.
    OfflineMirrorLagDisclosed,
    /// There is no safe local continuity; the exposure is active.
    NoSafeLocalContinuity,
    /// Continuity is pending the fix or a superseding action.
    ContinuityPendingFix,
}

impl M5AdvisoryContinuityClaim {
    /// Every continuity claim, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalUseUnaffected,
        Self::DegradedLocalMode,
        Self::RequiresDisablingAffectedProfile,
        Self::OfflineMirrorLagDisclosed,
        Self::NoSafeLocalContinuity,
        Self::ContinuityPendingFix,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUseUnaffected => "local_use_unaffected",
            Self::DegradedLocalMode => "degraded_local_mode",
            Self::RequiresDisablingAffectedProfile => "requires_disabling_affected_profile",
            Self::OfflineMirrorLagDisclosed => "offline_mirror_lag_disclosed",
            Self::NoSafeLocalContinuity => "no_safe_local_continuity",
            Self::ContinuityPendingFix => "continuity_pending_fix",
        }
    }
}

/// Controlled delivery profile for a notice. Aligns with the
/// `notice_profile_class` frozen in the advisory-surface contract so
/// local-only, managed, offline-mirror, and manual-import truth is preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryDeliveryProfile {
    /// Local-only cached-metadata projection.
    LocalOnly,
    /// Managed / administrator-authoritative delivery.
    Managed,
    /// Approved offline-mirror delivery.
    OfflineMirror,
    /// Manual-import bundle delivery.
    ManualImport,
}

impl M5AdvisoryDeliveryProfile {
    /// Every delivery profile, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::Managed,
        Self::OfflineMirror,
        Self::ManualImport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Managed => "managed",
            Self::OfflineMirror => "offline_mirror",
            Self::ManualImport => "manual_import",
        }
    }
}

/// Controlled mirror/distribution freshness state. Aligns with the
/// `mirror_freshness_class` frozen in the affected-install and severity
/// contracts so mirror lag auto-narrows a claim instead of staying green.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryFreshnessState {
    /// The distribution is up to date.
    UpToDate,
    /// Stale but within the grace window.
    StaleWithinGrace,
    /// Stale past the grace window.
    StalePastGrace,
    /// An offline snapshot has expired.
    OfflineExpired,
    /// Freshness is unknown.
    Unknown,
}

impl M5AdvisoryFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UpToDate,
        Self::StaleWithinGrace,
        Self::StalePastGrace,
        Self::OfflineExpired,
        Self::Unknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpToDate => "up_to_date",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StalePastGrace => "stale_past_grace",
            Self::OfflineExpired => "offline_expired",
            Self::Unknown => "unknown",
        }
    }
}

/// Controlled disclosure field a disclosure/history block must be able to carry,
/// so copy-safe ids, disclosure timing, visibility posture, and resolved-versus
/// -active history are never flattened into a link to an external page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryDisclosureField {
    /// The stable Aureline advisory id.
    AurelineAdvisoryId,
    /// The CVE alias id.
    CveAlias,
    /// The GHSA alias id.
    GhsaAlias,
    /// The disclosure timing (private / public / embargo).
    DisclosureTiming,
    /// The visibility posture.
    VisibilityPosture,
    /// The resolved-versus-active history state.
    HistoryState,
    /// A copy/open disclosure link with explicit offline/mirror posture.
    ExternalDisclosureLink,
}

impl M5AdvisoryDisclosureField {
    /// Every disclosure field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AurelineAdvisoryId,
        Self::CveAlias,
        Self::GhsaAlias,
        Self::DisclosureTiming,
        Self::VisibilityPosture,
        Self::HistoryState,
        Self::ExternalDisclosureLink,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AurelineAdvisoryId => "aureline_advisory_id",
            Self::CveAlias => "cve_alias",
            Self::GhsaAlias => "ghsa_alias",
            Self::DisclosureTiming => "disclosure_timing",
            Self::VisibilityPosture => "visibility_posture",
            Self::HistoryState => "history_state",
            Self::ExternalDisclosureLink => "external_disclosure_link",
        }
    }
}

/// Controlled native-notification behavior — how an advisory or emergency notice
/// hands off to the operating-system notification surface without leaking a
/// sensitive body and without letting an emergency be silenced by quiet hours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryNotificationBehavior {
    /// The OS notification carries a compact summary only.
    OsNotificationSummary,
    /// Clicking the notification opens the in-product advisory.
    ClickThroughToAdvisory,
    /// The notification respects quiet hours for non-emergency severities.
    RespectsQuietHours,
    /// The OS payload carries no sensitive body.
    NoSensitiveBodyInPayload,
    /// An emergency notice bypasses quiet hours.
    EmergencyBypassesQuietHours,
    /// Dismissing the OS notification syncs to the in-app dismissal state.
    DismissalSyncsToInApp,
}

impl M5AdvisoryNotificationBehavior {
    /// Every notification behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OsNotificationSummary,
        Self::ClickThroughToAdvisory,
        Self::RespectsQuietHours,
        Self::NoSensitiveBodyInPayload,
        Self::EmergencyBypassesQuietHours,
        Self::DismissalSyncsToInApp,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsNotificationSummary => "os_notification_summary",
            Self::ClickThroughToAdvisory => "click_through_to_advisory",
            Self::RespectsQuietHours => "respects_quiet_hours",
            Self::NoSensitiveBodyInPayload => "no_sensitive_body_in_payload",
            Self::EmergencyBypassesQuietHours => "emergency_bypasses_quiet_hours",
            Self::DismissalSyncsToInApp => "dismissal_syncs_to_in_app",
        }
    }
}

/// Controlled export field an advisory activity row promises to carry into the
/// support export, so a support bundle reconstructs advisory truth without a
/// screenshot and never silently drops a truth-bearing column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryExportField {
    /// The copy-safe advisory id.
    AdvisoryId,
    /// The severity class.
    Severity,
    /// The action state.
    ActionState,
    /// The affected surface class.
    AffectedSurface,
    /// The current mitigation state.
    MitigationState,
    /// The delivery profile.
    DeliveryProfile,
    /// The distribution freshness state.
    FreshnessState,
    /// The local-continuity note.
    ContinuityNote,
    /// The disclosure visibility posture.
    DisclosureVisibility,
    /// The resolved-versus-active history state.
    HistoryState,
}

impl M5AdvisoryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::AdvisoryId,
        Self::Severity,
        Self::ActionState,
        Self::AffectedSurface,
        Self::MitigationState,
        Self::DeliveryProfile,
        Self::FreshnessState,
        Self::ContinuityNote,
        Self::DisclosureVisibility,
        Self::HistoryState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryId => "advisory_id",
            Self::Severity => "severity",
            Self::ActionState => "action_state",
            Self::AffectedSurface => "affected_surface",
            Self::MitigationState => "mitigation_state",
            Self::DeliveryProfile => "delivery_profile",
            Self::FreshnessState => "freshness_state",
            Self::ContinuityNote => "continuity_note",
            Self::DisclosureVisibility => "disclosure_visibility",
            Self::HistoryState => "history_state",
        }
    }
}

/// Projection surface an advisory component's truth reaches, so update,
/// marketplace, Help/About, support, native notifications, and mirror/offline
/// drills all describe the same advisory truth from one model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryProjectionSurface {
    /// The update center.
    UpdateCenter,
    /// The marketplace / extension surface.
    Marketplace,
    /// The Help / About surface.
    HelpAbout,
    /// A support bundle export.
    SupportBundle,
    /// A native OS notification.
    NativeNotification,
    /// A mirror / offline continuity drill.
    MirrorOfflineDrill,
    /// The activity center / history.
    ActivityCenter,
    /// A release-evidence packet.
    ReleasePacket,
}

impl M5AdvisoryProjectionSurface {
    /// Every projection surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::UpdateCenter,
        Self::Marketplace,
        Self::HelpAbout,
        Self::SupportBundle,
        Self::NativeNotification,
        Self::MirrorOfflineDrill,
        Self::ActivityCenter,
        Self::ReleasePacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::Marketplace => "marketplace",
            Self::HelpAbout => "help_about",
            Self::SupportBundle => "support_bundle",
            Self::NativeNotification => "native_notification",
            Self::MirrorOfflineDrill => "mirror_offline_drill",
            Self::ActivityCenter => "activity_center",
            Self::ReleasePacket => "release_packet",
        }
    }
}

/// Non-visual / accessibility route every advisory component must offer so no
/// advisory truth is hover-only, pointer-only, or color-encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5AdvisoryAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed advisory component must be able to show. The first
/// three are hard requirements on every component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryRequiredLabel {
    /// The advisory's stable identity / what it represents.
    Identity,
    /// The advisory's severity.
    Severity,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The signer / source provenance of the distribution.
    Provenance,
    /// The primary action the component offers.
    PrimaryAction,
    /// The local-continuity note.
    ContinuityNote,
}

impl M5AdvisoryRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::Severity,
        Self::KeyboardRoute,
        Self::Provenance,
        Self::PrimaryAction,
        Self::ContinuityNote,
    ];

    /// The three labels every claimed advisory component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::Severity, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Severity => "severity",
            Self::KeyboardRoute => "keyboard_route",
            Self::Provenance => "provenance",
            Self::PrimaryAction => "primary_action",
            Self::ContinuityNote => "continuity_note",
        }
    }
}

/// Qualification class for an M5 advisory-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5AdvisoryQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows an advisory component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryDowngradeTrigger {
    /// The affected scope was hidden.
    AffectedScopeHidden,
    /// Exposure was hidden behind a generic banner.
    ExposureHiddenBehindGenericBanner,
    /// The fixed version or mitigation was missing.
    FixedVersionOrMitigationMissing,
    /// The signer / source continuity state was hidden.
    SignerSourceStateHidden,
    /// Local continuity was hidden.
    LocalContinuityHidden,
    /// A dismissal rule was violated (emergency dismissed without acknowledgement).
    DismissalRuleViolated,
    /// Forced-disable scope was hidden.
    ForcedDisableScopeHidden,
    /// Mirror lag was undisclosed.
    MirrorLagUndisclosed,
    /// Unsigned distribution was undisclosed.
    UnsignedDistributionUndisclosed,
    /// A stale notice state stayed silently green.
    StaleNoticeStateSilent,
    /// The advisory was reachable only via an external disclosure page.
    ExternalDisclosureOnly,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5AdvisoryDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::AffectedScopeHidden,
        Self::ExposureHiddenBehindGenericBanner,
        Self::FixedVersionOrMitigationMissing,
        Self::SignerSourceStateHidden,
        Self::LocalContinuityHidden,
        Self::DismissalRuleViolated,
        Self::ForcedDisableScopeHidden,
        Self::MirrorLagUndisclosed,
        Self::UnsignedDistributionUndisclosed,
        Self::StaleNoticeStateSilent,
        Self::ExternalDisclosureOnly,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AffectedScopeHidden => "affected_scope_hidden",
            Self::ExposureHiddenBehindGenericBanner => "exposure_hidden_behind_generic_banner",
            Self::FixedVersionOrMitigationMissing => "fixed_version_or_mitigation_missing",
            Self::SignerSourceStateHidden => "signer_source_state_hidden",
            Self::LocalContinuityHidden => "local_continuity_hidden",
            Self::DismissalRuleViolated => "dismissal_rule_violated",
            Self::ForcedDisableScopeHidden => "forced_disable_scope_hidden",
            Self::MirrorLagUndisclosed => "mirror_lag_undisclosed",
            Self::UnsignedDistributionUndisclosed => "unsigned_distribution_undisclosed",
            Self::StaleNoticeStateSilent => "stale_notice_state_silent",
            Self::ExternalDisclosureOnly => "external_disclosure_only",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed advisory component family bound to its
/// shell zone, layout classes, and the advisory truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryComponentRow {
    /// Governed component family.
    pub component_family: M5AdvisoryComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5AdvisoryQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this component attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this component must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this component keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5ShellSurfaceFamily>,
    /// Mandatory labels this component must be able to show (must include the
    /// three [`M5AdvisoryRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5AdvisoryRequiredLabel>,
    /// Severity classes this component renders (every advisory component).
    pub severity_classes: Vec<M5AdvisorySeverityClass>,
    /// Projection surfaces this component's truth reaches (every component).
    pub projection_surfaces: Vec<M5AdvisoryProjectionSurface>,
    /// Required advisory anatomy this component shows (advisory card only).
    pub anatomy_fields: Vec<M5AdvisoryAnatomyField>,
    /// Action states this component projects (action-bearing families only).
    pub action_states: Vec<M5AdvisoryActionState>,
    /// Primary next actions this component offers (action-bearing families only).
    pub required_actions: Vec<M5AdvisoryRequiredAction>,
    /// Dismissal states this component honours (emergency notice only).
    pub dismissal_states: Vec<M5AdvisoryDismissalState>,
    /// Local-continuity claims this component makes (affected-install only).
    pub continuity_claims: Vec<M5AdvisoryContinuityClaim>,
    /// Delivery profiles this component distinguishes (affected-install only).
    pub delivery_profiles: Vec<M5AdvisoryDeliveryProfile>,
    /// Freshness states this component preserves (affected-install only).
    pub freshness_states: Vec<M5AdvisoryFreshnessState>,
    /// Disclosure fields this component carries (disclosure block only).
    pub disclosure_fields: Vec<M5AdvisoryDisclosureField>,
    /// Native notification behaviors this component honours (native handoff only).
    pub notification_behaviors: Vec<M5AdvisoryNotificationBehavior>,
    /// Export fields this component promises (activity row only).
    pub export_fields: Vec<M5AdvisoryExportField>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5AdvisoryAccessibilityRoute>,
    /// Shell subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5AdvisoryDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never hides the affected scope. MUST be
    /// `false`.
    pub hides_affected_scope: bool,
    /// Hard invariant: this component never hides local continuity. MUST be
    /// `false`.
    pub hides_local_continuity: bool,
    /// Hard invariant: this component never invents generic advisory language.
    /// MUST be `false`.
    pub invents_generic_advisory_language: bool,
    /// Hard invariant: this component never stays silent on a stale or unsigned
    /// distribution state. MUST be `false`.
    pub stays_silent_on_stale_or_unsigned: bool,
}

impl M5AdvisoryComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5AdvisoryRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5AdvisoryRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_affected_scope
            && !self.hides_local_continuity
            && !self.invents_generic_advisory_language
            && !self.stays_silent_on_stale_or_unsigned
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Severity-class tokens.
    pub severity_classes: Vec<String>,
    /// Advisory-anatomy tokens.
    pub anatomy_fields: Vec<String>,
    /// Action-state tokens.
    pub action_states: Vec<String>,
    /// Required-action tokens.
    pub required_actions: Vec<String>,
    /// Dismissal-state tokens.
    pub dismissal_states: Vec<String>,
    /// Continuity-claim tokens.
    pub continuity_claims: Vec<String>,
    /// Delivery-profile tokens.
    pub delivery_profiles: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Disclosure-field tokens.
    pub disclosure_fields: Vec<String>,
    /// Notification-behavior tokens.
    pub notification_behaviors: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Projection-surface tokens.
    pub projection_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5AdvisoryComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5AdvisoryComponentFamily::ALL, |v| v.as_str()),
            severity_classes: tokens(&M5AdvisorySeverityClass::ALL, |v| v.as_str()),
            anatomy_fields: tokens(&M5AdvisoryAnatomyField::ALL, |v| v.as_str()),
            action_states: tokens(&M5AdvisoryActionState::ALL, |v| v.as_str()),
            required_actions: tokens(&M5AdvisoryRequiredAction::ALL, |v| v.as_str()),
            dismissal_states: tokens(&M5AdvisoryDismissalState::ALL, |v| v.as_str()),
            continuity_claims: tokens(&M5AdvisoryContinuityClaim::ALL, |v| v.as_str()),
            delivery_profiles: tokens(&M5AdvisoryDeliveryProfile::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5AdvisoryFreshnessState::ALL, |v| v.as_str()),
            disclosure_fields: tokens(&M5AdvisoryDisclosureField::ALL, |v| v.as_str()),
            notification_behaviors: tokens(&M5AdvisoryNotificationBehavior::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AdvisoryExportField::ALL, |v| v.as_str()),
            projection_surfaces: tokens(&M5AdvisoryProjectionSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5AdvisoryAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5AdvisoryRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryComponentGovernanceReview {
    /// The advisory card names its required anatomy.
    pub advisory_card_names_required_anatomy: bool,
    /// The severity vocabulary is a closed set aligned with the severity matrix.
    pub severity_vocabulary_is_closed: bool,
    /// The emergency notice declares blast radius and dismissal rules.
    pub emergency_notice_declares_blast_radius_and_dismissal: bool,
    /// The affected-install panel preserves local continuity.
    pub affected_install_panel_preserves_local_continuity: bool,
    /// The disclosure block keeps copy-safe ids and resolved/active history.
    pub disclosure_block_keeps_ids_and_history: bool,
    /// The activity row is reconstructable from the support export.
    pub activity_row_reconstructable_from_support_export: bool,
    /// The native notification carries no sensitive body.
    pub native_notification_carries_no_sensitive_body: bool,
    /// Mirror lag or unsigned distribution auto-narrows the claim.
    pub mirror_lag_or_unsigned_auto_narrows: bool,
    /// No component invents generic advisory language.
    pub no_component_invents_generic_advisory_language: bool,
    /// Every component is bound to a canonical shell zone.
    pub every_component_bound_to_shell_zone: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel advisory-component vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block: names how advisory truth projects into each
/// downstream surface from one matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryComponentConsumerProjection {
    /// The update center reads the advisory matrix.
    pub update_center_reads_advisory_matrix: bool,
    /// The marketplace reads the advisory matrix.
    pub marketplace_reads_advisory_matrix: bool,
    /// Help / About reads the advisory matrix.
    pub help_about_reads_advisory_matrix: bool,
    /// Support bundles read a single canonical advisory source.
    pub support_bundle_reads_single_source: bool,
    /// Native notifications read a single canonical advisory source.
    pub native_notifications_read_single_source: bool,
    /// Mirror / offline drills read a single canonical advisory source.
    pub mirror_offline_drills_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the advisory-component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryComponentReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting advisory-component audit for the lane.
    pub advisory_component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AdvisoryComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AdvisoryComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5AdvisoryComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AdvisoryComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AdvisoryComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AdvisoryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AdvisoryComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AdvisoryComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 advisory-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryComponentMatrixPacket {
    /// Record kind; must equal [`M5_ADVISORY_COMPONENTS_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ADVISORY_COMPONENTS_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5AdvisoryComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AdvisoryComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AdvisoryComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AdvisoryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AdvisoryComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AdvisoryComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AdvisoryComponentMatrixPacket {
    /// Builds an M5 advisory-component matrix packet from stable-lane input.
    pub fn new(input: M5AdvisoryComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_ADVISORY_COMPONENTS_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_ADVISORY_COMPONENTS_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 advisory-component matrix invariants.
    pub fn validate(&self) -> Vec<M5AdvisoryComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ADVISORY_COMPONENTS_MATRIX_RECORD_KIND {
            violations.push(M5AdvisoryComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ADVISORY_COMPONENTS_MATRIX_SCHEMA_VERSION {
            violations.push(M5AdvisoryComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AdvisoryComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 advisory component matrix packet serializes"),
        ) {
            violations.push(M5AdvisoryComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 advisory component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed
    /// component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,shell_zone_slot,severity_classes,responsive_classes,window_classes,projection_surfaces,required_labels,consumer_surfaces\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.severity_classes, |v| v.as_str()),
                join_tokens(&row.responsive_classes, |v| v.as_str()),
                join_tokens(&row.window_classes, |v| v.as_str()),
                join_tokens(&row.projection_surfaces, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Security-Advisory, Emergency-Notice, Affected-Install, and Disclosure-Link Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Severity classes: {}\n",
            self.vocabulary_set.severity_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Projection surfaces: {}\n",
            self.vocabulary_set.projection_surfaces.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 advisory-component matrix
/// export.
#[derive(Debug)]
pub enum M5AdvisoryComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AdvisoryComponentMatrixViolation>),
}

impl fmt::Display for M5AdvisoryComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 advisory component matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 advisory component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AdvisoryComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5AdvisoryComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AdvisoryComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component declares no severity classes.
    SeverityClassMissing,
    /// A component declares no projection surfaces.
    ProjectionSurfaceMissing,
    /// An advisory card declares no required anatomy.
    AnatomyFieldMissing,
    /// An action-bearing component declares no action states.
    ActionStateMissing,
    /// An action-bearing component declares no required actions.
    RequiredActionMissing,
    /// An emergency notice declares no dismissal states.
    DismissalStateMissing,
    /// An affected-install panel declares no continuity claims.
    ContinuityClaimMissing,
    /// An affected-install panel declares no delivery profiles.
    DeliveryProfileMissing,
    /// An affected-install panel declares no freshness states.
    FreshnessStateMissing,
    /// A disclosure block declares no disclosure fields.
    DisclosureFieldMissing,
    /// A native-notification handoff declares no notification behaviors.
    NotificationBehaviorMissing,
    /// An activity row declares no export fields.
    ExportFieldMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no responsive classes.
    ResponsiveClassMissing,
    /// A component declares no window classes.
    WindowClassMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (hidden scope, hidden continuity,
    /// generic advisory language, or silence on stale/unsigned state).
    AdvisoryInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AdvisoryComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::SeverityClassMissing => "severity_class_missing",
            Self::ProjectionSurfaceMissing => "projection_surface_missing",
            Self::AnatomyFieldMissing => "anatomy_field_missing",
            Self::ActionStateMissing => "action_state_missing",
            Self::RequiredActionMissing => "required_action_missing",
            Self::DismissalStateMissing => "dismissal_state_missing",
            Self::ContinuityClaimMissing => "continuity_claim_missing",
            Self::DeliveryProfileMissing => "delivery_profile_missing",
            Self::FreshnessStateMissing => "freshness_state_missing",
            Self::DisclosureFieldMissing => "disclosure_field_missing",
            Self::NotificationBehaviorMissing => "notification_behavior_missing",
            Self::ExportFieldMissing => "export_field_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ResponsiveClassMissing => "responsive_class_missing",
            Self::WindowClassMissing => "window_class_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::AdvisoryInvariantViolated => "advisory_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 advisory-component matrix export.
pub fn current_stable_m5_advisory_component_matrix_export(
) -> Result<M5AdvisoryComponentMatrixPacket, M5AdvisoryComponentMatrixArtifactError> {
    let packet: M5AdvisoryComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-advisory-proof/support_export.json"
    )))
    .map_err(M5AdvisoryComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AdvisoryComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AdvisoryComponentMatrixPacket,
    violations: &mut Vec<M5AdvisoryComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ADVISORY_COMPONENTS_SCHEMA_REF,
        M5_ADVISORY_COMPONENTS_DOC_REF,
        M5_ADVISORY_COMPONENTS_SHELL_ZONE_REF,
        M5_ADVISORY_COMPONENTS_ADVISORY_CARD_CONTRACT_REF,
        M5_ADVISORY_COMPONENTS_AFFECTED_INSTALL_CONTRACT_REF,
        M5_ADVISORY_COMPONENTS_SEVERITY_MATRIX_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AdvisoryComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AdvisoryComponentMatrixPacket,
    violations: &mut Vec<M5AdvisoryComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AdvisoryComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5AdvisoryComponentMatrixPacket,
    violations: &mut Vec<M5AdvisoryComponentMatrixViolation>,
) {
    let present: BTreeSet<M5AdvisoryComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5AdvisoryComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5AdvisoryComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5AdvisoryComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5AdvisoryComponentMatrixViolation::MandatoryLabelMissing);
        }
        if row.severity_classes.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::SeverityClassMissing);
        }
        if row.projection_surfaces.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::ProjectionSurfaceMissing);
        }
        if family.is_advisory_card() && row.anatomy_fields.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::AnatomyFieldMissing);
        }
        if family.carries_action() && row.action_states.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::ActionStateMissing);
        }
        if family.carries_action() && row.required_actions.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::RequiredActionMissing);
        }
        if family.is_emergency_notice() && row.dismissal_states.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::DismissalStateMissing);
        }
        if family.assesses_install() && row.continuity_claims.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::ContinuityClaimMissing);
        }
        if family.assesses_install() && row.delivery_profiles.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::DeliveryProfileMissing);
        }
        if family.assesses_install() && row.freshness_states.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::FreshnessStateMissing);
        }
        if family.discloses_history() && row.disclosure_fields.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::DisclosureFieldMissing);
        }
        if family.hands_off_native() && row.notification_behaviors.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::NotificationBehaviorMissing);
        }
        if family.is_activity_row() && row.export_fields.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::ExportFieldMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.responsive_classes.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::ResponsiveClassMissing);
        }
        if row.window_classes.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::WindowClassMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AdvisoryComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AdvisoryComponentMatrixViolation::AdvisoryInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5AdvisoryComponentMatrixPacket,
    violations: &mut Vec<M5AdvisoryComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.advisory_card_names_required_anatomy,
        review.severity_vocabulary_is_closed,
        review.emergency_notice_declares_blast_radius_and_dismissal,
        review.affected_install_panel_preserves_local_continuity,
        review.disclosure_block_keeps_ids_and_history,
        review.activity_row_reconstructable_from_support_export,
        review.native_notification_carries_no_sensitive_body,
        review.mirror_lag_or_unsigned_auto_narrows,
        review.no_component_invents_generic_advisory_language,
        review.every_component_bound_to_shell_zone,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5AdvisoryComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AdvisoryComponentMatrixPacket,
    violations: &mut Vec<M5AdvisoryComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.update_center_reads_advisory_matrix,
        projection.marketplace_reads_advisory_matrix,
        projection.help_about_reads_advisory_matrix,
        projection.support_bundle_reads_single_source,
        projection.native_notifications_read_single_source,
        projection.mirror_offline_drills_read_single_source,
    ] {
        if !ok {
            violations.push(M5AdvisoryComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AdvisoryComponentMatrixPacket,
    violations: &mut Vec<M5AdvisoryComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AdvisoryComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AdvisoryComponentMatrixPacket,
    violations: &mut Vec<M5AdvisoryComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.advisory_component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AdvisoryComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
