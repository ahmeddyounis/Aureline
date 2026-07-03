//! One reusable M5 affected-install assessment panel primitive: current build /
//! channel / install-mode identity, impacted components, current exposure, mitigation
//! status, mirror freshness, and the rollback / repin / help actions rendered with the
//! same model whenever a user, admin, or support needs one precise answer to "am I
//! affected?".
//!
//! Aureline's frozen advisory-component matrix
//! ([`crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`])
//! names the affected-install panel as a governed component family and freezes the
//! controlled severity classes, action states, required actions, continuity claims,
//! delivery profiles, mirror-freshness states, export fields, and accessibility routes
//! an advisory component may use. This module *implements* that affected-install
//! contract as one reusable assessment panel so a signed per-user install, a
//! per-machine install, a portable archive, an offline bundle, a managed deployment, or
//! a side-by-side preview reads the same everywhere it surfaces — instead of collapsing
//! into a generic "an update is available" banner that hides whether *this* build is
//! actually affected, whether the mirror metadata is fresh, and where the rollback /
//! repin / help action lives.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_affected_install`] — that takes one advisory affecting one
//!    install-profile lane (its copy-safe advisory id, severity, affected object, exact
//!    build identity, impacted components, install state, mirror freshness, delivery
//!    profile, fixed build or mitigation, signer / source state, action state, primary
//!    and help actions, and local-continuity claim) and produces one
//!    [`M5ResolvedAffectedInstall`] that resolves the advisory against the local install
//!    graph — no external website lookup — derives the current-exposure state from the
//!    install state and the "am I affected?" assessment verdict from that exposure and
//!    the mirror freshness (so a stale or expired mirror auto-narrows a clean answer
//!    instead of silently staying green), keeps the mirror freshness and install mode
//!    visible in the same surface, keeps the rollback / repin / help actions attached to
//!    the panel, keeps the panel visible, projects the same assessment truth into every
//!    claimed channel, and emits a copy-safe, export-safe summary. The resolver never
//!    hides the exposure, freshness, or next action behind a detail drawer and never
//!    drops the copy-safe advisory id.
//! 2. A parity matrix — [`M5AffectedInstallPanelPacket`] — that binds one row per
//!    claimed install-profile lane to the shared panel anatomy, the same severity
//!    vocabulary, the same channels, the same export fields, and the same accessibility
//!    routes, so update, Help/About, support-bundle, and admin-report surfaces render
//!    the same affected-install assessment from one shared model.
//!
//! The severity classes ([`M5AdvisorySeverityClass`]), action states
//! ([`M5AdvisoryActionState`]), required actions ([`M5AdvisoryRequiredAction`]),
//! continuity claims ([`M5AdvisoryContinuityClaim`]), delivery profiles
//! ([`M5AdvisoryDeliveryProfile`]), mirror-freshness states
//! ([`M5AdvisoryFreshnessState`]), export fields ([`M5AdvisoryExportField`]),
//! accessibility routes ([`M5AdvisoryAccessibilityRoute`]), qualification classes
//! ([`M5AdvisoryQualificationClass`]), and downgrade triggers
//! ([`M5AdvisoryDowngradeTrigger`]) are reused verbatim from the frozen advisory matrix;
//! the install state ([`M5AdvisoryInstallState`]) and derived exposure state
//! ([`M5AdvisoryExposureState`]) are reused from the advisory-row primitive; the shell
//! topology — zones, responsive classes, window classes, and consumer surfaces — is
//! reused from the frozen shell-zone matrix. This module mints new vocabulary only for
//! what the frozen matrix left implicit about the panel itself: its install-profile
//! lanes, its panel anatomy, its channels, its focus behaviors, and the derived
//! assessment verdict. No M5 surface invents a second install-assessment grammar.
//!
//! Raw hostnames, raw absolute paths, raw exploit payloads, raw signatures, private
//! registry URLs, credentials, and raw build bodies stay outside the support boundary;
//! opaque, export-safe reprs are the only material carried.
//!
//! The boundary schema is
//! [`schemas/security/m5-affected-install-panel.schema.json`](../../../../schemas/security/m5-affected-install-panel.schema.json)
//! and the contract doc is
//! [`docs/security/m5_affected_install_panel_primitive_contract.md`](../../../../docs/security/m5_affected_install_panel_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/security/m5-affected-install-panel-primitive/`](../../../../fixtures/security/m5-affected-install-panel-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_affected_install_panel_primitive_managed_deployed_beta_narrowed,
    seeded_m5_affected_install_panel_primitive_offline_bundle_preview_narrowed,
    seeded_m5_affected_install_panel_primitive_packet,
    M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_PACKET_ID,
};

// The severity classes, action states, required actions, continuity claims, delivery
// profiles, mirror-freshness states, export fields, accessibility routes,
// qualification classes, and downgrade triggers are frozen once, in the
// advisory-component matrix. This primitive reuses them verbatim so it never invents a
// parallel severity vocabulary or a second install-assessment grammar.
pub use crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix::{
    M5AdvisoryAccessibilityRoute, M5AdvisoryActionState, M5AdvisoryContinuityClaim,
    M5AdvisoryDeliveryProfile, M5AdvisoryDowngradeTrigger, M5AdvisoryExportField,
    M5AdvisoryFreshnessState, M5AdvisoryQualificationClass, M5AdvisoryRequiredAction,
    M5AdvisorySeverityClass,
};

