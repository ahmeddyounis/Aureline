//! Open/save/reveal-in-system-shell path truth, boundary labels, and
//! checkpoint-aware overwrite review for system dialog and reveal flows.
//!
//! Aureline's local-first promise has to survive the moment a flow *leaves the
//! product for a native dialog or a system shell*: the OS open/save dialog, an
//! in-place save, a Save-As to a new target, a "Reveal in system shell", or an
//! "Open in default browser". Each of those moments can quietly erase the
//! canonical-path, read-only, generated, and checkpoint vocabulary the
//! in-product save and restore flows are careful about. This module makes every
//! such flow explicit and reviewable. Each flow is projected as one typed
//! [`SystemDialogFlow`] that:
//!
//! - preserves the **literal target** the user selected (as an export-safe
//!   captured ref, never a raw path or secret body) alongside the **canonical
//!   target** Aureline resolved it to, and classifies their relationship with a
//!   [`PathTruthClass`] so a canonicalized alias, a generated/remote/read-only
//!   artifact, or a missing canonical target can never masquerade as the plain
//!   literal file the user picked;
//! - labels the **boundary** the target sits behind with a [`BoundaryLabel`]
//!   (local, remote-adjacent, generated, or read-only) so a remote or generated
//!   artifact is never silently treated like an ordinary local file;
//! - declares the **overwrite posture** and **checkpoint availability** with the
//!   *same* overwrite-review and checkpoint vocabulary the in-product save and
//!   restore flows use, so a system save can never overwrite without the
//!   checkpoint-aware review an in-product save would require;
//! - keeps **"Reveal in system shell"** and **"Open in default browser"** as
//!   stable, explicit actions whose label and external side effect are disclosed
//!   rather than hidden; and
//! - integrates with the **filesystem-identity** and **save-coordination**
//!   objects so a wrong-target save, an alias-path confusion, or an unavailable
//!   checkpoint stays inspectable in diagnostics and support packets.
//!
//! The resulting [`OpenSaveRevealReport`] is the canonical truth object for the
//! system-dialog path-truth lane. It is consumed by the live save/reveal
//! affordances, the headless inspector
//! (`aureline_workspace_m5_open_save_reveal`, the only mint-from-truth path for
//! the JSON fixtures under `fixtures/platform/m5-open-save-reveal/`), the
//! support-export wrapper and per-incident case exports, the markdown artifact
//! under `artifacts/platform/m5-open-save-reveal.md`, and the companion doc
//! under `docs/m5/open-save-reveal-path-truth.md`.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every required flow kind is present — open, save, save-as,
//!    reveal-in-system-shell, and open-in-default-browser — and each flow
//!    carries a literal target, a canonical target, a boundary label, the
//!    filesystem-identity and save-coordination refs it reuses, an
//!    active-profile owner, a trust checkpoint, and the canonical in-product
//!    command. The literal/canonical relationship is always classified.
//! 2. A save or save-as flow whose overwrite posture is
//!    [`OverwritePosture::OverwriteWithCheckpoint`] MUST name an available,
//!    pinned checkpoint and share the overwrite-review ref; otherwise it is an
//!    [`OverwriteWithoutCheckpointReview`] blocker.
//! 3. A flow whose boundary is read-only MUST block the write
//!    ([`OverwritePosture::WriteBlockedReadOnly`]); a writing posture against a
//!    read-only boundary is a [`ReadOnlyWriteAttempt`] blocker. A generated
//!    artifact MUST be exported rather than saved in place; an in-place posture
//!    on a generated artifact is a [`GeneratedTreatedAsInPlaceSave`] blocker.
//! 4. A reveal-in-system-shell or open-in-default-browser flow MUST disclose its
//!    external side effect and a stable action label; a hidden one is a
//!    [`RevealSideEffectHidden`] blocker.
//! 5. A non-exact target MUST offer at least one recovery action, and each path
//!    condition stays a distinct failure: a missing canonical target is a
//!    [`WrongTargetSave`] blocker, a network-share alias is an
//!    [`AliasPathConfusion`] blocker, a generated output is a
//!    [`GeneratedOutputUnrecoverable`] blocker, and a read-only destination is a
//!    [`ReadOnlyDestinationUnrecoverable`] blocker — the four never collapse
//!    into a single finding.
//! 6. Stale evidence on a marketed flow is a blocker so release tooling can
//!    narrow the surface instead of shipping it as implicitly stable.
//!
//! All identifiers, refs, and label strings are deterministic so the checked-in
//! fixtures under `fixtures/platform/m5-open-save-reveal/` are bit-for-bit equal
//! to the seeded report returned by [`seeded_open_save_reveal_report`].
//!
//! [`OverwriteWithoutCheckpointReview`]: FlowFailureMode::OverwriteWithoutCheckpointReview
//! [`ReadOnlyWriteAttempt`]: FlowFailureMode::ReadOnlyWriteAttempt
//! [`GeneratedTreatedAsInPlaceSave`]: FlowFailureMode::GeneratedTreatedAsInPlaceSave
//! [`RevealSideEffectHidden`]: FlowFailureMode::RevealSideEffectHidden
//! [`WrongTargetSave`]: FlowFailureMode::WrongTargetSave
//! [`AliasPathConfusion`]: FlowFailureMode::AliasPathConfusion
//! [`GeneratedOutputUnrecoverable`]: FlowFailureMode::GeneratedOutputUnrecoverable
//! [`ReadOnlyDestinationUnrecoverable`]: FlowFailureMode::ReadOnlyDestinationUnrecoverable

use serde::{Deserialize, Serialize};

use crate::recent_work::TargetKind;

#[cfg(test)]
mod tests;

/// Schema version exported with every open/save/reveal record.
pub const OPEN_SAVE_REVEAL_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every open/save/reveal surface.
pub const OPEN_SAVE_REVEAL_SHARED_CONTRACT_REF: &str = "workspace:m5_open_save_reveal:v1";

/// Stable record kind for [`OpenSaveRevealReport`] payloads.
pub const OPEN_SAVE_REVEAL_REPORT_RECORD_KIND: &str = "workspace_m5_open_save_reveal_report_record";

/// Stable record kind for [`SystemDialogFlowRow`] payloads.
pub const OPEN_SAVE_REVEAL_ROW_RECORD_KIND: &str = "workspace_m5_open_save_reveal_flow_record";

/// Stable record kind for [`OpenSaveRevealSupportExport`] payloads.
pub const OPEN_SAVE_REVEAL_SUPPORT_EXPORT_RECORD_KIND: &str =
    "workspace_m5_open_save_reveal_support_export_record";

/// Stable record kind for [`OpenSaveRevealCaseExport`] payloads.
pub const OPEN_SAVE_REVEAL_CASE_EXPORT_RECORD_KIND: &str =
    "workspace_m5_open_save_reveal_case_export_record";

/// Stable report id quoted across surfaces.
pub const OPEN_SAVE_REVEAL_REPORT_ID: &str = "workspace:m5_open_save_reveal:report:v1";

/// Stable support-export id quoted in the published wrapper.
pub const OPEN_SAVE_REVEAL_SUPPORT_EXPORT_ID: &str = "support-export:m5-open-save-reveal:001";

/// Source schema ref for the canonical path-boundary contract.
pub const OPEN_SAVE_REVEAL_SOURCE_SCHEMA_REF: &str =
    "schemas/platform/m5-path-boundary.schema.json";

/// Path of the published markdown artifact.
pub const OPEN_SAVE_REVEAL_PUBLISHED_REPORT_REF: &str = "artifacts/platform/m5-open-save-reveal.md";

/// Path of the published companion doc.
pub const OPEN_SAVE_REVEAL_PUBLISHED_DOC_REF: &str = "docs/m5/open-save-reveal-path-truth.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-16T00:00:00Z";

/// One system-dialog or reveal flow kind the path-truth layer governs.
///
/// These are the five flows the spec requires to expose literal/canonical path
/// truth, boundary labels, and overwrite/checkpoint posture consistently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogFlowKind {
    /// A system open dialog selecting a target to read.
    Open,
    /// An in-place save of the current document to its canonical target.
    Save,
    /// A save-as dialog choosing a new target.
    SaveAs,
    /// A "Reveal in system shell" action that selects the target in the OS file
    /// manager.
    RevealInSystemShell,
    /// An "Open in default browser" action that hands a target to the default
    /// browser.
    OpenInDefaultBrowser,
}

impl DialogFlowKind {
    /// Returns the stable schema token for this flow kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Save => "save",
            Self::SaveAs => "save_as",
            Self::RevealInSystemShell => "reveal_in_system_shell",
            Self::OpenInDefaultBrowser => "open_in_default_browser",
        }
    }

    /// Returns the reviewer-facing label for this flow kind.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::Save => "Save",
            Self::SaveAs => "Save As",
            Self::RevealInSystemShell => "Reveal in system shell",
            Self::OpenInDefaultBrowser => "Open in default browser",
        }
    }

    /// Returns the five required flow kinds in canonical order.
    pub const fn required_kinds() -> [Self; 5] {
        [
            Self::Open,
            Self::Save,
            Self::SaveAs,
            Self::RevealInSystemShell,
            Self::OpenInDefaultBrowser,
        ]
    }

    /// `true` when the flow performs a write to the canonical target. Only
    /// writing flows are held to the overwrite/checkpoint and read-only/generated
    /// posture rules.
    pub const fn is_write_flow(self) -> bool {
        matches!(self, Self::Save | Self::SaveAs)
    }

    /// The external side effect the flow is required to disclose.
    pub const fn expected_side_effect(self) -> RevealSideEffect {
        match self {
            Self::RevealInSystemShell => RevealSideEffect::SelectsTargetInFileManager,
            Self::OpenInDefaultBrowser => RevealSideEffect::OpensDefaultBrowser,
            Self::Open | Self::Save | Self::SaveAs => RevealSideEffect::NoExternalSideEffect,
        }
    }

    /// `true` when the flow hands the target to a system shell or browser and so
    /// MUST disclose a stable action label.
    pub const fn requires_reveal_label(self) -> bool {
        matches!(self, Self::RevealInSystemShell | Self::OpenInDefaultBrowser)
    }
}

