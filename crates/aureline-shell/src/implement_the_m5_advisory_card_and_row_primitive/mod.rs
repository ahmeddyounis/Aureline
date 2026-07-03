//! One reusable M5 security-advisory card / row primitive: advisory id, severity,
//! affected surface, current exposure or match state, fixed version or mitigation,
//! signer / source truth, and a primary action rendered with the same model across
//! every M5 channel that has to warn a user, admin, or support engineer.
//!
//! Aureline's frozen advisory-component matrix
//! ([`crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`])
//! names the security-advisory card and the advisory activity row as governed
//! component families and freezes the controlled severity classes, action states,
//! required actions, continuity claims, export fields, and accessibility routes an
//! advisory component may use. This module *implements* that advisory card / row
//! contract as one reusable primitive so a published vulnerability, revocation, or
//! security-impacting fix reads the same whether it affects the desktop app, an
//! extension, a remote helper, a managed service, a docs artifact, or a
//! signing / update path — instead of collapsing into a generic update banner that
//! only a screenshot can explain.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_advisory_row`] — that takes one advisory affecting one
//!    surface lane (its copy-safe id, severity, affected object, install state,
//!    fixed version or mitigation, signer / source state, action state, primary
//!    action, and local-continuity claim) and produces one
//!    [`M5ResolvedAdvisoryRow`] that names the current exposure state, keeps the row
//!    visible even when the affected object is blocked, disabled, or awaiting
//!    rollback, refuses to degrade to a generic update prompt, projects the same
//!    advisory truth into every claimed channel, and emits a copy-safe,
//!    export-safe summary. The resolver never hides the affected scope behind a
//!    detail drawer and never drops the copy-safe advisory id.
//! 2. A parity matrix — [`M5AdvisoryRowPrimitivePacket`] — that binds one row per
//!    claimed affected-surface lane to the shared advisory-row anatomy, the same
//!    severity vocabulary, the same channels, the same export fields, and the same
//!    accessibility routes, so update, marketplace, Help / About, and support
//!    surfaces render the same advisory row from one shared model.
//!
//! The severity classes ([`M5AdvisorySeverityClass`]), action states
//! ([`M5AdvisoryActionState`]), required actions ([`M5AdvisoryRequiredAction`]),
//! continuity claims ([`M5AdvisoryContinuityClaim`]), export fields
//! ([`M5AdvisoryExportField`]), accessibility routes
//! ([`M5AdvisoryAccessibilityRoute`]), qualification classes
//! ([`M5AdvisoryQualificationClass`]), and downgrade triggers
//! ([`M5AdvisoryDowngradeTrigger`]) are reused verbatim from the frozen advisory
//! matrix; the shell topology — zones, responsive classes, window classes, and
//! consumer surfaces — is reused from the frozen shell-zone matrix. This module
//! mints new vocabulary only for what the frozen matrix left implicit about the row
//! itself: its affected-surface lanes, its row anatomy, its channels, and its focus
//! behaviors. The install state and the exposure state are resolver-side
//! vocabularies, kept out of the frozen set. No M5 surface invents a second advisory
//! grammar or a parallel severity vocabulary.
//!
//! Raw reporter identities, raw exploit payloads, raw signatures, raw hostnames, raw
//! paths, private registry URLs, credentials, and raw evidence bodies stay outside
//! the support boundary; opaque, export-safe reprs are the only material carried.
//!
//! The boundary schema is
//! [`schemas/security/m5-advisory-card-row.schema.json`](../../../../schemas/security/m5-advisory-card-row.schema.json)
//! and the contract doc is
//! [`docs/security/m5_advisory_card_row_primitive_contract.md`](../../../../docs/security/m5_advisory_card_row_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/security/m5-advisory-card-row-primitive/`](../../../../fixtures/security/m5-advisory-card-row-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_advisory_card_row_primitive_extension_beta_narrowed,
    seeded_m5_advisory_card_row_primitive_packet,
    seeded_m5_advisory_card_row_primitive_signing_update_path_preview_narrowed,
    M5_ADVISORY_ROW_PRIMITIVE_PACKET_ID,
};

