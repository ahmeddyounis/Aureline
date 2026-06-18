//! Capstone certification that ties the M5 appearance-object families into one
//! certifiable story per claimed M5 surface.
//!
//! The milestone and technical-design docs turned appearance from broad visual
//! parity into specific, portable objects: a versioned **theme package**, a live
//! **appearance session**, a round-trip-safe **token overlay**, an
//! **imported-theme report**, and an **extension appearance descriptor**. Each
//! of those families already has its own frozen contract, seeded report, and
//! fail-closed gate. This lane is the final row: it certifies, for every claimed
//! M5 desktop, extension-backed, and embedded surface, that all five families
//! stay honest *together*, and it publishes one canonical evidence index that
//! release-center, Help/About, diagnostics, support-export, and claim-publication
//! surfaces consume instead of restating appearance behavior by hand.
//!
//! Two records carry the truth:
//!
//! - the **object-model index** ([`ObjectFamilyIndexEntry`]): one entry per
//!   [`AppearanceObjectFamily`] naming the family's canonical schema, owning
//!   vocabulary group, shared contract ref, and the source report id / record
//!   kind a consumer pivots to. This is the single canonical appearance-object
//!   evidence index; nothing else re-derives where appearance truth lives.
//! - the per-surface **certification** ([`SurfaceCertification`]): for one
//!   claimed [`M5AppearanceSurfaceFamily`], a [`FamilyCertification`] per family
//!   that records the family's [`M5QualificationStatus`], its disclosed
//!   compatibility/downgrade posture ([`AppearanceCompatibilityState`]),
//!   evidence freshness, the source report it is backed by, and a narrowing
//!   reason when the family is not fully certified.
//!
//! The certified claim scope ([`CertifiedClaimScope`]) of a surface is
//! **derived**, never asserted: a surface drops from `certified_full` to
//! `certified_narrowed` the moment any family is honestly narrowed or carries a
//! disclosed downgrade, and is `blocked` if any family hides a downgrade, is
//! stale on a certified row, or claims appearance with no backing evidence. That
//! derivation is the auto-narrowing the acceptance criteria require: a claimed
//! surface cannot keep marketing full appearance stability once its underlying
//! appearance objects go missing or stale.
//!
//! The records are inspectable, serde-serializable truth packets that carry no
//! raw token tables, raw screenshots, raw paths, or raw user content — only
//! opaque refs, closed vocabulary, counts, and short labels. They are consumed
//! by the headless inspector (`aureline_shell_m5_appearance_object_certification`),
//! the support-export wrapper, the docs page under
//! `docs/m5/appearance-object-certification.md`, the published report under
//! `artifacts/ux/m5/theme-package-certification/`, and the boundary schema
//! `schemas/ux/m5-appearance-object-certification.schema.json`.
//!
//! The surface, qualification, and freshness vocabulary
//! ([`M5AppearanceSurfaceFamily`], [`M5QualificationStatus`],
//! [`M5EvidenceFreshness`]) is re-exported by reference from the already-frozen
//! appearance-parity contract; the object-family index and the per-family
//! source report ids are pulled straight from each family module's own
//! constants, so this lane mints no parallel appearance vocabulary and cannot
//! drift from the families it certifies. Only the certification-specific
//! vocabulary ([`AppearanceObjectFamily`], [`AppearanceCompatibilityState`],
//! [`CertifiedClaimScope`], [`SurfaceLifecycle`]) is new.
//!
//! The seeded projection is deterministic so the checked-in fixtures under
//! `fixtures/ux/m5/appearance-object-certification/` are bit-for-bit equal to the
//! output of [`seeded_appearance_object_certification_report`]; a live runtime
//! would call [`build_appearance_object_certification_report`] with
//! [`aureline_build_info::exact_build_identity_ref`] instead.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::appearance_session::{
    APPEARANCE_SESSION_PUBLISHED_DOC_REF, APPEARANCE_SESSION_PUBLISHED_REPORT_REF,
    APPEARANCE_SESSION_REPORT_ID, APPEARANCE_SESSION_REPORT_RECORD_KIND,
    APPEARANCE_SESSION_SHARED_CONTRACT_REF, APPEARANCE_SESSION_SOURCE_SCHEMA_REF,
};
use crate::m5_appearance_parity::{
    M5AppearanceSurfaceFamily, M5EvidenceFreshness, M5QualificationStatus,
};
use crate::theme_import_reports::{
    M5_THEME_IMPORT_PUBLISHED_DOC_REF, M5_THEME_IMPORT_PUBLISHED_REPORT_REF,
    M5_THEME_IMPORT_REPORT_ID, M5_THEME_IMPORT_REPORT_RECORD_KIND,
    M5_THEME_IMPORT_SHARED_CONTRACT_REF, M5_THEME_IMPORT_SOURCE_SCHEMA_REF,
};
use crate::theme_packages::{
    THEME_PACKAGE_PUBLISHED_DOC_REF, THEME_PACKAGE_PUBLISHED_REPORT_REF, THEME_PACKAGE_REPORT_ID,
    THEME_PACKAGE_REPORT_RECORD_KIND, THEME_PACKAGE_SHARED_CONTRACT_REF,
    THEME_PACKAGE_SOURCE_SCHEMA_REF,
};
use crate::token_overlays::{
    TOKEN_OVERLAY_PUBLISHED_DOC_REF, TOKEN_OVERLAY_PUBLISHED_REPORT_REF, TOKEN_OVERLAY_REPORT_ID,
    TOKEN_OVERLAY_REPORT_RECORD_KIND, TOKEN_OVERLAY_SHARED_CONTRACT_REF,
    TOKEN_OVERLAY_SOURCE_SCHEMA_REF,
};
use aureline_extensions::appearance_descriptors::{
    EXTENSION_APPEARANCE_AUDIT_ID, EXTENSION_APPEARANCE_AUDIT_RECORD_KIND,
    EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_DOC_REF,
    EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_REPORT_REF,
    EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_REF,
    EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF,
};

#[cfg(test)]
mod tests;

/// Schema version exported with every record.
pub const M5_APPEARANCE_CERT_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface.
pub const M5_APPEARANCE_CERT_SHARED_CONTRACT_REF: &str =
    "shell:m5_appearance_object_certification:v1";

/// Stable record kind for [`AppearanceObjectCertificationReport`] payloads.
pub const M5_APPEARANCE_CERT_REPORT_RECORD_KIND: &str =
    "shell_m5_appearance_object_certification_report_record";

/// Stable record kind for [`SurfaceCertification`] payloads.
pub const M5_APPEARANCE_CERT_SURFACE_RECORD_KIND: &str =
    "shell_m5_appearance_object_certification_surface_record";

/// Stable record kind for [`AppearanceObjectCertificationSupportExport`] payloads.
pub const M5_APPEARANCE_CERT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_appearance_object_certification_support_export_record";

/// Stable report id used to pivot across surfaces.
pub const M5_APPEARANCE_CERT_REPORT_ID: &str = "shell:m5_appearance_object_certification:audit:v1";

/// Stable support-export id.
pub const M5_APPEARANCE_CERT_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-appearance-object-certification:001";

/// Repo-relative ref to the boundary schema this report conforms to.
pub const M5_APPEARANCE_CERT_SOURCE_SCHEMA_REF: &str =
    "schemas/ux/m5-appearance-object-certification.schema.json";

