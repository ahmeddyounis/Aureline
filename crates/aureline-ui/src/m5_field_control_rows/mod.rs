//! Canonical field- and control-row primitive contract for mutation-capable M5
//! forms, wizards, and review sheets.
//!
//! Where [`crate::m5_structured_input_and_staged_review`] freezes the *per-surface*
//! honesty claim of a whole form, this module freezes the *per-row* primitive it is
//! built from. Every launch-critical M5 form — provider/account mapping, source
//! registration, request-environment, package/install, and migration/import — is a
//! column of field and control rows, and each row must, on its own, tell the user
//! exactly what a value is, where it came from, and why it is currently blocked or
//! constrained, without leaving the form. Each [`FieldControlRow`] binds:
//!
//! * a **permanent label** ([`LabelMode`]) — the field's name stays visible rather
//!   than living in a placeholder that vanishes on focus, and a [`Requirement`] tag
//!   makes required/optional/conditional/system-managed status explicit;
//! * a **source-of-value tag** ([`SourceOfValueClass`]) shown on the row — default,
//!   detected, imported, policy-locked, user-override, or still-unset — and a user
//!   override stays visibly distinct from the value it replaced;
//! * an **exact validation anchor** ([`RowValidation`]) — a blocking or warning
//!   validation message is anchored directly to the field with exact rule text,
//!   never deferred to a form-level summary banner alone;
//! * a **lifecycle implication** ([`RowLifecycle`]) — restart-required,
//!   reconnect-required, trust-required, or policy-blocked state is surfaced on the
//!   affected control itself rather than only through a generic banner; and
//! * a **backing-freshness and verification** block so a stale, superseded, or
//!   unproven backing value narrows the row instead of reading as fresh.
//!
//! Each row re-derives a [`RowClaim`] ([`FieldControlRow::narrow`]) so a row can
//! never read wider than its evidence: a row that hides its label behind a
//! placeholder, hides its source-of-value tag, silently overrides a policy lock,
//! defers a blocking validation to a summary banner, or buries a
//! restart/reconnect/trust/policy implication in a generic banner floors to
//! [`RowClaim::Blocked`] and falls back to an explicit disabled state that shows the
//! reason on the row. A labelled, recoverable gap (pending async validation, a
//! stale/superseded backing source, a stale or missing verification proof, or an
//! unmarked requirement) holds a first-party row at [`RowClaim::Narrowed`] while it
//! stays usable, an imported/restore value sits at [`RowClaim::ReviewOverlay`] and
//! never reads as an editable local value, and a Labs/unadvertised row makes no
//! public claim.
//!
//! [`M5FieldControlRowSetPacket::validate`] confirms the set is well-formed and
//! honest: header/identity/redaction/freshness are present, every consumer lane,
//! source-of-value class, lifecycle implication, requirement class, and consumer
//! render surface is represented, overlay rows name their provenance, no rendering
//! overclaims, a floored row keeps its on-row fallback, at least one row
//! demonstrates the auto-narrowing rule, and no raw credential/body material crosses
//! the export. Downstream provider, source, request, package, import, settings, and
//! support surfaces ingest these row primitives rather than minting per-feature
//! field semantics.
//!
//! No credential bodies, secret values, raw provider payloads, absolute paths, or
//! URLs ever cross this boundary; the packet carries only typed class tokens,
//! counts, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/ux/m5-field-control-rows.schema.json`](../../../../schemas/ux/m5-field-control-rows.schema.json).
//! The contract doc is
//! [`docs/ux/m5-field-control-rows.md`](../../../../docs/ux/m5-field-control-rows.md).
//! The canonical support export is
//! [`artifacts/ux/m5-field-control-rows/support_export.json`](../../../../artifacts/ux/m5-field-control-rows/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/ux/m5-field-control-rows/`](../../../../fixtures/ux/m5-field-control-rows/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The canonical field-level vocabulary is reused from the surface lane rather than
// re-minted here: source-of-value classes, field interaction state, validation
// state, backing freshness, verification-proof currency, and the consumer render
// surfaces are one taxonomy across the structured-input model.
pub use crate::m5_structured_input_and_staged_review::{
    ConsumerSurface, FieldState, FreshnessState, ProofCurrency, SourceOfValueClass, ValidationState,
};

/// Stable record-kind tag carried by [`M5FieldControlRowSetPacket`].
pub const M5_FIELD_CONTROL_ROW_RECORD_KIND: &str = "m5_field_control_row_set_packet";

/// Schema version for the field-control-row set.
pub const M5_FIELD_CONTROL_ROW_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_FIELD_CONTROL_ROW_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical field-control-row set packet.
pub const M5_FIELD_CONTROL_ROW_PACKET_ID: &str = "m5-field-control-rows:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_FIELD_CONTROL_ROW_SCHEMA_REF: &str = "schemas/ux/m5-field-control-rows.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_FIELD_CONTROL_ROW_DOC_REF: &str = "docs/ux/m5-field-control-rows.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_FIELD_CONTROL_ROW_SUPPORT_EXPORT_REF: &str =
    "artifacts/ux/m5-field-control-rows/support_export.json";

/// Repo-relative path of the generated report.
pub const M5_FIELD_CONTROL_ROW_REPORT_REF: &str = "artifacts/ux/m5-field-control-rows/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_FIELD_CONTROL_ROW_FIXTURE_DIR: &str = "fixtures/ux/m5-field-control-rows";

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

/// The first-consumer surface lane a row is adopted in. These are the
/// highest-risk mutation-capable M5 forms that prove the shared primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowConsumerLane {
    /// Provider/account mapping form.
    ProviderAccountMapping,
    /// Source registration form.
    SourceRegistration,
    /// Request-environment run dialog.
    RequestEnvironment,
    /// Package/install review sheet.
    PackageInstall,
    /// Migration/import restore review.
    MigrationImport,
}

impl RowConsumerLane {
    /// Every consumer lane, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderAccountMapping,
        Self::SourceRegistration,
        Self::RequestEnvironment,
        Self::PackageInstall,
        Self::MigrationImport,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAccountMapping => "provider_account_mapping",
            Self::SourceRegistration => "source_registration",
            Self::RequestEnvironment => "request_environment",
            Self::PackageInstall => "package_install",
            Self::MigrationImport => "migration_import",
        }
    }
}

