//! M5 docs-authoring certification report across claimed docs/browser profiles.
//!
//! Where the frozen docs-authoring matrix locks the canonical depth for the five
//! docs-authoring surfaces — the Markdown authoring workspace, the CommonMark
//! safe-preview baseline, docs-maintenance suggestions, docs validation, and docs
//! evidence handoff — this module *certifies* that whole authoring stack against
//! every claimed deployment profile (desktop, mirrored, cached, pinned-pack,
//! extension-owned, and browser-handoff). Each [`DocsAuthoringProfileRow`] binds
//! one profile to the surfaces it covers, the three certification gates it must
//! pass — source/version/freshness truth, safe rendered-preview boundaries, and
//! export/support parity — and an auto-derived qualification class and verdict.
//!
//! The certification is *fail-closed and self-narrowing*: a profile that drops a
//! gate or lets its proof go stale is automatically narrowed below its claimed
//! class, and a profile whose rendered preview lacks safe capability boundaries is
//! blocked from promotion. The report never hides a narrowed or blocked profile;
//! it labels it. The companions release, support, diagnostics, and Help/About
//! tooling ingest are:
//!
//! - a [`DocsAuthoringCertIndex`] rolling certified / narrowed / blocked profiles
//!   into one canonical summary,
//! - a [`CertCompatibilityReport`] proving every profile stays compatible with —
//!   and no greener than — the frozen docs-authoring matrix,
//! - a [`CertDowngradeRule`] set release/support tooling auto-enforces,
//! - a derived [`WaiverAndDowngradeLog`] recording standing class caps and any
//!   automatic downgrade currently in effect.
//!
//! The report is canonical for claimed M5 docs-authoring support: no profile may
//! stay greener than it. It references upstream schemas and support exports by id
//! rather than embedding content. Raw document bodies, raw source files, rendered
//! HTML, raw provider payloads, credentials, and live vendor-doc snapshots stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/docs/m5-docs-authoring-cert-report.schema.json`](../../../../schemas/docs/m5-docs-authoring-cert-report.schema.json).
//! The contract doc is
//! [`docs/m5/docs-authoring-certification.md`](../../../../docs/m5/docs-authoring-certification.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/certification-corpus/`](../../../../fixtures/docs/m5/certification-corpus/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    DOCS_EVIDENCE_HANDOFF_ARTIFACT_REF, DOCS_EVIDENCE_HANDOFF_SCHEMA_REF,
    DOCS_SUGGESTION_PANEL_ARTIFACT_REF, DOCS_SUGGESTION_PANEL_SCHEMA_REF,
    DOCS_VALIDATION_REPORT_ARTIFACT_REF, DOCS_VALIDATION_REPORT_SCHEMA_REF,
    M5_AUTHORING_MATRIX_ARTIFACT_REF, M5_AUTHORING_MATRIX_SCHEMA_REF,
    M5_AUTHORING_MATRIX_SCHEMA_VERSION, MARKDOWN_WORKSPACE_ARTIFACT_REF,
    MARKDOWN_WORKSPACE_SCHEMA_REF, RELEASE_DOCS_MAINTENANCE_SCHEMA_REF,
    RENDERED_PREVIEW_BOUNDARY_ARTIFACT_REF, RENDERED_PREVIEW_BOUNDARY_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`DocsAuthoringCertReport`].
pub const DOCS_AUTHORING_CERT_RECORD_KIND: &str = "m5_docs_authoring_certification_report";

/// Stable record-kind tag carried by [`WaiverAndDowngradeLog`].
pub const DOCS_AUTHORING_WAIVER_LOG_RECORD_KIND: &str =
    "m5_docs_authoring_waiver_and_downgrade_log";

/// Schema version for docs-authoring certification records.
pub const DOCS_AUTHORING_CERT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_AUTHORING_CERT_SCHEMA_REF: &str =
    "schemas/docs/m5-docs-authoring-cert-report.schema.json";

/// Repo-relative path of the certification contract doc.
pub const DOCS_AUTHORING_CERT_DOC_REF: &str = "docs/m5/docs-authoring-certification.md";

/// Repo-relative path of the protected certification-corpus fixture directory.
pub const DOCS_AUTHORING_CERT_FIXTURE_DIR: &str = "fixtures/docs/m5/certification-corpus";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_AUTHORING_CERT_ARTIFACT_REF: &str =
    "artifacts/m5/docs-authoring/certification-report/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_AUTHORING_CERT_SUMMARY_REF: &str =
    "artifacts/m5/docs-authoring/certification-report.md";

/// Repo-relative path of the checked waiver-and-downgrade log artifact.
pub const DOCS_AUTHORING_WAIVER_LOG_REF: &str =
    "artifacts/m5/docs-authoring/waiver-and-downgrade-log/waiver_and_downgrade_log.json";

/// One claimed M5 docs/browser deployment profile certification runs over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsAuthoringProfile {
    /// Local desktop docs authoring with first-party packs.
    Desktop,
    /// Mirror-aware authoring backed by a pinned, signed mirror.
    Mirrored,
    /// Cached / last-known-good authoring while the source is offline.
    Cached,
    /// Pinned docs-pack authoring against a frozen pack revision.
    PinnedPack,
    /// Extension-owned docs surface running in a less-trusted host.
    ExtensionOwned,
    /// Browser-handoff companion docs editing returning to the IDE.
    BrowserHandoff,
}

impl DocsAuthoringProfile {
    /// Every profile, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Desktop,
        Self::Mirrored,
        Self::Cached,
        Self::PinnedPack,
        Self::ExtensionOwned,
        Self::BrowserHandoff,
    ];

    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Mirrored => "mirrored",
            Self::Cached => "cached",
            Self::PinnedPack => "pinned_pack",
            Self::ExtensionOwned => "extension_owned",
            Self::BrowserHandoff => "browser_handoff",
        }
    }
}

/// One docs-authoring surface a profile must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsAuthoringCertSurface {
    /// Governed Markdown authoring workspace.
    MarkdownAuthoringWorkspace,
    /// CommonMark safe-preview baseline with capability boundaries.
    #[serde(rename = "commonmark_preview")]
    CommonMarkPreview,
    /// Diff-first docs-maintenance and stale-example suggestions.
    DocsMaintenanceSuggestions,
    /// Typed example/link validation reports.
    DocsValidation,
    /// Evidence handoff binding prose changes to code/schema/release truth.
    DocsEvidenceHandoff,
}

