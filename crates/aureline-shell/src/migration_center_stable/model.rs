//! Canonical stable truth model for the migration-center diff, rollback, and
//! unsupported-gap taxonomy of an imported-user flow.
//!
//! ## Why one disclosure record per imported-user flow
//!
//! A switching user who imports a profile asks three questions before they trust
//! the result: *what exactly changed (the diff), can I undo it (the rollback),
//! and what did not come across (the unsupported-gap taxonomy)?* When the
//! migration center, the settings import history, the command palette, the
//! support export, Help/About, and docs each answer those questions with their
//! own bespoke status text, they drift — a row implies the import was lossless
//! while the taxonomy already shows unsupported gaps, a flow implies rollback is
//! available when no pre-apply checkpoint was verified for it, or an
//! unsupported gap is hidden until after apply.
//!
//! This module mints one governed [`MigrationFlowDisclosureRecord`] per imported
//! source ecosystem. The record binds, for a single canonical migration
//! identity:
//!
//! - **The diff** — a before/after review that is shown *before* apply, with
//!   every row carrying both sides and citing one rollback requirement.
//! - **The rollback** — a preview requirement plus optional checkpoint evidence.
//!   Undo and compare routes exist only when the execution layer supplies a real
//!   checkpoint and restore record for *this* flow.
//! - **The unsupported-gap taxonomy** — the canonical Exact / Translated /
//!   Partial / Shimmed / Unsupported counts, and the union of Unsupported and
//!   Shimmed gaps made visible before apply rather than discovered as missing
//!   behaviour afterwards.
//!
//! The taxonomy, domains, and source-ecosystem vocabulary are **not** reinvented
//! here: they are the canonical [`crate::import::diff_review`] and
//! [`crate::migration_corpus`] types, so there is no parallel model.
//!
//! ## The honesty invariants
//!
//! The builder refuses to mint a record that would lie. Each is a [`BuildError`],
//! not a warning, so a dishonest projection fails the row instead of shipping:
//!
//! - **No claim the product cannot prove.** A claim ceiling may not assert the
//!   diff was reviewed before apply unless it was, rollback availability unless a
//!   pre-apply checkpoint is verified for *this* flow with undo and compare, the
//!   absence of unsupported gaps unless the taxonomy has none, or full-fidelity
//!   import unless no Partial/Shimmed/Unsupported rows exist.
//! - **Automatic narrowing below Stable.** A flow missing any pillar of evidence
//!   (diff reviewed before apply, a live verified rollback, gaps visible before
//!   apply, a complete taxonomy) is narrowed below Stable with a named reason
//!   rather than inheriting an adjacent green row.
//! - **Gaps are never hidden.** Every gap in the taxonomy is visible before
//!   apply, and the record keeps a Review-gaps recovery route.
//! - **Recovery before trust.** Every flow exposes Reopen-report and
//!   Export-support routes; a flow with a live rollback also exposes Undo and
//!   Compare; a flow with gaps also exposes Review-gaps.
//! - **One model across surfaces.** The migration center, settings import
//!   history, and command-palette projections share identity and recovery
//!   behaviour, and the reopen surfaces stay settings / help / support-export.
//! - **Same routes everywhere.** The same flow is reachable from the migration
//!   center, settings import history, command palette, and a menu command, each
//!   keyboard reachable and pointing at the same flow.
//! - **Accessible in every layout.** Tab order, row narration (which discloses
//!   the source ecosystem), action labels, and recovery affordances are present
//!   and reachable in normal, high-contrast, and zoomed layouts.
//! - **No detour behind account or managed services.** Every row stays available
//!   without an account and without managed services.
//!
//! The record is the canonical truth source for this lane (suggested-output stem
//! `finish-the-migration-center-diff-rollback-and-unsupported`); its boundary
//! schema is
//! `schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json`
//! and its contract narrative is
//! `docs/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`.

use serde::{Deserialize, Serialize};

use crate::import::diff_review::{ImportMappingClassification, ImportReviewDomain};
use crate::migration_corpus::{IncumbentEcosystem, MIGRATION_SCOREBOARD_ID};

/// Stable record-kind tag carried in serialized disclosure records.
pub const MIGRATION_FLOW_DISCLOSURE_RECORD_KIND: &str = "migration_flow_disclosure_record";

/// Schema version for the [`MigrationFlowDisclosureRecord`] payload shape.
pub const MIGRATION_FLOW_DISCLOSURE_SCHEMA_VERSION: u32 = 2;

/// Shared contract ref consumed by every surface that ingests this record.
pub const MIGRATION_FLOW_DISCLOSURE_SHARED_CONTRACT_REF: &str =
    "shell:migration_flow_disclosure_stable:v2";

/// Reviewer-facing notice rendered on every disclosure surface.
pub const MIGRATION_FLOW_DISCLOSURE_NOTICE: &str =
    "Migration disclosure truth: the migration center, settings import history, command palette, \
     support exports, Help/About, and docs show the same before/after diff (reviewed before \
     apply), the same rollback posture, and the same Exact/Translated/Partial/Shimmed/Unsupported \
     taxonomy with every unsupported gap visible before apply; no row claims the diff was \
     reviewed, rollback is available, there are no unsupported gaps, or the import was \
     full-fidelity unless the product can prove it; a flow missing any pillar of evidence is \
     narrowed below Stable with a named reason rather than inheriting an adjacent green row; the \
     same flow opens from every surface, keyboard-first; and every row stays available without an \
     account or managed services.";

/// Canonical durable-object URI scheme. Every minted ref must be one of these.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a canonical object ref.
const MAX_REF_CHARS: usize = 200;
/// Upper bound on support/evidence lists carried by one flow or gap.
const MAX_SUPPORT_REFS: usize = 64;
/// Upper bound on classified rows or gap disclosures carried by one flow.
const MAX_FLOW_ROWS: u32 = 4096;
/// Upper bound on gap rows retained in one support-safe disclosure.
const MAX_GAPS: usize = 64;

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one is rejected so chrome cannot
/// wire an affordance to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] = &[
    "home",
    "dashboard",
    "landing",
    "index",
    "overview",
    "start",
    "root",
];

/// Returns true when `reference` is a canonical durable-object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let trimmed = reference.trim();
    if trimmed != reference {
        return false;
    }
    let reference = trimmed;
    if reference.is_empty()
        || reference.len() > MAX_REF_CHARS
        || reference.chars().any(char::is_control)
        || reference.chars().any(char::is_whitespace)
        || reference.contains('\\')
    {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty()
        || ident.is_empty()
        || !class
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        || ident.split('/').any(|segment| {
            segment.is_empty()
                || matches!(segment, "." | "..")
                || !segment.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-')
                })
        })
    {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    text == trimmed
        && !trimmed.is_empty()
        && trimmed.len() <= MAX_SENTENCE_CHARS
        && !trimmed.chars().any(char::is_control)
        && !trimmed.starts_with('/')
        && !trimmed.starts_with('\\')
        && !trimmed.starts_with('~')
        && !trimmed.contains("://")
        && !contains_file_scheme(trimmed)
        && !trimmed.contains("../")
        && !trimmed.contains("..\\")
        && !trimmed.as_bytes().get(1).is_some_and(|byte| *byte == b':')
        && !contains_absolute_path(trimmed)
}

fn contains_absolute_path(value: &str) -> bool {
    value.split_whitespace().any(looks_like_absolute_path)
        || value
            .split(|character: char| {
                matches!(
                    character,
                    '=' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | ':' | '`' | '"' | '\''
                )
            })
            .any(looks_like_absolute_path)
}

