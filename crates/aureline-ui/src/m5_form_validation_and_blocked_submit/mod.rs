//! Canonical form-level validation, cross-field dependency explanation, and
//! machine-readable blocked-submit reason truth for mutation-capable forms.
//!
//! Where [the structured-input / staged-review
//! model](crate::m5_structured_input_and_staged_review) freezes the *per-surface*
//! honesty claim of a whole form and [the field/control-row
//! model](crate::m5_field_control_rows) freezes the *per-row* primitive a form is
//! built from, this model freezes the layer between them: how a form rolls field
//! validity up into a **form-level validation summary** without replacing the
//! field-level anchors, how it **explains cross-field dependencies** when one
//! choice narrows or invalidates another, and how it emits a **machine-readable
//! blocked-submit reason packet** that desktop, CLI/headless, support-export, and
//! docs/help surfaces can all reuse to explain the same failure state.
//!
//! Each [`FormValidationRecord`] binds a mutation-capable surface to:
//!
//! * its **field-level validation anchors** — each field declares its validation
//!   state, whether it is labelled, whether a blocking/warning state is anchored
//!   directly to the field with exact rule text, and whether it is rolled up into
//!   the form-level summary (so field and form validation stay *linked* rather
//!   than duplicated or contradictory);
//! * its **form-level validation summary** — blocked values, missing
//!   prerequisites, derived constraints, and submit blockers summarized for the
//!   whole form, with explicit guarantees that the summary is consistent with the
//!   fields and never *replaces* the field-level anchors;
//! * its **cross-field dependencies** — provider/account mapping, environment
//!   selection, package source/registry auth, import/export mode, and derived
//!   field constraints, each declaring the relation (narrows, invalidates,
//!   requires, mutually-exclusive) and whether it is explained before submit;
//! * its **blocked-submit reasons** — each carries a stable machine code, a
//!   blocker class, whether it blocks submit, whether it is explained before
//!   submit, and the consumer surfaces (desktop, CLI/headless, support export,
//!   docs/help) that can reuse it; and
//! * its **submit gate** — submit cannot be reachable while any blocked
//!   prerequisite or cross-field invalidation is active, and the commit action
//!   names the blocker rather than being a generic Continue.
//!
//! Each record re-derives a [`FormClaim`] ([`FormValidationRecord::narrow`]) so a
//! form can never read wider than its evidence: a form whose submit gate is open
//! while a prerequisite or invalidation blocks, whose form-level summary
//! contradicts or replaces the field-level anchors, whose cross-field
//! invalidation is hidden before submit, whose blocked-submit reason is not
//! machine-readable or not reusable across the machine consumer surfaces, whose
//! blocking validation is deferred to a banner instead of an exact field anchor,
//! that hides a derived constraint, lets an imported/restore review read as a
//! local submit, or renders wider than its effective claim floors to
//! [`FormClaim::Blocked`] and falls back to an explicit blocked state that names
//! the reason rather than a clean-but-false Continue. A labelled, recoverable gap
//! (a deferred non-blocking dependency, a missing resolution hint, pending async
//! validation, a stale backing source, a stale verification proof) holds a
//! first-party form at [`FormClaim::Narrowed`]; an imported/restore review sits at
//! [`FormClaim::ReviewOverlay`] and never claims a local submit; and a
//! Labs/unadvertised form makes no public claim.
//!
//! [`M5FormValidationSetPacket::validate`] confirms the matrix is well-formed and
//! honest: header/identity/redaction/freshness are present, every form lane,
//! dependency kind, dependency relation, blocker class, blocked-submit consumer,
//! and consumer surface is represented, overlay surfaces name their provenance, no
//! rendering surface overclaims, a floored surface keeps a fallback, at least one
//! surface demonstrates the auto-narrowing rule, and no raw credential/body
//! material crosses the export. Downstream settings, marketplace, request,
//! support, admin, import, and project surfaces ingest this packet rather than
//! minting per-feature blocked-submit semantics.
//!
//! No credential bodies, secret values, raw provider payloads, absolute paths, or
//! URLs ever cross this boundary; the packet carries only typed class tokens,
//! counts, booleans, opaque ids, stable machine codes, and redaction-aware
//! reviewable labels.
//!
//! The boundary schema is
//! [`schemas/ux/m5-form-validation-and-blocked-submit.schema.json`](../../../../schemas/ux/m5-form-validation-and-blocked-submit.schema.json).
//! The contract doc is
//! [`docs/ux/m5-form-validation-and-blocked-submit.md`](../../../../docs/ux/m5-form-validation-and-blocked-submit.md).
//! The canonical support export is
//! [`artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json`](../../../../artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/ux/m5-form-validation-and-blocked-submit/`](../../../../fixtures/ux/m5-form-validation-and-blocked-submit/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5FormValidationSetPacket`].
pub const M5_FORM_VALIDATION_RECORD_KIND: &str = "m5_form_validation_set_packet";

/// Schema version for the form-validation set.
pub const M5_FORM_VALIDATION_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_FORM_VALIDATION_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical form-validation set packet.
pub const M5_FORM_VALIDATION_PACKET_ID: &str = "m5-form-validation:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_FORM_VALIDATION_SCHEMA_REF: &str =
    "schemas/ux/m5-form-validation-and-blocked-submit.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_FORM_VALIDATION_DOC_REF: &str = "docs/ux/m5-form-validation-and-blocked-submit.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_FORM_VALIDATION_SUPPORT_EXPORT_REF: &str =
    "artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json";

/// Repo-relative path of the generated report.
pub const M5_FORM_VALIDATION_REPORT_REF: &str =
    "artifacts/ux/m5-form-validation-and-blocked-submit/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_FORM_VALIDATION_FIXTURE_DIR: &str =
    "fixtures/ux/m5-form-validation-and-blocked-submit";

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

/// The product lane that owns a form surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormLane {
    /// Provider/credential configuration.
    Provider,
    /// Administrative/governance surfaces.
    Admin,
    /// Request-workspace / API-request execution.
    Request,
    /// Package / install / dependency surfaces.
    Package,
    /// Import / migration-center surfaces.
    Import,
    /// Settings / preferences surfaces.
    Settings,
    /// Project bootstrap / scaffolding surfaces.
    Projects,
}

impl FormLane {
    /// Every product lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Provider,
        Self::Admin,
        Self::Request,
        Self::Package,
        Self::Import,
        Self::Settings,
        Self::Projects,
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
            Self::Projects => "projects",
        }
    }
}

/// How a form and its backing values originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormOrigin {
    /// First-party local authoring.
    LocalAuthoring,
    /// First-party authoring against a remote target.
    RemoteTarget,
    /// A provider-backed form.
    ProviderBacked,
    /// A review of imported/migrated/restored state (an overlay).
    ImportedOrRestore,
}

impl FormOrigin {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAuthoring => "local_authoring",
            Self::RemoteTarget => "remote_target",
            Self::ProviderBacked => "provider_backed",
            Self::ImportedOrRestore => "imported_or_restore",
        }
    }

    /// Whether this origin is an inherently read-only review overlay.
    pub const fn is_overlay(self) -> bool {
        matches!(self, Self::ImportedOrRestore)
    }
}

/// The kind of cross-field dependency, naming the highest-risk M5 cases where one
/// choice narrows or invalidates another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyKind {
    /// Choosing a provider narrows or invalidates the eligible accounts.
    ProviderAccountMapping,
    /// Selecting an environment narrows or invalidates downstream choices.
    EnvironmentSelection,
    /// A package source/registry choice requires or invalidates registry auth.
    PackageSourceRegistryAuth,
    /// An import/export mode choice narrows or invalidates other modes.
    ImportExportMode,
    /// A derived field constraint computed from one field constrains another.
    DerivedFieldConstraint,
}

impl DependencyKind {
    /// Every dependency kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderAccountMapping,
        Self::EnvironmentSelection,
        Self::PackageSourceRegistryAuth,
        Self::ImportExportMode,
        Self::DerivedFieldConstraint,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAccountMapping => "provider_account_mapping",
            Self::EnvironmentSelection => "environment_selection",
            Self::PackageSourceRegistryAuth => "package_source_registry_auth",
            Self::ImportExportMode => "import_export_mode",
            Self::DerivedFieldConstraint => "derived_field_constraint",
        }
    }
}

/// How a cross-field dependency relates its source field to its target field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyRelation {
    /// The source choice narrows the target's allowed values.
    Narrows,
    /// The source choice invalidates the current target value.
    Invalidates,
    /// The source choice requires the target to be set.
    Requires,
    /// The two fields are mutually exclusive.
    MutuallyExclusive,
}

impl DependencyRelation {
    /// Every dependency relation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Narrows,
        Self::Invalidates,
        Self::Requires,
        Self::MutuallyExclusive,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Narrows => "narrows",
            Self::Invalidates => "invalidates",
            Self::Requires => "requires",
            Self::MutuallyExclusive => "mutually_exclusive",
        }
    }
}