// The severity classes, action states, required actions, continuity claims, export
// fields, accessibility routes, qualification classes, and downgrade triggers are
// frozen once, in the advisory-component matrix. This primitive reuses them verbatim
// so it never invents a parallel severity vocabulary or a second advisory grammar.
pub use crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix::{
    M5AdvisoryAccessibilityRoute, M5AdvisoryActionState, M5AdvisoryContinuityClaim,
    M5AdvisoryDowngradeTrigger, M5AdvisoryExportField, M5AdvisoryQualificationClass,
    M5AdvisoryRequiredAction, M5AdvisorySeverityClass,
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

/// Stable record-kind tag carried by [`M5AdvisoryRowPrimitivePacket`].
pub const M5_ADVISORY_ROW_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_advisory_card_and_row_severity_affected_surface_exposure_and_primary_action_parity_primitive";

/// Schema version for M5 advisory-card-row-primitive records.
pub const M5_ADVISORY_ROW_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the advisory-card-row-primitive boundary schema.
pub const M5_ADVISORY_ROW_SCHEMA_REF: &str = "schemas/security/m5-advisory-card-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ADVISORY_ROW_DOC_REF: &str =
    "docs/security/m5_advisory_card_row_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_ADVISORY_ROW_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen advisory-component matrix this primitive narrows
/// from.
pub const M5_ADVISORY_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/security/m5-advisory-component-matrix.schema.json";

/// Repo-relative path of the frozen advisory-card surface contract this primitive
/// aligns its severity, action, and exposure vocabulary to.
pub const M5_ADVISORY_ROW_ADVISORY_CARD_REF: &str = "schemas/security/advisory_card.schema.json";

/// Repo-relative path of the frozen affected-install contract this primitive aligns
/// its install / exposure vocabulary to.
pub const M5_ADVISORY_ROW_AFFECTED_INSTALL_REF: &str =
    "schemas/security/affected_install_assessment.schema.json";

/// Repo-relative path of the frozen severity matrix this primitive's severity
/// vocabulary projects from.
pub const M5_ADVISORY_ROW_SEVERITY_MATRIX_REF: &str = "docs/security/severity_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ADVISORY_ROW_FIXTURE_DIR: &str = "fixtures/security/m5-advisory-card-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ADVISORY_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-advisory-card-row-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_ADVISORY_ROW_CSV_REF: &str = "artifacts/release/m5-advisory-card-row-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_ADVISORY_ROW_REPORT_REF: &str = "artifacts/security/m5-advisory-card-row-primitive.md";

/// The export fields every advisory row's support / admin summary must carry so a
/// support bundle reconstructs the advisory without a screenshot and never silently
/// drops a truth-bearing column.
pub const MANDATORY_EXPORT_FIELDS: [M5AdvisoryExportField; 6] = [
    M5AdvisoryExportField::AdvisoryId,
    M5AdvisoryExportField::Severity,
    M5AdvisoryExportField::ActionState,
    M5AdvisoryExportField::AffectedSurface,
    M5AdvisoryExportField::MitigationState,
    M5AdvisoryExportField::ContinuityNote,
];

/// One claimed surface lane an advisory can affect. These are the surfaces the goal
/// names — anywhere a published vulnerability, revocation, or security-impacting fix
/// can land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffectedSurfaceLane {
    /// The desktop application itself.
    DesktopApp,
    /// An installed extension.
    Extension,
    /// A remote helper / connector.
    RemoteHelper,
    /// A managed / administrator-authoritative service.
    ManagedService,
    /// A signed docs / knowledge artifact.
    DocsArtifact,
    /// A signing or update distribution path.
    SigningUpdatePath,
}

impl M5AffectedSurfaceLane {
    /// Every affected-surface lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopApp,
        Self::Extension,
        Self::RemoteHelper,
        Self::ManagedService,
        Self::DocsArtifact,
        Self::SigningUpdatePath,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopApp => "desktop_app",
            Self::Extension => "extension",
            Self::RemoteHelper => "remote_helper",
            Self::ManagedService => "managed_service",
            Self::DocsArtifact => "docs_artifact",
            Self::SigningUpdatePath => "signing_update_path",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DesktopApp => "Desktop App",
            Self::Extension => "Extension",
            Self::RemoteHelper => "Remote Helper",
            Self::ManagedService => "Managed Service",
            Self::DocsArtifact => "Docs Artifact",
            Self::SigningUpdatePath => "Signing / Update Path",
        }
    }
}

/// One anatomy part the shared advisory card / row surfaces. Every part is mandatory:
/// the whole point of the primitive is that severity, affected scope, exposure, and
/// the next action are visible inline without opening a secondary detail drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryRowAnatomyPart {
    /// The copy-safe advisory id.
    AdvisoryId,
    /// The severity of the advisory.
    Severity,
    /// The affected surface the advisory names.
    AffectedSurface,
    /// The current exposure / match state.
    CurrentExposure,
    /// The fixed version or the available mitigation.
    FixedVersionOrMitigation,
    /// The signer / source continuity state.
    SignerSourceState,
    /// The primary action the user or admin can take.
    PrimaryAction,
}

impl M5AdvisoryRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AdvisoryId,
        Self::Severity,
        Self::AffectedSurface,
        Self::CurrentExposure,
        Self::FixedVersionOrMitigation,
        Self::SignerSourceState,
        Self::PrimaryAction,
    ];

    /// The anatomy parts every advisory row must render inline. All parts are
    /// mandatory — no advisory truth may hide behind a detail drawer.
    pub const MANDATORY: [Self; 7] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryId => "advisory_id",
            Self::Severity => "severity",
            Self::AffectedSurface => "affected_surface",
            Self::CurrentExposure => "current_exposure",
            Self::FixedVersionOrMitigation => "fixed_version_or_mitigation",
            Self::SignerSourceState => "signer_source_state",
            Self::PrimaryAction => "primary_action",
        }
    }
}

/// One channel that renders the shared advisory row. Every advisory row projects the
/// same severity, exposure, and primary action into all four so update, marketplace,
/// Help / About, and support surfaces describe the same advisory truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryRowChannel {
    /// The update center.
    UpdateCenter,
    /// The marketplace / extension surface.
    Marketplace,
    /// The Help / About surface.
    HelpAbout,
    /// A support bundle export.
    SupportBundle,
}