/// Shape hint for the literal target string the user selected.
///
/// The literal itself is captured as an opaque, export-safe ref; this class is
/// the only structural hint retained so support can reason about the target
/// without a raw path crossing the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathLiteralFormat {
    /// A Windows drive path (for example `C:\...`).
    WindowsDrivePath,
    /// A Windows UNC / network-share path (for example `\\server\share\...`).
    WindowsUncPath,
    /// A POSIX path.
    PosixPath,
    /// A `file://` URI.
    FileUri,
    /// An `http(s)://` URL handed to the browser.
    Url,
    /// The literal shape could not be classified.
    Unknown,
}

impl PathLiteralFormat {
    /// Returns the stable schema token for this literal format.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowsDrivePath => "windows_drive_path",
            Self::WindowsUncPath => "windows_unc_path",
            Self::PosixPath => "posix_path",
            Self::FileUri => "file_uri",
            Self::Url => "url",
            Self::Unknown => "unknown",
        }
    }
}

/// How the literal target the user selected relates to the canonical target
/// Aureline resolved.
///
/// This is the core path-truth disclosure: a user can tell whether a dialog is
/// targeting the literal file they picked, a canonicalized alias, a
/// boundary-labeled generated/remote/read-only artifact, or a target whose
/// canonical identity could not be resolved at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathTruthClass {
    /// The literal target the user selected is itself the canonical target.
    LiteralIsCanonical,
    /// The literal target is an alias (symlink, case variant, or network
    /// mapping) that resolves to a different canonical target.
    CanonicalAliasResolved,
    /// The target is a boundary-labeled generated, remote, or read-only artifact
    /// rather than a plain local file.
    BoundaryLabeledArtifact,
    /// The canonical target could not be resolved from the literal.
    CanonicalTargetMissing,
}

impl PathTruthClass {
    /// Returns the stable schema token for this path-truth class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiteralIsCanonical => "literal_is_canonical",
            Self::CanonicalAliasResolved => "canonical_alias_resolved",
            Self::BoundaryLabeledArtifact => "boundary_labeled_artifact",
            Self::CanonicalTargetMissing => "canonical_target_missing",
        }
    }

    /// Returns the reviewer-facing label for this path-truth class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::LiteralIsCanonical => "Literal target is canonical",
            Self::CanonicalAliasResolved => "Canonicalized alias",
            Self::BoundaryLabeledArtifact => "Boundary-labeled artifact",
            Self::CanonicalTargetMissing => "Canonical target missing",
        }
    }
}

/// The boundary a target sits behind, labeled so platform-native dialog
/// convenience never erases the remote, generated, or read-only distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryLabel {
    /// A local, writable target.
    LocalWritable,
    /// A target reached across a network share or remote-adjacent mount.
    RemoteAdjacent,
    /// A generated artifact whose canonical source is elsewhere.
    Generated,
    /// A read-only destination that cannot be written in place.
    ReadOnly,
}

impl BoundaryLabel {
    /// Returns the stable schema token for this boundary label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWritable => "local_writable",
            Self::RemoteAdjacent => "remote_adjacent",
            Self::Generated => "generated",
            Self::ReadOnly => "read_only",
        }
    }

    /// Returns the reviewer-facing label for this boundary.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::LocalWritable => "Local (writable)",
            Self::RemoteAdjacent => "Remote / network share",
            Self::Generated => "Generated artifact",
            Self::ReadOnly => "Read-only destination",
        }
    }

    /// Returns the four boundary labels in canonical order.
    pub const fn all() -> [Self; 4] {
        [
            Self::LocalWritable,
            Self::RemoteAdjacent,
            Self::Generated,
            Self::ReadOnly,
        ]
    }
}

/// The overwrite posture a flow takes, in the shared in-product save/restore
/// vocabulary so a system dialog is never more permissive than an in-product
/// save.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverwritePosture {
    /// The flow does not write (open, reveal, browser).
    NoWriteAction,
    /// The flow writes to a new target that does not already exist.
    CreateNewFile,
    /// The flow overwrites an existing target after a checkpoint is captured.
    OverwriteWithCheckpoint,
    /// The flow needs an explicit overwrite review before any write commits.
    OverwriteReviewRequired,
    /// The destination is read-only and the write is blocked.
    WriteBlockedReadOnly,
    /// The artifact is generated and is exported rather than saved in place.
    ExportNotInPlaceSave,
}

impl OverwritePosture {
    /// Returns the stable schema token for this overwrite posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoWriteAction => "no_write_action",
            Self::CreateNewFile => "create_new_file",
            Self::OverwriteWithCheckpoint => "overwrite_with_checkpoint",
            Self::OverwriteReviewRequired => "overwrite_review_required",
            Self::WriteBlockedReadOnly => "write_blocked_read_only",
            Self::ExportNotInPlaceSave => "export_not_in_place_save",
        }
    }

    /// Returns the reviewer-facing label for this overwrite posture.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::NoWriteAction => "No write",
            Self::CreateNewFile => "Create new file",
            Self::OverwriteWithCheckpoint => "Overwrite with checkpoint",
            Self::OverwriteReviewRequired => "Overwrite review required",
            Self::WriteBlockedReadOnly => "Write blocked (read-only)",
            Self::ExportNotInPlaceSave => "Export (not in-place save)",
        }
    }

    /// `true` when the posture writes in place to an existing target.
    pub const fn is_in_place_write(self) -> bool {
        matches!(self, Self::OverwriteWithCheckpoint)
    }
}

/// Availability of the checkpoint a flow's overwrite review depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointAvailability {
    /// A checkpoint is pinned and available for the overwrite review.
    Pinned,
    /// A checkpoint would be required but is unavailable.
    Unavailable,
    /// The flow does not depend on a checkpoint.
    NotApplicable,
}

impl CheckpointAvailability {
    /// Returns the stable schema token for this checkpoint availability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pinned => "pinned",
            Self::Unavailable => "unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The external side effect a reveal or browser flow performs, disclosed so a
/// reveal action never hides platform-specific behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevealSideEffect {
    /// No external side effect; the flow stays inside the product.
    NoExternalSideEffect,
    /// The flow opens the OS file manager and selects the target.
    SelectsTargetInFileManager,
    /// The flow hands the target to the default browser.
    OpensDefaultBrowser,
}

impl RevealSideEffect {
    /// Returns the stable schema token for this side effect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoExternalSideEffect => "no_external_side_effect",
            Self::SelectsTargetInFileManager => "selects_target_in_file_manager",
            Self::OpensDefaultBrowser => "opens_default_browser",
        }
    }
}

/// The condition of the path/destination at flow time.
///
/// [`ExactAvailable`](Self::ExactAvailable) is the clean state; the other four
/// are the required incident classes the failure-path fixtures cover, and each
/// stays a distinct failure when no recovery is offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathConditionClass {
    /// The target is exactly available.
    ExactAvailable,
    /// The canonical target could not be resolved from the literal.
    MissingCanonicalTarget,
    /// The literal resolves through a network-share alias.
    NetworkShareAlias,
    /// The target is a generated output rather than an editable source.
    GeneratedOutput,
    /// The destination is read-only.
    ReadOnlyDestination,
}

impl PathConditionClass {
    /// Returns the stable schema token for this path condition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAvailable => "exact_available",
            Self::MissingCanonicalTarget => "missing_canonical_target",
            Self::NetworkShareAlias => "network_share_alias",
            Self::GeneratedOutput => "generated_output",
            Self::ReadOnlyDestination => "read_only_destination",
        }
    }

    /// Returns the reviewer-facing label for this path condition.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ExactAvailable => "Exact / available",
            Self::MissingCanonicalTarget => "Missing canonical target",
            Self::NetworkShareAlias => "Network-share alias",
            Self::GeneratedOutput => "Generated output",
            Self::ReadOnlyDestination => "Read-only destination",
        }
    }

    /// Returns the five path conditions in canonical order.
    pub const fn all() -> [Self; 5] {
        [
            Self::ExactAvailable,
            Self::MissingCanonicalTarget,
            Self::NetworkShareAlias,
            Self::GeneratedOutput,
            Self::ReadOnlyDestination,
        ]
    }

    /// `true` when the condition is not exactly available and therefore requires
    /// at least one recovery action.
    pub const fn requires_recovery(self) -> bool {
        !matches!(self, Self::ExactAvailable)
    }

    /// The distinct failure mode a missing recovery action raises for this
    /// condition. The four recovery failure classes are never collapsed.
    pub const fn missing_recovery_failure_mode(self) -> Option<FlowFailureMode> {
        match self {
            Self::ExactAvailable => None,
            Self::MissingCanonicalTarget => Some(FlowFailureMode::WrongTargetSave),
            Self::NetworkShareAlias => Some(FlowFailureMode::AliasPathConfusion),
            Self::GeneratedOutput => Some(FlowFailureMode::GeneratedOutputUnrecoverable),
            Self::ReadOnlyDestination => Some(FlowFailureMode::ReadOnlyDestinationUnrecoverable),
        }
    }
}

/// A bounded recovery action a degraded flow offers instead of dead-ending.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAction {
    /// Let the user choose a different target.
    ChooseDifferentTarget,
    /// Show the canonical path the literal resolves to.
    ShowCanonicalPath,
    /// Resolve the network-share alias to its canonical target.
    ResolveShareAlias,
    /// Reconnect the network share or removable volume.
    ReconnectShare,
    /// Export the generated artifact instead of saving in place.
    ExportInsteadOfSave,
    /// Regenerate the artifact from its canonical source.
    RegenerateFromSource,
    /// Save a writable copy to a different destination.
    SaveWritableCopyElsewhere,
    /// Open the target read-only.
    OpenReadOnly,
    /// Reveal the parent folder instead of the missing target.
    RevealParentFolder,
}

impl RecoveryAction {
    /// Returns the stable schema token for this recovery action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChooseDifferentTarget => "choose_different_target",
            Self::ShowCanonicalPath => "show_canonical_path",
            Self::ResolveShareAlias => "resolve_share_alias",
            Self::ReconnectShare => "reconnect_share",
            Self::ExportInsteadOfSave => "export_instead_of_save",
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::SaveWritableCopyElsewhere => "save_writable_copy_elsewhere",
            Self::OpenReadOnly => "open_read_only",
            Self::RevealParentFolder => "reveal_parent_folder",
        }
    }
}