impl DocsAuthoringCertSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MarkdownAuthoringWorkspace,
        Self::CommonMarkPreview,
        Self::DocsMaintenanceSuggestions,
        Self::DocsValidation,
        Self::DocsEvidenceHandoff,
    ];

    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarkdownAuthoringWorkspace => "markdown_authoring_workspace",
            Self::CommonMarkPreview => "commonmark_preview",
            Self::DocsMaintenanceSuggestions => "docs_maintenance_suggestions",
            Self::DocsValidation => "docs_validation",
            Self::DocsEvidenceHandoff => "docs_evidence_handoff",
        }
    }

    /// Canonical schema ref for the certified surface.
    pub const fn schema_ref(self) -> &'static str {
        match self {
            Self::MarkdownAuthoringWorkspace => MARKDOWN_WORKSPACE_SCHEMA_REF,
            Self::CommonMarkPreview => RENDERED_PREVIEW_BOUNDARY_SCHEMA_REF,
            Self::DocsMaintenanceSuggestions => DOCS_SUGGESTION_PANEL_SCHEMA_REF,
            Self::DocsValidation => DOCS_VALIDATION_REPORT_SCHEMA_REF,
            Self::DocsEvidenceHandoff => DOCS_EVIDENCE_HANDOFF_SCHEMA_REF,
        }
    }

    /// Canonical support-export ref for the certified surface.
    pub const fn artifact_ref(self) -> &'static str {
        match self {
            Self::MarkdownAuthoringWorkspace => MARKDOWN_WORKSPACE_ARTIFACT_REF,
            Self::CommonMarkPreview => RENDERED_PREVIEW_BOUNDARY_ARTIFACT_REF,
            Self::DocsMaintenanceSuggestions => DOCS_SUGGESTION_PANEL_ARTIFACT_REF,
            Self::DocsValidation => DOCS_VALIDATION_REPORT_ARTIFACT_REF,
            Self::DocsEvidenceHandoff => DOCS_EVIDENCE_HANDOFF_ARTIFACT_REF,
        }
    }
}

/// Qualification class a profile is certified at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertQualificationClass {
    /// Certified for the Stable claim.
    Stable,
    /// Certified at Beta.
    Beta,
    /// Certified at Preview.
    Preview,
    /// Experimental; not claimed.
    Experimental,
    /// Unavailable on this build.
    Unavailable,
    /// Held pending evidence or upstream resolution.
    Held,
}

impl CertQualificationClass {
    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the class carries a publicly claimable promotion (Stable or Beta).
    pub const fn is_promoted(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }

    /// The class one rank below this one, used by automatic narrowing.
    pub const fn narrowed(self) -> Self {
        match self {
            Self::Stable => Self::Beta,
            Self::Beta => Self::Preview,
            Self::Preview => Self::Experimental,
            Self::Experimental => Self::Held,
            Self::Unavailable => Self::Unavailable,
            Self::Held => Self::Held,
        }
    }
}

/// Certification verdict recorded for a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertVerdict {
    /// Profile is certified at its claimed qualification with current proof.
    Certified,
    /// Profile was narrowed to a lower qualification to match its evidence.
    NarrowedToQualified,
    /// Profile is held pending evidence or upstream graduation.
    HeldPendingEvidence,
    /// Profile is blocked from promotion because it is underqualified.
    BlockedUnderqualified,
}

impl CertVerdict {
    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedToQualified => "narrowed_to_qualified",
            Self::HeldPendingEvidence => "held_pending_evidence",
            Self::BlockedUnderqualified => "blocked_underqualified",
        }
    }

    /// Whether the verdict allows the profile to keep a promoted public claim.
    pub const fn permits_promotion(self) -> bool {
        matches!(self, Self::Certified | Self::NarrowedToQualified)
    }
}

/// Proof-freshness state derived from proof age against the freshness window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertFreshnessState {
    /// Proof is within its freshness window.
    Current,
    /// Proof has aged past its freshness window.
    Stale,
}

impl CertFreshnessState {
    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
        }
    }
}

/// Downgrade trigger that can narrow a profile below its claimed class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Pinned, signed mirror is offline or unavailable.
    MirrorOffline,
    /// Source version no longer matches the indexed/pinned version.
    SourceVersionMismatch,
    /// Freshness window for the docs source expired.
    FreshnessExpired,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified authoring/handoff boundary.
    ScopeExpansionUnqualified,
    /// A rendered preview encountered unsafe content and was blocked.
    UnsafePreviewBlocked,
    /// Support/export parity was lost for the authoring stack.
    MissingExportParity,
    /// An upstream dependency surface narrowed.
    UpstreamDependencyNarrowed,
}

impl CertDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::MirrorOffline,
        Self::SourceVersionMismatch,
        Self::FreshnessExpired,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UnsafePreviewBlocked,
        Self::MissingExportParity,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::MirrorOffline => "mirror_offline",
            Self::SourceVersionMismatch => "source_version_mismatch",
            Self::FreshnessExpired => "freshness_expired",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UnsafePreviewBlocked => "unsafe_preview_blocked",
            Self::MissingExportParity => "missing_export_parity",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Automatic narrowing action a downgrade rule applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertDowngradeAction {
    /// Narrow the profile to Beta.
    NarrowToBeta,
    /// Narrow the profile to Preview.
    NarrowToPreview,
    /// Hold the profile pending evidence.
    Hold,
    /// Block promotion of the profile.
    BlockPromotion,
}

impl CertDowngradeAction {
    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowToBeta => "narrow_to_beta",
            Self::NarrowToPreview => "narrow_to_preview",
            Self::Hold => "hold",
            Self::BlockPromotion => "block_promotion",
        }
    }
}

/// Kind of waiver-and-downgrade-log entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaiverLogEntryKind {
    /// A standing class cap holding a profile below Stable by governance choice.
    ClassCap,
    /// An automatic downgrade applied because a certification gate failed.
    AutoDowngrade,
}

impl WaiverLogEntryKind {
    /// Stable token recorded in the log.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClassCap => "class_cap",
            Self::AutoDowngrade => "auto_downgrade",
        }
    }
}

/// One docs-authoring surface covered (or not) by a profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSurfaceCoverage {
    /// Covered docs-authoring surface.
    pub surface: DocsAuthoringCertSurface,
    /// Canonical schema ref for the surface.
    pub schema_ref: String,
    /// Canonical support-export ref for the surface.
    pub artifact_ref: String,
    /// True when the profile certifies this surface against current proof.
    pub covered: bool,
}

impl ProfileSurfaceCoverage {
    /// Builds a covered surface entry from the surface's canonical refs.
    pub fn covered(surface: DocsAuthoringCertSurface) -> Self {
        Self {
            surface,
            schema_ref: surface.schema_ref().to_owned(),
            artifact_ref: surface.artifact_ref().to_owned(),
            covered: true,
        }
    }
}

/// One certified profile row in the docs-authoring certification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsAuthoringProfileRow {
    /// Certified deployment profile.
    pub profile: DocsAuthoringProfile,
    /// Qualification this profile would claim when every gate passes.
    pub claimed_qualification: CertQualificationClass,
    /// Auto-derived qualification class after gate and freshness evaluation.
    pub qualification: CertQualificationClass,
    /// Auto-derived certification verdict.
    pub verdict: CertVerdict,
    /// Auto-derived proof-freshness state.
    pub freshness_state: CertFreshnessState,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Per-surface coverage for the docs-authoring stack on this profile.
    pub surface_coverage: Vec<ProfileSurfaceCoverage>,
    /// Gate 1: source / version / freshness truth stays visible on this profile.
    pub source_version_freshness_truth: bool,
    /// Gate 2: rendered preview keeps safe capability boundaries on this profile.
    pub safe_rendered_preview_boundaries: bool,
    /// Gate 3: support/export parity holds for the authoring stack on this profile.
    pub export_support_parity: bool,
    /// Age of the certification proof in hours.
    pub proof_age_hours: u32,
    /// Freshness window the proof must stay within, in hours.
    pub freshness_window_hours: u32,
    /// Evidence packet refs backing this certification.
    pub evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that can narrow this profile.
    pub downgrade_triggers: Vec<CertDowngradeTrigger>,
    /// True when the certified claim is not greener than the frozen matrix.
    pub not_greener_than_matrix: bool,
    /// Trigger that produced an automatic downgrade, when one applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_trigger: Option<CertDowngradeTrigger>,
    /// Human-readable reason a profile was narrowed or blocked, when it was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_reason: Option<String>,
    /// Trigger documenting a standing class cap below Stable, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_cap_trigger: Option<CertDowngradeTrigger>,
    /// Rationale for a standing class cap below Stable, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_cap_rationale: Option<String>,
}

