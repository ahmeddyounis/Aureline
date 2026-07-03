//! One reusable M5 disclosure / history block primitive: current status, affected
//! versions / components, the disclosure or learn-more path, copy-safe reference ids
//! (the Aureline advisory id plus its CVE / GHSA aliases), and the open-doc / open-browser
//! actions rendered with the same model whenever a user or support needs to inspect an
//! advisory's disclosure details and resolved-state history without abandoning product
//! context or losing provenance.
//!
//! Aureline's frozen advisory-component matrix
//! ([`crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`])
//! names the disclosure block as a governed component family and freezes the controlled
//! severity classes, action states, required actions, continuity claims, delivery
//! profiles, mirror-freshness states, disclosure fields, export fields, and accessibility
//! routes an advisory component may use. This module *implements* that disclosure
//! contract as one reusable disclosure / history block so a first-party signed
//! disclosure, a mirrored disclosure, an offline-imported disclosure, an externally
//! linked disclosure, a community postmortem, or an upstream vendor cross-reference reads
//! the same everywhere it surfaces — instead of collapsing into a bare "learn more" link
//! to an external page that hides whether the advisory is resolved, what the copy-safe id
//! is, and where the provenance came from.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_disclosure_block`] — that takes one advisory's disclosure
//!    state on one disclosure-source lane (its copy-safe advisory id, optional CVE / GHSA
//!    aliases, severity, affected object, current status, resolved-versus-active history
//!    state, delivery profile, mirror freshness, disclosure path, provenance, visibility
//!    posture, action state, and local-continuity claim) and produces one
//!    [`M5ResolvedDisclosureHistoryBlock`] that derives the display posture from the
//!    history state (so a resolved / superseded / withdrawn advisory steps down its
//!    visual weight but stays inspectable with current-status truth), derives the handoff
//!    posture from the disclosure-source lane (so mirrored, offline-imported, and
//!    externally linked sources keep their provenance visible and an external handoff
//!    never replaces the in-product disclosure state with a dead-end link), assembles the
//!    copy-safe reference ids, keeps the open-doc / open-browser actions attached, keeps
//!    the block visible, projects the same disclosure truth into every claimed channel,
//!    and emits a copy-safe, export-safe summary. The resolver never hides the current
//!    status, affected versions, or disclosure path behind a detail drawer and never
//!    drops the copy-safe advisory id.
//! 2. A parity matrix — [`M5DisclosureHistoryBlockPacket`] — that binds one row per
//!    claimed disclosure-source lane to the shared block anatomy, the same severity
//!    vocabulary, the same channels, the same disclosure fields, the same export fields,
//!    and the same accessibility routes, so Help/About, update-center, and support-bundle
//!    surfaces render the same disclosure / history block from one shared model.
//!
//! The severity classes ([`M5AdvisorySeverityClass`]), action states
//! ([`M5AdvisoryActionState`]), required actions ([`M5AdvisoryRequiredAction`]),
//! continuity claims ([`M5AdvisoryContinuityClaim`]), delivery profiles
//! ([`M5AdvisoryDeliveryProfile`]), mirror-freshness states
//! ([`M5AdvisoryFreshnessState`]), disclosure fields ([`M5AdvisoryDisclosureField`]),
//! export fields ([`M5AdvisoryExportField`]), accessibility routes
//! ([`M5AdvisoryAccessibilityRoute`]), qualification classes
//! ([`M5AdvisoryQualificationClass`]), and downgrade triggers
//! ([`M5AdvisoryDowngradeTrigger`]) are reused verbatim from the frozen advisory matrix;
//! the resolved-versus-active history state ([`M5DisclosureHistoryState`]) aligns
//! field-for-field with the frozen `entry_class` vocabulary in
//! `schemas/security/advisory_timeline_entry.schema.json`; the shell topology — zones,
//! responsive classes, window classes, and consumer surfaces — is reused from the frozen
//! shell-zone matrix. This module mints new vocabulary only for what the frozen matrix
//! left implicit about the block itself: its disclosure-source lanes, its block anatomy,
//! its channels, its focus behaviors, and the derived display / handoff postures. No M5
//! surface invents a second disclosure / history grammar.
//!
//! Raw hostnames, raw absolute paths, raw exploit payloads, raw signatures, private
//! registry URLs, credentials, and raw disclosure bodies stay outside the support
//! boundary; opaque, export-safe reprs are the only material carried, and the reference
//! ids are copy-safe identifiers, never links.
//!
//! The boundary schema is
//! [`schemas/security/m5-disclosure-history-block.schema.json`](../../../../schemas/security/m5-disclosure-history-block.schema.json)
//! and the contract doc is
//! [`docs/security/m5_disclosure_history_block_primitive_contract.md`](../../../../docs/security/m5_disclosure_history_block_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/security/m5-disclosure-history-block-primitive/`](../../../../fixtures/security/m5-disclosure-history-block-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_disclosure_history_block_primitive_externally_linked_preview_narrowed,
    seeded_m5_disclosure_history_block_primitive_offline_imported_beta_narrowed,
    seeded_m5_disclosure_history_block_primitive_packet,
    M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_PACKET_ID,
};

// The severity classes, action states, required actions, continuity claims, delivery
// profiles, mirror-freshness states, disclosure fields, export fields, accessibility
// routes, qualification classes, and downgrade triggers are frozen once, in the
// advisory-component matrix. This primitive reuses them verbatim so it never invents a
// parallel severity vocabulary or a second disclosure grammar.
pub use crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix::{
    M5AdvisoryAccessibilityRoute, M5AdvisoryActionState, M5AdvisoryContinuityClaim,
    M5AdvisoryDeliveryProfile, M5AdvisoryDisclosureField, M5AdvisoryDowngradeTrigger,
    M5AdvisoryExportField, M5AdvisoryFreshnessState, M5AdvisoryQualificationClass,
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

/// Stable record-kind tag carried by [`M5DisclosureHistoryBlockPacket`].
pub const M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_disclosure_and_history_block_current_status_resolved_downgrade_copy_safe_ids_and_open_doc_open_browser_parity_primitive";

/// Schema version for M5 disclosure-history-block-primitive records.
pub const M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the disclosure-history-block-primitive boundary schema.
pub const M5_DISCLOSURE_HISTORY_BLOCK_SCHEMA_REF: &str =
    "schemas/security/m5-disclosure-history-block.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DISCLOSURE_HISTORY_BLOCK_DOC_REF: &str =
    "docs/security/m5_disclosure_history_block_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_DISCLOSURE_HISTORY_BLOCK_SHELL_ZONE_REF: &str =
    "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen advisory-component matrix this primitive narrows
/// from.
pub const M5_DISCLOSURE_HISTORY_BLOCK_COMPONENT_MATRIX_REF: &str =
    "schemas/security/m5-advisory-component-matrix.schema.json";

/// Repo-relative path of the frozen advisory-identity record this primitive aligns its
/// copy-safe advisory-id / CVE / GHSA reference vocabulary to.
pub const M5_DISCLOSURE_HISTORY_BLOCK_IDENTITY_REF: &str =
    "schemas/security/advisory_identity.schema.json";

/// Repo-relative path of the frozen advisory history / resolution contract this
/// primitive aligns its history-state and resolved-state-downgrade behavior to.
pub const M5_DISCLOSURE_HISTORY_BLOCK_HISTORY_DOC_REF: &str =
    "docs/security/advisory_history_and_resolution_contract.md";

/// Repo-relative path of the frozen postmortem / compensating-control contract this
/// primitive aligns its disclosure / learn-more reference truth to.
pub const M5_DISCLOSURE_HISTORY_BLOCK_POSTMORTEM_DOC_REF: &str =
    "docs/security/postmortem_and_compensating_control_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DISCLOSURE_HISTORY_BLOCK_FIXTURE_DIR: &str =
    "fixtures/security/m5-disclosure-history-block-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DISCLOSURE_HISTORY_BLOCK_ARTIFACT_REF: &str =
    "artifacts/release/m5-disclosure-history-block-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DISCLOSURE_HISTORY_BLOCK_CSV_REF: &str =
    "artifacts/release/m5-disclosure-history-block-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DISCLOSURE_HISTORY_BLOCK_REPORT_REF: &str =
    "artifacts/security/m5-disclosure-history-block-primitive.md";

/// The export fields every disclosure block's support summary must carry so a support
/// bundle reconstructs the disclosure without a screenshot and never silently drops the
/// disclosure visibility or the resolved-versus-active history state.
pub const MANDATORY_EXPORT_FIELDS: [M5AdvisoryExportField; 8] = [
    M5AdvisoryExportField::AdvisoryId,
    M5AdvisoryExportField::Severity,
    M5AdvisoryExportField::ActionState,
    M5AdvisoryExportField::AffectedSurface,
    M5AdvisoryExportField::MitigationState,
    M5AdvisoryExportField::DisclosureVisibility,
    M5AdvisoryExportField::HistoryState,
    M5AdvisoryExportField::ContinuityNote,
];

/// One claimed disclosure-source lane a disclosure / history block can render. These are
/// the provenance sources the goal names — the block keeps the provenance visible whether
/// the disclosure is first-party, mirrored, offline-imported, or externally linked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureSourceLane {
    /// A first-party, signed, in-product disclosure.
    FirstPartySigned,
    /// A disclosure delivered through an approved offline mirror.
    Mirrored,
    /// A disclosure imported from a manual offline bundle.
    OfflineImported,
    /// A disclosure that links out to an external page (open-browser).
    ExternallyLinked,
    /// A community postmortem cross-reference.
    CommunityPostmortem,
    /// An upstream vendor / authority cross-reference.
    VendorCrossReference,
}

