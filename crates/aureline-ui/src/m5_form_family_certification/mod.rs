//! Canonical certification truth for M5 mutation-capable form families.
//!
//! Where [`crate::m5_field_control_rows`], [`crate::m5_form_validation_and_blocked_submit`],
//! [`crate::m5_parameter_source_and_precedence`], [`crate::m5_draft_state_and_autosave`],
//! [`crate::m5_staged_review_sheets`], [`crate::m5_structured_input_and_staged_review`], and
//! [`crate::m5_accessibility_and_continuity`] each freeze one *component* of the shared
//! structured-input contract, this module freezes the **promotion model that certifies a
//! whole claimed form family** against those components and **auto-narrows the family's
//! qualification claim** when its structured-input, provenance, draft-recovery, or
//! staged-review proof is stale, partial, missing, or failing. No claimed M5
//! mutation-capable form family can stay fully certified while the evidence behind it has
//! gone cold.
//!
//! A *form family* is a claimed mutation-capable surface family — the provider connect /
//! account-mapping forms, the admin source-management and batch-review sheets, the
//! request-workspace replay/mutation forms, the package install/update/remove review
//! sheets, the settings / structured config editors, the import / migration-center restore
//! reviews, and the generated-project / bootstrap wizards. Each [`FamilyRecord`] certifies
//! one family by binding an [`EvidenceCell`] per **required proof pair** — every
//! [`ProofDimension`] mapped onto the upstream [`ProofLane`](enum@ProofLane) that proves it:
//!
//! * **field/form validation** — [`ProofLane::FieldControlRows`] and
//!   [`ProofLane::FormValidationAndBlockedSubmit`];
//! * **parameter provenance** — [`ProofLane::ParameterSourceAndPrecedence`];
//! * **draft-versus-applied truth** — [`ProofLane::DraftStateAndAutosave`];
//! * **interruption recovery** — [`ProofLane::AccessibilityAndContinuity`]; and
//! * **staged-review-before-commit** — [`ProofLane::StagedReviewSheets`] and
//!   [`ProofLane::StructuredInputAndStagedReview`].
//!
//! Each cell carries an [`EvidenceState`] (current, stale, partial, missing, failing, or
//! not-applicable) and a ref to the lane's canonical support export. Each record re-derives
//! a [`FamilyDecision`] ([`FamilyRecord::narrow`]) that floors the family's claimed
//! [`QualificationTier`] by the weakest evidence it rests on: a stale or partial proof caps
//! the family at [`QualificationTier::Beta`], a missing required proof caps it at
//! [`QualificationTier::Preview`], a failing proof or a consumer surface that renders a
//! wider tier than the evidence supports withdraws it to [`QualificationTier::Withdrawn`],
//! and an elapsed certification-freshness window ages every certified family down to
//! [`QualificationTier::Beta`]. The re-derived [`CertificationVerdict`] therefore can never
//! read wider than the current evidence.
//!
//! Each family also renders its verdict to every [`ConsumerSurface`] — About, inline help,
//! service health, compatibility, the release packet, the support export, and the public
//! docs truth — so those surfaces *consume the same qualification state* instead of
//! restating a form-quality claim manually, and a rendering that shows a wider tier than the
//! effective verdict is caught as an overclaim.
//!
//! [`M5FormFamilyCertificationSetPacket::validate`] confirms the matrix is well-formed and
//! honest: header / identity / redaction / freshness are present; every form family, proof
//! dimension, proof lane, and consumer surface is represented; every required proof pair is
//! present and evidence-coherent; no certified cell claims fresher than its capture allows;
//! a narrowed family keeps an actionable rerun path; no rendering overclaims; at least one
//! family demonstrates the auto-narrowing rule; and no raw credential / body material crosses
//! the export. Downstream About / help / service-health / compatibility surfaces and the
//! release and support packets ingest this packet rather than re-deciding which M5 form
//! families are certified.
//!
//! No credential bodies, secret values, raw provider payloads, absolute paths, or URLs ever
//! cross this boundary; the packet carries only typed class tokens, booleans, opaque ids,
//! and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/ux/m5-form-family-certification.schema.json`](../../../../schemas/ux/m5-form-family-certification.schema.json).
//! The contract doc is
//! [`docs/ux/m5-form-family-certification.md`](../../../../docs/ux/m5-form-family-certification.md).
//! The canonical support export is
//! [`artifacts/ux/m5-form-family-certification/support_export.json`](../../../../artifacts/ux/m5-form-family-certification/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/ux/m5-form-family-certification/`](../../../../fixtures/ux/m5-form-family-certification/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5FormFamilyCertificationSetPacket`].
pub const M5_FORM_FAMILY_CERTIFICATION_RECORD_KIND: &str =
    "m5_form_family_certification_set_packet";

/// Schema version for the form-family certification set.
pub const M5_FORM_FAMILY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_FORM_FAMILY_CERTIFICATION_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical form-family certification set packet.
pub const M5_FORM_FAMILY_CERTIFICATION_PACKET_ID: &str = "m5-form-family-certification:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_FORM_FAMILY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ux/m5-form-family-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_FORM_FAMILY_CERTIFICATION_DOC_REF: &str = "docs/ux/m5-form-family-certification.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_FORM_FAMILY_CERTIFICATION_SUPPORT_EXPORT_REF: &str =
    "artifacts/ux/m5-form-family-certification/support_export.json";

/// Repo-relative path of the generated report.
pub const M5_FORM_FAMILY_CERTIFICATION_REPORT_REF: &str =
    "artifacts/ux/m5-form-family-certification/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_FORM_FAMILY_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ux/m5-form-family-certification";

/// Allowed packet redaction-class tokens.
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

