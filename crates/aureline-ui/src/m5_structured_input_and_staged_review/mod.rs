//! Canonical structured-input, parameter-provenance, draft-state, and
//! staged-review truth for mutation-capable forms, wizards, and review sheets.
//!
//! Aureline's launch-critical product surfaces — provider-configuration forms,
//! project-bootstrap wizards, request-workspace run dialogs, package/install
//! review sheets, admin policy-rollout sheets, and migration-center restore
//! reviews — all mutate state, and they all share one honesty contract that this
//! module freezes once instead of re-inventing per feature. Each
//! [`FormSurfaceRecord`] binds a mutation-capable surface to:
//!
//! * its **field provenance** — every field declares whether its value is a
//!   default, a detected value, an imported value, a policy lock, a user
//!   override, or a still-unset requirement, and a user override stays visibly
//!   distinct from the default/detected/imported value it replaced;
//! * its **validation state** — field- and form-level validity is labelled
//!   (valid, pending-async, warning, invalid-blocking, not-validated) rather than
//!   inferred from a greyed-out button;
//! * its **draft/applied state and recovery** — draft is always visibly distinct
//!   from applied, local drafts autosave, and interruption/restart/reconnect
//!   preserves a recoverable draft instead of silently discarding work;
//! * its **submit blockers** — blocked prerequisites and cross-field conflicts
//!   are explained *before* submit rather than surfaced as a dead Continue
//!   button; and
//! * its **staged review** — the commit sheet declares the target scope, the
//!   omitted defaults, the included/excluded/blocked members, the side effects,
//!   and the rollback/export path, and the commit action names that scope and
//!   effect instead of being a generic Continue.
//!
//! Each record re-derives a [`SurfaceClaim`] ([`FormSurfaceRecord::narrow`]) so a
//! surface can never read wider than its evidence: a form that hides a field's
//! source, blurs draft versus applied, hides the target scope or omitted
//! defaults, buries a blocked prerequisite or rollback consequence behind a
//! generic Continue, silently overrides a policy lock, submits over an
//! invalid-blocking field, lets an imported/restore review read as a local apply,
//! or discards a recoverable draft floors to [`SurfaceClaim::Unsafe`] and falls
//! back to an explicit blocked state with a reopen/keyboard recovery path. A
//! labelled, recoverable gap (pending validation, a stale backing source, a stale
//! verification proof) holds a first-party surface at [`SurfaceClaim::Narrowed`]
//! while keeping the draft recoverable and reopenable, an imported/restore review
//! sits at [`SurfaceClaim::ReviewOverlay`] and never claims a local apply, and a
//! Labs/unadvertised surface makes no public claim.
//!
//! [`M5StructuredInputSetPacket::validate`] confirms the matrix is well-formed and
//! honest: header/identity/redaction/freshness are present, every surface kind,
//! product lane, mutation-backing class, source-of-value class, and consumer
//! surface is represented, overlay surfaces name their provenance, no rendering
//! surface overclaims, a floored surface keeps a fallback, at least one surface
//! demonstrates the auto-narrowing rule, and no raw credential/body material
//! crosses the export. Downstream settings, marketplace, request, support, admin,
//! import, and project surfaces ingest this packet rather than minting per-feature
//! form semantics.
//!
//! No credential bodies, secret values, raw provider payloads, absolute paths, or
//! URLs ever cross this boundary; the packet carries only typed class tokens,
//! counts, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/ux/m5-structured-input-and-staged-review.schema.json`](../../../../schemas/ux/m5-structured-input-and-staged-review.schema.json).
//! The contract doc is
//! [`docs/ux/m5-structured-input-and-staged-review.md`](../../../../docs/ux/m5-structured-input-and-staged-review.md).
//! The canonical support export is
//! [`artifacts/ux/m5-structured-input-and-staged-review/support_export.json`](../../../../artifacts/ux/m5-structured-input-and-staged-review/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/ux/m5-structured-input-and-staged-review/`](../../../../fixtures/ux/m5-structured-input-and-staged-review/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5StructuredInputSetPacket`].
pub const M5_STRUCTURED_INPUT_RECORD_KIND: &str = "m5_structured_input_set_packet";

/// Schema version for the structured-input set.
pub const M5_STRUCTURED_INPUT_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_STRUCTURED_INPUT_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical structured-input set packet.
pub const M5_STRUCTURED_INPUT_PACKET_ID: &str = "m5-structured-input:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_STRUCTURED_INPUT_SCHEMA_REF: &str =
    "schemas/ux/m5-structured-input-and-staged-review.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_STRUCTURED_INPUT_DOC_REF: &str = "docs/ux/m5-structured-input-and-staged-review.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_STRUCTURED_INPUT_SUPPORT_EXPORT_REF: &str =
    "artifacts/ux/m5-structured-input-and-staged-review/support_export.json";

/// Repo-relative path of the generated report.
pub const M5_STRUCTURED_INPUT_REPORT_REF: &str =
    "artifacts/ux/m5-structured-input-and-staged-review/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_STRUCTURED_INPUT_FIXTURE_DIR: &str =
    "fixtures/ux/m5-structured-input-and-staged-review";

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

/// The kind of mutation-capable surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormSurfaceKind {
    /// A structured configuration/settings form.
    StructuredForm,
    /// A multi-step wizard (bootstrap, onboarding, generated project).
    MultiStepWizard,
    /// A publish/review dialog (marketplace publish, admin rollout).
    PublishReviewDialog,
    /// An import/restore dialog (migration center, backup restore).
    ImportRestoreDialog,
    /// A package/install review sheet.
    InstallReviewSheet,
    /// A parameterized run/workflow dialog (request execution, batch action).
    ParameterizedWorkflow,
}

impl FormSurfaceKind {
    /// Every surface kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StructuredForm,
        Self::MultiStepWizard,
        Self::PublishReviewDialog,
        Self::ImportRestoreDialog,
        Self::InstallReviewSheet,
        Self::ParameterizedWorkflow,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredForm => "structured_form",
            Self::MultiStepWizard => "multi_step_wizard",
            Self::PublishReviewDialog => "publish_review_dialog",
            Self::ImportRestoreDialog => "import_restore_dialog",
            Self::InstallReviewSheet => "install_review_sheet",
            Self::ParameterizedWorkflow => "parameterized_workflow",
        }
    }
}

/// The product lane that owns a surface.
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

/// What backs a surface's mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationBackingClass {
    /// Mutates local first-party state.
    Local,
    /// Mutates state on a remote target.
    Remote,
    /// Mutates provider-backed state.
    ProviderBacked,
    /// Mutates via import/export of an external artifact.
    ImportExport,
    /// Mutates policy-governed/locked state.
    PolicyLocked,
}

impl MutationBackingClass {
    /// Every mutation-backing class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Local,
        Self::Remote,
        Self::ProviderBacked,
        Self::ImportExport,
        Self::PolicyLocked,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::ProviderBacked => "provider_backed",
            Self::ImportExport => "import_export",
            Self::PolicyLocked => "policy_locked",
        }
    }
}

/// The source of a field's value. Keeping these distinct is what lets a form show
/// where every value came from before the user submits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceOfValueClass {
    /// A built-in default.
    DefaultValue,
    /// A value detected from the environment/workspace.
    DetectedValue,
    /// A value imported from an external artifact.
    ImportedValue,
    /// A value locked by policy and not user-editable.
    PolicyLocked,
    /// A value the user explicitly overrode.
    UserOverride,
    /// A required value not yet set.
    RequiredUnset,
}

impl SourceOfValueClass {
    /// Every source-of-value class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DefaultValue,
        Self::DetectedValue,
        Self::ImportedValue,
        Self::PolicyLocked,
        Self::UserOverride,
        Self::RequiredUnset,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultValue => "default_value",
            Self::DetectedValue => "detected_value",
            Self::ImportedValue => "imported_value",
            Self::PolicyLocked => "policy_locked",
            Self::UserOverride => "user_override",
            Self::RequiredUnset => "required_unset",
        }
    }
}

/// The interaction state of a single field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldState {
    /// Editable.
    Editable,
    /// Read-only because it is locked.
    ReadOnlyLocked,
    /// Disabled because a prerequisite blocks it.
    DisabledBlocked,
    /// A masked credential entry (the value itself never crosses this boundary).
    MaskedCredential,
    /// A computed/derived value.
    ComputedDerived,
}

impl FieldState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editable => "editable",
            Self::ReadOnlyLocked => "read_only_locked",
            Self::DisabledBlocked => "disabled_blocked",
            Self::MaskedCredential => "masked_credential",
            Self::ComputedDerived => "computed_derived",
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
}

