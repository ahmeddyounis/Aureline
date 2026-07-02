//! Keyboard, screen-reader, touch, and support-export parity for menu, keybinding-help, and command-doc
//! surfaces across every claimed M5 desktop profile.
//!
//! The [frozen discoverability matrix][matrix] already binds each governed M5 last-mile command-discovery
//! surface — menu items, menu groups, context menus, command bars, keybinding resolver layers, conflict
//! review sheets, import-bridge rows, disabled-command explainers, leader/sequence help overlays, and
//! command-documentation surfaces — to one canonical command record, and freezes the required-label,
//! why-unavailable-reason, feature-family, discovery-channel, and downgrade-trigger vocabulary those
//! surfaces project from. This lane is the **accessibility / support-export parity capstone** that
//! certifies, for every one of those ten surface families, that the discoverability surface stays *usable
//! without pointer hover* and *diagnosable after the fact*: it is fully keyboard- and screen-reader
//! addressable with a focus-return and touch / context-action equivalent for any hover-only reach; the
//! same command discoverability and blocked-state behaviour reconstructs from a structured, copy-safe
//! support/export packet rather than a screenshot or private team memory; it stays reachable and stable
//! across the claimed reduced-motion, high-zoom, compact-layout, and multi-window desktop profiles; and
//! the parity checks are wired into release evidence so a stale help anchor, missing narration text, or
//! hover-only discoverability regression auto-narrows the claim before release widening.
//!
//! For every surface family the lane certifies four things the acceptance criteria and implementation
//! requirements demand:
//!
//! - the surface's **non-pointer reach** is certified — a keyboard path, screen-reader narration, focus
//!   return, and a touch / context-action equivalent for any hover-only affordance — so the menu, help, or
//!   documentation surface is fully keyboard- and screen-reader-addressable rather than hover-gated
//!   ([`NonPointerReachState`], acceptance criterion 1 + implementation requirement 1);
//! - the surface's **support-export evidence** reconstructs the command id, source layer, conflict /
//!   blocker reason, lifecycle state, and help-anchor references from a structured, copy-safe export — so a
//!   support reviewer can explain command discoverability and blocked-state behaviour without a screenshot
//!   ([`SupportExportEvidenceState`], acceptance criterion 2 + implementation requirement 2);
//! - the surface stays **reachable and stable across every claimed desktop profile** — reduced-motion,
//!   high-zoom, compact-layout, and multi-window — so a discoverability / help surface never becomes
//!   unreachable or unstable on a constrained profile ([`ProfileStabilityState`], implementation
//!   requirement 3);
//! - and the **release-evidence parity checks gate release widening** — a stale help anchor, missing
//!   narration text, or hover-only discoverability regression auto-narrows the claim rather than shipping
//!   silently ([`ReleaseEvidenceFreshnessState`], acceptance criterion 3 + implementation requirement 4).
//!
//! Three records carry the truth:
//!
//! - the per-family **access-parity row** ([`AccessParityRow`]): one row per [`M5CommandSurfaceFamily`]
//!   naming the canonical command binding it projects from, the required labels and feature families it
//!   exposes, the non-pointer reach channels it certifies, the accessibility-incident fields it captures,
//!   the desktop access profiles it stays stable in, the consumer surfaces it evaluated, its non-pointer
//!   reach / support-export-evidence / profile-stability / release-evidence posture, whether the same
//!   parity survives headless/CLI execution, any active waiver, and a derived green/yellow/red
//!   [`AccessParityStatus`].
//! - the access-parity **packet** ([`AccessParityPacket`]): the full set of rows with derived per-row
//!   status, aggregate green/yellow/red counts, the active waivers, the exact conformance causes
//!   ([`AccessParityCause`]), and the blocking findings the lane refuses to ship with.
//! - the access-parity **dashboard** ([`AccessParityDashboard`]): a light projection the palette / menu /
//!   keybinding UI / help / Support Center / CLI tooling reads to auto-narrow a surface's accessibility /
//!   export claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment a surface
//! discloses a reduced touch fallback (a waivered narrowing), a disclosed partial support-export capture, a
//! disclosed reduced profile coverage, or a disclosed partial release-evidence refresh; it drops to `red`
//! if a surface renders hover-only or drops screen-reader narration, cannot reconstruct its blocked-state
//! evidence from durable capture, becomes unreachable or unstable on a claimed profile, ships a stale help
//! anchor or an unblocked hover-only regression, loses the same parity in a headless/CLI execution, or
//! fails to certify all five non-pointer reach channels, all five accessibility-incident fields, all four
//! desktop access profiles, or every declared consumer surface. That derivation is the auto-narrowing the
//! acceptance criteria require, and the reach-channel, incident-field, access-profile, and consumer-surface
//! completeness checks are the conformance lints that gate a stable accessibility / export claim.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local paths,
//! raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary, counts,
//! refs, and short labels. The surface-family, canonical-command-binding, required-label, lifecycle-label,
//! preview-class, feature-family, consumer-surface, downgrade-trigger, and qualification vocabulary is
//! re-exported by reference from the already frozen [matrix], and every family's canonical command binding,
//! qualification, owner, required labels, lifecycle label, feature families, declared consumer surfaces,
//! and applicable downgrade triggers are pulled straight from that matrix's seeded packet, so this lane
//! mints no parallel command vocabulary and cannot certify a surface the matrix does not anchor. Only the
//! access-parity-specific vocabulary ([`M5AccessParityDimension`], [`M5NonPointerReachChannel`],
//! [`M5AccessibilityIncidentField`], [`M5DesktopAccessProfile`], [`AccessParityStatus`],
//! [`NonPointerReachState`], [`SupportExportEvidenceState`], [`ProfileStabilityState`],
//! [`ReleaseEvidenceFreshnessState`], [`AccessParityWaiver`], [`AccessParityCause`],
//! [`AccessParityFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix as matrix;