// The install state and derived exposure state are minted once, in the advisory-row
// primitive; this panel reuses them so the "am I affected?" assessment reads from a
// single install-graph vocabulary.
pub use crate::implement_the_m5_advisory_card_and_row_primitive::{
    M5AdvisoryExposureState, M5AdvisoryInstallState,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AffectedInstallPanelPacket`].
pub const M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_affected_install_assessment_build_channel_install_mode_impacted_components_mirror_freshness_and_rollback_parity_primitive";

/// Schema version for M5 affected-install-panel-primitive records.
pub const M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the affected-install-panel-primitive boundary schema.
pub const M5_AFFECTED_INSTALL_PANEL_SCHEMA_REF: &str =
    "schemas/security/m5-affected-install-panel.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AFFECTED_INSTALL_PANEL_DOC_REF: &str =
    "docs/security/m5_affected_install_panel_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_AFFECTED_INSTALL_PANEL_SHELL_ZONE_REF: &str =
    "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen advisory-component matrix this primitive narrows
/// from.
pub const M5_AFFECTED_INSTALL_PANEL_COMPONENT_MATRIX_REF: &str =
    "schemas/security/m5-advisory-component-matrix.schema.json";

/// Repo-relative path of the frozen affected-install assessment record this primitive
/// aligns its assessment vocabulary to.
pub const M5_AFFECTED_INSTALL_PANEL_ASSESSMENT_REF: &str =
    "schemas/security/affected_install_assessment.schema.json";

/// Repo-relative path of the frozen install-row contract this primitive aligns its
/// install-mode / channel lane vocabulary to.
pub const M5_AFFECTED_INSTALL_PANEL_INSTALL_ROW_REF: &str =
    "schemas/release/install_row.schema.json";

/// Repo-relative path of the frozen advisory-identity / install-assessment contract
/// doc this primitive aligns its assessment truth to.
pub const M5_AFFECTED_INSTALL_PANEL_IDENTITY_DOC_REF: &str =
    "docs/security/advisory_identity_and_install_assessment_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_AFFECTED_INSTALL_PANEL_FIXTURE_DIR: &str =
    "fixtures/security/m5-affected-install-panel-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AFFECTED_INSTALL_PANEL_ARTIFACT_REF: &str =
    "artifacts/release/m5-affected-install-panel-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AFFECTED_INSTALL_PANEL_CSV_REF: &str =
    "artifacts/release/m5-affected-install-panel-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_AFFECTED_INSTALL_PANEL_REPORT_REF: &str =
    "artifacts/security/m5-affected-install-panel-primitive.md";

/// The export fields every affected-install assessment's support / admin summary must
/// carry so a support bundle or admin report reconstructs the assessment without a
/// screenshot and never silently drops the install mode or the mirror freshness.
pub const MANDATORY_EXPORT_FIELDS: [M5AdvisoryExportField; 8] = [
    M5AdvisoryExportField::AdvisoryId,
    M5AdvisoryExportField::Severity,
    M5AdvisoryExportField::ActionState,
    M5AdvisoryExportField::AffectedSurface,
    M5AdvisoryExportField::MitigationState,
    M5AdvisoryExportField::DeliveryProfile,
    M5AdvisoryExportField::FreshnessState,
    M5AdvisoryExportField::ContinuityNote,
];

/// One claimed install-profile lane an affected-install panel can assess. These are the
/// install modes the goal names — the panel binds advisory state to the actual install
/// mode Aureline is running. The tokens align field-for-field with the frozen
/// `install_mode_class` vocabulary in `schemas/release/install_row.schema.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallProfileLane {
    /// A per-user signed install.
    PerUserInstalled,
    /// A per-machine signed install.
    PerMachineInstalled,
    /// A portable archive install.
    Portable,
    /// An offline-bundle install.
    OfflineBundle,
    /// A managed / administrator-deployed install.
    ManagedDeployed,
    /// A side-by-side preview install.
    SideBySidePreview,
}

impl M5InstallProfileLane {
    /// Every install-profile lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PerUserInstalled,
        Self::PerMachineInstalled,
        Self::Portable,
        Self::OfflineBundle,
        Self::ManagedDeployed,
        Self::SideBySidePreview,
    ];

    /// Stable token recorded in the matrix (aligned to `install_mode_class`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerUserInstalled => "per_user_installed",
            Self::PerMachineInstalled => "per_machine_installed",
            Self::Portable => "portable",
            Self::OfflineBundle => "offline_bundle",
            Self::ManagedDeployed => "managed_deployed",
            Self::SideBySidePreview => "side_by_side_preview",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PerUserInstalled => "Per-User Installed",
            Self::PerMachineInstalled => "Per-Machine Installed",
            Self::Portable => "Portable Archive",
            Self::OfflineBundle => "Offline Bundle",
            Self::ManagedDeployed => "Managed Deployed",
            Self::SideBySidePreview => "Side-by-Side Preview",
        }
    }
}

/// One anatomy part the shared affected-install panel surfaces. Every part is
/// mandatory: the whole point of the primitive is that the install identity, impacted
/// components, current exposure, mitigation status, mirror freshness, and the primary /
/// help actions are visible inline without opening a secondary detail drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffectedInstallAnatomyPart {
    /// The current build / channel / install-mode identity.
    InstallIdentity,
    /// The impacted components in the local component graph.
    ImpactedComponents,
    /// The current-exposure / "am I affected?" state.
    CurrentExposure,
    /// The local mitigation status.
    MitigationStatus,
    /// The mirror / distribution freshness.
    MirrorFreshness,
    /// The primary action — rollback / repin / update / disable.
    PrimaryAction,
    /// The help / support action attached to the panel.
    HelpSupportAction,
}

impl M5AffectedInstallAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InstallIdentity,
        Self::ImpactedComponents,
        Self::CurrentExposure,
        Self::MitigationStatus,
        Self::MirrorFreshness,
        Self::PrimaryAction,
        Self::HelpSupportAction,
    ];

    /// The anatomy parts every affected-install panel must render inline. All parts are
    /// mandatory — no assessment truth may hide behind a detail drawer.
    pub const MANDATORY: [Self; 7] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallIdentity => "install_identity",
            Self::ImpactedComponents => "impacted_components",
            Self::CurrentExposure => "current_exposure",
            Self::MitigationStatus => "mitigation_status",
            Self::MirrorFreshness => "mirror_freshness",
            Self::PrimaryAction => "primary_action",
            Self::HelpSupportAction => "help_support_action",
        }
    }
}

/// One channel that renders the shared affected-install panel. Every panel projects the
/// same assessment verdict, mirror freshness, install mode, and attached actions into
/// all four so update, Help/About, support-bundle, and admin-report surfaces describe
/// the same affected-install truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffectedInstallChannel {
    /// The update center.
    UpdateCenter,
    /// The Help / About surface.
    HelpAbout,
    /// A support-bundle export.
    SupportBundle,
    /// An administrator report.
    AdminReport,
}

impl M5AffectedInstallChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UpdateCenter,
        Self::HelpAbout,
        Self::SupportBundle,
        Self::AdminReport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::HelpAbout => "help_about",
            Self::SupportBundle => "support_bundle",
            Self::AdminReport => "admin_report",
        }
    }
}

/// A focus / navigation behavior the affected-install panel supports so the exposure,
/// mirror freshness, the primary and help actions, and the impacted-component list stay
/// keyboard-reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffectedInstallFocusBehavior {
    /// The panel is reachable and operable by keyboard focus.
    PanelKeyboardFocusable,
    /// The impacted-component list is keyboard-reachable.
    ImpactedComponentsReachable,
    /// The primary (rollback / repin / update) action is keyboard-reachable.
    PrimaryActionReachable,
    /// The help / support action is keyboard-reachable.
    HelpActionReachable,
    /// The mirror freshness is announced to a screen reader, never color-only.
    FreshnessAnnouncedToScreenReader,
    /// A stable deep-link anchor jumps to the full assessment detail.
    DeepLinkToAssessmentDetail,
}

