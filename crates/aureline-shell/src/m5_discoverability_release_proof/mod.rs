//! Menu-affordance, keybinding-resolver, leader-help, and command-documentation release-evidence proof for
//! every claimed M5 command surface.
//!
//! The [frozen discoverability matrix][matrix] already binds each governed M5 last-mile command-discovery
//! surface — menu items, menu groups, context menus, command bars, keybinding resolver layers, conflict
//! review sheets, import-bridge rows, disabled-command explainers, leader/sequence help overlays, and
//! command-documentation surfaces — to one canonical command record, and freezes the required-label,
//! why-unavailable-reason, feature-family, discovery-channel, and downgrade-trigger vocabulary those
//! surfaces project from. The sibling parity lanes certify menu/context-menu parity, keybinding-resolver
//! inspectability, leader/blocked-command explainability, and command-documentation truth one dimension at a
//! time. This lane is the **release-evidence publication capstone** that bundles all four discoverability
//! truth dimensions into one release-evidence proof, ties every claimed surface family to its current
//! menu/help/keybinding/leader/documentation proof, and auto-narrows a surface whose parity, narration, or
//! docs/help anchors are stale or missing — so a discoverability regression is detected mechanically before
//! a stable/beta claim widens.
//!
//! For every surface family the lane certifies the four discoverability truth dimensions the acceptance
//! criteria demand a current proof row for:
//!
//! - **menu-affordance truth** — the canonical label, shortcut hint, blocked-state reason, and authority
//!   posture project identically across the menu, context-menu, command-bar, and convenience-affordance
//!   reaches rather than inventing an alternate label or widening authority
//!   ([`MenuAffordanceTruthState`], maps the command-surface-parity proof);
//! - **keybinding-resolver truth** — shortcut resolution is inspectable: the winning binding, the shadowed
//!   candidates, the import-bridge outcome, and the leader/sequence precedence are all visible rather than
//!   hidden behind private knowledge ([`KeybindingResolverTruthState`], maps the keybinding-resolver-inspector
//!   proof);
//! - **leader-help truth** — leader/partial-sequence overlays and disabled-command / why-unavailable
//!   explainers narrate blocked and in-progress keyboard-first intent with shared remediation rather than
//!   failing silently or inventing surface-local prose ([`LeaderHelpTruthState`], maps the command-explainer
//!   proof);
//! - and **command-documentation truth** — the command id, aliases, lifecycle/deprecation state, supported
//!   surfaces, and canonical examples the shipped command record carries stay fresh and consistent across
//!   every reach rather than shipping a stale or mismatched doc record ([`CommandDocumentationTruthState`],
//!   maps the command-documentation proof).
//!
//! Three records carry the truth:
//!
//! - the per-family **release-proof row** ([`ReleaseProofRow`]): one row per [`M5CommandSurfaceFamily`]
//!   naming the canonical command binding it projects from, the proof dimensions it certifies, the desktop
//!   profiles it certifies across (drawn from the reused [`M5DesktopProfile`] vocabulary), the consumer
//!   surfaces it evaluated, its four discoverability-truth postures, whether the same proof survives
//!   headless/CLI execution, any active waiver, and a derived green/yellow/red [`ReleaseProofStatus`].
//! - the release-proof **packet** ([`ReleaseProofPacket`]): the full set of rows with derived per-row status,
//!   aggregate green/yellow/red counts, the active waivers, the exact conformance causes
//!   ([`ReleaseProofCause`]), the blocking findings the lane refuses to ship with, the release-evidence index
//!   anchor, and the sibling proof lanes it bundles.
//! - the release-proof **dashboard** ([`ReleaseProofDashboard`]): a light projection the release center /
//!   palette / menu / keybinding UI / help / Support Center / CLI tooling reads to auto-narrow a surface's
//!   discoverability claim when its release-evidence proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment a surface
//! discloses a reduced affordance hint, a reduced resolver detail, a reduced explainer detail (a waivered
//! narrowing), or a reduced doc detail; it drops to `red` if a surface invents an alternate label or widens
//! authority, hides the winning or shadowed binding, lets blocked intent fail silently or go generic, ships a
//! stale or mismatched doc record, loses the same proof in a headless/CLI execution, or fails to certify all
//! four proof dimensions, all six desktop profiles, or every declared consumer surface. That derivation is
//! the auto-narrowing the acceptance criteria require, and the proof-dimension, profile, and consumer-surface
//! completeness checks are the conformance lints that gate a stable discoverability claim.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local paths,
//! raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary, counts, refs,
//! and short labels. The surface-family, canonical-command-binding, required-label, lifecycle-label,
//! preview-class, feature-family, consumer-surface, downgrade-trigger, and qualification vocabulary is
//! re-exported by reference from the already frozen [matrix], the desktop-profile vocabulary is re-exported
//! from the [profile certification][profiles], and every family's binding is pulled straight from the
//! matrix's seeded packet, so this lane mints no parallel command vocabulary and cannot certify a surface the
//! matrix does not anchor. Only the release-proof-specific vocabulary ([`M5DiscoverabilityProofDimension`],
//! [`ReleaseProofStatus`], [`MenuAffordanceTruthState`], [`KeybindingResolverTruthState`],
//! [`LeaderHelpTruthState`], [`CommandDocumentationTruthState`], [`ReleaseProofWaiver`],
//! [`ReleaseProofCause`], [`ReleaseProofFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix
//! [profiles]: crate::m5_desktop_profile_certification

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix as matrix;