impl M5AdvisoryRowChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UpdateCenter,
        Self::Marketplace,
        Self::HelpAbout,
        Self::SupportBundle,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::Marketplace => "marketplace",
            Self::HelpAbout => "help_about",
            Self::SupportBundle => "support_bundle",
        }
    }
}

/// A focus / navigation behavior the advisory row supports so severity, exposure, the
/// primary action, and the copy-safe id stay keyboard-reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryRowFocusBehavior {
    /// The row is reachable and operable by keyboard focus.
    RowKeyboardFocusable,
    /// The primary action is keyboard-reachable.
    PrimaryActionReachable,
    /// The copy-safe advisory id is keyboard-copyable.
    CopyAdvisoryIdReachable,
    /// The exposure state is announced to a screen reader, never color-only.
    ExposureAnnouncedToScreenReader,
    /// Keyboard navigation moves per row.
    PerRowNavigation,
    /// A stable deep-link anchor jumps to the full advisory detail.
    DeepLinkToAdvisoryDetail,
}

impl M5AdvisoryRowFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RowKeyboardFocusable,
        Self::PrimaryActionReachable,
        Self::CopyAdvisoryIdReachable,
        Self::ExposureAnnouncedToScreenReader,
        Self::PerRowNavigation,
        Self::DeepLinkToAdvisoryDetail,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RowKeyboardFocusable => "row_keyboard_focusable",
            Self::PrimaryActionReachable => "primary_action_reachable",
            Self::CopyAdvisoryIdReachable => "copy_advisory_id_reachable",
            Self::ExposureAnnouncedToScreenReader => "exposure_announced_to_screen_reader",
            Self::PerRowNavigation => "per_row_navigation",
            Self::DeepLinkToAdvisoryDetail => "deep_link_to_advisory_detail",
        }
    }
}

/// The install state of the affected object, before resolution. This is a
/// resolver-side vocabulary and is not part of the frozen advisory-matrix set. The
/// key property is that an installed-but-affected object — one that is active,
/// blocked, disabled, or awaiting rollback — keeps its advisory row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryInstallState {
    /// Installed and active; exposed to the advisory.
    InstalledActive,
    /// Installed with a mitigation applied in place.
    InstalledMitigated,
    /// Installed but blocked pending action.
    InstalledBlocked,
    /// Installed but disabled.
    InstalledDisabled,
    /// Installed and awaiting a rollback / repin.
    InstalledAwaitingRollback,
    /// Not installed on this device.
    NotInstalled,
    /// Superseded by a fixed release.
    Superseded,
}

impl M5AdvisoryInstallState {
    /// Every install state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InstalledActive,
        Self::InstalledMitigated,
        Self::InstalledBlocked,
        Self::InstalledDisabled,
        Self::InstalledAwaitingRollback,
        Self::NotInstalled,
        Self::Superseded,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstalledActive => "installed_active",
            Self::InstalledMitigated => "installed_mitigated",
            Self::InstalledBlocked => "installed_blocked",
            Self::InstalledDisabled => "installed_disabled",
            Self::InstalledAwaitingRollback => "installed_awaiting_rollback",
            Self::NotInstalled => "not_installed",
            Self::Superseded => "superseded",
        }
    }

    /// The normalized exposure state this install state resolves to.
    pub const fn exposure_state(self) -> M5AdvisoryExposureState {
        match self {
            Self::InstalledActive => M5AdvisoryExposureState::Exposed,
            Self::InstalledMitigated => M5AdvisoryExposureState::MitigatedInPlace,
            Self::InstalledBlocked => M5AdvisoryExposureState::ContainedByBlock,
            Self::InstalledDisabled => M5AdvisoryExposureState::ContainedByDisable,
            Self::InstalledAwaitingRollback => M5AdvisoryExposureState::AwaitingRollback,
            Self::NotInstalled => M5AdvisoryExposureState::NotAffected,
            Self::Superseded => M5AdvisoryExposureState::Resolved,
        }
    }

    /// `true` when the affected object is installed and still affected — active,
    /// blocked, disabled, or awaiting rollback. These rows must never disappear.
    pub const fn is_installed_but_affected(self) -> bool {
        matches!(
            self,
            Self::InstalledActive
                | Self::InstalledBlocked
                | Self::InstalledDisabled
                | Self::InstalledAwaitingRollback
        )
    }

    /// `true` when the affected object is installed but contained — blocked,
    /// disabled, or awaiting rollback. These are exactly the states that used to
    /// degrade into a generic update prompt.
    pub const fn is_contained(self) -> bool {
        matches!(
            self,
            Self::InstalledBlocked | Self::InstalledDisabled | Self::InstalledAwaitingRollback
        )
    }
}

/// The normalized current-exposure / match state an advisory row shows. This is a
/// resolver-side vocabulary and is not part of the frozen advisory-matrix set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryExposureState {
    /// Installed and exposed right now.
    Exposed,
    /// Installed with a mitigation applied in place.
    MitigatedInPlace,
    /// Contained because the affected object is blocked.
    ContainedByBlock,
    /// Contained because the affected object is disabled.
    ContainedByDisable,
    /// Installed and awaiting a rollback / repin.
    AwaitingRollback,
    /// Not affected — not installed on this device.
    NotAffected,
    /// Resolved — superseded by a fixed release.
    Resolved,
}

