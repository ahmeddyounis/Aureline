//! Keybinding resolver inspectors, conflict-review sheets, and import-bridge outcome rows for every
//! claimed M5 command surface.
//!
//! The [frozen discoverability matrix][matrix] already binds each governed M5 last-mile command-discovery
//! surface — menu items, menu groups, context menus, command bars, keybinding resolver layers, conflict
//! review sheets, import-bridge rows, disabled-command explainers, leader/sequence help overlays, and
//! command-documentation surfaces — to one canonical command record, and freezes the shortcut-source,
//! conflict-reason, and import-translation vocabulary those surfaces project from. This lane is the
//! **resolver-inspection capstone** that certifies, for every one of those ten surface families, that
//! shortcut resolution is *inspectable*: a user, a doc, an automation, or a support reviewer can see which
//! binding wins, why it wins, what lost, how an imported shortcut translated, and what would change the
//! result — without relying on hidden resolver knowledge.
//!
//! For every surface family the lane certifies four things the acceptance criteria and implementation
//! requirements demand:
//!
//! - the resolver inspector reveals the **winning source layer, the shadowed losing candidates, and the
//!   fallback command path** for a chord (plus scope, current mode, and reserved/unavailable state) so a
//!   user can inspect winning and shadowed shortcuts from the keybinding UI, the command palette, and the
//!   command-documentation surfaces ([`ResolverInspectionState`], acceptance criterion 1);
//! - conflict-review sheets and import-bridge rows report **controlled bridge-outcome states** (exact,
//!   translated, alias-only, partial, shimmed, unsupported) with the open-docs / manual-fix migration
//!   actions where migration remains incomplete, rather than generic imported wording
//!   ([`BridgeOutcomeState`], acceptance criterion 2);
//! - leader / multi-stroke and policy/mode-overlay shortcuts carry the **same precedence model, timeout /
//!   cancel hints, and accessibility narration** as ordinary bindings so no sequence requires hidden
//!   knowledge to explain why it is or is not available ([`LeaderSequenceInspectionState`], acceptance
//!   criterion 3);
//! - and the resolver/export packet is **copy-safe and persistent** so support bundles, docs/help, and
//!   migration packets can reconstruct the same command id and winning-source explanation without a
//!   screenshot ([`ResolverExportState`], the persistence implementation requirement).
//!
//! Three records carry the truth:
//!
//! - the per-family **inspection row** ([`ResolverInspectorRow`]): one row per [`M5CommandSurfaceFamily`]
//!   naming the canonical command binding it projects from, the shortcut-source layers it resolves, the
//!   derived winning source and the shadowed losers, the conflict reasons and import-translation states it
//!   reports, the inspector fields it certifies, the controlled bridge outcomes and migration actions it
//!   renders, the consumer surfaces it evaluated, its resolver-inspection / bridge-outcome /
//!   leader-sequence / resolver-export posture, whether the same resolution survives headless/CLI
//!   execution, any active waiver, and a derived green/yellow/red [`ResolverInspectorStatus`].
//! - the inspection **packet** ([`ResolverInspectorPacket`]): the full set of rows with derived per-row
//!   status, aggregate green/yellow/red counts, the active waivers, the exact conformance causes
//!   ([`ResolverInspectorCause`]), and the blocking findings the lane refuses to ship with.
//! - the inspection **dashboard** ([`ResolverInspectorDashboard`]): a light projection the keybinding UI /
//!   command palette / Support Center / CLI / help / migration tooling reads to auto-narrow a surface's
//!   resolver-inspection claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment a
//! surface discloses a reduced inspector detail, a disclosed partial bridge coverage, a disclosed,
//! waivered reduced sequence hint, or a disclosed partial resolver/export capture; it drops to `red` if a
//! surface hides its winning or shadowed binding, uses generic imported wording instead of a controlled
//! bridge-outcome state, leaves a sequence's availability requiring hidden knowledge, cannot reconstruct
//! its winning source from durable evidence, loses the same resolution in a headless/CLI execution, fails
//! to certify all seven inspector fields, all six controlled bridge outcomes, all three migration actions,
//! or every declared consumer surface. That derivation is the auto-narrowing the acceptance criteria
//! require, and the inspector-field, bridge-outcome, migration-action, and consumer-surface completeness
//! checks are the conformance lints that gate a stable resolver-inspection claim.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local paths,
//! raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary, counts,
//! refs, and short labels. The surface-family, canonical-command-binding, shortcut-source-class,
//! conflict-reason, import-translation-state, required-label, stale-target-state, unavailable-reason,
//! parity-surface, consumer-surface, feature-family, downgrade-trigger, and qualification vocabulary is
//! re-exported by reference from the already frozen [matrix], and every family's canonical command
//! binding, qualification, owner, shortcut-source classes, conflict reasons, import-translation states,
//! required labels, feature families, declared consumer surfaces, and applicable downgrade triggers are
//! pulled straight from that matrix's seeded packet, so this lane mints no parallel command vocabulary and
//! cannot certify a surface the matrix does not anchor. Only the inspection-specific vocabulary
//! ([`M5ResolverInspectionDimension`], [`M5InspectorField`], [`M5BridgeOutcomeState`],
//! [`M5MigrationAction`], [`ResolverInspectorStatus`], [`ResolverInspectionState`],
//! [`BridgeOutcomeState`], [`LeaderSequenceInspectionState`], [`ResolverExportState`],
//! [`ResolverInspectorWaiver`], [`ResolverInspectorCause`], [`ResolverInspectorFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix as matrix;

