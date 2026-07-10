//! Two reusable M5 protected-path governance components — the protected-path row and the ownership
//! card — so a user can tell *why* a file, package, or path is guarded more tightly, *who* owns the
//! resulting approval burden, whether the protection is advisory or authoritative, whether it is the
//! provider's final gate or a local estimate, whether owner backup coverage is present or missing,
//! and how ownership escalates, before they trust, merge, or escalate a governed change.
//!
//! Aureline's frozen protected-path governance component matrix
//! ([`crate::freeze_the_m5_protected_path_governance_component_matrix`]) names the protected-path
//! row and the ownership card as two governed component families and freezes their controlled
//! vocabulary — the one governance-state lexicon ([`M5GovernanceStateVocab`]): `advisory`,
//! `authoritative`, `covered`, `backup_missing`, `waived`, `expired`, `stale`,
//! `provider_authoritative`, and `local_estimate`. This module *implements* that contract as two
//! co-equal component vectors — a full protected-path row and an ownership card — that reuse the one
//! frozen lexicon and share one enforcement resolver so an advisory owner hint can never masquerade
//! as provider-authoritative enforcement, and a missing backup owner can never render as clean
//! coverage.
//!
//! The module has two derived resolvers:
//!
//! * [`resolve_enforcement_posture`] — takes an owner-enforcement source and derives the exact
//!   enforcement posture (provider-authoritative, locally-authoritative, advisory-only, or a local
//!   estimate), whether the posture is authoritative, whether it is the provider's authoritative
//!   gate, and which frozen governance-state token it maps to — so a bridge, hint, or heuristic
//!   owner match can never read as the provider's final enforcement. Both the protected-path row and
//!   the ownership card use it, so their enforcement language stays one truth.
//! * [`resolve_owner_coverage_posture`] — takes an owner-coverage source and derives the exact
//!   coverage posture (covered with backup, backup missing, unresolved, or policy-hidden), the
//!   continuity state, whether the coverage is clean, and which frozen governance-state token it maps
//!   to — so missing backup coverage, unresolved ownership, or policy-hidden owner state degrades
//!   explicitly instead of rendering as clean coverage.
//!
//! A single controls packet — [`ProtectedPathOwnershipControlsPacket`] — binds one vector of
//! protected-path rows and one vector of ownership cards to the same enforcement, coverage,
//! freshness, and escalation vocabulary, so protection reason, owner source, advisory-versus-
//! authoritative state, backup coverage, and escalation continuity stay explicit across the review-
//! workspace, owner-coverage, governance, shiproom, CLI, and support-export consumers.
//!
//! The governance component ([`M5GovernanceComponent`]), governance-state vocabulary
//! ([`M5GovernanceStateVocab`]), downgrade trigger
//! ([`M5GovernanceComponentDowngradeTrigger`]), rollback posture
//! ([`M5GovernanceComponentRollbackPosture`]), and consumer surface
//! ([`M5GovernanceComponentConsumerSurface`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: the owner-enforcement source, the derived enforcement posture, the owner-coverage
//! source, the derived coverage posture and continuity state, the owner-source class, the evaluation
//! freshness state, the rule-source kind, and the bounded row and card actions. No M5 governed
//! surface invents a second protected-path or ownership grammar.
//!
//! Raw CODEOWNERS bodies, raw manifests, raw provider payloads, person-specific private contact
//! detail, credentials, and secrets stay outside the export boundary; every owner is carried only as
//! an export-safe role alias, and every rule and escalation target is carried only as an opaque,
//! export-safe reference.

#[cfg(test)]
mod tests;

