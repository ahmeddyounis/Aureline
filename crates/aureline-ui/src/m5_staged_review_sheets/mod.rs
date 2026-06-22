//! Canonical staged-review (commit) sheet truth for consequential M5 mutation
//! flows.
//!
//! Where [`crate::m5_structured_input_and_staged_review`] binds the *form* a user
//! edits, this module makes the **commit sheet itself** the first-class object
//! every consequential M5 mutation flow stops at before it changes
//! remote/provider/admin/package/request/import state. One review model is reused
//! across provider publish-later flows, admin/source-management changes, request
//! replay/mutation, package install/update/remove, and import/export/publish
//! sheets, instead of each domain minting a one-off confirm dialog. Each
//! [`ReviewSheetRecord`] declares:
//!
//! * its **target scope** — a [`ScopeKind`] (single object, an explicit
//!   multi-object selection, a query-backed selection, or a workspace-wide action)
//!   with a labelled, visible scope so the user knows *what* the commit acts on;
//! * its **omitted defaults** — how many default values are being applied
//!   silently, disclosed with a count rather than hidden;
//! * its **included / excluded / blocked / hidden counts** — a reconciled
//!   [`MemberCounts`] block where `included + excluded + blocked + hidden` equals
//!   the total matched, so a query-backed or broad action can never hide how many
//!   objects it will touch behind a collapsed list;
//! * its **side-effect summary** — every [`SideEffectDescriptor`] disclosed before
//!   commit, with an aggregate [`RecoverabilityClass`] and a rollback/export path
//!   appropriate to how reversible the commit is; and
//! * its **commit action** — a scope-and-effect-specific confirm action rather than
//!   a generic Continue, plus an action-specific cancel.
//!
//! Each record re-derives a [`SheetClaim`] ([`ReviewSheetRecord::narrow`]) so a
//! sheet can never read wider than its evidence: a sheet that hides its target
//! scope, lets its member counts disagree, leaves hidden/collapsed members
//! uncounted, hides the included/excluded/blocked counts on a multi-object action,
//! hides omitted defaults, leaves a side effect undisclosed, buries a blocked
//! prerequisite or rollback consequence behind a generic Continue, lets an
//! imported/restore review read as a local apply, loses its reopen path, or renders
//! wider than its claim floors to [`SheetClaim::Unsafe`] and falls back to an
//! explicit blocked state with a reopen/keyboard recovery path. A labelled,
//! recoverable gap (an unlabelled member class, an aged verification proof, a stale
//! scope snapshot) holds a first-party sheet at [`SheetClaim::Narrowed`] while
//! keeping the scope reopenable, an import/export review sits at
//! [`SheetClaim::ReviewOverlay`] and never claims a local apply, and a
//! Labs/unadvertised sheet makes no public claim.
//!
//! [`M5StagedReviewSheetSetPacket::validate`] confirms the matrix is well-formed and
//! honest: header/identity/redaction/freshness are present, every mutation flow,
//! lane, scope kind, member class, side-effect class, and consumer surface is
//! represented, overlay sheets name their provenance, no rendering surface
//! overclaims, a floored sheet keeps a fallback, at least one sheet demonstrates the
//! auto-narrowing rule, and no raw credential/body material crosses the export.
//! Downstream marketplace, request, support, admin, import, settings, and project
//! surfaces ingest this packet rather than minting per-feature commit-sheet
//! semantics.
//!
//! No credential bodies, secret values, raw provider payloads, absolute paths, or
//! URLs ever cross this boundary; the packet carries only typed class tokens,
//! counts, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/ux/m5-staged-review-sheets.schema.json`](../../../../schemas/ux/m5-staged-review-sheets.schema.json).
//! The contract doc is
//! [`docs/ux/m5-staged-review-sheets.md`](../../../../docs/ux/m5-staged-review-sheets.md).
//! The canonical support export is
//! [`artifacts/ux/m5-staged-review-sheets/support_export.json`](../../../../artifacts/ux/m5-staged-review-sheets/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/ux/m5-staged-review-sheets/`](../../../../fixtures/ux/m5-staged-review-sheets/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5StagedReviewSheetSetPacket`].
pub const M5_STAGED_REVIEW_RECORD_KIND: &str = "m5_staged_review_sheet_set_packet";

/// Schema version for the staged-review sheet set.
pub const M5_STAGED_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_STAGED_REVIEW_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical staged-review sheet set packet.
pub const M5_STAGED_REVIEW_PACKET_ID: &str = "m5-staged-review-sheets:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_STAGED_REVIEW_SCHEMA_REF: &str = "schemas/ux/m5-staged-review-sheets.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_STAGED_REVIEW_DOC_REF: &str = "docs/ux/m5-staged-review-sheets.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_STAGED_REVIEW_SUPPORT_EXPORT_REF: &str =
    "artifacts/ux/m5-staged-review-sheets/support_export.json";

/// Repo-relative path of the generated report.
pub const M5_STAGED_REVIEW_REPORT_REF: &str = "artifacts/ux/m5-staged-review-sheets/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_STAGED_REVIEW_FIXTURE_DIR: &str = "fixtures/ux/m5-staged-review-sheets";

/// Allowed packet redaction-class tokens.
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

/// Deterministic seed timestamp for the canonical packet and report.
const SEED_AS_OF: &str = "2026-06-21T00:00:00Z";

// --------------------------------------------------------------------------- //
// Date helper (self-contained; no external crate dependency).
// --------------------------------------------------------------------------- //

fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let m = i64::from(month);
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Parses an RFC 3339 timestamp into epoch seconds, or `None` if malformed.
fn parse_rfc3339_to_epoch_seconds(value: &str) -> Option<i64> {
    let s = value.trim();
    let bytes = s.as_bytes();
    if s.len() < 19 {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    if *bytes.get(4)? != b'-' {
        return None;
    }
    let month: u32 = s.get(5..7)?.parse().ok()?;
    if *bytes.get(7)? != b'-' {
        return None;
    }
    let day: u32 = s.get(8..10)?.parse().ok()?;
    match bytes.get(10)? {
        b'T' | b't' | b' ' => {}
        _ => return None,
    }
    let hour: i64 = s.get(11..13)?.parse().ok()?;
    if *bytes.get(13)? != b':' {
        return None;
    }
    let minute: i64 = s.get(14..16)?.parse().ok()?;
    if *bytes.get(16)? != b':' {
        return None;
    }
    let second: i64 = s.get(17..19)?.parse().ok()?;

    let mut rest = &s[19..];
    if let Some(stripped) = rest.strip_prefix('.') {
        let digits = stripped.bytes().take_while(u8::is_ascii_digit).count();
        rest = &stripped[digits..];
    }

    let offset_seconds = if rest.is_empty() || rest == "Z" || rest == "z" {
        0
    } else {
        let sign = match rest.bytes().next()? {
            b'+' => 1,
            b'-' => -1,
            _ => return None,
        };
        let off = &rest[1..];
        let (oh, om) = if let Some((hh, mm)) = off.split_once(':') {
            (hh.parse::<i64>().ok()?, mm.parse::<i64>().ok()?)
        } else if off.len() == 4 {
            (
                off.get(0..2)?.parse::<i64>().ok()?,
                off.get(2..4)?.parse::<i64>().ok()?,
            )
        } else {
            return None;
        };
        sign * (oh * 3600 + om * 60)
    };

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let days = days_from_civil(year, month, day);
    Some(days * 86_400 + hour * 3600 + minute * 60 + second - offset_seconds)
}

/// Whether a reviewer-facing label is empty or one of the generic non-labels that
/// hide the real downgrade reason.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    matches!(
        trimmed.to_lowercase().as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "downgraded"
            | "unverified"
            | "narrowed"
            | "stale"
            | "blocked"
    )
}

/// Whether a serialized value carries forbidden raw boundary material (secrets,
/// credential bodies). The export must never leak these.
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

fn opt_present(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|s| !s.trim().is_empty())
}

// --------------------------------------------------------------------------- //
// Frozen taxonomies (mirror the boundary schema).
// --------------------------------------------------------------------------- //

/// The consequential M5 mutation flow a sheet gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationFlow {
    /// A provider publish-later commit (publish queued provider state).
    ProviderPublishLater,
    /// An admin source-management change.
    AdminSourceManagement,
    /// A request replay or mutation.
    RequestReplayMutation,
    /// A package install/update/remove lifecycle action.
    PackageLifecycle,
    /// An import/export/publish commit.
    ImportExportPublish,
    /// A bulk settings/configuration apply.
    SettingsBulkApply,
}

impl MutationFlow {
    /// Every mutation flow, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderPublishLater,
        Self::AdminSourceManagement,
        Self::RequestReplayMutation,
        Self::PackageLifecycle,
        Self::ImportExportPublish,
        Self::SettingsBulkApply,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderPublishLater => "provider_publish_later",
            Self::AdminSourceManagement => "admin_source_management",
            Self::RequestReplayMutation => "request_replay_mutation",
            Self::PackageLifecycle => "package_lifecycle",
            Self::ImportExportPublish => "import_export_publish",
            Self::SettingsBulkApply => "settings_bulk_apply",
        }
    }

    /// Whether the flow is inherently provider-owned or remote-bound, so its commit
    /// action must always name the scope and effect.
    pub const fn is_provider_or_remote(self) -> bool {
        matches!(
            self,
            Self::ProviderPublishLater | Self::RequestReplayMutation | Self::ImportExportPublish
        )
    }
}

/// The product lane the sheet belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowLane {
    /// Provider configuration / publish.
    Provider,
    /// Admin / source management.
    Admin,
    /// Request workspace.
    Request,
    /// Package / marketplace.
    Package,
    /// Import / migration center.
    Import,
    /// Settings / configuration.
    Settings,
}

impl FlowLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Provider,
        Self::Admin,
        Self::Request,
        Self::Package,
        Self::Import,
        Self::Settings,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provider => "provider",
            Self::Admin => "admin",
            Self::Request => "request",
            Self::Package => "package",
            Self::Import => "import",
            Self::Settings => "settings",
        }
    }
}