pub use crate::m5_desktop_profile_certification::M5DesktopProfile;
pub use matrix::{
    M5CanonicalCommandBinding, M5CommandSurfaceFamily, M5DisabledReasonMode,
    M5DiscoverabilityDowngradeTrigger, M5DiscoveryChannel, M5FeatureFamily, M5LifecycleLabel,
    M5PreviewClass, M5RequiredLabel, M5SurfaceQualificationClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_discoverability_release_proof_packet,
    seeded_m5_discoverability_release_proof_packet_doc_record_stale_blocked,
    seeded_m5_discoverability_release_proof_packet_explainer_blocked_intent_silent_blocked,
    seeded_m5_discoverability_release_proof_packet_import_bridge_headless_parity_lost_blocked,
    seeded_m5_discoverability_release_proof_packet_menu_item_alternate_label_blocked,
    seeded_m5_discoverability_release_proof_packet_resolver_binding_hidden_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_RELEASE_PROOF_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_RELEASE_PROOF_SHARED_CONTRACT_REF: &str =
    "commands:m5_discoverability_release_proof:v1";

/// Stable record kind for [`ReleaseProofPacket`] payloads.
pub const M5_RELEASE_PROOF_PACKET_RECORD_KIND: &str =
    "commands_m5_discoverability_release_proof_packet_record";

/// Stable record kind for [`ReleaseProofDashboard`] payloads.
pub const M5_RELEASE_PROOF_DASHBOARD_RECORD_KIND: &str =
    "commands_m5_discoverability_release_proof_dashboard_record";

/// Stable record kind for [`ReleaseProofSupportExport`] payloads.
pub const M5_RELEASE_PROOF_SUPPORT_EXPORT_RECORD_KIND: &str =
    "commands_m5_discoverability_release_proof_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_RELEASE_PROOF_PACKET_ID: &str = "m5-discoverability-release-proof:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_RELEASE_PROOF_DASHBOARD_ID: &str =
    "m5-discoverability-release-proof-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_RELEASE_PROOF_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-discoverability-release-proof:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_RELEASE_PROOF_SOURCE_SCHEMA_REF: &str =
    "schemas/commands/m5-discoverability-release-proof.schema.json";

/// Published markdown report ref reviewers reopen the release-evidence proof from.
pub const M5_RELEASE_PROOF_PUBLISHED_REPORT_REF: &str =
    "artifacts/commands/m5-discoverability-release-proof.md";

/// Published release-proof-packet artifact ref.
pub const M5_RELEASE_PROOF_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-discoverability-release-proof-proof/packet.json";

/// Published release-proof-dashboard artifact ref.
pub const M5_RELEASE_PROOF_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-discoverability-release-proof-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_RELEASE_PROOF_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-discoverability-release-proof-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_RELEASE_PROOF_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-discoverability-release-proof-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_RELEASE_PROOF_PUBLISHED_DOC_REF: &str =
    "docs/commands/m5_discoverability_release_proof_contract.md";

/// The release-evidence index anchor this lane is published under.
pub const M5_RELEASE_PROOF_RELEASE_EVIDENCE_INDEX_REF: &str =
    "release_center.discoverability_release_evidence_index";

/// Repo-relative ref to the frozen discoverability boundary schema.
pub const M5_RELEASE_PROOF_MATRIX_SCHEMA_REF: &str = matrix::M5_DISCOVERABILITY_SCHEMA_REF;

/// Frozen discoverability contract doc this proof mirrors.
pub const M5_RELEASE_PROOF_MATRIX_DOC_REF: &str = matrix::M5_DISCOVERABILITY_DOC_REF;

/// Canonical command-descriptor schema every certified surface projects from.
pub const M5_RELEASE_PROOF_COMMAND_DESCRIPTOR_REF: &str =
    matrix::M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF;

/// The sibling discoverability proof lanes this release-evidence proof bundles, in canonical order — the
/// menu/context-menu/command-bar parity proof, the keybinding-resolver-inspector proof, the leader/blocked
/// command-explainer proof, and the command-documentation proof. Naming them keeps the release-evidence index
/// pointing at the same per-dimension proofs this capstone rolls up.
pub const M5_RELEASE_PROOF_LINKED_PROOF_LANE_REFS: [&str; 4] = [
    "artifacts/release/m5-command-surface-parity-proof/packet.json",
    "artifacts/release/m5-keybinding-resolver-inspectors-proof/packet.json",
    "artifacts/release/m5-command-explainers-proof/packet.json",
    "artifacts/release/m5-command-documentation-proof/packet.json",
];

/// Every command-surface family the certification must cover, in canonical order. A certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_SURFACE_FAMILIES: [M5CommandSurfaceFamily; 10] = M5CommandSurfaceFamily::ALL;

/// Every discoverability proof dimension each family row certifies, in canonical order.
pub const REQUIRED_PROOF_DIMENSIONS: [M5DiscoverabilityProofDimension; 4] =
    M5DiscoverabilityProofDimension::ALL;

/// Every desktop profile each family row must certify across, in canonical order.
pub const REQUIRED_PROFILES: [M5DesktopProfile; 6] = M5DesktopProfile::ALL;

/// One of the four discoverability proof dimensions each surface-family row certifies.
///
/// These are exactly the four discoverability truth dimensions the acceptance criteria require a current
/// proof row for: menu-affordance parity, keybinding-resolver inspectability, leader/blocked-command
/// explainability, and command-documentation freshness. A row that certifies fewer leaves a claimed
/// discoverability surface without a current proof and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiscoverabilityProofDimension {
    /// Menu / context-menu / command-bar / convenience-affordance parity.
    MenuAffordance,
    /// Keybinding-resolver inspectability.
    KeybindingResolver,
    /// Leader / blocked-command explainability.
    LeaderHelp,
    /// Command-documentation freshness and cross-surface naming.
    CommandDocumentation,
}