/// The class of a blocked-submit reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmitBlockerClass {
    /// A field is invalid.
    InvalidField,
    /// A prerequisite is missing.
    MissingPrerequisite,
    /// Two fields conflict.
    CrossFieldConflict,
    /// A policy lock is unresolved.
    UnresolvedPolicyLock,
    /// Validation is still pending.
    PendingValidation,
    /// A side effect has not been reviewed.
    UnreviewedSideEffect,
}

impl SubmitBlockerClass {
    /// Every blocker class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InvalidField,
        Self::MissingPrerequisite,
        Self::CrossFieldConflict,
        Self::UnresolvedPolicyLock,
        Self::PendingValidation,
        Self::UnreviewedSideEffect,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidField => "invalid_field",
            Self::MissingPrerequisite => "missing_prerequisite",
            Self::CrossFieldConflict => "cross_field_conflict",
            Self::UnresolvedPolicyLock => "unresolved_policy_lock",
            Self::PendingValidation => "pending_validation",
            Self::UnreviewedSideEffect => "unreviewed_side_effect",
        }
    }
}

/// A consumer surface that must be able to reuse a blocked-submit reason packet to
/// explain the same failure state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedSubmitConsumer {
    /// The desktop product surface.
    Desktop,
    /// The CLI / headless runner.
    CliHeadless,
    /// A support-export bundle.
    SupportExport,
    /// Docs / inline help.
    DocsHelp,
}

impl BlockedSubmitConsumer {
    /// Every blocked-submit consumer, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Desktop,
        Self::CliHeadless,
        Self::SupportExport,
        Self::DocsHelp,
    ];

    /// The machine consumers a blocking reason must remain reusable by: a blocker
    /// that a headless run or a support bundle cannot reproduce is not actually a
    /// reusable blocked-submit reason.
    pub const MACHINE: [Self; 2] = [Self::CliHeadless, Self::SupportExport];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::DocsHelp => "docs_help",
        }
    }
}

/// Field- or form-level validation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationState {
    /// Validated and valid.
    Valid,
    /// Async validation in flight.
    PendingAsync,
    /// Valid but with a warning.
    Warning,
    /// Invalid and blocking submit.
    InvalidBlocking,
    /// Not yet validated.
    NotValidated,
}

impl ValidationState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::PendingAsync => "pending_async",
            Self::Warning => "warning",
            Self::InvalidBlocking => "invalid_blocking",
            Self::NotValidated => "not_validated",
        }
    }

    /// Whether this state must anchor an exact rule directly on the field rather
    /// than defer to a form-level banner.
    pub const fn needs_field_anchor(self) -> bool {
        matches!(self, Self::InvalidBlocking | Self::Warning)
    }
}

/// The freshness of a form's backing data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Live backing data.
    Live,
    /// A cached snapshot.
    CachedSnapshot,
    /// A stale/expired backing source.
    StaleExpired,
    /// Superseded by a newer source.
    SupersededByNewerSource,
    /// Backing data is missing.
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

/// The currency of a form's verification proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofCurrency {
    /// Verified within the freshness window.
    VerifiedCurrent,
    /// Cached within the freshness window.
    CachedWithinWindow,
    /// Stale/expired proof.
    StaleExpired,
    /// Proof requires review.
    RequiresReview,
    /// No proof is present.
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

/// The presentation a floored form drops its submit control to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedSubmitFallback {
    /// The submit action shows the blocked-submit reason in place.
    ShowsReasonOnSubmit,
    /// The submit action is disabled with a resolution hint.
    DisabledWithHint,
    /// Submit is silently dead with no reason (a contract violation).
    NoneSilent,
}

impl BlockedSubmitFallback {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShowsReasonOnSubmit => "shows_reason_on_submit",
            Self::DisabledWithHint => "disabled_with_hint",
            Self::NoneSilent => "none_silent",
        }
    }

    /// Whether this fallback keeps the floored reason visible to the user.
    pub const fn keeps_reason(self) -> bool {
        matches!(self, Self::ShowsReasonOnSubmit | Self::DisabledWithHint)
    }
}

/// Whether a form is publicly claimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPosture {
    /// Publicly claimed as stable.
    ClaimedStable,
    /// Labs/unadvertised; makes no public claim.
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

/// A consumer surface that renders a form-validation record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// The form/editor view itself.
    FormView,
    /// A wizard step.
    WizardStep,
    /// A staged-review sheet.
    ReviewSheet,
    /// The diagnostics panel.
    DiagnosticsPanel,
    /// A support export bundle.
    SupportExport,
    /// An AI-evidence consumer.
    AiEvidence,
    /// Inline help.
    HelpInline,
}

impl ConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FormView,
        Self::WizardStep,
        Self::ReviewSheet,
        Self::DiagnosticsPanel,
        Self::SupportExport,
        Self::AiEvidence,
        Self::HelpInline,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FormView => "form_view",
            Self::WizardStep => "wizard_step",
            Self::ReviewSheet => "review_sheet",
            Self::DiagnosticsPanel => "diagnostics_panel",
            Self::SupportExport => "support_export",
            Self::AiEvidence => "ai_evidence",
            Self::HelpInline => "help_inline",
        }
    }
}

// --------------------------------------------------------------------------- //
// Derived claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim a form's validation surface renders. A higher rank asserts
/// more authority, so a narrowed or floored form must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormClaim {
    /// The form-validation contract is broken: submit would be reachable while a
    /// prerequisite or invalidation blocks, the form-level summary contradicts or
    /// replaces the field anchors, a cross-field invalidation is hidden, a
    /// blocked-submit reason is not machine-readable or reusable, or an
    /// imported/restore review reads as a local submit. It must fall back to an
    /// explicit blocked state that names the reason rather than a clean submit.
    #[serde(rename = "form_blocked")]
    Blocked,
    /// A review of imported/migrated/restored state: attributable and reopenable
    /// but never reads as a local submit.
    #[serde(rename = "form_review_overlay")]
    ReviewOverlay,
    /// A first-party form held below certified by a labelled, recoverable gap (a
    /// deferred non-blocking dependency, a missing resolution hint, pending async
    /// validation, a stale source, a stale proof).
    #[serde(rename = "form_narrowed")]
    Narrowed,
    /// Full field-linked, summary-honest, dependency-explained, blocked-submit
    /// reusable form-validation contract.
    #[serde(rename = "form_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "form_labs_not_claimed")]
    LabsNotClaimed,
}

impl FormClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "form_blocked",
            Self::ReviewOverlay => "form_review_overlay",
            Self::Narrowed => "form_narrowed",
            Self::Certified => "form_certified",
            Self::LabsNotClaimed => "form_labs_not_claimed",
        }
    }

    /// Monotonic rank, or `None` for the non-claiming Labs token.
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::Blocked => Some(0),
            Self::ReviewOverlay => Some(1),
            Self::Narrowed => Some(2),
            Self::Certified => Some(3),
            Self::LabsNotClaimed => None,
        }
    }

    /// Whether rendering `rendered` would overclaim relative to this effective
    /// claim. A rendering surface must never render wider than the form's
    /// effective claim; the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: FormClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a form fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormNarrowingReason {
    /// Submit is reachable while a blocked prerequisite or invalidation is active.
    SubmitAllowedWhileBlockedHidden,
    /// A blocking blocked-submit reason is not explained before submit.
    BlockedReasonUnexplained,
    /// A cross-field invalidation is not explained before submit.
    CrossFieldInvalidationHidden,
    /// The form-level summary contradicts the field-level validation.
    FieldFormValidationContradicts,
    /// The form-level summary replaces the field-level anchors.
    FormSummaryReplacesFieldAnchors,
    /// A derived constraint that affects submit is not disclosed.
    DerivedConstraintHidden,
    /// A blocked-submit reason carries no stable machine code.
    BlockedReasonNotMachineReadable,
    /// A blocking blocked-submit reason is not reusable by the machine consumers.
    BlockedReasonNotReusable,
    /// A blocking/warning validation is deferred to a banner instead of a
    /// field-anchored exact rule.
    ValidationAnchorMissing,
    /// An imported/restore review reads as a local submit.
    ImportedSubmitReadsAsApplied,
    /// A rendering surface renders wider than the effective claim.
    RenderingOverclaims,
    /// Backing data is missing.
    ValidationBackingMissing,
    /// A non-blocking cross-field dependency is not yet explained.
    CrossFieldDependencyDeferred,
    /// A blocking blocked-submit reason has no resolution hint.
    ResolutionHintMissing,
    /// A field's validation state is not surfaced.
    ValidationStateUnlabeled,
    /// Async validation is pending.
    AsyncValidationPending,
    /// The freshness state is not surfaced.
    FreshnessUnlabeled,
    /// A superseded backing source is not marked.
    SupersededStateNotMarked,
    /// A first-party form is stale.
    FormStale,
    /// The verification proof is stale.
    VerificationProofStale,
    /// The verification proof is missing.
    VerificationProofMissing,
    /// The reopen-to-origin path is lost.
    ReopenPathLost,
}