pub use matrix::{
    M5CanonicalCommandBinding, M5CommandSurfaceFamily, M5DisabledReasonMode,
    M5DiscoverabilityDowngradeTrigger, M5DiscoveryChannel, M5FeatureFamily, M5LifecycleLabel,
    M5PreviewClass, M5RequiredLabel, M5SurfaceQualificationClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_discoverability_access_parity_packet,
    seeded_m5_discoverability_access_parity_packet_context_menu_evidence_absent_blocked,
    seeded_m5_discoverability_access_parity_packet_doc_stale_anchor_blocked,
    seeded_m5_discoverability_access_parity_packet_explainer_headless_parity_lost_blocked,
    seeded_m5_discoverability_access_parity_packet_menu_item_hover_only_blocked,
    seeded_m5_discoverability_access_parity_packet_resolver_profile_unstable_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_ACCESS_PARITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_ACCESS_PARITY_SHARED_CONTRACT_REF: &str =
    "commands:m5_discoverability_access_parity:v1";

/// Stable record kind for [`AccessParityPacket`] payloads.
pub const M5_ACCESS_PARITY_PACKET_RECORD_KIND: &str =
    "commands_m5_discoverability_access_parity_packet_record";

/// Stable record kind for [`AccessParityDashboard`] payloads.
pub const M5_ACCESS_PARITY_DASHBOARD_RECORD_KIND: &str =
    "commands_m5_discoverability_access_parity_dashboard_record";

/// Stable record kind for [`AccessParitySupportExport`] payloads.
pub const M5_ACCESS_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "commands_m5_discoverability_access_parity_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_ACCESS_PARITY_PACKET_ID: &str = "m5-discoverability-access-parity:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_ACCESS_PARITY_DASHBOARD_ID: &str =
    "m5-discoverability-access-parity-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_ACCESS_PARITY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-discoverability-access-parity:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_ACCESS_PARITY_SOURCE_SCHEMA_REF: &str =
    "schemas/commands/m5-discoverability-access-parity.schema.json";

/// Published markdown report ref reviewers reopen the parity proof from.
pub const M5_ACCESS_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/commands/m5-discoverability-access-parity.md";

/// Published parity-packet artifact ref.
pub const M5_ACCESS_PARITY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-discoverability-access-parity-proof/packet.json";

/// Published parity-dashboard artifact ref.
pub const M5_ACCESS_PARITY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-discoverability-access-parity-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_ACCESS_PARITY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-discoverability-access-parity-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_ACCESS_PARITY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-discoverability-access-parity-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_ACCESS_PARITY_PUBLISHED_DOC_REF: &str =
    "docs/commands/m5_discoverability_access_parity_contract.md";

/// Repo-relative ref to the frozen discoverability boundary schema.
pub const M5_ACCESS_PARITY_MATRIX_SCHEMA_REF: &str = matrix::M5_DISCOVERABILITY_SCHEMA_REF;

/// Frozen discoverability contract doc this proof mirrors.
pub const M5_ACCESS_PARITY_MATRIX_DOC_REF: &str = matrix::M5_DISCOVERABILITY_DOC_REF;

/// Canonical command-descriptor schema every certified surface projects from.
pub const M5_ACCESS_PARITY_COMMAND_DESCRIPTOR_REF: &str =
    matrix::M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF;

/// Every command-surface family the certification must cover, in canonical order. A certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_SURFACE_FAMILIES: [M5CommandSurfaceFamily; 10] = M5CommandSurfaceFamily::ALL;

/// Every access-parity dimension each family row certifies, in canonical order.
pub const REQUIRED_ACCESS_DIMENSIONS: [M5AccessParityDimension; 4] = M5AccessParityDimension::ALL;

/// Every non-pointer reach channel each family row must certify, in canonical order.
pub const REQUIRED_REACH_CHANNELS: [M5NonPointerReachChannel; 5] = M5NonPointerReachChannel::ALL;

/// Every accessibility-incident field each family row must capture, in canonical order.
pub const REQUIRED_INCIDENT_FIELDS: [M5AccessibilityIncidentField; 5] =
    M5AccessibilityIncidentField::ALL;

/// Every desktop access profile each family row must stay stable in, in canonical order.
pub const REQUIRED_ACCESS_PROFILES: [M5DesktopAccessProfile; 4] = M5DesktopAccessProfile::ALL;

/// One of the four access-parity dimensions each surface-family row certifies.
///
/// These are exactly the four ways the acceptance criteria and implementation requirements demand a claimed
/// M5 discoverability surface stay usable without pointer hover and diagnosable after the fact: the surface
/// stays reachable through non-pointer channels; a structured support-export reconstructs command
/// discoverability and blocked-state evidence; the surface stays reachable and stable across every claimed
/// desktop profile; and the parity checks gate release evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessParityDimension {
    /// The surface stays reachable through keyboard, screen-reader, focus-return, and touch channels.
    NonPointerReach,
    /// A structured support-export reconstructs command discoverability and blocked-state evidence.
    SupportExportEvidence,
    /// The surface stays reachable and stable across every claimed desktop profile.
    ProfileStability,
    /// The parity checks gate release evidence, auto-narrowing on regressions.
    ReleaseEvidence,
}

impl M5AccessParityDimension {
    /// Every access-parity dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NonPointerReach,
        Self::SupportExportEvidence,
        Self::ProfileStability,
        Self::ReleaseEvidence,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonPointerReach => "non_pointer_reach",
            Self::SupportExportEvidence => "support_export_evidence",
            Self::ProfileStability => "profile_stability",
            Self::ReleaseEvidence => "release_evidence",
        }
    }
}

/// One of the five non-pointer reach channels a discoverability surface must certify.
///
/// These are the exact channels the implementation requirements name for the parity fixtures — the pointer
/// default, a keyboard path, screen-reader narration, focus return, and a touch / context-action fallback
/// for any hover-only affordance. A surface reachable through fewer hides behind pointer hover and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NonPointerReachChannel {
    /// Reachable through pointer interaction (the default).
    PointerDefault,
    /// Reachable through a keyboard path, without a pointer hover.
    KeyboardPath,
    /// Reachable / announced through screen-reader narration.
    ScreenReaderNarration,
    /// Focus returns to a predictable place after the surface closes.
    FocusReturn,
    /// Reachable through a touch / context-action fallback.
    TouchContextAction,
}

impl M5NonPointerReachChannel {
    /// Every non-pointer reach channel, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PointerDefault,
        Self::KeyboardPath,
        Self::ScreenReaderNarration,
        Self::FocusReturn,
        Self::TouchContextAction,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PointerDefault => "pointer_default",
            Self::KeyboardPath => "keyboard_path",
            Self::ScreenReaderNarration => "screen_reader_narration",
            Self::FocusReturn => "focus_return",
            Self::TouchContextAction => "touch_context_action",
        }
    }
}