impl DocsAuthoringProfileRow {
    /// True when every docs-authoring surface is present and covered.
    pub fn all_surfaces_covered(&self) -> bool {
        let present: BTreeSet<DocsAuthoringCertSurface> = self
            .surface_coverage
            .iter()
            .filter(|entry| entry.covered)
            .map(|entry| entry.surface)
            .collect();
        DocsAuthoringCertSurface::ALL
            .iter()
            .all(|surface| present.contains(surface))
    }

    /// Whether this row carries a promoted, promotion-permitting certification.
    pub fn is_promoted_and_certified(&self) -> bool {
        self.qualification.is_promoted() && self.verdict.permits_promotion()
    }
}

/// Constructor input for [`certify_profile_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileRowInput {
    /// Certified deployment profile.
    pub profile: DocsAuthoringProfile,
    /// Qualification this profile would claim when every gate passes.
    pub claimed_qualification: CertQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Per-surface coverage for the docs-authoring stack on this profile.
    pub surface_coverage: Vec<ProfileSurfaceCoverage>,
    /// Gate 1: source / version / freshness truth stays visible.
    pub source_version_freshness_truth: bool,
    /// Gate 2: rendered preview keeps safe capability boundaries.
    pub safe_rendered_preview_boundaries: bool,
    /// Gate 3: support/export parity holds for the authoring stack.
    pub export_support_parity: bool,
    /// Age of the certification proof in hours.
    pub proof_age_hours: u32,
    /// Freshness window the proof must stay within, in hours.
    pub freshness_window_hours: u32,
    /// Evidence packet refs backing this certification.
    pub evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that can narrow this profile.
    pub downgrade_triggers: Vec<CertDowngradeTrigger>,
    /// Trigger documenting a standing class cap below Stable, when one applies.
    pub class_cap_trigger: Option<CertDowngradeTrigger>,
    /// Rationale for a standing class cap below Stable, when one applies.
    pub class_cap_rationale: Option<String>,
}

/// Derived outcome of evaluating one profile's certification gates.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ProfileOutcome {
    qualification: CertQualificationClass,
    verdict: CertVerdict,
    freshness_state: CertFreshnessState,
    narrowing_trigger: Option<CertDowngradeTrigger>,
    narrowed_reason: Option<String>,
}

/// Evaluates the certification gates for one profile and returns its outcome.
///
/// This is the single source of truth for automatic narrowing: a missing safe
/// rendered-preview boundary blocks promotion, and any other failed gate, missing
/// surface coverage, or stale proof narrows the profile one class below its claim.
fn evaluate_profile(
    claimed: CertQualificationClass,
    source_version_freshness_truth: bool,
    safe_rendered_preview_boundaries: bool,
    export_support_parity: bool,
    all_surfaces_covered: bool,
    proof_age_hours: u32,
    freshness_window_hours: u32,
) -> ProfileOutcome {
    let freshness_state = if proof_age_hours > freshness_window_hours {
        CertFreshnessState::Stale
    } else {
        CertFreshnessState::Current
    };

    if !safe_rendered_preview_boundaries {
        return ProfileOutcome {
            qualification: CertQualificationClass::Held,
            verdict: CertVerdict::BlockedUnderqualified,
            freshness_state,
            narrowing_trigger: Some(CertDowngradeTrigger::UnsafePreviewBlocked),
            narrowed_reason: Some(
                "rendered preview lacks safe capability boundaries; promotion is blocked until preview is sanitized, labeled, and escape-to-source is preserved"
                    .to_owned(),
            ),
        };
    }

    let mut reasons: Vec<&str> = Vec::new();
    let mut trigger: Option<CertDowngradeTrigger> = None;
    if !source_version_freshness_truth {
        reasons.push("missing source/version/freshness truth");
        trigger.get_or_insert(CertDowngradeTrigger::SourceVersionMismatch);
    }
    if !export_support_parity {
        reasons.push("missing export/support parity");
        trigger.get_or_insert(CertDowngradeTrigger::MissingExportParity);
    }
    if !all_surfaces_covered {
        reasons.push("incomplete docs-authoring surface coverage");
        trigger.get_or_insert(CertDowngradeTrigger::UpstreamDependencyNarrowed);
    }
    if freshness_state == CertFreshnessState::Stale {
        reasons.push("certification proof is stale");
        trigger.get_or_insert(CertDowngradeTrigger::ProofStale);
    }

    if reasons.is_empty() {
        ProfileOutcome {
            qualification: claimed,
            verdict: CertVerdict::Certified,
            freshness_state,
            narrowing_trigger: None,
            narrowed_reason: None,
        }
    } else {
        ProfileOutcome {
            qualification: claimed.narrowed(),
            verdict: CertVerdict::NarrowedToQualified,
            freshness_state,
            narrowing_trigger: trigger,
            narrowed_reason: Some(reasons.join("; ")),
        }
    }
}

/// Certifies one profile from stable-lane input, deriving its qualification.
pub fn certify_profile_row(input: ProfileRowInput) -> DocsAuthoringProfileRow {
    let all_surfaces_covered = {
        let present: BTreeSet<DocsAuthoringCertSurface> = input
            .surface_coverage
            .iter()
            .filter(|entry| entry.covered)
            .map(|entry| entry.surface)
            .collect();
        DocsAuthoringCertSurface::ALL
            .iter()
            .all(|surface| present.contains(surface))
    };
    let outcome = evaluate_profile(
        input.claimed_qualification,
        input.source_version_freshness_truth,
        input.safe_rendered_preview_boundaries,
        input.export_support_parity,
        all_surfaces_covered,
        input.proof_age_hours,
        input.freshness_window_hours,
    );
    DocsAuthoringProfileRow {
        profile: input.profile,
        claimed_qualification: input.claimed_qualification,
        qualification: outcome.qualification,
        verdict: outcome.verdict,
        freshness_state: outcome.freshness_state,
        scope_summary: input.scope_summary,
        surface_coverage: input.surface_coverage,
        source_version_freshness_truth: input.source_version_freshness_truth,
        safe_rendered_preview_boundaries: input.safe_rendered_preview_boundaries,
        export_support_parity: input.export_support_parity,
        proof_age_hours: input.proof_age_hours,
        freshness_window_hours: input.freshness_window_hours,
        evidence_packet_refs: input.evidence_packet_refs,
        downgrade_triggers: input.downgrade_triggers,
        not_greener_than_matrix: true,
        narrowing_trigger: outcome.narrowing_trigger,
        narrowed_reason: outcome.narrowed_reason,
        class_cap_trigger: input.class_cap_trigger,
        class_cap_rationale: input.class_cap_rationale,
    }
}