impl M5AdvisoryExposureState {
    /// Every exposure state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Exposed,
        Self::MitigatedInPlace,
        Self::ContainedByBlock,
        Self::ContainedByDisable,
        Self::AwaitingRollback,
        Self::NotAffected,
        Self::Resolved,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exposed => "exposed",
            Self::MitigatedInPlace => "mitigated_in_place",
            Self::ContainedByBlock => "contained_by_block",
            Self::ContainedByDisable => "contained_by_disable",
            Self::AwaitingRollback => "awaiting_rollback",
            Self::NotAffected => "not_affected",
            Self::Resolved => "resolved",
        }
    }
}

/// The full input to the advisory-row resolver for one advisory on one surface lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryRowResolutionInput {
    /// The surface lane this advisory affects.
    pub affected_surface: M5AffectedSurfaceLane,
    /// The copy-safe advisory id (never a raw reporter identity or URL).
    pub advisory_id: String,
    /// The advisory's severity.
    pub severity: M5AdvisorySeverityClass,
    /// Opaque, export-safe representation of the affected object.
    pub affected_object_repr: String,
    /// The install state of the affected object.
    pub install_state: M5AdvisoryInstallState,
    /// Opaque, export-safe representation of the fixed version or mitigation.
    pub fixed_version_or_mitigation_repr: String,
    /// Opaque, export-safe representation of the signer / source continuity state.
    pub signer_source_state_repr: String,
    /// The action state this advisory carries.
    pub action_state: M5AdvisoryActionState,
    /// The primary next action this advisory offers.
    pub primary_action: M5AdvisoryRequiredAction,
    /// The local-continuity claim this advisory makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
}

impl M5AdvisoryRowResolutionInput {
    /// True when any representation carries forbidden material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.advisory_id)
            || repr_is_forbidden(&self.affected_object_repr)
            || repr_is_forbidden(&self.fixed_version_or_mitigation_repr)
            || repr_is_forbidden(&self.signer_source_state_repr)
    }
}

/// One channel projection of a resolved advisory row. Every projection carries the
/// same core truth — severity, exposure, and primary action — so the channels stay
/// in parity; only the channel-scoped headline framing differs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedChannelProjection {
    /// The channel this projection renders on.
    pub channel: M5AdvisoryRowChannel,
    /// The channel-scoped headline (built from the shared advisory truth).
    pub headline: String,
    /// The advisory severity (identical across channels).
    pub severity: M5AdvisorySeverityClass,
    /// The current exposure state (identical across channels).
    pub exposure_state: M5AdvisoryExposureState,
    /// The primary next action (identical across channels).
    pub primary_action: M5AdvisoryRequiredAction,
}

/// One export column of the copy-safe advisory-row summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryExportColumn {
    /// The export field.
    pub field: M5AdvisoryExportField,
    /// The export-safe value.
    pub value: String,
}

/// The copy-safe, export-safe summary of a resolved advisory row, for support and
/// admin flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryRowExportSummary {
    /// The copy-safe advisory id.
    pub advisory_id: String,
    /// The mandatory export columns, in [`MANDATORY_EXPORT_FIELDS`] order.
    pub columns: Vec<M5AdvisoryExportColumn>,
}

/// The resolved advisory row for one advisory on one surface lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAdvisoryRow {
    /// The surface lane this advisory affects.
    pub affected_surface: M5AffectedSurfaceLane,
    /// The copy-safe advisory id.
    pub advisory_id: String,
    /// The advisory's severity.
    pub severity: M5AdvisorySeverityClass,
    /// The opaque affected-object representation.
    pub affected_object_repr: String,
    /// The install state of the affected object.
    pub install_state: M5AdvisoryInstallState,
    /// The normalized current-exposure state.
    pub exposure_state: M5AdvisoryExposureState,
    /// The opaque fixed-version-or-mitigation representation.
    pub fixed_version_or_mitigation_repr: String,
    /// The opaque signer / source continuity representation.
    pub signer_source_state_repr: String,
    /// The action state this advisory carries.
    pub action_state: M5AdvisoryActionState,
    /// The primary next action this advisory offers.
    pub primary_action: M5AdvisoryRequiredAction,
    /// The local-continuity claim this advisory makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
    /// True when the affected object is installed and still affected.
    pub installed_but_affected: bool,
    /// True — the primitive always keeps the advisory row visible.
    pub remains_visible: bool,
    /// False — the primitive never degrades to a generic update prompt.
    pub degrades_to_generic_prompt: bool,
    /// The same advisory truth projected into every channel.
    pub channel_projections: Vec<M5ResolvedChannelProjection>,
    /// The copy-safe, export-safe summary.
    pub export_summary: M5AdvisoryRowExportSummary,
}

/// Errors returned by [`resolve_advisory_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AdvisoryRowResolutionError {
    /// The advisory id was empty.
    EmptyAdvisoryId,
    /// The affected-object representation was empty.
    EmptyAffectedObject,
    /// The fixed-version-or-mitigation representation was empty.
    EmptyFixedVersionOrMitigation,
    /// The signer / source-state representation was empty.
    EmptySignerSourceState,
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5AdvisoryRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAdvisoryId => "empty_advisory_id",
            Self::EmptyAffectedObject => "empty_affected_object",
            Self::EmptyFixedVersionOrMitigation => "empty_fixed_version_or_mitigation",
            Self::EmptySignerSourceState => "empty_signer_source_state",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5AdvisoryRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "advisory-row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AdvisoryRowResolutionError {}

