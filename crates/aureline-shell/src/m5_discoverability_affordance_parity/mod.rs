//! Button, inline-affordance, tooltip, onboarding-tip, AI/voice-hint, and companion-handoff parity for
//! every claimed M5 command action.
//!
//! The [frozen discoverability matrix][matrix] already binds each governed M5 last-mile command-discovery
//! surface — menu items, menu groups, context menus, command bars, keybinding resolver layers, conflict
//! review sheets, import-bridge rows, disabled-command explainers, leader/sequence help overlays, and
//! command-documentation surfaces — to one canonical command record, and freezes the required-label,
//! preview-class, feature-family, discovery-channel, and downgrade-trigger vocabulary those surfaces project
//! from. This lane is the **convenience-affordance parity capstone**: it certifies that the last-mile
//! *convenience affordances* around those commands — a primary/action button's text, an inline
//! quick-action-card affordance, a tooltip / hovercard label, an onboarding tip reference, an AI hint
//! string, a voice hint string, and a companion / browser handoff affordance — all reuse the same one
//! command record rather than inventing a convenience-specific label, lifecycle language, side-effect story,
//! or authority shortcut.
//!
//! Each convenience affordance **drives** exactly one governed matrix surface family and pulls its canonical
//! command binding, qualification, owner, required labels, lifecycle label, preview class, feature families,
//! declared consumer surfaces, and applicable downgrade triggers straight from that family's frozen row, so
//! the lane mints no parallel command vocabulary and cannot certify an affordance the matrix does not
//! anchor. For every affordance the lane certifies four things the acceptance criteria and implementation
//! requirements demand:
//!
//! - the affordance **reuses the canonical label, alias, shortcut hint, and lifecycle badge** rather than
//!   inventing a private label or lifecycle language for a stable command
//!   ([`LabelReuseState`], acceptance criterion 1 + implementation requirement 1);
//! - the affordance **preserves the same side-effect class and preview / approval requirement** the
//!   canonical command record carries, so a tooltip, onboarding tip, AI/voice hint, or companion affordance
//!   never softens a destructive or preview-gated action into a one-tap convenience
//!   ([`SideEffectTruthState`], acceptance criterion 2);
//! - the affordance keeps a **focus / context-action equivalent for any hover-only reach and stays within
//!   the canonical command's authority** — a companion / browser hint may not imply a stronger or different
//!   action than the desktop command record allows ([`AuthorityReachState`], implementation requirement 3);
//! - and the **originating command identity is reconstructable** from a copy-safe, diffable export even when
//!   the user triggered the action from a convenience affordance rather than the palette or CLI
//!   ([`OriginExportState`], implementation requirement 4).
//!
//! Three records carry the truth:
//!
//! - the per-affordance **parity row** ([`AffordanceParityRow`]): one row per [`M5ConvenienceAffordance`]
//!   naming the surface family it drives, the canonical command binding it projects from, the required
//!   labels, lifecycle label, and preview class it exposes, the canonical record fields it reuses, the reach
//!   modes it stays reachable in, the consumer surfaces it evaluated, its label-reuse / side-effect-truth /
//!   authority-reach / origin-export posture, whether the same parity survives headless/CLI execution, any
//!   active waiver, and a derived green/yellow/red [`AffordanceParityStatus`].
//! - the parity **packet** ([`AffordanceParityPacket`]): the full set of rows with derived per-row status,
//!   aggregate green/yellow/red counts, the active waivers, the exact conformance causes
//!   ([`AffordanceParityCause`]), and the blocking findings the lane refuses to ship with.
//! - the parity **dashboard** ([`AffordanceParityDashboard`]): a light projection the button / tooltip /
//!   onboarding / AI / voice / companion tooling reads to auto-narrow a convenience affordance's parity claim
//!   when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment an
//! affordance discloses a shortened label, a summarized side-effect note, a reduced hover fallback (a
//! waivered narrowing), or a disclosed partial origin-export capture; it drops to `red` if an affordance
//! invents a private label or lifecycle language, weakens the side-effect or preview / approval truth,
//! renders hover-only or implies an action beyond the canonical authority, cannot reconstruct the
//! originating command identity from durable evidence, loses the same parity in a headless/CLI execution, or
//! fails to reuse all six canonical record fields, stay reachable in all five reach modes, or certify every
//! declared consumer surface. That derivation is the auto-narrowing the acceptance criteria require, and the
//! record-field, reach-mode, and consumer-surface completeness checks are the conformance lints that gate a
//! stable parity claim.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local paths,
//! raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary, counts, refs,
//! and short labels. The surface-family, canonical-command-binding, required-label, lifecycle-label,
//! preview-class, feature-family, consumer-surface, downgrade-trigger, and qualification vocabulary is
//! re-exported by reference from the already frozen [matrix], and every affordance's canonical command
//! binding, qualification, owner, required labels, lifecycle label, preview class, feature families,
//! declared consumer surfaces, and applicable downgrade triggers are pulled straight from that matrix's
//! seeded packet. Only the affordance-parity-specific vocabulary ([`M5ConvenienceAffordance`],
//! [`M5AffordanceParityDimension`], [`M5AffordanceRecordField`], [`M5AffordanceReachMode`],
//! [`AffordanceParityStatus`], [`LabelReuseState`], [`SideEffectTruthState`], [`AuthorityReachState`],
//! [`OriginExportState`], [`AffordanceParityWaiver`], [`AffordanceParityCause`],
//! [`AffordanceParityFinding`]) is new.
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
    seeded_m5_discoverability_affordance_parity_packet,
    seeded_m5_discoverability_affordance_parity_packet_ai_hint_headless_parity_lost_blocked,
    seeded_m5_discoverability_affordance_parity_packet_button_private_label_blocked,
    seeded_m5_discoverability_affordance_parity_packet_companion_authority_overreach_blocked,
    seeded_m5_discoverability_affordance_parity_packet_tooltip_side_effect_weakened_blocked,
    seeded_m5_discoverability_affordance_parity_packet_voice_hint_origin_absent_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_AFFORDANCE_PARITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_AFFORDANCE_PARITY_SHARED_CONTRACT_REF: &str =
    "commands:m5_discoverability_affordance_parity:v1";

/// Stable record kind for [`AffordanceParityPacket`] payloads.
pub const M5_AFFORDANCE_PARITY_PACKET_RECORD_KIND: &str =
    "commands_m5_discoverability_affordance_parity_packet_record";

/// Stable record kind for [`AffordanceParityDashboard`] payloads.
pub const M5_AFFORDANCE_PARITY_DASHBOARD_RECORD_KIND: &str =
    "commands_m5_discoverability_affordance_parity_dashboard_record";