/// How the commit's target scope is selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    /// A single named object.
    SingleObject,
    /// An explicit, individually-checked multi-object selection.
    MultiObjectExplicit,
    /// A selection derived from a filter/query (size may exceed what is shown).
    QueryBacked,
    /// A workspace-wide broad action.
    WorkspaceWide,
}

impl ScopeKind {
    /// Every scope kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SingleObject,
        Self::MultiObjectExplicit,
        Self::QueryBacked,
        Self::WorkspaceWide,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleObject => "single_object",
            Self::MultiObjectExplicit => "multi_object_explicit",
            Self::QueryBacked => "query_backed",
            Self::WorkspaceWide => "workspace_wide",
        }
    }

    /// Whether the scope acts on more than one object, so an included/excluded/
    /// blocked/hidden counts block must be surfaced.
    pub const fn is_multi_object(self) -> bool {
        matches!(
            self,
            Self::MultiObjectExplicit | Self::QueryBacked | Self::WorkspaceWide
        )
    }

    /// Whether the scope can match more objects than are individually rendered, so
    /// a hidden/collapsed count must be disclosed.
    pub const fn can_hide_members(self) -> bool {
        matches!(self, Self::QueryBacked | Self::WorkspaceWide)
    }
}

/// The membership class of one object in the staged review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewMemberClass {
    /// Included in the commit.
    Included,
    /// Excluded because it is a default.
    ExcludedByDefault,
    /// Excluded by the user.
    ExcludedByUser,
    /// Blocked by an unmet prerequisite.
    BlockedPrerequisite,
    /// Matched but collapsed/hidden from the per-object list (covered by a count).
    HiddenCollapsed,
}

impl ReviewMemberClass {
    /// Every member class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Included,
        Self::ExcludedByDefault,
        Self::ExcludedByUser,
        Self::BlockedPrerequisite,
        Self::HiddenCollapsed,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Included => "included",
            Self::ExcludedByDefault => "excluded_by_default",
            Self::ExcludedByUser => "excluded_by_user",
            Self::BlockedPrerequisite => "blocked_prerequisite",
            Self::HiddenCollapsed => "hidden_collapsed",
        }
    }
}

/// The side-effect class declared by a staged review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// Reversible with no extra step.
    ReversibleLocal,
    /// Reversible if the user exports/backs up first.
    ReversibleWithExport,
    /// Irreversible; requires explicit confirmation.
    IrreversibleConfirmed,
    /// Publishes to an external service.
    ExternalPublish,
    /// Governed by policy.
    PolicyGoverned,
}

impl SideEffectClass {
    /// Every side-effect class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReversibleLocal,
        Self::ReversibleWithExport,
        Self::IrreversibleConfirmed,
        Self::ExternalPublish,
        Self::PolicyGoverned,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleLocal => "reversible_local",
            Self::ReversibleWithExport => "reversible_with_export",
            Self::IrreversibleConfirmed => "irreversible_confirmed",
            Self::ExternalPublish => "external_publish",
            Self::PolicyGoverned => "policy_governed",
        }
    }

    /// Whether the effect requires an export/backup path to stay recoverable.
    pub const fn requires_export_path(self) -> bool {
        matches!(self, Self::IrreversibleConfirmed | Self::ExternalPublish)
    }
}

/// The aggregate reversibility posture of the whole commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoverabilityClass {
    /// Fully reversible from the product (an undo/rollback exists).
    FullyReversible,
    /// Reversible only if the user exports/backs up first.
    ReversibleViaExport,
    /// Some effects can be undone, others cannot.
    PartiallyReversible,
    /// Cannot be undone once committed.
    Irreversible,
}

impl RecoverabilityClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReversible => "fully_reversible",
            Self::ReversibleViaExport => "reversible_via_export",
            Self::PartiallyReversible => "partially_reversible",
            Self::Irreversible => "irreversible",
        }
    }

    /// Whether this posture requires an export/backup path before commit.
    pub const fn requires_export_path(self) -> bool {
        matches!(self, Self::ReversibleViaExport | Self::Irreversible)
    }

    /// Whether this posture commits a destructive change, so the commit action must
    /// name the scope and effect.
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::PartiallyReversible | Self::Irreversible)
    }
}

/// How the sheet and its scope originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SheetOrigin {
    /// A first-party local commit.
    LocalCommit,
    /// A first-party commit against a remote target.
    RemoteCommit,
    /// A provider-backed commit.
    ProviderCommit,
    /// A review of imported/migrated/restored state (an overlay).
    ImportedReview,
}

impl SheetOrigin {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCommit => "local_commit",
            Self::RemoteCommit => "remote_commit",
            Self::ProviderCommit => "provider_commit",
            Self::ImportedReview => "imported_review",
        }
    }

    /// Whether this origin is an inherently read-only review overlay.
    pub const fn is_overlay(self) -> bool {
        matches!(self, Self::ImportedReview)
    }

    /// Whether the origin is provider-owned or remote-bound.
    pub const fn is_provider_or_remote(self) -> bool {
        matches!(self, Self::RemoteCommit | Self::ProviderCommit)
    }
}

/// The freshness of the scope snapshot the sheet was computed from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Live.
    Live,
    /// A cached snapshot within its window.
    CachedSnapshot,
    /// Stale / expired.
    StaleExpired,
    /// Superseded by a newer source.
    SupersededByNewerSource,
    /// Missing.
    Missing,
}

impl FreshnessState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CachedSnapshot => "cached_snapshot",
            Self::StaleExpired => "stale_expired",
            Self::SupersededByNewerSource => "superseded_by_newer_source",
            Self::Missing => "missing",
        }
    }
}

/// Verification-proof currency for a sheet (distinct from scope freshness).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofCurrency {
    /// Verified and current.
    VerifiedCurrent,
    /// Cached within the verification window.
    CachedWithinWindow,
    /// Stale / expired.
    StaleExpired,
    /// Requires review before it can be trusted.
    RequiresReview,
    /// No proof anchors the sheet.
    MissingProof,
}

impl ProofCurrency {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::CachedWithinWindow => "cached_within_window",
            Self::StaleExpired => "stale_expired",
            Self::RequiresReview => "requires_review",
            Self::MissingProof => "missing_proof",
        }
    }
}

/// What a cancel/reopen returns the user to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenTarget {
    /// Reopen restores the sheet and its scope selection.
    SheetAndScope,
    /// Reopen restores the scope selection only.
    ScopeOnly,
    /// No reopen; a keyboard fallback to the originating surface remains.
    NoneKeyboardFallback,
}

impl ReopenTarget {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SheetAndScope => "sheet_and_scope",
            Self::ScopeOnly => "scope_only",
            Self::NoneKeyboardFallback => "none_keyboard_fallback",
        }
    }
}

/// Whether the sheet is publicly claimed or a Labs/unadvertised surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPosture {
    /// Publicly claimed and stable.
    ClaimedStable,
    /// Labs / unadvertised.
    LabsUnadvertised,
}

impl ClaimPosture {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimedStable => "claimed_stable",
            Self::LabsUnadvertised => "labs_unadvertised",
        }
    }
}

/// A consumer surface that re-renders a sheet record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// The staged-review sheet itself.
    ReviewSheet,
    /// The batch selection bar that opens the sheet.
    BatchSelectionBar,
    /// The diagnostics panel.
    DiagnosticsPanel,
    /// A support-export bundle.
    SupportExport,
    /// An AI-evidence consumer.
    AiEvidence,
    /// Inline help.
    HelpInline,
    /// A CLI/headless confirmation surface.
    CliConfirmation,
}

impl ConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReviewSheet,
        Self::BatchSelectionBar,
        Self::DiagnosticsPanel,
        Self::SupportExport,
        Self::AiEvidence,
        Self::HelpInline,
        Self::CliConfirmation,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewSheet => "review_sheet",
            Self::BatchSelectionBar => "batch_selection_bar",
            Self::DiagnosticsPanel => "diagnostics_panel",
            Self::SupportExport => "support_export",
            Self::AiEvidence => "ai_evidence",
            Self::HelpInline => "help_inline",
            Self::CliConfirmation => "cli_confirmation",
        }
    }
}

// --------------------------------------------------------------------------- //
// Derived claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim a staged-review sheet renders. A higher rank asserts more
/// authority, so a narrowed or floored sheet must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SheetClaim {
    /// The commit-review contract is broken: the sheet hides scope, counts,
    /// defaults, side effects, prerequisites, or rollback, lets an imported review
    /// read as an apply, or renders wider than its claim. It must fall back to an
    /// explicit blocked state with a reopen/keyboard recovery path instead of a
    /// clean commit.
    #[serde(rename = "sheet_unsafe")]
    Unsafe,
    /// A review of imported/migrated/restored state: attributable and reopenable but
    /// never reads as a local apply.
    #[serde(rename = "sheet_review_overlay")]
    ReviewOverlay,
    /// A first-party sheet held below certified by a labelled, recoverable gap; the
    /// scope stays reopenable.
    #[serde(rename = "sheet_narrowed")]
    Narrowed,
    /// Full scope-explicit, count-reconciled, side-effect-disclosed,
    /// rollback-visible commit-review contract.
    #[serde(rename = "sheet_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "sheet_labs_not_claimed")]
    LabsNotClaimed,
}

