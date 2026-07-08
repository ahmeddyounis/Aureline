//! Surface certification of commit-graph-header, history-graph-row,
//! branch-comparison-chip, worktree-row, stash-entry, reflog-recovery-banner,
//! rebase-todo-row, sequence-editor-header, cherry-pick / revert review-sheet,
//! patch-apply review-sheet, conflict-checkpoint-card, and force-push
//! review-dialog truth on every claimed M5 Git-history surface.
//!
//! This module is the closing certification capstone over the twelve shared
//! Git-history and risky-mutation components frozen in
//! [`crate::freeze_the_m5_git_history_sequence_component_matrix`], implemented by the
//! commit-graph / history-graph / branch-comparison / worktree identity lane, the
//! stash-entry / reflog-recovery lane, the rebase-todo / sequence-editor lane, and
//! the cherry-pick / revert / patch-apply / conflict-checkpoint / force-push
//! mutation-review lane, adopted by the shared consumers in
//! [`crate::add_shared_history_sidebar_review_workspace_command_help_support_and_export_consumers_so_git_history_components_keep_ref_worktree_recovery_language_aligned`],
//! and proven across assistive, headless, and exported forms by
//! [`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_repo_topology_checkpoint_availability_or_provider_linked_recovery_truth_is_partial_or_stale_across_claimed_m5_git_history_components`].
//!
//! Where the implement lanes ship the components and the consumer lane proves ref /
//! worktree / recovery / verb parity, this lane certifies the release claim: that on
//! every claimed M5 Git-history surface — history sidebar, risky-mutation review
//! sheet, review workspace, help surface, support export, exported recovery packet,
//! headless CLI, and diagnostics — the same controlled component truth is presented
//! with no hidden ref / worktree / recovery drift. Each certified surface row scores
//! six certification axes ([`GitHistoryCertificationAxis`]): the visual, keyboard,
//! screen-reader, and CLI/export axes that every claim must always pass, the
//! degraded-state axis that narrows a claim when repo topology, checkpoint
//! availability, or provider-linked recovery truth weakens, and the
//! local-recovery-provenance axis that keeps the certification honest — a certified
//! surface never implies its provider-linked review state is fresh or its recovery
//! checkpoint is reachable.
//!
//! A surface earns [`GitHistorySurfaceClaimStatus::CertifiedParity`] only when its
//! certified claim equals its claimed claim, no axis narrows, and component truth is
//! preserved. It narrows to [`GitHistorySurfaceClaimStatus::NarrowedParity`] the
//! moment an axis narrows or the certified claim drops below the claimed one, and it
//! fails to [`GitHistorySurfaceClaimStatus::ParityBlocked`] whenever the exact ref /
//! worktree identity, dirty / shallow / partial topology, stash contents,
//! sequence-edit intent, patch / apply target, approval invalidation, or recovery
//! destination is flattened out of the export. That last rule is the delta of this
//! capstone: certification may narrow a claim but may never drop the component's
//! meaning.
//!
//! The packet references upstream component, consumer, and accessibility contracts by
//! id rather than embedding their content. Raw paths, raw object bytes, raw branch
//! names, raw patch / reflog / stash bodies, raw provider payloads, and credentials
//! stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-git-history-surface-certification.schema.json`](../../../../schemas/ui/m5-git-history-surface-certification.schema.json).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_git_history_sequence_component_matrix::M5GitHistoryComponent;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_repo_topology_checkpoint_availability_or_provider_linked_recovery_truth_is_partial_or_stale_across_claimed_m5_git_history_components::GitHistoryClaimTier;

/// Stable record-kind tag carried by [`GitHistoryCertificationPacket`].
pub const M5_GIT_HISTORY_CERTIFICATION_RECORD_KIND: &str =
    "m5_git_history_surface_certification_truth";

/// Schema version for Git-history surface certification records.
pub const M5_GIT_HISTORY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_GIT_HISTORY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ui/m5-git-history-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_GIT_HISTORY_CERTIFICATION_DOC_REF: &str =
    "docs/git/m5/certify_git_history_component_truth_on_every_claimed_m5_git_history_surface.md";

/// Repo-relative path of the frozen component matrix this certification builds on.
pub const M5_GIT_HISTORY_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-sequence-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this certification builds on.
pub const M5_GIT_HISTORY_CERTIFICATION_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-component-consumer.schema.json";

/// Repo-relative path of the accessibility / headless / export parity contract this certification builds on.
pub const M5_GIT_HISTORY_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-component-accessibility-parity.schema.json";

/// Repo-relative path of the commit-graph / history-graph / branch-comparison /
/// worktree identity component contract.
pub const M5_GIT_HISTORY_CERTIFICATION_IDENTITY_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-identity-component.schema.json";

