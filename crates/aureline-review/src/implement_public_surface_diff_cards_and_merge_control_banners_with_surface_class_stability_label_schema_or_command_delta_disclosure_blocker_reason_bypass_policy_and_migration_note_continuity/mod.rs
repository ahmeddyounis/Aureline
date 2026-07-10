//! Two reusable M5 protected-path governance components — the public-surface diff card and the
//! merge-control banner — so a user can tell *which* public surface a change materially affects
//! (command, CLI flag, schema, manifest, SDK/WIT surface, token, message id, automation contract, or
//! compatibility claim), *how stable* that surface is, whether the change is breaking, compatible, a
//! deprecation, or a removal, whether the diff was machine-generated, provider-confirmed, a local
//! estimate, not evaluated here, or stale relative to base/head, and — for the merge gate — which
//! required checks and ruleset/branch-protection rules apply, what the current blocker is, what the
//! bypass policy is, and whether the gate is provider-confirmed rather than a local estimate, before
//! they trust, merge, or release a governed change.
//!
//! Aureline's frozen protected-path governance component matrix
//! ([`crate::freeze_the_m5_protected_path_governance_component_matrix`]) names the public-surface diff
//! card and the merge-control banner as two governed component families and freezes their controlled
//! vocabulary — the one governance-state lexicon ([`M5GovernanceStateVocab`]): `advisory`,
//! `authoritative`, `covered`, `backup_missing`, `waived`, `expired`, `stale`,
//! `provider_authoritative`, and `local_estimate`. This module *implements* that contract as two
//! co-equal component vectors — a full public-surface diff card and a merge-control banner — that reuse
//! the one frozen lexicon and share one confirmation-locus resolver so a local estimate or a
//! machine-generated local diff can never masquerade as the provider's authoritative confirmation, a
//! stable-contract change can never hide inside ordinary review without an explicit diff card and
//! migration/evidence links, and a guarded merge can never name a generic blocker instead of the
//! current gate.
//!
//! The module has two derived resolvers:
//!
//! * [`resolve_confirmation_locus`] — takes a confirmation-locus source and derives the exact locus
//!   posture (provider-confirmed, machine-generated locally, local estimate, not evaluated here, or
//!   stale relative to base/head), whether the posture is the provider's authoritative confirmation,
//!   whether it was evaluated here at all, whether it is stale, and which frozen governance-state token
//!   it maps to — so a local estimate or a machine-generated local diff can never read as the
//!   provider's confirmation, and a not-evaluated-here signal can never read as evaluated. Both the
//!   public-surface diff card and the merge-control banner use it, so their local-versus-provider
//!   parity stays one truth. This is the AC pinning the merge-control blocker honesty: a merge gate
//!   never widens from a local estimate to provider mergeability without provider confirmation.
//! * [`resolve_surface_change`] — takes a surface-change source and derives the exact change posture
//!   (breaking, compatible, deprecation, or removal), whether the change is breaking, whether it
//!   requires a migration note, and the notes it must carry — so a breaking or removing change on a
//!   stable surface degrades explicitly and can never collapse into generic `changed` language.
//!
//! A single controls packet — [`PublicSurfaceMergeControlControlsPacket`] — binds one vector of
//! public-surface diff cards and one vector of merge-control banners to the same confirmation-locus,
//! surface-change, and blocker vocabulary, so surface class, stability label, schema-or-command delta
//! disclosure, blocker reason, bypass policy, and migration-note continuity stay explicit across the
//! review-workspace, release-candidate, governance, shiproom, CLI, and support-export consumers.
//!
//! The governance component ([`M5GovernanceComponent`]), governance-state vocabulary
//! ([`M5GovernanceStateVocab`]), downgrade trigger
//! ([`M5GovernanceComponentDowngradeTrigger`]), rollback posture
//! ([`M5GovernanceComponentRollbackPosture`]), and consumer surface
//! ([`M5GovernanceComponentConsumerSurface`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: the confirmation-locus source, the derived locus posture, the public-surface class, the
//! stability class, the surface-change source, the derived change posture, the diff-evidence kind, the
//! merge-blocker class, the branch-protection state, the required-check state, the bypass-policy class,
//! and the bounded card and banner actions. No M5 governed surface invents a second public-surface or
//! merge-control grammar.
//!
//! Raw change generators, raw provider payloads, raw diff bodies, raw ruleset definitions, credentials,
//! and secrets stay outside the export boundary; every surface, evidence, and gate reference is carried
//! only as an opaque, export-safe reference.

#[cfg(test)]
mod tests;

// The governance component family, the frozen governance-state lexicon, and the downgrade /
// rollback / consumer vocabularies are frozen once, in the protected-path governance component
// matrix. This lane reuses them verbatim so it never invents a parallel public-surface or
// merge-control vocabulary.
pub use crate::freeze_the_m5_protected_path_governance_component_matrix::{
    M5GovernanceComponent, M5GovernanceComponentConsumerSurface,
    M5GovernanceComponentDowngradeTrigger, M5GovernanceComponentRollbackPosture,
    M5GovernanceStateVocab, M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`PublicSurfaceMergeControlControlsPacket`].
pub const PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_RECORD_KIND: &str =
    "implement_public_surface_diff_cards_and_merge_control_banners_with_surface_class_stability_label_schema_or_command_delta_disclosure_blocker_reason_bypass_policy_and_migration_note_continuity";

/// Schema version for M5 public-surface-diff-card / merge-control-banner control records.
pub const PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-public-surface-diff-merge-control-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_DOC_REF: &str =
    "docs/review/m5/implement_public_surface_diff_cards_and_merge_control_banners.md";

/// Repo-relative path of the protected fixture directory.
pub const PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-public-surface-diff-merge-control-controls";

/// Repo-relative path of the checked support-export artifact.
pub const PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-public-surface-diff-merge-control-controls-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_SUMMARY_REF: &str =
    "artifacts/release/m5-public-surface-diff-merge-control-controls-proof/summary.md";

// ---- shared confirmation-locus vocabulary --------------------------------

/// The source a confirmation-locus signal comes from, before it is resolved into a posture.
///
/// This is the honest input to [`resolve_confirmation_locus`]: it names whether a governed signal (a
/// public-surface diff, or a merge gate) was authoritatively confirmed by the provider, reported by the
/// provider, machine-generated locally, is only a local heuristic estimate, was not evaluated here at
/// all, or is stale against the current base/head — so a local estimate or a machine-generated local
/// diff can never be asserted as the provider's authoritative confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationLocusSource {
    /// The provider authoritatively confirmed the gate or signal.
    ProviderConfirmedGate,
    /// The provider reported the status authoritatively.
    ProviderReportedState,
    /// Aureline machine-generated the signal locally (e.g. a machine-generated diff).
    MachineGeneratedLocally,
    /// The signal is only a local heuristic estimate.
    LocalHeuristicEstimate,
    /// The signal was not evaluated on this build.
    NotEvaluatedHere,
    /// The evaluation is stale relative to the current base/head.
    StaleAgainstBaseHead,
}

impl ConfirmationLocusSource {
    /// Every confirmation-locus source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderConfirmedGate,
        Self::ProviderReportedState,
        Self::MachineGeneratedLocally,
        Self::LocalHeuristicEstimate,
        Self::NotEvaluatedHere,
        Self::StaleAgainstBaseHead,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderConfirmedGate => "provider_confirmed_gate",
            Self::ProviderReportedState => "provider_reported_state",
            Self::MachineGeneratedLocally => "machine_generated_locally",
            Self::LocalHeuristicEstimate => "local_heuristic_estimate",
            Self::NotEvaluatedHere => "not_evaluated_here",
            Self::StaleAgainstBaseHead => "stale_against_base_head",
        }
    }
}

/// Derived confirmation-locus posture a diff card or merge banner may present.
///
/// This is the AC-pinned local-versus-provider parity axis: the posture is derived from the frozen
/// locus source, never asserted, so a user can tell whether a signal is a local estimate,
/// machine-generated locally, provider-confirmed, not evaluated here, or stale relative to base/head —
/// without opening raw payloads. A merge gate never widens from a local estimate to provider
/// mergeability without provider confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationLocusPosture {
    /// The provider authoritatively confirmed the signal.
    ProviderConfirmed,
    /// The signal was machine-generated locally.
    MachineGeneratedLocal,
    /// The signal is only a local estimate.
    LocalEstimate,
    /// The signal was not evaluated on this build.
    NotEvaluatedHere,
    /// The signal is stale relative to the current base/head.
    StaleRelativeToHead,
}