fn contains_file_scheme(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.match_indices("file:").any(|(index, _)| {
        index == 0
            || lower[..index].chars().next_back().is_some_and(|character| {
                character.is_whitespace()
                    || matches!(
                        character,
                        '/' | '\\' | '=' | '(' | '[' | '{' | ',' | ';' | ':' | '`' | '"' | '\''
                    )
            })
    })
}

fn looks_like_absolute_path(token: &str) -> bool {
    let token = token.trim_matches(|character: char| {
        matches!(
            character,
            ',' | ';' | '(' | ')' | '[' | ']' | '`' | '"' | '\''
        )
    });
    (token.starts_with('/') && token.len() > 1)
        || (token.starts_with('\\') && token.len() > 1)
        || (token.starts_with('~') && token.len() > 1)
        || (token.as_bytes().get(1) == Some(&b':')
            && token
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphabetic)
            && token
                .as_bytes()
                .get(2)
                .is_some_and(|byte| matches!(*byte, b'/' | b'\\')))
}

fn is_safe_support_ref(reference: &str) -> bool {
    reference == reference.trim()
        && !reference.is_empty()
        && reference.len() <= 320
        && !reference.chars().any(char::is_control)
        && !reference.chars().any(char::is_whitespace)
        && reference.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'.' | b'_' | b'-' | b'/' | b'#')
        })
        && !reference.starts_with('/')
        && !reference.starts_with('\\')
        && !reference.starts_with('~')
        && !contains_file_scheme(reference)
        && !reference.contains("../")
        && !reference.contains("..\\")
        && !reference.contains("://")
        && !reference.contains("//")
        && !reference
            .as_bytes()
            .get(1)
            .is_some_and(|byte| *byte == b':')
}

fn has_duplicate_strings(values: &[String]) -> bool {
    values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].contains(value))
}

fn is_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() < 20
        || bytes.len() > 32
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes.last() != Some(&b'Z')
        || !(bytes.len() == 20 && bytes[19] == b'Z'
            || bytes.len() > 21
                && bytes[19] == b'.'
                && bytes[20..bytes.len() - 1].iter().all(u8::is_ascii_digit))
    {
        return false;
    }
    let Some(year) = timestamp_number(bytes, 0, 4) else {
        return false;
    };
    let Some(month) = timestamp_number(bytes, 5, 7) else {
        return false;
    };
    let Some(day) = timestamp_number(bytes, 8, 10) else {
        return false;
    };
    let Some(hour) = timestamp_number(bytes, 11, 13) else {
        return false;
    };
    let Some(minute) = timestamp_number(bytes, 14, 16) else {
        return false;
    };
    let Some(second) = timestamp_number(bytes, 17, 19) else {
        return false;
    };
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    year >= 1970 && day >= 1 && day <= days_in_month && hour <= 23 && minute <= 59 && second <= 59
}

fn timestamp_number(bytes: &[u8], start: usize, end: usize) -> Option<u32> {
    bytes
        .get(start..end)?
        .iter()
        .try_fold(0_u32, |value, byte| {
            byte.is_ascii_digit()
                .then_some(value * 10 + u32::from(*byte - b'0'))
        })
}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: "[redacted invalid ref]".to_owned(),
        })
    }
}

fn require_opaque_ref(
    field: &'static str,
    value: &str,
    required_prefix: &'static str,
) -> Result<(), BuildError> {
    let trimmed = value.trim();
    if trimmed != value {
        return Err(BuildError::InvalidOpaqueRef {
            field,
            value: "[redacted invalid ref]".to_owned(),
            required_prefix,
        });
    }
    let value = trimmed;
    let has_opaque_id = is_bounded_opaque_ref(value, required_prefix);
    if value.len() <= MAX_REF_CHARS && has_opaque_id {
        Ok(())
    } else {
        Err(BuildError::InvalidOpaqueRef {
            field,
            value: "[redacted invalid ref]".to_owned(),
            required_prefix,
        })
    }
}

fn is_bounded_opaque_ref(value: &str, required_prefix: &str) -> bool {
    value.len() <= MAX_REF_CHARS
        && value
            .strip_prefix(required_prefix)
            .is_some_and(|identifier| {
                !identifier.is_empty()
                    && identifier.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-')
                    })
            })
}

/// Public claim class for the lane, reusing the stable lifecycle cutline.
///
/// `Stable` sits at or above the launch cutline; everything else is narrowed
/// below it. The builder *derives* this from the evidence, so a flow can never
/// publish a claim wider than its proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The diff/rollback/gap-taxonomy disclosure is replacement-grade.
    Stable,
    /// Narrowed to the beta promise.
    Beta,
    /// Narrowed to the preview/limited-availability promise.
    Preview,
    /// No public promise yet.
    NotClaimed,
}

impl StableClaimClass {
    /// Returns the stable string vocabulary for this claim class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::NotClaimed => "not_claimed",
        }
    }

    /// Returns `true` when the claim sits at or above the launch cutline.
    pub const fn at_or_above_cutline(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Closed reason a flow is narrowed below Stable. Required whenever the claim
/// class is below the cutline; forbidden when it is Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableNarrowingReason {
    /// The before/after diff was not proven to be shown before apply.
    DiffNotReviewedBeforeApply,
    /// No pre-apply rollback checkpoint is verified for this flow, or undo /
    /// compare routes are unavailable.
    RollbackEvidenceIncomplete,
    /// At least one unsupported gap is not visible before apply.
    UnsupportedGapsHiddenBeforeApply,
    /// The taxonomy is incomplete: a row is unclassified, or no classification
    /// is present.
    TaxonomyIncomplete,
}

impl StableNarrowingReason {
    /// Returns the stable string vocabulary for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffNotReviewedBeforeApply => "diff_not_reviewed_before_apply",
            Self::RollbackEvidenceIncomplete => "rollback_evidence_incomplete",
            Self::UnsupportedGapsHiddenBeforeApply => "unsupported_gaps_hidden_before_apply",
            Self::TaxonomyIncomplete => "taxonomy_incomplete",
        }
    }
}

/// Surface a flow can be reached from. The same flow must be reachable from all
/// four so the migration center and in-product import surfaces stay consistent
/// for keyboard-only and assistive-technology users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationRouteSurface {
    /// The migration center page.
    MigrationCenter,
    /// The settings import-history list.
    SettingsImportHistory,
    /// The command palette.
    CommandPalette,
    /// An application menu command.
    MenuCommand,
}

impl MigrationRouteSurface {
    /// Returns the stable string vocabulary for this route surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigrationCenter => "migration_center",
            Self::SettingsImportHistory => "settings_import_history",
            Self::CommandPalette => "command_palette",
            Self::MenuCommand => "menu_command",
        }
    }

    /// The four surfaces that must all be able to reach a flow.
    pub const REQUIRED: [Self; 4] = [
        Self::MigrationCenter,
        Self::SettingsImportHistory,
        Self::CommandPalette,
        Self::MenuCommand,
    ];
}

/// Layout mode an accessibility disclosure is checked under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutMode {
    /// Default desktop layout.
    Normal,
    /// High-contrast theme.
    HighContrast,
    /// Zoomed / enlarged layout.
    Zoomed,
}

impl LayoutMode {
    /// Returns the stable string vocabulary for this layout mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::HighContrast => "high_contrast",
            Self::Zoomed => "zoomed",
        }
    }

    /// The three layout modes every disclosure must hold in.
    pub const REQUIRED: [Self; 3] = [Self::Normal, Self::HighContrast, Self::Zoomed];
}