// The governance component family, the frozen governance-state lexicon, and the downgrade /
// rollback / consumer vocabularies are frozen once, in the protected-path governance component
// matrix. This lane reuses them verbatim so it never invents a parallel protected-path or ownership
// vocabulary.
pub use crate::freeze_the_m5_protected_path_governance_component_matrix::{
    M5GovernanceComponent, M5GovernanceComponentConsumerSurface,
    M5GovernanceComponentDowngradeTrigger, M5GovernanceComponentRollbackPosture,
    M5GovernanceStateVocab, M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ProtectedPathOwnershipControlsPacket`].
pub const PROTECTED_PATH_OWNERSHIP_CONTROLS_RECORD_KIND: &str =
    "implement_protected_path_rows_and_ownership_cards_with_protection_reason_owner_source_advisory_versus_authoritative_state_backup_coverage_and_escalation_continuity";

/// Schema version for M5 protected-path-row / ownership-card control records.
pub const PROTECTED_PATH_OWNERSHIP_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const PROTECTED_PATH_OWNERSHIP_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-protected-path-ownership-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const PROTECTED_PATH_OWNERSHIP_CONTROLS_DOC_REF: &str =
    "docs/review/m5/implement_protected_path_rows_and_ownership_cards.md";

/// Repo-relative path of the protected fixture directory.
pub const PROTECTED_PATH_OWNERSHIP_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-protected-path-ownership-controls";

/// Repo-relative path of the checked support-export artifact.
pub const PROTECTED_PATH_OWNERSHIP_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-protected-path-ownership-controls-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PROTECTED_PATH_OWNERSHIP_CONTROLS_SUMMARY_REF: &str =
    "artifacts/release/m5-protected-path-ownership-controls-proof/summary.md";

// ---- shared enforcement vocabulary --------------------------------------

/// The source an owner-enforcement signal comes from, before it is resolved into a posture.
///
/// This is the honest input to [`resolve_enforcement_posture`]: it names whether the protection or
/// ownership is enforced by the provider, enforced by a local manifest, only an advisory local hint,
/// or a local heuristic / inferred match — so an advisory hint or a local estimate can never be
/// asserted as the provider's authoritative gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerEnforcementSource {
    /// Provider-enforced branch / path protection.
    ProviderBranchProtection,
    /// Provider-resolved CODEOWNERS ownership.
    ProviderResolvedCodeowners,
    /// A local ownership / protected-path manifest that Aureline enforces locally.
    LocalManifestEnforced,
    /// A local ownership / protected-path manifest that is only an advisory hint.
    LocalManifestAdvisory,
    /// A local heuristic path match, not an enforced rule.
    LocalHeuristicMatch,
    /// An owner inferred from recent authorship, not a recorded assignment.
    InferredFromAuthorship,
}

impl OwnerEnforcementSource {
    /// Every enforcement source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderBranchProtection,
        Self::ProviderResolvedCodeowners,
        Self::LocalManifestEnforced,
        Self::LocalManifestAdvisory,
        Self::LocalHeuristicMatch,
        Self::InferredFromAuthorship,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderBranchProtection => "provider_branch_protection",
            Self::ProviderResolvedCodeowners => "provider_resolved_codeowners",
            Self::LocalManifestEnforced => "local_manifest_enforced",
            Self::LocalManifestAdvisory => "local_manifest_advisory",
            Self::LocalHeuristicMatch => "local_heuristic_match",
            Self::InferredFromAuthorship => "inferred_from_authorship",
        }
    }
}

/// Derived enforcement posture a protected-path row or ownership card may present.
///
/// This is the advisory-versus-authoritative honesty axis: the posture is derived from the frozen
/// enforcement source, never asserted, so an advisory hint or local estimate can never present as
/// the provider's authoritative gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementPosture {
    /// The provider authoritatively enforces the protection or ownership.
    ProviderAuthoritative,
    /// Aureline authoritatively enforces the protection or ownership locally.
    LocallyAuthoritative,
    /// The signal is only an advisory hint, not enforced.
    AdvisoryOnly,
    /// The signal is a local estimate, not a provider-confirmed or enforced fact.
    LocalEstimate,
}

impl EnforcementPosture {
    /// Every enforcement posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProviderAuthoritative,
        Self::LocallyAuthoritative,
        Self::AdvisoryOnly,
        Self::LocalEstimate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::LocallyAuthoritative => "locally_authoritative",
            Self::AdvisoryOnly => "advisory_only",
            Self::LocalEstimate => "local_estimate",
        }
    }

    /// True when the posture is authoritatively enforced (provider or local), not advisory.
    pub const fn is_authoritative(self) -> bool {
        matches!(
            self,
            Self::ProviderAuthoritative | Self::LocallyAuthoritative
        )
    }

    /// True only when the provider is the authoritative enforcer.
    pub const fn is_provider_authoritative(self) -> bool {
        matches!(self, Self::ProviderAuthoritative)
    }

    /// True when the posture is only an advisory hint.
    pub const fn is_advisory(self) -> bool {
        matches!(self, Self::AdvisoryOnly)
    }

    /// True when the posture is a local estimate.
    pub const fn is_local_estimate(self) -> bool {
        matches!(self, Self::LocalEstimate)
    }

    /// The frozen governance-state token this posture must render under.
    pub const fn governance_vocab(self) -> M5GovernanceStateVocab {
        match self {
            Self::ProviderAuthoritative => M5GovernanceStateVocab::ProviderAuthoritative,
            Self::LocallyAuthoritative => M5GovernanceStateVocab::Authoritative,
            Self::AdvisoryOnly => M5GovernanceStateVocab::Advisory,
            Self::LocalEstimate => M5GovernanceStateVocab::LocalEstimate,
        }
    }
}

/// Enforcement disclosures a component must carry, derived from the owner-enforcement source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnforcementDisclosure {
    /// The derived enforcement posture this component may present.
    pub posture: EnforcementPosture,
    /// Whether the protection or ownership is authoritatively enforced.
    pub is_authoritative: bool,
    /// Whether the provider is the authoritative enforcer.
    pub is_provider_authoritative: bool,
    /// Whether the signal is only an advisory hint.
    pub is_advisory: bool,
    /// Whether the signal is a local estimate.
    pub is_local_estimate: bool,
    /// Whether the component must carry an explicit advisory note.
    pub needs_advisory_note: bool,
    /// Whether the component must carry an explicit local-estimate note.
    pub needs_local_estimate_note: bool,
    /// The frozen governance-state token this posture must render under.
    pub governance_vocab: M5GovernanceStateVocab,
}

/// Resolves the enforcement posture a protected-path row or ownership card may present.
///
/// A `provider_branch_protection` or `provider_resolved_codeowners` source is provider-authoritative;
/// a `local_manifest_enforced` source is locally authoritative; a `local_manifest_advisory` source
/// is advisory-only; and a `local_heuristic_match` or `inferred_from_authorship` source is a local
/// estimate — so an advisory hint or a local estimate can never read as the provider's final
/// enforcement.
pub fn resolve_enforcement_posture(source: OwnerEnforcementSource) -> EnforcementDisclosure {
    use EnforcementPosture as Posture;
    use OwnerEnforcementSource as Src;

    let posture = match source {
        Src::ProviderBranchProtection | Src::ProviderResolvedCodeowners => {
            Posture::ProviderAuthoritative
        }
        Src::LocalManifestEnforced => Posture::LocallyAuthoritative,
        Src::LocalManifestAdvisory => Posture::AdvisoryOnly,
        Src::LocalHeuristicMatch | Src::InferredFromAuthorship => Posture::LocalEstimate,
    };

    EnforcementDisclosure {
        posture,
        is_authoritative: posture.is_authoritative(),
        is_provider_authoritative: posture.is_provider_authoritative(),
        is_advisory: posture.is_advisory(),
        is_local_estimate: posture.is_local_estimate(),
        needs_advisory_note: posture.is_advisory(),
        needs_local_estimate_note: posture.is_local_estimate(),
        governance_vocab: posture.governance_vocab(),
    }
}

// ---- owner-coverage vocabulary ------------------------------------------

/// The source an owner-coverage signal comes from, before it is resolved into a posture.
///
/// This is the honest input to [`resolve_owner_coverage_posture`]: it names whether the guarded path
/// has a resolved primary and backup owner, only a primary owner with no backup, an unresolved
/// owner, or an owner hidden by policy — so missing backup, unresolved ownership, or policy-hidden
/// owner state can never render as clean coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerCoverageSource {
    /// Both a primary and a backup owner are resolved from a local manifest.
    PrimaryAndBackupResolved,
    /// The provider resolves a primary and backup owner.
    ProviderResolvedCovered,
    /// Only a primary owner is resolved; backup coverage is missing.
    PrimaryOnlyBackupMissing,
    /// The owner could not be resolved for the guarded path.
    OwnerUnresolved,
    /// The owner is hidden by policy on this build.
    PolicyHiddenOwner,
}

impl OwnerCoverageSource {
    /// Every coverage source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PrimaryAndBackupResolved,
        Self::ProviderResolvedCovered,
        Self::PrimaryOnlyBackupMissing,
        Self::OwnerUnresolved,
        Self::PolicyHiddenOwner,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryAndBackupResolved => "primary_and_backup_resolved",
            Self::ProviderResolvedCovered => "provider_resolved_covered",
            Self::PrimaryOnlyBackupMissing => "primary_only_backup_missing",
            Self::OwnerUnresolved => "owner_unresolved",
            Self::PolicyHiddenOwner => "policy_hidden_owner",
        }
    }
}

/// Derived owner-coverage posture an ownership card may present.
///
/// This is the AC-pinned coverage-honesty axis: only [`OwnerCoveragePosture::CoveredWithBackup`] is
/// clean coverage — every other posture degrades explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerCoveragePosture {
    /// A primary and a backup owner are both covered.
    CoveredWithBackup,
    /// A primary owner is covered but backup coverage is missing.
    BackupMissing,
    /// The owner is unresolved.
    Unresolved,
    /// The owner is hidden by policy.
    PolicyHidden,
}

impl OwnerCoveragePosture {
    /// Every coverage posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CoveredWithBackup,
        Self::BackupMissing,
        Self::Unresolved,
        Self::PolicyHidden,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoveredWithBackup => "covered_with_backup",
            Self::BackupMissing => "backup_missing",
            Self::Unresolved => "unresolved",
            Self::PolicyHidden => "policy_hidden",
        }
    }

    /// True only when the guarded path has clean primary-and-backup coverage.
    pub const fn is_clean_coverage(self) -> bool {
        matches!(self, Self::CoveredWithBackup)
    }

    /// The frozen governance-state token this posture must render under. Clean coverage renders as
    /// `covered`; every degraded posture renders as `backup_missing` so it never reads as covered.
    pub const fn governance_vocab(self) -> M5GovernanceStateVocab {
        match self {
            Self::CoveredWithBackup => M5GovernanceStateVocab::Covered,
            Self::BackupMissing | Self::Unresolved | Self::PolicyHidden => {
                M5GovernanceStateVocab::BackupMissing
            }
        }
    }
}

/// Derived owner-continuity state an ownership card may present — whether ownership continues
/// cleanly, is degraded by a missing backup, is unresolved, or is limited by policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerContinuityState {
    /// Ownership continues cleanly across a primary and a backup.
    Continuous,
    /// Ownership is degraded because backup coverage is missing.
    DegradedBackupMissing,
    /// Ownership is broken because the owner is unresolved.
    UnresolvedContinuity,
    /// Ownership continuity is limited because the owner is policy-hidden.
    PolicyLimited,
}

impl OwnerContinuityState {
    /// Every continuity state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Continuous,
        Self::DegradedBackupMissing,
        Self::UnresolvedContinuity,
        Self::PolicyLimited,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continuous => "continuous",
            Self::DegradedBackupMissing => "degraded_backup_missing",
            Self::UnresolvedContinuity => "unresolved_continuity",
            Self::PolicyLimited => "policy_limited",
        }
    }

    /// True only when ownership continues cleanly.
    pub const fn is_continuous(self) -> bool {
        matches!(self, Self::Continuous)
    }
}

/// Coverage disclosures an ownership card must carry, derived from the owner-coverage source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnerCoverageDisclosure {
    /// The derived coverage posture this card may present.
    pub posture: OwnerCoveragePosture,
    /// The derived continuity state this card may present.
    pub continuity_state: OwnerContinuityState,
    /// Whether the guarded path has clean primary-and-backup coverage.
    pub is_clean_coverage: bool,
    /// Whether the card must carry an explicit backup-missing note.
    pub needs_backup_missing_note: bool,
    /// Whether the card must carry an explicit unresolved-owner note.
    pub needs_unresolved_note: bool,
    /// Whether the card must carry an explicit policy-hidden note.
    pub needs_policy_hidden_note: bool,
    /// The frozen governance-state token this coverage must render under.
    pub governance_vocab: M5GovernanceStateVocab,
}

/// Resolves the owner-coverage posture and continuity state an ownership card may present.
///
/// A `primary_and_backup_resolved` or `provider_resolved_covered` source is covered with backup and
/// continuous; a `primary_only_backup_missing` source degrades to `backup_missing` and a degraded
/// continuity; an `owner_unresolved` source degrades to `unresolved`; and a `policy_hidden_owner`
/// source degrades to `policy_hidden` — so missing backup, unresolved ownership, or policy-hidden
/// owner state can never render as clean coverage.
pub fn resolve_owner_coverage_posture(source: OwnerCoverageSource) -> OwnerCoverageDisclosure {
    use OwnerContinuityState as Continuity;
    use OwnerCoveragePosture as Posture;
    use OwnerCoverageSource as Src;

    let posture = match source {
        Src::PrimaryAndBackupResolved | Src::ProviderResolvedCovered => Posture::CoveredWithBackup,
        Src::PrimaryOnlyBackupMissing => Posture::BackupMissing,
        Src::OwnerUnresolved => Posture::Unresolved,
        Src::PolicyHiddenOwner => Posture::PolicyHidden,
    };
    let continuity_state = match posture {
        Posture::CoveredWithBackup => Continuity::Continuous,
        Posture::BackupMissing => Continuity::DegradedBackupMissing,
        Posture::Unresolved => Continuity::UnresolvedContinuity,
        Posture::PolicyHidden => Continuity::PolicyLimited,
    };

    OwnerCoverageDisclosure {
        posture,
        continuity_state,
        is_clean_coverage: posture.is_clean_coverage(),
        needs_backup_missing_note: matches!(posture, Posture::BackupMissing),
        needs_unresolved_note: matches!(posture, Posture::Unresolved),
        needs_policy_hidden_note: matches!(posture, Posture::PolicyHidden),
        governance_vocab: posture.governance_vocab(),
    }
}

// ---- protected-path-specific vocabulary ---------------------------------

/// The class an owner source belongs to, so a card names *where* its owner came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerSourceClass {
    /// A CODEOWNERS entry.
    CodeownersEntry,
    /// A DRI registry entry.
    DriRegistry,
    /// A local ownership manifest.
    OwnershipManifest,
    /// An owner inferred from recent authorship.
    InferredAuthorship,
    /// The owner source could not be resolved.
    Unresolved,
}

impl OwnerSourceClass {
    /// Every owner-source class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CodeownersEntry,
        Self::DriRegistry,
        Self::OwnershipManifest,
        Self::InferredAuthorship,
        Self::Unresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeownersEntry => "codeowners_entry",
            Self::DriRegistry => "dri_registry",
            Self::OwnershipManifest => "ownership_manifest",
            Self::InferredAuthorship => "inferred_authorship",
            Self::Unresolved => "unresolved",
        }
    }
}

/// The freshness of a protected-path evaluation, so a stale or never-evaluated protection signal
/// never reads as currently evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationFreshnessState {
    /// The protection was evaluated against the current head.
    CurrentlyEvaluated,
    /// The evaluation was imported from another environment.
    Imported,
    /// The evaluation is stale relative to the current head.
    Stale,
    /// The protection has never been evaluated.
    NeverEvaluated,
    /// Freshness is unknown.
    Unknown,
}

impl EvaluationFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CurrentlyEvaluated,
        Self::Imported,
        Self::Stale,
        Self::NeverEvaluated,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentlyEvaluated => "currently_evaluated",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::NeverEvaluated => "never_evaluated",
            Self::Unknown => "unknown",
        }
    }

    /// True when the freshness signal must carry an explicit not-current note.
    pub const fn needs_note(self) -> bool {
        !matches!(self, Self::CurrentlyEvaluated)
    }
}

/// The kind of stable rule source a protected-path row can open, so a guarded path always names the
/// exact governing rule the user can reopen — never an anonymous lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleSourceKind {
    /// A CODEOWNERS rule.
    CodeownersRule,
    /// A protected-path policy rule.
    ProtectedPathPolicy,
    /// A provider branch-protection rule.
    BranchProtectionRule,
    /// A local ownership / protected-path manifest entry.
    ManifestEntry,
    /// No rule source is bound (the row names that it routes nowhere).
    NoRuleSource,
}

impl RuleSourceKind {
    /// Every rule-source kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CodeownersRule,
        Self::ProtectedPathPolicy,
        Self::BranchProtectionRule,
        Self::ManifestEntry,
        Self::NoRuleSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeownersRule => "codeowners_rule",
            Self::ProtectedPathPolicy => "protected_path_policy",
            Self::BranchProtectionRule => "branch_protection_rule",
            Self::ManifestEntry => "manifest_entry",
            Self::NoRuleSource => "no_rule_source",
        }
    }

    /// True when this kind names a resolvable rule source.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoRuleSource)
    }
}

/// One keyboard-complete default action a protected-path row offers.
///
/// `OpenRuleSource`, `InspectEnforcementAuthority`, and `ReviewProtectionReason` are always offered
/// so the exact governing rule, the enforcement authority, and the protection reason stay inspectable
/// before a user trusts the guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedPathRowAction {
    /// Open the exact governing rule source (always available).
    OpenRuleSource,
    /// Inspect the enforcement authority (always available).
    InspectEnforcementAuthority,
    /// Review the protection reason (always available).
    ReviewProtectionReason,
    /// Inspect the owner source.
    InspectOwnerSource,
    /// Review the evaluation freshness.
    ReviewEvaluationFreshness,
    /// Copy the path or pattern.
    CopyPathPattern,
}

impl ProtectedPathRowAction {
    /// Every protected-path-row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenRuleSource,
        Self::InspectEnforcementAuthority,
        Self::ReviewProtectionReason,
        Self::InspectOwnerSource,
        Self::ReviewEvaluationFreshness,
        Self::CopyPathPattern,
    ];

    /// The default actions every keyboard-complete protected-path row must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenRuleSource,
        Self::InspectEnforcementAuthority,
        Self::ReviewProtectionReason,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRuleSource => "open_rule_source",
            Self::InspectEnforcementAuthority => "inspect_enforcement_authority",
            Self::ReviewProtectionReason => "review_protection_reason",
            Self::InspectOwnerSource => "inspect_owner_source",
            Self::ReviewEvaluationFreshness => "review_evaluation_freshness",
            Self::CopyPathPattern => "copy_path_pattern",
        }
    }
}

/// One keyboard-complete default action an ownership card offers.
///
/// `InspectOwnerSource`, `ReviewBackupCoverage`, and `OpenEscalationPath` are always offered so the
/// owner source, the backup coverage, and the escalation path stay inspectable before a user relies
/// on the ownership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipCardAction {
    /// Inspect the owner source (always available).
    InspectOwnerSource,
    /// Review the backup coverage (always available).
    ReviewBackupCoverage,
    /// Open the escalation path (always available).
    OpenEscalationPath,
    /// Review the continuity state.
    ReviewContinuityState,
    /// Inspect the enforcement authority.
    InspectEnforcementAuthority,
    /// Copy the export-safe owner aliases.
    CopyOwnerAliases,
}

impl OwnershipCardAction {
    /// Every ownership-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectOwnerSource,
        Self::ReviewBackupCoverage,
        Self::OpenEscalationPath,
        Self::ReviewContinuityState,
        Self::InspectEnforcementAuthority,
        Self::CopyOwnerAliases,
    ];

    /// The default actions every keyboard-complete ownership card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::InspectOwnerSource,
        Self::ReviewBackupCoverage,
        Self::OpenEscalationPath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOwnerSource => "inspect_owner_source",
            Self::ReviewBackupCoverage => "review_backup_coverage",
            Self::OpenEscalationPath => "open_escalation_path",
            Self::ReviewContinuityState => "review_continuity_state",
            Self::InspectEnforcementAuthority => "inspect_enforcement_authority",
            Self::CopyOwnerAliases => "copy_owner_aliases",
        }
    }
}

// ---- component structs ---------------------------------------------------

/// A protected-path row naming its path or pattern, protection reason, owner-source label, advisory-
/// versus-authoritative enforcement, evaluation freshness, and the exact rule source it can open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedPathRow {
    /// Frozen component this control implements; must be `protected_path_row`.
    pub component: M5GovernanceComponent,
    /// Stable row id.
    pub row_id: String,
    /// Path or pattern label; required and non-empty.
    pub path_label: String,
    /// Protection reason label; required and non-empty so why the path is guarded is explicit.
    pub protection_reason_label: String,
    /// Owner-source label; required and non-empty so who owns the approval burden is explicit.
    pub owner_source_label: String,
    /// Owner-enforcement source, resolved into the enforcement posture.
    pub enforcement_source: OwnerEnforcementSource,
    /// Derived enforcement posture (must equal the resolved posture).
    pub derived_enforcement_posture: EnforcementPosture,
    /// Whether the row claims authoritative enforcement (must equal derived truth).
    pub claims_authoritative_enforcement: bool,
    /// Whether the row claims provider-authoritative enforcement (must equal derived truth).
    pub claims_provider_authoritative: bool,
    /// Frozen governance-state vocabulary this row renders (must include the derived posture token).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Advisory note; required when the enforcement is advisory-only.
    pub advisory_note: String,
    /// Local-estimate note; required when the enforcement is a local estimate.
    pub local_estimate_note: String,
    /// Evaluation freshness state.
    pub evaluation_freshness: EvaluationFreshnessState,
    /// Evaluation freshness label; always required so how current the evaluation is stays explicit.
    pub evaluation_freshness_label: String,
    /// Stale-evaluation note; required when the freshness is not currently evaluated.
    pub stale_evaluation_note: String,
    /// Kind of stable rule source this row can open.
    pub rule_source_kind: RuleSourceKind,
    /// Opaque stable rule-source reference; required when the kind resolves.
    pub rule_source_ref: String,
    /// Context note; always required so the row names what to check before trusting the guard.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub row_actions: Vec<ProtectedPathRowAction>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its protection reason or owner source. MUST be `false`.
    pub hides_protection_reason_or_owner_source: bool,
    /// Hard invariant: never lets an advisory hint masquerade as authoritative. MUST be `false`.
    pub lets_advisory_masquerade_as_authoritative: bool,
    /// Hard invariant: never lets a local estimate read as provider-authoritative. MUST be `false`.
    pub lets_local_estimate_read_as_provider_authoritative: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ProtectedPathRow {
    /// Enforcement disclosures this row must carry, derived from the frozen source.
    pub fn enforcement_disclosure(&self) -> EnforcementDisclosure {
        resolve_enforcement_posture(self.enforcement_source)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ProtectedPathRowAction> = self.row_actions.iter().copied().collect();
        ProtectedPathRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

/// An ownership card naming its primary and backup owners as export-safe role aliases, owner-source
/// class, advisory-versus-authoritative enforcement, coverage posture, continuity state, and
/// escalation path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipCard {
    /// Frozen component this control implements; must be `ownership_card`.
    pub component: M5GovernanceComponent,
    /// Stable card id.
    pub card_id: String,
    /// Owned path or surface label; required and non-empty.
    pub owned_path_label: String,
    /// Primary owner alias; required and non-empty, an export-safe role alias not a person's name.
    pub primary_owner_alias: String,
    /// Backup owner alias; an export-safe role alias, empty only when backup coverage is missing.
    pub backup_owner_alias: String,
    /// Owner-source class.
    pub owner_source_class: OwnerSourceClass,
    /// Owner-source label; required and non-empty so who owns the approval burden is explicit.
    pub owner_source_label: String,
    /// Owner-enforcement source, resolved into the enforcement posture.
    pub enforcement_source: OwnerEnforcementSource,
    /// Derived enforcement posture (must equal the resolved posture).
    pub derived_enforcement_posture: EnforcementPosture,
    /// Whether the card claims authoritative enforcement (must equal derived truth).
    pub claims_authoritative_enforcement: bool,
    /// Whether the card claims provider-authoritative enforcement (must equal derived truth).
    pub claims_provider_authoritative: bool,
    /// Owner-coverage source, resolved into the coverage posture and continuity state.
    pub coverage_source: OwnerCoverageSource,
    /// Derived coverage posture (must equal the resolved posture).
    pub derived_coverage_posture: OwnerCoveragePosture,
    /// Derived continuity state (must equal the resolved state).
    pub derived_continuity_state: OwnerContinuityState,
    /// Whether the card claims clean coverage (must equal derived truth).
    pub claims_clean_coverage: bool,
    /// Frozen governance-state vocabulary this card renders (must include the derived tokens).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Advisory note; required when the enforcement is advisory-only.
    pub advisory_note: String,
    /// Local-estimate note; required when the enforcement is a local estimate.
    pub local_estimate_note: String,
    /// Backup-missing note; required when backup coverage is missing.
    pub backup_missing_note: String,
    /// Unresolved-owner note; required when the owner is unresolved.
    pub unresolved_owner_note: String,
    /// Policy-hidden note; required when the owner is hidden by policy.
    pub policy_hidden_note: String,
    /// Escalation path label; always required so how ownership escalates stays explicit.
    pub escalation_path_label: String,
    /// Escalation boundary note; always required so escalation is a labeled handoff with a return
    /// path.
    pub escalation_boundary_note: String,
    /// Context note; always required so the card names what to check before relying on the owner.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub card_actions: Vec<OwnershipCardAction>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must project this card.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its owner source or coverage. MUST be `false`.
    pub hides_owner_source_or_coverage: bool,
    /// Hard invariant: never lets an advisory hint masquerade as authoritative. MUST be `false`.
    pub lets_advisory_masquerade_as_authoritative: bool,
    /// Hard invariant: never presents missing backup, unresolved, or policy-hidden owner state as
    /// clean coverage. MUST be `false`.
    pub presents_missing_backup_as_clean_coverage: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl OwnershipCard {
    /// Enforcement disclosures this card must carry, derived from the frozen source.
    pub fn enforcement_disclosure(&self) -> EnforcementDisclosure {
        resolve_enforcement_posture(self.enforcement_source)
    }

    /// Coverage disclosures this card must carry, derived from the frozen source.
    pub fn coverage_disclosure(&self) -> OwnerCoverageDisclosure {
        resolve_owner_coverage_posture(self.coverage_source)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<OwnershipCardAction> = self.card_actions.iter().copied().collect();
        OwnershipCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

// ---- review blocks -------------------------------------------------------

/// First-glance protected-path / ownership review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedPathOwnershipReview {
    /// The protected-path row names why the path is protected and how it is enforced.
    pub protected_path_row_shows_reason_and_enforcement: bool,
    /// The protected-path row names its owner source.
    pub protected_path_row_shows_owner_source: bool,
    /// The protected-path row offers an open-rule-source action.
    pub protected_path_row_offers_open_rule_source: bool,
    /// The ownership card names its primary and backup owners and owner source.
    pub ownership_card_shows_owners_and_source: bool,
    /// The ownership card names its coverage, continuity, and escalation path.
    pub ownership_card_shows_coverage_and_escalation: bool,
    /// The ownership card carries export-safe role aliases, never person-specific contact detail.
    pub ownership_card_uses_export_safe_role_aliases: bool,
    /// Enforcement posture is derived from state, never asserted.
    pub enforcement_posture_derived_never_asserted: bool,
    /// An advisory hint is never shown as authoritative enforcement.
    pub advisory_never_shown_as_authoritative: bool,
    /// A local estimate is never shown as provider-authoritative.
    pub local_estimate_never_shown_as_provider_authoritative: bool,
    /// Missing backup, unresolved, or policy-hidden owner state is never shown as clean coverage.
    pub missing_backup_never_shown_as_covered: bool,
    /// Evaluation freshness stays explicit.
    pub evaluation_freshness_always_explicit: bool,
    /// Every guarded path names one stable rule source it can open.
    pub every_guarded_path_names_stable_rule_source: bool,
    /// Escalation is a labeled handoff with a return path.
    pub escalation_handoff_always_explicit: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ProtectedPathOwnershipReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.protected_path_row_shows_reason_and_enforcement
            && self.protected_path_row_shows_owner_source
            && self.protected_path_row_offers_open_rule_source
            && self.ownership_card_shows_owners_and_source
            && self.ownership_card_shows_coverage_and_escalation
            && self.ownership_card_uses_export_safe_role_aliases
            && self.enforcement_posture_derived_never_asserted
            && self.advisory_never_shown_as_authoritative
            && self.local_estimate_never_shown_as_provider_authoritative
            && self.missing_backup_never_shown_as_covered
            && self.evaluation_freshness_always_explicit
            && self.every_guarded_path_names_stable_rule_source
            && self.escalation_handoff_always_explicit
            && self.no_surface_invents_alternate_state_label
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedPathOwnershipConsumerProjection {
    /// The review-workspace surface reads a single canonical source.
    pub review_workspace_reads_single_source: bool,
    /// The owner-coverage panel reads a single canonical source.
    pub owner_coverage_panel_reads_single_source: bool,
    /// The governance and shiproom surfaces read a single canonical source.
    pub governance_and_shiproom_read_single_source: bool,
    /// Protection reason and owner source are visible before a user trusts the guard.
    pub reason_and_owner_visible_before_trust: bool,
    /// Coverage and escalation are visible before a user relies on the owner.
    pub coverage_and_escalation_visible_before_trust: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl ProtectedPathOwnershipConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_reads_single_source
            && self.owner_coverage_panel_reads_single_source
            && self.governance_and_shiproom_read_single_source
            && self.reason_and_owner_visible_before_trust
            && self.coverage_and_escalation_visible_before_trust
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedPathOwnershipProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ProtectedPathOwnershipControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedPathOwnershipControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Protected-path rows.
    pub protected_path_rows: Vec<ProtectedPathRow>,
    /// Ownership cards.
    pub ownership_cards: Vec<OwnershipCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Protected-path / ownership review block.
    pub review: ProtectedPathOwnershipReview,
    /// Consumer projection block.
    pub consumer_projection: ProtectedPathOwnershipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ProtectedPathOwnershipProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe protected-path-row / ownership-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedPathOwnershipControlsPacket {
    /// Record kind; must equal [`PROTECTED_PATH_OWNERSHIP_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PROTECTED_PATH_OWNERSHIP_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Protected-path rows.
    pub protected_path_rows: Vec<ProtectedPathRow>,
    /// Ownership cards.
    pub ownership_cards: Vec<OwnershipCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Protected-path / ownership review block.
    pub review: ProtectedPathOwnershipReview,
    /// Consumer projection block.
    pub consumer_projection: ProtectedPathOwnershipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ProtectedPathOwnershipProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ProtectedPathOwnershipControlsPacket {
    /// Builds a protected-path-row / ownership-card controls packet from stable-lane input.
    pub fn new(input: ProtectedPathOwnershipControlsPacketInput) -> Self {
        Self {
            record_kind: PROTECTED_PATH_OWNERSHIP_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: PROTECTED_PATH_OWNERSHIP_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            protected_path_rows: input.protected_path_rows,
            ownership_cards: input.ownership_cards,
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

    /// Validates the protected-path-row / ownership-card control invariants.
    pub fn validate(&self) -> Vec<ProtectedPathOwnershipControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PROTECTED_PATH_OWNERSHIP_CONTROLS_RECORD_KIND {
            violations.push(ProtectedPathOwnershipControlsViolation::WrongRecordKind);
        }
        if self.schema_version != PROTECTED_PATH_OWNERSHIP_CONTROLS_SCHEMA_VERSION {
            violations.push(ProtectedPathOwnershipControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ProtectedPathOwnershipControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ProtectedPathOwnershipControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ProtectedPathOwnershipControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_protected_path_rows(self, &mut violations);
        validate_ownership_cards(self, &mut violations);
        validate_shared_coverage(self, &mut violations);

        if !self.review.all_hold() {
            violations.push(ProtectedPathOwnershipControlsViolation::ReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ProtectedPathOwnershipControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ProtectedPathOwnershipControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("protected-path ownership controls packet serializes"),
        ) {
            violations.push(ProtectedPathOwnershipControlsViolation::RawMaterialInExport);
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
            .expect("protected-path ownership controls packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let advisory_or_estimate = self
            .protected_path_rows
            .iter()
            .filter(|row| {
                let disclosure = row.enforcement_disclosure();
                disclosure.is_advisory || disclosure.is_local_estimate
            })
            .count();
        let degraded_coverage = self
            .ownership_cards
            .iter()
            .filter(|card| !card.coverage_disclosure().is_clean_coverage)
            .count();

        let mut out = String::new();
        out.push_str("# Protected-path rows and ownership cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Protected-path rows: {} ({} advisory or local estimate)\n",
            self.protected_path_rows.len(),
            advisory_or_estimate
        ));
        out.push_str(&format!(
            "- Ownership cards: {} ({} not clean coverage)\n",
            self.ownership_cards.len(),
            degraded_coverage
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Protected-path rows\n\n");
        for row in &self.protected_path_rows {
            let disclosure = row.enforcement_disclosure();
            out.push_str(&format!(
                "- **{}** — reason `{}`, enforcement `{}` → `{}`, freshness `{}`, rule source `{}`\n",
                row.path_label,
                row.protection_reason_label,
                row.enforcement_source.as_str(),
                disclosure.posture.as_str(),
                row.evaluation_freshness.as_str(),
                row.rule_source_kind.as_str(),
            ));
        }

        out.push_str("\n## Ownership cards\n\n");
        for card in &self.ownership_cards {
            let enforcement = card.enforcement_disclosure();
            let coverage = card.coverage_disclosure();
            out.push_str(&format!(
                "- **{}** — owner `{}`, source `{}`, enforcement `{}`, coverage `{}`, continuity `{}`\n",
                card.owned_path_label,
                card.primary_owner_alias,
                card.owner_source_class.as_str(),
                enforcement.posture.as_str(),
                coverage.posture.as_str(),
                coverage.continuity_state.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in protected-path-ownership controls export.
#[derive(Debug)]
pub enum ProtectedPathOwnershipControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ProtectedPathOwnershipControlsViolation>),
}

impl fmt::Display for ProtectedPathOwnershipControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "protected-path ownership controls export parse failed: {error}"
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
                    "protected-path ownership controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ProtectedPathOwnershipControlsArtifactError {}

/// Validation failures emitted by [`ProtectedPathOwnershipControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtectedPathOwnershipControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No protected-path rows are present.
    ProtectedPathRowsMissing,
    /// A protected-path row is incomplete.
    ProtectedPathRowIncomplete,
    /// A protected-path row carries the wrong frozen component class.
    ProtectedPathRowWrongComponentClass,
    /// No ownership cards are present.
    OwnershipCardsMissing,
    /// An ownership card is incomplete.
    OwnershipCardIncomplete,
    /// An ownership card carries the wrong frozen component class.
    OwnershipCardWrongComponentClass,
    /// A component misrepresents its derived enforcement posture.
    EnforcementPostureMisrepresented,
    /// An advisory component claims authoritative enforcement.
    AdvisoryClaimsAuthoritative,
    /// A local-estimate component claims provider-authoritative enforcement.
    LocalEstimateClaimsProviderAuthoritative,
    /// An advisory component does not name its advisory posture.
    AdvisoryNoteMissing,
    /// A local-estimate component does not name its local estimate.
    LocalEstimateNoteMissing,
    /// A component's governance vocabulary omits its derived enforcement token.
    GovernanceVocabMissingEnforcementToken,
    /// A protected-path row does not name its evaluation freshness.
    EvaluationFreshnessLabelMissing,
    /// A stale protected-path row does not name its stale evaluation.
    StaleEvaluationNoteMissing,
    /// A protected-path row does not offer an open-rule-source action.
    OpenRuleSourceActionMissing,
    /// An ownership card misrepresents its derived coverage posture or continuity state.
    CoveragePostureMisrepresented,
    /// An ownership card presents missing backup, unresolved, or policy-hidden owner as covered.
    MissingBackupPresentedAsCovered,
    /// A backup-missing ownership card does not name its missing backup.
    BackupMissingNoteMissing,
    /// An unresolved-owner ownership card does not name its unresolved owner.
    UnresolvedOwnerNoteMissing,
    /// A policy-hidden ownership card does not name its policy-hidden owner.
    PolicyHiddenNoteMissing,
    /// An ownership card's governance vocabulary omits its derived coverage token.
    GovernanceVocabMissingCoverageToken,
    /// An ownership card carries person-specific contact detail instead of a role alias.
    PersonContactDetailInAlias,
    /// An ownership card does not name its escalation path.
    EscalationPathMissing,
    /// A component names a rule source but not its stable reference.
    RuleSourceRefMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component omits a mandatory action.
    ComponentActionsIncomplete,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare any consumer surface.
    ConsumerSurfacesMissing,
    /// A component does not carry a governance-state vocabulary.
    GovernanceStateVocabMissing,
    /// The components do not cover every owner-enforcement source.
    EnforcementSourceCoverageMissing,
    /// The components do not cover every derived enforcement posture.
    EnforcementPostureCoverageMissing,
    /// The ownership cards do not cover every owner-coverage source.
    CoverageSourceCoverageMissing,
    /// The ownership cards do not cover every derived coverage posture.
    CoveragePostureCoverageMissing,
    /// The ownership cards do not cover every continuity state.
    ContinuityStateCoverageMissing,
    /// The protected-path rows do not cover every evaluation freshness state.
    FreshnessStateCoverageMissing,
    /// The protected-path rows do not cover every rule-source kind.
    RuleSourceKindCoverageMissing,
    /// A component hides its protection reason, owner source, or coverage.
    ReasonOwnerOrCoverageHidden,
    /// A component lets an advisory hint masquerade as authoritative.
    AdvisoryMasqueradesAsAuthoritative,
    /// A component lets a local estimate read as provider-authoritative, or missing backup as
    /// covered.
    LocalEstimateOrMissingBackupMisrepresented,
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

impl ProtectedPathOwnershipControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ProtectedPathRowsMissing => "protected_path_rows_missing",
            Self::ProtectedPathRowIncomplete => "protected_path_row_incomplete",
            Self::ProtectedPathRowWrongComponentClass => "protected_path_row_wrong_component_class",
            Self::OwnershipCardsMissing => "ownership_cards_missing",
            Self::OwnershipCardIncomplete => "ownership_card_incomplete",
            Self::OwnershipCardWrongComponentClass => "ownership_card_wrong_component_class",
            Self::EnforcementPostureMisrepresented => "enforcement_posture_misrepresented",
            Self::AdvisoryClaimsAuthoritative => "advisory_claims_authoritative",
            Self::LocalEstimateClaimsProviderAuthoritative => {
                "local_estimate_claims_provider_authoritative"
            }
            Self::AdvisoryNoteMissing => "advisory_note_missing",
            Self::LocalEstimateNoteMissing => "local_estimate_note_missing",
            Self::GovernanceVocabMissingEnforcementToken => {
                "governance_vocab_missing_enforcement_token"
            }
            Self::EvaluationFreshnessLabelMissing => "evaluation_freshness_label_missing",
            Self::StaleEvaluationNoteMissing => "stale_evaluation_note_missing",
            Self::OpenRuleSourceActionMissing => "open_rule_source_action_missing",
            Self::CoveragePostureMisrepresented => "coverage_posture_misrepresented",
            Self::MissingBackupPresentedAsCovered => "missing_backup_presented_as_covered",
            Self::BackupMissingNoteMissing => "backup_missing_note_missing",
            Self::UnresolvedOwnerNoteMissing => "unresolved_owner_note_missing",
            Self::PolicyHiddenNoteMissing => "policy_hidden_note_missing",
            Self::GovernanceVocabMissingCoverageToken => "governance_vocab_missing_coverage_token",
            Self::PersonContactDetailInAlias => "person_contact_detail_in_alias",
            Self::EscalationPathMissing => "escalation_path_missing",
            Self::RuleSourceRefMissing => "rule_source_ref_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::ComponentActionsIncomplete => "component_actions_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::GovernanceStateVocabMissing => "governance_state_vocab_missing",
            Self::EnforcementSourceCoverageMissing => "enforcement_source_coverage_missing",
            Self::EnforcementPostureCoverageMissing => "enforcement_posture_coverage_missing",
            Self::CoverageSourceCoverageMissing => "coverage_source_coverage_missing",
            Self::CoveragePostureCoverageMissing => "coverage_posture_coverage_missing",
            Self::ContinuityStateCoverageMissing => "continuity_state_coverage_missing",
            Self::FreshnessStateCoverageMissing => "freshness_state_coverage_missing",
            Self::RuleSourceKindCoverageMissing => "rule_source_kind_coverage_missing",
            Self::ReasonOwnerOrCoverageHidden => "reason_owner_or_coverage_hidden",
            Self::AdvisoryMasqueradesAsAuthoritative => "advisory_masquerades_as_authoritative",
            Self::LocalEstimateOrMissingBackupMisrepresented => {
                "local_estimate_or_missing_backup_misrepresented"
            }
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable protected-path-ownership controls export.
///
/// This is the first real consumer of the protected-path / ownership component lane: a review-
/// workspace, owner-coverage, governance, shiproom, or support-export surface calls it to ingest the
/// canonical components rather than cloning governance text.
///
/// # Errors
///
/// Returns [`ProtectedPathOwnershipControlsArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_protected_path_ownership_controls_export(
) -> Result<ProtectedPathOwnershipControlsPacket, ProtectedPathOwnershipControlsArtifactError> {
    let packet: ProtectedPathOwnershipControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-protected-path-ownership-controls-proof/support_export.json"
    )))
    .map_err(ProtectedPathOwnershipControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ProtectedPathOwnershipControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ProtectedPathOwnershipControlsPacket,
    violations: &mut Vec<ProtectedPathOwnershipControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PROTECTED_PATH_OWNERSHIP_CONTROLS_SCHEMA_REF,
        PROTECTED_PATH_OWNERSHIP_CONTROLS_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ProtectedPathOwnershipControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

/// The four hard-invariant bools every component maps into the shared check.
struct ControlInvariants {
    reason_owner_or_coverage_hidden: bool,
    advisory_masquerades_as_authoritative: bool,
    local_estimate_or_missing_backup_misrepresented: bool,
    invents_alternate_state_label: bool,
}

/// Validates the enforcement posture, notes, and cross-checks shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_shared_enforcement(
    disclosure: &EnforcementDisclosure,
    derived_enforcement_posture: EnforcementPosture,
    claims_authoritative_enforcement: bool,
    claims_provider_authoritative: bool,
    governance_state_vocab: &[M5GovernanceStateVocab],
    advisory_note: &str,
    local_estimate_note: &str,
    violations: &mut Vec<ProtectedPathOwnershipControlsViolation>,
) {
    if governance_state_vocab.is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::GovernanceStateVocabMissing);
    }
    if derived_enforcement_posture != disclosure.posture
        || claims_authoritative_enforcement != disclosure.is_authoritative
        || claims_provider_authoritative != disclosure.is_provider_authoritative
    {
        violations.push(ProtectedPathOwnershipControlsViolation::EnforcementPostureMisrepresented);
    }
    if disclosure.is_advisory && claims_authoritative_enforcement {
        violations.push(ProtectedPathOwnershipControlsViolation::AdvisoryClaimsAuthoritative);
    }
    if disclosure.is_local_estimate && claims_provider_authoritative {
        violations.push(
            ProtectedPathOwnershipControlsViolation::LocalEstimateClaimsProviderAuthoritative,
        );
    }
    if disclosure.needs_advisory_note && advisory_note.trim().is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::AdvisoryNoteMissing);
    }
    if disclosure.needs_local_estimate_note && local_estimate_note.trim().is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::LocalEstimateNoteMissing);
    }
    if !governance_state_vocab.contains(&disclosure.governance_vocab) {
        violations
            .push(ProtectedPathOwnershipControlsViolation::GovernanceVocabMissingEnforcementToken);
    }
}

/// Validates the axes shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_common_control(
    rule_source_kind: RuleSourceKind,
    rule_source_ref: &str,
    context_note: &str,
    declares_mandatory_actions: bool,
    downgrade_triggers: &[M5GovernanceComponentDowngradeTrigger],
    consumer_surfaces: &[M5GovernanceComponentConsumerSurface],
    invariants: ControlInvariants,
    violations: &mut Vec<ProtectedPathOwnershipControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::ContextNoteMissing);
    }
    if rule_source_kind.is_resolvable() && rule_source_ref.trim().is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::RuleSourceRefMissing);
    }
    if !declares_mandatory_actions {
        violations.push(ProtectedPathOwnershipControlsViolation::ComponentActionsIncomplete);
    }
    if downgrade_triggers.is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::DowngradeTriggersMissing);
    }
    if consumer_surfaces.is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::ConsumerSurfacesMissing);
    }
    if invariants.reason_owner_or_coverage_hidden {
        violations.push(ProtectedPathOwnershipControlsViolation::ReasonOwnerOrCoverageHidden);
    }
    if invariants.advisory_masquerades_as_authoritative {
        violations
            .push(ProtectedPathOwnershipControlsViolation::AdvisoryMasqueradesAsAuthoritative);
    }
    if invariants.local_estimate_or_missing_backup_misrepresented {
        violations.push(
            ProtectedPathOwnershipControlsViolation::LocalEstimateOrMissingBackupMisrepresented,
        );
    }
    if invariants.invents_alternate_state_label {
        violations.push(ProtectedPathOwnershipControlsViolation::AlternateStateLabelInvented);
    }
}