/// Draft-versus-applied state of a form session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftState {
    /// Clean: applied state, no pending edits.
    CleanApplied,
    /// Dirty: unsaved local edits.
    DirtyDraft,
    /// An autosaved local draft.
    AutosavedDraft,
    /// A draft recovered after an interruption.
    RecoveredDraft,
    /// A submit is in flight.
    Submitting,
    /// A submit failed but the draft is recoverable.
    SubmitFailedRecoverable,
}

impl DraftState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanApplied => "clean_applied",
            Self::DirtyDraft => "dirty_draft",
            Self::AutosavedDraft => "autosaved_draft",
            Self::RecoveredDraft => "recovered_draft",
            Self::Submitting => "submitting",
            Self::SubmitFailedRecoverable => "submit_failed_recoverable",
        }
    }
}

/// How a surface behaves on interruption/restart/reconnect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterruptionBehavior {
    /// Preserves a recoverable draft.
    PreservesRecoverableDraft,
    /// Prompts to restore on reopen.
    PromptsRestoreOnReopen,
    /// Discards only after an explicit confirmation.
    DiscardsWithExplicitConfirm,
    /// Reconnect resumes the session in place.
    ReconnectResumesInPlace,
    /// No recovery: work is lost.
    NoRecovery,
}

impl InterruptionBehavior {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservesRecoverableDraft => "preserves_recoverable_draft",
            Self::PromptsRestoreOnReopen => "prompts_restore_on_reopen",
            Self::DiscardsWithExplicitConfirm => "discards_with_explicit_confirm",
            Self::ReconnectResumesInPlace => "reconnect_resumes_in_place",
            Self::NoRecovery => "no_recovery",
        }
    }
}

/// The membership class of an item in a staged review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StagedReviewMemberClass {
    /// Included in the commit.
    Included,
    /// Excluded because it is a default.
    ExcludedByDefault,
    /// Excluded by the user.
    ExcludedByUser,
    /// Blocked by a prerequisite.
    BlockedPrerequisite,
}

impl StagedReviewMemberClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Included => "included",
            Self::ExcludedByDefault => "excluded_by_default",
            Self::ExcludedByUser => "excluded_by_user",
            Self::BlockedPrerequisite => "blocked_prerequisite",
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
}

/// The class of a submit blocker.
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

/// How the surface and its backing values originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceOrigin {
    /// First-party local authoring.
    LocalAuthoring,
    /// First-party authoring against a remote target.
    RemoteTarget,
    /// A provider-backed form.
    ProviderBacked,
    /// A review of imported/migrated/restored state (an overlay).
    ImportedOrRestore,
}

impl SurfaceOrigin {
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

/// The freshness of a surface's backing data.
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

/// The currency of a surface's verification proof.
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

/// Where a surface can reopen its draft to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenTarget {
    /// Reopens both the surface and its draft.
    SurfaceAndDraft,
    /// Reopens the draft only.
    DraftOnly,
    /// No reopen; only a keyboard fallback remains.
    NoneKeyboardFallback,
}

impl ReopenTarget {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceAndDraft => "surface_and_draft",
            Self::DraftOnly => "draft_only",
            Self::NoneKeyboardFallback => "none_keyboard_fallback",
        }
    }
}

/// Whether a surface is publicly claimed.
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

/// A consumer surface that renders a form record.
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

/// The effective claim a mutation-capable surface renders. A higher rank asserts
/// more authority, so a narrowed or floored surface must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClaim {
    /// The structured-input contract is broken: the form would submit from an
    /// ambiguous/source-hidden state, hide scope/defaults/prerequisites/rollback,
    /// or discard a recoverable draft. It must fall back to an explicit blocked
    /// state with a reopen/keyboard recovery path instead of a clean submit.
    #[serde(rename = "surface_unsafe")]
    Unsafe,
    /// A review of imported/migrated/restored state: attributable and reopenable
    /// but never reads as a local apply.
    #[serde(rename = "surface_review_overlay")]
    ReviewOverlay,
    /// A first-party surface held below certified by a labelled, recoverable gap
    /// (pending validation, stale source, stale proof); the draft stays
    /// recoverable and reopenable.
    #[serde(rename = "surface_narrowed")]
    Narrowed,
    /// Full source-explicit, validation-honest, scope-disclosed, recoverable,
    /// rollback-visible structured-input contract.
    #[serde(rename = "surface_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "surface_labs_not_claimed")]
    LabsNotClaimed,
}

impl SurfaceClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsafe => "surface_unsafe",
            Self::ReviewOverlay => "surface_review_overlay",
            Self::Narrowed => "surface_narrowed",
            Self::Certified => "surface_certified",
            Self::LabsNotClaimed => "surface_labs_not_claimed",
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
    /// claim. A rendering surface must never render wider than the surface's
    /// effective claim; the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: SurfaceClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a surface fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceNarrowingReason {
    /// A field's source-of-value class is hidden, or a user override is not
    /// distinct from the value it replaced.
    FieldSourceHidden,
    /// Draft versus applied state is not visibly distinct.
    DraftAppliedAmbiguous,
    /// A policy-locked value was silently overridden.
    PolicyLockOverriddenSilently,
    /// The form would submit while a field is invalid-blocking.
    SubmitAllowedWhileBlockingInvalid,
    /// A blocked prerequisite is not explained before submit.
    BlockedPrereqHidden,
    /// The staged review does not declare its target scope.
    TargetScopeHidden,
    /// The staged review hides omitted defaults.
    OmittedDefaultsHidden,
    /// A side effect is not disclosed before commit.
    SideEffectUndisclosed,
    /// The rollback/export consequence is not visible.
    RollbackConsequencesHidden,
    /// The commit action is a generic Continue that hides scope/effect.
    GenericContinueAction,
    /// A recoverable draft is discarded on interruption.
    DraftRecoveryLost,
    /// An imported/restore review reads as a local apply.
    ImportedStateReadsAsApplied,
    /// The reopen-to-origin path is lost.
    ReopenPathLost,
    /// A rendering surface renders wider than the effective claim.
    SurfaceOverclaims,
    /// Backing data is missing.
    FormBackingMissing,
    /// Validation state is not surfaced.
    ValidationStateUnlabeled,
    /// A cross-field dependency is not explained.
    CrossFieldDependencyUnexplained,
    /// Included/excluded/blocked members are not labelled.
    ExcludedMembersUnlabeled,
    /// Local draft autosave is unavailable.
    AutosaveUnavailable,
    /// No restore prompt on reopen (draft still preserved).
    RestorePromptMissing,
    /// Async validation is pending.
    AsyncValidationPending,
    /// The freshness state is not surfaced.
    FreshnessUnlabeled,
    /// A superseded backing source is not marked.
    SupersededStateNotMarked,
    /// A first-party surface is stale.
    SurfaceStale,
    /// The verification proof is stale.
    VerificationProofStale,
    /// The verification proof is missing.
    VerificationProofMissing,
}

