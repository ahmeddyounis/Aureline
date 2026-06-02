//! Finalize qualification rows for desktop local, remote/helper, provider-
//! linked, state/schema, and accessibility matrices.
//!
//! This module produces a stable qualification-matrix proof packet covering
//! five surfaces — desktop-local, remote/helper, provider-linked,
//! state/schema, and accessibility — across four deployment profiles (local
//! OSS, self-hosted, managed, and air-gapped), plus six accessibility-feature
//! rows.
//!
//! Each row carries:
//! - the surface (desktop_local, remote_helper, provider_linked, state_schema,
//!   accessibility),
//! - the deployment profile (local_oss, self_hosted, managed, air_gapped) or
//!   accessibility feature (keyboard, screen_reader, ime_grapheme_bidi, zoom,
//!   high_contrast, reduced_motion),
//! - the dependency class (local_only, network, managed, air_gapped),
//! - the local-core continuity declaration,
//! - the no-account local-use compatibility flag,
//! - the policy source ref (opaque),
//! - tenant and region refs (opaque, nullable),
//! - the failure-mode downgrade class,
//! - the qualification tier (stable/beta/preview/withdrawn), and
//! - the narrow reason.
//!
//! The stable claim holds when **all six** of the following conditions are
//! verified simultaneously:
//!
//! 1. All 22 required rows are present (16 surface × profile rows plus 6
//!    accessibility-feature rows).
//! 2. No raw private material is exposed on any row record.
//! 3. Every row explicitly declares its local-core continuity posture.
//! 4. Every row carries an explicit dependency class.
//! 5. Every row declares its no-account local-use compatibility.
//! 6. Every row names a typed failure-mode downgrade class.
//!
//! One condition forces `Withdrawn` immediately and cannot be overridden:
//!
//! - Any row record carries `raw_private_material_excluded: false`
//!   (narrow reason: [`NarrowReasonClass::RawPrivateMaterialExposed`]).
//!
//! A missing required row narrows to `Preview` rather than `Beta` because
//! the coverage gap prevents any verifiable claim for that row.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque refs only. Raw credentials, raw private keys, raw policy bodies, and
//! raw PII never appear on any record.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/finalize-qualification-rows-for-desktop-local-remote-helper.md`
//! - Artifact: `artifacts/enterprise/m4/finalize-qualification-rows-for-desktop-local-remote-helper.md`
//! - Contract ref: [`QUALIFICATION_MATRIX_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const QUALIFICATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const QUALIFICATION_MATRIX_SHARED_CONTRACT_REF: &str = "remote:qualification_matrix:desktop:v1";

/// Record-kind tag for [`QualificationMatrixPage`] payloads.
pub const QUALIFICATION_MATRIX_PAGE_RECORD_KIND: &str = "remote_qualification_matrix_page_record";

/// Record-kind tag for [`QualificationMatrixRow`] payloads.
pub const QUALIFICATION_MATRIX_ROW_RECORD_KIND: &str = "remote_qualification_matrix_row_record";

/// Record-kind tag for [`QualificationMatrixDefect`] payloads.
pub const QUALIFICATION_MATRIX_DEFECT_RECORD_KIND: &str =
    "remote_qualification_matrix_defect_record";

/// Record-kind tag for [`QualificationMatrixSummary`] payloads.
pub const QUALIFICATION_MATRIX_SUMMARY_RECORD_KIND: &str =
    "remote_qualification_matrix_summary_record";

/// Record-kind tag for [`QualificationMatrixSupportExport`] payloads.
pub const QUALIFICATION_MATRIX_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_qualification_matrix_support_export_record";

/// Record-kind tag for [`QualificationRecord`] payloads.
pub const QUALIFICATION_RECORD_KIND: &str = "remote_qualification_record";

/// Repo-relative path of the stable doc for this lane.
pub const QUALIFICATION_MATRIX_DOC_REF: &str =
    "docs/enterprise/m4/finalize-qualification-rows-for-desktop-local-remote-helper.md";

/// Repo-relative path of the artifact summary for this lane.
pub const QUALIFICATION_MATRIX_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/finalize-qualification-rows-for-desktop-local-remote-helper.md";

/// All required surface × profile row keys in canonical order.
///
/// Each element is `(surface_token, profile_token)`.
pub const REQUIRED_SURFACE_PROFILE_PAIRS: [(&str, &str); 16] = [
    ("desktop_local", "local_oss"),
    ("desktop_local", "self_hosted"),
    ("desktop_local", "managed"),
    ("desktop_local", "air_gapped"),
    ("remote_helper", "local_oss"),
    ("remote_helper", "self_hosted"),
    ("remote_helper", "managed"),
    ("remote_helper", "air_gapped"),
    ("provider_linked", "local_oss"),
    ("provider_linked", "self_hosted"),
    ("provider_linked", "managed"),
    ("provider_linked", "air_gapped"),
    ("state_schema", "local_oss"),
    ("state_schema", "self_hosted"),
    ("state_schema", "managed"),
    ("state_schema", "air_gapped"),
];

/// All required accessibility-feature row keys in canonical order.
pub const REQUIRED_ACCESSIBILITY_FEATURES: [&str; 6] = [
    "keyboard",
    "screen_reader",
    "ime_grapheme_bidi",
    "zoom",
    "high_contrast",
    "reduced_motion",
];

/// Total required row count (16 surface × profile + 6 accessibility).
pub const REQUIRED_ROW_COUNT: usize =
    REQUIRED_SURFACE_PROFILE_PAIRS.len() + REQUIRED_ACCESSIBILITY_FEATURES.len();

