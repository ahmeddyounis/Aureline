//! Canonical keyboard, assistive-tech, reduced-motion, and interruption-safe
//! continuity truth for M5 dense multi-step forms, inline validation links, and
//! batch-review sheets.
//!
//! Where [`crate::m5_field_control_rows`] freezes a single field's label and
//! validation anchor, [`crate::m5_form_validation_and_blocked_submit`] freezes how a
//! form rolls those anchors up and explains a blocked submit,
//! [`crate::m5_draft_state_and_autosave`] freezes the autosave journal a draft
//! recovers from, [`crate::m5_staged_review_sheets`] freezes the commit sheet a
//! mutation stops at, and [`crate::m5_parameter_source_and_precedence`] freezes the
//! source inspector behind a value, this module freezes the **accessibility and
//! interruption-safety contract** those same dense surfaces must hold so the shared
//! structured-input model stays fully usable under keyboard-only, assistive-tech,
//! reduced-motion, reconnect, and restart conditions. One contract is reused across
//! the provider, admin, request, package, settings, import, and project lanes instead
//! of each domain re-inventing focus order, screen-reader labelling, or recovery
//! semantics — so an extension or provider-owned surface cannot quietly regress
//! accessibility or interruption behavior.
//!
//! Each [`SurfaceRecord`] binds, for one mutation-capable form or review sheet:
//!
//! * its **keyboard reachability** — a [`KeyboardAccess`] recording a deterministic
//!   focus order, roving focus over dense collections, every interactive control
//!   reachable, batch-review action keyboard parity, and an escapable focus trap;
//! * its **assistive-tech reachability** — an [`AssistiveTech`] recording permanent
//!   screen-reader labels, inline validation links announced to AT (parity with the
//!   visual links), blocked-submit reasons surfaced through a live region, and the
//!   current step position announced;
//! * its **reduced-motion behavior** — a [`ReducedMotion`] binding the shared
//!   [`ReducedMotionSubstitutionClass`] design-system posture, whether state is
//!   conveyed without depending on animation, and whether step progress carries a
//!   non-motion marker; and
//! * its **interruption-safe continuity** — a [`ContinuitySnapshot`] recording whether
//!   a recovery journal backs the surface and whether the current step, blocked
//!   fields, and draft state are preserved so an interrupted flow resumes on the
//!   correct step across reconnect, restart, a missing dependency, and crash recovery.
//!
//! Each record re-derives a [`ContinuityClaim`] ([`SurfaceRecord::narrow`]) so a
//! surface can never read wider than its evidence: a surface that drops a keyboard
//! path, an undefined focus order, an unreachable batch action, a missing
//! screen-reader label, an inline validation link the AT cannot reach, an
//! un-announced blocked submit, a state carried only by motion, a lost step / blocked
//! field / draft on interruption, a mutable imported review, a lost recovery path, or
//! a missing continuity journal floors to [`ContinuityClaim::Unsafe`] and falls back
//! to an explicit blocked-submit state with a keyboard recovery path. A labelled,
//! recoverable gap (an un-announced step position, an unlabelled focus-trap escape or
//! reduced-motion substitution, a partial journal, an aged proof) holds a first-party
//! surface at [`ContinuityClaim::Narrowed`] while staying usable, an import/migration
//! review sits at [`ContinuityClaim::ReviewOverlay`] and never reads as an apply, and
//! a Labs/unadvertised surface makes no public claim.
//!
//! [`M5AccessibilityContinuitySetPacket::validate`] confirms the matrix is well-formed
//! and honest: header/identity/redaction/freshness are present; every surface kind,
//! lane, origin, reduced-motion substitution class, interruption path, and consumer
//! surface is represented; overlay surfaces name their provenance; no rendering
//! overclaims; a floored surface keeps a recovery fallback; at least one surface
//! demonstrates the auto-narrowing rule; and no raw credential/body material crosses
//! the export. Downstream settings, marketplace, request, support, admin, import, and
//! project surfaces — plus the accessibility-audit and CLI/headless paths and the
//! docs/help references — ingest this packet rather than minting per-feature
//! accessibility or recovery semantics.
//!
//! No credential bodies, secret values, raw provider payloads, absolute paths, or
//! URLs ever cross this boundary; the packet carries only typed class tokens,
//! booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/ux/m5-accessibility-and-continuity.schema.json`](../../../../schemas/ux/m5-accessibility-and-continuity.schema.json).
//! The contract doc is
//! [`docs/ux/m5-accessibility-and-continuity.md`](../../../../docs/ux/m5-accessibility-and-continuity.md).
//! The canonical support export is
//! [`artifacts/ux/m5-accessibility-and-continuity/support_export.json`](../../../../artifacts/ux/m5-accessibility-and-continuity/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/ux/m5-accessibility-and-continuity/`](../../../../fixtures/ux/m5-accessibility-and-continuity/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::motion::ReducedMotionSubstitutionClass;

/// Stable record-kind tag carried by [`M5AccessibilityContinuitySetPacket`].
pub const M5_ACCESSIBILITY_CONTINUITY_RECORD_KIND: &str = "m5_accessibility_continuity_set_packet";

/// Schema version for the accessibility-and-continuity set.
pub const M5_ACCESSIBILITY_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_ACCESSIBILITY_CONTINUITY_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical accessibility-and-continuity set packet.
pub const M5_ACCESSIBILITY_CONTINUITY_PACKET_ID: &str =
    "m5-accessibility-and-continuity:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_ACCESSIBILITY_CONTINUITY_SCHEMA_REF: &str =
    "schemas/ux/m5-accessibility-and-continuity.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ACCESSIBILITY_CONTINUITY_DOC_REF: &str = "docs/ux/m5-accessibility-and-continuity.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_ACCESSIBILITY_CONTINUITY_SUPPORT_EXPORT_REF: &str =
    "artifacts/ux/m5-accessibility-and-continuity/support_export.json";

/// Repo-relative path of the generated report.
pub const M5_ACCESSIBILITY_CONTINUITY_REPORT_REF: &str =
    "artifacts/ux/m5-accessibility-and-continuity/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_ACCESSIBILITY_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/ux/m5-accessibility-and-continuity";

/// Allowed packet redaction-class tokens.
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

/// Deterministic seed timestamp for the canonical packet and report.
const SEED_AS_OF: &str = "2026-06-21T00:00:00Z";

/// Every reduced-motion substitution class that must be represented in the set
/// (the shared design-system posture is not `Ord`, so coverage is checked by value).
const ALL_REDUCED_MOTION_CLASSES: [ReducedMotionSubstitutionClass; 5] = [
    ReducedMotionSubstitutionClass::CrossfadeOnly,
    ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
    ReducedMotionSubstitutionClass::SuppressEntirely,
    ReducedMotionSubstitutionClass::CollapseToInstant,
    ReducedMotionSubstitutionClass::NonMotionStateMarker,
];

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

/// The structural kind of a mutation-capable surface this contract governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// A dense multi-step form / wizard.
    MultiStepForm,
    /// An inline field-validation link group.
    InlineValidationLinks,
    /// A selection-bar batch-review sheet.
    BatchReviewSheet,
    /// A staged commit/review sheet.
    StagedReviewSheet,
    /// A structured configuration editor.
    ConfigEditor,
}