/// Roll-up index of certified, narrowed, and blocked profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsAuthoringCertIndex {
    /// Canonical ref of the certification report this index summarizes.
    pub report_ref: String,
    /// Every profile's proof is within its freshness window.
    pub all_profiles_current: bool,
    /// Every profile is certified at its claimed class.
    pub all_profiles_certified: bool,
    /// Profiles certified at their claimed class.
    pub certified_profiles: Vec<DocsAuthoringProfile>,
    /// Profiles narrowed below their claimed class but still promotion-permitting.
    pub narrowed_profiles: Vec<DocsAuthoringProfile>,
    /// Profiles blocked from promotion.
    pub blocked_profiles: Vec<DocsAuthoringProfile>,
    /// Docs-authoring surface tokens covered across the corpus.
    pub covered_surfaces: Vec<DocsAuthoringCertSurface>,
    /// Human-readable summary line.
    pub summary: String,
}

fn derive_certification_index(
    report_ref: &str,
    rows: &[DocsAuthoringProfileRow],
) -> DocsAuthoringCertIndex {
    let all_profiles_current = rows
        .iter()
        .all(|row| row.freshness_state == CertFreshnessState::Current);
    let all_profiles_certified =
        !rows.is_empty() && rows.iter().all(|row| row.verdict == CertVerdict::Certified);
    let certified_profiles = rows
        .iter()
        .filter(|row| row.verdict == CertVerdict::Certified)
        .map(|row| row.profile)
        .collect::<Vec<_>>();
    let narrowed_profiles = rows
        .iter()
        .filter(|row| row.verdict == CertVerdict::NarrowedToQualified)
        .map(|row| row.profile)
        .collect::<Vec<_>>();
    let blocked_profiles = rows
        .iter()
        .filter(|row| !row.verdict.permits_promotion())
        .map(|row| row.profile)
        .collect::<Vec<_>>();
    let covered_surfaces = {
        let set: BTreeSet<DocsAuthoringCertSurface> = rows
            .iter()
            .flat_map(|row| row.surface_coverage.iter())
            .filter(|entry| entry.covered)
            .map(|entry| entry.surface)
            .collect();
        set.into_iter().collect::<Vec<_>>()
    };
    let summary = format!(
        "{} profiles; certified={}, narrowed={}, blocked={}",
        rows.len(),
        certified_profiles.len(),
        narrowed_profiles.len(),
        blocked_profiles.len(),
    );
    DocsAuthoringCertIndex {
        report_ref: report_ref.to_owned(),
        all_profiles_current,
        all_profiles_certified,
        certified_profiles,
        narrowed_profiles,
        blocked_profiles,
        covered_surfaces,
        summary,
    }
}

/// Compatibility report binding the certification to the frozen authoring matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertCompatibilityReport {
    /// Ref of the frozen authoring-matrix support export this report certifies against.
    pub matrix_artifact_ref: String,
    /// Ref of the frozen authoring-matrix schema.
    pub matrix_schema_ref: String,
    /// Matrix schema version this certification is compatible with.
    pub matrix_schema_version: u32,
    /// Every claimed profile is present in the report.
    pub all_profiles_present: bool,
    /// No certified profile is greener than the frozen matrix.
    pub no_profile_greener_than_matrix: bool,
    /// Every covered surface references a checked-in schema and support export.
    pub every_surface_has_schema_and_artifact: bool,
    /// Downgrade rules are auto-enforced in release/support tooling.
    pub downgrade_rules_auto_enforced: bool,
}

/// One machine-readable downgrade rule consumed by release/support tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertDowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Trigger that fires the rule.
    pub trigger: CertDowngradeTrigger,
    /// Narrowing action the rule applies.
    pub action: CertDowngradeAction,
    /// Profiles the rule applies to.
    pub applies_to: Vec<DocsAuthoringProfile>,
    /// True when the rule is enforced automatically rather than by hand.
    pub auto_enforced: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertTrustReview {
    /// Docs stay source-canonical; rendered views remain safe and labeled.
    pub source_canonical_rendered_safe_and_labeled: bool,
    /// Rendered preview is sanitized and safe by default.
    pub rendered_preview_safe_by_default: bool,
    /// Rendered preview, diagram engines, and suggestions are never privileged execution paths.
    pub preview_not_privileged_execution_path: bool,
    /// Docs suggestions stay diff-first and are never silently auto-applied.
    pub suggestions_diff_first_never_auto_applied: bool,
    /// Source, version, and freshness truth stays visible.
    pub source_version_freshness_truth_visible: bool,
    /// Validation state is never silently upgraded to verified.
    pub validation_state_never_silently_upgraded: bool,
    /// Evidence handoff stays source-linked to code, schema, or release truth.
    pub evidence_handoff_source_linked: bool,
    /// Browser handoff never hides owner, origin, or boundary changes.
    pub handoff_never_hides_owner_origin_or_boundary: bool,
    /// Browser handoff never silently widens authority.
    pub handoff_never_silently_widens_authority: bool,
    /// Downgrade narrows the claim rather than hiding the profile.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified profiles automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
    /// No certified profile stays greener than this canonical report.
    pub no_profile_greener_than_report: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertConsumerProjection {
    /// Release gate ingests the certification report rather than cloning text.
    pub release_gate_consumes_report: bool,
    /// CLI / headless shows certification truth.
    pub cli_headless_shows_certification: bool,
    /// Support export shows certification truth.
    pub support_export_shows_certification: bool,
    /// Diagnostics shows certification truth.
    pub diagnostics_shows_certification: bool,
    /// Help / About shows certification truth.
    pub help_about_shows_certification: bool,
    /// Release center shows certification truth.
    pub release_center_shows_certification: bool,
    /// The M5 evidence index references this report.
    pub evidence_index_references_report: bool,
    /// Narrowed or blocked profiles are visibly labeled, not hidden.
    pub narrowed_profiles_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// One waiver-and-downgrade-log entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaiverLogEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Kind of log entry.
    pub kind: WaiverLogEntryKind,
    /// Profile the entry applies to.
    pub profile: DocsAuthoringProfile,
    /// Trigger that produced the cap or downgrade.
    pub trigger: CertDowngradeTrigger,
    /// Qualification the profile would otherwise hold.
    pub from_qualification: CertQualificationClass,
    /// Qualification the profile is held at after the cap or downgrade.
    pub to_qualification: CertQualificationClass,
    /// Human-readable rationale.
    pub rationale: String,
    /// True when the entry is enforced automatically rather than by hand.
    pub auto_enforced: bool,
    /// Expiry timestamp for a time-boxed waiver, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Derived waiver-and-downgrade log companion artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaiverAndDowngradeLog {
    /// Record kind; must equal [`DOCS_AUTHORING_WAIVER_LOG_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_AUTHORING_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Canonical ref of the certification report this log is derived from.
    pub report_ref: String,
    /// Log entries, in profile declaration order with caps before downgrades.
    pub entries: Vec<WaiverLogEntry>,
    /// Log mint timestamp.
    pub minted_at: String,
}

/// Derives the waiver-and-downgrade log from the report's profile rows.
fn derive_waiver_and_downgrade_log(
    rows: &[DocsAuthoringProfileRow],
    minted_at: &str,
) -> WaiverAndDowngradeLog {
    let mut entries = Vec::new();
    for row in rows {
        if row.claimed_qualification != CertQualificationClass::Stable {
            if let (Some(trigger), Some(rationale)) =
                (row.class_cap_trigger, row.class_cap_rationale.as_ref())
            {
                entries.push(WaiverLogEntry {
                    entry_id: format!("class-cap:{}", row.profile.as_str()),
                    kind: WaiverLogEntryKind::ClassCap,
                    profile: row.profile,
                    trigger,
                    from_qualification: CertQualificationClass::Stable,
                    to_qualification: row.claimed_qualification,
                    rationale: rationale.clone(),
                    auto_enforced: true,
                    expires_at: None,
                });
            }
        }
    }
    for row in rows {
        if row.verdict != CertVerdict::Certified {
            if let (Some(trigger), Some(reason)) =
                (row.narrowing_trigger, row.narrowed_reason.as_ref())
            {
                entries.push(WaiverLogEntry {
                    entry_id: format!("auto-downgrade:{}", row.profile.as_str()),
                    kind: WaiverLogEntryKind::AutoDowngrade,
                    profile: row.profile,
                    trigger,
                    from_qualification: row.claimed_qualification,
                    to_qualification: row.qualification,
                    rationale: reason.clone(),
                    auto_enforced: true,
                    expires_at: None,
                });
            }
        }
    }
    WaiverAndDowngradeLog {
        record_kind: DOCS_AUTHORING_WAIVER_LOG_RECORD_KIND.to_owned(),
        schema_version: DOCS_AUTHORING_CERT_SCHEMA_VERSION,
        report_ref: DOCS_AUTHORING_CERT_ARTIFACT_REF.to_owned(),
        entries,
        minted_at: minted_at.to_owned(),
    }
}

/// Constructor input for [`DocsAuthoringCertReport::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsAuthoringCertReportInput {
    /// Stable report id.
    pub report_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified profile rows.
    pub profile_rows: Vec<DocsAuthoringProfileRow>,
    /// Compatibility report.
    pub compatibility_report: CertCompatibilityReport,
    /// Downgrade rules.
    pub downgrade_rules: Vec<CertDowngradeRule>,
    /// Trust review block.
    pub trust_review: CertTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CertConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CertProofFreshness,
    /// Current known-limits notes published with the certification.
    pub known_limits: Vec<String>,
    /// Migration and evidence packet links published with the certification.
    pub migration_refs: Vec<String>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Report redaction class token.
    pub redaction_class_token: String,
    /// Report mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 docs-authoring certification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsAuthoringCertReport {
    /// Record kind; must equal [`DOCS_AUTHORING_CERT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_AUTHORING_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified profile rows.
    pub profile_rows: Vec<DocsAuthoringProfileRow>,
    /// Roll-up index of certified, narrowed, and blocked profiles.
    pub certification_index: DocsAuthoringCertIndex,
    /// Compatibility report.
    pub compatibility_report: CertCompatibilityReport,
    /// Downgrade rules.
    pub downgrade_rules: Vec<CertDowngradeRule>,
    /// Trust review block.
    pub trust_review: CertTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CertConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CertProofFreshness,
    /// Current known-limits notes published with the certification.
    pub known_limits: Vec<String>,
    /// Migration and evidence packet links published with the certification.
    pub migration_refs: Vec<String>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Report redaction class token.
    pub redaction_class_token: String,
    /// Report mint timestamp.
    pub minted_at: String,
}