impl FormNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SubmitAllowedWhileBlockedHidden => "submit_allowed_while_blocked_hidden",
            Self::BlockedReasonUnexplained => "blocked_reason_unexplained",
            Self::CrossFieldInvalidationHidden => "cross_field_invalidation_hidden",
            Self::FieldFormValidationContradicts => "field_form_validation_contradicts",
            Self::FormSummaryReplacesFieldAnchors => "form_summary_replaces_field_anchors",
            Self::DerivedConstraintHidden => "derived_constraint_hidden",
            Self::BlockedReasonNotMachineReadable => "blocked_reason_not_machine_readable",
            Self::BlockedReasonNotReusable => "blocked_reason_not_reusable",
            Self::ValidationAnchorMissing => "validation_anchor_missing",
            Self::ImportedSubmitReadsAsApplied => "imported_submit_reads_as_applied",
            Self::RenderingOverclaims => "rendering_overclaims",
            Self::ValidationBackingMissing => "validation_backing_missing",
            Self::CrossFieldDependencyDeferred => "cross_field_dependency_deferred",
            Self::ResolutionHintMissing => "resolution_hint_missing",
            Self::ValidationStateUnlabeled => "validation_state_unlabeled",
            Self::AsyncValidationPending => "async_validation_pending",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::SupersededStateNotMarked => "superseded_state_not_marked",
            Self::FormStale => "form_stale",
            Self::VerificationProofStale => "verification_proof_stale",
            Self::VerificationProofMissing => "verification_proof_missing",
            Self::ReopenPathLost => "reopen_path_lost",
        }
    }

    /// Deterministic ordering index (mirrors the validator's reason order).
    pub const fn order_index(self) -> u8 {
        match self {
            Self::SubmitAllowedWhileBlockedHidden => 0,
            Self::BlockedReasonUnexplained => 1,
            Self::CrossFieldInvalidationHidden => 2,
            Self::FieldFormValidationContradicts => 3,
            Self::FormSummaryReplacesFieldAnchors => 4,
            Self::DerivedConstraintHidden => 5,
            Self::BlockedReasonNotMachineReadable => 6,
            Self::BlockedReasonNotReusable => 7,
            Self::ValidationAnchorMissing => 8,
            Self::ImportedSubmitReadsAsApplied => 9,
            Self::RenderingOverclaims => 10,
            Self::ValidationBackingMissing => 11,
            Self::CrossFieldDependencyDeferred => 12,
            Self::ResolutionHintMissing => 13,
            Self::ValidationStateUnlabeled => 14,
            Self::AsyncValidationPending => 15,
            Self::FreshnessUnlabeled => 16,
            Self::SupersededStateNotMarked => 17,
            Self::FormStale => 18,
            Self::VerificationProofStale => 19,
            Self::VerificationProofMissing => 20,
            Self::ReopenPathLost => 21,
        }
    }

    /// Whether this reason breaks the contract outright (floors the form to
    /// [`FormClaim::Blocked`]) rather than merely aging it out.
    pub const fn is_floor(self) -> bool {
        self.order_index() <= Self::ValidationBackingMissing.order_index()
    }

    /// A reviewer-facing, non-generic description of the reason.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::SubmitAllowedWhileBlockedHidden => {
                "submit is reachable while a blocked prerequisite or invalidation is active"
            }
            Self::BlockedReasonUnexplained => {
                "a blocking blocked-submit reason is not explained before submit"
            }
            Self::CrossFieldInvalidationHidden => {
                "a cross-field invalidation is not explained before submit"
            }
            Self::FieldFormValidationContradicts => {
                "the form-level summary contradicts the field-level validation"
            }
            Self::FormSummaryReplacesFieldAnchors => {
                "the form-level summary replaces the field-level anchors"
            }
            Self::DerivedConstraintHidden => "a derived constraint that affects submit is hidden",
            Self::BlockedReasonNotMachineReadable => {
                "a blocked-submit reason carries no stable machine code"
            }
            Self::BlockedReasonNotReusable => {
                "a blocking blocked-submit reason is not reusable by the machine consumers"
            }
            Self::ValidationAnchorMissing => {
                "a blocking validation is deferred to a banner instead of an exact field anchor"
            }
            Self::ImportedSubmitReadsAsApplied => {
                "an imported/restore review reads as a local submit"
            }
            Self::RenderingOverclaims => {
                "a rendering surface renders wider than the effective claim"
            }
            Self::ValidationBackingMissing => "the backing data is missing",
            Self::CrossFieldDependencyDeferred => {
                "a non-blocking cross-field dependency is not yet explained"
            }
            Self::ResolutionHintMissing => {
                "a blocking blocked-submit reason has no resolution hint"
            }
            Self::ValidationStateUnlabeled => "a field's validation state is not surfaced",
            Self::AsyncValidationPending => "async validation is still pending",
            Self::FreshnessUnlabeled => "the backing freshness state is not surfaced",
            Self::SupersededStateNotMarked => "a superseded backing source is not marked",
            Self::FormStale => "the backing source is stale",
            Self::VerificationProofStale => "the verification proof is stale",
            Self::VerificationProofMissing => "the verification proof is missing",
            Self::ReopenPathLost => "the reopen-to-origin path is lost",
        }
    }
}

fn order_reasons(mut reasons: Vec<FormNarrowingReason>) -> Vec<FormNarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Surface sub-objects.
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

/// Stable identifiers binding a form-validation record to its origin. Absent refs
/// serialize as `null` so the schema's required keys stay present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormLineage {
    /// Form-session/context ref (required).
    pub session_ref: String,
    /// The form's own stable canonical ref (required for a real surface).
    pub canonical_surface_ref: Option<String>,
    /// Backlink to the structured-input surface this form validation belongs to.
    pub structured_input_ref: Option<String>,
    /// Provider ref (required for provider-backed/imported overlay forms).
    pub provider_ref: Option<String>,
    /// Imported/source-artifact ref backing the form.
    pub source_artifact_ref: Option<String>,
    /// Reopen backlink ref.
    pub reopen_backlink_ref: Option<String>,
}

/// One field's validation anchor, linking the field-level state to the form-level
/// summary without letting the summary replace the field anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldValidationAnchor {
    /// Stable field id.
    pub field_id: String,
    /// The field's validation state.
    pub validation_state: ValidationState,
    /// The validation state is surfaced on the field.
    pub state_labeled: bool,
    /// A blocking/warning validation is anchored directly to the field.
    pub anchored_to_field: bool,
    /// The exact rule text is present on the field (not just a banner).
    pub exact_rule_text_present: bool,
    /// This field's validity is rolled up into the form-level summary.
    pub rolled_up_into_summary: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The form-level validation summary that rolls field validity up without
/// replacing the field anchors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormValidationSummary {
    /// Count of blocked values summarized.
    pub blocked_value_count: u64,
    /// Count of missing prerequisites summarized.
    pub missing_prerequisite_count: u64,
    /// Count of derived constraints summarized.
    pub derived_constraint_count: u64,
    /// Count of submit blockers summarized.
    pub submit_blocker_count: u64,
    /// The summary is derived from the field-level anchors.
    pub summarizes_field_anchors: bool,
    /// The summary replaces the field-level anchors (must be false).
    pub replaces_field_anchors: bool,
    /// The summary is consistent with the field-level validation.
    pub consistent_with_fields: bool,
    /// Derived constraints are disclosed.
    pub derived_constraints_disclosed: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// One cross-field dependency, explaining how one choice narrows or invalidates