fn validate_protected_path_rows(
    packet: &ProtectedPathOwnershipControlsPacket,
    violations: &mut Vec<ProtectedPathOwnershipControlsViolation>,
) {
    if packet.protected_path_rows.is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::ProtectedPathRowsMissing);
        return;
    }

    for row in &packet.protected_path_rows {
        let disclosure = row.enforcement_disclosure();

        if row.row_id.trim().is_empty()
            || row.path_label.trim().is_empty()
            || row.protection_reason_label.trim().is_empty()
            || row.owner_source_label.trim().is_empty()
            || row.evaluation_freshness_label.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ProtectedPathOwnershipControlsViolation::ProtectedPathRowIncomplete);
        }
        if row.component != M5GovernanceComponent::ProtectedPathRow {
            violations
                .push(ProtectedPathOwnershipControlsViolation::ProtectedPathRowWrongComponentClass);
        }
        validate_shared_enforcement(
            &disclosure,
            row.derived_enforcement_posture,
            row.claims_authoritative_enforcement,
            row.claims_provider_authoritative,
            &row.governance_state_vocab,
            &row.advisory_note,
            &row.local_estimate_note,
            violations,
        );
        if row.evaluation_freshness.needs_note() && row.stale_evaluation_note.trim().is_empty() {
            violations.push(ProtectedPathOwnershipControlsViolation::StaleEvaluationNoteMissing);
        }
        if !row
            .row_actions
            .contains(&ProtectedPathRowAction::OpenRuleSource)
        {
            violations.push(ProtectedPathOwnershipControlsViolation::OpenRuleSourceActionMissing);
        }
        validate_common_control(
            row.rule_source_kind,
            &row.rule_source_ref,
            &row.context_note,
            row.declares_mandatory_actions(),
            &row.downgrade_triggers,
            &row.consumer_surfaces,
            ControlInvariants {
                reason_owner_or_coverage_hidden: row.hides_protection_reason_or_owner_source,
                advisory_masquerades_as_authoritative: row
                    .lets_advisory_masquerade_as_authoritative,
                local_estimate_or_missing_backup_misrepresented: row
                    .lets_local_estimate_read_as_provider_authoritative,
                invents_alternate_state_label: row.invents_alternate_state_label,
            },
            violations,
        );
    }

    let mut freshness: BTreeSet<EvaluationFreshnessState> = BTreeSet::new();
    let mut rule_kinds: BTreeSet<RuleSourceKind> = BTreeSet::new();
    for row in &packet.protected_path_rows {
        freshness.insert(row.evaluation_freshness);
        rule_kinds.insert(row.rule_source_kind);
    }
    for required in EvaluationFreshnessState::ALL {
        if !freshness.contains(&required) {
            violations.push(ProtectedPathOwnershipControlsViolation::FreshnessStateCoverageMissing);
            break;
        }
    }
    for required in RuleSourceKind::ALL {
        if !rule_kinds.contains(&required) {
            violations.push(ProtectedPathOwnershipControlsViolation::RuleSourceKindCoverageMissing);
            break;
        }
    }
}