/// Stable record kind for [`AffordanceParitySupportExport`] payloads.
pub const M5_AFFORDANCE_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "commands_m5_discoverability_affordance_parity_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_AFFORDANCE_PARITY_PACKET_ID: &str = "m5-discoverability-affordance-parity:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_AFFORDANCE_PARITY_DASHBOARD_ID: &str =
    "m5-discoverability-affordance-parity-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_AFFORDANCE_PARITY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-discoverability-affordance-parity:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_AFFORDANCE_PARITY_SOURCE_SCHEMA_REF: &str =
    "schemas/commands/m5-discoverability-affordance-parity.schema.json";

/// Published markdown report ref reviewers reopen the parity proof from.
pub const M5_AFFORDANCE_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/commands/m5-discoverability-affordance-parity.md";

/// Published parity-packet artifact ref.
pub const M5_AFFORDANCE_PARITY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-discoverability-affordance-parity-proof/packet.json";

/// Published parity-dashboard artifact ref.
pub const M5_AFFORDANCE_PARITY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-discoverability-affordance-parity-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_AFFORDANCE_PARITY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-discoverability-affordance-parity-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_AFFORDANCE_PARITY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-discoverability-affordance-parity-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_AFFORDANCE_PARITY_PUBLISHED_DOC_REF: &str =
    "docs/commands/m5_discoverability_affordance_parity_contract.md";

/// Repo-relative ref to the frozen discoverability boundary schema.
pub const M5_AFFORDANCE_PARITY_MATRIX_SCHEMA_REF: &str = matrix::M5_DISCOVERABILITY_SCHEMA_REF;

/// Frozen discoverability contract doc this proof mirrors.
pub const M5_AFFORDANCE_PARITY_MATRIX_DOC_REF: &str = matrix::M5_DISCOVERABILITY_DOC_REF;

/// Canonical command-descriptor schema every convenience affordance projects from.
pub const M5_AFFORDANCE_PARITY_COMMAND_DESCRIPTOR_REF: &str =
    matrix::M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF;

/// Every convenience affordance the certification must cover, in canonical order. A certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_AFFORDANCES: [M5ConvenienceAffordance; 7] = M5ConvenienceAffordance::ALL;

/// Every parity dimension each affordance row certifies, in canonical order.
pub const REQUIRED_PARITY_DIMENSIONS: [M5AffordanceParityDimension; 4] =
    M5AffordanceParityDimension::ALL;

/// Every canonical record field each affordance row must reuse, in canonical order.
pub const REQUIRED_RECORD_FIELDS: [M5AffordanceRecordField; 6] = M5AffordanceRecordField::ALL;

/// Every reach mode each affordance row must stay reachable in, in canonical order.
pub const REQUIRED_REACH_MODES: [M5AffordanceReachMode; 5] = M5AffordanceReachMode::ALL;

/// One convenience affordance the parity certification governs.
///
/// These are exactly the last-mile convenience affordances the goal, acceptance criteria, and
/// implementation requirements name: a primary / action button's text, an inline quick-action-card
/// affordance, a tooltip / hovercard label, an onboarding tip reference, an AI hint string, a voice hint
/// string, and a companion / browser handoff affordance. Each one drives exactly one governed matrix surface
/// family and reuses that family's canonical command record rather than inventing a convenience-specific
/// naming or authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConvenienceAffordance {
    /// A primary / action button's text.
    Button,
    /// An inline quick-action-card affordance.
    InlineAffordance,
    /// A tooltip / hovercard label.
    Tooltip,
    /// An onboarding tip reference.
    OnboardingTip,
    /// An AI hint string.
    AiHint,
    /// A voice hint string.
    VoiceHint,
    /// A companion / browser handoff affordance.
    CompanionHandoff,
}

impl M5ConvenienceAffordance {
    /// Every convenience affordance, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Button,
        Self::InlineAffordance,
        Self::Tooltip,
        Self::OnboardingTip,
        Self::AiHint,
        Self::VoiceHint,
        Self::CompanionHandoff,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::InlineAffordance => "inline_affordance",
            Self::Tooltip => "tooltip",
            Self::OnboardingTip => "onboarding_tip",
            Self::AiHint => "ai_hint",
            Self::VoiceHint => "voice_hint",
            Self::CompanionHandoff => "companion_handoff",
        }
    }

    /// The governed matrix surface family this affordance drives — the source of its canonical command
    /// binding and every pulled attribute. Each affordance maps to one distinct family so its parity proof
    /// is anchored to a real frozen command record.
    pub const fn driving_surface_family(self) -> M5CommandSurfaceFamily {
        match self {
            Self::Button => M5CommandSurfaceFamily::MenuItem,
            Self::InlineAffordance => M5CommandSurfaceFamily::CommandBar,
            Self::Tooltip => M5CommandSurfaceFamily::ContextMenu,
            Self::OnboardingTip => M5CommandSurfaceFamily::LeaderSequenceHelp,
            Self::AiHint => M5CommandSurfaceFamily::CommandDocumentationSurface,
            Self::VoiceHint => M5CommandSurfaceFamily::DisabledCommandExplainer,
            Self::CompanionHandoff => M5CommandSurfaceFamily::ImportBridgeRow,
        }
    }
}

/// One of the four parity dimensions each affordance row certifies.
///
/// These are exactly the four ways the acceptance criteria and implementation requirements demand a
/// convenience affordance reuse one command record: it reuses the canonical label / alias / lifecycle badge;
/// it preserves the side-effect and preview / approval truth; it keeps a focus / context-action equivalent
/// and stays within the canonical authority; and its originating command identity reconstructs from durable
/// evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffordanceParityDimension {
    /// The affordance reuses the canonical label / alias / lifecycle badge.
    LabelReuse,
    /// The affordance preserves the side-effect and preview / approval truth.
    SideEffectTruth,
    /// The affordance keeps a focus / context-action equivalent and bounded authority.
    AuthorityReach,
    /// The originating command identity reconstructs from durable evidence.
    OriginExport,
}

impl M5AffordanceParityDimension {
    /// Every parity dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LabelReuse,
        Self::SideEffectTruth,
        Self::AuthorityReach,
        Self::OriginExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabelReuse => "label_reuse",
            Self::SideEffectTruth => "side_effect_truth",
            Self::AuthorityReach => "authority_reach",
            Self::OriginExport => "origin_export",
        }
    }
}

