//! Implemented M5 docs-pane-header and boundary-fact-grid primitives.
//!
//! The frozen [embedded-boundary component matrix][matrix] names the reusable embedded /
//! browser-handoff boundary UI components and locks their controlled vocabulary. This module is
//! the first implement lane over that matrix: it turns the two documentation-facing components —
//! the **docs / help pane header** and the **boundary-fact grid** — into resolvers that produce
//! export-safe, honest projections instead of prose or one-off pane chrome.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — a user can tell whether a docs/help pane is project-local, mirrored vendor material,
//!   extension-contributed, or browser-handoff-required without leaving the pane.**
//!   [`resolve_docs_pane_header`] refuses to read as a clean pane when the source class cannot be
//!   distinguished, when the owner/origin is undisclosed, when the version / pack identity is
//!   missing, or when the last-updated state is unstated; it degrades instead. A clean header
//!   names its source class, owner/origin, pack identity, and freshness, and reports
//!   `distinguishable_source = true`.
//! * **AC2 — help panes never masquerade as approval or policy-authority surfaces and always
//!   expose an external handoff when the source contract requires it.**
//!   [`resolve_boundary_fact_grid`] degrades to
//!   [`M5BoundaryFactGridDegradeReason::MasqueradesAsApprovalAuthority`] the moment a grid claims
//!   approval / policy authority or reads as suitable for high-risk approval, and
//!   [`resolve_docs_pane_header`] degrades to
//!   [`M5DocsPaneHeaderDegradeReason::HandoffRequiredButNotExposed`] whenever the source contract
//!   requires a browser handoff but no open-externally action is offered.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EmbeddedBoundaryDisposition`] boundary-disposition vocabulary, the
//! [`M5EmbeddedFreshnessState`] freshness vocabulary, the [`WebviewOwnerClass`] owner/origin
//! vocabulary, the [`CapabilityLimitClass`] capability-limit vocabulary, and the
//! [`DataExitBoundary`] data-boundary vocabulary — so this lane can never fork its own owner,
//! origin, boundary, or fallback wording.
//!
//! [matrix]: crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_docs_boundary_controls, seeded_m5_docs_boundary_controls_docs_browser_beta_narrowed,
    seeded_m5_docs_boundary_controls_embedded_webview_preview_narrowed,
    M5_DOCS_BOUNDARY_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix::{
    M5EmbeddedAccessibilityRoute, M5EmbeddedBoundaryDisposition, M5EmbeddedConsumerSurface,
    M5EmbeddedDeploymentLine, M5EmbeddedDowngradeTrigger, M5EmbeddedFreshnessState,
    M5EmbeddedQualificationClass, M5EmbeddedRequiredLabel, BOUND_DATA_EXIT_BOUNDARIES,
    M5_BOUNDARY_FACT_GRID_SCHEMA_REF, M5_DOCS_PANE_HEADER_SCHEMA_REF,
    M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF, M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
};
use crate::m5_auth_boundaries::{
    CapabilityLimitClass, DataExitBoundary, WebviewOwnerClass, M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
};

/// Stable record-kind tag carried by [`M5DocsBoundaryControlsPacket`].
pub const M5_DOCS_BOUNDARY_CONTROLS_RECORD_KIND: &str =
    "implement_m5_docs_pane_header_and_boundary_fact_grid_controls";

/// Schema version for M5 docs-pane-header / boundary-fact-grid controls records.
pub const M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-docs-pane-header-boundary-fact-grid-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_DOCS_BOUNDARY_CONTROLS_DOC_REF: &str =
    "docs/help/m5_docs_pane_header_and_boundary_fact_grid_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_BOUNDARY_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-docs-pane-header-boundary-fact-grid-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_DOCS_BOUNDARY_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-docs-pane-header-boundary-fact-grid-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_DOCS_BOUNDARY_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-docs-pane-header-boundary-fact-grid-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_BOUNDARY_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-docs-pane-header-boundary-fact-grid-controls";

/// Consumer surface a docs-boundary controls row projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5DocsBoundaryConsumerSurface = M5EmbeddedConsumerSurface;

/// The single controlled source class a docs / help pane can carry. These are the exact
/// acceptance-criteria distinctions a user must be able to make — project-local, mirrored vendor
/// material, extension-contributed, browser-handoff-required — without leaving the pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSourceClass {
    /// Project-local documentation authored and shipped by Aureline itself.
    ProjectLocal,
    /// First-party documentation served from a first-party hosted surface.
    FirstPartyHosted,
    /// Mirrored vendor material rendered in-product, labelled as vendor-owned.
    MirroredVendor,
    /// Documentation contributed by an installed extension.
    ExtensionContributed,
    /// Content that can only be reached by handing off to the browser.
    BrowserHandoffRequired,
    /// The source class cannot currently be determined.
    SourceUnknown,
}

