//! Canonical local draft-state, autosave-journal, and recover-draft truth for
//! mutation-capable forms, config editors, and commit sheets.
//!
//! Where [the structured-input / staged-review
//! model](crate::m5_structured_input_and_staged_review) freezes the *per-surface*
//! honesty claim of a whole form, [the field/control-row
//! model](crate::m5_field_control_rows) freezes the *per-row* primitive a form is
//! built from, and [the form-validation
//! model](crate::m5_form_validation_and_blocked_submit) freezes how field validity
//! rolls up into a blocked-submit reason, this model freezes what happens *across
//! an interruption*: how a form's edits are autosaved to a **local draft journal**,
//! how the surface keeps **draft-versus-applied** state explicit, and how a
//! **recover-draft** action restores work after a crash, restart, reconnect, or
//! missing-dependency condition — without ever implying that local draft state was
//! written to a remote target, provider, or protected file.
//!
//! Each [`DraftJournalRecord`] binds a mutation-capable surface to:
//!
//! * its **autosave journal** — the [`DraftPersistence`] tier the draft actually
//!   lives in (in-memory, local journal, durable local checkpoint, committed
//!   locally, or committed to a remote target), the [`AutosaveStatus`] indicator,
//!   and the [`AutosaveClaimScope`] the indicator *claims* (local-only, remote
//!   synced, or nothing), so the indicator can never claim a draft reached a
//!   remote/provider target when only local state was saved;
//! * its **draft-versus-applied state** — an explicit [`DraftAppliedState`] label,
//!   whether draft and applied state are visibly distinct, the unsaved/applied
//!   field counts, and whether an applied state names its target, so a local draft
//!   never reads as applied and an applied write always names where it went;
//! * its **recovery semantics** — whether a recover-draft action is available when
//!   a journal exists ([`RecoveryAvailability`]), the [`InterruptionKind`] it
//!   survived, whether recovery preserves unrelated workspace/profile state,
//!   whether the restore surface can enumerate the affected forms/sheets, and the
//!   guarantee that recovering a draft never implies a remote write; and
//! * its **submit gate** — submit cannot proceed from an ambiguous draft/applied
//!   state, and the commit action names its scope rather than a generic Continue.
//!
//! Each record re-derives a [`DraftClaim`] ([`DraftJournalRecord::narrow`]) so a
//! surface can never read wider than its evidence: a surface whose autosave
//! indicator claims a remote/applied target while only local draft state was
//! saved, whose draft and applied state are not distinguished, whose local draft
//! reads as applied, whose applied state does not name its target, whose
//! recover-draft action implies a remote write or deletes unrelated state, that
//! loses the recover action while a journal exists, that cannot enumerate the
//! surfaces a crash affected, that lets a submit proceed from ambiguous state,
//! that lets an imported/restore review read as a local submit, or that renders
//! wider than its effective claim floors to [`DraftClaim::Blocked`] and falls back
//! to an explicit blocked state that names the reason. A labelled, recoverable gap
//! (an unlabeled autosave indicator, an autosave write still in flight, unsaved
//! in-memory edits, a stale/superseded backing source, a stale/missing proof)
//! holds a first-party surface at [`DraftClaim::Narrowed`]; an imported/restore
//! review sits at [`DraftClaim::ReviewOverlay`] and never claims a local submit;
//! and a Labs/unadvertised surface makes no public claim.
//!
//! [`M5DraftStateSetPacket::validate`] confirms the matrix is well-formed and
//! honest: header/identity/redaction/freshness are present, every form lane,
//! persistence tier, recovery availability, interruption kind, autosave claim
//! scope, and consumer surface is represented, overlay surfaces name their
//! provenance, no rendering surface overclaims, a floored surface keeps a
//! fallback, at least one surface demonstrates the auto-narrowing rule, and no raw
//! credential/body material crosses the export. Downstream settings, marketplace,
//! request, support, admin, import, and project surfaces ingest this packet rather
//! than minting per-feature draft/autosave/recovery semantics.
//!
//! No credential bodies, secret values, raw provider payloads, absolute paths, or
//! URLs ever cross this boundary; the packet carries only typed class tokens,
//! counts, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/ux/m5-draft-state-and-autosave.schema.json`](../../../../schemas/ux/m5-draft-state-and-autosave.schema.json).
//! The contract doc is
//! [`docs/ux/m5-draft-state-and-autosave.md`](../../../../docs/ux/m5-draft-state-and-autosave.md).
//! The canonical support export is
//! [`artifacts/ux/m5-draft-state-and-autosave/support_export.json`](../../../../artifacts/ux/m5-draft-state-and-autosave/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/ux/m5-draft-state-and-autosave/`](../../../../fixtures/ux/m5-draft-state-and-autosave/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DraftStateSetPacket`].
pub const M5_DRAFT_STATE_RECORD_KIND: &str = "m5_draft_state_set_packet";

/// Schema version for the draft-state set.
pub const M5_DRAFT_STATE_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_DRAFT_STATE_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical draft-state set packet.
pub const M5_DRAFT_STATE_PACKET_ID: &str = "m5-draft-state:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_DRAFT_STATE_SCHEMA_REF: &str = "schemas/ux/m5-draft-state-and-autosave.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DRAFT_STATE_DOC_REF: &str = "docs/ux/m5-draft-state-and-autosave.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_DRAFT_STATE_SUPPORT_EXPORT_REF: &str =
    "artifacts/ux/m5-draft-state-and-autosave/support_export.json";

/// Repo-relative path of the generated report.
pub const M5_DRAFT_STATE_REPORT_REF: &str = "artifacts/ux/m5-draft-state-and-autosave/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_DRAFT_STATE_FIXTURE_DIR: &str = "fixtures/ux/m5-draft-state-and-autosave";

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

/// The product lane that owns a draft surface.
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

/// How a surface and its backing values originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormOrigin {
    /// First-party local authoring.
    LocalAuthoring,
    /// First-party authoring against a remote target.
    RemoteTarget,
    /// A provider-backed surface.
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

/// The tier a draft's edits are actually persisted to — the heart of the
/// draft-versus-applied truth. A draft tier is recoverable but not committed; an
/// applied tier has reached its durable target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftPersistence {
    /// Held only in memory; nothing has been journaled yet.
    UnsavedInMemory,
    /// Written to a local autosave journal: recoverable, never the committed
    /// target, and never remote.
    LocalJournal,
    /// A durable local checkpoint that survives restart but is still a draft.
    LocalDurableCheckpoint,
    /// Committed to the durable *local* target (applied locally, never remote).
    CommittedLocal,
    /// Committed/applied to a remote or provider target.
    CommittedRemote,
}

impl DraftPersistence {
    /// Every persistence tier, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UnsavedInMemory,
        Self::LocalJournal,
        Self::LocalDurableCheckpoint,
        Self::CommittedLocal,
        Self::CommittedRemote,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsavedInMemory => "unsaved_in_memory",
            Self::LocalJournal => "local_journal",
            Self::LocalDurableCheckpoint => "local_durable_checkpoint",
            Self::CommittedLocal => "committed_local",
            Self::CommittedRemote => "committed_remote",
        }
    }

    /// Whether this tier is a *draft* (recoverable, not yet committed to a target).
    pub const fn is_draft_tier(self) -> bool {
        matches!(
            self,
            Self::UnsavedInMemory | Self::LocalJournal | Self::LocalDurableCheckpoint
        )
    }

    /// Whether the data has never left the local device (so an indicator may not
    /// claim a remote/synced target).
    pub const fn is_local_only(self) -> bool {
        !matches!(self, Self::CommittedRemote)
    }
}

