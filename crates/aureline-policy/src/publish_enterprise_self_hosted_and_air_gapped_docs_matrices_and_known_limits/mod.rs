//! Publish enterprise, self-hosted, and air-gapped docs, matrices, and
//! known-limits with current proof.
//!
//! This module produces a beta proof packet that demonstrates, for each
//! required enterprise deployment profile:
//!
//! 1. **Docs are current and explicit** — every non-local profile carries a
//!    docs-completeness token proving that enterprise, self-hosted, managed,
//!    and air-gapped documentation exists and is not stale.
//! 2. **Capability matrices are complete** — every non-local profile carries
//!    a matrix-completeness token proving that the capability matrix for that
//!    profile is published and covers all claimed features.
//! 3. **Known limits are fully disclosed** — every non-local profile carries
//!    a known-limit-completeness token proving that limitations, exclusions,
//!    and downgrade behavior are published rather than hidden.
//! 4. **Local-core continuity is explicit** — every row carries an explicit
//!    local-core continuity posture; enterprise features must not block local
//!    editing, save, search, or Git by default.
//! 5. **Proof is current, not aspirational** — self-hosted and air-gapped
//!    rows carry `proof_currency: current`; aspirational proof is a hard
//!    guardrail that withdraws the row.
//! 6. **Tenant/region ownership, policy source, and dependency class are
//!    visible** — every non-local enterprise profile row declares who owns
//!    the tenant/region, where policy originates, and what dependency class
//!    governs the profile.
//!
//! One condition forces `Withdrawn` immediately and cannot be overridden:
//!
//! - A row where [`LocalCoreContinuityPostureClass::BlockedByDefault`] is the
//!   stated posture (narrow reason:
//!   [`EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::LocalCoreBlockedByDefault`]).
//!   Enterprise features must not block local-core work by default.
//!
//! A second condition also forces `Withdrawn`:
//!
//! - A self-hosted or air-gapped row where [`ProofCurrencyClass::Aspirational`]
//!   is declared (narrow reason:
//!   [`EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::AspirationalProofOnSovereignProfile`]).
//!   Sovereignty claims require current proof, not roadmap promises.
//!
//! Surfaces (admin console, support export, shell trust center, headless
//! inspector, Help/About) read
//! [`seeded_enterprise_docs_matrices_known_limits_page`] rather than minting
//! parallel docs checks. The seed covers all five required enterprise profiles
//! ([`EnterpriseProfileClass::ALL`]).
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs.md`
//! - Artifact: `artifacts/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs.md`
//! - Contract ref: [`ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SHARED_CONTRACT_REF: &str =
    "policy:enterprise_docs_matrices_known_limits:v1";

/// Record-kind tag for [`EnterpriseDocsMatricesKnownLimitsPage`] payloads.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_PAGE_RECORD_KIND: &str =
    "policy_enterprise_docs_matrices_known_limits_page_record";

/// Record-kind tag for [`EnterpriseDocsMatricesKnownLimitsRow`] payloads.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_ROW_RECORD_KIND: &str =
    "policy_enterprise_docs_matrices_known_limits_row_record";

/// Record-kind tag for [`EnterpriseDocsMatricesKnownLimitsDefect`] payloads.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_DEFECT_RECORD_KIND: &str =
    "policy_enterprise_docs_matrices_known_limits_defect_record";

/// Record-kind tag for [`EnterpriseDocsMatricesKnownLimitsSummary`] payloads.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SUMMARY_RECORD_KIND: &str =
    "policy_enterprise_docs_matrices_known_limits_summary_record";

/// Record-kind tag for [`EnterpriseDocsMatricesKnownLimitsSupportExport`] payloads.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_enterprise_docs_matrices_known_limits_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_DOC_REF: &str =
    "docs/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs.md";

/// Repo-relative path of the artifact summary for this lane.
pub const ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs.md";

// ---------------------------------------------------------------------------
// Enterprise profile vocabulary
// ---------------------------------------------------------------------------

/// Enterprise deployment profile covered by the docs/matrices/known-limits row.
///
/// Uses the same token vocabulary as `deployment_profile` in the deployment
/// summary card so every row can be correlated with its residency proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseProfileClass {
    /// Desktop-local, single-user, no managed control plane.
    IndividualLocal,
    /// Customer-operated control plane with customer-managed keys and region.
    SelfHosted,
    /// Hybrid remote-attach with vendor-provided managed services.
    EnterpriseOnline,
    /// Offline-capable air-gapped mirror; no public egress.
    AirGapped,
    /// Vendor-operated SaaS with vendor-managed keys by default.
    ManagedCloud,
}

impl EnterpriseProfileClass {
    /// All required enterprise profiles in canonical order.
    pub const ALL: [Self; 5] = [
        Self::IndividualLocal,
        Self::SelfHosted,
        Self::EnterpriseOnline,
        Self::AirGapped,
        Self::ManagedCloud,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::AirGapped => "air_gapped",
            Self::ManagedCloud => "managed_cloud",
        }
    }

    /// True when this profile has no managed control plane and therefore no
    /// enterprise docs/matrix/known-limit scope beyond local help.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::IndividualLocal)
    }

    /// True when tenant/region ownership and policy source must be declared.
    pub const fn requires_tenant_region_declaration(self) -> bool {
        !self.is_local_only()
    }

    /// True when this profile claims sovereignty (self-hosted or air-gapped)
    /// and therefore must carry current proof rather than aspirational claims.
    pub const fn claims_sovereignty(self) -> bool {
        matches!(self, Self::SelfHosted | Self::AirGapped)
    }
}

// ---------------------------------------------------------------------------
// Docs completeness vocabulary
// ---------------------------------------------------------------------------

/// Current state of documentation for the enterprise profile.
///
/// Reported from the declared docs posture; the verifier does not inspect
/// raw doc artefacts, only the declared state tokens and opaque refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsCompletenessClass {
    /// Docs are current and cover all claimed capabilities for this profile.
    Current,
    /// Docs exist but are stale (last update outside the declared freshness
    /// window).
    Stale,
    /// Docs are missing for one or more claimed capability areas.
    Missing,
    /// No enterprise docs scope exists for this profile; docs do not apply.
    NotApplicable,
}