/// Repo-relative path of the stash-entry / reflog-recovery component contract.
pub const M5_GIT_HISTORY_CERTIFICATION_STASH_RECOVERY_CONTRACT_REF: &str =
    "schemas/ui/m5-stash-reflog-recovery-component.schema.json";

/// Repo-relative path of the rebase-todo / sequence-editor component contract.
pub const M5_GIT_HISTORY_CERTIFICATION_SEQUENCE_EDIT_CONTRACT_REF: &str =
    "schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json";

/// Repo-relative path of the cherry-pick / revert / patch-apply / conflict-checkpoint
/// / force-push mutation-review component contract.
pub const M5_GIT_HISTORY_CERTIFICATION_MUTATION_REVIEW_CONTRACT_REF: &str =
    "schemas/ui/m5-git-mutation-review-recovery-component.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_GIT_HISTORY_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-git-history-surface-certification";

/// Repo-relative path of the release-proof support export (the canonical export).
pub const M5_GIT_HISTORY_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/release/m5-git-history-surface-certification-proof/support_export.json";

/// Repo-relative path of the release-proof certification matrix CSV.
pub const M5_GIT_HISTORY_CERTIFICATION_MATRIX_REF: &str =
    "artifacts/release/m5-git-history-surface-certification-proof/matrix.csv";

/// Repo-relative path of the release-proof report.
pub const M5_GIT_HISTORY_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/release/m5-git-history-surface-certification-proof/report.md";

/// Canonical component contract that a certified surface row must cite for a
/// component it presents.
///
/// Each of the twelve shared components resolves to the checked-in schema of the
/// implement lane that owns it: the commit-graph / history-graph / branch-comparison
/// / worktree identity lane, the stash-entry / reflog-recovery lane, the rebase-todo
/// / sequence-editor lane, or the cherry-pick / revert / patch-apply /
/// conflict-checkpoint / force-push mutation-review lane.
pub const fn certification_component_canonical_schema_ref(
    component: M5GitHistoryComponent,
) -> &'static str {
    match component {
        M5GitHistoryComponent::CommitGraphHeader
        | M5GitHistoryComponent::HistoryGraphRow
        | M5GitHistoryComponent::BranchComparisonChip
        | M5GitHistoryComponent::WorktreeRow => M5_GIT_HISTORY_CERTIFICATION_IDENTITY_CONTRACT_REF,
        M5GitHistoryComponent::StashEntry | M5GitHistoryComponent::ReflogRecoveryBanner => {
            M5_GIT_HISTORY_CERTIFICATION_STASH_RECOVERY_CONTRACT_REF
        }
        M5GitHistoryComponent::RebaseTodoRow | M5GitHistoryComponent::SequenceEditorHeader => {
            M5_GIT_HISTORY_CERTIFICATION_SEQUENCE_EDIT_CONTRACT_REF
        }
        M5GitHistoryComponent::CherryPickRevertReviewSheet
        | M5GitHistoryComponent::PatchApplyReviewSheet
        | M5GitHistoryComponent::ConflictCheckpointCard
        | M5GitHistoryComponent::ForcePushReviewDialog => {
            M5_GIT_HISTORY_CERTIFICATION_MUTATION_REVIEW_CONTRACT_REF
        }
    }
}

/// A claimed M5 Git-history surface whose component truth this packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GitHistoryCertifiedSurface {
    /// Desktop history sidebar (commit graph, history rows, branch comparison, worktrees).
    HistorySidebar,
    /// Risky-mutation review sheet (rebase, cherry-pick / revert, patch-apply, force-push).
    RiskyMutationSheet,
    /// Review workspace surface (history overlays inside review).
    ReviewWorkspace,
    /// Help / About Git-history surface.
    HelpGitSurface,
    /// Support export bundle.
    SupportExport,
    /// Exported recovery packet (offline / local-only recovery pack).
    ExportedRecoveryPacket,
    /// Headless CLI Git-history output.
    CliHeadless,
    /// Diagnostics Git-history surface.
    Diagnostics,
}

impl M5GitHistoryCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HistorySidebar,
        Self::RiskyMutationSheet,
        Self::ReviewWorkspace,
        Self::HelpGitSurface,
        Self::SupportExport,
        Self::ExportedRecoveryPacket,
        Self::CliHeadless,
        Self::Diagnostics,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HistorySidebar => "history_sidebar",
            Self::RiskyMutationSheet => "risky_mutation_sheet",
            Self::ReviewWorkspace => "review_workspace",
            Self::HelpGitSurface => "help_git_surface",
            Self::SupportExport => "support_export",
            Self::ExportedRecoveryPacket => "exported_recovery_packet",
            Self::CliHeadless => "cli_headless",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// A certification axis scored on every certified surface row.