/// Deterministic seed timestamp for the canonical packet and report.
const SEED_AS_OF: &str = "2026-06-22T00:00:00Z";

/// Capture timestamp for the one stale proof in the canonical packet (well past the
/// freshness window so the request-workspace family narrows honestly at baseline).
const SEED_STALE_CAPTURED_AT: &str = "2026-05-01T00:00:00Z";

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

/// Whether a reviewer-facing label is empty or one of the generic non-labels that hide the
/// real downgrade reason.
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

/// Whether a serialized value carries forbidden raw boundary material (secrets, credential
/// bodies). The export must never leak these.
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

/// A claimed M5 mutation-capable form family this lane certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormFamily {
    /// Provider connect / account-mapping forms.
    ProviderConnect,
    /// Admin source-management and batch-review sheets.
    AdminSourceManagement,
    /// Request-workspace replay / mutation forms.
    RequestWorkspace,
    /// Package install / update / remove review sheets.
    PackageInstallReview,
    /// Settings / structured config editors.
    SettingsConfigEditor,
    /// Import / migration-center restore reviews.
    ImportMigrationCenter,
    /// Generated-project / bootstrap wizards.
    GeneratedProjectBootstrap,
}

impl FormFamily {
    /// Every form family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProviderConnect,
        Self::AdminSourceManagement,
        Self::RequestWorkspace,
        Self::PackageInstallReview,
        Self::SettingsConfigEditor,
        Self::ImportMigrationCenter,
        Self::GeneratedProjectBootstrap,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderConnect => "provider_connect",
            Self::AdminSourceManagement => "admin_source_management",
            Self::RequestWorkspace => "request_workspace",
            Self::PackageInstallReview => "package_install_review",
            Self::SettingsConfigEditor => "settings_config_editor",
            Self::ImportMigrationCenter => "import_migration_center",
            Self::GeneratedProjectBootstrap => "generated_project_bootstrap",
        }
    }
}

/// A proof dimension every claimed form family must certify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofDimension {
    /// Field-level and form-level validation and blocked-submit truth.
    FieldFormValidation,
    /// Parameter-source provenance and precedence.
    ParameterProvenance,
    /// Draft-versus-applied state and recovery.
    DraftVersusApplied,
    /// Interruption-safe recovery (keyboard, assistive tech, continuity).
    InterruptionRecovery,
    /// Staged-review-before-commit sheets.
    StagedReviewBeforeCommit,
}

impl ProofDimension {
    /// Every proof dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FieldFormValidation,
        Self::ParameterProvenance,
        Self::DraftVersusApplied,
        Self::InterruptionRecovery,
        Self::StagedReviewBeforeCommit,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FieldFormValidation => "field_form_validation",
            Self::ParameterProvenance => "parameter_provenance",
            Self::DraftVersusApplied => "draft_versus_applied",
            Self::InterruptionRecovery => "interruption_recovery",
            Self::StagedReviewBeforeCommit => "staged_review_before_commit",
        }
    }

    /// The narrowing reason raised when this dimension's proof is uncertified.
    pub const fn narrow_reason(self) -> NarrowingReason {
        match self {
            Self::FieldFormValidation => NarrowingReason::FieldFormValidationUncertified,
            Self::ParameterProvenance => NarrowingReason::ParameterProvenanceUncertified,
            Self::DraftVersusApplied => NarrowingReason::DraftRecoveryUncertified,
            Self::InterruptionRecovery => NarrowingReason::InterruptionRecoveryUncertified,
            Self::StagedReviewBeforeCommit => NarrowingReason::StagedReviewUncertified,
        }
    }
}

/// The upstream M5 form-component lane that produces a proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofLane {
    /// Per-row field control labels, sources, and anchors.
    FieldControlRows,
    /// Form-level validation rollups and machine-readable blocked-submit reasons.
    FormValidationAndBlockedSubmit,
    /// Parameter-source inspector and precedence resolution.
    ParameterSourceAndPrecedence,
    /// Local draft state, autosave journal, and recover-draft.
    DraftStateAndAutosave,
    /// Staged-review (commit) sheets.
    StagedReviewSheets,
    /// The shared structured-input and staged-review set.
    StructuredInputAndStagedReview,
    /// Keyboard, assistive-tech, reduced-motion, and continuity contract.
    AccessibilityAndContinuity,
}

impl ProofLane {
    /// Every proof lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FieldControlRows,
        Self::FormValidationAndBlockedSubmit,
        Self::ParameterSourceAndPrecedence,
        Self::DraftStateAndAutosave,
        Self::StagedReviewSheets,
        Self::StructuredInputAndStagedReview,
        Self::AccessibilityAndContinuity,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FieldControlRows => "field_control_rows",
            Self::FormValidationAndBlockedSubmit => "form_validation_and_blocked_submit",
            Self::ParameterSourceAndPrecedence => "parameter_source_and_precedence",
            Self::DraftStateAndAutosave => "draft_state_and_autosave",
            Self::StagedReviewSheets => "staged_review_sheets",
            Self::StructuredInputAndStagedReview => "structured_input_and_staged_review",
            Self::AccessibilityAndContinuity => "accessibility_and_continuity",
        }
    }

    /// Repo-relative ref to this lane's canonical support export — the evidence this
    /// certification rides on.
    pub const fn support_export_ref(self) -> &'static str {
        match self {
            Self::FieldControlRows => "artifacts/ux/m5-field-control-rows/support_export.json",
            Self::FormValidationAndBlockedSubmit => {
                "artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json"
            }
            Self::ParameterSourceAndPrecedence => {
                "artifacts/ux/m5-parameter-source-and-precedence/support_export.json"
            }
            Self::DraftStateAndAutosave => {
                "artifacts/ux/m5-draft-state-and-autosave/support_export.json"
            }
            Self::StagedReviewSheets => "artifacts/ux/m5-staged-review-sheets/support_export.json",
            Self::StructuredInputAndStagedReview => {
                "artifacts/ux/m5-structured-input-and-staged-review/support_export.json"
            }
            Self::AccessibilityAndContinuity => {
                "artifacts/ux/m5-accessibility-and-continuity/support_export.json"
            }
        }
    }
}

