//! Two reusable M5 write-scope primitives — the write-scope preview *tree* and the
//! write-scope preview *file node* — so a multi-file rename, refactor, replace, import,
//! AI apply, or repair flow is preview-first, blast-radius-honest, and never flattens its
//! ineligible files out of the picture before it applies.
//!
//! Aureline's frozen local-history / write-scope component matrix
//! ([`crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix`])
//! names the write-scope preview tree as one governed component family and freezes its
//! controlled vocabulary — the write-scope classes, the managed-file caveats, the mutation
//! classes, the surface families, the deployment lines, the consumer surfaces, the
//! accessibility routes, the qualification classes, and the downgrade triggers. This module
//! *implements* that contract as two reusable primitives so a user can tell — from the tree
//! and its file nodes alone — how wide the write scope reaches (single file, several files,
//! a whole directory, across packages, a generated tree, or out of the workspace), which
//! file-count bucket the change lands in, which workspace roots it groups under, which
//! change type touched each file, who or what authored that change, whether a file is
//! generated, read-only, binary, metadata-only, in conflict, or policy-blocked, and — before
//! any apply — exactly which files are in scope and which are excluded and *why*.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_write_scope_preview_tree`] — takes one change's write-scope class, mutation
//!    class, total / included / excluded file counts, distinct workspace-root count,
//!    generated-or-managed signal, out-of-workspace signal, conflict and policy-blocked
//!    signals, whether the scope is reviewable, whether the apply path is ready, and its
//!    opaque scope label, and produces one [`M5ResolvedWriteScopePreviewTree`] carrying the
//!    derived tree posture (focused versus broad versus generated/managed versus
//!    out-of-workspace versus conflict versus blocked), the derived file-count bucket,
//!    whether the apply can commit, whether the scope can narrow, and the bounded
//!    inspect-tree / expand-all / jump-to-diff / narrow-scope / exclude-generated /
//!    apply-scope / resolve-conflict actions. It never understates the write scope and never
//!    drops the excluded files out of the count.
//! 2. [`resolve_write_scope_file_node`] — takes one file node's change type, change actor,
//!    content class, managed-file caveat, read-only / conflict / policy-blocked /
//!    out-of-workspace signals, whether the caller opted the file out of the apply, whether a
//!    diff is available, and its opaque node label, and produces one
//!    [`M5ResolvedWriteScopeFileNode`] carrying the derived node disposition (included versus
//!    generated-excludable versus read-only-excluded versus conflict-held versus
//!    policy-blocked-excluded versus binary-included), the exact exclusion reason where the
//!    node is out of scope, whether the file is included in the apply, whether a diff jump is
//!    reachable, and the bounded view-provenance / jump-to-diff / toggle-include /
//!    view-exclusion-reason / resolve-conflict actions. It always keeps the file present in
//!    the preview — a policy-blocked, binary, metadata-only, read-only, or generated file is
//!    never silently dropped — and always exposes its actor provenance.
//!
//! A single parity matrix — [`M5WriteScopePreviewTreePacket`] — binds one row per claimed M5
//! multi-file change consumer (rename preview, refactor preview, search/replace preview,
//! import preview, AI-apply preview, and repair preview) to the shared tree and node anatomy,
//! the same write-scope classes, managed caveats, change types, change actors, content
//! classes, tree postures, node dispositions, file-count buckets, exclusion reasons, bounded
//! actions, export fields, and non-visual accessibility routes, so the scope / provenance /
//! exclusion vocabulary stays identical across rename, refactor, replace, import, AI apply,
//! and repair surfaces without ever flattening ineligible files into a generic list.
//!
//! The write-scope class ([`M5WriteScopeClass`]), managed-file caveat
//! ([`M5ManagedFileCaveat`]), mutation class ([`M5MutationClass`]), surface family
//! ([`M5HistorySurfaceFamily`]), deployment line ([`M5HistoryDeploymentLine`]), consumer
//! surface ([`M5HistoryConsumerSurface`]), accessibility route
//! ([`M5HistoryAccessibilityRoute`]), qualification class ([`M5HistoryQualificationClass`]),
//! and downgrade trigger ([`M5HistoryDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! tree and the node themselves: their multi-file change consumers, their anatomy parts,
//! their derived tree posture, their derived node disposition, their file-count buckets,
//! their change types, their change actors, their content classes, their exclusion reasons,
//! their bounded actions, and their export fields. No M5 multi-file change surface invents a
//! second write-scope grammar.
//!
//! Raw file bodies, diffs, pasted paths, credentials, and private endpoints stay outside the
//! support boundary; every node identity, scope label, and change descriptor is carried only
//! as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_write_scope_preview_tree_ai_apply_preview_beta_narrowed,
    seeded_m5_write_scope_preview_tree_import_preview_preview_narrowed,
    seeded_m5_write_scope_preview_tree_packet, M5_WRITE_SCOPE_PREVIEW_TREE_PACKET_ID,
};