pub use matrix::{
    M5CanonicalCommandBinding, M5CommandSurfaceFamily, M5ConflictReason, M5DisabledReasonMode,
    M5DiscoverabilityDowngradeTrigger, M5DiscoveryChannel, M5FeatureFamily,
    M5ImportTranslationState, M5LifecycleLabel, M5ParitySurface, M5PreviewClass, M5RequiredLabel,
    M5ShortcutSourceClass, M5StaleTargetState, M5SurfaceQualificationClass, M5UnavailableReason,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_keybinding_resolver_inspectors_packet,
    seeded_m5_keybinding_resolver_inspectors_packet_conflict_sheet_headless_parity_lost_blocked,
    seeded_m5_keybinding_resolver_inspectors_packet_documentation_export_absent_blocked,
    seeded_m5_keybinding_resolver_inspectors_packet_import_bridge_generic_wording_blocked,
    seeded_m5_keybinding_resolver_inspectors_packet_leader_hidden_knowledge_blocked,
    seeded_m5_keybinding_resolver_inspectors_packet_resolver_layer_hidden_binding_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_RESOLVER_INSPECTORS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_RESOLVER_INSPECTORS_SHARED_CONTRACT_REF: &str =
    "commands:m5_keybinding_resolver_inspectors:v1";

/// Stable record kind for [`ResolverInspectorPacket`] payloads.
pub const M5_RESOLVER_INSPECTORS_PACKET_RECORD_KIND: &str =
    "commands_m5_keybinding_resolver_inspectors_packet_record";

/// Stable record kind for [`ResolverInspectorDashboard`] payloads.
pub const M5_RESOLVER_INSPECTORS_DASHBOARD_RECORD_KIND: &str =
    "commands_m5_keybinding_resolver_inspectors_dashboard_record";

/// Stable record kind for [`ResolverInspectorSupportExport`] payloads.
pub const M5_RESOLVER_INSPECTORS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "commands_m5_keybinding_resolver_inspectors_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_RESOLVER_INSPECTORS_PACKET_ID: &str = "m5-keybinding-resolver-inspectors:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_RESOLVER_INSPECTORS_DASHBOARD_ID: &str =
    "m5-keybinding-resolver-inspectors-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_RESOLVER_INSPECTORS_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-keybinding-resolver-inspectors:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_RESOLVER_INSPECTORS_SOURCE_SCHEMA_REF: &str =
    "schemas/commands/m5-keybinding-resolver-inspectors.schema.json";

/// Published markdown report ref reviewers reopen the inspection proof from.
pub const M5_RESOLVER_INSPECTORS_PUBLISHED_REPORT_REF: &str =
    "artifacts/commands/m5-keybinding-resolver-inspectors.md";

/// Published inspection-packet artifact ref.
pub const M5_RESOLVER_INSPECTORS_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-keybinding-resolver-inspectors-proof/packet.json";

/// Published inspection-dashboard artifact ref.
pub const M5_RESOLVER_INSPECTORS_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-keybinding-resolver-inspectors-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_RESOLVER_INSPECTORS_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-keybinding-resolver-inspectors-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_RESOLVER_INSPECTORS_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-keybinding-resolver-inspectors-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_RESOLVER_INSPECTORS_PUBLISHED_DOC_REF: &str =
    "docs/commands/m5_keybinding_resolver_inspectors_contract.md";

/// Repo-relative ref to the frozen discoverability boundary schema.
pub const M5_RESOLVER_INSPECTORS_MATRIX_SCHEMA_REF: &str = matrix::M5_DISCOVERABILITY_SCHEMA_REF;

/// Frozen discoverability contract doc this proof mirrors.
pub const M5_RESOLVER_INSPECTORS_MATRIX_DOC_REF: &str = matrix::M5_DISCOVERABILITY_DOC_REF;

/// Canonical keybinding-resolver schema every inspector projects from.
pub const M5_RESOLVER_INSPECTORS_KEYBINDING_RESOLVER_REF: &str =
    matrix::M5_DISCOVERABILITY_KEYBINDING_RESOLVER_REF;

/// Every command-surface family the certification must cover, in canonical order. A certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_SURFACE_FAMILIES: [M5CommandSurfaceFamily; 10] = M5CommandSurfaceFamily::ALL;

/// Every inspection dimension each family row certifies, in canonical order.
pub const REQUIRED_INSPECTION_DIMENSIONS: [M5ResolverInspectionDimension; 4] =
    M5ResolverInspectionDimension::ALL;

/// Every inspector field each family row must reveal, in canonical order.
pub const REQUIRED_INSPECTOR_FIELDS: [M5InspectorField; 7] = M5InspectorField::ALL;

/// Every controlled bridge-outcome state each family row must render, in canonical order.
pub const REQUIRED_BRIDGE_OUTCOMES: [M5BridgeOutcomeState; 6] = M5BridgeOutcomeState::ALL;

/// Every migration action each family row must offer, in canonical order.
pub const REQUIRED_MIGRATION_ACTIONS: [M5MigrationAction; 3] = M5MigrationAction::ALL;

/// One of the four resolver-inspection dimensions each surface-family row certifies.
///
/// These are exactly the four ways the acceptance criteria and implementation requirements demand a
/// claimed M5 command surface make shortcut resolution inspectable: the inspector reveals winner /
/// shadowed / fallback truth; conflict and import rows use controlled bridge-outcome states; leader /
/// multi-stroke shortcuts carry the same precedence / timeout / narration model; and the resolver/export
/// packet reconstructs the winning source from durable evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResolverInspectionDimension {
    /// Winning source, shadowed losers, and fallback command path are inspectable.
    ResolverInspection,
    /// Conflict / import rows report controlled bridge-outcome states and migration actions.
    BridgeOutcome,
    /// Leader / multi-stroke shortcuts carry the same precedence / timeout / narration model.
    LeaderSequenceInspection,
    /// The resolver/export packet reconstructs the winning source from durable evidence.
    ResolverExport,
}

impl M5ResolverInspectionDimension {
    /// Every inspection dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ResolverInspection,
        Self::BridgeOutcome,
        Self::LeaderSequenceInspection,
        Self::ResolverExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolverInspection => "resolver_inspection",
            Self::BridgeOutcome => "bridge_outcome",
            Self::LeaderSequenceInspection => "leader_sequence_inspection",
            Self::ResolverExport => "resolver_export",
        }
    }
}

/// One of the seven fields a keybinding resolver inspector must reveal for a claimed M5 action.
///
/// These are the exact fields the implementation requirements name: the source layer, scope, current
/// mode, active winner, losing candidates, reserved/unavailable state, and fallback command path. A row
/// that reveals fewer leaves the user relying on hidden resolver knowledge and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InspectorField {
    /// The precedence source layer a binding comes from.
    SourceLayer,
    /// The when-context scope the binding is active in.
    Scope,
    /// The current keymap mode / profile the resolution was computed under.
    CurrentMode,
    /// The active winning binding for the chord.
    ActiveWinner,
    /// The shadowed losing candidates for the chord.
    LosingCandidates,
    /// The reserved / unavailable state (platform-reserved or otherwise unbindable).
    ReservedUnavailableState,
    /// The fallback command path when the winning binding is unavailable.
    FallbackCommandPath,
}

impl M5InspectorField {
    /// Every inspector field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SourceLayer,
        Self::Scope,
        Self::CurrentMode,
        Self::ActiveWinner,
        Self::LosingCandidates,
        Self::ReservedUnavailableState,
        Self::FallbackCommandPath,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLayer => "source_layer",
            Self::Scope => "scope",
            Self::CurrentMode => "current_mode",
            Self::ActiveWinner => "active_winner",
            Self::LosingCandidates => "losing_candidates",
            Self::ReservedUnavailableState => "reserved_unavailable_state",
            Self::FallbackCommandPath => "fallback_command_path",
        }
    }
}