/// Resolves one advisory into one advisory row.
///
/// The resolver derives the normalized exposure state from the install state, keeps
/// the row visible even when the affected object is blocked, disabled, or awaiting
/// rollback, refuses to degrade to a generic update prompt, projects the same
/// severity / exposure / primary-action truth into every channel, and emits a
/// copy-safe, export-safe summary. It never hides the affected scope behind a detail
/// drawer and never drops the copy-safe advisory id.
pub fn resolve_advisory_row(
    input: &M5AdvisoryRowResolutionInput,
) -> Result<M5ResolvedAdvisoryRow, M5AdvisoryRowResolutionError> {
    if input.advisory_id.trim().is_empty() {
        return Err(M5AdvisoryRowResolutionError::EmptyAdvisoryId);
    }
    if input.affected_object_repr.trim().is_empty() {
        return Err(M5AdvisoryRowResolutionError::EmptyAffectedObject);
    }
    if input.fixed_version_or_mitigation_repr.trim().is_empty() {
        return Err(M5AdvisoryRowResolutionError::EmptyFixedVersionOrMitigation);
    }
    if input.signer_source_state_repr.trim().is_empty() {
        return Err(M5AdvisoryRowResolutionError::EmptySignerSourceState);
    }
    if input.carries_forbidden_material() {
        return Err(M5AdvisoryRowResolutionError::ForbiddenMaterial);
    }

    let exposure_state = input.install_state.exposure_state();
    let installed_but_affected = input.install_state.is_installed_but_affected();

    let channel_projections = M5AdvisoryRowChannel::ALL
        .iter()
        .map(|channel| M5ResolvedChannelProjection {
            channel: *channel,
            headline: render_channel_headline(*channel, input, exposure_state),
            severity: input.severity,
            exposure_state,
            primary_action: input.primary_action,
        })
        .collect();

    let export_summary = build_export_summary(input);

    Ok(M5ResolvedAdvisoryRow {
        affected_surface: input.affected_surface,
        advisory_id: input.advisory_id.clone(),
        severity: input.severity,
        affected_object_repr: input.affected_object_repr.clone(),
        install_state: input.install_state,
        exposure_state,
        fixed_version_or_mitigation_repr: input.fixed_version_or_mitigation_repr.clone(),
        signer_source_state_repr: input.signer_source_state_repr.clone(),
        action_state: input.action_state,
        primary_action: input.primary_action,
        continuity_claim: input.continuity_claim,
        installed_but_affected,
        // The primitive structurally cannot hide an advisory row and cannot degrade
        // it into a generic update prompt: every advisory always resolves to a full,
        // visible row.
        remains_visible: true,
        degrades_to_generic_prompt: false,
        channel_projections,
        export_summary,
    })
}

/// Renders one channel-scoped headline from the shared advisory truth. Every channel
/// carries the same severity, exposure, and next action; only the channel prefix
/// differs.
fn render_channel_headline(
    channel: M5AdvisoryRowChannel,
    input: &M5AdvisoryRowResolutionInput,
    exposure_state: M5AdvisoryExposureState,
) -> String {
    format!(
        "[{}] {} · {} · {} · {} · next: {}",
        channel.as_str(),
        input.advisory_id,
        input.affected_surface.as_str(),
        input.severity.as_str(),
        exposure_state.as_str(),
        input.primary_action.as_str(),
    )
}

/// Builds the copy-safe, export-safe summary from the shared advisory truth.
fn build_export_summary(input: &M5AdvisoryRowResolutionInput) -> M5AdvisoryRowExportSummary {
    let columns = MANDATORY_EXPORT_FIELDS
        .iter()
        .map(|field| M5AdvisoryExportColumn {
            field: *field,
            value: export_value(*field, input),
        })
        .collect();
    M5AdvisoryRowExportSummary {
        advisory_id: input.advisory_id.clone(),
        columns,
    }
}