/// The autosave indicator state surfaced on a draft surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutosaveStatus {
    /// No unsaved changes; nothing to autosave.
    Idle,
    /// An autosave write is in flight.
    Saving,
    /// The draft has been autosaved to its tier.
    Saved,
    /// The last autosave attempt failed.
    Failed,
    /// Autosave is intentionally unavailable on this surface.
    Disabled,
}

impl AutosaveStatus {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Saving => "saving",
            Self::Saved => "saved",
            Self::Failed => "failed",
            Self::Disabled => "disabled",
        }
    }
}

/// What the autosave indicator *claims* about where the draft went. The indicator
/// can never claim a remote/synced target when only local state was saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutosaveClaimScope {
    /// The indicator claims a local-only draft ("saved on this device").
    ClaimsLocalOnly,
    /// The indicator claims a remote/synced target ("saved" / "synced").
    ClaimsRemoteSynced,
    /// The indicator makes no claim.
    ClaimsNone,
}

impl AutosaveClaimScope {
    /// Every claim scope, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ClaimsLocalOnly,
        Self::ClaimsRemoteSynced,
        Self::ClaimsNone,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsLocalOnly => "claims_local_only",
            Self::ClaimsRemoteSynced => "claims_remote_synced",
            Self::ClaimsNone => "claims_none",
        }
    }
}

/// The explicit draft-versus-applied label a surface shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftAppliedState {
    /// A local draft; nothing has been applied.
    DraftOnly,
    /// Some values are applied and some remain draft.
    PartiallyApplied,
    /// Fully applied to the named target.
    Applied,
    /// Draft and applied state are not distinguished (a contract violation).
    NotDistinguished,
}

impl DraftAppliedState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DraftOnly => "draft_only",
            Self::PartiallyApplied => "partially_applied",
            Self::Applied => "applied",
            Self::NotDistinguished => "not_distinguished",
        }
    }

    /// Whether this label asserts that some state has been applied to a target.
    pub const fn asserts_applied(self) -> bool {
        matches!(self, Self::PartiallyApplied | Self::Applied)
    }
}

/// Whether a recover-draft action is available for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAvailability {
    /// A journal exists and a recover-draft action is offered.
    Recoverable,
    /// The draft has already been recovered.
    Recovered,
    /// There is nothing to recover (the surface is clean or applied).
    NoJournal,
}

impl RecoveryAvailability {
    /// Every recovery availability, in declaration order.
    pub const ALL: [Self; 3] = [Self::Recoverable, Self::Recovered, Self::NoJournal];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recoverable => "recoverable",
            Self::Recovered => "recovered",
            Self::NoJournal => "no_journal",
        }
    }
}

/// The interruption class a draft is resilient to or has recovered from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterruptionKind {
    /// No interruption.
    None,
    /// A crash interrupted the session.
    Crash,
    /// A normal restart interrupted the session.
    Restart,
    /// A reconnect (network/provider) interrupted the session.
    Reconnect,
    /// A missing dependency blocked the session.
    MissingDependency,
}

impl InterruptionKind {
    /// Every interruption kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::Crash,
        Self::Restart,
        Self::Reconnect,
        Self::MissingDependency,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Crash => "crash",
            Self::Restart => "restart",
            Self::Reconnect => "reconnect",
            Self::MissingDependency => "missing_dependency",
        }
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

/// The presentation a floored surface drops its submit control to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedSubmitFallback {
    /// The submit action shows the blocked reason in place.
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

/// A consumer surface that renders a draft-state record.
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

/// The effective claim a draft surface renders. A higher rank asserts more
/// authority, so a narrowed or floored surface must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftClaim {
    /// The draft/autosave/recovery contract is broken: the autosave indicator
    /// overclaims a remote target, draft and applied state are ambiguous, a local
    /// draft reads as applied, recovery implies a remote write or deletes unrelated
    /// state, the recover action is lost while a journal exists, an imported review
    /// reads as a local submit, or a rendering overclaims. It must fall back to an
    /// explicit blocked state that names the reason rather than a clean submit.
    #[serde(rename = "draft_blocked")]
    Blocked,
    /// A review of imported/migrated/restored state: attributable and reopenable
    /// but never reads as a local submit.
    #[serde(rename = "draft_review_overlay")]
    ReviewOverlay,
    /// A first-party surface held below certified by a labelled, recoverable gap
    /// (an unlabeled autosave indicator, an in-flight save, unsaved in-memory
    /// edits, a stale source, a stale proof).
    #[serde(rename = "draft_narrowed")]
    Narrowed,
    /// Full draft-versus-applied-honest, autosave-truthful, recoverable contract.
    #[serde(rename = "draft_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "draft_labs_not_claimed")]
    LabsNotClaimed,
}

impl DraftClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "draft_blocked",
            Self::ReviewOverlay => "draft_review_overlay",
            Self::Narrowed => "draft_narrowed",
            Self::Certified => "draft_certified",
            Self::LabsNotClaimed => "draft_labs_not_claimed",
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
    /// claim. A rendering surface must never render wider than the surface's
    /// effective claim; the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: DraftClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a draft surface fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftNarrowingReason {
    /// The autosave indicator claims a remote/synced target while only local draft
    /// state was saved.
    AutosaveOverclaimsRemote,
    /// Draft and applied state are not distinguished.
    DraftAppliedAmbiguous,
    /// A local draft reads as applied.
    LocalDraftReadsAsApplied,
    /// Recovering a draft implies it was written to a remote target.
    RecoverImpliesRemoteWrite,
    /// Submit is reachable from an ambiguous draft/applied state.
    SubmitFromAmbiguousState,
    /// Recovery would delete unrelated workspace/profile state.
    RecoveryDeletesUnrelatedState,
    /// An applied state does not name its target.
    AppliedTargetUnnamed,
    /// A journal exists but the recover-draft action is gone.
    RecoverActionLost,
    /// A crash-recovery surface cannot enumerate the affected forms/sheets.
    AffectedSurfacesUnenumerable,
    /// An imported/restore review reads as a local submit.
    ImportedDraftReadsAsApplied,
    /// A rendering surface renders wider than the effective claim.
    RenderingOverclaims,
    /// The autosave journal backing is missing.
    JournalBackingMissing,
    /// The autosave indicator state is not surfaced.
    AutosaveStateUnlabeled,
    /// An autosave write is still in flight.
    AutosavePending,
    /// Unsaved in-memory edits are not yet journaled.
    DraftUnsavedPending,
    /// The freshness state is not surfaced.
    FreshnessUnlabeled,
    /// A superseded backing source is not marked.
    SupersededStateNotMarked,
    /// A first-party surface is stale.
    DraftStale,
    /// The verification proof is stale.
    VerificationProofStale,
    /// The verification proof is missing.
    VerificationProofMissing,
    /// The reopen-to-origin path is lost.
    ReopenPathLost,
}