fn validate_ownership_cards(
    packet: &ProtectedPathOwnershipControlsPacket,
    violations: &mut Vec<ProtectedPathOwnershipControlsViolation>,
) {
    if packet.ownership_cards.is_empty() {
        violations.push(ProtectedPathOwnershipControlsViolation::OwnershipCardsMissing);
        return;
    }

    let mut coverage_sources: BTreeSet<OwnerCoverageSource> = BTreeSet::new();
    let mut coverage_postures: BTreeSet<OwnerCoveragePosture> = BTreeSet::new();
    let mut continuities: BTreeSet<OwnerContinuityState> = BTreeSet::new();

    for card in &packet.ownership_cards {
        let enforcement = card.enforcement_disclosure();
        let coverage = card.coverage_disclosure();
        coverage_sources.insert(card.coverage_source);
        coverage_postures.insert(coverage.posture);
        continuities.insert(coverage.continuity_state);

        if card.card_id.trim().is_empty()
            || card.owned_path_label.trim().is_empty()
            || card.primary_owner_alias.trim().is_empty()
            || card.owner_source_label.trim().is_empty()
            || card.escalation_boundary_note.trim().is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(ProtectedPathOwnershipControlsViolation::OwnershipCardIncomplete);
        }
        if card.component != M5GovernanceComponent::OwnershipCard {
            violations
                .push(ProtectedPathOwnershipControlsViolation::OwnershipCardWrongComponentClass);
        }
        if alias_carries_contact_detail(&card.primary_owner_alias)
            || alias_carries_contact_detail(&card.backup_owner_alias)
        {
            violations.push(ProtectedPathOwnershipControlsViolation::PersonContactDetailInAlias);
        }
        validate_shared_enforcement(
            &enforcement,
            card.derived_enforcement_posture,
            card.claims_authoritative_enforcement,
            card.claims_provider_authoritative,
            &card.governance_state_vocab,
            &card.advisory_note,
            &card.local_estimate_note,
            violations,
        );
        if card.derived_coverage_posture != coverage.posture
            || card.derived_continuity_state != coverage.continuity_state
            || card.claims_clean_coverage != coverage.is_clean_coverage
        {
            violations.push(ProtectedPathOwnershipControlsViolation::CoveragePostureMisrepresented);
        }
        if !coverage.is_clean_coverage && card.claims_clean_coverage {
            violations
                .push(ProtectedPathOwnershipControlsViolation::MissingBackupPresentedAsCovered);
        }
        if coverage.needs_backup_missing_note && card.backup_missing_note.trim().is_empty() {
            violations.push(ProtectedPathOwnershipControlsViolation::BackupMissingNoteMissing);
        }
        if coverage.needs_unresolved_note && card.unresolved_owner_note.trim().is_empty() {
            violations.push(ProtectedPathOwnershipControlsViolation::UnresolvedOwnerNoteMissing);
        }
        if coverage.needs_policy_hidden_note && card.policy_hidden_note.trim().is_empty() {
            violations.push(ProtectedPathOwnershipControlsViolation::PolicyHiddenNoteMissing);
        }
        if !card
            .governance_state_vocab
            .contains(&coverage.governance_vocab)
        {
            violations
                .push(ProtectedPathOwnershipControlsViolation::GovernanceVocabMissingCoverageToken);
        }
        if card.escalation_path_label.trim().is_empty() {
            violations.push(ProtectedPathOwnershipControlsViolation::EscalationPathMissing);
        }
        // An ownership card binds no openable rule source of its own, but it must still name a
        // context note and declare its mandatory actions, triggers, and surfaces.
        validate_common_control(
            RuleSourceKind::NoRuleSource,
            "",
            &card.context_note,
            card.declares_mandatory_actions(),
            &card.downgrade_triggers,
            &card.consumer_surfaces,
            ControlInvariants {
                reason_owner_or_coverage_hidden: card.hides_owner_source_or_coverage,
                advisory_masquerades_as_authoritative: card
                    .lets_advisory_masquerade_as_authoritative,
                local_estimate_or_missing_backup_misrepresented: card
                    .presents_missing_backup_as_clean_coverage,
                invents_alternate_state_label: card.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in OwnerCoverageSource::ALL {
        if !coverage_sources.contains(&required) {
            violations.push(ProtectedPathOwnershipControlsViolation::CoverageSourceCoverageMissing);
            break;
        }
    }
    for required in OwnerCoveragePosture::ALL {
        if !coverage_postures.contains(&required) {
            violations
                .push(ProtectedPathOwnershipControlsViolation::CoveragePostureCoverageMissing);
            break;
        }
    }
    for required in OwnerContinuityState::ALL {
        if !continuities.contains(&required) {
            violations
                .push(ProtectedPathOwnershipControlsViolation::ContinuityStateCoverageMissing);
            break;
        }
    }
}

/// Validates that the union of both component vectors covers every enforcement source and posture.
fn validate_shared_coverage(
    packet: &ProtectedPathOwnershipControlsPacket,
    violations: &mut Vec<ProtectedPathOwnershipControlsViolation>,
) {
    let mut sources: BTreeSet<OwnerEnforcementSource> = BTreeSet::new();
    let mut postures: BTreeSet<EnforcementPosture> = BTreeSet::new();

    for row in &packet.protected_path_rows {
        sources.insert(row.enforcement_source);
        postures.insert(row.enforcement_disclosure().posture);
    }
    for card in &packet.ownership_cards {
        sources.insert(card.enforcement_source);
        postures.insert(card.enforcement_disclosure().posture);
    }

    for required in OwnerEnforcementSource::ALL {
        if !sources.contains(&required) {
            violations
                .push(ProtectedPathOwnershipControlsViolation::EnforcementSourceCoverageMissing);
            break;
        }
    }
    for required in EnforcementPosture::ALL {
        if !postures.contains(&required) {
            violations
                .push(ProtectedPathOwnershipControlsViolation::EnforcementPostureCoverageMissing);
            break;
        }
    }
}

/// Whether an owner alias carries person-specific private contact detail (an email address) rather
/// than an export-safe role alias.
fn alias_carries_contact_detail(alias: &str) -> bool {
    alias.contains('@')
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
