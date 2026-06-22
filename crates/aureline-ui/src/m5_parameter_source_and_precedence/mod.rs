//! Canonical parameter-source and precedence-inspector truth for M5
//! mutation-capable forms.
//!
//! Where [`crate::m5_field_control_rows`] freezes a single field's label, source
//! tag, and validation anchor, and [`crate::m5_staged_review_sheets`] freezes the
//! commit sheet a mutation stops at, this module freezes the **parameter-source
//! inspector** a user (or support agent) opens to answer *why a current form value
//! is present and which source actually wins* before they commit a change. One
//! inspector model is reused across the provider account-mapping, source-
//! registration, request-environment, package-install, settings-config, import-
//! migration, and project-bootstrap forms, instead of each domain inferring field
//! origin from a single collapsed "current value".
//!
//! Each [`ParameterFieldRecord`] binds, for one field:
//!
//! * its **source candidates** — a [`SourceCandidate`] per [`SourceLayer`]
//!   (`default`, `detected`, `imported`, `environment_resolved`, `user_override`,
//!   `policy_provided`), each carrying its [`ValueScope`] (personal/local,
//!   workspace/shared, policy-owned), whether it is `present`, and whether its
//!   source and scope stay individually labelled — so imported values, policy
//!   locks, detected values, and user overrides never collapse into one current
//!   field state;
//! * its **effective resolution** — an [`EffectiveResolution`] naming the winning
//!   [`SourceLayer`], its scope, and its declared precedence rank, which must be the
//!   highest-precedence *present* candidate;
//! * its **policy lock** — a [`PolicyLock`] that, when set, pins the effective value
//!   to `policy_provided` and forbids a silent user override;
//! * its **fallback disclosure** — a [`FallbackDisclosure`] that, when the effective
//!   value fell back to a built-in/auto source, explains *why*; and
//! * its **override gating** — whether submit is gated on source clarity, so a
//!   mutation-capable form can never submit from an ambiguous source-hidden state.
//!
//! Each record re-derives a [`ParameterClaim`] ([`ParameterFieldRecord::narrow`]) so
//! an inspector can never read wider than its evidence: a field that hides its
//! effective source, collapses its distinct source layers into one state, declares
//! an effective layer that is not the highest-precedence present candidate, hides or
//! fails to enforce a policy lock, lets an imported review read as a user-set value,
//! hides a fallback reason or the value scope, allows submit from an ambiguous state,
//! loses its inspect-to-source path, or renders wider than its claim floors to
//! [`ParameterClaim::Unsafe`] and falls back to an explicit blocked-submit state with
//! an inspect/keyboard recovery path. A labelled, recoverable gap (an unlabelled
//! non-winning candidate, an unlabelled fallback reason, a stale or superseded
//! detection snapshot, an aged proof) holds a first-party field at
//! [`ParameterClaim::Narrowed`] while keeping the source inspectable, an
//! import/migration review sits at [`ParameterClaim::ReviewOverlay`] and never
//! claims a user-set value, and a Labs/unadvertised field makes no public claim.
//!
//! [`M5ParameterSourceSetPacket::validate`] confirms the matrix is well-formed and
//! honest: header/identity/redaction/freshness are present, every form, lane, source
//! layer, value scope, and consumer surface is represented, overlay fields name their
//! provenance, no rendering surface overclaims, a floored field keeps a fallback, at
//! least one field demonstrates the auto-narrowing rule, and no raw credential/body
//! material crosses the export. Downstream settings, marketplace, request, support,
//! admin, import, and project surfaces — plus the CLI/headless inspect path and the
//! docs/help references — ingest this packet rather than minting per-feature
//! source-precedence semantics.
//!
//! No credential bodies, secret values, raw provider payloads, absolute paths, or
//! URLs ever cross this boundary; the packet carries only typed class tokens, counts,
//! booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/ux/m5-parameter-source-and-precedence.schema.json`](../../../../schemas/ux/m5-parameter-source-and-precedence.schema.json).
//! The contract doc is
//! [`docs/ux/m5-parameter-source-and-precedence.md`](../../../../docs/ux/m5-parameter-source-and-precedence.md).
//! The canonical support export is
//! [`artifacts/ux/m5-parameter-source-and-precedence/support_export.json`](../../../../artifacts/ux/m5-parameter-source-and-precedence/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/ux/m5-parameter-source-and-precedence/`](../../../../fixtures/ux/m5-parameter-source-and-precedence/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ParameterSourceSetPacket`].
pub const M5_PARAMETER_SOURCE_RECORD_KIND: &str = "m5_parameter_source_set_packet";

/// Schema version for the parameter-source set.
pub const M5_PARAMETER_SOURCE_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_PARAMETER_SOURCE_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical parameter-source set packet.
pub const M5_PARAMETER_SOURCE_PACKET_ID: &str = "m5-parameter-source-and-precedence:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_PARAMETER_SOURCE_SCHEMA_REF: &str =
    "schemas/ux/m5-parameter-source-and-precedence.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PARAMETER_SOURCE_DOC_REF: &str = "docs/ux/m5-parameter-source-and-precedence.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_PARAMETER_SOURCE_SUPPORT_EXPORT_REF: &str =
    "artifacts/ux/m5-parameter-source-and-precedence/support_export.json";

/// Repo-relative path of the generated report.
pub const M5_PARAMETER_SOURCE_REPORT_REF: &str =
    "artifacts/ux/m5-parameter-source-and-precedence/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_PARAMETER_SOURCE_FIXTURE_DIR: &str = "fixtures/ux/m5-parameter-source-and-precedence";

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

/// The origin layer a candidate value comes from. The precedence rank is the
/// shared, deterministic ordering used to pick the effective value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLayer {
    /// A built-in default value.
    Default,
    /// A value auto-detected from the host/system/project.
    Detected,
    /// A value brought in by an import/migration.
    Imported,
    /// A value resolved from an environment variable or environment profile.
    EnvironmentResolved,
    /// A value explicitly set by the user (an override).
    UserOverride,
    /// A value provided (and potentially locked) by policy.
    PolicyProvided,
}

impl SourceLayer {
    /// Every source layer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Default,
        Self::Detected,
        Self::Imported,
        Self::EnvironmentResolved,
        Self::UserOverride,
        Self::PolicyProvided,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Detected => "detected",
            Self::Imported => "imported",
            Self::EnvironmentResolved => "environment_resolved",
            Self::UserOverride => "user_override",
            Self::PolicyProvided => "policy_provided",
        }
    }

    /// The canonical precedence rank: a higher rank wins the effective value. A
    /// present policy-provided value (rank 5) outranks a user override (rank 4),
    /// which outranks an environment resolution (rank 3), an imported value (rank 2),
    /// a detected value (rank 1), and a built-in default (rank 0).
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::Default => 0,
            Self::Detected => 1,
            Self::Imported => 2,
            Self::EnvironmentResolved => 3,
            Self::UserOverride => 4,
            Self::PolicyProvided => 5,
        }
    }
}

/// The ownership/visibility scope of a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueScope {
    /// Personal / local-only to this user or machine.
    PersonalLocal,
    /// Shared across the workspace.
    WorkspaceShared,
    /// Owned and governed by policy.
    PolicyOwned,
}

impl ValueScope {
    /// Every value scope, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PersonalLocal,
        Self::WorkspaceShared,
        Self::PolicyOwned,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersonalLocal => "personal_local",
            Self::WorkspaceShared => "workspace_shared",
            Self::PolicyOwned => "policy_owned",
        }
    }
}

/// The M5 mutation-capable form a field belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldForm {
    /// The provider account-mapping form.
    ProviderAccountMapping,
    /// The admin source-registration form.
    SourceRegistration,
    /// The request-workspace environment form.
    RequestEnvironment,
    /// The package install/configuration form.
    PackageInstallConfig,
    /// The settings/configuration editor.
    SettingsConfigEditor,
    /// The import/migration mapping form.
    ImportMigrationMapping,
    /// The generated-project / bootstrap form.
    ProjectBootstrap,
}