/// Role a recovery action plays, used for placement and confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionRole {
    /// Opens or reopens the canonical migration artifact.
    Primary,
    /// Repairs, restores, or compares the imported state.
    Recovery,
    /// Non-destructive review or export.
    Secondary,
}

impl RecoveryActionRole {
    /// Returns the stable string vocabulary for this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Recovery => "recovery",
            Self::Secondary => "secondary",
        }
    }
}

/// Closed recovery-action vocabulary exposed on a migration flow row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationRecoveryAction {
    /// Reopen the retained migration mapping report.
    ReopenMigrationReport,
    /// Compare the captured before and after state.
    CompareBeforeAfter,
    /// Undo the apply via the pre-apply rollback checkpoint.
    UndoViaRollback,
    /// Review the unsupported / bridge gaps surfaced before apply.
    ReviewUnsupportedGaps,
    /// Export a redacted support packet for the migration.
    ExportSupportPacket,
}

impl MigrationRecoveryAction {
    /// Returns the stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopenMigrationReport => "reopen_migration_report",
            Self::CompareBeforeAfter => "compare_before_after",
            Self::UndoViaRollback => "undo_via_rollback",
            Self::ReviewUnsupportedGaps => "review_unsupported_gaps",
            Self::ExportSupportPacket => "export_support_packet",
        }
    }

    /// Returns the reviewer-facing action label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::ReopenMigrationReport => "Reopen migration report",
            Self::CompareBeforeAfter => "Compare before and after",
            Self::UndoViaRollback => "Undo via rollback checkpoint",
            Self::ReviewUnsupportedGaps => "Review unsupported gaps",
            Self::ExportSupportPacket => "Export support packet",
        }
    }

    /// Returns the placement / confirmation role for this action.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::ReopenMigrationReport => RecoveryActionRole::Primary,
            Self::CompareBeforeAfter | Self::UndoViaRollback => RecoveryActionRole::Recovery,
            Self::ReviewUnsupportedGaps | Self::ExportSupportPacket => {
                RecoveryActionRole::Secondary
            }
        }
    }

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }
}

/// Returns the recovery actions a flow must expose, in rendered order, given
/// whether a live rollback exists and whether the taxonomy carries gaps.
pub fn required_recovery_actions(
    live_rollback: bool,
    has_gaps: bool,
) -> Vec<MigrationRecoveryAction> {
    let mut actions = vec![MigrationRecoveryAction::ReopenMigrationReport];
    if live_rollback {
        actions.push(MigrationRecoveryAction::CompareBeforeAfter);
        actions.push(MigrationRecoveryAction::UndoViaRollback);
    }
    if has_gaps {
        actions.push(MigrationRecoveryAction::ReviewUnsupportedGaps);
    }
    actions.push(MigrationRecoveryAction::ExportSupportPacket);
    actions
}

fn privacy_safe_recovery_action_id(action_id: &str) -> String {
    [
        MigrationRecoveryAction::ReopenMigrationReport,
        MigrationRecoveryAction::CompareBeforeAfter,
        MigrationRecoveryAction::UndoViaRollback,
        MigrationRecoveryAction::ReviewUnsupportedGaps,
        MigrationRecoveryAction::ExportSupportPacket,
    ]
    .into_iter()
    .find(|action| action.as_str() == action_id)
    .map(|action| action.as_str().to_owned())
    .unwrap_or_else(|| "[redacted invalid action id]".to_owned())
}

/// The before/after diff disclosure for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffDisclosure {
    /// Canonical import-diff-preview ref.
    pub diff_preview_ref: String,
    /// Whether the diff is shown before apply.
    pub reviewed_before_apply: bool,
    /// Number of before/after rows in the diff.
    pub row_count: u32,
    /// Whether every row carries both a before and an after side.
    pub every_row_has_before_after: bool,
    /// Whether every row cites the one shared rollback requirement.
    pub every_row_uses_one_requirement: bool,
}

impl DiffDisclosure {
    /// Returns `true` when the diff is a reviewable before/after surface.
    pub fn is_reviewable_before_apply(&self) -> bool {
        self.reviewed_before_apply
            && self.row_count > 0
            && self.every_row_has_before_after
            && self.every_row_uses_one_requirement
    }
}

/// The rollback disclosure for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDisclosure {
    /// Preview requirement ref. This is never a checkpoint handle.
    pub rollback_requirement_ref: String,
    /// Canonical rollback-checkpoint ref, present only with execution evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Canonical migration-restore-record ref, present only with execution evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_record_ref: Option<String>,
    /// Whether the checkpoint was minted before apply.
    pub created_before_apply: bool,
    /// Whether the checkpoint protects every domain the apply may touch.
    pub protects_every_domain: bool,
    /// Whether a live pre-apply checkpoint is verified for *this* flow, rather
    /// than referenced from an adjacent flow's apply session.
    pub verified_for_this_flow: bool,
    /// Whether an undo route restores from the checkpoint.
    pub undo_available: bool,
    /// Canonical undo-action ref, present iff [`Self::undo_available`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_action_ref: Option<String>,
    /// Whether a compare route shows before vs after.
    pub compare_available: bool,
    /// Canonical compare-action ref, present iff [`Self::compare_available`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_action_ref: Option<String>,
}

impl RollbackDisclosure {
    /// Returns `true` when rollback is provably available for this flow.
    pub fn is_live_for_flow(&self) -> bool {
        self.checkpoint_ref.is_some()
            && self.restore_record_ref.is_some()
            && self.created_before_apply
            && self.protects_every_domain
            && self.verified_for_this_flow
            && self.undo_available
            && self.compare_available
    }
}

/// One Unsupported or Shimmed gap surfaced before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedGapDisclosure {
    /// Stable gap id.
    pub gap_id: String,
    /// Import domain the gap lives in.
    pub domain: ImportReviewDomain,
    /// Classification of the gap (`Unsupported` or `Shimmed`).
    pub classification: ImportMappingClassification,
    /// Redaction-aware source object label.
    pub source_label: String,
    /// Reviewer-facing description of the gap.
    pub gap_summary: String,
    /// Whether the gap is visible during preview, before apply.
    pub visible_before_apply: bool,
    /// Whether the gap remains visible in the retained report.
    pub retained_after_apply: bool,
    /// Docs/help refs that explain the gap (repo-relative source paths).
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that carry the gap into the export.
    pub support_export_refs: Vec<String>,
}

/// The Exact / Translated / Partial / Shimmed / Unsupported taxonomy for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GapTaxonomy {
    /// Number of `Exact` rows.
    pub exact: u32,
    /// Number of `Translated` rows.
    pub translated: u32,
    /// Number of `Partial` rows.
    pub partial: u32,
    /// Number of `Shimmed` rows.
    pub shimmed: u32,
    /// Number of `Unsupported` rows.
    pub unsupported: u32,
    /// Distinct classifications present, in canonical order.
    pub classifications_present: Vec<ImportMappingClassification>,
    /// Whether every Unsupported / Shimmed gap is visible before apply.
    pub unsupported_gaps_visible_before_apply: bool,
    /// The Unsupported / Shimmed gaps, sorted by gap id.
    pub gaps: Vec<UnsupportedGapDisclosure>,
}

impl GapTaxonomy {
    /// Returns the total number of classified rows.
    pub const fn total(&self) -> u32 {
        self.exact
            .saturating_add(self.translated)
            .saturating_add(self.partial)
            .saturating_add(self.shimmed)
            .saturating_add(self.unsupported)
    }