impl SurfaceKind {
    /// Every surface kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MultiStepForm,
        Self::InlineValidationLinks,
        Self::BatchReviewSheet,
        Self::StagedReviewSheet,
        Self::ConfigEditor,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MultiStepForm => "multi_step_form",
            Self::InlineValidationLinks => "inline_validation_links",
            Self::BatchReviewSheet => "batch_review_sheet",
            Self::StagedReviewSheet => "staged_review_sheet",
            Self::ConfigEditor => "config_editor",
        }
    }

    /// Whether this kind exposes batch-review actions whose keyboard parity must hold.
    pub const fn has_batch_actions(self) -> bool {
        matches!(self, Self::BatchReviewSheet | Self::StagedReviewSheet)
    }

    /// Whether this kind carries inline field-validation links that the AT must reach.
    pub const fn has_validation_links(self) -> bool {
        matches!(
            self,
            Self::MultiStepForm
                | Self::InlineValidationLinks
                | Self::StagedReviewSheet
                | Self::ConfigEditor
        )
    }
}

/// The product lane a surface belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceLane {
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

impl SurfaceLane {
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

/// How the surface originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceOrigin {
    /// A first-party local form/sheet.
    LocalForm,
    /// A first-party form bound to a remote target.
    RemoteForm,
    /// A provider-backed form.
    ProviderForm,
    /// A review of imported/migrated values (an overlay).
    ImportedReview,
}

impl SurfaceOrigin {
    /// Every origin, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalForm,
        Self::RemoteForm,
        Self::ProviderForm,
        Self::ImportedReview,
    ];

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

/// The completeness of the recovery journal that backs a surface's continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JournalState {
    /// A complete, current recovery journal.
    Complete,
    /// A journal that exists but is partial (some step/field state is not captured).
    Partial,
    /// A journal that exists but is stale.
    Stale,
    /// No recovery journal backs the surface.
    Missing,
}

impl JournalState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }
}

/// Verification-proof currency for a surface (distinct from journal completeness).
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
    /// No proof anchors the surface.
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

/// What a recovery/reopen returns the user to after an interruption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryTarget {
    /// Recovery restores the surface and the exact step the user was on.
    SurfaceAndStep,
    /// Recovery restores the step position only.
    StepOnly,
    /// No reopen; a keyboard fallback to the originating entry point remains.
    NoneKeyboardFallback,
}

impl RecoveryTarget {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceAndStep => "surface_and_step",
            Self::StepOnly => "step_only",
            Self::NoneKeyboardFallback => "none_keyboard_fallback",
        }
    }
}

/// Whether the surface is publicly claimed or a Labs/unadvertised surface.
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

/// A consumer surface that re-renders an accessibility-and-continuity record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// The live, running form/sheet surface.
    LiveSurface,
    /// A review/commit sheet.
    ReviewSheet,
    /// The diagnostics panel.
    DiagnosticsPanel,
    /// A support-export bundle.
    SupportExport,
    /// An accessibility-audit surface.
    AccessibilityAudit,
    /// Inline help / docs.
    HelpInline,
    /// A CLI/headless surface.
    CliHeadless,
}

impl ConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LiveSurface,
        Self::ReviewSheet,
        Self::DiagnosticsPanel,
        Self::SupportExport,
        Self::AccessibilityAudit,
        Self::HelpInline,
        Self::CliHeadless,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveSurface => "live_surface",
            Self::ReviewSheet => "review_sheet",
            Self::DiagnosticsPanel => "diagnostics_panel",
            Self::SupportExport => "support_export",
            Self::AccessibilityAudit => "accessibility_audit",
            Self::HelpInline => "help_inline",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// An interruption path a surface's continuity must survive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterruptionPath {
    /// A transport reconnect.
    Reconnect,
    /// An application restart.
    Restart,
    /// A newly missing dependency / prerequisite.
    MissingDependency,
    /// A crash and recovery.
    CrashRecovery,
}

impl InterruptionPath {
    /// Every interruption path, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Reconnect,
        Self::Restart,
        Self::MissingDependency,
        Self::CrashRecovery,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reconnect => "reconnect",
            Self::Restart => "restart",
            Self::MissingDependency => "missing_dependency",
            Self::CrashRecovery => "crash_recovery",
        }
    }
}

// --------------------------------------------------------------------------- //
// Derived claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim an accessibility-and-continuity surface renders. A higher rank
/// asserts more authority, so a narrowed or floored surface must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityClaim {
    /// The accessibility/continuity contract is broken: the surface drops a keyboard
    /// path, an undefined focus order, an unreachable batch action, a missing
    /// screen-reader label, an un-announced validation link or blocked submit, a
    /// motion-only state, a lost step/blocked-field/draft on interruption, a mutable
    /// imported review, a lost recovery path, a missing continuity journal, or renders
    /// wider than its claim. It must fall back to an explicit blocked-submit state with
    /// a keyboard recovery path.
    #[serde(rename = "continuity_unsafe")]
    Unsafe,
    /// A read-only review of imported/migrated values: keyboard-complete and AT-reachable
    /// but never reads as an apply.
    #[serde(rename = "continuity_review_overlay")]
    ReviewOverlay,
    /// A first-party surface held below certified by a labelled, recoverable gap; the
    /// surface stays keyboard-complete and recoverable.
    #[serde(rename = "continuity_narrowed")]
    Narrowed,
    /// Full keyboard-complete, AT-reachable, reduced-motion-safe, interruption-safe
    /// accessibility-and-continuity contract.
    #[serde(rename = "continuity_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "continuity_labs_not_claimed")]
    LabsNotClaimed,
}

impl ContinuityClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsafe => "continuity_unsafe",
            Self::ReviewOverlay => "continuity_review_overlay",
            Self::Narrowed => "continuity_narrowed",
            Self::Certified => "continuity_certified",
            Self::LabsNotClaimed => "continuity_labs_not_claimed",
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
    /// A rendering surface must never render wider than the surface's effective claim;
    /// the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: ContinuityClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a surface fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityNarrowingReason {
    /// Not every interactive control is reachable by keyboard, or a focus trap cannot
    /// be escaped.
    KeyboardPathIncomplete,
    /// No deterministic focus order is defined for the dense surface.
    FocusOrderUndefined,
    /// A batch-review action has no keyboard parity with its pointer affordance.
    BatchActionsKeyboardUnreachable,
    /// A control is missing a permanent screen-reader label.
    ScreenReaderLabelsMissing,
    /// An inline validation link is not announced to assistive tech.
    ValidationLinksNotAnnounced,
    /// A blocked-submit reason is not surfaced through a live region.
    BlockedSubmitNotAnnounced,
    /// A state is conveyed only by motion and is lost under reduced motion.
    MotionOnlyState,
    /// An interrupted flow does not resume on the correct step.
    CurrentStepLost,
    /// Blocked-field context is lost across an interruption.
    BlockedFieldsLost,
    /// Draft-state continuity is lost across an interruption.
    DraftStateLost,
    /// An imported/migration review surface is mutable / reads as an apply.
    ImportedReviewMutable,
    /// The keyboard recovery path back to the surface is lost.
    RecoveryPathLost,
    /// A rendering surface renders wider than the effective claim.
    SurfaceOverclaims,
    /// The continuity recovery journal is missing.
    ContinuityJournalMissing,
    /// The current step position is not announced to assistive tech.
    StepPositionUnannounced,
    /// The focus-trap escape affordance is present but unlabelled.
    FocusTrapEscapeUnlabeled,
    /// The reduced-motion substitution is present but generic/unlabelled.
    ReducedMotionSubstitutionUnlabeled,
    /// The step-progress non-motion marker is present but unlabelled.
    ProgressMarkerUnlabeled,
    /// The recovery journal exists but is partial or stale.
    JournalPartial,
    /// The verification proof is stale.
    ContinuityProofStale,
    /// The verification proof is missing.
    ContinuityProofMissing,
}

