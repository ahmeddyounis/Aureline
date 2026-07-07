//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 local-history / write-scope components.
//!
//! This module is the M05-898 accessibility-and-auto-narrowing capstone over the frozen
//! M5 local-history / write-scope component matrix
//! ([`crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix`]).
//! Where the freeze matrix defines the reusable local-history row, checkpoint-group card,
//! restore-preview card, retention/export card, write-scope preview tree, restore-granularity
//! selector, and history-export manifest primitives, and the 893-897 implementation /
//! consumer lanes resolve their per-surface truth, this lane certifies — per component
//! family — that mutation and recovery claims stay **keyboard-complete,
//! assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than presenting a
//! metadata-only capture, a partial or manual restore, a stale write scope, an unavailable
//! checkpoint, or an export-limited history as a still fully-restorable checkpoint:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same snapshot origin,
//!   actor lineage, capture fidelity, checkpoint lineage, restore granularity, restore drift,
//!   selectable apply scope, write-scope class, managed-file caveat, retention posture, and
//!   export/redaction posture the rich component shows — never a hover-only chip that strands
//!   assistive-tech or headless users. Hierarchy-heavy families (the write-scope preview
//!   tree's nested workspace-root / file-node hierarchy) additionally bind their tree to a
//!   flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each
//!   component's meaning from typed tokens and opaque refs without a screenshot, preserving
//!   the same actor, timestamp, file/object identity, restore class, capture class, stale
//!   scope, and redaction truth shown in-product so history / restore / write-scope truth can
//!   be reconstructed without screenshots or private team memory.
//! - **Honest auto-narrowing.** When capture is metadata-only, a restore is partial or
//!   manual, a write scope is stale, a checkpoint is unavailable, or history export is
//!   redaction-limited, the component's history-support claim auto-narrows from
//!   `RestorableCheckpoint` / `ReviewableHistory` to a narrowed-restore / metadata-only /
//!   stale-scope / unavailable-checkpoint history, discloses the narrowing with a precise
//!   trigger and binding dimension, and preserves the canonical snapshot / actor / file /
//!   checkpoint identity — the underlying history is never erased opaquely. A component with
//!   every dimension intact must NOT carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the editor timeline,
//!   checkpoint inspector, restore review, refactor preview, AI-apply review, recovery
//!   center, headless CLI, and support / release exports so product, docs, and release
//!   publication stay aligned on recovery downgrade behavior rather than drifting in copy — a
//!   restorable-looking checkpoint can never outrun the capture / restore / scope / retention
//!   proof it is being viewed away from.
//!
//! Each [`HistoryComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::M5LocalHistoryWriteScopeComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5HistoryRequiredLabel`] and
//! [`M5HistoryDowngradeTrigger`] and the shared [`M5HistoryConsumerSurface`] consumer
//! surfaces rather than minting parallel synonyms, so the certified labels stay byte-identical
//! to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw file bodies, snapshot contents, diff hunks, and
//! credential-bearing material never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, and redacted labels so support, release,
//! and diagnostics exports can reconstruct exactly what an accessible fallback would have
//! shown without leaking history material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::{
    M5HistoryConsumerSurface, M5HistoryDowngradeTrigger, M5HistoryRequiredLabel,
    M5LocalHistoryWriteScopeComponentFamily,
};

/// Schema version stamped on the M05-898 local-history / write-scope component accessibility
/// fallback packet.
pub const HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`HistoryComponentAccessibilityPacket`].
pub const HISTORY_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_local_history_write_scope_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`HistoryComponentAccessibilityRow`].
pub const HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_local_history_write_scope_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const HISTORY_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/recovery/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_capture_is_metadata_only_restore_is_partial_or_manual_scope_is_stale_or_checkpoints_are_unavailable_across_claimed_m5_recovery_components.md";

/// Repo-relative path of the frozen local-history / write-scope component matrix this lane
/// certifies.
pub const HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const HISTORY_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-local-history-write-scope-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const HISTORY_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const HISTORY_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const HISTORY_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the write-scope
/// preview tree's nested workspace-root / file-node hierarchy) and therefore MUST bind their
/// tree to an equivalent flat list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5LocalHistoryWriteScopeComponentFamily) -> bool {
    matches!(
        family,
        M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree
    )
}