impl M5DisclosureSourceLane {
    /// Every disclosure-source lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstPartySigned,
        Self::Mirrored,
        Self::OfflineImported,
        Self::ExternallyLinked,
        Self::CommunityPostmortem,
        Self::VendorCrossReference,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartySigned => "first_party_signed",
            Self::Mirrored => "mirrored",
            Self::OfflineImported => "offline_imported",
            Self::ExternallyLinked => "externally_linked",
            Self::CommunityPostmortem => "community_postmortem",
            Self::VendorCrossReference => "vendor_cross_reference",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FirstPartySigned => "First-Party Signed",
            Self::Mirrored => "Mirrored",
            Self::OfflineImported => "Offline Imported",
            Self::ExternallyLinked => "Externally Linked",
            Self::CommunityPostmortem => "Community Postmortem",
            Self::VendorCrossReference => "Vendor Cross-Reference",
        }
    }

    /// The handoff posture this lane resolves to. First-party disclosures open a bundled
    /// in-product doc; every other lane preserves the provenance of its remote source and
    /// hands off without replacing the in-product state with a dead-end link.
    pub const fn handoff_posture(self) -> M5DisclosureHandoffPosture {
        match self {
            Self::FirstPartySigned => M5DisclosureHandoffPosture::InProductDoc,
            Self::Mirrored => M5DisclosureHandoffPosture::MirrorProvenancePreserved,
            Self::OfflineImported => M5DisclosureHandoffPosture::OfflineImportProvenancePreserved,
            Self::ExternallyLinked | Self::CommunityPostmortem | Self::VendorCrossReference => {
                M5DisclosureHandoffPosture::ExternalBrowserProvenancePreserved
            }
        }
    }
}

/// One anatomy part the shared disclosure / history block surfaces. Every part is
/// mandatory: the whole point of the primitive is that the current status, affected
/// versions / components, the disclosure path, the copy-safe reference ids, the
/// provenance, the resolved-versus-active history state, and the open-doc / open-browser
/// actions are visible inline without opening a secondary detail drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureBlockAnatomyPart {
    /// The current status / current-mitigation state.
    CurrentStatus,
    /// The affected versions / components.
    AffectedVersionsComponents,
    /// The copy-safe reference ids (Aureline id plus CVE / GHSA aliases).
    ReferenceIds,
    /// The disclosure or learn-more path.
    DisclosurePath,
    /// The provenance / source of the disclosure.
    ProvenanceSource,
    /// The resolved-versus-active history state.
    HistoryState,
    /// The open-doc / open-browser actions.
    OpenActions,
}

impl M5DisclosureBlockAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CurrentStatus,
        Self::AffectedVersionsComponents,
        Self::ReferenceIds,
        Self::DisclosurePath,
        Self::ProvenanceSource,
        Self::HistoryState,
        Self::OpenActions,
    ];

    /// The anatomy parts every disclosure block must render inline. All parts are
    /// mandatory — no disclosure truth may hide behind a detail drawer.
    pub const MANDATORY: [Self; 7] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentStatus => "current_status",
            Self::AffectedVersionsComponents => "affected_versions_components",
            Self::ReferenceIds => "reference_ids",
            Self::DisclosurePath => "disclosure_path",
            Self::ProvenanceSource => "provenance_source",
            Self::HistoryState => "history_state",
            Self::OpenActions => "open_actions",
        }
    }
}

/// One channel that renders the shared disclosure / history block. Every block projects
/// the same current status, history state, display posture, and copy-safe reference ids
/// into all three so Help/About, update-center, and support-bundle surfaces describe the
/// same disclosure truth and share one copy-safe id behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureBlockChannel {
    /// The Help / About surface.
    HelpAbout,
    /// The update center.
    UpdateCenter,
    /// A support-bundle export.
    SupportBundle,
}

impl M5DisclosureBlockChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::HelpAbout, Self::UpdateCenter, Self::SupportBundle];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::UpdateCenter => "update_center",
            Self::SupportBundle => "support_bundle",
        }
    }
}

/// A focus / navigation behavior the disclosure block supports so the current status, the
/// copy-safe reference ids, the open-doc / open-browser actions, and the history state
/// stay keyboard-reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureBlockFocusBehavior {
    /// The block is reachable and operable by keyboard focus.
    BlockKeyboardFocusable,
    /// The copy-safe reference ids are keyboard-reachable and copyable.
    ReferenceIdsCopyable,
    /// The open-doc action is keyboard-reachable.
    OpenDocActionReachable,
    /// The open-browser action is keyboard-reachable.
    OpenBrowserActionReachable,
    /// The resolved-versus-active history state is announced to a screen reader, never
    /// color-only.
    HistoryStateAnnouncedToScreenReader,
    /// A stable deep-link anchor jumps to the full disclosure detail.
    DeepLinkToDisclosureDetail,
}

impl M5DisclosureBlockFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BlockKeyboardFocusable,
        Self::ReferenceIdsCopyable,
        Self::OpenDocActionReachable,
        Self::OpenBrowserActionReachable,
        Self::HistoryStateAnnouncedToScreenReader,
        Self::DeepLinkToDisclosureDetail,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockKeyboardFocusable => "block_keyboard_focusable",
            Self::ReferenceIdsCopyable => "reference_ids_copyable",
            Self::OpenDocActionReachable => "open_doc_action_reachable",
            Self::OpenBrowserActionReachable => "open_browser_action_reachable",
            Self::HistoryStateAnnouncedToScreenReader => "history_state_announced_to_screen_reader",
            Self::DeepLinkToDisclosureDetail => "deep_link_to_disclosure_detail",
        }
    }
}

