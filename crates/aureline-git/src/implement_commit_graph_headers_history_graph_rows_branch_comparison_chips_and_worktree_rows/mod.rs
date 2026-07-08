//! Commit-graph headers, history-graph rows, branch-comparison chips, and
//! worktree rows with working-context, divergence, dirty, shallow/partial, and
//! recovery truth.
//!
//! This module narrows the four identity/display components frozen in
//! [`crate::freeze_the_m5_git_history_sequence_component_matrix`] —
//! `commit_graph_header`, `history_graph_row`, `branch_comparison_chip`, and
//! `worktree_row` — into an implemented, export-safe row contract. Every
//! [`GitHistoryIdentityRow`] answers, from the component alone, which repo root
//! and checked-out ref it names, which worktree path it belongs to, how its ref
//! has diverged, whether the worktree is dirty or conflicted, whether the
//! checkout is shallow/partial/sparse, and — most importantly — whether the
//! component targets the current primary worktree, another linked worktree, or a
//! partial/shallow checkout. Multiple worktrees are never flattened into one
//! ambiguous branch list, and a non-current context never masquerades as the
//! current one.
//!
//! The working-context target is the core honesty axis: a row's claim to be the
//! current primary context is *derived* from the target, so a linked worktree or
//! a partial/shallow checkout can never silently pretend it is the current repo.
//! A dirty, conflicted, divergent, or detached row always keeps its
//! recovery/reflog availability explicit, and every separate working context
//! keeps its own semantics.
//!
//! The shared downgrade vocabulary ([`GitHistoryDowngradeState`]) and the shared
//! consumer surfaces ([`ComponentConsumerSurface`]) are reused directly from the
//! frozen matrix so downgrades and parity read the same everywhere. Raw paths,
//! raw object bytes, raw branch names, raw reflog/stash bodies, raw provider
//! payloads, and credentials stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-git-history-identity-component.schema.json`](../../../../schemas/ui/m5-git-history-identity-component.schema.json).
//! The contract doc is
//! [`docs/git/m5/implement_commit_graph_headers_history_graph_rows_branch_comparison_chips_and_worktree_rows.md`](../../../../docs/git/m5/implement_commit_graph_headers_history_graph_rows_branch_comparison_chips_and_worktree_rows.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-git-history-identity-components/`](../../../../fixtures/ui/m5-git-history-identity-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_git_history_sequence_component_matrix::{
    ComponentConsumerSurface, GitHistoryDowngradeState, M5GitHistoryComponent,
};

/// Stable record-kind tag carried by [`GitHistoryIdentityPacket`].
pub const GIT_HISTORY_IDENTITY_RECORD_KIND: &str =
    "git_history_identity_component_working_context_truth";

/// Schema version for Git-history identity-component records.
pub const GIT_HISTORY_IDENTITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GIT_HISTORY_IDENTITY_SCHEMA_REF: &str =
    "schemas/ui/m5-git-history-identity-component.schema.json";

/// Repo-relative path of the contract doc.
pub const GIT_HISTORY_IDENTITY_DOC_REF: &str =
    "docs/git/m5/implement_commit_graph_headers_history_graph_rows_branch_comparison_chips_and_worktree_rows.md";

/// Repo-relative path of the frozen component matrix this lane implements.
pub const GIT_HISTORY_IDENTITY_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-sequence-component-matrix.schema.json";

/// Repo-relative path of the canonical commit-history review contract.
pub const GIT_HISTORY_IDENTITY_COMMIT_HISTORY_CONTRACT_REF: &str =
    "schemas/git/git_history_review.schema.json";

/// Repo-relative path of the canonical repository-topology contract.
pub const GIT_HISTORY_IDENTITY_TOPOLOGY_CONTRACT_REF: &str =
    "schemas/review/repository-topology.schema.json";

/// Repo-relative path of the canonical recovery-checkpoint contract.
pub const GIT_HISTORY_IDENTITY_RECOVERY_CHECKPOINT_CONTRACT_REF: &str =
    "schemas/git/recovery_checkpoint.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const GIT_HISTORY_IDENTITY_FIXTURE_DIR: &str = "fixtures/ui/m5-git-history-identity-components";