/// Published markdown artifact ref reviewers reopen the report from.
pub const M5_APPEARANCE_CERT_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md";

/// Published companion doc ref.
pub const M5_APPEARANCE_CERT_PUBLISHED_DOC_REF: &str = "docs/m5/appearance-object-certification.md";

/// Deterministic generated-at value carried by the seeded report.
const GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps [`aureline_build_info::exact_build_identity_ref`] here;
/// the seed uses a fixed value so the checked-in fixtures stay reproducible. The
/// value matches the build the live-appearance evidence lane attributes its
/// captures to, so a reviewer can confirm both lanes certify the same build.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The claimed M5 surfaces this capstone must certify across every appearance
/// object family. These are exactly the appearance surface families the
/// appearance-parity contract already freezes; the lane broadens certified scope
/// to none beyond them.
pub const REQUIRED_SURFACE_FAMILIES: [M5AppearanceSurfaceFamily; 10] = [
    M5AppearanceSurfaceFamily::NotebookCellChrome,
    M5AppearanceSurfaceFamily::ResultGridRow,
    M5AppearanceSurfaceFamily::ProfilerPanel,
    M5AppearanceSurfaceFamily::TracePanel,
    M5AppearanceSurfaceFamily::PipelineCard,
    M5AppearanceSurfaceFamily::PreviewRouteBadge,
    M5AppearanceSurfaceFamily::DocsBrowserPane,
    M5AppearanceSurfaceFamily::CompanionSurface,
    M5AppearanceSurfaceFamily::SyncStatusSurface,
    M5AppearanceSurfaceFamily::OffboardingSurface,
];

/// One of the five canonical M5 appearance-object families this lane certifies.
///
/// Each family owns exactly one canonical schema and one seeded source report;
/// the [`ObjectFamilyIndexEntry`] for the family names both. The token values
/// match the frozen `object_family` vocabulary in the appearance-object matrix
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceObjectFamily {
    /// Versioned theme-package manifests and per-surface bindings.
    ThemePackage,
    /// Live appearance-session runtime and checkpoint state machine.
    AppearanceSession,
    /// Scope-explicit, round-trip-safe token overlays.
    TokenOverlay,
    /// Imported-theme mapping and rollback reports.
    ThemeImportReport,
    /// Extension and embedded-surface appearance-inheritance descriptors.
    ExtensionAppearanceDescriptor,
}

impl AppearanceObjectFamily {
    /// Every family, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::ThemePackage,
        Self::AppearanceSession,
        Self::TokenOverlay,
        Self::ThemeImportReport,
        Self::ExtensionAppearanceDescriptor,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThemePackage => "theme_package",
            Self::AppearanceSession => "appearance_session",
            Self::TokenOverlay => "token_overlay",
            Self::ThemeImportReport => "theme_import_report",
            Self::ExtensionAppearanceDescriptor => "extension_appearance_descriptor",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ThemePackage => "Theme package",
            Self::AppearanceSession => "Appearance session",
            Self::TokenOverlay => "Token overlay",
            Self::ThemeImportReport => "Imported-theme report",
            Self::ExtensionAppearanceDescriptor => "Extension appearance descriptor",
        }
    }

    /// The source report id this family's certification must be backed by.
    pub fn source_report_id(self) -> &'static str {
        match self {
            Self::ThemePackage => THEME_PACKAGE_REPORT_ID,
            Self::AppearanceSession => APPEARANCE_SESSION_REPORT_ID,
            Self::TokenOverlay => TOKEN_OVERLAY_REPORT_ID,
            Self::ThemeImportReport => M5_THEME_IMPORT_REPORT_ID,
            Self::ExtensionAppearanceDescriptor => EXTENSION_APPEARANCE_AUDIT_ID,
        }
    }
}

/// The disclosed compatibility / downgrade posture of one family certification.
///
/// `current` means the family is honored at full fidelity on the surface. Every
/// other value is a *disclosed downgrade* — honest only when
/// [`FamilyCertification::downgrade_disclosed`] is `true`; a hidden downgrade is
/// a blocker. The tokens match the frozen `compatibility_state` vocabulary in
/// the appearance-object matrix contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceCompatibilityState {
    /// The family is honored at full fidelity on this surface.
    Current,
    /// The family's evidence is stale and disclosed as such.
    StaleEvidence,
    /// A token / slot / mode the family expects is unsupported here.
    UnsupportedSlot,
    /// The family is only partially inherited on this surface.
    PartialInheritance,
    /// Applying the family's change needs a disclosed reload or restart.
    RestartOrReloadRequired,
}

impl AppearanceCompatibilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleEvidence => "stale_evidence",
            Self::UnsupportedSlot => "unsupported_slot",
            Self::PartialInheritance => "partial_inheritance",
            Self::RestartOrReloadRequired => "restart_or_reload_required",
        }
    }

    /// `true` when the family is honored at full fidelity.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// The derived appearance claim a certified surface may publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedClaimScope {
    /// Every applicable family is certified at full fidelity.
    CertifiedFull,
    /// At least one family is honestly narrowed or carries a disclosed
    /// downgrade; the surface may market only the narrowed appearance story.
    CertifiedNarrowed,
    /// At least one family hides a downgrade, is stale on a certified row, or
    /// claims appearance with no backing evidence. The surface may not market
    /// appearance stability until it is repaired.
    Blocked,
}

impl CertifiedClaimScope {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedFull => "certified_full",
            Self::CertifiedNarrowed => "certified_narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// `true` when the surface keeps a publishable (full or narrowed) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::CertifiedFull | Self::CertifiedNarrowed)
    }
}

/// Lifecycle of a certified surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceLifecycle {
    /// A stable, release-grade surface.
    Stable,
    /// A beta surface.
    Beta,
    /// A deprecated surface.
    Deprecated,
}

impl SurfaceLifecycle {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// One entry in the canonical appearance-object evidence index.
///
/// Each family names its canonical schema, owned vocabulary group, shared
/// contract ref, and the source report id / record kind a consumer pivots to.
/// Support and docs/help surfaces read this index instead of re-deriving where
/// appearance truth lives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectFamilyIndexEntry {
    /// The appearance-object family.
    pub object_family: AppearanceObjectFamily,
    /// Repo-relative ref to the family's canonical schema.
    pub canonical_schema_ref: String,
    /// Short label of the vocabulary group this family owns.
    pub vocabulary_group: String,
    /// The family's shared contract ref.
    pub source_contract_ref: String,
    /// The family's seeded source report id.
    pub source_report_id: String,
    /// The family's source report record kind.
    pub source_report_record_kind: String,
    /// The family's published report artifact ref.
    pub published_report_ref: String,
    /// The family's companion doc ref.
    pub published_doc_ref: String,
}

/// The certification of one appearance-object family on one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyCertification {
    /// The certified family.
    pub object_family: AppearanceObjectFamily,
    /// Qualification status the certification reports for this family.
    pub certification_status: M5QualificationStatus,
    /// Disclosed compatibility / downgrade posture.
    pub compatibility_state: AppearanceCompatibilityState,
    /// `true` when a non-current compatibility state is disclosed up front. MUST
    /// be `true` whenever the compatibility state is not `current`.
    pub downgrade_disclosed: bool,
    /// Freshness of the family's backing evidence.
    pub evidence_freshness: M5EvidenceFreshness,
    /// The source report id this certification is backed by. MUST equal the
    /// family's entry in the object-model index.
    pub source_report_id: String,
    /// Opaque refs into the family's source report (rows, bindings, descriptors).
    pub evidence_refs: Vec<String>,
    /// Required whenever the family is narrowed or carries a disclosed downgrade.
    pub narrowing_reason: Option<String>,
}