/// Whether a row reflects first-party local state or an imported/restored review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowOrigin {
    /// A first-party, locally-editable row.
    FirstParty,
    /// A row that reviews imported/migrated/restored state (read-only overlay).
    ImportedOrRestore,
}

impl RowOrigin {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::ImportedOrRestore => "imported_or_restore",
        }
    }

    /// Whether this origin is an inherently read-only review overlay.
    pub const fn is_overlay(self) -> bool {
        matches!(self, Self::ImportedOrRestore)
    }
}

/// Whether a row is publicly claimed or Labs/unadvertised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowClaimPosture {
    /// Publicly claimed as stable.
    ClaimedStable,
    /// Labs/unadvertised; makes no public claim.
    LabsUnadvertised,
}

impl RowClaimPosture {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimedStable => "claimed_stable",
            Self::LabsUnadvertised => "labs_unadvertised",
        }
    }
}

/// How a field's label is presented. A permanent label keeps the field's name
/// visible at all times; a placeholder-only or absent label hides the field's
/// meaning during and after entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelMode {
    /// A permanent, always-visible label.
    Permanent,
    /// Only a placeholder that vanishes on focus/entry.
    PlaceholderOnly,
    /// No label at all.
    None,
}

impl LabelMode {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Permanent => "permanent",
            Self::PlaceholderOnly => "placeholder_only",
            Self::None => "none",
        }
    }
}

/// The required/optional clarity of a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Requirement {
    /// Required before submit.
    Required,
    /// Optional.
    Optional,
    /// Required only when a cross-field condition holds.
    Conditional,
    /// Managed by the system, not directly editable.
    SystemManaged,
}

impl Requirement {
    /// Every requirement class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Required,
        Self::Optional,
        Self::Conditional,
        Self::SystemManaged,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Conditional => "conditional",
            Self::SystemManaged => "system_managed",
        }
    }
}

/// A lifecycle/activation implication that attaches to a single control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleImplication {
    /// No lifecycle implication.
    None,
    /// Changing this control requires an application restart.
    RestartRequired,
    /// Changing this control requires reconnecting the backing account/session.
    ReconnectRequired,
    /// Changing this control requires re-establishing trust.
    TrustRequired,
    /// This control is blocked by policy.
    PolicyBlocked,
}

impl LifecycleImplication {
    /// Every lifecycle implication, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::RestartRequired,
        Self::ReconnectRequired,
        Self::TrustRequired,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RestartRequired => "restart_required",
            Self::ReconnectRequired => "reconnect_required",
            Self::TrustRequired => "trust_required",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The recovery presentation a blocked/floored row falls back to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedFallback {
    /// The blocked row shows its reason inline on the row.
    ShowsReasonOnRow,
    /// The control is disabled but carries a resolution hint.
    DisabledWithHint,
    /// The row goes silent with no reason and no hint.
    NoneSilent,
}

impl BlockedFallback {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShowsReasonOnRow => "shows_reason_on_row",
            Self::DisabledWithHint => "disabled_with_hint",
            Self::NoneSilent => "none_silent",
        }
    }

    /// Whether this fallback keeps a recoverable, attributable presentation.
    pub const fn keeps_recovery(self) -> bool {
        matches!(self, Self::ShowsReasonOnRow | Self::DisabledWithHint)
    }
}

// --------------------------------------------------------------------------- //
// Derived claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim a field/control row renders. A higher rank asserts more
/// authority, so a narrowed or floored row must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowClaim {
    /// The row primitive contract is broken: the row hides its label or source,
    /// silently overrides a policy lock, defers a blocking validation to a summary
    /// banner, or buries a lifecycle implication. It falls back to an explicit
    /// disabled/blocked state that shows the reason on the row.
    #[serde(rename = "row_blocked")]
    Blocked,
    /// A row that reviews imported/migrated/restored state: attributable but never
    /// reads as an editable local value.
    #[serde(rename = "row_review_overlay")]
    ReviewOverlay,
    /// A first-party row held below certified by a labelled, recoverable gap
    /// (pending async validation, stale/superseded backing, unmarked requirement);
    /// it stays usable and attributable.
    #[serde(rename = "row_narrowed")]
    Narrowed,
    /// Full permanent-label, requirement-clear, source-tagged, validation-anchored,
    /// lifecycle-explicit row primitive.
    #[serde(rename = "row_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "row_labs_not_claimed")]
    LabsNotClaimed,
}

impl RowClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "row_blocked",
            Self::ReviewOverlay => "row_review_overlay",
            Self::Narrowed => "row_narrowed",
            Self::Certified => "row_certified",
            Self::LabsNotClaimed => "row_labs_not_claimed",
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
    /// claim. A rendering must never render wider than the row's effective claim;
    /// the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: RowClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a row fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowNarrowingReason {
    /// The field's label is not permanent (placeholder-only or absent).
    LabelNotPermanent,
    /// The source-of-value tag is hidden, a user override is not distinct from the
    /// value it replaced, or a rendering cannot reveal the row's provenance/anchor.
    SourceTagHidden,
    /// A policy-locked value was silently overridden.
    PolicyLockOverridden,
    /// A blocking/warning validation is deferred to a summary banner instead of an
    /// exact, field-anchored rule.
    ValidationAnchorMissing,
    /// A restart/reconnect/trust/policy lifecycle implication is not surfaced on the
    /// affected control.
    LifecycleImplicationHidden,
    /// An imported/restore value row reads as an editable local value.
    ImportedValueReadsAsEditable,
    /// A rendering renders wider than the row's effective claim.
    RowOverclaims,
    /// The backing value is missing.
    RowBackingMissing,
    /// The required/optional status is not marked on the row.
    RequirementUnmarked,
    /// The validation state is not surfaced on the row.
    ValidationStateUnlabeled,
    /// Async validation is still pending.
    AsyncValidationPending,
    /// The backing freshness state is not surfaced.
    FreshnessUnlabeled,
    /// A superseded backing source is not marked.
    SupersededStateNotMarked,
    /// A first-party backing source is stale.
    RowStale,
    /// The verification proof is stale.
    VerificationProofStale,
    /// The verification proof is missing.
    VerificationProofMissing,
}