impl DraftNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutosaveOverclaimsRemote => "autosave_overclaims_remote",
            Self::DraftAppliedAmbiguous => "draft_applied_ambiguous",
            Self::LocalDraftReadsAsApplied => "local_draft_reads_as_applied",
            Self::RecoverImpliesRemoteWrite => "recover_implies_remote_write",
            Self::SubmitFromAmbiguousState => "submit_from_ambiguous_state",
            Self::RecoveryDeletesUnrelatedState => "recovery_deletes_unrelated_state",
            Self::AppliedTargetUnnamed => "applied_target_unnamed",
            Self::RecoverActionLost => "recover_action_lost",
            Self::AffectedSurfacesUnenumerable => "affected_surfaces_unenumerable",
            Self::ImportedDraftReadsAsApplied => "imported_draft_reads_as_applied",
            Self::RenderingOverclaims => "rendering_overclaims",
            Self::JournalBackingMissing => "journal_backing_missing",
            Self::AutosaveStateUnlabeled => "autosave_state_unlabeled",
            Self::AutosavePending => "autosave_pending",
            Self::DraftUnsavedPending => "draft_unsaved_pending",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::SupersededStateNotMarked => "superseded_state_not_marked",
            Self::DraftStale => "draft_stale",
            Self::VerificationProofStale => "verification_proof_stale",
            Self::VerificationProofMissing => "verification_proof_missing",
            Self::ReopenPathLost => "reopen_path_lost",
        }
    }

    /// Deterministic ordering index (mirrors the validator's reason order).
    pub const fn order_index(self) -> u8 {
        match self {
            Self::AutosaveOverclaimsRemote => 0,
            Self::DraftAppliedAmbiguous => 1,
            Self::LocalDraftReadsAsApplied => 2,
            Self::RecoverImpliesRemoteWrite => 3,
            Self::SubmitFromAmbiguousState => 4,
            Self::RecoveryDeletesUnrelatedState => 5,
            Self::AppliedTargetUnnamed => 6,
            Self::RecoverActionLost => 7,
            Self::AffectedSurfacesUnenumerable => 8,
            Self::ImportedDraftReadsAsApplied => 9,
            Self::RenderingOverclaims => 10,
            Self::JournalBackingMissing => 11,
            Self::AutosaveStateUnlabeled => 12,
            Self::AutosavePending => 13,
            Self::DraftUnsavedPending => 14,
            Self::FreshnessUnlabeled => 15,
            Self::SupersededStateNotMarked => 16,
            Self::DraftStale => 17,
            Self::VerificationProofStale => 18,
            Self::VerificationProofMissing => 19,
            Self::ReopenPathLost => 20,
        }
    }

    /// Whether this reason breaks the contract outright (floors the surface to
    /// [`DraftClaim::Blocked`]) rather than merely aging it out.
    pub const fn is_floor(self) -> bool {
        self.order_index() <= Self::JournalBackingMissing.order_index()
    }

    /// A reviewer-facing, non-generic description of the reason.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::AutosaveOverclaimsRemote => {
                "the autosave indicator claims a remote target while only a local draft was saved"
            }
            Self::DraftAppliedAmbiguous => "draft and applied state are not distinguished",
            Self::LocalDraftReadsAsApplied => "a local draft reads as applied",
            Self::RecoverImpliesRemoteWrite => {
                "recovering a draft implies it was written to a remote target"
            }
            Self::SubmitFromAmbiguousState => {
                "submit is reachable from an ambiguous draft/applied state"
            }
            Self::RecoveryDeletesUnrelatedState => {
                "recovery would delete unrelated workspace/profile state"
            }
            Self::AppliedTargetUnnamed => "an applied state does not name its target",
            Self::RecoverActionLost => "a journal exists but the recover-draft action is gone",
            Self::AffectedSurfacesUnenumerable => {
                "a crash-recovery surface cannot enumerate the affected forms/sheets"
            }
            Self::ImportedDraftReadsAsApplied => {
                "an imported/restore review reads as a local submit"
            }
            Self::RenderingOverclaims => {
                "a rendering surface renders wider than the effective claim"
            }
            Self::JournalBackingMissing => "the autosave journal backing is missing",
            Self::AutosaveStateUnlabeled => "the autosave indicator state is not surfaced",
            Self::AutosavePending => "an autosave write is still in flight",
            Self::DraftUnsavedPending => "unsaved in-memory edits are not yet journaled",
            Self::FreshnessUnlabeled => "the backing freshness state is not surfaced",
            Self::SupersededStateNotMarked => "a superseded backing source is not marked",
            Self::DraftStale => "the backing source is stale",
            Self::VerificationProofStale => "the verification proof is stale",
            Self::VerificationProofMissing => "the verification proof is missing",
            Self::ReopenPathLost => "the reopen-to-origin path is lost",
        }
    }
}

fn order_reasons(mut reasons: Vec<DraftNarrowingReason>) -> Vec<DraftNarrowingReason> {
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

/// Stable identifiers binding a draft-state record to its origin. Absent refs
/// serialize as `null` so the schema's required keys stay present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormLineage {
    /// Form-session/context ref (required).
    pub session_ref: String,
    /// The surface's own stable canonical ref (required for a real surface).
    pub canonical_surface_ref: Option<String>,
    /// Backlink to the structured-input surface this draft belongs to.
    pub structured_input_ref: Option<String>,
    /// Opaque ref to the autosave journal backing this surface.
    pub journal_ref: Option<String>,
    /// Provider ref (required for provider-backed/imported overlay surfaces).
    pub provider_ref: Option<String>,
    /// Imported/source-artifact ref backing the surface.
    pub source_artifact_ref: Option<String>,
    /// Reopen backlink ref.
    pub reopen_backlink_ref: Option<String>,
}

/// The autosave journal for a surface: where the draft actually lives and what the
/// indicator claims about it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutosaveJournal {
    /// The tier the draft is actually persisted to.
    pub persistence_tier: DraftPersistence,
    /// The autosave indicator state.
    pub autosave_status: AutosaveStatus,
    /// What the autosave indicator claims about where the draft went.
    pub autosave_claim_scope: AutosaveClaimScope,
    /// Number of entries in the autosave journal.
    pub journal_entry_count: u64,
    /// The journal is local-only and never written to a remote target.
    pub local_only: bool,
    /// The autosave indicator state is surfaced on the surface.
    pub indicator_labeled: bool,
    /// Opaque ref to the last autosave journal entry, or `null` when none exists.
    pub last_autosave_ref: Option<String>,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The explicit draft-versus-applied truth for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftStateBlock {
    /// The explicit draft-versus-applied label.
    pub draft_applied_state: DraftAppliedState,
    /// Draft and applied state are visibly distinguished.
    pub draft_distinct_from_applied: bool,
    /// Count of unsaved (draft) changes.
    pub unsaved_change_count: u64,
    /// Count of fields already applied to the target.
    pub applied_field_count: u64,
    /// Count of fields still in draft.
    pub draft_field_count: u64,
    /// When applied, the applied state names its target (local/remote/provider).
    pub applied_target_named: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The recover-draft semantics for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryBlock {
    /// Whether a recover-draft action is available.
    pub availability: RecoveryAvailability,
    /// The interruption class this draft survived or recovered from.
    pub interruption_kind: InterruptionKind,
    /// A recover-draft action is offered when a journal exists.
    pub recover_action_present: bool,
    /// Recovery preserves unrelated workspace/profile state.
    pub recover_preserves_unrelated_state: bool,
    /// The restore surface can enumerate the affected forms/sheets.
    pub enumerates_affected_surfaces: bool,
    /// Recovering a draft implies it was written to a remote target (must be
    /// false).
    pub recover_implies_remote_write: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The submit gate for a draft surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitGate {
    /// Whether submit is currently reachable.
    pub submit_allowed: bool,
    /// Draft and applied state are disambiguated before submit.
    pub draft_applied_disambiguated_before_submit: bool,
    /// The commit action names the scope/effect rather than a generic Continue.
    pub commit_action_is_specific: bool,
    /// Reviewer-facing label summary.
    pub label_summary: String,
}

/// The headline draft-state invariants every record re-derives rather than
/// trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftStateIntegrity {
    /// Draft and applied state stay distinct.
    pub draft_applied_distinct: bool,
    /// The autosave indicator scope matches the actual persistence tier.
    pub autosave_scope_truthful: bool,
    /// A local draft never claims a remote write.
    pub local_draft_not_remote: bool,
    /// A recover-draft action is available whenever a journal exists.
    pub recovery_available_when_journal: bool,
    /// Recovery preserves unrelated workspace/profile state.
    pub recovery_preserves_unrelated: bool,
    /// The crash-recovery surface can enumerate the affected forms/sheets.
    pub affected_surfaces_enumerable: bool,
    /// An applied state names its target.
    pub applied_target_disclosed: bool,
    /// The freshness state is visible.
    pub freshness_state_visible: bool,
    /// A superseded backing source stays marked.
    pub superseded_state_marked: bool,
    /// Origin lineage / reopen is revealable on demand on every surface.
    pub reopen_visible_on_demand: bool,
}

/// Verification-proof currency for a surface (distinct from backing freshness).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormVerification {
    /// Currency of the verification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the surface.
    pub proof_ref: Option<String>,
}