// ---------------------------------------------------------------------------
// Surface vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the five qualification matrix surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixSurfaceClass {
    /// Desktop local editing: buffer, LSP, tree-sitter, keybindings, and
    /// all features that work with no account and no outbound connection.
    DesktopLocal,
    /// Remote connection helper: SSH target, remote agent, managed workspace
    /// tunnel, and helper services that mediate access to a remote host.
    RemoteHelper,
    /// Provider-linked authentication and services: VCS hosts, CI, issue
    /// trackers, identity providers, and partner APIs connected via OAuth or
    /// enterprise SSO.
    ProviderLinked,
    /// State and schema persistence: workspace config, document state,
    /// settings sync, and schema-versioned records whose integrity must
    /// survive upgrade, rollback, and offline transitions.
    StateSchema,
    /// Accessibility feature matrix: keyboard navigation, screen reader,
    /// IME/grapheme/bidi input, zoom, high-contrast, and reduced-motion
    /// claims evaluated across all touched surfaces.
    Accessibility,
}

impl MatrixSurfaceClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopLocal => "desktop_local",
            Self::RemoteHelper => "remote_helper",
            Self::ProviderLinked => "provider_linked",
            Self::StateSchema => "state_schema",
            Self::Accessibility => "accessibility",
        }
    }

    /// Human-readable surface label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DesktopLocal => "Desktop local",
            Self::RemoteHelper => "Remote / helper",
            Self::ProviderLinked => "Provider-linked",
            Self::StateSchema => "State / schema",
            Self::Accessibility => "Accessibility",
        }
    }
}

// ---------------------------------------------------------------------------
// Deployment profile vocabulary
// ---------------------------------------------------------------------------

/// Deployment profile for a qualification row.
///
/// Every surface × profile combination must carry an explicit qualification
/// row so that enterprise, self-hosted, managed, and air-gapped claims are
/// backed by concrete evidence rather than inherited from adjacent profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileClass {
    /// No-account, open-source, local-only. All features must work without
    /// any network connection or account registration.
    LocalOss,
    /// Self-hosted instance governed by an organization admin. The IDE
    /// connects to a self-hosted endpoint for policy, auth, and sync.
    SelfHosted,
    /// Cloud-managed enterprise service. Policy, auth, telemetry, and sync
    /// are mediated by a managed endpoint owned by the operator.
    Managed,
    /// Air-gapped or mirror-first. No direct internet egress; all network-
    /// dependent features operate against a declared signed mirror or are
    /// absent.
    AirGapped,
}

impl DeploymentProfileClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
        }
    }

    /// Human-readable profile label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOss => "Local OSS (no account)",
            Self::SelfHosted => "Self-hosted",
            Self::Managed => "Cloud-managed",
            Self::AirGapped => "Air-gapped / mirror-first",
        }
    }
}

// ---------------------------------------------------------------------------
// Accessibility feature vocabulary
// ---------------------------------------------------------------------------

/// Individual accessibility features that must be validated on every touched
/// surface.
///
/// Each feature carries its own qualification row so that gaps in a specific
/// feature are not hidden behind a passing overall accessibility claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityFeatureClass {
    /// Full keyboard navigation without pointer dependency; focus management,
    /// skip links, shortcut discoverability, and no keyboard traps.
    Keyboard,
    /// Screen reader compatibility (NVDA, JAWS, VoiceOver, Orca); accessible
    /// name, role, state, and live-region announcements on every touched
    /// component.
    ScreenReader,
    /// IME composition, grapheme-cluster cursor behavior, and bidirectional
    /// text layout correctness.
    ImeGraphemeBidi,
    /// Layout zoom (browser/OS text scaling) and pinch/scroll zoom; no
    /// horizontal overflow and no clipped interactive targets at 200 % zoom.
    Zoom,
    /// High-contrast and forced-color mode; all interactive elements remain
    /// distinguishable and no essential information is conveyed by color
    /// alone.
    HighContrast,
    /// Reduced-motion preference; no persistent or looping animations when
    /// `prefers-reduced-motion: reduce` is active.
    ReducedMotion,
}

impl AccessibilityFeatureClass {
    /// Stable closed-vocabulary token recorded in records and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::ImeGraphemeBidi => "ime_grapheme_bidi",
            Self::Zoom => "zoom",
            Self::HighContrast => "high_contrast",
            Self::ReducedMotion => "reduced_motion",
        }
    }

    /// Human-readable feature label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Keyboard => "Keyboard navigation",
            Self::ScreenReader => "Screen reader",
            Self::ImeGraphemeBidi => "IME / grapheme / bidi",
            Self::Zoom => "Zoom / scaling",
            Self::HighContrast => "High contrast",
            Self::ReducedMotion => "Reduced motion",
        }
    }
}

// ---------------------------------------------------------------------------
// Dependency class vocabulary
// ---------------------------------------------------------------------------

/// Ownership tier that records what external dependency a qualification row
/// carries.
///
/// Making the dependency class explicit on every row allows enterprise,
/// self-hosted, managed, and air-gapped claims to be evaluated without
/// inspecting raw configuration or reading subsystem-specific status strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    /// No external network dependency; works with no account and no outbound
    /// connection.
    LocalOnly,
    /// Requires a live network connection to a public or hosted endpoint;
    /// degrades gracefully when the network is unavailable.
    Network,
    /// Requires connectivity to a managed service endpoint controlled by an
    /// enterprise admin; local work continues without managed capabilities.
    Managed,
    /// Operates against a declared signed mirror or air-gapped media only;
    /// no direct internet egress is permitted.
    AirGapped,
}

