//! Leader / sequence overlays, partial-sequence help, disabled-command explainers, and why-unavailable
//! surfaces for every claimed M5 command family.
//!
//! The [frozen discoverability matrix][matrix] already binds each governed M5 last-mile command-discovery
//! surface — menu items, menu groups, context menus, command bars, keybinding resolver layers, conflict
//! review sheets, import-bridge rows, disabled-command explainers, leader/sequence help overlays, and
//! command-documentation surfaces — to one canonical command record, and freezes the required-label,
//! why-unavailable-reason, feature-family, discovery-channel, and downgrade-trigger vocabulary those
//! surfaces project from. This lane is the **explainer capstone** that certifies, for every one of those
//! ten surface families, that Aureline can *explain blocked or in-progress keyboard-first intent*: a leader
//! or partial-sequence overlay narrates the typed prefix, current mode, available next keys, resulting
//! command labels/ids, and timeout/cancel posture; a disabled-command / why-unavailable explainer names the
//! blocker class, the next safe action, and the copy-command-id / open-help actions; the same reason packet
//! and remediation language are reused across the palette, menu, keybinding UI, onboarding tips, and
//! support/export flows rather than reinvented per surface; and the blocker reason and command id are
//! reconstructable from a copy-safe, diffable export rather than a screenshot.
//!
//! For every surface family the lane certifies four things the acceptance criteria and implementation
//! requirements demand:
//!
//! - the surface's **leader / partial-sequence overlay** narrates the typed prefix, current mode, available
//!   next keys, resulting command labels/ids, timeout/cancel posture, and any surface-specific unsupported
//!   note — so next-available actions never require pointer hover or private docs
//!   ([`LeaderOverlayState`], acceptance criterion 3 + implementation requirement 1);
//! - the surface's **disabled-command / why-unavailable explainer** names the blocker class (context,
//!   trust, policy, lifecycle, missing dependency, stale target, mode overlay), the next safe action, and
//!   the copy-command-id / open-help actions — so a blocked command explains itself rather than failing
//!   silently or showing generic "unavailable" copy ([`BlockedExplainerState`], acceptance criterion 1 +
//!   implementation requirement 2);
//! - the surface reuses the **same reason packet and remediation language** across the palette, menu,
//!   keybinding UI, onboarding tips, and support/export flows — so the exact blocker and remediation path
//!   matches across every reach rather than drifting into surface-local error prose
//!   ([`RemediationParityState`], acceptance criterion 2 + implementation requirement 3);
//! - and the **blocker reason and command id are reconstructable** from a copy-safe, diffable export so
//!   support bundles, docs/help, and migration packets can name the same blocker and remediation without a
//!   screenshot ([`ExplainerExportState`], the copy-safe-introspection implementation requirement).
//!
//! Three records carry the truth:
//!
//! - the per-family **explainer row** ([`CommandExplainerRow`]): one row per [`M5CommandSurfaceFamily`]
//!   naming the canonical command binding it projects from, the required labels and feature families it
//!   exposes, the why-unavailable reasons it projects from the matrix, the leader-overlay fields it
//!   narrates, the blocker classes it names, the remediation actions it offers, the reach modes it stays
//!   reachable in, the consumer surfaces it evaluated, its leader-overlay / blocked-explainer /
//!   remediation-parity / explainer-export posture, whether the same explanation survives headless/CLI
//!   execution, any active waiver, and a derived green/yellow/red [`CommandExplainerStatus`].
//! - the explainer **packet** ([`CommandExplainerPacket`]): the full set of rows with derived per-row
//!   status, aggregate green/yellow/red counts, the active waivers, the exact conformance causes
//!   ([`CommandExplainerCause`]), and the blocking findings the lane refuses to ship with.
//! - the explainer **dashboard** ([`CommandExplainerDashboard`]): a light projection the palette / menu /
//!   keybinding UI / onboarding / Support Center / CLI tooling reads to auto-narrow a surface's explanation
//!   claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment a surface
//! discloses a reduced sequence overlay (a waivered narrowing), a disclosed reduced explainer detail, a
//! disclosed surface-local remediation note, or a disclosed partial explainer export/capture; it drops to
//! `red` if a leader sequence's availability requires hidden knowledge, a blocked command fails silently or
//! shows only generic copy, a surface invents surface-local error prose, the blocker reason cannot be
//! reconstructed from durable evidence, the same explanation is lost in a headless/CLI execution, or the
//! row fails to certify all six leader-overlay fields, all seven blocker classes, all three remediation
//! actions, all five reach modes, or every declared consumer surface. That derivation is the auto-narrowing
//! the acceptance criteria require, and the leader-overlay-field, blocker-class, remediation-action,
//! reach-mode, and consumer-surface completeness checks are the conformance lints that gate a stable
//! explanation claim.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local paths,
//! raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary, counts,
//! refs, and short labels. The surface-family, canonical-command-binding, required-label, lifecycle-label,
//! preview-class, feature-family, why-unavailable-reason, consumer-surface, downgrade-trigger, and
//! qualification vocabulary is re-exported by reference from the already frozen [matrix], and every
//! family's canonical command binding, qualification, owner, required labels, lifecycle label, feature
//! families, why-unavailable reasons, declared consumer surfaces, and applicable downgrade triggers are
//! pulled straight from that matrix's seeded packet, so this lane mints no parallel command vocabulary and
//! cannot certify a surface the matrix does not anchor. Only the explainer-specific vocabulary
//! ([`M5CommandExplainerDimension`], [`M5LeaderOverlayField`], [`M5BlockerClass`], [`M5RemediationAction`],
//! [`M5ExplanationReachMode`], [`CommandExplainerStatus`], [`LeaderOverlayState`], [`BlockedExplainerState`],
//! [`RemediationParityState`], [`ExplainerExportState`], [`CommandExplainerWaiver`],
//! [`CommandExplainerCause`], [`CommandExplainerFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix as matrix;