/// The resolved-versus-active history state a disclosure block shows. Aligns
/// field-for-field with the frozen `entry_class` vocabulary in
/// `schemas/security/advisory_timeline_entry.schema.json` so the block reads from one
/// stable history grammar. A resolved, superseded, or withdrawn advisory is downgraded to
/// history (its display posture steps down) but stays inspectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureHistoryState {
    /// A drafted advisory not yet published.
    Draft,
    /// Published in its declared visibility class; active.
    Published,
    /// Mitigation is available / applied but the advisory remains part of the active
    /// response chain.
    Mitigated,
    /// Superseded by another advisory; downgraded to history.
    Superseded,
    /// The response reached mitigation-complete; downgraded to history.
    Resolved,
    /// Explicitly retracted; remains visible as a withdrawn history row.
    Withdrawn,
}

impl M5DisclosureHistoryState {
    /// Every history state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Draft,
        Self::Published,
        Self::Mitigated,
        Self::Superseded,
        Self::Resolved,
        Self::Withdrawn,
    ];

    /// Stable token recorded in worked cases (aligned to `entry_class`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
            Self::Mitigated => "mitigated",
            Self::Superseded => "superseded",
            Self::Resolved => "resolved",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// `true` when the advisory is part of the active response chain — published or
    /// mitigated — and keeps full visual weight.
    pub const fn is_active_response(self) -> bool {
        matches!(self, Self::Published | Self::Mitigated)
    }

    /// `true` when the advisory has been downgraded to history — superseded, resolved, or
    /// withdrawn. These rows step down their visual weight but stay inspectable and never
    /// silently disappear.
    pub const fn is_resolved_history(self) -> bool {
        matches!(self, Self::Superseded | Self::Resolved | Self::Withdrawn)
    }

    /// The display posture this history state resolves to.
    pub const fn display_posture(self) -> M5DisclosureDisplayPosture {
        match self {
            Self::Published | Self::Mitigated => M5DisclosureDisplayPosture::FullWeight,
            Self::Superseded | Self::Resolved | Self::Withdrawn => {
                M5DisclosureDisplayPosture::SteppedDownInspectable
            }
            Self::Draft => M5DisclosureDisplayPosture::DraftRestricted,
        }
    }
}

/// The normalized display posture a disclosure block renders. This is a resolver-side
/// vocabulary. It is derived from the history state so a resolved advisory steps down its
/// visual weight without losing its current-status truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureDisplayPosture {
    /// Full visual weight — the advisory is part of the active response chain.
    FullWeight,
    /// Stepped-down weight but fully inspectable — the advisory is downgraded to history.
    SteppedDownInspectable,
    /// Restricted visibility — the advisory is still a draft.
    DraftRestricted,
}

impl M5DisclosureDisplayPosture {
    /// Every display posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::FullWeight,
        Self::SteppedDownInspectable,
        Self::DraftRestricted,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullWeight => "full_weight",
            Self::SteppedDownInspectable => "stepped_down_inspectable",
            Self::DraftRestricted => "draft_restricted",
        }
    }
}

/// The normalized handoff posture a disclosure block uses when the user opens the
/// disclosure. This is a resolver-side vocabulary derived from the disclosure-source
/// lane. Every posture keeps the provenance visible and preserves the in-product
/// disclosure state; an external handoff never replaces it with a dead-end link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureHandoffPosture {
    /// Opens a bundled in-product doc; no external navigation.
    InProductDoc,
    /// Opens the mirrored disclosure with its mirror provenance preserved.
    MirrorProvenancePreserved,
    /// Opens the offline-imported disclosure with its import provenance preserved.
    OfflineImportProvenancePreserved,
    /// Opens an external browser while preserving provenance and the in-product state.
    ExternalBrowserProvenancePreserved,
}

impl M5DisclosureHandoffPosture {
    /// Every handoff posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InProductDoc,
        Self::MirrorProvenancePreserved,
        Self::OfflineImportProvenancePreserved,
        Self::ExternalBrowserProvenancePreserved,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProductDoc => "in_product_doc",
            Self::MirrorProvenancePreserved => "mirror_provenance_preserved",
            Self::OfflineImportProvenancePreserved => "offline_import_provenance_preserved",
            Self::ExternalBrowserProvenancePreserved => "external_browser_provenance_preserved",
        }
    }

    /// `true` when this posture hands off to a remote / external source (mirror, offline
    /// import, or external browser) rather than a bundled in-product doc — exactly the
    /// postures that must keep provenance visible.
    pub const fn is_remote_source(self) -> bool {
        !matches!(self, Self::InProductDoc)
    }
}

/// The kind of one copy-safe reference id a disclosure block carries. Aligns with the
/// frozen advisory-identity id family so the Aureline advisory id, its CVE alias, and its
/// GHSA alias resolve from one stable id vocabulary without alias drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureReferenceKind {
    /// The stable Aureline advisory id (always present).
    AurelineAdvisoryId,
    /// The CVE alias id.
    CveAlias,
    /// The GHSA alias id.
    GhsaAlias,
}

impl M5DisclosureReferenceKind {
    /// Every reference kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::AurelineAdvisoryId, Self::CveAlias, Self::GhsaAlias];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AurelineAdvisoryId => "aureline_advisory_id",
            Self::CveAlias => "cve_alias",
            Self::GhsaAlias => "ghsa_alias",
        }
    }
}

/// One open action a disclosure block offers. The block always offers the open-doc,
/// open-browser, and copy-reference-ids actions so a user can move to the full disclosure
/// or copy the ids without abandoning product context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureOpenAction {
    /// Open the bundled in-product disclosure doc.
    OpenInProductDoc,
    /// Open the external disclosure page in a browser.
    OpenExternalBrowser,
    /// Copy the copy-safe reference ids.
    CopyReferenceIds,
}

impl M5DisclosureOpenAction {
    /// Every open action, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::OpenInProductDoc,
        Self::OpenExternalBrowser,
        Self::CopyReferenceIds,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInProductDoc => "open_in_product_doc",
            Self::OpenExternalBrowser => "open_external_browser",
            Self::CopyReferenceIds => "copy_reference_ids",
        }
    }
}

/// One copy-safe reference id carried by a resolved disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureReferenceId {
    /// The reference kind.
    pub kind: M5DisclosureReferenceKind,
    /// The copy-safe id value (never a link).
    pub value: String,
}

/// The full input to the disclosure-block resolver for one advisory's disclosure state on
/// one disclosure-source lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureBlockResolutionInput {
    /// The disclosure-source lane this block renders on.
    pub source_lane: M5DisclosureSourceLane,
    /// The copy-safe Aureline advisory id.
    pub advisory_id: String,
    /// The copy-safe CVE alias id, or empty when no CVE has been assigned.
    pub cve_alias: String,
    /// The copy-safe GHSA alias id, or empty when no GHSA has been minted.
    pub ghsa_alias: String,
    /// The advisory's severity.
    pub severity: M5AdvisorySeverityClass,
    /// Opaque, export-safe representation of the affected versions / components.
    pub affected_object_repr: String,
    /// Opaque, export-safe representation of the current status / current-mitigation
    /// state.
    pub current_status_repr: String,
    /// The resolved-versus-active history state.
    pub history_state: M5DisclosureHistoryState,
    /// The delivery profile of this lane.
    pub delivery_profile: M5AdvisoryDeliveryProfile,
    /// The mirror / distribution freshness of this lane.
    pub mirror_freshness: M5AdvisoryFreshnessState,
    /// Opaque, export-safe representation of the disclosure / learn-more path.
    pub disclosure_path_repr: String,
    /// Opaque, export-safe representation of the provenance / source.
    pub provenance_repr: String,
    /// Opaque, export-safe representation of the disclosure visibility posture.
    pub visibility_posture_repr: String,
    /// The action state this disclosure carries.
    pub action_state: M5AdvisoryActionState,
    /// The local-continuity claim this disclosure makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
}

