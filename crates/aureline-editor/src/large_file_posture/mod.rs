//! Large-file stability posture: the editor's governed, export-safe projection
//! that stabilizes large-file mode, binary-safe preview, and the restricted-write
//! posture into one record per posture.
//!
//! Two live truth sources feed this projection, and it ingests each verbatim
//! rather than re-deriving an outcome:
//!
//! 1. **At-open classification** — the large-file classifier
//!    ([`crate::large_file::ClassificationDecision`]) carries the activation
//!    mode, the trigger, the human-readable reason, and the bounded sniff
//!    summary (binary signal, BOM, null bytes, max line length, non-printable
//!    ratio). It is observed as a serializable
//!    [`LargeFileClassificationObservation`] so the projection stays
//!    serializable for fixtures and replay.
//! 2. **Limited-mode capability posture** — the constrained-viewer record
//!    ([`crate::large_file_mode::LimitedModeFileRecord`]) carries the canonical
//!    write target, the safe-preview class, the edit/write policy, the explicit
//!    override route, and the reduced-capability table.
//!
//! The projection proves the four claims the stable line is anchored on,
//! specialized to the large-file lane:
//!
//! - **Source fidelity** — the constrained viewer reads raw bytes in bounded
//!   pages and never decodes-then-reencodes the whole file, so source bytes are
//!   preserved exactly; binary-like content is given a binary-safe preview
//!   rather than a lossy text render, and a detected BOM is preserved, not
//!   silently stripped.
//! - **Canonical-path truth** — the write target is the VFS canonical object, so
//!   a constrained write cannot land on the wrong target.
//! - **Restore is no-rerun** — whole-file save participants, whole-file
//!   format-on-save, and whole-file AI apply are blocked, so no whole-file
//!   transform silently re-runs over a large or binary file; only reviewed
//!   range-only writes are admitted, and the escalation route is explicit.
//! - **Lineage / export honesty** — the record carries no raw source bytes and
//!   is safe for support export.
//!
//! When the projection cannot prove a claim on the captured posture it
//! auto-narrows below Stable with a named [`LargeFilePostureNarrowReason`]
//! instead of inheriting an adjacent green row. Limited mode being active is
//! itself the protective posture — a read-only constrained viewer over a binary
//! file is the contract working as designed, a pass, not a gap. Narrowing fires
//! only when a protection is missing: a non-byte-faithful read, a binary file
//! without a binary-safe preview, an unresolved canonical target, a whole-file
//! write that is not restricted, an undisclosed escalation, a destructive action
//! with no compare/checkpoint inspection path, or an export-unsafe record.
//!
//! Every record sets `raw_payload_excluded = true` and carries no raw source, so
//! it is safe for support export.

use serde::{Deserialize, Serialize};

use crate::large_file::{BomKind, ClassificationDecision, FileMode, LargeFileTrigger};
use crate::large_file_mode::{
    LimitedModeCapabilityRecord, LimitedModeCapabilityState, LimitedModeFileRecord,
};

/// Schema version for the large-file posture record.
pub const LARGE_FILE_POSTURE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the large-file posture record.
pub const LARGE_FILE_POSTURE_SCHEMA_REF: &str = "schemas/editor/large_file_posture.schema.json";

/// Stable record-kind tag for the large-file posture record.
pub const LARGE_FILE_POSTURE_RECORD_KIND: &str = "large_file_posture_record";

// Capability ids the posture projection evaluates. These are drawn verbatim from
// `default_limited_mode_capabilities` so the proof stays self-contained and the
// record shows exactly which capabilities drove each verdict.
const CAP_WHOLE_FILE_LOAD: &str = "whole_file_load_into_ram";
const CAP_SAVE_PARTICIPANT_WHOLE_FILE: &str = "save_participant_whole_file";
const CAP_SAVE_PARTICIPANT_RANGE_ONLY: &str = "save_participant_range_only";
const CAP_FULL_FILE_FORMAT_ON_SAVE: &str = "full_file_format_on_save";
const CAP_AI_APPLY_WHOLE_FILE: &str = "ai_apply_whole_file";