impl ContinuityNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardPathIncomplete => "keyboard_path_incomplete",
            Self::FocusOrderUndefined => "focus_order_undefined",
            Self::BatchActionsKeyboardUnreachable => "batch_actions_keyboard_unreachable",
            Self::ScreenReaderLabelsMissing => "screen_reader_labels_missing",
            Self::ValidationLinksNotAnnounced => "validation_links_not_announced",
            Self::BlockedSubmitNotAnnounced => "blocked_submit_not_announced",
            Self::MotionOnlyState => "motion_only_state",
            Self::CurrentStepLost => "current_step_lost",
            Self::BlockedFieldsLost => "blocked_fields_lost",
            Self::DraftStateLost => "draft_state_lost",
            Self::ImportedReviewMutable => "imported_review_mutable",
            Self::RecoveryPathLost => "recovery_path_lost",
            Self::SurfaceOverclaims => "surface_overclaims",
            Self::ContinuityJournalMissing => "continuity_journal_missing",
            Self::StepPositionUnannounced => "step_position_unannounced",
            Self::FocusTrapEscapeUnlabeled => "focus_trap_escape_unlabeled",
            Self::ReducedMotionSubstitutionUnlabeled => "reduced_motion_substitution_unlabeled",
            Self::ProgressMarkerUnlabeled => "progress_marker_unlabeled",
            Self::JournalPartial => "journal_partial",
            Self::ContinuityProofStale => "continuity_proof_stale",
            Self::ContinuityProofMissing => "continuity_proof_missing",
        }
    }

    /// Deterministic ordering index (mirrors the validator's reason order).
    pub const fn order_index(self) -> u8 {
        match self {
            Self::KeyboardPathIncomplete => 0,
            Self::FocusOrderUndefined => 1,
            Self::BatchActionsKeyboardUnreachable => 2,
            Self::ScreenReaderLabelsMissing => 3,
            Self::ValidationLinksNotAnnounced => 4,
            Self::BlockedSubmitNotAnnounced => 5,
            Self::MotionOnlyState => 6,
            Self::CurrentStepLost => 7,
            Self::BlockedFieldsLost => 8,
            Self::DraftStateLost => 9,
            Self::ImportedReviewMutable => 10,
            Self::RecoveryPathLost => 11,
            Self::SurfaceOverclaims => 12,
            Self::ContinuityJournalMissing => 13,
            Self::StepPositionUnannounced => 14,
            Self::FocusTrapEscapeUnlabeled => 15,
            Self::ReducedMotionSubstitutionUnlabeled => 16,
            Self::ProgressMarkerUnlabeled => 17,
            Self::JournalPartial => 18,
            Self::ContinuityProofStale => 19,
            Self::ContinuityProofMissing => 20,
        }
    }

    /// Whether this reason breaks the contract outright (floors the surface to
    /// [`ContinuityClaim::Unsafe`]) rather than merely aging it out.
    pub const fn is_floor(self) -> bool {
        self.order_index() <= Self::ContinuityJournalMissing.order_index()
    }

    /// A reviewer-facing, non-generic description of the reason.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::KeyboardPathIncomplete => {
                "an interactive control is not keyboard-reachable or a focus trap cannot be escaped"
            }
            Self::FocusOrderUndefined => "no deterministic focus order is defined for the surface",
            Self::BatchActionsKeyboardUnreachable => {
                "a batch-review action has no keyboard parity with its pointer affordance"
            }
            Self::ScreenReaderLabelsMissing => {
                "a control is missing a permanent screen-reader label"
            }
            Self::ValidationLinksNotAnnounced => {
                "an inline validation link is not announced to assistive tech"
            }
            Self::BlockedSubmitNotAnnounced => {
                "a blocked-submit reason is not surfaced through a live region"
            }
            Self::MotionOnlyState => {
                "a state is conveyed only by motion and is lost under reduced motion"
            }
            Self::CurrentStepLost => "an interrupted flow does not resume on the correct step",
            Self::BlockedFieldsLost => "blocked-field context is lost across an interruption",
            Self::DraftStateLost => "draft-state continuity is lost across an interruption",
            Self::ImportedReviewMutable => {
                "an imported/migration review surface is mutable or reads as an apply"
            }
            Self::RecoveryPathLost => "the keyboard recovery path back to the surface is lost",
            Self::SurfaceOverclaims => "a rendering surface renders wider than the effective claim",
            Self::ContinuityJournalMissing => "the continuity recovery journal is missing",
            Self::StepPositionUnannounced => {
                "the current step position is not announced to assistive tech"
            }
            Self::FocusTrapEscapeUnlabeled => {
                "the focus-trap escape affordance is present but unlabelled"
            }
            Self::ReducedMotionSubstitutionUnlabeled => {
                "the reduced-motion substitution is generic or unlabelled"
            }
            Self::ProgressMarkerUnlabeled => {
                "the step-progress non-motion marker is present but unlabelled"
            }
            Self::JournalPartial => "the recovery journal exists but is partial or stale",
            Self::ContinuityProofStale => "the verification proof is stale",
            Self::ContinuityProofMissing => "the verification proof is missing",
        }
    }
}

fn order_reasons(mut reasons: Vec<ContinuityNarrowingReason>) -> Vec<ContinuityNarrowingReason> {
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

/// Stable origin-lineage block; refs carry opaque ids only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceLineage {
    /// The session ref.
    pub session_ref: String,
    /// The canonical surface this view re-renders, when distinct.
    pub canonical_surface_ref: Option<String>,
    /// The form ref this surface belongs to.
    pub form_ref: Option<String>,
    /// The provider ref, for provider-backed surfaces.
    pub provider_ref: Option<String>,
    /// The imported source-artifact ref, for review overlays.
    pub source_artifact_ref: Option<String>,
    /// The policy ref, for policy-governed surfaces.
    pub policy_ref: Option<String>,
    /// The recovery-journal ref, for journal-backed surfaces.
    pub journal_ref: Option<String>,
    /// The recovery-to-surface backlink ref.
    pub recovery_backlink_ref: Option<String>,
}

/// Keyboard reachability and focus management for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardAccess {
    /// A deterministic focus order is defined.
    pub focus_order_defined: bool,
    /// Every interactive control is reachable by keyboard.
    pub all_controls_reachable: bool,
    /// Roving focus management is used across dense collections.
    pub roving_tabindex: bool,
    /// Batch-review actions have keyboard parity with their pointer affordances.
    pub batch_actions_keyboard_parity: bool,
    /// A modal/sheet focus trap can be escaped by keyboard.
    pub focus_trap_escapable: bool,
    /// The focus-trap escape affordance is labelled.
    pub focus_trap_escape_labeled: bool,
    /// Reviewer-facing keyboard label.
    pub keyboard_label: String,
}

/// Assistive-tech reachability for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistiveTech {
    /// Permanent screen-reader labels are present on every control.
    pub screen_reader_labels_present: bool,
    /// Inline validation links are announced to assistive tech.
    pub validation_links_announced: bool,
    /// Blocked-submit reasons are surfaced through a live region.
    pub blocked_submit_live_region: bool,
    /// The current step position is announced to assistive tech.
    pub step_position_announced: bool,
    /// Reviewer-facing assistive-tech label.
    pub at_label: String,
}

/// Reduced-motion behavior for a surface, bound to the shared design-system posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedMotion {
    /// The shared reduced-motion substitution class for this surface's transitions.
    pub substitution_class: ReducedMotionSubstitutionClass,
    /// State is conveyed without depending on animation.
    pub state_conveyed_without_motion: bool,
    /// Step progress carries a non-motion marker.
    pub progress_non_motion_marker: bool,
    /// The substitution class is specifically labelled (not generic).
    pub substitution_labeled: bool,
    /// The progress non-motion marker is labelled.
    pub progress_marker_labeled: bool,
    /// Reviewer-facing reduced-motion label.
    pub reduced_motion_label: String,
}