    /// Returns `true` when the import would be full-fidelity (no Partial,
    /// Shimmed, or Unsupported rows).
    pub const fn is_full_fidelity(&self) -> bool {
        self.partial == 0 && self.shimmed == 0 && self.unsupported == 0
    }

    /// Returns `true` when there are no Unsupported or Shimmed gaps.
    pub fn has_no_gaps(&self) -> bool {
        self.gaps.is_empty()
    }

    /// Returns `true` when the taxonomy is complete: at least one classification
    /// is present and every gap is visible before apply.
    pub fn is_complete(&self) -> bool {
        !self.classifications_present.is_empty()
            && self.total() > 0
            && self.unsupported_gaps_visible_before_apply
            && self.gaps.iter().all(|gap| gap.visible_before_apply)
    }
}

/// The public claim ceiling: what a flow row is allowed to assert. Each field
/// must be provable from the flow's real evidence; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MigrationClaimCeiling {
    /// Whether the row may claim the diff was reviewed before apply.
    pub asserts_diff_reviewed_before_apply: bool,
    /// Whether the row may claim rollback is available for this flow.
    pub asserts_rollback_available: bool,
    /// Whether the row may claim there are no unsupported gaps.
    pub asserts_no_unsupported_gaps: bool,
    /// Whether the row may claim the import was full-fidelity.
    pub asserts_full_fidelity_import: bool,
}

/// The derived stable-claim verdict for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableQualification {
    /// The derived claim class (Stable when fully qualified, else narrowed).
    pub claim_class: StableClaimClass,
    /// Whether the flow qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// The reasons the flow is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<StableNarrowingReason>,
}

/// One recovery route exposed on a flow row before the user commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRouteRecord {
    /// Stable action id from the canonical recovery vocabulary.
    pub action_id: String,
    /// Compact label rendered in rows and narrated by assistive tech.
    pub action_label: String,
    /// Placement / confirmation role.
    pub action_role: RecoveryActionRole,
    /// Whether the action is keyboard reachable.
    pub keyboard_reachable: bool,
}

/// One route to the same flow from one entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryRouteRecord {
    /// Surface that exposes the route.
    pub surface: MigrationRouteSurface,
    /// Canonical route ref pointing at the flow on this surface.
    pub route_ref: String,
    /// Whether the route is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether the route activates the same canonical flow identity.
    pub activates_same_flow: bool,
}

/// Accessibility disclosure for one layout mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutModeDisclosure {
    /// Layout mode this disclosure was checked under.
    pub mode: LayoutMode,
    /// Whether the row narration is available in this mode.
    pub row_narration_available: bool,
    /// Whether the recovery affordances stay reachable in this mode.
    pub recovery_affordances_reachable: bool,
}

/// Accessibility disclosure for one flow row across the required layout modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityDisclosure {
    /// Position of the row in the surface tab order.
    pub focus_order_index: u32,
    /// Number of keyboard tab stops the row and its actions expose.
    pub tab_stop_count: u32,
    /// Row narration read by assistive tech; discloses the source ecosystem.
    pub row_narration: String,
    /// Action labels in rendered order, narrated by assistive technology.
    pub action_labels: Vec<String>,
    /// Per-layout-mode disclosures for normal, high-contrast, and zoomed.
    pub layout_modes: Vec<LayoutModeDisclosure>,
}

/// Cross-surface parity between the migration center and settings projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParity {
    /// Migration center row id for this flow.
    pub migration_center_row_id: String,
    /// Settings import-history row id for this flow.
    pub settings_import_history_row_id: String,
    /// Command-palette command id that opens this flow.
    pub command_palette_command_id: String,
    /// Recovery action ids shared by both surfaces.
    pub recovery_action_ids: Vec<String>,
    /// Reopen surfaces (settings / help / support_export) the report retains.
    pub reopen_surfaces: Vec<String>,
    /// Whether the projections agree on identity and recovery behaviour.
    pub parity_holds: bool,
}

/// Header state rendered above migration-center and post-apply records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationFlowHeader {
    /// In-packet review correlation shared by preview surfaces. This is not an
    /// apply-session or lifecycle record.
    pub migration_review_ref: String,
    /// Source tool chip label.
    pub source_tool_label: String,
    /// Source version chip label. Must disclose unknown marker-only truth.
    pub source_version_label: String,
    /// Target profile/workspace ref where writes land.
    pub target_scope_ref: String,
    /// Reviewer-facing target scope label.
    pub target_scope_label: String,
    /// Short sentence describing where writes land.
    pub writes_land_in: String,
    /// Checkpoint-requirement notice text.
    pub checkpoint_requirement_notice: String,
    /// Preview requirement ref. This is never a checkpoint handle.
    pub rollback_requirement_ref: String,
    /// Canonical checkpoint ref, present only after execution publishes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Canonical restore record ref, present only after execution publishes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_record_ref: Option<String>,
    /// Canonical restore action ref, present only when restore is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_action_ref: Option<String>,
    /// Whether the restore action is executable for this record.
    pub restore_action_enabled: bool,
    /// Canonical compatibility report ref.
    pub compatibility_report_ref: String,
    /// Canonical compatibility-report open action ref.
    pub compatibility_report_action_ref: String,
    /// Canonical support export ref.
    pub support_export_ref: String,
    /// Canonical issue-template ref.
    pub issue_template_ref: String,
    /// Whether partial-apply context stays visible after review/apply.
    pub partial_apply_context_visible: bool,
    /// Whether downgrade/narrowing context stays visible after review/apply.
    pub downgrade_context_visible: bool,
    /// Whether restore context stays visible after review/apply.
    pub restore_context_visible: bool,
}

impl MigrationFlowHeader {
    /// Returns true when the header answers the required migration questions.
    pub fn answers_required_questions(&self) -> bool {
        let checkpoint_refs_are_canonical = self
            .checkpoint_ref
            .as_deref()
            .map(is_canonical_object_ref)
            .unwrap_or(true);
        let restore_record_is_canonical = self
            .restore_record_ref
            .as_deref()
            .map(is_canonical_object_ref)
            .unwrap_or(true);
        let restore_action_is_canonical = self
            .restore_action_ref
            .as_deref()
            .map(is_canonical_object_ref)
            .unwrap_or(true);
        let restore_evidence_consistent = if self.restore_action_enabled {
            self.checkpoint_ref.is_some()
                && self.restore_record_ref.is_some()
                && self.restore_action_ref.is_some()
        } else {
            self.checkpoint_ref.is_none()
                && self.restore_record_ref.is_none()
                && self.restore_action_ref.is_none()
        };

        is_reviewable_sentence(&self.source_tool_label)
            && is_reviewable_sentence(&self.source_version_label)
            && is_reviewable_sentence(&self.target_scope_label)
            && is_reviewable_sentence(&self.writes_land_in)
            && is_bounded_opaque_ref(&self.migration_review_ref, "migration-review:")
            && is_reviewable_sentence(&self.checkpoint_requirement_notice)
            && is_bounded_opaque_ref(&self.rollback_requirement_ref, "rollback-requirement:")
            && is_canonical_object_ref(&self.target_scope_ref)
            && checkpoint_refs_are_canonical
            && restore_record_is_canonical
            && restore_action_is_canonical
            && restore_evidence_consistent
            && is_canonical_object_ref(&self.compatibility_report_ref)
            && is_canonical_object_ref(&self.compatibility_report_action_ref)
            && is_canonical_object_ref(&self.support_export_ref)
            && is_canonical_object_ref(&self.issue_template_ref)
            && self.partial_apply_context_visible
            && self.downgrade_context_visible
            && self.restore_context_visible
    }
}