impl M5DisclosureBlockResolutionInput {
    /// True when any representation carries forbidden material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.advisory_id)
            || repr_is_forbidden(&self.cve_alias)
            || repr_is_forbidden(&self.ghsa_alias)
            || repr_is_forbidden(&self.affected_object_repr)
            || repr_is_forbidden(&self.current_status_repr)
            || repr_is_forbidden(&self.disclosure_path_repr)
            || repr_is_forbidden(&self.provenance_repr)
            || repr_is_forbidden(&self.visibility_posture_repr)
    }

    /// The copy-safe reference ids for this disclosure, in a stable order. The Aureline
    /// advisory id is always present; the CVE and GHSA aliases are added when non-empty.
    fn reference_ids(&self) -> Vec<M5DisclosureReferenceId> {
        let mut ids = vec![M5DisclosureReferenceId {
            kind: M5DisclosureReferenceKind::AurelineAdvisoryId,
            value: self.advisory_id.clone(),
        }];
        if !self.cve_alias.trim().is_empty() {
            ids.push(M5DisclosureReferenceId {
                kind: M5DisclosureReferenceKind::CveAlias,
                value: self.cve_alias.clone(),
            });
        }
        if !self.ghsa_alias.trim().is_empty() {
            ids.push(M5DisclosureReferenceId {
                kind: M5DisclosureReferenceKind::GhsaAlias,
                value: self.ghsa_alias.clone(),
            });
        }
        ids
    }
}

/// One channel projection of a resolved disclosure block. Every projection carries the
/// same core truth — history state, display posture, severity, primary reference id, and
/// handoff posture — so the channels stay in parity; only the channel-scoped headline
/// framing differs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDisclosureChannelProjection {
    /// The channel this projection renders on.
    pub channel: M5DisclosureBlockChannel,
    /// The channel-scoped headline (built from the shared disclosure truth).
    pub headline: String,
    /// The history state (identical across channels).
    pub history_state: M5DisclosureHistoryState,
    /// The display posture (identical across channels).
    pub display_posture: M5DisclosureDisplayPosture,
    /// The severity (identical across channels).
    pub severity: M5AdvisorySeverityClass,
    /// The primary (Aureline) copy-safe reference id (identical across channels).
    pub primary_reference_id: String,
    /// The handoff posture (identical across channels).
    pub handoff_posture: M5DisclosureHandoffPosture,
}

/// One export column of the copy-safe disclosure summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureExportColumn {
    /// The export field.
    pub field: M5AdvisoryExportField,
    /// The export-safe value.
    pub value: String,
}

/// The copy-safe, export-safe summary of a resolved disclosure block, for support flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureExportSummary {
    /// The copy-safe advisory id.
    pub advisory_id: String,
    /// The mandatory export columns, in [`MANDATORY_EXPORT_FIELDS`] order.
    pub columns: Vec<M5DisclosureExportColumn>,
}

/// The resolved disclosure / history block for one advisory on one disclosure-source lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDisclosureHistoryBlock {
    /// The disclosure-source lane this block renders on.
    pub source_lane: M5DisclosureSourceLane,
    /// The copy-safe Aureline advisory id.
    pub advisory_id: String,
    /// The copy-safe CVE alias id (empty when none).
    pub cve_alias: String,
    /// The copy-safe GHSA alias id (empty when none).
    pub ghsa_alias: String,
    /// The advisory's severity.
    pub severity: M5AdvisorySeverityClass,
    /// The opaque affected-versions / components representation.
    pub affected_object_repr: String,
    /// The opaque current-status representation.
    pub current_status_repr: String,
    /// The resolved-versus-active history state.
    pub history_state: M5DisclosureHistoryState,
    /// The derived display posture.
    pub display_posture: M5DisclosureDisplayPosture,
    /// The delivery profile of this lane.
    pub delivery_profile: M5AdvisoryDeliveryProfile,
    /// The mirror / distribution freshness of this lane.
    pub mirror_freshness: M5AdvisoryFreshnessState,
    /// The derived handoff posture.
    pub handoff_posture: M5DisclosureHandoffPosture,
    /// The opaque disclosure / learn-more path representation.
    pub disclosure_path_repr: String,
    /// The opaque provenance / source representation.
    pub provenance_repr: String,
    /// The opaque disclosure visibility posture representation.
    pub visibility_posture_repr: String,
    /// The action state this disclosure carries.
    pub action_state: M5AdvisoryActionState,
    /// The local-continuity claim this disclosure makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
    /// The copy-safe reference ids (Aureline id plus CVE / GHSA aliases).
    pub reference_ids: Vec<M5DisclosureReferenceId>,
    /// The open-doc / open-browser / copy-ids actions attached to the block.
    pub open_actions: Vec<M5DisclosureOpenAction>,
    /// True when the advisory is downgraded to history (superseded / resolved / withdrawn).
    pub is_resolved_history: bool,
    /// True — the disclosure state lives in the product, not only on an external page.
    pub disclosure_state_in_product: bool,
    /// True — a resolved advisory steps down but remains inspectable.
    pub remains_inspectable: bool,
    /// True — the current status stays visible, even for a resolved advisory.
    pub current_status_visible: bool,
    /// True — the provenance stays visible when the source is mirrored / offline / external.
    pub provenance_visible: bool,
    /// True — the reference ids are copy-safe (ids, never links).
    pub reference_ids_copy_safe: bool,
    /// True — an external handoff preserves the in-product disclosure state.
    pub preserves_in_product_state_on_handoff: bool,
    /// False — the block never replaces the in-product disclosure state with a dead-end
    /// link.
    pub is_dead_end_link: bool,
    /// True — the primitive always keeps the disclosure block visible.
    pub remains_visible: bool,
    /// The same disclosure truth projected into every channel.
    pub channel_projections: Vec<M5ResolvedDisclosureChannelProjection>,
    /// The copy-safe, export-safe summary.
    pub export_summary: M5DisclosureExportSummary,
}

/// Errors returned by [`resolve_disclosure_block`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DisclosureBlockResolutionError {
    /// The advisory id was empty.
    EmptyAdvisoryId,
    /// The affected-object representation was empty.
    EmptyAffectedObject,
    /// The current-status representation was empty.
    EmptyCurrentStatus,
    /// The disclosure-path representation was empty.
    EmptyDisclosurePath,
    /// The provenance representation was empty.
    EmptyProvenance,
    /// The visibility-posture representation was empty.
    EmptyVisibilityPosture,
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5DisclosureBlockResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAdvisoryId => "empty_advisory_id",
            Self::EmptyAffectedObject => "empty_affected_object",
            Self::EmptyCurrentStatus => "empty_current_status",
            Self::EmptyDisclosurePath => "empty_disclosure_path",
            Self::EmptyProvenance => "empty_provenance",
            Self::EmptyVisibilityPosture => "empty_visibility_posture",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5DisclosureBlockResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "disclosure-block resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DisclosureBlockResolutionError {}