impl FamilyCertification {
    /// `true` when the family is certified at full standing (qualified).
    pub fn is_certified(&self) -> bool {
        self.certification_status.projects_evidence()
    }

    /// `true` when the family claims appearance with no backing evidence — a
    /// blocker the surface cannot market past.
    pub fn is_blocking_status(&self) -> bool {
        matches!(
            self.certification_status,
            M5QualificationStatus::MissingEvidence
                | M5QualificationStatus::UnqualifiedLocalAppearance
        )
    }

    /// `true` when this family reduces the surface's claim scope to narrowed: an
    /// honestly narrowed family (excluding a not-applicable one, which the
    /// surface never claimed) or a certified family carrying a disclosed
    /// downgrade.
    pub fn reduces_claim_scope(&self) -> bool {
        let narrowed_status = matches!(
            self.certification_status,
            M5QualificationStatus::ExplicitlyNarrowed
                | M5QualificationStatus::PlatformOmitted
                | M5QualificationStatus::DeclaredCaptureGap
        );
        let disclosed_downgrade = self.is_certified() && !self.compatibility_state.is_current();
        narrowed_status || disclosed_downgrade
    }

    /// `true` when a narrowing reason is required for this family.
    pub fn requires_narrowing_reason(&self) -> bool {
        self.certification_status.requires_narrowing_reason()
            || (self.is_certified() && !self.compatibility_state.is_current())
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }
}

/// One claimed M5 surface, certified across all five appearance-object families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCertification {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable certification id quoted across surfaces.
    pub certification_id: String,
    /// Reviewer-facing title for the certification.
    pub title: String,
    /// The claimed surface family being certified.
    pub surface_family: M5AppearanceSurfaceFamily,
    /// Lifecycle of the surface.
    pub surface_lifecycle: SurfaceLifecycle,
    /// One certification per appearance-object family, in canonical order.
    pub family_certifications: Vec<FamilyCertification>,
    /// Derived appearance claim scope. Recomputed by the builder; never asserted.
    pub certified_claim_scope: CertifiedClaimScope,
    /// `true` when every certified family is current (no disclosed downgrade).
    pub all_families_current: bool,
    /// Required whenever the claim scope is narrowed or blocked.
    pub narrowing_reason: Option<String>,
    /// Release-center ref that routes this certification.
    pub release_center_ref: String,
    /// Help/About ref that surfaces this certification.
    pub help_about_ref: String,
    /// Diagnostics ref that inspects this certification.
    pub diagnostics_ref: String,
    /// Support/export ref that preserves this certification.
    pub support_export_ref: String,
    /// Claim-publication ref the claim scope feeds.
    pub claim_publication_ref: String,
    /// Docs/help refs the certification reopens from.
    pub docs_help_refs: Vec<String>,
    /// Reviewer-facing narrative summary.
    pub narrative: String,
}

impl SurfaceCertification {
    /// Returns the certification for `family`, if present.
    pub fn family(&self, family: AppearanceObjectFamily) -> Option<&FamilyCertification> {
        self.family_certifications
            .iter()
            .find(|certification| certification.object_family == family)
    }

    /// Recomputes the derived claim scope from the family certifications. This is
    /// the auto-narrowing rule: any blocker forces `blocked`, any honest
    /// narrowing or disclosed downgrade forces `certified_narrowed`, otherwise
    /// `certified_full`.
    pub fn recompute_claim_scope(&self) -> CertifiedClaimScope {
        let mut blocked = false;
        let mut narrowed = false;
        for certification in &self.family_certifications {
            if certification.is_blocking_status() {
                blocked = true;
            }
            if !certification.compatibility_state.is_current() && !certification.downgrade_disclosed
            {
                blocked = true;
            }
            if certification.is_certified()
                && matches!(certification.evidence_freshness, M5EvidenceFreshness::Stale)
            {
                blocked = true;
            }
            if certification.reduces_claim_scope() {
                narrowed = true;
            }
        }
        if blocked {
            CertifiedClaimScope::Blocked
        } else if narrowed {
            CertifiedClaimScope::CertifiedNarrowed
        } else {
            CertifiedClaimScope::CertifiedFull
        }
    }

    fn compute_all_families_current(&self) -> bool {
        self.family_certifications
            .iter()
            .filter(|certification| certification.is_certified())
            .all(|certification| certification.compatibility_state.is_current())
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    /// Returns deterministic compact lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("{} [{}]", self.title, self.surface_family.as_str()),
            format!(
                "  lifecycle={} scope={} all_families_current={}",
                self.surface_lifecycle.as_str(),
                self.certified_claim_scope.as_str(),
                self.all_families_current
            ),
        ];
        for certification in &self.family_certifications {
            lines.push(format!(
                "  {} status={} compat={} fresh={} source={}",
                certification.object_family.as_str(),
                certification.certification_status.as_str(),
                certification.compatibility_state.as_str(),
                certification.evidence_freshness.as_str(),
                certification.source_report_id,
            ));
        }
        if let Some(reason) = &self.narrowing_reason {
            lines.push(format!("  narrowing_reason: {reason}"));
        }
        lines
    }

    fn compute_findings(
        &self,
        index_source_ids: &[(AppearanceObjectFamily, &str)],
    ) -> Vec<CertificationBlockingFinding> {
        let mut findings = Vec::new();
        let certification_id = self.certification_id.clone();

        // Every family must be certified on every claimed surface.
        for family in AppearanceObjectFamily::ALL {
            if self.family(family).is_none() {
                findings.push(CertificationBlockingFinding::SurfaceFamilyMissing {
                    certification_id: certification_id.clone(),
                    family: family.as_str().to_owned(),
                });
            }
        }

        for certification in &self.family_certifications {
            let family_token = certification.object_family.as_str().to_owned();

            if certification.is_blocking_status() {
                findings.push(CertificationBlockingFinding::FamilyMissingEvidence {
                    certification_id: certification_id.clone(),
                    family: family_token.clone(),
                });
            }

            // A non-current compatibility state must be disclosed up front.
            if !certification.compatibility_state.is_current() && !certification.downgrade_disclosed
            {
                findings.push(CertificationBlockingFinding::HiddenDowngrade {
                    certification_id: certification_id.clone(),
                    family: family_token.clone(),
                });
            }

            // A certified family may not be marketed on stale evidence.
            if certification.is_certified()
                && matches!(certification.evidence_freshness, M5EvidenceFreshness::Stale)
            {
                findings.push(
                    CertificationBlockingFinding::StaleEvidenceOnCertifiedFamily {
                        certification_id: certification_id.clone(),
                        family: family_token.clone(),
                    },
                );
            }

            // A narrowed or downgraded family must carry a reason.
            if certification.requires_narrowing_reason() && !certification.has_reason() {
                findings.push(CertificationBlockingFinding::MissingFamilyNarrowingReason {
                    certification_id: certification_id.clone(),
                    family: family_token.clone(),
                });
            }

            // The certification must be backed by the family's canonical report.
            let expected_source = index_source_ids
                .iter()
                .find(|(family, _)| *family == certification.object_family)
                .map(|(_, source)| *source);
            if expected_source != Some(certification.source_report_id.as_str()) {
                findings.push(CertificationBlockingFinding::UnbackedFamilySource {
                    certification_id: certification_id.clone(),
                    family: family_token.clone(),
                });
            }
        }

        // The declared claim scope must match the derived auto-narrowed scope.
        let derived = self.recompute_claim_scope();
        if derived != self.certified_claim_scope {
            findings.push(CertificationBlockingFinding::ClaimScopeStale {
                certification_id: certification_id.clone(),
            });
        }

        // A narrowed or blocked surface must disclose why.
        if !matches!(derived, CertifiedClaimScope::CertifiedFull) && !self.has_reason() {
            findings.push(CertificationBlockingFinding::SurfaceNarrowedWithoutReason {
                certification_id: certification_id.clone(),
            });
        }

        findings
    }
}