/// another.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossFieldDependency {
    /// Stable dependency id.
    pub dependency_id: String,
    /// The dependency kind.
    pub dependency_kind: DependencyKind,
    /// How the source field relates to the target field.
    pub relation: DependencyRelation,
    /// Source-field ref.
    pub source_field_ref: String,
    /// Target-field ref.
    pub target_field_ref: String,
    /// The dependency is explained before submit.
    pub explained_before_submit: bool,
    /// The dependency currently blocks submit.
    pub blocks_submit: bool,
    /// A resolution hint is present.
    pub resolution_hint_present: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// One machine-readable blocked-submit reason, reusable across consumer surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedSubmitReason {
    /// Stable reason id.
    pub reason_id: String,
    /// The blocker class.
    pub blocker_class: SubmitBlockerClass,
    /// A stable machine code for CLI/headless and support reuse.
    pub machine_code: String,
    /// Whether this reason blocks submit.
    pub blocks_submit: bool,
    /// Whether the reason is explained before submit.
    pub explained_before_submit: bool,
    /// Whether a resolution hint is present.
    pub resolution_hint_present: bool,
    /// The consumer surfaces that can reuse this reason.
    pub reusable_by: Vec<BlockedSubmitConsumer>,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The submit gate for a form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitGate {
    /// Whether submit is currently reachable.
    pub submit_allowed: bool,
    /// Every active blocker is explained before submit.
    pub blockers_explained_before_submit: bool,
    /// The commit action names the scope/effect rather than a generic Continue.
    pub commit_action_is_specific: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The headline form-validation invariants every record re-derives rather than
/// trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormValidationIntegrity {
    /// Field-level anchors survive into the form (the summary does not replace
    /// them).
    pub field_anchors_preserved: bool,
    /// The form-level summary is linked to the field anchors.
    pub form_summary_linked: bool,
    /// Cross-field dependencies are explained before submit.
    pub cross_field_deps_explained: bool,
    /// Blocked-submit reasons carry stable machine codes.
    pub blocked_reasons_machine_readable: bool,
    /// Blocked-submit reasons are reusable across consumer surfaces.
    pub blocked_reasons_reusable: bool,
    /// Derived constraints are visible.
    pub derived_constraints_visible: bool,
    /// Imported/restore reviews stay read-only.
    pub imported_review_read_only: bool,
    /// Validation state is visible.
    pub validation_state_visible: bool,
    /// The freshness state is visible.
    pub freshness_state_visible: bool,
    /// A superseded backing source stays marked.
    pub superseded_state_marked: bool,
    /// Origin lineage / reopen is revealable on demand on every surface.
    pub reopen_visible_on_demand: bool,
}

/// Verification-proof currency for a form (distinct from backing freshness).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormVerification {
    /// Currency of the verification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the form.
    pub proof_ref: Option<String>,
}

/// One consumer surface that renders a form-validation record, with the claim it
/// shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormValidationRendering {
    /// The consumer surface.
    pub surface: ConsumerSurface,
    /// The claim this surface renders.
    pub rendered_claim: FormClaim,
    /// Whether field/scope provenance is revealable here.
    pub provenance_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical surface this view re-renders.
    pub source_surface_ref: String,
}

// --------------------------------------------------------------------------- //
// Record + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) mutation-capable form, with its field- and form-level
/// validation, cross-field dependencies, and machine-readable blocked-submit
/// reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormValidationRecord {
    /// Stable surface id.
    pub surface_id: String,
    /// The product lane.
    pub lane: FormLane,
    /// How the form/values originated.
    pub origin: FormOrigin,
    /// Reviewer-facing label summary.
    pub label_summary: String,
    /// Whether the form is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// Declared backing freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared submit-control fallback when floored.
    pub declared_blocked_fallback: BlockedSubmitFallback,
    /// Stable origin-lineage block.
    pub lineage: FormLineage,
    /// Field-level validation anchors.
    pub field_anchors: Vec<FieldValidationAnchor>,
    /// Form-level validation summary.
    pub form_summary: FormValidationSummary,
    /// Cross-field dependencies.
    pub dependencies: Vec<CrossFieldDependency>,
    /// Machine-readable blocked-submit reasons.
    pub blocked_submit_reasons: Vec<BlockedSubmitReason>,
    /// The submit gate.
    pub submit_gate: SubmitGate,
    /// Headline invariant block.
    pub integrity: FormValidationIntegrity,
    /// Verification-proof block.
    pub verification: FormVerification,
    /// Consumer surfaces that render this record.
    pub renderings: Vec<FormValidationRendering>,
}

/// The re-derived form decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormDecision {
    /// The headline claim the form is eligible to make.
    pub claimed_claim: FormClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_claim: FormClaim,
    /// Ordered, de-duplicated reasons the form fails to hold its headline.
    pub active_narrowing_reasons: Vec<FormNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl FormDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<FormNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this record would overclaim.
    pub fn surface_overclaims(&self, rendered: FormClaim) -> bool {
        self.effective_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(claimed: FormClaim, reasons: &[FormNarrowingReason]) -> FormClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        FormClaim::Blocked
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, FormClaim::ReviewOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we
        // can no longer certify even the read-only review, so it floors.
        FormClaim::Blocked
    } else {
        FormClaim::Narrowed
    }
}