impl M5DiscoverabilityProofDimension {
    /// Every discoverability proof dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MenuAffordance,
        Self::KeybindingResolver,
        Self::LeaderHelp,
        Self::CommandDocumentation,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MenuAffordance => "menu_affordance",
            Self::KeybindingResolver => "keybinding_resolver",
            Self::LeaderHelp => "leader_help",
            Self::CommandDocumentation => "command_documentation",
        }
    }
}

/// The derived discoverability release-evidence light a command surface carries.
///
/// `green` means every discoverability truth dimension holds — menu-affordance parity, keybinding-resolver
/// inspectability, leader/blocked-command explainability, and command-documentation freshness — across every
/// declared consumer surface and every claimed desktop profile, with the same proof surviving headless/CLI
/// execution. `yellow` is a disclosed narrowing. `red` is blocked and may not keep a discoverability claim
/// until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseProofStatus {
    /// Full standing: all four discoverability truth dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl ReleaseProofStatus {
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

/// How the menu-affordance parity holds across every reach.
///
/// `menu_affordance_parity_certified` means the canonical label, shortcut hint, blocked-state reason, and
/// authority posture project identically across the menu, context-menu, command-bar, and convenience-affordance
/// reaches. `disclosed_reduced_affordance_hint` means one dense affordance renders a disclosed shortened hint
/// while still projecting the canonical label and reason (a yellow narrowing). `alternate_label_or_authority_invented`
/// means a surface invents an alternate label for a stable command or widens its authority — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MenuAffordanceTruthState {
    /// Menu-affordance parity is certified across every reach.
    MenuAffordanceParityCertified,
    /// One dense affordance takes a disclosed shortened hint.
    DisclosedReducedAffordanceHint,
    /// A surface invents an alternate label or widens authority — a blocker.
    AlternateLabelOrAuthorityInvented,
}

impl MenuAffordanceTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MenuAffordanceParityCertified => "menu_affordance_parity_certified",
            Self::DisclosedReducedAffordanceHint => "disclosed_reduced_affordance_hint",
            Self::AlternateLabelOrAuthorityInvented => "alternate_label_or_authority_invented",
        }
    }

    /// `true` when menu-affordance parity is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::MenuAffordanceParityCertified)
    }

    /// `true` when the surface took a disclosed reduced-affordance-hint narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedAffordanceHint)
    }
}

/// How keybinding resolution stays inspectable.
///
/// `shortcut_resolution_inspectable` means the winning binding, the shadowed candidates, the import-bridge
/// outcome, and the leader/sequence precedence are all inspectable. `disclosed_reduced_resolver_detail` means
/// one resolver surface folds the shadowed-candidate detail into an expandable inspector while still naming
/// the winner and its source (a yellow narrowing). `winning_or_shadowed_binding_hidden` means the winning or
/// shadowed binding is hidden so the shortcut resolution requires private knowledge — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeybindingResolverTruthState {
    /// The full shortcut resolution is inspectable.
    ShortcutResolutionInspectable,
    /// One resolver surface takes a disclosed reduced inspector detail.
    DisclosedReducedResolverDetail,
    /// The winning or shadowed binding is hidden — a blocker.
    WinningOrShadowedBindingHidden,
}

impl KeybindingResolverTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShortcutResolutionInspectable => "shortcut_resolution_inspectable",
            Self::DisclosedReducedResolverDetail => "disclosed_reduced_resolver_detail",
            Self::WinningOrShadowedBindingHidden => "winning_or_shadowed_binding_hidden",
        }
    }

    /// `true` when keybinding resolution is inspectable at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ShortcutResolutionInspectable)
    }

    /// `true` when the surface took a disclosed reduced-resolver-detail narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedResolverDetail)
    }
}

/// How leader/blocked-command intent stays explainable.
///
/// `leader_and_blocked_explainer_certified` means leader/partial-sequence overlays and disabled-command /
/// why-unavailable explainers narrate the blocked and in-progress keyboard-first intent with shared
/// remediation. `disclosed_reduced_explainer_detail` means one surface renders a disclosed reduced explainer
/// — the next-safe-action detail folds into an expandable note while still naming the blocker class (a yellow
/// narrowing that **requires an active waiver**). `blocked_intent_silent_or_generic` means a blocked command
/// fails silently or shows generic prose without the shared blocker reason — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaderHelpTruthState {
    /// The leader overlay and blocked-command explainer are certified.
    LeaderAndBlockedExplainerCertified,
    /// One surface takes a disclosed, waivered reduced explainer detail.
    DisclosedReducedExplainerDetail,
    /// A blocked command fails silently or goes generic — a blocker.
    BlockedIntentSilentOrGeneric,
}

impl LeaderHelpTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeaderAndBlockedExplainerCertified => "leader_and_blocked_explainer_certified",
            Self::DisclosedReducedExplainerDetail => "disclosed_reduced_explainer_detail",
            Self::BlockedIntentSilentOrGeneric => "blocked_intent_silent_or_generic",
        }
    }

    /// `true` when leader/blocked-command explainability is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::LeaderAndBlockedExplainerCertified)
    }

    /// `true` when the surface took a disclosed reduced-explainer-detail narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedExplainerDetail)
    }
}