impl SurfaceNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FieldSourceHidden => "field_source_hidden",
            Self::DraftAppliedAmbiguous => "draft_applied_ambiguous",
            Self::PolicyLockOverriddenSilently => "policy_lock_overridden_silently",
            Self::SubmitAllowedWhileBlockingInvalid => "submit_allowed_while_blocking_invalid",
            Self::BlockedPrereqHidden => "blocked_prereq_hidden",
            Self::TargetScopeHidden => "target_scope_hidden",
            Self::OmittedDefaultsHidden => "omitted_defaults_hidden",
            Self::SideEffectUndisclosed => "side_effect_undisclosed",
            Self::RollbackConsequencesHidden => "rollback_consequences_hidden",
            Self::GenericContinueAction => "generic_continue_action",
            Self::DraftRecoveryLost => "draft_recovery_lost",
            Self::ImportedStateReadsAsApplied => "imported_state_reads_as_applied",
            Self::ReopenPathLost => "reopen_path_lost",
            Self::SurfaceOverclaims => "surface_overclaims",
            Self::FormBackingMissing => "form_backing_missing",
            Self::ValidationStateUnlabeled => "validation_state_unlabeled",
            Self::CrossFieldDependencyUnexplained => "cross_field_dependency_unexplained",
            Self::ExcludedMembersUnlabeled => "excluded_members_unlabeled",
            Self::AutosaveUnavailable => "autosave_unavailable",
            Self::RestorePromptMissing => "restore_prompt_missing",
            Self::AsyncValidationPending => "async_validation_pending",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::SupersededStateNotMarked => "superseded_state_not_marked",
            Self::SurfaceStale => "surface_stale",
            Self::VerificationProofStale => "verification_proof_stale",
            Self::VerificationProofMissing => "verification_proof_missing",
        }
    }

    /// Deterministic ordering index (mirrors the validator's reason order).
    pub const fn order_index(self) -> u8 {
        match self {
            Self::FieldSourceHidden => 0,
            Self::DraftAppliedAmbiguous => 1,
            Self::PolicyLockOverriddenSilently => 2,
            Self::SubmitAllowedWhileBlockingInvalid => 3,
            Self::BlockedPrereqHidden => 4,
            Self::TargetScopeHidden => 5,
            Self::OmittedDefaultsHidden => 6,
            Self::SideEffectUndisclosed => 7,
            Self::RollbackConsequencesHidden => 8,
            Self::GenericContinueAction => 9,
            Self::DraftRecoveryLost => 10,
            Self::ImportedStateReadsAsApplied => 11,
            Self::ReopenPathLost => 12,
            Self::SurfaceOverclaims => 13,
            Self::FormBackingMissing => 14,
            Self::ValidationStateUnlabeled => 15,
            Self::CrossFieldDependencyUnexplained => 16,
            Self::ExcludedMembersUnlabeled => 17,
            Self::AutosaveUnavailable => 18,
            Self::RestorePromptMissing => 19,
            Self::AsyncValidationPending => 20,
            Self::FreshnessUnlabeled => 21,
            Self::SupersededStateNotMarked => 22,
            Self::SurfaceStale => 23,
            Self::VerificationProofStale => 24,
            Self::VerificationProofMissing => 25,
        }
    }

    /// Whether this reason breaks the contract outright (floors the surface to
    /// [`SurfaceClaim::Unsafe`]) rather than merely aging it out.
    pub const fn is_floor(self) -> bool {
        self.order_index() <= Self::FormBackingMissing.order_index()
    }

    /// A reviewer-facing, non-generic description of the reason.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::FieldSourceHidden => {
                "a field's source-of-value class is hidden or a user override is not distinct"
            }
            Self::DraftAppliedAmbiguous => "draft versus applied state is not visibly distinct",
            Self::PolicyLockOverriddenSilently => "a policy-locked value was silently overridden",
            Self::SubmitAllowedWhileBlockingInvalid => {
                "submit is reachable while a field is invalid-blocking"
            }
            Self::BlockedPrereqHidden => "a blocked prerequisite is not explained before submit",
            Self::TargetScopeHidden => "the staged review does not declare its target scope",
            Self::OmittedDefaultsHidden => "the staged review hides omitted defaults",
            Self::SideEffectUndisclosed => "a side effect is not disclosed before commit",
            Self::RollbackConsequencesHidden => "the rollback/export consequence is not visible",
            Self::GenericContinueAction => {
                "the commit action is a generic Continue that hides scope and effect"
            }
            Self::DraftRecoveryLost => "a recoverable draft is discarded on interruption",
            Self::ImportedStateReadsAsApplied => {
                "an imported/restore review reads as a local apply"
            }
            Self::ReopenPathLost => "the reopen-to-origin path is lost",
            Self::SurfaceOverclaims => "a rendering surface renders wider than the effective claim",
            Self::FormBackingMissing => "the backing data is missing",
            Self::ValidationStateUnlabeled => "the validation state is not surfaced",
            Self::CrossFieldDependencyUnexplained => "a cross-field dependency is not explained",
            Self::ExcludedMembersUnlabeled => "included/excluded/blocked members are not labelled",
            Self::AutosaveUnavailable => "local draft autosave is unavailable",
            Self::RestorePromptMissing => "there is no restore prompt on reopen",
            Self::AsyncValidationPending => "async validation is still pending",
            Self::FreshnessUnlabeled => "the backing freshness state is not surfaced",
            Self::SupersededStateNotMarked => "a superseded backing source is not marked",
            Self::SurfaceStale => "the backing source is stale",
            Self::VerificationProofStale => "the verification proof is stale",
            Self::VerificationProofMissing => "the verification proof is missing",
        }
    }
}

fn order_reasons(mut reasons: Vec<SurfaceNarrowingReason>) -> Vec<SurfaceNarrowingReason> {
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

/// Stable identifiers binding a surface to its origin. Absent refs serialize as
/// `null` so the schema's required keys stay present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceLineage {
    /// Form-session/context ref (required).
    pub session_ref: String,
    /// The surface's own stable canonical ref (required for a real surface).
    pub canonical_surface_ref: Option<String>,
    /// Origin target ref (workspace/profile/remote target).
    pub origin_target_ref: Option<String>,
    /// Provider ref (required for provider-backed/imported overlay surfaces).
    pub provider_ref: Option<String>,
    /// Imported/source-artifact ref backing the surface.
    pub source_artifact_ref: Option<String>,
    /// Local draft-store ref.
    pub draft_store_ref: Option<String>,
    /// Rollback-plan ref.
    pub rollback_plan_ref: Option<String>,
    /// Reopen backlink ref.
    pub reopen_backlink_ref: Option<String>,
}

/// The draft/applied state of a form session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormSessionState {
    /// The draft-versus-applied state.
    pub draft_state: DraftState,
    /// Draft is visibly distinct from applied.
    pub draft_applied_distinct: bool,
    /// Local draft autosave is enabled.
    pub autosave_enabled: bool,
    /// A local draft is persisted.
    pub local_draft_persisted: bool,
    /// A restore prompt is shown on reopen.
    pub restore_prompt_on_reopen: bool,
    /// Behavior on interruption/restart.
    pub interruption_behavior: InterruptionBehavior,
    /// Reconnect resumes the session in place.
    pub reconnect_resumes: bool,
    /// Number of dirty fields.
    pub dirty_field_count: u64,
    /// A last-autosave marker is present.
    pub last_autosave_present: bool,
}

/// One field's provenance, state, and validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldProvenanceRow {
    /// Stable field id.
    pub field_id: String,
    /// The source of the field's value.
    pub source_class: SourceOfValueClass,
    /// The source class is surfaced.
    pub source_class_labeled: bool,
    /// Field interaction state.
    pub field_state: FieldState,
    /// Field validation state.
    pub validation_state: ValidationState,
    /// The validation state is surfaced.
    pub validation_state_labeled: bool,
    /// A user override is visibly distinct from the default/detected/imported
    /// value it replaced.
    pub override_distinct_from_default: bool,
    /// A policy lock is respected (not silently overridden).
    pub policy_lock_respected: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// One submit blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitBlocker {
    /// Stable blocker id.
    pub blocker_id: String,
    /// The blocker class.
    pub blocker_class: SubmitBlockerClass,
    /// Whether the blocker is explained before submit.
    pub explained_before_submit: bool,
    /// Whether this blocker blocks submit.
    pub blocks_submit: bool,
    /// Whether a resolution hint is present.
    pub resolution_hint_present: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// One member of a staged review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedReviewMember {
    /// Stable member id.
    pub member_id: String,
    /// The membership class.
    pub member_class: StagedReviewMemberClass,
    /// Whether the member's reason is labelled.
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

/// The staged-review (commit) sheet for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedReviewPacket {
    /// The target scope is declared.
    pub target_scope_declared: bool,
    /// Reviewer-facing target-scope label.
    pub target_scope_label: String,
    /// Omitted defaults are disclosed.
    pub omitted_defaults_disclosed: bool,
    /// Count of omitted defaults.
    pub omitted_default_count: u64,
    /// Included/excluded/blocked members.
    pub members: Vec<StagedReviewMember>,
    /// Member classes are labelled.
    pub members_classes_labeled: bool,
    /// Declared side effects.
    pub side_effects: Vec<SideEffectDescriptor>,
    /// Side effects are disclosed before commit.
    pub side_effects_disclosed: bool,
    /// A rollback path is present.
    pub rollback_path_present: bool,
    /// An export path is present.
    pub export_path_present: bool,
    /// The commit action names the scope/effect rather than a generic Continue.
    pub commit_action_is_specific: bool,
}

/// The draft-recovery summary for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftRecoverySummary {
    /// A recoverable draft survives interruption.
    pub recoverable_after_interruption: bool,
    /// The recovery behavior.
    pub recovery_behavior: InterruptionBehavior,
    /// A draft recovery handle is present.
    pub recovery_token_present: bool,
    /// Reconnect is safe (resumes in place).
    pub reconnect_safe: bool,
    /// Reviewer-facing recovery label.
    pub recovery_label: String,
}