/// The history dimension whose weakening a family primarily discloses. Every row must model
/// at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5LocalHistoryWriteScopeComponentFamily,
) -> M5HistoryClaimDimension {
    match family {
        M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow => {
            M5HistoryClaimDimension::CaptureFidelity
        }
        M5LocalHistoryWriteScopeComponentFamily::CheckpointGroupCard => {
            M5HistoryClaimDimension::CheckpointAvailability
        }
        M5LocalHistoryWriteScopeComponentFamily::RestorePreviewCard => {
            M5HistoryClaimDimension::RestoreGranularity
        }
        M5LocalHistoryWriteScopeComponentFamily::RetentionExportCard => {
            M5HistoryClaimDimension::ExportDisclosure
        }
        M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree => {
            M5HistoryClaimDimension::ScopeFreshness
        }
        M5LocalHistoryWriteScopeComponentFamily::RestoreGranularitySelector => {
            M5HistoryClaimDimension::RestoreScopeSelection
        }
        M5LocalHistoryWriteScopeComponentFamily::HistoryExportManifest => {
            M5HistoryClaimDimension::ManifestExportDisclosure
        }
    }
}

/// A rendered fallback modality for a local-history / write-scope component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryFallbackModality {
    /// A rich, structured (write-scope tree / grouped checkpoint) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5HistoryFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / headless path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only
/// browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryRenderingSurface {
    /// The full-capability desktop recovery surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5HistoryRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::HandoffPacket => "handoff_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl HistoryNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl HistoryExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl HistoryNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The history-support claim ceiling a component asserts: how strong a recovery posture it
/// lets a surface present. Auto-narrowing lowers this ceiling when a history dimension weakens
/// so a metadata-only capture, partial / manual restore, stale scope, unavailable checkpoint,
/// or export-limited history can never keep an old `RestorableCheckpoint` or
/// `ReviewableHistory` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistorySupportClaim {
    /// Restorable checkpoint: a full-fidelity, attributable, in-scope, exportable history
    /// that can be restored or applied as-is — the strongest claim.
    RestorableCheckpoint,
    /// Reviewable history: a self-sufficient, reviewable history / manifest (evidence a
    /// reviewer can read) that is not itself a certified restore claim.
    ReviewableHistory,
    /// Narrowed restore: usable, but restores / applies only a partial or manually chosen
    /// scope rather than the whole snapshot.
    NarrowedRestore,
    /// Metadata-only history: only metadata was captured (no full body); the history is
    /// attributable but cannot reconstruct file contents.
    MetadataOnlyHistory,
    /// Stale-scope history: the write / restore scope has drifted and must be re-resolved
    /// before it can be trusted.
    StaleScopeHistory,
    /// Unavailable checkpoint: the checkpoint is unavailable / expired or history export is
    /// policy-blocked; no restore or export can proceed.
    UnavailableCheckpoint,
}

impl M5HistorySupportClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::RestorableCheckpoint,
        Self::ReviewableHistory,
        Self::NarrowedRestore,
        Self::MetadataOnlyHistory,
        Self::StaleScopeHistory,
        Self::UnavailableCheckpoint,
    ];

    /// Capability rank; a higher rank asserts a stronger recovery posture. Narrowing lowers
    /// rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::RestorableCheckpoint => 5,
            Self::ReviewableHistory => 4,
            Self::NarrowedRestore => 3,
            Self::MetadataOnlyHistory => 2,
            Self::StaleScopeHistory => 1,
            Self::UnavailableCheckpoint => 0,
        }
    }

    /// Returns true when this claim asserts a fully restorable checkpoint.
    pub const fn asserts_restorable(self) -> bool {
        matches!(self, Self::RestorableCheckpoint)
    }

    /// Returns true when this claim asserts a fully self-sufficient (restorable or reviewable)
    /// history.
    pub const fn asserts_full_recovery(self) -> bool {
        matches!(self, Self::RestorableCheckpoint | Self::ReviewableHistory)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestorableCheckpoint => "restorable_checkpoint",
            Self::ReviewableHistory => "reviewable_history",
            Self::NarrowedRestore => "narrowed_restore",
            Self::MetadataOnlyHistory => "metadata_only_history",
            Self::StaleScopeHistory => "stale_scope_history",
            Self::UnavailableCheckpoint => "unavailable_checkpoint",
        }
    }
}