impl SheetClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsafe => "sheet_unsafe",
            Self::ReviewOverlay => "sheet_review_overlay",
            Self::Narrowed => "sheet_narrowed",
            Self::Certified => "sheet_certified",
            Self::LabsNotClaimed => "sheet_labs_not_claimed",
        }
    }

    /// Monotonic rank, or `None` for the non-claiming Labs token.
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::Unsafe => Some(0),
            Self::ReviewOverlay => Some(1),
            Self::Narrowed => Some(2),
            Self::Certified => Some(3),
            Self::LabsNotClaimed => None,
        }
    }

    /// Whether rendering `rendered` would overclaim relative to this effective
    /// claim. A rendering surface must never render wider than the sheet's effective
    /// claim; the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: SheetClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a sheet fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SheetNarrowingReason {
    /// The sheet does not declare its target scope.
    TargetScopeHidden,
    /// The member counts do not reconcile with the declared total.
    MemberCountsInconsistent,
    /// A query-backed/broad action collapses members but reports zero hidden.
    HiddenMembersUncounted,
    /// A multi-object action hides the included/excluded/blocked counts.
    IncludedExcludedBlockedCountsHidden,
    /// The sheet hides omitted defaults.
    OmittedDefaultsHidden,
    /// A side effect is not disclosed before commit.
    SideEffectUndisclosed,
    /// A blocked prerequisite is not explained before commit.
    BlockedPrereqHidden,
    /// The rollback/export consequence is not visible.
    RollbackConsequencesHidden,
    /// The commit action is a generic Continue that hides scope/effect.
    GenericContinueAction,
    /// An imported/restore review reads as a local apply.
    ImportedReviewReadsAsApply,
    /// The reopen-to-scope path is lost.
    ReopenPathLost,
    /// A rendering surface renders wider than the effective claim.
    SheetOverclaims,
    /// The scope snapshot is missing.
    SheetBackingMissing,
    /// Included/excluded/blocked/hidden member classes are not labelled.
    MemberClassesUnlabeled,
    /// The aggregate side-effect summary is not surfaced.
    SideEffectSummaryUnlabeled,
    /// The cancel action is not action-specific.
    CancelActionUnlabeled,
    /// The aggregate reversibility posture is not surfaced.
    RecoverabilityClassUnlabeled,
    /// The scope freshness state is not surfaced.
    FreshnessUnlabeled,
    /// A superseded scope snapshot is not marked.
    SupersededScopeNotMarked,
    /// A first-party scope snapshot is stale.
    ScopeStale,
    /// The verification proof is stale.
    VerificationProofStale,
    /// The verification proof is missing.
    VerificationProofMissing,
}

impl SheetNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetScopeHidden => "target_scope_hidden",
            Self::MemberCountsInconsistent => "member_counts_inconsistent",
            Self::HiddenMembersUncounted => "hidden_members_uncounted",
            Self::IncludedExcludedBlockedCountsHidden => "included_excluded_blocked_counts_hidden",
            Self::OmittedDefaultsHidden => "omitted_defaults_hidden",
            Self::SideEffectUndisclosed => "side_effect_undisclosed",
            Self::BlockedPrereqHidden => "blocked_prereq_hidden",
            Self::RollbackConsequencesHidden => "rollback_consequences_hidden",
            Self::GenericContinueAction => "generic_continue_action",
            Self::ImportedReviewReadsAsApply => "imported_review_reads_as_apply",
            Self::ReopenPathLost => "reopen_path_lost",
            Self::SheetOverclaims => "sheet_overclaims",
            Self::SheetBackingMissing => "sheet_backing_missing",
            Self::MemberClassesUnlabeled => "member_classes_unlabeled",
            Self::SideEffectSummaryUnlabeled => "side_effect_summary_unlabeled",
            Self::CancelActionUnlabeled => "cancel_action_unlabeled",
            Self::RecoverabilityClassUnlabeled => "recoverability_class_unlabeled",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::SupersededScopeNotMarked => "superseded_scope_not_marked",
            Self::ScopeStale => "scope_stale",
            Self::VerificationProofStale => "verification_proof_stale",
            Self::VerificationProofMissing => "verification_proof_missing",
        }
    }

    /// Deterministic ordering index (mirrors the validator's reason order).
    pub const fn order_index(self) -> u8 {
        match self {
            Self::TargetScopeHidden => 0,
            Self::MemberCountsInconsistent => 1,
            Self::HiddenMembersUncounted => 2,
            Self::IncludedExcludedBlockedCountsHidden => 3,
            Self::OmittedDefaultsHidden => 4,
            Self::SideEffectUndisclosed => 5,
            Self::BlockedPrereqHidden => 6,
            Self::RollbackConsequencesHidden => 7,
            Self::GenericContinueAction => 8,
            Self::ImportedReviewReadsAsApply => 9,
            Self::ReopenPathLost => 10,
            Self::SheetOverclaims => 11,
            Self::SheetBackingMissing => 12,
            Self::MemberClassesUnlabeled => 13,
            Self::SideEffectSummaryUnlabeled => 14,
            Self::CancelActionUnlabeled => 15,
            Self::RecoverabilityClassUnlabeled => 16,
            Self::FreshnessUnlabeled => 17,
            Self::SupersededScopeNotMarked => 18,
            Self::ScopeStale => 19,
            Self::VerificationProofStale => 20,
            Self::VerificationProofMissing => 21,
        }
    }

    /// Whether this reason breaks the contract outright (floors the sheet to
    /// [`SheetClaim::Unsafe`]) rather than merely aging it out.
    pub const fn is_floor(self) -> bool {
        self.order_index() <= Self::SheetBackingMissing.order_index()
    }

    /// A reviewer-facing, non-generic description of the reason.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::TargetScopeHidden => "the sheet does not declare its target scope",
            Self::MemberCountsInconsistent => {
                "the included/excluded/blocked/hidden counts do not reconcile with the total matched"
            }
            Self::HiddenMembersUncounted => {
                "a query-backed action collapses members but reports zero hidden"
            }
            Self::IncludedExcludedBlockedCountsHidden => {
                "a multi-object action hides the included/excluded/blocked counts"
            }
            Self::OmittedDefaultsHidden => "the sheet hides omitted defaults",
            Self::SideEffectUndisclosed => "a side effect is not disclosed before commit",
            Self::BlockedPrereqHidden => "a blocked prerequisite is not explained before commit",
            Self::RollbackConsequencesHidden => "the rollback/export consequence is not visible",
            Self::GenericContinueAction => {
                "the commit action is a generic Continue that hides scope and effect"
            }
            Self::ImportedReviewReadsAsApply => {
                "an imported/restore review reads as a local apply"
            }
            Self::ReopenPathLost => "the reopen-to-scope path is lost",
            Self::SheetOverclaims => "a rendering surface renders wider than the effective claim",
            Self::SheetBackingMissing => "the scope snapshot is missing",
            Self::MemberClassesUnlabeled => {
                "included/excluded/blocked/hidden member classes are not labelled"
            }
            Self::SideEffectSummaryUnlabeled => "the aggregate side-effect summary is not surfaced",
            Self::CancelActionUnlabeled => "the cancel action is not action-specific",
            Self::RecoverabilityClassUnlabeled => {
                "the aggregate reversibility posture is not surfaced"
            }
            Self::FreshnessUnlabeled => "the scope freshness state is not surfaced",
            Self::SupersededScopeNotMarked => "a superseded scope snapshot is not marked",
            Self::ScopeStale => "the scope snapshot is stale",
            Self::VerificationProofStale => "the verification proof is stale",
            Self::VerificationProofMissing => "the verification proof is missing",
        }
    }
}

fn order_reasons(mut reasons: Vec<SheetNarrowingReason>) -> Vec<SheetNarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Sheet sub-objects.
// --------------------------------------------------------------------------- //

/// The evidence freshness window for the packet's verification proofs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationFreshness {
    /// Hours after which a current proof ages out.
    pub verification_freshness_slo_hours: u32,
    /// Last verification refresh (RFC 3339).
    pub last_verification_refresh: String,
    /// Whether an elapsed window auto-downgrades a current proof.
    pub auto_downgrade_on_stale: bool,
}

/// Stable origin-lineage block; refs carry opaque ids only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SheetLineage {
    /// The commit session ref.
    pub session_ref: String,
    /// The canonical sheet this view re-renders, when distinct.
    pub canonical_sheet_ref: Option<String>,
    /// The remote/provider/scope target ref.
    pub target_ref: Option<String>,
    /// The provider ref, for provider-backed sheets.
    pub provider_ref: Option<String>,
    /// The imported source-artifact ref, for review overlays.
    pub source_artifact_ref: Option<String>,
    /// The rollback-plan ref.
    pub rollback_plan_ref: Option<String>,
    /// The export-bundle ref.
    pub export_bundle_ref: Option<String>,
    /// The reopen-to-origin backlink ref.
    pub reopen_backlink_ref: Option<String>,
}

/// The declared target scope of the commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetScope {
    /// How the scope is selected.
    pub scope_kind: ScopeKind,
    /// Whether the scope is declared on the sheet.
    pub scope_declared: bool,
    /// Reviewer-facing scope label.
    pub scope_label: String,
}

/// The reconciled included/excluded/blocked/hidden counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberCounts {
    /// Objects included in the commit.
    pub included: u64,
    /// Objects excluded (by default or by the user).
    pub excluded: u64,
    /// Objects blocked by an unmet prerequisite.
    pub blocked: u64,
    /// Objects matched but collapsed/hidden from the per-object list.
    pub hidden: u64,
    /// Total objects matched by the scope.
    pub total_matched: u64,
    /// Whether the counts are surfaced on the sheet.
    pub counts_visible: bool,
}

impl MemberCounts {
    /// Whether `included + excluded + blocked + hidden == total_matched`.
    pub fn reconciles(&self) -> bool {
        self.included
            .checked_add(self.excluded)
            .and_then(|v| v.checked_add(self.blocked))
            .and_then(|v| v.checked_add(self.hidden))
            == Some(self.total_matched)
    }
}