impl ConfirmationLocusPosture {
    /// Every confirmation-locus posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderConfirmed,
        Self::MachineGeneratedLocal,
        Self::LocalEstimate,
        Self::NotEvaluatedHere,
        Self::StaleRelativeToHead,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderConfirmed => "provider_confirmed",
            Self::MachineGeneratedLocal => "machine_generated_local",
            Self::LocalEstimate => "local_estimate",
            Self::NotEvaluatedHere => "not_evaluated_here",
            Self::StaleRelativeToHead => "stale_relative_to_head",
        }
    }

    /// True only when the provider authoritatively confirmed the signal.
    pub const fn is_provider_confirmed(self) -> bool {
        matches!(self, Self::ProviderConfirmed)
    }

    /// True when the signal was machine-generated locally.
    pub const fn is_machine_generated(self) -> bool {
        matches!(self, Self::MachineGeneratedLocal)
    }

    /// True when the signal is only a local estimate.
    pub const fn is_local_estimate(self) -> bool {
        matches!(self, Self::LocalEstimate)
    }

    /// True when the signal was evaluated on this build at all (stale counts as evaluated-but-old).
    pub const fn is_evaluated_here(self) -> bool {
        !matches!(self, Self::NotEvaluatedHere)
    }

    /// True when the signal is stale relative to the current base/head.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StaleRelativeToHead)
    }

    /// The frozen governance-state token this posture must render under, if the frozen lexicon names
    /// one. `machine_generated_local` and `not_evaluated_here` are honest states the frozen lexicon
    /// does not name, so they carry no governance token and must never borrow another state's label.
    pub const fn governance_vocab(self) -> Option<M5GovernanceStateVocab> {
        match self {
            Self::ProviderConfirmed => Some(M5GovernanceStateVocab::ProviderAuthoritative),
            Self::LocalEstimate => Some(M5GovernanceStateVocab::LocalEstimate),
            Self::StaleRelativeToHead => Some(M5GovernanceStateVocab::Stale),
            Self::MachineGeneratedLocal | Self::NotEvaluatedHere => None,
        }
    }
}

/// Locus disclosures a component must carry, derived from the confirmation-locus source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfirmationLocusDisclosure {
    /// The derived locus posture this component may present.
    pub posture: ConfirmationLocusPosture,
    /// Whether the provider authoritatively confirmed the signal.
    pub is_provider_confirmed: bool,
    /// Whether the signal was machine-generated locally.
    pub is_machine_generated: bool,
    /// Whether the signal is only a local estimate.
    pub is_local_estimate: bool,
    /// Whether the signal was evaluated on this build at all.
    pub is_evaluated_here: bool,
    /// Whether the signal is stale relative to the current base/head.
    pub is_stale: bool,
    /// Whether the component must carry an explicit local-estimate note.
    pub needs_local_estimate_note: bool,
    /// Whether the component must carry an explicit machine-generated note.
    pub needs_machine_generated_note: bool,
    /// Whether the component must carry an explicit not-evaluated-here note.
    pub needs_not_evaluated_note: bool,
    /// Whether the component must carry an explicit stale note.
    pub needs_stale_note: bool,
    /// The frozen governance-state token this posture must render under, if any.
    pub governance_vocab: Option<M5GovernanceStateVocab>,
}

/// Resolves the confirmation-locus posture a diff card or merge banner may present.
///
/// A `provider_confirmed_gate` or `provider_reported_state` source is provider-confirmed; a
/// `machine_generated_locally` source is machine-generated locally; a `local_heuristic_estimate` source
/// is a local estimate; a `not_evaluated_here` source is not-evaluated-here; and a
/// `stale_against_base_head` source is stale relative to base/head — so a local estimate or a
/// machine-generated local diff can never read as the provider's confirmation.
pub fn resolve_confirmation_locus(source: ConfirmationLocusSource) -> ConfirmationLocusDisclosure {
    use ConfirmationLocusPosture as Posture;
    use ConfirmationLocusSource as Src;

    let posture = match source {
        Src::ProviderConfirmedGate | Src::ProviderReportedState => Posture::ProviderConfirmed,
        Src::MachineGeneratedLocally => Posture::MachineGeneratedLocal,
        Src::LocalHeuristicEstimate => Posture::LocalEstimate,
        Src::NotEvaluatedHere => Posture::NotEvaluatedHere,
        Src::StaleAgainstBaseHead => Posture::StaleRelativeToHead,
    };

    ConfirmationLocusDisclosure {
        posture,
        is_provider_confirmed: posture.is_provider_confirmed(),
        is_machine_generated: posture.is_machine_generated(),
        is_local_estimate: posture.is_local_estimate(),
        is_evaluated_here: posture.is_evaluated_here(),
        is_stale: posture.is_stale(),
        needs_local_estimate_note: posture.is_local_estimate(),
        needs_machine_generated_note: posture.is_machine_generated(),
        needs_not_evaluated_note: !posture.is_evaluated_here(),
        needs_stale_note: posture.is_stale(),
        governance_vocab: posture.governance_vocab(),
    }
}

// ---- surface-change vocabulary -------------------------------------------

/// The source a surface-change signal comes from, before it is resolved into a posture.
///
/// This is the honest input to [`resolve_surface_change`]: it names whether a public surface had an
/// incompatible signature change, a semantic behavior break, a backward-compatible addition, a
/// clarifying compatible change, an announced deprecation, or an outright removal — so a breaking or
/// removing change can never collapse into generic `changed` language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceChangeSource {
    /// The surface's signature changed incompatibly.
    IncompatibleSignatureChange,
    /// The surface's behavior broke semantically.
    SemanticBehaviorBreak,
    /// A backward-compatible addition was made.
    BackwardCompatibleAddition,
    /// A clarifying change was made without behavior change.
    ClarifyingCompatibleChange,
    /// A deprecation was announced.
    DeprecationAnnounced,
    /// The surface was removed.
    SurfaceRemoved,
}

impl SurfaceChangeSource {
    /// Every surface-change source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IncompatibleSignatureChange,
        Self::SemanticBehaviorBreak,
        Self::BackwardCompatibleAddition,
        Self::ClarifyingCompatibleChange,
        Self::DeprecationAnnounced,
        Self::SurfaceRemoved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncompatibleSignatureChange => "incompatible_signature_change",
            Self::SemanticBehaviorBreak => "semantic_behavior_break",
            Self::BackwardCompatibleAddition => "backward_compatible_addition",
            Self::ClarifyingCompatibleChange => "clarifying_compatible_change",
            Self::DeprecationAnnounced => "deprecation_announced",
            Self::SurfaceRemoved => "surface_removed",
        }
    }
}

/// Derived surface-change posture a public-surface diff card may present.
///
/// This is the AC-pinned change-honesty axis: a breaking or removing change on a stable surface must
/// carry an explicit migration note and evidence link — it can never collapse into generic `changed`
/// language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceChangePosture {
    /// The change breaks the public contract.
    Breaking,
    /// The change is backward-compatible.
    Compatible,
    /// The change announces a deprecation.
    Deprecation,
    /// The change removes the surface.
    Removal,
}

impl SurfaceChangePosture {
    /// Every surface-change posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Breaking,
        Self::Compatible,
        Self::Deprecation,
        Self::Removal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Breaking => "breaking",
            Self::Compatible => "compatible",
            Self::Deprecation => "deprecation",
            Self::Removal => "removal",
        }
    }

    /// True when the change breaks or removes the public contract.
    pub const fn is_breaking(self) -> bool {
        matches!(self, Self::Breaking | Self::Removal)
    }

    /// True when the change is a removal.
    pub const fn is_removal(self) -> bool {
        matches!(self, Self::Removal)
    }

    /// True when the change is a deprecation.
    pub const fn is_deprecation(self) -> bool {
        matches!(self, Self::Deprecation)
    }

    /// True when the change is backward-compatible.
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// True when the change requires an explicit migration note (breaking, removal, or deprecation).
    pub const fn requires_migration_note(self) -> bool {
        matches!(self, Self::Breaking | Self::Removal | Self::Deprecation)
    }
}