// The write-scope class, managed-file caveat, mutation class, surface family, deployment
// line, consumer surface, accessibility route, qualification class, and downgrade triggers
// are frozen once, in the local-history / write-scope component matrix. These primitives
// reuse them verbatim so they never invent a parallel write-scope vocabulary.
pub use crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::{
    M5HistoryAccessibilityRoute, M5HistoryConsumerSurface, M5HistoryDeploymentLine,
    M5HistoryDowngradeTrigger, M5HistoryQualificationClass, M5HistorySurfaceFamily,
    M5ManagedFileCaveat, M5MutationClass, M5WriteScopeClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5WriteScopePreviewTreePacket`].
pub const M5_WRITE_SCOPE_PREVIEW_TREE_RECORD_KIND: &str =
    "implement_m5_write_scope_preview_trees_with_file_count_buckets_actor_provenance_selectable_scope_diff_jump_and_generated_read_only_conflict_exclusion_truth_across_claimed_m5_multi_file_change_flows";

/// Schema version for M5 write-scope-preview-tree records.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the tree / node boundary schema.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF: &str =
    "schemas/ui/m5-write-scope-preview-tree.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_DOC_REF: &str =
    "docs/recovery/m5_write_scope_preview_tree_primitive.md";

/// Repo-relative path of the frozen local-history / write-scope component matrix these
/// primitives narrow from.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-matrix.schema.json";

/// Repo-relative path of the write-boundary contract this primitive binds its write-scope
/// and generated/managed truth against.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_WRITE_BOUNDARY_REF: &str =
    "schemas/generated/write-boundary.schema.json";

/// Repo-relative path of the refactor-preview contract this primitive binds its multi-file
/// change-scope truth against.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_REFACTOR_PREVIEW_REF: &str =
    "schemas/editor/refactor_preview.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-write-scope-preview-tree-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_ARTIFACT_REF: &str =
    "artifacts/release/m5-write-scope-preview-tree-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_CSV_REF: &str =
    "artifacts/release/m5-write-scope-preview-tree-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_REPORT_REF: &str =
    "artifacts/design/m5-write-scope-preview-tree-primitive.md";

/// One claimed M5 multi-file change consumer that renders the shared write-scope preview
/// tree and its file nodes. These are the consumers the acceptance criteria name — a rename,
/// refactor, replace, import, AI apply, or repair flow — so the same tree and node grammar
/// works across every claimed multi-file mutation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeConsumerSurface {
    /// The rename / move preview surface.
    RenamePreview,
    /// The refactor-transaction preview surface.
    RefactorPreview,
    /// The search-and-replace preview surface.
    SearchReplacePreview,
    /// The importer / external-sync preview surface.
    ImportPreview,
    /// The AI-apply preview surface.
    AiApplyPreview,
    /// The repair-transaction preview surface.
    RepairPreview,
}

impl M5WriteScopeConsumerSurface {
    /// Every claimed multi-file change consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RenamePreview,
        Self::RefactorPreview,
        Self::SearchReplacePreview,
        Self::ImportPreview,
        Self::AiApplyPreview,
        Self::RepairPreview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenamePreview => "rename_preview",
            Self::RefactorPreview => "refactor_preview",
            Self::SearchReplacePreview => "search_replace_preview",
            Self::ImportPreview => "import_preview",
            Self::AiApplyPreview => "ai_apply_preview",
            Self::RepairPreview => "repair_preview",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RenamePreview => "Rename Preview",
            Self::RefactorPreview => "Refactor Preview",
            Self::SearchReplacePreview => "Search/Replace Preview",
            Self::ImportPreview => "Import Preview",
            Self::AiApplyPreview => "AI Apply Preview",
            Self::RepairPreview => "Repair Preview",
        }
    }
}

/// The derived posture of a write-scope preview tree — the resolver's verdict about how wide
/// the change reaches and whether it can apply. Computed in a fixed blocking-first order, so
/// an out-of-workspace, generated, conflicted, or blocked scope never reads as a focused
/// single-file change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeTreePosture {
    /// A focused, in-workspace change over a small file set.
    FocusedScope,
    /// A broad change across many files, a whole directory, or packages.
    BroadScope,
    /// A change that reaches a generated tree or managed files.
    GeneratedManagedScope,
    /// A change that writes outside the workspace root.
    OutOfWorkspaceScope,
    /// A change blocked behind a pending conflict that must resolve first.
    ConflictScope,
    /// A change whose apply path is unavailable.
    BlockedScope,
}

impl M5WriteScopeTreePosture {
    /// Every tree posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FocusedScope,
        Self::BroadScope,
        Self::GeneratedManagedScope,
        Self::OutOfWorkspaceScope,
        Self::ConflictScope,
        Self::BlockedScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FocusedScope => "focused_scope",
            Self::BroadScope => "broad_scope",
            Self::GeneratedManagedScope => "generated_managed_scope",
            Self::OutOfWorkspaceScope => "out_of_workspace_scope",
            Self::ConflictScope => "conflict_scope",
            Self::BlockedScope => "blocked_scope",
        }
    }

    /// True when a tree at this posture can still commit an apply.
    pub const fn can_apply(self) -> bool {
        !matches!(self, Self::ConflictScope | Self::BlockedScope)
    }

    /// True when the tree needs operator attention before an apply commits.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::GeneratedManagedScope
                | Self::OutOfWorkspaceScope
                | Self::ConflictScope
                | Self::BlockedScope
        )
    }
}

/// The derived file-count bucket a write-scope preview tree lands in, so the tree always
/// carries an honest order-of-magnitude count instead of hiding the blast radius behind a
/// collapsed group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeFileCountBucket {
    /// Zero files (every candidate excluded or blocked).
    Empty,
    /// Exactly one file.
    Single,
    /// A small change (2–5 files).
    Small,
    /// A medium change (6–25 files).
    Medium,
    /// A large change (26–100 files).
    Large,
    /// A sweeping change (more than 100 files).
    Sweeping,
}

impl M5WriteScopeFileCountBucket {
    /// Every file-count bucket, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Empty,
        Self::Single,
        Self::Small,
        Self::Medium,
        Self::Large,
        Self::Sweeping,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Single => "single",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Sweeping => "sweeping",
        }
    }

    /// Buckets a total file count into a fixed order-of-magnitude band.
    pub const fn from_total(total: u32) -> Self {
        match total {
            0 => Self::Empty,
            1 => Self::Single,
            2..=5 => Self::Small,
            6..=25 => Self::Medium,
            26..=100 => Self::Large,
            _ => Self::Sweeping,
        }
    }

    /// True when this bucket spans more than one file.
    pub const fn is_multi_file(self) -> bool {
        matches!(
            self,
            Self::Small | Self::Medium | Self::Large | Self::Sweeping
        )
    }
}

/// One bounded action a write-scope preview tree offers, so a tree never hides its inspect /
/// expand / diff-jump / narrow / exclude / apply / resolve affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeTreeAction {
    /// Inspect the write-scope tree (inspect-only, never mutating).
    InspectTree,
    /// Expand every collapsed level.
    ExpandAll,
    /// Jump to the diff for the scope.
    JumpToDiff,
    /// Narrow the apply scope (choose files / ranges).
    NarrowScope,
    /// Exclude generated files from the apply.
    ExcludeGenerated,
    /// Apply the currently selected scope.
    ApplyScope,
    /// Resolve the pending conflict before any apply.
    ResolveConflict,
}

impl M5WriteScopeTreeAction {
    /// Every tree action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InspectTree,
        Self::ExpandAll,
        Self::JumpToDiff,
        Self::NarrowScope,
        Self::ExcludeGenerated,
        Self::ApplyScope,
        Self::ResolveConflict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectTree => "inspect_tree",
            Self::ExpandAll => "expand_all",
            Self::JumpToDiff => "jump_to_diff",
            Self::NarrowScope => "narrow_scope",
            Self::ExcludeGenerated => "exclude_generated",
            Self::ApplyScope => "apply_scope",
            Self::ResolveConflict => "resolve_conflict",
        }
    }
}

/// Controlled write-scope-preview-tree anatomy part the shared tree surfaces. The parts in
/// [`M5WriteScopeTreeAnatomyPart::MANDATORY`] are required on every tree so the scope class,
/// file-count bucket, workspace-root grouping, actor provenance, and action row are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeTreeAnatomyPart {
    /// The write-scope-class cue.
    ScopeClassCue,
    /// The file-count-bucket cue.
    FileCountBucketCue,
    /// The workspace-root grouping cue.
    WorkspaceRootCue,
    /// The generated-or-managed-file caveat cue.
    ManagedCaveatCue,
    /// The conflict / policy-blocked cue.
    ConflictCue,
    /// The actor-provenance cue.
    ActorProvenanceCue,
    /// The narrowability cue.
    NarrowabilityCue,
    /// The bounded action row (inspect / expand / jump / narrow / apply / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5WriteScopeTreeAnatomyPart {
    /// Every tree anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ScopeClassCue,
        Self::FileCountBucketCue,
        Self::WorkspaceRootCue,
        Self::ManagedCaveatCue,
        Self::ConflictCue,
        Self::ActorProvenanceCue,
        Self::NarrowabilityCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The tree anatomy parts every tree must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ScopeClassCue,
        Self::FileCountBucketCue,
        Self::WorkspaceRootCue,
        Self::ActorProvenanceCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeClassCue => "scope_class_cue",
            Self::FileCountBucketCue => "file_count_bucket_cue",
            Self::WorkspaceRootCue => "workspace_root_cue",
            Self::ManagedCaveatCue => "managed_caveat_cue",
            Self::ConflictCue => "conflict_cue",
            Self::ActorProvenanceCue => "actor_provenance_cue",
            Self::NarrowabilityCue => "narrowability_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the tree export carries so write-scope-preview-tree truth is reconstructable. The
/// fields in [`M5WriteScopeTreeExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeTreeExportField {
    /// The opaque scope label.
    ScopeLabel,
    /// The write-scope class.
    WriteScopeClass,
    /// The derived file-count bucket.
    FileCountBucket,
    /// The derived tree posture.
    TreePosture,
    /// Whether the scope touches generated or managed files.
    TouchesGeneratedOrManaged,
    /// The included file count.
    IncludedFileCount,
    /// The excluded file count.
    ExcludedFileCount,
    /// Whether the apply can commit.
    CanApply,
    /// The bounded available actions.
    AvailableActions,
}

impl M5WriteScopeTreeExportField {
    /// Every tree export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ScopeLabel,
        Self::WriteScopeClass,
        Self::FileCountBucket,
        Self::TreePosture,
        Self::TouchesGeneratedOrManaged,
        Self::IncludedFileCount,
        Self::ExcludedFileCount,
        Self::CanApply,
        Self::AvailableActions,
    ];

    /// The tree export fields every tree must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ScopeLabel,
        Self::WriteScopeClass,
        Self::FileCountBucket,
        Self::TreePosture,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeLabel => "scope_label",
            Self::WriteScopeClass => "write_scope_class",
            Self::FileCountBucket => "file_count_bucket",
            Self::TreePosture => "tree_posture",
            Self::TouchesGeneratedOrManaged => "touches_generated_or_managed",
            Self::IncludedFileCount => "included_file_count",
            Self::ExcludedFileCount => "excluded_file_count",
            Self::CanApply => "can_apply",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// Controlled change type — what a write-scope file node did to the file, so a rename, move,
/// or content-replace is never flattened into a generic "modified".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeChangeType {
    /// The file is created.
    Created,
    /// The file's contents are modified.
    Modified,
    /// The file is renamed in place.
    Renamed,
    /// The file is deleted.
    Deleted,
    /// The file is moved to another path.
    Moved,
    /// The file's whole contents are replaced.
    ContentReplaced,
}

impl M5WriteScopeChangeType {
    /// Every change type, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Created,
        Self::Modified,
        Self::Renamed,
        Self::Deleted,
        Self::Moved,
        Self::ContentReplaced,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Modified => "modified",
            Self::Renamed => "renamed",
            Self::Deleted => "deleted",
            Self::Moved => "moved",
            Self::ContentReplaced => "content_replaced",
        }
    }
}

/// Controlled change actor — who or what authored a write-scope file node's change, so actor
/// provenance is always attributable and never masked behind a bare path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeChangeActor {
    /// A human edit.
    HumanEdit,
    /// An AI agent.
    AiAgent,
    /// A refactor engine.
    RefactorEngine,
    /// An import / external-sync bridge.
    ImportBridge,
    /// A repair engine.
    RepairEngine,
    /// A formatter / codemod.
    Formatter,
}