/// How command-documentation truth stays fresh across every reach.
///
/// `command_doc_record_certified` means the command id, aliases, lifecycle/deprecation state, supported
/// surfaces, and canonical examples the shipped command record carries stay fresh and consistent across every
/// reach. `disclosed_reduced_doc_detail` means one legacy doc surface renders a disclosed reduced form —
/// the example set folds into a "see full docs" link while the command id and lifecycle stay present (a
/// yellow narrowing). `doc_record_stale_or_mismatched` means a doc surface ships a stale or mismatched
/// command record, hiding the lifecycle or deprecation truth — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandDocumentationTruthState {
    /// The command-documentation record is certified fresh and consistent.
    CommandDocRecordCertified,
    /// One legacy doc surface takes a disclosed reduced doc detail.
    DisclosedReducedDocDetail,
    /// A doc surface ships a stale or mismatched command record — a blocker.
    DocRecordStaleOrMismatched,
}

impl CommandDocumentationTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandDocRecordCertified => "command_doc_record_certified",
            Self::DisclosedReducedDocDetail => "disclosed_reduced_doc_detail",
            Self::DocRecordStaleOrMismatched => "doc_record_stale_or_mismatched",
        }
    }

    /// `true` when command-documentation truth is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CommandDocRecordCertified)
    }

    /// `true` when the surface took a disclosed reduced-doc-detail narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedDocDetail)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather than
/// blocked — never lets an invented label, a hidden binding, a silent blocked command, or a stale doc record
/// hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseProofWaiver {
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

impl ReleaseProofWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a surface family's discoverability release-evidence proof.
///
/// The trigger token mirrors the frozen [`M5DiscoverabilityDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseProofCause {
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