impl M5DocsSourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProjectLocal,
        Self::FirstPartyHosted,
        Self::MirroredVendor,
        Self::ExtensionContributed,
        Self::BrowserHandoffRequired,
        Self::SourceUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectLocal => "project_local",
            Self::FirstPartyHosted => "first_party_hosted",
            Self::MirroredVendor => "mirrored_vendor",
            Self::ExtensionContributed => "extension_contributed",
            Self::BrowserHandoffRequired => "browser_handoff_required",
            Self::SourceUnknown => "source_unknown",
        }
    }

    /// Whether this source class is distinguishable (anything but unknown).
    pub const fn is_distinguishable(self) -> bool {
        !matches!(self, Self::SourceUnknown)
    }
}

/// The reading posture a boundary-fact grid discloses — whether the pane is safe to read locally,
/// from a hosted or mirrored snapshot, only offline, or requires a browser handoff — so a stale,
/// offline, or mirrored posture is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PaneReadingPosture {
    /// Local, first-party content safe to read in-product.
    LocalReadingSafe,
    /// Hosted first-party content safe to read in-product.
    HostedReadingSafe,
    /// A mirrored vendor snapshot safe to read in-product, labelled as mirrored.
    MirroredReadingSafe,
    /// An offline snapshot with no refresh path, readable but labelled as offline.
    OfflineReadingOnly,
    /// The reading posture cannot currently be resolved.
    PostureUnknown,
}

impl M5PaneReadingPosture {
    /// Every reading posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalReadingSafe,
        Self::HostedReadingSafe,
        Self::MirroredReadingSafe,
        Self::OfflineReadingOnly,
        Self::PostureUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReadingSafe => "local_reading_safe",
            Self::HostedReadingSafe => "hosted_reading_safe",
            Self::MirroredReadingSafe => "mirrored_reading_safe",
            Self::OfflineReadingOnly => "offline_reading_only",
            Self::PostureUnknown => "posture_unknown",
        }
    }

    /// Whether this posture is stated (anything but unknown).
    pub const fn is_stated(self) -> bool {
        !matches!(self, Self::PostureUnknown)
    }
}

/// One mandatory rendered part a docs-pane header or boundary-fact grid must be able to show, so no
/// boundary truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsBoundaryAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed boundary disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The source class behind the pane (docs-pane header).
    SourceClass,
    /// The owner and origin behind the pane (docs-pane header).
    OwnerOrigin,
    /// The version / pack identity behind the pane (docs-pane header).
    VersionOrPackIdentity,
    /// The last-updated / freshness state behind the pane (docs-pane header).
    LastUpdated,
    /// The open-externally action to the browser handoff (docs-pane header).
    OpenExternally,
    /// The find-in-page affordance where applicable (docs-pane header).
    FindInPage,
    /// The data boundary behind the pane (boundary-fact grid).
    DataBoundary,
    /// The offline / mirrored reading posture (boundary-fact grid).
    ReadingPosture,
    /// Why the pane is trustworthy for reading but not for high-risk approval (boundary-fact grid).
    ApprovalBoundary,
}

impl M5DocsBoundaryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::SourceClass,
        Self::OwnerOrigin,
        Self::VersionOrPackIdentity,
        Self::LastUpdated,
        Self::OpenExternally,
        Self::FindInPage,
        Self::DataBoundary,
        Self::ReadingPosture,
        Self::ApprovalBoundary,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::SourceClass => "source_class",
            Self::OwnerOrigin => "owner_origin",
            Self::VersionOrPackIdentity => "version_or_pack_identity",
            Self::LastUpdated => "last_updated",
            Self::OpenExternally => "open_externally",
            Self::FindInPage => "find_in_page",
            Self::DataBoundary => "data_boundary",
            Self::ReadingPosture => "reading_posture",
            Self::ApprovalBoundary => "approval_boundary",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsBoundaryNextAction {
    /// Open the source externally in the browser.
    OpenExternally,
    /// Open the source / owner-origin detail.
    OpenSourceInfo,
    /// View the data boundary and reading posture detail.
    ViewDataBoundary,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// Open find-in-page for the pane.
    OpenFindInPage,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5DocsBoundaryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenExternally,
        Self::OpenSourceInfo,
        Self::ViewDataBoundary,
        Self::ReviewDiagnostics,
        Self::OpenFindInPage,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExternally => "open_externally",
            Self::OpenSourceInfo => "open_source_info",
            Self::ViewDataBoundary => "view_data_boundary",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::OpenFindInPage => "open_find_in_page",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a docs-boundary controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsBoundaryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The boundary dispositions carried.
    BoundaryDispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The source class named by the docs-pane header.
    SourceClass,
    /// The owner and origin named by the docs-pane header.
    OwnerOrigin,
    /// The data boundary named by the boundary-fact grid.
    DataBoundary,
    /// The freshness / last-updated state.
    Freshness,
    /// The external-handoff exposure.
    ExternalHandoff,
    /// The accountable owner role.
    OwnerRole,
}