impl M5AffectedInstallFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PanelKeyboardFocusable,
        Self::ImpactedComponentsReachable,
        Self::PrimaryActionReachable,
        Self::HelpActionReachable,
        Self::FreshnessAnnouncedToScreenReader,
        Self::DeepLinkToAssessmentDetail,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PanelKeyboardFocusable => "panel_keyboard_focusable",
            Self::ImpactedComponentsReachable => "impacted_components_reachable",
            Self::PrimaryActionReachable => "primary_action_reachable",
            Self::HelpActionReachable => "help_action_reachable",
            Self::FreshnessAnnouncedToScreenReader => "freshness_announced_to_screen_reader",
            Self::DeepLinkToAssessmentDetail => "deep_link_to_assessment_detail",
        }
    }
}

/// The normalized "am I affected?" assessment verdict an affected-install panel shows.
/// This is a resolver-side vocabulary and is not part of the frozen advisory-matrix
/// set. It is derived from the current-exposure state and the mirror freshness so a
/// stale or expired mirror can never let a clean verdict stay silently green.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallAssessmentVerdict {
    /// Installed and affected right now.
    Affected,
    /// Installed with a mitigation applied in place; no further action needed.
    MitigatedNoActionNeeded,
    /// Installed but contained (blocked or disabled); action is advised.
    ContainedActionAdvised,
    /// Installed and awaiting a rollback / repin.
    AwaitingRollbackOrRepin,
    /// Not affected — not installed on this build.
    NotAffected,
    /// Resolved — superseded by a fixed build.
    Resolved,
    /// A clean verdict cannot be asserted because the mirror metadata is stale,
    /// expired, or unknown; the assessment discloses that a mirror refresh is pending.
    CleanPendingMirrorRefresh,
}

impl M5InstallAssessmentVerdict {
    /// Every assessment verdict, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Affected,
        Self::MitigatedNoActionNeeded,
        Self::ContainedActionAdvised,
        Self::AwaitingRollbackOrRepin,
        Self::NotAffected,
        Self::Resolved,
        Self::CleanPendingMirrorRefresh,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Affected => "affected",
            Self::MitigatedNoActionNeeded => "mitigated_no_action_needed",
            Self::ContainedActionAdvised => "contained_action_advised",
            Self::AwaitingRollbackOrRepin => "awaiting_rollback_or_repin",
            Self::NotAffected => "not_affected",
            Self::Resolved => "resolved",
            Self::CleanPendingMirrorRefresh => "clean_pending_mirror_refresh",
        }
    }
}

/// True when a mirror-freshness state is authoritative enough to assert a clean verdict.
/// Only an up-to-date mirror, or one stale within the grace window, is authoritative;
/// stale-past-grace, offline-expired, and unknown mirrors are not — they auto-narrow a
/// clean answer to [`M5InstallAssessmentVerdict::CleanPendingMirrorRefresh`] rather than
/// silently staying green.
pub const fn freshness_is_authoritative(freshness: M5AdvisoryFreshnessState) -> bool {
    matches!(
        freshness,
        M5AdvisoryFreshnessState::UpToDate | M5AdvisoryFreshnessState::StaleWithinGrace
    )
}

/// Derives the "am I affected?" assessment verdict from the current-exposure state and
/// the mirror freshness. A clean exposure (not affected, resolved, or mitigated) over a
/// non-authoritative mirror is narrowed to a mirror-refresh-pending verdict.
pub const fn assessment_verdict(
    exposure: M5AdvisoryExposureState,
    freshness: M5AdvisoryFreshnessState,
) -> M5InstallAssessmentVerdict {
    use M5AdvisoryExposureState as E;
    use M5InstallAssessmentVerdict as V;

    let clean = matches!(exposure, E::NotAffected | E::Resolved | E::MitigatedInPlace);
    if clean && !freshness_is_authoritative(freshness) {
        return V::CleanPendingMirrorRefresh;
    }
    match exposure {
        E::Exposed => V::Affected,
        E::MitigatedInPlace => V::MitigatedNoActionNeeded,
        E::ContainedByBlock | E::ContainedByDisable => V::ContainedActionAdvised,
        E::AwaitingRollback => V::AwaitingRollbackOrRepin,
        E::NotAffected => V::NotAffected,
        E::Resolved => V::Resolved,
    }
}

/// The full input to the affected-install resolver for one advisory on one
/// install-profile lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallResolutionInput {
    /// The install-profile lane this assessment renders on.
    pub install_profile: M5InstallProfileLane,
    /// The copy-safe advisory id (never a raw reporter identity or URL).
    pub advisory_id: String,
    /// The advisory's severity.
    pub severity: M5AdvisorySeverityClass,
    /// Opaque, export-safe representation of the affected object / subject.
    pub affected_object_repr: String,
    /// Opaque, export-safe representation of the exact build / channel identity.
    pub build_identity_repr: String,
    /// Opaque, export-safe representation of the impacted component graph.
    pub impacted_components_repr: String,
    /// The install state of the affected build on this lane.
    pub install_state: M5AdvisoryInstallState,
    /// The mirror / distribution freshness of this lane.
    pub mirror_freshness: M5AdvisoryFreshnessState,
    /// The delivery profile of this lane.
    pub delivery_profile: M5AdvisoryDeliveryProfile,
    /// Opaque, export-safe representation of the fixed build or mitigation.
    pub fixed_build_or_mitigation_repr: String,
    /// Opaque, export-safe representation of the signer / source continuity state.
    pub signer_source_state_repr: String,
    /// The action state this assessment carries.
    pub action_state: M5AdvisoryActionState,
    /// The primary next action — rollback / repin / update / disable.
    pub primary_action: M5AdvisoryRequiredAction,
    /// The help / support action attached to the panel.
    pub help_action: M5AdvisoryRequiredAction,
    /// The local-continuity claim this assessment makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
}

impl M5AffectedInstallResolutionInput {
    /// True when any representation carries forbidden material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.advisory_id)
            || repr_is_forbidden(&self.affected_object_repr)
            || repr_is_forbidden(&self.build_identity_repr)
            || repr_is_forbidden(&self.impacted_components_repr)
            || repr_is_forbidden(&self.fixed_build_or_mitigation_repr)
            || repr_is_forbidden(&self.signer_source_state_repr)
    }

    /// The rollback / repin / help actions attached to the resolved panel, in a stable
    /// order and deduplicated. AC3: these actions stay attached to the panel instead of
    /// being scattered across separate surfaces.
    fn attached_actions(&self) -> Vec<M5AdvisoryRequiredAction> {
        let mut actions = Vec::new();
        for action in [self.primary_action, self.help_action] {
            if !actions.contains(&action) {
                actions.push(action);
            }
        }
        actions
    }
}