impl FormValidationRecord {
    /// Whether this form is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this form is an inherently read-only review overlay.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin.is_overlay()
    }

    /// The headline claim this form is eligible to make.
    pub fn claimed_claim(&self) -> FormClaim {
        if self.is_labs() {
            FormClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            FormClaim::ReviewOverlay
        } else {
            FormClaim::Certified
        }
    }

    /// Whether any blocked-submit reason or cross-field dependency is currently an
    /// active blocker, or a field is invalid-blocking.
    fn any_active_blocker(&self) -> bool {
        self.blocked_submit_reasons.iter().any(|b| b.blocks_submit)
            || self.dependencies.iter().any(|d| d.blocks_submit)
            || self
                .field_anchors
                .iter()
                .any(|f| matches!(f.validation_state, ValidationState::InvalidBlocking))
    }

    /// Reasons that hold independently of how the consumer surfaces render — the
    /// intrinsic validation/dependency/blocked-submit gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<FormNarrowingReason> {
        use FormNarrowingReason as R;
        let summary = &self.form_summary;
        let gate = &self.submit_gate;
        let integ = &self.integrity;
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<R> = Vec::new();

        // Field-level validation anchors stay anchored to the field with exact
        // rule text and remain labelled.
        for f in &self.field_anchors {
            if f.validation_state.needs_field_anchor()
                && (!f.anchored_to_field || !f.exact_rule_text_present)
            {
                reasons.push(R::ValidationAnchorMissing);
            }
            if !f.state_labeled {
                reasons.push(R::ValidationStateUnlabeled);
            }
            if matches!(f.validation_state, ValidationState::PendingAsync) {
                reasons.push(R::AsyncValidationPending);
            }
        }

        // Form-level summary is linked, not duplicated/contradictory, and never
        // replaces the field anchors. A field that is invalid-blocking with no
        // corresponding blocked-submit reason is a field/form contradiction.
        let blocking_field = self
            .field_anchors
            .iter()
            .any(|f| matches!(f.validation_state, ValidationState::InvalidBlocking));
        let any_blocking_reason = self.blocked_submit_reasons.iter().any(|b| b.blocks_submit);
        if !summary.consistent_with_fields || (blocking_field && !any_blocking_reason) {
            reasons.push(R::FieldFormValidationContradicts);
        }
        if summary.replaces_field_anchors || !integ.field_anchors_preserved {
            reasons.push(R::FormSummaryReplacesFieldAnchors);
        }
        if !summary.summarizes_field_anchors || !integ.form_summary_linked {
            reasons.push(R::FieldFormValidationContradicts);
        }
        if summary.derived_constraint_count > 0
            && (!summary.derived_constraints_disclosed || !integ.derived_constraints_visible)
        {
            reasons.push(R::DerivedConstraintHidden);
        }

        // Cross-field dependencies are explained before submit.
        for d in &self.dependencies {
            if !d.explained_before_submit {
                if d.blocks_submit {
                    reasons.push(R::CrossFieldInvalidationHidden);
                } else {
                    reasons.push(R::CrossFieldDependencyDeferred);
                }
            }
        }
        if !integ.cross_field_deps_explained {
            reasons.push(R::CrossFieldInvalidationHidden);
        }

        // Blocked-submit reasons are explained, machine-readable, reusable, and
        // carry a resolution hint.
        for b in &self.blocked_submit_reasons {
            if b.blocks_submit && !b.explained_before_submit {
                reasons.push(R::BlockedReasonUnexplained);
            }
            if b.machine_code.trim().is_empty() {
                reasons.push(R::BlockedReasonNotMachineReadable);
            }
            if b.blocks_submit
                && BlockedSubmitConsumer::MACHINE
                    .iter()
                    .any(|m| !b.reusable_by.contains(m))
            {
                reasons.push(R::BlockedReasonNotReusable);
            }
            if b.blocks_submit && !b.resolution_hint_present {
                reasons.push(R::ResolutionHintMissing);
            }
        }
        if !integ.blocked_reasons_machine_readable {
            reasons.push(R::BlockedReasonNotMachineReadable);
        }
        if !integ.blocked_reasons_reusable {
            reasons.push(R::BlockedReasonNotReusable);
        }

        // The submit gate cannot be open while any prerequisite or invalidation
        // blocks, and an open gate must declare its blockers explained.
        if gate.submit_allowed
            && (self.any_active_blocker() || !gate.blockers_explained_before_submit)
        {
            reasons.push(R::SubmitAllowedWhileBlockedHidden);
        }

        // Imported/restore overlay must stay a read-only review, never a submit.
        if overlay
            && (!integ.imported_review_read_only
                || gate.submit_allowed
                || self.renderings.iter().any(|r| !r.read_only))
        {
            reasons.push(R::ImportedSubmitReadsAsApplied);
        }

        // Backing freshness.
        if !integ.freshness_state_visible {
            reasons.push(R::FreshnessUnlabeled);
        }
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::ValidationBackingMissing),
            FreshnessState::SupersededByNewerSource if !integ.superseded_state_marked => {
                reasons.push(R::SupersededStateNotMarked);
            }
            FreshnessState::StaleExpired if !overlay => reasons.push(R::FormStale),
            _ => {}
        }
        if !integ.validation_state_visible {
            reasons.push(R::ValidationStateUnlabeled);
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

        // Reopen-to-origin.
        if !integ.reopen_visible_on_demand || self.renderings.iter().any(|r| !r.provenance_visible)
        {
            reasons.push(R::ReopenPathLost);
        }

        reasons
    }

    /// All active narrowing reasons, including the rendering-surface overclaim
    /// check, ordered and de-duplicated.
    fn reasons(&self, stale_window: bool) -> Vec<FormNarrowingReason> {
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic_effective = derive_effective(self.claimed_claim(), &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic_effective.overclaims_as(r.rendered_claim))
        {
            reasons.push(FormNarrowingReason::RenderingOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive this form's claim decision.
    pub fn narrow(&self, stale_window: bool) -> FormDecision {
        let claimed = self.claimed_claim();
        if matches!(claimed, FormClaim::LabsNotClaimed) {
            return FormDecision {
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
        FormDecision {
            claimed_claim: claimed,
            effective_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// Whether a floored form still keeps a submit-control fallback that names the
    /// reason rather than a misleading clean submit.
    pub fn floored_keeps_fallback(&self, effective: FormClaim) -> bool {
        if !matches!(effective, FormClaim::Blocked) {
            return true;
        }
        self.declared_blocked_fallback.keeps_reason()
    }

    /// Whether any rendering overclaims relative to `effective`.
    pub fn surface_overclaims(&self, effective: FormClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// A reviewer-facing label for a narrowed/floored form, or `None` if the form
    /// holds its claim.
    pub fn narrowed_label(&self, decision: &FormDecision) -> Option<String> {
        let trigger = decision.downgrade_trigger()?;
        Some(match decision.effective_claim {
            FormClaim::Blocked => format!(
                "Floored to form_blocked below the {} claim: {}; falls back to an explicit blocked state that names the reason.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            FormClaim::Narrowed => format!(
                "Held at form_narrowed below the {} claim: {}; the form stays usable and reopenable until re-verified.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            _ => return None,
        })
    }

    /// Append per-record structural violations (schema-shape level).
    fn structural_violations(&self, out: &mut Vec<M5FormValidationViolation>) {
        use M5FormValidationViolation as V;
        if self.surface_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.lineage.session_ref.trim().is_empty()
        {
            out.push(V::SurfaceMissingIdentity);
        }
        if self.is_overlay_origin()
            && !opt_present(&self.lineage.provider_ref)
            && !opt_present(&self.lineage.source_artifact_ref)
        {
            out.push(V::OverlayMissingProvenanceRef);
        }
        if self.field_anchors.is_empty() {
            out.push(V::SurfaceMissingFields);
        }
        if self.renderings.is_empty() {
            out.push(V::SurfaceMissingRendering);
        }
        for r in &self.renderings {
            if r.source_surface_ref.trim().is_empty() {
                out.push(V::RenderingMissingSourceRef);
            }
        }
        for b in &self.blocked_submit_reasons {
            if b.reason_id.trim().is_empty() || b.label_summary.trim().is_empty() {
                out.push(V::BlockedReasonMissingIdentity);
            }
        }
    }
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for [`M5FormValidationSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FormValidationSetInput {
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
    /// Per-form rows.
    pub surfaces: Vec<FormValidationRecord>,
}

/// Export-safe M5 form-validation / blocked-submit set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FormValidationSetPacket {
    /// Record kind; must equal [`M5_FORM_VALIDATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FORM_VALIDATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_FORM_VALIDATION_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-form rows.
    pub surfaces: Vec<FormValidationRecord>,
}

/// The distribution of effective form claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormClaimDistribution {
    /// Forms effective at [`FormClaim::Certified`].
    pub certified: usize,
    /// Forms effective at [`FormClaim::Narrowed`].
    pub narrowed: usize,
    /// Forms effective at [`FormClaim::ReviewOverlay`].
    pub overlay: usize,
    /// Forms effective at [`FormClaim::Blocked`].
    pub blocked: usize,
    /// Forms effective at [`FormClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5FormValidationSetPacket {
    /// Builds a form-validation set packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5FormValidationSetInput) -> Self {
        Self {
            record_kind: M5_FORM_VALIDATION_RECORD_KIND.to_owned(),
            schema_version: M5_FORM_VALIDATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_FORM_VALIDATION_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            surfaces: input.surfaces,
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

    /// Re-derive the decision for every form, paired with its id.
    pub fn decisions(&self) -> Vec<(String, FormDecision)> {
        let stale_window = self.stale_window();
        self.surfaces
            .iter()
            .map(|s| (s.surface_id.clone(), s.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective form claims.
    pub fn claim_distribution(&self) -> FormClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = FormClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            blocked: 0,
            labs: 0,
        };
        for s in &self.surfaces {
            match s.narrow(stale_window).effective_claim {
                FormClaim::Certified => dist.certified += 1,
                FormClaim::Narrowed => dist.narrowed += 1,
                FormClaim::ReviewOverlay => dist.overlay += 1,
                FormClaim::Blocked => dist.blocked += 1,
                FormClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of forms whose effective claim ranks below their claimed claim.
    pub fn narrowed_surface_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.surfaces
            .iter()
            .filter(|s| s.narrow(stale_window).narrowed)
            .count()
    }

    /// Product lanes represented by some form.
    pub fn represented_lanes(&self) -> BTreeSet<FormLane> {
        self.surfaces.iter().map(|s| s.lane).collect()
    }

    /// Dependency kinds represented by some cross-field dependency.
    pub fn represented_dependency_kinds(&self) -> BTreeSet<DependencyKind> {
        self.surfaces
            .iter()
            .flat_map(|s| s.dependencies.iter().map(|d| d.dependency_kind))
            .collect()
    }

    /// Dependency relations represented by some cross-field dependency.
    pub fn represented_dependency_relations(&self) -> BTreeSet<DependencyRelation> {
        self.surfaces
            .iter()
            .flat_map(|s| s.dependencies.iter().map(|d| d.relation))
            .collect()
    }

    /// Blocker classes represented by some blocked-submit reason.
    pub fn represented_blocker_classes(&self) -> BTreeSet<SubmitBlockerClass> {
        self.surfaces
            .iter()
            .flat_map(|s| s.blocked_submit_reasons.iter().map(|b| b.blocker_class))
            .collect()
    }

    /// Blocked-submit consumers represented across the reusable-by declarations.
    pub fn represented_blocked_consumers(&self) -> BTreeSet<BlockedSubmitConsumer> {
        self.surfaces
            .iter()
            .flat_map(|s| {
                s.blocked_submit_reasons
                    .iter()
                    .flat_map(|b| b.reusable_by.iter().copied())
            })
            .collect()
    }

    /// Consumer surfaces represented by some rendering.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<ConsumerSurface> {
        self.surfaces
            .iter()
            .flat_map(|s| s.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Validate the form-validation invariants.
    pub fn validate(&self) -> Vec<M5FormValidationViolation> {
        use M5FormValidationViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_FORM_VALIDATION_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_FORM_VALIDATION_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_FORM_VALIDATION_TAXONOMY_VERSION {
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
        if self.surfaces.is_empty() {
            violations.push(V::EmptySurfaces);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for s in &self.surfaces {
            if !seen.insert(s.surface_id.as_str()) {
                violations.push(V::DuplicateSurfaceId);
            }
        }

        if FormLane::ALL
            .iter()
            .any(|l| !self.represented_lanes().contains(l))
        {
            violations.push(V::FormLaneMissing);
        }
        if DependencyKind::ALL
            .iter()
            .any(|k| !self.represented_dependency_kinds().contains(k))
        {
            violations.push(V::DependencyKindMissing);
        }
        if DependencyRelation::ALL
            .iter()
            .any(|r| !self.represented_dependency_relations().contains(r))
        {
            violations.push(V::DependencyRelationMissing);
        }
        if SubmitBlockerClass::ALL
            .iter()
            .any(|c| !self.represented_blocker_classes().contains(c))
        {
            violations.push(V::BlockerClassMissing);
        }
        if BlockedSubmitConsumer::ALL
            .iter()
            .any(|c| !self.represented_blocked_consumers().contains(c))
        {
            violations.push(V::BlockedConsumerMissing);
        }
        if ConsumerSurface::ALL
            .iter()
            .any(|s| !self.represented_consumer_surfaces().contains(s))
        {
            violations.push(V::ConsumerSurfaceMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for s in &self.surfaces {
            s.structural_violations(&mut violations);
            let decision = s.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || s.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedSurfaceMissingLabelOrTrigger);
                }
            }
            if !s.floored_keeps_fallback(decision.effective_claim) {
                violations.push(V::FlooredSurfaceLosesFallback);
            }
            if s.surface_overclaims(decision.effective_claim) {
                violations.push(V::RenderingSurfaceOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedSurfaceCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("form-validation packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        let mut out: Vec<M5FormValidationViolation> = Vec::new();
        for item in violations {
            if !out.contains(&item) {
                out.push(item);
            }
        }
        out
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("form-validation packet serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_report(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str(
            "# M5 Form Validation, Cross-Field Dependencies, and Blocked-Submit Reasons\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Forms: {}\n", self.surfaces.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} review overlay, {} blocked, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.blocked, dist.labs
        ));

        out.push_str("| Form | Lane | Origin | Deps | Blockers | Claimed | Effective |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for s in &self.surfaces {
            let decision = s.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                s.surface_id,
                s.lane.as_str(),
                s.origin.as_str(),
                s.dependencies.len(),
                s.blocked_submit_reasons.len(),
                decision.claimed_claim.as_str(),
                decision.effective_claim.as_str(),
            ));
        }

        out.push('\n');
        for s in &self.surfaces {
            let decision = s.narrow(stale_window);
            if let Some(label) = s.narrowed_label(&decision) {
                out.push_str(&format!("- {}: {}\n", s.surface_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum M5FormValidationArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5FormValidationViolation>),
}

impl fmt::Display for M5FormValidationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(f, "support export parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for M5FormValidationArtifactError {}

/// A form-validation packet validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FormValidationViolation {
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
    /// The set has no forms.
    EmptySurfaces,
    /// Two forms share a surface id.
    DuplicateSurfaceId,
    /// A product lane is unrepresented.
    FormLaneMissing,
    /// A dependency kind is unrepresented.
    DependencyKindMissing,
    /// A dependency relation is unrepresented.
    DependencyRelationMissing,
    /// A blocker class is unrepresented.
    BlockerClassMissing,
    /// A blocked-submit consumer is unrepresented.
    BlockedConsumerMissing,
    /// A consumer surface is unrepresented.
    ConsumerSurfaceMissing,
    /// A form lacks a required identity field.
    SurfaceMissingIdentity,
    /// An overlay form names no provider/source-artifact ref.
    OverlayMissingProvenanceRef,
    /// A form has no field anchors.
    SurfaceMissingFields,
    /// A form has no renderings.
    SurfaceMissingRendering,
    /// A rendering names no source surface ref.
    RenderingMissingSourceRef,
    /// A blocked-submit reason lacks a required identity field.
    BlockedReasonMissingIdentity,
    /// A narrowed form lacks a non-generic label or a downgrade trigger.
    NarrowedSurfaceMissingLabelOrTrigger,
    /// A floored form loses its submit-control fallback.
    FlooredSurfaceLosesFallback,
    /// A rendering surface renders wider than the effective claim.
    RenderingSurfaceOverclaims,
    /// No form demonstrates the auto-narrowing rule.
    DowngradedSurfaceCaseMissing,
    /// Raw boundary material crossed the export.
    RawBoundaryMaterialInExport,
}

impl M5FormValidationViolation {
    /// Stable token recorded in diagnostics and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptySurfaces => "empty_surfaces",
            Self::DuplicateSurfaceId => "duplicate_surface_id",
            Self::FormLaneMissing => "form_lane_missing",
            Self::DependencyKindMissing => "dependency_kind_missing",
            Self::DependencyRelationMissing => "dependency_relation_missing",
            Self::BlockerClassMissing => "blocker_class_missing",
            Self::BlockedConsumerMissing => "blocked_consumer_missing",
            Self::ConsumerSurfaceMissing => "consumer_surface_missing",
            Self::SurfaceMissingIdentity => "surface_missing_identity",
            Self::OverlayMissingProvenanceRef => "overlay_missing_provenance_ref",
            Self::SurfaceMissingFields => "surface_missing_fields",
            Self::SurfaceMissingRendering => "surface_missing_rendering",
            Self::RenderingMissingSourceRef => "rendering_missing_source_ref",
            Self::BlockedReasonMissingIdentity => "blocked_reason_missing_identity",
            Self::NarrowedSurfaceMissingLabelOrTrigger => {
                "narrowed_surface_missing_label_or_trigger"
            }
            Self::FlooredSurfaceLosesFallback => "floored_surface_loses_fallback",
            Self::RenderingSurfaceOverclaims => "rendering_surface_overclaims",
            Self::DowngradedSurfaceCaseMissing => "downgraded_surface_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// --------------------------------------------------------------------------- //
// Canonical artifact loader.
// --------------------------------------------------------------------------- //

/// Loads and validates the checked-in canonical support export.
///
/// This is the canonical entry point downstream settings, marketplace, request,
/// support, admin, import, and project surfaces use to ingest the frozen
/// form-validation / blocked-submit matrix instead of minting per-feature
/// semantics.
///
/// # Errors
///
/// Returns [`M5FormValidationArtifactError`] when the artifact cannot be parsed
/// or fails validation.
pub fn current_m5_form_validation_set(
) -> Result<M5FormValidationSetPacket, M5FormValidationArtifactError> {
    let packet: M5FormValidationSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json"
    )))
    .map_err(M5FormValidationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FormValidationArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded form-validation set: the in-crate source of truth the
/// checked-in support export and report are regenerated from.
pub fn seeded_m5_form_validation_set() -> M5FormValidationSetPacket {
    M5FormValidationSetPacket::new(M5FormValidationSetInput {
        packet_id: M5_FORM_VALIDATION_PACKET_ID.to_owned(),
        label:
            "M5 form validation — form-level summaries, cross-field dependencies, and machine-readable blocked-submit reasons across mutation-capable forms"
                .to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        surfaces: seed_surfaces(),
    })
}

/// Renderings that show `claim` cleanly across the named surfaces.
fn renderings(
    source_ref: &str,
    claim: FormClaim,
    surfaces: &[ConsumerSurface],
    read_only: bool,
) -> Vec<FormValidationRendering> {
    surfaces
        .iter()
        .map(|&surface| FormValidationRendering {
            surface,
            rendered_claim: claim,
            provenance_visible: true,
            read_only,
            source_surface_ref: source_ref.to_owned(),
        })
        .collect()
}

/// A clean first-party integrity block.
fn clean_integrity() -> FormValidationIntegrity {
    FormValidationIntegrity {
        field_anchors_preserved: true,
        form_summary_linked: true,
        cross_field_deps_explained: true,
        blocked_reasons_machine_readable: true,
        blocked_reasons_reusable: true,
        derived_constraints_visible: true,
        imported_review_read_only: true,
        validation_state_visible: true,
        freshness_state_visible: true,
        superseded_state_marked: true,
        reopen_visible_on_demand: true,
    }
}

/// A verified-current verification block.
fn verified(proof_ref: &str) -> FormVerification {
    FormVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

fn anchor(field_id: &str, validation_state: ValidationState, label: &str) -> FieldValidationAnchor {
    FieldValidationAnchor {
        field_id: field_id.to_owned(),
        validation_state,
        state_labeled: true,
        anchored_to_field: true,
        exact_rule_text_present: true,
        rolled_up_into_summary: true,
        label_summary: label.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn dependency(
    dependency_id: &str,
    dependency_kind: DependencyKind,
    relation: DependencyRelation,
    source_field_ref: &str,
    target_field_ref: &str,
    blocks_submit: bool,
    label: &str,
) -> CrossFieldDependency {
    CrossFieldDependency {
        dependency_id: dependency_id.to_owned(),
        dependency_kind,
        relation,
        source_field_ref: source_field_ref.to_owned(),
        target_field_ref: target_field_ref.to_owned(),
        explained_before_submit: true,
        blocks_submit,
        resolution_hint_present: true,
        label_summary: label.to_owned(),
    }
}

fn blocked_reason(
    reason_id: &str,
    blocker_class: SubmitBlockerClass,
    machine_code: &str,
    blocks_submit: bool,
    label: &str,
) -> BlockedSubmitReason {
    BlockedSubmitReason {
        reason_id: reason_id.to_owned(),
        blocker_class,
        machine_code: machine_code.to_owned(),
        blocks_submit,
        explained_before_submit: true,
        resolution_hint_present: true,
        reusable_by: BlockedSubmitConsumer::ALL.to_vec(),
        label_summary: label.to_owned(),
    }
}

fn gate(submit_allowed: bool, label: &str) -> SubmitGate {
    SubmitGate {
        submit_allowed,
        blockers_explained_before_submit: true,
        commit_action_is_specific: true,
        label_summary: label.to_owned(),
    }
}

/// The canonical forms: one per product lane, covering every dependency kind,
/// dependency relation, blocker class, blocked-submit consumer, and consumer
/// surface, plus a narrowed first-party form, a review overlay, and a Labs form.
fn seed_surfaces() -> Vec<FormValidationRecord> {
    use ConsumerSurface as CS;

    // Provider connection: a missing-account prerequisite blocks submit, the
    // provider→account mapping dependency is explained, and the blocked-submit
    // reason is reusable across every consumer. Certified: the gate is honestly
    // closed and the blocker is fully attributed.
    let provider = FormValidationRecord {
        surface_id: "form:provider-connection:0001".to_owned(),
        lane: FormLane::Provider,
        origin: FormOrigin::ProviderBacked,
        label_summary: "Provider connection form: choosing a provider narrows the account list; submit is blocked until an account is selected, with a reusable blocked-submit reason.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::ShowsReasonOnSubmit,
        lineage: FormLineage {
            session_ref: "form-session.provider.connection".to_owned(),
            canonical_surface_ref: Some("surface.provider.connection.0001".to_owned()),
            structured_input_ref: Some("form.provider.credentials.0001".to_owned()),
            provider_ref: Some("provider.connection.primary".to_owned()),
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.provider.connection.0001".to_owned()),
        },
        field_anchors: vec![
            anchor("provider_id", ValidationState::Valid, "Provider selected and valid."),
            anchor("account_id", ValidationState::InvalidBlocking, "Account required for the selected provider; blocking until set."),
        ],
        form_summary: FormValidationSummary {
            blocked_value_count: 1,
            missing_prerequisite_count: 1,
            derived_constraint_count: 0,
            submit_blocker_count: 1,
            summarizes_field_anchors: true,
            replaces_field_anchors: false,
            consistent_with_fields: true,
            derived_constraints_disclosed: true,
            label_summary: "1 blocked value, 1 missing prerequisite, 0 derived constraints.".to_owned(),
        },
        dependencies: vec![dependency(
            "dep.provider-account",
            DependencyKind::ProviderAccountMapping,
            DependencyRelation::Requires,
            "provider_id",
            "account_id",
            false,
            "Selecting a provider narrows the eligible accounts and requires one to be chosen.",
        )],
        blocked_submit_reasons: vec![blocked_reason(
            "reason.account-required",
            SubmitBlockerClass::MissingPrerequisite,
            "provider.account.required",
            true,
            "Select an account for the chosen provider before connecting.",
        )],
        submit_gate: gate(false, "Connect provider — blocked until an account is selected."),
        integrity: clean_integrity(),
        verification: verified("proof.provider.connection.0001"),
        renderings: renderings("surface.provider.connection.0001", FormClaim::Certified, &[CS::FormView, CS::DiagnosticsPanel, CS::SupportExport], false),
    };

    // Settings editor: a clean certified baseline with no active blockers.
    let settings = FormValidationRecord {
        surface_id: "form:settings-config:0001".to_owned(),
        lane: FormLane::Settings,
        origin: FormOrigin::LocalAuthoring,
        label_summary: "Settings editor: field-level validation rolls up into the form summary with no active blockers; submit is open.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.settings.config".to_owned(),
            canonical_surface_ref: Some("surface.settings.config.0001".to_owned()),
            structured_input_ref: Some("form.settings.config.0001".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.settings.config.0001".to_owned()),
        },
        field_anchors: vec![
            anchor("theme", ValidationState::Valid, "Theme valid."),
            anchor("font_size", ValidationState::Warning, "Font size unusually large; warning anchored to the field."),
        ],
        form_summary: FormValidationSummary {
            blocked_value_count: 0,
            missing_prerequisite_count: 0,
            derived_constraint_count: 0,
            submit_blocker_count: 0,
            summarizes_field_anchors: true,
            replaces_field_anchors: false,
            consistent_with_fields: true,
            derived_constraints_disclosed: true,
            label_summary: "No blocked values; 1 field warning.".to_owned(),
        },
        dependencies: vec![],
        blocked_submit_reasons: vec![],
        submit_gate: gate(true, "Apply settings."),
        integrity: clean_integrity(),
        verification: verified("proof.settings.config.0001"),
        renderings: renderings("surface.settings.config.0001", FormClaim::Certified, &[CS::FormView, CS::HelpInline], false),
    };

    // Project bootstrap wizard: a required-unset name is invalid-blocking and is
    // backed by an invalid-field blocked-submit reason.
    let projects = FormValidationRecord {
        surface_id: "wizard:project-bootstrap:0001".to_owned(),
        lane: FormLane::Projects,
        origin: FormOrigin::LocalAuthoring,
        label_summary: "Project bootstrap wizard: an empty project name is invalid-blocking with an exact field anchor and a reusable invalid-field blocked-submit reason.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::ShowsReasonOnSubmit,
        lineage: FormLineage {
            session_ref: "form-session.projects.bootstrap".to_owned(),
            canonical_surface_ref: Some("surface.projects.bootstrap.0001".to_owned()),
            structured_input_ref: Some("wizard.projects.bootstrap.0001".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.projects.bootstrap.0001".to_owned()),
        },
        field_anchors: vec![
            anchor("project_name", ValidationState::InvalidBlocking, "Project name required; blocking until set."),
            anchor("template", ValidationState::Valid, "Template valid."),
        ],
        form_summary: FormValidationSummary {
            blocked_value_count: 1,
            missing_prerequisite_count: 0,
            derived_constraint_count: 0,
            submit_blocker_count: 1,
            summarizes_field_anchors: true,
            replaces_field_anchors: false,
            consistent_with_fields: true,
            derived_constraints_disclosed: true,
            label_summary: "1 invalid field blocking submit.".to_owned(),
        },
        dependencies: vec![],
        blocked_submit_reasons: vec![blocked_reason(
            "reason.name-required",
            SubmitBlockerClass::InvalidField,
            "projects.name.required",
            true,
            "Enter a project name before creating the project.",
        )],
        submit_gate: gate(false, "Create project — blocked until a name is entered."),
        integrity: clean_integrity(),
        verification: verified("proof.projects.bootstrap.0001"),
        renderings: renderings("surface.projects.bootstrap.0001", FormClaim::Certified, &[CS::WizardStep, CS::SupportExport], false),
    };

    // Package install: a private registry source requires registry auth, and a
    // policy-gated lifecycle script plus an unreviewed side effect block submit.
    let package = FormValidationRecord {
        surface_id: "sheet:package-install:0001".to_owned(),
        lane: FormLane::Package,
        origin: FormOrigin::LocalAuthoring,
        label_summary: "Package install sheet: a private registry source requires auth; a policy-locked lifecycle script and an unreviewed side effect block submit, each with reusable reasons.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::ShowsReasonOnSubmit,
        lineage: FormLineage {
            session_ref: "form-session.package.install".to_owned(),
            canonical_surface_ref: Some("surface.package.install.0001".to_owned()),
            structured_input_ref: Some("sheet.package.install.0001".to_owned()),
            provider_ref: None,
            source_artifact_ref: Some("artifact.lockfile.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.package.install.0001".to_owned()),
        },
        field_anchors: vec![
            anchor("registry_source", ValidationState::Valid, "Registry source valid."),
            anchor("auth_token_ref", ValidationState::Valid, "Registry auth reference present."),
        ],
        form_summary: FormValidationSummary {
            blocked_value_count: 0,
            missing_prerequisite_count: 0,
            derived_constraint_count: 0,
            submit_blocker_count: 2,
            summarizes_field_anchors: true,
            replaces_field_anchors: false,
            consistent_with_fields: true,
            derived_constraints_disclosed: true,
            label_summary: "2 submit blockers: a policy-locked script and an unreviewed side effect.".to_owned(),
        },
        dependencies: vec![dependency(
            "dep.registry-auth",
            DependencyKind::PackageSourceRegistryAuth,
            DependencyRelation::Requires,
            "registry_source",
            "auth_token_ref",
            false,
            "A private registry source requires a registry auth reference.",
        )],
        blocked_submit_reasons: vec![
            blocked_reason(
                "reason.script-policy-locked",
                SubmitBlockerClass::UnresolvedPolicyLock,
                "package.lifecycle_script.policy_locked",
                true,
                "Lifecycle scripts are blocked by policy; request an exception to proceed.",
            ),
            blocked_reason(
                "reason.native-build-unreviewed",
                SubmitBlockerClass::UnreviewedSideEffect,
                "package.native_build.unreviewed",
                true,
                "Review the native build side effect before installing.",
            ),
        ],
        submit_gate: gate(false, "Install packages — blocked pending policy and side-effect review."),
        integrity: clean_integrity(),
        verification: verified("proof.package.install.0001"),
        renderings: renderings("surface.package.install.0001", FormClaim::Certified, &[CS::ReviewSheet, CS::AiEvidence], false),
    };

    // Admin policy rollout: a derived-field constraint invalidates the chosen ring
    // when the baseline changes; a cross-field conflict reason blocks submit.
    let admin = FormValidationRecord {
        surface_id: "sheet:admin-policy-rollout:0001".to_owned(),
        lane: FormLane::Admin,
        origin: FormOrigin::LocalAuthoring,
        label_summary: "Admin policy-rollout sheet: a changed baseline invalidates the chosen rollout ring (a disclosed derived constraint); a cross-field conflict reason blocks submit.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.admin.policy".to_owned(),
            canonical_surface_ref: Some("surface.admin.policy.0001".to_owned()),
            structured_input_ref: Some("sheet.admin.policy.0001".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.admin.policy.0001".to_owned()),
        },
        field_anchors: vec![
            anchor("enforced_baseline", ValidationState::Valid, "Enforced baseline valid."),
            anchor("rollout_ring", ValidationState::Valid, "Rollout ring valid after the derived-constraint recheck."),
        ],
        form_summary: FormValidationSummary {
            blocked_value_count: 0,
            missing_prerequisite_count: 0,
            derived_constraint_count: 1,
            submit_blocker_count: 1,
            summarizes_field_anchors: true,
            replaces_field_anchors: false,
            consistent_with_fields: true,
            derived_constraints_disclosed: true,
            label_summary: "1 derived constraint disclosed; 1 cross-field conflict blocking submit.".to_owned(),
        },
        dependencies: vec![dependency(
            "dep.baseline-ring",
            DependencyKind::DerivedFieldConstraint,
            DependencyRelation::Invalidates,
            "enforced_baseline",
            "rollout_ring",
            true,
            "Changing the enforced baseline invalidates the chosen rollout ring until it is reselected.",
        )],
        blocked_submit_reasons: vec![blocked_reason(
            "reason.ring-conflict",
            SubmitBlockerClass::CrossFieldConflict,
            "admin.rollout_ring.baseline_conflict",
            true,
            "The rollout ring conflicts with the enforced baseline; reselect a compatible ring.",
        )],
        submit_gate: gate(false, "Roll out policy — blocked until the ring conflict is resolved."),
        integrity: clean_integrity(),
        verification: verified("proof.admin.policy.0001"),
        renderings: renderings("surface.admin.policy.0001", FormClaim::Certified, &[CS::ReviewSheet, CS::SupportExport], false),
    };

    // Request run dialog: an in-flight async health check holds the form at
    // narrowed; the environment→region dependency is explained.
    let request = FormValidationRecord {
        surface_id: "dialog:request-run:0001".to_owned(),
        lane: FormLane::Request,
        origin: FormOrigin::RemoteTarget,
        label_summary: "Request-workspace run dialog: selecting an environment narrows the region; an in-flight async health check holds the form at narrowed until it resolves.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::CachedSnapshot,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.request.run".to_owned(),
            canonical_surface_ref: Some("surface.request.run.0001".to_owned()),
            structured_input_ref: Some("dialog.request.run.0001".to_owned()),
            provider_ref: Some("provider.remote.workspace".to_owned()),
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.request.run.0001".to_owned()),
        },
        field_anchors: vec![
            anchor("environment", ValidationState::Valid, "Environment valid."),
            anchor("endpoint_health", ValidationState::PendingAsync, "Endpoint health check in flight."),
        ],
        form_summary: FormValidationSummary {
            blocked_value_count: 0,
            missing_prerequisite_count: 0,
            derived_constraint_count: 0,
            submit_blocker_count: 1,
            summarizes_field_anchors: true,
            replaces_field_anchors: false,
            consistent_with_fields: true,
            derived_constraints_disclosed: true,
            label_summary: "1 pending validation holding submit.".to_owned(),
        },
        dependencies: vec![dependency(
            "dep.environment-region",
            DependencyKind::EnvironmentSelection,
            DependencyRelation::Narrows,
            "environment",
            "region",
            false,
            "Selecting an environment narrows the available regions.",
        )],
        blocked_submit_reasons: vec![blocked_reason(
            "reason.health-pending",
            SubmitBlockerClass::PendingValidation,
            "request.endpoint_health.pending",
            true,
            "Wait for the endpoint health check to finish before running.",
        )],
        submit_gate: gate(false, "Run request — blocked while the health check is in flight."),
        integrity: clean_integrity(),
        verification: verified("proof.request.run.0001"),
        renderings: renderings("surface.request.run.0001", FormClaim::Narrowed, &[CS::FormView, CS::AiEvidence], false),
    };

    // Migration restore review: an imported/restore overlay. Read-only; explains
    // the import/export mode dependency and a reusable blocked-submit reason but
    // never reads as a local submit.
    let import = FormValidationRecord {
        surface_id: "dialog:migration-restore:0001".to_owned(),
        lane: FormLane::Import,
        origin: FormOrigin::ImportedOrRestore,
        label_summary: "Migration-center restore review: import and export modes are mutually exclusive; the review explains its blocked-submit reason read-only and never reads as a local submit.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::CachedSnapshot,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.import.restore".to_owned(),
            canonical_surface_ref: Some("surface.import.restore.0001".to_owned()),
            structured_input_ref: Some("dialog.import.restore.0001".to_owned()),
            provider_ref: Some("provider.migration.center".to_owned()),
            source_artifact_ref: Some("artifact.import.backup.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.import.restore.0001".to_owned()),
        },
        field_anchors: vec![
            anchor("restore_mode", ValidationState::Valid, "Restore mode valid (read-only)."),
            anchor("merge_strategy", ValidationState::Valid, "Merge strategy valid (read-only)."),
        ],
        form_summary: FormValidationSummary {
            blocked_value_count: 0,
            missing_prerequisite_count: 1,
            derived_constraint_count: 0,
            submit_blocker_count: 1,
            summarizes_field_anchors: true,
            replaces_field_anchors: false,
            consistent_with_fields: true,
            derived_constraints_disclosed: true,
            label_summary: "Read-only review: 1 prerequisite documented for the eventual local restore.".to_owned(),
        },
        dependencies: vec![dependency(
            "dep.import-export-mode",
            DependencyKind::ImportExportMode,
            DependencyRelation::MutuallyExclusive,
            "restore_mode",
            "export_mode",
            false,
            "Import and export modes are mutually exclusive; only one runs at a time.",
        )],
        blocked_submit_reasons: vec![blocked_reason(
            "reason.target-confirm",
            SubmitBlockerClass::MissingPrerequisite,
            "import.target.confirm_required",
            false,
            "Confirm the restore target before applying locally.",
        )],
        submit_gate: gate(false, "Review restore (read-only); apply happens in the local restore step."),
        integrity: clean_integrity(),
        verification: FormVerification {
            proof_currency: ProofCurrency::CachedWithinWindow,
            proof_ref: Some("proof.import.restore.0001".to_owned()),
        },
        renderings: renderings("surface.import.restore.0001", FormClaim::ReviewOverlay, &[CS::ReviewSheet, CS::DiagnosticsPanel], true),
    };

    // Labs onboarding wizard: makes no public claim.
    let labs = FormValidationRecord {
        surface_id: "wizard:labs-onboarding:0001".to_owned(),
        lane: FormLane::Projects,
        origin: FormOrigin::LocalAuthoring,
        label_summary: "Experimental onboarding wizard behind a Labs flag; makes no public form-validation claim.".to_owned(),
        claim_posture: ClaimPosture::LabsUnadvertised,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.labs.onboarding".to_owned(),
            canonical_surface_ref: Some("surface.labs.onboarding.0001".to_owned()),
            structured_input_ref: Some("wizard.labs.onboarding.0001".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.labs.onboarding.0001".to_owned()),
        },
        field_anchors: vec![anchor(
            "experiment_choice",
            ValidationState::NotValidated,
            "Experimental choice; not yet validated.",
        )],
        form_summary: FormValidationSummary {
            blocked_value_count: 0,
            missing_prerequisite_count: 0,
            derived_constraint_count: 0,
            submit_blocker_count: 0,
            summarizes_field_anchors: true,
            replaces_field_anchors: false,
            consistent_with_fields: true,
            derived_constraints_disclosed: true,
            label_summary: "Experimental; no public validation claim.".to_owned(),
        },
        dependencies: vec![],
        blocked_submit_reasons: vec![],
        submit_gate: gate(true, "Try experiment."),
        integrity: clean_integrity(),
        verification: FormVerification {
            proof_currency: ProofCurrency::MissingProof,
            proof_ref: None,
        },
        renderings: renderings("surface.labs.onboarding.0001", FormClaim::LabsNotClaimed, &[CS::WizardStep], false),
    };

    vec![
        provider, settings, projects, package, admin, request, import, labs,
    ]
}