/// One member of the staged review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewMember {
    /// Stable member id.
    pub member_id: String,
    /// The membership class.
    pub member_class: ReviewMemberClass,
    /// Whether the member's reason (why included/excluded/blocked/hidden) is
    /// labelled.
    pub reason_labeled: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// One declared side effect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectDescriptor {
    /// Stable effect id.
    pub effect_id: String,
    /// The side-effect class.
    pub effect_class: SideEffectClass,
    /// Whether the effect is disclosed before commit.
    pub disclosed_before_commit: bool,
    /// Whether the effect is reversible.
    pub reversible: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The aggregate recoverability posture and the paths that back it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recoverability {
    /// The aggregate reversibility posture.
    pub recoverability_class: RecoverabilityClass,
    /// Whether the posture is labelled on the sheet.
    pub recoverability_class_labeled: bool,
    /// Whether a rollback path is present.
    pub rollback_path_present: bool,
    /// Whether an export/backup path is present.
    pub export_path_present: bool,
    /// Reviewer-facing recovery label.
    pub recovery_label: String,
}

/// The confirm/cancel actions on the commit sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitAction {
    /// Whether the commit action names the scope/effect rather than a generic
    /// Continue.
    pub commit_action_is_specific: bool,
    /// Reviewer-facing commit action label.
    pub commit_action_label: String,
    /// Whether the cancel action is action-specific.
    pub cancel_action_is_specific: bool,
    /// Reviewer-facing cancel action label.
    pub cancel_action_label: String,
}

/// The staged-review (commit) sheet payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedReviewSheet {
    /// The declared target scope.
    pub scope: TargetScope,
    /// Whether omitted defaults are disclosed.
    pub omitted_defaults_disclosed: bool,
    /// Count of omitted defaults.
    pub omitted_default_count: u64,
    /// The reconciled member counts.
    pub counts: MemberCounts,
    /// Per-object members (may be a representative subset for collapsed scopes).
    pub members: Vec<ReviewMember>,
    /// Whether the member classes are labelled.
    pub members_classes_labeled: bool,
    /// Declared side effects.
    pub side_effects: Vec<SideEffectDescriptor>,
    /// Whether side effects are disclosed before commit.
    pub side_effects_disclosed: bool,
    /// Whether the aggregate side-effect summary is surfaced.
    pub side_effect_summary_labeled: bool,
    /// The recoverability block.
    pub recoverability: Recoverability,
    /// The commit/cancel actions.
    pub commit: CommitAction,
}

/// The headline commit-review invariants every sheet re-derives rather than
/// trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SheetIntegrity {
    /// The target scope is visible on the sheet.
    pub target_scope_visible: bool,
    /// The member counts are visible.
    pub counts_visible: bool,
    /// Omitted defaults are visible.
    pub omitted_defaults_visible: bool,
    /// Side effects are disclosed before commit.
    pub side_effects_disclosed: bool,
    /// Blocked prerequisites are explained before commit.
    pub blocked_prereqs_explained: bool,
    /// The rollback consequence is visible.
    pub rollback_visible: bool,
    /// The commit action names scope/effect.
    pub commit_action_specific: bool,
    /// Imported/restore reviews stay read-only.
    pub imported_review_read_only: bool,
    /// Member classes are labelled.
    pub member_classes_labeled: bool,
    /// The recoverability posture is labelled.
    pub recoverability_labeled: bool,
    /// The freshness state is visible.
    pub freshness_state_visible: bool,
    /// A superseded scope snapshot stays marked.
    pub superseded_state_marked: bool,
    /// Reopen-to-scope is revealable on demand on every surface.
    pub reopen_visible_on_demand: bool,
}

/// Verification-proof currency for a sheet (distinct from scope freshness).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SheetVerification {
    /// Currency of the verification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the sheet.
    pub proof_ref: Option<String>,
}

/// One consumer surface that renders a sheet record, with the claim it shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SheetRendering {
    /// The consumer surface.
    pub surface: ConsumerSurface,
    /// The claim this surface renders.
    pub rendered_claim: SheetClaim,
    /// Whether scope provenance is revealable here.
    pub scope_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical sheet this view re-renders.
    pub source_sheet_ref: String,
}

// --------------------------------------------------------------------------- //
// Sheet + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) staged-review sheet for a consequential M5 mutation flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSheetRecord {
    /// Stable sheet id.
    pub sheet_id: String,
    /// The mutation flow this sheet gates.
    pub flow: MutationFlow,
    /// The product lane.
    pub lane: FlowLane,
    /// How the sheet/scope originated.
    pub origin: SheetOrigin,
    /// Reviewer-facing label summary.
    pub label_summary: String,
    /// Whether the sheet is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// Declared scope-snapshot freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared reopen target.
    pub declared_reopen_target: ReopenTarget,
    /// Stable origin-lineage block.
    pub lineage: SheetLineage,
    /// The staged-review (commit) sheet payload.
    pub sheet: StagedReviewSheet,
    /// Headline invariant block.
    pub integrity: SheetIntegrity,
    /// Verification-proof block.
    pub verification: SheetVerification,
    /// Consumer surfaces that render this record.
    pub renderings: Vec<SheetRendering>,
}

/// The re-derived sheet decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SheetDecision {
    /// The headline claim the sheet is eligible to make.
    pub claimed_claim: SheetClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_claim: SheetClaim,
    /// Ordered, de-duplicated reasons the sheet fails to hold its headline.
    pub active_narrowing_reasons: Vec<SheetNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl SheetDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<SheetNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this record would overclaim.
    pub fn surface_overclaims(&self, rendered: SheetClaim) -> bool {
        self.effective_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(claimed: SheetClaim, reasons: &[SheetNarrowingReason]) -> SheetClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        SheetClaim::Unsafe
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, SheetClaim::ReviewOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we
        // can no longer certify even the read-only review, so it floors.
        SheetClaim::Unsafe
    } else {
        SheetClaim::Narrowed
    }
}