/// Upstream ids the record is a genuine projection of, kept for support
/// traceability. These are upstream source refs, not canonical durable objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRefs {
    /// Wizard review correlation the preview requirement came from.
    pub wizard_review_ref: String,
    /// Wizard mapping report id retained after first run.
    pub wizard_mapping_report_ref: String,
    /// Raw upstream rollback-requirement ref; never a checkpoint handle.
    pub rollback_requirement_ref: String,
    /// Raw upstream import-diff-preview ref.
    pub import_diff_preview_ref: String,
    /// Migration corpus scoreboard id the taxonomy came from.
    pub corpus_scoreboard_ref: String,
    /// Source-ecosystem corpus section ref.
    pub corpus_section_ref: String,
}

/// Validated input used to mint a [`MigrationFlowDisclosureRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationFlowDisclosureInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// In-packet migration review correlation; not an apply session.
    pub migration_review_ref: String,
    /// Source ecosystem this flow imported from.
    pub source_ecosystem: IncumbentEcosystem,
    /// Header state rendered before review and retained after apply.
    pub header: MigrationFlowHeader,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The before/after diff disclosure.
    pub diff: DiffDisclosure,
    /// The rollback disclosure.
    pub rollback: RollbackDisclosure,
    /// The Exact/Translated/Partial/Shimmed/Unsupported taxonomy.
    pub taxonomy: GapTaxonomy,
    /// Public claim ceiling for this flow.
    pub claim_ceiling: MigrationClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same flow.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the flow stays available without an account.
    pub available_without_account: bool,
    /// Whether the flow stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: UpstreamRefs,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed migration-flow disclosure record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationFlowDisclosureRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// In-packet migration review correlation; not an apply session.
    pub migration_review_ref: String,
    /// Source ecosystem this flow imported from.
    pub source_ecosystem: IncumbentEcosystem,
    /// Compact source-ecosystem label (the vocabulary docs / Help/About ingest).
    pub source_ecosystem_label: String,
    /// Header state rendered before review and retained after apply.
    pub header: MigrationFlowHeader,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The before/after diff disclosure.
    pub diff: DiffDisclosure,
    /// The rollback disclosure.
    pub rollback: RollbackDisclosure,
    /// The Exact/Translated/Partial/Shimmed/Unsupported taxonomy.
    pub taxonomy: GapTaxonomy,
    /// Public claim ceiling.
    pub claim_ceiling: MigrationClaimCeiling,
    /// The derived stable-claim verdict (Stable, or narrowed with reasons).
    pub stable_qualification: StableQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same flow.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the flow stays available without an account.
    pub available_without_account: bool,
    /// Whether the flow stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or gapped to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: UpstreamRefs,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`MigrationFlowDisclosureRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// An upstream opaque ref did not carry its required object-kind prefix.
    InvalidOpaqueRef {
        field: &'static str,
        value: String,
        required_prefix: &'static str,
    },
    /// The claim ceiling asserted the diff was reviewed when it was not.
    OverclaimsDiffReviewed,
    /// The claim ceiling asserted rollback availability it cannot prove.
    OverclaimsRollbackAvailable,
    /// The claim ceiling asserted there were no unsupported gaps when there are.
    OverclaimsNoUnsupportedGaps,
    /// The claim ceiling asserted a full-fidelity import it cannot prove.
    OverclaimsFullFidelity,
    /// A taxonomy gap count did not match the gap rows present.
    TaxonomyGapCountMismatch,
    /// Taxonomy classification presence or gap-row shape was not exact.
    TaxonomyShapeMismatch,
    /// An undo / compare ref was present without availability, or vice versa.
    RollbackRefAvailabilityMismatch { field: &'static str },
    /// Checkpoint lifecycle booleans or refs claimed evidence that was absent.
    RollbackEvidenceStateMismatch,
    /// Upstream review or rollback-requirement identity drifted from the
    /// disclosure it is claimed to project.
    UpstreamTruthDrift { field: &'static str },
    /// A gap was not visible before apply.
    GapHiddenBeforeApply { gap_id: String },
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: MigrationRecoveryAction },
    /// Recovery routes contained duplicates, unsupported actions, reordered
    /// actions, or labels/roles that drifted from the canonical vocabulary.
    RecoveryRoutesMismatch,
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// The two surface projections disagreed on identity or recovery behaviour.
    SurfaceParityBroken,
    /// A required reopen surface was missing.
    ReopenSurfaceMissing { surface: &'static str },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: MigrationRouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: MigrationRouteSurface },
    /// An entry route did not activate the same canonical flow.
    RouteTargetsDifferentFlow { surface: MigrationRouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: MigrationRouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// The row narration did not disclose the source ecosystem.
    NarrationOmitsEcosystem,
    /// A flow was hidden when no account was present.
    HiddenWithoutAccount,
    /// A flow was hidden when managed services were absent.
    HiddenWithoutManagedServices,
    /// Header state is incomplete or drifts from the record.
    HeaderIncomplete,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::InvalidOpaqueRef {
                field,
                value,
                required_prefix,
            } => write!(
                f,
                "field `{field}` must start with `{required_prefix}`, got {value:?}"
            ),
            Self::OverclaimsDiffReviewed => write!(
                f,
                "claim ceiling may not assert the diff was reviewed before apply unless it was"
            ),
            Self::OverclaimsRollbackAvailable => write!(
                f,
                "claim ceiling may not assert rollback availability without a verified pre-apply checkpoint, undo, and compare"
            ),
            Self::OverclaimsNoUnsupportedGaps => write!(
                f,
                "claim ceiling may not assert there are no unsupported gaps when the taxonomy has them"
            ),
            Self::OverclaimsFullFidelity => write!(
                f,
                "claim ceiling may not assert a full-fidelity import with Partial/Shimmed/Unsupported rows"
            ),
            Self::TaxonomyGapCountMismatch => write!(
                f,
                "taxonomy gap rows must match the Unsupported and Shimmed counts"
            ),
            Self::TaxonomyShapeMismatch => write!(
                f,
                "taxonomy classifications and gap rows must be exact, unique, and bounded"
            ),
            Self::RollbackRefAvailabilityMismatch { field } => write!(
                f,
                "rollback `{field}` must be present iff the matching route is available"
            ),
            Self::RollbackEvidenceStateMismatch => write!(
                f,
                "rollback lifecycle evidence must be absent in preview or complete and internally consistent"
            ),
            Self::UpstreamTruthDrift { field } => write!(
                f,
                "upstream `{field}` must match the disclosure truth it projects"
            ),
            Self::GapHiddenBeforeApply { gap_id } => {
                write!(f, "gap `{gap_id}` must be visible before apply")
            }
            Self::MissingRecoveryRoute { action } => write!(
                f,
                "flow must expose recovery route `{}`",
                action.as_str()
            ),
            Self::RecoveryRoutesMismatch => write!(
                f,
                "recovery routes must exactly match the canonical actions, order, labels, and roles"
            ),
            Self::RecoveryRouteNotKeyboardReachable { action_id } => write!(
                f,
                "recovery route `{action_id}` must be keyboard reachable"
            ),
            Self::SurfaceParityBroken => write!(
                f,
                "migration center and settings projections must share identity and recovery behaviour"
            ),
            Self::ReopenSurfaceMissing { surface } => {
                write!(f, "reopen surface `{surface}` is missing")
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentFlow { surface } => write!(
                f,
                "entry route surface `{}` must activate the same flow",
                surface.as_str()
            ),
            Self::DuplicateRouteSurface { surface } => {
                write!(f, "entry route surface `{}` is duplicated", surface.as_str())
            }
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(f, "accessibility layout mode `{}` is missing", mode.as_str())
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::NarrationOmitsEcosystem => {
                write!(f, "row narration must disclose the source ecosystem")
            }
            Self::HiddenWithoutAccount => {
                write!(f, "a migration flow row must stay available without an account")
            }
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a migration flow row must stay available without managed services"
            ),
            Self::HeaderIncomplete => write!(
                f,
                "migration header must preserve source/version, target scope, rollback requirement, honest optional restore evidence, compatibility, support, and issue-template refs for the same review"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl MigrationFlowDisclosureRecord {
    /// Builds a governed disclosure record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about the diff, the rollback, the unsupported-gap taxonomy, recovery,
    /// cross-surface parity, route reachability, or accessibility. The stable
    /// claim class is *derived* from the evidence, so a flow can never publish a
    /// claim wider than its proof.
    pub fn build(input: MigrationFlowDisclosureInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        if !is_safe_support_ref(&input.record_id) {
            return Err(BuildError::InvalidSentence { field: "record_id" });
        }
        if !is_utc_timestamp(&input.as_of) {
            return Err(BuildError::InvalidSentence { field: "as_of" });
        }
        require_opaque_ref(
            "migration_review_ref",
            &input.migration_review_ref,
            "migration-review:",
        )?;
        require_opaque_ref(
            "rollback.rollback_requirement_ref",
            &input.rollback.rollback_requirement_ref,
            "rollback-requirement:",
        )?;
        if input.header.migration_review_ref != input.migration_review_ref
            || input.header.rollback_requirement_ref != input.rollback.rollback_requirement_ref
            || input.header.checkpoint_ref != input.rollback.checkpoint_ref
            || input.header.restore_record_ref != input.rollback.restore_record_ref
            || input.header.support_export_ref != input.support_export_ref
            || input.header.restore_action_enabled != input.rollback.is_live_for_flow()
            || input.header.checkpoint_requirement_notice
                != format!(
                    "Rollback checkpoint required before apply: {}",
                    input.rollback.rollback_requirement_ref
                )
            || !input.header.answers_required_questions()
        {
            return Err(BuildError::HeaderIncomplete);
        }
        if input.upstream.wizard_review_ref != input.migration_review_ref {
            return Err(BuildError::UpstreamTruthDrift {
                field: "wizard_review_ref",
            });
        }
        if input.upstream.rollback_requirement_ref != input.rollback.rollback_requirement_ref {
            return Err(BuildError::UpstreamTruthDrift {
                field: "rollback_requirement_ref",
            });
        }
        let Some(review_suffix) = input.migration_review_ref.strip_prefix("migration-review:")
        else {
            return Err(BuildError::UpstreamTruthDrift {
                field: "migration_review_ref",
            });
        };
        if input.upstream.wizard_mapping_report_ref != format!("mapping-report:{review_suffix}")
            || input.upstream.import_diff_preview_ref != format!("import-preview:{review_suffix}")
            || input.upstream.corpus_scoreboard_ref != MIGRATION_SCOREBOARD_ID
            || input.upstream.corpus_section_ref
                != input.source_ecosystem.source_ecosystem_row_ref()
        {
            return Err(BuildError::UpstreamTruthDrift {
                field: "derived_object_refs",
            });
        }
        for (field, reference) in [
            (
                "wizard_mapping_report_ref",
                input.upstream.wizard_mapping_report_ref.as_str(),
            ),
            (
                "import_diff_preview_ref",
                input.upstream.import_diff_preview_ref.as_str(),
            ),
            (
                "corpus_scoreboard_ref",
                input.upstream.corpus_scoreboard_ref.as_str(),
            ),
            (
                "corpus_section_ref",
                input.upstream.corpus_section_ref.as_str(),
            ),
        ] {
            if !is_safe_support_ref(reference) {
                return Err(BuildError::UpstreamTruthDrift { field });
            }
        }
        require_ref("diff.diff_preview_ref", &input.diff.diff_preview_ref)?;
        if let Some(checkpoint_ref) = &input.rollback.checkpoint_ref {
            require_ref("rollback.checkpoint_ref", checkpoint_ref)?;
        }
        if let Some(restore_record_ref) = &input.rollback.restore_record_ref {
            require_ref("rollback.restore_record_ref", restore_record_ref)?;
        }
        require_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_ref("support_export_ref", &input.support_export_ref)?;
        if input.evidence_refs.len() > MAX_SUPPORT_REFS
            || has_duplicate_strings(&input.evidence_refs)
        {
            return Err(BuildError::InvalidSentence {
                field: "evidence_refs",
            });
        }
        if input.narrative_refs.len() > MAX_SUPPORT_REFS
            || has_duplicate_strings(&input.narrative_refs)
        {
            return Err(BuildError::InvalidSentence {
                field: "narrative_refs",
            });
        }
        for evidence in &input.evidence_refs {
            require_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_ref("narrative_refs", narrative)?;
        }

        let checkpoint_evidence_present = match (
            input.rollback.checkpoint_ref.is_some(),
            input.rollback.restore_record_ref.is_some(),
        ) {
            (true, true) => true,
            (false, false) => false,
            _ => return Err(BuildError::RollbackEvidenceStateMismatch),
        };
        let lifecycle_evidence_claimed = input.rollback.created_before_apply
            || input.rollback.protects_every_domain
            || input.rollback.verified_for_this_flow
            || input.rollback.undo_available
            || input.rollback.compare_available;
        if lifecycle_evidence_claimed && !checkpoint_evidence_present {
            return Err(BuildError::RollbackEvidenceStateMismatch);
        }

        // --- rollback ref / availability consistency -------------------------
        match (
            &input.rollback.undo_available,
            &input.rollback.undo_action_ref,
        ) {
            (true, Some(reference)) => require_ref("rollback.undo_action_ref", reference)?,
            (false, None) => {}
            _ => {
                return Err(BuildError::RollbackRefAvailabilityMismatch {
                    field: "undo_action_ref",
                })
            }
        }
        match (
            &input.rollback.compare_available,
            &input.rollback.compare_action_ref,
        ) {
            (true, Some(reference)) => require_ref("rollback.compare_action_ref", reference)?,
            (false, None) => {}
            _ => {
                return Err(BuildError::RollbackRefAvailabilityMismatch {
                    field: "compare_action_ref",
                })
            }
        }

        // --- taxonomy integrity ----------------------------------------------
        if input.taxonomy.gaps.len() > MAX_GAPS {
            return Err(BuildError::TaxonomyShapeMismatch);
        }
        let classified_total = [
            input.taxonomy.exact,
            input.taxonomy.translated,
            input.taxonomy.partial,
            input.taxonomy.shimmed,
            input.taxonomy.unsupported,
        ]
        .into_iter()
        .try_fold(0_u32, u32::checked_add);
        if !classified_total.is_some_and(|total| total <= MAX_FLOW_ROWS) {
            return Err(BuildError::TaxonomyShapeMismatch);
        }
        let gap_unsupported = input
            .taxonomy
            .gaps
            .iter()
            .filter(|gap| gap.classification == ImportMappingClassification::Unsupported)
            .count() as u32;
        let gap_shimmed = input
            .taxonomy
            .gaps
            .iter()
            .filter(|gap| gap.classification == ImportMappingClassification::Shimmed)
            .count() as u32;
        if gap_unsupported != input.taxonomy.unsupported || gap_shimmed != input.taxonomy.shimmed {
            return Err(BuildError::TaxonomyGapCountMismatch);
        }
        let expected_classifications: Vec<ImportMappingClassification> = [
            (ImportMappingClassification::Exact, input.taxonomy.exact),
            (
                ImportMappingClassification::Translated,
                input.taxonomy.translated,
            ),
            (ImportMappingClassification::Partial, input.taxonomy.partial),
            (ImportMappingClassification::Shimmed, input.taxonomy.shimmed),
            (
                ImportMappingClassification::Unsupported,
                input.taxonomy.unsupported,
            ),
        ]
        .into_iter()
        .filter_map(|(classification, count)| (count > 0).then_some(classification))
        .collect();
        if input.taxonomy.classifications_present != expected_classifications
            || !input.taxonomy.unsupported_gaps_visible_before_apply
            || input
                .taxonomy
                .gaps
                .windows(2)
                .any(|pair| pair[0].gap_id >= pair[1].gap_id)
        {
            return Err(BuildError::TaxonomyShapeMismatch);
        }
        for gap in &input.taxonomy.gaps {
            if !matches!(
                gap.classification,
                ImportMappingClassification::Shimmed | ImportMappingClassification::Unsupported
            ) || !is_reviewable_sentence(&gap.source_label)
                || !is_reviewable_sentence(&gap.gap_summary)
                || !is_bounded_opaque_ref(&gap.gap_id, "migration-flow-gap:")
                || !gap.retained_after_apply
                || gap.docs_help_refs.is_empty()
                || gap.support_export_refs.is_empty()
                || gap.docs_help_refs.len() > MAX_SUPPORT_REFS
                || gap.support_export_refs.len() > MAX_SUPPORT_REFS
                || has_duplicate_strings(&gap.docs_help_refs)
                || has_duplicate_strings(&gap.support_export_refs)
                || gap
                    .docs_help_refs
                    .iter()
                    .chain(gap.support_export_refs.iter())
                    .any(|reference| !is_safe_support_ref(reference))
            {
                return Err(BuildError::TaxonomyShapeMismatch);
            }
            if !gap.visible_before_apply {
                return Err(BuildError::GapHiddenBeforeApply {
                    gap_id: gap.gap_id.clone(),
                });
            }
        }

        // --- claim ceiling: never claim what the product cannot prove ---------
        let diff_reviewed = input.diff.is_reviewable_before_apply();
        let rollback_live = input.rollback.is_live_for_flow();
        let has_gaps = !input.taxonomy.has_no_gaps();
        let taxonomy_complete = input.taxonomy.is_complete();

        if input.claim_ceiling.asserts_diff_reviewed_before_apply && !diff_reviewed {
            return Err(BuildError::OverclaimsDiffReviewed);
        }
        if input.claim_ceiling.asserts_rollback_available && !rollback_live {
            return Err(BuildError::OverclaimsRollbackAvailable);
        }
        if input.claim_ceiling.asserts_no_unsupported_gaps && has_gaps {
            return Err(BuildError::OverclaimsNoUnsupportedGaps);
        }
        if input.claim_ceiling.asserts_full_fidelity_import && !input.taxonomy.is_full_fidelity() {
            return Err(BuildError::OverclaimsFullFidelity);
        }

        // --- recovery routes -------------------------------------------------
        if input.recovery_routes.len() > 5 {
            return Err(BuildError::RecoveryRoutesMismatch);
        }
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        let required_actions = required_recovery_actions(rollback_live, has_gaps);
        for required in &required_actions {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: *required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: privacy_safe_recovery_action_id(&route.action_id),
                });
            }
        }
        if input.recovery_routes.len() != required_actions.len()
            || input
                .recovery_routes
                .iter()
                .zip(required_actions.iter())
                .any(|(actual, required)| actual != &required.route())
        {
            return Err(BuildError::RecoveryRoutesMismatch);
        }

        // --- cross-surface parity --------------------------------------------
        if !input.surfaces.parity_holds
            || !is_bounded_opaque_ref(&input.surfaces.migration_center_row_id, "migration-center:")
            || !is_bounded_opaque_ref(
                &input.surfaces.settings_import_history_row_id,
                "settings-import-history:",
            )
            || !is_bounded_opaque_ref(&input.surfaces.command_palette_command_id, "cmd:")
        {
            return Err(BuildError::SurfaceParityBroken);
        }
        if input.surfaces.recovery_action_ids.len() > 5 || input.surfaces.reopen_surfaces.len() > 3
        {
            return Err(BuildError::SurfaceParityBroken);
        }
        let parity_ids: Vec<&str> = input
            .surfaces
            .recovery_action_ids
            .iter()
            .map(String::as_str)
            .collect();
        if parity_ids != route_ids {
            return Err(BuildError::SurfaceParityBroken);
        }
        for required in ["settings", "help", "support_export"] {
            if !input
                .surfaces
                .reopen_surfaces
                .iter()
                .any(|surface| surface == required)
            {
                return Err(BuildError::ReopenSurfaceMissing { surface: required });
            }
        }
        if input.surfaces.reopen_surfaces.len() != 3 {
            return Err(BuildError::SurfaceParityBroken);
        }

        // --- route parity across surfaces ------------------------------------
        if input.routes.len() > MigrationRouteSurface::REQUIRED.len() {
            return Err(BuildError::SurfaceParityBroken);
        }
        let mut seen_surfaces = Vec::new();
        for route in &input.routes {
            if seen_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_surfaces.push(route.surface);
            require_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_flow {
                return Err(BuildError::RouteTargetsDifferentFlow {
                    surface: route.surface,
                });
            }
        }
        for required in MigrationRouteSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        let ecosystem_label = input.source_ecosystem.display_label().to_string();
        if !is_reviewable_sentence(&input.accessibility.row_narration)
            || !input.accessibility.row_narration.contains(&ecosystem_label)
        {
            return Err(BuildError::NarrationOmitsEcosystem);
        }
        if input.accessibility.layout_modes.len() != LayoutMode::REQUIRED.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability: never bury a flow behind account or services ------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- derive the stable-claim verdict from the evidence ---------------
        let mut narrowing_reasons = Vec::new();
        if !diff_reviewed {
            narrowing_reasons.push(StableNarrowingReason::DiffNotReviewedBeforeApply);
        }
        if !rollback_live {
            narrowing_reasons.push(StableNarrowingReason::RollbackEvidenceIncomplete);
        }
        if !input.taxonomy.unsupported_gaps_visible_before_apply
            || !input
                .taxonomy
                .gaps
                .iter()
                .all(|gap| gap.visible_before_apply)
        {
            narrowing_reasons.push(StableNarrowingReason::UnsupportedGapsHiddenBeforeApply);
        }
        if !taxonomy_complete {
            narrowing_reasons.push(StableNarrowingReason::TaxonomyIncomplete);
        }
        let claim_class = if narrowing_reasons.is_empty() {
            StableClaimClass::Stable
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = StableQualification {
            claim_class,
            qualifies_stable: narrowing_reasons.is_empty(),
            narrowing_reasons,
        };

        let honesty_marker_present = !stable_qualification.qualifies_stable
            || has_gaps
            || !input.taxonomy.is_full_fidelity();

        Ok(Self {
            record_kind: MIGRATION_FLOW_DISCLOSURE_RECORD_KIND.to_string(),
            schema_version: MIGRATION_FLOW_DISCLOSURE_SCHEMA_VERSION,
            notice: MIGRATION_FLOW_DISCLOSURE_NOTICE.to_string(),
            shared_contract_ref: MIGRATION_FLOW_DISCLOSURE_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            migration_review_ref: input.migration_review_ref,
            source_ecosystem: input.source_ecosystem,
            source_ecosystem_label: ecosystem_label,
            header: input.header,
            title: input.title,
            summary: input.summary,
            diff: input.diff,
            rollback: input.rollback,
            taxonomy: input.taxonomy,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            surfaces: input.surfaces,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: input.upstream,
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("migration_flow_disclosure: {}", self.record_id),
            format!("migration_review_ref: {}", self.migration_review_ref),
            format!(
                "header: source={} version={} target={} rollback_requirement={} checkpoint={} restore={} restore_enabled={} compatibility={} issue_template={}",
                self.header.source_tool_label,
                self.header.source_version_label,
                self.header.writes_land_in,
                self.header.rollback_requirement_ref,
                self.header.checkpoint_ref.as_deref().unwrap_or("not_available"),
                self.header.restore_action_ref.as_deref().unwrap_or("not_available"),
                self.header.restore_action_enabled,
                self.header.compatibility_report_ref,
                self.header.issue_template_ref
            ),
            format!("as_of: {}", self.as_of),
            format!(
                "source_ecosystem: {} ({})",
                self.source_ecosystem.as_str(),
                self.source_ecosystem_label
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "diff: rows={} reviewed_before_apply={} before_after={} one_rollback_requirement={}",
                self.diff.row_count,
                self.diff.reviewed_before_apply,
                self.diff.every_row_has_before_after,
                self.diff.every_row_uses_one_requirement
            ),
            format!(
                "rollback: created_before_apply={} protects_every_domain={} verified_for_flow={} undo={} compare={}",
                self.rollback.created_before_apply,
                self.rollback.protects_every_domain,
                self.rollback.verified_for_this_flow,
                self.rollback.undo_available,
                self.rollback.compare_available
            ),
            format!(
                "taxonomy: exact={} translated={} partial={} shimmed={} unsupported={} gaps_visible_before_apply={}",
                self.taxonomy.exact,
                self.taxonomy.translated,
                self.taxonomy.partial,
                self.taxonomy.shimmed,
                self.taxonomy.unsupported,
                self.taxonomy.unsupported_gaps_visible_before_apply
            ),
            format!(
                "claim_ceiling: diff_reviewed={} rollback_available={} no_unsupported_gaps={} full_fidelity={}",
                self.claim_ceiling.asserts_diff_reviewed_before_apply,
                self.claim_ceiling.asserts_rollback_available,
                self.claim_ceiling.asserts_no_unsupported_gaps,
                self.claim_ceiling.asserts_full_fidelity_import
            ),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ];
        lines.push("gaps:".to_string());
        for gap in &self.taxonomy.gaps {
            lines.push(format!(
                "  - {} ({}) [{}] visible_before_apply={} -- {}",
                gap.gap_id,
                gap.domain.as_str(),
                gap.classification.as_str(),
                gap.visible_before_apply,
                gap.gap_summary
            ));
        }
        lines.push("recovery_routes:".to_string());
        for route in &self.recovery_routes {
            lines.push(format!(
                "  - {} ({}) role={} keyboard={}",
                route.action_id,
                route.action_label,
                route.action_role.as_str(),
                route.keyboard_reachable
            ));
        }
        lines.push(format!(
            "surfaces: migration_center={} settings={} command={} parity_holds={} reopen=[{}]",
            self.surfaces.migration_center_row_id,
            self.surfaces.settings_import_history_row_id,
            self.surfaces.command_palette_command_id,
            self.surfaces.parity_holds,
            self.surfaces.reopen_surfaces.join(", ")
        ));
        lines.push("routes:".to_string());
        for route in &self.routes {
            lines.push(format!(
                "  - {} -> {} keyboard={} same_flow={}",
                route.surface.as_str(),
                route.route_ref,
                route.keyboard_reachable,
                route.activates_same_flow
            ));
        }
        lines.push(format!(
            "accessibility: tab_order={} tab_stops={} narration={:?}",
            self.accessibility.focus_order_index,
            self.accessibility.tab_stop_count,
            self.accessibility.row_narration
        ));
        for mode in &self.accessibility.layout_modes {
            lines.push(format!(
                "  layout {} narration={} affordances_reachable={}",
                mode.mode.as_str(),
                mode.row_narration_available,
                mode.recovery_affordances_reachable
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "upstream: wizard_review={} mapping_report={} rollback_requirement={} diff_preview={} scoreboard={} section={}",
            self.upstream.wizard_review_ref,
            self.upstream.wizard_mapping_report_ref,
            self.upstream.rollback_requirement_ref,
            self.upstream.import_diff_preview_ref,
            self.upstream.corpus_scoreboard_ref,
            self.upstream.corpus_section_ref
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

#[cfg(test)]
mod validation_tests {
    use super::{
        is_reviewable_sentence, is_utc_timestamp, privacy_safe_recovery_action_id, GapTaxonomy,
    };

    #[test]
    fn utc_timestamp_validation_rejects_impossible_or_non_utc_dates() {
        assert!(is_utc_timestamp("2024-02-29T23:59:59Z"));
        assert!(is_utc_timestamp("2024-02-29T23:59:59.123456Z"));
        for invalid in [
            "2023-02-29T00:00:00Z",
            "2024-13-01T00:00:00Z",
            "2024-01-01T24:00:00Z",
            "2024-01-01T00:00:00+00:00",
            "2024-01-01T00:00:00.Z",
        ] {
            assert!(!is_utc_timestamp(invalid), "accepted {invalid}");
        }
    }

    #[test]
    fn support_sentences_reject_private_paths_and_urls() {
        assert!(is_reviewable_sentence(
            "Review unsupported extension mappings."
        ));
        for private in [
            "/Users/alice/Secret Project/settings.json",
            "Open /Users/alice/Secret Project/settings.json",
            "C:\\Users\\alice\\secret.json",
            "Review C:\\Users\\alice\\secret.json",
            "https://alice@example.invalid/private?token=abc",
            "file:/Users/alice/private.json",
            "target=/Users/alice/private.json",
            "target:\\Users\\alice\\private.json",
            " leading whitespace",
            "trailing whitespace ",
            "../customer/private.json",
        ] {
            assert!(!is_reviewable_sentence(private), "accepted {private:?}");
        }
        assert!(is_reviewable_sentence("VS Code / Code OSS"));
    }

    #[test]
    fn recovery_error_ids_redact_noncanonical_input() {
        assert_eq!(
            privacy_safe_recovery_action_id("reopen_migration_report"),
            "reopen_migration_report"
        );
        assert_eq!(
            privacy_safe_recovery_action_id("/Users/alice/Secret Project"),
            "[redacted invalid action id]"
        );
    }

    #[test]
    fn taxonomy_total_saturates_instead_of_panicking_on_hostile_counts() {
        let taxonomy = GapTaxonomy {
            exact: u32::MAX,
            translated: u32::MAX,
            partial: u32::MAX,
            shimmed: u32::MAX,
            unsupported: u32::MAX,
            classifications_present: Vec::new(),
            unsupported_gaps_visible_before_apply: false,
            gaps: Vec::new(),
        };
        assert_eq!(taxonomy.total(), u32::MAX);
    }
}