/// Resolves one advisory's disclosure state into one disclosure / history block.
///
/// The resolver derives the display posture from the history state (so a resolved /
/// superseded / withdrawn advisory steps down its visual weight but stays inspectable
/// with current-status truth), derives the handoff posture from the disclosure-source
/// lane (so mirrored, offline-imported, and externally linked sources keep provenance
/// visible and never hand off to a dead-end link), assembles the copy-safe reference ids,
/// keeps the open-doc / open-browser actions attached, keeps the block visible, projects
/// the same disclosure truth into every channel, and emits a copy-safe, export-safe
/// summary. It never hides the current status, affected versions, or disclosure path
/// behind a detail drawer and never drops the copy-safe advisory id.
pub fn resolve_disclosure_block(
    input: &M5DisclosureBlockResolutionInput,
) -> Result<M5ResolvedDisclosureHistoryBlock, M5DisclosureBlockResolutionError> {
    if input.advisory_id.trim().is_empty() {
        return Err(M5DisclosureBlockResolutionError::EmptyAdvisoryId);
    }
    if input.affected_object_repr.trim().is_empty() {
        return Err(M5DisclosureBlockResolutionError::EmptyAffectedObject);
    }
    if input.current_status_repr.trim().is_empty() {
        return Err(M5DisclosureBlockResolutionError::EmptyCurrentStatus);
    }
    if input.disclosure_path_repr.trim().is_empty() {
        return Err(M5DisclosureBlockResolutionError::EmptyDisclosurePath);
    }
    if input.provenance_repr.trim().is_empty() {
        return Err(M5DisclosureBlockResolutionError::EmptyProvenance);
    }
    if input.visibility_posture_repr.trim().is_empty() {
        return Err(M5DisclosureBlockResolutionError::EmptyVisibilityPosture);
    }
    if input.carries_forbidden_material() {
        return Err(M5DisclosureBlockResolutionError::ForbiddenMaterial);
    }

    let display_posture = input.history_state.display_posture();
    let handoff_posture = input.source_lane.handoff_posture();
    let is_resolved_history = input.history_state.is_resolved_history();
    let reference_ids = input.reference_ids();
    // Every disclosure block offers the open-doc, open-browser, and copy-ids actions so a
    // user can move to the full disclosure or copy the ids without abandoning context.
    let open_actions = M5DisclosureOpenAction::ALL.to_vec();

    let channel_projections = M5DisclosureBlockChannel::ALL
        .iter()
        .map(|channel| M5ResolvedDisclosureChannelProjection {
            channel: *channel,
            headline: render_channel_headline(*channel, input, display_posture),
            history_state: input.history_state,
            display_posture,
            severity: input.severity,
            primary_reference_id: input.advisory_id.clone(),
            handoff_posture,
        })
        .collect();

    let export_summary = build_export_summary(input);

    Ok(M5ResolvedDisclosureHistoryBlock {
        source_lane: input.source_lane,
        advisory_id: input.advisory_id.clone(),
        cve_alias: input.cve_alias.clone(),
        ghsa_alias: input.ghsa_alias.clone(),
        severity: input.severity,
        affected_object_repr: input.affected_object_repr.clone(),
        current_status_repr: input.current_status_repr.clone(),
        history_state: input.history_state,
        display_posture,
        delivery_profile: input.delivery_profile,
        mirror_freshness: input.mirror_freshness,
        handoff_posture,
        disclosure_path_repr: input.disclosure_path_repr.clone(),
        provenance_repr: input.provenance_repr.clone(),
        visibility_posture_repr: input.visibility_posture_repr.clone(),
        action_state: input.action_state,
        continuity_claim: input.continuity_claim,
        reference_ids,
        open_actions,
        is_resolved_history,
        // The disclosure state lives in the product; the block is never only an external
        // page.
        disclosure_state_in_product: true,
        // A resolved advisory steps down but stays inspectable with current-status truth.
        remains_inspectable: true,
        current_status_visible: true,
        // The provenance stays visible whether the source is mirrored, offline, or
        // externally linked.
        provenance_visible: true,
        // The reference ids are copy-safe identifiers, never links.
        reference_ids_copy_safe: true,
        // An external handoff preserves the in-product disclosure state and is never a
        // dead-end link.
        preserves_in_product_state_on_handoff: true,
        is_dead_end_link: false,
        // The primitive structurally keeps the disclosure block visible.
        remains_visible: true,
        channel_projections,
        export_summary,
    })
}

/// Renders one channel-scoped headline from the shared disclosure truth. Every channel
/// carries the same history state, display posture, severity, and reference id; only the
/// channel prefix differs.
fn render_channel_headline(
    channel: M5DisclosureBlockChannel,
    input: &M5DisclosureBlockResolutionInput,
    display_posture: M5DisclosureDisplayPosture,
) -> String {
    format!(
        "[{}] {} · {} · {} · {} · source: {}",
        channel.as_str(),
        input.advisory_id,
        input.history_state.as_str(),
        display_posture.as_str(),
        input.severity.as_str(),
        input.source_lane.as_str(),
    )
}

/// Builds the copy-safe, export-safe summary from the shared disclosure truth.
fn build_export_summary(input: &M5DisclosureBlockResolutionInput) -> M5DisclosureExportSummary {
    let columns = MANDATORY_EXPORT_FIELDS
        .iter()
        .map(|field| M5DisclosureExportColumn {
            field: *field,
            value: export_value(*field, input),
        })
        .collect();
    M5DisclosureExportSummary {
        advisory_id: input.advisory_id.clone(),
        columns,
    }
}