/// Per-scope blocking-finding summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CertificationFindingSummary {
    /// Blocking findings owned by a surface certification.
    pub surface_findings: usize,
    /// Blocking findings owned by the report-level object-model index or
    /// required-surface coverage.
    pub index_findings: usize,
    /// Total blocking findings.
    pub total_blocking_findings: usize,
}

impl CertificationFindingSummary {
    fn record(&mut self, finding: &CertificationBlockingFinding) {
        match finding {
            CertificationBlockingFinding::IndexFamilyMissing { .. }
            | CertificationBlockingFinding::UncertifiedRequiredSurface { .. } => {
                self.index_findings += 1;
            }
            _ => self.surface_findings += 1,
        }
        self.total_blocking_findings += 1;
    }
}

/// A blocking finding the appearance-object certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum CertificationBlockingFinding {
    /// The object-model index does not register one of the five families.
    IndexFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A claimed M5 surface has no certification at all.
    UncertifiedRequiredSurface {
        /// The uncertified surface family token.
        surface_family: String,
    },
    /// A surface certification omits one of the five families.
    SurfaceFamilyMissing {
        /// Owning certification id.
        certification_id: String,
        /// The missing family token.
        family: String,
    },
    /// A family claims appearance with no backing evidence.
    FamilyMissingEvidence {
        /// Owning certification id.
        certification_id: String,
        /// The family token.
        family: String,
    },
    /// A family carries a non-current compatibility state without disclosing it.
    HiddenDowngrade {
        /// Owning certification id.
        certification_id: String,
        /// The family token.
        family: String,
    },
    /// A certified family is marketed on stale evidence.
    StaleEvidenceOnCertifiedFamily {
        /// Owning certification id.
        certification_id: String,
        /// The family token.
        family: String,
    },
    /// A narrowed or downgraded family carries no reason.
    MissingFamilyNarrowingReason {
        /// Owning certification id.
        certification_id: String,
        /// The family token.
        family: String,
    },
    /// A family certification cites a source report not in the object-model index.
    UnbackedFamilySource {
        /// Owning certification id.
        certification_id: String,
        /// The family token.
        family: String,
    },
    /// The declared claim scope does not match the derived auto-narrowed scope.
    ClaimScopeStale {
        /// Owning certification id.
        certification_id: String,
    },
    /// A narrowed or blocked surface does not disclose why.
    SurfaceNarrowedWithoutReason {
        /// Owning certification id.
        certification_id: String,
    },
}

impl CertificationBlockingFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::IndexFamilyMissing { .. } => "index_family_missing",
            Self::UncertifiedRequiredSurface { .. } => "uncertified_required_surface",
            Self::SurfaceFamilyMissing { .. } => "surface_family_missing",
            Self::FamilyMissingEvidence { .. } => "family_missing_evidence",
            Self::HiddenDowngrade { .. } => "hidden_downgrade",
            Self::StaleEvidenceOnCertifiedFamily { .. } => "stale_evidence_on_certified_family",
            Self::MissingFamilyNarrowingReason { .. } => "missing_family_narrowing_reason",
            Self::UnbackedFamilySource { .. } => "unbacked_family_source",
            Self::ClaimScopeStale { .. } => "claim_scope_stale",
            Self::SurfaceNarrowedWithoutReason { .. } => "surface_narrowed_without_reason",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::IndexFamilyMissing { family } => family,
            Self::UncertifiedRequiredSurface { surface_family } => surface_family,
            Self::SurfaceFamilyMissing {
                certification_id, ..
            }
            | Self::FamilyMissingEvidence {
                certification_id, ..
            }
            | Self::HiddenDowngrade {
                certification_id, ..
            }
            | Self::StaleEvidenceOnCertifiedFamily {
                certification_id, ..
            }
            | Self::MissingFamilyNarrowingReason {
                certification_id, ..
            }
            | Self::UnbackedFamilySource {
                certification_id, ..
            }
            | Self::ClaimScopeStale { certification_id }
            | Self::SurfaceNarrowedWithoutReason { certification_id } => certification_id,
        }
    }
}

fn covered_surface_families(surfaces: &[SurfaceCertification]) -> BTreeSet<&'static str> {
    surfaces
        .iter()
        .map(|surface| surface.surface_family.as_str())
        .collect()
}