/// The capability ids whose denial proves whole-file participant writes are
/// blocked in limited mode.
const WHOLE_FILE_WRITE_CAPABILITIES: [&str; 3] = [
    CAP_SAVE_PARTICIPANT_WHOLE_FILE,
    CAP_FULL_FILE_FORMAT_ON_SAVE,
    CAP_AI_APPLY_WHOLE_FILE,
];

/// The capability ids the projection embeds as the self-contained evidence set.
const EVALUATED_CAPABILITY_IDS: [&str; 5] = [
    CAP_WHOLE_FILE_LOAD,
    CAP_SAVE_PARTICIPANT_WHOLE_FILE,
    CAP_SAVE_PARTICIPANT_RANGE_ONLY,
    CAP_FULL_FILE_FORMAT_ON_SAVE,
    CAP_AI_APPLY_WHOLE_FILE,
];

// ---------------------------------------------------------------------------
// Classification observation.
// ---------------------------------------------------------------------------

/// A serializable observation of the at-open large-file classification.
///
/// This is the projection's serializable mirror of a
/// [`ClassificationDecision`]; the editor populates it from the live classifier
/// with [`LargeFileClassificationObservation::from_classification_decision`],
/// and fixtures / replay reconstruct it from JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargeFileClassificationObservation {
    /// Activation mode: `normal` or `large_file`.
    pub mode: String,
    /// Activation trigger token, when large-file mode is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    /// Human-readable activation reason.
    pub reason: String,
    /// Bytes observed on disk at classification time.
    pub bytes_on_disk: u64,
    /// Bytes actually inspected in the bounded sniff window.
    pub sniff_bytes: u64,
    /// Whether any NUL byte was observed in the sniff window.
    pub has_null_bytes: bool,
    /// Longest run of bytes without LF or CR in the sniff window.
    pub max_line_length_in_sniff: u64,
    /// BOM detected at byte 0, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bom_kind: Option<String>,
    /// Non-printable byte ratio in the sniff window, in parts per thousand.
    pub non_printable_per_mille: u16,
    /// Whether the sniff concluded the content looks binary.
    pub looks_binary: bool,
    /// Whether the sniff concluded the content looks minified.
    pub looks_minified: bool,
    /// Whether the path matches a configured large-file pack suffix.
    pub matches_pack_suffix: bool,
}

impl LargeFileClassificationObservation {
    /// Observes a live classifier decision as a serializable record.
    pub fn from_classification_decision(decision: &ClassificationDecision) -> Self {
        Self {
            mode: file_mode_token(decision.mode),
            trigger: decision.trigger.map(large_file_trigger_token),
            reason: decision.reason.clone(),
            bytes_on_disk: decision.bytes_on_disk,
            sniff_bytes: decision.sniff.sniff_bytes,
            has_null_bytes: decision.sniff.has_null_bytes,
            max_line_length_in_sniff: decision.sniff.max_line_length_in_sniff,
            bom_kind: decision.sniff.bom_kind.map(bom_kind_token),
            non_printable_per_mille: decision.sniff.non_printable_per_mille,
            looks_binary: decision.sniff.heuristics.looks_binary,
            looks_minified: decision.sniff.heuristics.looks_minified,
            matches_pack_suffix: decision.sniff.heuristics.matches_pack_suffix,
        }
    }

    /// Returns true when this observation activates large-file mode.
    pub fn is_large_file(&self) -> bool {
        self.mode == "large_file"
    }
}

// ---------------------------------------------------------------------------
// Inspection hooks.
// ---------------------------------------------------------------------------

/// Class of pre-destructive inspection / repair hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectionHookClass {
    /// Compare on-disk bytes / external-change state before any write.
    Compare,
    /// Create a recovery checkpoint before an override or write.
    Checkpoint,
    /// Export the posture record (support-safe, no raw bytes).
    Export,
    /// Re-classify or re-open without destructive cleanup.
    Repair,
}