impl RowNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabelNotPermanent => "label_not_permanent",
            Self::SourceTagHidden => "source_tag_hidden",
            Self::PolicyLockOverridden => "policy_lock_overridden",
            Self::ValidationAnchorMissing => "validation_anchor_missing",
            Self::LifecycleImplicationHidden => "lifecycle_implication_hidden",
            Self::ImportedValueReadsAsEditable => "imported_value_reads_as_editable",
            Self::RowOverclaims => "row_overclaims",
            Self::RowBackingMissing => "row_backing_missing",
            Self::RequirementUnmarked => "requirement_unmarked",
            Self::ValidationStateUnlabeled => "validation_state_unlabeled",
            Self::AsyncValidationPending => "async_validation_pending",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::SupersededStateNotMarked => "superseded_state_not_marked",
            Self::RowStale => "row_stale",
            Self::VerificationProofStale => "verification_proof_stale",
            Self::VerificationProofMissing => "verification_proof_missing",
        }
    }

    /// Deterministic ordering index (mirrors the validator's reason order).
    pub const fn order_index(self) -> u8 {
        match self {
            Self::LabelNotPermanent => 0,
            Self::SourceTagHidden => 1,
            Self::PolicyLockOverridden => 2,
            Self::ValidationAnchorMissing => 3,
            Self::LifecycleImplicationHidden => 4,
            Self::ImportedValueReadsAsEditable => 5,
            Self::RowOverclaims => 6,
            Self::RowBackingMissing => 7,
            Self::RequirementUnmarked => 8,
            Self::ValidationStateUnlabeled => 9,
            Self::AsyncValidationPending => 10,
            Self::FreshnessUnlabeled => 11,
            Self::SupersededStateNotMarked => 12,
            Self::RowStale => 13,
            Self::VerificationProofStale => 14,
            Self::VerificationProofMissing => 15,
        }
    }

    /// Whether this reason breaks the contract outright (floors the row to
    /// [`RowClaim::Blocked`]) rather than merely aging it out.
    pub const fn is_floor(self) -> bool {
        self.order_index() <= Self::RowBackingMissing.order_index()
    }

    /// A reviewer-facing, non-generic description of the reason.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::LabelNotPermanent => "the field's label is not permanent",
            Self::SourceTagHidden => {
                "the source-of-value tag is hidden or a user override is not distinct"
            }
            Self::PolicyLockOverridden => "a policy-locked value was silently overridden",
            Self::ValidationAnchorMissing => {
                "a blocking validation is deferred to a summary banner instead of an exact field anchor"
            }
            Self::LifecycleImplicationHidden => {
                "a restart/reconnect/trust/policy implication is not surfaced on the control"
            }
            Self::ImportedValueReadsAsEditable => "an imported value reads as an editable local value",
            Self::RowOverclaims => "a rendering renders wider than the effective claim",
            Self::RowBackingMissing => "the backing value is missing",
            Self::RequirementUnmarked => "the required/optional status is not marked",
            Self::ValidationStateUnlabeled => "the validation state is not surfaced",
            Self::AsyncValidationPending => "async validation is still pending",
            Self::FreshnessUnlabeled => "the backing freshness state is not surfaced",
            Self::SupersededStateNotMarked => "a superseded backing source is not marked",
            Self::RowStale => "the backing source is stale",
            Self::VerificationProofStale => "the verification proof is stale",
            Self::VerificationProofMissing => "the verification proof is missing",
        }
    }
}

fn order_reasons(mut reasons: Vec<RowNarrowingReason>) -> Vec<RowNarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

fn derive_effective(claimed: RowClaim, reasons: &[RowNarrowingReason]) -> RowClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        RowClaim::Blocked
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, RowClaim::ReviewOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we
        // can no longer attribute even the read-only review, so it floors.
        RowClaim::Blocked
    } else {
        RowClaim::Narrowed
    }
}

// --------------------------------------------------------------------------- //
// Row sub-objects.
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

/// The field-level validation presented on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowValidation {
    /// The validation state.
    pub state: ValidationState,
    /// The validation state is surfaced on the row.
    pub state_labeled: bool,
    /// The validation message is anchored directly to this field.
    pub anchored_to_field: bool,
    /// Exact rule text (not a generic message) is present.
    pub exact_rule_text_present: bool,
    /// The validation lives only in a form-level summary banner.
    pub summary_banner_only: bool,
    /// Reviewer-facing exact rule text.
    pub rule_text: String,
}

/// The lifecycle/activation implication attached to a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowLifecycle {
    /// The lifecycle implication class.
    pub implication: LifecycleImplication,
    /// The implication is surfaced directly on the affected control.
    pub surfaced_on_row: bool,
    /// Reviewer-facing implication label.
    pub implication_label: String,
}

/// The verification proof backing a row's value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowVerification {
    /// The proof currency.
    pub proof_currency: ProofCurrency,
    /// The proof ref (absent serializes as `null`).
    pub proof_ref: Option<String>,
}

/// A consumer rendering of a row, with the claim it shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowRendering {
    /// The consumer render surface.
    pub surface: ConsumerSurface,
    /// The claim this rendering shows.
    pub rendered_claim: RowClaim,
    /// The rendering can reveal the row's source/validation anchor.
    pub anchor_visible: bool,
    /// The rendering presents the row read-only.
    pub read_only: bool,
    /// Stable ref back to the source row.
    pub source_row_ref: String,
}