/// The accessibility posture (keyboard, assistive tech, reduced motion) of a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityPosture {
    /// Keyboard reachability and focus management.
    pub keyboard: KeyboardAccess,
    /// Assistive-tech reachability.
    pub assistive_tech: AssistiveTech,
    /// Reduced-motion behavior.
    pub reduced_motion: ReducedMotion,
}

/// The interruption-safe continuity snapshot for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuitySnapshot {
    /// Whether a recovery journal backs the surface.
    pub journal_backed: bool,
    /// The completeness of that journal.
    pub journal_state: JournalState,
    /// The current step is preserved across an interruption.
    pub current_step_preserved: bool,
    /// Blocked-field context is preserved across an interruption.
    pub blocked_fields_preserved: bool,
    /// Draft state is preserved across an interruption.
    pub draft_state_preserved: bool,
    /// The flow resumes after a transport reconnect.
    pub resume_on_reconnect: bool,
    /// The flow resumes after an application restart.
    pub resume_on_restart: bool,
    /// The flow resumes after a newly missing dependency.
    pub resume_on_missing_dependency: bool,
    /// The flow resumes after a crash.
    pub resume_on_crash: bool,
    /// Reviewer-facing continuity label.
    pub continuity_label: String,
}

impl ContinuitySnapshot {
    /// The interruption paths this snapshot preserves.
    pub fn preserved_paths(&self) -> BTreeSet<InterruptionPath> {
        let mut set = BTreeSet::new();
        if self.resume_on_reconnect {
            set.insert(InterruptionPath::Reconnect);
        }
        if self.resume_on_restart {
            set.insert(InterruptionPath::Restart);
        }
        if self.resume_on_missing_dependency {
            set.insert(InterruptionPath::MissingDependency);
        }
        if self.resume_on_crash {
            set.insert(InterruptionPath::CrashRecovery);
        }
        set
    }
}

/// The headline accessibility/continuity invariants every surface re-derives rather
/// than trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceIntegrity {
    /// The surface is keyboard-complete.
    pub keyboard_complete: bool,
    /// The focus order is deterministic.
    pub focus_order_deterministic: bool,
    /// Batch-review actions are keyboard-reachable.
    pub batch_actions_keyboard_reachable: bool,
    /// The surface is screen-reader reachable.
    pub screen_reader_reachable: bool,
    /// Inline validation links are present in the assistive-tech tree.
    pub validation_links_in_at: bool,
    /// Blocked-submit reasons are present in a live region.
    pub blocked_submit_in_live_region: bool,
    /// State is conveyed without depending on motion.
    pub state_without_motion: bool,
    /// An interrupted flow resumes on the correct step.
    pub step_resumes_correctly: bool,
    /// Blocked-field context is retained across an interruption.
    pub blocked_fields_retained: bool,
    /// Draft state is retained across an interruption.
    pub draft_state_retained: bool,
    /// Imported/migration reviews stay read-only.
    pub imported_review_read_only: bool,
    /// The current step position is announced.
    pub step_position_announced: bool,
}

/// Verification-proof currency for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceVerification {
    /// Currency of the verification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the surface.
    pub proof_ref: Option<String>,
}

/// One consumer surface that renders a surface record, with the claim it shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRendering {
    /// The consumer surface.
    pub surface: ConsumerSurface,
    /// The claim this surface renders.
    pub rendered_claim: ContinuityClaim,
    /// Whether the keyboard recovery path is reachable here.
    pub recovery_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical surface this view re-renders.
    pub source_surface_ref: String,
}

// --------------------------------------------------------------------------- //
// Surface record + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) accessibility-and-continuity contract for an M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRecord {
    /// Stable surface id.
    pub surface_id: String,
    /// The structural kind of the surface.
    pub surface_kind: SurfaceKind,
    /// The product lane.
    pub lane: SurfaceLane,
    /// How the surface originated.
    pub origin: SurfaceOrigin,
    /// Reviewer-facing label summary.
    pub label_summary: String,
    /// Whether the surface is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// Declared recovery target after an interruption.
    pub declared_recovery_target: RecoveryTarget,
    /// Stable origin-lineage block.
    pub lineage: SurfaceLineage,
    /// The accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// The interruption-safe continuity snapshot.
    pub continuity: ContinuitySnapshot,
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
    pub claimed_claim: ContinuityClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_claim: ContinuityClaim,
    /// Ordered, de-duplicated reasons the surface fails to hold its headline.
    pub active_narrowing_reasons: Vec<ContinuityNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl SurfaceDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<ContinuityNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this record would overclaim.
    pub fn surface_overclaims(&self, rendered: ContinuityClaim) -> bool {
        self.effective_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(
    claimed: ContinuityClaim,
    reasons: &[ContinuityNarrowingReason],
) -> ContinuityClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        ContinuityClaim::Unsafe
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, ContinuityClaim::ReviewOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we can
        // no longer certify even the read-only review, so it floors.
        ContinuityClaim::Unsafe
    } else {
        ContinuityClaim::Narrowed
    }
}