impl InspectionHookClass {
    /// Returns the stable string vocabulary for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compare => "compare",
            Self::Checkpoint => "checkpoint",
            Self::Export => "export",
            Self::Repair => "repair",
        }
    }
}

/// One pre-destructive inspection / repair hook the user can reach before any
/// destructive cleanup (override to the normal buffer, or a constrained write).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionHook {
    /// Hook class.
    pub hook_class: InspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable for this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default large-file inspection / repair hook table.
///
/// All four hooks are available by default: a large-file posture must always let
/// the user compare, checkpoint, export, and repair before any destructive
/// override or write.
pub fn default_large_file_inspection_hooks() -> Vec<InspectionHook> {
    vec![
        InspectionHook {
            hook_class: InspectionHookClass::Compare,
            action_id: "large_file.compare_on_disk".to_owned(),
            label: "Compare with on-disk bytes".to_owned(),
            available: true,
            disclosure:
                "Compares the constrained preview against the current on-disk bytes before any write."
                    .to_owned(),
        },
        InspectionHook {
            hook_class: InspectionHookClass::Checkpoint,
            action_id: "large_file.checkpoint".to_owned(),
            label: "Create recovery checkpoint".to_owned(),
            available: true,
            disclosure:
                "Records a local-history recovery checkpoint before escalating to the normal buffer or writing."
                    .to_owned(),
        },
        InspectionHook {
            hook_class: InspectionHookClass::Export,
            action_id: "large_file.export_posture".to_owned(),
            label: "Export posture record".to_owned(),
            available: true,
            disclosure: "Exports this posture record for support without raw file bytes.".to_owned(),
        },
        InspectionHook {
            hook_class: InspectionHookClass::Repair,
            action_id: "large_file.reclassify".to_owned(),
            label: "Re-classify and re-open".to_owned(),
            available: true,
            disclosure:
                "Re-runs classification and re-opens the document without clearing local state."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Narrow reasons + stable qualification.
// ---------------------------------------------------------------------------

/// Named reason a large-file posture record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LargeFilePostureNarrowReason {
    /// The constrained read path is not byte-faithful (whole-file load admitted).
    SourceReadNotByteFaithful,
    /// Binary-like content is not given a binary-safe preview.
    PreviewNotBinarySafe,
    /// The canonical write target could not be resolved.
    CanonicalTargetUnresolved,
    /// Whole-file participant writes are not restricted in limited mode.
    WholeFileWriteNotRestricted,
    /// The escalation / override route carries no disclosure.
    OverrideRouteNotDisclosed,
    /// A destructive action is reachable with no compare + checkpoint path.
    DestructiveActionNoCheckpoint,
    /// The record or its capability set is not export-safe.
    PostureExportUnsafe,
}

impl LargeFilePostureNarrowReason {
    /// Returns the stable string vocabulary for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceReadNotByteFaithful => "source_read_not_byte_faithful",
            Self::PreviewNotBinarySafe => "preview_not_binary_safe",
            Self::CanonicalTargetUnresolved => "canonical_target_unresolved",
            Self::WholeFileWriteNotRestricted => "whole_file_write_not_restricted",
            Self::OverrideRouteNotDisclosed => "override_route_not_disclosed",
            Self::DestructiveActionNoCheckpoint => "destructive_action_no_checkpoint",
            Self::PostureExportUnsafe => "posture_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a large-file posture record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargeFilePostureQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not qualified.
    pub narrow_reasons: Vec<LargeFilePostureNarrowReason>,
}

// ---------------------------------------------------------------------------
// Projected sub-records.
// ---------------------------------------------------------------------------

/// Why large-file mode is active, projected from the classification observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargeFileActivationSummary {
    /// Activation mode token: `normal` or `large_file`.
    pub mode: String,
    /// Activation trigger token, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_class: Option<String>,
    /// Human-readable activation reason.
    pub reason: String,
    /// Whether the content sniff concluded the file looks binary.
    pub looks_binary: bool,
    /// Whether the content sniff concluded the file looks minified.
    pub looks_minified: bool,
    /// Whether the path matched a configured large-file pack suffix.
    pub matches_pack_suffix: bool,
    /// BOM detected at byte 0, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bom_kind: Option<String>,
    /// Whether any NUL byte was observed in the sniff window.
    pub has_null_bytes: bool,
    /// Bytes inspected in the bounded sniff window.
    pub sniff_bytes: u64,
    /// Non-printable byte ratio in the sniff window, in parts per thousand.
    pub non_printable_per_mille: u16,
}

/// Binary-safe preview / source-fidelity posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewFidelitySummary {
    /// Safe-preview class token: `paged_raw_text_preview` or `binary_safe_preview`.
    pub safe_preview_class: String,
    /// Whether the constrained read path is byte-faithful (whole-file load blocked).
    pub byte_faithful_read: bool,
    /// Whether whole-file load into the normal buffer is blocked.
    pub whole_file_load_blocked: bool,
    /// Whether binary-like content is given a binary-safe preview.
    pub binary_safe_preview_selected: bool,
    /// Whether a detected BOM is preserved rather than silently stripped.
    pub bom_preserved: bool,
    /// Overall source-fidelity verdict for the constrained preview.
    pub source_fidelity_proven: bool,
}