/// Resolves the export-safe value for one export field.
fn export_value(field: M5AdvisoryExportField, input: &M5AdvisoryRowResolutionInput) -> String {
    match field {
        M5AdvisoryExportField::AdvisoryId => input.advisory_id.clone(),
        M5AdvisoryExportField::Severity => input.severity.as_str().to_owned(),
        M5AdvisoryExportField::ActionState => input.action_state.as_str().to_owned(),
        M5AdvisoryExportField::AffectedSurface => input.affected_surface.as_str().to_owned(),
        M5AdvisoryExportField::MitigationState => input.fixed_version_or_mitigation_repr.clone(),
        M5AdvisoryExportField::ContinuityNote => input.continuity_claim.as_str().to_owned(),
        // Only the mandatory-export fields are projected into the summary; any other
        // field resolves to its stable token so the mapping stays total.
        other => other.as_str().to_owned(),
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs advisory truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryRowResolutionCase {
    /// The resolver input.
    pub input: M5AdvisoryRowResolutionInput,
    /// The resolved advisory row. Must equal `resolve_advisory_row(&input)`.
    pub resolved: M5ResolvedAdvisoryRow,
}

impl M5AdvisoryRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AdvisoryRowResolutionInput) -> Self {
        let resolved = resolve_advisory_row(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_advisory_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one affected-surface lane bound to the shared
/// advisory-row anatomy, severity vocabulary, channels, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisorySurfaceRow {
    /// Affected-surface lane.
    pub affected_surface: M5AffectedSurfaceLane,
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
    pub anatomy_parts: Vec<M5AdvisoryRowAnatomyPart>,
    /// Severity classes this row can show.
    pub severity_classes: Vec<M5AdvisorySeverityClass>,
    /// Channels this row projects into (must include every channel — parity).
    pub channels: Vec<M5AdvisoryRowChannel>,
    /// Action states this row projects.
    pub action_states: Vec<M5AdvisoryActionState>,
    /// Primary next actions this row offers.
    pub required_actions: Vec<M5AdvisoryRequiredAction>,
    /// Local-continuity claims this row makes.
    pub continuity_claims: Vec<M5AdvisoryContinuityClaim>,
    /// Focus behaviors this row supports.
    pub focus_behaviors: Vec<M5AdvisoryRowFocusBehavior>,
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
    pub example_advisories: Vec<M5AdvisoryRowResolutionCase>,
    /// Hard invariant: this row never hides advisory truth behind a detail drawer.
    /// MUST be `false`.
    pub hides_field_behind_detail_drawer: bool,
    /// Hard invariant: this row never disappears for an installed-but-affected item.
    /// MUST be `false`.
    pub disappears_when_installed_but_affected: bool,
    /// Hard invariant: this row never degrades to a generic update prompt. MUST be
    /// `false`.
    pub degrades_to_generic_update_prompt: bool,
    /// Hard invariant: this row never drops the copy-safe id or export summary. MUST
    /// be `false`.
    pub drops_copy_safe_id_or_export: bool,
}

impl M5AdvisorySurfaceRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AdvisoryRowAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5AdvisoryRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every channel (all four projected in parity).
    fn declares_all_channels(&self) -> bool {
        let present: BTreeSet<M5AdvisoryRowChannel> = self.channels.iter().copied().collect();
        M5AdvisoryRowChannel::ALL
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
            && !self.disappears_when_installed_but_affected
            && !self.degrades_to_generic_update_prompt
            && !self.drops_copy_safe_id_or_export
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryRowVocabularySet {
    /// Affected-surface-lane tokens.
    pub affected_surfaces: Vec<String>,
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
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Export-field tokens (reused from the frozen matrix).
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5AdvisoryRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            affected_surfaces: tokens(&M5AffectedSurfaceLane::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5AdvisoryRowAnatomyPart::ALL, |v| v.as_str()),
            severity_classes: tokens(&M5AdvisorySeverityClass::ALL, |v| v.as_str()),
            action_states: tokens(&M5AdvisoryActionState::ALL, |v| v.as_str()),
            required_actions: tokens(&M5AdvisoryRequiredAction::ALL, |v| v.as_str()),
            continuity_claims: tokens(&M5AdvisoryContinuityClaim::ALL, |v| v.as_str()),
            channels: tokens(&M5AdvisoryRowChannel::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5AdvisoryRowFocusBehavior::ALL, |v| v.as_str()),
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
pub struct M5AdvisoryRowGovernanceReview {
    /// One advisory row model is reused across every channel.
    pub one_row_model_across_channels: bool,
    /// Severity, affected scope, exposure, and next action are visible without a
    /// secondary detail drawer.
    pub severity_scope_exposure_visible_without_drawer: bool,
    /// Installed-but-affected items keep their advisory row.
    pub installed_but_affected_stays_visible: bool,
    /// No advisory ever degrades to a generic update prompt.
    pub never_degrades_to_generic_update_prompt: bool,
    /// The copy-safe advisory id is always preserved.
    pub copy_safe_advisory_id_preserved: bool,
    /// The export summary reconstructs advisory truth for support / admin.
    pub export_summary_reconstructs_advisory_truth: bool,
    /// The primary action stays in parity across every channel.
    pub primary_action_parity_across_channels: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 lanes cannot invent parallel advisory-row vocabulary.
    pub later_lanes_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryRowConsumerProjection {
    /// The update center renders the shared advisory row.
    pub update_center_renders_shared_row: bool,
    /// The marketplace renders the shared advisory row.
    pub marketplace_renders_shared_row: bool,
    /// Help / About renders the shared advisory row.
    pub help_about_renders_shared_row: bool,
    /// Support / export reads a single canonical advisory-row source.
    pub support_export_reads_single_source: bool,
    /// The resolver reads a single canonical advisory vocabulary.
    pub resolver_reads_single_advisory_vocabulary: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the advisory-row primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryRowReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting advisory-row audit.
    pub advisory_row_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AdvisoryRowPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AdvisoryRowPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5AdvisorySurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AdvisoryRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AdvisoryRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AdvisoryRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AdvisoryRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AdvisoryRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 advisory-card-row-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdvisoryRowPrimitivePacket {
    /// Record kind; must equal [`M5_ADVISORY_ROW_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ADVISORY_ROW_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5AdvisorySurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AdvisoryRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AdvisoryRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AdvisoryRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AdvisoryRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AdvisoryRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AdvisoryRowPrimitivePacket {
    /// Builds an M5 advisory-card-row-primitive packet from stable-lane input.
    pub fn new(input: M5AdvisoryRowPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_ADVISORY_ROW_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_ADVISORY_ROW_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
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

    /// Validates the M5 advisory-card-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5AdvisoryRowPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ADVISORY_ROW_PRIMITIVE_RECORD_KIND {
            violations.push(M5AdvisoryRowPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ADVISORY_ROW_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5AdvisoryRowPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AdvisoryRowPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_channel_parity_covered(self, &mut violations);
        validate_inline_visibility_covered(self, &mut violations);
        validate_installed_but_affected_covered(self, &mut violations);
        validate_severity_coverage(self, &mut violations);
        validate_exposure_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 advisory-row primitive packet serializes"),
        ) {
            violations.push(M5AdvisoryRowPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 advisory-row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per affected-surface lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "affected_surface,qualification,owner,shell_zone_slot,severity_classes,channels,anatomy_parts,export_fields,accessibility_routes,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.affected_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.severity_classes, |v| v.as_str()),
                join_tokens(&row.channels, |v| v.as_str()),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                join_tokens(&row.accessibility_routes, |v| v.as_str()),
                row.example_advisories.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Security-Advisory Card / Row Primitive: Severity, Affected Surface, Exposure, and Primary-Action Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Affected-surface lanes: {} ({} stable)\n",
            self.surface_rows.len(),
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
            "- Export fields: {}\n",
            self.vocabulary_set.export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Affected-surface lanes\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.affected_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked advisories: {}\n",
                row.example_advisories.len()
            ));
            for case in &row.example_advisories {
                out.push_str(&format!(
                    "    - `{}` — {} ({}){}\n",
                    case.resolved.advisory_id,
                    case.resolved.severity.as_str(),
                    case.resolved.exposure_state.as_str(),
                    if case.resolved.installed_but_affected {
                        ", installed-but-affected, row stays visible"
                    } else {
                        ""
                    }
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 advisory-card-row-primitive export.
#[derive(Debug)]
pub enum M5AdvisoryRowPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AdvisoryRowPrimitiveViolation>),
}

impl fmt::Display for M5AdvisoryRowPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 advisory-row primitive export parse failed: {error}"
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
                    "m5 advisory-row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AdvisoryRowPrimitiveArtifactError {}

/// Validation failures emitted by [`M5AdvisoryRowPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AdvisoryRowPrimitiveViolation {
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
    /// A required affected-surface lane is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A surface row declares no severity classes.
    SeverityClassMissing,
    /// A surface row does not declare every channel (channel parity broken).
    ChannelParityMismatch,
    /// A surface row declares no action states.
    ActionStateMissing,
    /// A surface row declares no required actions.
    RequiredActionMissing,
    /// A surface row declares no continuity claims.
    ContinuityClaimMissing,
    /// A surface row declares no focus behaviors.
    FocusBehaviorMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no worked resolution cases.
    ExampleAdvisoryMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleAdvisoryDrift,
    /// A lane claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No worked resolution across the matrix projects every channel in parity.
    ChannelParityUnproven,
    /// No worked resolution across the matrix renders a full advisory row inline
    /// without a detail drawer.
    InlineVisibilityUnproven,
    /// No worked resolution across the matrix keeps an installed-but-affected row
    /// visible without degrading to a generic update prompt.
    InstalledButAffectedUnproven,
    /// No worked resolution across the matrix exercises every severity class.
    SeverityCoverageUnproven,
    /// No worked resolution across the matrix exercises every exposure state.
    ExposureCoverageUnproven,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
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

impl M5AdvisoryRowPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::SeverityClassMissing => "severity_class_missing",
            Self::ChannelParityMismatch => "channel_parity_mismatch",
            Self::ActionStateMissing => "action_state_missing",
            Self::RequiredActionMissing => "required_action_missing",
            Self::ContinuityClaimMissing => "continuity_claim_missing",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleAdvisoryMissing => "example_advisory_missing",
            Self::ExampleAdvisoryDrift => "example_advisory_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::ChannelParityUnproven => "channel_parity_unproven",
            Self::InlineVisibilityUnproven => "inline_visibility_unproven",
            Self::InstalledButAffectedUnproven => "installed_but_affected_unproven",
            Self::SeverityCoverageUnproven => "severity_coverage_unproven",
            Self::ExposureCoverageUnproven => "exposure_coverage_unproven",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 advisory-card-row-primitive export.
pub fn current_stable_m5_advisory_card_row_primitive_export(
) -> Result<M5AdvisoryRowPrimitivePacket, M5AdvisoryRowPrimitiveArtifactError> {
    let packet: M5AdvisoryRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-advisory-card-row-proof/support_export.json"
    )))
    .map_err(M5AdvisoryRowPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AdvisoryRowPrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ADVISORY_ROW_SCHEMA_REF,
        M5_ADVISORY_ROW_DOC_REF,
        M5_ADVISORY_ROW_SHELL_ZONE_REF,
        M5_ADVISORY_ROW_COMPONENT_MATRIX_REF,
        M5_ADVISORY_ROW_ADVISORY_CARD_REF,
        M5_ADVISORY_ROW_AFFECTED_INSTALL_REF,
        M5_ADVISORY_ROW_SEVERITY_MATRIX_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AdvisoryRowPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AdvisoryRowPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5AffectedSurfaceLane> = packet
        .surface_rows
        .iter()
        .map(|row| row.affected_surface)
        .collect();
    for required in M5AffectedSurfaceLane::ALL {
        if !present.contains(&required) {
            violations.push(M5AdvisoryRowPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5AdvisoryRowPrimitiveViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5AdvisoryRowPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.severity_classes.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::SeverityClassMissing);
        }
        if !row.declares_all_channels() {
            violations.push(M5AdvisoryRowPrimitiveViolation::ChannelParityMismatch);
        }
        if row.action_states.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::ActionStateMissing);
        }
        if row.required_actions.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::RequiredActionMissing);
        }
        if row.continuity_claims.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::ContinuityClaimMissing);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::FocusBehaviorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5AdvisoryRowPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5AdvisoryRowPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_advisories.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::ExampleAdvisoryMissing);
        }
        if row
            .example_advisories
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5AdvisoryRowPrimitiveViolation::ExampleAdvisoryDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AdvisoryRowPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AdvisoryRowPrimitiveViolation::SurfaceInvariantViolated);
        }
    }
}