/// One channel projection of a resolved affected-install panel. Every projection
/// carries the same core truth — assessment verdict, mirror freshness, install mode,
/// and primary action — so the channels stay in parity; only the channel-scoped
/// headline framing differs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAffectedInstallChannelProjection {
    /// The channel this projection renders on.
    pub channel: M5AffectedInstallChannel,
    /// The channel-scoped headline (built from the shared assessment truth).
    pub headline: String,
    /// The assessment verdict (identical across channels).
    pub assessment_verdict: M5InstallAssessmentVerdict,
    /// The mirror freshness (identical across channels).
    pub mirror_freshness: M5AdvisoryFreshnessState,
    /// The install-profile lane / install mode (identical across channels).
    pub install_profile: M5InstallProfileLane,
    /// The primary next action (identical across channels).
    pub primary_action: M5AdvisoryRequiredAction,
}

/// One export column of the copy-safe affected-install summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallExportColumn {
    /// The export field.
    pub field: M5AdvisoryExportField,
    /// The export-safe value.
    pub value: String,
}

/// The copy-safe, export-safe summary of a resolved affected-install panel, for support
/// and admin flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallExportSummary {
    /// The copy-safe advisory id.
    pub advisory_id: String,
    /// The mandatory export columns, in [`MANDATORY_EXPORT_FIELDS`] order.
    pub columns: Vec<M5AffectedInstallExportColumn>,
}

/// The resolved affected-install panel for one advisory on one install-profile lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAffectedInstall {
    /// The install-profile lane this assessment renders on.
    pub install_profile: M5InstallProfileLane,
    /// The copy-safe advisory id.
    pub advisory_id: String,
    /// The advisory's severity.
    pub severity: M5AdvisorySeverityClass,
    /// The opaque affected-object representation.
    pub affected_object_repr: String,
    /// The opaque exact-build / channel identity representation.
    pub build_identity_repr: String,
    /// The opaque impacted-component-graph representation.
    pub impacted_components_repr: String,
    /// The install state of the affected build on this lane.
    pub install_state: M5AdvisoryInstallState,
    /// The derived current-exposure state.
    pub exposure_state: M5AdvisoryExposureState,
    /// The mirror / distribution freshness of this lane.
    pub mirror_freshness: M5AdvisoryFreshnessState,
    /// The delivery profile of this lane.
    pub delivery_profile: M5AdvisoryDeliveryProfile,
    /// The derived "am I affected?" assessment verdict.
    pub assessment_verdict: M5InstallAssessmentVerdict,
    /// The opaque fixed-build-or-mitigation representation.
    pub fixed_build_or_mitigation_repr: String,
    /// The opaque signer / source continuity representation.
    pub signer_source_state_repr: String,
    /// The action state this assessment carries.
    pub action_state: M5AdvisoryActionState,
    /// The primary next action — rollback / repin / update / disable.
    pub primary_action: M5AdvisoryRequiredAction,
    /// The help / support action attached to the panel.
    pub help_action: M5AdvisoryRequiredAction,
    /// The local-continuity claim this assessment makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
    /// The rollback / repin / help actions attached to the panel, deduplicated.
    pub attached_actions: Vec<M5AdvisoryRequiredAction>,
    /// True when the affected object is installed and still affected.
    pub installed_but_affected: bool,
    /// True — the assessment resolves against the local install graph.
    pub resolved_from_local_graph: bool,
    /// False — the assessment never requires an external website lookup.
    pub requires_external_website_lookup: bool,
    /// True — the mirror freshness stays visible in this assessment surface.
    pub mirror_freshness_visible: bool,
    /// True — the install mode stays visible in this assessment surface.
    pub install_mode_visible: bool,
    /// True — the rollback / repin / help actions stay attached to this panel.
    pub actions_attached_to_panel: bool,
    /// True — the primitive always keeps the affected-install panel visible.
    pub remains_visible: bool,
    /// The same assessment truth projected into every channel.
    pub channel_projections: Vec<M5ResolvedAffectedInstallChannelProjection>,
    /// The copy-safe, export-safe summary.
    pub export_summary: M5AffectedInstallExportSummary,
}

/// Errors returned by [`resolve_affected_install`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AffectedInstallResolutionError {
    /// The advisory id was empty.
    EmptyAdvisoryId,
    /// The affected-object representation was empty.
    EmptyAffectedObject,
    /// The build-identity representation was empty.
    EmptyBuildIdentity,
    /// The impacted-components representation was empty.
    EmptyImpactedComponents,
    /// The fixed-build-or-mitigation representation was empty.
    EmptyFixedBuildOrMitigation,
    /// The signer / source-state representation was empty.
    EmptySignerSourceState,
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5AffectedInstallResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAdvisoryId => "empty_advisory_id",
            Self::EmptyAffectedObject => "empty_affected_object",
            Self::EmptyBuildIdentity => "empty_build_identity",
            Self::EmptyImpactedComponents => "empty_impacted_components",
            Self::EmptyFixedBuildOrMitigation => "empty_fixed_build_or_mitigation",
            Self::EmptySignerSourceState => "empty_signer_source_state",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5AffectedInstallResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "affected-install resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AffectedInstallResolutionError {}