/// Repo-relative path of the checked support-export artifact.
pub const GIT_HISTORY_IDENTITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-git-history-identity-components-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const GIT_HISTORY_IDENTITY_SUMMARY_REF: &str =
    "artifacts/release/m5-git-history-identity-components-proof/summary.md";

/// The four identity/display components this lane implements.
///
/// These are exactly the non-mutating components of the frozen matrix: a header,
/// a row, a chip, and a worktree row whose only job is to make repo/worktree/ref
/// identity and topology state obvious before a user compares, switches, or
/// mutates history.
pub const GIT_HISTORY_IDENTITY_COMPONENTS: [M5GitHistoryComponent; 4] = [
    M5GitHistoryComponent::CommitGraphHeader,
    M5GitHistoryComponent::HistoryGraphRow,
    M5GitHistoryComponent::BranchComparisonChip,
    M5GitHistoryComponent::WorktreeRow,
];

/// Which working context a component targets: the core honesty axis.
///
/// A row must let the reader tell the current primary worktree, another linked
/// worktree, a partial/shallow checkout, and a detached/bare root apart from the
/// component alone. The claim to be the current context is derived from this
/// axis, never asserted directly, so multiple worktrees or divergent roots can
/// never be flattened into one ambiguous context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitWorkingContextTarget {
    /// The current primary worktree the user is operating in.
    CurrentRepoWorktree,
    /// Another linked worktree with its own separate working context.
    LinkedWorktree,
    /// A partial/shallow/sparse checkout where history is incomplete.
    PartialOrShallowCheckout,
    /// A detached-HEAD or bare root; a divergent root, not the primary context.
    DetachedOrBareRoot,
}

impl GitWorkingContextTarget {
    /// Every working-context target, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CurrentRepoWorktree,
        Self::LinkedWorktree,
        Self::PartialOrShallowCheckout,
        Self::DetachedOrBareRoot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRepoWorktree => "current_repo_worktree",
            Self::LinkedWorktree => "linked_worktree",
            Self::PartialOrShallowCheckout => "partial_or_shallow_checkout",
            Self::DetachedOrBareRoot => "detached_or_bare_root",
        }
    }

    /// Whether a component of this target legitimately claims to be the current
    /// primary context. Only the current primary worktree may claim it.
    pub const fn asserts_current_primary_context(self) -> bool {
        matches!(self, Self::CurrentRepoWorktree)
    }

    /// Whether this target is a separate working context that must keep its own
    /// semantics rather than being flattened into one branch list.
    pub const fn is_separate_working_context(self) -> bool {
        matches!(self, Self::LinkedWorktree)
    }

    /// Whether this target inherently carries incomplete history.
    pub const fn has_incomplete_history(self) -> bool {
        matches!(self, Self::PartialOrShallowCheckout)
    }
}

/// Divergence relation shown by an identity component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DivergenceState {
    /// The ref is current with its upstream.
    Current,
    /// The ref is ahead of its upstream.
    Ahead,
    /// The ref is behind its upstream.
    Behind,
    /// The ref and its upstream have diverged.
    Diverged,
    /// The ref is detached and has no upstream to compare against.
    DetachedNoUpstream,
    /// Divergence cannot be computed (for example, offline or shallow).
    Unknown,
}

impl DivergenceState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Ahead => "ahead",
            Self::Behind => "behind",
            Self::Diverged => "diverged",
            Self::DetachedNoUpstream => "detached_no_upstream",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this relation represents ahead/behind/diverged history.
    pub const fn is_divergent(self) -> bool {
        matches!(self, Self::Ahead | Self::Behind | Self::Diverged)
    }

    /// Whether this relation is a detached ref with no upstream.
    pub const fn is_detached(self) -> bool {
        matches!(self, Self::DetachedNoUpstream)
    }
}

/// Dirty/conflicted state of the worktree an identity component names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorktreeDirtyState {
    /// The worktree has no uncommitted changes.
    Clean,
    /// The worktree has uncommitted changes.
    DirtyUncommitted,
    /// The worktree has unresolved merge conflicts.
    Conflicted,
}

impl WorktreeDirtyState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::DirtyUncommitted => "dirty_uncommitted",
            Self::Conflicted => "conflicted",
        }
    }

    /// Whether this state is dirty or conflicted (recovery must stay reachable).
    pub const fn is_dirty_or_conflicted(self) -> bool {
        matches!(self, Self::DirtyUncommitted | Self::Conflicted)
    }
}