/// One consumer surface that renders a draft-state record, with the claim it
/// shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftStateRendering {
    /// The consumer surface.
    pub surface: ConsumerSurface,
    /// The claim this surface renders.
    pub rendered_claim: DraftClaim,
    /// Whether draft/scope provenance is revealable here.
    pub provenance_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical surface this view re-renders.
    pub source_surface_ref: String,
}

// --------------------------------------------------------------------------- //
// Record + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) mutation-capable surface, with its autosave journal,
/// draft-versus-applied state, and recover-draft semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftJournalRecord {
    /// Stable surface id.
    pub surface_id: String,
    /// The product lane.
    pub lane: FormLane,
    /// How the surface/values originated.
    pub origin: FormOrigin,
    /// Reviewer-facing label summary.
    pub label_summary: String,
    /// Whether the surface is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// Declared backing freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared submit-control fallback when floored.
    pub declared_blocked_fallback: BlockedSubmitFallback,
    /// Stable origin-lineage block.
    pub lineage: FormLineage,
    /// The autosave journal.
    pub journal: AutosaveJournal,
    /// The draft-versus-applied state.
    pub draft_state: DraftStateBlock,
    /// The recover-draft semantics.
    pub recovery: RecoveryBlock,
    /// The submit gate.
    pub submit_gate: SubmitGate,
    /// Headline invariant block.
    pub integrity: DraftStateIntegrity,
    /// Verification-proof block.
    pub verification: FormVerification,
    /// Consumer surfaces that render this record.
    pub renderings: Vec<DraftStateRendering>,
}

/// The re-derived draft decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftDecision {
    /// The headline claim the surface is eligible to make.
    pub claimed_claim: DraftClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_claim: DraftClaim,
    /// Ordered, de-duplicated reasons the surface fails to hold its headline.
    pub active_narrowing_reasons: Vec<DraftNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl DraftDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<DraftNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this record would overclaim.
    pub fn surface_overclaims(&self, rendered: DraftClaim) -> bool {
        self.effective_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(claimed: DraftClaim, reasons: &[DraftNarrowingReason]) -> DraftClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        DraftClaim::Blocked
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, DraftClaim::ReviewOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we
        // can no longer certify even the read-only review, so it floors.
        DraftClaim::Blocked
    } else {
        DraftClaim::Narrowed
    }
}