impl FieldForm {
    /// Every form, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProviderAccountMapping,
        Self::SourceRegistration,
        Self::RequestEnvironment,
        Self::PackageInstallConfig,
        Self::SettingsConfigEditor,
        Self::ImportMigrationMapping,
        Self::ProjectBootstrap,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAccountMapping => "provider_account_mapping",
            Self::SourceRegistration => "source_registration",
            Self::RequestEnvironment => "request_environment",
            Self::PackageInstallConfig => "package_install_config",
            Self::SettingsConfigEditor => "settings_config_editor",
            Self::ImportMigrationMapping => "import_migration_mapping",
            Self::ProjectBootstrap => "project_bootstrap",
        }
    }
}

/// The product lane the field belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldLane {
    /// Provider configuration.
    Provider,
    /// Admin / source management.
    Admin,
    /// Request workspace.
    Request,
    /// Package / marketplace.
    Package,
    /// Settings / configuration.
    Settings,
    /// Import / migration center.
    Import,
    /// Projects / bootstrap.
    Projects,
}

impl FieldLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Provider,
        Self::Admin,
        Self::Request,
        Self::Package,
        Self::Settings,
        Self::Import,
        Self::Projects,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provider => "provider",
            Self::Admin => "admin",
            Self::Request => "request",
            Self::Package => "package",
            Self::Settings => "settings",
            Self::Import => "import",
            Self::Projects => "projects",
        }
    }
}

/// How the inspector and its field originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldOrigin {
    /// A first-party local form field.
    LocalForm,
    /// A first-party form bound to a remote target.
    RemoteForm,
    /// A provider-backed form field.
    ProviderForm,
    /// A review of imported/migrated values (an overlay).
    ImportedReview,
}

impl FieldOrigin {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalForm => "local_form",
            Self::RemoteForm => "remote_form",
            Self::ProviderForm => "provider_form",
            Self::ImportedReview => "imported_review",
        }
    }

    /// Whether this origin is an inherently read-only review overlay.
    pub const fn is_overlay(self) -> bool {
        matches!(self, Self::ImportedReview)
    }
}

/// The freshness of the detection snapshot a field's detected/auto values came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionState {
    /// Live.
    Live,
    /// A cached snapshot within its window.
    CachedSnapshot,
    /// Stale / expired.
    StaleExpired,
    /// Superseded by a newer detection source.
    SupersededByNewerSource,
    /// Missing.
    Missing,
}

impl DetectionState {
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

/// Verification-proof currency for a field (distinct from detection freshness).
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
    /// No proof anchors the inspector.
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
    /// Reopen restores the field and its source inspector.
    FieldAndInspector,
    /// Reopen restores the source inspector only.
    InspectorOnly,
    /// No reopen; a keyboard fallback to the originating field remains.
    NoneKeyboardFallback,
}

impl ReopenTarget {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FieldAndInspector => "field_and_inspector",
            Self::InspectorOnly => "inspector_only",
            Self::NoneKeyboardFallback => "none_keyboard_fallback",
        }
    }
}

/// Whether the field is publicly claimed or a Labs/unadvertised surface.
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

/// A consumer surface that re-renders a parameter-source inspector record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// The full parameter-source inspector panel.
    InspectorPanel,
    /// The inline source/precedence popover on the field itself.
    FieldPopover,
    /// The diagnostics panel.
    DiagnosticsPanel,
    /// A support-export bundle.
    SupportExport,
    /// An AI-evidence consumer.
    AiEvidence,
    /// Inline help / docs.
    HelpInline,
    /// A CLI/headless inspect surface.
    CliInspect,
}

impl ConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InspectorPanel,
        Self::FieldPopover,
        Self::DiagnosticsPanel,
        Self::SupportExport,
        Self::AiEvidence,
        Self::HelpInline,
        Self::CliInspect,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectorPanel => "inspector_panel",
            Self::FieldPopover => "field_popover",
            Self::DiagnosticsPanel => "diagnostics_panel",
            Self::SupportExport => "support_export",
            Self::AiEvidence => "ai_evidence",
            Self::HelpInline => "help_inline",
            Self::CliInspect => "cli_inspect",
        }
    }
}

// --------------------------------------------------------------------------- //
// Derived claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim a parameter-source inspector renders. A higher rank asserts
/// more authority, so a narrowed or floored field must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterClaim {
    /// The source/precedence contract is broken: the inspector hides the effective
    /// source, collapses its distinct layers, mis-orders precedence, hides or fails
    /// to enforce a policy lock, lets an imported review read as a user-set value,
    /// hides a fallback reason or scope, allows an ambiguous submit, loses its
    /// inspect path, or renders wider than its claim. It must fall back to an explicit
    /// blocked-submit state with an inspect/keyboard recovery path.
    #[serde(rename = "parameter_unsafe")]
    Unsafe,
    /// A review of imported/migrated values: attributable and inspectable but never
    /// reads as a user-set value.
    #[serde(rename = "parameter_review_overlay")]
    ReviewOverlay,
    /// A first-party field held below certified by a labelled, recoverable gap; the
    /// source stays inspectable.
    #[serde(rename = "parameter_narrowed")]
    Narrowed,
    /// Full source-explicit, precedence-correct, scope-explicit, lock-honoured
    /// parameter-source contract.
    #[serde(rename = "parameter_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "parameter_labs_not_claimed")]
    LabsNotClaimed,
}

impl ParameterClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsafe => "parameter_unsafe",
            Self::ReviewOverlay => "parameter_review_overlay",
            Self::Narrowed => "parameter_narrowed",
            Self::Certified => "parameter_certified",
            Self::LabsNotClaimed => "parameter_labs_not_claimed",
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

    /// Whether rendering `rendered` would overclaim relative to this effective claim.
    /// A rendering surface must never render wider than the field's effective claim;
    /// the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: ParameterClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a field fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterNarrowingReason {
    /// The effective value's source layer is not surfaced.
    EffectiveSourceHidden,
    /// The distinct source layers collapse into one current field state.
    SourcesCollapsed,
    /// The declared effective layer is not the highest-precedence present candidate.
    PrecedenceInconsistent,
    /// A policy lock is not surfaced.
    PolicyLockHidden,
    /// A policy-locked field still allows a silent user override, or its effective
    /// value is not the policy-provided value.
    PolicyLockNotEnforced,
    /// An imported/migration review value reads as a user-set value.
    ImportedValueReadsAsUserSet,
    /// The effective value fell back to a built-in/auto source but the reason is not
    /// disclosed.
    FallbackReasonHidden,
    /// The effective value's scope (personal/local vs workspace/shared vs
    /// policy-owned) is not surfaced.
    ValueScopeHidden,
    /// A mutation-capable field allows submit from an ambiguous source-hidden state.
    AmbiguousSubmitAllowed,
    /// The inspect-to-source path is lost.
    InspectPathLost,
    /// A rendering surface renders wider than the effective claim.
    InspectorOverclaims,
    /// The source-provenance snapshot is missing.
    ProvenanceBackingMissing,
    /// A non-winning present candidate's source layer is not labelled.
    SourceLabelsUnlabeled,
    /// A non-winning present candidate's scope tag is not labelled.
    ScopeLabelsUnlabeled,
    /// The fallback reason is disclosed but generic/unlabelled.
    FallbackReasonUnlabeled,
    /// The precedence explanation is not surfaced.
    PrecedenceExplanationUnlabeled,
    /// The detection freshness state is not surfaced.
    DetectionStateUnlabeled,
    /// A superseded detection snapshot is not marked.
    DetectionSupersededUnmarked,
    /// The detection snapshot is stale.
    DetectionStale,
    /// The verification proof is stale.
    VerificationProofStale,
    /// The verification proof is missing.
    VerificationProofMissing,
}