/// Surface-change disclosures a public-surface diff card must carry, derived from the surface-change
/// source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceChangeDisclosure {
    /// The derived surface-change posture this card may present.
    pub posture: SurfaceChangePosture,
    /// Whether the change breaks or removes the public contract.
    pub is_breaking: bool,
    /// Whether the change is a removal.
    pub is_removal: bool,
    /// Whether the change is a deprecation.
    pub is_deprecation: bool,
    /// Whether the change is backward-compatible.
    pub is_compatible: bool,
    /// Whether the change requires an explicit migration note.
    pub requires_migration_note: bool,
    /// Whether the card must carry an explicit breaking note.
    pub needs_breaking_note: bool,
    /// Whether the card must carry an explicit deprecation note.
    pub needs_deprecation_note: bool,
    /// Whether the card must carry an explicit removal note.
    pub needs_removal_note: bool,
}

/// Resolves the surface-change posture a public-surface diff card may present.
///
/// An `incompatible_signature_change` or `semantic_behavior_break` source is breaking; a
/// `backward_compatible_addition` or `clarifying_compatible_change` source is compatible; a
/// `deprecation_announced` source is a deprecation; and a `surface_removed` source is a removal — so a
/// breaking or removing change can never collapse into generic `changed` language.
pub fn resolve_surface_change(source: SurfaceChangeSource) -> SurfaceChangeDisclosure {
    use SurfaceChangePosture as Posture;
    use SurfaceChangeSource as Src;

    let posture = match source {
        Src::IncompatibleSignatureChange | Src::SemanticBehaviorBreak => Posture::Breaking,
        Src::BackwardCompatibleAddition | Src::ClarifyingCompatibleChange => Posture::Compatible,
        Src::DeprecationAnnounced => Posture::Deprecation,
        Src::SurfaceRemoved => Posture::Removal,
    };

    SurfaceChangeDisclosure {
        posture,
        is_breaking: posture.is_breaking(),
        is_removal: posture.is_removal(),
        is_deprecation: posture.is_deprecation(),
        is_compatible: posture.is_compatible(),
        requires_migration_note: posture.requires_migration_note(),
        needs_breaking_note: matches!(posture, Posture::Breaking),
        needs_deprecation_note: posture.is_deprecation(),
        needs_removal_note: posture.is_removal(),
    }
}

// ---- public-surface-diff-card-specific vocabulary ------------------------

/// The public surface a diff card names as materially affected, so a change never hides which
/// externally depended-on contract it touches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicSurfaceClass {
    /// A command.
    Command,
    /// A CLI flag.
    CliFlag,
    /// A schema.
    Schema,
    /// A manifest.
    Manifest,
    /// An SDK or WIT surface.
    SdkWitSurface,
    /// A token.
    Token,
    /// A message id.
    MessageId,
    /// An automation contract.
    AutomationContract,
    /// A compatibility claim.
    CompatibilityClaim,
}

impl PublicSurfaceClass {
    /// Every public-surface class, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Command,
        Self::CliFlag,
        Self::Schema,
        Self::Manifest,
        Self::SdkWitSurface,
        Self::Token,
        Self::MessageId,
        Self::AutomationContract,
        Self::CompatibilityClaim,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Command => "command",
            Self::CliFlag => "cli_flag",
            Self::Schema => "schema",
            Self::Manifest => "manifest",
            Self::SdkWitSurface => "sdk_wit_surface",
            Self::Token => "token",
            Self::MessageId => "message_id",
            Self::AutomationContract => "automation_contract",
            Self::CompatibilityClaim => "compatibility_claim",
        }
    }
}

/// The stability class of the public surface a diff card names, so a stable-contract change never reads
/// as an experimental one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilityClass {
    /// A stable, externally depended-on contract.
    Stable,
    /// A beta surface.
    Beta,
    /// An experimental surface.
    Experimental,
    /// An internal surface.
    Internal,
}

impl StabilityClass {
    /// Every stability class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Stable, Self::Beta, Self::Experimental, Self::Internal];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Experimental => "experimental",
            Self::Internal => "internal",
        }
    }

    /// True only for a stable, externally depended-on contract.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// The kind of stable diff evidence a public-surface diff card can open, so a change always names the
/// exact machine-generated diff, migration guide, or compatibility report a user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffEvidenceKind {
    /// A machine-generated diff.
    MachineGeneratedDiff,
    /// A migration guide.
    MigrationGuide,
    /// A compatibility report.
    CompatibilityReport,
    /// A changelog entry.
    ChangelogEntry,
    /// No diff evidence is bound (the card names that it routes nowhere).
    NoDiffEvidence,
}

impl DiffEvidenceKind {
    /// Every diff-evidence kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MachineGeneratedDiff,
        Self::MigrationGuide,
        Self::CompatibilityReport,
        Self::ChangelogEntry,
        Self::NoDiffEvidence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MachineGeneratedDiff => "machine_generated_diff",
            Self::MigrationGuide => "migration_guide",
            Self::CompatibilityReport => "compatibility_report",
            Self::ChangelogEntry => "changelog_entry",
            Self::NoDiffEvidence => "no_diff_evidence",
        }
    }

    /// True when this kind names a resolvable diff-evidence link.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDiffEvidence)
    }
}

/// One keyboard-complete default action a public-surface diff card offers.
///
/// `OpenDiffEvidence`, `InspectSurfaceChange`, and `ReviewMigrationNote` are always offered so the
/// machine-generated diff, the surface change, and the migration note stay inspectable before a user
/// trusts a public-surface change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicSurfaceDiffCardAction {
    /// Open the machine-generated diff evidence (always available).
    OpenDiffEvidence,
    /// Inspect the surface change (always available).
    InspectSurfaceChange,
    /// Review the migration note (always available).
    ReviewMigrationNote,
    /// Inspect the stability label.
    InspectStabilityLabel,
    /// Compare the base/head identity.
    CompareBaseHead,
    /// Copy the export-safe surface digest.
    CopySurfaceDigest,
}

impl PublicSurfaceDiffCardAction {
    /// Every diff-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDiffEvidence,
        Self::InspectSurfaceChange,
        Self::ReviewMigrationNote,
        Self::InspectStabilityLabel,
        Self::CompareBaseHead,
        Self::CopySurfaceDigest,
    ];

    /// The default actions every keyboard-complete diff card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenDiffEvidence,
        Self::InspectSurfaceChange,
        Self::ReviewMigrationNote,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDiffEvidence => "open_diff_evidence",
            Self::InspectSurfaceChange => "inspect_surface_change",
            Self::ReviewMigrationNote => "review_migration_note",
            Self::InspectStabilityLabel => "inspect_stability_label",
            Self::CompareBaseHead => "compare_base_head",
            Self::CopySurfaceDigest => "copy_surface_digest",
        }
    }
}

// ---- merge-control-banner-specific vocabulary ----------------------------

/// The class of the current merge blocker, so a guarded merge names the current gate honestly rather
/// than a generic blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeBlockerClass {
    /// A required check is failing.
    RequiredCheckFailing,
    /// A required review is missing.
    RequiredReviewMissing,
    /// A branch-protection rule blocks the merge.
    BranchProtectionRule,
    /// A ruleset violation blocks the merge.
    RulesetViolation,
    /// A merge conflict blocks the merge.
    MergeConflict,
    /// There is no current blocker.
    NoBlocker,
}

impl MergeBlockerClass {
    /// Every merge-blocker class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RequiredCheckFailing,
        Self::RequiredReviewMissing,
        Self::BranchProtectionRule,
        Self::RulesetViolation,
        Self::MergeConflict,
        Self::NoBlocker,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredCheckFailing => "required_check_failing",
            Self::RequiredReviewMissing => "required_review_missing",
            Self::BranchProtectionRule => "branch_protection_rule",
            Self::RulesetViolation => "ruleset_violation",
            Self::MergeConflict => "merge_conflict",
            Self::NoBlocker => "no_blocker",
        }
    }

    /// True when this class names a current blocker.
    pub const fn is_blocking(self) -> bool {
        !matches!(self, Self::NoBlocker)
    }
}

/// The ruleset / branch-protection state a merge-control banner names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectionState {
    /// The provider authoritatively enforces the gate.
    ProviderEnforced,
    /// A ruleset enforces the gate.
    RulesetEnforced,
    /// The protection is advisory only.
    AdvisoryOnly,
    /// No protection is configured.
    NotConfigured,
}