/// One of the six canonical command-record fields a convenience affordance must reuse.
///
/// These are the exact fields the implementation requirements name for the parity fixtures — the canonical
/// label, the alias set, the shortcut hint, the side-effect class, the preview requirement, and the
/// lifecycle badge — compared across visible affordances and help/export surfaces. An affordance that reuses
/// fewer has invented a convenience-specific record and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffordanceRecordField {
    /// The canonical primary label.
    CanonicalLabel,
    /// The canonical alias set.
    AliasSet,
    /// The canonical shortcut hint.
    ShortcutHint,
    /// The canonical side-effect class.
    SideEffectClass,
    /// The canonical preview / approval requirement.
    PreviewRequirement,
    /// The canonical lifecycle / deprecation badge.
    LifecycleBadge,
}

impl M5AffordanceRecordField {
    /// Every canonical record field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CanonicalLabel,
        Self::AliasSet,
        Self::ShortcutHint,
        Self::SideEffectClass,
        Self::PreviewRequirement,
        Self::LifecycleBadge,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalLabel => "canonical_label",
            Self::AliasSet => "alias_set",
            Self::ShortcutHint => "shortcut_hint",
            Self::SideEffectClass => "side_effect_class",
            Self::PreviewRequirement => "preview_requirement",
            Self::LifecycleBadge => "lifecycle_badge",
        }
    }
}

/// One of the five reach modes a convenience affordance must stay reachable in.
///
/// These are the fallback cases the implementation requirements name: a hover-only affordance must still be
/// reachable keyboard-focus, through a screen reader, in a compact layout, and through a touch /
/// context-action fallback — plus the pointer default. An affordance reachable in fewer hides itself behind
/// hover and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffordanceReachMode {
    /// Reachable through pointer interaction (the default).
    PointerDefault,
    /// Reachable through keyboard focus, without a pointer hover.
    KeyboardFocus,
    /// Reachable / announced through a screen reader.
    ScreenReader,
    /// Reachable in a compact / constrained layout.
    CompactLayout,
    /// Reachable through a touch / context-action fallback.
    TouchContextAction,
}

impl M5AffordanceReachMode {
    /// Every reach mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PointerDefault,
        Self::KeyboardFocus,
        Self::ScreenReader,
        Self::CompactLayout,
        Self::TouchContextAction,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PointerDefault => "pointer_default",
            Self::KeyboardFocus => "keyboard_focus",
            Self::ScreenReader => "screen_reader",
            Self::CompactLayout => "compact_layout",
            Self::TouchContextAction => "touch_context_action",
        }
    }
}

/// The derived parity light a convenience affordance carries.
///
/// `green` means the affordance reuses the canonical label, alias, shortcut hint, and lifecycle badge,
/// preserves the side-effect and preview / approval truth, keeps a focus / context-action equivalent within
/// the canonical authority, and reconstructs its originating command identity from durable evidence — across
/// every declared consumer surface and every reach mode, with the same parity surviving headless/CLI
/// execution. `yellow` is a disclosed narrowing. `red` is blocked and may not keep a parity claim until
/// repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffordanceParityStatus {
    /// Full standing: all four parity dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl AffordanceParityStatus {
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

/// How the affordance reuses the canonical label, alias, shortcut hint, and lifecycle badge.
///
/// `canonical_label_alias_and_lifecycle_reused` means the affordance shows the canonical primary label,
/// alias set, shortcut hint, and lifecycle / deprecation badge rather than inventing convenience-specific
/// text. `disclosed_shortened_affordance_label` means a space-constrained affordance renders a disclosed
/// shortened label while still linking the canonical id, alias, and lifecycle badge (a yellow narrowing).
/// `private_label_or_lifecycle_invented` means the affordance invented a private label or lifecycle language
/// for a stable command — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelReuseState {
    /// The canonical label, alias, shortcut hint, and lifecycle badge are reused.
    CanonicalLabelAliasAndLifecycleReused,
    /// A constrained affordance takes a disclosed shortened label.
    DisclosedShortenedAffordanceLabel,
    /// The affordance invented a private label or lifecycle language — a blocker.
    PrivateLabelOrLifecycleInvented,
}

impl LabelReuseState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalLabelAliasAndLifecycleReused => {
                "canonical_label_alias_and_lifecycle_reused"
            }
            Self::DisclosedShortenedAffordanceLabel => "disclosed_shortened_affordance_label",
            Self::PrivateLabelOrLifecycleInvented => "private_label_or_lifecycle_invented",
        }
    }

    /// `true` when label reuse is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CanonicalLabelAliasAndLifecycleReused)
    }

    /// `true` when the affordance took a disclosed shortened-label narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedShortenedAffordanceLabel)
    }
}

/// How the affordance preserves the side-effect and preview / approval truth.
///
/// `side_effect_and_preview_truth_preserved` means the affordance carries the same side-effect class and
/// preview / approval requirement the canonical command record pins — a destructive or preview-gated action
/// never reads as a one-tap convenience. `disclosed_summarized_side_effect_note` means a constrained
/// affordance folds the full side-effect prose into a disclosed summary while still surfacing the preview /
/// approval requirement (a yellow narrowing). `side_effect_or_preview_truth_weakened` means the affordance
/// dropped or softened the side-effect class or preview / approval requirement — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectTruthState {
    /// The side-effect class and preview / approval requirement are preserved.
    SideEffectAndPreviewTruthPreserved,
    /// A constrained affordance takes a disclosed summarized side-effect note.
    DisclosedSummarizedSideEffectNote,
    /// The affordance weakened the side-effect or preview / approval truth — a blocker.
    SideEffectOrPreviewTruthWeakened,
}

impl SideEffectTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SideEffectAndPreviewTruthPreserved => "side_effect_and_preview_truth_preserved",
            Self::DisclosedSummarizedSideEffectNote => "disclosed_summarized_side_effect_note",
            Self::SideEffectOrPreviewTruthWeakened => "side_effect_or_preview_truth_weakened",
        }
    }

    /// `true` when side-effect truth is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::SideEffectAndPreviewTruthPreserved)
    }

    /// `true` when the affordance took a disclosed summarized-side-effect-note narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedSummarizedSideEffectNote)
    }
}

/// How the affordance keeps a focus / context-action equivalent and stays within the canonical authority.
///
/// `focus_equivalent_and_bounded_authority` means any hover-only reach has a focus / context-action
/// equivalent and no companion / browser hint implies a stronger or different action than the desktop
/// command record allows. `disclosed_reduced_hover_fallback` means a hover affordance falls back to a
/// disclosed reduced form on a touch / narrow surface while still keeping a keyboard-focus and
/// context-action equivalent (a yellow narrowing that **requires an active waiver**).
/// `hover_only_or_authority_overreach` means the affordance is hover-only with no equivalent, or a companion
/// hint implies an action beyond the canonical authority — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReachState {
    /// A focus / context-action equivalent exists and authority stays bounded.
    FocusEquivalentAndBoundedAuthority,
    /// A hover affordance takes a disclosed, waivered reduced fallback.
    DisclosedReducedHoverFallback,
    /// The affordance is hover-only or overreaches the canonical authority — a blocker.
    HoverOnlyOrAuthorityOverreach,
}