///
/// The first four axes are always-on: a claimed component must always pass them on
/// every surface. [`DegradedState`](Self::DegradedState) narrows a claim when repo
/// topology, checkpoint availability, or provider-linked recovery truth weakens.
/// [`LocalRecoveryProvenance`](Self::LocalRecoveryProvenance) is the
/// certification-specific separation axis: it keeps the local-recovery-vs-provider
/// distinction explicit so a certified surface never implies its provider-linked
/// review state is fresh or its recovery checkpoint is reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryCertificationAxis {
    /// Visual rendering carries the controlled component truth.
    Visual,
    /// Keyboard reach and operation carry the controlled component truth.
    Keyboard,
    /// Screen-reader labelling carries the controlled component truth.
    ScreenReader,
    /// CLI and export forms carry the controlled component truth.
    CliExport,
    /// Degraded topology / checkpoint / provider-recovery state narrows the claim honestly.
    DegradedState,
    /// The local-recovery-vs-provider distinction stays explicit; certified never implies fresh.
    LocalRecoveryProvenance,
}

impl GitHistoryCertificationAxis {
    /// Every axis, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Visual,
        Self::Keyboard,
        Self::ScreenReader,
        Self::CliExport,
        Self::DegradedState,
        Self::LocalRecoveryProvenance,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::LocalRecoveryProvenance => "local_recovery_provenance",
        }
    }

    /// Whether this axis must always be certified on every claimed surface.
    pub const fn is_always_on(self) -> bool {
        matches!(
            self,
            Self::Visual | Self::Keyboard | Self::ScreenReader | Self::CliExport
        )
    }
}

/// The certification state of a single axis on a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryAxisCertificationState {
    /// The axis is fully certified on this surface.
    Certified,
    /// The axis is certified but narrowed (an honest fallback is disclosed).
    NarrowedCertified,
    /// The axis is not certified on this surface (it is honestly out of scope here).
    NotCertifiedHere,
}

impl GitHistoryAxisCertificationState {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedCertified => "narrowed_certified",
            Self::NotCertifiedHere => "not_certified_here",
        }
    }
}

/// The certification status a surface row earns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistorySurfaceClaimStatus {
    /// Green: certified claim equals claimed claim, no axis narrows, truth preserved.
    CertifiedParity,
    /// Yellow: certification is narrowed but component truth is preserved.
    NarrowedParity,
    /// Red: component truth was flattened out of this surface.
    ParityBlocked,
}

impl GitHistorySurfaceClaimStatus {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedParity => "certified_parity",
            Self::NarrowedParity => "narrowed_parity",
            Self::ParityBlocked => "parity_blocked",
        }
    }

    /// Whether the surface is fully certified (green).
    pub const fn is_green(self) -> bool {
        matches!(self, Self::CertifiedParity)
    }

    /// Whether the surface is blocked (red).
    pub const fn is_red(self) -> bool {
        matches!(self, Self::ParityBlocked)
    }
}

/// Downgrade trigger that can narrow a certified surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryCertificationDowngradeTrigger {
    /// Proof packet has gone stale relative to its freshness SLO.
    ProofStale,
    /// An upstream evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-linked review state is stale relative to local Git truth.
    ProviderReviewStateStale,
    /// Repo topology is shallow / partial / sparse; the history shown is incomplete.
    RepoTopologyPartial,
    /// No checkpoint exists; only reflog-only recovery is available.
    CheckpointRecoveryUnavailable,
    /// The surface is offline / local-only; provider handoff is unavailable.
    OfflineLocalOnly,
    /// A worktree divergence or dirty state is unresolved.
    WorktreeDivergenceUnresolved,
    /// Consumer or workspace trust narrowed.
    TrustNarrowing,
    /// An upstream dependency row narrowed.
    UpstreamDependencyNarrowed,
}

impl GitHistoryCertificationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::EvidencePacketInvalid,
        Self::PolicyBlocked,
        Self::ProviderReviewStateStale,
        Self::RepoTopologyPartial,
        Self::CheckpointRecoveryUnavailable,
        Self::OfflineLocalOnly,
        Self::WorktreeDivergenceUnresolved,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderReviewStateStale => "provider_review_state_stale",
            Self::RepoTopologyPartial => "repo_topology_partial",
            Self::CheckpointRecoveryUnavailable => "checkpoint_recovery_unavailable",
            Self::OfflineLocalOnly => "offline_local_only",
            Self::WorktreeDivergenceUnresolved => "worktree_divergence_unresolved",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Derives the certification status of a surface from its claims and axis narrowing.