/// A desktop platform the flow is claimed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    /// macOS desktop platform.
    Macos,
    /// Windows desktop platform.
    Windows,
    /// Linux desktop platform.
    Linux,
}

impl Platform {
    /// Returns the stable schema token for this platform.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Returns the three platforms in canonical order.
    pub const fn all() -> [Self; 3] {
        [Self::Macos, Self::Windows, Self::Linux]
    }
}

/// Freshness of the captured flow evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed flow.
    Stale,
}

impl EvidenceFreshness {
    /// Returns the stable schema token for this freshness.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// A distinct open/save/reveal failure class.
///
/// Each class names a materially different way a system dialog or reveal flow
/// can erase Aureline's canonical-path, read-only, generated, or checkpoint
/// vocabulary. They are never collapsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowFailureMode {
    /// A save targeting a missing canonical target offered no recovery.
    WrongTargetSave,
    /// A network-share alias was not resolved or recovered.
    AliasPathConfusion,
    /// A generated output offered no export or regenerate recovery.
    GeneratedOutputUnrecoverable,
    /// A read-only destination offered no writable-copy recovery.
    ReadOnlyDestinationUnrecoverable,
    /// An overwrite committed without a pinned checkpoint and shared review.
    OverwriteWithoutCheckpointReview,
    /// A writing posture targeted a read-only boundary.
    ReadOnlyWriteAttempt,
    /// A generated artifact was saved in place instead of exported.
    GeneratedTreatedAsInPlaceSave,
    /// A reveal or browser action hid its external side effect or label.
    RevealSideEffectHidden,
    /// The flow recorded no canonical target identity.
    CanonicalPathHidden,
    /// The overwrite/checkpoint review vocabulary was not shared with the
    /// in-product save/restore flows.
    CheckpointVocabularyDivergence,
    /// The flow bypassed trust / profile / policy evaluation.
    TrustEvaluationBypassed,
}

impl FlowFailureMode {
    /// Returns the stable schema token for this failure mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongTargetSave => "wrong_target_save",
            Self::AliasPathConfusion => "alias_path_confusion",
            Self::GeneratedOutputUnrecoverable => "generated_output_unrecoverable",
            Self::ReadOnlyDestinationUnrecoverable => "read_only_destination_unrecoverable",
            Self::OverwriteWithoutCheckpointReview => "overwrite_without_checkpoint_review",
            Self::ReadOnlyWriteAttempt => "read_only_write_attempt",
            Self::GeneratedTreatedAsInPlaceSave => "generated_treated_as_in_place_save",
            Self::RevealSideEffectHidden => "reveal_side_effect_hidden",
            Self::CanonicalPathHidden => "canonical_path_hidden",
            Self::CheckpointVocabularyDivergence => "checkpoint_vocabulary_divergence",
            Self::TrustEvaluationBypassed => "trust_evaluation_bypassed",
        }
    }
}

/// Cross-links to the canonical upstream packets the path-truth layer reuses so
/// path, checkpoint, and boundary vocabulary cannot drift independently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenSaveRevealCrossLinks {
    /// Filesystem-identity / canonical-path lineage packet.
    pub filesystem_identity_ref: String,
    /// Save-coordination / artifact-save-truth packet.
    pub save_coordination_ref: String,
    /// Restore-continuity / recovery vocabulary packet.
    pub restore_continuity_ref: String,
    /// Native-desktop handler-ownership and reopen matrix.
    pub native_desktop_matrix_ref: String,
    /// System-open and file-association intake report.
    pub system_entry_intake_ref: String,
    /// Help/About and docs surface the report is ingested by.
    pub help_about_ref: String,
}