/// The required `(dimension, lane)` proof pairs every claimed family must certify.
const REQUIRED_PROOF_PAIRS: [(ProofDimension, ProofLane); 7] = [
    (
        ProofDimension::FieldFormValidation,
        ProofLane::FieldControlRows,
    ),
    (
        ProofDimension::FieldFormValidation,
        ProofLane::FormValidationAndBlockedSubmit,
    ),
    (
        ProofDimension::ParameterProvenance,
        ProofLane::ParameterSourceAndPrecedence,
    ),
    (
        ProofDimension::DraftVersusApplied,
        ProofLane::DraftStateAndAutosave,
    ),
    (
        ProofDimension::InterruptionRecovery,
        ProofLane::AccessibilityAndContinuity,
    ),
    (
        ProofDimension::StagedReviewBeforeCommit,
        ProofLane::StagedReviewSheets,
    ),
    (
        ProofDimension::StagedReviewBeforeCommit,
        ProofLane::StructuredInputAndStagedReview,
    ),
];

/// The freshness/pass state of a single proof an evidence cell records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    /// Proof is present, passing, and within the freshness window.
    Current,
    /// Proof passed but is past its freshness window.
    Stale,
    /// Proof covers only part of the dimension.
    Partial,
    /// No proof is present for this dimension.
    Missing,
    /// Proof is present but currently failing.
    Failing,
    /// The dimension does not apply to this family.
    NotApplicable,
}

impl EvidenceState {
    /// Every evidence state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Current,
        Self::Stale,
        Self::Partial,
        Self::Missing,
        Self::Failing,
        Self::NotApplicable,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Missing => "missing",
            Self::Failing => "failing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// The qualification tier this state floors a claim to, or `None` when it does not narrow.
    pub const fn floor(self) -> Option<QualificationTier> {
        match self {
            Self::Current | Self::NotApplicable => None,
            Self::Stale | Self::Partial => Some(QualificationTier::Beta),
            Self::Missing => Some(QualificationTier::Preview),
            Self::Failing => Some(QualificationTier::Withdrawn),
        }
    }

    /// Whether this state leaves the dimension stale, weakened, or unproven.
    pub const fn is_stale_or_missing(self) -> bool {
        matches!(
            self,
            Self::Stale | Self::Partial | Self::Missing | Self::Failing
        )
    }

    /// Whether a proof actually ran for this state (and so carries a ref and capture time).
    pub const fn has_capture(self) -> bool {
        matches!(
            self,
            Self::Current | Self::Stale | Self::Partial | Self::Failing
        )
    }
}

/// A claimed release-qualification tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationTier {
    /// Fully certified and promotable.
    Stable,
    /// Certified with a labelled, recoverable gap.
    Beta,
    /// Promotion held pending missing proof.
    Preview,
    /// Withdrawn from the claim entirely.
    Withdrawn,
}

impl QualificationTier {
    /// Every tier, widest first.
    pub const ALL: [Self; 4] = [Self::Stable, Self::Beta, Self::Preview, Self::Withdrawn];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Narrowing rank: `0` is the widest claim, `3` the narrowest.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Stable => 0,
            Self::Beta => 1,
            Self::Preview => 2,
            Self::Withdrawn => 3,
        }
    }

    /// The tier at a given rank.
    const fn from_rank(rank: u8) -> Self {
        match rank {
            0 => Self::Stable,
            1 => Self::Beta,
            2 => Self::Preview,
            _ => Self::Withdrawn,
        }
    }
}

/// The re-derived certification outcome for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationVerdict {
    /// The effective tier matches the claimed tier.
    Certified,
    /// The effective tier is narrower than the claimed tier but not withdrawn.
    Narrowed,
    /// The family is withdrawn from the claim.
    Withdrawn,
}

impl CertificationVerdict {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// A surface that consumes the certification verdict rather than restating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// The About / version surface.
    About,
    /// Inline help.
    HelpInline,
    /// The service-health surface.
    ServiceHealth,
    /// The compatibility surface.
    Compatibility,
    /// The release evidence packet.
    ReleasePacket,
    /// The support export.
    SupportExport,
    /// The public docs truth surface.
    DocsPublicTruth,
}

impl ConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::About,
        Self::HelpInline,
        Self::ServiceHealth,
        Self::Compatibility,
        Self::ReleasePacket,
        Self::SupportExport,
        Self::DocsPublicTruth,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::About => "about",
            Self::HelpInline => "help_inline",
            Self::ServiceHealth => "service_health",
            Self::Compatibility => "compatibility",
            Self::ReleasePacket => "release_packet",
            Self::SupportExport => "support_export",
            Self::DocsPublicTruth => "docs_public_truth",
        }
    }
}

/// A reason a family's claim narrowed below its claimed tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// Field/form validation proof is stale, partial, missing, or failing.
    FieldFormValidationUncertified,
    /// Parameter-provenance proof is stale, partial, missing, or failing.
    ParameterProvenanceUncertified,
    /// Draft-recovery proof is stale, partial, missing, or failing.
    DraftRecoveryUncertified,
    /// Interruption-recovery proof is stale, partial, missing, or failing.
    InterruptionRecoveryUncertified,
    /// Staged-review proof is stale, partial, missing, or failing.
    StagedReviewUncertified,
    /// A required proof pair has no evidence cell at all.
    RequiredProofMissing,
    /// A consumer surface renders a wider tier than the evidence supports.
    VerdictOverclaim,
    /// A consumer surface does not reuse the verdict.
    SurfaceReuseIncomplete,
    /// The certification-freshness window has elapsed.
    CertificationProofStale,
}