///
/// Component truth is the hard gate: if the exact ref / worktree identity, topology
/// state, stash contents, sequence-edit intent, patch / apply target, approval
/// invalidation, or recovery destination is flattened, the surface is
/// [`GitHistorySurfaceClaimStatus::ParityBlocked`] regardless of the claim tiers.
/// Otherwise a certified claim below the claimed one, or any narrowed axis, narrows
/// the surface to [`GitHistorySurfaceClaimStatus::NarrowedParity`]; only a full,
/// un-narrowed claim earns [`GitHistorySurfaceClaimStatus::CertifiedParity`].
pub const fn derive_git_history_surface_claim_status(
    claimed: GitHistoryClaimTier,
    certified: GitHistoryClaimTier,
    component_truth_preserved: bool,
    has_narrowed_axes: bool,
) -> GitHistorySurfaceClaimStatus {
    if !component_truth_preserved {
        GitHistorySurfaceClaimStatus::ParityBlocked
    } else if certified.rank() < claimed.rank() || has_narrowed_axes {
        GitHistorySurfaceClaimStatus::NarrowedParity
    } else {
        GitHistorySurfaceClaimStatus::CertifiedParity
    }
}

/// One axis outcome on a certified surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryCertAxisOutcome {
    /// The certification axis scored.
    pub axis: GitHistoryCertificationAxis,
    /// The state the axis earned on this surface.
    pub state: GitHistoryAxisCertificationState,
    /// Human-readable note explaining the outcome (never empty).
    pub note: String,
}

/// One certified surface row: a claimed M5 Git-history surface and the component
/// truth it presents, scored across the six certification axes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryCertifiedSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// The claimed M5 Git-history surface.
    pub surface: M5GitHistoryCertifiedSurface,
    /// The shared components this surface presents (non-empty).
    pub components_present: Vec<M5GitHistoryComponent>,
    /// The claim tier the surface claims for its components.
    pub claimed_claim: GitHistoryClaimTier,
    /// The claim tier the certification actually earns.
    pub certified_claim: GitHistoryClaimTier,
    /// The certification status the surface earns.
    pub status: GitHistorySurfaceClaimStatus,
    /// Per-axis outcomes; must cover all six axes.
    pub axis_outcomes: Vec<GitHistoryCertAxisOutcome>,
    /// The axes that narrowed on this surface (subset of the axis outcomes).
    pub narrowed_axes: Vec<GitHistoryCertificationAxis>,
    /// The downgrade trigger disclosed when the surface narrows.
    pub downgrade_trigger: Option<GitHistoryCertificationDowngradeTrigger>,
    /// Delta invariant: the component's exact ref / worktree identity, topology,
    /// stash contents, sequence-edit intent, patch target, approval invalidation, and
    /// recovery destination truth is preserved (never flattened).
    pub component_truth_preserved: bool,
    /// Keyboard reach / operation label (never empty).
    pub keyboard_label: String,
    /// Screen-reader label (never empty).
    pub screen_reader_label: String,
    /// CLI enum token (never empty).
    pub cli_enum_token: String,
    /// Export enum token (never empty).
    pub export_enum_token: String,
    /// Human-readable explanation field (never empty).
    pub explanation_field: String,
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl GitHistoryCertifiedSurfaceRow {
    /// The status this row should carry, derived from its claims and narrowing.
    pub fn derived_status(&self) -> GitHistorySurfaceClaimStatus {
        derive_git_history_surface_claim_status(
            self.claimed_claim,
            self.certified_claim,
            self.component_truth_preserved,
            !self.narrowed_axes.is_empty(),
        )
    }

    /// Whether the recorded status matches the derived one.
    pub fn status_is_consistent(&self) -> bool {
        self.status == self.derived_status()
    }

    /// Whether every axis is scored on this row.
    pub fn covers_all_axes(&self) -> bool {
        GitHistoryCertificationAxis::ALL.iter().all(|axis| {
            self.axis_outcomes
                .iter()
                .any(|outcome| outcome.axis == *axis)
        })
    }

    /// Whether every parity / export field is present.
    pub fn parity_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether the certified claim stays at or below the claimed one.
    pub fn certified_claim_within_claimed(&self) -> bool {
        self.certified_claim.rank() <= self.claimed_claim.rank()
    }

    /// Whether the narrowed axes agree with the axis outcomes marked narrowed.
    pub fn narrowed_axes_consistent(&self) -> bool {
        let narrowed: BTreeSet<GitHistoryCertificationAxis> =
            self.narrowed_axes.iter().copied().collect();
        for outcome in &self.axis_outcomes {
            let marked_narrowed =
                outcome.state == GitHistoryAxisCertificationState::NarrowedCertified;
            if marked_narrowed != narrowed.contains(&outcome.axis) {
                return false;
            }
        }
        true
    }

    /// Whether this row cites the canonical matrix and each present component's schema.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_GIT_HISTORY_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF) {
            return false;
        }
        self.components_present.iter().all(|component| {
            refs.contains(certification_component_canonical_schema_ref(*component))
        })
    }
}