/// One field or control row: the primitive every mutation-capable M5 form is
/// built from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldControlRow {
    /// Stable row id.
    pub row_id: String,
    /// The first-consumer surface lane this row is adopted in.
    pub consumer_lane: RowConsumerLane,
    /// Stable ref to the consumer surface that hosts this row.
    pub consumer_surface_ref: String,
    /// Whether the row is first-party or an imported/restore review.
    pub origin: RowOrigin,
    /// Whether the row is publicly claimed or Labs/unadvertised.
    pub claim_posture: RowClaimPosture,
    /// The field interaction state.
    pub field_state: FieldState,
    /// How the field's label is presented.
    pub label_mode: LabelMode,
    /// The required/optional clarity.
    pub requirement: Requirement,
    /// The required/optional status is explicitly marked on the row.
    pub requirement_marked: bool,
    /// The source of the field's value.
    pub source_class: SourceOfValueClass,
    /// The source-of-value tag is shown on the row.
    pub source_tag_visible: bool,
    /// A user override is visibly distinct from the value it replaced.
    pub override_distinct_from_origin: bool,
    /// A policy lock is respected (not silently overridden).
    pub policy_lock_respected: bool,
    /// The field-level validation.
    pub validation: RowValidation,
    /// The lifecycle/activation implication.
    pub lifecycle: RowLifecycle,
    /// The backing freshness state.
    pub declared_freshness_state: FreshnessState,
    /// The freshness state is surfaced on the row.
    pub freshness_state_visible: bool,
    /// A superseded backing source is marked.
    pub superseded_state_marked: bool,
    /// The verification proof.
    pub verification: RowVerification,
    /// The presentation a blocked/floored row falls back to.
    pub blocked_fallback: BlockedFallback,
    /// Provenance ref for an imported/restore overlay row.
    pub provenance_ref: Option<String>,
    /// Consumer renderings.
    pub renderings: Vec<RowRendering>,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The re-derived claim decision for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowDecision {
    /// The headline claim the row is eligible to make.
    pub claimed_claim: RowClaim,
    /// The effective claim after narrowing/flooring.
    pub effective_claim: RowClaim,
    /// The active narrowing reasons, ordered most-severe first.
    pub active_narrowing_reasons: Vec<RowNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl RowDecision {
    /// The headline (most severe) narrowing reason, if any.
    pub fn downgrade_trigger(&self) -> Option<RowNarrowingReason> {
        self.active_narrowing_reasons.first().copied()
    }
}

impl FieldControlRow {
    /// Whether this row is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, RowClaimPosture::LabsUnadvertised)
    }

    /// Whether this row is an inherently read-only review overlay.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin.is_overlay()
    }

    /// The headline claim this row is eligible to make.
    pub fn claimed_claim(&self) -> RowClaim {
        if self.is_labs() {
            RowClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            RowClaim::ReviewOverlay
        } else {
            RowClaim::Certified
        }
    }

    /// Reasons that hold independently of how the consumer surfaces render — the
    /// intrinsic label/source/validation/lifecycle/freshness gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<RowNarrowingReason> {
        use RowNarrowingReason as R;
        let overlay = self.is_overlay_origin();
        let v = &self.validation;
        let mut reasons: Vec<R> = Vec::new();

        // Permanent label.
        if !matches!(self.label_mode, LabelMode::Permanent) {
            reasons.push(R::LabelNotPermanent);
        }

        // Source-of-value tag. A hidden tag, a user override that is not distinct
        // from the value it replaced, or a rendering that cannot reveal the
        // provenance/anchor all hide where the value came from.
        let override_not_distinct = matches!(self.source_class, SourceOfValueClass::UserOverride)
            && !self.override_distinct_from_origin;
        if !self.source_tag_visible
            || override_not_distinct
            || self.renderings.iter().any(|r| !r.anchor_visible)
        {
            reasons.push(R::SourceTagHidden);
        }

        // Policy lock.
        if matches!(self.source_class, SourceOfValueClass::PolicyLocked)
            && !self.policy_lock_respected
        {
            reasons.push(R::PolicyLockOverridden);
        }

        // Validation anchoring. A blocking or warning validation must be anchored
        // directly to the field with exact rule text, not deferred to a banner.
        let needs_anchor = matches!(
            v.state,
            ValidationState::InvalidBlocking | ValidationState::Warning
        );
        if needs_anchor
            && (v.summary_banner_only || !v.anchored_to_field || !v.exact_rule_text_present)
        {
            reasons.push(R::ValidationAnchorMissing);
        }

        // Lifecycle implication surfaced on the control.
        if !matches!(self.lifecycle.implication, LifecycleImplication::None)
            && !self.lifecycle.surfaced_on_row
        {
            reasons.push(R::LifecycleImplicationHidden);
        }

        // Imported/restore overlay must stay read-only.
        if overlay
            && (matches!(self.field_state, FieldState::Editable)
                || self.renderings.iter().any(|r| !r.read_only))
        {
            reasons.push(R::ImportedValueReadsAsEditable);
        }

        // Backing freshness.
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::RowBackingMissing),
            FreshnessState::SupersededByNewerSource if !self.superseded_state_marked => {
                reasons.push(R::SupersededStateNotMarked);
            }
            FreshnessState::StaleExpired if !overlay => reasons.push(R::RowStale),
            _ => {}
        }

        // Required/optional clarity (recoverable: the field is still visible).
        if !self.requirement_marked {
            reasons.push(R::RequirementUnmarked);
        }

        // Validation visibility (recoverable).
        if !v.state_labeled {
            reasons.push(R::ValidationStateUnlabeled);
        }
        if matches!(v.state, ValidationState::PendingAsync) {
            reasons.push(R::AsyncValidationPending);
        }

        // Freshness visibility (recoverable).
        if !self.freshness_state_visible {
            reasons.push(R::FreshnessUnlabeled);
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

        reasons
    }

    /// All active narrowing reasons, including the rendering-overclaim check,
    /// ordered and de-duplicated.
    fn reasons(&self, stale_window: bool) -> Vec<RowNarrowingReason> {
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic_effective = derive_effective(self.claimed_claim(), &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic_effective.overclaims_as(r.rendered_claim))
        {
            reasons.push(RowNarrowingReason::RowOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive this row's claim decision.
    pub fn narrow(&self, stale_window: bool) -> RowDecision {
        let claimed = self.claimed_claim();
        if matches!(claimed, RowClaim::LabsNotClaimed) {
            return RowDecision {
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
        RowDecision {
            claimed_claim: claimed,
            effective_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// Whether a floored row still keeps an on-row reason/hint fallback rather than
    /// going silent.
    pub fn floored_keeps_fallback(&self, effective: RowClaim) -> bool {
        if !matches!(effective, RowClaim::Blocked) {
            return true;
        }
        self.blocked_fallback.keeps_recovery()
    }

    /// Whether any rendering overclaims relative to `effective`.
    pub fn row_overclaims(&self, effective: RowClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// A reviewer-facing label for a narrowed/floored row, or `None` if the row
    /// holds its claim.
    pub fn narrowed_label(&self, decision: &RowDecision) -> Option<String> {
        let trigger = decision.downgrade_trigger()?;
        Some(match decision.effective_claim {
            RowClaim::Blocked => format!(
                "Floored to row_blocked below the {} claim: {}; falls back to an explicit blocked state that shows the reason on the row.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            RowClaim::Narrowed => format!(
                "Held at row_narrowed below the {} claim: {}; the row stays usable and attributable until re-verified.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            _ => return None,
        })
    }

    /// Append per-row structural violations (schema-shape level).
    fn structural_violations(&self, out: &mut Vec<M5FieldControlRowViolation>) {
        use M5FieldControlRowViolation as V;
        if self.row_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.consumer_surface_ref.trim().is_empty()
        {
            out.push(V::RowMissingIdentity);
        }
        if self.is_overlay_origin() && !opt_present(&self.provenance_ref) {
            out.push(V::OverlayMissingProvenanceRef);
        }
        if self.renderings.is_empty() {
            out.push(V::RowMissingRendering);
        }
        for r in &self.renderings {
            if r.source_row_ref.trim().is_empty() {
                out.push(V::RenderingMissingSourceRef);
            }
        }
    }
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for [`M5FieldControlRowSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FieldControlRowSetInput {
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
    /// Per-row primitives.
    pub rows: Vec<FieldControlRow>,
}

/// Export-safe M5 field/control-row set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FieldControlRowSetPacket {
    /// Record kind; must equal [`M5_FIELD_CONTROL_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FIELD_CONTROL_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_FIELD_CONTROL_ROW_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-row primitives.
    pub rows: Vec<FieldControlRow>,
}

/// The distribution of effective row claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowClaimDistribution {
    /// Rows effective at [`RowClaim::Certified`].
    pub certified: usize,
    /// Rows effective at [`RowClaim::Narrowed`].
    pub narrowed: usize,
    /// Rows effective at [`RowClaim::ReviewOverlay`].
    pub overlay: usize,
    /// Rows effective at [`RowClaim::Blocked`].
    pub blocked: usize,
    /// Rows effective at [`RowClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5FieldControlRowSetPacket {
    /// Builds a field-control-row set packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5FieldControlRowSetInput) -> Self {
        Self {
            record_kind: M5_FIELD_CONTROL_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_FIELD_CONTROL_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_FIELD_CONTROL_ROW_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            rows: input.rows,
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

    /// Re-derive the decision for every row, paired with its id.
    pub fn decisions(&self) -> Vec<(String, RowDecision)> {
        let stale_window = self.stale_window();
        self.rows
            .iter()
            .map(|r| (r.row_id.clone(), r.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective row claims.
    pub fn claim_distribution(&self) -> RowClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = RowClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            blocked: 0,
            labs: 0,
        };
        for r in &self.rows {
            match r.narrow(stale_window).effective_claim {
                RowClaim::Certified => dist.certified += 1,
                RowClaim::Narrowed => dist.narrowed += 1,
                RowClaim::ReviewOverlay => dist.overlay += 1,
                RowClaim::Blocked => dist.blocked += 1,
                RowClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of rows whose effective claim ranks below their claimed claim.
    pub fn narrowed_row_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.rows
            .iter()
            .filter(|r| r.narrow(stale_window).narrowed)
            .count()
    }

    /// Consumer lanes represented by some row.
    pub fn represented_lanes(&self) -> BTreeSet<RowConsumerLane> {
        self.rows.iter().map(|r| r.consumer_lane).collect()
    }

    /// Source-of-value classes represented by some row.
    pub fn represented_source_classes(&self) -> BTreeSet<SourceOfValueClass> {
        self.rows.iter().map(|r| r.source_class).collect()
    }

    /// Lifecycle implications represented by some row.
    pub fn represented_lifecycle_implications(&self) -> BTreeSet<LifecycleImplication> {
        self.rows.iter().map(|r| r.lifecycle.implication).collect()
    }

    /// Requirement classes represented by some row.
    pub fn represented_requirements(&self) -> BTreeSet<Requirement> {
        self.rows.iter().map(|r| r.requirement).collect()
    }

    /// Consumer render surfaces represented by some rendering.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<ConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.renderings.iter().map(|x| x.surface))
            .collect()
    }

    /// Validate the field-control-row invariants.
    pub fn validate(&self) -> Vec<M5FieldControlRowViolation> {
        use M5FieldControlRowViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_FIELD_CONTROL_ROW_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_FIELD_CONTROL_ROW_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_FIELD_CONTROL_ROW_TAXONOMY_VERSION {
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
        if self.rows.is_empty() {
            violations.push(V::EmptyRows);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for r in &self.rows {
            if !seen.insert(r.row_id.as_str()) {
                violations.push(V::DuplicateRowId);
            }
        }

        if RowConsumerLane::ALL
            .iter()
            .any(|l| !self.represented_lanes().contains(l))
        {
            violations.push(V::ConsumerLaneMissing);
        }
        if SourceOfValueClass::ALL
            .iter()
            .any(|c| !self.represented_source_classes().contains(c))
        {
            violations.push(V::SourceOfValueClassMissing);
        }
        if LifecycleImplication::ALL
            .iter()
            .any(|i| !self.represented_lifecycle_implications().contains(i))
        {
            violations.push(V::LifecycleImplicationMissing);
        }
        if Requirement::ALL
            .iter()
            .any(|q| !self.represented_requirements().contains(q))
        {
            violations.push(V::RequirementClassMissing);
        }
        if ConsumerSurface::ALL
            .iter()
            .any(|s| !self.represented_consumer_surfaces().contains(s))
        {
            violations.push(V::ConsumerSurfaceMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for r in &self.rows {
            r.structural_violations(&mut violations);
            let decision = r.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || r.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedRowMissingLabelOrTrigger);
                }
            }
            if !r.floored_keeps_fallback(decision.effective_claim) {
                violations.push(V::FlooredRowLosesFallback);
            }
            if r.row_overclaims(decision.effective_claim) {
                violations.push(V::RenderingRowOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedRowCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("field-control-row packet serializes"),
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
        serde_json::to_string_pretty(self).expect("field-control-row packet serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_report(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Field And Control Row Primitives\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Rows: {}\n", self.rows.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} review overlay, {} blocked, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.blocked, dist.labs
        ));

        out.push_str("| Row | Lane | Source | Lifecycle | Origin | Claimed | Effective |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for r in &self.rows {
            let decision = r.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                r.row_id,
                r.consumer_lane.as_str(),
                r.source_class.as_str(),
                r.lifecycle.implication.as_str(),
                r.origin.as_str(),
                decision.claimed_claim.as_str(),
                decision.effective_claim.as_str(),
            ));
        }

        out.push('\n');
        for r in &self.rows {
            let decision = r.narrow(stale_window);
            if let Some(label) = r.narrowed_label(&decision) {
                out.push_str(&format!("- {}: {}\n", r.row_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum M5FieldControlRowArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5FieldControlRowViolation>),
}

impl fmt::Display for M5FieldControlRowArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(f, "support export parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for M5FieldControlRowArtifactError {}

/// A structured-input invariant a field-control-row set can violate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5FieldControlRowViolation {
    /// The record-kind tag is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// The taxonomy version is wrong.
    WrongTaxonomyVersion,
    /// Packet identity is incomplete.
    MissingIdentity,
    /// The redaction class token is not recognized.
    InvalidRedactionClass,
    /// The evidence freshness window is incomplete.
    EvidenceFreshnessIncomplete,
    /// The packet has no rows.
    EmptyRows,
    /// Two rows share an id.
    DuplicateRowId,
    /// A consumer lane is unrepresented.
    ConsumerLaneMissing,
    /// A source-of-value class is unrepresented.
    SourceOfValueClassMissing,
    /// A lifecycle implication is unrepresented.
    LifecycleImplicationMissing,
    /// A requirement class is unrepresented.
    RequirementClassMissing,
    /// A consumer render surface is unrepresented.
    ConsumerSurfaceMissing,
    /// A row is missing identity (id/label/host ref).
    RowMissingIdentity,
    /// An overlay row does not name its provenance.
    OverlayMissingProvenanceRef,
    /// A row has no renderings.
    RowMissingRendering,
    /// A rendering is missing its source ref.
    RenderingMissingSourceRef,
    /// A narrowed/floored row lacks a non-generic label or trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// A floored row goes silent instead of keeping an on-row fallback.
    FlooredRowLosesFallback,
    /// A rendering renders wider than the row's effective claim.
    RenderingRowOverclaims,
    /// No row demonstrates the auto-narrowing rule.
    DowngradedRowCaseMissing,
    /// Raw credential/body material crosses the export boundary.
    RawBoundaryMaterialInExport,
}

impl M5FieldControlRowViolation {
    /// Stable token recorded in diagnostics and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyRows => "empty_rows",
            Self::DuplicateRowId => "duplicate_row_id",
            Self::ConsumerLaneMissing => "consumer_lane_missing",
            Self::SourceOfValueClassMissing => "source_of_value_class_missing",
            Self::LifecycleImplicationMissing => "lifecycle_implication_missing",
            Self::RequirementClassMissing => "requirement_class_missing",
            Self::ConsumerSurfaceMissing => "consumer_surface_missing",
            Self::RowMissingIdentity => "row_missing_identity",
            Self::OverlayMissingProvenanceRef => "overlay_missing_provenance_ref",
            Self::RowMissingRendering => "row_missing_rendering",
            Self::RenderingMissingSourceRef => "rendering_missing_source_ref",
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::FlooredRowLosesFallback => "floored_row_loses_fallback",
            Self::RenderingRowOverclaims => "rendering_row_overclaims",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked-in canonical support export.
///
/// # Errors
///
/// Returns an error if the checked-in artifact cannot be parsed or fails to
/// validate.
pub fn current_m5_field_control_row_set(
) -> Result<M5FieldControlRowSetPacket, M5FieldControlRowArtifactError> {
    let packet: M5FieldControlRowSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ux/m5-field-control-rows/support_export.json"
    )))
    .map_err(M5FieldControlRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FieldControlRowArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded field-control-row set: the in-crate source of truth the
/// checked-in support export and report are regenerated from.
pub fn seeded_m5_field_control_row_set() -> M5FieldControlRowSetPacket {
    M5FieldControlRowSetPacket::new(M5FieldControlRowSetInput {
        packet_id: M5_FIELD_CONTROL_ROW_PACKET_ID.to_owned(),
        label:
            "M5 field and control rows — permanent labels, validation anchors, source-of-value tags, and lifecycle state across mutation-capable forms"
                .to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        rows: seed_rows(),
    })
}

/// A validation block that is valid and field-anchored with exact rule text.
fn valid_anchored(rule: &str) -> RowValidation {
    RowValidation {
        state: ValidationState::Valid,
        state_labeled: true,
        anchored_to_field: true,
        exact_rule_text_present: true,
        summary_banner_only: false,
        rule_text: rule.to_owned(),
    }
}

/// A not-yet-validated block (no error to anchor) that still carries exact rule
/// text and a field anchor so the rule is visible before entry.
fn not_validated(rule: &str) -> RowValidation {
    RowValidation {
        state: ValidationState::NotValidated,
        state_labeled: true,
        anchored_to_field: true,
        exact_rule_text_present: true,
        summary_banner_only: false,
        rule_text: rule.to_owned(),
    }
}

/// A pending-async validation block.
fn pending_async(rule: &str) -> RowValidation {
    RowValidation {
        state: ValidationState::PendingAsync,
        state_labeled: true,
        anchored_to_field: true,
        exact_rule_text_present: true,
        summary_banner_only: false,
        rule_text: rule.to_owned(),
    }
}

/// A lifecycle block with no implication.
fn lifecycle_none() -> RowLifecycle {
    RowLifecycle {
        implication: LifecycleImplication::None,
        surfaced_on_row: true,
        implication_label: "No restart, reconnect, trust, or policy implication".to_owned(),
    }
}

/// A lifecycle block whose implication is surfaced on the row.
fn lifecycle(implication: LifecycleImplication, label: &str) -> RowLifecycle {
    RowLifecycle {
        implication,
        surfaced_on_row: true,
        implication_label: label.to_owned(),
    }
}

/// A current verification proof.
fn proof_current(reference: &str) -> RowVerification {
    RowVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(reference.to_owned()),
    }
}

/// Renderings that show `claim` across the named consumer surfaces.
fn renderings(
    source_ref: &str,
    claim: RowClaim,
    surfaces: &[ConsumerSurface],
    read_only: bool,
) -> Vec<RowRendering> {
    surfaces
        .iter()
        .map(|&surface| RowRendering {
            surface,
            rendered_claim: claim,
            anchor_visible: true,
            read_only,
            source_row_ref: source_ref.to_owned(),
        })
        .collect()
}

#[allow(clippy::too_many_lines, clippy::vec_init_then_push)]
fn seed_rows() -> Vec<FieldControlRow> {
    use ConsumerSurface as Cs;
    let mut rows = Vec::new();

    // --- Provider / account mapping ---------------------------------------- //
    rows.push(FieldControlRow {
        row_id: "row:provider-endpoint:0001".to_owned(),
        consumer_lane: RowConsumerLane::ProviderAccountMapping,
        consumer_surface_ref: "form:provider-credentials:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Required,
        requirement_marked: true,
        source_class: SourceOfValueClass::DetectedValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: valid_anchored("Must be a reachable https endpoint"),
        lifecycle: lifecycle(
            LifecycleImplication::ReconnectRequired,
            "Changing the endpoint reconnects the account",
        ),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:provider-endpoint"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:provider-endpoint:0001",
            RowClaim::Certified,
            &[Cs::FormView, Cs::ReviewSheet],
            false,
        ),
        label_summary: "Provider endpoint — detected from the workspace, reconnects on change"
            .to_owned(),
    });
    rows.push(FieldControlRow {
        row_id: "row:provider-region:0001".to_owned(),
        consumer_lane: RowConsumerLane::ProviderAccountMapping,
        consumer_surface_ref: "form:provider-credentials:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Optional,
        requirement_marked: true,
        source_class: SourceOfValueClass::DefaultValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: not_validated("Defaults to the account's home region"),
        lifecycle: lifecycle_none(),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:provider-region"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:provider-region:0001",
            RowClaim::Certified,
            &[Cs::FormView, Cs::HelpInline],
            false,
        ),
        label_summary: "Account region — default value, optional".to_owned(),
    });
    rows.push(FieldControlRow {
        row_id: "row:provider-token:0001".to_owned(),
        consumer_lane: RowConsumerLane::ProviderAccountMapping,
        consumer_surface_ref: "form:provider-credentials:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::MaskedCredential,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Conditional,
        requirement_marked: true,
        source_class: SourceOfValueClass::UserOverride,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: valid_anchored("Required when token auth is selected; 40+ characters"),
        lifecycle: lifecycle(
            LifecycleImplication::TrustRequired,
            "Saving stores a trusted credential reference",
        ),
        declared_freshness_state: FreshnessState::CachedSnapshot,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:provider-token"),
        blocked_fallback: BlockedFallback::DisabledWithHint,
        provenance_ref: None,
        renderings: renderings(
            "row:provider-token:0001",
            RowClaim::Certified,
            &[Cs::FormView, Cs::DiagnosticsPanel],
            false,
        ),
        label_summary: "Provider token — masked user override, conditional, trust-gated".to_owned(),
    });

    // --- Source registration ----------------------------------------------- //
    rows.push(FieldControlRow {
        row_id: "row:source-url:0001".to_owned(),
        consumer_lane: RowConsumerLane::SourceRegistration,
        consumer_surface_ref: "form:source-registration:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Required,
        requirement_marked: true,
        source_class: SourceOfValueClass::UserOverride,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: valid_anchored("Must be an https git or registry URL"),
        lifecycle: lifecycle_none(),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:source-url"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:source-url:0001",
            RowClaim::Certified,
            &[Cs::FormView, Cs::WizardStep],
            false,
        ),
        label_summary: "Source URL — user override, required".to_owned(),
    });
    rows.push(FieldControlRow {
        row_id: "row:source-kind:0001".to_owned(),
        consumer_lane: RowConsumerLane::SourceRegistration,
        consumer_surface_ref: "form:source-registration:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Optional,
        requirement_marked: true,
        source_class: SourceOfValueClass::DefaultValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: not_validated("Defaults to the detected source kind"),
        lifecycle: lifecycle_none(),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:source-kind"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:source-kind:0001",
            RowClaim::Certified,
            &[Cs::WizardStep, Cs::SupportExport],
            false,
        ),
        label_summary: "Source kind — default value, optional".to_owned(),
    });
    rows.push(FieldControlRow {
        row_id: "row:source-trust-policy:0001".to_owned(),
        consumer_lane: RowConsumerLane::SourceRegistration,
        consumer_surface_ref: "form:source-registration:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::ReadOnlyLocked,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::SystemManaged,
        requirement_marked: true,
        source_class: SourceOfValueClass::PolicyLocked,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: valid_anchored("Trust level is fixed by the organization policy"),
        lifecycle: lifecycle(
            LifecycleImplication::PolicyBlocked,
            "Locked by the organization trust policy",
        ),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:source-trust"),
        blocked_fallback: BlockedFallback::DisabledWithHint,
        provenance_ref: None,
        renderings: renderings(
            "row:source-trust-policy:0001",
            RowClaim::Certified,
            &[Cs::FormView, Cs::ReviewSheet],
            false,
        ),
        label_summary: "Trust policy — policy-locked, read-only".to_owned(),
    });

    // --- Request environment ------------------------------------------------ //
    rows.push(FieldControlRow {
        row_id: "row:request-environment-name:0001".to_owned(),
        consumer_lane: RowConsumerLane::RequestEnvironment,
        consumer_surface_ref: "dialog:request-workspace-run:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Required,
        requirement_marked: true,
        source_class: SourceOfValueClass::DetectedValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: valid_anchored("Detected from the active workspace"),
        lifecycle: lifecycle_none(),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:request-env-name"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:request-environment-name:0001",
            RowClaim::Certified,
            &[Cs::FormView, Cs::SupportExport],
            false,
        ),
        label_summary: "Environment name — detected, required".to_owned(),
    });
    rows.push(FieldControlRow {
        row_id: "row:request-base-url:0001".to_owned(),
        consumer_lane: RowConsumerLane::RequestEnvironment,
        consumer_surface_ref: "dialog:request-workspace-run:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Required,
        requirement_marked: true,
        source_class: SourceOfValueClass::RequiredUnset,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: not_validated("Required before the run can start"),
        lifecycle: lifecycle_none(),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:request-base-url"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:request-base-url:0001",
            RowClaim::Certified,
            &[Cs::FormView, Cs::DiagnosticsPanel],
            false,
        ),
        label_summary: "Base URL — required and still unset".to_owned(),
    });
    // The single canonical narrowing demonstrator: a pending async check.
    rows.push(FieldControlRow {
        row_id: "row:request-endpoint-health:0001".to_owned(),
        consumer_lane: RowConsumerLane::RequestEnvironment,
        consumer_surface_ref: "dialog:request-workspace-run:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Optional,
        requirement_marked: true,
        source_class: SourceOfValueClass::DetectedValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: pending_async("Checking endpoint reachability"),
        lifecycle: lifecycle(
            LifecycleImplication::ReconnectRequired,
            "A failed check reconnects the session",
        ),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:request-endpoint-health"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:request-endpoint-health:0001",
            RowClaim::Narrowed,
            &[Cs::FormView, Cs::AiEvidence],
            false,
        ),
        label_summary: "Endpoint health — async validation pending".to_owned(),
    });

    // --- Package / install -------------------------------------------------- //
    rows.push(FieldControlRow {
        row_id: "row:package-install-scope:0001".to_owned(),
        consumer_lane: RowConsumerLane::PackageInstall,
        consumer_surface_ref: "sheet:package-install-review:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::SystemManaged,
        requirement_marked: true,
        source_class: SourceOfValueClass::DetectedValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: valid_anchored("Scope detected from the workspace manifest"),
        lifecycle: lifecycle(
            LifecycleImplication::RestartRequired,
            "Installing reloads the workspace",
        ),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:package-scope"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:package-install-scope:0001",
            RowClaim::Certified,
            &[Cs::ReviewSheet, Cs::SupportExport],
            false,
        ),
        label_summary: "Install scope — detected, restart-required".to_owned(),
    });
    rows.push(FieldControlRow {
        row_id: "row:package-target-dir:0001".to_owned(),
        consumer_lane: RowConsumerLane::PackageInstall,
        consumer_surface_ref: "sheet:package-install-review:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::Editable,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Optional,
        requirement_marked: true,
        source_class: SourceOfValueClass::DefaultValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: valid_anchored("Must be a writable directory"),
        lifecycle: lifecycle_none(),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:package-target-dir"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:package-target-dir:0001",
            RowClaim::Certified,
            &[Cs::ReviewSheet, Cs::HelpInline],
            false,
        ),
        label_summary: "Target directory — default value, optional".to_owned(),
    });

    // --- Migration / import ------------------------------------------------- //
    // A read-only imported value: an attributable review overlay, never editable.
    rows.push(FieldControlRow {
        row_id: "row:import-mapping:0001".to_owned(),
        consumer_lane: RowConsumerLane::MigrationImport,
        consumer_surface_ref: "dialog:migration-restore-review:0001".to_owned(),
        origin: RowOrigin::ImportedOrRestore,
        claim_posture: RowClaimPosture::ClaimedStable,
        field_state: FieldState::ReadOnlyLocked,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Required,
        requirement_marked: true,
        source_class: SourceOfValueClass::ImportedValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: valid_anchored("Imported from the migration bundle"),
        lifecycle: lifecycle_none(),
        declared_freshness_state: FreshnessState::CachedSnapshot,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:import-mapping"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: Some("import-bundle:migration-restore:0001".to_owned()),
        renderings: renderings(
            "row:import-mapping:0001",
            RowClaim::ReviewOverlay,
            &[Cs::ReviewSheet, Cs::DiagnosticsPanel],
            true,
        ),
        label_summary: "Imported mapping — read-only review overlay".to_owned(),
    });
    // A Labs/unadvertised preview row that makes no public claim.
    rows.push(FieldControlRow {
        row_id: "row:labs-import-preview:0001".to_owned(),
        consumer_lane: RowConsumerLane::MigrationImport,
        consumer_surface_ref: "dialog:migration-restore-review:0001".to_owned(),
        origin: RowOrigin::FirstParty,
        claim_posture: RowClaimPosture::LabsUnadvertised,
        field_state: FieldState::ComputedDerived,
        label_mode: LabelMode::Permanent,
        requirement: Requirement::Optional,
        requirement_marked: true,
        source_class: SourceOfValueClass::DefaultValue,
        source_tag_visible: true,
        override_distinct_from_origin: true,
        policy_lock_respected: true,
        validation: not_validated("Experimental preview; not validated"),
        lifecycle: lifecycle_none(),
        declared_freshness_state: FreshnessState::Live,
        freshness_state_visible: true,
        superseded_state_marked: true,
        verification: proof_current("proof:labs-import-preview"),
        blocked_fallback: BlockedFallback::ShowsReasonOnRow,
        provenance_ref: None,
        renderings: renderings(
            "row:labs-import-preview:0001",
            RowClaim::LabsNotClaimed,
            &[Cs::FormView, Cs::HelpInline],
            false,
        ),
        label_summary: "Labs import preview — unadvertised, makes no claim".to_owned(),
    });

    rows
}