/// Resolves the export-safe value for one export field.
fn export_value(field: M5AdvisoryExportField, input: &M5DisclosureBlockResolutionInput) -> String {
    match field {
        M5AdvisoryExportField::AdvisoryId => input.advisory_id.clone(),
        M5AdvisoryExportField::Severity => input.severity.as_str().to_owned(),
        M5AdvisoryExportField::ActionState => input.action_state.as_str().to_owned(),
        M5AdvisoryExportField::AffectedSurface => input.affected_object_repr.clone(),
        M5AdvisoryExportField::MitigationState => input.current_status_repr.clone(),
        M5AdvisoryExportField::DisclosureVisibility => input.visibility_posture_repr.clone(),
        M5AdvisoryExportField::HistoryState => input.history_state.as_str().to_owned(),
        M5AdvisoryExportField::ContinuityNote => input.continuity_claim.as_str().to_owned(),
        // Only the mandatory-export fields are projected into the summary; any other
        // field resolves to its stable token so the mapping stays total.
        other => other.as_str().to_owned(),
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs disclosure truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureResolutionCase {
    /// The resolver input.
    pub input: M5DisclosureBlockResolutionInput,
    /// The resolved disclosure block. Must equal `resolve_disclosure_block(&input)`.
    pub resolved: M5ResolvedDisclosureHistoryBlock,
}

impl M5DisclosureResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DisclosureBlockResolutionInput) -> Self {
        let resolved = resolve_disclosure_block(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_disclosure_block(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one disclosure-source lane bound to the shared block
/// anatomy, severity vocabulary, channels, disclosure fields, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureSourceRow {
    /// Disclosure-source lane.
    pub source_lane: M5DisclosureSourceLane,
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
    pub anatomy_parts: Vec<M5DisclosureBlockAnatomyPart>,
    /// Severity classes this row can show.
    pub severity_classes: Vec<M5AdvisorySeverityClass>,
    /// Channels this row projects into (must include every channel — parity).
    pub channels: Vec<M5DisclosureBlockChannel>,
    /// Action states this row projects.
    pub action_states: Vec<M5AdvisoryActionState>,
    /// Required actions this row can reference.
    pub required_actions: Vec<M5AdvisoryRequiredAction>,
    /// Local-continuity claims this row makes.
    pub continuity_claims: Vec<M5AdvisoryContinuityClaim>,
    /// Delivery profiles this row can carry.
    pub delivery_profiles: Vec<M5AdvisoryDeliveryProfile>,
    /// Mirror-freshness states this row can carry.
    pub freshness_states: Vec<M5AdvisoryFreshnessState>,
    /// Disclosure fields this row carries (must include every disclosure field).
    pub disclosure_fields: Vec<M5AdvisoryDisclosureField>,
    /// History states this row can render (must include every history state).
    pub history_states: Vec<M5DisclosureHistoryState>,
    /// Focus behaviors this row supports.
    pub focus_behaviors: Vec<M5DisclosureBlockFocusBehavior>,
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
    pub example_disclosures: Vec<M5DisclosureResolutionCase>,
    /// Hard invariant: this row never flattens the disclosure into a bare external link.
    /// MUST be `false`.
    pub flattens_disclosure_into_external_link: bool,
    /// Hard invariant: this row never hides disclosure truth behind a detail drawer. MUST
    /// be `false`.
    pub hides_field_behind_detail_drawer: bool,
    /// Hard invariant: this row never drops a resolved advisory out of inspectable
    /// history. MUST be `false`.
    pub drops_resolved_history_from_inspection: bool,
    /// Hard invariant: this row never hides provenance when the source is mirrored,
    /// offline, or external. MUST be `false`.
    pub hides_provenance_when_mirrored_or_external: bool,
    /// Hard invariant: this row never drops the copy-safe id or export summary. MUST be
    /// `false`.
    pub drops_copy_safe_id_or_export: bool,
}

impl M5DisclosureSourceRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DisclosureBlockAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DisclosureBlockAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every channel (all three projected in parity).
    fn declares_all_channels(&self) -> bool {
        let present: BTreeSet<M5DisclosureBlockChannel> = self.channels.iter().copied().collect();
        M5DisclosureBlockChannel::ALL
            .iter()
            .all(|channel| present.contains(channel))
    }

    /// True when the row declares every disclosure field.
    fn declares_all_disclosure_fields(&self) -> bool {
        let present: BTreeSet<M5AdvisoryDisclosureField> =
            self.disclosure_fields.iter().copied().collect();
        M5AdvisoryDisclosureField::ALL
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every history state.
    fn declares_all_history_states(&self) -> bool {
        let present: BTreeSet<M5DisclosureHistoryState> =
            self.history_states.iter().copied().collect();
        M5DisclosureHistoryState::ALL
            .iter()
            .all(|state| present.contains(state))
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
        !self.flattens_disclosure_into_external_link
            && !self.hides_field_behind_detail_drawer
            && !self.drops_resolved_history_from_inspection
            && !self.hides_provenance_when_mirrored_or_external
            && !self.drops_copy_safe_id_or_export
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureBlockVocabularySet {
    /// Disclosure-source-lane tokens.
    pub source_lanes: Vec<String>,
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
    /// Disclosure-field tokens (reused from the frozen matrix).
    pub disclosure_fields: Vec<String>,
    /// History-state tokens.
    pub history_states: Vec<String>,
    /// Display-posture tokens.
    pub display_postures: Vec<String>,
    /// Handoff-posture tokens.
    pub handoff_postures: Vec<String>,
    /// Reference-kind tokens.
    pub reference_kinds: Vec<String>,
    /// Open-action tokens.
    pub open_actions: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Export-field tokens (reused from the frozen matrix).
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5DisclosureBlockVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            source_lanes: tokens(&M5DisclosureSourceLane::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DisclosureBlockAnatomyPart::ALL, |v| v.as_str()),
            severity_classes: tokens(&M5AdvisorySeverityClass::ALL, |v| v.as_str()),
            action_states: tokens(&M5AdvisoryActionState::ALL, |v| v.as_str()),
            required_actions: tokens(&M5AdvisoryRequiredAction::ALL, |v| v.as_str()),
            continuity_claims: tokens(&M5AdvisoryContinuityClaim::ALL, |v| v.as_str()),
            delivery_profiles: tokens(&M5AdvisoryDeliveryProfile::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5AdvisoryFreshnessState::ALL, |v| v.as_str()),
            disclosure_fields: tokens(&M5AdvisoryDisclosureField::ALL, |v| v.as_str()),
            history_states: tokens(&M5DisclosureHistoryState::ALL, |v| v.as_str()),
            display_postures: tokens(&M5DisclosureDisplayPosture::ALL, |v| v.as_str()),
            handoff_postures: tokens(&M5DisclosureHandoffPosture::ALL, |v| v.as_str()),
            reference_kinds: tokens(&M5DisclosureReferenceKind::ALL, |v| v.as_str()),
            open_actions: tokens(&M5DisclosureOpenAction::ALL, |v| v.as_str()),
            channels: tokens(&M5DisclosureBlockChannel::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5DisclosureBlockFocusBehavior::ALL, |v| v.as_str()),
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
pub struct M5DisclosureBlockGovernanceReview {
    /// One disclosure / history block model is reused across every disclosure-source lane.
    pub one_block_model_across_source_lanes: bool,
    /// Current status, affected versions, and the disclosure path are visible without a
    /// secondary detail drawer.
    pub current_status_versions_path_visible_without_drawer: bool,
    /// The reference ids are copy-safe identifiers, never links.
    pub reference_ids_are_copy_safe: bool,
    /// The open-doc and open-browser actions are present on the block.
    pub open_doc_and_open_browser_actions_present: bool,
    /// A resolved advisory steps down its visual weight but remains inspectable.
    pub resolved_advisories_step_down_but_remain_inspectable: bool,
    /// The provenance stays visible when the source is mirrored, offline, or external.
    pub provenance_visible_when_mirrored_offline_or_external: bool,
    /// An external handoff preserves the in-product state instead of a dead-end link.
    pub external_handoff_preserves_in_product_state: bool,
    /// The copy-safe advisory id is always preserved.
    pub copy_safe_advisory_id_preserved: bool,
    /// The export summary reconstructs disclosure truth for support.
    pub export_summary_reconstructs_disclosure_truth: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 lanes cannot invent parallel disclosure vocabulary.
    pub later_lanes_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureBlockConsumerProjection {
    /// Help/About renders the shared disclosure block.
    pub help_about_renders_shared_block: bool,
    /// The update center renders the shared disclosure block.
    pub update_center_renders_shared_block: bool,
    /// The support bundle renders the shared disclosure block.
    pub support_bundle_renders_shared_block: bool,
    /// The history view reads a single canonical disclosure source.
    pub history_view_reads_single_source: bool,
    /// The resolver reads a single canonical disclosure vocabulary.
    pub resolver_reads_single_disclosure_vocabulary: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureBlockProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the disclosure-history-block primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureBlockReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting disclosure audit.
    pub disclosure_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DisclosureHistoryBlockPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DisclosureHistoryBlockPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Disclosure-source rows.
    pub source_rows: Vec<M5DisclosureSourceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DisclosureBlockVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DisclosureBlockGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DisclosureBlockConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DisclosureBlockProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DisclosureBlockReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 disclosure-history-block-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisclosureHistoryBlockPacket {
    /// Record kind; must equal [`M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Disclosure-source rows.
    pub source_rows: Vec<M5DisclosureSourceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DisclosureBlockVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DisclosureBlockGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DisclosureBlockConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DisclosureBlockProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DisclosureBlockReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DisclosureHistoryBlockPacket {
    /// Builds an M5 disclosure-history-block-primitive packet from stable-lane input.
    pub fn new(input: M5DisclosureHistoryBlockPacketInput) -> Self {
        Self {
            record_kind: M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            source_rows: input.source_rows,
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

    /// Validates the M5 disclosure-history-block-primitive invariants.
    pub fn validate(&self) -> Vec<M5DisclosureHistoryBlockViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_RECORD_KIND {
            violations.push(M5DisclosureHistoryBlockViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5DisclosureHistoryBlockViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DisclosureHistoryBlockViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_source_rows(self, &mut violations);
        validate_shared_primitive_parity(self, &mut violations);
        validate_resolved_step_down(self, &mut violations);
        validate_provenance_handoff(self, &mut violations);
        validate_history_state_coverage(self, &mut violations);
        validate_severity_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 disclosure history block packet serializes"),
        ) {
            violations.push(M5DisclosureHistoryBlockViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 disclosure history block packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per disclosure-source lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "source_lane,qualification,owner,shell_zone_slot,severity_classes,channels,anatomy_parts,history_states,disclosure_fields,export_fields,accessibility_routes,example_count\n",
        );
        for row in &self.source_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.source_lane.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.severity_classes, |v| v.as_str()),
                join_tokens(&row.channels, |v| v.as_str()),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.history_states, |v| v.as_str()),
                join_tokens(&row.disclosure_fields, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                join_tokens(&row.accessibility_routes, |v| v.as_str()),
                row.example_disclosures.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .source_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Disclosure / History Block Primitive: Current Status, Resolved-State Downgrade, Copy-Safe CVE / GHSA IDs, and Open-Doc / Open-Browser Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Disclosure-source lanes: {} ({} stable)\n",
            self.source_rows.len(),
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
            "- History states: {}\n",
            self.vocabulary_set.history_states.join(", ")
        ));
        out.push_str(&format!(
            "- Display postures: {}\n",
            self.vocabulary_set.display_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Export fields: {}\n",
            self.vocabulary_set.export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Disclosure-source lanes\n\n");
        for row in &self.source_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.source_lane.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked disclosures: {}\n",
                row.example_disclosures.len()
            ));
            for case in &row.example_disclosures {
                out.push_str(&format!(
                    "    - `{}` — {} ({}), posture `{}`, handoff `{}`\n",
                    case.resolved.advisory_id,
                    case.resolved.severity.as_str(),
                    case.resolved.history_state.as_str(),
                    case.resolved.display_posture.as_str(),
                    case.resolved.handoff_posture.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 disclosure-history-block-primitive
/// export.
#[derive(Debug)]
pub enum M5DisclosureHistoryBlockArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DisclosureHistoryBlockViolation>),
}

impl fmt::Display for M5DisclosureHistoryBlockArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 disclosure history block export parse failed: {error}"
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
                    "m5 disclosure history block export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DisclosureHistoryBlockArtifactError {}

/// Validation failures emitted by [`M5DisclosureHistoryBlockPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DisclosureHistoryBlockViolation {
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
    /// A required disclosure-source lane is missing from the matrix.
    RequiredSourceLaneMissing,
    /// A disclosure-source row is incomplete.
    SourceRowIncomplete,
    /// A disclosure-source row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A disclosure-source row declares no severity classes.
    SeverityClassMissing,
    /// A disclosure-source row does not declare every channel (channel parity broken).
    ChannelParityMismatch,
    /// A disclosure-source row declares no action states.
    ActionStateMissing,
    /// A disclosure-source row declares no required actions.
    RequiredActionMissing,
    /// A disclosure-source row declares no continuity claims.
    ContinuityClaimMissing,
    /// A disclosure-source row declares no delivery profiles.
    DeliveryProfileMissing,
    /// A disclosure-source row declares no mirror-freshness states.
    FreshnessStateMissing,
    /// A disclosure-source row does not declare every disclosure field.
    DisclosureFieldMissing,
    /// A disclosure-source row does not declare every history state.
    HistoryStateMissing,
    /// A disclosure-source row declares no focus behaviors.
    FocusBehaviorMissing,
    /// A disclosure-source row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A disclosure-source row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A disclosure-source row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A disclosure-source row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A disclosure-source row declares no worked resolution cases.
    ExampleDisclosureMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleDisclosureDrift,
    /// A lane claiming Stable is missing required proof packet refs.
    StableLaneMissingProof,
    /// The worked resolutions do not prove the shared primitive parity and copy-safe id
    /// behavior across every channel.
    SharedPrimitiveParityUnproven,
    /// No worked resolution proves a resolved advisory steps down but remains inspectable.
    ResolvedStepDownUnproven,
    /// The worked resolutions do not prove an external handoff preserves provenance
    /// without a dead-end link.
    ProvenanceHandoffUnproven,
    /// No worked resolution across the matrix exercises every history state.
    HistoryStateCoverageUnproven,
    /// No worked resolution across the matrix exercises every severity class.
    SeverityCoverageUnproven,
    /// A disclosure-source row violates a hard invariant.
    SourceInvariantViolated,
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

impl M5DisclosureHistoryBlockViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSourceLaneMissing => "required_source_lane_missing",
            Self::SourceRowIncomplete => "source_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::SeverityClassMissing => "severity_class_missing",
            Self::ChannelParityMismatch => "channel_parity_mismatch",
            Self::ActionStateMissing => "action_state_missing",
            Self::RequiredActionMissing => "required_action_missing",
            Self::ContinuityClaimMissing => "continuity_claim_missing",
            Self::DeliveryProfileMissing => "delivery_profile_missing",
            Self::FreshnessStateMissing => "freshness_state_missing",
            Self::DisclosureFieldMissing => "disclosure_field_missing",
            Self::HistoryStateMissing => "history_state_missing",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleDisclosureMissing => "example_disclosure_missing",
            Self::ExampleDisclosureDrift => "example_disclosure_drift",
            Self::StableLaneMissingProof => "stable_lane_missing_proof",
            Self::SharedPrimitiveParityUnproven => "shared_primitive_parity_unproven",
            Self::ResolvedStepDownUnproven => "resolved_step_down_unproven",
            Self::ProvenanceHandoffUnproven => "provenance_handoff_unproven",
            Self::HistoryStateCoverageUnproven => "history_state_coverage_unproven",
            Self::SeverityCoverageUnproven => "severity_coverage_unproven",
            Self::SourceInvariantViolated => "source_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 disclosure-history-block-primitive export.
pub fn current_stable_m5_disclosure_history_block_primitive_export(
) -> Result<M5DisclosureHistoryBlockPacket, M5DisclosureHistoryBlockArtifactError> {
    let packet: M5DisclosureHistoryBlockPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-disclosure-history-block-proof/support_export.json"
    )))
    .map_err(M5DisclosureHistoryBlockArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DisclosureHistoryBlockArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DISCLOSURE_HISTORY_BLOCK_SCHEMA_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_DOC_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_SHELL_ZONE_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_COMPONENT_MATRIX_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_IDENTITY_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_HISTORY_DOC_REF,
        M5_DISCLOSURE_HISTORY_BLOCK_POSTMORTEM_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DisclosureHistoryBlockViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DisclosureHistoryBlockViolation::VocabularySetDrift);
    }
}

fn validate_source_rows(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let present: BTreeSet<M5DisclosureSourceLane> = packet
        .source_rows
        .iter()
        .map(|row| row.source_lane)
        .collect();
    for required in M5DisclosureSourceLane::ALL {
        if !present.contains(&required) {
            violations.push(M5DisclosureHistoryBlockViolation::RequiredSourceLaneMissing);
            return;
        }
    }

    for row in &packet.source_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5DisclosureHistoryBlockViolation::SourceRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DisclosureHistoryBlockViolation::MandatoryAnatomyMissing);
        }
        if row.severity_classes.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::SeverityClassMissing);
        }
        if !row.declares_all_channels() {
            violations.push(M5DisclosureHistoryBlockViolation::ChannelParityMismatch);
        }
        if row.action_states.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::ActionStateMissing);
        }
        if row.required_actions.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::RequiredActionMissing);
        }
        if row.continuity_claims.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::ContinuityClaimMissing);
        }
        if row.delivery_profiles.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::DeliveryProfileMissing);
        }
        if row.freshness_states.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::FreshnessStateMissing);
        }
        if !row.declares_all_disclosure_fields() {
            violations.push(M5DisclosureHistoryBlockViolation::DisclosureFieldMissing);
        }
        if !row.declares_all_history_states() {
            violations.push(M5DisclosureHistoryBlockViolation::HistoryStateMissing);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::FocusBehaviorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DisclosureHistoryBlockViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5DisclosureHistoryBlockViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::DowngradeTriggersMissing);
        }
        if row.example_disclosures.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::ExampleDisclosureMissing);
        }
        if row
            .example_disclosures
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DisclosureHistoryBlockViolation::ExampleDisclosureDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DisclosureHistoryBlockViolation::StableLaneMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DisclosureHistoryBlockViolation::SourceInvariantViolated);
        }
    }
}