impl OpenSaveRevealCrossLinks {
    /// Returns the cross-link fields as `(label, ref)` pairs in canonical order.
    pub fn as_pairs(&self) -> [(&'static str, &str); 6] {
        [
            ("filesystem_identity_ref", &self.filesystem_identity_ref),
            ("save_coordination_ref", &self.save_coordination_ref),
            ("restore_continuity_ref", &self.restore_continuity_ref),
            ("native_desktop_matrix_ref", &self.native_desktop_matrix_ref),
            ("system_entry_intake_ref", &self.system_entry_intake_ref),
            ("help_about_ref", &self.help_about_ref),
        ]
    }

    /// The canonical cross-link set every report carries.
    pub fn canonical() -> Self {
        Self {
            filesystem_identity_ref: "schemas/workspace/canonical_identity_lineage.schema.json"
                .to_owned(),
            save_coordination_ref: "schemas/state/artifact_save_truth.schema.json".to_owned(),
            restore_continuity_ref: "docs/workspace/entry_restore_object_model.md".to_owned(),
            native_desktop_matrix_ref: "artifacts/platform/m5-native-desktop-matrix.md".to_owned(),
            system_entry_intake_ref: "artifacts/platform/m5-system-open-and-file-association.md"
                .to_owned(),
            help_about_ref: "docs/help/open_save_reveal_path_truth.md".to_owned(),
        }
    }
}

/// Canonical descriptor for one open/save/reveal flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemDialogFlow {
    /// Stable flow id (e.g. `flow:save.local_file`).
    pub flow_id: String,
    /// The system-dialog or reveal flow kind.
    pub flow_kind: DialogFlowKind,
    /// Descriptor revision the report was produced against. MUST be non-empty.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Export-safe captured ref for the literal target the user selected. MUST
    /// be non-empty. Never a raw path or secret body.
    pub literal_target_ref: String,
    /// Shape hint for the literal target.
    pub literal_format: PathLiteralFormat,
    /// Canonical target identity Aureline resolved the literal to. MUST be
    /// non-empty.
    pub canonical_target_ref: String,
    /// How the literal relates to the canonical target.
    pub path_truth_class: PathTruthClass,
    /// Canonical target kind, in the shared workspace vocabulary.
    pub detected_target_kind: TargetKind,
    /// Boundary the target sits behind.
    pub boundary_label: BoundaryLabel,
    /// Visible boundary-label ref user surfaces render. MUST be non-empty.
    pub boundary_label_ref: String,
    /// Overwrite posture the flow takes, in the shared save vocabulary.
    pub write_posture: OverwritePosture,
    /// Availability of the checkpoint the overwrite review depends on.
    pub checkpoint_availability: CheckpointAvailability,
    /// Checkpoint ref (required when [`Self::checkpoint_availability`] is
    /// [`CheckpointAvailability::Pinned`]).
    pub checkpoint_ref: Option<String>,
    /// Shared overwrite-review ref the in-product save/restore flows use. MUST
    /// be non-empty.
    pub overwrite_review_ref: String,
    /// External side effect the flow performs.
    pub reveal_side_effect: RevealSideEffect,
    /// Stable action label for a reveal/browser flow (required for those kinds).
    pub reveal_action_label_ref: Option<String>,
    /// Filesystem-identity object the flow reuses. MUST be non-empty.
    pub filesystem_identity_ref: String,
    /// Save-coordination object the flow reuses. MUST be non-empty.
    pub save_coordination_ref: String,
    /// Active profile owner the flow routes through. MUST be non-empty.
    pub active_profile_owner_ref: String,
    /// Trust / profile / policy checkpoint the flow routes through. MUST be
    /// non-empty.
    pub trust_checkpoint_ref: String,
    /// Canonical in-product command the dialog reuses. MUST be non-empty.
    pub canonical_command_ref: String,
    /// Condition of the path/destination at flow time.
    pub path_condition: PathConditionClass,
    /// Recovery actions offered when the path condition is not exactly
    /// available.
    pub recovery_actions: Vec<RecoveryAction>,
    /// Continuity note retained on the descriptor. MUST be non-empty.
    pub continuity_note: String,
    /// Exact degraded-state vocabulary user-visible surfaces MUST use. MUST be
    /// non-empty.
    pub degraded_state_vocabulary: Vec<String>,
    /// Claimed platforms. MUST be non-empty.
    pub claimed_platforms: Vec<Platform>,
    /// Freshness of the captured evidence.
    pub evidence_freshness: EvidenceFreshness,
    /// Timestamp the evidence was captured.
    pub evidence_captured_at: String,
    /// Rule user-visible surfaces follow when evidence goes stale. MUST be
    /// non-empty.
    pub downgrade_rule_ref: String,
    /// `true` when the flow is marketed and must pass the report or narrow.
    pub marketed: bool,
    /// `true` once the flow rides the governed path-truth harness. MUST be
    /// `true`.
    pub registered_on_path_truth_harness: bool,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum FlowBlockingFinding {
    /// A save targeting a missing canonical target offered no recovery.
    WrongTargetSave {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// A network-share alias was not resolved or recovered.
    AliasPathConfusion {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// A generated output offered no export or regenerate recovery.
    GeneratedOutputUnrecoverable {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// A read-only destination offered no writable-copy recovery.
    ReadOnlyDestinationUnrecoverable {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// An overwrite committed without a pinned checkpoint and shared review.
    OverwriteWithoutCheckpointReview {
        /// Flow that exposes the gap.
        flow_id: String,
        /// Checkpoint availability observed.
        checkpoint_availability: CheckpointAvailability,
    },
    /// A writing posture targeted a read-only boundary.
    ReadOnlyWriteAttempt {
        /// Flow that exposes the gap.
        flow_id: String,
        /// Overwrite posture observed.
        write_posture: OverwritePosture,
    },
    /// A generated artifact was saved in place instead of exported.
    GeneratedTreatedAsInPlaceSave {
        /// Flow that exposes the gap.
        flow_id: String,
        /// Overwrite posture observed.
        write_posture: OverwritePosture,
    },
    /// A reveal or browser action hid its external side effect or label.
    RevealSideEffectHidden {
        /// Flow that exposes the gap.
        flow_id: String,
        /// Flow kind whose side effect was hidden.
        flow_kind: DialogFlowKind,
    },
    /// The flow recorded no canonical target identity.
    CanonicalPathHidden {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The overwrite/checkpoint review vocabulary was not shared.
    CheckpointVocabularyDivergence {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow bypassed trust / policy evaluation (no trust checkpoint).
    TrustEvaluationBypassed {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow recorded no literal target.
    MissingLiteralTarget {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow recorded no boundary label ref.
    MissingBoundaryLabel {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow reused no filesystem-identity object.
    MissingFilesystemIdentityRef {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow reused no save-coordination object.
    MissingSaveCoordinationRef {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow recorded no active-profile owner.
    MissingActiveProfileOwner {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow reused no canonical in-product command.
    MissingCanonicalCommand {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow recorded no continuity note.
    MissingContinuityNote {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow recorded no degraded-state vocabulary.
    MissingDegradedStateVocabulary {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow claimed no platform.
    MissingClaimedPlatforms {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow recorded no downgrade rule.
    MissingDowngradeRule {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// A marketed flow carries stale evidence.
    StaleEvidenceOnMarketedFlow {
        /// Flow that exposes the gap.
        flow_id: String,
    },
    /// The flow drives its own path off the governed harness.
    FlowNotOnHarness {
        /// Flow that exposes the gap.
        flow_id: String,
    },
}

impl FlowBlockingFinding {
    /// Returns the stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::WrongTargetSave { .. } => "wrong_target_save",
            Self::AliasPathConfusion { .. } => "alias_path_confusion",
            Self::GeneratedOutputUnrecoverable { .. } => "generated_output_unrecoverable",
            Self::ReadOnlyDestinationUnrecoverable { .. } => "read_only_destination_unrecoverable",
            Self::OverwriteWithoutCheckpointReview { .. } => "overwrite_without_checkpoint_review",
            Self::ReadOnlyWriteAttempt { .. } => "read_only_write_attempt",
            Self::GeneratedTreatedAsInPlaceSave { .. } => "generated_treated_as_in_place_save",
            Self::RevealSideEffectHidden { .. } => "reveal_side_effect_hidden",
            Self::CanonicalPathHidden { .. } => "canonical_path_hidden",
            Self::CheckpointVocabularyDivergence { .. } => "checkpoint_vocabulary_divergence",
            Self::TrustEvaluationBypassed { .. } => "trust_evaluation_bypassed",
            Self::MissingLiteralTarget { .. } => "missing_literal_target",
            Self::MissingBoundaryLabel { .. } => "missing_boundary_label",
            Self::MissingFilesystemIdentityRef { .. } => "missing_filesystem_identity_ref",
            Self::MissingSaveCoordinationRef { .. } => "missing_save_coordination_ref",
            Self::MissingActiveProfileOwner { .. } => "missing_active_profile_owner",
            Self::MissingCanonicalCommand { .. } => "missing_canonical_command",
            Self::MissingContinuityNote { .. } => "missing_continuity_note",
            Self::MissingDegradedStateVocabulary { .. } => "missing_degraded_state_vocabulary",
            Self::MissingClaimedPlatforms { .. } => "missing_claimed_platforms",
            Self::MissingDowngradeRule { .. } => "missing_downgrade_rule",
            Self::StaleEvidenceOnMarketedFlow { .. } => "stale_evidence_on_marketed_flow",
            Self::FlowNotOnHarness { .. } => "flow_not_on_harness",
        }
    }

    /// Returns the flow id this finding is attached to.
    pub fn flow_id(&self) -> &str {
        match self {
            Self::WrongTargetSave { flow_id }
            | Self::AliasPathConfusion { flow_id }
            | Self::GeneratedOutputUnrecoverable { flow_id }
            | Self::ReadOnlyDestinationUnrecoverable { flow_id }
            | Self::OverwriteWithoutCheckpointReview { flow_id, .. }
            | Self::ReadOnlyWriteAttempt { flow_id, .. }
            | Self::GeneratedTreatedAsInPlaceSave { flow_id, .. }
            | Self::RevealSideEffectHidden { flow_id, .. }
            | Self::CanonicalPathHidden { flow_id }
            | Self::CheckpointVocabularyDivergence { flow_id }
            | Self::TrustEvaluationBypassed { flow_id }
            | Self::MissingLiteralTarget { flow_id }
            | Self::MissingBoundaryLabel { flow_id }
            | Self::MissingFilesystemIdentityRef { flow_id }
            | Self::MissingSaveCoordinationRef { flow_id }
            | Self::MissingActiveProfileOwner { flow_id }
            | Self::MissingCanonicalCommand { flow_id }
            | Self::MissingContinuityNote { flow_id }
            | Self::MissingDegradedStateVocabulary { flow_id }
            | Self::MissingClaimedPlatforms { flow_id }
            | Self::MissingDowngradeRule { flow_id }
            | Self::StaleEvidenceOnMarketedFlow { flow_id }
            | Self::FlowNotOnHarness { flow_id } => flow_id,
        }
    }

    /// Returns the distinct failure mode this finding maps to, when it maps to a
    /// contract-honesty failure class (rather than a missing-field gap).
    pub fn failure_mode(&self) -> Option<FlowFailureMode> {
        match self {
            Self::WrongTargetSave { .. } => Some(FlowFailureMode::WrongTargetSave),
            Self::AliasPathConfusion { .. } => Some(FlowFailureMode::AliasPathConfusion),
            Self::GeneratedOutputUnrecoverable { .. } => {
                Some(FlowFailureMode::GeneratedOutputUnrecoverable)
            }
            Self::ReadOnlyDestinationUnrecoverable { .. } => {
                Some(FlowFailureMode::ReadOnlyDestinationUnrecoverable)
            }
            Self::OverwriteWithoutCheckpointReview { .. } => {
                Some(FlowFailureMode::OverwriteWithoutCheckpointReview)
            }
            Self::ReadOnlyWriteAttempt { .. } => Some(FlowFailureMode::ReadOnlyWriteAttempt),
            Self::GeneratedTreatedAsInPlaceSave { .. } => {
                Some(FlowFailureMode::GeneratedTreatedAsInPlaceSave)
            }
            Self::RevealSideEffectHidden { .. } => Some(FlowFailureMode::RevealSideEffectHidden),
            Self::CanonicalPathHidden { .. } => Some(FlowFailureMode::CanonicalPathHidden),
            Self::CheckpointVocabularyDivergence { .. } => {
                Some(FlowFailureMode::CheckpointVocabularyDivergence)
            }
            Self::TrustEvaluationBypassed { .. } => Some(FlowFailureMode::TrustEvaluationBypassed),
            _ => None,
        }
    }
}

/// One per-flow open/save/reveal row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemDialogFlowRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the flow.
    pub descriptor: SystemDialogFlow,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<FlowBlockingFinding>,
    /// `true` when the flow is marketed.
    pub marketed: bool,
}

/// One `(class, count)` blocking-finding tally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowFindingCount {
    /// Finding class token.
    pub class: String,
    /// Number of findings in this class.
    pub count: usize,
}

/// Per-class blocking-finding summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowFindingSummary {
    /// Total blocking findings across the report.
    pub total_blocking_findings: usize,
    /// Per-class tallies, sorted by class token.
    pub by_class: Vec<FlowFindingCount>,
}

/// Per-flow-kind presence summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowKindCoverage {
    /// Flow kind this summary covers.
    pub flow_kind: DialogFlowKind,
    /// Number of registered flows of this kind.
    pub flow_count: usize,
}

/// Per-boundary-label coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryLabelCoverage {
    /// Boundary label this summary covers.
    pub boundary_label: BoundaryLabel,
    /// Number of flows that sit behind this boundary.
    pub flow_count: usize,
    /// Number of those flows whose write is blocked or exported (not in-place).
    pub write_protected_count: usize,
}

/// Per-path-condition coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathConditionCoverage {
    /// Path condition this summary covers.
    pub path_condition: PathConditionClass,
    /// Number of flows in this condition.
    pub flow_count: usize,
    /// Number of those flows that offer at least one recovery action.
    pub recovered_count: usize,
}

/// A single path-truth index entry so platform QA, docs, and support surfaces
/// can quote what each flow targets and how it writes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathTruthIndexEntry {
    /// Flow id the entry covers.
    pub flow_id: String,
    /// Flow kind the entry covers.
    pub flow_kind: DialogFlowKind,
    /// Canonical target identity the literal resolved to.
    pub canonical_target_ref: String,
    /// How the literal relates to the canonical target.
    pub path_truth_class: PathTruthClass,
    /// Boundary the target sits behind.
    pub boundary_label: BoundaryLabel,
    /// Overwrite posture the flow takes.
    pub write_posture: OverwritePosture,
    /// Path condition the flow is in.
    pub path_condition: PathConditionClass,
}

/// One marketed flow release tooling should narrow because a control failed or
/// its evidence is stale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowNarrowableEntry {
    /// Flow id that must narrow.
    pub flow_id: String,
    /// Failure mode that drives the narrowing, when control-scoped.
    pub failure_mode: Option<FlowFailureMode>,
    /// Stable reason the flow is narrowable.
    pub reason: String,
}

/// Open/save/reveal path-truth report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenSaveRevealReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required flow kinds, in canonical order.
    pub required_flow_kinds: Vec<DialogFlowKind>,
    /// Union of claimed platforms across all flows, sorted.
    pub claimed_platforms: Vec<Platform>,
    /// Cross-links to upstream packets.
    pub cross_links: OpenSaveRevealCrossLinks,
    /// Per-flow rows, sorted by `descriptor.flow_id`.
    pub entries: Vec<SystemDialogFlowRow>,
    /// Per-flow-kind presence summary, in canonical kind order.
    pub flow_kind_coverage: Vec<FlowKindCoverage>,
    /// Per-boundary-label coverage summary, in canonical boundary order.
    pub boundary_label_coverage: Vec<BoundaryLabelCoverage>,
    /// Per-path-condition coverage summary, in canonical condition order.
    pub path_condition_coverage: Vec<PathConditionCoverage>,
    /// Per-class blocking-finding summary.
    pub findings_summary: FlowFindingSummary,
    /// Canonical path-truth index, sorted by flow id.
    pub path_truth_index: Vec<PathTruthIndexEntry>,
    /// Number of registered flows present.
    pub registered_flow_count: usize,
    /// Number of flows marketed.
    pub marketed_flow_count: usize,
    /// Number of write flows (save / save-as).
    pub write_flow_count: usize,
    /// Marketed flows release tooling should narrow.
    pub narrowable_marketed_entries: Vec<FlowNarrowableEntry>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this report is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the report can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the report can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the report was generated.
    pub generated_at: String,
}

impl OpenSaveRevealReport {
    /// Returns `true` when every required flow kind has at least one registered
    /// flow.
    pub fn every_kind_present(&self) -> bool {
        DialogFlowKind::required_kinds().into_iter().all(|kind| {
            self.entries
                .iter()
                .any(|entry| entry.descriptor.flow_kind == kind)
        })
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: flows={}, marketed={}, write_flows={}, blocking={}, clean={}",
            self.registered_flow_count,
            self.marketed_flow_count,
            self.write_flow_count,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for entry in &self.entries {
            lines.push(format!(
                "{}: kind={}, truth={}, boundary={}, posture={}, checkpoint={}, condition={}",
                entry.descriptor.flow_id,
                entry.descriptor.flow_kind.as_str(),
                entry.descriptor.path_truth_class.as_str(),
                entry.descriptor.boundary_label.as_str(),
                entry.descriptor.write_posture.as_str(),
                entry.descriptor.checkpoint_availability.as_str(),
                entry.descriptor.path_condition.as_str(),
            ));
        }
        for entry in &self.entries {
            for finding in &entry.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {}",
                    finding.class_token(),
                    finding.flow_id(),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_entries {
            lines.push(format!(
                "narrowable: {} -- {}",
                narrowable.flow_id, narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 open/save/reveal path truth\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::m5_open_save_reveal`](../../crates/aureline-workspace/src/m5_open_save_reveal/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- report-md > \\\n  artifacts/platform/m5-open-save-reveal.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Claimed platforms: {}\n",
            self.claimed_platforms
                .iter()
                .map(|platform| format!("`{}`", platform.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Registered flows: `{}`\n",
            self.registered_flow_count
        ));
        out.push_str(&format!(
            "- Marketed flows: `{}`\n",
            self.marketed_flow_count
        ));
        out.push_str(&format!("- Write flows: `{}`\n", self.write_flow_count));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed flows: `{}`\n",
            self.narrowable_marketed_entries.len()
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

        out.push_str("## Cross-links\n\n");
        out.push_str("| Upstream packet | Ref |\n| --------------- | --- |\n");
        for (label, value) in self.cross_links.as_pairs() {
            out.push_str(&format!("| `{label}` | `{value}` |\n"));
        }
        out.push('\n');

        out.push_str("## Per-flow-kind coverage\n\n");
        out.push_str("| Flow kind | Registered flows |\n| --------- | ---------------: |\n");
        for coverage in &self.flow_kind_coverage {
            out.push_str(&format!(
                "| {} | {} |\n",
                coverage.flow_kind.display_label(),
                coverage.flow_count,
            ));
        }
        out.push('\n');

        out.push_str("## Per-boundary coverage\n\n");
        out.push_str(
            "| Boundary | Flows | Write-protected |\n\
             | -------- | ----: | --------------: |\n",
        );
        for coverage in &self.boundary_label_coverage {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                coverage.boundary_label.display_label(),
                coverage.flow_count,
                coverage.write_protected_count,
            ));
        }
        out.push('\n');

        out.push_str("## Per-path-condition coverage\n\n");
        out.push_str(
            "| Path condition | Flows | With recovery |\n\
             | -------------- | ----: | ------------: |\n",
        );
        for coverage in &self.path_condition_coverage {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                coverage.path_condition.display_label(),
                coverage.flow_count,
                coverage.recovered_count,
            ));
        }
        out.push('\n');

        out.push_str("## Path-truth index\n\n");
        out.push_str(
            "| Flow | Kind | Path truth | Boundary | Overwrite posture | Condition |\n\
             | ---- | ---- | ---------- | -------- | ----------------- | --------- |\n",
        );
        for entry in &self.path_truth_index {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | `{}` |\n",
                entry.flow_id,
                entry.flow_kind.display_label(),
                entry.path_truth_class.as_str(),
                entry.boundary_label.as_str(),
                entry.write_posture.as_str(),
                entry.path_condition.as_str(),
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        for tally in &self.findings_summary.by_class {
            out.push_str(&format!("| `{}` | {} |\n", tally.class, tally.count));
        }
        if self.findings_summary.by_class.is_empty() {
            out.push_str("| _(none)_ | 0 |\n");
        }
        out.push('\n');

        out.push_str("## Per-flow rows\n\n");
        for entry in &self.entries {
            let d = &entry.descriptor;
            out.push_str(&format!(
                "### `{}` ({})\n\n",
                d.flow_id,
                d.flow_kind.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                d.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Literal target: `{}` (`{}`)\n",
                d.literal_target_ref,
                d.literal_format.as_str(),
            ));
            out.push_str(&format!(
                "- Canonical target: `{}` (`{}`)\n",
                d.canonical_target_ref,
                d.path_truth_class.as_str(),
            ));
            out.push_str(&format!(
                "- Detected target kind: `{}`\n",
                d.detected_target_kind.as_str()
            ));
            out.push_str(&format!(
                "- Boundary: `{}` (`{}`)\n",
                d.boundary_label.as_str(),
                d.boundary_label_ref,
            ));
            out.push_str(&format!(
                "- Overwrite posture: `{}` (checkpoint: `{}`)\n",
                d.write_posture.as_str(),
                d.checkpoint_availability.as_str(),
            ));
            if let Some(checkpoint) = &d.checkpoint_ref {
                out.push_str(&format!("- Checkpoint: `{checkpoint}`\n"));
            }
            out.push_str(&format!(
                "- Overwrite review: `{}`\n",
                d.overwrite_review_ref
            ));
            out.push_str(&format!(
                "- Reveal side effect: `{}`\n",
                d.reveal_side_effect.as_str()
            ));
            if let Some(label) = &d.reveal_action_label_ref {
                out.push_str(&format!("- Reveal action label: `{label}`\n"));
            }
            out.push_str(&format!(
                "- Filesystem identity: `{}`\n",
                d.filesystem_identity_ref
            ));
            out.push_str(&format!(
                "- Save coordination: `{}`\n",
                d.save_coordination_ref
            ));
            out.push_str(&format!(
                "- Active profile owner: `{}`\n",
                d.active_profile_owner_ref
            ));
            out.push_str(&format!(
                "- Trust checkpoint: `{}`\n",
                d.trust_checkpoint_ref
            ));
            out.push_str(&format!(
                "- Canonical command: `{}`\n",
                d.canonical_command_ref
            ));
            out.push_str(&format!(
                "- Path condition: `{}`\n",
                d.path_condition.as_str()
            ));
            if d.recovery_actions.is_empty() {
                out.push_str("- Recovery actions: _(none required)_\n");
            } else {
                out.push_str(&format!(
                    "- Recovery actions: {}\n",
                    d.recovery_actions
                        .iter()
                        .map(|action| format!("`{}`", action.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            out.push_str(&format!(
                "- Claimed platforms: {}\n",
                d.claimed_platforms
                    .iter()
                    .map(|platform| format!("`{}`", platform.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "- Evidence freshness: `{}` (captured `{}`)\n",
                d.evidence_freshness.as_str(),
                d.evidence_captured_at,
            ));
            out.push_str(&format!("- Downgrade rule: `{}`\n", d.downgrade_rule_ref));
            out.push_str(&format!(
                "- Marketed: `{}`\n",
                if entry.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!("- Continuity note: {}\n", d.continuity_note));
            out.push_str("- Degraded-state vocabulary:\n");
            for phrase in &d.degraded_state_vocabulary {
                out.push_str(&format!("  - {phrase}\n"));
            }
            out.push('\n');

            if entry.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &entry.blocking_findings {
                    out.push_str(&format!("- `{}`\n", finding.class_token()));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- validate\n",
        );
        out.push_str("cargo test -p aureline-workspace --test m5_open_save_reveal_fixtures\n");
        out.push_str("python3 tools/ci/m5/open_save_reveal_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the full open/save/reveal report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenSaveRevealSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: OpenSaveRevealReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl OpenSaveRevealSupportExport {
    /// Builds the support-export wrapper for a report.
    pub fn from_report(support_export_id: impl Into<String>, report: OpenSaveRevealReport) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for entry in &report.entries {
            case_ids.push(entry.descriptor.flow_id.clone());
            case_ids.push(entry.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: OPEN_SAVE_REVEAL_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: OPEN_SAVE_REVEAL_SCHEMA_VERSION,
            shared_contract_ref: OPEN_SAVE_REVEAL_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Per-incident support-export packet for a single degraded flow.
///
/// This is the export a reviewer reproduces a missing-canonical-target,
/// network-share-alias, generated-output, or read-only-destination flow from —
/// the typed diagnostic that replaces a screenshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenSaveRevealCaseExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable case-export id.
    pub case_export_id: String,
    /// Stable case label (e.g. `network_share_alias`).
    pub case_label: String,
    /// Path condition that defines the incident class.
    pub path_condition: PathConditionClass,
    /// Boundary label of the incident target.
    pub boundary_label: BoundaryLabel,
    /// The flow row in full.
    pub flow: SystemDialogFlowRow,
    /// Recovery actions the incident offers.
    pub recovery_actions: Vec<RecoveryAction>,
    /// Stable reproduction note for support.
    pub reproduction_note: String,
}

impl OpenSaveRevealCaseExport {
    /// Builds a per-incident case export from a degraded flow row.
    pub fn from_row(
        case_export_id: impl Into<String>,
        case_label: impl Into<String>,
        reproduction_note: impl Into<String>,
        row: SystemDialogFlowRow,
    ) -> Self {
        let path_condition = row.descriptor.path_condition;
        let boundary_label = row.descriptor.boundary_label;
        let recovery_actions = row.descriptor.recovery_actions.clone();
        Self {
            record_kind: OPEN_SAVE_REVEAL_CASE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: OPEN_SAVE_REVEAL_SCHEMA_VERSION,
            shared_contract_ref: OPEN_SAVE_REVEAL_SHARED_CONTRACT_REF.to_owned(),
            case_export_id: case_export_id.into(),
            case_label: case_label.into(),
            path_condition,
            boundary_label,
            flow: row,
            recovery_actions,
            reproduction_note: reproduction_note.into(),
        }
    }
}

/// Computes the per-flow blocking findings from a descriptor.
fn compute_flow_findings(descriptor: &SystemDialogFlow) -> Vec<FlowBlockingFinding> {
    let mut findings = Vec::new();
    let flow_id = descriptor.flow_id.clone();

    // Identity and ownership integrity.
    if descriptor.literal_target_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::MissingLiteralTarget {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.canonical_target_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::CanonicalPathHidden {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.boundary_label_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::MissingBoundaryLabel {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.overwrite_review_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::CheckpointVocabularyDivergence {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.filesystem_identity_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::MissingFilesystemIdentityRef {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.save_coordination_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::MissingSaveCoordinationRef {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.active_profile_owner_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::MissingActiveProfileOwner {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.trust_checkpoint_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::TrustEvaluationBypassed {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.canonical_command_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::MissingCanonicalCommand {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.continuity_note.trim().is_empty() {
        findings.push(FlowBlockingFinding::MissingContinuityNote {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor
        .degraded_state_vocabulary
        .iter()
        .all(|phrase| phrase.trim().is_empty())
    {
        findings.push(FlowBlockingFinding::MissingDegradedStateVocabulary {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.claimed_platforms.is_empty() {
        findings.push(FlowBlockingFinding::MissingClaimedPlatforms {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.downgrade_rule_ref.trim().is_empty() {
        findings.push(FlowBlockingFinding::MissingDowngradeRule {
            flow_id: flow_id.clone(),
        });
    }
    if !descriptor.registered_on_path_truth_harness {
        findings.push(FlowBlockingFinding::FlowNotOnHarness {
            flow_id: flow_id.clone(),
        });
    }
    if descriptor.marketed && descriptor.evidence_freshness == EvidenceFreshness::Stale {
        findings.push(FlowBlockingFinding::StaleEvidenceOnMarketedFlow {
            flow_id: flow_id.clone(),
        });
    }

    // Overwrite / checkpoint discipline: an in-place overwrite must pin an
    // available checkpoint, reusing the in-product save/restore review.
    if descriptor.write_posture.is_in_place_write() {
        let checkpoint_named = descriptor
            .checkpoint_ref
            .as_deref()
            .map(str::trim)
            .map(str::is_empty)
            == Some(false);
        if descriptor.checkpoint_availability != CheckpointAvailability::Pinned || !checkpoint_named
        {
            findings.push(FlowBlockingFinding::OverwriteWithoutCheckpointReview {
                flow_id: flow_id.clone(),
                checkpoint_availability: descriptor.checkpoint_availability,
            });
        }
    }

    // Read-only discipline: a writing posture against a read-only boundary or
    // destination is a distinct failure.
    let is_read_only = descriptor.boundary_label == BoundaryLabel::ReadOnly
        || descriptor.path_condition == PathConditionClass::ReadOnlyDestination;
    if descriptor.flow_kind.is_write_flow() && is_read_only {
        let writes_anyway = matches!(
            descriptor.write_posture,
            OverwritePosture::CreateNewFile
                | OverwritePosture::OverwriteWithCheckpoint
                | OverwritePosture::OverwriteReviewRequired
        );
        if writes_anyway {
            findings.push(FlowBlockingFinding::ReadOnlyWriteAttempt {
                flow_id: flow_id.clone(),
                write_posture: descriptor.write_posture,
            });
        }
    }

    // Generated discipline: a generated artifact must be exported, never saved
    // in place.
    let is_generated = descriptor.boundary_label == BoundaryLabel::Generated
        || descriptor.path_condition == PathConditionClass::GeneratedOutput;
    if descriptor.flow_kind.is_write_flow() && is_generated {
        let in_place = matches!(
            descriptor.write_posture,
            OverwritePosture::OverwriteWithCheckpoint | OverwritePosture::OverwriteReviewRequired
        );
        if in_place {
            findings.push(FlowBlockingFinding::GeneratedTreatedAsInPlaceSave {
                flow_id: flow_id.clone(),
                write_posture: descriptor.write_posture,
            });
        }
    }

    // Reveal / browser discipline: the external side effect and a stable label
    // must be disclosed. A wrong side effect and a missing label are the same
    // failure class (a hidden reveal side effect), so they collapse into one
    // finding rather than stacking.
    let expected = descriptor.flow_kind.expected_side_effect();
    let side_effect_mismatch = descriptor.reveal_side_effect != expected;
    let label_missing = descriptor.flow_kind.requires_reveal_label()
        && descriptor
            .reveal_action_label_ref
            .as_deref()
            .map(str::trim)
            .map(str::is_empty)
            != Some(false);
    if side_effect_mismatch || label_missing {
        findings.push(FlowBlockingFinding::RevealSideEffectHidden {
            flow_id: flow_id.clone(),
            flow_kind: descriptor.flow_kind,
        });
    }

    // Recovery: a non-exact path condition must offer a recovery action, and
    // each condition stays a distinct failure.
    if descriptor.path_condition.requires_recovery() && descriptor.recovery_actions.is_empty() {
        if let Some(mode) = descriptor.path_condition.missing_recovery_failure_mode() {
            let finding = match mode {
                FlowFailureMode::WrongTargetSave => FlowBlockingFinding::WrongTargetSave {
                    flow_id: flow_id.clone(),
                },
                FlowFailureMode::AliasPathConfusion => FlowBlockingFinding::AliasPathConfusion {
                    flow_id: flow_id.clone(),
                },
                FlowFailureMode::GeneratedOutputUnrecoverable => {
                    FlowBlockingFinding::GeneratedOutputUnrecoverable {
                        flow_id: flow_id.clone(),
                    }
                }
                FlowFailureMode::ReadOnlyDestinationUnrecoverable => {
                    FlowBlockingFinding::ReadOnlyDestinationUnrecoverable {
                        flow_id: flow_id.clone(),
                    }
                }
                _ => FlowBlockingFinding::WrongTargetSave {
                    flow_id: flow_id.clone(),
                },
            };
            findings.push(finding);
        }
    }

    findings
}

/// Builds a [`SystemDialogFlowRow`] from a descriptor, computing the per-flow
/// blocking findings.
pub fn build_open_save_reveal_row(descriptor: SystemDialogFlow) -> SystemDialogFlowRow {
    let marketed = descriptor.marketed;
    let blocking_findings = compute_flow_findings(&descriptor);

    SystemDialogFlowRow {
        record_kind: OPEN_SAVE_REVEAL_ROW_RECORD_KIND.to_owned(),
        schema_version: OPEN_SAVE_REVEAL_SCHEMA_VERSION,
        shared_contract_ref: OPEN_SAVE_REVEAL_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        blocking_findings,
        marketed,
    }
}

/// Computes the per-kind, per-boundary, per-condition, and per-class summaries
/// from finished rows.
fn summarize_report(
    entries: &[SystemDialogFlowRow],
) -> (
    Vec<FlowKindCoverage>,
    Vec<BoundaryLabelCoverage>,
    Vec<PathConditionCoverage>,
    FlowFindingSummary,
) {
    let mut kind_coverage: Vec<FlowKindCoverage> = DialogFlowKind::required_kinds()
        .into_iter()
        .map(|flow_kind| FlowKindCoverage {
            flow_kind,
            flow_count: 0,
        })
        .collect();

    let mut boundary_coverage: Vec<BoundaryLabelCoverage> = BoundaryLabel::all()
        .into_iter()
        .map(|boundary_label| BoundaryLabelCoverage {
            boundary_label,
            flow_count: 0,
            write_protected_count: 0,
        })
        .collect();

    let mut condition_coverage: Vec<PathConditionCoverage> = PathConditionClass::all()
        .into_iter()
        .map(|path_condition| PathConditionCoverage {
            path_condition,
            flow_count: 0,
            recovered_count: 0,
        })
        .collect();

    let mut class_counts: Vec<FlowFindingCount> = Vec::new();
    let mut total = 0usize;

    for entry in entries {
        let descriptor = &entry.descriptor;
        if let Some(kind_row) = kind_coverage
            .iter_mut()
            .find(|row| row.flow_kind == descriptor.flow_kind)
        {
            kind_row.flow_count += 1;
        }
        if let Some(boundary_row) = boundary_coverage
            .iter_mut()
            .find(|row| row.boundary_label == descriptor.boundary_label)
        {
            boundary_row.flow_count += 1;
            if matches!(
                descriptor.write_posture,
                OverwritePosture::WriteBlockedReadOnly | OverwritePosture::ExportNotInPlaceSave
            ) {
                boundary_row.write_protected_count += 1;
            }
        }
        if let Some(condition_row) = condition_coverage
            .iter_mut()
            .find(|row| row.path_condition == descriptor.path_condition)
        {
            condition_row.flow_count += 1;
            if !descriptor.recovery_actions.is_empty() {
                condition_row.recovered_count += 1;
            }
        }
        for finding in &entry.blocking_findings {
            total += 1;
            let class = finding.class_token();
            if let Some(tally) = class_counts.iter_mut().find(|tally| tally.class == class) {
                tally.count += 1;
            } else {
                class_counts.push(FlowFindingCount {
                    class: class.to_owned(),
                    count: 1,
                });
            }
        }
    }

    class_counts.sort_by(|left, right| left.class.cmp(&right.class));
    (
        kind_coverage,
        boundary_coverage,
        condition_coverage,
        FlowFindingSummary {
            total_blocking_findings: total,
            by_class: class_counts,
        },
    )
}

/// Computes the marketed flows release tooling should narrow because a control
/// failed or their evidence is stale.
fn compute_narrowable_entries(entries: &[SystemDialogFlowRow]) -> Vec<FlowNarrowableEntry> {
    let mut narrowable = Vec::new();
    for entry in entries {
        if !entry.marketed {
            continue;
        }
        for finding in &entry.blocking_findings {
            narrowable.push(FlowNarrowableEntry {
                flow_id: entry.descriptor.flow_id.clone(),
                failure_mode: finding.failure_mode(),
                reason: format!("blocking_finding:{}", finding.class_token()),
            });
        }
    }
    narrowable
}

/// Builds a full [`OpenSaveRevealReport`] from per-flow rows.
pub fn build_open_save_reveal_report(entries: Vec<SystemDialogFlowRow>) -> OpenSaveRevealReport {
    let mut entries = entries;
    entries.sort_by(|left, right| left.descriptor.flow_id.cmp(&right.descriptor.flow_id));

    let registered_flow_count = entries.len();
    let marketed_flow_count = entries.iter().filter(|entry| entry.marketed).count();
    let write_flow_count = entries
        .iter()
        .filter(|entry| entry.descriptor.flow_kind.is_write_flow())
        .count();

    let (flow_kind_coverage, boundary_label_coverage, path_condition_coverage, findings_summary) =
        summarize_report(&entries);
    let narrowable_marketed_entries = compute_narrowable_entries(&entries);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut platform_set: Vec<Platform> = Vec::new();
    for entry in &entries {
        for platform in &entry.descriptor.claimed_platforms {
            if !platform_set.contains(platform) {
                platform_set.push(*platform);
            }
        }
    }
    platform_set.sort();

    let mut path_truth_index: Vec<PathTruthIndexEntry> = entries
        .iter()
        .map(|entry| PathTruthIndexEntry {
            flow_id: entry.descriptor.flow_id.clone(),
            flow_kind: entry.descriptor.flow_kind,
            canonical_target_ref: entry.descriptor.canonical_target_ref.clone(),
            path_truth_class: entry.descriptor.path_truth_class,
            boundary_label: entry.descriptor.boundary_label,
            write_posture: entry.descriptor.write_posture,
            path_condition: entry.descriptor.path_condition,
        })
        .collect();
    path_truth_index.sort_by(|left, right| left.flow_id.cmp(&right.flow_id));

    OpenSaveRevealReport {
        record_kind: OPEN_SAVE_REVEAL_REPORT_RECORD_KIND.to_owned(),
        schema_version: OPEN_SAVE_REVEAL_SCHEMA_VERSION,
        shared_contract_ref: OPEN_SAVE_REVEAL_SHARED_CONTRACT_REF.to_owned(),
        report_id: OPEN_SAVE_REVEAL_REPORT_ID.to_owned(),
        source_schema_ref: OPEN_SAVE_REVEAL_SOURCE_SCHEMA_REF.to_owned(),
        required_flow_kinds: DialogFlowKind::required_kinds().to_vec(),
        claimed_platforms: platform_set,
        cross_links: OpenSaveRevealCrossLinks::canonical(),
        entries,
        flow_kind_coverage,
        boundary_label_coverage,
        path_condition_coverage,
        findings_summary,
        path_truth_index,
        registered_flow_count,
        marketed_flow_count,
        write_flow_count,
        narrowable_marketed_entries,
        report_clean,
        published_report_ref: OPEN_SAVE_REVEAL_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: OPEN_SAVE_REVEAL_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            OPEN_SAVE_REVEAL_PUBLISHED_DOC_REF.to_owned(),
            "docs/help/open_save_reveal_path_truth.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-open-save-reveal".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_open_save_reveal_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum OpenSaveRevealValidationError {
    /// The report has no registered flows.
    NoRegisteredFlows,
    /// A required flow kind has no registered flow.
    RequiredFlowKindMissing {
        /// Flow kind token that is missing.
        flow_kind: String,
    },
    /// A blocking finding remains on a flow.
    BlockingFindingPresent {
        /// Flow id the finding is attached to.
        flow_id: String,
        /// Finding class token.
        class: String,
    },
    /// A cross-link ref is empty.
    CrossLinkMissing {
        /// Cross-link field that is empty.
        field: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A flow's descriptor revision ref is empty.
    MissingDescriptorRevisionRef {
        /// Flow id that exposes the gap.
        flow_id: String,
    },
}

/// Validates a report against the open/save/reveal acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_open_save_reveal_report(
    report: &OpenSaveRevealReport,
) -> Result<(), Vec<OpenSaveRevealValidationError>> {
    let mut errors = Vec::new();

    if report.entries.is_empty() {
        errors.push(OpenSaveRevealValidationError::NoRegisteredFlows);
    }

    for kind in DialogFlowKind::required_kinds() {
        let present = report
            .entries
            .iter()
            .any(|entry| entry.descriptor.flow_kind == kind);
        if !present {
            errors.push(OpenSaveRevealValidationError::RequiredFlowKindMissing {
                flow_kind: kind.as_str().to_owned(),
            });
        }
    }

    for entry in &report.entries {
        if entry.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(
                OpenSaveRevealValidationError::MissingDescriptorRevisionRef {
                    flow_id: entry.descriptor.flow_id.clone(),
                },
            );
        }
        for finding in &entry.blocking_findings {
            errors.push(OpenSaveRevealValidationError::BlockingFindingPresent {
                flow_id: finding.flow_id().to_owned(),
                class: finding.class_token().to_owned(),
            });
        }
    }

    for (field, value) in report.cross_links.as_pairs() {
        if value.trim().is_empty() {
            errors.push(OpenSaveRevealValidationError::CrossLinkMissing {
                field: field.to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(OpenSaveRevealValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(OpenSaveRevealValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_open_save_reveal_report`].
struct FlowSeed {
    flow_id: &'static str,
    flow_kind: DialogFlowKind,
    literal_target_ref: &'static str,
    literal_format: PathLiteralFormat,
    canonical_target_ref: &'static str,
    path_truth_class: PathTruthClass,
    detected_target_kind: TargetKind,
    boundary_label: BoundaryLabel,
    write_posture: OverwritePosture,
    checkpoint_availability: CheckpointAvailability,
    checkpoint_ref: Option<&'static str>,
    reveal_side_effect: RevealSideEffect,
    reveal_action_label_ref: Option<&'static str>,
    canonical_command_ref: &'static str,
    path_condition: PathConditionClass,
    recovery_actions: &'static [RecoveryAction],
    continuity_note: &'static str,
    degraded_state_vocabulary: &'static [&'static str],
}

fn build_flow_from_seed(seed: &FlowSeed) -> SystemDialogFlowRow {
    let descriptor = SystemDialogFlow {
        flow_id: seed.flow_id.to_owned(),
        flow_kind: seed.flow_kind,
        descriptor_revision_ref: format!("{}:rev:2026.06.01-01", seed.flow_id),
        primary_label_ref: format!("label:{}:primary", seed.flow_id),
        literal_target_ref: seed.literal_target_ref.to_owned(),
        literal_format: seed.literal_format,
        canonical_target_ref: seed.canonical_target_ref.to_owned(),
        path_truth_class: seed.path_truth_class,
        detected_target_kind: seed.detected_target_kind,
        boundary_label: seed.boundary_label,
        boundary_label_ref: format!("boundary:{}:{}", seed.flow_id, seed.boundary_label.as_str()),
        write_posture: seed.write_posture,
        checkpoint_availability: seed.checkpoint_availability,
        checkpoint_ref: seed.checkpoint_ref.map(str::to_owned),
        overwrite_review_ref: "save:overwrite_review:checkpoint_aware:v1".to_owned(),
        reveal_side_effect: seed.reveal_side_effect,
        reveal_action_label_ref: seed.reveal_action_label_ref.map(str::to_owned),
        filesystem_identity_ref: format!("filesystem-identity:{}", seed.flow_id),
        save_coordination_ref: format!("save-coordination:{}", seed.flow_id),
        active_profile_owner_ref: format!("profile-owner:{}", seed.flow_id),
        trust_checkpoint_ref: format!("trust:{}:profile_policy", seed.flow_id),
        canonical_command_ref: seed.canonical_command_ref.to_owned(),
        path_condition: seed.path_condition,
        recovery_actions: seed.recovery_actions.to_vec(),
        continuity_note: seed.continuity_note.to_owned(),
        degraded_state_vocabulary: seed
            .degraded_state_vocabulary
            .iter()
            .map(|phrase| (*phrase).to_owned())
            .collect(),
        claimed_platforms: Platform::all().to_vec(),
        evidence_freshness: EvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:open_save_reveal:narrow_on_stale_evidence".to_owned(),
        marketed: true,
        registered_on_path_truth_harness: true,
    };
    build_open_save_reveal_row(descriptor)
}

const FLOW_SEEDS: &[FlowSeed] = &[
    // ---- Clean flow-kind rows: one per required kind. ----
    // Open: a plain local file open, literal is canonical.
    FlowSeed {
        flow_id: "flow:open.local_file",
        flow_kind: DialogFlowKind::Open,
        literal_target_ref: "literal:open.local_file:captured",
        literal_format: PathLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:open.local_file:single_file",
        path_truth_class: PathTruthClass::LiteralIsCanonical,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::LocalWritable,
        write_posture: OverwritePosture::NoWriteAction,
        checkpoint_availability: CheckpointAvailability::NotApplicable,
        checkpoint_ref: None,
        reveal_side_effect: RevealSideEffect::NoExternalSideEffect,
        reveal_action_label_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        path_condition: PathConditionClass::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "A system open of a local file targets the literal file the user selected, which is its own canonical target, and reads it without widening scope.",
        degraded_state_vocabulary: &[
            "Open this file",
            "This file is no longer at the path you selected",
            "Choose a different file",
        ],
    },
    // Save: an in-place save that overwrites with a pinned checkpoint.
    FlowSeed {
        flow_id: "flow:save.local_file",
        flow_kind: DialogFlowKind::Save,
        literal_target_ref: "literal:save.local_file:captured",
        literal_format: PathLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:save.local_file:single_file",
        path_truth_class: PathTruthClass::LiteralIsCanonical,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::LocalWritable,
        write_posture: OverwritePosture::OverwriteWithCheckpoint,
        checkpoint_availability: CheckpointAvailability::Pinned,
        checkpoint_ref: Some("checkpoint:save.local_file:pre_overwrite"),
        reveal_side_effect: RevealSideEffect::NoExternalSideEffect,
        reveal_action_label_ref: None,
        canonical_command_ref: "cmd:workspace.save.target",
        path_condition: PathConditionClass::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "An in-place save overwrites the canonical target only after pinning a checkpoint, using the same checkpoint-aware overwrite review the in-product save flow performs.",
        degraded_state_vocabulary: &[
            "Save changes to this file",
            "This will overwrite the file on disk",
            "Restore from the checkpoint taken before this save",
        ],
    },
    // Save As: a save to a new target that does not yet exist.
    FlowSeed {
        flow_id: "flow:save_as.local_file",
        flow_kind: DialogFlowKind::SaveAs,
        literal_target_ref: "literal:save_as.local_file:captured",
        literal_format: PathLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:save_as.local_file:new_file",
        path_truth_class: PathTruthClass::LiteralIsCanonical,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::LocalWritable,
        write_posture: OverwritePosture::CreateNewFile,
        checkpoint_availability: CheckpointAvailability::NotApplicable,
        checkpoint_ref: None,
        reveal_side_effect: RevealSideEffect::NoExternalSideEffect,
        reveal_action_label_ref: None,
        canonical_command_ref: "cmd:workspace.save_as.target",
        path_condition: PathConditionClass::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "A save-as writes to the new literal target the user named; because no file exists there yet it creates a new file rather than overwriting, so no checkpoint is required.",
        degraded_state_vocabulary: &[
            "Save a copy to a new file",
            "A file with this name already exists here",
            "Choose a different name or location",
        ],
    },
    // Reveal in system shell: selects the local file in the OS file manager.
    FlowSeed {
        flow_id: "flow:reveal.local_file",
        flow_kind: DialogFlowKind::RevealInSystemShell,
        literal_target_ref: "literal:reveal.local_file:captured",
        literal_format: PathLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:reveal.local_file:single_file",
        path_truth_class: PathTruthClass::LiteralIsCanonical,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::LocalWritable,
        write_posture: OverwritePosture::NoWriteAction,
        checkpoint_availability: CheckpointAvailability::NotApplicable,
        checkpoint_ref: None,
        reveal_side_effect: RevealSideEffect::SelectsTargetInFileManager,
        reveal_action_label_ref: Some("action:reveal.local_file:reveal_in_system_shell"),
        canonical_command_ref: "cmd:workspace.reveal_in_system_shell",
        path_condition: PathConditionClass::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "Reveal in system shell is a stable, explicit action: it opens the OS file manager and selects the canonical target, and discloses that external side effect rather than hiding it.",
        degraded_state_vocabulary: &[
            "Reveal in system shell",
            "This opens your file manager and selects the file",
            "Reveal the parent folder instead",
        ],
    },
    // Open in default browser: hands a generated preview to the browser.
    FlowSeed {
        flow_id: "flow:open_in_browser.generated_preview",
        flow_kind: DialogFlowKind::OpenInDefaultBrowser,
        literal_target_ref: "literal:open_in_browser.generated_preview:captured",
        literal_format: PathLiteralFormat::Url,
        canonical_target_ref: "canonical:open_in_browser.generated_preview:generated_html",
        path_truth_class: PathTruthClass::BoundaryLabeledArtifact,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::Generated,
        write_posture: OverwritePosture::NoWriteAction,
        checkpoint_availability: CheckpointAvailability::NotApplicable,
        checkpoint_ref: None,
        reveal_side_effect: RevealSideEffect::OpensDefaultBrowser,
        reveal_action_label_ref: Some("action:open_in_browser.generated_preview:open_in_default_browser"),
        canonical_command_ref: "cmd:workspace.open_in_default_browser",
        path_condition: PathConditionClass::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "Open in default browser is a stable, explicit action that hands a generated preview artifact to the default browser; the target is labeled generated so it is never mistaken for an editable source.",
        degraded_state_vocabulary: &[
            "Open in default browser",
            "This is a generated preview, not the source file",
            "Open the source that generated this instead",
        ],
    },
    // ---- Degraded case rows: the four required failure-path fixtures. ----
    // Missing canonical target: a save-as whose canonical target cannot be
    // resolved from the literal.
    FlowSeed {
        flow_id: "flow:case.missing_canonical_target",
        flow_kind: DialogFlowKind::SaveAs,
        literal_target_ref: "literal:case.missing_canonical_target:captured",
        literal_format: PathLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:case.missing_canonical_target:unresolved",
        path_truth_class: PathTruthClass::CanonicalTargetMissing,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::LocalWritable,
        write_posture: OverwritePosture::OverwriteReviewRequired,
        checkpoint_availability: CheckpointAvailability::NotApplicable,
        checkpoint_ref: None,
        reveal_side_effect: RevealSideEffect::NoExternalSideEffect,
        reveal_action_label_ref: None,
        canonical_command_ref: "cmd:workspace.save_as.target",
        path_condition: PathConditionClass::MissingCanonicalTarget,
        recovery_actions: &[
            RecoveryAction::ChooseDifferentTarget,
            RecoveryAction::ShowCanonicalPath,
        ],
        continuity_note: "When the canonical target cannot be resolved from the literal the user selected, the save is held for explicit review instead of writing to a guessed path, with a target picker and the canonical-path detail offered.",
        degraded_state_vocabulary: &[
            "The file you selected no longer resolves to a known location",
            "Show where this path points",
            "Choose a different file to save to",
        ],
    },
    // Network-share alias: a save through a network-share alias that resolves to
    // a different canonical target.
    FlowSeed {
        flow_id: "flow:case.network_share_alias",
        flow_kind: DialogFlowKind::Save,
        literal_target_ref: "literal:case.network_share_alias:captured",
        literal_format: PathLiteralFormat::WindowsUncPath,
        canonical_target_ref: "canonical:case.network_share_alias:share_target",
        path_truth_class: PathTruthClass::CanonicalAliasResolved,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::RemoteAdjacent,
        write_posture: OverwritePosture::OverwriteReviewRequired,
        checkpoint_availability: CheckpointAvailability::Unavailable,
        checkpoint_ref: None,
        reveal_side_effect: RevealSideEffect::NoExternalSideEffect,
        reveal_action_label_ref: None,
        canonical_command_ref: "cmd:workspace.save.target",
        path_condition: PathConditionClass::NetworkShareAlias,
        recovery_actions: &[
            RecoveryAction::ResolveShareAlias,
            RecoveryAction::ReconnectShare,
            RecoveryAction::ShowCanonicalPath,
        ],
        continuity_note: "A save through a network-share alias discloses the canonical share target the alias resolves to and holds the write for review, so an alias-path confusion can never silently land on the wrong remote target.",
        degraded_state_vocabulary: &[
            "This path is a network-share alias",
            "Show the share target it points to",
            "Reconnect the network share to continue",
        ],
    },
    // Generated output: a save of a generated artifact, correctly exported rather
    // than saved in place.
    FlowSeed {
        flow_id: "flow:case.generated_output",
        flow_kind: DialogFlowKind::Save,
        literal_target_ref: "literal:case.generated_output:captured",
        literal_format: PathLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:case.generated_output:generated_file",
        path_truth_class: PathTruthClass::BoundaryLabeledArtifact,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::Generated,
        write_posture: OverwritePosture::ExportNotInPlaceSave,
        checkpoint_availability: CheckpointAvailability::NotApplicable,
        checkpoint_ref: None,
        reveal_side_effect: RevealSideEffect::NoExternalSideEffect,
        reveal_action_label_ref: None,
        canonical_command_ref: "cmd:workspace.save.target",
        path_condition: PathConditionClass::GeneratedOutput,
        recovery_actions: &[
            RecoveryAction::ExportInsteadOfSave,
            RecoveryAction::RegenerateFromSource,
            RecoveryAction::ShowCanonicalPath,
        ],
        continuity_note: "Saving a generated artifact is presented as an export, not an in-place save, with a path to regenerate from the canonical source, so a generated output is never mistaken for an editable file.",
        degraded_state_vocabulary: &[
            "This is a generated file",
            "Export a copy instead of editing it in place",
            "Regenerate it from its source",
        ],
    },
    // Read-only destination: a save to a read-only destination, correctly
    // blocked.
    FlowSeed {
        flow_id: "flow:case.read_only_destination",
        flow_kind: DialogFlowKind::Save,
        literal_target_ref: "literal:case.read_only_destination:captured",
        literal_format: PathLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:case.read_only_destination:read_only_file",
        path_truth_class: PathTruthClass::BoundaryLabeledArtifact,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::ReadOnly,
        write_posture: OverwritePosture::WriteBlockedReadOnly,
        checkpoint_availability: CheckpointAvailability::NotApplicable,
        checkpoint_ref: None,
        reveal_side_effect: RevealSideEffect::NoExternalSideEffect,
        reveal_action_label_ref: None,
        canonical_command_ref: "cmd:workspace.save.target",
        path_condition: PathConditionClass::ReadOnlyDestination,
        recovery_actions: &[
            RecoveryAction::SaveWritableCopyElsewhere,
            RecoveryAction::OpenReadOnly,
            RecoveryAction::ShowCanonicalPath,
        ],
        continuity_note: "A save to a read-only destination blocks the in-place write and offers a writable copy elsewhere, so platform-native dialog convenience never lets a read-only target appear writable.",
        degraded_state_vocabulary: &[
            "This destination is read-only",
            "Save a writable copy somewhere else",
            "Open it read-only instead",
        ],
    },
];

/// Seeded report builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/platform/m5-open-save-reveal/`.
pub fn seeded_open_save_reveal_report() -> OpenSaveRevealReport {
    let entries = FLOW_SEEDS.iter().map(build_flow_from_seed).collect();
    build_open_save_reveal_report(entries)
}

/// Stable case-id label for the four required failure-path fixtures.
pub const OPEN_SAVE_REVEAL_CASE_LABELS: [(&str, &str); 4] = [
    (
        "flow:case.missing_canonical_target",
        "missing_canonical_target",
    ),
    ("flow:case.network_share_alias", "network_share_alias"),
    ("flow:case.generated_output", "generated_output"),
    ("flow:case.read_only_destination", "read_only_destination"),
];

/// Builds the four per-incident case exports from the seeded report, in
/// canonical order.
pub fn seeded_open_save_reveal_case_exports() -> Vec<OpenSaveRevealCaseExport> {
    let report = seeded_open_save_reveal_report();
    OPEN_SAVE_REVEAL_CASE_LABELS
        .iter()
        .filter_map(|(flow_id, label)| {
            let row = report
                .entries
                .iter()
                .find(|entry| entry.descriptor.flow_id == *flow_id)?
                .clone();
            Some(OpenSaveRevealCaseExport::from_row(
                format!("support-export:m5-open-save-reveal:case:{label}"),
                *label,
                format!(
                    "Reproduce the {label} flow from this typed diagnostic: the literal target the user selected, the canonical target Aureline resolved, the boundary label, the overwrite/checkpoint posture, and the offered recovery actions.",
                ),
                row,
            ))
        })
        .collect()
}