impl M5WriteScopeChangeActor {
    /// Every change actor, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HumanEdit,
        Self::AiAgent,
        Self::RefactorEngine,
        Self::ImportBridge,
        Self::RepairEngine,
        Self::Formatter,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanEdit => "human_edit",
            Self::AiAgent => "ai_agent",
            Self::RefactorEngine => "refactor_engine",
            Self::ImportBridge => "import_bridge",
            Self::RepairEngine => "repair_engine",
            Self::Formatter => "formatter",
        }
    }
}

/// Controlled content class — what kind of content a write-scope file node carries, so a
/// binary or metadata-only file stays visible in the preview instead of being dropped as
/// ineligible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeFileContentClass {
    /// A text source file.
    TextSource,
    /// A binary blob.
    BinaryBlob,
    /// A metadata-only change (mode / rename with no content delta).
    MetadataOnly,
    /// A generated-output file.
    GeneratedOutput,
    /// A symbolic link.
    SymbolicLink,
}

impl M5WriteScopeFileContentClass {
    /// Every content class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TextSource,
        Self::BinaryBlob,
        Self::MetadataOnly,
        Self::GeneratedOutput,
        Self::SymbolicLink,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextSource => "text_source",
            Self::BinaryBlob => "binary_blob",
            Self::MetadataOnly => "metadata_only",
            Self::GeneratedOutput => "generated_output",
            Self::SymbolicLink => "symbolic_link",
        }
    }
}

/// The derived disposition of a write-scope file node — the resolver's verdict about whether
/// a file is in scope, excludable, or held out, and why. Computed in a fixed blocking-first
/// order, so a policy-blocked, conflicted, or read-only file never reads as an ordinary
/// included change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeNodeDisposition {
    /// An ordinary text change in scope.
    IncludedInScope,
    /// A binary change kept in scope (with a binary-diff cue).
    BinaryIncluded,
    /// A generated / managed file, included but excludable.
    GeneratedExcludable,
    /// A read-only / protected file excluded from the apply.
    ReadOnlyExcluded,
    /// A file held out behind a pending conflict.
    ConflictHeld,
    /// A file excluded because policy blocks writing it.
    PolicyBlockedExcluded,
}

impl M5WriteScopeNodeDisposition {
    /// Every node disposition, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IncludedInScope,
        Self::BinaryIncluded,
        Self::GeneratedExcludable,
        Self::ReadOnlyExcluded,
        Self::ConflictHeld,
        Self::PolicyBlockedExcluded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludedInScope => "included_in_scope",
            Self::BinaryIncluded => "binary_included",
            Self::GeneratedExcludable => "generated_excludable",
            Self::ReadOnlyExcluded => "read_only_excluded",
            Self::ConflictHeld => "conflict_held",
            Self::PolicyBlockedExcluded => "policy_blocked_excluded",
        }
    }

    /// True when this disposition forces the file out of the apply regardless of caller
    /// intent.
    pub const fn is_hard_excluded(self) -> bool {
        matches!(
            self,
            Self::ReadOnlyExcluded | Self::ConflictHeld | Self::PolicyBlockedExcluded
        )
    }
}

/// Controlled exclusion reason — why a file is out of the apply scope, surfaced where safe
/// so a user never has to guess why an ineligible file will not be written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeExclusionReason {
    /// Writing the file is blocked by policy.
    PolicyBlocked,
    /// The file is read-only / protected.
    ReadOnlyProtected,
    /// The file is held behind a pending conflict.
    ConflictPending,
    /// A generated file was opted out of the apply.
    GeneratedOptedOut,
    /// The file writes outside the workspace root and was opted out.
    OutOfWorkspace,
}

impl M5WriteScopeExclusionReason {
    /// Every exclusion reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PolicyBlocked,
        Self::ReadOnlyProtected,
        Self::ConflictPending,
        Self::GeneratedOptedOut,
        Self::OutOfWorkspace,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBlocked => "policy_blocked",
            Self::ReadOnlyProtected => "read_only_protected",
            Self::ConflictPending => "conflict_pending",
            Self::GeneratedOptedOut => "generated_opted_out",
            Self::OutOfWorkspace => "out_of_workspace",
        }
    }
}

/// One bounded action a write-scope file node offers, so provenance stays inspectable, a diff
/// jump stays reachable where safe, and an exclusion reason is never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeNodeAction {
    /// Jump to the file's diff.
    JumpToDiff,
    /// Inspect the file's actor provenance (always available).
    ViewProvenance,
    /// Toggle whether the file is included in the apply.
    ToggleInclude,
    /// View the reason the file is excluded.
    ViewExclusionReason,
    /// Resolve the file's pending conflict before any apply.
    ResolveConflict,
}

impl M5WriteScopeNodeAction {
    /// Every node action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::JumpToDiff,
        Self::ViewProvenance,
        Self::ToggleInclude,
        Self::ViewExclusionReason,
        Self::ResolveConflict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JumpToDiff => "jump_to_diff",
            Self::ViewProvenance => "view_provenance",
            Self::ToggleInclude => "toggle_include",
            Self::ViewExclusionReason => "view_exclusion_reason",
            Self::ResolveConflict => "resolve_conflict",
        }
    }
}

/// Controlled write-scope-file-node anatomy part the shared node surfaces. The parts in
/// [`M5WriteScopeNodeAnatomyPart::MANDATORY`] are required on every node so the change type,
/// actor provenance, content class, exclusion reason, and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeNodeAnatomyPart {
    /// The change-type cue.
    ChangeTypeCue,
    /// The actor-provenance cue.
    ActorProvenanceCue,
    /// The content-class cue.
    ContentClassCue,
    /// The generated-or-managed-file caveat cue.
    ManagedCaveatCue,
    /// The exclusion-reason cue.
    ExclusionReasonCue,
    /// The diff-jump cue.
    DiffJumpCue,
    /// The bounded action row.
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5WriteScopeNodeAnatomyPart {
    /// Every node anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ChangeTypeCue,
        Self::ActorProvenanceCue,
        Self::ContentClassCue,
        Self::ManagedCaveatCue,
        Self::ExclusionReasonCue,
        Self::DiffJumpCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The node anatomy parts every node must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ChangeTypeCue,
        Self::ActorProvenanceCue,
        Self::ContentClassCue,
        Self::ExclusionReasonCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeTypeCue => "change_type_cue",
            Self::ActorProvenanceCue => "actor_provenance_cue",
            Self::ContentClassCue => "content_class_cue",
            Self::ManagedCaveatCue => "managed_caveat_cue",
            Self::ExclusionReasonCue => "exclusion_reason_cue",
            Self::DiffJumpCue => "diff_jump_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the node export carries so write-scope-file-node truth is reconstructable. The
/// fields in [`M5WriteScopeNodeExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteScopeNodeExportField {
    /// The opaque node identity.
    NodeIdentity,
    /// The change type.
    ChangeType,
    /// The actor provenance.
    ActorProvenance,
    /// The content class.
    ContentClass,
    /// The derived node disposition.
    NodeDisposition,
    /// The exclusion reason (where excluded).
    ExclusionReason,
    /// Whether the file is included in the apply.
    IsIncludedInApply,
    /// The bounded available actions.
    AvailableActions,
}

impl M5WriteScopeNodeExportField {
    /// Every node export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NodeIdentity,
        Self::ChangeType,
        Self::ActorProvenance,
        Self::ContentClass,
        Self::NodeDisposition,
        Self::ExclusionReason,
        Self::IsIncludedInApply,
        Self::AvailableActions,
    ];

    /// The node export fields every node must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::NodeIdentity,
        Self::ChangeType,
        Self::ActorProvenance,
        Self::NodeDisposition,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NodeIdentity => "node_identity",
            Self::ChangeType => "change_type",
            Self::ActorProvenance => "actor_provenance",
            Self::ContentClass => "content_class",
            Self::NodeDisposition => "node_disposition",
            Self::ExclusionReason => "exclusion_reason",
            Self::IsIncludedInApply => "is_included_in_apply",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when a managed-file caveat marks a generated, managed, vendored, protected, or
/// ignored file (anything other than a plain unmanaged file).
pub const fn caveat_is_managed(caveat: M5ManagedFileCaveat) -> bool {
    !matches!(caveat, M5ManagedFileCaveat::Unmanaged)
}

/// True when a write-scope class reaches beyond a single file into a broad, multi-root, or
/// cross-package change.
pub const fn scope_class_is_broad(class: M5WriteScopeClass) -> bool {
    matches!(
        class,
        M5WriteScopeClass::MultiFile
            | M5WriteScopeClass::WholeDirectory
            | M5WriteScopeClass::CrossPackage
    )
}

// ---- write-scope-preview-tree resolver ----------------------------------