/// One of the five accessibility-incident fields a support-export packet must capture.
///
/// These are the exact fields the implementation requirements name for an export-safe discoverability /
/// blocked-state incident packet — the command id, the source layer, the conflict / blocker reason, the
/// lifecycle state, and the help-anchor references — that let a support reviewer explain the surface without
/// a screenshot. An export that captures fewer cannot reconstruct the incident and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessibilityIncidentField {
    /// The canonical command id the surface projected.
    CommandId,
    /// The source layer (menu / keybinding / doc / import) the incident was reached from.
    SourceLayer,
    /// The conflict or blocker reason that narrowed the command.
    ConflictOrBlockerReason,
    /// The lifecycle / deprecation state the command carried.
    LifecycleState,
    /// The help-anchor references a reviewer reopens the surface from.
    HelpAnchorRef,
}

impl M5AccessibilityIncidentField {
    /// Every accessibility-incident field, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CommandId,
        Self::SourceLayer,
        Self::ConflictOrBlockerReason,
        Self::LifecycleState,
        Self::HelpAnchorRef,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandId => "command_id",
            Self::SourceLayer => "source_layer",
            Self::ConflictOrBlockerReason => "conflict_or_blocker_reason",
            Self::LifecycleState => "lifecycle_state",
            Self::HelpAnchorRef => "help_anchor_ref",
        }
    }
}

/// One of the four desktop access profiles a discoverability surface must stay reachable and stable in.
///
/// These are the exact profiles the implementation requirements name — reduced-motion, high-zoom,
/// compact-layout, and multi-window — the surface must remain reachable and stable across. A surface stable
/// in fewer profiles could become unreachable or unstable on a constrained desktop profile and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesktopAccessProfile {
    /// Reduced-motion profile.
    ReducedMotion,
    /// High-zoom / large-text profile.
    HighZoom,
    /// Compact / constrained-layout profile.
    CompactLayout,
    /// Multi-window / multi-monitor profile.
    MultiWindow,
}

impl M5DesktopAccessProfile {
    /// Every desktop access profile, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReducedMotion,
        Self::HighZoom,
        Self::CompactLayout,
        Self::MultiWindow,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReducedMotion => "reduced_motion",
            Self::HighZoom => "high_zoom",
            Self::CompactLayout => "compact_layout",
            Self::MultiWindow => "multi_window",
        }
    }
}

/// The derived accessibility / export light a command surface carries.
///
/// `green` means the surface is fully keyboard- and screen-reader-addressable with a focus-return and touch
/// / context-action equivalent, reconstructs its command and blocked-state evidence from a structured
/// support-export, stays reachable and stable across every claimed desktop profile, and its parity checks
/// gate release evidence — across every declared consumer surface, with the same parity surviving
/// headless/CLI execution. `yellow` is a disclosed narrowing. `red` is blocked and may not keep an
/// accessibility / export claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessParityStatus {
    /// Full standing: all four access-parity dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl AccessParityStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the surface stays reachable without pointer hover.
///
/// `keyboard_screen_reader_and_touch_parity_certified` means the surface is fully keyboard- and
/// screen-reader-addressable, returns focus predictably, and offers a touch / context-action equivalent for
/// any hover-only reach. `disclosed_reduced_touch_fallback` means a constrained touch surface falls back to
/// a disclosed reduced form while still keeping the keyboard path and screen-reader narration (a yellow
/// narrowing that **requires an active waiver**). `hover_only_or_narration_missing` means the surface is
/// hover-only with no keyboard / touch equivalent, or drops its screen-reader narration — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonPointerReachState {
    /// Keyboard, screen-reader, focus-return, and touch parity are certified.
    KeyboardScreenReaderAndTouchParityCertified,
    /// A constrained touch surface takes a disclosed, waivered reduced fallback.
    DisclosedReducedTouchFallback,
    /// The surface is hover-only or drops screen-reader narration — a blocker.
    HoverOnlyOrNarrationMissing,
}

impl NonPointerReachState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardScreenReaderAndTouchParityCertified => {
                "keyboard_screen_reader_and_touch_parity_certified"
            }
            Self::DisclosedReducedTouchFallback => "disclosed_reduced_touch_fallback",
            Self::HoverOnlyOrNarrationMissing => "hover_only_or_narration_missing",
        }
    }

    /// `true` when non-pointer reach is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::KeyboardScreenReaderAndTouchParityCertified)
    }

    /// `true` when the surface took a disclosed reduced-touch-fallback narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedTouchFallback)
    }
}

/// How a structured support-export reconstructs command discoverability and blocked-state evidence.
///
/// `structured_incident_evidence_reconstructable` means a support bundle, doc, or migration packet can
/// reconstruct the command id, source layer, conflict / blocker reason, lifecycle state, and help-anchor
/// references from a durable, copy-safe, diffable export without a screenshot. `disclosed_partial_capture`
/// means one legacy export captures the command id and blocker reason but not the full incident-field set,
/// while still disclosing the gap (a yellow narrowing). `blocked_state_absent_from_capture` means the
/// blocked-state evidence is absent from durable capture — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportEvidenceState {
    /// The structured incident evidence is reconstructable from durable evidence.
    StructuredIncidentEvidenceReconstructable,
    /// One legacy export takes a disclosed partial capture.
    DisclosedPartialCapture,
    /// The blocked-state evidence is absent from durable capture — a blocker.
    BlockedStateAbsentFromCapture,
}

impl SupportExportEvidenceState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredIncidentEvidenceReconstructable => {
                "structured_incident_evidence_reconstructable"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::BlockedStateAbsentFromCapture => "blocked_state_absent_from_capture",
        }
    }

    /// `true` when support-export evidence is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::StructuredIncidentEvidenceReconstructable)
    }

    /// `true` when the surface took a disclosed partial-capture narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }
}

/// How the surface stays reachable and stable across the claimed desktop profiles.
///
/// `reachable_and_stable_across_all_profiles` means the surface remains reachable and stable across the
/// reduced-motion, high-zoom, compact-layout, and multi-window profiles. `disclosed_reduced_profile_coverage`
/// means one constrained profile renders a disclosed reduced form while still keeping the surface reachable
/// and stable (a yellow narrowing). `surface_unreachable_or_unstable_in_profile` means the surface becomes
/// unreachable or unstable on a claimed profile — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileStabilityState {
    /// The surface is reachable and stable across all claimed profiles.
    ReachableAndStableAcrossAllProfiles,
    /// One constrained profile takes a disclosed reduced coverage.
    DisclosedReducedProfileCoverage,
    /// The surface is unreachable or unstable on a claimed profile — a blocker.
    SurfaceUnreachableOrUnstableInProfile,
}