/// Every channel must be projected in parity by some worked resolution — the
/// acceptance-criterion proof (AC1) that update, marketplace, Help / About, and
/// support surfaces render the same advisory row model.
fn validate_channel_parity_covered(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5AdvisoryRowChannel> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_advisories.iter())
        .flat_map(|case| case.resolved.channel_projections.iter())
        .map(|projection| projection.channel)
        .collect();
    if !M5AdvisoryRowChannel::ALL
        .iter()
        .all(|channel| present.contains(channel))
    {
        violations.push(M5AdvisoryRowPrimitiveViolation::ChannelParityUnproven);
    }
}

/// At least one worked resolution must render the full advisory row inline — severity,
/// affected scope, exposure, and next action visible with a complete export summary —
/// the acceptance-criterion proof (AC2) that advisory truth never hides behind a
/// secondary detail drawer.
fn validate_inline_visibility_covered(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let proven = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_advisories.iter())
        .any(|case| {
            let row = &case.resolved;
            row.remains_visible
                && !row.degrades_to_generic_prompt
                && !row.advisory_id.trim().is_empty()
                && !row.affected_object_repr.trim().is_empty()
                && !row.fixed_version_or_mitigation_repr.trim().is_empty()
                && !row.signer_source_state_repr.trim().is_empty()
                && row.export_summary.columns.len() >= MANDATORY_EXPORT_FIELDS.len()
                && row
                    .export_summary
                    .columns
                    .iter()
                    .all(|column| !column.value.trim().is_empty())
        });
    if !proven {
        violations.push(M5AdvisoryRowPrimitiveViolation::InlineVisibilityUnproven);
    }
}