/// The full input to the write-scope-preview-tree resolver for one change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopePreviewTreeResolutionInput {
    /// The write-scope class the change reaches.
    pub write_scope_class: M5WriteScopeClass,
    /// The mutation class the change belongs to.
    pub mutation_class: M5MutationClass,
    /// The total number of files in the preview (including excluded ones).
    pub total_file_count: u32,
    /// The number of files that will be written on apply.
    pub included_file_count: u32,
    /// The number of files present but held out of the apply.
    pub excluded_file_count: u32,
    /// The number of distinct workspace roots the change groups under.
    pub distinct_workspace_root_count: u32,
    /// True when the change touches generated or managed files.
    pub touches_generated_or_managed: bool,
    /// True when the change writes outside the workspace root.
    pub has_out_of_workspace_target: bool,
    /// True when any file is held behind a pending conflict.
    pub has_conflict: bool,
    /// True when any file is blocked by policy.
    pub has_policy_blocked: bool,
    /// True when the scope is reviewable (the user can inspect and narrow it).
    pub scope_is_reviewable: bool,
    /// True when the apply path for this change is available.
    pub apply_path_ready: bool,
    /// The opaque scope label (must be non-empty).
    pub scope_label: String,
}

/// The resolved write-scope-preview-tree truth for one change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWriteScopePreviewTree {
    /// The write-scope class the change reaches.
    pub write_scope_class: M5WriteScopeClass,
    /// The mutation class the change belongs to.
    pub mutation_class: M5MutationClass,
    /// The total number of files in the preview.
    pub total_file_count: u32,
    /// The number of files that will be written on apply.
    pub included_file_count: u32,
    /// The number of files present but held out of the apply.
    pub excluded_file_count: u32,
    /// The number of distinct workspace roots the change groups under.
    pub distinct_workspace_root_count: u32,
    /// True when the change touches generated or managed files.
    pub touches_generated_or_managed: bool,
    /// True when the change writes outside the workspace root.
    pub has_out_of_workspace_target: bool,
    /// The opaque scope label, preserved exactly from the input.
    pub scope_label: String,
    /// The derived tree posture.
    pub tree_posture: M5WriteScopeTreePosture,
    /// The derived file-count bucket.
    pub file_count_bucket: M5WriteScopeFileCountBucket,
    /// The bounded actions this tree offers.
    pub available_actions: Vec<M5WriteScopeTreeAction>,
    /// True when the apply can commit.
    pub can_apply: bool,
    /// True when the scope can narrow below apply-all.
    pub can_narrow: bool,
    /// True when the tree needs operator attention before an apply commits.
    pub needs_attention: bool,
    /// Always true: the tree preserves every file in the preview (ineligible files are never
    /// dropped).
    pub preserves_all_files: bool,
    /// Always false: the tree never understates its write scope.
    pub understates_scope: bool,
}

/// Errors returned by [`resolve_write_scope_preview_tree`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WriteScopePreviewTreeResolutionError {
    /// The scope label was empty.
    EmptyScopeLabel,
    /// The included / excluded counts do not sum to the total.
    FileCountMismatch,
    /// A tree descriptor carried forbidden material.
    ForbiddenTreeMaterial,
}

impl M5WriteScopePreviewTreeResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyScopeLabel => "empty_scope_label",
            Self::FileCountMismatch => "file_count_mismatch",
            Self::ForbiddenTreeMaterial => "forbidden_tree_material",
        }
    }
}

impl fmt::Display for M5WriteScopePreviewTreeResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "write scope preview tree resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WriteScopePreviewTreeResolutionError {}

/// Resolves one write-scope preview tree from its declared change scope.
///
/// The derived tree posture is computed in a fixed blocking-first order: an unavailable apply
/// path wins first, then a pending conflict that must resolve first, then an out-of-workspace
/// target, then a generated-or-managed tree, then a broad multi-file / multi-root change, and
/// otherwise a focused change. The file-count bucket is derived from the honest total —
/// including the files that will not be written — so the blast radius is never understated by
/// counting only the applied files. Every file stays in the preview and the scope is never
/// understated.
pub fn resolve_write_scope_preview_tree(
    input: &M5WriteScopePreviewTreeResolutionInput,
) -> Result<M5ResolvedWriteScopePreviewTree, M5WriteScopePreviewTreeResolutionError> {
    if input.scope_label.trim().is_empty() {
        return Err(M5WriteScopePreviewTreeResolutionError::EmptyScopeLabel);
    }
    if input.included_file_count + input.excluded_file_count != input.total_file_count {
        return Err(M5WriteScopePreviewTreeResolutionError::FileCountMismatch);
    }
    if value_repr_is_forbidden(&input.scope_label) {
        return Err(M5WriteScopePreviewTreeResolutionError::ForbiddenTreeMaterial);
    }

    let file_count_bucket = M5WriteScopeFileCountBucket::from_total(input.total_file_count);
    let tree_posture = derive_tree_posture(
        input.write_scope_class,
        file_count_bucket,
        input.distinct_workspace_root_count,
        input.touches_generated_or_managed,
        input.has_out_of_workspace_target,
        input.has_conflict,
        input.apply_path_ready,
    );
    let can_apply = tree_posture.can_apply();
    let can_narrow = input.scope_is_reviewable
        && (input.included_file_count > 1 || input.excluded_file_count > 0);
    let available_actions = derive_tree_actions(
        tree_posture,
        can_apply,
        can_narrow,
        input.scope_is_reviewable,
        input.touches_generated_or_managed,
    );

    Ok(M5ResolvedWriteScopePreviewTree {
        write_scope_class: input.write_scope_class,
        mutation_class: input.mutation_class,
        total_file_count: input.total_file_count,
        included_file_count: input.included_file_count,
        excluded_file_count: input.excluded_file_count,
        distinct_workspace_root_count: input.distinct_workspace_root_count,
        touches_generated_or_managed: input.touches_generated_or_managed,
        has_out_of_workspace_target: input.has_out_of_workspace_target,
        scope_label: input.scope_label.clone(),
        tree_posture,
        file_count_bucket,
        available_actions,
        can_apply,
        can_narrow,
        needs_attention: tree_posture.needs_attention(),
        preserves_all_files: true,
        understates_scope: false,
    })
}

/// The fixed blocking-first tree-posture ladder.
fn derive_tree_posture(
    write_scope_class: M5WriteScopeClass,
    file_count_bucket: M5WriteScopeFileCountBucket,
    distinct_workspace_root_count: u32,
    touches_generated_or_managed: bool,
    has_out_of_workspace_target: bool,
    has_conflict: bool,
    apply_path_ready: bool,
) -> M5WriteScopeTreePosture {
    if !apply_path_ready {
        M5WriteScopeTreePosture::BlockedScope
    } else if has_conflict {
        M5WriteScopeTreePosture::ConflictScope
    } else if has_out_of_workspace_target
        || matches!(write_scope_class, M5WriteScopeClass::OutOfWorkspace)
    {
        M5WriteScopeTreePosture::OutOfWorkspaceScope
    } else if touches_generated_or_managed
        || matches!(write_scope_class, M5WriteScopeClass::GeneratedTree)
    {
        M5WriteScopeTreePosture::GeneratedManagedScope
    } else if scope_class_is_broad(write_scope_class)
        || distinct_workspace_root_count > 1
        || file_count_bucket.is_multi_file()
    {
        M5WriteScopeTreePosture::BroadScope
    } else {
        M5WriteScopeTreePosture::FocusedScope
    }
}

/// Derives the bounded tree action set from the posture and reviewable / narrowable / managed
/// signals.
///
/// Inspect-tree and expand-all are always offered so the scope is always fully inspectable;
/// jump-to-diff follows the reviewable signal; narrow-scope follows the narrowable state;
/// exclude-generated is offered only for a generated / managed scope that can apply;
/// apply-scope follows the appliable state; resolve-conflict is offered only for a conflict
/// scope.
fn derive_tree_actions(
    posture: M5WriteScopeTreePosture,
    can_apply: bool,
    can_narrow: bool,
    scope_is_reviewable: bool,
    touches_generated_or_managed: bool,
) -> Vec<M5WriteScopeTreeAction> {
    use M5WriteScopeTreeAction as Action;
    let mut actions = vec![Action::InspectTree, Action::ExpandAll];
    if scope_is_reviewable {
        actions.push(Action::JumpToDiff);
    }
    if can_narrow {
        actions.push(Action::NarrowScope);
    }
    if touches_generated_or_managed && can_apply {
        actions.push(Action::ExcludeGenerated);
    }
    if can_apply {
        actions.push(Action::ApplyScope);
    }
    if matches!(posture, M5WriteScopeTreePosture::ConflictScope) {
        actions.push(Action::ResolveConflict);
    }
    actions
}

// ---- write-scope-file-node resolver -------------------------------------