impl ProtectionState {
    /// Every protection state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProviderEnforced,
        Self::RulesetEnforced,
        Self::AdvisoryOnly,
        Self::NotConfigured,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderEnforced => "provider_enforced",
            Self::RulesetEnforced => "ruleset_enforced",
            Self::AdvisoryOnly => "advisory_only",
            Self::NotConfigured => "not_configured",
        }
    }
}

/// The state of one required check a merge-control banner names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredCheckState {
    /// The check is passing.
    Passing,
    /// The check is failing.
    Failing,
    /// The check is pending.
    Pending,
    /// The check has not been reported.
    Missing,
}

impl RequiredCheckState {
    /// Every required-check state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Passing, Self::Failing, Self::Pending, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passing => "passing",
            Self::Failing => "failing",
            Self::Pending => "pending",
            Self::Missing => "missing",
        }
    }
}

/// One required check a merge-control banner makes explicit, so a guarded merge never hides which
/// checks the gate requires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequiredCheck {
    /// The required-check label; required and non-empty.
    pub check_label: String,
    /// The state this check is in.
    pub check_state: RequiredCheckState,
    /// Whether this check is currently blocking the merge.
    pub is_blocking: bool,
}

/// The bypass policy a merge-control banner names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BypassPolicyClass {
    /// No bypass is allowed.
    NoBypassAllowed,
    /// An admin bypass is allowed.
    AdminBypassAllowed,
    /// An emergency bypass is allowed.
    EmergencyBypassAllowed,
    /// A bypass was used.
    BypassUsed,
}

impl BypassPolicyClass {
    /// Every bypass-policy class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoBypassAllowed,
        Self::AdminBypassAllowed,
        Self::EmergencyBypassAllowed,
        Self::BypassUsed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBypassAllowed => "no_bypass_allowed",
            Self::AdminBypassAllowed => "admin_bypass_allowed",
            Self::EmergencyBypassAllowed => "emergency_bypass_allowed",
            Self::BypassUsed => "bypass_used",
        }
    }
}

/// One keyboard-complete default action a merge-control banner offers.
///
/// `InspectMergeGate`, `ReviewRequiredChecks`, and `ReviewBypassPolicy` are always offered so the
/// current gate, the required checks, and the bypass policy stay inspectable before a user trusts the
/// merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeControlBannerAction {
    /// Inspect the current merge gate (always available).
    InspectMergeGate,
    /// Review the required checks (always available).
    ReviewRequiredChecks,
    /// Review the bypass policy (always available).
    ReviewBypassPolicy,
    /// Inspect the ruleset / branch-protection state.
    InspectProtectionState,
    /// Open the blocker evidence.
    OpenBlockerEvidence,
    /// Copy the export-safe merge-control summary.
    CopyMergeControlSummary,
}

impl MergeControlBannerAction {
    /// Every merge-control-banner action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectMergeGate,
        Self::ReviewRequiredChecks,
        Self::ReviewBypassPolicy,
        Self::InspectProtectionState,
        Self::OpenBlockerEvidence,
        Self::CopyMergeControlSummary,
    ];

    /// The default actions every keyboard-complete merge-control banner must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::InspectMergeGate,
        Self::ReviewRequiredChecks,
        Self::ReviewBypassPolicy,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectMergeGate => "inspect_merge_gate",
            Self::ReviewRequiredChecks => "review_required_checks",
            Self::ReviewBypassPolicy => "review_bypass_policy",
            Self::InspectProtectionState => "inspect_protection_state",
            Self::OpenBlockerEvidence => "open_blocker_evidence",
            Self::CopyMergeControlSummary => "copy_merge_control_summary",
        }
    }
}

// ---- component structs ---------------------------------------------------