/// One of the six controlled bridge-outcome states a conflict-review sheet or import-bridge row reports
/// for a foreign-keymap binding.
///
/// These are the controlled translation states the implementation requirements name — `exact`,
/// `translated`, `alias_only`, `partial`, `shimmed`, and `unsupported` — so an imported keymap outcome
/// reads with one closed vocabulary rather than generic "imported" wording. The last three
/// ([`Self::Partial`], [`Self::Shimmed`], [`Self::Unsupported`]) mean migration remains incomplete and an
/// open-docs / manual-fix action must be offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BridgeOutcomeState {
    /// The foreign binding maps exactly to a native command and chord.
    Exact,
    /// The foreign binding translated to a native equivalent.
    Translated,
    /// Only an alias of the command could be bound, not the primary chord.
    AliasOnly,
    /// The binding translated only partially; part of the mapping remains open.
    Partial,
    /// The binding is served by a compatibility shim rather than a native binding.
    Shimmed,
    /// The foreign binding has no native equivalent and is unsupported.
    Unsupported,
}

impl M5BridgeOutcomeState {
    /// Every bridge-outcome state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Exact,
        Self::Translated,
        Self::AliasOnly,
        Self::Partial,
        Self::Shimmed,
        Self::Unsupported,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Translated => "translated",
            Self::AliasOnly => "alias_only",
            Self::Partial => "partial",
            Self::Shimmed => "shimmed",
            Self::Unsupported => "unsupported",
        }
    }

    /// `true` when this outcome means migration remains incomplete and a manual-fix / open-docs action is
    /// required.
    pub const fn migration_incomplete(self) -> bool {
        matches!(self, Self::Partial | Self::Shimmed | Self::Unsupported)
    }
}

/// One of the three migration actions an import-bridge row offers where migration remains incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrationAction {
    /// Open the migration documentation for the affected binding.
    OpenDocs,
    /// Apply a manual fix (rebind or accept an alternative).
    ManualFix,
    /// No action is needed; the outcome is exact or translated.
    NoActionNeeded,
}

impl M5MigrationAction {
    /// Every migration action, in declaration order.
    pub const ALL: [Self; 3] = [Self::OpenDocs, Self::ManualFix, Self::NoActionNeeded];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDocs => "open_docs",
            Self::ManualFix => "manual_fix",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// The derived resolver-inspection light a command surface carries.
///
/// `green` means the inspector reveals the winning source, shadowed losers, and fallback path across all
/// seven inspector fields, reports controlled bridge-outcome states with migration actions, keeps leader /
/// multi-stroke shortcuts on the same precedence / timeout / narration model, and reconstructs its winning
/// source from durable evidence — across every declared consumer surface, with the same resolution
/// surviving headless/CLI execution. `yellow` is a disclosed narrowing. `red` is blocked and may not keep
/// a resolver-inspection claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverInspectorStatus {
    /// Full standing: all four inspection dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl ResolverInspectorStatus {
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

/// How the resolver inspector reveals the winning source, the shadowed losers, and the fallback path.
///
/// `winner_shadowed_source_and_fallback_certified` means the inspector names the winning source layer, the
/// shadowed losing candidates, the scope, the current mode, the reserved/unavailable state, and the
/// fallback command path. `disclosed_reduced_inspector_detail` means the inspector folds the full
/// losing-candidate list into an expandable "N shadowed" summary on a constrained surface while still
/// naming the winner, its source layer, and the fallback path (a yellow narrowing).
/// `winning_or_shadowed_binding_hidden` means the inspector hid the winner or the shadowed losers so the
/// resolution cannot be inspected — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverInspectionState {
    /// Winner, shadowed losers, and fallback path are certified.
    WinnerShadowedSourceAndFallbackCertified,
    /// The inspector takes a disclosed reduced inspector detail on a constrained surface.
    DisclosedReducedInspectorDetail,
    /// The inspector hid the winning or shadowed binding — a blocker.
    WinningOrShadowedBindingHidden,
}

impl ResolverInspectionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WinnerShadowedSourceAndFallbackCertified => {
                "winner_shadowed_source_and_fallback_certified"
            }
            Self::DisclosedReducedInspectorDetail => "disclosed_reduced_inspector_detail",
            Self::WinningOrShadowedBindingHidden => "winning_or_shadowed_binding_hidden",
        }
    }

    /// `true` when resolver inspection is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::WinnerShadowedSourceAndFallbackCertified)
    }

    /// `true` when the surface took a disclosed reduced-inspector-detail narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedInspectorDetail)
    }
}

/// How a conflict-review sheet / import-bridge row reports its bridge outcomes.
///
/// `controlled_states_and_migration_actions_certified` means every imported outcome uses one of the six
/// controlled bridge-outcome states and offers an open-docs / manual-fix action where migration remains
/// incomplete. `disclosed_partial_bridge_coverage` means one slice of imported bindings still awaits
/// manual review, the gap is disclosed with a controlled `partial` / `shimmed` state, and a migration
/// action is offered (a yellow narrowing). `generic_imported_wording_used` means the row fell back to
/// generic "imported" wording instead of a controlled state — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeOutcomeState {
    /// Controlled bridge-outcome states and migration actions are certified.
    ControlledStatesAndMigrationActionsCertified,
    /// One slice of imported bindings takes a disclosed partial bridge coverage.
    DisclosedPartialBridgeCoverage,
    /// The row fell back to generic imported wording — a blocker.
    GenericImportedWordingUsed,
}

impl BridgeOutcomeState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledStatesAndMigrationActionsCertified => {
                "controlled_states_and_migration_actions_certified"
            }
            Self::DisclosedPartialBridgeCoverage => "disclosed_partial_bridge_coverage",
            Self::GenericImportedWordingUsed => "generic_imported_wording_used",
        }
    }

    /// `true` when the bridge outcome is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ControlledStatesAndMigrationActionsCertified)
    }

    /// `true` when the surface took a disclosed partial-bridge-coverage narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialBridgeCoverage)
    }
}

/// How leader / multi-stroke and policy/mode-overlay shortcuts are made inspectable.
///
/// `precedence_timeout_cancel_narration_certified` means a leader / multi-stroke shortcut carries the same
/// precedence model, the timeout / cancel hints, and the accessibility narration used for ordinary
/// bindings. `disclosed_reduced_sequence_hint` means a half-typed sequence continuation renders a reduced,
/// disclosed hint under a waivered exception while its resolution still names the winning source and
/// fallback (a yellow narrowing that **requires an active waiver**).
/// `sequence_availability_requires_hidden_knowledge` means a sequence's availability could only be
/// explained with hidden resolver knowledge — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaderSequenceInspectionState {
    /// Precedence, timeout / cancel hints, and narration are certified.
    PrecedenceTimeoutCancelNarrationCertified,
    /// A sequence continuation renders a disclosed, waivered reduced sequence hint.
    DisclosedReducedSequenceHint,
    /// A sequence's availability requires hidden knowledge to explain — a blocker.
    SequenceAvailabilityRequiresHiddenKnowledge,
}