impl ReviewSheetRecord {
    /// Whether this sheet is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this sheet is an inherently read-only review overlay.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin.is_overlay()
    }

    /// Whether the commit is consequential enough that its commit action must name
    /// the scope and effect: every flow in this packet mutates remote/provider/
    /// admin/package/request/import state, so the answer is always yes.
    pub fn requires_specific_commit(&self) -> bool {
        let s = &self.sheet;
        s.scope.scope_kind.is_multi_object()
            || s.recoverability.recoverability_class.is_destructive()
            || s.side_effects.iter().any(|e| {
                matches!(
                    e.effect_class,
                    SideEffectClass::IrreversibleConfirmed | SideEffectClass::ExternalPublish
                )
            })
            || self.origin.is_provider_or_remote()
            || self.flow.is_provider_or_remote()
    }

    /// Whether this commit needs an export/backup path before commit.
    fn needs_export_path(&self) -> bool {
        let s = &self.sheet;
        s.recoverability.recoverability_class.requires_export_path()
            || s.side_effects
                .iter()
                .any(|e| e.effect_class.requires_export_path())
    }

    /// The headline claim this sheet is eligible to make.
    pub fn claimed_claim(&self) -> SheetClaim {
        if self.is_labs() {
            SheetClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            SheetClaim::ReviewOverlay
        } else {
            SheetClaim::Certified
        }
    }

    /// Reasons that hold independently of how the consumer surfaces render — the
    /// intrinsic scope/count/side-effect/recovery gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<SheetNarrowingReason> {
        use SheetNarrowingReason as R;
        let s = &self.sheet;
        let integ = &self.integrity;
        let scope_kind = s.scope.scope_kind;
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<R> = Vec::new();

        // Target scope.
        if !s.scope.scope_declared || !integ.target_scope_visible {
            reasons.push(R::TargetScopeHidden);
        }

        // Member counts.
        if !s.counts.reconciles() {
            reasons.push(R::MemberCountsInconsistent);
        }
        let has_hidden_member = s
            .members
            .iter()
            .any(|m| matches!(m.member_class, ReviewMemberClass::HiddenCollapsed));
        if scope_kind.can_hide_members() && has_hidden_member && s.counts.hidden == 0 {
            reasons.push(R::HiddenMembersUncounted);
        }
        if scope_kind.is_multi_object() && (!s.counts.counts_visible || !integ.counts_visible) {
            reasons.push(R::IncludedExcludedBlockedCountsHidden);
        }

        // Omitted defaults.
        if !s.omitted_defaults_disclosed || !integ.omitted_defaults_visible {
            reasons.push(R::OmittedDefaultsHidden);
        }

        // Side effects.
        if s.side_effects.iter().any(|e| !e.disclosed_before_commit)
            || !s.side_effects_disclosed
            || !integ.side_effects_disclosed
        {
            reasons.push(R::SideEffectUndisclosed);
        }

        // Blocked prerequisites.
        let blocked_member_unlabeled = s.members.iter().any(|m| {
            matches!(m.member_class, ReviewMemberClass::BlockedPrerequisite) && !m.reason_labeled
        });
        if blocked_member_unlabeled || !integ.blocked_prereqs_explained {
            reasons.push(R::BlockedPrereqHidden);
        }

        // Rollback / export consequence.
        let recovery_path_present =
            s.recoverability.rollback_path_present || s.recoverability.export_path_present;
        if !recovery_path_present
            || (self.needs_export_path() && !s.recoverability.export_path_present)
            || !integ.rollback_visible
        {
            reasons.push(R::RollbackConsequencesHidden);
        }

        // Commit action.
        if self.requires_specific_commit()
            && (!s.commit.commit_action_is_specific || !integ.commit_action_specific)
        {
            reasons.push(R::GenericContinueAction);
        }

        // Member class labelling (non-floor).
        let non_blocked_member_unlabeled = s.members.iter().any(|m| {
            !matches!(m.member_class, ReviewMemberClass::BlockedPrerequisite) && !m.reason_labeled
        });
        if !s.members_classes_labeled
            || !integ.member_classes_labeled
            || non_blocked_member_unlabeled
        {
            reasons.push(R::MemberClassesUnlabeled);
        }

        // Side-effect summary (non-floor).
        if !s.side_effect_summary_labeled {
            reasons.push(R::SideEffectSummaryUnlabeled);
        }

        // Cancel action (non-floor).
        if !s.commit.cancel_action_is_specific {
            reasons.push(R::CancelActionUnlabeled);
        }

        // Recoverability label (non-floor).
        if !s.recoverability.recoverability_class_labeled || !integ.recoverability_labeled {
            reasons.push(R::RecoverabilityClassUnlabeled);
        }

        // Scope freshness.
        if !integ.freshness_state_visible {
            reasons.push(R::FreshnessUnlabeled);
        }
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::SheetBackingMissing),
            FreshnessState::SupersededByNewerSource if !integ.superseded_state_marked => {
                reasons.push(R::SupersededScopeNotMarked);
            }
            FreshnessState::StaleExpired if !overlay => reasons.push(R::ScopeStale),
            _ => {}
        }

        // Verification proof.
        match self.verification.proof_currency {
            ProofCurrency::MissingProof => reasons.push(R::VerificationProofMissing),
            ProofCurrency::StaleExpired | ProofCurrency::RequiresReview => {
                reasons.push(R::VerificationProofStale);
            }
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow if stale_window => {
                reasons.push(R::VerificationProofStale);
            }
            _ => {}
        }

        // Overlay read-only.
        if overlay && !integ.imported_review_read_only {
            reasons.push(R::ImportedReviewReadsAsApply);
        }

        // Reopen-to-scope.
        if !integ.reopen_visible_on_demand || self.renderings.iter().any(|r| !r.scope_visible) {
            reasons.push(R::ReopenPathLost);
        }
        if matches!(
            self.declared_reopen_target,
            ReopenTarget::NoneKeyboardFallback
        ) {
            reasons.push(R::ReopenPathLost);
        }

        reasons
    }

    /// All active narrowing reasons, including the rendering-surface overclaim
    /// check, ordered and de-duplicated.
    fn reasons(&self, stale_window: bool) -> Vec<SheetNarrowingReason> {
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic_effective = derive_effective(self.claimed_claim(), &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic_effective.overclaims_as(r.rendered_claim))
        {
            reasons.push(SheetNarrowingReason::SheetOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive this sheet's claim decision.
    pub fn narrow(&self, stale_window: bool) -> SheetDecision {
        let claimed = self.claimed_claim();
        if matches!(claimed, SheetClaim::LabsNotClaimed) {
            return SheetDecision {
                claimed_claim: claimed,
                effective_claim: claimed,
                active_narrowing_reasons: Vec::new(),
                narrowed: false,
            };
        }
        let reasons = self.reasons(stale_window);
        let effective = derive_effective(claimed, &reasons);
        let narrowed = match (effective.rank(), claimed.rank()) {
            (Some(e), Some(c)) => e < c,
            _ => false,
        };
        SheetDecision {
            claimed_claim: claimed,
            effective_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// Whether a floored sheet still keeps a reopen/keyboard recovery fallback
    /// rather than a misleading clean commit.
    pub fn floored_keeps_fallback(&self, effective: SheetClaim) -> bool {
        if !matches!(effective, SheetClaim::Unsafe) {
            return true;
        }
        matches!(
            self.declared_reopen_target,
            ReopenTarget::ScopeOnly | ReopenTarget::NoneKeyboardFallback
        ) || opt_present(&self.lineage.reopen_backlink_ref)
    }

    /// Whether any rendering overclaims relative to `effective`.
    pub fn surface_overclaims(&self, effective: SheetClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// A reviewer-facing label for a narrowed/floored sheet, or `None` if the sheet
    /// holds its claim.
    pub fn narrowed_label(&self, decision: &SheetDecision) -> Option<String> {
        let trigger = decision.downgrade_trigger()?;
        Some(match decision.effective_claim {
            SheetClaim::Unsafe => format!(
                "Floored to sheet_unsafe below the {} claim: {}; falls back to an explicit blocked state with reopen/keyboard recovery.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            SheetClaim::Narrowed => format!(
                "Held at sheet_narrowed below the {} claim: {}; the scope stays reopenable until re-verified.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            _ => return None,
        })
    }

    /// Append per-sheet structural violations (schema-shape level).
    fn structural_violations(&self, out: &mut Vec<M5StagedReviewViolation>) {
        use M5StagedReviewViolation as V;
        if self.sheet_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.lineage.session_ref.trim().is_empty()
        {
            out.push(V::SheetMissingIdentity);
        }
        if self.is_overlay_origin()
            && !opt_present(&self.lineage.provider_ref)
            && !opt_present(&self.lineage.source_artifact_ref)
        {
            out.push(V::OverlayMissingProvenanceRef);
        }
        if self.sheet.members.is_empty() {
            out.push(V::SheetMissingMembers);
        }
        if self.renderings.is_empty() {
            out.push(V::SheetMissingRendering);
        }
        for r in &self.renderings {
            if r.source_sheet_ref.trim().is_empty() {
                out.push(V::RenderingMissingSourceRef);
            }
        }
    }
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for [`M5StagedReviewSheetSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedReviewSheetSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-sheet rows.
    pub sheets: Vec<ReviewSheetRecord>,
}

/// Export-safe M5 staged-review sheet set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedReviewSheetSetPacket {
    /// Record kind; must equal [`M5_STAGED_REVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STAGED_REVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_STAGED_REVIEW_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-sheet rows.
    pub sheets: Vec<ReviewSheetRecord>,
}

/// The distribution of effective sheet claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SheetClaimDistribution {
    /// Sheets effective at [`SheetClaim::Certified`].
    pub certified: usize,
    /// Sheets effective at [`SheetClaim::Narrowed`].
    pub narrowed: usize,
    /// Sheets effective at [`SheetClaim::ReviewOverlay`].
    pub overlay: usize,
    /// Sheets effective at [`SheetClaim::Unsafe`].
    pub unsafe_sheets: usize,
    /// Sheets effective at [`SheetClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5StagedReviewSheetSetPacket {
    /// Builds a staged-review sheet set packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5StagedReviewSheetSetInput) -> Self {
        Self {
            record_kind: M5_STAGED_REVIEW_RECORD_KIND.to_owned(),
            schema_version: M5_STAGED_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_STAGED_REVIEW_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            sheets: input.sheets,
        }
    }

    /// Whether the verification window has elapsed by `as_of`.
    pub fn freshness_stale_at(&self, as_of: &str) -> bool {
        if !self.verification_freshness.auto_downgrade_on_stale {
            return false;
        }
        let last =
            parse_rfc3339_to_epoch_seconds(&self.verification_freshness.last_verification_refresh);
        let now = parse_rfc3339_to_epoch_seconds(as_of);
        match (last, now) {
            (Some(last), Some(now)) => {
                now - last
                    > i64::from(self.verification_freshness.verification_freshness_slo_hours) * 3600
            }
            _ => false,
        }
    }

    /// Whether the window has elapsed by the packet's own `as_of`.
    pub fn stale_window(&self) -> bool {
        self.freshness_stale_at(&self.as_of)
    }

    /// Re-derive the decision for every sheet, paired with its id.
    pub fn decisions(&self) -> Vec<(String, SheetDecision)> {
        let stale_window = self.stale_window();
        self.sheets
            .iter()
            .map(|s| (s.sheet_id.clone(), s.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective sheet claims.
    pub fn claim_distribution(&self) -> SheetClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = SheetClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            unsafe_sheets: 0,
            labs: 0,
        };
        for s in &self.sheets {
            match s.narrow(stale_window).effective_claim {
                SheetClaim::Certified => dist.certified += 1,
                SheetClaim::Narrowed => dist.narrowed += 1,
                SheetClaim::ReviewOverlay => dist.overlay += 1,
                SheetClaim::Unsafe => dist.unsafe_sheets += 1,
                SheetClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of sheets whose effective claim ranks below their claimed claim.
    pub fn narrowed_sheet_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.sheets
            .iter()
            .filter(|s| s.narrow(stale_window).narrowed)
            .count()
    }

    /// Mutation flows represented by some sheet.
    pub fn represented_flows(&self) -> BTreeSet<MutationFlow> {
        self.sheets.iter().map(|s| s.flow).collect()
    }

    /// Product lanes represented by some sheet.
    pub fn represented_lanes(&self) -> BTreeSet<FlowLane> {
        self.sheets.iter().map(|s| s.lane).collect()
    }

    /// Scope kinds represented by some sheet.
    pub fn represented_scope_kinds(&self) -> BTreeSet<ScopeKind> {
        self.sheets
            .iter()
            .map(|s| s.sheet.scope.scope_kind)
            .collect()
    }

    /// Member classes represented by some member.
    pub fn represented_member_classes(&self) -> BTreeSet<ReviewMemberClass> {
        self.sheets
            .iter()
            .flat_map(|s| s.sheet.members.iter().map(|m| m.member_class))
            .collect()
    }

    /// Side-effect classes represented by some side effect.
    pub fn represented_side_effect_classes(&self) -> BTreeSet<SideEffectClass> {
        self.sheets
            .iter()
            .flat_map(|s| s.sheet.side_effects.iter().map(|e| e.effect_class))
            .collect()
    }

    /// Consumer surfaces represented by some rendering.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<ConsumerSurface> {
        self.sheets
            .iter()
            .flat_map(|s| s.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Validate the staged-review-sheet invariants.
    pub fn validate(&self) -> Vec<M5StagedReviewViolation> {
        use M5StagedReviewViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_STAGED_REVIEW_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_STAGED_REVIEW_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_STAGED_REVIEW_TAXONOMY_VERSION {
            violations.push(V::WrongTaxonomyVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(V::MissingIdentity);
        }
        if !REDACTION_CLASS_TOKENS.contains(&self.redaction_class_token.as_str()) {
            violations.push(V::InvalidRedactionClass);
        }
        if self.verification_freshness.verification_freshness_slo_hours == 0
            || self
                .verification_freshness
                .last_verification_refresh
                .trim()
                .is_empty()
        {
            violations.push(V::EvidenceFreshnessIncomplete);
        }
        if self.sheets.is_empty() {
            violations.push(V::EmptySheets);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for s in &self.sheets {
            if !seen.insert(s.sheet_id.as_str()) {
                violations.push(V::DuplicateSheetId);
            }
        }

        if MutationFlow::ALL
            .iter()
            .any(|f| !self.represented_flows().contains(f))
        {
            violations.push(V::MutationFlowMissing);
        }
        if FlowLane::ALL
            .iter()
            .any(|l| !self.represented_lanes().contains(l))
        {
            violations.push(V::FlowLaneMissing);
        }
        if ScopeKind::ALL
            .iter()
            .any(|k| !self.represented_scope_kinds().contains(k))
        {
            violations.push(V::ScopeKindMissing);
        }
        if ReviewMemberClass::ALL
            .iter()
            .any(|m| !self.represented_member_classes().contains(m))
        {
            violations.push(V::MemberClassMissing);
        }
        if SideEffectClass::ALL
            .iter()
            .any(|e| !self.represented_side_effect_classes().contains(e))
        {
            violations.push(V::SideEffectClassMissing);
        }
        if ConsumerSurface::ALL
            .iter()
            .any(|s| !self.represented_consumer_surfaces().contains(s))
        {
            violations.push(V::ConsumerSurfaceMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for s in &self.sheets {
            s.structural_violations(&mut violations);
            let decision = s.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || s.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedSheetMissingLabelOrTrigger);
                }
            }
            if !s.floored_keeps_fallback(decision.effective_claim) {
                violations.push(V::FlooredSheetLosesFallback);
            }
            if s.surface_overclaims(decision.effective_claim) {
                violations.push(V::RenderingSheetOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedSheetCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("staged-review packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("staged-review packet serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_report(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Staged-Review Sheets Across Mutation Flows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Sheets: {}\n", self.sheets.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} review overlay, {} unsafe, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.unsafe_sheets, dist.labs
        ));

        out.push_str("| Sheet | Flow | Lane | Scope | Origin | Claimed | Effective |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for s in &self.sheets {
            let decision = s.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                s.sheet_id,
                s.flow.as_str(),
                s.lane.as_str(),
                s.sheet.scope.scope_kind.as_str(),
                s.origin.as_str(),
                decision.claimed_claim.as_str(),
                decision.effective_claim.as_str(),
            ));
        }

        out.push('\n');
        for s in &self.sheets {
            let decision = s.narrow(stale_window);
            if let Some(label) = s.narrowed_label(&decision) {
                out.push_str(&format!("- {}: {}\n", s.sheet_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum M5StagedReviewArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5StagedReviewViolation>),
}

impl fmt::Display for M5StagedReviewArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(f, "support export parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for M5StagedReviewArtifactError {}

/// A staged-review-sheet packet validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StagedReviewViolation {
    /// `record_kind` is not the expected tag.
    WrongRecordKind,
    /// `schema_version` is not the expected version.
    WrongSchemaVersion,
    /// `taxonomy_version` is not the expected version.
    WrongTaxonomyVersion,
    /// A required header identity field is empty.
    MissingIdentity,
    /// The redaction-class token is not recognized.
    InvalidRedactionClass,
    /// The evidence freshness window is incomplete.
    EvidenceFreshnessIncomplete,
    /// The set has no sheets.
    EmptySheets,
    /// Two sheets share a sheet id.
    DuplicateSheetId,
    /// A mutation flow is unrepresented.
    MutationFlowMissing,
    /// A product lane is unrepresented.
    FlowLaneMissing,
    /// A scope kind is unrepresented.
    ScopeKindMissing,
    /// A member class is unrepresented.
    MemberClassMissing,
    /// A side-effect class is unrepresented.
    SideEffectClassMissing,
    /// A consumer surface is unrepresented.
    ConsumerSurfaceMissing,
    /// A sheet lacks a required identity field.
    SheetMissingIdentity,
    /// An overlay sheet names no provider/source-artifact ref.
    OverlayMissingProvenanceRef,
    /// A sheet has no members.
    SheetMissingMembers,
    /// A sheet has no renderings.
    SheetMissingRendering,
    /// A rendering names no source sheet ref.
    RenderingMissingSourceRef,
    /// A narrowed sheet lacks a non-generic label or a downgrade trigger.
    NarrowedSheetMissingLabelOrTrigger,
    /// A floored sheet loses its reopen/keyboard fallback.
    FlooredSheetLosesFallback,
    /// A rendering surface renders wider than the effective claim.
    RenderingSheetOverclaims,
    /// No sheet demonstrates the auto-narrowing rule.
    DowngradedSheetCaseMissing,
    /// Raw boundary material crossed the export.
    RawBoundaryMaterialInExport,
}

impl M5StagedReviewViolation {
    /// Stable token recorded in diagnostics and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptySheets => "empty_sheets",
            Self::DuplicateSheetId => "duplicate_sheet_id",
            Self::MutationFlowMissing => "mutation_flow_missing",
            Self::FlowLaneMissing => "flow_lane_missing",
            Self::ScopeKindMissing => "scope_kind_missing",
            Self::MemberClassMissing => "member_class_missing",
            Self::SideEffectClassMissing => "side_effect_class_missing",
            Self::ConsumerSurfaceMissing => "consumer_surface_missing",
            Self::SheetMissingIdentity => "sheet_missing_identity",
            Self::OverlayMissingProvenanceRef => "overlay_missing_provenance_ref",
            Self::SheetMissingMembers => "sheet_missing_members",
            Self::SheetMissingRendering => "sheet_missing_rendering",
            Self::RenderingMissingSourceRef => "rendering_missing_source_ref",
            Self::NarrowedSheetMissingLabelOrTrigger => "narrowed_sheet_missing_label_or_trigger",
            Self::FlooredSheetLosesFallback => "floored_sheet_loses_fallback",
            Self::RenderingSheetOverclaims => "rendering_sheet_overclaims",
            Self::DowngradedSheetCaseMissing => "downgraded_sheet_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// --------------------------------------------------------------------------- //
// Canonical artifact loader.
// --------------------------------------------------------------------------- //

/// Loads and validates the checked-in canonical support export.
///
/// This is the canonical entry point downstream marketplace, request, support,
/// admin, import, settings, and project surfaces use to ingest the frozen
/// staged-review sheet matrix instead of minting per-feature commit-sheet
/// semantics.
///
/// # Errors
///
/// Returns [`M5StagedReviewArtifactError`] when the artifact cannot be parsed or
/// fails validation.
pub fn current_m5_staged_review_sheet_set(
) -> Result<M5StagedReviewSheetSetPacket, M5StagedReviewArtifactError> {
    let packet: M5StagedReviewSheetSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ux/m5-staged-review-sheets/support_export.json"
    )))
    .map_err(M5StagedReviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5StagedReviewArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded staged-review sheet set: the in-crate source of truth the
/// checked-in support export and report are regenerated from.
pub fn seeded_m5_staged_review_sheet_set() -> M5StagedReviewSheetSetPacket {
    M5StagedReviewSheetSetPacket::new(M5StagedReviewSheetSetInput {
        packet_id: M5_STAGED_REVIEW_PACKET_ID.to_owned(),
        label:
            "M5 staged-review sheets — target scope, omitted defaults, side effects, and included/excluded/blocked/hidden counts across mutation flows"
                .to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        sheets: seed_sheets(),
    })
}

/// Renderings that show `claim` cleanly across the named surfaces.
fn renderings(
    source_ref: &str,
    claim: SheetClaim,
    surfaces: &[ConsumerSurface],
    read_only: bool,
) -> Vec<SheetRendering> {
    surfaces
        .iter()
        .map(|&surface| SheetRendering {
            surface,
            rendered_claim: claim,
            scope_visible: true,
            read_only,
            source_sheet_ref: source_ref.to_owned(),
        })
        .collect()
}

/// A clean first-party integrity block.
fn clean_integrity() -> SheetIntegrity {
    SheetIntegrity {
        target_scope_visible: true,
        counts_visible: true,
        omitted_defaults_visible: true,
        side_effects_disclosed: true,
        blocked_prereqs_explained: true,
        rollback_visible: true,
        commit_action_specific: true,
        imported_review_read_only: true,
        member_classes_labeled: true,
        recoverability_labeled: true,
        freshness_state_visible: true,
        superseded_state_marked: true,
        reopen_visible_on_demand: true,
    }
}

/// A verified-current verification block.
fn verified(proof_ref: &str) -> SheetVerification {
    SheetVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

fn member(member_id: &str, member_class: ReviewMemberClass, label: &str) -> ReviewMember {
    ReviewMember {
        member_id: member_id.to_owned(),
        member_class,
        reason_labeled: true,
        label_summary: label.to_owned(),
    }
}

fn side_effect(
    effect_id: &str,
    effect_class: SideEffectClass,
    reversible: bool,
    label: &str,
) -> SideEffectDescriptor {
    SideEffectDescriptor {
        effect_id: effect_id.to_owned(),
        effect_class,
        disclosed_before_commit: true,
        reversible,
        label_summary: label.to_owned(),
    }
}

fn commit(commit_label: &str, cancel_label: &str) -> CommitAction {
    CommitAction {
        commit_action_is_specific: true,
        commit_action_label: commit_label.to_owned(),
        cancel_action_is_specific: true,
        cancel_action_label: cancel_label.to_owned(),
    }
}

/// The canonical sheets: one per mutation flow, covering every scope kind, member
/// class, side-effect class, and consumer surface, plus a narrowed first-party
/// sheet, a review overlay, and a Labs sheet.
fn seed_sheets() -> Vec<ReviewSheetRecord> {
    use ConsumerSurface as CS;

    // 1. Provider publish-later: a single provider object, external publish,
    //    reversible only via export. Certified.
    let provider = ReviewSheetRecord {
        sheet_id: "sheet:provider-publish-later:0001".to_owned(),
        flow: MutationFlow::ProviderPublishLater,
        lane: FlowLane::Provider,
        origin: SheetOrigin::ProviderCommit,
        label_summary: "Publish the staged provider profile: one connection, external publish to the provider, reversible only after exporting the prior profile.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SheetAndScope,
        lineage: SheetLineage {
            session_ref: "session:provider-publish:0001".to_owned(),
            canonical_sheet_ref: None,
            target_ref: Some("target:provider-profile:staged".to_owned()),
            provider_ref: Some("provider:registered:0001".to_owned()),
            source_artifact_ref: None,
            rollback_plan_ref: Some("rollback:provider-profile:0001".to_owned()),
            export_bundle_ref: Some("export:provider-profile:0001".to_owned()),
            reopen_backlink_ref: Some("reopen:provider-form:0001".to_owned()),
        },
        sheet: StagedReviewSheet {
            scope: TargetScope {
                scope_kind: ScopeKind::SingleObject,
                scope_declared: true,
                scope_label: "Target: the registered provider connection 'primary'.".to_owned(),
            },
            omitted_defaults_disclosed: true,
            omitted_default_count: 2,
            counts: MemberCounts {
                included: 1,
                excluded: 0,
                blocked: 0,
                hidden: 0,
                total_matched: 1,
                counts_visible: true,
            },
            members: vec![member(
                "member:provider-profile",
                ReviewMemberClass::Included,
                "Included: the staged provider profile is published.",
            )],
            members_classes_labeled: true,
            side_effects: vec![
                side_effect(
                    "effect:publish",
                    SideEffectClass::ExternalPublish,
                    false,
                    "Publishes the profile to the provider; cannot be un-published from here.",
                ),
                side_effect(
                    "effect:supersede-prior",
                    SideEffectClass::ReversibleWithExport,
                    true,
                    "Supersedes the prior profile; reversible if you export it first.",
                ),
            ],
            side_effects_disclosed: true,
            side_effect_summary_labeled: true,
            recoverability: Recoverability {
                recoverability_class: RecoverabilityClass::ReversibleViaExport,
                recoverability_class_labeled: true,
                rollback_path_present: true,
                export_path_present: true,
                recovery_label: "Reversible only after exporting the prior profile.".to_owned(),
            },
            commit: commit(
                "Publish provider profile",
                "Keep staged, don't publish",
            ),
        },
        integrity: clean_integrity(),
        verification: verified("proof:provider-publish:0001"),
        renderings: renderings(
            "sheet:provider-publish-later:0001",
            SheetClaim::Certified,
            &[CS::ReviewSheet, CS::CliConfirmation, CS::AiEvidence],
            false,
        ),
    };

    // 2. Settings bulk apply: explicit multi-object selection, fully reversible
    //    local. The clean baseline. Certified.
    let settings = ReviewSheetRecord {
        sheet_id: "sheet:settings-bulk-apply:0001".to_owned(),
        flow: MutationFlow::SettingsBulkApply,
        lane: FlowLane::Settings,
        origin: SheetOrigin::LocalCommit,
        label_summary: "Apply the selected configuration changes: an explicit set of settings, fully reversible from local history.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SheetAndScope,
        lineage: SheetLineage {
            session_ref: "session:settings-apply:0001".to_owned(),
            canonical_sheet_ref: None,
            target_ref: Some("target:workspace-settings".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            rollback_plan_ref: Some("rollback:settings:0001".to_owned()),
            export_bundle_ref: None,
            reopen_backlink_ref: Some("reopen:settings-editor:0001".to_owned()),
        },
        sheet: StagedReviewSheet {
            scope: TargetScope {
                scope_kind: ScopeKind::MultiObjectExplicit,
                scope_declared: true,
                scope_label: "Target: 4 explicitly selected settings.".to_owned(),
            },
            omitted_defaults_disclosed: true,
            omitted_default_count: 3,
            counts: MemberCounts {
                included: 2,
                excluded: 2,
                blocked: 0,
                hidden: 0,
                total_matched: 4,
                counts_visible: true,
            },
            members: vec![
                member(
                    "member:theme",
                    ReviewMemberClass::Included,
                    "Included: switch theme to high-contrast.",
                ),
                member(
                    "member:font",
                    ReviewMemberClass::Included,
                    "Included: set editor font size to 14.",
                ),
                member(
                    "member:telemetry",
                    ReviewMemberClass::ExcludedByDefault,
                    "Excluded by default: telemetry stays at its current value.",
                ),
                member(
                    "member:keymap",
                    ReviewMemberClass::ExcludedByUser,
                    "Excluded by you: keymap change deselected.",
                ),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect(
                "effect:local-write",
                SideEffectClass::ReversibleLocal,
                true,
                "Writes to local workspace settings; reversible from settings history.",
            )],
            side_effects_disclosed: true,
            side_effect_summary_labeled: true,
            recoverability: Recoverability {
                recoverability_class: RecoverabilityClass::FullyReversible,
                recoverability_class_labeled: true,
                rollback_path_present: true,
                export_path_present: false,
                recovery_label: "Fully reversible from settings history.".to_owned(),
            },
            commit: commit("Apply 2 settings", "Cancel, keep current settings"),
        },
        integrity: clean_integrity(),
        verification: verified("proof:settings-apply:0001"),
        renderings: renderings(
            "sheet:settings-bulk-apply:0001",
            SheetClaim::Certified,
            &[CS::ReviewSheet, CS::BatchSelectionBar, CS::HelpInline],
            false,
        ),
    };

    // 3. Package lifecycle (install/update/remove): explicit multi-object set with a
    //    blocked prerequisite; partially reversible. Certified.
    let package = ReviewSheetRecord {
        sheet_id: "sheet:package-lifecycle:0001".to_owned(),
        flow: MutationFlow::PackageLifecycle,
        lane: FlowLane::Package,
        origin: SheetOrigin::LocalCommit,
        label_summary: "Install, update, and remove the selected packages: 3 proceed, 1 blocked by an unmet prerequisite, removal irreversible without an export.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SheetAndScope,
        lineage: SheetLineage {
            session_ref: "session:package-lifecycle:0001".to_owned(),
            canonical_sheet_ref: None,
            target_ref: Some("target:workspace-packages".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            rollback_plan_ref: Some("rollback:packages:0001".to_owned()),
            export_bundle_ref: Some("export:package-state:0001".to_owned()),
            reopen_backlink_ref: Some("reopen:package-manager:0001".to_owned()),
        },
        sheet: StagedReviewSheet {
            scope: TargetScope {
                scope_kind: ScopeKind::MultiObjectExplicit,
                scope_declared: true,
                scope_label: "Target: 4 selected packages (3 actionable, 1 blocked).".to_owned(),
            },
            omitted_defaults_disclosed: true,
            omitted_default_count: 1,
            counts: MemberCounts {
                included: 3,
                excluded: 0,
                blocked: 1,
                hidden: 0,
                total_matched: 4,
                counts_visible: true,
            },
            members: vec![
                member(
                    "member:pkg-install",
                    ReviewMemberClass::Included,
                    "Included: install package 'formatter' v2.1.",
                ),
                member(
                    "member:pkg-update",
                    ReviewMemberClass::Included,
                    "Included: update package 'linter' to v3.0.",
                ),
                member(
                    "member:pkg-remove",
                    ReviewMemberClass::Included,
                    "Included: remove package 'legacy-helper'.",
                ),
                member(
                    "member:pkg-blocked",
                    ReviewMemberClass::BlockedPrerequisite,
                    "Blocked: 'runtime-bridge' needs a newer host runtime first.",
                ),
            ],
            members_classes_labeled: true,
            side_effects: vec![
                side_effect(
                    "effect:remove",
                    SideEffectClass::IrreversibleConfirmed,
                    false,
                    "Removing 'legacy-helper' deletes its local state; export first to recover it.",
                ),
                side_effect(
                    "effect:install",
                    SideEffectClass::ReversibleWithExport,
                    true,
                    "Installs/updates packages; reversible by restoring the exported state.",
                ),
            ],
            side_effects_disclosed: true,
            side_effect_summary_labeled: true,
            recoverability: Recoverability {
                recoverability_class: RecoverabilityClass::PartiallyReversible,
                recoverability_class_labeled: true,
                rollback_path_present: true,
                export_path_present: true,
                recovery_label: "Installs/updates revert; removal needs the exported backup."
                    .to_owned(),
            },
            commit: commit(
                "Apply 3 package changes",
                "Cancel, change nothing",
            ),
        },
        integrity: clean_integrity(),
        verification: verified("proof:package-lifecycle:0001"),
        renderings: renderings(
            "sheet:package-lifecycle:0001",
            SheetClaim::Certified,
            &[CS::ReviewSheet, CS::DiagnosticsPanel, CS::SupportExport],
            false,
        ),
    };

    // 4. Admin source-management: a workspace-wide, policy-governed action with
    //    collapsed members covered by a hidden count. Certified.
    let admin = ReviewSheetRecord {
        sheet_id: "sheet:admin-source-management:0001".to_owned(),
        flow: MutationFlow::AdminSourceManagement,
        lane: FlowLane::Admin,
        origin: SheetOrigin::LocalCommit,
        label_summary: "Rotate the trust policy across every managed source: 12 shown, 230 collapsed into a hidden count, policy-governed and partially reversible.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::CachedSnapshot,
        declared_reopen_target: ReopenTarget::SheetAndScope,
        lineage: SheetLineage {
            session_ref: "session:admin-source:0001".to_owned(),
            canonical_sheet_ref: None,
            target_ref: Some("target:managed-sources:all".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            rollback_plan_ref: Some("rollback:source-policy:0001".to_owned()),
            export_bundle_ref: Some("export:source-policy:0001".to_owned()),
            reopen_backlink_ref: Some("reopen:admin-console:0001".to_owned()),
        },
        sheet: StagedReviewSheet {
            scope: TargetScope {
                scope_kind: ScopeKind::WorkspaceWide,
                scope_declared: true,
                scope_label: "Target: all 242 managed sources in the workspace.".to_owned(),
            },
            omitted_defaults_disclosed: true,
            omitted_default_count: 4,
            counts: MemberCounts {
                included: 12,
                excluded: 0,
                blocked: 0,
                hidden: 230,
                total_matched: 242,
                counts_visible: true,
            },
            members: vec![
                member(
                    "member:source-shown",
                    ReviewMemberClass::Included,
                    "Included: 12 sources shown individually rotate to the new policy.",
                ),
                member(
                    "member:source-hidden",
                    ReviewMemberClass::HiddenCollapsed,
                    "Hidden: 230 further sources collapsed into the count, same rotation.",
                ),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect(
                "effect:policy-rotate",
                SideEffectClass::PolicyGoverned,
                true,
                "Rotates trust policy under workspace governance; reversible within the audit window.",
            )],
            side_effects_disclosed: true,
            side_effect_summary_labeled: true,
            recoverability: Recoverability {
                recoverability_class: RecoverabilityClass::PartiallyReversible,
                recoverability_class_labeled: true,
                rollback_path_present: true,
                export_path_present: true,
                recovery_label: "Reversible within the audit window; export the prior policy first."
                    .to_owned(),
            },
            commit: commit(
                "Rotate policy on 242 sources",
                "Cancel, keep current policy",
            ),
        },
        integrity: clean_integrity(),
        verification: verified("proof:admin-source:0001"),
        renderings: renderings(
            "sheet:admin-source-management:0001",
            SheetClaim::Certified,
            &[CS::ReviewSheet, CS::BatchSelectionBar, CS::AiEvidence],
            false,
        ),
    };

    // 5. Request replay/mutation: a query-backed remote action with a hidden count;
    //    narrowed because the verification proof requires review.
    let request = ReviewSheetRecord {
        sheet_id: "sheet:request-replay:0001".to_owned(),
        flow: MutationFlow::RequestReplayMutation,
        lane: FlowLane::Request,
        origin: SheetOrigin::RemoteCommit,
        label_summary: "Replay the matched requests against the live endpoint: 8 shown, 3 collapsed, irreversible re-issue, export the response log first.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SheetAndScope,
        lineage: SheetLineage {
            session_ref: "session:request-replay:0001".to_owned(),
            canonical_sheet_ref: None,
            target_ref: Some("target:request-endpoint:live".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            rollback_plan_ref: None,
            export_bundle_ref: Some("export:response-log:0001".to_owned()),
            reopen_backlink_ref: Some("reopen:request-workspace:0001".to_owned()),
        },
        sheet: StagedReviewSheet {
            scope: TargetScope {
                scope_kind: ScopeKind::QueryBacked,
                scope_declared: true,
                scope_label: "Target: 11 requests matching the current filter.".to_owned(),
            },
            omitted_defaults_disclosed: true,
            omitted_default_count: 2,
            counts: MemberCounts {
                included: 8,
                excluded: 0,
                blocked: 0,
                hidden: 3,
                total_matched: 11,
                counts_visible: true,
            },
            members: vec![
                member(
                    "member:req-shown",
                    ReviewMemberClass::Included,
                    "Included: 8 requests shown individually are replayed.",
                ),
                member(
                    "member:req-hidden",
                    ReviewMemberClass::HiddenCollapsed,
                    "Hidden: 3 further matched requests collapsed into the count, also replayed.",
                ),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect(
                "effect:reissue",
                SideEffectClass::ExternalPublish,
                false,
                "Re-issues the requests to the live endpoint; sent requests cannot be recalled.",
            )],
            side_effects_disclosed: true,
            side_effect_summary_labeled: true,
            recoverability: Recoverability {
                recoverability_class: RecoverabilityClass::Irreversible,
                recoverability_class_labeled: true,
                rollback_path_present: false,
                export_path_present: true,
                recovery_label: "Irreversible once sent; export the response log to keep a record."
                    .to_owned(),
            },
            commit: commit(
                "Replay 8 requests now",
                "Cancel, send nothing",
            ),
        },
        integrity: clean_integrity(),
        verification: SheetVerification {
            proof_currency: ProofCurrency::RequiresReview,
            proof_ref: Some("proof:request-replay:0001".to_owned()),
        },
        renderings: renderings(
            "sheet:request-replay:0001",
            SheetClaim::Narrowed,
            &[CS::ReviewSheet, CS::CliConfirmation, CS::DiagnosticsPanel],
            false,
        ),
    };

    // 6. Import/export/publish: a review overlay of a migration bundle before
    //    publish; read-only, query-backed, with excluded defaults and a hidden
    //    count. Review overlay.
    let import = ReviewSheetRecord {
        sheet_id: "sheet:import-export-publish:0001".to_owned(),
        flow: MutationFlow::ImportExportPublish,
        lane: FlowLane::Import,
        origin: SheetOrigin::ImportedReview,
        label_summary: "Review the migration bundle before publishing it: 5 shown, 2 excluded as defaults, 40 collapsed; a read-only review, not a local apply.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::CachedSnapshot,
        declared_reopen_target: ReopenTarget::SheetAndScope,
        lineage: SheetLineage {
            session_ref: "session:import-publish:0001".to_owned(),
            canonical_sheet_ref: None,
            target_ref: Some("target:publish-channel".to_owned()),
            provider_ref: None,
            source_artifact_ref: Some("artifact:migration-bundle:0001".to_owned()),
            rollback_plan_ref: Some("rollback:publish:0001".to_owned()),
            export_bundle_ref: Some("export:migration-bundle:0001".to_owned()),
            reopen_backlink_ref: Some("reopen:migration-center:0001".to_owned()),
        },
        sheet: StagedReviewSheet {
            scope: TargetScope {
                scope_kind: ScopeKind::QueryBacked,
                scope_declared: true,
                scope_label: "Target: 47 entries in the migration bundle.".to_owned(),
            },
            omitted_defaults_disclosed: true,
            omitted_default_count: 6,
            counts: MemberCounts {
                included: 5,
                excluded: 2,
                blocked: 0,
                hidden: 40,
                total_matched: 47,
                counts_visible: true,
            },
            members: vec![
                member(
                    "member:entry-shown",
                    ReviewMemberClass::Included,
                    "Included: 5 entries shown individually are published.",
                ),
                member(
                    "member:entry-default",
                    ReviewMemberClass::ExcludedByDefault,
                    "Excluded by default: 2 entries already present at the target.",
                ),
                member(
                    "member:entry-hidden",
                    ReviewMemberClass::HiddenCollapsed,
                    "Hidden: 40 further entries collapsed into the count, also published.",
                ),
            ],
            members_classes_labeled: true,
            side_effects: vec![
                side_effect(
                    "effect:publish-bundle",
                    SideEffectClass::ExternalPublish,
                    false,
                    "Publishes the bundle to the channel; cannot be un-published from here.",
                ),
                side_effect(
                    "effect:bundle-supersede",
                    SideEffectClass::ReversibleWithExport,
                    true,
                    "Supersedes the prior bundle; reversible if you export it first.",
                ),
            ],
            side_effects_disclosed: true,
            side_effect_summary_labeled: true,
            recoverability: Recoverability {
                recoverability_class: RecoverabilityClass::ReversibleViaExport,
                recoverability_class_labeled: true,
                rollback_path_present: true,
                export_path_present: true,
                recovery_label: "Reversible only after exporting the prior bundle.".to_owned(),
            },
            commit: commit(
                "Publish reviewed bundle",
                "Close review, publish nothing",
            ),
        },
        integrity: clean_integrity(),
        verification: verified("proof:import-publish:0001"),
        renderings: renderings(
            "sheet:import-export-publish:0001",
            SheetClaim::ReviewOverlay,
            &[CS::ReviewSheet, CS::SupportExport, CS::HelpInline],
            true,
        ),
    };

    // 7. Labs experimental sheet: makes no public claim.
    let labs = ReviewSheetRecord {
        sheet_id: "sheet:experimental-quick-apply:0001".to_owned(),
        flow: MutationFlow::SettingsBulkApply,
        lane: FlowLane::Settings,
        origin: SheetOrigin::LocalCommit,
        label_summary:
            "Experimental quick-apply sheet (Labs): single object, fully reversible, unadvertised."
                .to_owned(),
        claim_posture: ClaimPosture::LabsUnadvertised,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SheetAndScope,
        lineage: SheetLineage {
            session_ref: "session:labs-quick-apply:0001".to_owned(),
            canonical_sheet_ref: None,
            target_ref: Some("target:workspace-settings".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            rollback_plan_ref: Some("rollback:labs:0001".to_owned()),
            export_bundle_ref: None,
            reopen_backlink_ref: Some("reopen:labs:0001".to_owned()),
        },
        sheet: StagedReviewSheet {
            scope: TargetScope {
                scope_kind: ScopeKind::SingleObject,
                scope_declared: true,
                scope_label: "Target: one experimental layout preference.".to_owned(),
            },
            omitted_defaults_disclosed: true,
            omitted_default_count: 0,
            counts: MemberCounts {
                included: 1,
                excluded: 0,
                blocked: 0,
                hidden: 0,
                total_matched: 1,
                counts_visible: true,
            },
            members: vec![member(
                "member:labs-layout",
                ReviewMemberClass::Included,
                "Included: apply the experimental layout preference.",
            )],
            members_classes_labeled: true,
            side_effects: vec![side_effect(
                "effect:labs-local",
                SideEffectClass::ReversibleLocal,
                true,
                "Writes a local preference; reversible immediately.",
            )],
            side_effects_disclosed: true,
            side_effect_summary_labeled: true,
            recoverability: Recoverability {
                recoverability_class: RecoverabilityClass::FullyReversible,
                recoverability_class_labeled: true,
                rollback_path_present: true,
                export_path_present: false,
                recovery_label: "Fully reversible.".to_owned(),
            },
            commit: commit("Apply experimental layout", "Cancel"),
        },
        integrity: clean_integrity(),
        verification: verified("proof:labs-quick-apply:0001"),
        renderings: renderings(
            "sheet:experimental-quick-apply:0001",
            SheetClaim::LabsNotClaimed,
            &[CS::ReviewSheet, CS::HelpInline],
            false,
        ),
    };

    vec![provider, settings, package, admin, request, import, labs]
}