/// At least one worked resolution must keep an installed-but-affected item's row
/// visible while the item is blocked, disabled, or awaiting rollback — the
/// acceptance-criterion proof (AC3) that such items no longer disappear or degrade to
/// a generic update prompt.
fn validate_installed_but_affected_covered(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let proven = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_advisories.iter())
        .any(|case| {
            case.input.install_state.is_contained()
                && case.resolved.installed_but_affected
                && case.resolved.remains_visible
                && !case.resolved.degrades_to_generic_prompt
        });
    if !proven {
        violations.push(M5AdvisoryRowPrimitiveViolation::InstalledButAffectedUnproven);
    }
}

/// Every severity class must be exercised by some worked resolution so the row is
/// proven to render every severity.
fn validate_severity_coverage(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5AdvisorySeverityClass> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_advisories.iter())
        .map(|case| case.resolved.severity)
        .collect();
    if !M5AdvisorySeverityClass::ALL
        .iter()
        .all(|severity| present.contains(severity))
    {
        violations.push(M5AdvisoryRowPrimitiveViolation::SeverityCoverageUnproven);
    }
}

/// Every exposure state must be exercised by some worked resolution so the row is
/// proven to render every current-exposure / match state.
fn validate_exposure_coverage(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5AdvisoryExposureState> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_advisories.iter())
        .map(|case| case.resolved.exposure_state)
        .collect();
    if !M5AdvisoryExposureState::ALL
        .iter()
        .all(|state| present.contains(state))
    {
        violations.push(M5AdvisoryRowPrimitiveViolation::ExposureCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_row_model_across_channels,
        review.severity_scope_exposure_visible_without_drawer,
        review.installed_but_affected_stays_visible,
        review.never_degrades_to_generic_update_prompt,
        review.copy_safe_advisory_id_preserved,
        review.export_summary_reconstructs_advisory_truth,
        review.primary_action_parity_across_channels,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_lanes_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5AdvisoryRowPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.update_center_renders_shared_row,
        projection.marketplace_renders_shared_row,
        projection.help_about_renders_shared_row,
        projection.support_export_reads_single_source,
        projection.resolver_reads_single_advisory_vocabulary,
    ] {
        if !ok {
            violations.push(M5AdvisoryRowPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AdvisoryRowPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AdvisoryRowPrimitivePacket,
    violations: &mut Vec<M5AdvisoryRowPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.advisory_row_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AdvisoryRowPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces
/// a stray comma.
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