/// The history dimension whose state governs how far a component may claim to be a restorable
/// checkpoint. The five spec axes the lane must auto-narrow on — metadata-only capture,
/// partial / manual restore, stale scope, unavailable checkpoints, and export-limited history
/// — are [`Self::CaptureFidelity`], [`Self::RestoreGranularity`], [`Self::ScopeFreshness`],
/// [`Self::CheckpointAvailability`], and [`Self::ExportDisclosure`]; the remaining dimensions
/// cover the restore-granularity-selector and history-export-manifest families' primary
/// weakening axes so every frozen family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryClaimDimension {
    /// Capture fidelity: did the local-history row capture the full body, or only metadata?
    CaptureFidelity,
    /// Checkpoint availability: is the checkpoint-group card's checkpoint present and
    /// restorable, or expired / unavailable?
    CheckpointAvailability,
    /// Restore granularity: does the restore-preview card restore the whole snapshot, or only
    /// a partial / manually chosen scope?
    RestoreGranularity,
    /// Export disclosure: does the retention/export card export full evidence, or only a
    /// redacted / policy-limited history?
    ExportDisclosure,
    /// Scope freshness: is the write-scope preview tree's scope current, or drifted / stale
    /// and needing re-resolution?
    ScopeFreshness,
    /// Restore scope selection: does the restore-granularity selector offer the whole apply
    /// scope, or only a partial / manual selection?
    RestoreScopeSelection,
    /// Manifest export disclosure: does the history-export manifest carry full lineage, or a
    /// redacted / metadata-only manifest?
    ManifestExportDisclosure,
}

impl M5HistoryClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CaptureFidelity,
        Self::CheckpointAvailability,
        Self::RestoreGranularity,
        Self::ExportDisclosure,
        Self::ScopeFreshness,
        Self::RestoreScopeSelection,
        Self::ManifestExportDisclosure,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a narrowing.
    /// Each dimension maps to the on-topic frozen trigger the freeze matrix already governs,
    /// so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5HistoryDowngradeTrigger {
        match self {
            Self::CaptureFidelity => M5HistoryDowngradeTrigger::CaptureFidelityMasked,
            Self::CheckpointAvailability => M5HistoryDowngradeTrigger::CheckpointLineageUnstated,
            Self::RestoreGranularity => M5HistoryDowngradeTrigger::RestoreGranularityCollapsed,
            Self::ExportDisclosure => M5HistoryDowngradeTrigger::RetentionOrRedactionUndisclosed,
            Self::ScopeFreshness => M5HistoryDowngradeTrigger::WriteScopeUnderstated,
            Self::RestoreScopeSelection => M5HistoryDowngradeTrigger::RestoreGranularityCollapsed,
            Self::ManifestExportDisclosure => {
                M5HistoryDowngradeTrigger::RetentionOrRedactionUndisclosed
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CaptureFidelity => "capture_fidelity",
            Self::CheckpointAvailability => "checkpoint_availability",
            Self::RestoreGranularity => "restore_granularity",
            Self::ExportDisclosure => "export_disclosure",
            Self::ScopeFreshness => "scope_freshness",
            Self::RestoreScopeSelection => "restore_scope_selection",
            Self::ManifestExportDisclosure => "manifest_export_disclosure",
        }
    }
}

/// The observed condition of one history dimension. Anything weaker than [`Self::Captured`]
/// imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryConditionState {
    /// Fully captured / available / in-scope — imposes no ceiling.
    Captured,
    /// Narrowed-restore — only a partial or manually chosen scope restores / applies; support
    /// drops to narrowed-restore.
    NarrowedRestore,
    /// Metadata-only — only metadata was captured, no full body; support drops to
    /// metadata-only.
    MetadataOnly,
    /// Stale-scope — the write / restore scope has drifted and must be re-resolved; support
    /// drops to stale-scope.
    StaleScope,
    /// Unavailable — the checkpoint is unavailable / expired or export is policy-blocked;
    /// support drops to unavailable.
    Unavailable,
}

impl M5HistoryConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Captured,
        Self::NarrowedRestore,
        Self::MetadataOnly,
        Self::StaleScope,
        Self::Unavailable,
    ];

    /// Returns true when the dimension is weaker than captured and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Captured)
    }

    /// The strongest history-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5HistorySupportClaim {
        match self {
            Self::Captured => M5HistorySupportClaim::RestorableCheckpoint,
            Self::NarrowedRestore => M5HistorySupportClaim::NarrowedRestore,
            Self::MetadataOnly => M5HistorySupportClaim::MetadataOnlyHistory,
            Self::StaleScope => M5HistorySupportClaim::StaleScopeHistory,
            Self::Unavailable => M5HistorySupportClaim::UnavailableCheckpoint,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::NarrowedRestore => "narrowed_restore",
            Self::MetadataOnly => "metadata_only",
            Self::StaleScope => "stale_scope",
            Self::Unavailable => "unavailable",
        }
    }
}