impl NarrowingReason {
    /// Stable token recorded in diagnostics and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FieldFormValidationUncertified => "field_form_validation_uncertified",
            Self::ParameterProvenanceUncertified => "parameter_provenance_uncertified",
            Self::DraftRecoveryUncertified => "draft_recovery_uncertified",
            Self::InterruptionRecoveryUncertified => "interruption_recovery_uncertified",
            Self::StagedReviewUncertified => "staged_review_uncertified",
            Self::RequiredProofMissing => "required_proof_missing",
            Self::VerdictOverclaim => "verdict_overclaim",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
            Self::CertificationProofStale => "certification_proof_stale",
        }
    }

    /// Deterministic ordering index (most severe first).
    const fn order_index(self) -> u8 {
        match self {
            Self::FieldFormValidationUncertified => 0,
            Self::ParameterProvenanceUncertified => 1,
            Self::DraftRecoveryUncertified => 2,
            Self::InterruptionRecoveryUncertified => 3,
            Self::StagedReviewUncertified => 4,
            Self::RequiredProofMissing => 5,
            Self::VerdictOverclaim => 6,
            Self::SurfaceReuseIncomplete => 7,
            Self::CertificationProofStale => 8,
        }
    }

    /// A reviewer-facing, non-generic narrow phrase.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::FieldFormValidationUncertified => "field/form validation proof uncertified",
            Self::ParameterProvenanceUncertified => "parameter provenance proof uncertified",
            Self::DraftRecoveryUncertified => "draft recovery proof uncertified",
            Self::InterruptionRecoveryUncertified => "interruption recovery proof uncertified",
            Self::StagedReviewUncertified => "staged-review proof uncertified",
            Self::RequiredProofMissing => "a required proof is missing",
            Self::VerdictOverclaim => "a consumer surface overclaims the verdict",
            Self::SurfaceReuseIncomplete => "a consumer surface does not reuse the verdict",
            Self::CertificationProofStale => "the certification freshness window elapsed",
        }
    }
}

/// Sorts narrowing reasons into deterministic order and dedups.
fn order_reasons(mut reasons: Vec<NarrowingReason>) -> Vec<NarrowingReason> {
    reasons.sort_by_key(|r| r.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Records.
// --------------------------------------------------------------------------- //

/// The certification-freshness window for the whole packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationFreshness {
    /// Freshness budget in hours before certification ages out.
    pub certification_freshness_slo_hours: u32,
    /// When certification was last refreshed (RFC 3339).
    pub last_certification_refresh: String,
    /// Whether an elapsed window auto-narrows certified families.
    pub auto_downgrade_on_stale: bool,
}

/// Where a family's certification evidence and rerun path come from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyLineage {
    /// The evidence run / packet this certification rides on.
    pub evidence_run_ref: String,
    /// How to re-run the proof to refresh a narrowed family (a refresh path).
    pub rerun_ref: Option<String>,
}

/// One proof's evidence for a `(dimension, lane)` pair of a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceCell {
    /// Which proof dimension this cell answers.
    pub dimension: ProofDimension,
    /// Which upstream lane produced the proof.
    pub source_lane: ProofLane,
    /// The freshness/pass state of the proof.
    pub state: EvidenceState,
    /// Repo-relative ref to the lane's support export (present iff the proof ran).
    pub proof_ref: Option<String>,
    /// When the proof was captured (present iff the proof ran).
    pub captured_at: Option<String>,
    /// Reviewer-facing, non-generic label for this cell.
    pub proof_label: String,
}

impl EvidenceCell {
    /// Whether this cell's `(dimension, lane)` matches the given pair.
    fn matches(&self, dimension: ProofDimension, lane: ProofLane) -> bool {
        self.dimension == dimension && self.source_lane == lane
    }

    /// Whether a `current` cell's capture is older than the freshness window allows — the
    /// cell would be claiming fresher than its capture supports.
    fn freshness_overclaims(&self, as_of: &str, slo_hours: u32) -> bool {
        if self.state != EvidenceState::Current {
            return false;
        }
        let captured = match self
            .captured_at
            .as_deref()
            .and_then(parse_rfc3339_to_epoch_seconds)
        {
            Some(value) => value,
            None => return false,
        };
        match parse_rfc3339_to_epoch_seconds(as_of) {
            Some(now) => now - captured > i64::from(slo_hours) * 3600,
            None => false,
        }
    }
}

/// How a consumer surface renders a family's verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyRendering {
    /// The consumer surface.
    pub surface: ConsumerSurface,
    /// The tier the surface renders.
    pub rendered_tier: QualificationTier,
    /// Ref back to the family verdict the surface consumes (never a re-stated claim).
    pub source_family_ref: String,
}

/// One claimed M5 form family certified by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyRecord {
    /// Stable family id.
    pub family_id: String,
    /// Which form family this row certifies.
    pub family: FormFamily,
    /// The tier the family publicly claims (the certification target).
    pub claimed_tier: QualificationTier,
    /// Human-readable summary label.
    pub label_summary: String,
    /// Where the evidence and rerun path come from.
    pub lineage: FamilyLineage,
    /// One cell per required proof pair.
    pub evidence: Vec<EvidenceCell>,
    /// How each consumer surface renders the verdict.
    pub renderings: Vec<FamilyRendering>,
}