impl DraftJournalRecord {
    /// Whether this surface is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this surface is an inherently read-only review overlay.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin.is_overlay()
    }

    /// The headline claim this surface is eligible to make.
    pub fn claimed_claim(&self) -> DraftClaim {
        if self.is_labs() {
            DraftClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            DraftClaim::ReviewOverlay
        } else {
            DraftClaim::Certified
        }
    }

    /// Reasons that hold independently of how the consumer surfaces render — the
    /// intrinsic autosave/draft/recovery gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<DraftNarrowingReason> {
        use DraftNarrowingReason as R;
        let j = &self.journal;
        let ds = &self.draft_state;
        let rec = &self.recovery;
        let gate = &self.submit_gate;
        let integ = &self.integrity;
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<R> = Vec::new();

        // The autosave indicator can never claim a remote/synced target while only
        // local draft state was saved.
        if matches!(
            j.autosave_claim_scope,
            AutosaveClaimScope::ClaimsRemoteSynced
        ) && j.persistence_tier.is_local_only()
        {
            reasons.push(R::AutosaveOverclaimsRemote);
        }
        if !integ.autosave_scope_truthful {
            reasons.push(R::AutosaveOverclaimsRemote);
        }

        // Draft and applied state must be distinguished.
        if matches!(ds.draft_applied_state, DraftAppliedState::NotDistinguished)
            || !ds.draft_distinct_from_applied
            || !integ.draft_applied_distinct
        {
            reasons.push(R::DraftAppliedAmbiguous);
        }

        // A draft-tier persistence can never read as applied.
        if j.persistence_tier.is_draft_tier()
            && matches!(ds.draft_applied_state, DraftAppliedState::Applied)
        {
            reasons.push(R::LocalDraftReadsAsApplied);
        }
        if !integ.local_draft_not_remote {
            reasons.push(R::LocalDraftReadsAsApplied);
        }

        // An applied state must name its target.
        if ds.draft_applied_state.asserts_applied()
            && (!ds.applied_target_named || !integ.applied_target_disclosed)
        {
            reasons.push(R::AppliedTargetUnnamed);
        }

        // Recovering a draft must never imply a remote write.
        if rec.recover_implies_remote_write {
            reasons.push(R::RecoverImpliesRemoteWrite);
        }

        // Submit cannot proceed from an ambiguous draft/applied state.
        if gate.submit_allowed
            && (matches!(ds.draft_applied_state, DraftAppliedState::NotDistinguished)
                || !ds.draft_distinct_from_applied
                || !gate.draft_applied_disambiguated_before_submit)
        {
            reasons.push(R::SubmitFromAmbiguousState);
        }

        // Recovery preserves unrelated workspace/profile state.
        if !rec.recover_preserves_unrelated_state || !integ.recovery_preserves_unrelated {
            reasons.push(R::RecoveryDeletesUnrelatedState);
        }

        // A journal that exists must keep its recover-draft action.
        if matches!(rec.availability, RecoveryAvailability::Recoverable)
            && (!rec.recover_action_present || !integ.recovery_available_when_journal)
        {
            reasons.push(R::RecoverActionLost);
        }

        // Crash-recovery surfaces must enumerate the affected forms/sheets.
        if !rec.enumerates_affected_surfaces || !integ.affected_surfaces_enumerable {
            reasons.push(R::AffectedSurfacesUnenumerable);
        }

        // Imported/restore overlay must stay a read-only review, never a submit.
        if overlay
            && (gate.submit_allowed
                || matches!(ds.draft_applied_state, DraftAppliedState::Applied)
                || self.renderings.iter().any(|r| !r.read_only))
        {
            reasons.push(R::ImportedDraftReadsAsApplied);
        }

        // Backing freshness → journal backing.
        if !integ.freshness_state_visible {
            reasons.push(R::FreshnessUnlabeled);
        }
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::JournalBackingMissing),
            FreshnessState::SupersededByNewerSource if !integ.superseded_state_marked => {
                reasons.push(R::SupersededStateNotMarked);
            }
            FreshnessState::StaleExpired if !overlay => reasons.push(R::DraftStale),
            _ => {}
        }

        // Autosave indicator labelling and pending states.
        if !j.indicator_labeled {
            reasons.push(R::AutosaveStateUnlabeled);
        }
        if matches!(j.autosave_status, AutosaveStatus::Saving) {
            reasons.push(R::AutosavePending);
        }
        if matches!(j.persistence_tier, DraftPersistence::UnsavedInMemory)
            && ds.unsaved_change_count > 0
        {
            reasons.push(R::DraftUnsavedPending);
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
    fn reasons(&self, stale_window: bool) -> Vec<DraftNarrowingReason> {
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic_effective = derive_effective(self.claimed_claim(), &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic_effective.overclaims_as(r.rendered_claim))
        {
            reasons.push(DraftNarrowingReason::RenderingOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive this surface's claim decision.
    pub fn narrow(&self, stale_window: bool) -> DraftDecision {
        let claimed = self.claimed_claim();
        if matches!(claimed, DraftClaim::LabsNotClaimed) {
            return DraftDecision {
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
        DraftDecision {
            claimed_claim: claimed,
            effective_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// Whether a floored surface still keeps a submit-control fallback that names
    /// the reason rather than a misleading clean submit.
    pub fn floored_keeps_fallback(&self, effective: DraftClaim) -> bool {
        if !matches!(effective, DraftClaim::Blocked) {
            return true;
        }
        self.declared_blocked_fallback.keeps_reason()
    }

    /// Whether any rendering overclaims relative to `effective`.
    pub fn surface_overclaims(&self, effective: DraftClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// A reviewer-facing label for a narrowed/floored surface, or `None` if the
    /// surface holds its claim.
    pub fn narrowed_label(&self, decision: &DraftDecision) -> Option<String> {
        let trigger = decision.downgrade_trigger()?;
        Some(match decision.effective_claim {
            DraftClaim::Blocked => format!(
                "Floored to draft_blocked below the {} claim: {}; falls back to an explicit blocked state that names the reason.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            DraftClaim::Narrowed => format!(
                "Held at draft_narrowed below the {} claim: {}; the draft stays recoverable and reopenable until re-verified.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            _ => return None,
        })
    }

    /// Append per-record structural violations (schema-shape level).
    fn structural_violations(&self, out: &mut Vec<M5DraftStateViolation>) {
        use M5DraftStateViolation as V;
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

/// Constructor input for [`M5DraftStateSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftStateSetInput {
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
    pub surfaces: Vec<DraftJournalRecord>,
}

/// Export-safe M5 draft-state / autosave / recover-draft set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftStateSetPacket {
    /// Record kind; must equal [`M5_DRAFT_STATE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DRAFT_STATE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_DRAFT_STATE_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-surface rows.
    pub surfaces: Vec<DraftJournalRecord>,
}

/// The distribution of effective draft claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftClaimDistribution {
    /// Surfaces effective at [`DraftClaim::Certified`].
    pub certified: usize,
    /// Surfaces effective at [`DraftClaim::Narrowed`].
    pub narrowed: usize,
    /// Surfaces effective at [`DraftClaim::ReviewOverlay`].
    pub overlay: usize,
    /// Surfaces effective at [`DraftClaim::Blocked`].
    pub blocked: usize,
    /// Surfaces effective at [`DraftClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5DraftStateSetPacket {
    /// Builds a draft-state set packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5DraftStateSetInput) -> Self {
        Self {
            record_kind: M5_DRAFT_STATE_RECORD_KIND.to_owned(),
            schema_version: M5_DRAFT_STATE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_DRAFT_STATE_TAXONOMY_VERSION,
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
    pub fn decisions(&self) -> Vec<(String, DraftDecision)> {
        let stale_window = self.stale_window();
        self.surfaces
            .iter()
            .map(|s| (s.surface_id.clone(), s.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective draft claims.
    pub fn claim_distribution(&self) -> DraftClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = DraftClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            blocked: 0,
            labs: 0,
        };
        for s in &self.surfaces {
            match s.narrow(stale_window).effective_claim {
                DraftClaim::Certified => dist.certified += 1,
                DraftClaim::Narrowed => dist.narrowed += 1,
                DraftClaim::ReviewOverlay => dist.overlay += 1,
                DraftClaim::Blocked => dist.blocked += 1,
                DraftClaim::LabsNotClaimed => dist.labs += 1,
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

    /// Product lanes represented by some surface.
    pub fn represented_lanes(&self) -> BTreeSet<FormLane> {
        self.surfaces.iter().map(|s| s.lane).collect()
    }

    /// Persistence tiers represented by some surface.
    pub fn represented_persistence_tiers(&self) -> BTreeSet<DraftPersistence> {
        self.surfaces
            .iter()
            .map(|s| s.journal.persistence_tier)
            .collect()
    }

    /// Recovery availabilities represented by some surface.
    pub fn represented_recovery_availabilities(&self) -> BTreeSet<RecoveryAvailability> {
        self.surfaces
            .iter()
            .map(|s| s.recovery.availability)
            .collect()
    }

    /// Interruption kinds represented by some surface.
    pub fn represented_interruption_kinds(&self) -> BTreeSet<InterruptionKind> {
        self.surfaces
            .iter()
            .map(|s| s.recovery.interruption_kind)
            .collect()
    }

    /// Autosave claim scopes represented by some surface.
    pub fn represented_autosave_claim_scopes(&self) -> BTreeSet<AutosaveClaimScope> {
        self.surfaces
            .iter()
            .map(|s| s.journal.autosave_claim_scope)
            .collect()
    }

    /// Consumer surfaces represented by some rendering.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<ConsumerSurface> {
        self.surfaces
            .iter()
            .flat_map(|s| s.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Validate the draft-state invariants.
    pub fn validate(&self) -> Vec<M5DraftStateViolation> {
        use M5DraftStateViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_DRAFT_STATE_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_DRAFT_STATE_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_DRAFT_STATE_TAXONOMY_VERSION {
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
        if DraftPersistence::ALL
            .iter()
            .any(|t| !self.represented_persistence_tiers().contains(t))
        {
            violations.push(V::PersistenceTierMissing);
        }
        if RecoveryAvailability::ALL
            .iter()
            .any(|a| !self.represented_recovery_availabilities().contains(a))
        {
            violations.push(V::RecoveryAvailabilityMissing);
        }
        if InterruptionKind::ALL
            .iter()
            .any(|k| !self.represented_interruption_kinds().contains(k))
        {
            violations.push(V::InterruptionKindMissing);
        }
        if AutosaveClaimScope::ALL
            .iter()
            .any(|c| !self.represented_autosave_claim_scopes().contains(c))
        {
            violations.push(V::AutosaveClaimScopeMissing);
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
            &serde_json::to_value(self).expect("draft-state packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        let mut out: Vec<M5DraftStateViolation> = Vec::new();
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
        serde_json::to_string_pretty(self).expect("draft-state packet serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_report(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Draft State, Autosave Journals, and Recover-Draft Semantics\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Surfaces: {}\n", self.surfaces.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} review overlay, {} blocked, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.blocked, dist.labs
        ));

        out.push_str("| Surface | Lane | Origin | Persistence | Draft/Applied | Recovery | Claimed | Effective |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for s in &self.surfaces {
            let decision = s.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
                s.surface_id,
                s.lane.as_str(),
                s.origin.as_str(),
                s.journal.persistence_tier.as_str(),
                s.draft_state.draft_applied_state.as_str(),
                s.recovery.availability.as_str(),
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
pub enum M5DraftStateArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5DraftStateViolation>),
}

impl fmt::Display for M5DraftStateArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(f, "support export parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for M5DraftStateArtifactError {}

/// A draft-state packet validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DraftStateViolation {
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
    /// A product lane is unrepresented.
    FormLaneMissing,
    /// A persistence tier is unrepresented.
    PersistenceTierMissing,
    /// A recovery availability is unrepresented.
    RecoveryAvailabilityMissing,
    /// An interruption kind is unrepresented.
    InterruptionKindMissing,
    /// An autosave claim scope is unrepresented.
    AutosaveClaimScopeMissing,
    /// A consumer surface is unrepresented.
    ConsumerSurfaceMissing,
    /// A surface lacks a required identity field.
    SurfaceMissingIdentity,
    /// An overlay surface names no provider/source-artifact ref.
    OverlayMissingProvenanceRef,
    /// A surface has no renderings.
    SurfaceMissingRendering,
    /// A rendering names no source surface ref.
    RenderingMissingSourceRef,
    /// A narrowed surface lacks a non-generic label or a downgrade trigger.
    NarrowedSurfaceMissingLabelOrTrigger,
    /// A floored surface loses its submit-control fallback.
    FlooredSurfaceLosesFallback,
    /// A rendering surface renders wider than the effective claim.
    RenderingSurfaceOverclaims,
    /// No surface demonstrates the auto-narrowing rule.
    DowngradedSurfaceCaseMissing,
    /// Raw boundary material crossed the export.
    RawBoundaryMaterialInExport,
}

impl M5DraftStateViolation {
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
            Self::PersistenceTierMissing => "persistence_tier_missing",
            Self::RecoveryAvailabilityMissing => "recovery_availability_missing",
            Self::InterruptionKindMissing => "interruption_kind_missing",
            Self::AutosaveClaimScopeMissing => "autosave_claim_scope_missing",
            Self::ConsumerSurfaceMissing => "consumer_surface_missing",
            Self::SurfaceMissingIdentity => "surface_missing_identity",
            Self::OverlayMissingProvenanceRef => "overlay_missing_provenance_ref",
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
/// draft-state / autosave / recover-draft matrix instead of minting per-feature
/// semantics.
///
/// # Errors
///
/// Returns [`M5DraftStateArtifactError`] when the artifact cannot be parsed or
/// fails validation.
pub fn current_m5_draft_state_set() -> Result<M5DraftStateSetPacket, M5DraftStateArtifactError> {
    let packet: M5DraftStateSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ux/m5-draft-state-and-autosave/support_export.json"
    )))
    .map_err(M5DraftStateArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DraftStateArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded draft-state set: the in-crate source of truth the
/// checked-in support export and report are regenerated from.
pub fn seeded_m5_draft_state_set() -> M5DraftStateSetPacket {
    M5DraftStateSetPacket::new(M5DraftStateSetInput {
        packet_id: M5_DRAFT_STATE_PACKET_ID.to_owned(),
        label:
            "M5 draft state — local autosave journals, explicit draft-versus-applied truth, and recover-draft semantics across mutation-capable surfaces"
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
    claim: DraftClaim,
    surfaces: &[ConsumerSurface],
    read_only: bool,
) -> Vec<DraftStateRendering> {
    surfaces
        .iter()
        .map(|&surface| DraftStateRendering {
            surface,
            rendered_claim: claim,
            provenance_visible: true,
            read_only,
            source_surface_ref: source_ref.to_owned(),
        })
        .collect()
}

/// A clean first-party integrity block.
fn clean_integrity() -> DraftStateIntegrity {
    DraftStateIntegrity {
        draft_applied_distinct: true,
        autosave_scope_truthful: true,
        local_draft_not_remote: true,
        recovery_available_when_journal: true,
        recovery_preserves_unrelated: true,
        affected_surfaces_enumerable: true,
        applied_target_disclosed: true,
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

fn gate(submit_allowed: bool, label: &str) -> SubmitGate {
    SubmitGate {
        submit_allowed,
        draft_applied_disambiguated_before_submit: true,
        commit_action_is_specific: true,
        label_summary: label.to_owned(),
    }
}

/// The canonical surfaces: one per product lane, covering every persistence tier,
/// recovery availability, interruption kind, autosave claim scope, and consumer
/// surface, plus a narrowed first-party surface, a review overlay, and a Labs
/// surface.
fn seed_surfaces() -> Vec<DraftJournalRecord> {
    use ConsumerSurface as CS;

    // Provider source-registration form: edits are autosaved to a local journal
    // and recoverable after a reconnect; the indicator honestly claims a local
    // draft and never that the credentials reached the provider.
    let provider = DraftJournalRecord {
        surface_id: "form:provider-connection:0001".to_owned(),
        lane: FormLane::Provider,
        origin: FormOrigin::ProviderBacked,
        label_summary: "Provider connection form: edits autosave to a local journal and recover after a reconnect; the indicator claims a local draft, never a provider write.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::ShowsReasonOnSubmit,
        lineage: FormLineage {
            session_ref: "form-session.provider.connection".to_owned(),
            canonical_surface_ref: Some("surface.provider.connection.0001".to_owned()),
            structured_input_ref: Some("form.provider.credentials.0001".to_owned()),
            journal_ref: Some("journal.provider.connection.0001".to_owned()),
            provider_ref: Some("provider.connection.primary".to_owned()),
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.provider.connection.0001".to_owned()),
        },
        journal: AutosaveJournal {
            persistence_tier: DraftPersistence::LocalJournal,
            autosave_status: AutosaveStatus::Saved,
            autosave_claim_scope: AutosaveClaimScope::ClaimsLocalOnly,
            journal_entry_count: 4,
            local_only: true,
            indicator_labeled: true,
            last_autosave_ref: Some("journal.provider.connection.0001#4".to_owned()),
            label_summary: "Draft saved on this device; not sent to the provider.".to_owned(),
        },
        draft_state: DraftStateBlock {
            draft_applied_state: DraftAppliedState::DraftOnly,
            draft_distinct_from_applied: true,
            unsaved_change_count: 0,
            applied_field_count: 0,
            draft_field_count: 3,
            applied_target_named: true,
            label_summary: "Local draft; nothing connected to the provider yet.".to_owned(),
        },
        recovery: RecoveryBlock {
            availability: RecoveryAvailability::Recoverable,
            interruption_kind: InterruptionKind::Reconnect,
            recover_action_present: true,
            recover_preserves_unrelated_state: true,
            enumerates_affected_surfaces: true,
            recover_implies_remote_write: false,
            label_summary: "Recover the local draft after the reconnect; recovery touches only this form.".to_owned(),
        },
        submit_gate: gate(true, "Connect provider (applies the draft to the provider)."),
        integrity: clean_integrity(),
        verification: verified("proof.provider.connection.0001"),
        renderings: renderings("surface.provider.connection.0001", DraftClaim::Certified, &[CS::FormView, CS::DiagnosticsPanel, CS::SupportExport], false),
    };

    // Settings editor: applied to the durable local target; the autosave indicator
    // claims a local-only save and names the local target it was applied to.
    let settings = DraftJournalRecord {
        surface_id: "form:settings-config:0001".to_owned(),
        lane: FormLane::Settings,
        origin: FormOrigin::LocalAuthoring,
        label_summary: "Settings editor: changes are applied to the local settings store and labelled applied-locally; the indicator never claims a remote sync.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.settings.config".to_owned(),
            canonical_surface_ref: Some("surface.settings.config.0001".to_owned()),
            structured_input_ref: Some("form.settings.config.0001".to_owned()),
            journal_ref: Some("journal.settings.config.0001".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.settings.config.0001".to_owned()),
        },
        journal: AutosaveJournal {
            persistence_tier: DraftPersistence::CommittedLocal,
            autosave_status: AutosaveStatus::Saved,
            autosave_claim_scope: AutosaveClaimScope::ClaimsLocalOnly,
            journal_entry_count: 7,
            local_only: true,
            indicator_labeled: true,
            last_autosave_ref: Some("journal.settings.config.0001#7".to_owned()),
            label_summary: "Applied to local settings on this device.".to_owned(),
        },
        draft_state: DraftStateBlock {
            draft_applied_state: DraftAppliedState::Applied,
            draft_distinct_from_applied: true,
            unsaved_change_count: 0,
            applied_field_count: 5,
            draft_field_count: 0,
            applied_target_named: true,
            label_summary: "Applied to the local settings store; no remote target.".to_owned(),
        },
        recovery: RecoveryBlock {
            availability: RecoveryAvailability::NoJournal,
            interruption_kind: InterruptionKind::None,
            recover_action_present: false,
            recover_preserves_unrelated_state: true,
            enumerates_affected_surfaces: true,
            recover_implies_remote_write: false,
            label_summary: "Nothing to recover; the draft was applied locally and the journal cleared.".to_owned(),
        },
        submit_gate: gate(true, "Apply settings to this device."),
        integrity: clean_integrity(),
        verification: verified("proof.settings.config.0001"),
        renderings: renderings("surface.settings.config.0001", DraftClaim::Certified, &[CS::FormView, CS::HelpInline], false),
    };

    // Project bootstrap wizard: only in-memory so far, with no unsaved changes yet;
    // the indicator makes no claim and there is nothing to recover.
    let projects = DraftJournalRecord {
        surface_id: "wizard:project-bootstrap:0001".to_owned(),
        lane: FormLane::Projects,
        origin: FormOrigin::LocalAuthoring,
        label_summary: "Project bootstrap wizard: freshly opened with no unsaved edits; the autosave indicator makes no claim and there is no journal to recover.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::ShowsReasonOnSubmit,
        lineage: FormLineage {
            session_ref: "form-session.projects.bootstrap".to_owned(),
            canonical_surface_ref: Some("surface.projects.bootstrap.0001".to_owned()),
            structured_input_ref: Some("wizard.projects.bootstrap.0001".to_owned()),
            journal_ref: None,
            provider_ref: None,
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.projects.bootstrap.0001".to_owned()),
        },
        journal: AutosaveJournal {
            persistence_tier: DraftPersistence::UnsavedInMemory,
            autosave_status: AutosaveStatus::Idle,
            autosave_claim_scope: AutosaveClaimScope::ClaimsNone,
            journal_entry_count: 0,
            local_only: true,
            indicator_labeled: true,
            last_autosave_ref: None,
            label_summary: "No unsaved changes yet.".to_owned(),
        },
        draft_state: DraftStateBlock {
            draft_applied_state: DraftAppliedState::DraftOnly,
            draft_distinct_from_applied: true,
            unsaved_change_count: 0,
            applied_field_count: 0,
            draft_field_count: 0,
            applied_target_named: true,
            label_summary: "Nothing entered; no project created yet.".to_owned(),
        },
        recovery: RecoveryBlock {
            availability: RecoveryAvailability::NoJournal,
            interruption_kind: InterruptionKind::None,
            recover_action_present: false,
            recover_preserves_unrelated_state: true,
            enumerates_affected_surfaces: true,
            recover_implies_remote_write: false,
            label_summary: "No draft journal yet; nothing to recover.".to_owned(),
        },
        submit_gate: gate(true, "Create project (writes the project locally)."),
        integrity: clean_integrity(),
        verification: verified("proof.projects.bootstrap.0001"),
        renderings: renderings("surface.projects.bootstrap.0001", DraftClaim::Certified, &[CS::WizardStep, CS::SupportExport], false),
    };

    // Package install sheet: a durable local checkpoint with some dependencies
    // already installed (partially applied) and the local target named; recoverable
    // after a crash.
    let package = DraftJournalRecord {
        surface_id: "sheet:package-install:0001".to_owned(),
        lane: FormLane::Package,
        origin: FormOrigin::LocalAuthoring,
        label_summary: "Package install sheet: a durable local checkpoint records which dependencies are applied versus drafted; recoverable after a crash without touching other state.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::ShowsReasonOnSubmit,
        lineage: FormLineage {
            session_ref: "form-session.package.install".to_owned(),
            canonical_surface_ref: Some("surface.package.install.0001".to_owned()),
            structured_input_ref: Some("sheet.package.install.0001".to_owned()),
            journal_ref: Some("journal.package.install.0001".to_owned()),
            provider_ref: None,
            source_artifact_ref: Some("artifact.lockfile.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.package.install.0001".to_owned()),
        },
        journal: AutosaveJournal {
            persistence_tier: DraftPersistence::LocalDurableCheckpoint,
            autosave_status: AutosaveStatus::Saved,
            autosave_claim_scope: AutosaveClaimScope::ClaimsLocalOnly,
            journal_entry_count: 12,
            local_only: true,
            indicator_labeled: true,
            last_autosave_ref: Some("journal.package.install.0001#12".to_owned()),
            label_summary: "Checkpoint saved locally; survives restart.".to_owned(),
        },
        draft_state: DraftStateBlock {
            draft_applied_state: DraftAppliedState::PartiallyApplied,
            draft_distinct_from_applied: true,
            unsaved_change_count: 2,
            applied_field_count: 3,
            draft_field_count: 2,
            applied_target_named: true,
            label_summary: "3 dependencies applied to the local environment; 2 still drafted.".to_owned(),
        },
        recovery: RecoveryBlock {
            availability: RecoveryAvailability::Recoverable,
            interruption_kind: InterruptionKind::Crash,
            recover_action_present: true,
            recover_preserves_unrelated_state: true,
            enumerates_affected_surfaces: true,
            recover_implies_remote_write: false,
            label_summary: "Recover the install checkpoint after a crash; already-applied dependencies are not reverted.".to_owned(),
        },
        submit_gate: gate(true, "Install remaining packages into the local environment."),
        integrity: clean_integrity(),
        verification: verified("proof.package.install.0001"),
        renderings: renderings("surface.package.install.0001", DraftClaim::Certified, &[CS::ReviewSheet, CS::AiEvidence], false),
    };

    // Admin policy editor: committed to a remote policy service and labelled
    // applied-remotely with the target named; the indicator honestly claims a
    // remote sync. Recoverable after a restart is moot (committed), so no journal.
    let admin = DraftJournalRecord {
        surface_id: "sheet:admin-policy:0001".to_owned(),
        lane: FormLane::Admin,
        origin: FormOrigin::RemoteTarget,
        label_summary: "Admin policy editor: the policy was committed to the remote policy service and labelled applied-remotely with the target named; the indicator honestly claims a remote sync.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.admin.policy".to_owned(),
            canonical_surface_ref: Some("surface.admin.policy.0001".to_owned()),
            structured_input_ref: Some("sheet.admin.policy.0001".to_owned()),
            journal_ref: Some("journal.admin.policy.0001".to_owned()),
            provider_ref: Some("provider.policy.service".to_owned()),
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.admin.policy.0001".to_owned()),
        },
        journal: AutosaveJournal {
            persistence_tier: DraftPersistence::CommittedRemote,
            autosave_status: AutosaveStatus::Saved,
            autosave_claim_scope: AutosaveClaimScope::ClaimsRemoteSynced,
            journal_entry_count: 9,
            local_only: false,
            indicator_labeled: true,
            last_autosave_ref: Some("journal.admin.policy.0001#9".to_owned()),
            label_summary: "Applied and synced to the policy service.".to_owned(),
        },
        draft_state: DraftStateBlock {
            draft_applied_state: DraftAppliedState::Applied,
            draft_distinct_from_applied: true,
            unsaved_change_count: 0,
            applied_field_count: 6,
            draft_field_count: 0,
            applied_target_named: true,
            label_summary: "Applied to the policy service; target named in the apply summary.".to_owned(),
        },
        recovery: RecoveryBlock {
            availability: RecoveryAvailability::NoJournal,
            interruption_kind: InterruptionKind::Restart,
            recover_action_present: false,
            recover_preserves_unrelated_state: true,
            enumerates_affected_surfaces: true,
            recover_implies_remote_write: false,
            label_summary: "Applied remotely; the local draft journal was cleared after commit.".to_owned(),
        },
        submit_gate: gate(false, "Policy applied; reopen to edit a new draft."),
        integrity: clean_integrity(),
        verification: verified("proof.admin.policy.0001"),
        renderings: renderings("surface.admin.policy.0001", DraftClaim::Certified, &[CS::ReviewSheet, CS::SupportExport], false),
    };

    // Request-workspace composer: a local draft with an autosave write in flight;
    // the in-flight save holds the surface at narrowed until it lands.
    let request = DraftJournalRecord {
        surface_id: "dialog:request-run:0001".to_owned(),
        lane: FormLane::Request,
        origin: FormOrigin::RemoteTarget,
        label_summary: "Request-workspace composer: the request body is a local draft with an autosave write in flight; the surface narrows until the save lands.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::CachedSnapshot,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.request.run".to_owned(),
            canonical_surface_ref: Some("surface.request.run.0001".to_owned()),
            structured_input_ref: Some("dialog.request.run.0001".to_owned()),
            journal_ref: Some("journal.request.run.0001".to_owned()),
            provider_ref: Some("provider.remote.workspace".to_owned()),
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.request.run.0001".to_owned()),
        },
        journal: AutosaveJournal {
            persistence_tier: DraftPersistence::LocalJournal,
            autosave_status: AutosaveStatus::Saving,
            autosave_claim_scope: AutosaveClaimScope::ClaimsLocalOnly,
            journal_entry_count: 5,
            local_only: true,
            indicator_labeled: true,
            last_autosave_ref: Some("journal.request.run.0001#5".to_owned()),
            label_summary: "Saving the draft locally…".to_owned(),
        },
        draft_state: DraftStateBlock {
            draft_applied_state: DraftAppliedState::DraftOnly,
            draft_distinct_from_applied: true,
            unsaved_change_count: 1,
            applied_field_count: 0,
            draft_field_count: 4,
            applied_target_named: true,
            label_summary: "Local request draft; nothing has been run against the workspace.".to_owned(),
        },
        recovery: RecoveryBlock {
            availability: RecoveryAvailability::Recoverable,
            interruption_kind: InterruptionKind::Crash,
            recover_action_present: true,
            recover_preserves_unrelated_state: true,
            enumerates_affected_surfaces: true,
            recover_implies_remote_write: false,
            label_summary: "Recover the request draft after a crash; recovery does not re-run the request.".to_owned(),
        },
        submit_gate: gate(false, "Run request — waiting for the autosave to land first."),
        integrity: clean_integrity(),
        verification: verified("proof.request.run.0001"),
        renderings: renderings("surface.request.run.0001", DraftClaim::Narrowed, &[CS::FormView, CS::AiEvidence], false),
    };

    // Migration restore review: an imported/restore overlay. Read-only; the
    // restored snapshot is a local durable checkpoint already recovered from a
    // backup after a missing-dependency interruption, and never reads as a submit.
    let import = DraftJournalRecord {
        surface_id: "dialog:migration-restore:0001".to_owned(),
        lane: FormLane::Import,
        origin: FormOrigin::ImportedOrRestore,
        label_summary: "Migration-center restore review: a read-only review of a recovered backup snapshot after a missing-dependency interruption; it enumerates the affected surfaces and never reads as a local submit.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_freshness_state: FreshnessState::CachedSnapshot,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.import.restore".to_owned(),
            canonical_surface_ref: Some("surface.import.restore.0001".to_owned()),
            structured_input_ref: Some("dialog.import.restore.0001".to_owned()),
            journal_ref: Some("journal.import.restore.0001".to_owned()),
            provider_ref: Some("provider.migration.center".to_owned()),
            source_artifact_ref: Some("artifact.import.backup.0001".to_owned()),
            reopen_backlink_ref: Some("reopen.import.restore.0001".to_owned()),
        },
        journal: AutosaveJournal {
            persistence_tier: DraftPersistence::LocalDurableCheckpoint,
            autosave_status: AutosaveStatus::Saved,
            autosave_claim_scope: AutosaveClaimScope::ClaimsLocalOnly,
            journal_entry_count: 1,
            local_only: true,
            indicator_labeled: true,
            last_autosave_ref: Some("journal.import.restore.0001#1".to_owned()),
            label_summary: "Restored snapshot held locally (read-only review).".to_owned(),
        },
        draft_state: DraftStateBlock {
            draft_applied_state: DraftAppliedState::DraftOnly,
            draft_distinct_from_applied: true,
            unsaved_change_count: 0,
            applied_field_count: 0,
            draft_field_count: 4,
            applied_target_named: true,
            label_summary: "Read-only review: nothing restored locally until you apply it in the restore step.".to_owned(),
        },
        recovery: RecoveryBlock {
            availability: RecoveryAvailability::Recovered,
            interruption_kind: InterruptionKind::MissingDependency,
            recover_action_present: false,
            recover_preserves_unrelated_state: true,
            enumerates_affected_surfaces: true,
            recover_implies_remote_write: false,
            label_summary: "Recovered from the backup snapshot; the restore enumerates each affected form before applying.".to_owned(),
        },
        submit_gate: gate(false, "Review restore (read-only); apply happens in the local restore step."),
        integrity: clean_integrity(),
        verification: FormVerification {
            proof_currency: ProofCurrency::CachedWithinWindow,
            proof_ref: Some("proof.import.restore.0001".to_owned()),
        },
        renderings: renderings("surface.import.restore.0001", DraftClaim::ReviewOverlay, &[CS::ReviewSheet, CS::DiagnosticsPanel], true),
    };

    // Labs onboarding wizard: makes no public claim.
    let labs = DraftJournalRecord {
        surface_id: "wizard:labs-onboarding:0001".to_owned(),
        lane: FormLane::Projects,
        origin: FormOrigin::LocalAuthoring,
        label_summary:
            "Experimental onboarding wizard behind a Labs flag; makes no public draft-state claim."
                .to_owned(),
        claim_posture: ClaimPosture::LabsUnadvertised,
        declared_freshness_state: FreshnessState::Live,
        declared_blocked_fallback: BlockedSubmitFallback::DisabledWithHint,
        lineage: FormLineage {
            session_ref: "form-session.labs.onboarding".to_owned(),
            canonical_surface_ref: Some("surface.labs.onboarding.0001".to_owned()),
            structured_input_ref: Some("wizard.labs.onboarding.0001".to_owned()),
            journal_ref: None,
            provider_ref: None,
            source_artifact_ref: None,
            reopen_backlink_ref: Some("reopen.labs.onboarding.0001".to_owned()),
        },
        journal: AutosaveJournal {
            persistence_tier: DraftPersistence::UnsavedInMemory,
            autosave_status: AutosaveStatus::Disabled,
            autosave_claim_scope: AutosaveClaimScope::ClaimsNone,
            journal_entry_count: 0,
            local_only: true,
            indicator_labeled: true,
            last_autosave_ref: None,
            label_summary: "Experimental; autosave is off behind the Labs flag.".to_owned(),
        },
        draft_state: DraftStateBlock {
            draft_applied_state: DraftAppliedState::DraftOnly,
            draft_distinct_from_applied: true,
            unsaved_change_count: 0,
            applied_field_count: 0,
            draft_field_count: 1,
            applied_target_named: true,
            label_summary: "Experimental; no public draft-state claim.".to_owned(),
        },
        recovery: RecoveryBlock {
            availability: RecoveryAvailability::NoJournal,
            interruption_kind: InterruptionKind::None,
            recover_action_present: false,
            recover_preserves_unrelated_state: true,
            enumerates_affected_surfaces: true,
            recover_implies_remote_write: false,
            label_summary: "Experimental; no recovery claim.".to_owned(),
        },
        submit_gate: gate(true, "Try experiment."),
        integrity: clean_integrity(),
        verification: FormVerification {
            proof_currency: ProofCurrency::MissingProof,
            proof_ref: None,
        },
        renderings: renderings(
            "surface.labs.onboarding.0001",
            DraftClaim::LabsNotClaimed,
            &[CS::WizardStep],
            false,
        ),
    };

    vec![
        provider, settings, projects, package, admin, request, import, labs,
    ]
}