impl ProfileStabilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndStableAcrossAllProfiles => "reachable_and_stable_across_all_profiles",
            Self::DisclosedReducedProfileCoverage => "disclosed_reduced_profile_coverage",
            Self::SurfaceUnreachableOrUnstableInProfile => {
                "surface_unreachable_or_unstable_in_profile"
            }
        }
    }

    /// `true` when profile stability is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ReachableAndStableAcrossAllProfiles)
    }

    /// `true` when the surface took a disclosed reduced-profile-coverage narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedProfileCoverage)
    }
}

/// How the parity checks gate release evidence.
///
/// `parity_checks_gate_release_evidence` means a stale help anchor, missing narration text, or hover-only
/// discoverability regression auto-narrows the claim in the release evidence before release widening.
/// `disclosed_partial_evidence_refresh` means one legacy release-evidence surface refreshes on a disclosed
/// delayed cadence while still gating the claim (a yellow narrowing). `stale_anchor_or_regression_unblocked`
/// means a stale help anchor or hover-only regression ships without narrowing the claim — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseEvidenceFreshnessState {
    /// The parity checks gate release evidence and auto-narrow on regressions.
    ParityChecksGateReleaseEvidence,
    /// One release-evidence surface takes a disclosed partial evidence refresh.
    DisclosedPartialEvidenceRefresh,
    /// A stale anchor or regression ships without narrowing the claim — a blocker.
    StaleAnchorOrRegressionUnblocked,
}

impl ReleaseEvidenceFreshnessState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityChecksGateReleaseEvidence => "parity_checks_gate_release_evidence",
            Self::DisclosedPartialEvidenceRefresh => "disclosed_partial_evidence_refresh",
            Self::StaleAnchorOrRegressionUnblocked => "stale_anchor_or_regression_unblocked",
        }
    }

    /// `true` when release-evidence freshness is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ParityChecksGateReleaseEvidence)
    }

    /// `true` when the surface took a disclosed partial-evidence-refresh narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialEvidenceRefresh)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather than
/// blocked — never lets a hover-only surface, missing narration, an uncapturable blocked state, an unstable
/// profile, or an unblocked stale-anchor regression hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessParityWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The surface family the waiver applies to.
    pub surface_family: M5CommandSurfaceFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl AccessParityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a surface family's accessibility / export parity.
///
/// The trigger token mirrors the frozen [`M5DiscoverabilityDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessParityCause {
    /// The surface family the cause applies to.
    pub surface_family: M5CommandSurfaceFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5DiscoverabilityDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is a
    /// blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl AccessParityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One surface family, certified across its non-pointer-reach, support-export-evidence, profile-stability,