/// The re-derived decision for one family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyDecision {
    /// The claimed tier.
    pub claimed_tier: QualificationTier,
    /// The re-derived effective tier.
    pub effective_tier: QualificationTier,
    /// The verdict.
    pub verdict: CertificationVerdict,
    /// Whether the family is still fully certified.
    pub certified: bool,
    /// Whether the effective tier narrowed below the claimed tier.
    pub narrowed: bool,
    /// The ordered narrowing reasons.
    pub reasons: Vec<NarrowingReason>,
    /// The dimensions whose proof is stale or missing.
    pub stale_or_missing_dimensions: Vec<ProofDimension>,
}

impl FamilyDecision {
    /// The most severe narrowing reason, if any.
    pub fn downgrade_trigger(&self) -> Option<NarrowingReason> {
        self.reasons.first().copied()
    }
}

/// Whether `rendered` reads wider (a lower rank) than `effective`.
fn tier_overclaims(effective: QualificationTier, rendered: QualificationTier) -> bool {
    rendered.rank() < effective.rank()
}

impl FamilyRecord {
    /// The cell for a `(dimension, lane)` pair, if present.
    fn cell(&self, dimension: ProofDimension, lane: ProofLane) -> Option<&EvidenceCell> {
        self.evidence.iter().find(|c| c.matches(dimension, lane))
    }

    /// Whether every consumer surface reuses this family's verdict.
    fn surface_reuse_complete(&self) -> bool {
        ConsumerSurface::ALL
            .iter()
            .all(|surface| self.renderings.iter().any(|r| r.surface == *surface))
    }

    /// Re-derive the certification decision for this family, given whether the packet's
    /// freshness window has elapsed.
    pub fn narrow(&self, stale_window: bool) -> FamilyDecision {
        let claimed = self.claimed_tier;
        let mut rank = claimed.rank();
        let mut reasons: Vec<NarrowingReason> = Vec::new();
        let mut stale_or_missing: BTreeSet<ProofDimension> = BTreeSet::new();

        for (dimension, lane) in REQUIRED_PROOF_PAIRS {
            match self.cell(dimension, lane) {
                None => {
                    rank = rank.max(QualificationTier::Preview.rank());
                    reasons.push(NarrowingReason::RequiredProofMissing);
                    stale_or_missing.insert(dimension);
                }
                Some(cell) => {
                    if let Some(floor) = cell.state.floor() {
                        rank = rank.max(floor.rank());
                        reasons.push(dimension.narrow_reason());
                    }
                    if cell.state.is_stale_or_missing() {
                        stale_or_missing.insert(dimension);
                    }
                }
            }
        }

        if stale_window {
            rank = rank.max(QualificationTier::Beta.rank());
            reasons.push(NarrowingReason::CertificationProofStale);
        }
        if !self.surface_reuse_complete() {
            rank = rank.max(QualificationTier::Beta.rank());
            reasons.push(NarrowingReason::SurfaceReuseIncomplete);
        }

        // Overclaim is judged against the intrinsic effective tier, then floors to withdrawn.
        let intrinsic = QualificationTier::from_rank(rank);
        if self
            .renderings
            .iter()
            .any(|r| tier_overclaims(intrinsic, r.rendered_tier))
        {
            reasons.push(NarrowingReason::VerdictOverclaim);
            rank = rank.max(QualificationTier::Withdrawn.rank());
        }

        let effective = QualificationTier::from_rank(rank);
        let narrowed = effective.rank() > claimed.rank();
        let verdict = if effective == QualificationTier::Withdrawn {
            CertificationVerdict::Withdrawn
        } else if narrowed {
            CertificationVerdict::Narrowed
        } else {
            CertificationVerdict::Certified
        };

        FamilyDecision {
            claimed_tier: claimed,
            effective_tier: effective,
            verdict,
            certified: verdict == CertificationVerdict::Certified,
            narrowed,
            reasons: order_reasons(reasons),
            stale_or_missing_dimensions: stale_or_missing.into_iter().collect(),
        }
    }

    /// Whether a narrowed family keeps an actionable rerun path. A certified family needs
    /// none; any narrowed or withdrawn family must declare a rerun ref.
    pub fn floored_keeps_fallback(&self, effective: QualificationTier) -> bool {
        if effective == QualificationTier::Stable {
            return true;
        }
        opt_present(&self.lineage.rerun_ref)
    }

    /// Whether any rendering reads wider than the effective tier.
    pub fn surface_overclaims(&self, effective: QualificationTier) -> bool {
        self.renderings
            .iter()
            .any(|r| tier_overclaims(effective, r.rendered_tier))
    }

    /// A reviewer-facing, non-generic narrow label, or `None` when not narrowed.
    pub fn narrowed_label(&self, decision: &FamilyDecision) -> Option<String> {
        if !decision.narrowed {
            return None;
        }
        let trigger = decision.downgrade_trigger()?;
        Some(format!(
            "{} narrowed to {}: {}",
            self.family.as_str(),
            decision.effective_tier.as_str(),
            trigger.phrase()
        ))
    }