impl ParameterNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectiveSourceHidden => "effective_source_hidden",
            Self::SourcesCollapsed => "sources_collapsed",
            Self::PrecedenceInconsistent => "precedence_inconsistent",
            Self::PolicyLockHidden => "policy_lock_hidden",
            Self::PolicyLockNotEnforced => "policy_lock_not_enforced",
            Self::ImportedValueReadsAsUserSet => "imported_value_reads_as_user_set",
            Self::FallbackReasonHidden => "fallback_reason_hidden",
            Self::ValueScopeHidden => "value_scope_hidden",
            Self::AmbiguousSubmitAllowed => "ambiguous_submit_allowed",
            Self::InspectPathLost => "inspect_path_lost",
            Self::InspectorOverclaims => "inspector_overclaims",
            Self::ProvenanceBackingMissing => "provenance_backing_missing",
            Self::SourceLabelsUnlabeled => "source_labels_unlabeled",
            Self::ScopeLabelsUnlabeled => "scope_labels_unlabeled",
            Self::FallbackReasonUnlabeled => "fallback_reason_unlabeled",
            Self::PrecedenceExplanationUnlabeled => "precedence_explanation_unlabeled",
            Self::DetectionStateUnlabeled => "detection_state_unlabeled",
            Self::DetectionSupersededUnmarked => "detection_superseded_unmarked",
            Self::DetectionStale => "detection_stale",
            Self::VerificationProofStale => "verification_proof_stale",
            Self::VerificationProofMissing => "verification_proof_missing",
        }
    }

    /// Deterministic ordering index (mirrors the validator's reason order).
    pub const fn order_index(self) -> u8 {
        match self {
            Self::EffectiveSourceHidden => 0,
            Self::SourcesCollapsed => 1,
            Self::PrecedenceInconsistent => 2,
            Self::PolicyLockHidden => 3,
            Self::PolicyLockNotEnforced => 4,
            Self::ImportedValueReadsAsUserSet => 5,
            Self::FallbackReasonHidden => 6,
            Self::ValueScopeHidden => 7,
            Self::AmbiguousSubmitAllowed => 8,
            Self::InspectPathLost => 9,
            Self::InspectorOverclaims => 10,
            Self::ProvenanceBackingMissing => 11,
            Self::SourceLabelsUnlabeled => 12,
            Self::ScopeLabelsUnlabeled => 13,
            Self::FallbackReasonUnlabeled => 14,
            Self::PrecedenceExplanationUnlabeled => 15,
            Self::DetectionStateUnlabeled => 16,
            Self::DetectionSupersededUnmarked => 17,
            Self::DetectionStale => 18,
            Self::VerificationProofStale => 19,
            Self::VerificationProofMissing => 20,
        }
    }

    /// Whether this reason breaks the contract outright (floors the field to
    /// [`ParameterClaim::Unsafe`]) rather than merely aging it out.
    pub const fn is_floor(self) -> bool {
        self.order_index() <= Self::ProvenanceBackingMissing.order_index()
    }

    /// A reviewer-facing, non-generic description of the reason.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::EffectiveSourceHidden => "the effective value's source layer is not surfaced",
            Self::SourcesCollapsed => {
                "the distinct source layers collapse into one current field state"
            }
            Self::PrecedenceInconsistent => {
                "the declared effective layer is not the highest-precedence present candidate"
            }
            Self::PolicyLockHidden => "a policy lock is not surfaced",
            Self::PolicyLockNotEnforced => {
                "a policy-locked field still allows a silent user override"
            }
            Self::ImportedValueReadsAsUserSet => {
                "an imported/migration review value reads as a user-set value"
            }
            Self::FallbackReasonHidden => {
                "the effective value fell back to a default but the reason is not disclosed"
            }
            Self::ValueScopeHidden => "the effective value's scope is not surfaced",
            Self::AmbiguousSubmitAllowed => {
                "submit is allowed from an ambiguous source-hidden state"
            }
            Self::InspectPathLost => "the inspect-to-source path is lost",
            Self::InspectorOverclaims => {
                "a rendering surface renders wider than the effective claim"
            }
            Self::ProvenanceBackingMissing => "the source-provenance snapshot is missing",
            Self::SourceLabelsUnlabeled => "a non-winning candidate's source layer is not labelled",
            Self::ScopeLabelsUnlabeled => "a non-winning candidate's scope tag is not labelled",
            Self::FallbackReasonUnlabeled => "the fallback reason is generic or unlabelled",
            Self::PrecedenceExplanationUnlabeled => "the precedence explanation is not surfaced",
            Self::DetectionStateUnlabeled => "the detection freshness state is not surfaced",
            Self::DetectionSupersededUnmarked => "a superseded detection snapshot is not marked",
            Self::DetectionStale => "the detection snapshot is stale",
            Self::VerificationProofStale => "the verification proof is stale",
            Self::VerificationProofMissing => "the verification proof is missing",
        }
    }
}

fn order_reasons(mut reasons: Vec<ParameterNarrowingReason>) -> Vec<ParameterNarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Inspector sub-objects.
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
pub struct FieldLineage {
    /// The inspect/session ref.
    pub session_ref: String,
    /// The canonical inspector this view re-renders, when distinct.
    pub canonical_inspector_ref: Option<String>,
    /// The form ref this field belongs to.
    pub form_ref: Option<String>,
    /// The provider ref, for provider-backed fields.
    pub provider_ref: Option<String>,
    /// The imported source-artifact ref, for review overlays.
    pub source_artifact_ref: Option<String>,
    /// The policy ref, for policy-provided/locked values.
    pub policy_ref: Option<String>,
    /// The environment-profile ref, for environment-resolved values.
    pub environment_profile_ref: Option<String>,
    /// The reopen-to-field backlink ref.
    pub reopen_backlink_ref: Option<String>,
}

/// One candidate value for a field, from a single [`SourceLayer`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceCandidate {
    /// Stable candidate id.
    pub candidate_id: String,
    /// The source layer this candidate comes from.
    pub source_layer: SourceLayer,
    /// The candidate value's scope.
    pub value_scope: ValueScope,
    /// Whether this layer supplies a value for the field.
    pub present: bool,
    /// Whether the source layer is individually labelled (kept visually distinct).
    pub source_labeled: bool,
    /// Whether the value scope is individually labelled.
    pub scope_labeled: bool,
    /// Reviewer-facing label summary (no raw value bodies).
    pub label_summary: String,
}

/// The effective (winning) resolution for a field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveResolution {
    /// The winning source layer.
    pub effective_source_layer: SourceLayer,
    /// The winning value's scope.
    pub effective_value_scope: ValueScope,
    /// Whether the effective source is surfaced.
    pub effective_source_visible: bool,
    /// Whether the effective scope is surfaced.
    pub effective_scope_visible: bool,
    /// The declared precedence rank of the effective layer (must equal the layer's
    /// canonical [`SourceLayer::precedence_rank`]).
    pub precedence_rank_declared: u8,
    /// Reviewer-facing effective-value label.
    pub value_label: String,
}

/// The policy-lock posture of a field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLock {
    /// Whether the field is policy-locked.
    pub policy_locked: bool,
    /// Whether the lock state is surfaced.
    pub lock_surfaced: bool,
    /// Whether the form still allows a user override despite the lock.
    pub override_allowed_despite_lock: bool,
    /// Reviewer-facing lock label.
    pub lock_label: String,
}

/// The fallback disclosure for a field whose effective value fell back to a
/// built-in/auto source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackDisclosure {
    /// Whether the effective value is a fallback (a built-in/auto source won because
    /// no higher-precedence value was set).
    pub is_fallback: bool,
    /// Whether the fallback reason is disclosed.
    pub fallback_reason_disclosed: bool,
    /// Whether the fallback reason is specifically labelled (not generic).
    pub fallback_reason_labeled: bool,
    /// Reviewer-facing fallback label.
    pub fallback_label: String,
}

/// The parameter-source inspector payload for a field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterInspector {
    /// The per-layer source candidates.
    pub candidates: Vec<SourceCandidate>,
    /// Whether the distinct source layers stay visually distinct (not collapsed).
    pub sources_distinct: bool,
    /// Whether the precedence explanation is surfaced.
    pub precedence_explained: bool,
    /// The effective resolution.
    pub effective: EffectiveResolution,
    /// The policy-lock posture.
    pub policy_lock: PolicyLock,
    /// The fallback disclosure.
    pub fallback: FallbackDisclosure,
}