/// and release-evidence dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessParityRow {
    /// The surface family being certified.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short reviewer-facing family label.
    pub surface_label: String,
    /// Qualification class the matrix earned for the surface. Pulled from the matrix.
    pub qualification: M5SurfaceQualificationClass,
    /// Owner role accountable for keeping this surface's parity governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short conformance scope summary.
    pub scope_summary: String,
    /// The canonical command-record binding this surface projects from. Pulled from the matrix.
    pub canonical_command_binding: M5CanonicalCommandBinding,
    /// The pinned lifecycle / deprecation label. Pulled from the canonical command binding.
    pub lifecycle_label: M5LifecycleLabel,
    /// The pinned preview / approval class. Pulled from the canonical command binding.
    pub preview_class: M5PreviewClass,
    /// The pinned disabled-reason mode. Pulled from the canonical command binding.
    pub disabled_reason_mode: M5DisabledReasonMode,
    /// Mandatory labels this surface must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5RequiredLabel>,
    /// M5 feature families whose commands this surface projects. Pulled from the matrix.
    pub feature_families: Vec<M5FeatureFamily>,
    /// The non-pointer reach channels this row certifies (must be all five).
    pub certified_reach_channels: Vec<M5NonPointerReachChannel>,
    /// The accessibility-incident fields this row captures (must be all five).
    pub certified_incident_fields: Vec<M5AccessibilityIncidentField>,
    /// The desktop access profiles this row stays stable in (must be all four).
    pub certified_access_profiles: Vec<M5DesktopAccessProfile>,
    /// Consumer surfaces the matrix declares the surface must project to.
    pub required_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Non-pointer-reach posture.
    pub non_pointer_reach: NonPointerReachState,
    /// Support-export-evidence posture.
    pub support_export_evidence: SupportExportEvidenceState,
    /// Profile-stability posture.
    pub profile_stability: ProfileStabilityState,
    /// Release-evidence posture.
    pub release_evidence: ReleaseEvidenceFreshnessState,
    /// `true` when the same parity survives a headless / CLI execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the surface. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced touch fallback is in force.
    pub active_waiver: Option<AccessParityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: AccessParityStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<AccessParityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl AccessParityRow {
    /// `true` when the row certified every consumer surface the matrix declares for the surface — no
    /// declared surface is left uncertified and none is invented.
    pub fn consumer_surfaces_complete(&self) -> bool {
        let mut evaluated: Vec<&str> = self
            .evaluated_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = self
            .required_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        evaluated.sort_unstable();
        required.sort_unstable();
        !required.is_empty() && evaluated == required
    }

    /// `true` when the row certifies every one of the five non-pointer reach channels — the structural
    /// proof that the surface is not hidden behind pointer hover.
    pub fn reach_channels_complete(&self) -> bool {
        complete_tokens(
            &self.certified_reach_channels,
            |channel| channel.as_str(),
            &REQUIRED_REACH_CHANNELS,
            |channel| channel.as_str(),
        )
    }

    /// `true` when the row captures every one of the five accessibility-incident fields — the structural
    /// proof that a support-export can reconstruct the incident without a screenshot.
    pub fn incident_fields_complete(&self) -> bool {
        complete_tokens(
            &self.certified_incident_fields,
            |field| field.as_str(),
            &REQUIRED_INCIDENT_FIELDS,
            |field| field.as_str(),
        )
    }

    /// `true` when the row stays stable in every one of the four desktop access profiles — the structural
    /// proof that the surface never becomes unreachable on a constrained profile.
    pub fn access_profiles_complete(&self) -> bool {
        complete_tokens(
            &self.certified_access_profiles,
            |profile| profile.as_str(),
            &REQUIRED_ACCESS_PROFILES,
            |profile| profile.as_str(),
        )
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.reach_channels_complete() {
            return true;
        }
        if !self.incident_fields_complete() {
            return true;
        }
        if !self.access_profiles_complete() {
            return true;
        }
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.non_pointer_reach,
            NonPointerReachState::HoverOnlyOrNarrationMissing
        ) {
            return true;
        }
        if matches!(
            self.support_export_evidence,
            SupportExportEvidenceState::BlockedStateAbsentFromCapture
        ) {
            return true;
        }
        if matches!(
            self.profile_stability,
            ProfileStabilityState::SurfaceUnreachableOrUnstableInProfile
        ) {
            return true;
        }
        if matches!(
            self.release_evidence,
            ReleaseEvidenceFreshnessState::StaleAnchorOrRegressionUnblocked
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.non_pointer_reach.is_disclosed_narrowing()
            || self.support_export_evidence.is_disclosed_narrowing()
            || self.profile_stability.is_disclosed_narrowing()
            || self.release_evidence.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the accessibility / export posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> AccessParityStatus {
        if self.has_hard_blocker() {
            AccessParityStatus::Red
        } else if self.has_narrowing() {
            AccessParityStatus::Yellow
        } else {
            AccessParityStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (non-pointer reach,
    /// support-export evidence, profile stability, release evidence, then structural completeness and
    /// headless parity).
    pub fn recompute_causes(&self) -> Vec<AccessParityCause> {
        let mut causes = Vec::new();
        match self.non_pointer_reach {
            NonPointerReachState::KeyboardScreenReaderAndTouchParityCertified => {}
            NonPointerReachState::DisclosedReducedTouchFallback => {
                causes.push(AccessParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                    disclosed: true,
                    detail: "On a constrained touch surface the affordance falls back to a disclosed, \
                             waivered reduced form while the keyboard path and screen-reader narration stay \
                             present — so the reach is narrowed and disclosed rather than hover-only."
                        .to_owned(),
                });
            }
            NonPointerReachState::HoverOnlyOrNarrationMissing => {
                causes.push(AccessParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                    disclosed: false,
                    detail: "The surface is hover-only with no keyboard or touch / context-action \
                             equivalent, or it drops its screen-reader narration, so the menu / help / \
                             documentation surface cannot be addressed without a pointer."
                        .to_owned(),
                });
            }
        }
        match self.support_export_evidence {
            SupportExportEvidenceState::StructuredIncidentEvidenceReconstructable => {}
            SupportExportEvidenceState::DisclosedPartialCapture => {
                causes.push(AccessParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One legacy support-export takes a disclosed partial capture — the export \
                             captures the command id and blocker reason but not the full incident-field set, \
                             while still disclosing the gap — so the support-export evidence is narrowed and \
                             disclosed rather than absent."
                        .to_owned(),
                });
            }
            SupportExportEvidenceState::BlockedStateAbsentFromCapture => {
                causes.push(AccessParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail: "The blocked-state evidence — command id, source layer, conflict / blocker \
                             reason, lifecycle state, and help-anchor references — is absent from the \
                             durable, diffable export, so a support reviewer cannot explain the surface \
                             without a screenshot or private team memory."
                        .to_owned(),
                });
            }
        }
        match self.profile_stability {
            ProfileStabilityState::ReachableAndStableAcrossAllProfiles => {}
            ProfileStabilityState::DisclosedReducedProfileCoverage => {
                causes.push(AccessParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                    disclosed: true,
                    detail: "On one constrained desktop profile the surface renders a disclosed reduced \
                             form while still staying reachable and stable — so the profile coverage is \
                             narrowed and disclosed rather than unreachable."
                        .to_owned(),
                });
            }
            ProfileStabilityState::SurfaceUnreachableOrUnstableInProfile => {
                causes.push(AccessParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                    disclosed: false,
                    detail: "The surface becomes unreachable or unstable on a claimed desktop profile — \
                             reduced-motion, high-zoom, compact-layout, or multi-window — so the \
                             discoverability / help surface cannot be relied on across the profiles."
                        .to_owned(),
                });
            }
        }
        match self.release_evidence {
            ReleaseEvidenceFreshnessState::ParityChecksGateReleaseEvidence => {}
            ReleaseEvidenceFreshnessState::DisclosedPartialEvidenceRefresh => {
                causes.push(AccessParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One release-evidence surface refreshes on a disclosed delayed cadence while \
                             still gating the claim, so the release-evidence freshness is narrowed and \
                             disclosed rather than stale."
                        .to_owned(),
                });
            }
            ReleaseEvidenceFreshnessState::StaleAnchorOrRegressionUnblocked => {
                causes.push(AccessParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail: "A stale help anchor, missing narration text, or hover-only discoverability \
                             regression shipped without narrowing the claim, so the release evidence \
                             overclaims accessibility the surface no longer provides."
                        .to_owned(),
                });
            }
        }
        if !self.reach_channels_complete() {
            causes.push(AccessParityCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "The surface is not reachable through all five non-pointer reach channels — pointer \
                         default, keyboard path, screen-reader narration, focus return, and touch / \
                         context-action fallback — so it could be hidden behind hover in some channels."
                    .to_owned(),
            });
        }
        if !self.incident_fields_complete() {
            causes.push(AccessParityCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                disclosed: false,
                detail: "The support-export does not capture all five accessibility-incident fields — \
                         command id, source layer, conflict / blocker reason, lifecycle state, and \
                         help-anchor references — so a reviewer could not reconstruct the incident from \
                         structured evidence."
                    .to_owned(),
            });
        }
        if !self.access_profiles_complete() {
            causes.push(AccessParityCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "The surface is not certified stable across all four desktop access profiles — \
                         reduced-motion, high-zoom, compact-layout, and multi-window — so it could become \
                         unreachable on an uncertified profile."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(AccessParityCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "A headless / CLI execution of this surface lost the shared accessibility / export \
                         parity, so the same command explains a different reach or blocked-state depending \
                         on how it is reached."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced touch fallback may only stay yellow (rather than red) when a waiver discloses it —
    /// reducing a surface's touch reach is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.non_pointer_reach,
            NonPointerReachState::DisclosedReducedTouchFallback
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<AccessParityFinding> {
        let mut findings = Vec::new();
        let family = self.surface_family.as_str().to_owned();

        if !self.reach_channels_complete() {
            findings.push(AccessParityFinding::ReachChannelsIncomplete {
                family: family.clone(),
            });
        }
        if !self.incident_fields_complete() {
            findings.push(AccessParityFinding::IncidentFieldsIncomplete {
                family: family.clone(),
            });
        }
        if !self.access_profiles_complete() {
            findings.push(AccessParityFinding::AccessProfilesIncomplete {
                family: family.clone(),
            });
        }
        if !self.consumer_surfaces_complete() {
            findings.push(AccessParityFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(AccessParityFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.non_pointer_reach,
            NonPointerReachState::HoverOnlyOrNarrationMissing
        ) {
            findings.push(AccessParityFinding::NonPointerReachBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.support_export_evidence,
            SupportExportEvidenceState::BlockedStateAbsentFromCapture
        ) {
            findings.push(AccessParityFinding::SupportExportEvidenceBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.profile_stability,
            ProfileStabilityState::SurfaceUnreachableOrUnstableInProfile
        ) {
            findings.push(AccessParityFinding::ProfileStabilityBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.release_evidence,
            ReleaseEvidenceFreshnessState::StaleAnchorOrRegressionUnblocked
        ) {
            findings.push(AccessParityFinding::ReleaseEvidenceBroken {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, AccessParityStatus::Green) && !self.has_reason() {
            findings.push(AccessParityFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(AccessParityFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.surface_family != self.surface_family {
                findings.push(AccessParityFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(AccessParityFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(AccessParityFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(AccessParityFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} reach={} evidence={} profile={} release={} headless={} lifecycle={} preview={} channels={} fields={} profiles={} surfaces={} waiver={}",
            self.surface_family.as_str(),
            self.derived_status.as_str(),
            self.non_pointer_reach.as_str(),
            self.support_export_evidence.as_str(),
            self.profile_stability.as_str(),
            self.release_evidence.as_str(),
            self.headless_parity_preserved,
            self.lifecycle_label.as_str(),
            self.preview_class.as_str(),
            self.certified_reach_channels.len(),
            self.certified_incident_fields.len(),
            self.certified_access_profiles.len(),
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// `true` when `certified` (deduped) equals the required token set exactly.
fn complete_tokens<T, R>(
    certified: &[T],
    cert_token: impl Fn(&T) -> &'static str,
    required: &[R],
    req_token: impl Fn(&R) -> &'static str,
) -> bool {
    let mut got: Vec<&str> = certified.iter().map(&cert_token).collect();
    let mut want: Vec<&str> = required.iter().map(&req_token).collect();
    got.sort_unstable();
    got.dedup();
    want.sort_unstable();
    got == want
}

/// A blocking finding the access-parity certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum AccessParityFinding {
    /// A surface family has no access-parity row.
    SurfaceFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row did not certify every non-pointer reach channel.
    ReachChannelsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not capture every accessibility-incident field.
    IncidentFieldsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row is not stable in every desktop access profile.
    AccessProfilesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless / CLI execution lost the shared accessibility / export parity.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// A surface is hover-only or drops its screen-reader narration.
    NonPointerReachBroken {
        /// The family token.
        family: String,
    },
    /// A surface's blocked-state evidence is absent from durable capture.
    SupportExportEvidenceBroken {
        /// The family token.
        family: String,
    },
    /// A surface is unreachable or unstable on a claimed desktop profile.
    ProfileStabilityBroken {
        /// The family token.
        family: String,
    },
    /// A stale help anchor or hover-only regression shipped without narrowing the claim.
    ReleaseEvidenceBroken {
        /// The family token.
        family: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The family token.
        family: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The family token.
        family: String,
    },
    /// An attached waiver does not point at the row's family.
    WaiverFamilyMismatch {
        /// The family token.
        family: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The family token.
        family: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The family token.
        family: String,
    },
    /// The declared conformance causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl AccessParityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::SurfaceFamilyMissing { .. } => "surface_family_missing",
            Self::ReachChannelsIncomplete { .. } => "reach_channels_incomplete",
            Self::IncidentFieldsIncomplete { .. } => "incident_fields_incomplete",
            Self::AccessProfilesIncomplete { .. } => "access_profiles_incomplete",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::NonPointerReachBroken { .. } => "non_pointer_reach_broken",
            Self::SupportExportEvidenceBroken { .. } => "support_export_evidence_broken",
            Self::ProfileStabilityBroken { .. } => "profile_stability_broken",
            Self::ReleaseEvidenceBroken { .. } => "release_evidence_broken",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::SurfaceFamilyMissing { family }
            | Self::ReachChannelsIncomplete { family }
            | Self::IncidentFieldsIncomplete { family }
            | Self::AccessProfilesIncomplete { family }
            | Self::ConsumerSurfacesIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::NonPointerReachBroken { family }
            | Self::SupportExportEvidenceBroken { family }
            | Self::ProfileStabilityBroken { family }
            | Self::ReleaseEvidenceBroken { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The access-parity packet shared by the palette / menu / keybinding UI / help / Support Center / CLI
/// tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessParityPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen discoverability matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen discoverability boundary schema.
    pub matrix_schema_ref: String,
    /// Frozen discoverability contract doc this proof mirrors.
    pub matrix_doc_ref: String,
    /// Canonical command-descriptor schema every certified surface projects from.
    pub command_descriptor_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four access-parity dimensions every family row certifies.
    pub required_access_dimensions: Vec<String>,
    /// The five non-pointer reach channels every family row must certify.
    pub required_reach_channels: Vec<String>,
    /// The five accessibility-incident fields every family row must capture.
    pub required_incident_fields: Vec<String>,
    /// The four desktop access profiles every family row must stay stable in.
    pub required_access_profiles: Vec<String>,
    /// The ten surface families the certification must cover.
    pub required_surface_families: Vec<String>,
    /// Per-family access-parity rows, in canonical order.
    pub rows: Vec<AccessParityRow>,
    /// Surface families certified, in canonical (sorted) order.
    pub covered_surface_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-conformance) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked — the stable-claim gate.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<AccessParityWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<AccessParityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<AccessParityFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Command / accessibility automation refs that consume this packet to auto-narrow a surface.
    pub command_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help / onboarding refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published access-parity-packet ref.
    pub published_packet_ref: String,
    /// Published access-parity-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl AccessParityPacket {
    /// Returns the access-parity row for `family`, if present.
    pub fn row(&self, family: M5CommandSurfaceFamily) -> Option<&AccessParityRow> {
        self.rows.iter().find(|row| row.surface_family == family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.surface_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.conformance_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.surface_family.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light access-parity dashboard the command automation consumes.
    pub fn dashboard(&self) -> AccessParityDashboard {
        AccessParityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 access-parity packet serializes")
    }

    /// Deterministic, machine-readable parity CSV: one row per surface family naming its status, the four
    /// accessibility / export postures, headless parity, the lifecycle label and preview class, the channel
    /// / field / profile counts, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,status,non_pointer_reach,support_export_evidence,profile_stability,release_evidence,headless_parity,lifecycle,preview_class,reach_channels,incident_fields,access_profiles,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.derived_status.as_str(),
                row.non_pointer_reach.as_str(),
                row.support_export_evidence.as_str(),
                row.profile_stability.as_str(),
                row.release_evidence.as_str(),
                row.headless_parity_preserved,
                row.lifecycle_label.as_str(),
                row.preview_class.as_str(),
                row.certified_reach_channels.len(),
                row.certified_incident_fields.len(),
                row.certified_access_profiles.len(),
                row.evaluated_consumer_surfaces.len(),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 discoverability access parity: keyboard, screen-reader, touch, and support-export parity for menu, keybinding-help, and command-doc surfaces across every claimed M5 desktop profile\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_discoverability_access_parity`](../../crates/aureline-shell/src/m5_discoverability_access_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- markdown > \\\n  artifacts/commands/m5-discoverability-access-parity.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!(
            "- Required access dimensions: {}\n",
            self.required_access_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Non-pointer reach channels: {}\n",
            self.required_reach_channels
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Accessibility-incident fields: {}\n",
            self.required_incident_fields
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Desktop access profiles: {}\n",
            self.required_access_profiles
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Surface families certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full conformance): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable (stable-claim gate): `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
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

        out.push_str("## Access-parity rows\n\n");
        out.push_str(
            "| Surface family | Status | Non-pointer reach | Support-export evidence | Profile stability | Release evidence | Lifecycle | Headless | Waiver |\n\
             | -------------- | ------ | ----------------- | ----------------------- | ----------------- | ---------------- | --------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.non_pointer_reach.as_str(),
                row.support_export_evidence.as_str(),
                row.profile_stability.as_str(),
                row.release_evidence.as_str(),
                row.lifecycle_label.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&AccessParityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, AccessParityStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed M5 command surface stays fully keyboard- and screen-reader-addressable with a focus-return and touch / context-action equivalent, reconstructs its command and blocked-state evidence from a structured support-export, stays reachable and stable across every claimed desktop profile, and gates its accessibility claim on fresh release evidence across every declared consumer surface.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.surface_family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact conformance causes\n\n");
        if self.conformance_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.conformance_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.surface_family.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.surface_family.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_discoverability_access_parity_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light access-parity dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessParityDashboardRow {
    /// The surface family.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short family label.
    pub surface_label: String,
    /// Qualification class earned by the surface.
    pub qualification: M5SurfaceQualificationClass,
    /// Derived green/yellow/red status.
    pub status: AccessParityStatus,
    /// The pinned lifecycle / deprecation label.
    pub lifecycle_label: M5LifecycleLabel,
    /// The pinned preview / approval class.
    pub preview_class: M5PreviewClass,
    /// Number of non-pointer reach channels certified.
    pub certified_reach_channel_count: usize,
    /// Number of accessibility-incident fields captured.
    pub certified_incident_field_count: usize,
    /// Number of desktop access profiles covered.
    pub certified_access_profile_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Non-pointer-reach posture.
    pub non_pointer_reach: NonPointerReachState,
    /// Support-export-evidence posture.
    pub support_export_evidence: SupportExportEvidenceState,
    /// Profile-stability posture.
    pub profile_stability: ProfileStabilityState,
    /// Release-evidence posture.
    pub release_evidence: ReleaseEvidenceFreshnessState,
    /// `true` when headless / CLI parity is preserved.
    pub headless_parity_preserved: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light access-parity dashboard the palette / menu / keybinding UI / help / Support Center / CLI
/// tooling reads to auto-narrow a surface's accessibility / export claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessParityDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<AccessParityDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Command / accessibility automation refs that consume the dashboard.
    pub command_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl AccessParityDashboard {
    /// Projects the dashboard from an access-parity packet.
    pub fn from_packet(packet: &AccessParityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| AccessParityDashboardRow {
                surface_family: row.surface_family,
                surface_label: row.surface_label.clone(),
                qualification: row.qualification,
                status: row.derived_status,
                lifecycle_label: row.lifecycle_label,
                preview_class: row.preview_class,
                certified_reach_channel_count: row.certified_reach_channels.len(),
                certified_incident_field_count: row.certified_incident_fields.len(),
                certified_access_profile_count: row.certified_access_profiles.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                non_pointer_reach: row.non_pointer_reach,
                support_export_evidence: row.support_export_evidence,
                profile_stability: row.profile_stability,
                release_evidence: row.release_evidence,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .conformance_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_ACCESS_PARITY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_ACCESS_PARITY_SCHEMA_VERSION,
            dashboard_id: M5_ACCESS_PARITY_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            command_automation_refs: packet.command_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 access-parity dashboard serializes")
    }
}

/// Support-export wrapper for the access-parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: AccessParityPacket,
    /// Dashboard quoted in full.
    pub dashboard: AccessParityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl AccessParitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each surface family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the palette / keybinding / help tooling —
    /// can name the same surface and waiver the runtime certified.
    pub fn from_packet(support_export_id: impl Into<String>, packet: AccessParityPacket) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.surface_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_ACCESS_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ACCESS_PARITY_SCHEMA_VERSION,
            shared_contract_ref: M5_ACCESS_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_access_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessParityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen discoverability matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family access-parity rows.
    pub rows: Vec<AccessParityRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The access-parity packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds an [`AccessParityPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-family access-parity rows.
///
/// Each row's derived status and conformance causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the auto-narrowing
/// cannot be asserted.
pub fn build_m5_access_parity_packet(input: AccessParityInput) -> AccessParityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<AccessParityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<AccessParityFinding> = Vec::new();

    // Every surface family must carry an access-parity row.
    let present: BTreeSet<M5CommandSurfaceFamily> =
        rows.iter().map(|row| row.surface_family).collect();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(AccessParityFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_surface_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AccessParityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AccessParityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AccessParityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(AccessParityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<AccessParityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<AccessParityCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_access_dimensions: Vec<String> = REQUIRED_ACCESS_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_reach_channels: Vec<String> = REQUIRED_REACH_CHANNELS
        .iter()
        .map(|channel| channel.as_str().to_owned())
        .collect();
    let required_incident_fields: Vec<String> = REQUIRED_INCIDENT_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    let required_access_profiles: Vec<String> = REQUIRED_ACCESS_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    let required_surface_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = AccessParityPacket {
        record_kind: M5_ACCESS_PARITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_ACCESS_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_ACCESS_PARITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_ACCESS_PARITY_PACKET_ID.to_owned(),
        source_schema_ref: M5_ACCESS_PARITY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Keyboard, screen-reader, touch, and support-export parity for every claimed M5 command \
                   surface: each of the ten governed surface families — menu items, menu groups, context \
                   menus, command bars, keybinding resolver layers, conflict review sheets, import-bridge \
                   rows, disabled-command explainers, leader/sequence help overlays, and \
                   command-documentation surfaces — certified so a keyboard-first user, screen-reader \
                   listener, touch user, or support reviewer can reach and diagnose the same command: the \
                   surface is fully keyboard- and screen-reader-addressable with a focus-return and touch / \
                   context-action equivalent; a structured, copy-safe support-export reconstructs the \
                   command id, source layer, conflict / blocker reason, lifecycle state, and help-anchor \
                   references without a screenshot; the surface stays reachable and stable across the \
                   reduced-motion, high-zoom, compact-layout, and multi-window desktop profiles; and the \
                   parity checks gate release evidence so a stale help anchor, missing narration, or \
                   hover-only regression auto-narrows the claim — across every declared consumer surface, \
                   with the same parity preserved in headless/CLI execution, each surface's green/yellow/red \
                   claim auto-narrowed from its four accessibility / export postures, and any surface that \
                   renders hover-only, cannot reconstruct its blocked-state evidence, becomes unstable on a \
                   profile, or ships a stale anchor blocked from a stable claim."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_ACCESS_PARITY_MATRIX_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_ACCESS_PARITY_MATRIX_DOC_REF.to_owned(),
        command_descriptor_ref: M5_ACCESS_PARITY_COMMAND_DESCRIPTOR_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_access_dimensions,
        required_reach_channels,
        required_incident_fields,
        required_access_profiles,
        required_surface_families,
        rows,
        covered_surface_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        conformance_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        command_automation_refs: vec![
            "command_status.access_parity_registry".to_owned(),
            "accessibility_automation.auto_narrow.access_parity_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.discoverability_access_parity".to_owned(),
            M5_ACCESS_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_ACCESS_PARITY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-discoverability-access-parity".to_owned()],
        published_report_ref: M5_ACCESS_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_ACCESS_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_ACCESS_PARITY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_ACCESS_PARITY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("access-parity packet serializes"),
    ) {
        blocking_findings.push(AccessParityFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_access_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum AccessParityValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The declared required access dimensions do not match the lane constants.
    RequiredAccessDimensionsStale,
    /// The declared required reach channels do not match the lane constants.
    RequiredReachChannelsStale,
    /// The declared required incident fields do not match the lane constants.
    RequiredIncidentFieldsStale,
    /// The declared required access profiles do not match the lane constants.
    RequiredAccessProfilesStale,
    /// The declared required surface families do not match the lane constants.
    RequiredSurfaceFamiliesStale,
    /// The rows do not cover all ten surface families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared conformance causes do not match the recomputed causes.
    ConformanceCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the access-parity invariants.
///
/// The checks encode the track invariant and acceptance criteria: every surface family carries a current
/// access-parity row; each row's status is the derived value, never asserted; a green row cannot keep a
/// claim while it renders hover-only or drops screen-reader narration, cannot reconstruct its blocked-state
/// evidence from durable capture, becomes unreachable or unstable on a claimed profile, ships a stale help
/// anchor or unblocked hover-only regression, loses headless/CLI parity, fails to certify all five
/// non-pointer reach channels, fails to capture all five accessibility-incident fields, fails to stay stable
/// in all four desktop profiles, or fails to certify every declared consumer surface; and a disclosed
/// narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_access_parity_packet(
    packet: &AccessParityPacket,
) -> Result<(), Vec<AccessParityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(AccessParityValidationError::NoRows);
    }
    if packet.record_kind != M5_ACCESS_PARITY_PACKET_RECORD_KIND {
        errors.push(AccessParityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_ACCESS_PARITY_SCHEMA_VERSION {
        errors.push(AccessParityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(AccessParityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(AccessParityValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_ACCESS_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_access_dimensions != expected_dimensions {
        errors.push(AccessParityValidationError::RequiredAccessDimensionsStale);
    }
    let expected_reach_channels: Vec<String> = REQUIRED_REACH_CHANNELS
        .iter()
        .map(|channel| channel.as_str().to_owned())
        .collect();
    if packet.required_reach_channels != expected_reach_channels {
        errors.push(AccessParityValidationError::RequiredReachChannelsStale);
    }
    let expected_incident_fields: Vec<String> = REQUIRED_INCIDENT_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    if packet.required_incident_fields != expected_incident_fields {
        errors.push(AccessParityValidationError::RequiredIncidentFieldsStale);
    }
    let expected_access_profiles: Vec<String> = REQUIRED_ACCESS_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    if packet.required_access_profiles != expected_access_profiles {
        errors.push(AccessParityValidationError::RequiredAccessProfilesStale);
    }
    let expected_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_surface_families != expected_families {
        errors.push(AccessParityValidationError::RequiredSurfaceFamiliesStale);
    }

    let present: BTreeSet<M5CommandSurfaceFamily> =
        packet.rows.iter().map(|row| row.surface_family).collect();
    let coverage_complete = REQUIRED_SURFACE_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_SURFACE_FAMILIES.len() {
        errors.push(AccessParityValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_surface_families {
        errors.push(AccessParityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AccessParityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AccessParityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AccessParityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(AccessParityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<AccessParityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(AccessParityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<AccessParityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(AccessParityValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<AccessParityFinding> = Vec::new();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(AccessParityFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(AccessParityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("access-parity packet serializes"),
    ) {
        recomputed.push(AccessParityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(AccessParityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(AccessParityValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(AccessParityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(AccessParityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(AccessParityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(AccessParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