/// Restricted-write posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedWritePosture {
    /// Edit policy token from the limited-mode record.
    pub edit_policy_class: String,
    /// Write policy token from the limited-mode record.
    pub write_policy_class: String,
    /// Whether whole-file participant writes are blocked.
    pub whole_file_participants_blocked: bool,
    /// Whether reviewed range-only writes may be admitted.
    pub range_only_reviewed_writes: bool,
    /// Stable override-action id.
    pub override_action_id: String,
    /// Whether the override / escalation route carries a disclosure.
    pub override_disclosed: bool,
    /// Whether the canonical write target resolved.
    pub canonical_target_resolved: bool,
    /// Overall restricted-write verdict.
    pub restricted_write_proven: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe large-file posture record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargeFilePostureRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub large_file_posture_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Workspace ref the document belongs to.
    pub workspace_ref: String,
    /// Logical document ref.
    pub document_ref: String,
    /// Source limited-mode file record id.
    pub limited_mode_file_ref: String,
    /// Canonical write target as resolved by VFS.
    pub canonical_uri: String,
    /// Bytes observed on disk at open.
    pub bytes_on_disk: u64,
    /// Why large-file mode is active.
    pub activation: LargeFileActivationSummary,
    /// Binary-safe preview / source-fidelity posture.
    pub preview_fidelity: PreviewFidelitySummary,
    /// Restricted-write posture.
    pub write_posture: RestrictedWritePosture,
    /// The capability rows the projection evaluated, in evaluation order.
    pub evaluated_capabilities: Vec<LimitedModeCapabilityRecord>,
    /// Total count of reduced or denied capabilities in limited mode.
    pub reduced_capability_count: usize,
    /// Pre-destructive inspection / repair hooks.
    pub inspection_hooks: Vec<InspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: LargeFilePostureQualification,
    /// Whether support export may include this record without raw bytes.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl LargeFilePostureRecord {
    /// Returns true when the record is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == LARGE_FILE_POSTURE_SCHEMA_REF
            && self.record_kind == LARGE_FILE_POSTURE_RECORD_KIND
            && !self.evaluated_capabilities.is_empty()
    }

    /// Returns true when the record proves the contract on the claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(&self, class: InspectionHookClass) -> Option<&InspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed large-file posture record from the two live truth