/// The full input to the write-scope-file-node resolver for one file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopeFileNodeResolutionInput {
    /// The change type applied to the file.
    pub change_type: M5WriteScopeChangeType,
    /// The actor that authored the change.
    pub change_actor: M5WriteScopeChangeActor,
    /// The content class of the file.
    pub content_class: M5WriteScopeFileContentClass,
    /// The managed-file caveat of the file.
    pub managed_caveat: M5ManagedFileCaveat,
    /// True when policy blocks writing this file.
    pub is_policy_blocked: bool,
    /// True when the file is read-only / protected.
    pub is_read_only: bool,
    /// True when the file is held behind a pending conflict.
    pub has_conflict: bool,
    /// True when the file writes outside the workspace root.
    pub is_out_of_workspace: bool,
    /// True when the caller opted this (otherwise-includible) file out of the apply.
    pub opt_out_of_apply: bool,
    /// True when a diff is available for this file.
    pub diff_available: bool,
    /// The opaque node label / path (must be non-empty).
    pub node_label: String,
}

/// The resolved write-scope-file-node truth for one file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWriteScopeFileNode {
    /// The change type applied to the file.
    pub change_type: M5WriteScopeChangeType,
    /// The actor that authored the change.
    pub change_actor: M5WriteScopeChangeActor,
    /// The content class of the file.
    pub content_class: M5WriteScopeFileContentClass,
    /// The managed-file caveat of the file.
    pub managed_caveat: M5ManagedFileCaveat,
    /// The opaque node label / path, preserved exactly from the input.
    pub node_label: String,
    /// The derived node disposition.
    pub node_disposition: M5WriteScopeNodeDisposition,
    /// The exact exclusion reason, present only when the file is out of scope.
    pub exclusion_reason: Option<M5WriteScopeExclusionReason>,
    /// The bounded actions this node offers.
    pub available_actions: Vec<M5WriteScopeNodeAction>,
    /// True when the file will be written on apply.
    pub is_included_in_apply: bool,
    /// True when a diff jump is reachable for this file.
    pub can_jump_to_diff: bool,
    /// True when the file touches generated or managed content.
    pub touches_generated_or_managed: bool,
    /// Always true: the node stays present in the preview even when ineligible.
    pub preserves_file_in_preview: bool,
    /// Always false: the node never flattens its provenance or exclusion into a generic
    /// entry.
    pub flattens_into_generic_entry: bool,
}

/// Errors returned by [`resolve_write_scope_file_node`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WriteScopeFileNodeResolutionError {
    /// The node label was empty.
    EmptyNodeLabel,
    /// A node descriptor carried forbidden material.
    ForbiddenNodeMaterial,
}

impl M5WriteScopeFileNodeResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyNodeLabel => "empty_node_label",
            Self::ForbiddenNodeMaterial => "forbidden_node_material",
        }
    }
}

impl fmt::Display for M5WriteScopeFileNodeResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "write scope file node resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WriteScopeFileNodeResolutionError {}

/// Resolves one write-scope file node from its declared change state.
///
/// The derived node disposition is computed in a fixed blocking-first order: a policy-blocked
/// file wins first, then a conflict-held file, then a read-only / protected file, then a
/// generated / managed (excludable) file, then a binary file kept in scope, and otherwise an
/// ordinary included change. The exclusion reason is surfaced exactly whenever the file is
/// out of the apply — a hard exclusion always, and an opted-out generated or out-of-workspace
/// file too. The file always stays present in the preview and always exposes its actor
/// provenance; a binary, metadata-only, read-only, generated, or policy-blocked file is never
/// silently dropped.
pub fn resolve_write_scope_file_node(
    input: &M5WriteScopeFileNodeResolutionInput,
) -> Result<M5ResolvedWriteScopeFileNode, M5WriteScopeFileNodeResolutionError> {
    if input.node_label.trim().is_empty() {
        return Err(M5WriteScopeFileNodeResolutionError::EmptyNodeLabel);
    }
    if value_repr_is_forbidden(&input.node_label) {
        return Err(M5WriteScopeFileNodeResolutionError::ForbiddenNodeMaterial);
    }

    let touches_generated_or_managed = caveat_is_managed(input.managed_caveat)
        || matches!(
            input.content_class,
            M5WriteScopeFileContentClass::GeneratedOutput
        );
    let node_disposition = derive_node_disposition(
        input.content_class,
        input.managed_caveat,
        input.is_policy_blocked,
        input.is_read_only,
        input.has_conflict,
    );
    let is_included_in_apply = !node_disposition.is_hard_excluded() && !input.opt_out_of_apply;
    let exclusion_reason = derive_exclusion_reason(
        node_disposition,
        is_included_in_apply,
        input.opt_out_of_apply,
        input.is_out_of_workspace,
    );
    let can_jump_to_diff = input.diff_available;
    let can_toggle_include = !node_disposition.is_hard_excluded();
    let available_actions = derive_node_actions(
        node_disposition,
        can_jump_to_diff,
        can_toggle_include,
        exclusion_reason.is_some(),
    );

    Ok(M5ResolvedWriteScopeFileNode {
        change_type: input.change_type,
        change_actor: input.change_actor,
        content_class: input.content_class,
        managed_caveat: input.managed_caveat,
        node_label: input.node_label.clone(),
        node_disposition,
        exclusion_reason,
        available_actions,
        is_included_in_apply,
        can_jump_to_diff,
        touches_generated_or_managed,
        preserves_file_in_preview: true,
        flattens_into_generic_entry: false,
    })
}

/// The fixed blocking-first node-disposition ladder.
fn derive_node_disposition(
    content_class: M5WriteScopeFileContentClass,
    managed_caveat: M5ManagedFileCaveat,
    is_policy_blocked: bool,
    is_read_only: bool,
    has_conflict: bool,
) -> M5WriteScopeNodeDisposition {
    if is_policy_blocked {
        M5WriteScopeNodeDisposition::PolicyBlockedExcluded
    } else if has_conflict {
        M5WriteScopeNodeDisposition::ConflictHeld
    } else if is_read_only {
        M5WriteScopeNodeDisposition::ReadOnlyExcluded
    } else if caveat_is_managed(managed_caveat)
        || matches!(content_class, M5WriteScopeFileContentClass::GeneratedOutput)
    {
        M5WriteScopeNodeDisposition::GeneratedExcludable
    } else if matches!(content_class, M5WriteScopeFileContentClass::BinaryBlob) {
        M5WriteScopeNodeDisposition::BinaryIncluded
    } else {
        M5WriteScopeNodeDisposition::IncludedInScope
    }
}

/// Derives the exact exclusion reason for a node, present only when the file is out of scope.
fn derive_exclusion_reason(
    disposition: M5WriteScopeNodeDisposition,
    is_included_in_apply: bool,
    opt_out_of_apply: bool,
    is_out_of_workspace: bool,
) -> Option<M5WriteScopeExclusionReason> {
    use M5WriteScopeExclusionReason as Reason;
    if is_included_in_apply {
        return None;
    }
    match disposition {
        M5WriteScopeNodeDisposition::PolicyBlockedExcluded => Some(Reason::PolicyBlocked),
        M5WriteScopeNodeDisposition::ConflictHeld => Some(Reason::ConflictPending),
        M5WriteScopeNodeDisposition::ReadOnlyExcluded => Some(Reason::ReadOnlyProtected),
        _ if opt_out_of_apply && is_out_of_workspace => Some(Reason::OutOfWorkspace),
        _ if opt_out_of_apply => Some(Reason::GeneratedOptedOut),
        _ => None,
    }
}

/// Derives the bounded node action set.
///
/// View-provenance is always offered so actor provenance is always inspectable; jump-to-diff
/// follows diff availability; toggle-include is offered only for a file that is not hard-
/// excluded; view-exclusion-reason is offered whenever the file is excluded; resolve-conflict
/// is offered only for a conflict-held file.
fn derive_node_actions(
    disposition: M5WriteScopeNodeDisposition,
    can_jump_to_diff: bool,
    can_toggle_include: bool,
    is_excluded: bool,
) -> Vec<M5WriteScopeNodeAction> {
    use M5WriteScopeNodeAction as Action;
    let mut actions = Vec::new();
    if can_jump_to_diff {
        actions.push(Action::JumpToDiff);
    }
    actions.push(Action::ViewProvenance);
    if can_toggle_include {
        actions.push(Action::ToggleInclude);
    }
    if is_excluded {
        actions.push(Action::ViewExclusionReason);
    }
    if matches!(disposition, M5WriteScopeNodeDisposition::ConflictHeld) {
        actions.push(Action::ResolveConflict);
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked write-scope-preview-tree resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopePreviewTreeResolutionCase {
    /// The resolver input.
    pub input: M5WriteScopePreviewTreeResolutionInput,
    /// The resolved truth. Must equal `resolve_write_scope_preview_tree(&input)`.
    pub resolved: M5ResolvedWriteScopePreviewTree,
}

impl M5WriteScopePreviewTreeResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5WriteScopePreviewTreeResolutionInput) -> Self {
        let resolved = resolve_write_scope_preview_tree(&input).expect("seed tree case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_write_scope_preview_tree(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved scope label preserves the input label exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.scope_label == self.input.scope_label
    }
}

/// One worked write-scope-file-node resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopeFileNodeResolutionCase {
    /// The resolver input.
    pub input: M5WriteScopeFileNodeResolutionInput,
    /// The resolved truth. Must equal `resolve_write_scope_file_node(&input)`.
    pub resolved: M5ResolvedWriteScopeFileNode,
}