fn compute_index_findings(
    index: &[ObjectFamilyIndexEntry],
    surfaces: &[SurfaceCertification],
) -> Vec<CertificationBlockingFinding> {
    let mut findings = Vec::new();

    // The object-model index must register every family.
    for family in AppearanceObjectFamily::ALL {
        if !index.iter().any(|entry| entry.object_family == family) {
            findings.push(CertificationBlockingFinding::IndexFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }

    // Every claimed surface must carry a certification.
    let covered = covered_surface_families(surfaces);
    for surface in REQUIRED_SURFACE_FAMILIES {
        if !covered.contains(surface.as_str()) {
            findings.push(CertificationBlockingFinding::UncertifiedRequiredSurface {
                surface_family: surface.as_str().to_owned(),
            });
        }
    }

    findings
}

/// The appearance-object certification report shared by the release/evidence
/// center, the support-export wrapper, and the docs/help surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceObjectCertificationReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the report.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable report id used to pivot across surfaces.
    pub report_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the surfaces.
    pub headline: String,
    /// Exact-build identity ref the report was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The canonical appearance-object evidence index, one entry per family.
    pub object_model_index: Vec<ObjectFamilyIndexEntry>,
    /// Per-surface certifications in canonical order.
    pub surfaces: Vec<SurfaceCertification>,
    /// Surface families certified, in canonical (sorted) order.
    pub covered_surface_families: Vec<String>,
    /// Number of certifications.
    pub surface_count: usize,
    /// Number of surfaces certified at full fidelity.
    pub certified_full_surface_count: usize,
    /// Number of surfaces auto-narrowed to a disclosed appearance story.
    pub narrowed_surface_count: usize,
    /// Number of surfaces blocked from marketing appearance stability.
    pub blocked_surface_count: usize,
    /// `true` when no claimed surface is blocked.
    pub all_surfaces_publishable: bool,
    /// Per-scope blocking-finding summary.
    pub findings_summary: CertificationFindingSummary,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<CertificationBlockingFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Release / evidence-center refs that route the report.
    pub release_evidence_refs: Vec<String>,
    /// Extension-inspection refs that consume the report.
    pub extension_inspection_refs: Vec<String>,
    /// Sync / import refs that preserve the certification.
    pub sync_refs: Vec<String>,
    /// Docs/help refs the report reopens from.
    pub docs_help_refs: Vec<String>,
    /// Support / export refs that preserve the report.
    pub support_export_refs: Vec<String>,
    /// Claim-publication refs the scopes feed.
    pub claim_publication_refs: Vec<String>,
    /// Published markdown artifact ref.
    pub published_report_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl AppearanceObjectCertificationReport {
    /// Returns the certification registered under `certification_id`, if any.
    pub fn surface(&self, certification_id: &str) -> Option<&SurfaceCertification> {
        self.surfaces
            .iter()
            .find(|surface| surface.certification_id == certification_id)
    }

    /// Returns the object-model index entry for `family`, if any.
    pub fn index_entry(&self, family: AppearanceObjectFamily) -> Option<&ObjectFamilyIndexEntry> {
        self.object_model_index
            .iter()
            .find(|entry| entry.object_family == family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: id={}, surfaces={}, certified_full={}, narrowed={}, blocked={}, clean={}",
            self.report_id,
            self.surface_count,
            self.certified_full_surface_count,
            self.narrowed_surface_count,
            self.blocked_surface_count,
            self.report_clean,
        ));
        lines.push(format!(
            "build={} channel={} all_surfaces_publishable={}",
            self.build_identity_ref, self.release_channel_class, self.all_surfaces_publishable,
        ));
        for entry in &self.object_model_index {
            lines.push(format!(
                "index: {} schema={} report={}",
                entry.object_family.as_str(),
                entry.canonical_schema_ref,
                entry.source_report_id,
            ));
        }
        for surface in &self.surfaces {
            lines.extend(surface.compact_lines());
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 appearance-object certification\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::appearance_object_certification`](../../../../crates/aureline-shell/src/appearance_object_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- markdown > \\\n  artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!("- Surfaces certified: {}\n", self.surface_count));
        out.push_str(&format!(
            "- Certified (full): {}\n",
            self.certified_full_surface_count
        ));
        out.push_str(&format!(
            "- Auto-narrowed: {}\n",
            self.narrowed_surface_count
        ));
        out.push_str(&format!("- Blocked: {}\n", self.blocked_surface_count));
        out.push_str(&format!(
            "- All surfaces publishable: `{}`\n",
            self.all_surfaces_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Canonical appearance-object index\n\n");
        out.push_str(
            "| Family | Canonical schema | Source report | Contract |\n\
             | ------ | ---------------- | ------------- | -------- |\n",
        );
        for entry in &self.object_model_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` |\n",
                entry.object_family.display_label(),
                entry.canonical_schema_ref,
                entry.source_report_id,
                entry.source_contract_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface certification\n\n");
        out.push_str(
            "| Surface | Lifecycle | Scope | Theme package | Appearance session | Token overlay | Imported theme | Extension |\n\
             | ------- | --------- | ----- | ------------- | ------------------ | ------------- | -------------- | --------- |\n",
        );
        for surface in &self.surfaces {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | {} | {} | {} | {} | {} |\n",
                surface.surface_family.display_label(),
                surface.surface_lifecycle.as_str(),
                surface.certified_claim_scope.as_str(),
                family_cell(surface, AppearanceObjectFamily::ThemePackage),
                family_cell(surface, AppearanceObjectFamily::AppearanceSession),
                family_cell(surface, AppearanceObjectFamily::TokenOverlay),
                family_cell(surface, AppearanceObjectFamily::ThemeImportReport),
                family_cell(
                    surface,
                    AppearanceObjectFamily::ExtensionAppearanceDescriptor
                ),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed surfaces\n\n");
        let narrowed: Vec<&SurfaceCertification> = self
            .surfaces
            .iter()
            .filter(|surface| {
                !matches!(
                    surface.certified_claim_scope,
                    CertifiedClaimScope::CertifiedFull
                )
            })
            .collect();
        if narrowed.is_empty() {
            out.push_str("None — every claimed surface certifies at full fidelity.\n\n");
        } else {
            for surface in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    surface.surface_family.as_str(),
                    surface.certified_claim_scope.as_str(),
                    surface
                        .narrowing_reason
                        .as_deref()
                        .unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_appearance_object_certification_fixtures\n",
        );
        out.push_str(
            "python3 tools/ci/m5/appearance_object_certification_check.py --repo-root .\n",
        );
        out.push_str("```\n");
        out
    }
}

fn family_cell(surface: &SurfaceCertification, family: AppearanceObjectFamily) -> String {
    match surface.family(family) {
        Some(certification) => {
            if certification.compatibility_state.is_current() {
                format!("`{}`", certification.certification_status.as_str())
            } else {
                format!(
                    "`{}` / `{}`",
                    certification.certification_status.as_str(),
                    certification.compatibility_state.as_str()
                )
            }
        }
        None => "—".to_owned(),
    }
}

/// Support-export wrapper for the appearance-object certification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceObjectCertificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: AppearanceObjectCertificationReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl AppearanceObjectCertificationSupportExport {
    /// Builds the support-export wrapper for a report.
    ///
    /// Every report id, the exact-build ref, each family's canonical schema and
    /// source report id, each certification id, and each family source report /
    /// evidence ref is quoted as a case id so a support reviewer — or a
    /// release-evidence pack — can name the same surface, family, and backing
    /// report the runtime certified.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: AppearanceObjectCertificationReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone(), report.build_identity_ref.clone()];
        for entry in &report.object_model_index {
            case_ids.push(entry.source_report_id.clone());
            case_ids.push(entry.canonical_schema_ref.clone());
        }
        for surface in &report.surfaces {
            case_ids.push(surface.certification_id.clone());
            for certification in &surface.family_certifications {
                case_ids.push(certification.source_report_id.clone());
                for evidence_ref in &certification.evidence_refs {
                    case_ids.push(evidence_ref.clone());
                }
            }
        }
        Self {
            record_kind: M5_APPEARANCE_CERT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_APPEARANCE_CERT_SCHEMA_VERSION,
            shared_contract_ref: M5_APPEARANCE_CERT_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Builds an [`AppearanceObjectCertificationReport`] from the exact build
/// identity, the canonical object-model index, and the per-surface
/// certifications.
///
/// A live runtime passes [`aureline_build_info::exact_build_identity_ref`] and
/// [`aureline_build_info::release_channel_class`]. Each surface's derived claim
/// scope and `all_families_current` flag, the coverage summaries, and the
/// blocking findings are recomputed here so the report is the single source of
/// truth.
pub fn build_appearance_object_certification_report(
    build_identity_ref: impl Into<String>,
    release_channel_class: impl Into<String>,
    object_model_index: Vec<ObjectFamilyIndexEntry>,
    surfaces: Vec<SurfaceCertification>,
) -> AppearanceObjectCertificationReport {
    let build_identity_ref = build_identity_ref.into();
    let release_channel_class = release_channel_class.into();

    let index_source_ids: Vec<(AppearanceObjectFamily, &str)> = object_model_index
        .iter()
        .map(|entry| (entry.object_family, entry.source_report_id.as_str()))
        .collect();

    // Recompute each surface's derived scope and current-flag so the packet is
    // self-consistent and the auto-narrowing is the single source of truth.
    let surfaces: Vec<SurfaceCertification> = surfaces
        .into_iter()
        .map(|mut surface| {
            surface.certified_claim_scope = surface.recompute_claim_scope();
            surface.all_families_current = surface.compute_all_families_current();
            surface
        })
        .collect();

    let mut findings_summary = CertificationFindingSummary::default();
    let mut blocking_findings: Vec<CertificationBlockingFinding> = Vec::new();
    for finding in compute_index_findings(&object_model_index, &surfaces) {
        findings_summary.record(&finding);
        blocking_findings.push(finding);
    }
    for surface in &surfaces {
        for finding in surface.compute_findings(&index_source_ids) {
            findings_summary.record(&finding);
            blocking_findings.push(finding);
        }
    }
    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });

    let covered_surface_families: Vec<String> = covered_surface_families(&surfaces)
        .into_iter()
        .map(str::to_owned)
        .collect();

    let surface_count = surfaces.len();
    let certified_full_surface_count = surfaces
        .iter()
        .filter(|surface| {
            matches!(
                surface.certified_claim_scope,
                CertifiedClaimScope::CertifiedFull
            )
        })
        .count();
    let narrowed_surface_count = surfaces
        .iter()
        .filter(|surface| {
            matches!(
                surface.certified_claim_scope,
                CertifiedClaimScope::CertifiedNarrowed
            )
        })
        .count();
    let blocked_surface_count = surfaces
        .iter()
        .filter(|surface| matches!(surface.certified_claim_scope, CertifiedClaimScope::Blocked))
        .count();
    let all_surfaces_publishable = blocked_surface_count == 0;
    let report_clean = findings_summary.total_blocking_findings == 0;

    AppearanceObjectCertificationReport {
        record_kind: M5_APPEARANCE_CERT_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_APPEARANCE_CERT_SCHEMA_VERSION,
        shared_contract_ref: M5_APPEARANCE_CERT_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_APPEARANCE_CERT_REPORT_ID.to_owned(),
        source_schema_ref: M5_APPEARANCE_CERT_SOURCE_SCHEMA_REF.to_owned(),
        headline: "One certifiable appearance-object story for every claimed M5 desktop, \
                   extension-backed, and embedded surface: theme package, appearance session, \
                   token overlay, imported-theme report, and extension inheritance certified \
                   together, with claim scope auto-narrowed from the underlying object evidence."
            .to_owned(),
        build_identity_ref,
        release_channel_class,
        object_model_index,
        surfaces,
        covered_surface_families,
        surface_count,
        certified_full_surface_count,
        narrowed_surface_count,
        blocked_surface_count,
        all_surfaces_publishable,
        findings_summary,
        blocking_findings,
        report_clean,
        release_evidence_refs: vec![
            "release_center.appearance_object_certification".to_owned(),
            "docs/release/release_evidence_object_model.md#appearance-object-certification"
                .to_owned(),
        ],
        extension_inspection_refs: vec![
            "extensions.appearance_inspection.object_certification".to_owned()
        ],
        sync_refs: vec!["sync.appearance_objects.certification".to_owned()],
        docs_help_refs: vec![
            M5_APPEARANCE_CERT_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/theme-package-and-appearance-objects.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-appearance-object-certification".to_owned()],
        claim_publication_refs: vec!["claim_publication.appearance_object_certification".to_owned()],
        published_report_ref: M5_APPEARANCE_CERT_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_APPEARANCE_CERT_PUBLISHED_DOC_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_appearance_object_certification_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CertificationValidationError {
    /// The report has no registered surfaces.
    NoRegisteredSurfaces,
    /// The report's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The object-model index does not register all five families.
    ObjectModelIndexIncomplete,
    /// The declared surface coverage does not match the surfaces.
    SurfaceCoverageStale,
    /// One of the declared scope counts does not match the surfaces.
    SurfaceCountsStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the report.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a report against the appearance-object certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: the canonical
/// index registers all five families; every claimed surface is certified across
/// all five; each family is backed by its canonical source report; claim scope
/// is the derived auto-narrowed value, never an asserted one; and no surface
/// keeps a full claim while an underlying object is missing, stale, or hiding a
/// downgrade.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_appearance_object_certification_report(
    report: &AppearanceObjectCertificationReport,
) -> Result<(), Vec<CertificationValidationError>> {
    let mut errors = Vec::new();

    if report.surfaces.is_empty() {
        errors.push(CertificationValidationError::NoRegisteredSurfaces);
    }
    if report.build_identity_ref.trim().is_empty() {
        errors.push(CertificationValidationError::BuildIdentityRefMissing);
    }

    let index_complete = AppearanceObjectFamily::ALL.iter().all(|family| {
        report
            .object_model_index
            .iter()
            .any(|entry| entry.object_family == *family)
    });
    if !index_complete || report.object_model_index.len() != AppearanceObjectFamily::ALL.len() {
        errors.push(CertificationValidationError::ObjectModelIndexIncomplete);
    }

    let covered: Vec<String> = covered_surface_families(&report.surfaces)
        .into_iter()
        .map(str::to_owned)
        .collect();
    if covered != report.covered_surface_families {
        errors.push(CertificationValidationError::SurfaceCoverageStale);
    }

    let certified_full = report
        .surfaces
        .iter()
        .filter(|surface| {
            matches!(
                surface.recompute_claim_scope(),
                CertifiedClaimScope::CertifiedFull
            )
        })
        .count();
    let narrowed = report
        .surfaces
        .iter()
        .filter(|surface| {
            matches!(
                surface.recompute_claim_scope(),
                CertifiedClaimScope::CertifiedNarrowed
            )
        })
        .count();
    let blocked = report
        .surfaces
        .iter()
        .filter(|surface| {
            matches!(
                surface.recompute_claim_scope(),
                CertifiedClaimScope::Blocked
            )
        })
        .count();
    if report.surface_count != report.surfaces.len()
        || report.certified_full_surface_count != certified_full
        || report.narrowed_surface_count != narrowed
        || report.blocked_surface_count != blocked
        || report.all_surfaces_publishable != (blocked == 0)
    {
        errors.push(CertificationValidationError::SurfaceCountsStale);
    }

    // Recompute findings and assert the declared set matches.
    let index_source_ids: Vec<(AppearanceObjectFamily, &str)> = report
        .object_model_index
        .iter()
        .map(|entry| (entry.object_family, entry.source_report_id.as_str()))
        .collect();
    let mut recomputed: Vec<CertificationBlockingFinding> =
        compute_index_findings(&report.object_model_index, &report.surfaces);
    for surface in &report.surfaces {
        recomputed.extend(surface.compute_findings(&index_source_ids));
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != report.blocking_findings {
        errors.push(CertificationValidationError::BlockingFindingsStale);
    }
    for finding in &report.blocking_findings {
        errors.push(CertificationValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(CertificationValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(CertificationValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the canonical appearance-object evidence index from each family
/// module's own constants, so the index can never drift from the families it
/// certifies.
pub fn canonical_object_model_index() -> Vec<ObjectFamilyIndexEntry> {
    vec![
        ObjectFamilyIndexEntry {
            object_family: AppearanceObjectFamily::ThemePackage,
            canonical_schema_ref: THEME_PACKAGE_SOURCE_SCHEMA_REF.to_owned(),
            vocabulary_group: "theme_class, theme_mode, provenance_class".to_owned(),
            source_contract_ref: THEME_PACKAGE_SHARED_CONTRACT_REF.to_owned(),
            source_report_id: THEME_PACKAGE_REPORT_ID.to_owned(),
            source_report_record_kind: THEME_PACKAGE_REPORT_RECORD_KIND.to_owned(),
            published_report_ref: THEME_PACKAGE_PUBLISHED_REPORT_REF.to_owned(),
            published_doc_ref: THEME_PACKAGE_PUBLISHED_DOC_REF.to_owned(),
        },
        ObjectFamilyIndexEntry {
            object_family: AppearanceObjectFamily::AppearanceSession,
            canonical_schema_ref: APPEARANCE_SESSION_SOURCE_SCHEMA_REF.to_owned(),
            vocabulary_group: "appearance_axis, live_apply_capability, atomicity_class".to_owned(),
            source_contract_ref: APPEARANCE_SESSION_SHARED_CONTRACT_REF.to_owned(),
            source_report_id: APPEARANCE_SESSION_REPORT_ID.to_owned(),
            source_report_record_kind: APPEARANCE_SESSION_REPORT_RECORD_KIND.to_owned(),
            published_report_ref: APPEARANCE_SESSION_PUBLISHED_REPORT_REF.to_owned(),
            published_doc_ref: APPEARANCE_SESSION_PUBLISHED_DOC_REF.to_owned(),
        },
        ObjectFamilyIndexEntry {
            object_family: AppearanceObjectFamily::TokenOverlay,
            canonical_schema_ref: TOKEN_OVERLAY_SOURCE_SCHEMA_REF.to_owned(),
            vocabulary_group: "override_scope, value_state, portability_class".to_owned(),
            source_contract_ref: TOKEN_OVERLAY_SHARED_CONTRACT_REF.to_owned(),
            source_report_id: TOKEN_OVERLAY_REPORT_ID.to_owned(),
            source_report_record_kind: TOKEN_OVERLAY_REPORT_RECORD_KIND.to_owned(),
            published_report_ref: TOKEN_OVERLAY_PUBLISHED_REPORT_REF.to_owned(),
            published_doc_ref: TOKEN_OVERLAY_PUBLISHED_DOC_REF.to_owned(),
        },
        ObjectFamilyIndexEntry {
            object_family: AppearanceObjectFamily::ThemeImportReport,
            canonical_schema_ref: M5_THEME_IMPORT_SOURCE_SCHEMA_REF.to_owned(),
            vocabulary_group: "mapping_state, parity_claim_state, rollback_path_class".to_owned(),
            source_contract_ref: M5_THEME_IMPORT_SHARED_CONTRACT_REF.to_owned(),
            source_report_id: M5_THEME_IMPORT_REPORT_ID.to_owned(),
            source_report_record_kind: M5_THEME_IMPORT_REPORT_RECORD_KIND.to_owned(),
            published_report_ref: M5_THEME_IMPORT_PUBLISHED_REPORT_REF.to_owned(),
            published_doc_ref: M5_THEME_IMPORT_PUBLISHED_DOC_REF.to_owned(),
        },
        ObjectFamilyIndexEntry {
            object_family: AppearanceObjectFamily::ExtensionAppearanceDescriptor,
            canonical_schema_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_REF.to_owned(),
            vocabulary_group: "inheritance_axis, inheritance_state, parity_claim_state".to_owned(),
            source_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF.to_owned(),
            source_report_id: EXTENSION_APPEARANCE_AUDIT_ID.to_owned(),
            source_report_record_kind: EXTENSION_APPEARANCE_AUDIT_RECORD_KIND.to_owned(),
            published_report_ref: EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_REPORT_REF.to_owned(),
            published_doc_ref: EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_DOC_REF.to_owned(),
        },
    ]
}

/// Compact description of one certified family before its refs are filled in.
struct FamilySeed {
    family: AppearanceObjectFamily,
    status: M5QualificationStatus,
    compatibility_state: AppearanceCompatibilityState,
    freshness: M5EvidenceFreshness,
    reason: Option<&'static str>,
}

impl FamilySeed {
    /// A family certified at full fidelity with fresh evidence.
    const fn current(family: AppearanceObjectFamily) -> Self {
        Self {
            family,
            status: M5QualificationStatus::Qualified,
            compatibility_state: AppearanceCompatibilityState::Current,
            freshness: M5EvidenceFreshness::Fresh,
            reason: None,
        }
    }

    /// A family that does not apply to this surface, disclosed with a reason.
    const fn not_applicable(family: AppearanceObjectFamily, reason: &'static str) -> Self {
        Self {
            family,
            status: M5QualificationStatus::NotApplicable,
            compatibility_state: AppearanceCompatibilityState::Current,
            freshness: M5EvidenceFreshness::Fresh,
            reason: Some(reason),
        }
    }

    fn expand(&self, surface_slug: &str) -> FamilyCertification {
        let downgrade_disclosed = !self.compatibility_state.is_current();
        FamilyCertification {
            object_family: self.family,
            certification_status: self.status,
            compatibility_state: self.compatibility_state,
            downgrade_disclosed,
            evidence_freshness: self.freshness,
            source_report_id: self.family.source_report_id().to_owned(),
            evidence_refs: vec![format!(
                "{}#certified:{surface_slug}",
                self.family.source_report_id()
            )],
            narrowing_reason: self.reason.map(str::to_owned),
        }
    }
}

/// Compact description of one claimed surface before its certification is built.
struct SurfaceSeed {
    slug: &'static str,
    surface_family: M5AppearanceSurfaceFamily,
    lifecycle: SurfaceLifecycle,
    title: &'static str,
    families: &'static [FamilySeed],
    narrowing_reason: Option<&'static str>,
    narrative: &'static str,
}

impl SurfaceSeed {
    fn expand(&self) -> SurfaceCertification {
        let family_certifications: Vec<FamilyCertification> = self
            .families
            .iter()
            .map(|seed| seed.expand(self.slug))
            .collect();
        let certification_id = format!("appearance-cert:{}", self.slug);
        let mut surface = SurfaceCertification {
            record_kind: M5_APPEARANCE_CERT_SURFACE_RECORD_KIND.to_owned(),
            schema_version: M5_APPEARANCE_CERT_SCHEMA_VERSION,
            shared_contract_ref: M5_APPEARANCE_CERT_SHARED_CONTRACT_REF.to_owned(),
            certification_id,
            title: self.title.to_owned(),
            surface_family: self.surface_family,
            surface_lifecycle: self.lifecycle,
            family_certifications,
            // Recomputed by the builder; the seed value is the derived scope.
            certified_claim_scope: CertifiedClaimScope::CertifiedFull,
            all_families_current: true,
            narrowing_reason: self.narrowing_reason.map(str::to_owned),
            release_center_ref: format!("release_center.appearance.{}", self.slug),
            help_about_ref: format!("help_about.appearance.{}", self.slug),
            diagnostics_ref: format!("diagnostics.appearance.{}", self.slug),
            support_export_ref: format!("support_export.appearance.{}", self.slug),
            claim_publication_ref: format!("claim_publication.appearance.{}", self.slug),
            docs_help_refs: vec![M5_APPEARANCE_CERT_PUBLISHED_DOC_REF.to_owned()],
            narrative: self.narrative.to_owned(),
        };
        surface.certified_claim_scope = surface.recompute_claim_scope();
        surface.all_families_current = surface.compute_all_families_current();
        surface
    }
}

/// A family certification carrying an honestly narrowed family for the seed.
const fn narrowed_family(family: AppearanceObjectFamily, reason: &'static str) -> FamilySeed {
    FamilySeed {
        family,
        status: M5QualificationStatus::ExplicitlyNarrowed,
        compatibility_state: AppearanceCompatibilityState::UnsupportedSlot,
        freshness: M5EvidenceFreshness::Fresh,
        reason: Some(reason),
    }
}

/// A certified family carrying a disclosed downgrade for the seed.
const fn downgraded_family(
    family: AppearanceObjectFamily,
    compatibility_state: AppearanceCompatibilityState,
    reason: &'static str,
) -> FamilySeed {
    FamilySeed {
        family,
        status: M5QualificationStatus::Qualified,
        compatibility_state,
        freshness: M5EvidenceFreshness::Fresh,
        reason: Some(reason),
    }
}

const EXTENSION_NA_REASON: &str =
    "Host-rendered surface; no extension or embedded webview hosts this appearance.";

/// Builds the seeded per-surface certifications.
fn seeded_surfaces() -> Vec<SurfaceCertification> {
    use AppearanceObjectFamily::*;

    const SEEDS: &[SurfaceSeed] = &[
        SurfaceSeed {
            slug: "notebook-cell-chrome",
            surface_family: M5AppearanceSurfaceFamily::NotebookCellChrome,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Notebook cell chrome",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::not_applicable(ExtensionAppearanceDescriptor, EXTENSION_NA_REASON),
            ],
            narrowing_reason: None,
            narrative:
                "Notebook cell chrome certifies its theme package, live appearance session, \
                        token overlays, and imported-theme parity at full fidelity; it hosts no \
                        extension, so extension inheritance is disclosed as not applicable.",
        },
        SurfaceSeed {
            slug: "result-grid-row",
            surface_family: M5AppearanceSurfaceFamily::ResultGridRow,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Result-grid row",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::not_applicable(ExtensionAppearanceDescriptor, EXTENSION_NA_REASON),
            ],
            narrowing_reason: None,
            narrative:
                "Result-grid rows certify all four host appearance objects; severity coloring \
                        survives every certified theme and overlay.",
        },
        SurfaceSeed {
            slug: "profiler-panel",
            surface_family: M5AppearanceSurfaceFamily::ProfilerPanel,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Profiler panel",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::not_applicable(ExtensionAppearanceDescriptor, EXTENSION_NA_REASON),
            ],
            narrowing_reason: None,
            narrative:
                "The profiler flame panel certifies its appearance objects with fresh evidence \
                        bound to the exact build.",
        },
        SurfaceSeed {
            slug: "trace-panel",
            surface_family: M5AppearanceSurfaceFamily::TracePanel,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Trace panel",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::not_applicable(ExtensionAppearanceDescriptor, EXTENSION_NA_REASON),
            ],
            narrowing_reason: None,
            narrative:
                "Trace-span severity stays above the contrast threshold across every certified \
                        appearance object.",
        },
        SurfaceSeed {
            slug: "pipeline-card",
            surface_family: M5AppearanceSurfaceFamily::PipelineCard,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Pipeline card",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::not_applicable(ExtensionAppearanceDescriptor, EXTENSION_NA_REASON),
            ],
            narrowing_reason: None,
            narrative:
                "Review / pipeline status cards certify their appearance objects; lifecycle \
                        cues do not depend on the OS accent for meaning.",
        },
        SurfaceSeed {
            slug: "preview-route-badge",
            surface_family: M5AppearanceSurfaceFamily::PreviewRouteBadge,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Preview-route badge (embedded)",
            families: &[
                FamilySeed::current(ThemePackage),
                downgraded_family(
                    AppearanceSession,
                    AppearanceCompatibilityState::RestartOrReloadRequired,
                    "Forced-colors re-theme on the embedded preview reloads the surface from one \
                     checkpoint; the reload posture is disclosed up front.",
                ),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::current(ExtensionAppearanceDescriptor),
            ],
            narrowing_reason: Some(
                "Embedded preview certifies every appearance object, but its appearance session \
                 discloses a reload-required posture for forced-colors, so the surface markets the \
                 narrowed appearance story.",
            ),
            narrative:
                "The embedded preview-route badge inherits Aureline appearance and certifies \
                        all five families; the live appearance session discloses a reload-required \
                        posture, auto-narrowing the surface's claim.",
        },
        SurfaceSeed {
            slug: "docs-browser-pane",
            surface_family: M5AppearanceSurfaceFamily::DocsBrowserPane,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Docs / browser pane (embedded)",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                downgraded_family(
                    ExtensionAppearanceDescriptor,
                    AppearanceCompatibilityState::PartialInheritance,
                    "The embedded docs pane inherits theme, focus, and contrast but discloses \
                     partial density inheritance.",
                ),
            ],
            narrowing_reason: Some(
                "The embedded docs/help pane certifies its host appearance objects and discloses \
                 partial extension density inheritance, so the surface markets the narrowed story.",
            ),
            narrative: "The embedded docs/browser pane certifies its host appearance objects and \
                        discloses partial density inheritance for the extension surface, \
                        auto-narrowing the claim.",
        },
        SurfaceSeed {
            slug: "companion-surface",
            surface_family: M5AppearanceSurfaceFamily::CompanionSurface,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Companion surface",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::not_applicable(ExtensionAppearanceDescriptor, EXTENSION_NA_REASON),
            ],
            narrowing_reason: None,
            narrative:
                "Companion / cross-device surfaces certify their appearance objects; reduce-motion \
                        downgrades presence transitions through the certified appearance session.",
        },
        SurfaceSeed {
            slug: "sync-status-surface",
            surface_family: M5AppearanceSurfaceFamily::SyncStatusSurface,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Sync status surface",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                FamilySeed::current(TokenOverlay),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::not_applicable(ExtensionAppearanceDescriptor, EXTENSION_NA_REASON),
            ],
            narrowing_reason: None,
            narrative:
                "Workspace sync status certifies its appearance objects, including imported-theme \
                        parity for themes synced in from other workspaces.",
        },
        SurfaceSeed {
            slug: "offboarding-surface",
            surface_family: M5AppearanceSurfaceFamily::OffboardingSurface,
            lifecycle: SurfaceLifecycle::Stable,
            title: "Offboarding surface",
            families: &[
                FamilySeed::current(ThemePackage),
                FamilySeed::current(AppearanceSession),
                narrowed_family(
                    TokenOverlay,
                    "Export-and-wipe offboarding renders from the base token set; portable token \
                     overlays are not round-tripped here yet, so the family is narrowed honestly.",
                ),
                FamilySeed::current(ThemeImportReport),
                FamilySeed::not_applicable(ExtensionAppearanceDescriptor, EXTENSION_NA_REASON),
            ],
            narrowing_reason: Some(
                "The offboarding / export-and-wipe surface narrows token-overlay round-trip and \
                 markets only the narrowed appearance story.",
            ),
            narrative:
                "The offboarding surface certifies most appearance objects but honestly narrows \
                        token-overlay round-trip, auto-narrowing the surface's claim scope.",
        },
    ];

    SEEDS.iter().map(SurfaceSeed::expand).collect()
}

/// Builds the seeded appearance-object certification report.
///
/// Uses the frozen [`SEED_BUILD_IDENTITY_REF`] so the checked-in fixtures stay
/// reproducible. A live runtime would call
/// [`build_appearance_object_certification_report`] with
/// [`aureline_build_info::exact_build_identity_ref`] instead.
pub fn seeded_appearance_object_certification_report() -> AppearanceObjectCertificationReport {
    build_appearance_object_certification_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        canonical_object_model_index(),
        seeded_surfaces(),
    )
}