impl DependencyClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Network => "network",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
        }
    }

    /// Returns `true` when this dependency class requires an explicit policy
    /// source ref for the stable claim.
    pub const fn requires_policy_source_ref(self) -> bool {
        matches!(self, Self::Network | Self::Managed | Self::AirGapped)
    }
}

// ---------------------------------------------------------------------------
// Failure-mode downgrade vocabulary
// ---------------------------------------------------------------------------

/// How a surface row behaves when its dependency becomes unavailable.
///
/// Every row must carry an explicit failure-mode downgrade class so that
/// support, diagnostics, and release documentation can describe the exact
/// user-visible impact of a dependency failure without relying on the reader
/// to infer it from the dependency class alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureDowngradeClass {
    /// Managed or network failure has no effect on the local editing floor;
    /// all local features continue unaffected.
    LocalCoreUnaffected,
    /// Managed or network-dependent features degrade but the local editing
    /// floor is preserved and unblocked.
    DegradedFeatures,
    /// An offline-grace window applies; the last validated bundle or state
    /// is extended until the window expires.
    OfflineGrace,
    /// Traffic falls back to a declared signed mirror; local core continues
    /// from the mirror-served content.
    MirrorFallback,
    /// No failure mode applies; the row has no external dependency and
    /// cannot fail due to connectivity loss.
    NotApplicable,
}

impl FailureDowngradeClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCoreUnaffected => "local_core_unaffected",
            Self::DegradedFeatures => "degraded_features",
            Self::OfflineGrace => "offline_grace",
            Self::MirrorFallback => "mirror_fallback",
            Self::NotApplicable => "not_applicable",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification tier vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and individual rows.
///
/// The tier is derived, not asserted: it is set by comparing the audit defect
/// list against the six stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete row coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationTierClass {
    /// All six stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required row is missing; the coverage gap prevents a beta claim.
    Preview,
    /// Raw private material was exposed on a row record; the packet is
    /// withdrawn immediately and cannot be overridden.
    Withdrawn,
}

impl QualificationTierClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Returns `true` when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

// ---------------------------------------------------------------------------
// Narrow reason vocabulary
// ---------------------------------------------------------------------------

/// Typed reason a packet or qualification row was narrowed below
/// [`QualificationTierClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// A row record carries `raw_private_material_excluded: false`; the
    /// packet is withdrawn immediately.
    RawPrivateMaterialExposed,
    /// A required row is absent from the matrix snapshot; narrows to
    /// preview.
    RequiredRowMissing,
    /// A row does not declare its local-core continuity posture explicitly.
    LocalCoreContinuityUndeclared,
    /// A row does not carry an explicit dependency class.
    DependencyClassUndeclared,
    /// A row does not declare its no-account local-use compatibility.
    NoAccountCompatibilityUndeclared,
    /// A row does not name a typed failure-mode downgrade class.
    FailureDowngradeUndeclared,
}

impl NarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::RequiredRowMissing => "required_row_missing",
            Self::LocalCoreContinuityUndeclared => "local_core_continuity_undeclared",
            Self::DependencyClassUndeclared => "dependency_class_undeclared",
            Self::NoAccountCompatibilityUndeclared => "no_account_compatibility_undeclared",
            Self::FailureDowngradeUndeclared => "failure_downgrade_undeclared",
        }
    }

    /// Returns `true` when this reason forces an immediate withdrawal.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::RawPrivateMaterialExposed)
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredRowMissing)
    }
}

// ---------------------------------------------------------------------------
// Qualification record (per surface × profile or accessibility feature)
// ---------------------------------------------------------------------------

/// Per-row qualification record in the matrix snapshot.
///
/// Each record represents a single (surface, profile) pair for primary
/// surface rows, or a single accessibility feature for accessibility rows.
/// Together the 22 required records form the [`QualificationSnapshot`] that
/// the proof packet embeds as evidence.
///
/// No raw credentials, raw private keys, raw policy bundle bodies, raw PII,
/// or raw endpoint URLs may appear on this record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row key: `{surface_token}:{profile_token}` for primary rows
    /// or `accessibility:{feature_token}` for accessibility rows.
    pub row_key: String,
    /// Surface token.
    pub surface_token: String,
    /// Profile token. Empty for accessibility rows.
    pub profile_token: String,
    /// Accessibility feature token. Empty for primary surface rows.
    pub accessibility_feature_token: String,
    /// Dependency class for this row.
    pub dependency_class: DependencyClass,
    /// Stable token for [`Self::dependency_class`].
    pub dependency_class_token: String,
    /// `true` when the local-core editing floor is preserved regardless of
    /// whether this row's dependency is available.
    pub local_core_continuity_allowed: bool,
    /// `true` when this row's features are fully available with no account
    /// and no outbound network connection.
    pub no_account_local_compatible: bool,
    /// How the row's surface degrades when its dependency becomes
    /// unavailable.
    pub failure_downgrade: FailureDowngradeClass,
    /// Stable token for [`Self::failure_downgrade`].
    pub failure_downgrade_token: String,
    /// Opaque ref identifying the policy source that governs this row.
    pub policy_source_ref: String,
    /// Opaque ref identifying the tenant context. `None` for rows that
    /// operate without tenant attribution.
    pub tenant_ref: Option<String>,
    /// Opaque ref identifying the region. `None` when region attribution is
    /// not applicable.
    pub region_ref: Option<String>,
    /// `true` when no raw credentials, raw private keys, or raw PII are
    /// present on this record. Must be `true` for the stable claim to hold.
    pub raw_private_material_excluded: bool,
    /// Plain-language summary safe for UI, support exports, and diagnostics.
    pub summary: String,
}