impl DocsCompletenessClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the docs state indicates a gap that narrows the row.
    pub const fn is_deficient(self) -> bool {
        matches!(self, Self::Stale | Self::Missing)
    }
}

// ---------------------------------------------------------------------------
// Matrix completeness vocabulary
// ---------------------------------------------------------------------------

/// Current state of the capability matrix for the enterprise profile.
///
/// A matrix that is incomplete or missing for a claimed capability area
/// narrows the row to beta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixCompletenessClass {
    /// Matrix is complete and covers all claimed capabilities.
    Complete,
    /// Matrix is partially complete; some claimed capabilities lack matrix rows.
    Partial,
    /// Matrix is missing for this profile.
    Missing,
    /// No enterprise matrix scope exists for this profile.
    NotApplicable,
}

impl MatrixCompletenessClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the matrix state indicates a gap that narrows the row.
    pub const fn is_deficient(self) -> bool {
        matches!(self, Self::Partial | Self::Missing)
    }
}

// ---------------------------------------------------------------------------
// Known-limit completeness vocabulary
// ---------------------------------------------------------------------------

/// Current state of known-limit disclosure for the enterprise profile.
///
/// Known limits must be fully disclosed; hidden limitations or undisclosed
/// exclusions narrow the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitCompletenessClass {
    /// All known limits are fully disclosed for this profile.
    FullyDisclosed,
    /// Some known limits are disclosed, but at least one claimed capability
    /// area lacks disclosed limitations.
    PartiallyDisclosed,
    /// Known limits are missing or undisclosed for this profile.
    Undisclosed,
    /// No enterprise known-limit scope exists for this profile.
    NotApplicable,
}

impl KnownLimitCompletenessClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyDisclosed => "fully_disclosed",
            Self::PartiallyDisclosed => "partially_disclosed",
            Self::Undisclosed => "undisclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the known-limit state indicates a gap that narrows the row.
    pub const fn is_deficient(self) -> bool {
        matches!(self, Self::PartiallyDisclosed | Self::Undisclosed)
    }
}

// ---------------------------------------------------------------------------
// Local-core continuity posture vocabulary
// ---------------------------------------------------------------------------

/// Explicit statement of local-core continuity for this enterprise profile.
///
/// Every row must carry an explicit posture token; a missing or ambiguous
/// token narrows the row. The local-editing floor is the floor below which no
/// enterprise feature or profile switch may reduce capability without an
/// explicit user decision and a visible downgrade label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCoreContinuityPostureClass {
    /// The local editing floor is fully preserved for this profile; no managed
    /// capability change or profile switch may remove it without explicit user
    /// consent.
    Preserved,
    /// A managed dependency may degrade some local-core capabilities under
    /// specific conditions, but the local editing floor is still intact; the
    /// dependency and conditions are named explicitly.
    ImpairedManagedDependency,
    /// The profile blocks local-core capabilities by default. This is a hard
    /// guardrail violation and withdraws the row.
    BlockedByDefault,
}

impl LocalCoreContinuityPostureClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::ImpairedManagedDependency => "impaired_managed_dependency",
            Self::BlockedByDefault => "blocked_by_default",
        }
    }

    /// True when this posture triggers immediate withdrawal.
    pub const fn is_withdrawal_trigger(self) -> bool {
        matches!(self, Self::BlockedByDefault)
    }
}

// ---------------------------------------------------------------------------
// Proof currency vocabulary
// ---------------------------------------------------------------------------

/// Currency of the proof backing claims for self-hosted and air-gapped
/// profiles.
///
/// Sovereignty claims (self-hosted, air-gapped) require current proof.
/// Aspirational proof — roadmap promises without current evidence — is a hard
/// guardrail violation that withdraws the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofCurrencyClass {
    /// Proof is current and verified.
    Current,
    /// Proof exists but is stale (outside the declared validity window).
    Stale,
    /// Proof is aspirational — a roadmap promise without current evidence.
    /// This is a hard guardrail for sovereignty profiles.
    Aspirational,
    /// No proof scope applies (local-only or non-sovereignty profiles).
    NotApplicable,
}

impl ProofCurrencyClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Aspirational => "aspirational",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this proof currency triggers immediate withdrawal for
    /// sovereignty profiles.
    pub const fn is_withdrawal_trigger(self) -> bool {
        matches!(self, Self::Aspirational)
    }

    /// True when this proof currency narrows a sovereignty profile to beta.
    pub const fn is_deficient(self) -> bool {
        matches!(self, Self::Stale)
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Qualification tier for the publish page and its rows.
///
/// The tier is derived, not asserted: it is set by the audit against the
/// required conditions. A caller may never assert `stable` without a clean
/// audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseDocsMatricesKnownLimitsQualificationClass {
    /// All required conditions hold.
    Stable,
    /// One or more non-critical conditions are unmet.
    Beta,
    /// A required enterprise profile has no row; coverage gap prevents a beta
    /// claim.
    Preview,
    /// A hard guardrail was triggered; the page is withdrawn immediately.
    Withdrawn,
}