pub use matrix::{
    M5CanonicalCommandBinding, M5CommandSurfaceFamily, M5DisabledReasonMode,
    M5DiscoverabilityDowngradeTrigger, M5DiscoveryChannel, M5FeatureFamily, M5LifecycleLabel,
    M5PreviewClass, M5RequiredLabel, M5SurfaceQualificationClass, M5UnavailableReason,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_command_explainers_packet,
    seeded_m5_command_explainers_packet_context_menu_surface_local_prose_blocked,
    seeded_m5_command_explainers_packet_explainer_headless_parity_lost_blocked,
    seeded_m5_command_explainers_packet_import_bridge_capture_absent_blocked,
    seeded_m5_command_explainers_packet_leader_hidden_knowledge_blocked,
    seeded_m5_command_explainers_packet_menu_item_silent_failure_blocked, SEED_BUILD_IDENTITY_REF,
    SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_COMMAND_EXPLAINERS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_COMMAND_EXPLAINERS_SHARED_CONTRACT_REF: &str = "commands:m5_command_explainers:v1";

/// Stable record kind for [`CommandExplainerPacket`] payloads.
pub const M5_COMMAND_EXPLAINERS_PACKET_RECORD_KIND: &str =
    "commands_m5_command_explainers_packet_record";

/// Stable record kind for [`CommandExplainerDashboard`] payloads.
pub const M5_COMMAND_EXPLAINERS_DASHBOARD_RECORD_KIND: &str =
    "commands_m5_command_explainers_dashboard_record";

/// Stable record kind for [`CommandExplainerSupportExport`] payloads.
pub const M5_COMMAND_EXPLAINERS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "commands_m5_command_explainers_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_COMMAND_EXPLAINERS_PACKET_ID: &str = "m5-command-explainers:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_COMMAND_EXPLAINERS_DASHBOARD_ID: &str = "m5-command-explainers-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_COMMAND_EXPLAINERS_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-command-explainers:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_COMMAND_EXPLAINERS_SOURCE_SCHEMA_REF: &str =
    "schemas/commands/m5-command-explainers.schema.json";

/// Published markdown report ref reviewers reopen the explainer proof from.
pub const M5_COMMAND_EXPLAINERS_PUBLISHED_REPORT_REF: &str =
    "artifacts/commands/m5-command-explainers.md";

/// Published explainer-packet artifact ref.
pub const M5_COMMAND_EXPLAINERS_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-command-explainers-proof/packet.json";

/// Published explainer-dashboard artifact ref.
pub const M5_COMMAND_EXPLAINERS_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-command-explainers-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_COMMAND_EXPLAINERS_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-command-explainers-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_COMMAND_EXPLAINERS_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-command-explainers-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_COMMAND_EXPLAINERS_PUBLISHED_DOC_REF: &str =
    "docs/commands/m5_command_explainers_contract.md";

/// Repo-relative ref to the frozen discoverability boundary schema.
pub const M5_COMMAND_EXPLAINERS_MATRIX_SCHEMA_REF: &str = matrix::M5_DISCOVERABILITY_SCHEMA_REF;

/// Frozen discoverability contract doc this proof mirrors.
pub const M5_COMMAND_EXPLAINERS_MATRIX_DOC_REF: &str = matrix::M5_DISCOVERABILITY_DOC_REF;

/// Canonical command-descriptor schema every explainer surface projects from.
pub const M5_COMMAND_EXPLAINERS_COMMAND_DESCRIPTOR_REF: &str =
    matrix::M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF;

/// Every command-surface family the certification must cover, in canonical order. A certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_SURFACE_FAMILIES: [M5CommandSurfaceFamily; 10] = M5CommandSurfaceFamily::ALL;

/// Every explainer dimension each family row certifies, in canonical order.
pub const REQUIRED_EXPLAINER_DIMENSIONS: [M5CommandExplainerDimension; 4] =
    M5CommandExplainerDimension::ALL;

/// Every leader-overlay field each family row must narrate, in canonical order.
pub const REQUIRED_LEADER_OVERLAY_FIELDS: [M5LeaderOverlayField; 6] = M5LeaderOverlayField::ALL;

/// Every blocker class each family row must be able to name, in canonical order.
pub const REQUIRED_BLOCKER_CLASSES: [M5BlockerClass; 7] = M5BlockerClass::ALL;

/// Every remediation action each family row must offer, in canonical order.
pub const REQUIRED_REMEDIATION_ACTIONS: [M5RemediationAction; 3] = M5RemediationAction::ALL;

/// Every reach mode each family row must stay reachable in, in canonical order.
pub const REQUIRED_REACH_MODES: [M5ExplanationReachMode; 5] = M5ExplanationReachMode::ALL;

/// One of the four explainer dimensions each surface-family row certifies.
///
/// These are exactly the four ways the acceptance criteria and implementation requirements demand a claimed
/// M5 command surface explain blocked or in-progress keyboard-first intent: a leader / partial-sequence
/// overlay narrates the next-available actions; a disabled-command / why-unavailable explainer names the
/// blocker class, the next safe action, and the copy-id / open-help actions; the same reason packet and
/// remediation language stay stable across surfaces; and the blocker reason and command id reconstruct from
/// durable evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandExplainerDimension {
    /// The leader / partial-sequence overlay narrates next-available actions.
    LeaderOverlay,
    /// The disabled-command / why-unavailable explainer names the blocker and remediation.
    BlockedExplainer,
    /// The same reason packet and remediation language stay stable across surfaces.
    RemediationParity,
    /// The blocker reason and command id reconstruct from durable evidence.
    ExplainerExport,
}

impl M5CommandExplainerDimension {
    /// Every explainer dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LeaderOverlay,
        Self::BlockedExplainer,
        Self::RemediationParity,
        Self::ExplainerExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeaderOverlay => "leader_overlay",
            Self::BlockedExplainer => "blocked_explainer",
            Self::RemediationParity => "remediation_parity",
            Self::ExplainerExport => "explainer_export",
        }
    }
}

/// One of the six fields a leader / partial-sequence overlay must narrate for a claimed M5 command.
///
/// These are the exact fields the implementation requirements name: the typed prefix so far, the current
/// mode, the available next keys, the resulting command labels / ids, the timeout / cancel posture, and any
/// surface-specific unsupported note. An overlay that narrates fewer cannot honestly explain next-available
/// actions and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LeaderOverlayField {
    /// The prefix typed so far in the current sequence.
    TypedPrefix,
    /// The current keyboard mode / leader context.
    CurrentMode,
    /// The available next keys that continue the sequence.
    AvailableNextKeys,
    /// The resulting command labels and canonical ids each next key resolves to.
    ResultingCommandLabelAndId,
    /// The timeout / cancel posture of the pending sequence.
    TimeoutCancelPosture,
    /// A surface-specific note about an unsupported mode on this surface.
    SurfaceUnsupportedNote,
}

impl M5LeaderOverlayField {
    /// Every leader-overlay field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TypedPrefix,
        Self::CurrentMode,
        Self::AvailableNextKeys,
        Self::ResultingCommandLabelAndId,
        Self::TimeoutCancelPosture,
        Self::SurfaceUnsupportedNote,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypedPrefix => "typed_prefix",
            Self::CurrentMode => "current_mode",
            Self::AvailableNextKeys => "available_next_keys",
            Self::ResultingCommandLabelAndId => "resulting_command_label_and_id",
            Self::TimeoutCancelPosture => "timeout_cancel_posture",
            Self::SurfaceUnsupportedNote => "surface_unsupported_note",
        }
    }
}