impl QualificationRecord {
    /// Construct a surface × profile qualification record.
    #[allow(clippy::too_many_arguments)]
    pub fn new_surface(
        surface: MatrixSurfaceClass,
        profile: DeploymentProfileClass,
        dependency_class: DependencyClass,
        local_core_continuity_allowed: bool,
        no_account_local_compatible: bool,
        failure_downgrade: FailureDowngradeClass,
        policy_source_ref: impl Into<String>,
        tenant_ref: Option<impl Into<String>>,
        region_ref: Option<impl Into<String>>,
        summary: impl Into<String>,
    ) -> Self {
        let row_key = format!("{}:{}", surface.as_str(), profile.as_str());
        Self {
            record_kind: QUALIFICATION_RECORD_KIND.to_owned(),
            schema_version: QUALIFICATION_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: QUALIFICATION_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            row_key,
            surface_token: surface.as_str().to_owned(),
            profile_token: profile.as_str().to_owned(),
            accessibility_feature_token: String::new(),
            dependency_class,
            dependency_class_token: dependency_class.as_str().to_owned(),
            local_core_continuity_allowed,
            no_account_local_compatible,
            failure_downgrade,
            failure_downgrade_token: failure_downgrade.as_str().to_owned(),
            policy_source_ref: policy_source_ref.into(),
            tenant_ref: tenant_ref.map(Into::into),
            region_ref: region_ref.map(Into::into),
            raw_private_material_excluded: true,
            summary: summary.into(),
        }
    }