impl M5WriteScopeFileNodeResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5WriteScopeFileNodeResolutionInput) -> Self {
        let resolved = resolve_write_scope_file_node(&input).expect("seed node case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_write_scope_file_node(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved node label preserves the input label exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.node_label == self.input.node_label
    }
}

/// One row in the primitive matrix: one multi-file change consumer bound to the shared tree
/// and node anatomy, write-scope classes, managed caveats, change types, change actors,
/// content classes, tree postures, node dispositions, file-count buckets, exclusion reasons,
/// bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopePreviewTreeRow {
    /// Multi-file change consumer family.
    pub consumer_surface: M5WriteScopeConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5HistoryQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 mutation / recovery surface families that render / consume these
    /// components.
    pub surface_families: Vec<M5HistorySurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5HistoryDeploymentLine>,
    /// Tree anatomy parts this row renders (must include the mandatory parts).
    pub tree_anatomy_parts: Vec<M5WriteScopeTreeAnatomyPart>,
    /// Node anatomy parts this row renders (must include the mandatory parts).
    pub node_anatomy_parts: Vec<M5WriteScopeNodeAnatomyPart>,
    /// Write-scope classes this consumer distinguishes.
    pub write_scope_classes: Vec<M5WriteScopeClass>,
    /// Mutation classes this consumer distinguishes.
    pub mutation_classes: Vec<M5MutationClass>,
    /// Managed-file caveats this consumer distinguishes.
    pub managed_caveats: Vec<M5ManagedFileCaveat>,
    /// Change types this consumer distinguishes.
    pub change_types: Vec<M5WriteScopeChangeType>,
    /// Change actors this consumer distinguishes.
    pub change_actors: Vec<M5WriteScopeChangeActor>,
    /// Content classes this consumer distinguishes.
    pub content_classes: Vec<M5WriteScopeFileContentClass>,
    /// File-count buckets this consumer distinguishes.
    pub file_count_buckets: Vec<M5WriteScopeFileCountBucket>,
    /// Tree postures this consumer distinguishes.
    pub tree_postures: Vec<M5WriteScopeTreePosture>,
    /// Node dispositions this consumer distinguishes.
    pub node_dispositions: Vec<M5WriteScopeNodeDisposition>,
    /// Exclusion reasons this consumer discloses.
    pub exclusion_reasons: Vec<M5WriteScopeExclusionReason>,
    /// Bounded tree actions this consumer offers.
    pub tree_actions: Vec<M5WriteScopeTreeAction>,
    /// Bounded node actions this consumer offers.
    pub node_actions: Vec<M5WriteScopeNodeAction>,
    /// Tree export fields this row carries (must include the mandatory fields).
    pub tree_export_fields: Vec<M5WriteScopeTreeExportField>,
    /// Node export fields this row carries (must include the mandatory fields).
    pub node_export_fields: Vec<M5WriteScopeNodeExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5HistoryAccessibilityRoute>,
    /// Mutation / recovery subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5HistoryConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5HistoryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked write-scope-preview-tree resolutions proving the tree resolver on this consumer.
    pub tree_examples: Vec<M5WriteScopePreviewTreeResolutionCase>,
    /// Worked write-scope-file-node resolutions proving the node resolver on this consumer.
    pub node_examples: Vec<M5WriteScopeFileNodeResolutionCase>,
    /// Hard invariant: this consumer never flattens its files into a generic list. MUST be
    /// `false`.
    pub flattens_into_generic_file_list: bool,
    /// Hard invariant: this consumer never drops ineligible files from the preview. MUST be
    /// `false`.
    pub drops_ineligible_files: bool,
    /// Hard invariant: this consumer never understates its write scope. MUST be `false`.
    pub understates_write_scope: bool,
    /// Hard invariant: this consumer never hides a file node's actor provenance. MUST be
    /// `false`.
    pub hides_actor_provenance: bool,
}