    /// Structural (non-narrowing) violations for this family.
    fn structural_violations(&self, violations: &mut Vec<M5FormFamilyCertificationViolation>) {
        use M5FormFamilyCertificationViolation as V;
        if self.family_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.lineage.evidence_run_ref.trim().is_empty()
        {
            violations.push(V::FamilyMissingIdentity);
        }
        if self.renderings.is_empty() {
            violations.push(V::FamilyMissingRendering);
        }
        for r in &self.renderings {
            if r.source_family_ref.trim().is_empty() {
                violations.push(V::RenderingMissingSourceRef);
            }
        }
        for (dimension, lane) in REQUIRED_PROOF_PAIRS {
            match self.cell(dimension, lane) {
                None => violations.push(V::RequiredProofPairMissing),
                Some(cell) => {
                    let requires_ref = cell.state.has_capture();
                    if requires_ref != opt_present(&cell.proof_ref)
                        || requires_ref != opt_present(&cell.captured_at)
                    {
                        violations.push(V::EvidenceRefIncoherent);
                    }
                    if label_is_generic(&cell.proof_label) {
                        violations.push(V::EvidenceLabelGeneric);
                    }
                }
            }
        }
    }
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for [`M5FormFamilyCertificationSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FormFamilyCertificationSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Certification freshness window.
    pub certification_freshness: CertificationFreshness,
    /// Per-family certification rows.
    pub families: Vec<FamilyRecord>,
}

/// Export-safe M5 form-family certification set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FormFamilyCertificationSetPacket {
    /// Record kind; must equal [`M5_FORM_FAMILY_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FORM_FAMILY_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_FORM_FAMILY_CERTIFICATION_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Certification freshness window.
    pub certification_freshness: CertificationFreshness,
    /// Per-family certification rows.
    pub families: Vec<FamilyRecord>,
}

/// The distribution of effective verdicts across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerdictDistribution {
    /// Families effective at [`CertificationVerdict::Certified`].
    pub certified: usize,
    /// Families effective at [`CertificationVerdict::Narrowed`].
    pub narrowed: usize,
    /// Families effective at [`CertificationVerdict::Withdrawn`].
    pub withdrawn: usize,
}

impl VerdictDistribution {
    /// The overall promotion decision token for the set: `certified` only when every family
    /// is certified, `withdrawn` when any family is withdrawn, else `narrowed`.
    pub const fn overall_decision_token(self) -> &'static str {
        if self.withdrawn > 0 {
            "withdrawn"
        } else if self.narrowed > 0 {
            "narrowed"
        } else {
            "certified"
        }
    }
}

impl M5FormFamilyCertificationSetPacket {
    /// Builds a certification set packet, sealing the record-kind, schema, and taxonomy
    /// version constants.
    pub fn new(input: M5FormFamilyCertificationSetInput) -> Self {
        Self {
            record_kind: M5_FORM_FAMILY_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_FORM_FAMILY_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_FORM_FAMILY_CERTIFICATION_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            certification_freshness: input.certification_freshness,
            families: input.families,
        }
    }

    /// Whether the certification window has elapsed by `as_of`.
    pub fn freshness_stale_at(&self, as_of: &str) -> bool {
        if !self.certification_freshness.auto_downgrade_on_stale {
            return false;
        }
        let last = parse_rfc3339_to_epoch_seconds(
            &self.certification_freshness.last_certification_refresh,
        );
        let now = parse_rfc3339_to_epoch_seconds(as_of);
        match (last, now) {
            (Some(last), Some(now)) => {
                now - last
                    > i64::from(
                        self.certification_freshness
                            .certification_freshness_slo_hours,
                    ) * 3600
            }
            _ => false,
        }
    }

    /// Whether the window has elapsed by the packet's own `as_of`.
    pub fn stale_window(&self) -> bool {
        self.freshness_stale_at(&self.as_of)
    }