impl M5DocsBoundaryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::BoundaryDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SourceClass,
        Self::OwnerOrigin,
        Self::DataBoundary,
        Self::Freshness,
        Self::ExternalHandoff,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::BoundaryDispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::BoundaryDispositions => "boundary_dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SourceClass => "source_class",
            Self::OwnerOrigin => "owner_origin",
            Self::DataBoundary => "data_boundary",
            Self::Freshness => "freshness",
            Self::ExternalHandoff => "external_handoff",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a docs-pane header degraded below a clean, fully-legible state. The degrade-first ladder
/// returns one of these instead of ever letting an ambiguous pane header read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsPaneHeaderDegradeReason {
    /// The source class is unstated; a user cannot tell what the pane is showing (AC1 violation).
    SourceClassUnstated,
    /// The owner / origin behind the pane is undisclosed.
    OwnerOrOriginUnstated,
    /// The version or pack identity is missing.
    VersionOrPackIdentityMissing,
    /// The last-updated / freshness state is unstated.
    LastUpdatedUnstated,
    /// The source contract requires a browser handoff but no open-externally action is exposed
    /// (AC2 violation).
    HandoffRequiredButNotExposed,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DocsPaneHeaderDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceClassUnstated,
        Self::OwnerOrOriginUnstated,
        Self::VersionOrPackIdentityMissing,
        Self::LastUpdatedUnstated,
        Self::HandoffRequiredButNotExposed,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClassUnstated => "source_class_unstated",
            Self::OwnerOrOriginUnstated => "owner_or_origin_unstated",
            Self::VersionOrPackIdentityMissing => "version_or_pack_identity_missing",
            Self::LastUpdatedUnstated => "last_updated_unstated",
            Self::HandoffRequiredButNotExposed => "handoff_required_but_not_exposed",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DocsBoundaryNextAction {
        match self {
            Self::SourceClassUnstated
            | Self::OwnerOrOriginUnstated
            | Self::VersionOrPackIdentityMissing => M5DocsBoundaryNextAction::OpenSourceInfo,
            Self::LastUpdatedUnstated | Self::ProofStale => {
                M5DocsBoundaryNextAction::ReviewDiagnostics
            }
            Self::HandoffRequiredButNotExposed => M5DocsBoundaryNextAction::OpenExternally,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::SourceClassUnstated => M5EmbeddedDowngradeTrigger::GenericChromeWordingUsed,
            Self::OwnerOrOriginUnstated => M5EmbeddedDowngradeTrigger::OwnerOrOriginUnstated,
            Self::VersionOrPackIdentityMissing | Self::LastUpdatedUnstated => {
                M5EmbeddedDowngradeTrigger::FreshnessOrLastUpdatedUnstated
            }
            Self::HandoffRequiredButNotExposed => {
                M5EmbeddedDowngradeTrigger::BrowserFallbackHiddenInMenusOnly
            }
            Self::ProofStale => M5EmbeddedDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a boundary-fact grid degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryFactGridDegradeReason {
    /// The data boundary is unstated.
    DataBoundaryUnstated,
    /// The grid claims approval / policy authority or reads as suitable for high-risk approval
    /// (AC2 violation).
    MasqueradesAsApprovalAuthority,
    /// The offline / mirrored reading posture is unstated.
    OfflineOrMirroredPostureUnstated,
    /// Why the pane is trustworthy for in-product reading is not explained.
    ReadingTrustNotExplained,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5BoundaryFactGridDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DataBoundaryUnstated,
        Self::MasqueradesAsApprovalAuthority,
        Self::OfflineOrMirroredPostureUnstated,
        Self::ReadingTrustNotExplained,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataBoundaryUnstated => "data_boundary_unstated",
            Self::MasqueradesAsApprovalAuthority => "masquerades_as_approval_authority",
            Self::OfflineOrMirroredPostureUnstated => "offline_or_mirrored_posture_unstated",
            Self::ReadingTrustNotExplained => "reading_trust_not_explained",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DocsBoundaryNextAction {
        match self {
            Self::DataBoundaryUnstated | Self::ReadingTrustNotExplained => {
                M5DocsBoundaryNextAction::ViewDataBoundary
            }
            Self::MasqueradesAsApprovalAuthority => M5DocsBoundaryNextAction::OpenExternally,
            Self::OfflineOrMirroredPostureUnstated | Self::ProofStale => {
                M5DocsBoundaryNextAction::ReviewDiagnostics
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EmbeddedDowngradeTrigger {
        match self {
            Self::DataBoundaryUnstated => M5EmbeddedDowngradeTrigger::DataBoundaryUnstated,
            Self::MasqueradesAsApprovalAuthority => {
                M5EmbeddedDowngradeTrigger::ImitatesNativeApprovalChrome
            }
            Self::OfflineOrMirroredPostureUnstated => {
                M5EmbeddedDowngradeTrigger::StaleOrBlockedShownAsFresh
            }
            Self::ReadingTrustNotExplained => M5EmbeddedDowngradeTrigger::GenericChromeWordingUsed,
            Self::ProofStale => M5EmbeddedDowngradeTrigger::ProofStale,
        }
    }
}

/// Maps a docs source class and freshness state to the single controlled boundary disposition.
fn disposition_for_source(
    source: M5DocsSourceClass,
    freshness: M5EmbeddedFreshnessState,
) -> M5EmbeddedBoundaryDisposition {
    use M5EmbeddedBoundaryDisposition as D;
    if matches!(source, M5DocsSourceClass::BrowserHandoffRequired) {
        return D::BrowserHandoffOnly;
    }
    match freshness {
        M5EmbeddedFreshnessState::StaleSnapshot => D::StaleSnapshot,
        M5EmbeddedFreshnessState::OfflineSnapshot => D::OfflineSnapshot,
        _ => match source {
            M5DocsSourceClass::ProjectLocal => D::LiveFirstPartyLocal,
            M5DocsSourceClass::FirstPartyHosted => D::LiveFirstPartyHosted,
            M5DocsSourceClass::MirroredVendor | M5DocsSourceClass::ExtensionContributed => {
                D::LiveProviderOwned
            }
            M5DocsSourceClass::BrowserHandoffRequired => D::BrowserHandoffOnly,
            M5DocsSourceClass::SourceUnknown => D::NotEvaluated,
        },
    }
}

/// Maps a docs source class and reading posture to the single controlled boundary disposition.
fn disposition_for_posture(
    source: M5DocsSourceClass,
    posture: M5PaneReadingPosture,
) -> M5EmbeddedBoundaryDisposition {
    use M5EmbeddedBoundaryDisposition as D;
    if matches!(source, M5DocsSourceClass::BrowserHandoffRequired) {
        return D::BrowserHandoffOnly;
    }
    match posture {
        M5PaneReadingPosture::LocalReadingSafe => D::LiveFirstPartyLocal,
        M5PaneReadingPosture::HostedReadingSafe => D::LiveFirstPartyHosted,
        M5PaneReadingPosture::MirroredReadingSafe => D::LiveProviderOwned,
        M5PaneReadingPosture::OfflineReadingOnly => D::OfflineSnapshot,
        M5PaneReadingPosture::PostureUnknown => D::NotEvaluated,
    }
}

/// Input to [`resolve_docs_pane_header`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsPaneHeaderResolutionInput {
    /// Stable identity of the header instance.
    pub header_id: String,
    /// The source class of the pane.
    pub source_class: M5DocsSourceClass,
    /// The owner / origin class behind the pane.
    pub owner_class: WebviewOwnerClass,
    /// True when the owner / origin is disclosed on the pane, never menu-only.
    pub owner_disclosed: bool,
    /// The version or pack identity of the source (empty means unstated).
    pub pack_identity: String,
    /// The freshness / last-updated state of the pane.
    pub freshness: M5EmbeddedFreshnessState,
    /// True when the last-updated state is stated on the pane.
    pub last_updated_stated: bool,
    /// Capability limits the pane names relative to native trusted chrome.
    pub capability_limits: Vec<CapabilityLimitClass>,
    /// True when the source contract requires a browser handoff.
    pub handoff_required: bool,
    /// True when an open-externally action is offered on the pane, never menu-only.
    pub open_externally_available: bool,
    /// True when a find-in-page affordance applies to this pane.
    pub find_in_page_applicable: bool,
    /// True when the find-in-page affordance is available.
    pub find_in_page_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe docs-pane header projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDocsPaneHeader {
    /// Stable identity of the header instance.
    pub header_id: String,
    /// The source class token named by the header.
    pub source_class: String,
    /// Single controlled boundary disposition carried by the header.
    pub boundary_disposition: M5EmbeddedBoundaryDisposition,
    /// Owner / origin token named by the header.
    pub owner_origin: String,
    /// Version / pack identity named by the header.
    pub pack_identity: String,
    /// Freshness token named by the header.
    pub freshness: String,
    /// Whether the last-updated state is stated.
    pub last_updated_stated: bool,
    /// Capability-limit tokens named by the header.
    pub capability_limits: Vec<String>,
    /// Whether the source contract requires a browser handoff.
    pub handoff_required: bool,
    /// Whether an open-externally action is offered.
    pub open_externally_available: bool,
    /// Whether a find-in-page affordance applies to this pane.
    pub find_in_page_applicable: bool,
    /// Whether the find-in-page affordance is available.
    pub find_in_page_available: bool,
    /// Degrade reason, if the header could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5DocsPaneHeaderDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DocsBoundaryNextAction,
    /// AC1: whether a user can distinguish the source class from this header alone.
    pub distinguishable_source: bool,
    /// Guardrail (MUST be `false` on a clean header): the source contract requires a handoff that
    /// is not exposed.
    pub hides_required_handoff: bool,
}

impl M5ResolvedDocsPaneHeader {
    /// Whether this header reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_boundary_fact_grid`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BoundaryFactGridResolutionInput {
    /// Stable identity of the grid instance.
    pub grid_id: String,
    /// The source class of the pane the grid describes.
    pub source_class: M5DocsSourceClass,
    /// The data-exit boundary the grid names.
    pub data_exit_boundary: DataExitBoundary,
    /// True when the data boundary is stated on the grid.
    pub data_boundary_stated: bool,
    /// The offline / mirrored reading posture.
    pub reading_posture: M5PaneReadingPosture,
    /// True when the reading posture is stated on the grid.
    pub posture_stated: bool,
    /// True when the grid explains why the pane is trustworthy for in-product reading.
    pub reading_trust_explained: bool,
    /// True when the pane is trustworthy enough for in-product reading.
    pub trustworthy_for_in_product_reading: bool,
    /// True when the grid claims approval / policy authority (a masquerade — AC2 violation).
    pub claims_approval_or_policy_authority: bool,
    /// True when the grid reads as suitable for high-risk approval (a masquerade — AC2 violation).
    pub suitable_for_high_risk_approval: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe boundary-fact grid projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBoundaryFactGrid {
    /// Stable identity of the grid instance.
    pub grid_id: String,
    /// The source class token named by the grid.
    pub source_class: String,
    /// The data-exit boundary token named by the grid.
    pub data_exit_boundary: String,
    /// The reading-posture token named by the grid.
    pub reading_posture: String,
    /// Single controlled boundary disposition carried by the grid.
    pub boundary_disposition: M5EmbeddedBoundaryDisposition,
    /// Whether the data boundary is stated.
    pub data_boundary_stated: bool,
    /// Whether the pane is trustworthy enough for in-product reading.
    pub trustworthy_for_in_product_reading: bool,
    /// Whether the grid reads as suitable for high-risk approval (MUST be `false` on a clean grid).
    pub suitable_for_high_risk_approval: bool,
    /// Degrade reason, if the grid could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5BoundaryFactGridDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DocsBoundaryNextAction,
    /// Guardrail (MUST be `false` on a clean grid): the grid masquerades as an approval / policy
    /// authority.
    pub masquerades_as_approval_authority: bool,
}

impl M5ResolvedBoundaryFactGrid {
    /// Whether this grid reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5DocsBoundaryResolutionError {
    /// The docs-pane header id was empty.
    EmptyHeaderId,
    /// The boundary-fact grid id was empty.
    EmptyGridId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5DocsBoundaryResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyHeaderId => "empty_header_id",
            Self::EmptyGridId => "empty_grid_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5DocsBoundaryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 docs-boundary resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DocsBoundaryResolutionError {}

/// Resolves a docs-pane header, proving AC1: the header names its source class, owner/origin,
/// version/pack identity, and last-updated state, and never reads as a clean pane when the source
/// class is unstated or a required browser handoff is not exposed.
pub fn resolve_docs_pane_header(
    input: M5DocsPaneHeaderResolutionInput,
) -> Result<M5ResolvedDocsPaneHeader, M5DocsBoundaryResolutionError> {
    if input.header_id.trim().is_empty() {
        return Err(M5DocsBoundaryResolutionError::EmptyHeaderId);
    }
    if string_is_forbidden(&input.header_id) || string_is_forbidden(&input.pack_identity) {
        return Err(M5DocsBoundaryResolutionError::ForbiddenMaterial);
    }

    let distinguishable_source = input.source_class.is_distinguishable();
    let hides_required_handoff = input.handoff_required && !input.open_externally_available;

    let degrade_reason = if !distinguishable_source {
        Some(M5DocsPaneHeaderDegradeReason::SourceClassUnstated)
    } else if !input.owner_disclosed {
        Some(M5DocsPaneHeaderDegradeReason::OwnerOrOriginUnstated)
    } else if input.pack_identity.trim().is_empty() {
        Some(M5DocsPaneHeaderDegradeReason::VersionOrPackIdentityMissing)
    } else if !input.last_updated_stated
        || input.freshness == M5EmbeddedFreshnessState::FreshnessUnknown
    {
        Some(M5DocsPaneHeaderDegradeReason::LastUpdatedUnstated)
    } else if hides_required_handoff {
        Some(M5DocsPaneHeaderDegradeReason::HandoffRequiredButNotExposed)
    } else if !input.proof_fresh {
        Some(M5DocsPaneHeaderDegradeReason::ProofStale)
    } else {
        None
    };

    let boundary_disposition = match degrade_reason {
        Some(_) => M5EmbeddedBoundaryDisposition::NotEvaluated,
        None => disposition_for_source(input.source_class, input.freshness),
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None if input.handoff_required => M5DocsBoundaryNextAction::OpenExternally,
        None => M5DocsBoundaryNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedDocsPaneHeader {
        header_id: input.header_id,
        source_class: input.source_class.as_str().to_owned(),
        boundary_disposition,
        owner_origin: input.owner_class.as_str().to_owned(),
        pack_identity: input.pack_identity,
        freshness: input.freshness.as_str().to_owned(),
        last_updated_stated: input.last_updated_stated,
        capability_limits: input
            .capability_limits
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        handoff_required: input.handoff_required,
        open_externally_available: input.open_externally_available,
        find_in_page_applicable: input.find_in_page_applicable,
        find_in_page_available: input.find_in_page_available,
        degrade_reason,
        next_action,
        distinguishable_source,
        hides_required_handoff,
    })
}

/// Resolves a boundary-fact grid, proving AC2: the grid names its data boundary and reading
/// posture, explains why the pane is trustworthy for in-product reading but not high-risk
/// approval, and never masquerades as an approval / policy authority.
pub fn resolve_boundary_fact_grid(
    input: M5BoundaryFactGridResolutionInput,
) -> Result<M5ResolvedBoundaryFactGrid, M5DocsBoundaryResolutionError> {
    if input.grid_id.trim().is_empty() {
        return Err(M5DocsBoundaryResolutionError::EmptyGridId);
    }
    if string_is_forbidden(&input.grid_id) {
        return Err(M5DocsBoundaryResolutionError::ForbiddenMaterial);
    }

    let masquerades_as_approval_authority =
        input.claims_approval_or_policy_authority || input.suitable_for_high_risk_approval;

    let degrade_reason = if !input.data_boundary_stated {
        Some(M5BoundaryFactGridDegradeReason::DataBoundaryUnstated)
    } else if masquerades_as_approval_authority {
        Some(M5BoundaryFactGridDegradeReason::MasqueradesAsApprovalAuthority)
    } else if !input.posture_stated || !input.reading_posture.is_stated() {
        Some(M5BoundaryFactGridDegradeReason::OfflineOrMirroredPostureUnstated)
    } else if !input.reading_trust_explained || !input.trustworthy_for_in_product_reading {
        Some(M5BoundaryFactGridDegradeReason::ReadingTrustNotExplained)
    } else if !input.proof_fresh {
        Some(M5BoundaryFactGridDegradeReason::ProofStale)
    } else {
        None
    };

    let boundary_disposition = match degrade_reason {
        Some(_) => M5EmbeddedBoundaryDisposition::NotEvaluated,
        None => disposition_for_posture(input.source_class, input.reading_posture),
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DocsBoundaryNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedBoundaryFactGrid {
        grid_id: input.grid_id,
        source_class: input.source_class.as_str().to_owned(),
        data_exit_boundary: input.data_exit_boundary.as_str().to_owned(),
        reading_posture: input.reading_posture.as_str().to_owned(),
        boundary_disposition,
        data_boundary_stated: input.data_boundary_stated,
        trustworthy_for_in_product_reading: input.trustworthy_for_in_product_reading,
        suitable_for_high_risk_approval: input.suitable_for_high_risk_approval,
        degrade_reason,
        next_action,
        masquerades_as_approval_authority,
    })
}

/// One controls row: one consumer surface bound to the resolved docs-pane header and boundary-fact
/// grid examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBoundaryControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5DocsBoundaryConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5EmbeddedQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5EmbeddedDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5EmbeddedRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5EmbeddedAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5DocsBoundaryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5DocsBoundaryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    /// Resolved docs-pane header examples.
    pub docs_pane_header_examples: Vec<M5ResolvedDocsPaneHeader>,
    /// Resolved boundary-fact grid examples.
    pub boundary_fact_grid_examples: Vec<M5ResolvedBoundaryFactGrid>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masquerade as native permission or irreversible approval UI.
    pub masquerades_as_native_approval_chrome: bool,
    /// Hard invariant: never hide owner/origin or the browser handoff behind menus only.
    pub hides_owner_origin_or_handoff_in_menus_only: bool,
    /// Hard invariant: never render a stale, offline, or blocked pane as fresh first-party truth.
    pub renders_stale_or_blocked_as_fresh_first_party_truth: bool,
    /// Hard invariant: never embed a high-risk approval without a native step-up.
    pub embeds_high_risk_approval_without_native_step_up: bool,
}

impl M5DocsBoundaryControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DocsBoundaryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DocsBoundaryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DocsBoundaryExportField> =
            self.export_fields.iter().copied().collect();
        M5DocsBoundaryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.masquerades_as_native_approval_chrome
            && !self.hides_owner_origin_or_handoff_in_menus_only
            && !self.renders_stale_or_blocked_as_fresh_first_party_truth
            && !self.embeds_high_risk_approval_without_native_step_up
    }

    /// True when every resolved example on this row is honest: no clean header hides a required
    /// handoff and no clean grid masquerades as an approval authority.
    fn examples_are_honest(&self) -> bool {
        self.docs_pane_header_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.hides_required_handoff))
            && self
                .boundary_fact_grid_examples
                .iter()
                .all(|ex| !(ex.is_clean() && ex.masquerades_as_approval_authority))
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBoundaryVocabularySet {
    /// Boundary-disposition tokens (bound from the frozen matrix).
    pub boundary_dispositions: Vec<String>,
    /// Source-class tokens.
    pub source_classes: Vec<String>,
    /// Reading-posture tokens.
    pub reading_postures: Vec<String>,
    /// Owner-class tokens (bound from the auth-boundary object model).
    pub owner_classes: Vec<String>,
    /// Data-exit-boundary tokens (bound from the public-truth object model).
    pub data_exit_boundaries: Vec<String>,
    /// Capability-limit tokens (bound from the auth-boundary object model).
    pub capability_limits: Vec<String>,
    /// Freshness-state tokens (bound from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Header degrade-reason tokens.
    pub header_degrade_reasons: Vec<String>,
    /// Grid degrade-reason tokens.
    pub grid_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5DocsBoundaryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            boundary_dispositions: tokens(&M5EmbeddedBoundaryDisposition::ALL, |v| v.as_str()),
            source_classes: tokens(&M5DocsSourceClass::ALL, |v| v.as_str()),
            reading_postures: tokens(&M5PaneReadingPosture::ALL, |v| v.as_str()),
            owner_classes: tokens(&WebviewOwnerClass::ALL, |v| v.as_str()),
            data_exit_boundaries: tokens(&BOUND_DATA_EXIT_BOUNDARIES, |v| v.as_str()),
            capability_limits: tokens(&CapabilityLimitClass::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5EmbeddedFreshnessState::ALL, |v| v.as_str()),
            header_degrade_reasons: tokens(&M5DocsPaneHeaderDegradeReason::ALL, |v| v.as_str()),
            grid_degrade_reasons: tokens(&M5BoundaryFactGridDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DocsBoundaryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5DocsBoundaryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DocsBoundaryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EmbeddedConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5DocsBoundaryGovernanceReview {
    /// The docs-pane header always names its source class and owner/origin.
    pub docs_pane_header_names_source_class_and_owner: bool,
    /// The boundary-fact grid always names its data boundary and reading posture.
    pub boundary_fact_grid_names_data_boundary_and_posture: bool,
    /// The source class is always distinguishable or the header degrades.
    pub source_class_always_distinguishable_or_degraded: bool,
    /// Owner and origin are always explicit, never menu-only.
    pub owner_and_origin_always_explicit: bool,
    /// An external handoff is always exposed when the source contract requires it.
    pub external_handoff_exposed_when_required: bool,
    /// No pane masquerades as an approval or policy authority.
    pub no_pane_masquerades_as_approval_authority: bool,
    /// A stale, offline, or blocked pane is never shown as fresh first-party truth.
    pub stale_or_offline_never_shown_as_fresh: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBoundaryConsumerProjection {
    /// Docs / help surfaces consume the shared source-class vocabulary.
    pub docs_surfaces_consume_source_class_vocabulary: bool,
    /// Embedded surfaces consume the shared capability-limit vocabulary.
    pub embedded_surfaces_consume_capability_limit_vocabulary: bool,
    /// Boundary grids consume a single canonical data-boundary source.
    pub boundary_grids_consume_single_data_boundary_source: bool,
    /// Support / export reads a single canonical boundary source.
    pub support_export_reads_single_boundary_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBoundaryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBoundaryReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DocsBoundaryControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsBoundaryControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DocsBoundaryControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsBoundaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsBoundaryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsBoundaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsBoundaryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsBoundaryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 docs-pane-header / boundary-fact-grid controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBoundaryControlsPacket {
    /// Record kind; must equal [`M5_DOCS_BOUNDARY_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DocsBoundaryControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsBoundaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsBoundaryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsBoundaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsBoundaryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsBoundaryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsBoundaryControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5DocsBoundaryControlsPacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_BOUNDARY_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5DocsBoundaryControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_BOUNDARY_CONTROLS_RECORD_KIND {
            violations.push(M5DocsBoundaryControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_VERSION {
            violations.push(M5DocsBoundaryControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsBoundaryControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5DocsBoundaryControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 docs-boundary controls packet serializes"),
        ) {
            violations.push(M5DocsBoundaryControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 docs-boundary controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,header_examples,grid_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .docs_pane_header_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.boundary_fact_grid_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.docs_pane_header_examples.len(),
                row.boundary_fact_grid_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Docs-Pane-Header and Boundary-Fact-Grid Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Source classes: {}\n",
            self.vocabulary_set.source_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Header examples: {} / grid examples: {}\n",
                row.docs_pane_header_examples.len(),
                row.boundary_fact_grid_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5DocsBoundaryControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsBoundaryControlsViolation>),
}

impl fmt::Display for M5DocsBoundaryControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 docs-boundary controls export parse failed: {error}"
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
                    "m5 docs-boundary controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsBoundaryControlsArtifactError {}

/// Validation failures emitted by [`M5DocsBoundaryControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsBoundaryControlsViolation {
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
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (masquerade or hidden handoff).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// AC1 is not proven: clean headers do not cover the four distinguishable source classes, or
    /// no unstated source class degrades.
    Ac1NotProven,
    /// AC2 is not proven: no grid masquerade degrades, or no required-but-unexposed handoff
    /// degrades.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DocsBoundaryControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::Ac1NotProven => "ac1_not_proven",
            Self::Ac2NotProven => "ac2_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_docs_boundary_controls_export(
) -> Result<M5DocsBoundaryControlsPacket, M5DocsBoundaryControlsArtifactError> {
    let packet: M5DocsBoundaryControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-docs-pane-header-boundary-fact-grid-controls-proof/support_export.json"
    )))
    .map_err(M5DocsBoundaryControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsBoundaryControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DocsBoundaryControlsPacket,
    violations: &mut Vec<M5DocsBoundaryControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_REF,
        M5_DOCS_BOUNDARY_CONTROLS_DOC_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_DOCS_PANE_HEADER_SCHEMA_REF,
        M5_BOUNDARY_FACT_GRID_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsBoundaryControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5DocsBoundaryControlsPacket,
    violations: &mut Vec<M5DocsBoundaryControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5DocsBoundaryControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5DocsBoundaryControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DocsBoundaryControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DocsBoundaryControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_DOCS_PANE_HEADER_SCHEMA_REF)
            || !refs.contains(M5_BOUNDARY_FACT_GRID_SCHEMA_REF)
        {
            violations.push(M5DocsBoundaryControlsViolation::ComponentSchemaRefMissing);
        }
        if row.docs_pane_header_examples.is_empty() || row.boundary_fact_grid_examples.is_empty() {
            violations.push(M5DocsBoundaryControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5DocsBoundaryControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5DocsBoundaryControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5DocsBoundaryControlsPacket,
    violations: &mut Vec<M5DocsBoundaryControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.docs_pane_header_names_source_class_and_owner,
        review.boundary_fact_grid_names_data_boundary_and_posture,
        review.source_class_always_distinguishable_or_degraded,
        review.owner_and_origin_always_explicit,
        review.external_handoff_exposed_when_required,
        review.no_pane_masquerades_as_approval_authority,
        review.stale_or_offline_never_shown_as_fresh,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5DocsBoundaryControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsBoundaryControlsPacket,
    violations: &mut Vec<M5DocsBoundaryControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.docs_surfaces_consume_source_class_vocabulary,
        projection.embedded_surfaces_consume_capability_limit_vocabulary,
        projection.boundary_grids_consume_single_data_boundary_source,
        projection.support_export_reads_single_boundary_source,
    ] {
        if !ok {
            violations.push(M5DocsBoundaryControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsBoundaryControlsPacket,
    violations: &mut Vec<M5DocsBoundaryControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsBoundaryControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DocsBoundaryControlsPacket,
    violations: &mut Vec<M5DocsBoundaryControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DocsBoundaryControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5DocsBoundaryControlsPacket,
    violations: &mut Vec<M5DocsBoundaryControlsViolation>,
) {
    let header_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.docs_pane_header_examples.iter())
    };
    let grid_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.boundary_fact_grid_examples.iter())
    };

    // AC1: clean headers cover each of the four distinguishable source classes a user must be able
    // to tell apart, and at least one unstated source class degrades rather than reading clean.
    let clean_source_classes: BTreeSet<&str> = header_examples()
        .filter(|ex| ex.is_clean() && ex.distinguishable_source)
        .map(|ex| ex.source_class.as_str())
        .collect();
    let covers_required_source_classes = [
        M5DocsSourceClass::ProjectLocal,
        M5DocsSourceClass::MirroredVendor,
        M5DocsSourceClass::ExtensionContributed,
        M5DocsSourceClass::BrowserHandoffRequired,
    ]
    .iter()
    .all(|class| clean_source_classes.contains(class.as_str()));
    let unstated_source_degrades = header_examples()
        .any(|ex| ex.degrade_reason == Some(M5DocsPaneHeaderDegradeReason::SourceClassUnstated));
    if !(covers_required_source_classes && unstated_source_degrades) {
        violations.push(M5DocsBoundaryControlsViolation::Ac1NotProven);
    }

    // AC2: at least one grid masquerade degrades and at least one required-but-unexposed handoff
    // degrades, and no clean example masquerades or hides a required handoff.
    let masquerade_degrades = grid_examples().any(|ex| {
        ex.degrade_reason == Some(M5BoundaryFactGridDegradeReason::MasqueradesAsApprovalAuthority)
            && ex.masquerades_as_approval_authority
    });
    let handoff_degrades = header_examples().any(|ex| {
        ex.degrade_reason == Some(M5DocsPaneHeaderDegradeReason::HandoffRequiredButNotExposed)
            && ex.hides_required_handoff
    });
    let no_clean_masquerades =
        grid_examples().all(|ex| !(ex.is_clean() && ex.masquerades_as_approval_authority));
    let no_clean_hides_handoff =
        header_examples().all(|ex| !(ex.is_clean() && ex.hides_required_handoff));
    if !(masquerade_degrades && handoff_degrades && no_clean_masquerades && no_clean_hides_handoff)
    {
        violations.push(M5DocsBoundaryControlsViolation::Ac2NotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