impl AuthorityReachState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FocusEquivalentAndBoundedAuthority => "focus_equivalent_and_bounded_authority",
            Self::DisclosedReducedHoverFallback => "disclosed_reduced_hover_fallback",
            Self::HoverOnlyOrAuthorityOverreach => "hover_only_or_authority_overreach",
        }
    }

    /// `true` when authority reach is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::FocusEquivalentAndBoundedAuthority)
    }

    /// `true` when the affordance took a disclosed reduced-hover-fallback narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedHoverFallback)
    }
}

/// How the parity packet reconstructs the originating command identity.
///
/// `origin_command_identity_reconstructable` means a support bundle, doc, or migration packet can
/// reconstruct the originating command id, label, and lifecycle from a durable, copy-safe, diffable export
/// even when the action was triggered from a convenience affordance rather than the palette or CLI.
/// `disclosed_partial_capture` means one legacy export captures the affordance and command id but not the
/// full canonical record, while still disclosing the gap (a yellow narrowing).
/// `originating_command_absent_from_capture` means the originating command id is absent from durable
/// evidence — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginExportState {
    /// The originating command identity is reconstructable from durable evidence.
    OriginCommandIdentityReconstructable,
    /// One legacy export takes a disclosed partial capture.
    DisclosedPartialCapture,
    /// The originating command id is absent from durable evidence — a blocker.
    OriginatingCommandAbsentFromCapture,
}

impl OriginExportState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginCommandIdentityReconstructable => "origin_command_identity_reconstructable",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::OriginatingCommandAbsentFromCapture => "originating_command_absent_from_capture",
        }
    }

    /// `true` when origin-export parity is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::OriginCommandIdentityReconstructable)
    }

    /// `true` when the affordance took a disclosed partial-capture narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather than
/// blocked — never lets a private label, weakened side-effect truth, hover-only / authority overreach, or an
/// absent origin capture hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffordanceParityWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The convenience affordance the waiver applies to.
    pub affordance: M5ConvenienceAffordance,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl AffordanceParityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a convenience affordance's parity.
///
/// The trigger token mirrors the frozen [`M5DiscoverabilityDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffordanceParityCause {
    /// The convenience affordance the cause applies to.
    pub affordance: M5ConvenienceAffordance,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5DiscoverabilityDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is a
    /// blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl AffordanceParityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One convenience affordance, certified across its label-reuse, side-effect-truth, authority-reach, and