    /// Re-derive the decision for every family, paired with its id.
    pub fn decisions(&self) -> Vec<(String, FamilyDecision)> {
        let stale_window = self.stale_window();
        self.families
            .iter()
            .map(|f| (f.family_id.clone(), f.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective verdicts.
    pub fn verdict_distribution(&self) -> VerdictDistribution {
        let stale_window = self.stale_window();
        let mut dist = VerdictDistribution {
            certified: 0,
            narrowed: 0,
            withdrawn: 0,
        };
        for f in &self.families {
            match f.narrow(stale_window).verdict {
                CertificationVerdict::Certified => dist.certified += 1,
                CertificationVerdict::Narrowed => dist.narrowed += 1,
                CertificationVerdict::Withdrawn => dist.withdrawn += 1,
            }
        }
        dist
    }

    /// Count of families whose effective tier ranks below their claimed tier.
    pub fn narrowed_family_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.families
            .iter()
            .filter(|f| f.narrow(stale_window).narrowed)
            .count()
    }

    /// Form families represented by some row.
    pub fn represented_families(&self) -> BTreeSet<FormFamily> {
        self.families.iter().map(|f| f.family).collect()
    }

    /// Proof dimensions represented by some evidence cell.
    pub fn represented_dimensions(&self) -> BTreeSet<ProofDimension> {
        self.families
            .iter()
            .flat_map(|f| f.evidence.iter().map(|c| c.dimension))
            .collect()
    }

    /// Proof lanes represented by some evidence cell.
    pub fn represented_lanes(&self) -> BTreeSet<ProofLane> {
        self.families
            .iter()
            .flat_map(|f| f.evidence.iter().map(|c| c.source_lane))
            .collect()
    }

    /// Consumer surfaces represented by some rendering.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<ConsumerSurface> {
        self.families
            .iter()
            .flat_map(|f| f.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Validate the form-family certification invariants.
    pub fn validate(&self) -> Vec<M5FormFamilyCertificationViolation> {
        use M5FormFamilyCertificationViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_FORM_FAMILY_CERTIFICATION_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_FORM_FAMILY_CERTIFICATION_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_FORM_FAMILY_CERTIFICATION_TAXONOMY_VERSION {
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
        if self
            .certification_freshness
            .certification_freshness_slo_hours
            == 0
            || self
                .certification_freshness
                .last_certification_refresh
                .trim()
                .is_empty()
        {
            violations.push(V::EvidenceFreshnessIncomplete);
        }
        if self.families.is_empty() {
            violations.push(V::EmptyFamilies);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for f in &self.families {
            if !seen.insert(f.family_id.as_str()) {
                violations.push(V::DuplicateFamilyId);
            }
        }

        if FormFamily::ALL
            .iter()
            .any(|x| !self.represented_families().contains(x))
        {
            violations.push(V::FormFamilyMissing);
        }
        if ProofDimension::ALL
            .iter()
            .any(|d| !self.represented_dimensions().contains(d))
        {
            violations.push(V::ProofDimensionMissing);
        }
        if ProofLane::ALL
            .iter()
            .any(|l| !self.represented_lanes().contains(l))
        {
            violations.push(V::ProofLaneMissing);
        }
        if ConsumerSurface::ALL
            .iter()
            .any(|c| !self.represented_consumer_surfaces().contains(c))
        {
            violations.push(V::ConsumerSurfaceMissing);
        }

        let stale_window = self.stale_window();
        let slo = self
            .certification_freshness
            .certification_freshness_slo_hours;
        let mut demonstrates_narrowing = false;
        for f in &self.families {
            f.structural_violations(&mut violations);
            for cell in &f.evidence {
                if cell.freshness_overclaims(&self.as_of, slo) {
                    violations.push(V::EvidenceFreshnessOverclaim);
                }
            }
            let decision = f.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || f.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedFamilyMissingLabelOrTrigger);
                }
            }
            if !f.floored_keeps_fallback(decision.effective_tier) {
                violations.push(V::FlooredFamilyLosesFallback);
            }
            if f.surface_overclaims(decision.effective_tier) {
                violations.push(V::RenderingSurfaceOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedFamilyCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("form-family certification packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        let mut out: Vec<M5FormFamilyCertificationViolation> = Vec::new();
        for v in violations {
            if !out.contains(&v) {
                out.push(v);
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
        serde_json::to_string_pretty(self).expect("form-family certification packet serializes")
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_report(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.verdict_distribution();
        let mut out = String::new();
        out.push_str("# M5 Mutation-Capable Form Family Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Families: {}\n", self.families.len()));
        out.push_str(&format!(
            "- Overall decision: `{}`\n",
            dist.overall_decision_token()
        ));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} withdrawn\n\n",
            dist.certified, dist.narrowed, dist.withdrawn
        ));

        out.push_str("| Family | Claimed | Effective | Verdict | Stale/missing dimensions |\n");
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for f in &self.families {
            let decision = f.narrow(stale_window);
            let dims = decision
                .stale_or_missing_dimensions
                .iter()
                .map(|d| d.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                f.family.as_str(),
                decision.claimed_tier.as_str(),
                decision.effective_tier.as_str(),
                decision.verdict.as_str(),
                if dims.is_empty() { "—" } else { &dims },
            ));
        }

        out.push('\n');
        for f in &self.families {
            let decision = f.narrow(stale_window);
            if let Some(label) = f.narrowed_label(&decision) {
                out.push_str(&format!("- {}: {}\n", f.family_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or validate.
#[derive(Debug)]
pub enum M5FormFamilyCertificationArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5FormFamilyCertificationViolation>),
}

impl fmt::Display for M5FormFamilyCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(f, "support export parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "support export failed validation: {violations:?}")
            }
        }
    }
}

impl Error for M5FormFamilyCertificationArtifactError {}

/// A form-family certification packet validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FormFamilyCertificationViolation {
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
    /// The certification freshness window is incomplete.
    EvidenceFreshnessIncomplete,
    /// The set has no families.
    EmptyFamilies,
    /// Two families share a family id.
    DuplicateFamilyId,
    /// A form family is unrepresented.
    FormFamilyMissing,
    /// A proof dimension is unrepresented.
    ProofDimensionMissing,
    /// A proof lane is unrepresented.
    ProofLaneMissing,
    /// A consumer surface is unrepresented.
    ConsumerSurfaceMissing,
    /// A family lacks a required identity field.
    FamilyMissingIdentity,
    /// A family has no renderings.
    FamilyMissingRendering,
    /// A rendering names no source family ref.
    RenderingMissingSourceRef,
    /// A required `(dimension, lane)` proof pair has no evidence cell.
    RequiredProofPairMissing,
    /// An evidence ref/capture is incoherent with the cell state.
    EvidenceRefIncoherent,
    /// An evidence cell carries a generic, non-informative label.
    EvidenceLabelGeneric,
    /// A `current` cell claims fresher than its capture allows.
    EvidenceFreshnessOverclaim,
    /// A narrowed family lacks a non-generic label or a downgrade trigger.
    NarrowedFamilyMissingLabelOrTrigger,
    /// A narrowed family loses its actionable rerun fallback.
    FlooredFamilyLosesFallback,
    /// A rendering surface renders wider than the effective tier.
    RenderingSurfaceOverclaims,
    /// No family demonstrates the auto-narrowing rule.
    DowngradedFamilyCaseMissing,
    /// Raw boundary material crossed the export.
    RawBoundaryMaterialInExport,
}

impl M5FormFamilyCertificationViolation {
    /// Stable token recorded in diagnostics and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyFamilies => "empty_families",
            Self::DuplicateFamilyId => "duplicate_family_id",
            Self::FormFamilyMissing => "form_family_missing",
            Self::ProofDimensionMissing => "proof_dimension_missing",
            Self::ProofLaneMissing => "proof_lane_missing",
            Self::ConsumerSurfaceMissing => "consumer_surface_missing",
            Self::FamilyMissingIdentity => "family_missing_identity",
            Self::FamilyMissingRendering => "family_missing_rendering",
            Self::RenderingMissingSourceRef => "rendering_missing_source_ref",
            Self::RequiredProofPairMissing => "required_proof_pair_missing",
            Self::EvidenceRefIncoherent => "evidence_ref_incoherent",
            Self::EvidenceLabelGeneric => "evidence_label_generic",
            Self::EvidenceFreshnessOverclaim => "evidence_freshness_overclaim",
            Self::NarrowedFamilyMissingLabelOrTrigger => "narrowed_family_missing_label_or_trigger",
            Self::FlooredFamilyLosesFallback => "floored_family_loses_fallback",
            Self::RenderingSurfaceOverclaims => "rendering_surface_overclaims",
            Self::DowngradedFamilyCaseMissing => "downgraded_family_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// --------------------------------------------------------------------------- //
// Canonical artifact loader.
// --------------------------------------------------------------------------- //

/// Loads and validates the checked-in canonical support export.
///
/// This is the canonical entry point downstream About / help / service-health /
/// compatibility surfaces and the release and support packets use to ingest the frozen
/// certification state instead of re-deciding which M5 form families are certified.
///
/// # Errors
///
/// Returns [`M5FormFamilyCertificationArtifactError`] when the artifact cannot be parsed or
/// fails validation.
pub fn current_m5_form_family_certification_set(
) -> Result<M5FormFamilyCertificationSetPacket, M5FormFamilyCertificationArtifactError> {
    let packet: M5FormFamilyCertificationSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ux/m5-form-family-certification/support_export.json"
    )))
    .map_err(M5FormFamilyCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FormFamilyCertificationArtifactError::Validation(
            violations,
        ))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded certification set: the in-crate source of truth the checked-in
/// support export and report are regenerated from.
pub fn seeded_m5_form_family_certification_set() -> M5FormFamilyCertificationSetPacket {
    M5FormFamilyCertificationSetPacket::new(M5FormFamilyCertificationSetInput {
        packet_id: M5_FORM_FAMILY_CERTIFICATION_PACKET_ID.to_owned(),
        label:
            "M5 mutation-capable form family certification across provider, admin, request, package, settings, import, and project lanes"
                .to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        certification_freshness: CertificationFreshness {
            certification_freshness_slo_hours: 168,
            last_certification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        families: seed_families(),
    })
}

/// A full set of `current` evidence cells (one per required proof pair) for a family.
fn current_evidence(family: FormFamily) -> Vec<EvidenceCell> {
    REQUIRED_PROOF_PAIRS
        .iter()
        .map(|&(dimension, lane)| EvidenceCell {
            dimension,
            source_lane: lane,
            state: EvidenceState::Current,
            proof_ref: Some(lane.support_export_ref().to_owned()),
            captured_at: Some(SEED_AS_OF.to_owned()),
            proof_label: format!("{} {} proof current", family.as_str(), dimension.as_str()),
        })
        .collect()
}

/// Renderings that show `tier` across every consumer surface, sourced from `family_id`.
fn faithful_renderings(family_id: &str, tier: QualificationTier) -> Vec<FamilyRendering> {
    ConsumerSurface::ALL
        .iter()
        .map(|&surface| FamilyRendering {
            surface,
            rendered_tier: tier,
            source_family_ref: family_id.to_owned(),
        })
        .collect()
}

/// A certified family with all-current evidence.
fn certified_family(family: FormFamily, family_id: &str, label: &str) -> FamilyRecord {
    FamilyRecord {
        family_id: family_id.to_owned(),
        family,
        claimed_tier: QualificationTier::Stable,
        label_summary: label.to_owned(),
        lineage: FamilyLineage {
            evidence_run_ref: format!("evidence:{}:0001", family.as_str()),
            rerun_ref: Some(format!("rerun:{}:certify", family.as_str())),
        },
        evidence: current_evidence(family),
        renderings: faithful_renderings(family_id, QualificationTier::Stable),
    }
}

/// The seven canonical family rows: six certified, one narrowed at baseline (its parameter
/// provenance proof has aged past the freshness window).
fn seed_families() -> Vec<FamilyRecord> {
    let provider = certified_family(
        FormFamily::ProviderConnect,
        "family:provider-connect:0001",
        "Provider connect and account-mapping forms",
    );
    let admin = certified_family(
        FormFamily::AdminSourceManagement,
        "family:admin-source-management:0001",
        "Admin source-management and batch-review sheets",
    );

    // Request-workspace: provenance proof has gone stale, so the family narrows to beta and
    // keeps its rerun path. Its consumer surfaces faithfully render beta, not stable.
    let mut request = certified_family(
        FormFamily::RequestWorkspace,
        "family:request-workspace:0001",
        "Request-workspace replay and mutation forms",
    );
    for cell in &mut request.evidence {
        if cell.dimension == ProofDimension::ParameterProvenance {
            cell.state = EvidenceState::Stale;
            cell.captured_at = Some(SEED_STALE_CAPTURED_AT.to_owned());
            cell.proof_label =
                "request_workspace parameter_provenance proof stale past window".to_owned();
        }
    }
    request.renderings =
        faithful_renderings("family:request-workspace:0001", QualificationTier::Beta);

    let package = certified_family(
        FormFamily::PackageInstallReview,
        "family:package-install-review:0001",
        "Package install, update, and remove review sheets",
    );
    let settings = certified_family(
        FormFamily::SettingsConfigEditor,
        "family:settings-config-editor:0001",
        "Settings and structured config editors",
    );
    let import = certified_family(
        FormFamily::ImportMigrationCenter,
        "family:import-migration-center:0001",
        "Import and migration-center restore reviews",
    );
    let projects = certified_family(
        FormFamily::GeneratedProjectBootstrap,
        "family:generated-project-bootstrap:0001",
        "Generated-project and bootstrap wizards",
    );

    vec![
        provider, admin, request, package, settings, import, projects,
    ]
}