impl SurfaceRecord {
    /// Whether this surface is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this surface is an inherently read-only review overlay.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin.is_overlay()
    }

    /// The headline claim this surface is eligible to make.
    pub fn claimed_claim(&self) -> ContinuityClaim {
        if self.is_labs() {
            ContinuityClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            ContinuityClaim::ReviewOverlay
        } else {
            ContinuityClaim::Certified
        }
    }

    /// Reasons that hold independently of how the consumer surfaces render — the
    /// intrinsic accessibility/continuity gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<ContinuityNarrowingReason> {
        use ContinuityNarrowingReason as R;
        let kb = &self.accessibility.keyboard;
        let at = &self.accessibility.assistive_tech;
        let rm = &self.accessibility.reduced_motion;
        let cont = &self.continuity;
        let integ = &self.integrity;
        let overlay = self.is_overlay_origin();
        let batch = self.surface_kind.has_batch_actions();
        let has_validation = self.surface_kind.has_validation_links();
        let mut reasons: Vec<R> = Vec::new();

        // Keyboard completeness: every control reachable and the focus trap escapable.
        if !kb.all_controls_reachable || !kb.focus_trap_escapable || !integ.keyboard_complete {
            reasons.push(R::KeyboardPathIncomplete);
        }

        // Deterministic focus order.
        if !kb.focus_order_defined || !integ.focus_order_deterministic {
            reasons.push(R::FocusOrderUndefined);
        }

        // Batch-review action keyboard parity (only where batch actions exist).
        if batch && (!kb.batch_actions_keyboard_parity || !integ.batch_actions_keyboard_reachable) {
            reasons.push(R::BatchActionsKeyboardUnreachable);
        }

        // Screen-reader labels.
        if !at.screen_reader_labels_present || !integ.screen_reader_reachable {
            reasons.push(R::ScreenReaderLabelsMissing);
        }

        // Inline validation links announced (only where validation links exist).
        if has_validation && (!at.validation_links_announced || !integ.validation_links_in_at) {
            reasons.push(R::ValidationLinksNotAnnounced);
        }

        // Blocked-submit live region (only for mutation-capable, non-overlay surfaces).
        if !overlay && (!at.blocked_submit_live_region || !integ.blocked_submit_in_live_region) {
            reasons.push(R::BlockedSubmitNotAnnounced);
        }

        // Motion-only state (the reduced-motion guardrail).
        if !rm.state_conveyed_without_motion || !integ.state_without_motion {
            reasons.push(R::MotionOnlyState);
        }

        // Interruption-safe continuity (only for mutation-capable, non-overlay surfaces;
        // an overlay is a read-only review with no draft to recover).
        if !overlay {
            if !cont.current_step_preserved || !integ.step_resumes_correctly {
                reasons.push(R::CurrentStepLost);
            }
            if !cont.blocked_fields_preserved || !integ.blocked_fields_retained {
                reasons.push(R::BlockedFieldsLost);
            }
            if !cont.draft_state_preserved || !integ.draft_state_retained {
                reasons.push(R::DraftStateLost);
            }
        }

        // Imported overlay read-only.
        if overlay && !integ.imported_review_read_only {
            reasons.push(R::ImportedReviewMutable);
        }

        // Keyboard recovery path.
        if self.renderings.iter().any(|r| !r.recovery_visible)
            || matches!(
                self.declared_recovery_target,
                RecoveryTarget::NoneKeyboardFallback
            )
        {
            reasons.push(R::RecoveryPathLost);
        }

        // Continuity journal backing (only for mutation-capable, non-overlay surfaces).
        if !overlay {
            match cont.journal_state {
                JournalState::Missing => reasons.push(R::ContinuityJournalMissing),
                JournalState::Partial | JournalState::Stale => reasons.push(R::JournalPartial),
                JournalState::Complete => {}
            }
        }

        // Step position announced (non-floor).
        if !at.step_position_announced || !integ.step_position_announced {
            reasons.push(R::StepPositionUnannounced);
        }

        // Focus-trap escape labelling (non-floor).
        if !kb.focus_trap_escape_labeled {
            reasons.push(R::FocusTrapEscapeUnlabeled);
        }

        // Reduced-motion substitution labelling (non-floor).
        if !rm.substitution_labeled {
            reasons.push(R::ReducedMotionSubstitutionUnlabeled);
        }

        // Progress non-motion marker labelling (non-floor; only when a marker is used).
        if rm.progress_non_motion_marker && !rm.progress_marker_labeled {
            reasons.push(R::ProgressMarkerUnlabeled);
        }

        // Verification proof.
        match self.verification.proof_currency {
            ProofCurrency::MissingProof => reasons.push(R::ContinuityProofMissing),
            ProofCurrency::StaleExpired | ProofCurrency::RequiresReview => {
                reasons.push(R::ContinuityProofStale);
            }
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow if stale_window => {
                reasons.push(R::ContinuityProofStale);
            }
            _ => {}
        }

        reasons
    }

    /// All active narrowing reasons, including the rendering-surface overclaim check,
    /// ordered and de-duplicated.
    fn reasons(&self, stale_window: bool) -> Vec<ContinuityNarrowingReason> {
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic_effective = derive_effective(self.claimed_claim(), &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic_effective.overclaims_as(r.rendered_claim))
        {
            reasons.push(ContinuityNarrowingReason::SurfaceOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive this surface's claim decision.
    pub fn narrow(&self, stale_window: bool) -> SurfaceDecision {
        let claimed = self.claimed_claim();
        if matches!(claimed, ContinuityClaim::LabsNotClaimed) {
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

    /// Whether a floored surface still keeps a keyboard recovery fallback rather than a
    /// misleading clean submit.
    pub fn floored_keeps_fallback(&self, effective: ContinuityClaim) -> bool {
        if !matches!(effective, ContinuityClaim::Unsafe) {
            return true;
        }
        matches!(
            self.declared_recovery_target,
            RecoveryTarget::StepOnly | RecoveryTarget::NoneKeyboardFallback
        ) || opt_present(&self.lineage.recovery_backlink_ref)
    }

    /// Whether any rendering overclaims relative to `effective`.
    pub fn surface_overclaims(&self, effective: ContinuityClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// A reviewer-facing label for a narrowed/floored surface, or `None` if it holds.
    pub fn narrowed_label(&self, decision: &SurfaceDecision) -> Option<String> {
        let trigger = decision.downgrade_trigger()?;
        Some(match decision.effective_claim {
            ContinuityClaim::Unsafe => format!(
                "Floored to continuity_unsafe below the {} claim: {}; falls back to an explicit blocked-submit state with a keyboard recovery path.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            ContinuityClaim::Narrowed => format!(
                "Held at continuity_narrowed below the {} claim: {}; the surface stays keyboard-complete and recoverable until re-verified.",
                decision.claimed_claim.as_str(),
                trigger.describe(),
            ),
            _ => return None,
        })
    }

    /// Append per-surface structural violations (schema-shape level).
    fn structural_violations(&self, out: &mut Vec<M5AccessibilityContinuityViolation>) {
        use M5AccessibilityContinuityViolation as V;
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

/// Constructor input for [`M5AccessibilityContinuitySetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AccessibilityContinuitySetInput {
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
    pub surfaces: Vec<SurfaceRecord>,
}

/// Export-safe M5 accessibility-and-continuity set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AccessibilityContinuitySetPacket {
    /// Record kind; must equal [`M5_ACCESSIBILITY_CONTINUITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ACCESSIBILITY_CONTINUITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_ACCESSIBILITY_CONTINUITY_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-surface rows.
    pub surfaces: Vec<SurfaceRecord>,
}

/// The distribution of effective surface claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceClaimDistribution {
    /// Surfaces effective at [`ContinuityClaim::Certified`].
    pub certified: usize,
    /// Surfaces effective at [`ContinuityClaim::Narrowed`].
    pub narrowed: usize,
    /// Surfaces effective at [`ContinuityClaim::ReviewOverlay`].
    pub overlay: usize,
    /// Surfaces effective at [`ContinuityClaim::Unsafe`].
    pub unsafe_surfaces: usize,
    /// Surfaces effective at [`ContinuityClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5AccessibilityContinuitySetPacket {
    /// Builds an accessibility-and-continuity set packet, sealing the record-kind,
    /// schema, and taxonomy version constants.
    pub fn new(input: M5AccessibilityContinuitySetInput) -> Self {
        Self {
            record_kind: M5_ACCESSIBILITY_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: M5_ACCESSIBILITY_CONTINUITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_ACCESSIBILITY_CONTINUITY_TAXONOMY_VERSION,
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
                ContinuityClaim::Certified => dist.certified += 1,
                ContinuityClaim::Narrowed => dist.narrowed += 1,
                ContinuityClaim::ReviewOverlay => dist.overlay += 1,
                ContinuityClaim::Unsafe => dist.unsafe_surfaces += 1,
                ContinuityClaim::LabsNotClaimed => dist.labs += 1,
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
    pub fn represented_kinds(&self) -> BTreeSet<SurfaceKind> {
        self.surfaces.iter().map(|s| s.surface_kind).collect()
    }

    /// Product lanes represented by some surface.
    pub fn represented_lanes(&self) -> BTreeSet<SurfaceLane> {
        self.surfaces.iter().map(|s| s.lane).collect()
    }

    /// Surface origins represented by some surface.
    pub fn represented_origins(&self) -> BTreeSet<SurfaceOrigin> {
        self.surfaces.iter().map(|s| s.origin).collect()
    }

    /// Interruption paths preserved by some surface.
    pub fn represented_interruption_paths(&self) -> BTreeSet<InterruptionPath> {
        self.surfaces
            .iter()
            .flat_map(|s| s.continuity.preserved_paths())
            .collect()
    }

    /// Consumer surfaces represented by some rendering.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<ConsumerSurface> {
        self.surfaces
            .iter()
            .flat_map(|s| s.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Whether every reduced-motion substitution class is represented by some surface.
    fn all_reduced_motion_classes_represented(&self) -> bool {
        ALL_REDUCED_MOTION_CLASSES.iter().all(|class| {
            self.surfaces
                .iter()
                .any(|s| s.accessibility.reduced_motion.substitution_class == *class)
        })
    }

    /// Validate the accessibility-and-continuity invariants.
    pub fn validate(&self) -> Vec<M5AccessibilityContinuityViolation> {
        use M5AccessibilityContinuityViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_ACCESSIBILITY_CONTINUITY_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_ACCESSIBILITY_CONTINUITY_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_ACCESSIBILITY_CONTINUITY_TAXONOMY_VERSION {
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

        if SurfaceKind::ALL
            .iter()
            .any(|x| !self.represented_kinds().contains(x))
        {
            violations.push(V::SurfaceKindMissing);
        }
        if SurfaceLane::ALL
            .iter()
            .any(|l| !self.represented_lanes().contains(l))
        {
            violations.push(V::SurfaceLaneMissing);
        }
        if SurfaceOrigin::ALL
            .iter()
            .any(|o| !self.represented_origins().contains(o))
        {
            violations.push(V::SurfaceOriginMissing);
        }
        if InterruptionPath::ALL
            .iter()
            .any(|p| !self.represented_interruption_paths().contains(p))
        {
            violations.push(V::InterruptionPathMissing);
        }
        if !self.all_reduced_motion_classes_represented() {
            violations.push(V::ReducedMotionClassMissing);
        }
        if ConsumerSurface::ALL
            .iter()
            .any(|c| !self.represented_consumer_surfaces().contains(c))
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
            &serde_json::to_value(self).expect("accessibility-continuity packet serializes"),
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
        serde_json::to_string_pretty(self).expect("accessibility-continuity packet serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_report(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str(
            "# M5 Keyboard, Assistive-Tech, Reduced-Motion, And Interruption-Safe Continuity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Surfaces: {}\n", self.surfaces.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} review overlay, {} unsafe, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.unsafe_surfaces, dist.labs
        ));

        out.push_str("| Surface | Kind | Lane | Origin | Reduced motion | Claimed | Effective |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for s in &self.surfaces {
            let decision = s.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {:?} | {} | {} |\n",
                s.surface_id,
                s.surface_kind.as_str(),
                s.lane.as_str(),
                s.origin.as_str(),
                s.accessibility.reduced_motion.substitution_class,
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

/// Error returned when the checked support-export artifact fails to load or validate.
#[derive(Debug)]
pub enum M5AccessibilityContinuityArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5AccessibilityContinuityViolation>),
}

impl fmt::Display for M5AccessibilityContinuityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(f, "support export parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for M5AccessibilityContinuityArtifactError {}

/// An accessibility-and-continuity packet validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessibilityContinuityViolation {
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
    SurfaceLaneMissing,
    /// A surface origin is unrepresented.
    SurfaceOriginMissing,
    /// An interruption path is unrepresented.
    InterruptionPathMissing,
    /// A reduced-motion substitution class is unrepresented.
    ReducedMotionClassMissing,
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
    /// A floored surface loses its keyboard recovery fallback.
    FlooredSurfaceLosesFallback,
    /// A rendering surface renders wider than the effective claim.
    RenderingSurfaceOverclaims,
    /// No surface demonstrates the auto-narrowing rule.
    DowngradedSurfaceCaseMissing,
    /// Raw boundary material crossed the export.
    RawBoundaryMaterialInExport,
}

impl M5AccessibilityContinuityViolation {
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
            Self::SurfaceLaneMissing => "surface_lane_missing",
            Self::SurfaceOriginMissing => "surface_origin_missing",
            Self::InterruptionPathMissing => "interruption_path_missing",
            Self::ReducedMotionClassMissing => "reduced_motion_class_missing",
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
/// support, admin, import, and project surfaces — plus the accessibility-audit and
/// CLI/headless paths and the docs/help references — use to ingest the frozen
/// accessibility-and-continuity matrix instead of minting per-feature accessibility or
/// recovery semantics.
///
/// # Errors
///
/// Returns [`M5AccessibilityContinuityArtifactError`] when the artifact cannot be
/// parsed or fails validation.
pub fn current_m5_accessibility_continuity_set(
) -> Result<M5AccessibilityContinuitySetPacket, M5AccessibilityContinuityArtifactError> {
    let packet: M5AccessibilityContinuitySetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ux/m5-accessibility-and-continuity/support_export.json"
    )))
    .map_err(M5AccessibilityContinuityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AccessibilityContinuityArtifactError::Validation(
            violations,
        ))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded accessibility-and-continuity set: the in-crate source of truth
/// the checked-in support export and report are regenerated from.
pub fn seeded_m5_accessibility_continuity_set() -> M5AccessibilityContinuitySetPacket {
    M5AccessibilityContinuitySetPacket::new(M5AccessibilityContinuitySetInput {
        packet_id: M5_ACCESSIBILITY_CONTINUITY_PACKET_ID.to_owned(),
        label:
            "M5 keyboard, assistive-tech, reduced-motion, and interruption-safe continuity for dense multi-step forms, inline validation links, and batch-review sheets"
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
    claim: ContinuityClaim,
    surfaces: &[ConsumerSurface],
    read_only: bool,
) -> Vec<SurfaceRendering> {
    surfaces
        .iter()
        .map(|&surface| SurfaceRendering {
            surface,
            rendered_claim: claim,
            recovery_visible: true,
            read_only,
            source_surface_ref: source_ref.to_owned(),
        })
        .collect()
}

/// A clean first-party keyboard block.
fn clean_keyboard(label: &str) -> KeyboardAccess {
    KeyboardAccess {
        focus_order_defined: true,
        all_controls_reachable: true,
        roving_tabindex: true,
        batch_actions_keyboard_parity: true,
        focus_trap_escapable: true,
        focus_trap_escape_labeled: true,
        keyboard_label: label.to_owned(),
    }
}

/// A clean first-party assistive-tech block.
fn clean_at(label: &str) -> AssistiveTech {
    AssistiveTech {
        screen_reader_labels_present: true,
        validation_links_announced: true,
        blocked_submit_live_region: true,
        step_position_announced: true,
        at_label: label.to_owned(),
    }
}

/// A clean reduced-motion block bound to the shared substitution class.
fn clean_reduced_motion(
    substitution_class: ReducedMotionSubstitutionClass,
    label: &str,
) -> ReducedMotion {
    ReducedMotion {
        substitution_class,
        state_conveyed_without_motion: true,
        progress_non_motion_marker: true,
        substitution_labeled: true,
        progress_marker_labeled: true,
        reduced_motion_label: label.to_owned(),
    }
}

/// A clean first-party continuity snapshot with a complete journal.
fn clean_continuity(label: &str) -> ContinuitySnapshot {
    ContinuitySnapshot {
        journal_backed: true,
        journal_state: JournalState::Complete,
        current_step_preserved: true,
        blocked_fields_preserved: true,
        draft_state_preserved: true,
        resume_on_reconnect: true,
        resume_on_restart: true,
        resume_on_missing_dependency: true,
        resume_on_crash: true,
        continuity_label: label.to_owned(),
    }
}

/// A clean first-party integrity block.
fn clean_integrity() -> SurfaceIntegrity {
    SurfaceIntegrity {
        keyboard_complete: true,
        focus_order_deterministic: true,
        batch_actions_keyboard_reachable: true,
        screen_reader_reachable: true,
        validation_links_in_at: true,
        blocked_submit_in_live_region: true,
        state_without_motion: true,
        step_resumes_correctly: true,
        blocked_fields_retained: true,
        draft_state_retained: true,
        imported_review_read_only: true,
        step_position_announced: true,
    }
}

/// A verified-current verification block.
fn verified(proof_ref: &str) -> SurfaceVerification {
    SurfaceVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

/// The canonical surfaces: one per lane, covering every surface kind, origin,
/// reduced-motion substitution class, interruption path, and consumer surface, plus a
/// narrowed first-party surface, an import review overlay, and a Labs surface.
fn seed_surfaces() -> Vec<SurfaceRecord> {
    use ConsumerSurface as CS;
    use ReducedMotionSubstitutionClass as RM;

    // 1. Provider connect wizard: a dense multi-step form. Certified.
    let provider = SurfaceRecord {
        surface_id: "surface:provider-connect-wizard:0001".to_owned(),
        surface_kind: SurfaceKind::MultiStepForm,
        lane: SurfaceLane::Provider,
        origin: SurfaceOrigin::ProviderForm,
        label_summary: "Provider connect wizard: a dense multi-step form, keyboard-complete and screen-reader reachable, that resumes on the correct step after a reconnect or restart.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_recovery_target: RecoveryTarget::SurfaceAndStep,
        lineage: SurfaceLineage {
            session_ref: "session:provider-connect:0001".to_owned(),
            canonical_surface_ref: None,
            form_ref: Some("form:provider-connect-wizard".to_owned()),
            provider_ref: Some("provider:registered:0001".to_owned()),
            source_artifact_ref: None,
            policy_ref: None,
            journal_ref: Some("journal:provider-connect:0001".to_owned()),
            recovery_backlink_ref: Some("recover:provider-connect:0001".to_owned()),
        },
        accessibility: AccessibilityPosture {
            keyboard: clean_keyboard(
                "Roving focus across each step; every control is keyboard-reachable and the step dialog's focus trap is escapable.",
            ),
            assistive_tech: clean_at(
                "Every control carries a permanent screen-reader label; the step position is announced and blocked prerequisites are read from a live region.",
            ),
            reduced_motion: clean_reduced_motion(
                RM::MaintainEssentialKeepSimplified,
                "Step transitions keep a simplified motion under reduced motion; the step state never depends on animation.",
            ),
        },
        continuity: clean_continuity(
            "The wizard autosaves each step; a reconnect, restart, missing dependency, or crash resumes on the correct step with blocked fields and draft intact.",
        ),
        integrity: clean_integrity(),
        verification: verified("proof:provider-connect:0001"),
        renderings: renderings(
            "surface:provider-connect-wizard:0001",
            ContinuityClaim::Certified,
            &[CS::LiveSurface, CS::DiagnosticsPanel, CS::CliHeadless],
            false,
        ),
    };

    // 2. Admin batch source-trust review sheet: keyboard-parity batch actions.
    //    Certified.
    let admin = SurfaceRecord {
        surface_id: "surface:admin-source-batch-review:0001".to_owned(),
        surface_kind: SurfaceKind::BatchReviewSheet,
        lane: SurfaceLane::Admin,
        origin: SurfaceOrigin::RemoteForm,
        label_summary: "Admin source-trust batch-review sheet: every batch action has keyboard parity with its pointer affordance, and the result scope is announced.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_recovery_target: RecoveryTarget::SurfaceAndStep,
        lineage: SurfaceLineage {
            session_ref: "session:admin-source-batch:0001".to_owned(),
            canonical_surface_ref: None,
            form_ref: Some("form:admin-source-batch-review".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: Some("policy:source-trust:0001".to_owned()),
            journal_ref: Some("journal:admin-source-batch:0001".to_owned()),
            recovery_backlink_ref: Some("recover:admin-console:0001".to_owned()),
        },
        accessibility: AccessibilityPosture {
            keyboard: clean_keyboard(
                "Roving focus across the selection list; select-all, exclude, and apply all have keyboard equivalents.",
            ),
            assistive_tech: clean_at(
                "The selection bar announces the included/excluded/blocked counts; each batch action carries a screen-reader label.",
            ),
            reduced_motion: clean_reduced_motion(
                RM::NonMotionStateMarker,
                "Selection and apply progress are carried by non-motion state markers (chips/counts), not animation.",
            ),
        },
        continuity: clean_continuity(
            "The selection and per-member decisions autosave; a reconnect or restart resumes the review with the same included/excluded set.",
        ),
        integrity: clean_integrity(),
        verification: verified("proof:admin-source-batch:0001"),
        renderings: renderings(
            "surface:admin-source-batch-review:0001",
            ContinuityClaim::Certified,
            &[CS::ReviewSheet, CS::AccessibilityAudit, CS::SupportExport],
            false,
        ),
    };

    // 3. Request inline validation-links group: narrows because its recovery journal is
    //    partial (it captures the draft but not the cross-field blocker state yet).
    let request = SurfaceRecord {
        surface_id: "surface:request-environment-validation:0001".to_owned(),
        surface_kind: SurfaceKind::InlineValidationLinks,
        lane: SurfaceLane::Request,
        origin: SurfaceOrigin::RemoteForm,
        label_summary: "Request environment inline validation links: each error links to its field and is announced to assistive tech; its recovery journal is partial.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_recovery_target: RecoveryTarget::SurfaceAndStep,
        lineage: SurfaceLineage {
            session_ref: "session:request-environment:0001".to_owned(),
            canonical_surface_ref: None,
            form_ref: Some("form:request-environment".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: None,
            journal_ref: Some("journal:request-environment:0001".to_owned()),
            recovery_backlink_ref: Some("recover:request-workspace:0001".to_owned()),
        },
        accessibility: AccessibilityPosture {
            keyboard: clean_keyboard(
                "Each inline validation link moves focus to the offending field; the group is fully keyboard-navigable.",
            ),
            assistive_tech: clean_at(
                "Validation links are exposed as in-context references to their fields and announced through a live region as they appear.",
            ),
            reduced_motion: clean_reduced_motion(
                RM::CollapseToInstant,
                "Error reveal collapses to an instant state change under reduced motion; the error text never depends on the animation.",
            ),
        },
        continuity: ContinuitySnapshot {
            journal_backed: true,
            journal_state: JournalState::Partial,
            current_step_preserved: true,
            blocked_fields_preserved: true,
            draft_state_preserved: true,
            resume_on_reconnect: true,
            resume_on_restart: true,
            resume_on_missing_dependency: true,
            resume_on_crash: true,
            continuity_label:
                "The draft is journalled, but the cross-field blocker state is not yet fully captured, so the journal is partial."
                    .to_owned(),
        },
        integrity: clean_integrity(),
        verification: verified("proof:request-environment:0001"),
        renderings: renderings(
            "surface:request-environment-validation:0001",
            ContinuityClaim::Narrowed,
            &[CS::LiveSurface, CS::SupportExport, CS::CliHeadless],
            false,
        ),
    };

    // 4. Package staged install-review sheet: a commit sheet with batch members.
    //    Certified.
    let package = SurfaceRecord {
        surface_id: "surface:package-install-review:0001".to_owned(),
        surface_kind: SurfaceKind::StagedReviewSheet,
        lane: SurfaceLane::Package,
        origin: SurfaceOrigin::LocalForm,
        label_summary: "Package staged install-review sheet: the target scope, included/excluded members, and rollback path are keyboard-reachable and announced before commit.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_recovery_target: RecoveryTarget::SurfaceAndStep,
        lineage: SurfaceLineage {
            session_ref: "session:package-install:0001".to_owned(),
            canonical_surface_ref: None,
            form_ref: Some("form:package-install-review".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: None,
            journal_ref: Some("journal:package-install:0001".to_owned()),
            recovery_backlink_ref: Some("recover:package-manager:0001".to_owned()),
        },
        accessibility: AccessibilityPosture {
            keyboard: clean_keyboard(
                "The member list and the scope-specific commit action are reachable in a deterministic focus order; the sheet's focus trap is escapable.",
            ),
            assistive_tech: clean_at(
                "Omitted defaults, side effects, and the rollback path are announced; a blocked prerequisite is read from a live region before commit.",
            ),
            reduced_motion: clean_reduced_motion(
                RM::CrossfadeOnly,
                "The sheet opens with an opacity-only crossfade; the commit scope is fully legible without it.",
            ),
        },
        continuity: clean_continuity(
            "The staged decisions autosave; a crash or restart resumes the review on the same step with the same included/excluded members.",
        ),
        integrity: clean_integrity(),
        verification: verified("proof:package-install:0001"),
        renderings: renderings(
            "surface:package-install-review:0001",
            ContinuityClaim::Certified,
            &[CS::ReviewSheet, CS::LiveSurface, CS::HelpInline],
            false,
        ),
    };

    // 5. Settings config editor: a structured configuration editor. Certified.
    let settings = SurfaceRecord {
        surface_id: "surface:settings-config-editor:0001".to_owned(),
        surface_kind: SurfaceKind::ConfigEditor,
        lane: SurfaceLane::Settings,
        origin: SurfaceOrigin::LocalForm,
        label_summary: "Settings configuration editor: dense field groups stay keyboard-complete and screen-reader reachable, with reduced-motion-safe state and crash-safe drafts.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_recovery_target: RecoveryTarget::SurfaceAndStep,
        lineage: SurfaceLineage {
            session_ref: "session:settings-config:0001".to_owned(),
            canonical_surface_ref: None,
            form_ref: Some("form:settings-config-editor".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: None,
            journal_ref: Some("journal:settings-config:0001".to_owned()),
            recovery_backlink_ref: Some("recover:settings-editor:0001".to_owned()),
        },
        accessibility: AccessibilityPosture {
            keyboard: clean_keyboard(
                "Field groups use roving focus; every editor control and its reset/override affordance is keyboard-reachable.",
            ),
            assistive_tech: clean_at(
                "Each field carries a permanent label and source tag; validation links are announced and a blocked save is read from a live region.",
            ),
            reduced_motion: clean_reduced_motion(
                RM::SuppressEntirely,
                "Non-essential editor motion is suppressed entirely under reduced motion; static state markers remain.",
            ),
        },
        continuity: clean_continuity(
            "Edits autosave to a journal; a crash, restart, or reconnect recovers the draft with the unsaved-versus-applied state intact.",
        ),
        integrity: clean_integrity(),
        verification: verified("proof:settings-config:0001"),
        renderings: renderings(
            "surface:settings-config-editor:0001",
            ContinuityClaim::Certified,
            &[CS::LiveSurface, CS::DiagnosticsPanel, CS::AccessibilityAudit],
            false,
        ),
    };

    // 6. Import/migration review overlay: a read-only review of imported values. Review
    //    overlay (never reads as an apply).
    let import = SurfaceRecord {
        surface_id: "surface:import-migration-review:0001".to_owned(),
        surface_kind: SurfaceKind::StagedReviewSheet,
        lane: SurfaceLane::Import,
        origin: SurfaceOrigin::ImportedReview,
        label_summary: "Import/migration review overlay: a read-only, keyboard-complete review of what the migration brought in, never a value you have applied.".to_owned(),
        claim_posture: ClaimPosture::ClaimedStable,
        declared_recovery_target: RecoveryTarget::SurfaceAndStep,
        lineage: SurfaceLineage {
            session_ref: "session:import-migration:0001".to_owned(),
            canonical_surface_ref: None,
            form_ref: Some("form:import-migration-review".to_owned()),
            provider_ref: None,
            source_artifact_ref: Some("artifact:migration-bundle:0001".to_owned()),
            policy_ref: None,
            journal_ref: None,
            recovery_backlink_ref: Some("recover:migration-center:0001".to_owned()),
        },
        accessibility: AccessibilityPosture {
            keyboard: clean_keyboard(
                "The imported-member list is keyboard-navigable with roving focus; the review dialog's focus trap is escapable.",
            ),
            assistive_tech: clean_at(
                "Each imported member is announced as a read-only review item with its source; validation links to the bundle are announced.",
            ),
            reduced_motion: clean_reduced_motion(
                RM::NonMotionStateMarker,
                "Review status is carried by non-motion markers; the read-only state never depends on animation.",
            ),
        },
        continuity: ContinuitySnapshot {
            journal_backed: false,
            journal_state: JournalState::Missing,
            current_step_preserved: false,
            blocked_fields_preserved: false,
            draft_state_preserved: false,
            resume_on_reconnect: true,
            resume_on_restart: true,
            resume_on_missing_dependency: true,
            resume_on_crash: true,
            continuity_label:
                "A read-only review has no draft to recover; reopening returns to the same review from the migration bundle."
                    .to_owned(),
        },
        integrity: clean_integrity(),
        verification: verified("proof:import-migration:0001"),
        renderings: renderings(
            "surface:import-migration-review:0001",
            ContinuityClaim::ReviewOverlay,
            &[CS::ReviewSheet, CS::SupportExport, CS::HelpInline],
            true,
        ),
    };

    // 7. Labs project-bootstrap wizard: makes no public claim.
    let labs = SurfaceRecord {
        surface_id: "surface:project-bootstrap-wizard:0001".to_owned(),
        surface_kind: SurfaceKind::MultiStepForm,
        lane: SurfaceLane::Projects,
        origin: SurfaceOrigin::LocalForm,
        label_summary:
            "Experimental project-bootstrap wizard (Labs): a multi-step form that makes no public accessibility/continuity claim while unadvertised."
                .to_owned(),
        claim_posture: ClaimPosture::LabsUnadvertised,
        declared_recovery_target: RecoveryTarget::SurfaceAndStep,
        lineage: SurfaceLineage {
            session_ref: "session:project-bootstrap:0001".to_owned(),
            canonical_surface_ref: None,
            form_ref: Some("form:project-bootstrap-wizard".to_owned()),
            provider_ref: None,
            source_artifact_ref: None,
            policy_ref: None,
            journal_ref: Some("journal:project-bootstrap:0001".to_owned()),
            recovery_backlink_ref: Some("recover:project-bootstrap:0001".to_owned()),
        },
        accessibility: AccessibilityPosture {
            keyboard: clean_keyboard(
                "Keyboard navigation across the experimental steps is present but unadvertised.",
            ),
            assistive_tech: clean_at(
                "Screen-reader labels are present on the experimental steps but the surface is not publicly claimed.",
            ),
            reduced_motion: clean_reduced_motion(
                RM::CollapseToInstant,
                "Experimental step transitions collapse to instant under reduced motion.",
            ),
        },
        continuity: clean_continuity(
            "Experimental drafts journal locally; recovery is present but unadvertised.",
        ),
        integrity: clean_integrity(),
        verification: verified("proof:project-bootstrap:0001"),
        renderings: renderings(
            "surface:project-bootstrap-wizard:0001",
            ContinuityClaim::LabsNotClaimed,
            &[CS::LiveSurface, CS::HelpInline],
            false,
        ),
    };

    vec![provider, admin, request, package, settings, import, labs]
}