impl EnterpriseDocsMatricesKnownLimitsQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`EnterpriseDocsMatricesKnownLimitsQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseDocsMatricesKnownLimitsNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// The local-core continuity posture is `blocked_by_default`. This is a
    /// hard guardrail and withdraws the packet.
    LocalCoreBlockedByDefault,
    /// A sovereignty profile (self-hosted or air-gapped) carries aspirational
    /// proof rather than current evidence. This is a hard guardrail and
    /// withdraws the packet.
    AspirationalProofOnSovereignProfile,
    /// Local-core continuity posture is not explicitly stated on a row.
    LocalCoreContinuityNotExplicit,
    /// Docs are stale for a profile that claims enterprise docs.
    DocsStale,
    /// Docs are missing for a profile that claims enterprise docs.
    DocsMissing,
    /// Capability matrix is partial for a profile that claims enterprise
    /// capabilities.
    MatrixPartial,
    /// Capability matrix is missing for a profile that claims enterprise
    /// capabilities.
    MatrixMissing,
    /// Known limits are only partially disclosed for a profile.
    KnownLimitsPartiallyDisclosed,
    /// Known limits are undisclosed for a profile.
    KnownLimitsUndisclosed,
    /// Tenant/region ownership is not declared for a non-local enterprise
    /// profile.
    TenantRegionOwnershipNotDeclared,
    /// Policy source is not declared for a non-local enterprise profile.
    PolicySourceNotDeclared,
    /// Dependency class is not declared for a non-local enterprise profile.
    DependencyClassNotDeclared,
    /// Proof is stale for a sovereignty profile.
    ProofStale,
    /// A required enterprise profile has no row; narrows to preview.
    ProfileCoverageGap,
}

impl EnterpriseDocsMatricesKnownLimitsNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::LocalCoreBlockedByDefault => "local_core_blocked_by_default",
            Self::AspirationalProofOnSovereignProfile => "aspirational_proof_on_sovereign_profile",
            Self::LocalCoreContinuityNotExplicit => "local_core_continuity_not_explicit",
            Self::DocsStale => "docs_stale",
            Self::DocsMissing => "docs_missing",
            Self::MatrixPartial => "matrix_partial",
            Self::MatrixMissing => "matrix_missing",
            Self::KnownLimitsPartiallyDisclosed => "known_limits_partially_disclosed",
            Self::KnownLimitsUndisclosed => "known_limits_undisclosed",
            Self::TenantRegionOwnershipNotDeclared => "tenant_region_ownership_not_declared",
            Self::PolicySourceNotDeclared => "policy_source_not_declared",
            Self::DependencyClassNotDeclared => "dependency_class_not_declared",
            Self::ProofStale => "proof_stale",
            Self::ProfileCoverageGap => "profile_coverage_gap",
        }
    }

    /// True when this reason triggers immediate withdrawal and cannot be
    /// overridden.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::LocalCoreBlockedByDefault | Self::AspirationalProofOnSovereignProfile
        )
    }
}

// ---------------------------------------------------------------------------
// Row-level declarations
// ---------------------------------------------------------------------------

/// Docs declaration for one enterprise profile row.
///
/// All fields are opaque refs or closed-vocabulary tokens; raw doc URLs,
/// raw hostnames, and raw credentials stay outside the support boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsDeclaration {
    /// Current docs completeness state for this profile.
    pub docs_state: DocsCompletenessClass,
    /// Stable token for [`Self::docs_state`].
    pub docs_state_token: String,
    /// Opaque ref to the last verified docs build artefact.
    /// Empty when `docs_state` is `not_applicable`.
    pub last_docs_build_ref: String,
    /// ISO 8601 timestamp reference for the last docs build.
    /// Empty when `docs_state` is `not_applicable`.
    pub last_docs_build_time: String,
    /// Declared freshness window token (e.g., `rolling_30d`, `per_milestone`).
    pub freshness_window_token: String,
    /// Plain-language labels for doc scopes covered for this profile.
    pub covered_doc_scope_labels: Vec<String>,
    /// Plain-language labels for doc scopes explicitly missing with a brief reason.
    pub missing_doc_scope_labels: Vec<String>,
}

impl DocsDeclaration {
    /// True when docs fields are fully declared for profiles that require them.
    pub fn is_declared_for_profile(&self, profile: EnterpriseProfileClass) -> bool {
        if profile.is_local_only() {
            return true;
        }
        self.docs_state != DocsCompletenessClass::Missing
            && !self.freshness_window_token.is_empty()
    }
}

/// Capability matrix declaration for one enterprise profile row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixDeclaration {
    /// Current matrix completeness state for this profile.
    pub matrix_state: MatrixCompletenessClass,
    /// Stable token for [`Self::matrix_state`].
    pub matrix_state_token: String,
    /// Opaque ref to the latest published matrix artefact.
    /// Empty when `matrix_state` is `not_applicable`.
    pub matrix_ref: String,
    /// ISO 8601 timestamp reference for the last matrix publication.
    /// Empty when `matrix_state` is `not_applicable`.
    pub last_published_time: String,
    /// Plain-language labels for capability families covered in the matrix.
    pub covered_capability_labels: Vec<String>,
    /// Plain-language labels for capability families missing from the matrix.
    pub missing_capability_labels: Vec<String>,
}

impl MatrixDeclaration {
    /// True when the matrix declaration is complete for profiles that require it.
    pub fn is_declared_for_profile(&self, profile: EnterpriseProfileClass) -> bool {
        if profile.is_local_only() {
            return true;
        }
        self.matrix_state != MatrixCompletenessClass::Missing
            && !self.matrix_ref.is_empty()
    }
}

/// Known-limits declaration for one enterprise profile row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitsDeclaration {
    /// Current known-limit disclosure state for this profile.
    pub known_limits_state: KnownLimitCompletenessClass,
    /// Stable token for [`Self::known_limits_state`].
    pub known_limits_state_token: String,
    /// Number of known-limit notes disclosed for this profile.
    pub disclosed_limit_count: usize,
    /// Number of known-limit notes that remain undisclosed.
    pub undisclosed_limit_count: usize,
    /// Opaque ref to the canonical known-limit index for this profile.
    /// Empty when `known_limits_state` is `not_applicable`.
    pub known_limit_index_ref: String,
    /// Plain-language labels for limitation classes disclosed.
    pub disclosed_limitation_classes: Vec<String>,
    /// Plain-language labels for limitation classes that remain undisclosed.
    pub undisclosed_limitation_classes: Vec<String>,
}

impl KnownLimitsDeclaration {
    /// True when the known-limits declaration is complete for profiles that
    /// require it.
    pub fn is_declared_for_profile(&self, profile: EnterpriseProfileClass) -> bool {
        if profile.is_local_only() {
            return true;
        }
        self.known_limits_state != KnownLimitCompletenessClass::Undisclosed
            && !self.known_limit_index_ref.is_empty()
    }
}

/// Proof-currency declaration for one enterprise profile row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofCurrencyDeclaration {
    /// Current proof currency for this profile.
    pub proof_currency: ProofCurrencyClass,
    /// Stable token for [`Self::proof_currency`].
    pub proof_currency_token: String,
    /// Opaque ref to the current proof packet backing claims for this profile.
    /// Empty when `proof_currency` is `not_applicable`.
    pub proof_packet_ref: String,
    /// ISO 8601 timestamp reference for the last proof verification.
    /// Empty when `proof_currency` is `not_applicable`.
    pub last_verified_time: String,
    /// Declared proof validity window token.
    pub proof_validity_window_token: String,
}

impl ProofCurrencyDeclaration {
    /// True when the proof currency declaration is complete for profiles that
    /// require it.
    pub fn is_declared_for_profile(&self, profile: EnterpriseProfileClass) -> bool {
        if !profile.claims_sovereignty() {
            return true;
        }
        !self.proof_packet_ref.is_empty() && !self.proof_validity_window_token.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Row, summary, defect
// ---------------------------------------------------------------------------

/// Publish row for one enterprise deployment profile.
///
/// The row is the unit of qualification. Each row must carry a fully declared
/// docs state, matrix state, known-limits state, local-core continuity posture,
/// and proof currency. Failure on any required condition narrows the row and
/// the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDocsMatricesKnownLimitsRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Enterprise profile this row covers.
    pub enterprise_profile: EnterpriseProfileClass,
    /// Stable token for [`Self::enterprise_profile`].
    pub enterprise_profile_token: String,
    /// Docs declaration for this profile.
    pub docs: DocsDeclaration,
    /// Matrix declaration for this profile.
    pub matrix: MatrixDeclaration,
    /// Known-limits declaration for this profile.
    pub known_limits: KnownLimitsDeclaration,
    /// Proof-currency declaration for this profile.
    pub proof_currency: ProofCurrencyDeclaration,
    /// Explicit local-core continuity posture for this profile.
    pub local_core_posture: LocalCoreContinuityPostureClass,
    /// Stable token for [`Self::local_core_posture`].
    pub local_core_posture_token: String,
    /// Opaque ref identifying the tenant/org scope owner for this profile.
    /// Empty for `individual_local`.
    pub tenant_region_owner_ref: String,
    /// Opaque ref identifying the policy source that governs docs, matrix, and
    /// known-limit publication for this profile.
    /// Empty for `individual_local`.
    pub policy_source_ref: String,
    /// Declared dependency class token for this profile's docs/matrix/known-limit
    /// path. Empty for `individual_local`.
    pub dependency_class_token: String,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

/// Aggregate summary for the publish page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EnterpriseDocsMatricesKnownLimitsSummary {
    /// Total row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Enterprise profile tokens present on the page.
    pub profiles_covered: Vec<String>,
    /// Number of rows with current docs.
    pub docs_current_row_count: usize,
    /// Number of rows with complete matrices.
    pub matrix_complete_row_count: usize,
    /// Number of rows with fully disclosed known limits.
    pub known_limits_fully_disclosed_row_count: usize,
    /// Number of rows with current proof currency.
    pub proof_current_row_count: usize,
    /// Number of rows with local-core continuity explicitly preserved.
    pub local_core_preserved_row_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl EnterpriseDocsMatricesKnownLimitsSummary {
    fn from_rows(rows: &[EnterpriseDocsMatricesKnownLimitsRow]) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut profiles: BTreeSet<String> = BTreeSet::new();
        let mut docs_current = 0usize;
        let mut matrix_complete = 0usize;
        let mut known_limits_disclosed = 0usize;
        let mut proof_current = 0usize;
        let mut local_core_preserved = 0usize;

        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            profiles.insert(row.enterprise_profile_token.clone());
            if row.docs.docs_state == DocsCompletenessClass::Current
                || row.docs.docs_state == DocsCompletenessClass::NotApplicable
            {
                docs_current += 1;
            }
            if row.matrix.matrix_state == MatrixCompletenessClass::Complete
                || row.matrix.matrix_state == MatrixCompletenessClass::NotApplicable
            {
                matrix_complete += 1;
            }
            if row.known_limits.known_limits_state == KnownLimitCompletenessClass::FullyDisclosed
                || row.known_limits.known_limits_state == KnownLimitCompletenessClass::NotApplicable
            {
                known_limits_disclosed += 1;
            }
            if row.proof_currency.proof_currency == ProofCurrencyClass::Current
                || row.proof_currency.proof_currency == ProofCurrencyClass::NotApplicable
            {
                proof_current += 1;
            }
            if row.local_core_posture == LocalCoreContinuityPostureClass::Preserved
                || row.local_core_posture == LocalCoreContinuityPostureClass::ImpairedManagedDependency
            {
                local_core_preserved += 1;
            }
        }

        let overall = if withdrawn > 0 {
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn
        } else if preview > 0 {
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Preview
        } else if beta > 0 {
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta
        } else {
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Stable
        };

        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            profiles_covered: profiles.into_iter().collect(),
            docs_current_row_count: docs_current,
            matrix_complete_row_count: matrix_complete,
            known_limits_fully_disclosed_row_count: known_limits_disclosed,
            proof_current_row_count: proof_current,
            local_core_preserved_row_count: local_core_preserved,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

/// Typed defect emitted by the publish-page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDocsMatricesKnownLimitsDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: EnterpriseDocsMatricesKnownLimitsNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (row id or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl EnterpriseDocsMatricesKnownLimitsDefect {
    fn new(
        narrow_reason: EnterpriseDocsMatricesKnownLimitsNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_DEFECT_RECORD_KIND.to_owned(),
            schema_version: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SCHEMA_VERSION,
            shared_contract_ref: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SHARED_CONTRACT_REF
                .to_owned(),
            defect_id: format!(
                "policy:defect:enterprise-docs-matrices-known-limits:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Beta proof packet for enterprise, self-hosted, and air-gapped docs,
/// matrices, and known-limits.
///
/// This is the single inspectable record that proves the claims for this lane.
/// Dashboards, docs, Help/About surfaces, and support exports should ingest it
/// rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDocsMatricesKnownLimitsPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from all rows.
    pub summary: EnterpriseDocsMatricesKnownLimitsSummary,
    /// Per-profile qualification rows (one per enterprise profile).
    pub rows: Vec<EnterpriseDocsMatricesKnownLimitsRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<EnterpriseDocsMatricesKnownLimitsDefect>,
}

impl EnterpriseDocsMatricesKnownLimitsPage {
    /// Build the publish page from a set of rows.
    ///
    /// Defects are derived automatically from the audit. Rows are
    /// re-qualified based on the combined audit result.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<EnterpriseDocsMatricesKnownLimitsRow>,
    ) -> Self {
        let defects = audit_enterprise_docs_matrices_known_limits_rows(&rows);
        let qualified_rows = qualify_rows(rows, &defects);
        let summary = EnterpriseDocsMatricesKnownLimitsSummary::from_rows(&qualified_rows);
        Self {
            record_kind: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_PAGE_RECORD_KIND.to_owned(),
            schema_version: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SCHEMA_VERSION,
            shared_contract_ref: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SHARED_CONTRACT_REF
                .to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows: qualified_rows,
            defects,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when all required enterprise profiles are covered.
    pub fn covers_all_required_profiles(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .map(|r| r.enterprise_profile_token.as_str())
            .collect();
        EnterpriseProfileClass::ALL
            .iter()
            .all(|p| covered.contains(p.as_str()))
    }

    /// True when all rows carry an explicitly preserved local-core continuity
    /// posture.
    pub fn all_rows_preserve_local_core(&self) -> bool {
        self.rows.iter().all(|r| {
            r.local_core_posture == LocalCoreContinuityPostureClass::Preserved
                || r.local_core_posture == LocalCoreContinuityPostureClass::ImpairedManagedDependency
        })
    }

    /// True when no row declares a blocking local-core posture.
    pub fn no_row_blocks_local_core(&self) -> bool {
        self.rows
            .iter()
            .all(|r| !r.local_core_posture.is_withdrawal_trigger())
    }

    /// True when all sovereignty profiles carry current proof.
    pub fn all_sovereignty_profiles_have_current_proof(&self) -> bool {
        self.rows.iter().all(|r| {
            !r.enterprise_profile.claims_sovereignty()
                || r.proof_currency.proof_currency == ProofCurrencyClass::Current
        })
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the publish page plus a metadata-safe
/// defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDocsMatricesKnownLimitsSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The publish page embedded as evidence.
    pub page: EnterpriseDocsMatricesKnownLimitsPage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<EnterpriseDocsMatricesKnownLimitsNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw private material is excluded from the export.
    pub raw_private_material_excluded: bool,
}

impl EnterpriseDocsMatricesKnownLimitsSupportExport {
    /// Wrap a publish page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: EnterpriseDocsMatricesKnownLimitsPage,
    ) -> Self {
        let mut reasons: Vec<EnterpriseDocsMatricesKnownLimitsNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SUPPORT_EXPORT_RECORD_KIND
                .to_owned(),
            schema_version: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SCHEMA_VERSION,
            shared_contract_ref: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SHARED_CONTRACT_REF
                .to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Public audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the publish audit over the rows in the page.
pub fn audit_enterprise_docs_matrices_known_limits_page(
    page: &EnterpriseDocsMatricesKnownLimitsPage,
) -> Vec<EnterpriseDocsMatricesKnownLimitsDefect> {
    audit_enterprise_docs_matrices_known_limits_rows(&page.rows)
}

/// Validate the publish page; returns `Ok` when the audit is clean.
pub fn validate_enterprise_docs_matrices_known_limits_page(
    page: &EnterpriseDocsMatricesKnownLimitsPage,
) -> Result<(), Vec<EnterpriseDocsMatricesKnownLimitsDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Build the seeded publish page covering all five required enterprise
/// profiles with docs, matrices, known-limits, and proof currency declared.
pub fn seeded_enterprise_docs_matrices_known_limits_page() -> EnterpriseDocsMatricesKnownLimitsPage {
    EnterpriseDocsMatricesKnownLimitsPage::new(
        "policy:enterprise-docs-matrices-known-limits:seeded:0001",
        "Enterprise, self-hosted, and air-gapped docs, matrices, and known-limits \
         publish packet",
        "2026-06-01T00:00:00Z",
        seeded_rows(),
    )
}

// ---------------------------------------------------------------------------
// Internal audit helpers
// ---------------------------------------------------------------------------

fn audit_enterprise_docs_matrices_known_limits_rows(
    rows: &[EnterpriseDocsMatricesKnownLimitsRow],
) -> Vec<EnterpriseDocsMatricesKnownLimitsDefect> {
    let mut defects: Vec<EnterpriseDocsMatricesKnownLimitsDefect> = Vec::new();

    for row in rows {
        // Hard guardrail 1: local-core blocked by default.
        if row.local_core_posture.is_withdrawal_trigger() {
            defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::LocalCoreBlockedByDefault,
                row.row_id.clone(),
                "row declares local_core_posture: blocked_by_default; enterprise features must \
                 not block local-core capabilities",
            ));
            // Withdrawal defect: skip further checks for this row.
            continue;
        }

        // Hard guardrail 2: aspirational proof on sovereignty profile.
        if row.enterprise_profile.claims_sovereignty()
            && row.proof_currency.proof_currency.is_withdrawal_trigger()
        {
            defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::AspirationalProofOnSovereignProfile,
                row.row_id.clone(),
                "sovereignty profile (self-hosted or air-gapped) carries aspirational proof; \
                 sovereignty claims require current evidence, not roadmap promises",
            ));
            continue;
        }

        // Local-core continuity must be explicitly stated.
        if row.local_core_posture_token.is_empty() {
            defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::LocalCoreContinuityNotExplicit,
                row.row_id.clone(),
                "row does not carry an explicit local_core_posture_token",
            ));
        }

        // Docs state checks for enterprise profiles.
        if !row.enterprise_profile.is_local_only() {
            if row.docs.docs_state == DocsCompletenessClass::Stale {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::DocsStale,
                    row.row_id.clone(),
                    "docs are stale; the last docs build is outside the declared freshness window",
                ));
            }
            if row.docs.docs_state == DocsCompletenessClass::Missing {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::DocsMissing,
                    row.row_id.clone(),
                    "docs are missing for one or more claimed capability areas",
                ));
            }
            // Matrix completeness checks.
            if row.matrix.matrix_state == MatrixCompletenessClass::Partial {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::MatrixPartial,
                    row.row_id.clone(),
                    "capability matrix is partial; some claimed capabilities lack matrix rows",
                ));
            }
            if row.matrix.matrix_state == MatrixCompletenessClass::Missing {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::MatrixMissing,
                    row.row_id.clone(),
                    "capability matrix is missing for this profile",
                ));
            }
            // Known-limits disclosure checks.
            if row.known_limits.known_limits_state == KnownLimitCompletenessClass::PartiallyDisclosed
            {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::KnownLimitsPartiallyDisclosed,
                    row.row_id.clone(),
                    "known limits are only partially disclosed; at least one claimed capability \
                     area lacks disclosed limitations",
                ));
            }
            if row.known_limits.known_limits_state == KnownLimitCompletenessClass::Undisclosed {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::KnownLimitsUndisclosed,
                    row.row_id.clone(),
                    "known limits are undisclosed for this profile",
                ));
            }
            // Proof currency checks for sovereignty profiles.
            if row.enterprise_profile.claims_sovereignty()
                && row.proof_currency.proof_currency.is_deficient()
            {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::ProofStale,
                    row.row_id.clone(),
                    "proof is stale for a sovereignty profile; current evidence is required",
                ));
            }
            // Tenant/region ownership must be declared.
            if row.tenant_region_owner_ref.is_empty() {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::TenantRegionOwnershipNotDeclared,
                    row.row_id.clone(),
                    "tenant/region ownership ref is missing for a non-local enterprise profile",
                ));
            }
            // Policy source must be declared.
            if row.policy_source_ref.is_empty() {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::PolicySourceNotDeclared,
                    row.row_id.clone(),
                    "policy source ref is missing for a non-local enterprise profile",
                ));
            }
            // Dependency class must be declared.
            if row.dependency_class_token.is_empty() {
                defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
                    EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::DependencyClassNotDeclared,
                    row.row_id.clone(),
                    "dependency class token is missing for a non-local enterprise profile",
                ));
            }
        }
    }

    // Coverage check: all required enterprise profiles must appear.
    let required_profiles: BTreeSet<&str> = EnterpriseProfileClass::ALL
        .iter()
        .map(|p| p.as_str())
        .collect();
    let observed_profiles: BTreeSet<&str> = rows
        .iter()
        .map(|r| r.enterprise_profile_token.as_str())
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(EnterpriseDocsMatricesKnownLimitsDefect::new(
            EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::ProfileCoverageGap,
            "page",
            format!(
                "missing row for required enterprise profile '{missing}'; packet is narrowed to \
                 preview"
            ),
        ));
    }

    defects
}

fn qualify_rows(
    mut rows: Vec<EnterpriseDocsMatricesKnownLimitsRow>,
    page_defects: &[EnterpriseDocsMatricesKnownLimitsDefect],
) -> Vec<EnterpriseDocsMatricesKnownLimitsRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects.iter().any(|d| {
        d.narrow_reason == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::ProfileCoverageGap
    });

    let (overall_qual, overall_reason) = if has_withdrawal {
        let r = page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::LocalCoreBlockedByDefault);
        (
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn,
            r,
        )
    } else if has_preview {
        (
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Preview,
            EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::ProfileCoverageGap,
        )
    } else if !page_defects.is_empty() {
        let r = page_defects[0].narrow_reason;
        (EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta, r)
    } else {
        (
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Stable,
            EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::NotNarrowed,
        )
    };

    for row in &mut rows {
        let row_qual = if has_withdrawal {
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn
        } else if has_preview {
            EnterpriseDocsMatricesKnownLimitsQualificationClass::Preview
        } else {
            let row_has_defect = page_defects.iter().any(|d| d.source == row.row_id);
            if row_has_defect || !page_defects.is_empty() {
                EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta
            } else {
                EnterpriseDocsMatricesKnownLimitsQualificationClass::Stable
            }
        };

        let row_reason = if row_qual == overall_qual {
            overall_reason
        } else {
            page_defects
                .iter()
                .find(|d| d.source == row.row_id)
                .map(|d| d.narrow_reason)
                .unwrap_or(EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::NotNarrowed)
        };

        row.qualification_token = row_qual.as_str().to_owned();
        row.narrow_reason_token = row_reason.as_str().to_owned();
        row.plain_language_summary = build_row_summary(
            &row.row_id,
            &row.enterprise_profile_token,
            row_qual,
            row_reason,
        );
    }

    rows
}

fn build_row_summary(
    row_id: &str,
    profile_token: &str,
    qual: EnterpriseDocsMatricesKnownLimitsQualificationClass,
    narrow_reason: EnterpriseDocsMatricesKnownLimitsNarrowReasonClass,
) -> String {
    match qual {
        EnterpriseDocsMatricesKnownLimitsQualificationClass::Stable => format!(
            "Row '{row_id}' ({profile_token}) qualifies stable: docs current, matrix complete, \
             known limits fully disclosed, local-core continuity explicit."
        ),
        EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta => format!(
            "Row '{row_id}' ({profile_token}) narrowed to beta (reason: {}): one or more required \
             conditions are unmet.",
            narrow_reason.as_str()
        ),
        EnterpriseDocsMatricesKnownLimitsQualificationClass::Preview => format!(
            "Row '{row_id}' ({profile_token}) narrowed to preview: a required enterprise profile \
             is missing from the page."
        ),
        EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn => format!(
            "Row '{row_id}' ({profile_token}) is withdrawn (reason: {}): hard guardrail \
             triggered — enterprise feature blocks local-core work by default or sovereignty \
             claim lacks current proof.",
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded rows
// ---------------------------------------------------------------------------

fn seeded_rows() -> Vec<EnterpriseDocsMatricesKnownLimitsRow> {
    vec![
        row_individual_local(),
        row_self_hosted(),
        row_enterprise_online(),
        row_air_gapped(),
        row_managed_cloud(),
    ]
}

fn make_row(
    row_id: &str,
    profile: EnterpriseProfileClass,
    docs_state: DocsCompletenessClass,
    last_docs_build_ref: &str,
    last_docs_build_time: &str,
    freshness_window_token: &str,
    covered_doc_scopes: Vec<&str>,
    missing_doc_scopes: Vec<&str>,
    matrix_state: MatrixCompletenessClass,
    matrix_ref: &str,
    last_published_time: &str,
    covered_capabilities: Vec<&str>,
    missing_capabilities: Vec<&str>,
    known_limits_state: KnownLimitCompletenessClass,
    disclosed_limit_count: usize,
    undisclosed_limit_count: usize,
    known_limit_index_ref: &str,
    disclosed_limitation_classes: Vec<&str>,
    undisclosed_limitation_classes: Vec<&str>,
    proof_currency: ProofCurrencyClass,
    proof_packet_ref: &str,
    last_verified_time: &str,
    proof_validity_window_token: &str,
    local_core_posture: LocalCoreContinuityPostureClass,
    tenant_region_owner_ref: &str,
    policy_source_ref: &str,
    dependency_class_token: &str,
) -> EnterpriseDocsMatricesKnownLimitsRow {
    EnterpriseDocsMatricesKnownLimitsRow {
        record_kind: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_ROW_RECORD_KIND.to_owned(),
        schema_version: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SCHEMA_VERSION,
        shared_contract_ref: ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        enterprise_profile: profile,
        enterprise_profile_token: profile.as_str().to_owned(),
        docs: DocsDeclaration {
            docs_state,
            docs_state_token: docs_state.as_str().to_owned(),
            last_docs_build_ref: last_docs_build_ref.to_owned(),
            last_docs_build_time: last_docs_build_time.to_owned(),
            freshness_window_token: freshness_window_token.to_owned(),
            covered_doc_scope_labels: covered_doc_scopes
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            missing_doc_scope_labels: missing_doc_scopes
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
        },
        matrix: MatrixDeclaration {
            matrix_state,
            matrix_state_token: matrix_state.as_str().to_owned(),
            matrix_ref: matrix_ref.to_owned(),
            last_published_time: last_published_time.to_owned(),
            covered_capability_labels: covered_capabilities
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            missing_capability_labels: missing_capabilities
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
        },
        known_limits: KnownLimitsDeclaration {
            known_limits_state,
            known_limits_state_token: known_limits_state.as_str().to_owned(),
            disclosed_limit_count,
            undisclosed_limit_count,
            known_limit_index_ref: known_limit_index_ref.to_owned(),
            disclosed_limitation_classes: disclosed_limitation_classes
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            undisclosed_limitation_classes: undisclosed_limitation_classes
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
        },
        proof_currency: ProofCurrencyDeclaration {
            proof_currency,
            proof_currency_token: proof_currency.as_str().to_owned(),
            proof_packet_ref: proof_packet_ref.to_owned(),
            last_verified_time: last_verified_time.to_owned(),
            proof_validity_window_token: proof_validity_window_token.to_owned(),
        },
        local_core_posture,
        local_core_posture_token: local_core_posture.as_str().to_owned(),
        tenant_region_owner_ref: tenant_region_owner_ref.to_owned(),
        policy_source_ref: policy_source_ref.to_owned(),
        dependency_class_token: dependency_class_token.to_owned(),
        // Qualification fields are filled in by qualify_rows.
        qualification_token: EnterpriseDocsMatricesKnownLimitsQualificationClass::Stable
            .as_str()
            .to_owned(),
        narrow_reason_token: EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::NotNarrowed
            .as_str()
            .to_owned(),
        plain_language_summary: String::new(),
    }
}

fn row_individual_local() -> EnterpriseDocsMatricesKnownLimitsRow {
    make_row(
        "enterprise-docs-matrices-known-limits:individual_local",
        EnterpriseProfileClass::IndividualLocal,
        DocsCompletenessClass::NotApplicable,
        "",
        "",
        "",
        vec![],
        vec![],
        MatrixCompletenessClass::NotApplicable,
        "",
        "",
        vec![],
        vec![],
        KnownLimitCompletenessClass::NotApplicable,
        0,
        0,
        "",
        vec![],
        vec![],
        ProofCurrencyClass::NotApplicable,
        "",
        "",
        "",
        LocalCoreContinuityPostureClass::Preserved,
        "",
        "",
        "",
    )
}

fn row_self_hosted() -> EnterpriseDocsMatricesKnownLimitsRow {
    make_row(
        "enterprise-docs-matrices-known-limits:self_hosted",
        EnterpriseProfileClass::SelfHosted,
        DocsCompletenessClass::Current,
        "docs:enterprise:self-hosted:build:2026.05.0001",
        "2026-05-28T02:00:00Z",
        "rolling_30d",
        vec![
            "installation and topology",
            "identity and SSO",
            "policy bundles and offline entitlement",
            "secret broker and keychain integration",
            "network proxy and PAC",
            "backup and restore",
            "support export parity",
        ],
        vec![],
        MatrixCompletenessClass::Complete,
        "matrix:enterprise:self-hosted:capabilities:v1",
        "2026-05-28T02:00:00Z",
        vec![
            "desktop editing",
            "local Git",
            "self-hosted policy enforcement",
            "SSO / OIDC",
            "passkey authentication",
            "secret broker",
            "offline entitlement",
            "admin console",
            "support export",
        ],
        vec![],
        KnownLimitCompletenessClass::FullyDisclosed,
        4,
        0,
        "known-limits:enterprise:self-hosted:index:v1",
        vec![
            "platform_narrowed",
            "workflow_narrowed",
            "support_export_narrowed",
            "migration_path_limited",
        ],
        vec![],
        ProofCurrencyClass::Current,
        "proof:enterprise:self-hosted:current:v1",
        "2026-05-28T02:00:00Z",
        "rolling_90d",
        LocalCoreContinuityPostureClass::Preserved,
        "tenant-region-owner:customer-operated:self-hosted",
        "policy-source:self-hosted-admin-bundle",
        "customer_operated_control_plane",
    )
}

fn row_enterprise_online() -> EnterpriseDocsMatricesKnownLimitsRow {
    make_row(
        "enterprise-docs-matrices-known-limits:enterprise_online",
        EnterpriseProfileClass::EnterpriseOnline,
        DocsCompletenessClass::Current,
        "docs:enterprise:enterprise-online:build:2026.05.0002",
        "2026-05-29T03:00:00Z",
        "rolling_30d",
        vec![
            "installation and topology",
            "identity and SSO",
            "policy bundles and managed entitlement",
            "secret broker and keychain integration",
            "network proxy and PAC",
            "backup and restore",
            "support export parity",
            "relay and transport governance",
        ],
        vec![],
        MatrixCompletenessClass::Complete,
        "matrix:enterprise:enterprise-online:capabilities:v1",
        "2026-05-29T03:00:00Z",
        vec![
            "desktop editing",
            "local Git",
            "managed policy enforcement",
            "SSO / OIDC",
            "passkey authentication",
            "secret broker",
            "managed entitlement",
            "admin console",
            "support export",
            "relay transport",
        ],
        vec![],
        KnownLimitCompletenessClass::FullyDisclosed,
        5,
        0,
        "known-limits:enterprise:enterprise-online:index:v1",
        vec![
            "platform_narrowed",
            "workflow_narrowed",
            "persona_narrowed",
            "support_export_narrowed",
            "migration_path_limited",
        ],
        vec![],
        ProofCurrencyClass::NotApplicable,
        "",
        "",
        "",
        LocalCoreContinuityPostureClass::Preserved,
        "tenant-region-owner:vendor-assisted:enterprise-online",
        "policy-source:enterprise-online-managed-bundle",
        "vendor_assisted_managed_services",
    )
}

fn row_air_gapped() -> EnterpriseDocsMatricesKnownLimitsRow {
    make_row(
        "enterprise-docs-matrices-known-limits:air_gapped",
        EnterpriseProfileClass::AirGapped,
        DocsCompletenessClass::Current,
        "docs:enterprise:air-gapped:build:2026.04.0001",
        "2026-04-01T06:00:00Z",
        "rolling_90d",
        vec![
            "installation and topology",
            "identity and SSO (offline mirror)",
            "policy bundles (signed mirror snapshots)",
            "secret broker and keychain integration",
            "network proxy (deny-all egress)",
            "backup and restore (offline)",
            "support export parity",
            "mirror sync and offline grace",
        ],
        vec![],
        MatrixCompletenessClass::Complete,
        "matrix:enterprise:air-gapped:capabilities:v1",
        "2026-04-01T06:00:00Z",
        vec![
            "desktop editing",
            "local Git",
            "offline policy enforcement (mirror snapshots)",
            "offline entitlement",
            "secret broker (local-only mode)",
            "admin console (offline)",
            "support export",
            "mirror sync",
        ],
        vec![],
        KnownLimitCompletenessClass::FullyDisclosed,
        6,
        0,
        "known-limits:enterprise:air-gapped:index:v1",
        vec![
            "platform_narrowed",
            "workflow_narrowed",
            "persona_narrowed",
            "corpus_narrowed",
            "support_export_narrowed",
            "migration_path_limited",
        ],
        vec![],
        ProofCurrencyClass::Current,
        "proof:enterprise:air-gapped:current:v1",
        "2026-04-01T06:00:00Z",
        "rolling_90d",
        LocalCoreContinuityPostureClass::Preserved,
        "tenant-region-owner:customer-operated:air-gapped",
        "policy-source:air-gapped-mirror-bundle",
        "customer_operated_air_gapped",
    )
}

fn row_managed_cloud() -> EnterpriseDocsMatricesKnownLimitsRow {
    make_row(
        "enterprise-docs-matrices-known-limits:managed_cloud",
        EnterpriseProfileClass::ManagedCloud,
        DocsCompletenessClass::Current,
        "docs:enterprise:managed-cloud:build:2026.05.0003",
        "2026-05-30T04:00:00Z",
        "rolling_30d",
        vec![
            "installation and topology",
            "identity and SSO",
            "policy bundles and managed entitlement",
            "secret broker and keychain integration",
            "network proxy and PAC",
            "backup and restore",
            "support export parity",
            "relay and transport governance",
        ],
        vec![],
        MatrixCompletenessClass::Complete,
        "matrix:enterprise:managed-cloud:capabilities:v1",
        "2026-05-30T04:00:00Z",
        vec![
            "desktop editing",
            "local Git",
            "managed policy enforcement",
            "SSO / OIDC",
            "passkey authentication",
            "secret broker",
            "managed entitlement",
            "admin console",
            "support export",
            "relay transport",
            "cloud workspace sync",
        ],
        vec![],
        KnownLimitCompletenessClass::FullyDisclosed,
        5,
        0,
        "known-limits:enterprise:managed-cloud:index:v1",
        vec![
            "platform_narrowed",
            "workflow_narrowed",
            "persona_narrowed",
            "support_export_narrowed",
            "migration_path_limited",
        ],
        vec![],
        ProofCurrencyClass::NotApplicable,
        "",
        "",
        "",
        LocalCoreContinuityPostureClass::Preserved,
        "tenant-region-owner:vendor-managed:managed-cloud",
        "policy-source:managed-cloud-vendor-bundle",
        "vendor_managed_cloud",
    )
}