/// Aggregate certification summary across all surface rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryCertificationSummary {
    /// Total certified surface rows.
    pub total_rows: u32,
    /// Count of green (fully certified) surfaces.
    pub certified_count: u32,
    /// Count of yellow (narrowed) surfaces.
    pub narrowed_count: u32,
    /// Count of red (blocked) surfaces.
    pub blocked_count: u32,
    /// True when every surface preserves component truth (no red).
    pub all_rows_preserve_component_truth: bool,
    /// True when all eight claimed surfaces are covered.
    pub all_surfaces_covered: bool,
    /// True when all twelve shared components appear across the surfaces.
    pub all_components_covered: bool,
    /// Human-readable certification note.
    pub certification_note: String,
}

impl GitHistoryCertificationSummary {
    /// Recomputes the summary from a surface row set.
    pub fn from_rows(rows: &[GitHistoryCertifiedSurfaceRow]) -> Self {
        let mut certified = 0u32;
        let mut narrowed = 0u32;
        let mut blocked = 0u32;
        let mut seen_surfaces: BTreeSet<M5GitHistoryCertifiedSurface> = BTreeSet::new();
        let mut seen_components: BTreeSet<M5GitHistoryComponent> = BTreeSet::new();
        for row in rows {
            match row.status {
                GitHistorySurfaceClaimStatus::CertifiedParity => certified += 1,
                GitHistorySurfaceClaimStatus::NarrowedParity => narrowed += 1,
                GitHistorySurfaceClaimStatus::ParityBlocked => blocked += 1,
            }
            seen_surfaces.insert(row.surface);
            for component in &row.components_present {
                seen_components.insert(*component);
            }
        }
        let all_surfaces_covered = M5GitHistoryCertifiedSurface::ALL
            .iter()
            .all(|surface| seen_surfaces.contains(surface));
        let all_components_covered = M5GitHistoryComponent::ALL
            .iter()
            .all(|component| seen_components.contains(component));
        let all_preserve = blocked == 0;
        let certification_note = if all_preserve {
            format!(
                "{certified} surface(s) certified, {narrowed} narrowed; all preserve component truth"
            )
        } else {
            format!("{blocked} surface(s) blocked: component truth was flattened")
        };
        Self {
            total_rows: rows.len() as u32,
            certified_count: certified,
            narrowed_count: narrowed,
            blocked_count: blocked,
            all_rows_preserve_component_truth: all_preserve,
            all_surfaces_covered,
            all_components_covered,
            certification_note,
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryCertificationTrustReview {
    /// Every claimed surface presents the same controlled component truth.
    pub same_component_truth_on_every_surface: bool,
    /// Exact repo / worktree / ref identity stays explicit on every surface.
    pub ref_worktree_identity_explicit: bool,
    /// Dirty / shallow / partial topology state stays explicit, never flattened.
    pub topology_state_explicit: bool,
    /// Stash contents and sequence-edit intent stay explicit.
    pub stash_and_sequence_intent_explicit: bool,
    /// Patch / apply target and approval invalidation stay explicit.
    pub patch_target_and_approval_invalidation_explicit: bool,
    /// Provider-linked review freshness stays explicit; certified never implies fresh.
    pub certified_never_implies_fresh: bool,
    /// Recovery destination stays explicit and local-only recovery is never hidden.
    pub recovery_destination_explicit_local_kept: bool,
    /// Local-only continuation is preserved when provider recovery is degraded.
    pub local_continuation_preserved: bool,
    /// Certification narrows a claim rather than dropping the component's meaning.
    pub narrows_instead_of_dropping_meaning: bool,
    /// A surface that flattens component truth blocks its certification.
    pub flattened_truth_blocks_certification: bool,
}

impl GitHistoryCertificationTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.same_component_truth_on_every_surface
            && self.ref_worktree_identity_explicit
            && self.topology_state_explicit
            && self.stash_and_sequence_intent_explicit
            && self.patch_target_and_approval_invalidation_explicit
            && self.certified_never_implies_fresh
            && self.recovery_destination_explicit_local_kept
            && self.local_continuation_preserved
            && self.narrows_instead_of_dropping_meaning
            && self.flattened_truth_blocks_certification
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryCertificationConsumerProjection {
    /// History sidebar shows the certified component truth.
    pub history_sidebar_shows_certification: bool,
    /// Risky-mutation review sheet shows the certified component truth.
    pub risky_mutation_sheet_shows_certification: bool,
    /// Review workspace shows the certified component truth.
    pub review_workspace_shows_certification: bool,
    /// Help / About Git surface shows the certified component truth.
    pub help_git_surface_shows_certification: bool,
    /// Support export shows the certified component truth.
    pub support_export_shows_certification: bool,
    /// Exported recovery packet shows the certified component truth.
    pub exported_recovery_packet_shows_certification: bool,
    /// CLI / headless shows the certified component truth.
    pub cli_headless_shows_certification: bool,
    /// Diagnostics shows the certified component truth.
    pub diagnostics_shows_certification: bool,
    /// Narrowed surfaces are visibly labelled rather than silently downgraded.
    pub narrowed_surfaces_visibly_labelled: bool,
}

impl GitHistoryCertificationConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.history_sidebar_shows_certification
            && self.risky_mutation_sheet_shows_certification
            && self.review_workspace_shows_certification
            && self.help_git_surface_shows_certification
            && self.support_export_shows_certification
            && self.exported_recovery_packet_shows_certification
            && self.cli_headless_shows_certification
            && self.diagnostics_shows_certification
            && self.narrowed_surfaces_visibly_labelled
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// Per-surface observation fed to [`GitHistoryCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHistoryCertObservation {
    /// Surface the observation applies to.
    pub surface: M5GitHistoryCertifiedSurface,
    /// True when the surface's provider-linked recovery backing is currently fresh.
    pub provider_recovery_fresh: bool,
    /// True when the surface still preserves component truth.
    pub component_truth_preserved: bool,
}

/// Constructor input for [`GitHistoryCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHistoryCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<GitHistoryCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: GitHistoryCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: GitHistoryCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GitHistoryCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GitHistoryCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe Git-history surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryCertificationPacket {
    /// Record kind; must equal [`M5_GIT_HISTORY_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GIT_HISTORY_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<GitHistoryCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: GitHistoryCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: GitHistoryCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GitHistoryCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GitHistoryCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GitHistoryCertificationPacket {
    /// Builds a Git-history surface certification packet from stable-lane input.
    pub fn new(input: GitHistoryCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_GIT_HISTORY_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_GIT_HISTORY_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            surface_rows: input.surface_rows,
            summary: input.summary,
            downgrade_triggers: input.downgrade_triggers,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows surfaces whose provider-linked recovery backing is no longer fresh and
    /// blocks surfaces that flatten component truth, then recomputes the summary.
    ///
    /// This is the downgrade automation: a surface reported with a flattened component
    /// truth blocks (red); a still-green surface whose provider-linked recovery
    /// backing went stale narrows its recoverable-in-product claim to
    /// locally-recoverable, marks the local-recovery-provenance axis narrowed, and
    /// discloses the stale trigger. Observations for surfaces not present in the
    /// packet are ignored; surfaces without an observation are left unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[GitHistoryCertObservation]) {
        for row in &mut self.surface_rows {
            let Some(observation) = observations.iter().find(|obs| obs.surface == row.surface)
            else {
                continue;
            };
            if !observation.component_truth_preserved {
                row.component_truth_preserved = false;
            } else if !observation.provider_recovery_fresh
                && row.status == GitHistorySurfaceClaimStatus::CertifiedParity
            {
                if row.certified_claim.rank() > GitHistoryClaimTier::LocallyRecoverable.rank() {
                    row.certified_claim = GitHistoryClaimTier::LocallyRecoverable;
                }
                if !row
                    .narrowed_axes
                    .contains(&GitHistoryCertificationAxis::LocalRecoveryProvenance)
                {
                    row.narrowed_axes
                        .push(GitHistoryCertificationAxis::LocalRecoveryProvenance);
                }
                for outcome in &mut row.axis_outcomes {
                    if outcome.axis == GitHistoryCertificationAxis::LocalRecoveryProvenance {
                        outcome.state = GitHistoryAxisCertificationState::NarrowedCertified;
                        outcome.note =
                            "Provider-linked recovery went stale; the claim narrows to locally recoverable and the local-recovery-vs-provider distinction stays explicit"
                                .to_owned();
                    }
                }
                row.downgrade_trigger =
                    Some(GitHistoryCertificationDowngradeTrigger::ProviderReviewStateStale);
            }
            row.status = row.derived_status();
        }
        self.summary = GitHistoryCertificationSummary::from_rows(&self.surface_rows);
    }

    /// Validates the Git-history surface certification invariants.
    pub fn validate(&self) -> Vec<GitHistoryCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GIT_HISTORY_CERTIFICATION_RECORD_KIND {
            violations.push(GitHistoryCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GIT_HISTORY_CERTIFICATION_SCHEMA_VERSION {
            violations.push(GitHistoryCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GitHistoryCertificationViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GitHistoryCertificationViolation::DowngradeTriggersMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_summary(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(GitHistoryCertificationViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(GitHistoryCertificationViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GitHistoryCertificationViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("git-history certification packet serializes"),
        ) {
            violations.push(GitHistoryCertificationViolation::RawGitHistoryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("git-history certification packet serializes")
    }

    /// Deterministic certification matrix CSV for release proof.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n",
        );
        for row in &self.surface_rows {
            let narrowed = row
                .narrowed_axes
                .iter()
                .map(|axis| axis.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_claim.as_str(),
                row.certified_claim.as_str(),
                row.status.as_str(),
                narrowed,
                row.component_truth_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Git-History Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_rows,
            self.summary.certified_count,
            self.summary.narrowed_count,
            self.summary.blocked_count,
        ));
        out.push_str(&format!(
            "- All surfaces preserve component truth: {}\n",
            self.summary.all_rows_preserve_component_truth
        ));
        out.push_str(&format!("- Note: {}\n", self.summary.certification_note));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Certified surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: `{}` (claimed `{}`, certified `{}`)\n",
                row.surface.as_str(),
                row.row_id,
                row.status.as_str(),
                row.claimed_claim.as_str(),
                row.certified_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in Git-history certification export.
#[derive(Debug)]
pub enum GitHistoryCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GitHistoryCertificationViolation>),
}

impl fmt::Display for GitHistoryCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "git-history certification export parse failed: {error}"
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
                    "git-history certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GitHistoryCertificationArtifactError {}

/// Validation failures emitted by [`GitHistoryCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitHistoryCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No surface rows are present.
    SurfaceRowsMissing,
    /// A surface row is incomplete.
    RowIncomplete,
    /// A surface row lists no components.
    ComponentsMissingOnRow,
    /// A row is missing its keyboard label.
    KeyboardLabelMissing,
    /// A row is missing its screen-reader label.
    ScreenReaderLabelMissing,
    /// A row is missing its CLI enum token.
    CliEnumTokenMissing,
    /// A row is missing its export enum token.
    ExportEnumTokenMissing,
    /// A row is missing its explanation field.
    ExplanationFieldMissing,
    /// A row does not score all six certification axes.
    AxisCoverageMissing,
    /// An axis outcome is missing its explanatory note.
    AxisNoteMissing,
    /// A certified claim exceeds the claimed claim it certifies.
    CertifiedClaimExceedsClaimed,
    /// The recorded status does not agree with the derived one.
    StatusMismatch,
    /// The narrowed-axis list disagrees with the axis outcomes marked narrowed.
    NarrowedAxesInconsistent,
    /// A narrowed surface is missing its disclosed downgrade trigger.
    NarrowingWithoutTrigger,
    /// A surface flattened the component's ref / worktree / topology / stash / recovery truth.
    GitHistoryComponentTruthDropped,
    /// A row does not cite the canonical matrix and component contracts.
    CanonicalContractReferenceMissing,
    /// Not every claimed surface appears among the rows.
    SurfaceCoverageMissing,
    /// Not every shared component appears across the surfaces.
    ComponentCoverageMissing,
    /// The summary does not agree with the surface rows.
    SummaryMismatch,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// Export contains raw Git-history boundary material.
    RawGitHistoryMaterialInExport,
}

impl GitHistoryCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SurfaceRowsMissing => "surface_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::ComponentsMissingOnRow => "components_missing_on_row",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::AxisCoverageMissing => "axis_coverage_missing",
            Self::AxisNoteMissing => "axis_note_missing",
            Self::CertifiedClaimExceedsClaimed => "certified_claim_exceeds_claimed",
            Self::StatusMismatch => "status_mismatch",
            Self::NarrowedAxesInconsistent => "narrowed_axes_inconsistent",
            Self::NarrowingWithoutTrigger => "narrowing_without_trigger",
            Self::GitHistoryComponentTruthDropped => "git_history_component_truth_dropped",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::SurfaceCoverageMissing => "surface_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::SummaryMismatch => "summary_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RawGitHistoryMaterialInExport => "raw_git_history_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable Git-history certification export.
pub fn current_git_history_certification_export(
) -> Result<GitHistoryCertificationPacket, GitHistoryCertificationArtifactError> {
    let packet: GitHistoryCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-git-history-surface-certification-proof/support_export.json"
    )))
    .map_err(GitHistoryCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GitHistoryCertificationArtifactError::Validation(violations))
    }
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> GitHistoryCertificationTrustReview {
    GitHistoryCertificationTrustReview {
        same_component_truth_on_every_surface: true,
        ref_worktree_identity_explicit: true,
        topology_state_explicit: true,
        stash_and_sequence_intent_explicit: true,
        patch_target_and_approval_invalidation_explicit: true,
        certified_never_implies_fresh: true,
        recovery_destination_explicit_local_kept: true,
        local_continuation_preserved: true,
        narrows_instead_of_dropping_meaning: true,
        flattened_truth_blocks_certification: true,
    }
}

/// Canonical consumer projection block with every surface projecting certification truth.
pub fn canonical_consumer_projection() -> GitHistoryCertificationConsumerProjection {
    GitHistoryCertificationConsumerProjection {
        history_sidebar_shows_certification: true,
        risky_mutation_sheet_shows_certification: true,
        review_workspace_shows_certification: true,
        help_git_surface_shows_certification: true,
        support_export_shows_certification: true,
        exported_recovery_packet_shows_certification: true,
        cli_headless_shows_certification: true,
        diagnostics_shows_certification: true,
        narrowed_surfaces_visibly_labelled: true,
    }
}

/// Canonical source contract refs that every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_GIT_HISTORY_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_GIT_HISTORY_CERTIFICATION_DOC_REF.to_owned(),
        M5_GIT_HISTORY_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_CERTIFICATION_CONSUMER_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_CERTIFICATION_IDENTITY_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_CERTIFICATION_STASH_RECOVERY_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_CERTIFICATION_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_CERTIFICATION_MUTATION_REVIEW_CONTRACT_REF.to_owned(),
    ]
}