/// Every worked resolution must project all three channels with identical core truth, and
/// at least one worked resolution must carry a copy-safe reference set (the Aureline id
/// plus at least one alias) with a full export — the acceptance-criterion proof (AC1) that
/// Help/About, update, and support lanes share one disclosure / history primitive and one
/// copy-safe id behavior.
fn validate_shared_primitive_parity(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let cases: Vec<&M5ResolvedDisclosureHistoryBlock> = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .map(|case| &case.resolved)
        .collect();
    if cases.is_empty() {
        violations.push(M5DisclosureHistoryBlockViolation::SharedPrimitiveParityUnproven);
        return;
    }
    let all_channels_projected = cases.iter().all(|block| {
        let present: BTreeSet<M5DisclosureBlockChannel> = block
            .channel_projections
            .iter()
            .map(|projection| projection.channel)
            .collect();
        M5DisclosureBlockChannel::ALL
            .iter()
            .all(|channel| present.contains(channel))
            && block.channel_projections.iter().all(|projection| {
                projection.history_state == block.history_state
                    && projection.display_posture == block.display_posture
                    && projection.severity == block.severity
                    && projection.primary_reference_id == block.advisory_id
                    && projection.handoff_posture == block.handoff_posture
            })
    });
    let copy_safe_ids_proven = cases.iter().any(|block| {
        block.reference_ids_copy_safe
            && block.reference_ids.len() >= 2
            && block
                .reference_ids
                .iter()
                .any(|id| id.kind == M5DisclosureReferenceKind::AurelineAdvisoryId)
            && block
                .reference_ids
                .iter()
                .all(|id| !id.value.trim().is_empty() && !repr_is_forbidden(&id.value))
            && block.export_summary.columns.len() >= MANDATORY_EXPORT_FIELDS.len()
            && block
                .export_summary
                .columns
                .iter()
                .all(|column| !column.value.trim().is_empty())
    });
    if !all_channels_projected || !copy_safe_ids_proven {
        violations.push(M5DisclosureHistoryBlockViolation::SharedPrimitiveParityUnproven);
    }
}