/// The headline structured-input invariants every surface re-derives rather than
/// trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceIntegrity {
    /// Field provenance survives into the surface.
    pub preserves_field_provenance: bool,
    /// Draft versus applied is visibly distinct.
    pub draft_applied_distinct: bool,
    /// The target scope is visible on the commit sheet.
    pub target_scope_visible: bool,
    /// Omitted defaults are visible.
    pub omitted_defaults_visible: bool,
    /// Blocked prerequisites are explained before submit.
    pub blocked_prereqs_explained: bool,
    /// The rollback consequence is visible.
    pub rollback_visible: bool,
    /// The commit action names scope/effect.
    pub commit_action_specific: bool,
    /// Imported/restore reviews stay read-only.
    pub imported_review_read_only: bool,
    /// A recoverable draft is preserved on interruption.
    pub recoverable_draft_preserved: bool,
    /// Validation state is visible.
    pub validation_state_visible: bool,
    /// Policy locks are respected.
    pub policy_locks_respected: bool,
    /// The freshness state is visible.
    pub freshness_state_visible: bool,
    /// A superseded backing source stays marked.
    pub superseded_state_marked: bool,
    /// Origin lineage / reopen is revealable on demand on every surface.
    pub reopen_visible_on_demand: bool,
}

/// Verification-proof currency for a surface (distinct from backing freshness).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceVerification {
    /// Currency of the verification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the surface.
    pub proof_ref: Option<String>,
}

/// One consumer surface that renders a form record, with the claim it shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRendering {
    /// The consumer surface.
    pub surface: ConsumerSurface,
    /// The claim this surface renders.
    pub rendered_claim: SurfaceClaim,
    /// Whether field/scope provenance is revealable here.
    pub provenance_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical surface this view re-renders.
    pub source_surface_ref: String,
}

// --------------------------------------------------------------------------- //
// Surface + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) mutation-capable form, wizard, or review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormSurfaceRecord {
    /// Stable surface id.
    pub surface_id: String,
    /// The kind of surface.
    pub surface_kind: FormSurfaceKind,
    /// The product lane.
    pub lane: FormLane,
    /// What backs the mutation.
    pub mutation_class: MutationBackingClass,
    /// How the surface/values originated.
    pub origin: SurfaceOrigin,
    /// Reviewer-facing label summary.
    pub label_summary: String,
    /// Whether the surface is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// Declared backing freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared reopen target.
    pub declared_reopen_target: ReopenTarget,
    /// Stable origin-lineage block.
    pub lineage: SurfaceLineage,
    /// Form-session draft/applied block.
    pub session: FormSessionState,
    /// Field-provenance rows.
    pub fields: Vec<FieldProvenanceRow>,
    /// Submit blockers.
    pub submit_blockers: Vec<SubmitBlocker>,
    /// Staged-review (commit) sheet.
    pub staged_review: StagedReviewPacket,
    /// Draft-recovery summary.
    pub draft_recovery: DraftRecoverySummary,
    /// Headline invariant block.
    pub integrity: SurfaceIntegrity,
    /// Verification-proof block.
    pub verification: SurfaceVerification,
    /// Consumer surfaces that render this record.
    pub renderings: Vec<SurfaceRendering>,
}

/// The re-derived surface decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceDecision {
    /// The headline claim the surface is eligible to make.
    pub claimed_claim: SurfaceClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_claim: SurfaceClaim,
    /// Ordered, de-duplicated reasons the surface fails to hold its headline.
    pub active_narrowing_reasons: Vec<SurfaceNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl SurfaceDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<SurfaceNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this record would overclaim.
    pub fn surface_overclaims(&self, rendered: SurfaceClaim) -> bool {
        self.effective_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(claimed: SurfaceClaim, reasons: &[SurfaceNarrowingReason]) -> SurfaceClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        SurfaceClaim::Unsafe
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, SurfaceClaim::ReviewOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we
        // can no longer certify even the read-only review, so it floors.
        SurfaceClaim::Unsafe
    } else {
        SurfaceClaim::Narrowed
    }
}