/// Resolves one advisory into one affected-install assessment panel.
///
/// The resolver resolves the advisory against the local install graph — no external
/// website lookup — derives the current-exposure state from the install state and the
/// "am I affected?" verdict from that exposure and the mirror freshness, keeps the
/// mirror freshness and install mode visible, keeps the rollback / repin / help actions
/// attached to the panel, keeps the panel visible, projects the same
/// verdict / freshness / install-mode / primary-action truth into every channel, and
/// emits a copy-safe, export-safe summary. It never hides the exposure, freshness, or
/// next action behind a detail drawer and never drops the copy-safe advisory id.
pub fn resolve_affected_install(
    input: &M5AffectedInstallResolutionInput,
) -> Result<M5ResolvedAffectedInstall, M5AffectedInstallResolutionError> {
    if input.advisory_id.trim().is_empty() {
        return Err(M5AffectedInstallResolutionError::EmptyAdvisoryId);
    }
    if input.affected_object_repr.trim().is_empty() {
        return Err(M5AffectedInstallResolutionError::EmptyAffectedObject);
    }
    if input.build_identity_repr.trim().is_empty() {
        return Err(M5AffectedInstallResolutionError::EmptyBuildIdentity);
    }
    if input.impacted_components_repr.trim().is_empty() {
        return Err(M5AffectedInstallResolutionError::EmptyImpactedComponents);
    }
    if input.fixed_build_or_mitigation_repr.trim().is_empty() {
        return Err(M5AffectedInstallResolutionError::EmptyFixedBuildOrMitigation);
    }
    if input.signer_source_state_repr.trim().is_empty() {
        return Err(M5AffectedInstallResolutionError::EmptySignerSourceState);
    }
    if input.carries_forbidden_material() {
        return Err(M5AffectedInstallResolutionError::ForbiddenMaterial);
    }

    let exposure_state = input.install_state.exposure_state();
    let assessment_verdict = assessment_verdict(exposure_state, input.mirror_freshness);
    let installed_but_affected = input.install_state.is_installed_but_affected();
    let attached_actions = input.attached_actions();

    let channel_projections = M5AffectedInstallChannel::ALL
        .iter()
        .map(|channel| M5ResolvedAffectedInstallChannelProjection {
            channel: *channel,
            headline: render_channel_headline(*channel, input, assessment_verdict),
            assessment_verdict,
            mirror_freshness: input.mirror_freshness,
            install_profile: input.install_profile,
            primary_action: input.primary_action,
        })
        .collect();

    let export_summary = build_export_summary(input);

    Ok(M5ResolvedAffectedInstall {
        install_profile: input.install_profile,
        advisory_id: input.advisory_id.clone(),
        severity: input.severity,
        affected_object_repr: input.affected_object_repr.clone(),
        build_identity_repr: input.build_identity_repr.clone(),
        impacted_components_repr: input.impacted_components_repr.clone(),
        install_state: input.install_state,
        exposure_state,
        mirror_freshness: input.mirror_freshness,
        delivery_profile: input.delivery_profile,
        assessment_verdict,
        fixed_build_or_mitigation_repr: input.fixed_build_or_mitigation_repr.clone(),
        signer_source_state_repr: input.signer_source_state_repr.clone(),
        action_state: input.action_state,
        primary_action: input.primary_action,
        help_action: input.help_action,
        continuity_claim: input.continuity_claim,
        attached_actions,
        installed_but_affected,
        // The assessment resolves against the local install graph: every worked panel
        // answers "am I affected?" without an external website lookup.
        resolved_from_local_graph: true,
        requires_external_website_lookup: false,
        // The mirror freshness and install mode are structurally kept in the same
        // assessment surface.
        mirror_freshness_visible: true,
        install_mode_visible: true,
        // The rollback / repin / help actions stay attached to the panel.
        actions_attached_to_panel: true,
        // The primitive structurally keeps the affected-install panel visible.
        remains_visible: true,
        channel_projections,
        export_summary,
    })
}

/// Renders one channel-scoped headline from the shared assessment truth. Every channel
/// carries the same install mode, verdict, mirror freshness, and next action; only the
/// channel prefix differs.
fn render_channel_headline(
    channel: M5AffectedInstallChannel,
    input: &M5AffectedInstallResolutionInput,
    assessment_verdict: M5InstallAssessmentVerdict,
) -> String {
    format!(
        "[{}] {} · {} · {} · mirror: {} · build: {} · next: {}",
        channel.as_str(),
        input.advisory_id,
        input.install_profile.as_str(),
        assessment_verdict.as_str(),
        input.mirror_freshness.as_str(),
        input.build_identity_repr,
        input.primary_action.as_str(),
    )
}

/// Builds the copy-safe, export-safe summary from the shared assessment truth.
fn build_export_summary(
    input: &M5AffectedInstallResolutionInput,
) -> M5AffectedInstallExportSummary {
    let columns = MANDATORY_EXPORT_FIELDS
        .iter()
        .map(|field| M5AffectedInstallExportColumn {
            field: *field,
            value: export_value(*field, input),
        })
        .collect();
    M5AffectedInstallExportSummary {
        advisory_id: input.advisory_id.clone(),
        columns,
    }
}