impl ReleaseProofCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One surface family, certified across its menu-affordance, keybinding-resolver, leader-help, and
/// command-documentation discoverability truth dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseProofRow {
    /// The surface family being certified.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short reviewer-facing family label.
    pub surface_label: String,
    /// Qualification class the matrix earned for the surface. Pulled from the matrix.
    pub qualification: M5SurfaceQualificationClass,
    /// Owner role accountable for keeping this surface's proof governed. Pulled from the matrix.
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
    /// The discoverability proof dimensions this row certifies (must be all four).
    pub certified_proof_dimensions: Vec<M5DiscoverabilityProofDimension>,
    /// The desktop profiles this row certifies across (must be all six).
    pub certified_profiles: Vec<M5DesktopProfile>,
    /// Consumer surfaces the matrix declares the surface must project to.
    pub required_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Menu-affordance-truth posture.
    pub menu_affordance_truth: MenuAffordanceTruthState,
    /// Keybinding-resolver-truth posture.
    pub keybinding_resolver_truth: KeybindingResolverTruthState,
    /// Leader-help-truth posture.
    pub leader_help_truth: LeaderHelpTruthState,
    /// Command-documentation-truth posture.
    pub command_documentation_truth: CommandDocumentationTruthState,
    /// `true` when the same proof survives a headless / CLI execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the surface. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced explainer detail is in force.
    pub active_waiver: Option<ReleaseProofWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: ReleaseProofStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<ReleaseProofCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl ReleaseProofRow {
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

    /// `true` when the row certifies every one of the four discoverability proof dimensions — the structural
    /// proof that the surface has a current proof row for every named truth dimension.
    pub fn proof_dimensions_complete(&self) -> bool {
        complete_tokens(
            &self.certified_proof_dimensions,
            |dimension| dimension.as_str(),
            &REQUIRED_PROOF_DIMENSIONS,
            |dimension| dimension.as_str(),
        )
    }

    /// `true` when the row certifies across every one of the six desktop profiles — the structural proof
    /// that the release evidence ties the proof to every claimed profile.
    pub fn profiles_complete(&self) -> bool {
        complete_tokens(
            &self.certified_profiles,
            |profile| profile.as_str(),
            &REQUIRED_PROFILES,
            |profile| profile.as_str(),
        )
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.proof_dimensions_complete() {
            return true;
        }
        if !self.profiles_complete() {
            return true;
        }
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.menu_affordance_truth,
            MenuAffordanceTruthState::AlternateLabelOrAuthorityInvented
        ) {
            return true;
        }
        if matches!(
            self.keybinding_resolver_truth,
            KeybindingResolverTruthState::WinningOrShadowedBindingHidden
        ) {
            return true;
        }
        if matches!(
            self.leader_help_truth,
            LeaderHelpTruthState::BlockedIntentSilentOrGeneric
        ) {
            return true;
        }
        if matches!(
            self.command_documentation_truth,
            CommandDocumentationTruthState::DocRecordStaleOrMismatched
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.menu_affordance_truth.is_disclosed_narrowing()
            || self.keybinding_resolver_truth.is_disclosed_narrowing()
            || self.leader_help_truth.is_disclosed_narrowing()
            || self.command_documentation_truth.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the discoverability truth posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> ReleaseProofStatus {
        if self.has_hard_blocker() {
            ReleaseProofStatus::Red
        } else if self.has_narrowing() {
            ReleaseProofStatus::Yellow
        } else {
            ReleaseProofStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (menu affordance,
    /// keybinding resolver, leader help, command documentation, then structural completeness and headless
    /// parity).
    pub fn recompute_causes(&self) -> Vec<ReleaseProofCause> {
        let mut causes = Vec::new();
        match self.menu_affordance_truth {
            MenuAffordanceTruthState::MenuAffordanceParityCertified => {}
            MenuAffordanceTruthState::DisclosedReducedAffordanceHint => {
                causes.push(ReleaseProofCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                    disclosed: true,
                    detail: "One dense affordance renders a disclosed shortened shortcut / label hint while \
                             still projecting the canonical label and blocked-state reason, so the \
                             menu-affordance parity is narrowed and disclosed rather than inventing an \
                             alternate label."
                        .to_owned(),
                });
            }
            MenuAffordanceTruthState::AlternateLabelOrAuthorityInvented => {
                causes.push(ReleaseProofCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                    disclosed: false,
                    detail: "A surface invents an alternate label for a stable command or widens its \
                             authority posture, so the same action changes its name or reach depending on \
                             where it is reached from."
                        .to_owned(),
                });
            }
        }
        match self.keybinding_resolver_truth {
            KeybindingResolverTruthState::ShortcutResolutionInspectable => {}
            KeybindingResolverTruthState::DisclosedReducedResolverDetail => {
                causes.push(ReleaseProofCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                    disclosed: true,
                    detail: "One resolver surface folds the shadowed-candidate detail into an expandable \
                             inspector while still naming the winning binding and its source layer, so the \
                             shortcut resolution is narrowed and disclosed rather than hidden."
                        .to_owned(),
                });
            }
            KeybindingResolverTruthState::WinningOrShadowedBindingHidden => {
                causes.push(ReleaseProofCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ConflictWinnerAmbiguous,
                    disclosed: false,
                    detail: "The winning or shadowed binding is hidden, so which shortcut wins — and what it \
                             shadowed or how an import translated it — requires private knowledge rather than \
                             an inspectable resolver."
                        .to_owned(),
                });
            }
        }
        match self.leader_help_truth {
            LeaderHelpTruthState::LeaderAndBlockedExplainerCertified => {}
            LeaderHelpTruthState::DisclosedReducedExplainerDetail => {
                causes.push(ReleaseProofCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                    disclosed: true,
                    detail: "One surface renders a disclosed, waivered reduced explainer — the next-safe-action \
                             detail folds into an expandable note while the blocker class and command id stay \
                             present — so the leader / blocked-command explanation is narrowed and disclosed \
                             rather than silent."
                        .to_owned(),
                });
            }
            LeaderHelpTruthState::BlockedIntentSilentOrGeneric => {
                causes.push(ReleaseProofCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::DisabledReasonHidden,
                    disclosed: false,
                    detail: "A blocked command fails silently or shows generic prose without the shared \
                             blocker reason and next-safe-action, so a keyboard-first user cannot tell why \
                             the command is unavailable or how to proceed."
                        .to_owned(),
                });
            }
        }
        match self.command_documentation_truth {
            CommandDocumentationTruthState::CommandDocRecordCertified => {}
            CommandDocumentationTruthState::DisclosedReducedDocDetail => {
                causes.push(ReleaseProofCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One legacy doc surface folds the example set into a \"see full docs\" link while \
                             the command id, lifecycle state, and supported surfaces stay present, so the \
                             command-documentation truth is narrowed and disclosed rather than stale."
                        .to_owned(),
                });
            }
            CommandDocumentationTruthState::DocRecordStaleOrMismatched => {
                causes.push(ReleaseProofCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::LifecycleOrDeprecationHidden,
                    disclosed: false,
                    detail: "A doc surface ships a stale or mismatched command record — the lifecycle / \
                             deprecation state, aliases, or supported surfaces no longer match the shipped \
                             command — so the documentation overclaims a command the runtime no longer honors."
                        .to_owned(),
                });
            }
        }
        if !self.proof_dimensions_complete() {
            causes.push(ReleaseProofCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                disclosed: false,
                detail: "The surface does not certify all four discoverability proof dimensions — \
                         menu-affordance parity, keybinding-resolver inspectability, leader/blocked-command \
                         explainability, and command-documentation freshness — so a claimed truth dimension \
                         has no current proof."
                    .to_owned(),
            });
        }
        if !self.profiles_complete() {
            causes.push(ReleaseProofCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                disclosed: false,
                detail: "The release evidence does not tie the proof to all six claimed desktop profiles, so \
                         a discoverability regression could widen a claim on an uncertified profile."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(ReleaseProofCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "A headless / CLI execution of this surface lost the shared discoverability proof, \
                         so the same command explains a different menu / keybinding / leader / documentation \
                         truth depending on how it is reached."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced explainer detail may only stay yellow (rather than red) when a waiver discloses
    /// it — reducing how a blocked command explains itself is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.leader_help_truth,
            LeaderHelpTruthState::DisclosedReducedExplainerDetail
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<ReleaseProofFinding> {
        let mut findings = Vec::new();
        let family = self.surface_family.as_str().to_owned();

        if !self.proof_dimensions_complete() {
            findings.push(ReleaseProofFinding::ProofDimensionsIncomplete {
                family: family.clone(),
            });
        }
        if !self.profiles_complete() {
            findings.push(ReleaseProofFinding::ProfilesIncomplete {
                family: family.clone(),
            });
        }
        if !self.consumer_surfaces_complete() {
            findings.push(ReleaseProofFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(ReleaseProofFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.menu_affordance_truth,
            MenuAffordanceTruthState::AlternateLabelOrAuthorityInvented
        ) {
            findings.push(ReleaseProofFinding::MenuAffordanceTruthBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.keybinding_resolver_truth,
            KeybindingResolverTruthState::WinningOrShadowedBindingHidden
        ) {
            findings.push(ReleaseProofFinding::KeybindingResolverTruthBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.leader_help_truth,
            LeaderHelpTruthState::BlockedIntentSilentOrGeneric
        ) {
            findings.push(ReleaseProofFinding::LeaderHelpTruthBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.command_documentation_truth,
            CommandDocumentationTruthState::DocRecordStaleOrMismatched
        ) {
            findings.push(ReleaseProofFinding::CommandDocumentationTruthBroken {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, ReleaseProofStatus::Green) && !self.has_reason() {
            findings.push(ReleaseProofFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(ReleaseProofFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.surface_family != self.surface_family {
                findings.push(ReleaseProofFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(ReleaseProofFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(ReleaseProofFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(ReleaseProofFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} menu={} resolver={} leader={} docs={} headless={} lifecycle={} preview={} dimensions={} profiles={} surfaces={} waiver={}",
            self.surface_family.as_str(),
            self.derived_status.as_str(),
            self.menu_affordance_truth.as_str(),
            self.keybinding_resolver_truth.as_str(),
            self.leader_help_truth.as_str(),
            self.command_documentation_truth.as_str(),
            self.headless_parity_preserved,
            self.lifecycle_label.as_str(),
            self.preview_class.as_str(),
            self.certified_proof_dimensions.len(),
            self.certified_profiles.len(),
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

/// A blocking finding the release-evidence certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ReleaseProofFinding {
    /// A surface family has no release-proof row.
    SurfaceFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row did not certify every discoverability proof dimension.
    ProofDimensionsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not certify across every desktop profile.
    ProfilesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless / CLI execution lost the shared discoverability proof.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// A surface invents an alternate label or widens authority.
    MenuAffordanceTruthBroken {
        /// The family token.
        family: String,
    },
    /// A surface hides the winning or shadowed binding.
    KeybindingResolverTruthBroken {
        /// The family token.
        family: String,
    },
    /// A blocked command fails silently or goes generic.
    LeaderHelpTruthBroken {
        /// The family token.
        family: String,
    },
    /// A doc surface ships a stale or mismatched command record.
    CommandDocumentationTruthBroken {
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

impl ReleaseProofFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::SurfaceFamilyMissing { .. } => "surface_family_missing",
            Self::ProofDimensionsIncomplete { .. } => "proof_dimensions_incomplete",
            Self::ProfilesIncomplete { .. } => "profiles_incomplete",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::MenuAffordanceTruthBroken { .. } => "menu_affordance_truth_broken",
            Self::KeybindingResolverTruthBroken { .. } => "keybinding_resolver_truth_broken",
            Self::LeaderHelpTruthBroken { .. } => "leader_help_truth_broken",
            Self::CommandDocumentationTruthBroken { .. } => "command_documentation_truth_broken",
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
            | Self::ProofDimensionsIncomplete { family }
            | Self::ProfilesIncomplete { family }
            | Self::ConsumerSurfacesIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::MenuAffordanceTruthBroken { family }
            | Self::KeybindingResolverTruthBroken { family }
            | Self::LeaderHelpTruthBroken { family }
            | Self::CommandDocumentationTruthBroken { family }
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

/// The release-proof packet shared by the release center / palette / menu / keybinding UI / help / Support
/// Center / CLI tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseProofPacket {
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
    /// The release-evidence index anchor this lane is published under.
    pub release_evidence_index_ref: String,
    /// The sibling per-dimension proof lanes this release-evidence proof bundles.
    pub linked_proof_lane_refs: Vec<String>,
    /// The four discoverability proof dimensions every family row certifies.
    pub required_proof_dimensions: Vec<String>,
    /// The six desktop profiles every family row must certify across.
    pub required_profiles: Vec<String>,
    /// The ten surface families the certification must cover.
    pub required_surface_families: Vec<String>,
    /// Per-family release-proof rows, in canonical order.
    pub rows: Vec<ReleaseProofRow>,
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
    pub active_waivers: Vec<ReleaseProofWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<ReleaseProofCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<ReleaseProofFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Command / release automation refs that consume this packet to auto-narrow a surface.
    pub command_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help / onboarding refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published release-proof-packet ref.
    pub published_packet_ref: String,
    /// Published release-proof-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ReleaseProofPacket {
    /// Returns the release-proof row for `family`, if present.
    pub fn row(&self, family: M5CommandSurfaceFamily) -> Option<&ReleaseProofRow> {
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
                "matrix={} build={} channel={} index={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.release_evidence_index_ref,
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

    /// Projects the light release-proof dashboard the command / release automation consumes.
    pub fn dashboard(&self) -> ReleaseProofDashboard {
        ReleaseProofDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 release-proof packet serializes")
    }

    /// Deterministic, machine-readable proof CSV: one row per surface family naming its status, the four
    /// discoverability-truth postures, headless parity, the lifecycle label and preview class, the dimension
    /// / profile counts, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,status,menu_affordance_truth,keybinding_resolver_truth,leader_help_truth,command_documentation_truth,headless_parity,lifecycle,preview_class,proof_dimensions,profiles,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.derived_status.as_str(),
                row.menu_affordance_truth.as_str(),
                row.keybinding_resolver_truth.as_str(),
                row.leader_help_truth.as_str(),
                row.command_documentation_truth.as_str(),
                row.headless_parity_preserved,
                row.lifecycle_label.as_str(),
                row.preview_class.as_str(),
                row.certified_proof_dimensions.len(),
                row.certified_profiles.len(),
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
            "# M5 discoverability release proof: menu-affordance, keybinding-resolver, leader-help, and command-documentation truth for every claimed M5 command surface\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_discoverability_release_proof`](../../crates/aureline-shell/src/m5_discoverability_release_proof/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_release_proof -- markdown > \\\n  artifacts/commands/m5-discoverability-release-proof.md\n",
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
            "- Release-evidence index: `{}`\n",
            self.release_evidence_index_ref
        ));
        out.push_str(&format!(
            "- Bundled proof lanes: {}\n",
            self.linked_proof_lane_refs
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Required proof dimensions: {}\n",
            self.required_proof_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Desktop profiles: {}\n",
            self.required_profiles
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

        out.push_str("## Release-proof rows\n\n");
        out.push_str(
            "| Surface family | Status | Menu affordance | Keybinding resolver | Leader help | Command documentation | Lifecycle | Headless | Waiver |\n\
             | -------------- | ------ | --------------- | ------------------- | ----------- | --------------------- | --------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.menu_affordance_truth.as_str(),
                row.keybinding_resolver_truth.as_str(),
                row.leader_help_truth.as_str(),
                row.command_documentation_truth.as_str(),
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
        let narrowed: Vec<&ReleaseProofRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, ReleaseProofStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed M5 command surface keeps a current menu-affordance, keybinding-resolver, leader-help, and command-documentation proof across every declared consumer surface and every claimed desktop profile.\n\n",
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_release_proof -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_discoverability_release_proof_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light release-proof dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseProofDashboardRow {
    /// The surface family.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short family label.
    pub surface_label: String,
    /// Qualification class earned by the surface.
    pub qualification: M5SurfaceQualificationClass,
    /// Derived green/yellow/red status.
    pub status: ReleaseProofStatus,
    /// The pinned lifecycle / deprecation label.
    pub lifecycle_label: M5LifecycleLabel,
    /// The pinned preview / approval class.
    pub preview_class: M5PreviewClass,
    /// Number of discoverability proof dimensions certified.
    pub certified_proof_dimension_count: usize,
    /// Number of desktop profiles certified across.
    pub certified_profile_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Menu-affordance-truth posture.
    pub menu_affordance_truth: MenuAffordanceTruthState,
    /// Keybinding-resolver-truth posture.
    pub keybinding_resolver_truth: KeybindingResolverTruthState,
    /// Leader-help-truth posture.
    pub leader_help_truth: LeaderHelpTruthState,
    /// Command-documentation-truth posture.
    pub command_documentation_truth: CommandDocumentationTruthState,
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

/// The light release-proof dashboard the release center / palette / menu / keybinding UI / help / Support
/// Center / CLI tooling reads to auto-narrow a surface's discoverability claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseProofDashboard {
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
    pub rows: Vec<ReleaseProofDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// The release-evidence index anchor this dashboard is published under.
    pub release_evidence_index_ref: String,
    /// Command / release automation refs that consume the dashboard.
    pub command_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ReleaseProofDashboard {
    /// Projects the dashboard from a release-proof packet.
    pub fn from_packet(packet: &ReleaseProofPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| ReleaseProofDashboardRow {
                surface_family: row.surface_family,
                surface_label: row.surface_label.clone(),
                qualification: row.qualification,
                status: row.derived_status,
                lifecycle_label: row.lifecycle_label,
                preview_class: row.preview_class,
                certified_proof_dimension_count: row.certified_proof_dimensions.len(),
                certified_profile_count: row.certified_profiles.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                menu_affordance_truth: row.menu_affordance_truth,
                keybinding_resolver_truth: row.keybinding_resolver_truth,
                leader_help_truth: row.leader_help_truth,
                command_documentation_truth: row.command_documentation_truth,
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
            record_kind: M5_RELEASE_PROOF_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_RELEASE_PROOF_SCHEMA_VERSION,
            dashboard_id: M5_RELEASE_PROOF_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            release_evidence_index_ref: packet.release_evidence_index_ref.clone(),
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
        serde_json::to_string_pretty(self).expect("m5 release-proof dashboard serializes")
    }
}

/// Support-export wrapper for the release-proof packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseProofSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: ReleaseProofPacket,
    /// Dashboard quoted in full.
    pub dashboard: ReleaseProofDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ReleaseProofSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each surface family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the release center / palette / keybinding /
    /// help tooling — can name the same surface and waiver the runtime certified.
    pub fn from_packet(support_export_id: impl Into<String>, packet: ReleaseProofPacket) -> Self {
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
            record_kind: M5_RELEASE_PROOF_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_RELEASE_PROOF_SCHEMA_VERSION,
            shared_contract_ref: M5_RELEASE_PROOF_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_release_proof_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseProofInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen discoverability matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family release-proof rows.
    pub rows: Vec<ReleaseProofRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The release-proof packet carries only closed vocabulary, refs, and short labels, so raw URLs,
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

/// Builds a [`ReleaseProofPacket`] from the exact build identity, the frozen matrix ref, and the per-family
/// release-proof rows.
///
/// Each row's derived status and conformance causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the auto-narrowing
/// cannot be asserted.
pub fn build_m5_release_proof_packet(input: ReleaseProofInput) -> ReleaseProofPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<ReleaseProofRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<ReleaseProofFinding> = Vec::new();

    // Every surface family must carry a release-proof row.
    let present: BTreeSet<M5CommandSurfaceFamily> =
        rows.iter().map(|row| row.surface_family).collect();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(ReleaseProofFinding::SurfaceFamilyMissing {
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
        .filter(|row| matches!(row.derived_status, ReleaseProofStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ReleaseProofStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ReleaseProofStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(ReleaseProofFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<ReleaseProofWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<ReleaseProofCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_proof_dimensions: Vec<String> = REQUIRED_PROOF_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_profiles: Vec<String> = REQUIRED_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    let required_surface_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    let linked_proof_lane_refs: Vec<String> = M5_RELEASE_PROOF_LINKED_PROOF_LANE_REFS
        .iter()
        .map(|lane| (*lane).to_owned())
        .collect();

    let mut packet = ReleaseProofPacket {
        record_kind: M5_RELEASE_PROOF_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_RELEASE_PROOF_SCHEMA_VERSION,
        shared_contract_ref: M5_RELEASE_PROOF_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_RELEASE_PROOF_PACKET_ID.to_owned(),
        source_schema_ref: M5_RELEASE_PROOF_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Menu-affordance, keybinding-resolver, leader-help, and command-documentation \
                   release-evidence proof for every claimed M5 command surface: each of the ten governed \
                   surface families — menu items, menu groups, context menus, command bars, keybinding \
                   resolver layers, conflict review sheets, import-bridge rows, disabled-command explainers, \
                   leader/sequence help overlays, and command-documentation surfaces — bundled into one \
                   release-evidence proof so every claimed surface keeps a current proof row for all four \
                   discoverability truth dimensions: the canonical label, shortcut hint, blocked-state \
                   reason, and authority posture project identically across every reach; shortcut resolution \
                   stays inspectable; leader/partial-sequence overlays and disabled-command / why-unavailable \
                   explainers narrate blocked and in-progress keyboard-first intent with shared remediation; \
                   and the command id, aliases, lifecycle/deprecation state, supported surfaces, and \
                   canonical examples stay fresh across every reach — across every declared consumer surface \
                   and every claimed desktop profile, with the same proof preserved in headless/CLI \
                   execution, each surface's green/yellow/red claim auto-narrowed from its four discoverability \
                   postures, and any surface that invents an alternate label, hides the winning binding, lets \
                   blocked intent fail silently, or ships a stale doc record blocked from a stable claim before \
                   the claim widens."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_RELEASE_PROOF_MATRIX_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_RELEASE_PROOF_MATRIX_DOC_REF.to_owned(),
        command_descriptor_ref: M5_RELEASE_PROOF_COMMAND_DESCRIPTOR_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        release_evidence_index_ref: M5_RELEASE_PROOF_RELEASE_EVIDENCE_INDEX_REF.to_owned(),
        linked_proof_lane_refs,
        required_proof_dimensions,
        required_profiles,
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
            "command_status.release_proof_registry".to_owned(),
            "release_automation.auto_narrow.discoverability_release_proof_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            M5_RELEASE_PROOF_RELEASE_EVIDENCE_INDEX_REF.to_owned(),
            M5_RELEASE_PROOF_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_RELEASE_PROOF_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-discoverability-release-proof".to_owned()],
        published_report_ref: M5_RELEASE_PROOF_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_RELEASE_PROOF_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_RELEASE_PROOF_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_RELEASE_PROOF_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("release-proof packet serializes"),
    ) {
        blocking_findings.push(ReleaseProofFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_release_proof_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ReleaseProofValidationError {
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
    /// The packet's release-evidence index ref is empty.
    ReleaseEvidenceIndexRefMissing,
    /// The declared bundled proof lane refs do not match the lane constants.
    LinkedProofLaneRefsStale,
    /// The declared required proof dimensions do not match the lane constants.
    RequiredProofDimensionsStale,
    /// The declared required profiles do not match the lane constants.
    RequiredProfilesStale,
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

/// Validates a packet against the release-proof invariants.
///
/// The checks encode the track invariant and acceptance criteria: every surface family carries a current
/// release-proof row for menu/help/keybinding/leader/documentation truth; each row's status is the derived
/// value, never asserted; a green row cannot keep a claim while it invents an alternate label or widens
/// authority, hides the winning or shadowed binding, lets blocked intent fail silently or go generic, ships a
/// stale or mismatched doc record, loses headless/CLI parity, fails to certify all four proof dimensions,
/// fails to certify across all six desktop profiles, or fails to certify every declared consumer surface; and
/// a disclosed narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_release_proof_packet(
    packet: &ReleaseProofPacket,
) -> Result<(), Vec<ReleaseProofValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(ReleaseProofValidationError::NoRows);
    }
    if packet.record_kind != M5_RELEASE_PROOF_PACKET_RECORD_KIND {
        errors.push(ReleaseProofValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_RELEASE_PROOF_SCHEMA_VERSION {
        errors.push(ReleaseProofValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(ReleaseProofValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(ReleaseProofValidationError::MatrixPacketRefMissing);
    }
    if packet.release_evidence_index_ref.trim().is_empty() {
        errors.push(ReleaseProofValidationError::ReleaseEvidenceIndexRefMissing);
    }
    let expected_lane_refs: Vec<String> = M5_RELEASE_PROOF_LINKED_PROOF_LANE_REFS
        .iter()
        .map(|lane| (*lane).to_owned())
        .collect();
    if packet.linked_proof_lane_refs != expected_lane_refs {
        errors.push(ReleaseProofValidationError::LinkedProofLaneRefsStale);
    }
    let expected_dimensions: Vec<String> = REQUIRED_PROOF_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_proof_dimensions != expected_dimensions {
        errors.push(ReleaseProofValidationError::RequiredProofDimensionsStale);
    }
    let expected_profiles: Vec<String> = REQUIRED_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    if packet.required_profiles != expected_profiles {
        errors.push(ReleaseProofValidationError::RequiredProfilesStale);
    }
    let expected_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_surface_families != expected_families {
        errors.push(ReleaseProofValidationError::RequiredSurfaceFamiliesStale);
    }

    let present: BTreeSet<M5CommandSurfaceFamily> =
        packet.rows.iter().map(|row| row.surface_family).collect();
    let coverage_complete = REQUIRED_SURFACE_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_SURFACE_FAMILIES.len() {
        errors.push(ReleaseProofValidationError::CoverageIncomplete);
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
        errors.push(ReleaseProofValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ReleaseProofStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ReleaseProofStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ReleaseProofStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(ReleaseProofValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<ReleaseProofWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(ReleaseProofValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<ReleaseProofCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(ReleaseProofValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<ReleaseProofFinding> = Vec::new();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(ReleaseProofFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(ReleaseProofFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("release-proof packet serializes"),
    ) {
        recomputed.push(ReleaseProofFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(ReleaseProofValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(ReleaseProofValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(ReleaseProofValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(ReleaseProofValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(ReleaseProofValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(ReleaseProofValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