impl FormSurfaceRecord {
    /// Whether this surface is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this surface is an inherently read-only review overlay.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin.is_overlay()
    }

    /// The headline claim this surface is eligible to make.
    pub fn claimed_claim(&self) -> SurfaceClaim {
        if self.is_labs() {
            SurfaceClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            SurfaceClaim::ReviewOverlay
        } else {
            SurfaceClaim::Certified
        }
    }

    /// Reasons that hold independently of how the consumer surfaces render — the
    /// intrinsic provenance/validation/scope/recovery gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<SurfaceNarrowingReason> {
        use SurfaceNarrowingReason as R;
        let integ = &self.integrity;
        let session = &self.session;
        let review = &self.staged_review;
        let recovery = &self.draft_recovery;
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<R> = Vec::new();

        // Field provenance / source-of-value. A field whose source class is
        // hidden, or a user override that is not distinct from the value it
        // replaced, both hide where the value came from.
        for f in &self.fields {
            let override_not_distinct = matches!(f.source_class, SourceOfValueClass::UserOverride)
                && !f.override_distinct_from_default;
            if !f.source_class_labeled || override_not_distinct {
                reasons.push(R::FieldSourceHidden);
            }
            if matches!(f.source_class, SourceOfValueClass::PolicyLocked)
                && !f.policy_lock_respected
            {
                reasons.push(R::PolicyLockOverriddenSilently);
            }
            if !f.validation_state_labeled {
                reasons.push(R::ValidationStateUnlabeled);
            }
        }
        let has_blocking_invalid = self
            .fields
            .iter()
            .any(|f| matches!(f.validation_state, ValidationState::InvalidBlocking));
        let has_blocking_submit_blocker = self.submit_blockers.iter().any(|b| b.blocks_submit);
        if has_blocking_invalid && !has_blocking_submit_blocker {
            reasons.push(R::SubmitAllowedWhileBlockingInvalid);
        }

        // Headline integrity invariants.
        if !integ.preserves_field_provenance {
            reasons.push(R::FieldSourceHidden);
        }
        if !integ.draft_applied_distinct || !session.draft_applied_distinct {
            reasons.push(R::DraftAppliedAmbiguous);
        }
        if !integ.policy_locks_respected {
            reasons.push(R::PolicyLockOverriddenSilently);
        }

        // Submit blockers.
        for b in &self.submit_blockers {
            if b.blocks_submit && !b.explained_before_submit {
                match b.blocker_class {
                    SubmitBlockerClass::CrossFieldConflict => {
                        reasons.push(R::CrossFieldDependencyUnexplained);
                    }
                    SubmitBlockerClass::PendingValidation => {
                        reasons.push(R::AsyncValidationPending);
                    }
                    _ => reasons.push(R::BlockedPrereqHidden),
                }
            }
        }
        if !integ.blocked_prereqs_explained {
            reasons.push(R::BlockedPrereqHidden);
        }

        // Staged review.
        if !review.target_scope_declared || !integ.target_scope_visible {
            reasons.push(R::TargetScopeHidden);
        }
        if !review.omitted_defaults_disclosed || !integ.omitted_defaults_visible {
            reasons.push(R::OmittedDefaultsHidden);
        }
        if !review.members_classes_labeled {
            reasons.push(R::ExcludedMembersUnlabeled);
        }
        if review
            .side_effects
            .iter()
            .any(|s| !s.disclosed_before_commit)
            || !review.side_effects_disclosed
        {
            reasons.push(R::SideEffectUndisclosed);
        }
        if !review.rollback_path_present || !integ.rollback_visible {
            reasons.push(R::RollbackConsequencesHidden);
        }
        if !review.commit_action_is_specific || !integ.commit_action_specific {
            reasons.push(R::GenericContinueAction);
        }

        // Draft recovery + session.
        if !recovery.recoverable_after_interruption
            || matches!(recovery.recovery_behavior, InterruptionBehavior::NoRecovery)
            || !integ.recoverable_draft_preserved
        {
            reasons.push(R::DraftRecoveryLost);
        }
        if !session.autosave_enabled {
            reasons.push(R::AutosaveUnavailable);
        }
        if !session.restore_prompt_on_reopen {
            reasons.push(R::RestorePromptMissing);
        }

        // Validation visibility.
        if !integ.validation_state_visible {
            reasons.push(R::ValidationStateUnlabeled);
        }

        // Backing freshness.
        if !integ.freshness_state_visible {
            reasons.push(R::FreshnessUnlabeled);
        }
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::FormBackingMissing),
            FreshnessState::SupersededByNewerSource if !integ.superseded_state_marked => {
                reasons.push(R::SupersededStateNotMarked);
            }
            FreshnessState::StaleExpired if !overlay => reasons.push(R::SurfaceStale),
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
            reasons.push(R::ImportedStateReadsAsApplied);
        }

        // Reopen-to-origin.
        if !integ.reopen_visible_on_demand || self.renderings.iter().any(|r| !r.provenance_visible)
        {
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
    fn reasons(&self, stale_window: bool) -> Vec<SurfaceNarrowingReason> {
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic_effective = derive_effective(self.claimed_claim(), &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic_effective.overclaims_as(r.rendered_claim))
        {
            reasons.push(SurfaceNarrowingReason::SurfaceOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive this surface's claim decision.
    pub fn narrow(&self, stale_window: bool) -> SurfaceDecision {
        let claimed = self.claimed_claim();
        if matches!(claimed, SurfaceClaim::LabsNotClaimed) {
            return SurfaceDecision {
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
        SurfaceDecision {
            claimed_claim: claimed,
            effective_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// Whether a floored surface still keeps a reopen/keyboard recovery fallback
    /// rather than a misleading clean submit.
    pub fn floored_keeps_fallback(&self, effective: SurfaceClaim) -> bool {
        if !matches!(effective, SurfaceClaim::Unsafe) {
            return true;
        }
        matches!(
            self.declared_reopen_target,
            ReopenTarget::DraftOnly | ReopenTarget::NoneKeyboardFallback
        ) || opt_present(&self.lineage.reopen_backlink_ref)
    }

    /// Whether any rendering overclaims relative to `effective`.
    pub fn surface_overclaims(&self, effective: SurfaceClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// A reviewer-facing label for a narrowed/floored surface, or `None` if the
    /// surface holds its claim.
    pub fn narrowed_label(&self, decision: &SurfaceDecision) -> Option<String> {
        let trigger = decision.downgrade_trigger()?;
        Some(match decision.effective_claim {
            SurfaceClaim::Unsafe => format!(
                "Floored to surface_unsafe below the {} claim: {}; falls back to an explicit blocked state with reopen/keyboard recovery.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            SurfaceClaim::Narrowed => format!(
                "Held at surface_narrowed below the {} claim: {}; the draft stays recoverable and reopenable until re-verified.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            _ => return None,
        })
    }

    /// Append per-surface structural violations (schema-shape level).
    fn structural_violations(&self, out: &mut Vec<M5StructuredInputViolation>) {
        use M5StructuredInputViolation as V;
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
        if self.fields.is_empty() {
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
    }
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for [`M5StructuredInputSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StructuredInputSetInput {
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
    /// Per-surface rows.
    pub surfaces: Vec<FormSurfaceRecord>,
}

/// Export-safe M5 structured-input / staged-review set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StructuredInputSetPacket {
    /// Record kind; must equal [`M5_STRUCTURED_INPUT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STRUCTURED_INPUT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_STRUCTURED_INPUT_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-surface rows.
    pub surfaces: Vec<FormSurfaceRecord>,
}

/// The distribution of effective surface claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceClaimDistribution {
    /// Surfaces effective at [`SurfaceClaim::Certified`].
    pub certified: usize,
    /// Surfaces effective at [`SurfaceClaim::Narrowed`].
    pub narrowed: usize,
    /// Surfaces effective at [`SurfaceClaim::ReviewOverlay`].
    pub overlay: usize,
    /// Surfaces effective at [`SurfaceClaim::Unsafe`].
    pub unsafe_surfaces: usize,
    /// Surfaces effective at [`SurfaceClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5StructuredInputSetPacket {
    /// Builds a structured-input set packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5StructuredInputSetInput) -> Self {
        Self {
            record_kind: M5_STRUCTURED_INPUT_RECORD_KIND.to_owned(),
            schema_version: M5_STRUCTURED_INPUT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_STRUCTURED_INPUT_TAXONOMY_VERSION,
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

    /// Re-derive the decision for every surface, paired with its id.
    pub fn decisions(&self) -> Vec<(String, SurfaceDecision)> {
        let stale_window = self.stale_window();
        self.surfaces
            .iter()
            .map(|s| (s.surface_id.clone(), s.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective surface claims.
    pub fn claim_distribution(&self) -> SurfaceClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = SurfaceClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            unsafe_surfaces: 0,
            labs: 0,
        };
        for s in &self.surfaces {
            match s.narrow(stale_window).effective_claim {
                SurfaceClaim::Certified => dist.certified += 1,
                SurfaceClaim::Narrowed => dist.narrowed += 1,
                SurfaceClaim::ReviewOverlay => dist.overlay += 1,
                SurfaceClaim::Unsafe => dist.unsafe_surfaces += 1,
                SurfaceClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of surfaces whose effective claim ranks below their claimed claim.
    pub fn narrowed_surface_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.surfaces
            .iter()
            .filter(|s| s.narrow(stale_window).narrowed)
            .count()
    }

    /// Surface kinds represented by some surface.
    pub fn represented_kinds(&self) -> BTreeSet<FormSurfaceKind> {
        self.surfaces.iter().map(|s| s.surface_kind).collect()
    }

    /// Product lanes represented by some surface.
    pub fn represented_lanes(&self) -> BTreeSet<FormLane> {
        self.surfaces.iter().map(|s| s.lane).collect()
    }

    /// Mutation-backing classes represented by some surface.
    pub fn represented_mutation_classes(&self) -> BTreeSet<MutationBackingClass> {
        self.surfaces.iter().map(|s| s.mutation_class).collect()
    }

    /// Source-of-value classes represented by some field.
    pub fn represented_source_classes(&self) -> BTreeSet<SourceOfValueClass> {
        self.surfaces
            .iter()
            .flat_map(|s| s.fields.iter().map(|f| f.source_class))
            .collect()
    }

    /// Consumer surfaces represented by some rendering.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<ConsumerSurface> {
        self.surfaces
            .iter()
            .flat_map(|s| s.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Validate the structured-input invariants.
    pub fn validate(&self) -> Vec<M5StructuredInputViolation> {
        use M5StructuredInputViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_STRUCTURED_INPUT_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_STRUCTURED_INPUT_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_STRUCTURED_INPUT_TAXONOMY_VERSION {
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

        if FormSurfaceKind::ALL
            .iter()
            .any(|k| !self.represented_kinds().contains(k))
        {
            violations.push(V::SurfaceKindMissing);
        }
        if FormLane::ALL
            .iter()
            .any(|l| !self.represented_lanes().contains(l))
        {
            violations.push(V::FormLaneMissing);
        }
        if MutationBackingClass::ALL
            .iter()
            .any(|m| !self.represented_mutation_classes().contains(m))
        {
            violations.push(V::MutationClassMissing);
        }
        if SourceOfValueClass::ALL
            .iter()
            .any(|c| !self.represented_source_classes().contains(c))
        {
            violations.push(V::SourceOfValueClassMissing);
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
            &serde_json::to_value(self).expect("structured-input packet serializes"),
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
        serde_json::to_string_pretty(self).expect("structured-input packet serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_report(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str(
            "# M5 Structured Input, Parameter Provenance, Draft State, and Staged Review\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Surfaces: {}\n", self.surfaces.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} review overlay, {} unsafe, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.unsafe_surfaces, dist.labs
        ));

        out.push_str("| Surface | Kind | Lane | Mutation | Origin | Claimed | Effective |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for s in &self.surfaces {
            let decision = s.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                s.surface_id,
                s.surface_kind.as_str(),
                s.lane.as_str(),
                s.mutation_class.as_str(),
                s.origin.as_str(),
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
pub enum M5StructuredInputArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5StructuredInputViolation>),
}

impl fmt::Display for M5StructuredInputArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(f, "support export parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for M5StructuredInputArtifactError {}

/// A structured-input packet validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StructuredInputViolation {
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
    /// The set has no surfaces.
    EmptySurfaces,
    /// Two surfaces share a surface id.
    DuplicateSurfaceId,
    /// A surface kind is unrepresented.
    SurfaceKindMissing,
    /// A product lane is unrepresented.
    FormLaneMissing,
    /// A mutation-backing class is unrepresented.
    MutationClassMissing,
    /// A source-of-value class is unrepresented.
    SourceOfValueClassMissing,
    /// A consumer surface is unrepresented.
    ConsumerSurfaceMissing,
    /// A surface lacks a required identity field.
    SurfaceMissingIdentity,
    /// An overlay surface names no provider/source-artifact ref.
    OverlayMissingProvenanceRef,
    /// A surface has no fields.
    SurfaceMissingFields,
    /// A surface has no renderings.
    SurfaceMissingRendering,
    /// A rendering names no source surface ref.
    RenderingMissingSourceRef,
    /// A narrowed surface lacks a non-generic label or a downgrade trigger.
    NarrowedSurfaceMissingLabelOrTrigger,
    /// A floored surface loses its reopen/keyboard fallback.
    FlooredSurfaceLosesFallback,
    /// A rendering surface renders wider than the effective claim.
    RenderingSurfaceOverclaims,
    /// No surface demonstrates the auto-narrowing rule.
    DowngradedSurfaceCaseMissing,
    /// Raw boundary material crossed the export.
    RawBoundaryMaterialInExport,
}

impl M5StructuredInputViolation {
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
            Self::SurfaceKindMissing => "surface_kind_missing",
            Self::FormLaneMissing => "form_lane_missing",
            Self::MutationClassMissing => "mutation_class_missing",
            Self::SourceOfValueClassMissing => "source_of_value_class_missing",
            Self::ConsumerSurfaceMissing => "consumer_surface_missing",
            Self::SurfaceMissingIdentity => "surface_missing_identity",
            Self::OverlayMissingProvenanceRef => "overlay_missing_provenance_ref",
            Self::SurfaceMissingFields => "surface_missing_fields",
            Self::SurfaceMissingRendering => "surface_missing_rendering",
            Self::RenderingMissingSourceRef => "rendering_missing_source_ref",
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
/// structured-input matrix instead of minting per-feature form semantics.
///
/// # Errors
///
/// Returns [`M5StructuredInputArtifactError`] when the artifact cannot be parsed
/// or fails validation.
pub fn current_m5_structured_input_set(
) -> Result<M5StructuredInputSetPacket, M5StructuredInputArtifactError> {
    let packet: M5StructuredInputSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ux/m5-structured-input-and-staged-review/support_export.json"
    )))
    .map_err(M5StructuredInputArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5StructuredInputArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded structured-input set: the in-crate source of truth the
/// checked-in support export and report are regenerated from.
pub fn seeded_m5_structured_input_set() -> M5StructuredInputSetPacket {
    M5StructuredInputSetPacket::new(M5StructuredInputSetInput {
        packet_id: M5_STRUCTURED_INPUT_PACKET_ID.to_owned(),
        label:
            "M5 structured input — field provenance, validation, draft state, and staged review across mutation-capable forms"
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
    claim: SurfaceClaim,
    surfaces: &[ConsumerSurface],
    read_only: bool,
) -> Vec<SurfaceRendering> {
    surfaces
        .iter()
        .map(|&surface| SurfaceRendering {
            surface,
            rendered_claim: claim,
            provenance_visible: true,
            read_only,
            source_surface_ref: source_ref.to_owned(),
        })
        .collect()
}

/// A clean first-party integrity block.
fn clean_integrity() -> SurfaceIntegrity {
    SurfaceIntegrity {
        preserves_field_provenance: true,
        draft_applied_distinct: true,
        target_scope_visible: true,
        omitted_defaults_visible: true,
        blocked_prereqs_explained: true,
        rollback_visible: true,
        commit_action_specific: true,
        imported_review_read_only: true,
        recoverable_draft_preserved: true,
        validation_state_visible: true,
        policy_locks_respected: true,
        freshness_state_visible: true,
        superseded_state_marked: true,
        reopen_visible_on_demand: true,
    }
}

/// A clean first-party session block.
fn clean_session(draft_state: DraftState, dirty_field_count: u64) -> FormSessionState {
    FormSessionState {
        draft_state,
        draft_applied_distinct: true,
        autosave_enabled: true,
        local_draft_persisted: true,
        restore_prompt_on_reopen: true,
        interruption_behavior: InterruptionBehavior::PreservesRecoverableDraft,
        reconnect_resumes: true,
        dirty_field_count,
        last_autosave_present: true,
    }
}

/// A clean draft-recovery block.
fn clean_recovery(label: &str) -> DraftRecoverySummary {
    DraftRecoverySummary {
        recoverable_after_interruption: true,
        recovery_behavior: InterruptionBehavior::PromptsRestoreOnReopen,
        recovery_token_present: true,
        reconnect_safe: true,
        recovery_label: label.to_owned(),
    }
}

/// A verified-current verification block.
fn verified(proof_ref: &str) -> SurfaceVerification {
    SurfaceVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

fn field(
    field_id: &str,
    source_class: SourceOfValueClass,
    field_state: FieldState,
    validation_state: ValidationState,
    label: &str,
) -> FieldProvenanceRow {
    FieldProvenanceRow {
        field_id: field_id.to_owned(),
        source_class,
        source_class_labeled: true,
        field_state,
        validation_state,
        validation_state_labeled: true,
        override_distinct_from_default: true,
        policy_lock_respected: true,
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

fn member(
    member_id: &str,
    member_class: StagedReviewMemberClass,
    label: &str,
) -> StagedReviewMember {
    StagedReviewMember {
        member_id: member_id.to_owned(),
        member_class,
        reason_labeled: true,
        label_summary: label.to_owned(),
    }
}

/// The canonical surfaces: one (or more) per product lane, covering every surface
/// kind, mutation class, source-of-value class, and consumer surface, plus a
/// narrowed first-party surface, a review overlay, and a Labs surface.
fn seed_surfaces() -> Vec<FormSurfaceRecord> {
    use ConsumerSurface as CS;

    let provider = FormSurfaceRecord {
        surface_id: "form:provider-credentials:0001".to_owned(),
        surface_kind: FormSurfaceKind::StructuredForm,
        lane: FormLane::Provider,
        mutation_class: MutationBackingClass::ProviderBacked,
        origin: SurfaceOrigin::ProviderBacked,
        label_summary: "Provider connection form: detected endpoint, policy-locked region, and user-set credential reference each labelled by source.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SurfaceAndDraft,
        lineage: SurfaceLineage {
            session_ref: "form-session.provider.primary".to_owned(),
            canonical_surface_ref: Some("surface.provider.credentials.0001".to_owned()),
            origin_target_ref: Some("target.workspace.primary".to_owned()),
            provider_ref: Some("provider.connection.primary".to_owned()),
            source_artifact_ref: None,
            draft_store_ref: Some("draft.provider.credentials.0001".to_owned()),
            rollback_plan_ref: Some("rollback.provider.credentials.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.provider.credentials.0001".to_owned()),
        },
        session: clean_session(DraftState::DirtyDraft, 1),
        fields: vec![
            field("endpoint", SourceOfValueClass::DetectedValue, FieldState::Editable, ValidationState::Valid, "Endpoint detected from the workspace, editable."),
            field("region", SourceOfValueClass::PolicyLocked, FieldState::ReadOnlyLocked, ValidationState::Valid, "Region locked by organization policy."),
            field("credential_reference", SourceOfValueClass::UserOverride, FieldState::MaskedCredential, ValidationState::Warning, "User-set credential reference (masked); warning until first use."),
        ],
        submit_blockers: vec![],
        staged_review: StagedReviewPacket {
            target_scope_declared: true,
            target_scope_label: "Applies to the primary workspace provider connection.".to_owned(),
            omitted_defaults_disclosed: true,
            omitted_default_count: 2,
            members: vec![
                member("endpoint", StagedReviewMemberClass::Included, "Endpoint included."),
                member("timeout", StagedReviewMemberClass::ExcludedByDefault, "Timeout left at default."),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect("reconnect", SideEffectClass::ReversibleLocal, true, "Reconnects the provider session.")],
            side_effects_disclosed: true,
            rollback_path_present: true,
            export_path_present: true,
            commit_action_is_specific: true,
        },
        draft_recovery: clean_recovery("Draft of the provider form is autosaved and restored on reopen."),
        integrity: clean_integrity(),
        verification: verified("proof.provider.credentials.0001"),
        renderings: renderings("surface.provider.credentials.0001", SurfaceClaim::Certified, &[CS::FormView, CS::DiagnosticsPanel, CS::SupportExport], false),
    };

    let settings = FormSurfaceRecord {
        surface_id: "form:settings-config:0001".to_owned(),
        surface_kind: FormSurfaceKind::StructuredForm,
        lane: FormLane::Settings,
        mutation_class: MutationBackingClass::Local,
        origin: SurfaceOrigin::LocalAuthoring,
        label_summary: "Settings editor: defaults and user overrides stay distinct, with a scoped apply preview.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SurfaceAndDraft,
        lineage: SurfaceLineage {
            session_ref: "form-session.settings.primary".to_owned(),
            canonical_surface_ref: Some("surface.settings.config.0001".to_owned()),
            origin_target_ref: Some("scope.user.profile".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            draft_store_ref: Some("draft.settings.config.0001".to_owned()),
            rollback_plan_ref: Some("rollback.settings.config.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.settings.config.0001".to_owned()),
        },
        session: clean_session(DraftState::CleanApplied, 0),
        fields: vec![
            field("theme", SourceOfValueClass::DefaultValue, FieldState::Editable, ValidationState::Valid, "Theme at its default value."),
            field("font_size", SourceOfValueClass::UserOverride, FieldState::Editable, ValidationState::Valid, "Font size overridden by the user, distinct from default."),
        ],
        submit_blockers: vec![],
        staged_review: StagedReviewPacket {
            target_scope_declared: true,
            target_scope_label: "Applies to the user profile scope.".to_owned(),
            omitted_defaults_disclosed: true,
            omitted_default_count: 14,
            members: vec![
                member("font_size", StagedReviewMemberClass::Included, "Font size included."),
                member("theme", StagedReviewMemberClass::ExcludedByDefault, "Theme left at default."),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect("reload_ui", SideEffectClass::ReversibleLocal, true, "Reloads the UI surfaces.")],
            side_effects_disclosed: true,
            rollback_path_present: true,
            export_path_present: true,
            commit_action_is_specific: true,
        },
        draft_recovery: clean_recovery("Settings draft is autosaved per scope and restored on reopen."),
        integrity: clean_integrity(),
        verification: verified("proof.settings.config.0001"),
        renderings: renderings("surface.settings.config.0001", SurfaceClaim::Certified, &[CS::FormView, CS::HelpInline], false),
    };

    let projects = FormSurfaceRecord {
        surface_id: "wizard:project-bootstrap:0001".to_owned(),
        surface_kind: FormSurfaceKind::MultiStepWizard,
        lane: FormLane::Projects,
        mutation_class: MutationBackingClass::Local,
        origin: SurfaceOrigin::LocalAuthoring,
        label_summary: "Project bootstrap wizard: a required-unset name blocks submit with an explained blocker; defaults and overrides stay distinct.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SurfaceAndDraft,
        lineage: SurfaceLineage {
            session_ref: "form-session.projects.bootstrap".to_owned(),
            canonical_surface_ref: Some("surface.projects.bootstrap.0001".to_owned()),
            origin_target_ref: Some("target.local.folder".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            draft_store_ref: Some("draft.projects.bootstrap.0001".to_owned()),
            rollback_plan_ref: Some("rollback.projects.bootstrap.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.projects.bootstrap.0001".to_owned()),
        },
        session: clean_session(DraftState::DirtyDraft, 2),
        fields: vec![
            field("project_name", SourceOfValueClass::RequiredUnset, FieldState::Editable, ValidationState::InvalidBlocking, "Project name required and not yet set."),
            field("template", SourceOfValueClass::DefaultValue, FieldState::Editable, ValidationState::Valid, "Template at its default value."),
            field("license", SourceOfValueClass::UserOverride, FieldState::Editable, ValidationState::Valid, "License overridden by the user."),
        ],
        submit_blockers: vec![SubmitBlocker {
            blocker_id: "name_required".to_owned(),
            blocker_class: SubmitBlockerClass::InvalidField,
            explained_before_submit: true,
            blocks_submit: true,
            resolution_hint_present: true,
            label_summary: "Enter a project name before continuing.".to_owned(),
        }],
        staged_review: StagedReviewPacket {
            target_scope_declared: true,
            target_scope_label: "Creates a new project in the selected local folder.".to_owned(),
            omitted_defaults_disclosed: true,
            omitted_default_count: 5,
            members: vec![
                member("template", StagedReviewMemberClass::Included, "Template files included."),
                member("git_init", StagedReviewMemberClass::ExcludedByUser, "Git init excluded by the user."),
                member("project_name", StagedReviewMemberClass::BlockedPrerequisite, "Blocked until a name is set."),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect("write_files", SideEffectClass::ReversibleWithExport, true, "Writes scaffold files to disk.")],
            side_effects_disclosed: true,
            rollback_path_present: true,
            export_path_present: true,
            commit_action_is_specific: true,
        },
        draft_recovery: clean_recovery("Wizard step state is preserved and restored on reopen."),
        integrity: clean_integrity(),
        verification: verified("proof.projects.bootstrap.0001"),
        renderings: renderings("surface.projects.bootstrap.0001", SurfaceClaim::Certified, &[CS::WizardStep, CS::SupportExport], false),
    };

    let package = FormSurfaceRecord {
        surface_id: "sheet:package-install-review:0001".to_owned(),
        surface_kind: FormSurfaceKind::InstallReviewSheet,
        lane: FormLane::Package,
        mutation_class: MutationBackingClass::Local,
        origin: SurfaceOrigin::LocalAuthoring,
        label_summary: "Install review sheet: imported lockfile versions and defaults distinct; included/excluded/blocked members and side effects disclosed before commit.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SurfaceAndDraft,
        lineage: SurfaceLineage {
            session_ref: "form-session.package.install".to_owned(),
            canonical_surface_ref: Some("surface.package.install.0001".to_owned()),
            origin_target_ref: Some("target.workspace.primary".to_owned()),
            provider_ref: None,
            source_artifact_ref: Some("artifact.lockfile.0001".to_owned()),
            draft_store_ref: Some("draft.package.install.0001".to_owned()),
            rollback_plan_ref: Some("rollback.package.install.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.package.install.0001".to_owned()),
        },
        session: clean_session(DraftState::DirtyDraft, 1),
        fields: vec![
            field("resolved_versions", SourceOfValueClass::ImportedValue, FieldState::ReadOnlyLocked, ValidationState::Valid, "Resolved versions imported from the lockfile."),
            field("install_scripts", SourceOfValueClass::DefaultValue, FieldState::Editable, ValidationState::Warning, "Install scripts at default (disabled); warning shown."),
        ],
        submit_blockers: vec![],
        staged_review: StagedReviewPacket {
            target_scope_declared: true,
            target_scope_label: "Installs 12 packages into the primary workspace.".to_owned(),
            omitted_defaults_disclosed: true,
            omitted_default_count: 3,
            members: vec![
                member("dep_alpha", StagedReviewMemberClass::Included, "Direct dependency included."),
                member("dep_beta_transitive", StagedReviewMemberClass::ExcludedByDefault, "Transitive dev dependency excluded by default."),
                member("dep_gamma_native", StagedReviewMemberClass::BlockedPrerequisite, "Blocked: native toolchain prerequisite missing."),
            ],
            members_classes_labeled: true,
            side_effects: vec![
                side_effect("run_scripts", SideEffectClass::PolicyGoverned, false, "Lifecycle scripts gated by policy."),
                side_effect("write_lockfile", SideEffectClass::ReversibleWithExport, true, "Writes the lockfile."),
            ],
            side_effects_disclosed: true,
            rollback_path_present: true,
            export_path_present: true,
            commit_action_is_specific: true,
        },
        draft_recovery: clean_recovery("Install selection is preserved and restored on reopen."),
        integrity: clean_integrity(),
        verification: verified("proof.package.install.0001"),
        renderings: renderings("surface.package.install.0001", SurfaceClaim::Certified, &[CS::ReviewSheet, CS::AiEvidence], false),
    };

    let admin = FormSurfaceRecord {
        surface_id: "sheet:admin-policy-rollout:0001".to_owned(),
        surface_kind: FormSurfaceKind::PublishReviewDialog,
        lane: FormLane::Admin,
        mutation_class: MutationBackingClass::PolicyLocked,
        origin: SurfaceOrigin::LocalAuthoring,
        label_summary: "Admin policy-rollout review: policy-locked values respected; target scope, side effects, and rollback disclosed before commit.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::SurfaceAndDraft,
        lineage: SurfaceLineage {
            session_ref: "form-session.admin.policy".to_owned(),
            canonical_surface_ref: Some("surface.admin.policy.0001".to_owned()),
            origin_target_ref: Some("scope.organization".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            draft_store_ref: Some("draft.admin.policy.0001".to_owned()),
            rollback_plan_ref: Some("rollback.admin.policy.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.admin.policy.0001".to_owned()),
        },
        session: clean_session(DraftState::DirtyDraft, 1),
        fields: vec![
            field("enforced_baseline", SourceOfValueClass::PolicyLocked, FieldState::ReadOnlyLocked, ValidationState::Valid, "Enforced baseline locked by a higher policy tier."),
            field("rollout_ring", SourceOfValueClass::DefaultValue, FieldState::Editable, ValidationState::Valid, "Rollout ring at its default."),
        ],
        submit_blockers: vec![],
        staged_review: StagedReviewPacket {
            target_scope_declared: true,
            target_scope_label: "Rolls out the policy to the organization staged ring.".to_owned(),
            omitted_defaults_disclosed: true,
            omitted_default_count: 4,
            members: vec![
                member("rollout_ring", StagedReviewMemberClass::Included, "Rollout ring included."),
                member("enforced_baseline", StagedReviewMemberClass::ExcludedByDefault, "Enforced baseline left untouched (policy-locked)."),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect("apply_policy", SideEffectClass::PolicyGoverned, true, "Applies the policy to the staged ring; reversible by re-running rollback.")],
            side_effects_disclosed: true,
            rollback_path_present: true,
            export_path_present: true,
            commit_action_is_specific: true,
        },
        draft_recovery: clean_recovery("Policy-rollout draft is preserved and restored on reopen."),
        integrity: clean_integrity(),
        verification: verified("proof.admin.policy.0001"),
        renderings: renderings("surface.admin.policy.0001", SurfaceClaim::Certified, &[CS::ReviewSheet, CS::SupportExport], false),
    };

    // A remote parameterized run dialog narrowed by a verification proof that
    // requires review: it stays recoverable and reopenable, and its renderings
    // honestly show the narrowed claim.
    let request = FormSurfaceRecord {
        surface_id: "dialog:request-workspace-run:0001".to_owned(),
        surface_kind: FormSurfaceKind::ParameterizedWorkflow,
        lane: FormLane::Request,
        mutation_class: MutationBackingClass::Remote,
        origin: SurfaceOrigin::RemoteTarget,
        label_summary: "Request-workspace run dialog: parameter sources labelled; narrowed while a remote verification proof requires review.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::CachedSnapshot,
        declared_reopen_target: ReopenTarget::SurfaceAndDraft,
        lineage: SurfaceLineage {
            session_ref: "form-session.request.run".to_owned(),
            canonical_surface_ref: Some("surface.request.run.0001".to_owned()),
            origin_target_ref: Some("target.remote.workspace".to_owned()),
            provider_ref: Some("provider.remote.workspace".to_owned()),
            source_artifact_ref: None,
            draft_store_ref: Some("draft.request.run.0001".to_owned()),
            rollback_plan_ref: Some("rollback.request.run.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.request.run.0001".to_owned()),
        },
        session: clean_session(DraftState::DirtyDraft, 2),
        fields: vec![
            field("environment", SourceOfValueClass::DetectedValue, FieldState::Editable, ValidationState::Valid, "Environment detected from the remote target."),
            field("parameters", SourceOfValueClass::UserOverride, FieldState::Editable, ValidationState::Valid, "Run parameters overridden by the user."),
        ],
        submit_blockers: vec![],
        staged_review: StagedReviewPacket {
            target_scope_declared: true,
            target_scope_label: "Runs against the remote workspace target.".to_owned(),
            omitted_defaults_disclosed: true,
            omitted_default_count: 6,
            members: vec![
                member("parameters", StagedReviewMemberClass::Included, "User parameters included."),
                member("retries", StagedReviewMemberClass::ExcludedByDefault, "Retry policy left at default."),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect("remote_execute", SideEffectClass::ExternalPublish, false, "Executes against the remote target.")],
            side_effects_disclosed: true,
            rollback_path_present: true,
            export_path_present: true,
            commit_action_is_specific: true,
        },
        draft_recovery: clean_recovery("Run parameters are preserved; reconnect resumes the dialog in place."),
        integrity: clean_integrity(),
        verification: SurfaceVerification {
            proof_currency: ProofCurrency::RequiresReview,
            proof_ref: Some("proof.request.run.0001".to_owned()),
        },
        renderings: renderings("surface.request.run.0001", SurfaceClaim::Narrowed, &[CS::FormView, CS::AiEvidence], false),
    };

    // An import/restore review overlay: it reviews migrated state read-only and
    // never reads as a local apply.
    let import = FormSurfaceRecord {
        surface_id: "dialog:migration-restore-review:0001".to_owned(),
        surface_kind: FormSurfaceKind::ImportRestoreDialog,
        lane: FormLane::Import,
        mutation_class: MutationBackingClass::ImportExport,
        origin: SurfaceOrigin::ImportedOrRestore,
        label_summary: "Migration-center restore review: imported values shown read-only with target scope and rollback, never as a local apply.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::CachedSnapshot,
        declared_reopen_target: ReopenTarget::SurfaceAndDraft,
        lineage: SurfaceLineage {
            session_ref: "form-session.import.restore".to_owned(),
            canonical_surface_ref: Some("surface.import.restore.0001".to_owned()),
            origin_target_ref: Some("target.workspace.primary".to_owned()),
            provider_ref: Some("provider.migration.center".to_owned()),
            source_artifact_ref: Some("artifact.import.backup.0001".to_owned()),
            draft_store_ref: Some("draft.import.restore.0001".to_owned()),
            rollback_plan_ref: Some("rollback.import.restore.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.import.restore.0001".to_owned()),
        },
        session: clean_session(DraftState::DirtyDraft, 1),
        fields: vec![
            field("restored_settings", SourceOfValueClass::ImportedValue, FieldState::ReadOnlyLocked, ValidationState::Valid, "Settings imported from the backup, shown read-only."),
            field("merge_strategy", SourceOfValueClass::DefaultValue, FieldState::Editable, ValidationState::Valid, "Merge strategy at its default."),
        ],
        submit_blockers: vec![],
        staged_review: StagedReviewPacket {
            target_scope_declared: true,
            target_scope_label: "Restores into the primary workspace from the imported backup.".to_owned(),
            omitted_defaults_disclosed: true,
            omitted_default_count: 7,
            members: vec![
                member("restored_settings", StagedReviewMemberClass::Included, "Imported settings included."),
                member("device_keys", StagedReviewMemberClass::ExcludedByDefault, "Device keys excluded by default."),
            ],
            members_classes_labeled: true,
            side_effects: vec![side_effect("restore_apply", SideEffectClass::ReversibleWithExport, true, "Applies the restore; reversible if the current state is exported first.")],
            side_effects_disclosed: true,
            rollback_path_present: true,
            export_path_present: true,
            commit_action_is_specific: true,
        },
        draft_recovery: clean_recovery("Restore selection is preserved and restored on reopen."),
        integrity: clean_integrity(),
        verification: SurfaceVerification {
            proof_currency: ProofCurrency::CachedWithinWindow,
            proof_ref: Some("proof.import.restore.0001".to_owned()),
        },
        renderings: renderings("surface.import.restore.0001", SurfaceClaim::ReviewOverlay, &[CS::ReviewSheet, CS::DiagnosticsPanel], true),
    };

    // A Labs/unadvertised onboarding wizard: makes no public claim.
    let labs = FormSurfaceRecord {
        surface_id: "wizard:experimental-onboarding:0001".to_owned(),
        surface_kind: FormSurfaceKind::MultiStepWizard,
        lane: FormLane::Projects,
        mutation_class: MutationBackingClass::Local,
        origin: SurfaceOrigin::LocalAuthoring,
        label_summary: "Experimental onboarding wizard behind a Labs flag; makes no public structured-input claim.".to_owned(),
        claim_posture: ClaimPosture::LabsUnadvertised,
        declared_freshness_state: FreshnessState::Live,
        declared_reopen_target: ReopenTarget::DraftOnly,
        lineage: SurfaceLineage {
            session_ref: "form-session.labs.onboarding".to_owned(),
            canonical_surface_ref: Some("surface.labs.onboarding.0001".to_owned()),
            origin_target_ref: Some("target.local.folder".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            draft_store_ref: Some("draft.labs.onboarding.0001".to_owned()),
            rollback_plan_ref: None,
            reopen_backlink_ref: Some("reopen.labs.onboarding.0001".to_owned()),
        },
        session: clean_session(DraftState::DirtyDraft, 1),
        fields: vec![field(
            "experiment_choice",
            SourceOfValueClass::DefaultValue,
            FieldState::Editable,
            ValidationState::NotValidated,
            "Experimental choice; not yet validated.",
        )],
        submit_blockers: vec![],
        staged_review: StagedReviewPacket {
            target_scope_declared: true,
            target_scope_label: "Experimental; scope not publicly claimed.".to_owned(),
            omitted_defaults_disclosed: true,
            omitted_default_count: 0,
            members: vec![member("experiment_choice", StagedReviewMemberClass::Included, "Experimental choice included.")],
            members_classes_labeled: true,
            side_effects: vec![side_effect("labs_apply", SideEffectClass::ReversibleLocal, true, "Applies the experimental choice locally.")],
            side_effects_disclosed: true,
            rollback_path_present: true,
            export_path_present: false,
            commit_action_is_specific: true,
        },
        draft_recovery: clean_recovery("Experimental draft is preserved locally."),
        integrity: clean_integrity(),
        verification: SurfaceVerification {
            proof_currency: ProofCurrency::MissingProof,
            proof_ref: None,
        },
        renderings: renderings("surface.labs.onboarding.0001", SurfaceClaim::LabsNotClaimed, &[CS::WizardStep], false),
    };

    vec![
        provider, settings, projects, package, admin, request, import, labs,
    ]
}