/// Resolves the export-safe value for one export field.
fn export_value(field: M5AdvisoryExportField, input: &M5AffectedInstallResolutionInput) -> String {
    match field {
        M5AdvisoryExportField::AdvisoryId => input.advisory_id.clone(),
        M5AdvisoryExportField::Severity => input.severity.as_str().to_owned(),
        M5AdvisoryExportField::ActionState => input.action_state.as_str().to_owned(),
        M5AdvisoryExportField::AffectedSurface => input.affected_object_repr.clone(),
        M5AdvisoryExportField::MitigationState => input.fixed_build_or_mitigation_repr.clone(),
        M5AdvisoryExportField::DeliveryProfile => input.delivery_profile.as_str().to_owned(),
        M5AdvisoryExportField::FreshnessState => input.mirror_freshness.as_str().to_owned(),
        M5AdvisoryExportField::ContinuityNote => input.continuity_claim.as_str().to_owned(),
        // Only the mandatory-export fields are projected into the summary; any other
        // field resolves to its stable token so the mapping stays total.
        other => other.as_str().to_owned(),
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs assessment truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallResolutionCase {
    /// The resolver input.
    pub input: M5AffectedInstallResolutionInput,
    /// The resolved affected-install panel. Must equal `resolve_affected_install(&input)`.
    pub resolved: M5ResolvedAffectedInstall,
}

impl M5AffectedInstallResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AffectedInstallResolutionInput) -> Self {
        let resolved = resolve_affected_install(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_affected_install(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one install-profile lane bound to the shared panel
/// anatomy, severity vocabulary, channels, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallProfileRow {
    /// Install-profile lane.
    pub install_profile: M5InstallProfileLane,
    /// Qualification class earned by this lane.
    pub qualification: M5AdvisoryQualificationClass,
    /// Owner role accountable for keeping this lane governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this row attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this row must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this row keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Anatomy parts this row renders inline (must include the mandatory parts).
    pub anatomy_parts: Vec<M5AffectedInstallAnatomyPart>,
    /// Severity classes this row can show.
    pub severity_classes: Vec<M5AdvisorySeverityClass>,
    /// Channels this row projects into (must include every channel — parity).
    pub channels: Vec<M5AffectedInstallChannel>,
    /// Action states this row projects.
    pub action_states: Vec<M5AdvisoryActionState>,
    /// Primary / help actions this row offers.
    pub required_actions: Vec<M5AdvisoryRequiredAction>,
    /// Local-continuity claims this row makes.
    pub continuity_claims: Vec<M5AdvisoryContinuityClaim>,
    /// Delivery profiles this row can carry.
    pub delivery_profiles: Vec<M5AdvisoryDeliveryProfile>,
    /// Mirror-freshness states this row can carry.
    pub freshness_states: Vec<M5AdvisoryFreshnessState>,
    /// Focus behaviors this row supports.
    pub focus_behaviors: Vec<M5AffectedInstallFocusBehavior>,
    /// Export fields this row carries (must include the mandatory truth fields).
    pub export_fields: Vec<M5AdvisoryExportField>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5AdvisoryAccessibilityRoute>,
    /// Shell subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5AdvisoryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this lane.
    pub example_assessments: Vec<M5AffectedInstallResolutionCase>,
    /// Hard invariant: this row never hides assessment truth behind a detail drawer.
    /// MUST be `false`.
    pub hides_field_behind_detail_drawer: bool,
    /// Hard invariant: this row never degrades to a generic "update available" prompt.
    /// MUST be `false`.
    pub degrades_to_generic_update_prompt: bool,
    /// Hard invariant: this row never requires an external website lookup to resolve.
    /// MUST be `false`.
    pub requires_external_website_lookup: bool,
    /// Hard invariant: this row never lets stale mirror state stay silently green. MUST
    /// be `false`.
    pub stale_mirror_stays_silently_green: bool,
    /// Hard invariant: this row never drops the copy-safe id or export summary. MUST be
    /// `false`.
    pub drops_copy_safe_id_or_export: bool,
}

impl M5InstallProfileRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AffectedInstallAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5AffectedInstallAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every channel (all four projected in parity).
    fn declares_all_channels(&self) -> bool {
        let present: BTreeSet<M5AffectedInstallChannel> = self.channels.iter().copied().collect();
        M5AffectedInstallChannel::ALL
            .iter()
            .all(|channel| present.contains(channel))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AdvisoryExportField> = self.export_fields.iter().copied().collect();
        MANDATORY_EXPORT_FIELDS
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_field_behind_detail_drawer
            && !self.degrades_to_generic_update_prompt
            && !self.requires_external_website_lookup
            && !self.stale_mirror_stays_silently_green
            && !self.drops_copy_safe_id_or_export
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallVocabularySet {
    /// Install-profile-lane tokens.
    pub install_profiles: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Severity-class tokens (reused from the frozen matrix).
    pub severity_classes: Vec<String>,
    /// Action-state tokens (reused from the frozen matrix).
    pub action_states: Vec<String>,
    /// Required-action tokens (reused from the frozen matrix).
    pub required_actions: Vec<String>,
    /// Continuity-claim tokens (reused from the frozen matrix).
    pub continuity_claims: Vec<String>,
    /// Delivery-profile tokens (reused from the frozen matrix).
    pub delivery_profiles: Vec<String>,
    /// Mirror-freshness-state tokens (reused from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Install-state tokens (reused from the advisory-row primitive).
    pub install_states: Vec<String>,
    /// Exposure-state tokens (reused from the advisory-row primitive).
    pub exposure_states: Vec<String>,
    /// Assessment-verdict tokens.
    pub assessment_verdicts: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Export-field tokens (reused from the frozen matrix).
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5AffectedInstallVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            install_profiles: tokens(&M5InstallProfileLane::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5AffectedInstallAnatomyPart::ALL, |v| v.as_str()),
            severity_classes: tokens(&M5AdvisorySeverityClass::ALL, |v| v.as_str()),
            action_states: tokens(&M5AdvisoryActionState::ALL, |v| v.as_str()),
            required_actions: tokens(&M5AdvisoryRequiredAction::ALL, |v| v.as_str()),
            continuity_claims: tokens(&M5AdvisoryContinuityClaim::ALL, |v| v.as_str()),
            delivery_profiles: tokens(&M5AdvisoryDeliveryProfile::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5AdvisoryFreshnessState::ALL, |v| v.as_str()),
            install_states: tokens(&M5AdvisoryInstallState::ALL, |v| v.as_str()),
            exposure_states: tokens(&M5AdvisoryExposureState::ALL, |v| v.as_str()),
            assessment_verdicts: tokens(&M5InstallAssessmentVerdict::ALL, |v| v.as_str()),
            channels: tokens(&M5AffectedInstallChannel::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5AffectedInstallFocusBehavior::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AdvisoryExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5AdvisoryAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5AffectedInstallGovernanceReview {
    /// One affected-install panel model is reused across every install-profile lane.
    pub one_panel_model_across_install_profiles: bool,
    /// Install identity, impacted components, exposure, and the next action are visible
    /// without a secondary detail drawer.
    pub identity_components_exposure_visible_without_drawer: bool,
    /// The assessment resolves against the local install graph without an external
    /// website lookup.
    pub resolves_against_local_install_graph: bool,
    /// The mirror freshness and install mode stay visible in the same assessment.
    pub mirror_freshness_and_install_mode_visible: bool,
    /// A stale mirror auto-narrows a clean verdict instead of staying silently green.
    pub stale_mirror_auto_narrows_clean_verdict: bool,
    /// The rollback / repin / help actions stay attached to the panel.
    pub rollback_repin_help_actions_attached: bool,
    /// The copy-safe advisory id is always preserved.
    pub copy_safe_advisory_id_preserved: bool,
    /// The export summary reconstructs assessment truth for support / admin.
    pub export_summary_reconstructs_assessment_truth: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 lanes cannot invent parallel affected-install vocabulary.
    pub later_lanes_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallConsumerProjection {
    /// The update center renders the shared affected-install panel.
    pub update_center_renders_shared_panel: bool,
    /// Help/About renders the shared affected-install panel.
    pub help_about_renders_shared_panel: bool,
    /// The support bundle renders the shared affected-install panel.
    pub support_bundle_renders_shared_panel: bool,
    /// The admin report reads a single canonical affected-install source.
    pub admin_report_reads_single_source: bool,
    /// The resolver reads a single canonical install-graph vocabulary.
    pub resolver_reads_single_install_vocabulary: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the affected-install-panel primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting affected-install audit.
    pub affected_install_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AffectedInstallPanelPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AffectedInstallPanelPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Install-profile rows.
    pub install_rows: Vec<M5InstallProfileRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AffectedInstallVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AffectedInstallGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AffectedInstallConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AffectedInstallProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AffectedInstallReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 affected-install-panel-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AffectedInstallPanelPacket {
    /// Record kind; must equal [`M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Install-profile rows.
    pub install_rows: Vec<M5InstallProfileRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AffectedInstallVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AffectedInstallGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AffectedInstallConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AffectedInstallProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AffectedInstallReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AffectedInstallPanelPacket {
    /// Builds an M5 affected-install-panel-primitive packet from stable-lane input.
    pub fn new(input: M5AffectedInstallPanelPacketInput) -> Self {
        Self {
            record_kind: M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            install_rows: input.install_rows,
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

    /// Validates the M5 affected-install-panel-primitive invariants.
    pub fn validate(&self) -> Vec<M5AffectedInstallPanelViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_RECORD_KIND {
            violations.push(M5AffectedInstallPanelViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5AffectedInstallPanelViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AffectedInstallPanelViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_install_rows(self, &mut violations);
        validate_local_graph_resolution_covered(self, &mut violations);
        validate_mirror_freshness_install_mode_covered(self, &mut violations);
        validate_attached_actions_covered(self, &mut violations);
        validate_verdict_coverage(self, &mut violations);
        validate_severity_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 affected-install panel packet serializes"),
        ) {
            violations.push(M5AffectedInstallPanelViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 affected-install panel packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per install-profile lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "install_profile,qualification,owner,shell_zone_slot,severity_classes,channels,anatomy_parts,delivery_profiles,freshness_states,export_fields,accessibility_routes,example_count\n",
        );
        for row in &self.install_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.install_profile.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.severity_classes, |v| v.as_str()),
                join_tokens(&row.channels, |v| v.as_str()),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.delivery_profiles, |v| v.as_str()),
                join_tokens(&row.freshness_states, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                join_tokens(&row.accessibility_routes, |v| v.as_str()),
                row.example_assessments.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .install_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Affected-Install Assessment Panel Primitive: Build / Channel / Install-Mode Identity, Impacted Components, Mirror Freshness, and Rollback / Repin Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Install-profile lanes: {} ({} stable)\n",
            self.install_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Anatomy parts: {}\n",
            self.vocabulary_set.anatomy_parts.join(", ")
        ));
        out.push_str(&format!(
            "- Severity classes: {}\n",
            self.vocabulary_set.severity_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Channels: {}\n",
            self.vocabulary_set.channels.join(", ")
        ));
        out.push_str(&format!(
            "- Assessment verdicts: {}\n",
            self.vocabulary_set.assessment_verdicts.join(", ")
        ));
        out.push_str(&format!(
            "- Export fields: {}\n",
            self.vocabulary_set.export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Install-profile lanes\n\n");
        for row in &self.install_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.install_profile.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked assessments: {}\n",
                row.example_assessments.len()
            ));
            for case in &row.example_assessments {
                out.push_str(&format!(
                    "    - `{}` — {} ({}), exposure `{}`, mirror `{}`\n",
                    case.resolved.advisory_id,
                    case.resolved.severity.as_str(),
                    case.resolved.assessment_verdict.as_str(),
                    case.resolved.exposure_state.as_str(),
                    case.resolved.mirror_freshness.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 affected-install-panel-primitive
/// export.
#[derive(Debug)]
pub enum M5AffectedInstallPanelArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AffectedInstallPanelViolation>),
}

impl fmt::Display for M5AffectedInstallPanelArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 affected-install panel export parse failed: {error}"
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
                    "m5 affected-install panel export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AffectedInstallPanelArtifactError {}

/// Validation failures emitted by [`M5AffectedInstallPanelPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AffectedInstallPanelViolation {
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
    /// A required install-profile lane is missing from the matrix.
    RequiredInstallProfileMissing,
    /// An install-profile row is incomplete.
    InstallRowIncomplete,
    /// An install-profile row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// An install-profile row declares no severity classes.
    SeverityClassMissing,
    /// An install-profile row does not declare every channel (channel parity broken).
    ChannelParityMismatch,
    /// An install-profile row declares no action states.
    ActionStateMissing,
    /// An install-profile row declares no required actions.
    RequiredActionMissing,
    /// An install-profile row declares no continuity claims.
    ContinuityClaimMissing,
    /// An install-profile row declares no delivery profiles.
    DeliveryProfileMissing,
    /// An install-profile row declares no mirror-freshness states.
    FreshnessStateMissing,
    /// An install-profile row declares no focus behaviors.
    FocusBehaviorMissing,
    /// An install-profile row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// An install-profile row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// An install-profile row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// An install-profile row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// An install-profile row declares no worked resolution cases.
    ExampleAssessmentMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleAssessmentDrift,
    /// A lane claiming Stable is missing required proof packet refs.
    StableInstallMissingProof,
    /// No worked resolution resolves an installed-but-affected build against the local
    /// install graph.
    LocalGraphResolutionUnproven,
    /// The worked resolutions do not keep the mirror freshness and install mode visible,
    /// or do not prove that a stale mirror auto-narrows a clean verdict.
    MirrorFreshnessInstallModeUnproven,
    /// The worked resolutions do not prove the rollback / repin / help actions stay
    /// attached to the panel.
    AttachedActionsUnproven,
    /// No worked resolution across the matrix exercises every assessment verdict.
    VerdictCoverageUnproven,
    /// No worked resolution across the matrix exercises every severity class.
    SeverityCoverageUnproven,
    /// An install-profile row violates a hard invariant.
    InstallInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AffectedInstallPanelViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredInstallProfileMissing => "required_install_profile_missing",
            Self::InstallRowIncomplete => "install_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::SeverityClassMissing => "severity_class_missing",
            Self::ChannelParityMismatch => "channel_parity_mismatch",
            Self::ActionStateMissing => "action_state_missing",
            Self::RequiredActionMissing => "required_action_missing",
            Self::ContinuityClaimMissing => "continuity_claim_missing",
            Self::DeliveryProfileMissing => "delivery_profile_missing",
            Self::FreshnessStateMissing => "freshness_state_missing",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleAssessmentMissing => "example_assessment_missing",
            Self::ExampleAssessmentDrift => "example_assessment_drift",
            Self::StableInstallMissingProof => "stable_install_missing_proof",
            Self::LocalGraphResolutionUnproven => "local_graph_resolution_unproven",
            Self::MirrorFreshnessInstallModeUnproven => "mirror_freshness_install_mode_unproven",
            Self::AttachedActionsUnproven => "attached_actions_unproven",
            Self::VerdictCoverageUnproven => "verdict_coverage_unproven",
            Self::SeverityCoverageUnproven => "severity_coverage_unproven",
            Self::InstallInvariantViolated => "install_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 affected-install-panel-primitive export.
pub fn current_stable_m5_affected_install_panel_primitive_export(
) -> Result<M5AffectedInstallPanelPacket, M5AffectedInstallPanelArtifactError> {
    let packet: M5AffectedInstallPanelPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-affected-install-panel-proof/support_export.json"
    )))
    .map_err(M5AffectedInstallPanelArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AffectedInstallPanelArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AFFECTED_INSTALL_PANEL_SCHEMA_REF,
        M5_AFFECTED_INSTALL_PANEL_DOC_REF,
        M5_AFFECTED_INSTALL_PANEL_SHELL_ZONE_REF,
        M5_AFFECTED_INSTALL_PANEL_COMPONENT_MATRIX_REF,
        M5_AFFECTED_INSTALL_PANEL_ASSESSMENT_REF,
        M5_AFFECTED_INSTALL_PANEL_INSTALL_ROW_REF,
        M5_AFFECTED_INSTALL_PANEL_IDENTITY_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AffectedInstallPanelViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AffectedInstallPanelViolation::VocabularySetDrift);
    }
}

fn validate_install_rows(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let present: BTreeSet<M5InstallProfileLane> = packet
        .install_rows
        .iter()
        .map(|row| row.install_profile)
        .collect();
    for required in M5InstallProfileLane::ALL {
        if !present.contains(&required) {
            violations.push(M5AffectedInstallPanelViolation::RequiredInstallProfileMissing);
            return;
        }
    }

    for row in &packet.install_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5AffectedInstallPanelViolation::InstallRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5AffectedInstallPanelViolation::MandatoryAnatomyMissing);
        }
        if row.severity_classes.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::SeverityClassMissing);
        }
        if !row.declares_all_channels() {
            violations.push(M5AffectedInstallPanelViolation::ChannelParityMismatch);
        }
        if row.action_states.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::ActionStateMissing);
        }
        if row.required_actions.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::RequiredActionMissing);
        }
        if row.continuity_claims.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::ContinuityClaimMissing);
        }
        if row.delivery_profiles.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::DeliveryProfileMissing);
        }
        if row.freshness_states.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::FreshnessStateMissing);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::FocusBehaviorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5AffectedInstallPanelViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5AffectedInstallPanelViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::DowngradeTriggersMissing);
        }
        if row.example_assessments.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::ExampleAssessmentMissing);
        }
        if row
            .example_assessments
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5AffectedInstallPanelViolation::ExampleAssessmentDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AffectedInstallPanelViolation::StableInstallMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AffectedInstallPanelViolation::InstallInvariantViolated);
        }
    }
}