/// One of the seven blocker classes a disabled-command / why-unavailable explainer must be able to name.
///
/// These are the exact blocker classes the implementation requirements name — context, trust, policy,
/// lifecycle, missing dependency, stale target, and mode overlay. They are the coarse taxonomy every
/// explainer groups the matrix's finer [`M5UnavailableReason`] set under, so a reader sees one governed
/// blocker vocabulary rather than a per-surface reinvention. An explainer that cannot name every class
/// cannot honestly explain a blocked command and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BlockerClass {
    /// The command needs a context (selection / focus / target) it does not have.
    Context,
    /// The command is blocked by a trust / entitlement boundary.
    Trust,
    /// The command is blocked by a policy or legal control.
    Policy,
    /// The command is blocked by a lifecycle / deprecation state.
    Lifecycle,
    /// The command is blocked by a missing capability / dependency.
    MissingDependency,
    /// The command's target moved, was removed, or lost its context.
    StaleTarget,
    /// The command is blocked by an active mode overlay (modal / leader).
    ModeOverlay,
}

impl M5BlockerClass {
    /// Every blocker class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Context,
        Self::Trust,
        Self::Policy,
        Self::Lifecycle,
        Self::MissingDependency,
        Self::StaleTarget,
        Self::ModeOverlay,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::Trust => "trust",
            Self::Policy => "policy",
            Self::Lifecycle => "lifecycle",
            Self::MissingDependency => "missing_dependency",
            Self::StaleTarget => "stale_target",
            Self::ModeOverlay => "mode_overlay",
        }
    }
}

/// One of the three remediation actions a disabled-command / why-unavailable explainer must offer.
///
/// These are the actions the implementation requirements name: the next safe action a user can take, the
/// copy-command-id action, and the open-help action. An explainer that offers fewer leaves a user stuck at
/// a blocked command and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemediationAction {
    /// The next safe action the user can take to unblock the command.
    NextSafeAction,
    /// Copy the canonical command id for support / search.
    CopyCommandId,
    /// Open the command's help / docs anchor.
    OpenHelp,
}

impl M5RemediationAction {
    /// Every remediation action, in declaration order.
    pub const ALL: [Self; 3] = [Self::NextSafeAction, Self::CopyCommandId, Self::OpenHelp];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NextSafeAction => "next_safe_action",
            Self::CopyCommandId => "copy_command_id",
            Self::OpenHelp => "open_help",
        }
    }
}

/// One of the five reach modes an explanation surface must stay reachable in.
///
/// These are the fallback cases the implementation requirements name — the explanation must be reachable
/// keyboard-only, through a screen reader, in a compact layout, and through a touch / context-action
/// fallback, without depending on pointer hover — plus the pointer default. A surface reachable in fewer
/// hides its explanation behind hover or modal confusion and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplanationReachMode {
    /// Reachable through pointer interaction (the default).
    PointerDefault,
    /// Reachable keyboard-only, without a pointer.
    KeyboardOnly,
    /// Reachable / announced through a screen reader.
    ScreenReader,
    /// Reachable in a compact / constrained layout.
    CompactLayout,
    /// Reachable through a touch / context-action fallback.
    TouchContextAction,
}

impl M5ExplanationReachMode {
    /// Every reach mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PointerDefault,
        Self::KeyboardOnly,
        Self::ScreenReader,
        Self::CompactLayout,
        Self::TouchContextAction,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PointerDefault => "pointer_default",
            Self::KeyboardOnly => "keyboard_only",
            Self::ScreenReader => "screen_reader",
            Self::CompactLayout => "compact_layout",
            Self::TouchContextAction => "touch_context_action",
        }
    }
}

/// The derived explanation light a command surface carries.
///
/// `green` means the surface narrates a full leader / partial-sequence overlay, names the blocker class,
/// next safe action, and copy-id / open-help actions on a blocked command, reuses the shared reason packet
/// and remediation language across every reach, and reconstructs its blocker reason and command id from
/// durable evidence — across every declared consumer surface and every reach mode, with the same
/// explanation surviving headless/CLI execution. `yellow` is a disclosed narrowing. `red` is blocked and
/// may not keep an explanation claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandExplainerStatus {
    /// Full standing: all four explainer dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl CommandExplainerStatus {
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

/// How the surface narrates its leader / partial-sequence overlay.
///
/// `typed_prefix_next_keys_and_timeout_narrated` means the overlay shows the typed prefix, the current
/// mode, the available next keys, the resulting command labels / ids, the timeout / cancel posture, and any
/// surface-specific unsupported note — so next-available actions never require pointer hover or private
/// docs. `disclosed_reduced_sequence_overlay` means a constrained surface folds the resulting-label detail
/// into an expandable hint while still showing the typed prefix, mode, next keys, and timeout / cancel
/// posture (a yellow narrowing that **requires an active waiver**).
/// `sequence_availability_requires_hidden_knowledge` means the sequence's next-available actions can only be
/// discovered by already knowing them — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaderOverlayState {
    /// The overlay narrates typed prefix, next keys, timeout / cancel, and resulting labels / ids.
    TypedPrefixNextKeysAndTimeoutNarrated,
    /// A constrained surface takes a disclosed, waivered reduced sequence overlay.
    DisclosedReducedSequenceOverlay,
    /// Sequence availability requires hidden knowledge — a blocker.
    SequenceAvailabilityRequiresHiddenKnowledge,
}

impl LeaderOverlayState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypedPrefixNextKeysAndTimeoutNarrated => {
                "typed_prefix_next_keys_and_timeout_narrated"
            }
            Self::DisclosedReducedSequenceOverlay => "disclosed_reduced_sequence_overlay",
            Self::SequenceAvailabilityRequiresHiddenKnowledge => {
                "sequence_availability_requires_hidden_knowledge"
            }
        }
    }

    /// `true` when the leader overlay is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::TypedPrefixNextKeysAndTimeoutNarrated)
    }

    /// `true` when the surface took a disclosed reduced-sequence-overlay narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedSequenceOverlay)
    }
}

/// How the surface explains a disabled / unavailable command.
///
/// `blocker_class_next_action_and_actions_certified` means the explainer names the blocker class, the next
/// safe action, and the copy-command-id / open-help actions rather than greying the command out silently.
/// `disclosed_reduced_explainer_detail` means a constrained surface folds the next-safe-action detail into
/// an expandable section while still naming the blocker class and offering copy-id / open-help (a yellow
/// narrowing). `blocked_command_fails_silently_or_generic` means the command greys out with no explanation
/// or only generic "unavailable" copy — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedExplainerState {
    /// The blocker class, next safe action, and copy-id / open-help actions are certified.
    BlockerClassNextActionAndActionsCertified,
    /// A constrained surface takes a disclosed reduced explainer detail.
    DisclosedReducedExplainerDetail,
    /// The blocked command fails silently or shows only generic copy — a blocker.
    BlockedCommandFailsSilentlyOrGeneric,
}