/// origin-export dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffordanceParityRow {
    /// The convenience affordance being certified.
    pub affordance: M5ConvenienceAffordance,
    /// Short reviewer-facing affordance label.
    pub affordance_label: String,
    /// The governed matrix surface family this affordance drives. Every pulled attribute comes from this
    /// family's frozen row.
    pub driving_surface_family: M5CommandSurfaceFamily,
    /// Qualification class the matrix earned for the driving surface. Pulled from the matrix.
    pub qualification: M5SurfaceQualificationClass,
    /// Owner role accountable for keeping this affordance's parity governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short conformance scope summary.
    pub scope_summary: String,
    /// The canonical command-record binding this affordance projects from. Pulled from the matrix.
    pub canonical_command_binding: M5CanonicalCommandBinding,
    /// The pinned lifecycle / deprecation label. Pulled from the canonical command binding.
    pub lifecycle_label: M5LifecycleLabel,
    /// The pinned preview / approval class. Pulled from the canonical command binding.
    pub preview_class: M5PreviewClass,
    /// The pinned disabled-reason mode. Pulled from the canonical command binding.
    pub disabled_reason_mode: M5DisabledReasonMode,
    /// Mandatory labels the driving surface must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5RequiredLabel>,
    /// M5 feature families whose commands this affordance projects. Pulled from the matrix.
    pub feature_families: Vec<M5FeatureFamily>,
    /// The canonical record fields this affordance reuses (must be all six).
    pub certified_record_fields: Vec<M5AffordanceRecordField>,
    /// The reach modes this affordance stays reachable in (must be all five).
    pub certified_reach_modes: Vec<M5AffordanceReachMode>,
    /// Consumer surfaces the matrix declares the driving surface must project to.
    pub required_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Label-reuse posture.
    pub label_reuse: LabelReuseState,
    /// Side-effect-truth posture.
    pub side_effect_truth: SideEffectTruthState,
    /// Authority-reach posture.
    pub authority_reach: AuthorityReachState,
    /// Origin-export posture.
    pub origin_export: OriginExportState,
    /// `true` when the same parity survives a headless / CLI execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the driving surface. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced hover fallback is in force.
    pub active_waiver: Option<AffordanceParityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: AffordanceParityStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<AffordanceParityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl AffordanceParityRow {
    /// `true` when the row certified every consumer surface the matrix declares for the driving surface — no
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

    /// `true` when the row reuses every one of the six canonical record fields — the structural proof that
    /// the affordance projects one command record rather than a convenience-specific one.
    pub fn record_fields_complete(&self) -> bool {
        complete_tokens(
            &self.certified_record_fields,
            |field| field.as_str(),
            &REQUIRED_RECORD_FIELDS,
            |field| field.as_str(),
        )
    }

    /// `true` when the row stays reachable in every one of the five reach modes — the structural proof that
    /// the affordance is not hidden behind hover.
    pub fn reach_modes_complete(&self) -> bool {
        complete_tokens(
            &self.certified_reach_modes,
            |mode| mode.as_str(),
            &REQUIRED_REACH_MODES,
            |mode| mode.as_str(),
        )
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.record_fields_complete() {
            return true;
        }
        if !self.reach_modes_complete() {
            return true;
        }
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.label_reuse,
            LabelReuseState::PrivateLabelOrLifecycleInvented
        ) {
            return true;
        }
        if matches!(
            self.side_effect_truth,
            SideEffectTruthState::SideEffectOrPreviewTruthWeakened
        ) {
            return true;
        }
        if matches!(
            self.authority_reach,
            AuthorityReachState::HoverOnlyOrAuthorityOverreach
        ) {
            return true;
        }
        if matches!(
            self.origin_export,
            OriginExportState::OriginatingCommandAbsentFromCapture
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.label_reuse.is_disclosed_narrowing()
            || self.side_effect_truth.is_disclosed_narrowing()
            || self.authority_reach.is_disclosed_narrowing()
            || self.origin_export.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the parity posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces `yellow`,
    /// otherwise `green`.
    pub fn recompute_status(&self) -> AffordanceParityStatus {
        if self.has_hard_blocker() {
            AffordanceParityStatus::Red
        } else if self.has_narrowing() {
            AffordanceParityStatus::Yellow
        } else {
            AffordanceParityStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (label reuse, side-effect
    /// truth, authority reach, origin export, then structural completeness and headless parity).
    pub fn recompute_causes(&self) -> Vec<AffordanceParityCause> {
        let mut causes = Vec::new();
        match self.label_reuse {
            LabelReuseState::CanonicalLabelAliasAndLifecycleReused => {}
            LabelReuseState::DisclosedShortenedAffordanceLabel => {
                causes.push(AffordanceParityCause {
                    affordance: self.affordance,
                    trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                    disclosed: true,
                    detail: "On a space-constrained affordance the label renders a disclosed shortened form \
                             while the affordance still links the canonical command id, alias set, shortcut \
                             hint, and lifecycle badge — so the label is narrowed and disclosed rather than \
                             an invented convenience-specific label."
                        .to_owned(),
                });
            }
            LabelReuseState::PrivateLabelOrLifecycleInvented => {
                causes.push(AffordanceParityCause {
                    affordance: self.affordance,
                    trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                    disclosed: false,
                    detail: "The affordance invented a private label or lifecycle language for a stable \
                             command, so the same action reads under a different name or lifecycle depending \
                             on whether it is reached from the affordance or the canonical command record."
                        .to_owned(),
                });
            }
        }
        match self.side_effect_truth {
            SideEffectTruthState::SideEffectAndPreviewTruthPreserved => {}
            SideEffectTruthState::DisclosedSummarizedSideEffectNote => {
                causes.push(AffordanceParityCause {
                    affordance: self.affordance,
                    trigger: M5DiscoverabilityDowngradeTrigger::PreviewApprovalMasked,
                    disclosed: true,
                    detail: "On a constrained affordance the full side-effect prose is folded into a \
                             disclosed summary while the preview / approval requirement and side-effect \
                             class stay visible — so the side-effect truth is narrowed and disclosed rather \
                             than softened into a one-tap convenience."
                        .to_owned(),
                });
            }
            SideEffectTruthState::SideEffectOrPreviewTruthWeakened => {
                causes.push(AffordanceParityCause {
                    affordance: self.affordance,
                    trigger: M5DiscoverabilityDowngradeTrigger::PreviewApprovalMasked,
                    disclosed: false,
                    detail: "The affordance dropped or softened the side-effect class or the preview / \
                             approval requirement the canonical command record pins, so a destructive or \
                             preview-gated action reads as a one-tap convenience."
                        .to_owned(),
                });
            }
        }
        match self.authority_reach {
            AuthorityReachState::FocusEquivalentAndBoundedAuthority => {}
            AuthorityReachState::DisclosedReducedHoverFallback => {
                causes.push(AffordanceParityCause {
                    affordance: self.affordance,
                    trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                    disclosed: true,
                    detail: "On a touch / narrow surface a hover affordance falls back to a disclosed, \
                             waivered reduced form while still keeping a keyboard-focus and context-action \
                             equivalent — so the reach is narrowed and disclosed rather than hover-only."
                        .to_owned(),
                });
            }
            AuthorityReachState::HoverOnlyOrAuthorityOverreach => {
                causes.push(AffordanceParityCause {
                    affordance: self.affordance,
                    trigger: M5DiscoverabilityDowngradeTrigger::AuthorityWidened,
                    disclosed: false,
                    detail: "The affordance is hover-only with no focus / context-action equivalent, or a \
                             companion / browser hint implies a stronger or different action than the \
                             desktop command record allows, so the affordance widens authority beyond the \
                             canonical command."
                        .to_owned(),
                });
            }
        }
        match self.origin_export {
            OriginExportState::OriginCommandIdentityReconstructable => {}
            OriginExportState::DisclosedPartialCapture => {
                causes.push(AffordanceParityCause {
                    affordance: self.affordance,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One legacy export takes a disclosed partial capture — the export captures the \
                             affordance and command id but not the full canonical record, while still \
                             disclosing the gap — so the origin-export parity is narrowed and disclosed \
                             rather than absent."
                        .to_owned(),
                });
            }
            OriginExportState::OriginatingCommandAbsentFromCapture => {
                causes.push(AffordanceParityCause {
                    affordance: self.affordance,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail: "The originating command id is absent from the durable, diffable export, so a \
                             support bundle, doc, or migration packet cannot reconstruct which command the \
                             convenience affordance triggered without a screenshot."
                        .to_owned(),
                });
            }
        }
        if !self.record_fields_complete() {
            causes.push(AffordanceParityCause {
                affordance: self.affordance,
                trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                disclosed: false,
                detail: "The affordance does not reuse all six canonical record fields — canonical label, \
                         alias set, shortcut hint, side-effect class, preview requirement, and lifecycle \
                         badge — so it projects a partial, convenience-specific record rather than the one \
                         command record."
                    .to_owned(),
            });
        }
        if !self.reach_modes_complete() {
            causes.push(AffordanceParityCause {
                affordance: self.affordance,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "The affordance is not reachable in all five reach modes — pointer, keyboard focus, \
                         screen reader, compact layout, and touch / context-action fallback — so the \
                         affordance could be hidden behind hover in some modes."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(AffordanceParityCause {
                affordance: self.affordance,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "A headless / CLI execution of this affordance lost the shared command record, so \
                         the same action projects a different label, side-effect, or authority depending on \
                         how it is reached."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced hover fallback may only stay yellow (rather than red) when a waiver discloses it —
    /// reducing a hover affordance's reach is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.authority_reach,
            AuthorityReachState::DisclosedReducedHoverFallback
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<AffordanceParityFinding> {
        let mut findings = Vec::new();
        let affordance = self.affordance.as_str().to_owned();

        if !self.record_fields_complete() {
            findings.push(AffordanceParityFinding::RecordFieldsIncomplete {
                affordance: affordance.clone(),
            });
        }
        if !self.reach_modes_complete() {
            findings.push(AffordanceParityFinding::ReachModesIncomplete {
                affordance: affordance.clone(),
            });
        }
        if !self.consumer_surfaces_complete() {
            findings.push(AffordanceParityFinding::ConsumerSurfacesIncomplete {
                affordance: affordance.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(AffordanceParityFinding::HeadlessParityLost {
                affordance: affordance.clone(),
            });
        }
        if matches!(
            self.label_reuse,
            LabelReuseState::PrivateLabelOrLifecycleInvented
        ) {
            findings.push(AffordanceParityFinding::LabelReuseBroken {
                affordance: affordance.clone(),
            });
        }
        if matches!(
            self.side_effect_truth,
            SideEffectTruthState::SideEffectOrPreviewTruthWeakened
        ) {
            findings.push(AffordanceParityFinding::SideEffectTruthBroken {
                affordance: affordance.clone(),
            });
        }
        if matches!(
            self.authority_reach,
            AuthorityReachState::HoverOnlyOrAuthorityOverreach
        ) {
            findings.push(AffordanceParityFinding::AuthorityReachBroken {
                affordance: affordance.clone(),
            });
        }
        if matches!(
            self.origin_export,
            OriginExportState::OriginatingCommandAbsentFromCapture
        ) {
            findings.push(AffordanceParityFinding::OriginExportBroken {
                affordance: affordance.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, AffordanceParityStatus::Green) && !self.has_reason() {
            findings.push(AffordanceParityFinding::NarrowedRowWithoutReason {
                affordance: affordance.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(AffordanceParityFinding::NarrowedRowWithoutWaiver {
                affordance: affordance.clone(),
            });
        }
        // An attached waiver must still be active and must point at this affordance.
        if let Some(waiver) = &self.active_waiver {
            if waiver.affordance != self.affordance {
                findings.push(AffordanceParityFinding::WaiverAffordanceMismatch {
                    affordance: affordance.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(AffordanceParityFinding::WaiverExpired {
                    affordance: affordance.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(AffordanceParityFinding::RowStatusStale {
                affordance: affordance.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(AffordanceParityFinding::RowCausesStale { affordance });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} label={} side_effect={} authority={} origin={} headless={} lifecycle={} preview={} fields={} modes={} surfaces={} waiver={}",
            self.affordance.as_str(),
            self.derived_status.as_str(),
            self.label_reuse.as_str(),
            self.side_effect_truth.as_str(),
            self.authority_reach.as_str(),
            self.origin_export.as_str(),
            self.headless_parity_preserved,
            self.lifecycle_label.as_str(),
            self.preview_class.as_str(),
            self.certified_record_fields.len(),
            self.certified_reach_modes.len(),
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

/// A blocking finding the affordance-parity certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum AffordanceParityFinding {
    /// A convenience affordance has no parity row.
    AffordanceMissing {
        /// The missing affordance token.
        affordance: String,
    },
    /// A row did not reuse every canonical record field.
    RecordFieldsIncomplete {
        /// The affordance token.
        affordance: String,
    },
    /// A row is not reachable in every reach mode.
    ReachModesIncomplete {
        /// The affordance token.
        affordance: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The affordance token.
        affordance: String,
    },
    /// A headless / CLI execution lost the shared command record.
    HeadlessParityLost {
        /// The affordance token.
        affordance: String,
    },
    /// An affordance invented a private label or lifecycle language.
    LabelReuseBroken {
        /// The affordance token.
        affordance: String,
    },
    /// An affordance weakened the side-effect or preview / approval truth.
    SideEffectTruthBroken {
        /// The affordance token.
        affordance: String,
    },
    /// An affordance is hover-only or overreaches the canonical authority.
    AuthorityReachBroken {
        /// The affordance token.
        affordance: String,
    },
    /// The originating command id is absent from the durable export.
    OriginExportBroken {
        /// The affordance token.
        affordance: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The affordance token.
        affordance: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The affordance token.
        affordance: String,
    },
    /// An attached waiver does not point at the row's affordance.
    WaiverAffordanceMismatch {
        /// The affordance token.
        affordance: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The affordance token.
        affordance: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The affordance token.
        affordance: String,
    },
    /// The declared conformance causes do not match the recomputed causes.
    RowCausesStale {
        /// The affordance token.
        affordance: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered affordances do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl AffordanceParityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::AffordanceMissing { .. } => "affordance_missing",
            Self::RecordFieldsIncomplete { .. } => "record_fields_incomplete",
            Self::ReachModesIncomplete { .. } => "reach_modes_incomplete",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::LabelReuseBroken { .. } => "label_reuse_broken",
            Self::SideEffectTruthBroken { .. } => "side_effect_truth_broken",
            Self::AuthorityReachBroken { .. } => "authority_reach_broken",
            Self::OriginExportBroken { .. } => "origin_export_broken",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverAffordanceMismatch { .. } => "waiver_affordance_mismatch",
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
            Self::AffordanceMissing { affordance }
            | Self::RecordFieldsIncomplete { affordance }
            | Self::ReachModesIncomplete { affordance }
            | Self::ConsumerSurfacesIncomplete { affordance }
            | Self::HeadlessParityLost { affordance }
            | Self::LabelReuseBroken { affordance }
            | Self::SideEffectTruthBroken { affordance }
            | Self::AuthorityReachBroken { affordance }
            | Self::OriginExportBroken { affordance }
            | Self::NarrowedRowWithoutReason { affordance }
            | Self::NarrowedRowWithoutWaiver { affordance }
            | Self::WaiverAffordanceMismatch { affordance, .. }
            | Self::WaiverExpired { affordance, .. }
            | Self::RowStatusStale { affordance }
            | Self::RowCausesStale { affordance } => affordance,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The parity packet shared by the button / tooltip / onboarding / AI / voice / companion tooling and
/// Support Center / CLI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffordanceParityPacket {
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
    /// Canonical command-descriptor schema every convenience affordance projects from.
    pub command_descriptor_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four parity dimensions every affordance row certifies.
    pub required_parity_dimensions: Vec<String>,
    /// The six canonical record fields every affordance row must reuse.
    pub required_record_fields: Vec<String>,
    /// The five reach modes every affordance row must stay reachable in.
    pub required_reach_modes: Vec<String>,
    /// The seven convenience affordances the certification must cover.
    pub required_affordances: Vec<String>,
    /// Per-affordance parity rows, in canonical order.
    pub rows: Vec<AffordanceParityRow>,
    /// Affordances certified, in canonical (sorted) order.
    pub covered_affordances: Vec<String>,
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
    pub active_waivers: Vec<AffordanceParityWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<AffordanceParityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<AffordanceParityFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Command / affordance automation refs that consume this packet to auto-narrow a surface.
    pub command_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help / onboarding refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published parity-packet ref.
    pub published_packet_ref: String,
    /// Published parity-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl AffordanceParityPacket {
    /// Returns the parity row for `affordance`, if present.
    pub fn row(&self, affordance: M5ConvenienceAffordance) -> Option<&AffordanceParityRow> {
        self.rows.iter().find(|row| row.affordance == affordance)
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
                waiver.affordance.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.conformance_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.affordance.as_str(),
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

    /// Projects the light parity dashboard the command automation consumes.
    pub fn dashboard(&self) -> AffordanceParityDashboard {
        AffordanceParityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 affordance-parity packet serializes")
    }

    /// Deterministic, machine-readable parity CSV: one row per convenience affordance naming its status, the
    /// four parity postures, headless parity, the lifecycle label and preview class, the field / mode
    /// counts, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "affordance,driving_surface_family,status,label_reuse,side_effect_truth,authority_reach,origin_export,headless_parity,lifecycle,preview_class,record_fields,reach_modes,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.affordance.as_str(),
                row.driving_surface_family.as_str(),
                row.derived_status.as_str(),
                row.label_reuse.as_str(),
                row.side_effect_truth.as_str(),
                row.authority_reach.as_str(),
                row.origin_export.as_str(),
                row.headless_parity_preserved,
                row.lifecycle_label.as_str(),
                row.preview_class.as_str(),
                row.certified_record_fields.len(),
                row.certified_reach_modes.len(),
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
            "# M5 discoverability affordance parity: buttons, inline affordances, tooltips, onboarding tips, AI/voice hints, and companion handoffs reuse one command record across every claimed M5 action\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_discoverability_affordance_parity`](../../crates/aureline-shell/src/m5_discoverability_affordance_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- markdown > \\\n  artifacts/commands/m5-discoverability-affordance-parity.md\n",
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
            "- Required parity dimensions: {}\n",
            self.required_parity_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Canonical record fields reused: {}\n",
            self.required_record_fields
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Reach modes: {}\n",
            self.required_reach_modes
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Convenience affordances certified: {}\n",
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

        out.push_str("## Parity rows\n\n");
        out.push_str(
            "| Affordance | Drives | Status | Label reuse | Side-effect truth | Authority reach | Origin export | Lifecycle | Preview | Headless | Waiver |\n\
             | ---------- | ------ | ------ | ----------- | ----------------- | --------------- | ------------- | --------- | ------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.affordance_label,
                row.driving_surface_family.as_str(),
                row.derived_status.as_str(),
                row.label_reuse.as_str(),
                row.side_effect_truth.as_str(),
                row.authority_reach.as_str(),
                row.origin_export.as_str(),
                row.lifecycle_label.as_str(),
                row.preview_class.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&AffordanceParityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, AffordanceParityStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed M5 convenience affordance reuses the canonical label, alias, shortcut hint, and lifecycle badge, preserves the side-effect and preview / approval truth, keeps a focus / context-action equivalent within the canonical authority, and reconstructs its originating command identity from durable evidence across every declared consumer surface.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.affordance.as_str(),
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
                    cause.affordance.as_str(),
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
                    waiver.affordance.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_discoverability_affordance_parity_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light parity dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffordanceParityDashboardRow {
    /// The convenience affordance.
    pub affordance: M5ConvenienceAffordance,
    /// Short affordance label.
    pub affordance_label: String,
    /// The surface family the affordance drives.
    pub driving_surface_family: M5CommandSurfaceFamily,
    /// Qualification class earned by the driving surface.
    pub qualification: M5SurfaceQualificationClass,
    /// Derived green/yellow/red status.
    pub status: AffordanceParityStatus,
    /// The pinned lifecycle / deprecation label.
    pub lifecycle_label: M5LifecycleLabel,
    /// The pinned preview / approval class.
    pub preview_class: M5PreviewClass,
    /// Number of canonical record fields reused.
    pub certified_record_field_count: usize,
    /// Number of reach modes covered.
    pub certified_reach_mode_count: usize,
    /// Number of declared consumer surfaces certified for this affordance.
    pub evaluated_surface_count: usize,
    /// Label-reuse posture.
    pub label_reuse: LabelReuseState,
    /// Side-effect-truth posture.
    pub side_effect_truth: SideEffectTruthState,
    /// Authority-reach posture.
    pub authority_reach: AuthorityReachState,
    /// Origin-export posture.
    pub origin_export: OriginExportState,
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

/// The light parity dashboard the button / tooltip / onboarding / AI / voice / companion tooling and Support
/// Center / CLI reads to auto-narrow a convenience affordance's parity claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffordanceParityDashboard {
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
    pub rows: Vec<AffordanceParityDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Command / affordance automation refs that consume the dashboard.
    pub command_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl AffordanceParityDashboard {
    /// Projects the dashboard from a parity packet.
    pub fn from_packet(packet: &AffordanceParityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| AffordanceParityDashboardRow {
                affordance: row.affordance,
                affordance_label: row.affordance_label.clone(),
                driving_surface_family: row.driving_surface_family,
                qualification: row.qualification,
                status: row.derived_status,
                lifecycle_label: row.lifecycle_label,
                preview_class: row.preview_class,
                certified_record_field_count: row.certified_record_fields.len(),
                certified_reach_mode_count: row.certified_reach_modes.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                label_reuse: row.label_reuse,
                side_effect_truth: row.side_effect_truth,
                authority_reach: row.authority_reach,
                origin_export: row.origin_export,
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
            record_kind: M5_AFFORDANCE_PARITY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_AFFORDANCE_PARITY_SCHEMA_VERSION,
            dashboard_id: M5_AFFORDANCE_PARITY_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 affordance-parity dashboard serializes")
    }
}

/// Support-export wrapper for the parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffordanceParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: AffordanceParityPacket,
    /// Dashboard quoted in full.
    pub dashboard: AffordanceParityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl AffordanceParitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each convenience affordance, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the button / tooltip / companion
    /// tooling — can name the same affordance and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: AffordanceParityPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.affordance.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_AFFORDANCE_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_AFFORDANCE_PARITY_SCHEMA_VERSION,
            shared_contract_ref: M5_AFFORDANCE_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_affordance_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffordanceParityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen discoverability matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-affordance parity rows.
    pub rows: Vec<AffordanceParityRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The parity packet carries only closed vocabulary, refs, and short labels, so raw URLs, credentials, or
/// tokens must never appear.
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

/// Builds an [`AffordanceParityPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-affordance parity rows.
///
/// Each row's derived status and conformance causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the auto-narrowing
/// cannot be asserted.
pub fn build_m5_affordance_parity_packet(input: AffordanceParityInput) -> AffordanceParityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<AffordanceParityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<AffordanceParityFinding> = Vec::new();

    // Every convenience affordance must carry a parity row.
    let present: BTreeSet<M5ConvenienceAffordance> =
        rows.iter().map(|row| row.affordance).collect();
    for affordance in REQUIRED_AFFORDANCES {
        if !present.contains(&affordance) {
            blocking_findings.push(AffordanceParityFinding::AffordanceMissing {
                affordance: affordance.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_affordances: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|affordance| affordance.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AffordanceParityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AffordanceParityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AffordanceParityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(AffordanceParityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<AffordanceParityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<AffordanceParityCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_parity_dimensions: Vec<String> = REQUIRED_PARITY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_record_fields: Vec<String> = REQUIRED_RECORD_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    let required_reach_modes: Vec<String> = REQUIRED_REACH_MODES
        .iter()
        .map(|mode| mode.as_str().to_owned())
        .collect();
    let required_affordances: Vec<String> = REQUIRED_AFFORDANCES
        .iter()
        .map(|affordance| affordance.as_str().to_owned())
        .collect();

    let mut packet = AffordanceParityPacket {
        record_kind: M5_AFFORDANCE_PARITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_AFFORDANCE_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_AFFORDANCE_PARITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_AFFORDANCE_PARITY_PACKET_ID.to_owned(),
        source_schema_ref: M5_AFFORDANCE_PARITY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Convenience-affordance parity for every claimed M5 command action: each of the seven \
                   governed convenience affordances — button, inline affordance, tooltip, onboarding tip, \
                   AI hint, voice hint, and companion handoff — certified so a pointer, keyboard, \
                   screen-reader, touch, AI/voice, or companion reach reuses one command record rather than \
                   inventing a convenience-specific label, lifecycle language, side-effect story, or \
                   authority shortcut: each affordance reuses the canonical label, alias, shortcut hint, and \
                   lifecycle badge; preserves the side-effect class and preview / approval requirement; \
                   keeps a focus / context-action equivalent within the canonical authority; and \
                   reconstructs its originating command identity from a copy-safe, diffable export — across \
                   every declared consumer surface and every reach mode, with the same parity preserved in \
                   headless/CLI execution, each affordance's green/yellow/red claim auto-narrowed from its \
                   four parity postures, and any affordance that invents a private label, weakens the \
                   side-effect or preview truth, renders hover-only or overreaches authority, or cannot \
                   reconstruct its originating command blocked from a stable claim."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_AFFORDANCE_PARITY_MATRIX_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_AFFORDANCE_PARITY_MATRIX_DOC_REF.to_owned(),
        command_descriptor_ref: M5_AFFORDANCE_PARITY_COMMAND_DESCRIPTOR_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_parity_dimensions,
        required_record_fields,
        required_reach_modes,
        required_affordances,
        rows,
        covered_affordances,
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
            "command_status.affordance_parity_registry".to_owned(),
            "affordance_automation.auto_narrow.affordance_parity_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.discoverability_affordance_parity".to_owned(),
            M5_AFFORDANCE_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_AFFORDANCE_PARITY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-discoverability-affordance-parity".to_owned()],
        published_report_ref: M5_AFFORDANCE_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_AFFORDANCE_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_AFFORDANCE_PARITY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_AFFORDANCE_PARITY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("affordance-parity packet serializes"),
    ) {
        blocking_findings.push(AffordanceParityFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_affordance_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum AffordanceParityValidationError {
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
    /// The declared required parity dimensions do not match the lane constants.
    RequiredParityDimensionsStale,
    /// The declared required record fields do not match the lane constants.
    RequiredRecordFieldsStale,
    /// The declared required reach modes do not match the lane constants.
    RequiredReachModesStale,
    /// The declared required affordances do not match the lane constants.
    RequiredAffordancesStale,
    /// The rows do not cover all seven convenience affordances.
    CoverageIncomplete,
    /// The declared covered affordances do not match the rows.
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

/// Validates a packet against the affordance-parity invariants.
///
/// The checks encode the track invariant and acceptance criteria: every convenience affordance carries a
/// current parity row; each row's status is the derived value, never asserted; a green row cannot keep a
/// claim while it invents a private label or lifecycle language, weakens the side-effect or preview /
/// approval truth, renders hover-only or overreaches the canonical authority, cannot reconstruct its
/// originating command identity from durable evidence, loses headless/CLI parity, fails to reuse all six
/// canonical record fields, fails to stay reachable in all five reach modes, or fails to certify every
/// declared consumer surface; and a disclosed narrowing is backed by a reason and, where required, an active
/// waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_affordance_parity_packet(
    packet: &AffordanceParityPacket,
) -> Result<(), Vec<AffordanceParityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(AffordanceParityValidationError::NoRows);
    }
    if packet.record_kind != M5_AFFORDANCE_PARITY_PACKET_RECORD_KIND {
        errors.push(AffordanceParityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_AFFORDANCE_PARITY_SCHEMA_VERSION {
        errors.push(AffordanceParityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(AffordanceParityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(AffordanceParityValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_PARITY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_parity_dimensions != expected_dimensions {
        errors.push(AffordanceParityValidationError::RequiredParityDimensionsStale);
    }
    let expected_record_fields: Vec<String> = REQUIRED_RECORD_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    if packet.required_record_fields != expected_record_fields {
        errors.push(AffordanceParityValidationError::RequiredRecordFieldsStale);
    }
    let expected_reach_modes: Vec<String> = REQUIRED_REACH_MODES
        .iter()
        .map(|mode| mode.as_str().to_owned())
        .collect();
    if packet.required_reach_modes != expected_reach_modes {
        errors.push(AffordanceParityValidationError::RequiredReachModesStale);
    }
    let expected_affordances: Vec<String> = REQUIRED_AFFORDANCES
        .iter()
        .map(|affordance| affordance.as_str().to_owned())
        .collect();
    if packet.required_affordances != expected_affordances {
        errors.push(AffordanceParityValidationError::RequiredAffordancesStale);
    }

    let present: BTreeSet<M5ConvenienceAffordance> =
        packet.rows.iter().map(|row| row.affordance).collect();
    let coverage_complete = REQUIRED_AFFORDANCES
        .iter()
        .all(|affordance| present.contains(affordance));
    if !coverage_complete || packet.rows.len() != REQUIRED_AFFORDANCES.len() {
        errors.push(AffordanceParityValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|affordance| affordance.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_affordances {
        errors.push(AffordanceParityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AffordanceParityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AffordanceParityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AffordanceParityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(AffordanceParityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<AffordanceParityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(AffordanceParityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<AffordanceParityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(AffordanceParityValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<AffordanceParityFinding> = Vec::new();
    for affordance in REQUIRED_AFFORDANCES {
        if !present.contains(&affordance) {
            recomputed.push(AffordanceParityFinding::AffordanceMissing {
                affordance: affordance.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(AffordanceParityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("affordance-parity packet serializes"),
    ) {
        recomputed.push(AffordanceParityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(AffordanceParityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(AffordanceParityValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(AffordanceParityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(AffordanceParityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(AffordanceParityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(AffordanceParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