impl DocsAuthoringCertReport {
    /// Builds a certification report from stable-lane input, deriving the index.
    pub fn new(input: DocsAuthoringCertReportInput) -> Self {
        let certification_index =
            derive_certification_index(DOCS_AUTHORING_CERT_ARTIFACT_REF, &input.profile_rows);
        Self {
            record_kind: DOCS_AUTHORING_CERT_RECORD_KIND.to_owned(),
            schema_version: DOCS_AUTHORING_CERT_SCHEMA_VERSION,
            report_id: input.report_id,
            certification_label: input.certification_label,
            profile_rows: input.profile_rows,
            certification_index,
            compatibility_report: input.compatibility_report,
            downgrade_rules: input.downgrade_rules,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            known_limits: input.known_limits,
            migration_refs: input.migration_refs,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Profiles whose certification narrows the claim but stays promotion-permitting.
    pub fn narrowed_profiles(&self) -> Vec<DocsAuthoringProfile> {
        self.profile_rows
            .iter()
            .filter(|row| row.verdict == CertVerdict::NarrowedToQualified)
            .map(|row| row.profile)
            .collect()
    }

    /// Profiles blocked from promotion.
    pub fn promotion_blockers(&self) -> Vec<DocsAuthoringProfile> {
        self.profile_rows
            .iter()
            .filter(|row| !row.verdict.permits_promotion())
            .map(|row| row.profile)
            .collect()
    }

    /// The derived waiver-and-downgrade log companion artifact.
    pub fn waiver_and_downgrade_log(&self) -> WaiverAndDowngradeLog {
        derive_waiver_and_downgrade_log(&self.profile_rows, &self.minted_at)
    }

    /// Validates the certification invariants.
    pub fn validate(&self) -> Vec<CertViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DOCS_AUTHORING_CERT_RECORD_KIND {
            violations.push(CertViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_AUTHORING_CERT_SCHEMA_VERSION {
            violations.push(CertViolation::WrongSchemaVersion);
        }
        if self.report_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CertViolation::MissingIdentity);
        }
        if self.known_limits.iter().any(|note| note.trim().is_empty())
            || self.known_limits.is_empty()
            || self.migration_refs.is_empty()
        {
            violations.push(CertViolation::KnownLimitsMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_profile_rows(self, &mut violations);
        validate_certification_index(self, &mut violations);
        validate_compatibility_report(self, &mut violations);
        validate_downgrade_rules(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("certification report serializes"),
        ) {
            violations.push(CertViolation::RawBoundaryMaterialInExport);
        }

        violations.sort_by_key(|violation| violation.as_str());
        violations.dedup();
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only report fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification report serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let certified = self
            .profile_rows
            .iter()
            .filter(|row| row.verdict == CertVerdict::Certified)
            .count();
        let mut out = String::new();
        out.push_str("# M5 Docs Authoring Certification\n\n");
        out.push_str(&format!("- Report: `{}`\n", self.report_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Profiles: {} ({} certified, {} narrowed, {} blocked)\n",
            self.profile_rows.len(),
            certified,
            self.narrowed_profiles().len(),
            self.promotion_blockers().len(),
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Profiles\n\n");
        for row in &self.profile_rows {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}` (freshness `{}`)\n",
                row.profile.as_str(),
                row.qualification.as_str(),
                row.verdict.as_str(),
                row.freshness_state.as_str(),
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            if let Some(reason) = &row.narrowed_reason {
                out.push_str(&format!("  - Narrowed: {reason}\n"));
            }
        }
        let blockers = self.promotion_blockers();
        if !blockers.is_empty() {
            out.push_str("\n## Promotion blockers\n\n");
            for profile in blockers {
                out.push_str(&format!("- `{}`\n", profile.as_str()));
            }
        }
        out.push_str("\n## Known limits\n\n");
        for note in &self.known_limits {
            out.push_str(&format!("- {note}\n"));
        }
        out
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum DocsAuthoringCertArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CertViolation>),
}

impl fmt::Display for DocsAuthoringCertArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "docs-authoring certification export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs-authoring certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsAuthoringCertArtifactError {}

/// Validation failures emitted by [`DocsAuthoringCertReport::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CertViolation {
    /// Report record kind is wrong.
    WrongRecordKind,
    /// Report schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required profile is missing from the report.
    RequiredProfileMissing,
    /// A profile row is incomplete.
    ProfileRowIncomplete,
    /// A profile does not cover every docs-authoring surface.
    SurfaceCoverageIncomplete,
    /// A surface coverage entry references the wrong canonical schema or artifact.
    SurfaceRefMismatch,
    /// A certified-and-promoted profile is missing evidence packet refs.
    CertifiedProfileMissingEvidence,
    /// A profile's stored qualification disagrees with the derived qualification.
    DerivedQualificationMismatch,
    /// A profile's stored verdict disagrees with the derived verdict.
    DerivedVerdictMismatch,
    /// A profile's stored freshness state disagrees with the derived state.
    FreshnessStateMismatch,
    /// A profile has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A profile claims to be no greener than the matrix but is not flagged so.
    ProfileGreenerThanMatrix,
    /// The certification index disagrees with the derived roll-up.
    IndexMismatch,
    /// Compatibility report does not satisfy required invariants.
    CompatibilityReportIncomplete,
    /// Downgrade rules are missing or not auto-enforced.
    DowngradeRulesIncomplete,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Known-limits or migration links are missing.
    KnownLimitsMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl CertViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredProfileMissing => "required_profile_missing",
            Self::ProfileRowIncomplete => "profile_row_incomplete",
            Self::SurfaceCoverageIncomplete => "surface_coverage_incomplete",
            Self::SurfaceRefMismatch => "surface_ref_mismatch",
            Self::CertifiedProfileMissingEvidence => "certified_profile_missing_evidence",
            Self::DerivedQualificationMismatch => "derived_qualification_mismatch",
            Self::DerivedVerdictMismatch => "derived_verdict_mismatch",
            Self::FreshnessStateMismatch => "freshness_state_mismatch",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ProfileGreenerThanMatrix => "profile_greener_than_matrix",
            Self::IndexMismatch => "index_mismatch",
            Self::CompatibilityReportIncomplete => "compatibility_report_incomplete",
            Self::DowngradeRulesIncomplete => "downgrade_rules_incomplete",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::KnownLimitsMissing => "known_limits_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable docs-authoring certification export.
pub fn current_stable_docs_authoring_cert_report(
) -> Result<DocsAuthoringCertReport, DocsAuthoringCertArtifactError> {
    let report: DocsAuthoringCertReport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/docs-authoring/certification-report/support_export.json"
    )))
    .map_err(DocsAuthoringCertArtifactError::SupportExport)?;
    let violations = report.validate();
    if violations.is_empty() {
        Ok(report)
    } else {
        Err(DocsAuthoringCertArtifactError::Validation(violations))
    }
}

/// Seeded stable certification input for emitters, the artifact, and tests.
pub fn seeded_stable_docs_authoring_cert_input() -> DocsAuthoringCertReportInput {
    DocsAuthoringCertReportInput {
        report_id: "m5-docs-authoring-certification:stable:0001".to_owned(),
        certification_label: "M5 Docs Authoring Certification".to_owned(),
        profile_rows: seeded_profile_rows(),
        compatibility_report: seeded_compatibility_report(),
        downgrade_rules: seeded_downgrade_rules(),
        trust_review: seeded_trust_review(),
        consumer_projection: seeded_consumer_projection(),
        proof_freshness: CertProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-18T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        known_limits: seeded_known_limits(),
        migration_refs: seeded_migration_refs(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-18T00:00:00Z".to_owned(),
    }
}

/// Seeded stable certification report for emitters, the artifact, and tests.
pub fn seeded_stable_docs_authoring_cert_report() -> DocsAuthoringCertReport {
    DocsAuthoringCertReport::new(seeded_stable_docs_authoring_cert_input())
}

/// Full per-surface coverage for the docs-authoring stack, every surface covered.
///
/// Emitters and fixtures use this to build a profile that certifies the entire
/// authoring stack before mutating individual gates.
pub fn full_surface_coverage() -> Vec<ProfileSurfaceCoverage> {
    DocsAuthoringCertSurface::ALL
        .iter()
        .map(|surface| ProfileSurfaceCoverage::covered(*surface))
        .collect()
}

fn evidence_refs(profile: DocsAuthoringProfile) -> Vec<String> {
    vec![
        format!(
            "evidence:docs-authoring:{}:m5",
            profile.as_str().replace('_', "-")
        ),
        DOCS_AUTHORING_CERT_ARTIFACT_REF.to_owned(),
    ]
}

fn seeded_profile_rows() -> Vec<DocsAuthoringProfileRow> {
    use CertDowngradeTrigger as T;
    use CertQualificationClass as Q;
    use DocsAuthoringProfile as P;

    let stable_profile =
        |profile: DocsAuthoringProfile, scope: &str, triggers: Vec<CertDowngradeTrigger>| {
            certify_profile_row(ProfileRowInput {
                profile,
                claimed_qualification: Q::Stable,
                scope_summary: scope.to_owned(),
                surface_coverage: full_surface_coverage(),
                source_version_freshness_truth: true,
                safe_rendered_preview_boundaries: true,
                export_support_parity: true,
                proof_age_hours: 12,
                freshness_window_hours: 168,
                evidence_packet_refs: evidence_refs(profile),
                downgrade_triggers: triggers,
                class_cap_trigger: None,
                class_cap_rationale: None,
            })
        };

    vec![
        stable_profile(
            P::Desktop,
            "Local desktop docs authoring with first-party packs: workspace, CommonMark preview, maintenance suggestions, validation, and evidence handoff all certified with current proof",
            vec![T::ProofStale, T::SourceVersionMismatch, T::UnsafePreviewBlocked, T::MissingExportParity],
        ),
        stable_profile(
            P::Mirrored,
            "Mirror-aware authoring backed by a pinned, signed mirror that outranks live vendor docs; recall falls back to last-known-good with explicit freshness labels",
            vec![T::ProofStale, T::MirrorOffline, T::FreshnessExpired, T::SourceVersionMismatch, T::UnsafePreviewBlocked],
        ),
        stable_profile(
            P::Cached,
            "Cached / last-known-good authoring while the source is offline, with visible freshness and source-version truth on every surface",
            vec![T::ProofStale, T::FreshnessExpired, T::SourceVersionMismatch, T::UnsafePreviewBlocked],
        ),
        stable_profile(
            P::PinnedPack,
            "Pinned docs-pack authoring against a frozen pack revision; the pinned revision and its signature stay visible across the authoring stack",
            vec![T::ProofStale, T::SourceVersionMismatch, T::MirrorOffline, T::UnsafePreviewBlocked],
        ),
        certify_profile_row(ProfileRowInput {
            profile: P::ExtensionOwned,
            claimed_qualification: Q::Beta,
            scope_summary:
                "Extension-owned docs surface running in a less-trusted host; the authoring stack is capped at Beta and rendered preview stays sanitized with no authority expansion"
                    .to_owned(),
            surface_coverage: full_surface_coverage(),
            source_version_freshness_truth: true,
            safe_rendered_preview_boundaries: true,
            export_support_parity: true,
            proof_age_hours: 12,
            freshness_window_hours: 168,
            evidence_packet_refs: evidence_refs(P::ExtensionOwned),
            downgrade_triggers: vec![
                T::ProofStale,
                T::TrustNarrowing,
                T::ScopeExpansionUnqualified,
                T::UnsafePreviewBlocked,
                T::PolicyBlocked,
            ],
            class_cap_trigger: Some(T::TrustNarrowing),
            class_cap_rationale: Some(
                "Extension-owned docs authoring runs in a less-trusted host, so the authoring stack is capped at Beta until the host trust class is raised"
                    .to_owned(),
            ),
        }),
        certify_profile_row(ProfileRowInput {
            profile: P::BrowserHandoff,
            claimed_qualification: Q::Beta,
            scope_summary:
                "Browser-handoff companion docs editing with a safe return path to the IDE; the narrow companion surface is capped at Beta and never widens authority"
                    .to_owned(),
            surface_coverage: full_surface_coverage(),
            source_version_freshness_truth: true,
            safe_rendered_preview_boundaries: true,
            export_support_parity: true,
            proof_age_hours: 12,
            freshness_window_hours: 168,
            evidence_packet_refs: evidence_refs(P::BrowserHandoff),
            downgrade_triggers: vec![
                T::ProofStale,
                T::TrustNarrowing,
                T::ScopeExpansionUnqualified,
                T::UnsafePreviewBlocked,
                T::PolicyBlocked,
            ],
            class_cap_trigger: Some(T::ScopeExpansionUnqualified),
            class_cap_rationale: Some(
                "Browser-handoff docs editing is a narrow companion surface, so the authoring stack is capped at Beta and never widens authority beyond the handoff scope"
                    .to_owned(),
            ),
        }),
    ]
}

fn seeded_compatibility_report() -> CertCompatibilityReport {
    CertCompatibilityReport {
        matrix_artifact_ref: M5_AUTHORING_MATRIX_ARTIFACT_REF.to_owned(),
        matrix_schema_ref: M5_AUTHORING_MATRIX_SCHEMA_REF.to_owned(),
        matrix_schema_version: M5_AUTHORING_MATRIX_SCHEMA_VERSION,
        all_profiles_present: true,
        no_profile_greener_than_matrix: true,
        every_surface_has_schema_and_artifact: true,
        downgrade_rules_auto_enforced: true,
    }
}

fn seeded_downgrade_rules() -> Vec<CertDowngradeRule> {
    use CertDowngradeAction as A;
    use CertDowngradeTrigger as T;
    vec![
        CertDowngradeRule {
            rule_id: "downgrade:unsafe_preview:block".to_owned(),
            trigger: T::UnsafePreviewBlocked,
            action: A::BlockPromotion,
            applies_to: DocsAuthoringProfile::ALL.to_vec(),
            auto_enforced: true,
            rationale: "A rendered preview that loses its safe capability boundaries blocks promotion on every profile; preview is never a privileged execution path.".to_owned(),
        },
        CertDowngradeRule {
            rule_id: "downgrade:proof_stale:narrow".to_owned(),
            trigger: T::ProofStale,
            action: A::NarrowToBeta,
            applies_to: DocsAuthoringProfile::ALL.to_vec(),
            auto_enforced: true,
            rationale: "When proof ages past the freshness window, the profile narrows one class below its claim until it is re-proven.".to_owned(),
        },
        CertDowngradeRule {
            rule_id: "downgrade:missing_export_parity:narrow".to_owned(),
            trigger: T::MissingExportParity,
            action: A::NarrowToBeta,
            applies_to: DocsAuthoringProfile::ALL.to_vec(),
            auto_enforced: true,
            rationale: "A profile that loses support/export parity for the authoring stack narrows below Stable until parity is restored.".to_owned(),
        },
        CertDowngradeRule {
            rule_id: "downgrade:mirror_offline:narrow_recall".to_owned(),
            trigger: T::MirrorOffline,
            action: A::NarrowToBeta,
            applies_to: vec![
                DocsAuthoringProfile::Mirrored,
                DocsAuthoringProfile::PinnedPack,
            ],
            auto_enforced: true,
            rationale: "A pinned, signed mirror going offline narrows mirror-backed authoring to Beta with explicit offline/freshness labels instead of serving live docs as canonical.".to_owned(),
        },
        CertDowngradeRule {
            rule_id: "downgrade:scope_expansion:block_handoff".to_owned(),
            trigger: T::ScopeExpansionUnqualified,
            action: A::BlockPromotion,
            applies_to: vec![
                DocsAuthoringProfile::ExtensionOwned,
                DocsAuthoringProfile::BrowserHandoff,
            ],
            auto_enforced: true,
            rationale: "Any scope expansion beyond the qualified extension/handoff boundary blocks promotion until the surface is re-qualified.".to_owned(),
        },
        CertDowngradeRule {
            rule_id: "downgrade:source_version_mismatch:narrow".to_owned(),
            trigger: T::SourceVersionMismatch,
            action: A::NarrowToBeta,
            applies_to: DocsAuthoringProfile::ALL.to_vec(),
            auto_enforced: true,
            rationale: "When source/version/freshness truth is lost, the profile narrows below Stable rather than presenting drifted docs as current.".to_owned(),
        },
    ]
}

fn seeded_trust_review() -> CertTrustReview {
    CertTrustReview {
        source_canonical_rendered_safe_and_labeled: true,
        rendered_preview_safe_by_default: true,
        preview_not_privileged_execution_path: true,
        suggestions_diff_first_never_auto_applied: true,
        source_version_freshness_truth_visible: true,
        validation_state_never_silently_upgraded: true,
        evidence_handoff_source_linked: true,
        handoff_never_hides_owner_origin_or_boundary: true,
        handoff_never_silently_widens_authority: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
        no_profile_greener_than_report: true,
    }
}

fn seeded_consumer_projection() -> CertConsumerProjection {
    CertConsumerProjection {
        release_gate_consumes_report: true,
        cli_headless_shows_certification: true,
        support_export_shows_certification: true,
        diagnostics_shows_certification: true,
        help_about_shows_certification: true,
        release_center_shows_certification: true,
        evidence_index_references_report: true,
        narrowed_profiles_labeled_not_hidden: true,
    }
}

fn seeded_known_limits() -> Vec<String> {
    vec![
        "Extension-owned and browser-handoff docs authoring are capped at Beta because they run outside the first-party desktop trust boundary.".to_owned(),
        "Rendered preview never executes diagrams, math, or custom components as privileged code; unsafe or unrequested capabilities are blocked, not rendered.".to_owned(),
        "Cached and mirrored profiles serve last-known-good docs with explicit freshness labels and never present stale content as current.".to_owned(),
        "This certification covers the desktop/local-first docs-authoring contract only; no browser-first docs product, collaborative rich-text editor, or remote CMS workflow is claimed.".to_owned(),
    ]
}

fn seeded_migration_refs() -> Vec<String> {
    vec![
        DOCS_AUTHORING_CERT_DOC_REF.to_owned(),
        crate::M5_AUTHORING_MATRIX_DOC_REF.to_owned(),
        DOCS_AUTHORING_WAIVER_LOG_REF.to_owned(),
    ]
}

fn seeded_source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        DOCS_AUTHORING_CERT_SCHEMA_REF.to_owned(),
        DOCS_AUTHORING_CERT_DOC_REF.to_owned(),
        M5_AUTHORING_MATRIX_ARTIFACT_REF.to_owned(),
        M5_AUTHORING_MATRIX_SCHEMA_REF.to_owned(),
        RELEASE_DOCS_MAINTENANCE_SCHEMA_REF.to_owned(),
    ];
    for surface in DocsAuthoringCertSurface::ALL {
        refs.push(surface.schema_ref().to_owned());
    }
    refs
}

fn required_source_contracts() -> Vec<&'static str> {
    let mut refs = vec![
        DOCS_AUTHORING_CERT_SCHEMA_REF,
        DOCS_AUTHORING_CERT_DOC_REF,
        M5_AUTHORING_MATRIX_ARTIFACT_REF,
        M5_AUTHORING_MATRIX_SCHEMA_REF,
        RELEASE_DOCS_MAINTENANCE_SCHEMA_REF,
    ];
    for surface in DocsAuthoringCertSurface::ALL {
        refs.push(surface.schema_ref());
    }
    refs
}

fn validate_source_contracts(
    report: &DocsAuthoringCertReport,
    violations: &mut Vec<CertViolation>,
) {
    let refs: BTreeSet<&str> = report
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for needed in required_source_contracts() {
        if !refs.contains(needed) {
            violations.push(CertViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_profile_rows(report: &DocsAuthoringCertReport, violations: &mut Vec<CertViolation>) {
    let present: BTreeSet<DocsAuthoringProfile> =
        report.profile_rows.iter().map(|row| row.profile).collect();
    for required in DocsAuthoringProfile::ALL {
        if !present.contains(&required) {
            violations.push(CertViolation::RequiredProfileMissing);
            return;
        }
    }

    for row in &report.profile_rows {
        if row.scope_summary.trim().is_empty()
            || row.evidence_packet_refs.iter().any(|r| r.trim().is_empty())
            || row.freshness_window_hours == 0
        {
            violations.push(CertViolation::ProfileRowIncomplete);
        }
        if !row.all_surfaces_covered() {
            violations.push(CertViolation::SurfaceCoverageIncomplete);
        }
        for entry in &row.surface_coverage {
            if entry.schema_ref != entry.surface.schema_ref()
                || entry.artifact_ref != entry.surface.artifact_ref()
            {
                violations.push(CertViolation::SurfaceRefMismatch);
            }
        }
        if row.is_promoted_and_certified() && row.evidence_packet_refs.is_empty() {
            violations.push(CertViolation::CertifiedProfileMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(CertViolation::DowngradeTriggersMissing);
        }
        if !row.not_greener_than_matrix {
            violations.push(CertViolation::ProfileGreenerThanMatrix);
        }

        // Re-derive the outcome and confirm the stored row never overstates it.
        let outcome = evaluate_profile(
            row.claimed_qualification,
            row.source_version_freshness_truth,
            row.safe_rendered_preview_boundaries,
            row.export_support_parity,
            row.all_surfaces_covered(),
            row.proof_age_hours,
            row.freshness_window_hours,
        );
        if row.qualification != outcome.qualification {
            violations.push(CertViolation::DerivedQualificationMismatch);
        }
        if row.verdict != outcome.verdict {
            violations.push(CertViolation::DerivedVerdictMismatch);
        }
        if row.freshness_state != outcome.freshness_state {
            violations.push(CertViolation::FreshnessStateMismatch);
        }
        if row.narrowing_trigger != outcome.narrowing_trigger {
            violations.push(CertViolation::DerivedVerdictMismatch);
        }
    }
}

fn validate_certification_index(
    report: &DocsAuthoringCertReport,
    violations: &mut Vec<CertViolation>,
) {
    let derived =
        derive_certification_index(DOCS_AUTHORING_CERT_ARTIFACT_REF, &report.profile_rows);
    if report.certification_index != derived {
        violations.push(CertViolation::IndexMismatch);
    }
}

fn validate_compatibility_report(
    report: &DocsAuthoringCertReport,
    violations: &mut Vec<CertViolation>,
) {
    let compat = &report.compatibility_report;
    let refs_ok = compat.matrix_artifact_ref == M5_AUTHORING_MATRIX_ARTIFACT_REF
        && compat.matrix_schema_ref == M5_AUTHORING_MATRIX_SCHEMA_REF
        && compat.matrix_schema_version == M5_AUTHORING_MATRIX_SCHEMA_VERSION;
    let flags_ok = compat.all_profiles_present
        && compat.no_profile_greener_than_matrix
        && compat.every_surface_has_schema_and_artifact
        && compat.downgrade_rules_auto_enforced;
    if !refs_ok || !flags_ok {
        violations.push(CertViolation::CompatibilityReportIncomplete);
    }
}

fn validate_downgrade_rules(report: &DocsAuthoringCertReport, violations: &mut Vec<CertViolation>) {
    if report.downgrade_rules.is_empty() {
        violations.push(CertViolation::DowngradeRulesIncomplete);
        return;
    }
    for rule in &report.downgrade_rules {
        if rule.rule_id.trim().is_empty()
            || rule.rationale.trim().is_empty()
            || rule.applies_to.is_empty()
            || !rule.auto_enforced
        {
            violations.push(CertViolation::DowngradeRulesIncomplete);
            return;
        }
    }
}

fn validate_trust_review(report: &DocsAuthoringCertReport, violations: &mut Vec<CertViolation>) {
    let review = &report.trust_review;
    for ok in [
        review.source_canonical_rendered_safe_and_labeled,
        review.rendered_preview_safe_by_default,
        review.preview_not_privileged_execution_path,
        review.suggestions_diff_first_never_auto_applied,
        review.source_version_freshness_truth_visible,
        review.validation_state_never_silently_upgraded,
        review.evidence_handoff_source_linked,
        review.handoff_never_hides_owner_origin_or_boundary,
        review.handoff_never_silently_widens_authority,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
        review.no_profile_greener_than_report,
    ] {
        if !ok {
            violations.push(CertViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    report: &DocsAuthoringCertReport,
    violations: &mut Vec<CertViolation>,
) {
    let projection = &report.consumer_projection;
    for ok in [
        projection.release_gate_consumes_report,
        projection.cli_headless_shows_certification,
        projection.support_export_shows_certification,
        projection.diagnostics_shows_certification,
        projection.help_about_shows_certification,
        projection.release_center_shows_certification,
        projection.evidence_index_references_report,
        projection.narrowed_profiles_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(CertViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(report: &DocsAuthoringCertReport, violations: &mut Vec<CertViolation>) {
    if report.proof_freshness.proof_freshness_slo_hours == 0
        || report.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(CertViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
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