/// The headline source/precedence invariants every field re-derives rather than
/// trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorIntegrity {
    /// The effective source layer is visible.
    pub effective_source_visible: bool,
    /// The distinct source layers stay visually distinct.
    pub sources_visually_distinct: bool,
    /// The precedence ordering is visible.
    pub precedence_visible: bool,
    /// The policy-lock state is visible.
    pub policy_lock_visible: bool,
    /// The effective value scope is visible.
    pub value_scope_visible: bool,
    /// The fallback reason is visible.
    pub fallback_reason_visible: bool,
    /// Imported/migration reviews stay read-only.
    pub imported_review_read_only: bool,
    /// Submit is gated on source clarity (no ambiguous submit).
    pub submit_gated_on_source_clarity: bool,
    /// The detection freshness state is visible.
    pub detection_state_visible: bool,
    /// A superseded detection snapshot stays marked.
    pub superseded_state_marked: bool,
}

/// Verification-proof currency for a field (distinct from detection freshness).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldVerification {
    /// Currency of the verification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the field.
    pub proof_ref: Option<String>,
}

/// One consumer surface that renders a field record, with the claim it shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorRendering {
    /// The consumer surface.
    pub surface: ConsumerSurface,
    /// The claim this surface renders.
    pub rendered_claim: ParameterClaim,
    /// Whether the source provenance is inspectable here.
    pub source_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical field this view re-renders.
    pub source_field_ref: String,
}

// --------------------------------------------------------------------------- //
// Field record + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) parameter-source inspector for an M5 form field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterFieldRecord {
    /// Stable field id.
    pub field_id: String,
    /// The form this field belongs to.
    pub form: FieldForm,
    /// The product lane.
    pub lane: FieldLane,
    /// How the inspector/field originated.
    pub origin: FieldOrigin,
    /// Reviewer-facing label summary.
    pub label_summary: String,
    /// Whether the field is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// Declared detection-snapshot freshness state.
    pub declared_detection_state: DetectionState,
    /// Declared reopen target.
    pub declared_reopen_target: ReopenTarget,
    /// Stable origin-lineage block.
    pub lineage: FieldLineage,
    /// The parameter-source inspector payload.
    pub inspector: ParameterInspector,
    /// Headline invariant block.
    pub integrity: InspectorIntegrity,
    /// Verification-proof block.
    pub verification: FieldVerification,
    /// Consumer surfaces that render this record.
    pub renderings: Vec<InspectorRendering>,
}

/// The re-derived field decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDecision {
    /// The headline claim the field is eligible to make.
    pub claimed_claim: ParameterClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_claim: ParameterClaim,
    /// Ordered, de-duplicated reasons the field fails to hold its headline.
    pub active_narrowing_reasons: Vec<ParameterNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl FieldDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<ParameterNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this record would overclaim.
    pub fn surface_overclaims(&self, rendered: ParameterClaim) -> bool {
        self.effective_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(
    claimed: ParameterClaim,
    reasons: &[ParameterNarrowingReason],
) -> ParameterClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        ParameterClaim::Unsafe
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, ParameterClaim::ReviewOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we can
        // no longer certify even the read-only review, so it floors.
        ParameterClaim::Unsafe
    } else {
        ParameterClaim::Narrowed
    }
}