/// At least one worked resolution must resolve an installed-but-affected build against
/// the local install graph — the acceptance-criterion proof (AC1) that claimed M5
/// install profiles can answer "am I affected?" against the local install graph without
/// requiring an external website lookup.
fn validate_local_graph_resolution_covered(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let proven = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .any(|case| {
            let panel = &case.resolved;
            panel.resolved_from_local_graph
                && !panel.requires_external_website_lookup
                && panel.installed_but_affected
                && panel.remains_visible
                && !panel.advisory_id.trim().is_empty()
                && !panel.affected_object_repr.trim().is_empty()
                && !panel.build_identity_repr.trim().is_empty()
                && !panel.impacted_components_repr.trim().is_empty()
                && panel.export_summary.columns.len() >= MANDATORY_EXPORT_FIELDS.len()
                && panel
                    .export_summary
                    .columns
                    .iter()
                    .all(|column| !column.value.trim().is_empty())
        });
    if !proven {
        violations.push(M5AffectedInstallPanelViolation::LocalGraphResolutionUnproven);
    }
}

/// Every worked resolution must keep the mirror freshness and install mode visible in
/// the same assessment surface, and at least one worked resolution must prove that a
/// stale / expired / unknown mirror auto-narrows a clean verdict to
/// [`M5InstallAssessmentVerdict::CleanPendingMirrorRefresh`] — the acceptance-criterion
/// proof (AC2) that mirror freshness and install mode remain visible and that mirror lag
/// never stays silently green.
fn validate_mirror_freshness_install_mode_covered(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let cases: Vec<&M5ResolvedAffectedInstall> = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .map(|case| &case.resolved)
        .collect();
    if cases.is_empty() {
        violations.push(M5AffectedInstallPanelViolation::MirrorFreshnessInstallModeUnproven);
        return;
    }
    let all_visible = cases.iter().all(|panel| {
        panel.mirror_freshness_visible
            && panel.install_mode_visible
            && !panel.build_identity_repr.trim().is_empty()
    });
    let stale_narrows = cases.iter().any(|panel| {
        !freshness_is_authoritative(panel.mirror_freshness)
            && panel.assessment_verdict == M5InstallAssessmentVerdict::CleanPendingMirrorRefresh
    });
    if !all_visible || !stale_narrows {
        violations.push(M5AffectedInstallPanelViolation::MirrorFreshnessInstallModeUnproven);
    }
}