impl BlockedExplainerState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockerClassNextActionAndActionsCertified => {
                "blocker_class_next_action_and_actions_certified"
            }
            Self::DisclosedReducedExplainerDetail => "disclosed_reduced_explainer_detail",
            Self::BlockedCommandFailsSilentlyOrGeneric => {
                "blocked_command_fails_silently_or_generic"
            }
        }
    }

    /// `true` when the blocked-command explainer is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::BlockerClassNextActionAndActionsCertified)
    }

    /// `true` when the surface took a disclosed reduced-explainer-detail narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedExplainerDetail)
    }
}

/// How the surface keeps the same reason packet and remediation language stable across the palette, menu,
/// keybinding UI, onboarding tips, and support/export flows.
///
/// `shared_reason_packet_across_all_surfaces` means every surface projects the same reason packet and
/// remediation language rather than reinventing surface-local prose. `disclosed_surface_local_remediation_note`
/// means one constrained surface appends a disclosed, short surface-local remediation note while still
/// projecting the shared reason packet and remediation language (a yellow narrowing).
/// `surface_local_error_prose_invented` means a surface invented its own error / remediation prose that
/// disagrees with the shared reason packet — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemediationParityState {
    /// The shared reason packet and remediation language are certified stable across surfaces.
    SharedReasonPacketAcrossAllSurfaces,
    /// One constrained surface appends a disclosed short surface-local remediation note.
    DisclosedSurfaceLocalRemediationNote,
    /// A surface invented surface-local error / remediation prose — a blocker.
    SurfaceLocalErrorProseInvented,
}

impl RemediationParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedReasonPacketAcrossAllSurfaces => "shared_reason_packet_across_all_surfaces",
            Self::DisclosedSurfaceLocalRemediationNote => {
                "disclosed_surface_local_remediation_note"
            }
            Self::SurfaceLocalErrorProseInvented => "surface_local_error_prose_invented",
        }
    }

    /// `true` when remediation parity is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::SharedReasonPacketAcrossAllSurfaces)
    }

    /// `true` when the surface took a disclosed surface-local-remediation-note narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedSurfaceLocalRemediationNote)
    }
}

/// How the explainer packet reconstructs the blocker reason and command id.
///
/// `blocker_and_command_id_reconstructable` means a support bundle, doc, or migration packet can
/// reconstruct the blocker class, reason, and command id from a durable, copy-safe, diffable export without
/// a screenshot. `disclosed_partial_capture` means one legacy export captures the blocker class and command
/// id but not the full remediation-action list, while still disclosing the gap (a yellow narrowing).
/// `blocker_reason_absent_from_capture` means the blocker reason or command id is absent from durable
/// evidence — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainerExportState {
    /// Blocker reason and command id are reconstructable from durable evidence.
    BlockerAndCommandIdReconstructable,
    /// One legacy export takes a disclosed partial capture.
    DisclosedPartialCapture,
    /// The blocker reason or command id is absent from durable evidence — a blocker.
    BlockerReasonAbsentFromCapture,
}

impl ExplainerExportState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockerAndCommandIdReconstructable => "blocker_and_command_id_reconstructable",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::BlockerReasonAbsentFromCapture => "blocker_reason_absent_from_capture",
        }
    }

    /// `true` when explainer export parity is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::BlockerAndCommandIdReconstructable)
    }

    /// `true` when the surface took a disclosed partial-capture narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather than
/// blocked — never lets a hidden-knowledge sequence, a silent failure, invented prose, or an uncapturable
/// blocker reason hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExplainerWaiver {
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

impl CommandExplainerWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a surface family's explanation.
///
/// The trigger token mirrors the frozen [`M5DiscoverabilityDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExplainerCause {
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