impl M5WriteScopePreviewTreeRow {
    /// True when the row declares every mandatory tree anatomy part.
    fn declares_mandatory_tree_anatomy(&self) -> bool {
        let present: BTreeSet<M5WriteScopeTreeAnatomyPart> =
            self.tree_anatomy_parts.iter().copied().collect();
        M5WriteScopeTreeAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory node anatomy part.
    fn declares_mandatory_node_anatomy(&self) -> bool {
        let present: BTreeSet<M5WriteScopeNodeAnatomyPart> =
            self.node_anatomy_parts.iter().copied().collect();
        M5WriteScopeNodeAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory tree export field.
    fn declares_mandatory_tree_export(&self) -> bool {
        let present: BTreeSet<M5WriteScopeTreeExportField> =
            self.tree_export_fields.iter().copied().collect();
        M5WriteScopeTreeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory node export field.
    fn declares_mandatory_node_export(&self) -> bool {
        let present: BTreeSet<M5WriteScopeNodeExportField> =
            self.node_export_fields.iter().copied().collect();
        M5WriteScopeNodeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.flattens_into_generic_file_list
            && !self.drops_ineligible_files
            && !self.understates_write_scope
            && !self.hides_actor_provenance
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopePreviewTreeVocabularySet {
    /// Multi-file-change-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Tree-anatomy-part tokens.
    pub tree_anatomy_parts: Vec<String>,
    /// Node-anatomy-part tokens.
    pub node_anatomy_parts: Vec<String>,
    /// Tree-posture tokens.
    pub tree_postures: Vec<String>,
    /// File-count-bucket tokens.
    pub file_count_buckets: Vec<String>,
    /// Node-disposition tokens.
    pub node_dispositions: Vec<String>,
    /// Exclusion-reason tokens.
    pub exclusion_reasons: Vec<String>,
    /// Change-type tokens.
    pub change_types: Vec<String>,
    /// Change-actor tokens.
    pub change_actors: Vec<String>,
    /// Content-class tokens.
    pub content_classes: Vec<String>,
    /// Tree-action tokens.
    pub tree_actions: Vec<String>,
    /// Node-action tokens.
    pub node_actions: Vec<String>,
    /// Tree-export-field tokens.
    pub tree_export_fields: Vec<String>,
    /// Node-export-field tokens.
    pub node_export_fields: Vec<String>,
    /// Write-scope-class tokens (reused from the frozen matrix).
    pub write_scope_classes: Vec<String>,
    /// Managed-file-caveat tokens (reused from the frozen matrix).
    pub managed_caveats: Vec<String>,
    /// Mutation-class tokens (reused from the frozen matrix).
    pub mutation_classes: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5WriteScopePreviewTreeVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5WriteScopeConsumerSurface::ALL, |v| v.as_str()),
            tree_anatomy_parts: tokens(&M5WriteScopeTreeAnatomyPart::ALL, |v| v.as_str()),
            node_anatomy_parts: tokens(&M5WriteScopeNodeAnatomyPart::ALL, |v| v.as_str()),
            tree_postures: tokens(&M5WriteScopeTreePosture::ALL, |v| v.as_str()),
            file_count_buckets: tokens(&M5WriteScopeFileCountBucket::ALL, |v| v.as_str()),
            node_dispositions: tokens(&M5WriteScopeNodeDisposition::ALL, |v| v.as_str()),
            exclusion_reasons: tokens(&M5WriteScopeExclusionReason::ALL, |v| v.as_str()),
            change_types: tokens(&M5WriteScopeChangeType::ALL, |v| v.as_str()),
            change_actors: tokens(&M5WriteScopeChangeActor::ALL, |v| v.as_str()),
            content_classes: tokens(&M5WriteScopeFileContentClass::ALL, |v| v.as_str()),
            tree_actions: tokens(&M5WriteScopeTreeAction::ALL, |v| v.as_str()),
            node_actions: tokens(&M5WriteScopeNodeAction::ALL, |v| v.as_str()),
            tree_export_fields: tokens(&M5WriteScopeTreeExportField::ALL, |v| v.as_str()),
            node_export_fields: tokens(&M5WriteScopeNodeExportField::ALL, |v| v.as_str()),
            write_scope_classes: tokens(&M5WriteScopeClass::ALL, |v| v.as_str()),
            managed_caveats: tokens(&M5ManagedFileCaveat::ALL, |v| v.as_str()),
            mutation_classes: tokens(&M5MutationClass::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5HistoryAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5WriteScopePreviewTreeGovernanceReview {
    /// One primitive pair carries tree and node truth on every consumer.
    pub one_primitive_carries_tree_and_node_truth: bool,
    /// The write scope is never understated (file-count bucket over the honest total).
    pub write_scope_never_understated: bool,
    /// The file-count bucket is always shown.
    pub file_count_bucket_always_shown: bool,
    /// Workspace-root grouping is always shown.
    pub workspace_root_grouping_always_shown: bool,
    /// Actor provenance is always attributable on every file node.
    pub actor_provenance_always_attributable: bool,
    /// Generated, read-only, conflict, and policy-blocked truth is never flattened away.
    pub generated_readonly_conflict_never_flattened: bool,
    /// Ineligible files are never dropped from the preview.
    pub ineligible_files_never_dropped: bool,
    /// Every exclusion carries an explicit reason where safe.
    pub exclusion_reason_always_explicit: bool,
    /// The scope can be inspected and narrowed without losing file-count truth.
    pub scope_inspectable_and_narrowable: bool,
    /// A diff jump is reachable where a diff is available.
    pub diff_jump_reachable_where_available: bool,
    /// The support / export packet reconstructs tree and node truth.
    pub support_export_reconstructs_tree_and_node_truth: bool,
    /// No consumer invents a second write-scope grammar.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopePreviewTreeConsumerProjection {
    /// Rename, refactor, replace, import, AI-apply, and repair consumers all consume the
    /// shared primitive pair.
    pub change_surfaces_consume_shared_primitive: bool,
    /// The tree-posture resolver reads a single canonical source.
    pub tree_posture_reads_single_source: bool,
    /// The node-disposition resolver reads a single canonical source.
    pub node_disposition_reads_single_source: bool,
    /// The bounded-action derivation reads a single canonical source.
    pub actions_read_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopePreviewTreeProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopePreviewTreeReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting recovery audit.
    pub recovery_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WriteScopePreviewTreePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WriteScopePreviewTreePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Multi-file change rows.
    pub rows: Vec<M5WriteScopePreviewTreeRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WriteScopePreviewTreeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WriteScopePreviewTreeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WriteScopePreviewTreeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WriteScopePreviewTreeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WriteScopePreviewTreeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 write-scope-preview-tree primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteScopePreviewTreePacket {
    /// Record kind; must equal [`M5_WRITE_SCOPE_PREVIEW_TREE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Multi-file change rows.
    pub rows: Vec<M5WriteScopePreviewTreeRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WriteScopePreviewTreeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WriteScopePreviewTreeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WriteScopePreviewTreeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WriteScopePreviewTreeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WriteScopePreviewTreeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WriteScopePreviewTreePacket {
    /// Builds an M5 write-scope-preview-tree primitive packet from stable-lane input.
    pub fn new(input: M5WriteScopePreviewTreePacketInput) -> Self {
        Self {
            record_kind: M5_WRITE_SCOPE_PREVIEW_TREE_RECORD_KIND.to_owned(),
            schema_version: M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 write-scope-preview-tree primitive invariants.
    pub fn validate(&self) -> Vec<M5WriteScopePreviewTreeViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WRITE_SCOPE_PREVIEW_TREE_RECORD_KIND {
            violations.push(M5WriteScopePreviewTreeViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_VERSION {
            violations.push(M5WriteScopePreviewTreeViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WriteScopePreviewTreeViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_tree_scope_coverage(self, &mut violations);
        validate_tree_managed_caveat_coverage(self, &mut violations);
        validate_tree_file_count_coverage(self, &mut violations);
        validate_tree_apply_coverage(self, &mut violations);
        validate_node_exclusion_coverage(self, &mut violations);
        validate_node_provenance_preservation(self, &mut violations);
        validate_node_ineligible_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 write-scope preview tree packet serializes"),
        ) {
            violations.push(M5WriteScopePreviewTreeViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 write-scope preview tree packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per multi-file change consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,tree_anatomy,node_anatomy,write_scope_classes,file_count_buckets,tree_postures,node_dispositions,exclusion_reasons,tree_actions,node_actions,tree_examples,node_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.tree_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.node_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.write_scope_classes, |v| v.as_str()),
                join_tokens(&row.file_count_buckets, |v| v.as_str()),
                join_tokens(&row.tree_postures, |v| v.as_str()),
                join_tokens(&row.node_dispositions, |v| v.as_str()),
                join_tokens(&row.exclusion_reasons, |v| v.as_str()),
                join_tokens(&row.tree_actions, |v| v.as_str()),
                join_tokens(&row.node_actions, |v| v.as_str()),
                row.tree_examples.len(),
                row.node_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Write-Scope-Preview-Tree Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Multi-file change consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Tree postures: {}\n",
            self.vocabulary_set.tree_postures.join(", ")
        ));
        out.push_str(&format!(
            "- File-count buckets: {}\n",
            self.vocabulary_set.file_count_buckets.join(", ")
        ));
        out.push_str(&format!(
            "- Node dispositions: {}\n",
            self.vocabulary_set.node_dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Multi-file change consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked trees: {}\n", row.tree_examples.len()));
            for case in &row.tree_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (bucket `{}`, apply `{}`, narrow `{}`, managed `{}`)\n",
                    case.resolved.scope_label,
                    case.resolved.write_scope_class.as_str(),
                    case.resolved.tree_posture.as_str(),
                    case.resolved.file_count_bucket.as_str(),
                    case.resolved.can_apply,
                    case.resolved.can_narrow,
                    case.resolved.touches_generated_or_managed,
                ));
            }
            out.push_str(&format!("  - Worked nodes: {}\n", row.node_examples.len()));
            for case in &row.node_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (included `{}`, reason `{}`, actor `{}`)\n",
                    case.resolved.node_label,
                    case.resolved.change_type.as_str(),
                    case.resolved.node_disposition.as_str(),
                    case.resolved.is_included_in_apply,
                    case.resolved
                        .exclusion_reason
                        .map(|r| r.as_str())
                        .unwrap_or("none"),
                    case.resolved.change_actor.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 write-scope-preview-tree export.
#[derive(Debug)]
pub enum M5WriteScopePreviewTreeArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WriteScopePreviewTreeViolation>),
}

impl fmt::Display for M5WriteScopePreviewTreeArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 write-scope preview tree export parse failed: {error}"
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
                    "m5 write-scope preview tree export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WriteScopePreviewTreeArtifactError {}

/// Validation failures emitted by [`M5WriteScopePreviewTreePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WriteScopePreviewTreeViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required multi-file change consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A multi-file change row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory tree anatomy parts.
    MandatoryTreeAnatomyMissing,
    /// A row omits one of the mandatory node anatomy parts.
    MandatoryNodeAnatomyMissing,
    /// A row omits one of the mandatory tree export fields.
    MandatoryTreeExportMissing,
    /// A row omits one of the mandatory node export fields.
    MandatoryNodeExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked tree resolutions.
    TreeExampleMissing,
    /// A row declares no worked node resolutions.
    NodeExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked tree resolution proves both a focused scope and a broad / out-of-workspace /
    /// generated scope.
    TreeScopeCoverageUnproven,
    /// No worked tree resolution proves a generated-or-managed-file scope.
    TreeManagedCaveatCoverageUnproven,
    /// No worked tree resolution proves both a single-file and a multi-file bucket.
    TreeFileCountCoverageUnproven,
    /// No worked tree resolution proves both an appliable and a blocked / conflict scope.
    TreeApplyCoverageUnproven,
    /// No worked node resolution proves both an excluded-with-reason and an included file.
    NodeExclusionCoverageUnproven,
    /// A worked node resolution does not preserve the file in the preview.
    NodeProvenancePreservationUnproven,
    /// No worked node resolution proves an ineligible file (policy-blocked, read-only,
    /// binary, or metadata-only) preserved in the preview.
    NodeIneligiblePreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
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

impl M5WriteScopePreviewTreeViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryTreeAnatomyMissing => "mandatory_tree_anatomy_missing",
            Self::MandatoryNodeAnatomyMissing => "mandatory_node_anatomy_missing",
            Self::MandatoryTreeExportMissing => "mandatory_tree_export_missing",
            Self::MandatoryNodeExportMissing => "mandatory_node_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::TreeExampleMissing => "tree_example_missing",
            Self::NodeExampleMissing => "node_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::TreeScopeCoverageUnproven => "tree_scope_coverage_unproven",
            Self::TreeManagedCaveatCoverageUnproven => "tree_managed_caveat_coverage_unproven",
            Self::TreeFileCountCoverageUnproven => "tree_file_count_coverage_unproven",
            Self::TreeApplyCoverageUnproven => "tree_apply_coverage_unproven",
            Self::NodeExclusionCoverageUnproven => "node_exclusion_coverage_unproven",
            Self::NodeProvenancePreservationUnproven => "node_provenance_preservation_unproven",
            Self::NodeIneligiblePreservationUnproven => "node_ineligible_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 write-scope-preview-tree export.
pub fn current_stable_m5_write_scope_preview_tree_export(
) -> Result<M5WriteScopePreviewTreePacket, M5WriteScopePreviewTreeArtifactError> {
    let packet: M5WriteScopePreviewTreePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-write-scope-preview-tree-primitive-proof/support_export.json"
    )))
    .map_err(M5WriteScopePreviewTreeArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WriteScopePreviewTreeArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_DOC_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_COMPONENT_MATRIX_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_WRITE_BOUNDARY_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_REFACTOR_PREVIEW_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5WriteScopePreviewTreeViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5WriteScopePreviewTreeViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let present: BTreeSet<M5WriteScopeConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5WriteScopeConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5WriteScopePreviewTreeViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.tree_anatomy_parts.is_empty()
            || row.node_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.write_scope_classes.is_empty()
            || row.mutation_classes.is_empty()
            || row.managed_caveats.is_empty()
            || row.change_types.is_empty()
            || row.change_actors.is_empty()
            || row.content_classes.is_empty()
            || row.file_count_buckets.is_empty()
            || row.tree_postures.is_empty()
            || row.node_dispositions.is_empty()
            || row.exclusion_reasons.is_empty()
            || row.tree_actions.is_empty()
            || row.node_actions.is_empty()
        {
            violations.push(M5WriteScopePreviewTreeViolation::RowIncomplete);
        }
        if !row.declares_mandatory_tree_anatomy() {
            violations.push(M5WriteScopePreviewTreeViolation::MandatoryTreeAnatomyMissing);
        }
        if !row.declares_mandatory_node_anatomy() {
            violations.push(M5WriteScopePreviewTreeViolation::MandatoryNodeAnatomyMissing);
        }
        if !row.declares_mandatory_tree_export() {
            violations.push(M5WriteScopePreviewTreeViolation::MandatoryTreeExportMissing);
        }
        if !row.declares_mandatory_node_export() {
            violations.push(M5WriteScopePreviewTreeViolation::MandatoryNodeExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5WriteScopePreviewTreeViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5WriteScopePreviewTreeViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5WriteScopePreviewTreeViolation::DowngradeTriggersMissing);
        }
        if row.tree_examples.is_empty() {
            violations.push(M5WriteScopePreviewTreeViolation::TreeExampleMissing);
        }
        if row.node_examples.is_empty() {
            violations.push(M5WriteScopePreviewTreeViolation::NodeExampleMissing);
        }
        if row
            .tree_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .node_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5WriteScopePreviewTreeViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5WriteScopePreviewTreeViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5WriteScopePreviewTreeViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked tree resolution across the matrix must prove a focused scope and at
/// least one must prove a broad, out-of-workspace, or generated / managed scope — the
/// acceptance-criterion example that a broad change is shown honestly, never as a focused one.
fn validate_tree_scope_coverage(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let has_focused = packet.rows.iter().any(|row| {
        row.tree_examples.iter().any(|case| {
            matches!(
                case.resolved.tree_posture,
                M5WriteScopeTreePosture::FocusedScope
            )
        })
    });
    let has_broad = packet.rows.iter().any(|row| {
        row.tree_examples.iter().any(|case| {
            matches!(
                case.resolved.tree_posture,
                M5WriteScopeTreePosture::BroadScope
                    | M5WriteScopeTreePosture::OutOfWorkspaceScope
                    | M5WriteScopeTreePosture::GeneratedManagedScope
            )
        })
    });
    if !(has_focused && has_broad) {
        violations.push(M5WriteScopePreviewTreeViolation::TreeScopeCoverageUnproven);
    }
}

/// At least one worked tree resolution must prove a scope that touches generated or managed
/// files — the acceptance-criterion example that a generated tree is never hidden.
fn validate_tree_managed_caveat_coverage(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.tree_examples
            .iter()
            .any(|case| case.resolved.touches_generated_or_managed)
    });
    if !proven {
        violations.push(M5WriteScopePreviewTreeViolation::TreeManagedCaveatCoverageUnproven);
    }
}

/// At least one worked tree resolution must prove a single-file bucket and at least one must
/// prove a multi-file bucket — the acceptance-criterion example that file-count truth is
/// preserved across scope sizes.
fn validate_tree_file_count_coverage(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let has_single = packet.rows.iter().any(|row| {
        row.tree_examples.iter().any(|case| {
            matches!(
                case.resolved.file_count_bucket,
                M5WriteScopeFileCountBucket::Single
            )
        })
    });
    let has_multi = packet.rows.iter().any(|row| {
        row.tree_examples
            .iter()
            .any(|case| case.resolved.file_count_bucket.is_multi_file())
    });
    if !(has_single && has_multi) {
        violations.push(M5WriteScopePreviewTreeViolation::TreeFileCountCoverageUnproven);
    }
}

/// At least one worked tree resolution must prove an appliable scope and at least one must
/// prove a blocked / conflict scope — the acceptance-criterion example that a tree never
/// claims an apply path it does not have.
fn validate_tree_apply_coverage(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let has_appliable = packet
        .rows
        .iter()
        .any(|row| row.tree_examples.iter().any(|case| case.resolved.can_apply));
    let has_blocked = packet.rows.iter().any(|row| {
        row.tree_examples
            .iter()
            .any(|case| !case.resolved.can_apply)
    });
    if !(has_appliable && has_blocked) {
        violations.push(M5WriteScopePreviewTreeViolation::TreeApplyCoverageUnproven);
    }
}

/// At least one worked node resolution must prove an excluded file that carries an explicit
/// exclusion reason and at least one must prove an included file — the acceptance-criterion
/// example that exclusion reasons stay explicit and are never flattened away.
fn validate_node_exclusion_coverage(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let has_excluded = packet.rows.iter().any(|row| {
        row.node_examples.iter().any(|case| {
            !case.resolved.is_included_in_apply && case.resolved.exclusion_reason.is_some()
        })
    });
    let has_included = packet.rows.iter().any(|row| {
        row.node_examples
            .iter()
            .any(|case| case.resolved.is_included_in_apply)
    });
    if !(has_excluded && has_included) {
        violations.push(M5WriteScopePreviewTreeViolation::NodeExclusionCoverageUnproven);
    }
}

/// Every worked node resolution must keep the file present in the preview — the
/// acceptance-criterion example that ineligible files are never dropped.
fn validate_node_provenance_preservation(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.node_examples.iter())
        .all(|case| case.resolved.preserves_file_in_preview && case.preserves_identity());
    if !preserved {
        violations.push(M5WriteScopePreviewTreeViolation::NodeProvenancePreservationUnproven);
    }
}

/// At least one worked node resolution must prove an ineligible file (policy-blocked,
/// read-only, binary, or metadata-only) still present in the preview — the acceptance-criterion
/// example that policy-blocked, binary, metadata-only, and generated truth is preserved rather
/// than dropped.
fn validate_node_ineligible_preservation(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.node_examples.iter().any(|case| {
            matches!(
                case.resolved.node_disposition,
                M5WriteScopeNodeDisposition::PolicyBlockedExcluded
                    | M5WriteScopeNodeDisposition::ReadOnlyExcluded
                    | M5WriteScopeNodeDisposition::BinaryIncluded
            ) || matches!(
                case.resolved.content_class,
                M5WriteScopeFileContentClass::MetadataOnly
                    | M5WriteScopeFileContentClass::BinaryBlob
            )
        })
    });
    if !proven {
        violations.push(M5WriteScopePreviewTreeViolation::NodeIneligiblePreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_tree_and_node_truth,
        review.write_scope_never_understated,
        review.file_count_bucket_always_shown,
        review.workspace_root_grouping_always_shown,
        review.actor_provenance_always_attributable,
        review.generated_readonly_conflict_never_flattened,
        review.ineligible_files_never_dropped,
        review.exclusion_reason_always_explicit,
        review.scope_inspectable_and_narrowable,
        review.diff_jump_reachable_where_available,
        review.support_export_reconstructs_tree_and_node_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5WriteScopePreviewTreeViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.change_surfaces_consume_shared_primitive,
        projection.tree_posture_reads_single_source,
        projection.node_disposition_reads_single_source,
        projection.actions_read_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5WriteScopePreviewTreeViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5WriteScopePreviewTreeViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WriteScopePreviewTreePacket,
    violations: &mut Vec<M5WriteScopePreviewTreeViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.recovery_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5WriteScopePreviewTreeViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
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
fn value_repr_is_forbidden(value: &str) -> bool {
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
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