/// A public-surface diff card naming its affected public surfaces, stability label, schema-or-command
/// delta disclosure, breaking/compatible/deprecation/removal change, machine-generated-versus-provider
/// confirmation parity, diff evidence, and migration note where relevant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicSurfaceDiffCard {
    /// Frozen component this control implements; must be `public_surface_diff_card`.
    pub component: M5GovernanceComponent,
    /// Stable card id.
    pub card_id: String,
    /// Change title label; required and non-empty.
    pub change_title_label: String,
    /// Affected public-surface classes; required and non-empty so which surfaces are affected is
    /// explicit.
    pub surface_classes: Vec<PublicSurfaceClass>,
    /// Surface-class label; required and non-empty.
    pub surface_class_label: String,
    /// Stability class of the affected surface.
    pub stability_class: StabilityClass,
    /// Stability label; required and non-empty so the stability of the surface stays explicit.
    pub stability_label: String,
    /// Surface-change source, resolved into the change posture.
    pub surface_change_source: SurfaceChangeSource,
    /// Derived surface-change posture (must equal the resolved posture).
    pub derived_surface_change: SurfaceChangePosture,
    /// Whether the card claims a breaking change (must equal derived truth).
    pub claims_breaking: bool,
    /// Schema-or-command delta disclosure label; required and non-empty so the exact delta stays
    /// explicit.
    pub delta_disclosure_label: String,
    /// Confirmation-locus source, resolved into the locus posture.
    pub confirmation_locus_source: ConfirmationLocusSource,
    /// Derived confirmation-locus posture (must equal the resolved posture).
    pub derived_confirmation_locus: ConfirmationLocusPosture,
    /// Whether the card claims provider-confirmed evaluation (must equal derived truth).
    pub claims_provider_confirmed: bool,
    /// Frozen governance-state vocabulary this card renders (must include the derived locus token).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Local-estimate note; required when the diff is a local estimate.
    pub local_estimate_note: String,
    /// Machine-generated note; required when the diff was machine-generated locally.
    pub machine_generated_note: String,
    /// Not-evaluated-here note; required when the diff was not evaluated on this build.
    pub not_evaluated_note: String,
    /// Stale note; required when the diff is stale relative to base/head.
    pub stale_note: String,
    /// Breaking note; required when the change is breaking.
    pub breaking_note: String,
    /// Deprecation note; required when the change is a deprecation.
    pub deprecation_note: String,
    /// Removal note; required when the change is a removal.
    pub removal_note: String,
    /// Migration note; required when a stable surface has a migration-worthy change.
    pub migration_note: String,
    /// Opaque migration-evidence reference; required when a stable surface has a migration-worthy
    /// change.
    pub migration_evidence_ref: String,
    /// Kind of stable diff-evidence link this card can open.
    pub diff_evidence_kind: DiffEvidenceKind,
    /// Opaque stable diff-evidence reference; required when the kind resolves.
    pub diff_evidence_ref: String,
    /// Context note; always required so the card names what to check before trusting the change.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub card_actions: Vec<PublicSurfaceDiffCardAction>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must project this card.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its surface class or stability. MUST be `false`.
    pub hides_surface_class_or_stability: bool,
    /// Hard invariant: never lets a stable breaking change hide without migration/evidence. MUST be
    /// `false`.
    pub lets_stable_breaking_change_hide_without_migration: bool,
    /// Hard invariant: never lets a local estimate or machine-generated diff read as provider-confirmed.
    /// MUST be `false`.
    pub lets_local_estimate_read_as_provider_confirmed: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl PublicSurfaceDiffCard {
    /// Locus disclosures this card must carry, derived from the frozen source.
    pub fn locus_disclosure(&self) -> ConfirmationLocusDisclosure {
        resolve_confirmation_locus(self.confirmation_locus_source)
    }

    /// Surface-change disclosures this card must carry, derived from the frozen source.
    pub fn change_disclosure(&self) -> SurfaceChangeDisclosure {
        resolve_surface_change(self.surface_change_source)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<PublicSurfaceDiffCardAction> =
            self.card_actions.iter().copied().collect();
        PublicSurfaceDiffCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

/// A merge-control banner naming its current blocker, required checks, ruleset/branch-protection state,
/// bypass policy, local-versus-provider mergeability parity, and export-packet parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeControlBanner {
    /// Frozen component this control implements; must be `merge_control_banner`.
    pub component: M5GovernanceComponent,
    /// Stable banner id.
    pub banner_id: String,
    /// Gate title label; required and non-empty.
    pub gate_title_label: String,
    /// Current merge-blocker class.
    pub blocker_class: MergeBlockerClass,
    /// Blocker reason label; required and non-empty when the blocker class is blocking so the current
    /// gate is named honestly.
    pub blocker_reason_label: String,
    /// Required checks this gate names; required and non-empty.
    pub required_checks: Vec<RequiredCheck>,
    /// Required-checks label; required and non-empty.
    pub required_checks_label: String,
    /// Ruleset / branch-protection state.
    pub protection_state: ProtectionState,
    /// Protection-state label; required and non-empty.
    pub protection_state_label: String,
    /// Bypass policy.
    pub bypass_policy: BypassPolicyClass,
    /// Bypass-policy label; required and non-empty.
    pub bypass_policy_label: String,
    /// Confirmation-locus source, resolved into the locus posture.
    pub confirmation_locus_source: ConfirmationLocusSource,
    /// Derived confirmation-locus posture (must equal the resolved posture).
    pub derived_confirmation_locus: ConfirmationLocusPosture,
    /// Whether the banner claims provider-confirmed mergeability (must equal derived truth).
    pub claims_provider_confirmed: bool,
    /// Whether the banner claims it was evaluated here (must equal derived truth).
    pub claims_evaluated_here: bool,
    /// Mergeability label; always required so the local-versus-provider mergeability stays explicit.
    pub mergeability_label: String,
    /// Export-parity label; always required so export-packet parity stays explicit.
    pub export_parity_label: String,
    /// Frozen governance-state vocabulary this banner renders (must include the derived locus token).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Local-estimate note; required when the gate is a local estimate.
    pub local_estimate_note: String,
    /// Machine-generated note; required when the gate signal was machine-generated locally.
    pub machine_generated_note: String,
    /// Not-evaluated-here note; required when the gate was not evaluated on this build.
    pub not_evaluated_note: String,
    /// Stale note; required when the gate is stale relative to base/head.
    pub stale_note: String,
    /// Context note; always required so the banner names what to check before trusting the merge.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub banner_actions: Vec<MergeControlBannerAction>,
    /// Downgrade triggers this banner can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must project this banner.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this banner.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its blocker reason or bypass policy. MUST be `false`.
    pub hides_blocker_reason_or_bypass_policy: bool,
    /// Hard invariant: never lets a local estimate read as provider-confirmed mergeability. MUST be
    /// `false`.
    pub lets_local_estimate_read_as_provider_mergeable: bool,
    /// Hard invariant: never names a generic blocker instead of the current gate. MUST be `false`.
    pub names_generic_blocker_instead_of_current_gate: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl MergeControlBanner {
    /// Locus disclosures this banner must carry, derived from the frozen source.
    pub fn locus_disclosure(&self) -> ConfirmationLocusDisclosure {
        resolve_confirmation_locus(self.confirmation_locus_source)
    }

    /// Whether the banner offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<MergeControlBannerAction> =
            self.banner_actions.iter().copied().collect();
        MergeControlBannerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

// ---- review blocks -------------------------------------------------------

/// First-glance public-surface-diff / merge-control review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicSurfaceMergeControlReview {
    /// The diff card names its surface class and stability.
    pub diff_card_shows_surface_class_and_stability: bool,
    /// The diff card names its schema-or-command delta disclosure.
    pub diff_card_shows_schema_or_command_delta: bool,
    /// The diff card offers an open-diff-evidence action.
    pub diff_card_offers_open_diff_evidence: bool,
    /// The merge banner names its blocker reason and bypass policy.
    pub merge_banner_shows_blocker_reason_and_bypass: bool,
    /// The merge banner names its required checks and protection state.
    pub merge_banner_shows_required_checks_and_protection: bool,
    /// The merge banner names its export-packet parity.
    pub merge_banner_shows_export_parity: bool,
    /// Confirmation-locus parity is derived from state, never asserted.
    pub confirmation_locus_derived_never_asserted: bool,
    /// A local estimate or machine-generated diff is never shown as provider-confirmed.
    pub local_or_machine_never_shown_as_provider_confirmed: bool,
    /// A not-evaluated-here signal is never shown as evaluated.
    pub not_evaluated_here_never_shown_as_evaluated: bool,
    /// A stable-contract change never hides inside ordinary review without an explicit diff card.
    pub stable_breaking_change_never_hides_without_migration: bool,
    /// A merge blocker never reads as a generic blocker instead of the current gate.
    pub merge_blocker_never_generic: bool,
    /// A stable-contract change always carries migration and evidence links.
    pub migration_and_evidence_required_for_stable_change: bool,
    /// Staleness relative to base/head stays explicit.
    pub stale_relative_to_base_head_always_explicit: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl PublicSurfaceMergeControlReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.diff_card_shows_surface_class_and_stability
            && self.diff_card_shows_schema_or_command_delta
            && self.diff_card_offers_open_diff_evidence
            && self.merge_banner_shows_blocker_reason_and_bypass
            && self.merge_banner_shows_required_checks_and_protection
            && self.merge_banner_shows_export_parity
            && self.confirmation_locus_derived_never_asserted
            && self.local_or_machine_never_shown_as_provider_confirmed
            && self.not_evaluated_here_never_shown_as_evaluated
            && self.stable_breaking_change_never_hides_without_migration
            && self.merge_blocker_never_generic
            && self.migration_and_evidence_required_for_stable_change
            && self.stale_relative_to_base_head_always_explicit
            && self.no_surface_invents_alternate_state_label
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicSurfaceMergeControlConsumerProjection {
    /// The review-workspace surface reads a single canonical source.
    pub review_workspace_reads_single_source: bool,
    /// The release-candidate surface reads a single canonical source.
    pub release_candidate_reads_single_source: bool,
    /// The governance and shiproom surfaces read a single canonical source.
    pub governance_and_shiproom_read_single_source: bool,
    /// Surface class and delta are visible before a merge or publish feels safe.
    pub surface_class_and_delta_visible_before_merge: bool,
    /// Blocker and bypass policy are visible before a merge feels safe.
    pub blocker_and_bypass_visible_before_merge: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl PublicSurfaceMergeControlConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_reads_single_source
            && self.release_candidate_reads_single_source
            && self.governance_and_shiproom_read_single_source
            && self.surface_class_and_delta_visible_before_merge
            && self.blocker_and_bypass_visible_before_merge
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicSurfaceMergeControlProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PublicSurfaceMergeControlControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicSurfaceMergeControlControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Public-surface diff cards.
    pub public_surface_diff_cards: Vec<PublicSurfaceDiffCard>,
    /// Merge-control banners.
    pub merge_control_banners: Vec<MergeControlBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Public-surface-diff / merge-control review block.
    pub review: PublicSurfaceMergeControlReview,
    /// Consumer projection block.
    pub consumer_projection: PublicSurfaceMergeControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PublicSurfaceMergeControlProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe public-surface-diff-card / merge-control-banner controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicSurfaceMergeControlControlsPacket {
    /// Record kind; must equal [`PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Public-surface diff cards.
    pub public_surface_diff_cards: Vec<PublicSurfaceDiffCard>,
    /// Merge-control banners.
    pub merge_control_banners: Vec<MergeControlBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Public-surface-diff / merge-control review block.
    pub review: PublicSurfaceMergeControlReview,
    /// Consumer projection block.
    pub consumer_projection: PublicSurfaceMergeControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PublicSurfaceMergeControlProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PublicSurfaceMergeControlControlsPacket {
    /// Builds a public-surface-diff-card / merge-control-banner controls packet from stable-lane input.
    pub fn new(input: PublicSurfaceMergeControlControlsPacketInput) -> Self {
        Self {
            record_kind: PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            public_surface_diff_cards: input.public_surface_diff_cards,
            merge_control_banners: input.merge_control_banners,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            review: input.review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the public-surface-diff-card / merge-control-banner control invariants.
    pub fn validate(&self) -> Vec<PublicSurfaceMergeControlControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_RECORD_KIND {
            violations.push(PublicSurfaceMergeControlControlsViolation::WrongRecordKind);
        }
        if self.schema_version != PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_SCHEMA_VERSION {
            violations.push(PublicSurfaceMergeControlControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PublicSurfaceMergeControlControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(PublicSurfaceMergeControlControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(PublicSurfaceMergeControlControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_public_surface_diff_cards(self, &mut violations);
        validate_merge_control_banners(self, &mut violations);
        validate_shared_coverage(self, &mut violations);

        if !self.review.all_hold() {
            violations.push(PublicSurfaceMergeControlControlsViolation::ReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(PublicSurfaceMergeControlControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(PublicSurfaceMergeControlControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("public-surface merge-control controls packet serializes"),
        ) {
            violations.push(PublicSurfaceMergeControlControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("public-surface merge-control controls packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_breaking = self
            .public_surface_diff_cards
            .iter()
            .filter(|card| card.stability_class.is_stable() && card.change_disclosure().is_breaking)
            .count();
        let blocking_banners = self
            .merge_control_banners
            .iter()
            .filter(|banner| banner.blocker_class.is_blocking())
            .count();

        let mut out = String::new();
        out.push_str("# Public-surface diff cards and merge-control banners\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Public-surface diff cards: {} ({} stable breaking)\n",
            self.public_surface_diff_cards.len(),
            stable_breaking
        ));
        out.push_str(&format!(
            "- Merge-control banners: {} ({} with a current blocker)\n",
            self.merge_control_banners.len(),
            blocking_banners
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Public-surface diff cards\n\n");
        for card in &self.public_surface_diff_cards {
            let locus = card.locus_disclosure();
            let change = card.change_disclosure();
            out.push_str(&format!(
                "- **{}** — stability `{}`, change `{}`, parity `{}`, evidence `{}`\n",
                card.change_title_label,
                card.stability_class.as_str(),
                change.posture.as_str(),
                locus.posture.as_str(),
                card.diff_evidence_kind.as_str(),
            ));
        }

        out.push_str("\n## Merge-control banners\n\n");
        for banner in &self.merge_control_banners {
            let locus = banner.locus_disclosure();
            out.push_str(&format!(
                "- **{}** — blocker `{}`, protection `{}`, bypass `{}`, parity `{}`\n",
                banner.gate_title_label,
                banner.blocker_class.as_str(),
                banner.protection_state.as_str(),
                banner.bypass_policy.as_str(),
                locus.posture.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in public-surface merge-control controls export.
#[derive(Debug)]
pub enum PublicSurfaceMergeControlControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PublicSurfaceMergeControlControlsViolation>),
}

impl fmt::Display for PublicSurfaceMergeControlControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "public-surface merge-control controls export parse failed: {error}"
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
                    "public-surface merge-control controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PublicSurfaceMergeControlControlsArtifactError {}

/// Validation failures emitted by [`PublicSurfaceMergeControlControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PublicSurfaceMergeControlControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No public-surface diff cards are present.
    PublicSurfaceDiffCardsMissing,
    /// A public-surface diff card is incomplete.
    PublicSurfaceDiffCardIncomplete,
    /// A public-surface diff card carries the wrong frozen component class.
    PublicSurfaceDiffCardWrongComponentClass,
    /// No merge-control banners are present.
    MergeControlBannersMissing,
    /// A merge-control banner is incomplete.
    MergeControlBannerIncomplete,
    /// A merge-control banner carries the wrong frozen component class.
    MergeControlBannerWrongComponentClass,
    /// A component misrepresents its derived confirmation-locus posture.
    ConfirmationLocusMisrepresented,
    /// A local-estimate or machine-generated component claims provider-confirmed evaluation.
    LocalOrMachineClaimsProviderConfirmed,
    /// A not-evaluated-here component claims it was evaluated.
    NotEvaluatedClaimsEvaluated,
    /// A local-estimate component does not name its local estimate.
    LocalEstimateNoteMissing,
    /// A machine-generated component does not name its machine-generated diff.
    MachineGeneratedNoteMissing,
    /// A not-evaluated-here component does not name that it was not evaluated here.
    NotEvaluatedNoteMissing,
    /// A stale component does not name its staleness.
    StaleNoteMissing,
    /// A component's governance vocabulary omits its derived locus token.
    GovernanceVocabMissingLocusToken,
    /// A public-surface diff card misrepresents its derived surface-change posture.
    SurfaceChangeMisrepresented,
    /// A stable-surface migration-worthy change does not name its migration note or evidence.
    StableChangeMissingMigrationOrEvidence,
    /// A breaking diff card does not name its breaking change.
    BreakingNoteMissing,
    /// A deprecation diff card does not name its deprecation.
    DeprecationNoteMissing,
    /// A removal diff card does not name its removal.
    RemovalNoteMissing,
    /// A public-surface diff card does not offer an open-diff-evidence action.
    OpenDiffEvidenceActionMissing,
    /// A public-surface diff card does not name any affected surface class.
    SurfaceClassSetMissing,
    /// A blocking merge-control banner does not name its blocker reason.
    MergeBlockerReasonMissing,
    /// A merge-control banner does not name any required check.
    RequiredChecksMissing,
    /// A merge-control banner lists an incomplete required check.
    RequiredCheckIncomplete,
    /// A component names a diff-evidence link but not its stable reference.
    DiffEvidenceRefMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component omits a mandatory action.
    ComponentActionsIncomplete,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare any consumer surface.
    ConsumerSurfacesMissing,
    /// The components do not cover every confirmation-locus source.
    LocusSourceCoverageMissing,
    /// The components do not cover every derived confirmation-locus posture.
    LocusPostureCoverageMissing,
    /// The diff cards do not cover every surface-change source.
    SurfaceChangeSourceCoverageMissing,
    /// The diff cards do not cover every derived surface-change posture.
    SurfaceChangePostureCoverageMissing,
    /// The diff cards alone do not cover every public-surface class.
    PublicSurfaceClassCoverageMissing,
    /// The diff cards do not cover every stability class.
    StabilityClassCoverageMissing,
    /// The diff cards do not cover every diff-evidence kind.
    DiffEvidenceKindCoverageMissing,
    /// The merge-control banners do not cover every merge-blocker class.
    MergeBlockerClassCoverageMissing,
    /// The merge-control banners do not cover every bypass-policy class.
    BypassPolicyCoverageMissing,
    /// The merge-control banners do not cover every protection state.
    ProtectionStateCoverageMissing,
    /// A component hides its surface class, stability, blocker reason, or bypass policy.
    SurfaceClassOrBlockerHidden,
    /// A component lets a local estimate masquerade as provider-confirmed.
    LocalEstimateMasqueradesAsProviderConfirmed,
    /// A component lets a stable breaking change hide, or names a generic blocker.
    MigrationOrGateMisrepresented,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// Review does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl PublicSurfaceMergeControlControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::PublicSurfaceDiffCardsMissing => "public_surface_diff_cards_missing",
            Self::PublicSurfaceDiffCardIncomplete => "public_surface_diff_card_incomplete",
            Self::PublicSurfaceDiffCardWrongComponentClass => {
                "public_surface_diff_card_wrong_component_class"
            }
            Self::MergeControlBannersMissing => "merge_control_banners_missing",
            Self::MergeControlBannerIncomplete => "merge_control_banner_incomplete",
            Self::MergeControlBannerWrongComponentClass => {
                "merge_control_banner_wrong_component_class"
            }
            Self::ConfirmationLocusMisrepresented => "confirmation_locus_misrepresented",
            Self::LocalOrMachineClaimsProviderConfirmed => {
                "local_or_machine_claims_provider_confirmed"
            }
            Self::NotEvaluatedClaimsEvaluated => "not_evaluated_claims_evaluated",
            Self::LocalEstimateNoteMissing => "local_estimate_note_missing",
            Self::MachineGeneratedNoteMissing => "machine_generated_note_missing",
            Self::NotEvaluatedNoteMissing => "not_evaluated_note_missing",
            Self::StaleNoteMissing => "stale_note_missing",
            Self::GovernanceVocabMissingLocusToken => "governance_vocab_missing_locus_token",
            Self::SurfaceChangeMisrepresented => "surface_change_misrepresented",
            Self::StableChangeMissingMigrationOrEvidence => {
                "stable_change_missing_migration_or_evidence"
            }
            Self::BreakingNoteMissing => "breaking_note_missing",
            Self::DeprecationNoteMissing => "deprecation_note_missing",
            Self::RemovalNoteMissing => "removal_note_missing",
            Self::OpenDiffEvidenceActionMissing => "open_diff_evidence_action_missing",
            Self::SurfaceClassSetMissing => "surface_class_set_missing",
            Self::MergeBlockerReasonMissing => "merge_blocker_reason_missing",
            Self::RequiredChecksMissing => "required_checks_missing",
            Self::RequiredCheckIncomplete => "required_check_incomplete",
            Self::DiffEvidenceRefMissing => "diff_evidence_ref_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::ComponentActionsIncomplete => "component_actions_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::LocusSourceCoverageMissing => "locus_source_coverage_missing",
            Self::LocusPostureCoverageMissing => "locus_posture_coverage_missing",
            Self::SurfaceChangeSourceCoverageMissing => "surface_change_source_coverage_missing",
            Self::SurfaceChangePostureCoverageMissing => "surface_change_posture_coverage_missing",
            Self::PublicSurfaceClassCoverageMissing => "public_surface_class_coverage_missing",
            Self::StabilityClassCoverageMissing => "stability_class_coverage_missing",
            Self::DiffEvidenceKindCoverageMissing => "diff_evidence_kind_coverage_missing",
            Self::MergeBlockerClassCoverageMissing => "merge_blocker_class_coverage_missing",
            Self::BypassPolicyCoverageMissing => "bypass_policy_coverage_missing",
            Self::ProtectionStateCoverageMissing => "protection_state_coverage_missing",
            Self::SurfaceClassOrBlockerHidden => "surface_class_or_blocker_hidden",
            Self::LocalEstimateMasqueradesAsProviderConfirmed => {
                "local_estimate_masquerades_as_provider_confirmed"
            }
            Self::MigrationOrGateMisrepresented => "migration_or_gate_misrepresented",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable public-surface merge-control controls export.
///
/// This is the first real consumer of the public-surface-diff / merge-control component lane: a
/// review-workspace, release-candidate, governance, shiproom, or support-export surface calls it to
/// ingest the canonical components rather than cloning governance text.
///
/// # Errors
///
/// Returns [`PublicSurfaceMergeControlControlsArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_public_surface_merge_control_controls_export(
) -> Result<PublicSurfaceMergeControlControlsPacket, PublicSurfaceMergeControlControlsArtifactError>
{
    let packet: PublicSurfaceMergeControlControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-public-surface-diff-merge-control-controls-proof/support_export.json"
    )))
    .map_err(PublicSurfaceMergeControlControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PublicSurfaceMergeControlControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &PublicSurfaceMergeControlControlsPacket,
    violations: &mut Vec<PublicSurfaceMergeControlControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_SCHEMA_REF,
        PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PublicSurfaceMergeControlControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

/// The four hard-invariant bools every component maps into the shared check.
struct ControlInvariants {
    surface_class_or_blocker_hidden: bool,
    local_estimate_masquerades_as_provider_confirmed: bool,
    migration_or_gate_misrepresented: bool,
    invents_alternate_state_label: bool,
}

/// Validates the confirmation-locus posture, notes, and cross-checks shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_shared_locus(
    disclosure: &ConfirmationLocusDisclosure,
    derived_confirmation_locus: ConfirmationLocusPosture,
    claims_provider_confirmed: bool,
    governance_state_vocab: &[M5GovernanceStateVocab],
    local_estimate_note: &str,
    machine_generated_note: &str,
    not_evaluated_note: &str,
    stale_note: &str,
    violations: &mut Vec<PublicSurfaceMergeControlControlsViolation>,
) {
    if derived_confirmation_locus != disclosure.posture
        || claims_provider_confirmed != disclosure.is_provider_confirmed
    {
        violations
            .push(PublicSurfaceMergeControlControlsViolation::ConfirmationLocusMisrepresented);
    }
    if (disclosure.is_local_estimate || disclosure.is_machine_generated)
        && claims_provider_confirmed
    {
        violations.push(
            PublicSurfaceMergeControlControlsViolation::LocalOrMachineClaimsProviderConfirmed,
        );
    }
    if disclosure.needs_local_estimate_note && local_estimate_note.trim().is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::LocalEstimateNoteMissing);
    }
    if disclosure.needs_machine_generated_note && machine_generated_note.trim().is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::MachineGeneratedNoteMissing);
    }
    if disclosure.needs_not_evaluated_note && not_evaluated_note.trim().is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::NotEvaluatedNoteMissing);
    }
    if disclosure.needs_stale_note && stale_note.trim().is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::StaleNoteMissing);
    }
    if let Some(token) = disclosure.governance_vocab {
        if !governance_state_vocab.contains(&token) {
            violations
                .push(PublicSurfaceMergeControlControlsViolation::GovernanceVocabMissingLocusToken);
        }
    }
}

/// Validates the axes shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_common_control(
    evidence_kind: DiffEvidenceKind,
    evidence_ref: &str,
    context_note: &str,
    declares_mandatory_actions: bool,
    downgrade_triggers: &[M5GovernanceComponentDowngradeTrigger],
    consumer_surfaces: &[M5GovernanceComponentConsumerSurface],
    invariants: ControlInvariants,
    violations: &mut Vec<PublicSurfaceMergeControlControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::ContextNoteMissing);
    }
    if evidence_kind.is_resolvable() && evidence_ref.trim().is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::DiffEvidenceRefMissing);
    }
    if !declares_mandatory_actions {
        violations.push(PublicSurfaceMergeControlControlsViolation::ComponentActionsIncomplete);
    }
    if downgrade_triggers.is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::DowngradeTriggersMissing);
    }
    if consumer_surfaces.is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::ConsumerSurfacesMissing);
    }
    if invariants.surface_class_or_blocker_hidden {
        violations.push(PublicSurfaceMergeControlControlsViolation::SurfaceClassOrBlockerHidden);
    }
    if invariants.local_estimate_masquerades_as_provider_confirmed {
        violations.push(
            PublicSurfaceMergeControlControlsViolation::LocalEstimateMasqueradesAsProviderConfirmed,
        );
    }
    if invariants.migration_or_gate_misrepresented {
        violations.push(PublicSurfaceMergeControlControlsViolation::MigrationOrGateMisrepresented);
    }
    if invariants.invents_alternate_state_label {
        violations.push(PublicSurfaceMergeControlControlsViolation::AlternateStateLabelInvented);
    }
}