impl CommandExplainerCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One surface family, certified across its leader-overlay, blocked-explainer, remediation-parity, and
/// explainer-export dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExplainerRow {
    /// The surface family being certified.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short reviewer-facing family label.
    pub surface_label: String,
    /// Qualification class the matrix earned for the surface. Pulled from the matrix.
    pub qualification: M5SurfaceQualificationClass,
    /// Owner role accountable for keeping this surface's explanation governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short conformance scope summary.
    pub scope_summary: String,
    /// The canonical command-record binding this surface projects from. Pulled from the matrix.
    pub canonical_command_binding: M5CanonicalCommandBinding,
    /// The pinned lifecycle / deprecation label. Pulled from the canonical command binding.
    pub lifecycle_label: M5LifecycleLabel,
    /// Mandatory labels this surface must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5RequiredLabel>,
    /// M5 feature families whose commands this surface explains. Pulled from the matrix.
    pub feature_families: Vec<M5FeatureFamily>,
    /// Why-unavailable reasons the surface projects from the matrix (the fine taxonomy the blocker classes
    /// group). Pulled from the matrix.
    pub covered_unavailable_reasons: Vec<M5UnavailableReason>,
    /// The leader-overlay fields this row narrates (must be all six).
    pub certified_leader_overlay_fields: Vec<M5LeaderOverlayField>,
    /// The blocker classes this row can name (must be all seven).
    pub certified_blocker_classes: Vec<M5BlockerClass>,
    /// The remediation actions this row offers (must be all three).
    pub certified_remediation_actions: Vec<M5RemediationAction>,
    /// The reach modes this row stays reachable in (must be all five).
    pub certified_reach_modes: Vec<M5ExplanationReachMode>,
    /// Consumer surfaces the matrix declares the surface must project to.
    pub required_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Leader-overlay posture.
    pub leader_overlay: LeaderOverlayState,
    /// Blocked-explainer posture.
    pub blocked_explainer: BlockedExplainerState,
    /// Remediation-parity posture.
    pub remediation_parity: RemediationParityState,
    /// Explainer-export posture.
    pub explainer_export: ExplainerExportState,
    /// `true` when the same explanation survives a headless / CLI execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the surface. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced sequence overlay is in force.
    pub active_waiver: Option<CommandExplainerWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: CommandExplainerStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<CommandExplainerCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl CommandExplainerRow {
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

    /// `true` when the row narrates every one of the six leader-overlay fields — the structural proof that
    /// the overlay explains next-available actions.
    pub fn leader_overlay_fields_complete(&self) -> bool {
        complete_tokens(
            &self.certified_leader_overlay_fields,
            |field| field.as_str(),
            &REQUIRED_LEADER_OVERLAY_FIELDS,
            |field| field.as_str(),
        )
    }

    /// `true` when the row can name every one of the seven blocker classes — the structural proof that the
    /// explainer names the blocker rather than greying out silently.
    pub fn blocker_classes_complete(&self) -> bool {
        complete_tokens(
            &self.certified_blocker_classes,
            |class| class.as_str(),
            &REQUIRED_BLOCKER_CLASSES,
            |class| class.as_str(),
        )
    }

    /// `true` when the row offers every one of the three remediation actions — the structural proof that a
    /// blocked command offers a next safe action, copy-id, and open-help.
    pub fn remediation_actions_complete(&self) -> bool {
        complete_tokens(
            &self.certified_remediation_actions,
            |action| action.as_str(),
            &REQUIRED_REMEDIATION_ACTIONS,
            |action| action.as_str(),
        )
    }

    /// `true` when the row stays reachable in every one of the five reach modes — the structural proof that
    /// the explanation is not hidden behind hover or modal confusion.
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
        if !self.leader_overlay_fields_complete() {
            return true;
        }
        if !self.blocker_classes_complete() {
            return true;
        }
        if !self.remediation_actions_complete() {
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
            self.leader_overlay,
            LeaderOverlayState::SequenceAvailabilityRequiresHiddenKnowledge
        ) {
            return true;
        }
        if matches!(
            self.blocked_explainer,
            BlockedExplainerState::BlockedCommandFailsSilentlyOrGeneric
        ) {
            return true;
        }
        if matches!(
            self.remediation_parity,
            RemediationParityState::SurfaceLocalErrorProseInvented
        ) {
            return true;
        }
        if matches!(
            self.explainer_export,
            ExplainerExportState::BlockerReasonAbsentFromCapture
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.leader_overlay.is_disclosed_narrowing()
            || self.blocked_explainer.is_disclosed_narrowing()
            || self.remediation_parity.is_disclosed_narrowing()
            || self.explainer_export.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the explanation posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> CommandExplainerStatus {
        if self.has_hard_blocker() {
            CommandExplainerStatus::Red
        } else if self.has_narrowing() {
            CommandExplainerStatus::Yellow
        } else {
            CommandExplainerStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (leader overlay, blocked
    /// explainer, remediation parity, explainer export, then structural completeness and headless parity).
    pub fn recompute_causes(&self) -> Vec<CommandExplainerCause> {
        let mut causes = Vec::new();
        match self.leader_overlay {
            LeaderOverlayState::TypedPrefixNextKeysAndTimeoutNarrated => {}
            LeaderOverlayState::DisclosedReducedSequenceOverlay => {
                causes.push(CommandExplainerCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                    disclosed: true,
                    detail: "On a constrained surface the leader / sequence overlay takes a disclosed, \
                             waivered reduced form — the resulting command labels / ids are folded into an \
                             expandable hint while the typed prefix, current mode, available next keys, and \
                             timeout / cancel posture stay visible — so the overlay is narrowed and \
                             disclosed rather than hiding next-available actions behind hover."
                        .to_owned(),
                });
            }
            LeaderOverlayState::SequenceAvailabilityRequiresHiddenKnowledge => {
                causes.push(CommandExplainerCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ConflictWinnerAmbiguous,
                    disclosed: false,
                    detail: "The leader / sequence overlay does not narrate the typed prefix, next keys, or \
                             timeout / cancel posture, so the sequence's next-available actions can only be \
                             discovered by already knowing them — a reader cannot learn the sequence from \
                             the surface."
                        .to_owned(),
                });
            }
        }
        match self.blocked_explainer {
            BlockedExplainerState::BlockerClassNextActionAndActionsCertified => {}
            BlockedExplainerState::DisclosedReducedExplainerDetail => {
                causes.push(CommandExplainerCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::DisabledReasonHidden,
                    disclosed: true,
                    detail: "On a constrained surface the disabled-command explainer takes a disclosed \
                             reduced detail — the next-safe-action guidance is folded into an expandable \
                             section while the blocker class and the copy-command-id / open-help actions \
                             stay visible — so the explainer is narrowed and disclosed rather than failing \
                             silently or showing only generic copy."
                        .to_owned(),
                });
            }
            BlockedExplainerState::BlockedCommandFailsSilentlyOrGeneric => {
                causes.push(CommandExplainerCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::DisabledReasonHidden,
                    disclosed: false,
                    detail: "The command greys out with no explanation, or with only generic \"unavailable\" \
                             copy, so a reader cannot see the blocker class, the next safe action, or how to \
                             copy the command id or open help."
                        .to_owned(),
                });
            }
        }
        match self.remediation_parity {
            RemediationParityState::SharedReasonPacketAcrossAllSurfaces => {}
            RemediationParityState::DisclosedSurfaceLocalRemediationNote => {
                causes.push(CommandExplainerCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                    disclosed: true,
                    detail: "One constrained surface appends a disclosed short surface-local remediation \
                             note while still projecting the shared reason packet and remediation language, \
                             so the remediation is narrowed and disclosed rather than an invented \
                             surface-local error prose."
                        .to_owned(),
                });
            }
            RemediationParityState::SurfaceLocalErrorProseInvented => {
                causes.push(CommandExplainerCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                    disclosed: false,
                    detail: "A palette / menu / keybinding UI / onboarding / support surface invented its \
                             own error and remediation prose that disagrees with the shared reason packet, \
                             so the exact blocker and remediation path reads differently depending on where \
                             the command is reached."
                        .to_owned(),
                });
            }
        }
        match self.explainer_export {
            ExplainerExportState::BlockerAndCommandIdReconstructable => {}
            ExplainerExportState::DisclosedPartialCapture => {
                causes.push(CommandExplainerCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One legacy explainer export takes a disclosed partial capture — the export \
                             captures the blocker class and command id but not the full remediation-action \
                             list, while still disclosing the gap — so the copy-safe export parity is \
                             narrowed and disclosed rather than absent."
                        .to_owned(),
                });
            }
            ExplainerExportState::BlockerReasonAbsentFromCapture => {
                causes.push(CommandExplainerCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail: "The blocker class / reason or the command id is absent from the durable, \
                             diffable explainer export, so a support bundle, doc, or migration packet cannot \
                             reconstruct the same blocker and remediation without a screenshot."
                        .to_owned(),
                });
            }
        }
        if !self.leader_overlay_fields_complete() {
            causes.push(CommandExplainerCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                disclosed: false,
                detail: "The leader / sequence overlay does not narrate all six fields — typed prefix, \
                         current mode, available next keys, resulting command label / id, timeout / cancel \
                         posture, and surface-specific unsupported note — so the next-available actions are \
                         incompletely explained."
                    .to_owned(),
            });
        }
        if !self.blocker_classes_complete() {
            causes.push(CommandExplainerCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::DisabledReasonHidden,
                disclosed: false,
                detail: "The disabled-command explainer cannot name all seven blocker classes — context, \
                         trust, policy, lifecycle, missing dependency, stale target, and mode overlay — so \
                         some blocked commands would fall back to a generic unavailable state."
                    .to_owned(),
            });
        }
        if !self.remediation_actions_complete() {
            causes.push(CommandExplainerCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::CommandIdMissing,
                disclosed: false,
                detail: "The explainer does not offer all three remediation actions — next safe action, \
                         copy command id, and open help — so a reader could be left at a blocked command \
                         without a way to copy the command id or open help."
                    .to_owned(),
            });
        }
        if !self.reach_modes_complete() {
            causes.push(CommandExplainerCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "The explanation is not reachable in all five reach modes — pointer, keyboard-only, \
                         screen reader, compact layout, and touch / context-action fallback — so the \
                         explanation could be hidden behind hover or modal confusion in some modes."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(CommandExplainerCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "A headless / CLI execution of this surface lost the shared explanation, so the \
                         same command explains a different blocker or remediation depending on how it is \
                         reached."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced sequence overlay may only stay yellow (rather than red) when a waiver discloses
    /// it — reducing the leader overlay's resulting-label detail is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.leader_overlay,
            LeaderOverlayState::DisclosedReducedSequenceOverlay
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<CommandExplainerFinding> {
        let mut findings = Vec::new();
        let family = self.surface_family.as_str().to_owned();

        if !self.leader_overlay_fields_complete() {
            findings.push(CommandExplainerFinding::LeaderOverlayFieldsIncomplete {
                family: family.clone(),
            });
        }
        if !self.blocker_classes_complete() {
            findings.push(CommandExplainerFinding::BlockerClassesIncomplete {
                family: family.clone(),
            });
        }
        if !self.remediation_actions_complete() {
            findings.push(CommandExplainerFinding::RemediationActionsIncomplete {
                family: family.clone(),
            });
        }
        if !self.reach_modes_complete() {
            findings.push(CommandExplainerFinding::ReachModesIncomplete {
                family: family.clone(),
            });
        }
        if !self.consumer_surfaces_complete() {
            findings.push(CommandExplainerFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(CommandExplainerFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.leader_overlay,
            LeaderOverlayState::SequenceAvailabilityRequiresHiddenKnowledge
        ) {
            findings.push(CommandExplainerFinding::LeaderOverlayBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.blocked_explainer,
            BlockedExplainerState::BlockedCommandFailsSilentlyOrGeneric
        ) {
            findings.push(CommandExplainerFinding::BlockedExplainerBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.remediation_parity,
            RemediationParityState::SurfaceLocalErrorProseInvented
        ) {
            findings.push(CommandExplainerFinding::RemediationParityBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.explainer_export,
            ExplainerExportState::BlockerReasonAbsentFromCapture
        ) {
            findings.push(CommandExplainerFinding::ExplainerExportBroken {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, CommandExplainerStatus::Green) && !self.has_reason() {
            findings.push(CommandExplainerFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(CommandExplainerFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.surface_family != self.surface_family {
                findings.push(CommandExplainerFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(CommandExplainerFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(CommandExplainerFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(CommandExplainerFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} leader={} blocked={} remediation={} export={} headless={} lifecycle={} fields={} classes={} actions={} modes={} surfaces={} waiver={}",
            self.surface_family.as_str(),
            self.derived_status.as_str(),
            self.leader_overlay.as_str(),
            self.blocked_explainer.as_str(),
            self.remediation_parity.as_str(),
            self.explainer_export.as_str(),
            self.headless_parity_preserved,
            self.lifecycle_label.as_str(),
            self.certified_leader_overlay_fields.len(),
            self.certified_blocker_classes.len(),
            self.certified_remediation_actions.len(),
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

/// A blocking finding the explainer certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum CommandExplainerFinding {
    /// A surface family has no explainer row.
    SurfaceFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row did not narrate every leader-overlay field.
    LeaderOverlayFieldsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row could not name every blocker class.
    BlockerClassesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not offer every remediation action.
    RemediationActionsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row is not reachable in every reach mode.
    ReachModesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless / CLI execution lost the shared explanation.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// A leader sequence's availability requires hidden knowledge.
    LeaderOverlayBroken {
        /// The family token.
        family: String,
    },
    /// A blocked command fails silently or shows only generic copy.
    BlockedExplainerBroken {
        /// The family token.
        family: String,
    },
    /// A surface invented surface-local error / remediation prose.
    RemediationParityBroken {
        /// The family token.
        family: String,
    },
    /// The blocker reason or command id is absent from the durable explainer export.
    ExplainerExportBroken {
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

impl CommandExplainerFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::SurfaceFamilyMissing { .. } => "surface_family_missing",
            Self::LeaderOverlayFieldsIncomplete { .. } => "leader_overlay_fields_incomplete",
            Self::BlockerClassesIncomplete { .. } => "blocker_classes_incomplete",
            Self::RemediationActionsIncomplete { .. } => "remediation_actions_incomplete",
            Self::ReachModesIncomplete { .. } => "reach_modes_incomplete",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::LeaderOverlayBroken { .. } => "leader_overlay_broken",
            Self::BlockedExplainerBroken { .. } => "blocked_explainer_broken",
            Self::RemediationParityBroken { .. } => "remediation_parity_broken",
            Self::ExplainerExportBroken { .. } => "explainer_export_broken",
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
            | Self::LeaderOverlayFieldsIncomplete { family }
            | Self::BlockerClassesIncomplete { family }
            | Self::RemediationActionsIncomplete { family }
            | Self::ReachModesIncomplete { family }
            | Self::ConsumerSurfacesIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::LeaderOverlayBroken { family }
            | Self::BlockedExplainerBroken { family }
            | Self::RemediationParityBroken { family }
            | Self::ExplainerExportBroken { family }
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

/// The explainer packet shared by the palette / menu / keybinding UI / onboarding / Support Center / CLI
/// tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExplainerPacket {
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
    /// Canonical command-descriptor schema every explainer surface projects from.
    pub command_descriptor_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four explainer dimensions every family row certifies.
    pub required_explainer_dimensions: Vec<String>,
    /// The six leader-overlay fields every family row must narrate.
    pub required_leader_overlay_fields: Vec<String>,
    /// The seven blocker classes every family row must be able to name.
    pub required_blocker_classes: Vec<String>,
    /// The three remediation actions every family row must offer.
    pub required_remediation_actions: Vec<String>,
    /// The five reach modes every family row must stay reachable in.
    pub required_reach_modes: Vec<String>,
    /// The ten surface families the certification must cover.
    pub required_surface_families: Vec<String>,
    /// Per-family explainer rows, in canonical order.
    pub rows: Vec<CommandExplainerRow>,
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
    pub active_waivers: Vec<CommandExplainerWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<CommandExplainerCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<CommandExplainerFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Command / explainer automation refs that consume this packet to auto-narrow a surface.
    pub command_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help / onboarding refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published explainer-packet ref.
    pub published_packet_ref: String,
    /// Published explainer-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl CommandExplainerPacket {
    /// Returns the explainer row for `family`, if present.
    pub fn row(&self, family: M5CommandSurfaceFamily) -> Option<&CommandExplainerRow> {
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

    /// Projects the light explainer dashboard the command automation consumes.
    pub fn dashboard(&self) -> CommandExplainerDashboard {
        CommandExplainerDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 command-explainers packet serializes")
    }

    /// Deterministic, machine-readable explainer CSV: one row per surface family naming its status, the
    /// four explanation postures, headless parity, the lifecycle label, the field / class / action / mode
    /// counts, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,status,leader_overlay,blocked_explainer,remediation_parity,explainer_export,headless_parity,lifecycle,leader_fields,blocker_classes,remediation_actions,reach_modes,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.derived_status.as_str(),
                row.leader_overlay.as_str(),
                row.blocked_explainer.as_str(),
                row.remediation_parity.as_str(),
                row.explainer_export.as_str(),
                row.headless_parity_preserved,
                row.lifecycle_label.as_str(),
                row.certified_leader_overlay_fields.len(),
                row.certified_blocker_classes.len(),
                row.certified_remediation_actions.len(),
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
            "# M5 command explainers: leader/partial-sequence overlays, disabled-command and why-unavailable explainers, shared remediation, and copy-safe blocker export across every claimed M5 command surface\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_command_explainers`](../../crates/aureline-shell/src/m5_command_explainers/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_command_explainers -- markdown > \\\n  artifacts/commands/m5-command-explainers.md\n",
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
            "- Required explainer dimensions: {}\n",
            self.required_explainer_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Leader-overlay fields narrated: {}\n",
            self.required_leader_overlay_fields
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Blocker classes named: {}\n",
            self.required_blocker_classes
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Remediation actions offered: {}\n",
            self.required_remediation_actions
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

        out.push_str("## Explainer rows\n\n");
        out.push_str(
            "| Surface family | Status | Leader overlay | Blocked explainer | Remediation parity | Explainer export | Lifecycle | Headless | Waiver |\n\
             | -------------- | ------ | -------------- | ----------------- | ------------------ | ---------------- | --------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.leader_overlay.as_str(),
                row.blocked_explainer.as_str(),
                row.remediation_parity.as_str(),
                row.explainer_export.as_str(),
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
        let narrowed: Vec<&CommandExplainerRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, CommandExplainerStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed M5 command surface narrates a full leader / partial-sequence overlay, names the blocker class, next safe action, and copy-id / open-help actions on a blocked command, reuses the shared reason packet and remediation language across every reach, and reconstructs its blocker reason and command id from durable evidence across every declared consumer surface.\n\n",
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_command_explainers -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_command_explainers_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light explainer dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExplainerDashboardRow {
    /// The surface family.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short family label.
    pub surface_label: String,
    /// Qualification class earned by the surface.
    pub qualification: M5SurfaceQualificationClass,
    /// Derived green/yellow/red status.
    pub status: CommandExplainerStatus,
    /// The pinned lifecycle / deprecation label.
    pub lifecycle_label: M5LifecycleLabel,
    /// Number of leader-overlay fields narrated.
    pub certified_leader_overlay_field_count: usize,
    /// Number of blocker classes named.
    pub certified_blocker_class_count: usize,
    /// Number of remediation actions offered.
    pub certified_remediation_action_count: usize,
    /// Number of reach modes covered.
    pub certified_reach_mode_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Leader-overlay posture.
    pub leader_overlay: LeaderOverlayState,
    /// Blocked-explainer posture.
    pub blocked_explainer: BlockedExplainerState,
    /// Remediation-parity posture.
    pub remediation_parity: RemediationParityState,
    /// Explainer-export posture.
    pub explainer_export: ExplainerExportState,
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

/// The light explainer dashboard the palette / menu / keybinding UI / onboarding / Support Center / CLI
/// tooling reads to auto-narrow a surface's explanation claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExplainerDashboard {
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
    pub rows: Vec<CommandExplainerDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Command / explainer automation refs that consume the dashboard.
    pub command_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl CommandExplainerDashboard {
    /// Projects the dashboard from an explainer packet.
    pub fn from_packet(packet: &CommandExplainerPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| CommandExplainerDashboardRow {
                surface_family: row.surface_family,
                surface_label: row.surface_label.clone(),
                qualification: row.qualification,
                status: row.derived_status,
                lifecycle_label: row.lifecycle_label,
                certified_leader_overlay_field_count: row.certified_leader_overlay_fields.len(),
                certified_blocker_class_count: row.certified_blocker_classes.len(),
                certified_remediation_action_count: row.certified_remediation_actions.len(),
                certified_reach_mode_count: row.certified_reach_modes.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                leader_overlay: row.leader_overlay,
                blocked_explainer: row.blocked_explainer,
                remediation_parity: row.remediation_parity,
                explainer_export: row.explainer_export,
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
            record_kind: M5_COMMAND_EXPLAINERS_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_COMMAND_EXPLAINERS_SCHEMA_VERSION,
            dashboard_id: M5_COMMAND_EXPLAINERS_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 command-explainers dashboard serializes")
    }
}

/// Support-export wrapper for the explainer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExplainerSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: CommandExplainerPacket,
    /// Dashboard quoted in full.
    pub dashboard: CommandExplainerDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl CommandExplainerSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each surface family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the palette / keybinding tooling — can
    /// name the same surface and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: CommandExplainerPacket,
    ) -> Self {
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
            record_kind: M5_COMMAND_EXPLAINERS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_COMMAND_EXPLAINERS_SCHEMA_VERSION,
            shared_contract_ref: M5_COMMAND_EXPLAINERS_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_command_explainers_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandExplainerInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen discoverability matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family explainer rows.
    pub rows: Vec<CommandExplainerRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The explainer packet carries only closed vocabulary, refs, and short labels, so raw URLs, credentials,
/// or tokens must never appear.
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

/// Builds a [`CommandExplainerPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-family explainer rows.
///
/// Each row's derived status and conformance causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the auto-narrowing
/// cannot be asserted.
pub fn build_m5_command_explainers_packet(input: CommandExplainerInput) -> CommandExplainerPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<CommandExplainerRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<CommandExplainerFinding> = Vec::new();

    // Every surface family must carry an explainer row.
    let present: BTreeSet<M5CommandSurfaceFamily> =
        rows.iter().map(|row| row.surface_family).collect();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(CommandExplainerFinding::SurfaceFamilyMissing {
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
        .filter(|row| matches!(row.derived_status, CommandExplainerStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CommandExplainerStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CommandExplainerStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(CommandExplainerFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<CommandExplainerWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<CommandExplainerCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_explainer_dimensions: Vec<String> = REQUIRED_EXPLAINER_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_leader_overlay_fields: Vec<String> = REQUIRED_LEADER_OVERLAY_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    let required_blocker_classes: Vec<String> = REQUIRED_BLOCKER_CLASSES
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();
    let required_remediation_actions: Vec<String> = REQUIRED_REMEDIATION_ACTIONS
        .iter()
        .map(|action| action.as_str().to_owned())
        .collect();
    let required_reach_modes: Vec<String> = REQUIRED_REACH_MODES
        .iter()
        .map(|mode| mode.as_str().to_owned())
        .collect();
    let required_surface_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = CommandExplainerPacket {
        record_kind: M5_COMMAND_EXPLAINERS_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_COMMAND_EXPLAINERS_SCHEMA_VERSION,
        shared_contract_ref: M5_COMMAND_EXPLAINERS_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_COMMAND_EXPLAINERS_PACKET_ID.to_owned(),
        source_schema_ref: M5_COMMAND_EXPLAINERS_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Command explainers for every claimed M5 command surface: each of the ten governed \
                   surface families certified so a keyboard-first user, screen-reader listener, doc, \
                   automation, or support reviewer can explain blocked or in-progress intent — a leader / \
                   partial-sequence overlay narrating the typed prefix, current mode, available next keys, \
                   resulting command labels / ids, and timeout / cancel posture; a disabled-command / \
                   why-unavailable explainer naming the blocker class, the next safe action, and the \
                   copy-command-id / open-help actions; the same reason packet and remediation language \
                   reused across palette, menu, keybinding UI, onboarding, and support/export flows; and a \
                   copy-safe, diffable export that reconstructs the blocker reason and command id — across \
                   every declared consumer surface and every reach mode, with the same explanation \
                   preserved in headless/CLI execution, each surface's green/yellow/red claim auto-narrowed \
                   from its four explanation postures, and any surface that hides a sequence behind hidden \
                   knowledge, fails a blocked command silently, invents surface-local prose, or cannot \
                   reconstruct its blocker from durable evidence blocked from a stable claim."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_COMMAND_EXPLAINERS_MATRIX_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_COMMAND_EXPLAINERS_MATRIX_DOC_REF.to_owned(),
        command_descriptor_ref: M5_COMMAND_EXPLAINERS_COMMAND_DESCRIPTOR_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_explainer_dimensions,
        required_leader_overlay_fields,
        required_blocker_classes,
        required_remediation_actions,
        required_reach_modes,
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
            "command_status.explainer_registry".to_owned(),
            "explainer_automation.auto_narrow.command_explainer_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.command_explainers".to_owned(),
            M5_COMMAND_EXPLAINERS_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_COMMAND_EXPLAINERS_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-command-explainers".to_owned()],
        published_report_ref: M5_COMMAND_EXPLAINERS_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_COMMAND_EXPLAINERS_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_COMMAND_EXPLAINERS_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_COMMAND_EXPLAINERS_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("explainer packet serializes"),
    ) {
        blocking_findings.push(CommandExplainerFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_command_explainers_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CommandExplainerValidationError {
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
    /// The declared required explainer dimensions do not match the lane constants.
    RequiredExplainerDimensionsStale,
    /// The declared required leader-overlay fields do not match the lane constants.
    RequiredLeaderOverlayFieldsStale,
    /// The declared required blocker classes do not match the lane constants.
    RequiredBlockerClassesStale,
    /// The declared required remediation actions do not match the lane constants.
    RequiredRemediationActionsStale,
    /// The declared required reach modes do not match the lane constants.
    RequiredReachModesStale,
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

/// Validates a packet against the explainer invariants.
///
/// The checks encode the track invariant and acceptance criteria: every surface family carries a current
/// explainer row; each row's status is the derived value, never asserted; a green row cannot keep a claim
/// while it hides a sequence behind hidden knowledge, fails a blocked command silently or with generic
/// copy, invents surface-local error prose, cannot reconstruct its blocker reason from durable evidence,
/// loses headless/CLI parity, fails to narrate all six leader-overlay fields, fails to name all seven
/// blocker classes, fails to offer all three remediation actions, fails to stay reachable in all five reach
/// modes, or fails to certify every declared consumer surface; and a disclosed narrowing is backed by a
/// reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_command_explainers_packet(
    packet: &CommandExplainerPacket,
) -> Result<(), Vec<CommandExplainerValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(CommandExplainerValidationError::NoRows);
    }
    if packet.record_kind != M5_COMMAND_EXPLAINERS_PACKET_RECORD_KIND {
        errors.push(CommandExplainerValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_COMMAND_EXPLAINERS_SCHEMA_VERSION {
        errors.push(CommandExplainerValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(CommandExplainerValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(CommandExplainerValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_EXPLAINER_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_explainer_dimensions != expected_dimensions {
        errors.push(CommandExplainerValidationError::RequiredExplainerDimensionsStale);
    }
    let expected_leader_fields: Vec<String> = REQUIRED_LEADER_OVERLAY_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    if packet.required_leader_overlay_fields != expected_leader_fields {
        errors.push(CommandExplainerValidationError::RequiredLeaderOverlayFieldsStale);
    }
    let expected_blocker_classes: Vec<String> = REQUIRED_BLOCKER_CLASSES
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();
    if packet.required_blocker_classes != expected_blocker_classes {
        errors.push(CommandExplainerValidationError::RequiredBlockerClassesStale);
    }
    let expected_remediation_actions: Vec<String> = REQUIRED_REMEDIATION_ACTIONS
        .iter()
        .map(|action| action.as_str().to_owned())
        .collect();
    if packet.required_remediation_actions != expected_remediation_actions {
        errors.push(CommandExplainerValidationError::RequiredRemediationActionsStale);
    }
    let expected_reach_modes: Vec<String> = REQUIRED_REACH_MODES
        .iter()
        .map(|mode| mode.as_str().to_owned())
        .collect();
    if packet.required_reach_modes != expected_reach_modes {
        errors.push(CommandExplainerValidationError::RequiredReachModesStale);
    }
    let expected_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_surface_families != expected_families {
        errors.push(CommandExplainerValidationError::RequiredSurfaceFamiliesStale);
    }

    let present: BTreeSet<M5CommandSurfaceFamily> =
        packet.rows.iter().map(|row| row.surface_family).collect();
    let coverage_complete = REQUIRED_SURFACE_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_SURFACE_FAMILIES.len() {
        errors.push(CommandExplainerValidationError::CoverageIncomplete);
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
        errors.push(CommandExplainerValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandExplainerStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandExplainerStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandExplainerStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(CommandExplainerValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<CommandExplainerWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(CommandExplainerValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<CommandExplainerCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(CommandExplainerValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<CommandExplainerFinding> = Vec::new();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(CommandExplainerFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(CommandExplainerFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("explainer packet serializes"),
    ) {
        recomputed.push(CommandExplainerFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(CommandExplainerValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(CommandExplainerValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(CommandExplainerValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(CommandExplainerValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(CommandExplainerValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(CommandExplainerValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