/// The worked resolutions must prove the rollback / repin / help actions stay attached
/// to the panel — the acceptance-criterion proof (AC3) that rollback / repin / help
/// actions are not scattered across separate surfaces: every worked panel keeps its
/// actions attached, and the union of attached actions covers both a rollback / repin
/// action and a help / support action.
fn validate_attached_actions_covered(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let cases: Vec<&M5ResolvedAffectedInstall> = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .map(|case| &case.resolved)
        .collect();
    if cases.is_empty() {
        violations.push(M5AffectedInstallPanelViolation::AttachedActionsUnproven);
        return;
    }
    let all_attached = cases.iter().all(|panel| panel.actions_attached_to_panel);
    let actions: BTreeSet<M5AdvisoryRequiredAction> = cases
        .iter()
        .flat_map(|panel| panel.attached_actions.iter().copied())
        .collect();
    let has_rollback = actions.contains(&M5AdvisoryRequiredAction::RollbackOrRepin);
    let has_help = actions.contains(&M5AdvisoryRequiredAction::ExportSupportPacket)
        || actions.contains(&M5AdvisoryRequiredAction::ContactAdmin);
    if !all_attached || !has_rollback || !has_help {
        violations.push(M5AffectedInstallPanelViolation::AttachedActionsUnproven);
    }
}

/// Every assessment verdict must be exercised by some worked resolution so the panel is
/// proven to render every "am I affected?" answer — including the mirror-refresh-pending
/// verdict that auto-narrows a stale clean answer.
fn validate_verdict_coverage(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let present: BTreeSet<M5InstallAssessmentVerdict> = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .map(|case| case.resolved.assessment_verdict)
        .collect();
    if !M5InstallAssessmentVerdict::ALL
        .iter()
        .all(|verdict| present.contains(verdict))
    {
        violations.push(M5AffectedInstallPanelViolation::VerdictCoverageUnproven);
    }
}

/// Every severity class must be exercised by some worked resolution so the panel is
/// proven to render every severity.
fn validate_severity_coverage(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let present: BTreeSet<M5AdvisorySeverityClass> = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .map(|case| case.resolved.severity)
        .collect();
    if !M5AdvisorySeverityClass::ALL
        .iter()
        .all(|severity| present.contains(severity))
    {
        violations.push(M5AffectedInstallPanelViolation::SeverityCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_panel_model_across_install_profiles,
        review.identity_components_exposure_visible_without_drawer,
        review.resolves_against_local_install_graph,
        review.mirror_freshness_and_install_mode_visible,
        review.stale_mirror_auto_narrows_clean_verdict,
        review.rollback_repin_help_actions_attached,
        review.copy_safe_advisory_id_preserved,
        review.export_summary_reconstructs_assessment_truth,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_lanes_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5AffectedInstallPanelViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.update_center_renders_shared_panel,
        projection.help_about_renders_shared_panel,
        projection.support_bundle_renders_shared_panel,
        projection.admin_report_reads_single_source,
        projection.resolver_reads_single_install_vocabulary,
    ] {
        if !ok {
            violations.push(M5AffectedInstallPanelViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AffectedInstallPanelViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AffectedInstallPanelPacket,
    violations: &mut Vec<M5AffectedInstallPanelViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.affected_install_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AffectedInstallPanelViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// True when a single representation carries obviously forbidden material.
fn repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