/// At least one worked resolution must resolve a downgraded advisory (superseded,
/// resolved, or withdrawn) to a stepped-down display posture that stays inspectable, and
/// every worked resolution must remain inspectable with current-status truth — the
/// acceptance-criterion proof (AC2) that resolved advisories step down visually but remain
/// inspectable.
fn validate_resolved_step_down(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let cases: Vec<&M5ResolvedDisclosureHistoryBlock> = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .map(|case| &case.resolved)
        .collect();
    if cases.is_empty() {
        violations.push(M5DisclosureHistoryBlockViolation::ResolvedStepDownUnproven);
        return;
    }
    let all_inspectable = cases
        .iter()
        .all(|block| block.remains_inspectable && block.current_status_visible);
    let stepped_down = cases.iter().any(|block| {
        block.is_resolved_history
            && block.history_state.is_resolved_history()
            && block.display_posture == M5DisclosureDisplayPosture::SteppedDownInspectable
            && block.remains_inspectable
            && block.current_status_visible
    });
    if !all_inspectable || !stepped_down {
        violations.push(M5DisclosureHistoryBlockViolation::ResolvedStepDownUnproven);
    }
}

/// Every worked resolution must preserve the in-product disclosure state on handoff and
/// keep provenance visible, and at least one worked resolution must exercise a remote /
/// external provenance-preserved handoff — the acceptance-criterion proof (AC3) that an
/// external handoff preserves provenance and does not replace the in-product disclosure
/// state with a dead-end link.
fn validate_provenance_handoff(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let cases: Vec<&M5ResolvedDisclosureHistoryBlock> = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .map(|case| &case.resolved)
        .collect();
    if cases.is_empty() {
        violations.push(M5DisclosureHistoryBlockViolation::ProvenanceHandoffUnproven);
        return;
    }
    let all_preserved = cases.iter().all(|block| {
        block.preserves_in_product_state_on_handoff
            && !block.is_dead_end_link
            && block.provenance_visible
            && !block.provenance_repr.trim().is_empty()
    });
    let remote_source_proven = cases
        .iter()
        .any(|block| block.handoff_posture.is_remote_source());
    if !all_preserved || !remote_source_proven {
        violations.push(M5DisclosureHistoryBlockViolation::ProvenanceHandoffUnproven);
    }
}

/// Every history state must be exercised by some worked resolution so the block is proven
/// to render every resolved-versus-active state — including the resolved / superseded /
/// withdrawn states that step down their visual weight.
fn validate_history_state_coverage(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let present: BTreeSet<M5DisclosureHistoryState> = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .map(|case| case.resolved.history_state)
        .collect();
    if !M5DisclosureHistoryState::ALL
        .iter()
        .all(|state| present.contains(state))
    {
        violations.push(M5DisclosureHistoryBlockViolation::HistoryStateCoverageUnproven);
    }
}

/// Every severity class must be exercised by some worked resolution so the block is proven
/// to render every severity.
fn validate_severity_coverage(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let present: BTreeSet<M5AdvisorySeverityClass> = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .map(|case| case.resolved.severity)
        .collect();
    if !M5AdvisorySeverityClass::ALL
        .iter()
        .all(|severity| present.contains(severity))
    {
        violations.push(M5DisclosureHistoryBlockViolation::SeverityCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_block_model_across_source_lanes,
        review.current_status_versions_path_visible_without_drawer,
        review.reference_ids_are_copy_safe,
        review.open_doc_and_open_browser_actions_present,
        review.resolved_advisories_step_down_but_remain_inspectable,
        review.provenance_visible_when_mirrored_offline_or_external,
        review.external_handoff_preserves_in_product_state,
        review.copy_safe_advisory_id_preserved,
        review.export_summary_reconstructs_disclosure_truth,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_lanes_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DisclosureHistoryBlockViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.help_about_renders_shared_block,
        projection.update_center_renders_shared_block,
        projection.support_bundle_renders_shared_block,
        projection.history_view_reads_single_source,
        projection.resolver_reads_single_disclosure_vocabulary,
    ] {
        if !ok {
            violations.push(M5DisclosureHistoryBlockViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DisclosureHistoryBlockViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DisclosureHistoryBlockPacket,
    violations: &mut Vec<M5DisclosureHistoryBlockViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.disclosure_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DisclosureHistoryBlockViolation::ReleasePostureIncomplete);
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