fn validate_public_surface_diff_cards(
    packet: &PublicSurfaceMergeControlControlsPacket,
    violations: &mut Vec<PublicSurfaceMergeControlControlsViolation>,
) {
    if packet.public_surface_diff_cards.is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::PublicSurfaceDiffCardsMissing);
        return;
    }

    for card in &packet.public_surface_diff_cards {
        let locus = card.locus_disclosure();
        let change = card.change_disclosure();

        if card.card_id.trim().is_empty()
            || card.change_title_label.trim().is_empty()
            || card.surface_class_label.trim().is_empty()
            || card.stability_label.trim().is_empty()
            || card.delta_disclosure_label.trim().is_empty()
            || card.context_note.trim().is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations
                .push(PublicSurfaceMergeControlControlsViolation::PublicSurfaceDiffCardIncomplete);
        }
        if card.surface_classes.is_empty() {
            violations.push(PublicSurfaceMergeControlControlsViolation::SurfaceClassSetMissing);
        }
        if card.component != M5GovernanceComponent::PublicSurfaceDiffCard {
            violations.push(
                PublicSurfaceMergeControlControlsViolation::PublicSurfaceDiffCardWrongComponentClass,
            );
        }
        validate_shared_locus(
            &locus,
            card.derived_confirmation_locus,
            card.claims_provider_confirmed,
            &card.governance_state_vocab,
            &card.local_estimate_note,
            &card.machine_generated_note,
            &card.not_evaluated_note,
            &card.stale_note,
            violations,
        );
        if card.derived_surface_change != change.posture
            || card.claims_breaking != change.is_breaking
        {
            violations
                .push(PublicSurfaceMergeControlControlsViolation::SurfaceChangeMisrepresented);
        }
        // AC-1: a stable-contract change cannot hide inside ordinary review without an explicit
        // migration note and evidence link.
        if card.stability_class.is_stable()
            && change.requires_migration_note
            && (card.migration_note.trim().is_empty()
                || card.migration_evidence_ref.trim().is_empty())
        {
            violations.push(
                PublicSurfaceMergeControlControlsViolation::StableChangeMissingMigrationOrEvidence,
            );
        }
        if change.needs_breaking_note && card.breaking_note.trim().is_empty() {
            violations.push(PublicSurfaceMergeControlControlsViolation::BreakingNoteMissing);
        }
        if change.needs_deprecation_note && card.deprecation_note.trim().is_empty() {
            violations.push(PublicSurfaceMergeControlControlsViolation::DeprecationNoteMissing);
        }
        if change.needs_removal_note && card.removal_note.trim().is_empty() {
            violations.push(PublicSurfaceMergeControlControlsViolation::RemovalNoteMissing);
        }
        if !card
            .card_actions
            .contains(&PublicSurfaceDiffCardAction::OpenDiffEvidence)
        {
            violations
                .push(PublicSurfaceMergeControlControlsViolation::OpenDiffEvidenceActionMissing);
        }
        validate_common_control(
            card.diff_evidence_kind,
            &card.diff_evidence_ref,
            &card.context_note,
            card.declares_mandatory_actions(),
            &card.downgrade_triggers,
            &card.consumer_surfaces,
            ControlInvariants {
                surface_class_or_blocker_hidden: card.hides_surface_class_or_stability,
                local_estimate_masquerades_as_provider_confirmed: card
                    .lets_local_estimate_read_as_provider_confirmed,
                migration_or_gate_misrepresented: card
                    .lets_stable_breaking_change_hide_without_migration,
                invents_alternate_state_label: card.invents_alternate_state_label,
            },
            violations,
        );
    }

    let mut change_sources: BTreeSet<SurfaceChangeSource> = BTreeSet::new();
    let mut change_postures: BTreeSet<SurfaceChangePosture> = BTreeSet::new();
    let mut surface_classes: BTreeSet<PublicSurfaceClass> = BTreeSet::new();
    let mut stability_classes: BTreeSet<StabilityClass> = BTreeSet::new();
    let mut evidence_kinds: BTreeSet<DiffEvidenceKind> = BTreeSet::new();
    for card in &packet.public_surface_diff_cards {
        change_sources.insert(card.surface_change_source);
        change_postures.insert(card.change_disclosure().posture);
        for class in &card.surface_classes {
            surface_classes.insert(*class);
        }
        stability_classes.insert(card.stability_class);
        evidence_kinds.insert(card.diff_evidence_kind);
    }
    if SurfaceChangeSource::ALL
        .iter()
        .any(|source| !change_sources.contains(source))
    {
        violations
            .push(PublicSurfaceMergeControlControlsViolation::SurfaceChangeSourceCoverageMissing);
    }
    if SurfaceChangePosture::ALL
        .iter()
        .any(|posture| !change_postures.contains(posture))
    {
        violations
            .push(PublicSurfaceMergeControlControlsViolation::SurfaceChangePostureCoverageMissing);
    }
    // AC-1: the diff cards alone name every public surface a change can materially affect.
    if PublicSurfaceClass::ALL
        .iter()
        .any(|class| !surface_classes.contains(class))
    {
        violations
            .push(PublicSurfaceMergeControlControlsViolation::PublicSurfaceClassCoverageMissing);
    }
    if StabilityClass::ALL
        .iter()
        .any(|class| !stability_classes.contains(class))
    {
        violations.push(PublicSurfaceMergeControlControlsViolation::StabilityClassCoverageMissing);
    }
    if DiffEvidenceKind::ALL
        .iter()
        .any(|kind| !evidence_kinds.contains(kind))
    {
        violations
            .push(PublicSurfaceMergeControlControlsViolation::DiffEvidenceKindCoverageMissing);
    }
}