impl LeaderSequenceInspectionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrecedenceTimeoutCancelNarrationCertified => {
                "precedence_timeout_cancel_narration_certified"
            }
            Self::DisclosedReducedSequenceHint => "disclosed_reduced_sequence_hint",
            Self::SequenceAvailabilityRequiresHiddenKnowledge => {
                "sequence_availability_requires_hidden_knowledge"
            }
        }
    }

    /// `true` when leader-sequence inspection is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::PrecedenceTimeoutCancelNarrationCertified)
    }

    /// `true` when the surface took a disclosed reduced-sequence-hint narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedSequenceHint)
    }
}

/// How the resolver / export packet reconstructs the command id and winning source.
///
/// `command_id_and_winning_source_reconstructable` means a support bundle, doc, or migration packet can
/// reconstruct the command id and the winning-source explanation from a durable, copy-safe export without
/// a screenshot. `disclosed_partial_capture` means one legacy export captures the command id and winning
/// source but not the full shadowed list, while still disclosing the gap (a yellow narrowing).
/// `winning_source_absent_from_capture` means the winning source (or command id) is absent from durable
/// evidence — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverExportState {
    /// Command id and winning source are reconstructable from durable evidence.
    CommandIdAndWinningSourceReconstructable,
    /// One legacy export takes a disclosed partial capture.
    DisclosedPartialCapture,
    /// The winning source or command id is absent from durable evidence — a blocker.
    WinningSourceAbsentFromCapture,
}

impl ResolverExportState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandIdAndWinningSourceReconstructable => {
                "command_id_and_winning_source_reconstructable"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::WinningSourceAbsentFromCapture => "winning_source_absent_from_capture",
        }
    }

    /// `true` when resolver export is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CommandIdAndWinningSourceReconstructable)
    }

    /// `true` when the surface took a disclosed partial-capture narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather than
/// blocked — never lets a hidden winner, generic imported wording, a hidden-knowledge sequence, or an
/// uncapturable winning source hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolverInspectorWaiver {
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

impl ResolverInspectorWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a surface family's resolver inspection.
///
/// The trigger token mirrors the frozen [`M5DiscoverabilityDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolverInspectorCause {
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