/// One history dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5HistoryClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5HistoryConditionState,
}

/// An honest history-support-claim auto-narrow block. When a history dimension weakens, the
/// component's support claim lowers to the permitted ceiling, names the binding dimension and
/// frozen trigger, and preserves the canonical snapshot / actor / file / checkpoint identity
/// rather than silently dropping it — the underlying history is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5HistorySupportClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest
    /// ceiling constraint).
    pub binding_dimension: M5HistoryClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5HistoryDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical snapshot origin, actor, file/object identity, and checkpoint lineage are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying history is preserved (never erased) across the narrowing; must hold so
    /// metadata-only, partial, stale, and unavailable states never fail opaquely.
    pub preserves_history_integrity: bool,
}

impl HistoryClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and history
    /// integrity and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_history_integrity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable
/// as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl HistoryCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered,
    /// at least one export field is named, and screenshots are prohibited as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5HistoryRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: HistoryNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a history accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims recovery, or drops state
    /// silently (red).
    Stranded,
}

impl HistoryComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one local-history / write-scope component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryComponentAccessibilityRow {
    /// Record kind; must equal [`HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5LocalHistoryWriteScopeComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the checkpoint / snapshot / restore / write-scope object this component
    /// acts on; stays visible on every surface, so this is never empty.
    pub history_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5HistoryFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical snapshot, actor, capture, restore,
    /// scope, retention, and export truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: HistoryNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: HistoryNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: HistoryNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: HistoryExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: HistoryCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5HistorySupportClaim,
    /// The observed condition of each modeled history dimension.
    #[serde(default)]
    pub claim_conditions: Vec<HistoryClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<HistoryClaimAutoNarrow>,
    /// Whether the underlying history is preserved on this component regardless of narrowing;
    /// must hold so metadata-only, partial, stale, and unavailable states never fail opaquely.
    pub history_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5HistoryRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<HistoryRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5HistoryRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5HistoryConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl HistoryComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Captured` when the row does not
    /// model that dimension.
    pub fn condition_for(&self, dimension: M5HistoryClaimDimension) -> M5HistoryConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5HistoryConditionState::Captured)
    }

    /// Whether any modeled dimension is weaker than captured.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5HistorySupportClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5HistoryClaimDimension> {
        let mut binding: Option<(M5HistoryClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_support_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition.dimension, rank)),
            }
        }
        binding.map(|(dimension, _)| dimension)
    }

    /// The support claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5HistorySupportClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC / auto-narrowing honesty: a metadata-only capture, partial / manual restore, stale
    /// scope, unavailable checkpoint, or export-limited history can no longer keep an old
    /// `RestorableCheckpoint` / `ReviewableHistory` label. The effective claim never exceeds
    /// the permitted ceiling; when a dimension narrows below the full claim, an honest narrow
    /// block is present, narrows to exactly the permitted ceiling, binds to the
    /// ceiling-imposing dimension with its frozen trigger, and preserves canonical identity
    /// and history integrity. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_dimension()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding
                    && narrow.trigger == binding.default_trigger()
                    && self.condition_for(binding).is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.history_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-erasure: metadata-only, partial, stale, and unavailable states preserve the
    /// underlying history. The row must assert `history_preserved`, and any narrow block must
    /// preserve history integrity too.
    pub fn preserves_history_integrity(&self) -> bool {
        self.history_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_history_integrity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay
    /// aligned on the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5HistoryRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> HistoryComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_history_integrity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return HistoryComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            HistoryComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            HistoryComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.history_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_support_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-898 local-history / write-scope component accessibility
/// fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryComponentAccessibilitySummary {
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_history_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`HistoryComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<HistoryComponentAccessibilityRow>,
}

/// Checked-in M05-898 local-history / write-scope component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<HistoryComponentAccessibilityRow>,
    pub summary: HistoryComponentAccessibilitySummary,
}