fn validate_merge_control_banners(
    packet: &PublicSurfaceMergeControlControlsPacket,
    violations: &mut Vec<PublicSurfaceMergeControlControlsViolation>,
) {
    if packet.merge_control_banners.is_empty() {
        violations.push(PublicSurfaceMergeControlControlsViolation::MergeControlBannersMissing);
        return;
    }

    let mut blocker_classes: BTreeSet<MergeBlockerClass> = BTreeSet::new();
    let mut bypass_policies: BTreeSet<BypassPolicyClass> = BTreeSet::new();
    let mut protection_states: BTreeSet<ProtectionState> = BTreeSet::new();

    for banner in &packet.merge_control_banners {
        let locus = banner.locus_disclosure();
        blocker_classes.insert(banner.blocker_class);
        bypass_policies.insert(banner.bypass_policy);
        protection_states.insert(banner.protection_state);

        if banner.banner_id.trim().is_empty()
            || banner.gate_title_label.trim().is_empty()
            || banner.required_checks_label.trim().is_empty()
            || banner.protection_state_label.trim().is_empty()
            || banner.bypass_policy_label.trim().is_empty()
            || banner.mergeability_label.trim().is_empty()
            || banner.export_parity_label.trim().is_empty()
            || banner.context_note.trim().is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations
                .push(PublicSurfaceMergeControlControlsViolation::MergeControlBannerIncomplete);
        }
        if banner.required_checks.is_empty() {
            violations.push(PublicSurfaceMergeControlControlsViolation::RequiredChecksMissing);
        }
        if banner.component != M5GovernanceComponent::MergeControlBanner {
            violations.push(
                PublicSurfaceMergeControlControlsViolation::MergeControlBannerWrongComponentClass,
            );
        }
        // AC-2: a guarded merge names the current gate honestly.
        if banner.blocker_class.is_blocking() && banner.blocker_reason_label.trim().is_empty() {
            violations.push(PublicSurfaceMergeControlControlsViolation::MergeBlockerReasonMissing);
        }
        validate_shared_locus(
            &locus,
            banner.derived_confirmation_locus,
            banner.claims_provider_confirmed,
            &banner.governance_state_vocab,
            &banner.local_estimate_note,
            &banner.machine_generated_note,
            &banner.not_evaluated_note,
            &banner.stale_note,
            violations,
        );
        if banner.claims_evaluated_here != locus.is_evaluated_here {
            violations
                .push(PublicSurfaceMergeControlControlsViolation::ConfirmationLocusMisrepresented);
        }
        if !locus.is_evaluated_here && banner.claims_evaluated_here {
            violations
                .push(PublicSurfaceMergeControlControlsViolation::NotEvaluatedClaimsEvaluated);
        }
        for check in &banner.required_checks {
            if check.check_label.trim().is_empty() {
                violations
                    .push(PublicSurfaceMergeControlControlsViolation::RequiredCheckIncomplete);
            }
        }
        // A merge banner binds no openable diff-evidence link of its own, but it must still name a
        // context note and declare its mandatory actions, triggers, and surfaces.
        validate_common_control(
            DiffEvidenceKind::NoDiffEvidence,
            "",
            &banner.context_note,
            banner.declares_mandatory_actions(),
            &banner.downgrade_triggers,
            &banner.consumer_surfaces,
            ControlInvariants {
                surface_class_or_blocker_hidden: banner.hides_blocker_reason_or_bypass_policy,
                local_estimate_masquerades_as_provider_confirmed: banner
                    .lets_local_estimate_read_as_provider_mergeable,
                migration_or_gate_misrepresented: banner
                    .names_generic_blocker_instead_of_current_gate,
                invents_alternate_state_label: banner.invents_alternate_state_label,
            },
            violations,
        );
    }

    if MergeBlockerClass::ALL
        .iter()
        .any(|class| !blocker_classes.contains(class))
    {
        violations
            .push(PublicSurfaceMergeControlControlsViolation::MergeBlockerClassCoverageMissing);
    }
    if BypassPolicyClass::ALL
        .iter()
        .any(|policy| !bypass_policies.contains(policy))
    {
        violations.push(PublicSurfaceMergeControlControlsViolation::BypassPolicyCoverageMissing);
    }
    if ProtectionState::ALL
        .iter()
        .any(|state| !protection_states.contains(state))
    {
        violations.push(PublicSurfaceMergeControlControlsViolation::ProtectionStateCoverageMissing);
    }
}

/// Validates that the union of both component vectors covers every locus source and posture.
fn validate_shared_coverage(
    packet: &PublicSurfaceMergeControlControlsPacket,
    violations: &mut Vec<PublicSurfaceMergeControlControlsViolation>,
) {
    let mut sources: BTreeSet<ConfirmationLocusSource> = BTreeSet::new();
    let mut postures: BTreeSet<ConfirmationLocusPosture> = BTreeSet::new();

    for card in &packet.public_surface_diff_cards {
        sources.insert(card.confirmation_locus_source);
        postures.insert(card.locus_disclosure().posture);
    }
    for banner in &packet.merge_control_banners {
        sources.insert(banner.confirmation_locus_source);
        postures.insert(banner.locus_disclosure().posture);
    }

    if ConfirmationLocusSource::ALL
        .iter()
        .any(|source| !sources.contains(source))
    {
        violations.push(PublicSurfaceMergeControlControlsViolation::LocusSourceCoverageMissing);
    }
    if ConfirmationLocusPosture::ALL
        .iter()
        .any(|posture| !postures.contains(posture))
    {
        violations.push(PublicSurfaceMergeControlControlsViolation::LocusPostureCoverageMissing);
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