fn validate_source_contracts(
    packet: &GitHistoryCertificationPacket,
    violations: &mut Vec<GitHistoryCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GIT_HISTORY_CERTIFICATION_SCHEMA_REF,
        M5_GIT_HISTORY_CERTIFICATION_DOC_REF,
        M5_GIT_HISTORY_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF,
        M5_GIT_HISTORY_CERTIFICATION_CONSUMER_CONTRACT_REF,
        M5_GIT_HISTORY_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF,
        M5_GIT_HISTORY_CERTIFICATION_IDENTITY_CONTRACT_REF,
        M5_GIT_HISTORY_CERTIFICATION_STASH_RECOVERY_CONTRACT_REF,
        M5_GIT_HISTORY_CERTIFICATION_SEQUENCE_EDIT_CONTRACT_REF,
        M5_GIT_HISTORY_CERTIFICATION_MUTATION_REVIEW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GitHistoryCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &GitHistoryCertificationPacket,
    violations: &mut Vec<GitHistoryCertificationViolation>,
) {
    if packet.surface_rows.is_empty() {
        violations.push(GitHistoryCertificationViolation::SurfaceRowsMissing);
        return;
    }

    let mut seen_surfaces: BTreeSet<M5GitHistoryCertifiedSurface> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5GitHistoryComponent> = BTreeSet::new();

    for row in &packet.surface_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(GitHistoryCertificationViolation::RowIncomplete);
        }
        if row.components_present.is_empty() {
            violations.push(GitHistoryCertificationViolation::ComponentsMissingOnRow);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(GitHistoryCertificationViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(GitHistoryCertificationViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(GitHistoryCertificationViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(GitHistoryCertificationViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(GitHistoryCertificationViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_axes() {
            violations.push(GitHistoryCertificationViolation::AxisCoverageMissing);
        }
        if row
            .axis_outcomes
            .iter()
            .any(|outcome| outcome.note.trim().is_empty())
        {
            violations.push(GitHistoryCertificationViolation::AxisNoteMissing);
        }

        // AC2 core: a certified claim may never exceed the claim it certifies.
        if !row.certified_claim_within_claimed() {
            violations.push(GitHistoryCertificationViolation::CertifiedClaimExceedsClaimed);
        }

        if !row.narrowed_axes_consistent() {
            violations.push(GitHistoryCertificationViolation::NarrowedAxesInconsistent);
        }

        // A narrowed surface must disclose its downgrade trigger.
        if !row.narrowed_axes.is_empty() && row.downgrade_trigger.is_none() {
            violations.push(GitHistoryCertificationViolation::NarrowingWithoutTrigger);
        }

        // Delta: certification may narrow a claim but never drop component truth.
        if !row.component_truth_preserved {
            violations.push(GitHistoryCertificationViolation::GitHistoryComponentTruthDropped);
        }

        // The recorded status must agree with the derived one.
        if !row.status_is_consistent() {
            violations.push(GitHistoryCertificationViolation::StatusMismatch);
        }

        if !row.points_at_canonical_contracts() {
            violations.push(GitHistoryCertificationViolation::CanonicalContractReferenceMissing);
        }

        seen_surfaces.insert(row.surface);
        for component in &row.components_present {
            seen_components.insert(*component);
        }
    }

    for surface in M5GitHistoryCertifiedSurface::ALL {
        if !seen_surfaces.contains(&surface) {
            violations.push(GitHistoryCertificationViolation::SurfaceCoverageMissing);
            break;
        }
    }
    for component in M5GitHistoryComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(GitHistoryCertificationViolation::ComponentCoverageMissing);
            break;
        }
    }
}

fn validate_summary(
    packet: &GitHistoryCertificationPacket,
    violations: &mut Vec<GitHistoryCertificationViolation>,
) {
    let recomputed = GitHistoryCertificationSummary::from_rows(&packet.surface_rows);
    if recomputed != packet.summary {
        violations.push(GitHistoryCertificationViolation::SummaryMismatch);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