impl HistoryComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: HistoryComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: HISTORY_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: HistoryComponentAccessibilitySummary {
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
                all_history_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5LocalHistoryWriteScopeComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5HistoryClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5HistorySupportClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5HistoryConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> HistoryComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5HistoryConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&HistoryComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                HistoryComponentAccessibilityStatus::Parity => green += 1,
                HistoryComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                HistoryComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        HistoryComponentAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(HistoryComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(HistoryComponentAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(HistoryComponentAccessibilityRow::export_preserves_meaning),
            all_history_preserved: self
                .rows
                .iter()
                .all(HistoryComponentAccessibilityRow::preserves_history_integrity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(HistoryComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<HistoryComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(HistoryComponentAccessibilityViolation::SchemaVersion {
                expected: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != HISTORY_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(HistoryComponentAccessibilityViolation::RecordKind {
                expected: HISTORY_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(HistoryComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(HistoryComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(HistoryComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    HistoryComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory history label.
            if !row.preserves_mandatory_labels() {
                violations.push(HistoryComponentAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5HistoryFallbackModality::Structured)
            {
                violations.push(
                    HistoryComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a restorable / reviewable history for a weakened
            // one.
            if !row.claim_is_honest() {
                violations.push(HistoryComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(HistoryComponentAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    HistoryComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: metadata-only, partial, stale, and unavailable states preserve history.
            if !row.preserves_history_integrity() {
                violations.push(HistoryComponentAccessibilityViolation::HistoryErased {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    HistoryComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(HistoryComponentAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == HistoryComponentAccessibilityStatus::Stranded {
                violations.push(HistoryComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5LocalHistoryWriteScopeComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    HistoryComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5HistoryClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    HistoryComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full
        // narrowing spectrum (restorable-checkpoint → … → unavailable-checkpoint) is proven
        // end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5HistorySupportClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    HistoryComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Cross-surface: the same narrowed state must reach the editor timeline, checkpoint
        // inspector, restore review, refactor preview, AI-apply review, recovery center, CLI,
        // and support / release exports — so every consumer surface is exercised at least once
        // across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5HistoryConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    HistoryComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(HistoryComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("local-history / write-scope accessibility fallback packet serializes"),
        ) {
            violations.push(HistoryComponentAccessibilityViolation::RawHistoryMaterialInExport);
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
            .expect("local-history / write-scope accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_support_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Local-History / Write-Scope Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5LocalHistoryWriteScopeComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_support_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in local-history / write-scope component accessibility
/// fallback export.
pub fn current_m5_history_component_a11y_fallback_export(
) -> Result<HistoryComponentAccessibilityPacket, HistoryComponentAccessibilityArtifactError> {
    let packet: HistoryComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-local-history-write-scope-component-accessibility-fallback/support_export.json"
    )))
    .map_err(HistoryComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(HistoryComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in local-history / write-scope component
/// accessibility fallback export.
#[derive(Debug)]
pub enum HistoryComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<HistoryComponentAccessibilityViolation>),
}

impl fmt::Display for HistoryComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "local-history / write-scope accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "local-history / write-scope accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for HistoryComponentAccessibilityArtifactError {}

/// Validation failure for M05-898 local-history / write-scope component accessibility fallback
/// packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryComponentAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5HistoryClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    HistoryErased {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5LocalHistoryWriteScopeComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5HistoryClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5HistorySupportClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5HistoryConsumerSurface,
    },
    SummaryMismatch,
    RawHistoryMaterialInExport,
}

impl fmt::Display for HistoryComponentAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory history label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a restorable / reviewable history for a weakened one, or narrows spuriously"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::HistoryErased { id } => {
                write!(
                    f,
                    "row {id} does not preserve history integrity across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "support claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawHistoryMaterialInExport => {
                write!(f, "export contains raw history material")
            }
        }
    }
}

impl Error for HistoryComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "stale"
            | "metadata only"
            | "partial"
            | "manual"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in local-history / write-scope component accessibility
/// fallback packet. This is the one source of truth shared by the tests and the on-disk
/// support export so both stay byte-aligned.
pub fn seeded_m5_history_component_a11y_fallback_packet() -> HistoryComponentAccessibilityPacket {
    HistoryComponentAccessibilityPacket::new(HistoryComponentAccessibilityPacketInput {
        packet_id: "m5-local-history-write-scope-component-accessibility-fallback:stable:0001"
            .to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:local-history-write-scope-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5HistoryRequiredLabel> {
    M5HistoryRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> HistoryCopyExportParity {
    HistoryCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5HistoryClaimDimension,
    state: M5HistoryConditionState,
) -> HistoryClaimConditionEntry {
    HistoryClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5HistoryConsumerSurface]) -> Vec<M5HistoryConsumerSurface> {
    let mut out = vec![
        M5HistoryConsumerSurface::SupportExport,
        M5HistoryConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row
/// keeps full label and summary parity on the narrower surfaces; a narrowed row discloses the
/// reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: HistoryNarrowingDisclosureState,
) -> Vec<HistoryRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        HistoryRenderingNarrowingDisclosure {
            rendering_surface: M5HistoryRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        HistoryRenderingNarrowingDisclosure {
            rendering_surface: M5HistoryRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_restore".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<HistoryRenderingNarrowingDisclosure> {
    surface_disclosures(labels, HistoryNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<HistoryRenderingNarrowingDisclosure> {
    surface_disclosures(labels, HistoryNarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5HistoryRenderingSurface> {
    vec![
        M5HistoryRenderingSurface::DesktopFull,
        M5HistoryRenderingSurface::CliHeadless,
        M5HistoryRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<HistoryComponentAccessibilityRow> {
    vec![
        // Checkpoint-group card — the checkpoint has expired and is no longer restorable, so
        // the card auto-narrows to an unavailable checkpoint rather than presenting a
        // restorable one, while keeping its lineage and actor visible (yellow).
        HistoryComponentAccessibilityRow {
            record_kind: HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:checkpoint-group-card".to_owned(),
            component_family: M5LocalHistoryWriteScopeComponentFamily::CheckpointGroupCard,
            source_family_schema_ref: HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            history_context_ref: "history:checkpoint-group:0001".to_owned(),
            fallback_modalities: vec![
                M5HistoryFallbackModality::List,
                M5HistoryFallbackModality::Textual,
                M5HistoryFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            cli_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            export_summary: HistoryExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:checkpoint-group-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "checkpoint_lineage",
                "mutation_class",
                "actor",
                "timestamp",
            ]),
            full_support_claim: M5HistorySupportClaim::RestorableCheckpoint,
            claim_conditions: vec![condition(
                M5HistoryClaimDimension::CheckpointAvailability,
                M5HistoryConditionState::Unavailable,
            )],
            claim_narrow: Some(HistoryClaimAutoNarrow {
                narrowed_to: M5HistorySupportClaim::UnavailableCheckpoint,
                binding_dimension: M5HistoryClaimDimension::CheckpointAvailability,
                trigger: M5HistoryDowngradeTrigger::CheckpointLineageUnstated,
                narrowed_label:
                    "Checkpoint has expired past retention and can no longer be restored — shown unavailable with its lineage and actor still preserved"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_history_integrity: true,
            }),
            history_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "checkpoint_lineage",
                "mutation_class",
                "actor",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5HistoryConsumerSurface::CheckpointInspectorUi,
                M5HistoryConsumerSurface::RecoveryCenterUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §11.9 checkpoint-group / local history contract".to_owned(),
                HISTORY_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("checkpoint-group-card"),
        },
        // History-export manifest — the manifest carries full lineage and is a self-sufficient
        // reviewable evidence bundle (not itself a restore claim), reachable on every surface
        // (green).
        HistoryComponentAccessibilityRow {
            record_kind: HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:history-export-manifest".to_owned(),
            component_family: M5LocalHistoryWriteScopeComponentFamily::HistoryExportManifest,
            source_family_schema_ref: HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            history_context_ref: "history:export-manifest:0002".to_owned(),
            fallback_modalities: vec![
                M5HistoryFallbackModality::List,
                M5HistoryFallbackModality::Textual,
                M5HistoryFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            cli_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            export_summary: HistoryExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:history-export-manifest:a11y".to_owned(),
            copy_export: copy_export(&[
                "manifest_class",
                "export_redaction",
                "lineage_scope",
                "actor",
            ]),
            full_support_claim: M5HistorySupportClaim::ReviewableHistory,
            claim_conditions: vec![condition(
                M5HistoryClaimDimension::ManifestExportDisclosure,
                M5HistoryConditionState::Captured,
            )],
            claim_narrow: None,
            history_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "manifest_class",
                "export_redaction",
                "lineage_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5HistoryConsumerSurface::AiApplyReviewUi,
                M5HistoryConsumerSurface::CheckpointInspectorUi,
            ]),
            source_refs: vec![
                "UI/UX Spec Appendix CK history-export manifest grammar".to_owned(),
                HISTORY_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("history-export-manifest"),
        },
        // Local-history row — only metadata was captured (no full body), so the row
        // auto-narrows to a metadata-only history rather than presenting a full-fidelity
        // restorable snapshot (yellow).
        HistoryComponentAccessibilityRow {
            record_kind: HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:local-history-row".to_owned(),
            component_family: M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow,
            source_family_schema_ref: HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            history_context_ref: "history:local-history-row:0003".to_owned(),
            fallback_modalities: vec![
                M5HistoryFallbackModality::List,
                M5HistoryFallbackModality::Textual,
                M5HistoryFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            cli_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            export_summary: HistoryExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:local-history-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "snapshot_origin",
                "actor_class",
                "capture_fidelity",
                "timestamp",
            ]),
            full_support_claim: M5HistorySupportClaim::RestorableCheckpoint,
            claim_conditions: vec![condition(
                M5HistoryClaimDimension::CaptureFidelity,
                M5HistoryConditionState::MetadataOnly,
            )],
            claim_narrow: Some(HistoryClaimAutoNarrow {
                narrowed_to: M5HistorySupportClaim::MetadataOnlyHistory,
                binding_dimension: M5HistoryClaimDimension::CaptureFidelity,
                trigger: M5HistoryDowngradeTrigger::CaptureFidelityMasked,
                narrowed_label:
                    "Only metadata was captured for this entry — shown metadata-only with actor and timestamp preserved, not a full-body restorable snapshot"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_history_integrity: true,
            }),
            history_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "snapshot_origin",
                "actor_class",
                "capture_fidelity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5HistoryConsumerSurface::EditorTimelineUi,
                M5HistoryConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §11.9 local-history row capture fidelity".to_owned(),
                HISTORY_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("local-history-row"),
        },
        // Restore-preview card — the restore covers only a partial or manually chosen scope, so
        // the card auto-narrows to a narrowed restore rather than presenting a whole-snapshot
        // restore (yellow).
        HistoryComponentAccessibilityRow {
            record_kind: HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:restore-preview-card".to_owned(),
            component_family: M5LocalHistoryWriteScopeComponentFamily::RestorePreviewCard,
            source_family_schema_ref: HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            history_context_ref: "history:restore-preview-card:0004".to_owned(),
            fallback_modalities: vec![
                M5HistoryFallbackModality::List,
                M5HistoryFallbackModality::Textual,
                M5HistoryFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            cli_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            export_summary: HistoryExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:restore-preview-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "restore_granularity",
                "restore_drift",
                "file_identity",
                "actor",
            ]),
            full_support_claim: M5HistorySupportClaim::RestorableCheckpoint,
            claim_conditions: vec![condition(
                M5HistoryClaimDimension::RestoreGranularity,
                M5HistoryConditionState::NarrowedRestore,
            )],
            claim_narrow: Some(HistoryClaimAutoNarrow {
                narrowed_to: M5HistorySupportClaim::NarrowedRestore,
                binding_dimension: M5HistoryClaimDimension::RestoreGranularity,
                trigger: M5HistoryDowngradeTrigger::RestoreGranularityCollapsed,
                narrowed_label:
                    "Restore covers only a partial, manually chosen scope — shown narrowed to the selected files, not the whole snapshot"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_history_integrity: true,
            }),
            history_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "restore_granularity",
                "restore_drift",
                "file_identity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5HistoryConsumerSurface::RestoreReviewUi,
                M5HistoryConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UX Style Guide §31.24 restore-preview patterns".to_owned(),
                HISTORY_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("restore-preview-card"),
        },
        // Retention/export card — history export is redaction-limited to metadata only, so the
        // card auto-narrows to a metadata-only history rather than presenting a full evidence
        // export (yellow).
        HistoryComponentAccessibilityRow {
            record_kind: HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:retention-export-card".to_owned(),
            component_family: M5LocalHistoryWriteScopeComponentFamily::RetentionExportCard,
            source_family_schema_ref: HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            history_context_ref: "history:retention-export-card:0005".to_owned(),
            fallback_modalities: vec![
                M5HistoryFallbackModality::List,
                M5HistoryFallbackModality::Textual,
                M5HistoryFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            cli_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            export_summary: HistoryExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:retention-export-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "retention_posture",
                "export_redaction",
                "scope",
                "actor",
            ]),
            full_support_claim: M5HistorySupportClaim::RestorableCheckpoint,
            claim_conditions: vec![condition(
                M5HistoryClaimDimension::ExportDisclosure,
                M5HistoryConditionState::MetadataOnly,
            )],
            claim_narrow: Some(HistoryClaimAutoNarrow {
                narrowed_to: M5HistorySupportClaim::MetadataOnlyHistory,
                binding_dimension: M5HistoryClaimDimension::ExportDisclosure,
                trigger: M5HistoryDowngradeTrigger::RetentionOrRedactionUndisclosed,
                narrowed_label:
                    "History export is redaction-limited to metadata only — shown metadata-only with retention disclosed, never the full redacted bodies"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_history_integrity: true,
            }),
            history_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "retention_posture",
                "export_redaction",
                "scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5HistoryConsumerSurface::RecoveryCenterUi,
                M5HistoryConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UX Style Guide §18.10 local-history / retention patterns".to_owned(),
                HISTORY_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("retention-export-card"),
        },
        // Write-scope preview tree — hierarchy-heavy (nested workspace-root / file-node
        // hierarchy); the scope has drifted from the working tree, so the tree auto-narrows to
        // a stale-scope history and binds its tree to a flat list / textual path (yellow).
        HistoryComponentAccessibilityRow {
            record_kind: HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:write-scope-preview-tree".to_owned(),
            component_family: M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree,
            source_family_schema_ref: HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            history_context_ref: "history:write-scope-preview-tree:0006".to_owned(),
            fallback_modalities: vec![
                M5HistoryFallbackModality::Structured,
                M5HistoryFallbackModality::List,
                M5HistoryFallbackModality::Textual,
                M5HistoryFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: HistoryNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            export_summary: HistoryExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:write-scope-preview-tree:a11y".to_owned(),
            copy_export: copy_export(&[
                "write_scope_class",
                "managed_file_caveat",
                "file_count_bucket",
                "actor",
            ]),
            full_support_claim: M5HistorySupportClaim::RestorableCheckpoint,
            claim_conditions: vec![condition(
                M5HistoryClaimDimension::ScopeFreshness,
                M5HistoryConditionState::StaleScope,
            )],
            claim_narrow: Some(HistoryClaimAutoNarrow {
                narrowed_to: M5HistorySupportClaim::StaleScopeHistory,
                binding_dimension: M5HistoryClaimDimension::ScopeFreshness,
                trigger: M5HistoryDowngradeTrigger::WriteScopeUnderstated,
                narrowed_label:
                    "Write scope has drifted from the working tree — shown stale and held for re-resolution before any multi-file apply commits"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_history_integrity: true,
            }),
            history_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "write_scope_class",
                "managed_file_caveat",
                "file_count_bucket",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5HistoryConsumerSurface::RefactorPreviewUi,
                M5HistoryConsumerSurface::EditorTimelineUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.23 write-scope preview trees".to_owned(),
                HISTORY_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("write-scope-preview-tree"),
        },
        // Restore-granularity selector — the selector offers a fully selectable apply scope, so
        // it carries a fully restorable checkpoint and is reachable on every surface (green).
        HistoryComponentAccessibilityRow {
            record_kind: HISTORY_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:restore-granularity-selector".to_owned(),
            component_family: M5LocalHistoryWriteScopeComponentFamily::RestoreGranularitySelector,
            source_family_schema_ref: HISTORY_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            history_context_ref: "history:restore-granularity-selector:0007".to_owned(),
            fallback_modalities: vec![
                M5HistoryFallbackModality::List,
                M5HistoryFallbackModality::Textual,
                M5HistoryFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            cli_reach: HistoryNonVisualReachState::ReachableAndLabeled,
            export_summary: HistoryExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:restore-granularity-selector:a11y".to_owned(),
            copy_export: copy_export(&[
                "selection_mode",
                "apply_scope",
                "file_identity",
                "actor",
            ]),
            full_support_claim: M5HistorySupportClaim::RestorableCheckpoint,
            claim_conditions: vec![condition(
                M5HistoryClaimDimension::RestoreScopeSelection,
                M5HistoryConditionState::Captured,
            )],
            claim_narrow: None,
            history_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "selection_mode",
                "apply_scope",
                "file_identity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5HistoryConsumerSurface::RestoreReviewUi,
                M5HistoryConsumerSurface::RecoveryCenterUi,
            ]),
            source_refs: vec![
                "UX Style Guide §18.12 refactor-preview / restore-granularity patterns".to_owned(),
                HISTORY_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("restore-granularity-selector"),
        },
    ]
}