/// Topology completeness of the checkout an identity component names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyCompleteness {
    /// A full checkout; history shown is complete.
    Complete,
    /// A shallow clone; history depth is truncated.
    Shallow,
    /// A partial clone; some objects are omitted.
    Partial,
    /// A sparse checkout; some paths are omitted.
    Sparse,
}

impl TopologyCompleteness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Shallow => "shallow",
            Self::Partial => "partial",
            Self::Sparse => "sparse",
        }
    }

    /// Whether history/objects/paths are incomplete here and must be marked.
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::Shallow | Self::Partial | Self::Sparse)
    }
}

/// A direct action an identity component exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityComponentAction {
    /// Open the target inside the Aureline workspace.
    OpenInWorkspace,
    /// Switch into another worktree's separate working context.
    SwitchWorktreeContext,
    /// Compare the two refs the component names.
    CompareRefs,
    /// Open the reflog/recovery destination associated with the target.
    OpenRecoveryReflog,
    /// Deepen or hydrate a shallow/partial checkout.
    DeepenOrHydrateHistory,
    /// Hand off to the hosted provider view in the browser.
    OpenProviderInBrowser,
}

impl IdentityComponentAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInWorkspace => "open_in_workspace",
            Self::SwitchWorktreeContext => "switch_worktree_context",
            Self::CompareRefs => "compare_refs",
            Self::OpenRecoveryReflog => "open_recovery_reflog",
            Self::DeepenOrHydrateHistory => "deepen_or_hydrate_history",
            Self::OpenProviderInBrowser => "open_provider_in_browser",
        }
    }

    /// Whether this action stays inside the product rather than forcing raw-provider navigation.
    pub const fn is_in_product(self) -> bool {
        !matches!(self, Self::OpenProviderInBrowser)
    }
}

/// Disclosures an identity component must carry, derived from its working-context
/// target and topology state.
///
/// This is the resolver output that anchors the honesty invariants: a non-current
/// context never claims to be current, a separate worktree keeps its own context,
/// a shallow/partial checkout is always marked, and a dirty/conflicted/divergent
/// or detached row keeps its recovery/reflog availability explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GitHistoryIdentityDisclosure {
    /// Whether the component asserts it is the current primary context.
    pub asserts_current_primary_context: bool,
    /// Whether the component must keep a separate worktree working context explicit.
    pub needs_separate_worktree_context: bool,
    /// Whether the component must carry an incomplete-history marker.
    pub needs_incomplete_history_marker: bool,
    /// Whether the component must keep recovery/reflog availability explicit.
    pub needs_recovery_reflog_availability: bool,
}

/// Resolves the disclosures a component must carry from its working-context target
/// and topology state.
///
/// The current-context claim follows the target alone, so a linked worktree or a
/// partial/shallow checkout can never pretend it is the current repo. A separate
/// worktree always keeps its own context; a shallow/partial/sparse checkout (or a
/// partial-checkout target) always carries an incomplete-history marker; and any
/// dirty, conflicted, divergent, or detached target always forces explicit
/// recovery/reflog availability.
pub fn resolve_git_history_identity_disclosure(
    target: GitWorkingContextTarget,
    divergence: DivergenceState,
    dirty: WorktreeDirtyState,
    completeness: TopologyCompleteness,
) -> GitHistoryIdentityDisclosure {
    GitHistoryIdentityDisclosure {
        asserts_current_primary_context: target.asserts_current_primary_context(),
        needs_separate_worktree_context: target.is_separate_working_context(),
        needs_incomplete_history_marker: completeness.is_incomplete()
            || target.has_incomplete_history(),
        needs_recovery_reflog_availability: dirty.is_dirty_or_conflicted()
            || divergence.is_divergent()
            || divergence.is_detached()
            || target == GitWorkingContextTarget::DetachedOrBareRoot,
    }
}