/// sources: the at-open classification observation and the limited-mode file
/// record.
///
/// The projection is deterministic and read-only. It never re-runs a
/// participant, mutates a buffer, opens a file, or widens authority; it pins the
/// source-fidelity, canonical-path, restricted-write, and inspection contracts
/// and auto-narrows below Stable with a named reason when a claim cannot be
/// proven on the captured posture.
pub fn project_large_file_posture(
    posture_id: impl Into<String>,
    classification: &LargeFileClassificationObservation,
    limited_mode: &LimitedModeFileRecord,
) -> LargeFilePostureRecord {
    project_large_file_posture_with_hooks(
        posture_id,
        classification,
        limited_mode,
        default_large_file_inspection_hooks(),
    )
}

/// Projects a large-file posture record with an explicit inspection-hook set.
///
/// This is the same projection as [`project_large_file_posture`] but takes the
/// inspection / repair hooks explicitly so fixtures and replay can model a
/// posture whose compare or checkpoint path is unavailable.
pub fn project_large_file_posture_with_hooks(
    posture_id: impl Into<String>,
    classification: &LargeFileClassificationObservation,
    limited_mode: &LimitedModeFileRecord,
    inspection_hooks: Vec<InspectionHook>,
) -> LargeFilePostureRecord {
    let activation = project_activation(classification);
    let preview_fidelity = project_preview_fidelity(classification, limited_mode);
    let write_posture = project_write_posture(limited_mode);

    let evaluated_capabilities: Vec<LimitedModeCapabilityRecord> = EVALUATED_CAPABILITY_IDS
        .iter()
        .filter_map(|id| limited_mode.capability(id).cloned())
        .collect();
    let reduced_capability_count = limited_mode
        .capabilities
        .iter()
        .filter(|cap| cap.state != LimitedModeCapabilityState::Allowed)
        .count();

    // A destructive action (escalation to the normal buffer, or a constrained
    // write) is always reachable in limited mode, so the compare and checkpoint
    // inspection paths must both be available before it.
    let compare_available = hook_available(&inspection_hooks, InspectionHookClass::Compare);
    let checkpoint_available = hook_available(&inspection_hooks, InspectionHookClass::Checkpoint);
    let pre_destructive_inspection_available = compare_available && checkpoint_available;

    let export_safe = limited_mode.is_support_export_safe() && !evaluated_capabilities.is_empty();

    // Evaluate narrow reasons in a fixed order so the record is deterministic.
    let mut narrow_reasons = Vec::new();
    if !preview_fidelity.byte_faithful_read {
        narrow_reasons.push(LargeFilePostureNarrowReason::SourceReadNotByteFaithful);
    }
    if !preview_fidelity.binary_safe_preview_selected {
        narrow_reasons.push(LargeFilePostureNarrowReason::PreviewNotBinarySafe);
    }
    if !write_posture.canonical_target_resolved {
        narrow_reasons.push(LargeFilePostureNarrowReason::CanonicalTargetUnresolved);
    }
    if !write_posture.whole_file_participants_blocked {
        narrow_reasons.push(LargeFilePostureNarrowReason::WholeFileWriteNotRestricted);
    }
    if !write_posture.override_disclosed {
        narrow_reasons.push(LargeFilePostureNarrowReason::OverrideRouteNotDisclosed);
    }
    if !pre_destructive_inspection_available {
        narrow_reasons.push(LargeFilePostureNarrowReason::DestructiveActionNoCheckpoint);
    }
    if !export_safe {
        narrow_reasons.push(LargeFilePostureNarrowReason::PostureExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = LargeFilePostureQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(&activation, &stable_qualification, reduced_capability_count);

    LargeFilePostureRecord {
        record_kind: LARGE_FILE_POSTURE_RECORD_KIND.to_owned(),
        large_file_posture_schema_version: LARGE_FILE_POSTURE_SCHEMA_VERSION,
        schema_ref: LARGE_FILE_POSTURE_SCHEMA_REF.to_owned(),
        posture_id: posture_id.into(),
        workspace_ref: limited_mode.workspace_ref.clone(),
        document_ref: limited_mode.document_ref.clone(),
        limited_mode_file_ref: limited_mode.limited_mode_file_id.clone(),
        canonical_uri: limited_mode.canonical_uri.clone(),
        bytes_on_disk: limited_mode.bytes_on_disk,
        activation,
        preview_fidelity,
        write_posture,
        evaluated_capabilities,
        reduced_capability_count,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

fn project_activation(
    classification: &LargeFileClassificationObservation,
) -> LargeFileActivationSummary {
    LargeFileActivationSummary {
        mode: classification.mode.clone(),
        trigger_class: classification.trigger.clone(),
        reason: classification.reason.clone(),
        looks_binary: classification.looks_binary,
        looks_minified: classification.looks_minified,
        matches_pack_suffix: classification.matches_pack_suffix,
        bom_kind: classification.bom_kind.clone(),
        has_null_bytes: classification.has_null_bytes,
        sniff_bytes: classification.sniff_bytes,
        non_printable_per_mille: classification.non_printable_per_mille,
    }
}

fn project_preview_fidelity(
    classification: &LargeFileClassificationObservation,
    limited_mode: &LimitedModeFileRecord,
) -> PreviewFidelitySummary {
    let safe_preview_class = limited_mode.safe_preview_class.as_str().to_owned();
    // The constrained read path is byte-faithful when whole-file load into the
    // normal buffer is blocked: the paged reader reads raw bytes in bounded
    // windows and never decodes-then-reencodes the whole file.
    let whole_file_load_blocked =
        capability_denied(limited_mode, CAP_WHOLE_FILE_LOAD).unwrap_or(false);
    let byte_faithful_read = whole_file_load_blocked;
    // Binary-like content must be given a binary-safe preview; non-binary
    // content is served the paged raw-text preview.
    let binary_safe_preview_selected = !classification.looks_binary
        || limited_mode.safe_preview_class
            == crate::large_file_mode::LimitedModeSafePreviewClass::BinarySafePreview;
    // A raw byte read preserves any BOM; the constrained viewer never strips it.
    let bom_preserved = byte_faithful_read;
    let source_fidelity_proven =
        byte_faithful_read && binary_safe_preview_selected && bom_preserved;

    PreviewFidelitySummary {
        safe_preview_class,
        byte_faithful_read,
        whole_file_load_blocked,
        binary_safe_preview_selected,
        bom_preserved,
        source_fidelity_proven,
    }
}

fn project_write_posture(limited_mode: &LimitedModeFileRecord) -> RestrictedWritePosture {
    let edit_policy_class = limited_mode.edit_policy_class.as_str().to_owned();
    let write_policy_class = limited_mode.write_policy_class.as_str().to_owned();
    // Whole-file participant writes are blocked when every whole-file write
    // capability is denied.
    let whole_file_participants_blocked = WHOLE_FILE_WRITE_CAPABILITIES
        .iter()
        .all(|id| capability_denied(limited_mode, id).unwrap_or(false));
    let range_only_reviewed_writes = limited_mode
        .capability(CAP_SAVE_PARTICIPANT_RANGE_ONLY)
        .map(|cap| cap.state != LimitedModeCapabilityState::Denied)
        .unwrap_or(false);
    let override_disclosed = !limited_mode.override_action.disclosure.trim().is_empty();
    let canonical_target_resolved = canonical_target_resolved(&limited_mode.canonical_uri);
    let restricted_write_proven =
        whole_file_participants_blocked && override_disclosed && canonical_target_resolved;

    RestrictedWritePosture {
        edit_policy_class,
        write_policy_class,
        whole_file_participants_blocked,
        range_only_reviewed_writes,
        override_action_id: limited_mode.override_action.action_id.clone(),
        override_disclosed,
        canonical_target_resolved,
        restricted_write_proven,
    }
}

// ---------------------------------------------------------------------------
// Derivations.
// ---------------------------------------------------------------------------

/// Returns `Some(true)` when the named capability is denied, `Some(false)` when
/// it is present but not denied, and `None` when the capability is absent.
fn capability_denied(record: &LimitedModeFileRecord, capability_id: &str) -> Option<bool> {
    record
        .capability(capability_id)
        .map(|cap| cap.state == LimitedModeCapabilityState::Denied)
}

/// Returns true when the canonical write target resolved to a usable URI.
fn canonical_target_resolved(canonical_uri: &str) -> bool {
    let trimmed = canonical_uri.trim();
    !trimmed.is_empty() && trimmed != "unknown"
}

fn hook_available(hooks: &[InspectionHook], class: InspectionHookClass) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn file_mode_token(mode: FileMode) -> String {
    mode.as_str().to_owned()
}

fn large_file_trigger_token(trigger: LargeFileTrigger) -> String {
    trigger.as_str().to_owned()
}

fn bom_kind_token(bom: BomKind) -> String {
    bom.as_str().to_owned()
}

fn build_summary(
    activation: &LargeFileActivationSummary,
    qualification: &LargeFilePostureQualification,
    reduced_capability_count: usize,
) -> String {
    let trigger = activation.trigger_class.as_deref().unwrap_or("none");
    if qualification.qualified {
        format!(
            "Large-file posture proven Stable: mode {mode}, trigger {trigger}, \
             {reduced} reduced capability(ies), binary-safe preview and restricted writes enforced.",
            mode = activation.mode,
            reduced = reduced_capability_count,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Large-file posture narrowed below Stable (mode {mode}, trigger {trigger}): {reasons}.",
            mode = activation.mode,
            reasons = reasons.join(", "),
        )
    }
}

/// Renders the export-safe human-readable lines for a large-file posture record.
///
/// This is the shared projection consumed by the editor large-file status
/// surface, the headless CLI emitter, Help/About, and support export, so they
/// never clone status text from each other.
pub fn large_file_posture_lines(record: &LargeFilePostureRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Large-file posture — {} ({})",
        record.activation.mode, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} document={} limited_mode_file={}",
        record.workspace_ref, record.document_ref, record.limited_mode_file_ref
    ));
    lines.push(format!(
        "canonical_uri={} bytes_on_disk={}",
        record.canonical_uri, record.bytes_on_disk
    ));
    lines.push(format!(
        "trigger={} reason={}",
        record.activation.trigger_class.as_deref().unwrap_or("none"),
        record.activation.reason,
    ));
    lines.push(format!(
        "looks_binary={} looks_minified={} pack_suffix={} bom={} null_bytes={}",
        record.activation.looks_binary,
        record.activation.looks_minified,
        record.activation.matches_pack_suffix,
        record.activation.bom_kind.as_deref().unwrap_or("none"),
        record.activation.has_null_bytes,
    ));
    lines.push(format!(
        "preview={} byte_faithful_read={} whole_file_load_blocked={} binary_safe={} bom_preserved={} source_fidelity_proven={}",
        record.preview_fidelity.safe_preview_class,
        record.preview_fidelity.byte_faithful_read,
        record.preview_fidelity.whole_file_load_blocked,
        record.preview_fidelity.binary_safe_preview_selected,
        record.preview_fidelity.bom_preserved,
        record.preview_fidelity.source_fidelity_proven,
    ));
    lines.push(format!(
        "edit_policy={} write_policy={} whole_file_blocked={} range_only={} canonical_target_resolved={} override={} override_disclosed={} restricted_write_proven={}",
        record.write_posture.edit_policy_class,
        record.write_posture.write_policy_class,
        record.write_posture.whole_file_participants_blocked,
        record.write_posture.range_only_reviewed_writes,
        record.write_posture.canonical_target_resolved,
        record.write_posture.override_action_id,
        record.write_posture.override_disclosed,
        record.write_posture.restricted_write_proven,
    ));

    lines.push("Evaluated capabilities:".to_owned());
    for cap in &record.evaluated_capabilities {
        lines.push(format!(
            "  {id} = {state} — {disclosure}",
            id = cap.capability_id,
            state = cap.state.as_str(),
            disclosure = cap.disclosure,
        ));
    }

    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }

    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }

    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