impl ParameterFieldRecord {
    /// Whether this field is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this field is an inherently read-only review overlay.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin.is_overlay()
    }

    /// The highest-precedence *present* candidate layer, if any.
    pub fn highest_present_layer(&self) -> Option<SourceLayer> {
        self.inspector
            .candidates
            .iter()
            .filter(|c| c.present)
            .map(|c| c.source_layer)
            .max_by_key(|l| l.precedence_rank())
    }

    /// The candidate matching the declared effective layer, if any.
    fn effective_candidate(&self) -> Option<&SourceCandidate> {
        let eff = self.inspector.effective.effective_source_layer;
        self.inspector
            .candidates
            .iter()
            .find(|c| c.source_layer == eff)
    }

    /// The headline claim this field is eligible to make.
    pub fn claimed_claim(&self) -> ParameterClaim {
        if self.is_labs() {
            ParameterClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            ParameterClaim::ReviewOverlay
        } else {
            ParameterClaim::Certified
        }
    }

    /// Reasons that hold independently of how the consumer surfaces render — the
    /// intrinsic source/precedence/scope/recovery gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<ParameterNarrowingReason> {
        use ParameterNarrowingReason as R;
        let insp = &self.inspector;
        let eff = &insp.effective;
        let integ = &self.integrity;
        let overlay = self.is_overlay_origin();
        let eff_layer = eff.effective_source_layer;
        let eff_candidate = self.effective_candidate();
        let mut reasons: Vec<R> = Vec::new();

        // Effective source visible + the winning candidate present and labelled.
        if !eff.effective_source_visible
            || !integ.effective_source_visible
            || eff_candidate.map_or(true, |c| !c.present || !c.source_labeled)
        {
            reasons.push(R::EffectiveSourceHidden);
        }

        // Distinct source layers.
        if !insp.sources_distinct || !integ.sources_visually_distinct {
            reasons.push(R::SourcesCollapsed);
        }

        // Precedence: the effective layer must be the highest-precedence present
        // candidate, and its declared rank must match the canonical rank.
        if self.highest_present_layer() != Some(eff_layer)
            || eff.precedence_rank_declared != eff_layer.precedence_rank()
        {
            reasons.push(R::PrecedenceInconsistent);
        }

        // Policy lock: when locked, surface the lock and pin to policy_provided.
        if insp.policy_lock.policy_locked
            && (!insp.policy_lock.lock_surfaced || !integ.policy_lock_visible)
        {
            reasons.push(R::PolicyLockHidden);
        }
        if insp.policy_lock.policy_locked
            && (insp.policy_lock.override_allowed_despite_lock
                || eff_layer != SourceLayer::PolicyProvided)
        {
            reasons.push(R::PolicyLockNotEnforced);
        }

        // Imported overlay read-only.
        if overlay && !integ.imported_review_read_only {
            reasons.push(R::ImportedValueReadsAsUserSet);
        }

        // Fallback reason.
        if insp.fallback.is_fallback
            && (!insp.fallback.fallback_reason_disclosed || !integ.fallback_reason_visible)
        {
            reasons.push(R::FallbackReasonHidden);
        }

        // Value scope.
        if !eff.effective_scope_visible
            || !integ.value_scope_visible
            || eff_candidate.map_or(true, |c| !c.scope_labeled)
        {
            reasons.push(R::ValueScopeHidden);
        }

        // Ambiguous submit (the guardrail): a mutation-capable field must gate submit
        // on source clarity. An overlay is read-only, so the gate does not apply.
        if !overlay && !integ.submit_gated_on_source_clarity {
            reasons.push(R::AmbiguousSubmitAllowed);
        }

        // Inspect-to-source path.
        if self.renderings.iter().any(|r| !r.source_visible)
            || matches!(
                self.declared_reopen_target,
                ReopenTarget::NoneKeyboardFallback
            )
        {
            reasons.push(R::InspectPathLost);
        }

        // Provenance backing freshness.
        match self.declared_detection_state {
            DetectionState::Missing => reasons.push(R::ProvenanceBackingMissing),
            DetectionState::SupersededByNewerSource if !integ.superseded_state_marked => {
                reasons.push(R::DetectionSupersededUnmarked);
            }
            DetectionState::StaleExpired if !overlay => reasons.push(R::DetectionStale),
            _ => {}
        }
        if !integ.detection_state_visible {
            reasons.push(R::DetectionStateUnlabeled);
        }

        // Non-winning candidate labelling (non-floor).
        let non_winning_source_unlabeled = insp
            .candidates
            .iter()
            .any(|c| c.present && c.source_layer != eff_layer && !c.source_labeled);
        if non_winning_source_unlabeled {
            reasons.push(R::SourceLabelsUnlabeled);
        }
        let non_winning_scope_unlabeled = insp
            .candidates
            .iter()
            .any(|c| c.present && c.source_layer != eff_layer && !c.scope_labeled);
        if non_winning_scope_unlabeled {
            reasons.push(R::ScopeLabelsUnlabeled);
        }

        // Fallback reason labelling (non-floor).
        if insp.fallback.is_fallback && !insp.fallback.fallback_reason_labeled {
            reasons.push(R::FallbackReasonUnlabeled);
        }

        // Precedence explanation (non-floor).
        if !insp.precedence_explained || !integ.precedence_visible {
            reasons.push(R::PrecedenceExplanationUnlabeled);
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

    /// All active narrowing reasons, including the rendering-surface overclaim check,
    /// ordered and de-duplicated.
    fn reasons(&self, stale_window: bool) -> Vec<ParameterNarrowingReason> {
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic_effective = derive_effective(self.claimed_claim(), &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic_effective.overclaims_as(r.rendered_claim))
        {
            reasons.push(ParameterNarrowingReason::InspectorOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive this field's claim decision.
    pub fn narrow(&self, stale_window: bool) -> FieldDecision {
        let claimed = self.claimed_claim();
        if matches!(claimed, ParameterClaim::LabsNotClaimed) {
            return FieldDecision {
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
        FieldDecision {
            claimed_claim: claimed,
            effective_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// Whether a floored field still keeps an inspect/keyboard recovery fallback
    /// rather than a misleading clean submit.
    pub fn floored_keeps_fallback(&self, effective: ParameterClaim) -> bool {
        if !matches!(effective, ParameterClaim::Unsafe) {
            return true;
        }
        matches!(
            self.declared_reopen_target,
            ReopenTarget::InspectorOnly | ReopenTarget::NoneKeyboardFallback
        ) || opt_present(&self.lineage.reopen_backlink_ref)
    }

    /// Whether any rendering overclaims relative to `effective`.
    pub fn surface_overclaims(&self, effective: ParameterClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// A reviewer-facing label for a narrowed/floored field, or `None` if the field
    /// holds its claim.
    pub fn narrowed_label(&self, decision: &FieldDecision) -> Option<String> {
        let trigger = decision.downgrade_trigger()?;
        Some(match decision.effective_claim {
            ParameterClaim::Unsafe => format!(
                "Floored to parameter_unsafe below the {} claim: {}; falls back to an explicit blocked-submit state with inspect/keyboard recovery.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            ParameterClaim::Narrowed => format!(
                "Held at parameter_narrowed below the {} claim: {}; the source stays inspectable until re-verified.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            _ => return None,
        })
    }

    /// Append per-field structural violations (schema-shape level).
    fn structural_violations(&self, out: &mut Vec<M5ParameterSourceViolation>) {
        use M5ParameterSourceViolation as V;
        if self.field_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.lineage.session_ref.trim().is_empty()
        {
            out.push(V::FieldMissingIdentity);
        }
        if self.is_overlay_origin()
            && !opt_present(&self.lineage.provider_ref)
            && !opt_present(&self.lineage.source_artifact_ref)
        {
            out.push(V::OverlayMissingProvenanceRef);
        }
        if self.inspector.candidates.is_empty() {
            out.push(V::FieldMissingCandidates);
        }
        if self.renderings.is_empty() {
            out.push(V::FieldMissingRendering);
        }
        for r in &self.renderings {
            if r.source_field_ref.trim().is_empty() {
                out.push(V::RenderingMissingSourceRef);
            }
        }
    }
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for [`M5ParameterSourceSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ParameterSourceSetInput {
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
    /// Per-field rows.
    pub fields: Vec<ParameterFieldRecord>,
}

/// Export-safe M5 parameter-source set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ParameterSourceSetPacket {
    /// Record kind; must equal [`M5_PARAMETER_SOURCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PARAMETER_SOURCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_PARAMETER_SOURCE_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-field rows.
    pub fields: Vec<ParameterFieldRecord>,
}

/// The distribution of effective field claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldClaimDistribution {
    /// Fields effective at [`ParameterClaim::Certified`].
    pub certified: usize,
    /// Fields effective at [`ParameterClaim::Narrowed`].
    pub narrowed: usize,
    /// Fields effective at [`ParameterClaim::ReviewOverlay`].
    pub overlay: usize,
    /// Fields effective at [`ParameterClaim::Unsafe`].
    pub unsafe_fields: usize,
    /// Fields effective at [`ParameterClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5ParameterSourceSetPacket {
    /// Builds a parameter-source set packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5ParameterSourceSetInput) -> Self {
        Self {
            record_kind: M5_PARAMETER_SOURCE_RECORD_KIND.to_owned(),
            schema_version: M5_PARAMETER_SOURCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_PARAMETER_SOURCE_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            fields: input.fields,
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

    /// Re-derive the decision for every field, paired with its id.
    pub fn decisions(&self) -> Vec<(String, FieldDecision)> {
        let stale_window = self.stale_window();
        self.fields
            .iter()
            .map(|f| (f.field_id.clone(), f.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective field claims.
    pub fn claim_distribution(&self) -> FieldClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = FieldClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            unsafe_fields: 0,
            labs: 0,
        };
        for f in &self.fields {
            match f.narrow(stale_window).effective_claim {
                ParameterClaim::Certified => dist.certified += 1,
                ParameterClaim::Narrowed => dist.narrowed += 1,
                ParameterClaim::ReviewOverlay => dist.overlay += 1,
                ParameterClaim::Unsafe => dist.unsafe_fields += 1,
                ParameterClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of fields whose effective claim ranks below their claimed claim.
    pub fn narrowed_field_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.fields
            .iter()
            .filter(|f| f.narrow(stale_window).narrowed)
            .count()
    }

    /// Forms represented by some field.
    pub fn represented_forms(&self) -> BTreeSet<FieldForm> {
        self.fields.iter().map(|f| f.form).collect()
    }

    /// Product lanes represented by some field.
    pub fn represented_lanes(&self) -> BTreeSet<FieldLane> {
        self.fields.iter().map(|f| f.lane).collect()
    }

    /// Source layers represented by some candidate.
    pub fn represented_source_layers(&self) -> BTreeSet<SourceLayer> {
        self.fields
            .iter()
            .flat_map(|f| f.inspector.candidates.iter().map(|c| c.source_layer))
            .collect()
    }

    /// Value scopes represented by some candidate.
    pub fn represented_value_scopes(&self) -> BTreeSet<ValueScope> {
        self.fields
            .iter()
            .flat_map(|f| f.inspector.candidates.iter().map(|c| c.value_scope))
            .collect()
    }

    /// Consumer surfaces represented by some rendering.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<ConsumerSurface> {
        self.fields
            .iter()
            .flat_map(|f| f.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Validate the parameter-source invariants.
    pub fn validate(&self) -> Vec<M5ParameterSourceViolation> {
        use M5ParameterSourceViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_PARAMETER_SOURCE_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_PARAMETER_SOURCE_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_PARAMETER_SOURCE_TAXONOMY_VERSION {
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
        if self.fields.is_empty() {
            violations.push(V::EmptyFields);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for f in &self.fields {
            if !seen.insert(f.field_id.as_str()) {
                violations.push(V::DuplicateFieldId);
            }
        }

        if FieldForm::ALL
            .iter()
            .any(|x| !self.represented_forms().contains(x))
        {
            violations.push(V::FormMissing);
        }
        if FieldLane::ALL
            .iter()
            .any(|l| !self.represented_lanes().contains(l))
        {
            violations.push(V::FieldLaneMissing);
        }
        if SourceLayer::ALL
            .iter()
            .any(|s| !self.represented_source_layers().contains(s))
        {
            violations.push(V::SourceLayerMissing);
        }
        if ValueScope::ALL
            .iter()
            .any(|s| !self.represented_value_scopes().contains(s))
        {
            violations.push(V::ValueScopeMissing);
        }
        if ConsumerSurface::ALL
            .iter()
            .any(|s| !self.represented_consumer_surfaces().contains(s))
        {
            violations.push(V::ConsumerSurfaceMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for f in &self.fields {
            f.structural_violations(&mut violations);
            let decision = f.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || f.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedFieldMissingLabelOrTrigger);
                }
            }
            if !f.floored_keeps_fallback(decision.effective_claim) {
                violations.push(V::FlooredFieldLosesFallback);
            }
            if f.surface_overclaims(decision.effective_claim) {
                violations.push(V::RenderingFieldOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedFieldCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("parameter-source packet serializes"),
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
        serde_json::to_string_pretty(self).expect("parameter-source packet serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_report(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Parameter-Source And Precedence Inspectors Across Forms\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Fields: {}\n", self.fields.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} review overlay, {} unsafe, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.unsafe_fields, dist.labs
        ));

        out.push_str("| Field | Form | Lane | Effective source | Origin | Claimed | Effective |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for f in &self.fields {
            let decision = f.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                f.field_id,
                f.form.as_str(),
                f.lane.as_str(),
                f.inspector.effective.effective_source_layer.as_str(),
                f.origin.as_str(),
                decision.claimed_claim.as_str(),
                decision.effective_claim.as_str(),
            ));
        }

        out.push('\n');
        for f in &self.fields {
            let decision = f.narrow(stale_window);
            if let Some(label) = f.narrowed_label(&decision) {
                out.push_str(&format!("- {}: {}\n", f.field_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum M5ParameterSourceArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5ParameterSourceViolation>),
}

impl fmt::Display for M5ParameterSourceArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(f, "support export parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for M5ParameterSourceArtifactError {}

/// A parameter-source packet validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ParameterSourceViolation {
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
    /// The set has no fields.
    EmptyFields,
    /// Two fields share a field id.
    DuplicateFieldId,
    /// A form is unrepresented.
    FormMissing,
    /// A product lane is unrepresented.
    FieldLaneMissing,
    /// A source layer is unrepresented.
    SourceLayerMissing,
    /// A value scope is unrepresented.
    ValueScopeMissing,
    /// A consumer surface is unrepresented.
    ConsumerSurfaceMissing,
    /// A field lacks a required identity field.
    FieldMissingIdentity,
    /// An overlay field names no provider/source-artifact ref.
    OverlayMissingProvenanceRef,
    /// A field has no source candidates.
    FieldMissingCandidates,
    /// A field has no renderings.
    FieldMissingRendering,
    /// A rendering names no source field ref.
    RenderingMissingSourceRef,
    /// A narrowed field lacks a non-generic label or a downgrade trigger.
    NarrowedFieldMissingLabelOrTrigger,
    /// A floored field loses its inspect/keyboard fallback.
    FlooredFieldLosesFallback,
    /// A rendering surface renders wider than the effective claim.
    RenderingFieldOverclaims,
    /// No field demonstrates the auto-narrowing rule.
    DowngradedFieldCaseMissing,
    /// Raw boundary material crossed the export.
    RawBoundaryMaterialInExport,
}

impl M5ParameterSourceViolation {
    /// Stable token recorded in diagnostics and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyFields => "empty_fields",
            Self::DuplicateFieldId => "duplicate_field_id",
            Self::FormMissing => "form_missing",
            Self::FieldLaneMissing => "field_lane_missing",
            Self::SourceLayerMissing => "source_layer_missing",
            Self::ValueScopeMissing => "value_scope_missing",
            Self::ConsumerSurfaceMissing => "consumer_surface_missing",
            Self::FieldMissingIdentity => "field_missing_identity",
            Self::OverlayMissingProvenanceRef => "overlay_missing_provenance_ref",
            Self::FieldMissingCandidates => "field_missing_candidates",
            Self::FieldMissingRendering => "field_missing_rendering",
            Self::RenderingMissingSourceRef => "rendering_missing_source_ref",
            Self::NarrowedFieldMissingLabelOrTrigger => "narrowed_field_missing_label_or_trigger",
            Self::FlooredFieldLosesFallback => "floored_field_loses_fallback",
            Self::RenderingFieldOverclaims => "rendering_field_overclaims",
            Self::DowngradedFieldCaseMissing => "downgraded_field_case_missing",
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
/// support, admin, import, and project surfaces — plus the CLI/headless inspect
/// path and the docs/help references — use to ingest the frozen parameter-source
/// matrix instead of minting per-feature source-precedence semantics.
///
/// # Errors
///
/// Returns [`M5ParameterSourceArtifactError`] when the artifact cannot be parsed or
/// fails validation.
pub fn current_m5_parameter_source_set(
) -> Result<M5ParameterSourceSetPacket, M5ParameterSourceArtifactError> {
    let packet: M5ParameterSourceSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ux/m5-parameter-source-and-precedence/support_export.json"
    )))
    .map_err(M5ParameterSourceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ParameterSourceArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded parameter-source set: the in-crate source of truth the
/// checked-in support export and report are regenerated from.
pub fn seeded_m5_parameter_source_set() -> M5ParameterSourceSetPacket {
    M5ParameterSourceSetPacket::new(M5ParameterSourceSetInput {
        packet_id: M5_PARAMETER_SOURCE_PACKET_ID.to_owned(),
        label:
            "M5 parameter-source and precedence inspectors — default, detected, imported, environment, policy, and user-override values with effective precedence, scope, and locks across M5 forms"
                .to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        fields: seed_fields(),
    })
}

/// Renderings that show `claim` cleanly across the named surfaces.
fn renderings(
    source_ref: &str,
    claim: ParameterClaim,
    surfaces: &[ConsumerSurface],
    read_only: bool,
) -> Vec<InspectorRendering> {
    surfaces
        .iter()
        .map(|&surface| InspectorRendering {
            surface,
            rendered_claim: claim,
            source_visible: true,
            read_only,
            source_field_ref: source_ref.to_owned(),
        })
        .collect()
}

/// A clean first-party integrity block.
fn clean_integrity() -> InspectorIntegrity {
    InspectorIntegrity {
        effective_source_visible: true,
        sources_visually_distinct: true,
        precedence_visible: true,
        policy_lock_visible: true,
        value_scope_visible: true,
        fallback_reason_visible: true,
        imported_review_read_only: true,
        submit_gated_on_source_clarity: true,
        detection_state_visible: true,
        superseded_state_marked: true,
    }
}

/// A verified-current verification block.
fn verified(proof_ref: &str) -> FieldVerification {
    FieldVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

fn candidate(
    candidate_id: &str,
    source_layer: SourceLayer,
    value_scope: ValueScope,
    present: bool,
    label: &str,
) -> SourceCandidate {
    SourceCandidate {
        candidate_id: candidate_id.to_owned(),
        source_layer,
        value_scope,
        present,
        source_labeled: true,
        scope_labeled: true,
        label_summary: label.to_owned(),
    }
}

/// An effective-resolution block whose declared rank matches the layer.
fn effective(layer: SourceLayer, scope: ValueScope, label: &str) -> EffectiveResolution {
    EffectiveResolution {
        effective_source_layer: layer,
        effective_value_scope: scope,
        effective_source_visible: true,
        effective_scope_visible: true,
        precedence_rank_declared: layer.precedence_rank(),
        value_label: label.to_owned(),
    }
}

/// An unlocked policy block.
fn no_lock() -> PolicyLock {
    PolicyLock {
        policy_locked: false,
        lock_surfaced: false,
        override_allowed_despite_lock: false,
        lock_label: "Not policy-locked; you may override this value.".to_owned(),
    }
}

/// A non-fallback disclosure block.
fn no_fallback() -> FallbackDisclosure {
    FallbackDisclosure {
        is_fallback: false,
        fallback_reason_disclosed: false,
        fallback_reason_labeled: false,
        fallback_label: "Not a fallback; an explicit value supplies this field.".to_owned(),
    }
}

/// The canonical fields: one per form, covering every source layer (as a candidate
/// and as an effective value), every value scope, and every consumer surface, plus a
/// narrowed first-party field, an import review overlay, and a Labs field.
fn seed_fields() -> Vec<ParameterFieldRecord> {
    use ConsumerSurface as CS;

    // 1. Provider account-mapping: the effective value was auto-detected, with a
    //    built-in default below it. Certified.
    let provider = ParameterFieldRecord {
        field_id: "field:provider-account-mapping:0001".to_owned(),
        form: FieldForm::ProviderAccountMapping,
        lane: FieldLane::Provider,
        origin: FieldOrigin::ProviderForm,
        label_summary: "Provider account: the current value was auto-detected from the connected provider; the built-in default sits below it.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_detection_state: DetectionState::CachedSnapshot,
        declared_reopen_target: ReopenTarget::FieldAndInspector,
        lineage: FieldLineage {
            session_ref: "session:provider-account:0001".to_owned(),
            canonical_inspector_ref: None,
            form_ref: Some("form:provider-account-mapping".to_owned()),
            provider_ref: Some("provider:registered:0001".to_owned()),
            source_artifact_ref: None,
            policy_ref: None,
            environment_profile_ref: None,
            reopen_backlink_ref: Some("reopen:provider-form:0001".to_owned()),
        },
        inspector: ParameterInspector {
            candidates: vec![
                candidate(
                    "candidate:provider-detected",
                    SourceLayer::Detected,
                    ValueScope::WorkspaceShared,
                    true,
                    "Detected: account inferred from the connected provider (workspace-shared).",
                ),
                candidate(
                    "candidate:provider-default",
                    SourceLayer::Default,
                    ValueScope::PersonalLocal,
                    true,
                    "Default: the built-in placeholder account (personal/local).",
                ),
            ],
            sources_distinct: true,
            precedence_explained: true,
            effective: effective(
                SourceLayer::Detected,
                ValueScope::WorkspaceShared,
                "Effective: the detected account wins over the built-in default.",
            ),
            policy_lock: no_lock(),
            fallback: no_fallback(),
        },
        integrity: clean_integrity(),
        verification: verified("proof:provider-account:0001"),
        renderings: renderings(
            "field:provider-account-mapping:0001",
            ParameterClaim::Certified,
            &[CS::InspectorPanel, CS::FieldPopover, CS::CliInspect],
            false,
        ),
    };

    // 2. Admin source-registration: policy-provided and policy-locked, with a user
    //    override present below the lock so the user can see it loses to policy.
    //    Certified.
    let admin = ParameterFieldRecord {
        field_id: "field:source-registration:0001".to_owned(),
        form: FieldForm::SourceRegistration,
        lane: FieldLane::Admin,
        origin: FieldOrigin::RemoteForm,
        label_summary: "Source trust policy: the value is policy-provided and locked; your override is shown below the lock so you can see it does not win.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_detection_state: DetectionState::Live,
        declared_reopen_target: ReopenTarget::FieldAndInspector,
        lineage: FieldLineage {
            session_ref: "session:source-registration:0001".to_owned(),
            canonical_inspector_ref: None,
            form_ref: Some("form:source-registration".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: Some("policy:source-trust:0001".to_owned()),
            environment_profile_ref: None,
            reopen_backlink_ref: Some("reopen:admin-console:0001".to_owned()),
        },
        inspector: ParameterInspector {
            candidates: vec![
                candidate(
                    "candidate:source-policy",
                    SourceLayer::PolicyProvided,
                    ValueScope::PolicyOwned,
                    true,
                    "Policy-provided: the trust level required by workspace policy (policy-owned).",
                ),
                candidate(
                    "candidate:source-override",
                    SourceLayer::UserOverride,
                    ValueScope::PersonalLocal,
                    true,
                    "Your override: kept visible, but it does not win while the policy lock holds.",
                ),
                candidate(
                    "candidate:source-default",
                    SourceLayer::Default,
                    ValueScope::PersonalLocal,
                    true,
                    "Default: the built-in trust level (personal/local).",
                ),
            ],
            sources_distinct: true,
            precedence_explained: true,
            effective: effective(
                SourceLayer::PolicyProvided,
                ValueScope::PolicyOwned,
                "Effective: the policy-provided trust level wins and is locked.",
            ),
            policy_lock: PolicyLock {
                policy_locked: true,
                lock_surfaced: true,
                override_allowed_despite_lock: false,
                lock_label: "Locked by workspace policy; your override is recorded but cannot apply.".to_owned(),
            },
            fallback: no_fallback(),
        },
        integrity: clean_integrity(),
        verification: verified("proof:source-registration:0001"),
        renderings: renderings(
            "field:source-registration:0001",
            ParameterClaim::Certified,
            &[CS::InspectorPanel, CS::DiagnosticsPanel, CS::AiEvidence],
            false,
        ),
    };

    // 3. Request-workspace environment: resolved from an environment profile, with a
    //    built-in default below it. Certified.
    let request = ParameterFieldRecord {
        field_id: "field:request-environment:0001".to_owned(),
        form: FieldForm::RequestEnvironment,
        lane: FieldLane::Request,
        origin: FieldOrigin::RemoteForm,
        label_summary: "Request base URL: resolved from the active environment profile; the built-in default sits below it. Its verification proof requires review.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_detection_state: DetectionState::Live,
        declared_reopen_target: ReopenTarget::FieldAndInspector,
        lineage: FieldLineage {
            session_ref: "session:request-environment:0001".to_owned(),
            canonical_inspector_ref: None,
            form_ref: Some("form:request-environment".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: None,
            environment_profile_ref: Some("environment:staging-profile:0001".to_owned()),
            reopen_backlink_ref: Some("reopen:request-workspace:0001".to_owned()),
        },
        inspector: ParameterInspector {
            candidates: vec![
                candidate(
                    "candidate:request-env",
                    SourceLayer::EnvironmentResolved,
                    ValueScope::WorkspaceShared,
                    true,
                    "Environment-resolved: the base URL from the active environment profile (workspace-shared).",
                ),
                candidate(
                    "candidate:request-default",
                    SourceLayer::Default,
                    ValueScope::PersonalLocal,
                    true,
                    "Default: the built-in localhost base URL (personal/local).",
                ),
            ],
            sources_distinct: true,
            precedence_explained: true,
            effective: effective(
                SourceLayer::EnvironmentResolved,
                ValueScope::WorkspaceShared,
                "Effective: the environment-resolved base URL wins over the default.",
            ),
            policy_lock: no_lock(),
            fallback: no_fallback(),
        },
        integrity: clean_integrity(),
        // This field narrows: its verification proof requires review.
        verification: FieldVerification {
            proof_currency: ProofCurrency::RequiresReview,
            proof_ref: Some("proof:request-environment:0001".to_owned()),
        },
        renderings: renderings(
            "field:request-environment:0001",
            ParameterClaim::Narrowed,
            &[CS::InspectorPanel, CS::SupportExport, CS::CliInspect],
            false,
        ),
    };

    // 4. Package install-config: the effective value fell back to the built-in
    //    default because no override was set; the fallback reason is disclosed.
    //    Certified.
    let package = ParameterFieldRecord {
        field_id: "field:package-install-config:0001".to_owned(),
        form: FieldForm::PackageInstallConfig,
        lane: FieldLane::Package,
        origin: FieldOrigin::LocalForm,
        label_summary: "Package install location: no override is set, so the value falls back to the built-in default; the reason is shown.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_detection_state: DetectionState::Live,
        declared_reopen_target: ReopenTarget::FieldAndInspector,
        lineage: FieldLineage {
            session_ref: "session:package-install:0001".to_owned(),
            canonical_inspector_ref: None,
            form_ref: Some("form:package-install-config".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: None,
            environment_profile_ref: None,
            reopen_backlink_ref: Some("reopen:package-manager:0001".to_owned()),
        },
        inspector: ParameterInspector {
            candidates: vec![
                candidate(
                    "candidate:package-default",
                    SourceLayer::Default,
                    ValueScope::PersonalLocal,
                    true,
                    "Default: the built-in install location (personal/local).",
                ),
                candidate(
                    "candidate:package-override",
                    SourceLayer::UserOverride,
                    ValueScope::PersonalLocal,
                    false,
                    "Your override: not set, so it does not contribute a value.",
                ),
            ],
            sources_distinct: true,
            precedence_explained: true,
            effective: effective(
                SourceLayer::Default,
                ValueScope::PersonalLocal,
                "Effective: the built-in default applies because no higher source is set.",
            ),
            policy_lock: no_lock(),
            fallback: FallbackDisclosure {
                is_fallback: true,
                fallback_reason_disclosed: true,
                fallback_reason_labeled: true,
                fallback_label: "Fallback: no override, environment, import, or policy value is set for this field.".to_owned(),
            },
        },
        integrity: clean_integrity(),
        verification: verified("proof:package-install:0001"),
        renderings: renderings(
            "field:package-install-config:0001",
            ParameterClaim::Certified,
            &[CS::InspectorPanel, CS::FieldPopover, CS::HelpInline],
            false,
        ),
    };

    // 5. Settings config editor: a user override wins over imported, detected, and
    //    default candidates — the rich, clean certified baseline.
    let settings = ParameterFieldRecord {
        field_id: "field:settings-config-editor:0001".to_owned(),
        form: FieldForm::SettingsConfigEditor,
        lane: FieldLane::Settings,
        origin: FieldOrigin::LocalForm,
        label_summary: "Editor font size: your override wins over an imported value, a detected value, and the built-in default — all four stay visually distinct.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_detection_state: DetectionState::Live,
        declared_reopen_target: ReopenTarget::FieldAndInspector,
        lineage: FieldLineage {
            session_ref: "session:settings-config:0001".to_owned(),
            canonical_inspector_ref: None,
            form_ref: Some("form:settings-config-editor".to_owned()),
            provider_ref: None,
            source_artifact_ref: Some("artifact:imported-settings:0001".to_owned()),
            policy_ref: None,
            environment_profile_ref: None,
            reopen_backlink_ref: Some("reopen:settings-editor:0001".to_owned()),
        },
        inspector: ParameterInspector {
            candidates: vec![
                candidate(
                    "candidate:settings-override",
                    SourceLayer::UserOverride,
                    ValueScope::PersonalLocal,
                    true,
                    "Your override: font size set to 14 (personal/local).",
                ),
                candidate(
                    "candidate:settings-imported",
                    SourceLayer::Imported,
                    ValueScope::WorkspaceShared,
                    true,
                    "Imported: font size 13 brought in from a migrated profile (workspace-shared).",
                ),
                candidate(
                    "candidate:settings-detected",
                    SourceLayer::Detected,
                    ValueScope::WorkspaceShared,
                    true,
                    "Detected: font size 12 suggested by the display profile (workspace-shared).",
                ),
                candidate(
                    "candidate:settings-default",
                    SourceLayer::Default,
                    ValueScope::PersonalLocal,
                    true,
                    "Default: the built-in font size 13 (personal/local).",
                ),
            ],
            sources_distinct: true,
            precedence_explained: true,
            effective: effective(
                SourceLayer::UserOverride,
                ValueScope::PersonalLocal,
                "Effective: your override wins over imported, detected, and default values.",
            ),
            policy_lock: no_lock(),
            fallback: no_fallback(),
        },
        integrity: clean_integrity(),
        verification: verified("proof:settings-config:0001"),
        renderings: renderings(
            "field:settings-config-editor:0001",
            ParameterClaim::Certified,
            &[CS::InspectorPanel, CS::FieldPopover, CS::AiEvidence],
            false,
        ),
    };

    // 6. Import/migration mapping: a read-only review of imported values; the imported
    //    value is the effective one, but it never reads as a user-set value. Review
    //    overlay.
    let import = ParameterFieldRecord {
        field_id: "field:import-migration-mapping:0001".to_owned(),
        form: FieldForm::ImportMigrationMapping,
        lane: FieldLane::Import,
        origin: FieldOrigin::ImportedReview,
        label_summary: "Imported keymap: review the value the migration brought in before applying it. A read-only review, not a value you have set.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_detection_state: DetectionState::CachedSnapshot,
        declared_reopen_target: ReopenTarget::FieldAndInspector,
        lineage: FieldLineage {
            session_ref: "session:import-migration:0001".to_owned(),
            canonical_inspector_ref: None,
            form_ref: Some("form:import-migration-mapping".to_owned()),
            provider_ref: None,
            source_artifact_ref: Some("artifact:migration-bundle:0001".to_owned()),
            policy_ref: None,
            environment_profile_ref: None,
            reopen_backlink_ref: Some("reopen:migration-center:0001".to_owned()),
        },
        inspector: ParameterInspector {
            candidates: vec![
                candidate(
                    "candidate:import-imported",
                    SourceLayer::Imported,
                    ValueScope::WorkspaceShared,
                    true,
                    "Imported: the keymap brought in by the migration (workspace-shared).",
                ),
                candidate(
                    "candidate:import-default",
                    SourceLayer::Default,
                    ValueScope::PersonalLocal,
                    true,
                    "Default: the built-in keymap (personal/local).",
                ),
            ],
            sources_distinct: true,
            precedence_explained: true,
            effective: effective(
                SourceLayer::Imported,
                ValueScope::WorkspaceShared,
                "Effective in this review: the imported keymap; nothing is applied until you confirm.",
            ),
            policy_lock: no_lock(),
            fallback: no_fallback(),
        },
        integrity: clean_integrity(),
        verification: verified("proof:import-migration:0001"),
        renderings: renderings(
            "field:import-migration-mapping:0001",
            ParameterClaim::ReviewOverlay,
            &[CS::InspectorPanel, CS::SupportExport, CS::HelpInline],
            true,
        ),
    };

    // 7. Labs project-bootstrap field: makes no public claim.
    let labs = ParameterFieldRecord {
        field_id: "field:project-bootstrap:0001".to_owned(),
        form: FieldForm::ProjectBootstrap,
        lane: FieldLane::Projects,
        origin: FieldOrigin::LocalForm,
        label_summary:
            "Experimental project-template field (Labs): a user override over a default, unadvertised."
                .to_owned(),
        claim_posture: ClaimPosture::LabsUnadvertised,
        declared_detection_state: DetectionState::Live,
        declared_reopen_target: ReopenTarget::FieldAndInspector,
        lineage: FieldLineage {
            session_ref: "session:project-bootstrap:0001".to_owned(),
            canonical_inspector_ref: None,
            form_ref: Some("form:project-bootstrap".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: None,
            environment_profile_ref: None,
            reopen_backlink_ref: Some("reopen:project-bootstrap:0001".to_owned()),
        },
        inspector: ParameterInspector {
            candidates: vec![
                candidate(
                    "candidate:bootstrap-override",
                    SourceLayer::UserOverride,
                    ValueScope::PersonalLocal,
                    true,
                    "Your override: the experimental project template (personal/local).",
                ),
                candidate(
                    "candidate:bootstrap-default",
                    SourceLayer::Default,
                    ValueScope::PersonalLocal,
                    true,
                    "Default: the built-in project template (personal/local).",
                ),
            ],
            sources_distinct: true,
            precedence_explained: true,
            effective: effective(
                SourceLayer::UserOverride,
                ValueScope::PersonalLocal,
                "Effective: your override wins over the default.",
            ),
            policy_lock: no_lock(),
            fallback: no_fallback(),
        },
        integrity: clean_integrity(),
        verification: verified("proof:project-bootstrap:0001"),
        renderings: renderings(
            "field:project-bootstrap:0001",
            ParameterClaim::LabsNotClaimed,
            &[CS::InspectorPanel, CS::HelpInline],
            false,
        ),
    };

    vec![provider, admin, request, package, settings, import, labs]
}