/// One identity component (header / row / chip / worktree row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryIdentityRow {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component this row implements; must be an identity component.
    pub component: M5GitHistoryComponent,
    /// Which working context the component targets.
    pub working_context_target: GitWorkingContextTarget,
    /// Human-readable repo root / identity label.
    pub repo_identity_label: String,
    /// Human-readable checked-out ref label.
    pub checked_out_ref_label: String,
    /// Human-readable worktree path label.
    pub worktree_path_label: String,
    /// Divergence relation shown by the component.
    pub divergence: DivergenceState,
    /// Dirty/conflicted state of the worktree the component names.
    pub dirty_state: WorktreeDirtyState,
    /// Topology completeness of the checkout the component names.
    pub topology_completeness: TopologyCompleteness,
    /// Whether the component claims to be the current primary context; must match the target.
    pub claims_current_primary_context: bool,
    /// Separate-worktree context note; required and non-empty when the disclosure demands it.
    pub separate_worktree_context_note: String,
    /// Incomplete-history marker; required and non-empty when the disclosure demands it.
    pub incomplete_history_marker: String,
    /// Recovery/reflog availability note; required and non-empty when the disclosure demands it.
    pub recovery_reflog_availability: String,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Direct actions the component exposes, in display order.
    pub actions: Vec<IdentityComponentAction>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl GitHistoryIdentityRow {
    /// Disclosures this component must carry, derived from its target and topology.
    pub fn disclosure(&self) -> GitHistoryIdentityDisclosure {
        resolve_git_history_identity_disclosure(
            self.working_context_target,
            self.divergence,
            self.dirty_state,
            self.topology_completeness,
        )
    }

    /// Whether this component exposes at least one in-product action.
    pub fn has_in_product_action(&self) -> bool {
        self.actions.iter().any(|action| action.is_in_product())
    }

    /// Whether this component is one of the four identity/display components.
    pub fn is_identity_component(&self) -> bool {
        GIT_HISTORY_IDENTITY_COMPONENTS.contains(&self.component)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryIdentityTrustReview {
    /// Multiple worktrees are never flattened into one ambiguous branch list.
    pub worktree_identity_never_flattened: bool,
    /// Exact repo/ref identity is always explicit on the component.
    pub exact_repo_ref_identity_explicit: bool,
    /// Divergence (ahead/behind/diverged/detached) is explicit.
    pub divergence_state_explicit: bool,
    /// Dirty/conflicted worktree state is explicit.
    pub dirty_state_explicit: bool,
    /// Shallow/partial/sparse topology is marked, never hidden.
    pub shallow_partial_sparse_marked: bool,
    /// The worktree path is explicit on worktree-scoped components.
    pub worktree_path_explicit: bool,
    /// Recovery/reflog availability stays explicit for degraded contexts.
    pub recovery_reflog_availability_explicit: bool,
    /// Each separate working context keeps its own semantics.
    pub separate_working_context_preserved: bool,
    /// The current context versus another context is always unambiguous.
    pub current_versus_other_context_unambiguous: bool,
    /// One component contract is reused with no hidden per-surface meaning.
    pub one_component_contract_no_hidden_meaning: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified components automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl GitHistoryIdentityTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.worktree_identity_never_flattened
            && self.exact_repo_ref_identity_explicit
            && self.divergence_state_explicit
            && self.dirty_state_explicit
            && self.shallow_partial_sparse_marked
            && self.worktree_path_explicit
            && self.recovery_reflog_availability_explicit
            && self.separate_working_context_preserved
            && self.current_versus_other_context_unambiguous
            && self.one_component_contract_no_hidden_meaning
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryIdentityConsumerProjection {
    /// Review surfaces reuse one component contract.
    pub review_reuses_one_contract: bool,
    /// Shell surfaces reuse one component contract.
    pub shell_reuses_one_contract: bool,
    /// Help surfaces reuse one component contract.
    pub help_reuses_one_contract: bool,
    /// Support/export surfaces reuse one component contract.
    pub support_export_reuses_one_contract: bool,
    /// The component distinguishes current, other-worktree, and partial/shallow contexts.
    pub component_distinguishes_current_other_partial: bool,
    /// CLI / headless shows component truth.
    pub cli_headless_shows_truth: bool,
    /// Provider overlay shows component truth without overwriting local truth.
    pub provider_overlay_shows_truth: bool,
    /// AI-context assembly shows component truth.
    pub ai_context_shows_truth: bool,
}

impl GitHistoryIdentityConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_reuses_one_contract
            && self.shell_reuses_one_contract
            && self.help_reuses_one_contract
            && self.support_export_reuses_one_contract
            && self.component_distinguishes_current_other_partial
            && self.cli_headless_shows_truth
            && self.provider_overlay_shows_truth
            && self.ai_context_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryIdentityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GitHistoryIdentityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHistoryIdentityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Identity component rows.
    pub rows: Vec<GitHistoryIdentityRow>,
    /// Shared downgrade states that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryDowngradeState>,
    /// Consumer surfaces that must reuse this component contract.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: GitHistoryIdentityTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GitHistoryIdentityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GitHistoryIdentityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe Git-history identity-component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryIdentityPacket {
    /// Record kind; must equal [`GIT_HISTORY_IDENTITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GIT_HISTORY_IDENTITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Identity component rows.
    pub rows: Vec<GitHistoryIdentityRow>,
    /// Shared downgrade states that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryDowngradeState>,
    /// Consumer surfaces that must reuse this component contract.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: GitHistoryIdentityTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GitHistoryIdentityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GitHistoryIdentityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GitHistoryIdentityPacket {
    /// Builds a Git-history identity-component packet from stable-lane input.
    pub fn new(input: GitHistoryIdentityPacketInput) -> Self {
        Self {
            record_kind: GIT_HISTORY_IDENTITY_RECORD_KIND.to_owned(),
            schema_version: GIT_HISTORY_IDENTITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            rows: input.rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the identity-component invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<GitHistoryIdentityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GIT_HISTORY_IDENTITY_RECORD_KIND {
            violations.push(GitHistoryIdentityViolation::WrongRecordKind);
        }
        if self.schema_version != GIT_HISTORY_IDENTITY_SCHEMA_VERSION {
            violations.push(GitHistoryIdentityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GitHistoryIdentityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GitHistoryIdentityViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(GitHistoryIdentityViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(GitHistoryIdentityViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(GitHistoryIdentityViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GitHistoryIdentityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("git history identity packet serializes"),
        ) {
            violations.push(GitHistoryIdentityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("git history identity packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let linked = self
            .rows
            .iter()
            .filter(|row| row.working_context_target == GitWorkingContextTarget::LinkedWorktree)
            .count();
        let partial = self
            .rows
            .iter()
            .filter(|row| row.topology_completeness.is_incomplete())
            .count();
        let recovery = self
            .rows
            .iter()
            .filter(|row| row.disclosure().needs_recovery_reflog_availability)
            .count();

        let mut out = String::new();
        out.push_str("# Git-History Identity Components: Working-Context and Topology Truth\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Components: {} ({} on a linked worktree, {} on a partial/shallow checkout, {} carrying recovery/reflog availability)\n",
            self.rows.len(),
            linked,
            partial,
            recovery
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Components\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: repo `{}`, ref `{}`, worktree `{}` — divergence `{}`, dirty `{}`, topology `{}`\n",
                row.component.as_str(),
                row.working_context_target.as_str(),
                row.repo_identity_label,
                row.checked_out_ref_label,
                row.worktree_path_label,
                row.divergence.as_str(),
                row.dirty_state.as_str(),
                row.topology_completeness.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in identity-component export.
#[derive(Debug)]
pub enum GitHistoryIdentityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GitHistoryIdentityViolation>),
}

impl fmt::Display for GitHistoryIdentityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "git history identity export parse failed: {error}"
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
                    "git history identity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GitHistoryIdentityArtifactError {}

/// Validation failures emitted by [`GitHistoryIdentityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitHistoryIdentityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No rows are present.
    RowsMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's component is not one of the four identity/display components.
    NonIdentityComponent,
    /// A row's current-context claim does not match its working-context target.
    AmbiguousContextClaimed,
    /// A row that must keep a separate worktree context explicit is missing it.
    SeparateWorktreeContextMissing,
    /// A row that must carry an incomplete-history marker is missing it.
    IncompleteHistoryMarkerMissing,
    /// A row that must keep recovery/reflog availability explicit is missing it.
    RecoveryReflogAvailabilityMissing,
    /// A row exposes no in-product action and forces raw-provider navigation.
    ForcedRawProviderNavigation,
    /// The row set does not cover current, linked-worktree, and partial/shallow contexts.
    WorkingContextCoverageMissing,
    /// The row set does not cover all four identity components.
    ComponentCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl GitHistoryIdentityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsMissing => "rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::NonIdentityComponent => "non_identity_component",
            Self::AmbiguousContextClaimed => "ambiguous_context_claimed",
            Self::SeparateWorktreeContextMissing => "separate_worktree_context_missing",
            Self::IncompleteHistoryMarkerMissing => "incomplete_history_marker_missing",
            Self::RecoveryReflogAvailabilityMissing => "recovery_reflog_availability_missing",
            Self::ForcedRawProviderNavigation => "forced_raw_provider_navigation",
            Self::WorkingContextCoverageMissing => "working_context_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable identity-component export.
///
/// # Errors
///
/// Returns [`GitHistoryIdentityArtifactError`] when the checked-in export fails to
/// parse or violates the contract.
pub fn current_git_history_identity_export(
) -> Result<GitHistoryIdentityPacket, GitHistoryIdentityArtifactError> {
    let packet: GitHistoryIdentityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-git-history-identity-components-proof/support_export.json"
    )))
    .map_err(GitHistoryIdentityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GitHistoryIdentityArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &GitHistoryIdentityPacket,
    violations: &mut Vec<GitHistoryIdentityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GIT_HISTORY_IDENTITY_SCHEMA_REF,
        GIT_HISTORY_IDENTITY_DOC_REF,
        GIT_HISTORY_IDENTITY_COMPONENT_MATRIX_CONTRACT_REF,
        GIT_HISTORY_IDENTITY_COMMIT_HISTORY_CONTRACT_REF,
        GIT_HISTORY_IDENTITY_TOPOLOGY_CONTRACT_REF,
        GIT_HISTORY_IDENTITY_RECOVERY_CHECKPOINT_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GitHistoryIdentityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &GitHistoryIdentityPacket,
    violations: &mut Vec<GitHistoryIdentityViolation>,
) {
    if packet.rows.is_empty() {
        violations.push(GitHistoryIdentityViolation::RowsMissing);
        return;
    }

    let mut targets_present: BTreeSet<GitWorkingContextTarget> = BTreeSet::new();
    let mut components_present: BTreeSet<M5GitHistoryComponent> = BTreeSet::new();

    for row in &packet.rows {
        targets_present.insert(row.working_context_target);
        components_present.insert(row.component);

        if row.row_id.trim().is_empty()
            || row.repo_identity_label.trim().is_empty()
            || row.checked_out_ref_label.trim().is_empty()
            || row.worktree_path_label.trim().is_empty()
            || row.downgrade_vocab.is_empty()
            || row.actions.is_empty()
            || row.fields_shown.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(GitHistoryIdentityViolation::RowIncomplete);
        }

        if !row.is_identity_component() {
            violations.push(GitHistoryIdentityViolation::NonIdentityComponent);
        }

        let disclosure = row.disclosure();

        if row.claims_current_primary_context != disclosure.asserts_current_primary_context {
            violations.push(GitHistoryIdentityViolation::AmbiguousContextClaimed);
        }
        if disclosure.needs_separate_worktree_context
            && row.separate_worktree_context_note.trim().is_empty()
        {
            violations.push(GitHistoryIdentityViolation::SeparateWorktreeContextMissing);
        }
        if disclosure.needs_incomplete_history_marker
            && row.incomplete_history_marker.trim().is_empty()
        {
            violations.push(GitHistoryIdentityViolation::IncompleteHistoryMarkerMissing);
        }
        if disclosure.needs_recovery_reflog_availability
            && row.recovery_reflog_availability.trim().is_empty()
        {
            violations.push(GitHistoryIdentityViolation::RecoveryReflogAvailabilityMissing);
        }
        if !row.has_in_product_action() {
            violations.push(GitHistoryIdentityViolation::ForcedRawProviderNavigation);
        }
    }

    for required in [
        GitWorkingContextTarget::CurrentRepoWorktree,
        GitWorkingContextTarget::LinkedWorktree,
        GitWorkingContextTarget::PartialOrShallowCheckout,
    ] {
        if !targets_present.contains(&required) {
            violations.push(GitHistoryIdentityViolation::WorkingContextCoverageMissing);
            break;
        }
    }

    for required in GIT_HISTORY_IDENTITY_COMPONENTS {
        if !components_present.contains(&required) {
            violations.push(GitHistoryIdentityViolation::ComponentCoverageMissing);
            break;
        }
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