    /// Construct an accessibility feature qualification record.
    pub fn new_accessibility(
        feature: AccessibilityFeatureClass,
        policy_source_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        let row_key = format!("accessibility:{}", feature.as_str());
        Self {
            record_kind: QUALIFICATION_RECORD_KIND.to_owned(),
            schema_version: QUALIFICATION_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: QUALIFICATION_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            row_key,
            surface_token: MatrixSurfaceClass::Accessibility.as_str().to_owned(),
            profile_token: String::new(),
            accessibility_feature_token: feature.as_str().to_owned(),
            dependency_class: DependencyClass::LocalOnly,
            dependency_class_token: DependencyClass::LocalOnly.as_str().to_owned(),
            local_core_continuity_allowed: true,
            no_account_local_compatible: true,
            failure_downgrade: FailureDowngradeClass::NotApplicable,
            failure_downgrade_token: FailureDowngradeClass::NotApplicable.as_str().to_owned(),
            policy_source_ref: policy_source_ref.into(),
            tenant_ref: None,
            region_ref: None,
            raw_private_material_excluded: true,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification snapshot (aggregate of all row records)
// ---------------------------------------------------------------------------

/// Aggregate of all qualification records in the matrix.
///
/// The snapshot must contain one record per required row key to satisfy the
/// stable claim. Missing required rows cause the proof packet to narrow to
/// `Preview`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationSnapshot {
    /// All qualification records in this snapshot.
    pub records: Vec<QualificationRecord>,
}

impl QualificationSnapshot {
    /// Returns the record for the given row key, if present.
    pub fn record_for_key(&self, row_key: &str) -> Option<&QualificationRecord> {
        self.records.iter().find(|r| r.row_key == row_key)
    }

    /// Returns the set of row keys covered by this snapshot.
    pub fn covered_row_keys(&self) -> BTreeSet<&str> {
        self.records.iter().map(|r| r.row_key.as_str()).collect()
    }
}

// ---------------------------------------------------------------------------
// Qualification matrix row (derived stability row)
// ---------------------------------------------------------------------------

/// Stability qualification for one row in the proof packet.
///
/// Each matrix row is derived from a single [`QualificationRecord`] in the
/// snapshot. The qualification is computed from the record against the six
/// stability conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationMatrixRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row key for this matrix row.
    pub row_key: String,
    /// Surface token.
    pub surface_token: String,
    /// Profile token (empty for accessibility rows).
    pub profile_token: String,
    /// Accessibility feature token (empty for primary surface rows).
    pub accessibility_feature_token: String,
    /// Dependency class token from the record.
    pub dependency_class_token: String,
    /// `true` when local-core continuity is explicitly declared.
    pub local_core_continuity_declared: bool,
    /// `true` when no-account local-use compatibility is declared.
    pub no_account_compatibility_declared: bool,
    /// Failure-mode downgrade class token from the record.
    pub failure_downgrade_token: String,
    /// `true` when raw private material is excluded from the record.
    pub raw_private_material_excluded: bool,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the qualification matrix page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct QualificationMatrixSummary {
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
    /// Row keys covered by the snapshot.
    pub rows_covered: Vec<String>,
    /// Number of rows with explicit local-core continuity declaration.
    pub local_core_continuity_declared_count: usize,
    /// Number of rows with explicit no-account compatibility declaration.
    pub no_account_compatibility_declared_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl QualificationMatrixSummary {
    fn from_rows(rows: &[QualificationMatrixRow], snapshot: &QualificationSnapshot) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        let overall = if withdrawn > 0 {
            QualificationTierClass::Withdrawn
        } else if preview > 0 {
            QualificationTierClass::Preview
        } else if beta > 0 {
            QualificationTierClass::Beta
        } else {
            QualificationTierClass::Stable
        };
        let rows_covered: Vec<String> =
            snapshot.records.iter().map(|r| r.row_key.clone()).collect();
        let local_core_continuity_declared_count = rows
            .iter()
            .filter(|r| r.local_core_continuity_declared)
            .count();
        let no_account_compatibility_declared_count = rows
            .iter()
            .filter(|r| r.no_account_compatibility_declared)
            .count();
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            rows_covered,
            local_core_continuity_declared_count,
            no_account_compatibility_declared_count,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the qualification matrix audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationMatrixDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: NarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject row key or `page`.
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl QualificationMatrixDefect {
    fn new(
        narrow_reason: NarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: QUALIFICATION_MATRIX_DEFECT_RECORD_KIND.to_owned(),
            schema_version: QUALIFICATION_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: QUALIFICATION_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:qualification-matrix:{}:{}",
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
// Qualification matrix page (proof packet)
// ---------------------------------------------------------------------------

/// Stable proof packet for the qualification matrix.
///
/// The page is the single inspectable record that proves stable claims for
/// desktop-local, remote/helper, provider-linked, state/schema, and
/// accessibility surfaces across all deployment profiles. Dashboards,
/// docs/Help/About surfaces, support exports, and release evidence should
/// ingest this packet rather than cloning subsystem-specific status strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationMatrixPage {
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
    pub summary: QualificationMatrixSummary,
    /// Per-row qualification entries.
    pub rows: Vec<QualificationMatrixRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<QualificationMatrixDefect>,
    /// The qualification snapshot embedded as evidence.
    pub snapshot: QualificationSnapshot,
}

impl QualificationMatrixPage {
    /// Build the qualification matrix page from a snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        snapshot: QualificationSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&snapshot);
        let rows = derive_matrix_rows(&snapshot, &defects);
        let summary = QualificationMatrixSummary::from_rows(&rows, &snapshot);
        Self {
            record_kind: QUALIFICATION_MATRIX_PAGE_RECORD_KIND.to_owned(),
            schema_version: QUALIFICATION_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: QUALIFICATION_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token == QualificationTierClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all 22 required rows are covered.
    pub fn covers_all_required_rows(&self) -> bool {
        let covered = self.snapshot.covered_row_keys();
        let all_surface_covered = REQUIRED_SURFACE_PROFILE_PAIRS
            .iter()
            .all(|(s, p)| covered.contains(format!("{s}:{p}").as_str()));
        let all_a11y_covered = REQUIRED_ACCESSIBILITY_FEATURES
            .iter()
            .all(|f| covered.contains(format!("accessibility:{f}").as_str()));
        all_surface_covered && all_a11y_covered
    }

    /// Returns `true` when every record explicitly declares local-core
    /// continuity.
    pub fn all_rows_declare_local_core_continuity(&self) -> bool {
        self.snapshot
            .records
            .iter()
            .all(|r| r.local_core_continuity_allowed)
    }

    /// Returns `true` when every record carries a typed failure-downgrade
    /// class (non-empty token).
    pub fn all_rows_have_typed_failure_downgrade(&self) -> bool {
        self.snapshot
            .records
            .iter()
            .all(|r| !r.failure_downgrade_token.is_empty())
    }

    /// Returns `true` when every record declares no-account local-use
    /// compatibility for the `local_oss` and `air_gapped` profiles.
    pub fn all_no_account_rows_declare_compatibility(&self) -> bool {
        self.snapshot.records.iter().all(|r| {
            if r.profile_token == DeploymentProfileClass::LocalOss.as_str()
                || r.profile_token == DeploymentProfileClass::AirGapped.as_str()
                || r.surface_token == MatrixSurfaceClass::Accessibility.as_str()
            {
                r.no_account_local_compatible
            } else {
                true
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the qualification matrix page plus a
/// metadata-safe defect roll-up.
///
/// No raw credentials, raw private keys, raw policy bodies, or raw PII may
/// appear in this export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationMatrixSupportExport {
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
    /// The qualification matrix page embedded as evidence.
    pub page: QualificationMatrixPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<NarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl QualificationMatrixSupportExport {
    /// Wrap a qualification matrix page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: QualificationMatrixPage,
    ) -> Self {
        let mut reasons: Vec<NarrowReasonClass> = Vec::new();
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
            record_kind: QUALIFICATION_MATRIX_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: QUALIFICATION_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: QUALIFICATION_MATRIX_SHARED_CONTRACT_REF.to_owned(),
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
// Public API functions
// ---------------------------------------------------------------------------

/// Re-run the qualification audit over the snapshot embedded in a page.
pub fn audit_qualification_matrix_page(
    page: &QualificationMatrixPage,
) -> Vec<QualificationMatrixDefect> {
    audit_snapshot(&page.snapshot)
}

/// Validate a qualification matrix page; returns `Ok` when the audit is
/// clean.
///
/// # Errors
///
/// Returns the defect list when one or more stability conditions are violated.
pub fn validate_qualification_matrix_page(
    page: &QualificationMatrixPage,
) -> Result<(), Vec<QualificationMatrixDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &QualificationSnapshot) -> Vec<QualificationMatrixDefect> {
    let mut defects: Vec<QualificationMatrixDefect> = Vec::new();

    // Hard guardrail: raw private material exposed — withdraw immediately.
    for record in &snapshot.records {
        if !record.raw_private_material_excluded {
            defects.push(QualificationMatrixDefect::new(
                NarrowReasonClass::RawPrivateMaterialExposed,
                record.row_key.clone(),
                format!(
                    "row '{}' has raw_private_material_excluded: false; packet is withdrawn",
                    record.row_key
                ),
            ));
            return defects;
        }
    }

    let covered = snapshot.covered_row_keys();

    // Coverage check: all required surface × profile rows must be present.
    for (surface, profile) in &REQUIRED_SURFACE_PROFILE_PAIRS {
        let key = format!("{surface}:{profile}");
        if !covered.contains(key.as_str()) {
            defects.push(QualificationMatrixDefect::new(
                NarrowReasonClass::RequiredRowMissing,
                key.clone(),
                format!(
                    "required row '{key}' has no qualification record; \
                     packet is narrowed to preview"
                ),
            ));
        }
    }

    // Coverage check: all required accessibility-feature rows must be present.
    for feature in &REQUIRED_ACCESSIBILITY_FEATURES {
        let key = format!("accessibility:{feature}");
        if !covered.contains(key.as_str()) {
            defects.push(QualificationMatrixDefect::new(
                NarrowReasonClass::RequiredRowMissing,
                key.clone(),
                format!(
                    "required accessibility row '{key}' has no qualification record; \
                     packet is narrowed to preview"
                ),
            ));
        }
    }

    // Per-record checks.
    for record in &snapshot.records {
        if !record.local_core_continuity_allowed {
            defects.push(QualificationMatrixDefect::new(
                NarrowReasonClass::LocalCoreContinuityUndeclared,
                record.row_key.clone(),
                format!(
                    "row '{}' does not declare local-core continuity; local work \
                     may be blocked by managed or network-dependent capabilities",
                    record.row_key
                ),
            ));
        }

        if record.dependency_class_token.is_empty() {
            defects.push(QualificationMatrixDefect::new(
                NarrowReasonClass::DependencyClassUndeclared,
                record.row_key.clone(),
                format!(
                    "row '{}' has an empty dependency_class_token; dependency class \
                     must be explicit",
                    record.row_key
                ),
            ));
        }

        let requires_no_account_declaration = record.profile_token
            == DeploymentProfileClass::LocalOss.as_str()
            || record.profile_token == DeploymentProfileClass::AirGapped.as_str()
            || record.surface_token == MatrixSurfaceClass::Accessibility.as_str();
        if requires_no_account_declaration && !record.no_account_local_compatible {
            defects.push(QualificationMatrixDefect::new(
                NarrowReasonClass::NoAccountCompatibilityUndeclared,
                record.row_key.clone(),
                format!(
                    "row '{}' ({} profile) does not declare no-account local-use \
                     compatibility; local and air-gapped rows must confirm no-account \
                     continuity",
                    record.row_key, record.profile_token
                ),
            ));
        }

        if record.failure_downgrade_token.is_empty() {
            defects.push(QualificationMatrixDefect::new(
                NarrowReasonClass::FailureDowngradeUndeclared,
                record.row_key.clone(),
                format!(
                    "row '{}' has an empty failure_downgrade_token; failure-mode \
                     downgrade class must be explicit for release documentation",
                    record.row_key
                ),
            ));
        }
    }

    defects
}

fn derive_matrix_rows(
    snapshot: &QualificationSnapshot,
    page_defects: &[QualificationMatrixDefect],
) -> Vec<QualificationMatrixRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_preview_reason());

    let overall_narrow_reason = if has_withdrawal {
        NarrowReasonClass::RawPrivateMaterialExposed
    } else if has_preview {
        NarrowReasonClass::RequiredRowMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        NarrowReasonClass::NotNarrowed
    };

    snapshot
        .records
        .iter()
        .map(|record| {
            let row_narrow = find_row_narrow_reason(record, page_defects, overall_narrow_reason);
            let row_qual = if row_narrow.is_withdrawal_reason() {
                QualificationTierClass::Withdrawn
            } else if row_narrow.is_preview_reason() {
                QualificationTierClass::Preview
            } else if row_narrow != NarrowReasonClass::NotNarrowed {
                QualificationTierClass::Beta
            } else {
                QualificationTierClass::Stable
            };
            let summary = build_row_summary(&record.row_key, &row_qual, row_narrow);
            QualificationMatrixRow {
                record_kind: QUALIFICATION_MATRIX_ROW_RECORD_KIND.to_owned(),
                schema_version: QUALIFICATION_MATRIX_SCHEMA_VERSION,
                shared_contract_ref: QUALIFICATION_MATRIX_SHARED_CONTRACT_REF.to_owned(),
                row_key: record.row_key.clone(),
                surface_token: record.surface_token.clone(),
                profile_token: record.profile_token.clone(),
                accessibility_feature_token: record.accessibility_feature_token.clone(),
                dependency_class_token: record.dependency_class_token.clone(),
                local_core_continuity_declared: record.local_core_continuity_allowed,
                no_account_compatibility_declared: record.no_account_local_compatible,
                failure_downgrade_token: record.failure_downgrade_token.clone(),
                raw_private_material_excluded: record.raw_private_material_excluded,
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn find_row_narrow_reason(
    record: &QualificationRecord,
    page_defects: &[QualificationMatrixDefect],
    overall_narrow_reason: NarrowReasonClass,
) -> NarrowReasonClass {
    if let Some(defect) = page_defects.iter().find(|d| d.source == record.row_key) {
        return defect.narrow_reason;
    }
    overall_narrow_reason
}

fn build_row_summary(
    row_key: &str,
    qual: &QualificationTierClass,
    narrow_reason: NarrowReasonClass,
) -> String {
    match qual {
        QualificationTierClass::Stable => format!(
            "Row '{row_key}' qualifies stable: all six stability conditions hold, \
             local-core continuity is declared, dependency class is explicit, \
             no-account compatibility is confirmed, and failure-mode downgrade \
             is typed."
        ),
        _ => format!(
            "Row '{row_key}' narrowed to {} ({}): see defect list for details.",
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet consumed by integration tests and the
/// fixture generator.
///
/// The seeded page produces zero defects: all 22 required rows are covered,
/// no raw private material is exposed, every row declares local-core
/// continuity, all dependency classes are explicit, all no-account rows
/// declare compatibility, and all rows carry typed failure-mode downgrade
/// classes.
pub fn seeded_qualification_matrix_page() -> QualificationMatrixPage {
    QualificationMatrixPage::new(
        "remote:qualification_matrix:desktop:default",
        "Qualification matrix — desktop local, remote/helper, provider-linked, \
         state/schema, and accessibility — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_qualification_snapshot(),
    )
}

/// Build the seeded qualification snapshot used by the seeded page.
///
/// Each of the 22 required rows is represented with a fully-typed, clean
/// record that passes all six stability conditions.
pub fn seeded_qualification_snapshot() -> QualificationSnapshot {
    QualificationSnapshot {
        records: vec![
            // ---------------------------------------------------------------
            // desktop_local surface
            // ---------------------------------------------------------------
            QualificationRecord::new_surface(
                MatrixSurfaceClass::DesktopLocal,
                DeploymentProfileClass::LocalOss,
                DependencyClass::LocalOnly,
                true,
                true,
                FailureDowngradeClass::NotApplicable,
                "policy:desktop-local:local-default:v1",
                None::<String>,
                None::<String>,
                "Desktop local (local OSS): all editing features work with no account \
                 and no outbound connection; local-core continuity is unconditional.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::DesktopLocal,
                DeploymentProfileClass::SelfHosted,
                DependencyClass::Managed,
                true,
                false,
                FailureDowngradeClass::LocalCoreUnaffected,
                "policy:desktop-local:self-hosted-admin:v1",
                Some("tenant:self-hosted:default"),
                None::<String>,
                "Desktop local (self-hosted): admin policy governs feature flags and \
                 sync; local editing floor is preserved when the self-hosted endpoint \
                 is unreachable.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::DesktopLocal,
                DeploymentProfileClass::Managed,
                DependencyClass::Managed,
                true,
                false,
                FailureDowngradeClass::LocalCoreUnaffected,
                "policy:desktop-local:managed-enterprise:v1",
                Some("tenant:managed:default"),
                Some("region:us-east-1"),
                "Desktop local (managed): cloud-managed policy governs feature flags, \
                 telemetry, and sync; local editing floor is preserved when the managed \
                 endpoint is unreachable.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::DesktopLocal,
                DeploymentProfileClass::AirGapped,
                DependencyClass::LocalOnly,
                true,
                true,
                FailureDowngradeClass::NotApplicable,
                "policy:desktop-local:air-gapped-local:v1",
                None::<String>,
                None::<String>,
                "Desktop local (air-gapped): all editing features work fully offline; \
                 no internet egress is required; local-core continuity is unconditional.",
            ),
            // ---------------------------------------------------------------
            // remote_helper surface
            // ---------------------------------------------------------------
            QualificationRecord::new_surface(
                MatrixSurfaceClass::RemoteHelper,
                DeploymentProfileClass::LocalOss,
                DependencyClass::Network,
                true,
                true,
                FailureDowngradeClass::DegradedFeatures,
                "policy:remote-helper:local-oss-network:v1",
                None::<String>,
                None::<String>,
                "Remote/helper (local OSS): SSH and remote-agent connections require \
                 network; local editing continues without remote connectivity; remote \
                 features degrade gracefully.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::RemoteHelper,
                DeploymentProfileClass::SelfHosted,
                DependencyClass::Managed,
                true,
                false,
                FailureDowngradeClass::DegradedFeatures,
                "policy:remote-helper:self-hosted-endpoint:v1",
                Some("tenant:self-hosted:default"),
                None::<String>,
                "Remote/helper (self-hosted): self-hosted endpoint governs tunnel and \
                 helper auth; local editing continues when the self-hosted endpoint is \
                 unreachable; remote features degrade.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::RemoteHelper,
                DeploymentProfileClass::Managed,
                DependencyClass::Managed,
                true,
                false,
                FailureDowngradeClass::DegradedFeatures,
                "policy:remote-helper:managed-endpoint:v1",
                Some("tenant:managed:default"),
                Some("region:us-east-1"),
                "Remote/helper (managed): managed endpoint governs tunnel auth and \
                 workspace routing; local editing continues when managed connectivity \
                 is unavailable; remote features degrade.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::RemoteHelper,
                DeploymentProfileClass::AirGapped,
                DependencyClass::AirGapped,
                true,
                true,
                FailureDowngradeClass::MirrorFallback,
                "policy:remote-helper:air-gapped-mirror:v1",
                None::<String>,
                None::<String>,
                "Remote/helper (air-gapped): remote connections are limited to \
                 declared signed-mirror targets; local editing continues fully; \
                 remote connections outside the declared mirror set are blocked.",
            ),
            // ---------------------------------------------------------------
            // provider_linked surface
            // ---------------------------------------------------------------
            QualificationRecord::new_surface(
                MatrixSurfaceClass::ProviderLinked,
                DeploymentProfileClass::LocalOss,
                DependencyClass::Network,
                true,
                true,
                FailureDowngradeClass::DegradedFeatures,
                "policy:provider-linked:local-oss-network:v1",
                None::<String>,
                None::<String>,
                "Provider-linked (local OSS): VCS, CI, and issue-tracker connections \
                 require network; local repo and workspace continue without provider \
                 connectivity; provider features degrade gracefully.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::ProviderLinked,
                DeploymentProfileClass::SelfHosted,
                DependencyClass::Managed,
                true,
                false,
                FailureDowngradeClass::DegradedFeatures,
                "policy:provider-linked:self-hosted-provider:v1",
                Some("tenant:self-hosted:default"),
                None::<String>,
                "Provider-linked (self-hosted): self-hosted VCS and identity provider \
                 endpoint governs auth and API access; local repo and workspace continue \
                 when the self-hosted endpoint is unreachable.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::ProviderLinked,
                DeploymentProfileClass::Managed,
                DependencyClass::Managed,
                true,
                false,
                FailureDowngradeClass::DegradedFeatures,
                "policy:provider-linked:managed-provider:v1",
                Some("tenant:managed:default"),
                Some("region:us-east-1"),
                "Provider-linked (managed): cloud-managed VCS and identity provider \
                 governs auth, PR review, and CI integration; local repo and workspace \
                 continue when managed connectivity is unavailable.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::ProviderLinked,
                DeploymentProfileClass::AirGapped,
                DependencyClass::AirGapped,
                true,
                true,
                FailureDowngradeClass::MirrorFallback,
                "policy:provider-linked:air-gapped-mirror:v1",
                None::<String>,
                None::<String>,
                "Provider-linked (air-gapped): provider connections are limited to \
                 declared signed-mirror endpoints; local repo and workspace continue \
                 fully; provider connections outside the declared mirror set are blocked.",
            ),
            // ---------------------------------------------------------------
            // state_schema surface
            // ---------------------------------------------------------------
            QualificationRecord::new_surface(
                MatrixSurfaceClass::StateSchema,
                DeploymentProfileClass::LocalOss,
                DependencyClass::LocalOnly,
                true,
                true,
                FailureDowngradeClass::NotApplicable,
                "policy:state-schema:local-default:v1",
                None::<String>,
                None::<String>,
                "State/schema (local OSS): all workspace state, config, and schema \
                 records are stored locally; no external dependency; state integrity \
                 survives upgrade, rollback, and offline transitions.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::StateSchema,
                DeploymentProfileClass::SelfHosted,
                DependencyClass::Managed,
                true,
                false,
                FailureDowngradeClass::OfflineGrace,
                "policy:state-schema:self-hosted-sync:v1",
                Some("tenant:self-hosted:default"),
                None::<String>,
                "State/schema (self-hosted): state is synced to a self-hosted endpoint \
                 when reachable; local state continues within the offline-grace window \
                 when the endpoint is unreachable.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::StateSchema,
                DeploymentProfileClass::Managed,
                DependencyClass::Managed,
                true,
                false,
                FailureDowngradeClass::OfflineGrace,
                "policy:state-schema:managed-sync:v1",
                Some("tenant:managed:default"),
                Some("region:us-east-1"),
                "State/schema (managed): state is synced to the managed cloud endpoint; \
                 local state is served from cache within the offline-grace window; \
                 schema migrations are validated before applying.",
            ),
            QualificationRecord::new_surface(
                MatrixSurfaceClass::StateSchema,
                DeploymentProfileClass::AirGapped,
                DependencyClass::LocalOnly,
                true,
                true,
                FailureDowngradeClass::NotApplicable,
                "policy:state-schema:air-gapped-local:v1",
                None::<String>,
                None::<String>,
                "State/schema (air-gapped): all state is stored locally with no sync; \
                 schema migrations are applied from local or mirror-sourced bundles \
                 only; no internet egress is required.",
            ),
            // ---------------------------------------------------------------
            // accessibility features
            // ---------------------------------------------------------------
            QualificationRecord::new_accessibility(
                AccessibilityFeatureClass::Keyboard,
                "policy:accessibility:keyboard:v1",
                "Keyboard navigation: full keyboard access to all interactive elements \
                 on every touched surface; focus management, skip links, shortcut \
                 discoverability, and no keyboard traps; validated across all five \
                 surfaces.",
            ),
            QualificationRecord::new_accessibility(
                AccessibilityFeatureClass::ScreenReader,
                "policy:accessibility:screen-reader:v1",
                "Screen reader: accessible name, role, state, and live-region \
                 announcements on every touched component; validated with NVDA, JAWS, \
                 VoiceOver, and Orca across all five surfaces.",
            ),
            QualificationRecord::new_accessibility(
                AccessibilityFeatureClass::ImeGraphemeBidi,
                "policy:accessibility:ime-grapheme-bidi:v1",
                "IME/grapheme/bidi: IME composition events handled correctly; grapheme- \
                 cluster cursor navigation in multi-codepoint sequences; bidirectional \
                 text rendered and selected correctly across all five surfaces.",
            ),
            QualificationRecord::new_accessibility(
                AccessibilityFeatureClass::Zoom,
                "policy:accessibility:zoom:v1",
                "Zoom/scaling: no horizontal overflow and no clipped interactive targets \
                 at 200 % zoom; layout adapts to OS text-scaling preferences across all \
                 five surfaces.",
            ),
            QualificationRecord::new_accessibility(
                AccessibilityFeatureClass::HighContrast,
                "policy:accessibility:high-contrast:v1",
                "High contrast: all interactive elements remain distinguishable in \
                 forced-color and high-contrast modes; no essential information is \
                 conveyed by color alone; validated across all five surfaces.",
            ),
            QualificationRecord::new_accessibility(
                AccessibilityFeatureClass::ReducedMotion,
                "policy:accessibility:reduced-motion:v1",
                "Reduced motion: no persistent or looping animations when \
                 `prefers-reduced-motion: reduce` is active; all transitions are \
                 suppressed or replaced with instant state changes across all five \
                 surfaces.",
            ),
        ],
    }
}