impl ResolverInspectorCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One surface family, certified across its resolver-inspection, bridge-outcome, leader-sequence, and
/// resolver-export dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolverInspectorRow {
    /// The surface family being certified.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short reviewer-facing family label.
    pub surface_label: String,
    /// Qualification class the matrix earned for the surface. Pulled from the matrix.
    pub qualification: M5SurfaceQualificationClass,
    /// Owner role accountable for keeping this surface's inspection governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short conformance scope summary.
    pub scope_summary: String,
    /// The canonical command-record binding this surface projects from. Pulled from the matrix.
    pub canonical_command_binding: M5CanonicalCommandBinding,
    /// Mandatory labels this surface must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5RequiredLabel>,
    /// The shortcut-source layers this surface resolves / explains. Pulled from the matrix.
    pub shortcut_source_classes: Vec<M5ShortcutSourceClass>,
    /// The derived winning source layer for the surface's chords. Recomputed by the builder; the highest
    /// precedence source among [`Self::shortcut_source_classes`], or `None` when the surface resolves no
    /// shortcuts.
    pub winning_source_class: Option<M5ShortcutSourceClass>,
    /// The shadowed losing source layers. Recomputed by the builder.
    pub shadowed_source_classes: Vec<M5ShortcutSourceClass>,
    /// The controlled conflict reasons this surface reports. Pulled from the matrix.
    pub conflict_reasons: Vec<M5ConflictReason>,
    /// The frozen import-translation states this surface reports. Pulled from the matrix.
    pub import_translation_states: Vec<M5ImportTranslationState>,
    /// The stale-target invalidation states this surface honours. Pulled from the matrix.
    pub stale_target_states: Vec<M5StaleTargetState>,
    /// The why-unavailable explanation classes this surface reports. Pulled from the matrix.
    pub unavailable_reasons: Vec<M5UnavailableReason>,
    /// M5 feature families whose commands this surface exposes. Pulled from the matrix.
    pub feature_families: Vec<M5FeatureFamily>,
    /// The inspector fields this row certifies (must be all seven).
    pub certified_inspector_fields: Vec<M5InspectorField>,
    /// The controlled bridge-outcome states this row renders (must be all six).
    pub certified_bridge_outcomes: Vec<M5BridgeOutcomeState>,
    /// The migration actions this row offers (must be all three).
    pub certified_migration_actions: Vec<M5MigrationAction>,
    /// Consumer surfaces the matrix declares the surface must project to.
    pub required_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Resolver-inspection posture.
    pub resolver_inspection: ResolverInspectionState,
    /// Bridge-outcome posture.
    pub bridge_outcome: BridgeOutcomeState,
    /// Leader-sequence-inspection posture.
    pub leader_sequence_inspection: LeaderSequenceInspectionState,
    /// Resolver-export posture.
    pub resolver_export: ResolverExportState,
    /// `true` when the same resolution survives a headless / CLI execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the surface. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced sequence hint is in force.
    pub active_waiver: Option<ResolverInspectorWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: ResolverInspectorStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<ResolverInspectorCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl ResolverInspectorRow {
    /// Recomputes the winning source layer from the declared shortcut-source classes: the highest
    /// precedence source, or `None` when the surface resolves no shortcuts.
    pub fn recompute_winner(&self) -> Option<M5ShortcutSourceClass> {
        self.shortcut_source_classes
            .iter()
            .copied()
            .max_by_key(|source| source.precedence_rank())
    }

    /// Recomputes the shadowed (losing) source layers — every declared source that is not the winner.
    pub fn recompute_shadowed(&self) -> Vec<M5ShortcutSourceClass> {
        let winner = self.recompute_winner();
        self.shortcut_source_classes
            .iter()
            .copied()
            .filter(|source| Some(*source) != winner)
            .collect()
    }

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

    /// `true` when the row reveals every one of the seven inspector fields — the structural proof that a
    /// user never needs hidden resolver knowledge.
    pub fn inspector_fields_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_inspector_fields
            .iter()
            .map(|field| field.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_INSPECTOR_FIELDS
            .iter()
            .map(|field| field.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when the row renders every one of the six controlled bridge-outcome states — the structural
    /// proof that no imported outcome falls back to generic wording.
    pub fn bridge_outcomes_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_bridge_outcomes
            .iter()
            .map(|outcome| outcome.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_BRIDGE_OUTCOMES
            .iter()
            .map(|outcome| outcome.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when the row offers every one of the three migration actions — the structural proof that an
    /// incomplete migration always offers open-docs / manual-fix.
    pub fn migration_actions_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_migration_actions
            .iter()
            .map(|action| action.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_MIGRATION_ACTIONS
            .iter()
            .map(|action| action.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the declared winner / shadowed set matches the recomputed resolution.
    fn resolution_is_current(&self) -> bool {
        self.winning_source_class == self.recompute_winner()
            && self.shadowed_source_classes == self.recompute_shadowed()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.inspector_fields_complete() {
            return true;
        }
        if !self.bridge_outcomes_complete() {
            return true;
        }
        if !self.migration_actions_complete() {
            return true;
        }
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.resolver_inspection,
            ResolverInspectionState::WinningOrShadowedBindingHidden
        ) {
            return true;
        }
        if matches!(
            self.bridge_outcome,
            BridgeOutcomeState::GenericImportedWordingUsed
        ) {
            return true;
        }
        if matches!(
            self.leader_sequence_inspection,
            LeaderSequenceInspectionState::SequenceAvailabilityRequiresHiddenKnowledge
        ) {
            return true;
        }
        if matches!(
            self.resolver_export,
            ResolverExportState::WinningSourceAbsentFromCapture
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.resolver_inspection.is_disclosed_narrowing()
            || self.bridge_outcome.is_disclosed_narrowing()
            || self.leader_sequence_inspection.is_disclosed_narrowing()
            || self.resolver_export.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the inspection posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> ResolverInspectorStatus {
        if self.has_hard_blocker() {
            ResolverInspectorStatus::Red
        } else if self.has_narrowing() {
            ResolverInspectorStatus::Yellow
        } else {
            ResolverInspectorStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (resolver inspection,
    /// bridge outcome, leader sequence, resolver export, then structural completeness and headless
    /// parity).
    pub fn recompute_causes(&self) -> Vec<ResolverInspectorCause> {
        let mut causes = Vec::new();
        match self.resolver_inspection {
            ResolverInspectionState::WinnerShadowedSourceAndFallbackCertified => {}
            ResolverInspectionState::DisclosedReducedInspectorDetail => {
                causes.push(ResolverInspectorCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                    disclosed: true,
                    detail: "On a constrained surface the resolver inspector takes a disclosed reduced \
                             inspector detail — the full losing-candidate list is folded into an \
                             expandable \"N shadowed\" summary while the winning source layer, the \
                             fallback command path, and the reserved/unavailable state stay visible — so \
                             the shadowed truth is narrowed and disclosed rather than hidden."
                        .to_owned(),
                });
            }
            ResolverInspectionState::WinningOrShadowedBindingHidden => {
                causes.push(ResolverInspectorCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ConflictWinnerAmbiguous,
                    disclosed: false,
                    detail: "The resolver inspector hid the winning binding or its shadowed losers, so a \
                             user cannot see which binding wins, why it wins, or what lost without hidden \
                             resolver knowledge."
                        .to_owned(),
                });
            }
        }
        match self.bridge_outcome {
            BridgeOutcomeState::ControlledStatesAndMigrationActionsCertified => {}
            BridgeOutcomeState::DisclosedPartialBridgeCoverage => {
                causes.push(ResolverInspectorCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ImportTranslationUntruthful,
                    disclosed: true,
                    detail: "One slice of imported bindings takes a disclosed partial bridge coverage — \
                             it is reported with a controlled `partial` / `shimmed` state and an \
                             open-docs / manual-fix action while manual review completes — so the import \
                             outcome is narrowed and disclosed rather than generic imported wording."
                        .to_owned(),
                });
            }
            BridgeOutcomeState::GenericImportedWordingUsed => {
                causes.push(ResolverInspectorCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ImportTranslationUntruthful,
                    disclosed: false,
                    detail: "An imported binding was reported with generic \"imported\" wording instead \
                             of one of the controlled bridge-outcome states, so a user cannot tell how \
                             the shortcut translated or what migration is still required."
                        .to_owned(),
                });
            }
        }
        match self.leader_sequence_inspection {
            LeaderSequenceInspectionState::PrecedenceTimeoutCancelNarrationCertified => {}
            LeaderSequenceInspectionState::DisclosedReducedSequenceHint => {
                causes.push(ResolverInspectorCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                    disclosed: true,
                    detail: "A half-typed leader / multi-key sequence continuation renders a disclosed, \
                             waivered reduced sequence hint — the armed-sequence overlay folds the \
                             next-key list into a compact hint while the precedence, timeout / cancel \
                             hints, and narration stay available — so the sequence availability is \
                             narrowed and disclosed rather than requiring hidden knowledge."
                        .to_owned(),
                });
            }
            LeaderSequenceInspectionState::SequenceAvailabilityRequiresHiddenKnowledge => {
                causes.push(ResolverInspectorCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ConflictWinnerAmbiguous,
                    disclosed: false,
                    detail: "A leader / multi-stroke or policy/mode-overlay shortcut's availability could \
                             only be explained with hidden resolver knowledge — no precedence, timeout, \
                             or narration hint told the user why the sequence was or was not available."
                        .to_owned(),
                });
            }
        }
        match self.resolver_export {
            ResolverExportState::CommandIdAndWinningSourceReconstructable => {}
            ResolverExportState::DisclosedPartialCapture => {
                causes.push(ResolverInspectorCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One legacy resolver/export surface takes a disclosed partial capture — the \
                             export captures the command id and the winning source but not the full \
                             shadowed list, while still disclosing the gap — so the resolver/export \
                             parity is narrowed and disclosed rather than absent."
                        .to_owned(),
                });
            }
            ResolverExportState::WinningSourceAbsentFromCapture => {
                causes.push(ResolverInspectorCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail: "The command id or the winning-source explanation is absent from the durable \
                             resolver/export packet, so a support bundle, doc, or migration packet cannot \
                             reconstruct which binding won without a screenshot."
                        .to_owned(),
                });
            }
        }
        if !self.inspector_fields_complete() {
            causes.push(ResolverInspectorCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                disclosed: false,
                detail: "The inspector does not reveal all seven inspector fields — source layer, scope, \
                         current mode, active winner, losing candidates, reserved/unavailable state, and \
                         fallback command path — so a claimed resolution cannot be fully inspected."
                    .to_owned(),
            });
        }
        if !self.bridge_outcomes_complete() {
            causes.push(ResolverInspectorCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ImportTranslationUntruthful,
                disclosed: false,
                detail: "The surface does not render all six controlled bridge-outcome states — exact, \
                         translated, alias-only, partial, shimmed, and unsupported — so an imported \
                         outcome could fall back to generic wording."
                    .to_owned(),
            });
        }
        if !self.migration_actions_complete() {
            causes.push(ResolverInspectorCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ImportTranslationUntruthful,
                disclosed: false,
                detail: "The surface does not offer all three migration actions — open-docs, manual-fix, \
                         and no-action-needed — so an incomplete migration could leave the user with no \
                         next step."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(ResolverInspectorCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "A headless / CLI execution of this surface lost the shared resolution, so the \
                         same chord reports a different winner, shadowed set, or bridge outcome depending \
                         on how it is reached."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced sequence hint may only stay yellow (rather than red) when a waiver discloses
    /// it — reducing a leader-sequence hint is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.leader_sequence_inspection,
            LeaderSequenceInspectionState::DisclosedReducedSequenceHint
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<ResolverInspectorFinding> {
        let mut findings = Vec::new();
        let family = self.surface_family.as_str().to_owned();

        if !self.inspector_fields_complete() {
            findings.push(ResolverInspectorFinding::InspectorFieldsIncomplete {
                family: family.clone(),
            });
        }
        if !self.bridge_outcomes_complete() {
            findings.push(ResolverInspectorFinding::BridgeOutcomesIncomplete {
                family: family.clone(),
            });
        }
        if !self.migration_actions_complete() {
            findings.push(ResolverInspectorFinding::MigrationActionsIncomplete {
                family: family.clone(),
            });
        }
        if !self.consumer_surfaces_complete() {
            findings.push(ResolverInspectorFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(ResolverInspectorFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.resolver_inspection,
            ResolverInspectionState::WinningOrShadowedBindingHidden
        ) {
            findings.push(ResolverInspectorFinding::ResolverInspectionBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.bridge_outcome,
            BridgeOutcomeState::GenericImportedWordingUsed
        ) {
            findings.push(ResolverInspectorFinding::BridgeOutcomeBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.leader_sequence_inspection,
            LeaderSequenceInspectionState::SequenceAvailabilityRequiresHiddenKnowledge
        ) {
            findings.push(ResolverInspectorFinding::LeaderSequenceInspectionBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.resolver_export,
            ResolverExportState::WinningSourceAbsentFromCapture
        ) {
            findings.push(ResolverInspectorFinding::ResolverExportBroken {
                family: family.clone(),
            });
        }
        if !self.resolution_is_current() {
            findings.push(ResolverInspectorFinding::ResolutionStale {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, ResolverInspectorStatus::Green) && !self.has_reason() {
            findings.push(ResolverInspectorFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(ResolverInspectorFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.surface_family != self.surface_family {
                findings.push(ResolverInspectorFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(ResolverInspectorFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(ResolverInspectorFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(ResolverInspectorFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} inspection={} bridge={} leader={} export={} headless={} winner={} fields={} outcomes={} surfaces={} waiver={}",
            self.surface_family.as_str(),
            self.derived_status.as_str(),
            self.resolver_inspection.as_str(),
            self.bridge_outcome.as_str(),
            self.leader_sequence_inspection.as_str(),
            self.resolver_export.as_str(),
            self.headless_parity_preserved,
            self.winning_source_class
                .map(|source| source.as_str())
                .unwrap_or("none"),
            self.certified_inspector_fields.len(),
            self.certified_bridge_outcomes.len(),
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the resolver inspection refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ResolverInspectorFinding {
    /// A surface family has no inspection row.
    SurfaceFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row did not reveal every inspector field.
    InspectorFieldsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not render every controlled bridge-outcome state.
    BridgeOutcomesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not offer every migration action.
    MigrationActionsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless / CLI execution lost the shared resolution.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// The inspector hid the winning binding or its shadowed losers.
    ResolverInspectionBroken {
        /// The family token.
        family: String,
    },
    /// The surface used generic imported wording instead of a controlled bridge-outcome state.
    BridgeOutcomeBroken {
        /// The family token.
        family: String,
    },
    /// A sequence's availability could only be explained with hidden knowledge.
    LeaderSequenceInspectionBroken {
        /// The family token.
        family: String,
    },
    /// The winning source is absent from the durable resolver/export packet.
    ResolverExportBroken {
        /// The family token.
        family: String,
    },
    /// The declared winner / shadowed set does not match the recomputed resolution.
    ResolutionStale {
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

impl ResolverInspectorFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::SurfaceFamilyMissing { .. } => "surface_family_missing",
            Self::InspectorFieldsIncomplete { .. } => "inspector_fields_incomplete",
            Self::BridgeOutcomesIncomplete { .. } => "bridge_outcomes_incomplete",
            Self::MigrationActionsIncomplete { .. } => "migration_actions_incomplete",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::ResolverInspectionBroken { .. } => "resolver_inspection_broken",
            Self::BridgeOutcomeBroken { .. } => "bridge_outcome_broken",
            Self::LeaderSequenceInspectionBroken { .. } => "leader_sequence_inspection_broken",
            Self::ResolverExportBroken { .. } => "resolver_export_broken",
            Self::ResolutionStale { .. } => "resolution_stale",
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
            | Self::InspectorFieldsIncomplete { family }
            | Self::BridgeOutcomesIncomplete { family }
            | Self::MigrationActionsIncomplete { family }
            | Self::ConsumerSurfacesIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::ResolverInspectionBroken { family }
            | Self::BridgeOutcomeBroken { family }
            | Self::LeaderSequenceInspectionBroken { family }
            | Self::ResolverExportBroken { family }
            | Self::ResolutionStale { family }
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

/// The resolver inspection packet shared by the keybinding UI / command palette / Support Center / CLI /
/// help / migration tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolverInspectorPacket {
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
    /// Canonical keybinding-resolver schema every inspector projects from.
    pub keybinding_resolver_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four inspection dimensions every family row certifies.
    pub required_inspection_dimensions: Vec<String>,
    /// The seven inspector fields every family row must reveal.
    pub required_inspector_fields: Vec<String>,
    /// The six controlled bridge-outcome states every family row must render.
    pub required_bridge_outcomes: Vec<String>,
    /// The three migration actions every family row must offer.
    pub required_migration_actions: Vec<String>,
    /// The ten surface families the certification must cover.
    pub required_surface_families: Vec<String>,
    /// Per-family inspection rows, in canonical order.
    pub rows: Vec<ResolverInspectorRow>,
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
    pub active_waivers: Vec<ResolverInspectorWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<ResolverInspectorCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<ResolverInspectorFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Command / keybinding automation refs that consume this packet to auto-narrow a surface.
    pub command_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help / migration refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published inspection-packet ref.
    pub published_packet_ref: String,
    /// Published inspection-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ResolverInspectorPacket {
    /// Returns the inspection row for `family`, if present.
    pub fn row(&self, family: M5CommandSurfaceFamily) -> Option<&ResolverInspectorRow> {
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

    /// Projects the light inspection dashboard the command automation consumes.
    pub fn dashboard(&self) -> ResolverInspectorDashboard {
        ResolverInspectorDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 resolver-inspectors packet serializes")
    }

    /// Deterministic, machine-readable inspection CSV: one row per surface family naming its status, the
    /// four inspection postures, headless parity, the winning source, the inspector-field / bridge-outcome
    /// counts, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,status,resolver_inspection,bridge_outcome,leader_sequence_inspection,resolver_export,headless_parity,winning_source,inspector_fields,bridge_outcomes,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.derived_status.as_str(),
                row.resolver_inspection.as_str(),
                row.bridge_outcome.as_str(),
                row.leader_sequence_inspection.as_str(),
                row.resolver_export.as_str(),
                row.headless_parity_preserved,
                row.winning_source_class
                    .map(|source| source.as_str())
                    .unwrap_or("none"),
                row.certified_inspector_fields.len(),
                row.certified_bridge_outcomes.len(),
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
            "# M5 keybinding resolver inspectors: winning/shadowed shortcut inspection, controlled bridge outcomes, and copy-safe resolver export across every claimed M5 command surface\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_keybinding_resolver_inspectors`](../../crates/aureline-shell/src/m5_keybinding_resolver_inspectors/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- markdown > \\\n  artifacts/commands/m5-keybinding-resolver-inspectors.md\n",
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
            "- Required inspection dimensions: {}\n",
            self.required_inspection_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Inspector fields revealed: {}\n",
            self.required_inspector_fields
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Controlled bridge outcomes: {}\n",
            self.required_bridge_outcomes
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Migration actions: {}\n",
            self.required_migration_actions
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

        out.push_str("## Inspection rows\n\n");
        out.push_str(
            "| Surface family | Status | Resolver inspection | Bridge outcome | Leader sequence | Resolver export | Winner | Headless | Waiver |\n\
             | -------------- | ------ | ------------------- | -------------- | --------------- | --------------- | ------ | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.resolver_inspection.as_str(),
                row.bridge_outcome.as_str(),
                row.leader_sequence_inspection.as_str(),
                row.resolver_export.as_str(),
                row.winning_source_class
                    .map(|source| source.as_str())
                    .unwrap_or("none"),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&ResolverInspectorRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, ResolverInspectorStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed M5 command surface reveals its winning source, shadowed losers, and fallback path, reports controlled bridge outcomes with migration actions, keeps leader/multi-stroke shortcuts on the same precedence/timeout/narration model, and reconstructs its winning source from durable evidence across every declared consumer surface.\n\n",
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_keybinding_resolver_inspectors_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light inspection dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolverInspectorDashboardRow {
    /// The surface family.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short family label.
    pub surface_label: String,
    /// Qualification class earned by the surface.
    pub qualification: M5SurfaceQualificationClass,
    /// Derived green/yellow/red status.
    pub status: ResolverInspectorStatus,
    /// The derived winning source layer, when the surface resolves shortcuts.
    pub winning_source_class: Option<M5ShortcutSourceClass>,
    /// Number of shadowed losing source layers.
    pub shadowed_source_count: usize,
    /// Number of inspector fields revealed.
    pub certified_inspector_field_count: usize,
    /// Number of controlled bridge-outcome states rendered.
    pub certified_bridge_outcome_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Resolver-inspection posture.
    pub resolver_inspection: ResolverInspectionState,
    /// Bridge-outcome posture.
    pub bridge_outcome: BridgeOutcomeState,
    /// Leader-sequence-inspection posture.
    pub leader_sequence_inspection: LeaderSequenceInspectionState,
    /// Resolver-export posture.
    pub resolver_export: ResolverExportState,
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

/// The light inspection dashboard the keybinding UI / command palette / Support Center / CLI / help /
/// migration tooling reads to auto-narrow a surface's resolver-inspection claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolverInspectorDashboard {
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
    pub rows: Vec<ResolverInspectorDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Command / keybinding automation refs that consume the dashboard.
    pub command_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ResolverInspectorDashboard {
    /// Projects the dashboard from an inspection packet.
    pub fn from_packet(packet: &ResolverInspectorPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| ResolverInspectorDashboardRow {
                surface_family: row.surface_family,
                surface_label: row.surface_label.clone(),
                qualification: row.qualification,
                status: row.derived_status,
                winning_source_class: row.winning_source_class,
                shadowed_source_count: row.shadowed_source_classes.len(),
                certified_inspector_field_count: row.certified_inspector_fields.len(),
                certified_bridge_outcome_count: row.certified_bridge_outcomes.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                resolver_inspection: row.resolver_inspection,
                bridge_outcome: row.bridge_outcome,
                leader_sequence_inspection: row.leader_sequence_inspection,
                resolver_export: row.resolver_export,
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
            record_kind: M5_RESOLVER_INSPECTORS_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_RESOLVER_INSPECTORS_SCHEMA_VERSION,
            dashboard_id: M5_RESOLVER_INSPECTORS_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 resolver-inspectors dashboard serializes")
    }
}

/// Support-export wrapper for the resolver inspection packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolverInspectorSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: ResolverInspectorPacket,
    /// Dashboard quoted in full.
    pub dashboard: ResolverInspectorDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ResolverInspectorSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each surface family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the migration tooling — can name the
    /// same surface and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: ResolverInspectorPacket,
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
            record_kind: M5_RESOLVER_INSPECTORS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_RESOLVER_INSPECTORS_SCHEMA_VERSION,
            shared_contract_ref: M5_RESOLVER_INSPECTORS_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_keybinding_resolver_inspectors_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolverInspectorInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen discoverability matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family inspection rows.
    pub rows: Vec<ResolverInspectorRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The inspection packet carries only closed vocabulary, refs, and short labels, so raw URLs,
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

/// Builds a [`ResolverInspectorPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-family inspection rows.
///
/// Each row's derived winner/shadowed resolution, derived status, and conformance causes, the aggregate
/// counts, the active waivers, and the blocking findings are recomputed here so the packet is the single
/// source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_keybinding_resolver_inspectors_packet(
    input: ResolverInspectorInput,
) -> ResolverInspectorPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived resolution, status, and causes so the packet is self-consistent and
    // the auto-narrowing is the single source of truth.
    let rows: Vec<ResolverInspectorRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.winning_source_class = row.recompute_winner();
            row.shadowed_source_classes = row.recompute_shadowed();
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<ResolverInspectorFinding> = Vec::new();

    // Every surface family must carry an inspection row.
    let present: BTreeSet<M5CommandSurfaceFamily> =
        rows.iter().map(|row| row.surface_family).collect();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(ResolverInspectorFinding::SurfaceFamilyMissing {
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
        .filter(|row| matches!(row.derived_status, ResolverInspectorStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ResolverInspectorStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ResolverInspectorStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(ResolverInspectorFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<ResolverInspectorWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<ResolverInspectorCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_inspection_dimensions: Vec<String> = REQUIRED_INSPECTION_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_inspector_fields: Vec<String> = REQUIRED_INSPECTOR_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    let required_bridge_outcomes: Vec<String> = REQUIRED_BRIDGE_OUTCOMES
        .iter()
        .map(|outcome| outcome.as_str().to_owned())
        .collect();
    let required_migration_actions: Vec<String> = REQUIRED_MIGRATION_ACTIONS
        .iter()
        .map(|action| action.as_str().to_owned())
        .collect();
    let required_surface_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = ResolverInspectorPacket {
        record_kind: M5_RESOLVER_INSPECTORS_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_RESOLVER_INSPECTORS_SCHEMA_VERSION,
        shared_contract_ref: M5_RESOLVER_INSPECTORS_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_RESOLVER_INSPECTORS_PACKET_ID.to_owned(),
        source_schema_ref: M5_RESOLVER_INSPECTORS_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Resolver inspection for every claimed M5 command surface: each of the ten governed \
                   surface families certified so shortcut resolution is inspectable — a user, doc, \
                   automation, or support reviewer can see which binding wins, why it wins, what lost, how \
                   an imported shortcut translated with one of the controlled bridge-outcome states, and \
                   how a leader / multi-stroke shortcut resolves — across every declared consumer surface, \
                   with the same resolution preserved in headless/CLI execution, each surface's \
                   green/yellow/red claim auto-narrowed from its four inspection postures, and any surface \
                   that still hides a winner or shadowed binding, falls back to generic imported wording, \
                   requires hidden knowledge to explain a sequence, or cannot reconstruct its winning \
                   source from durable evidence blocked from a stable claim."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_RESOLVER_INSPECTORS_MATRIX_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_RESOLVER_INSPECTORS_MATRIX_DOC_REF.to_owned(),
        keybinding_resolver_ref: M5_RESOLVER_INSPECTORS_KEYBINDING_RESOLVER_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_inspection_dimensions,
        required_inspector_fields,
        required_bridge_outcomes,
        required_migration_actions,
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
            "command_status.resolver_inspection_registry".to_owned(),
            "keybinding_automation.auto_narrow.resolver_inspection_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.keybinding_resolver_inspectors".to_owned(),
            M5_RESOLVER_INSPECTORS_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_RESOLVER_INSPECTORS_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-keybinding-resolver-inspectors".to_owned()],
        published_report_ref: M5_RESOLVER_INSPECTORS_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_RESOLVER_INSPECTORS_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_RESOLVER_INSPECTORS_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_RESOLVER_INSPECTORS_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("inspection packet serializes"),
    ) {
        blocking_findings.push(ResolverInspectorFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_keybinding_resolver_inspectors_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ResolverInspectorValidationError {
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
    /// The declared required inspection dimensions do not match the lane constants.
    RequiredInspectionDimensionsStale,
    /// The declared required inspector fields do not match the lane constants.
    RequiredInspectorFieldsStale,
    /// The declared required bridge outcomes do not match the lane constants.
    RequiredBridgeOutcomesStale,
    /// The declared required migration actions do not match the lane constants.
    RequiredMigrationActionsStale,
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

/// Validates a packet against the resolver inspection invariants.
///
/// The checks encode the track invariant and acceptance criteria: every surface family carries a current
/// inspection row; each row's winner/shadowed resolution and status are the derived values, never
/// asserted; a green row cannot keep a claim while it hides a winner or shadowed binding, uses generic
/// imported wording, requires hidden knowledge to explain a sequence, cannot reconstruct its winning
/// source from durable evidence, loses headless/CLI parity, fails to reveal all seven inspector fields,
/// fails to render all six controlled bridge outcomes, fails to offer all three migration actions, or
/// fails to certify every declared consumer surface; and a disclosed narrowing is backed by a reason and,
/// where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_keybinding_resolver_inspectors_packet(
    packet: &ResolverInspectorPacket,
) -> Result<(), Vec<ResolverInspectorValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(ResolverInspectorValidationError::NoRows);
    }
    if packet.record_kind != M5_RESOLVER_INSPECTORS_PACKET_RECORD_KIND {
        errors.push(ResolverInspectorValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_RESOLVER_INSPECTORS_SCHEMA_VERSION {
        errors.push(ResolverInspectorValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(ResolverInspectorValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(ResolverInspectorValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_INSPECTION_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_inspection_dimensions != expected_dimensions {
        errors.push(ResolverInspectorValidationError::RequiredInspectionDimensionsStale);
    }
    let expected_inspector_fields: Vec<String> = REQUIRED_INSPECTOR_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    if packet.required_inspector_fields != expected_inspector_fields {
        errors.push(ResolverInspectorValidationError::RequiredInspectorFieldsStale);
    }
    let expected_bridge_outcomes: Vec<String> = REQUIRED_BRIDGE_OUTCOMES
        .iter()
        .map(|outcome| outcome.as_str().to_owned())
        .collect();
    if packet.required_bridge_outcomes != expected_bridge_outcomes {
        errors.push(ResolverInspectorValidationError::RequiredBridgeOutcomesStale);
    }
    let expected_migration_actions: Vec<String> = REQUIRED_MIGRATION_ACTIONS
        .iter()
        .map(|action| action.as_str().to_owned())
        .collect();
    if packet.required_migration_actions != expected_migration_actions {
        errors.push(ResolverInspectorValidationError::RequiredMigrationActionsStale);
    }
    let expected_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_surface_families != expected_families {
        errors.push(ResolverInspectorValidationError::RequiredSurfaceFamiliesStale);
    }

    let present: BTreeSet<M5CommandSurfaceFamily> =
        packet.rows.iter().map(|row| row.surface_family).collect();
    let coverage_complete = REQUIRED_SURFACE_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_SURFACE_FAMILIES.len() {
        errors.push(ResolverInspectorValidationError::CoverageIncomplete);
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
        errors.push(ResolverInspectorValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResolverInspectorStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResolverInspectorStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResolverInspectorStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(ResolverInspectorValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<ResolverInspectorWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(ResolverInspectorValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<ResolverInspectorCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(ResolverInspectorValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<ResolverInspectorFinding> = Vec::new();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(ResolverInspectorFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(ResolverInspectorFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("inspection packet serializes"),
    ) {
        recomputed.push(ResolverInspectorFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(ResolverInspectorValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(ResolverInspectorValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(ResolverInspectorValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(ResolverInspectorValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(ResolverInspectorValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(ResolverInspectorValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
