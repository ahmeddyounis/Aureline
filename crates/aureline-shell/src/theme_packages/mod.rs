//! Versioned theme-package manifests and per-surface active-package bindings
//! for the M5 depth surfaces.
//!
//! The M5 depth lanes ship many new panes — notebook cell chrome, result
//! grids, profiler timelines, preview/browser panes, docs/help panes,
//! companion surfaces, and extension-backed panels. Each of those surfaces
//! paints under a theme, but "broad theme parity" no longer says enough:
//! the product needs every claimed surface to declare *which versioned theme
//! package* and *which supported appearance modes* actually apply, where the
//! package came from, and what it expects the surface to inherit — so support,
//! export, diagnostics, and release-evidence flows can reason about appearance
//! truth without screenshots or feature-local style code.
//!
//! This module carries the canonical theme-package object forward into those
//! lanes. It models:
//!
//! - [`ThemePackageManifest`] — a versioned theme-package manifest: a stable
//!   package id, a version label and revision ref, provenance, signature
//!   state, the supported theme / density / motion modes, the semantic /
//!   component / syntax token sets, contrast metadata, the design-token schema
//!   range, the build-compatibility state, and the inheritance axes the
//!   package expects consuming surfaces to honour. Manifests are a re-export
//!   projection of the canonical `theme_package_manifest_record` frozen in
//!   `schemas/ux/theme_package_manifest.schema.json`; this lane mints no
//!   parallel theme vocabulary.
//! - [`ThemePackageSurfaceBinding`] — one M5 surface's *active package
//!   identity*: the package it rides, the modes it honours, the inheritance
//!   posture and any disclosed inheritance gaps, the package provenance and
//!   stale-or-disabled evidence state it discloses, and whether it rides the
//!   shared appearance-session model rather than painting its own appearance.
//! - [`ThemePackageManifestReport`] — the canonical M5 theme-package manifest
//!   audit: the registered manifests, the per-surface bindings, per-package
//!   coverage, a provenance index, a blocking-finding summary, and the list of
//!   marketed surfaces release tooling should narrow.
//!
//! The resulting report is the single source the About/help provenance card,
//! the diagnostics inspector, the support-export wrapper, and the
//! release-center packets consume; they read these objects rather than
//! restating theme behaviour manually.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered surface names an `active_package_id` that resolves to a
//!    manifest in the registry; an unknown package is a blocker.
//! 2. Every mode a surface claims to honour (theme, density, motion) is one the
//!    active package supports; claiming an unsupported mode is a blocker.
//! 3. Every inheritance axis the active package expects is either inherited or
//!    disclosed as a gap; a hidden inheritance gap is a blocker.
//! 4. Package provenance is disclosed, and a stale-or-disabled package state is
//!    disclosed too; an undisclosed downgrade (stale evidence on a marketed
//!    surface, or a disabled package still rendering without disclosure) is a
//!    blocker.
//! 5. Every surface carries a canonical appearance anchor, a non-empty
//!    accessibility note, and `registered_on_appearance_session = true`; a
//!    missing anchor or note, or a surface that paints its own appearance
//!    outside the session model, is a blocker.
//! 6. First-party (built-in) manifests carry the semantic, component, and
//!    syntax token sets and cover the dark, light, and reduced-motion modes the
//!    product already claims; a missing token set or required mode is a blocker.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/ux/m5/theme-package-modes/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_theme_package_manifest_audit`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every theme-package manifest record.
pub const THEME_PACKAGE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every theme-package record.
pub const THEME_PACKAGE_SHARED_CONTRACT_REF: &str = "shell:m5_theme_packages:v1";

/// Stable record kind for [`ThemePackageManifestReport`] payloads.
pub const THEME_PACKAGE_REPORT_RECORD_KIND: &str = "shell_m5_theme_package_manifest_report_record";

/// Stable record kind for [`ThemePackageSurfaceBinding`] payloads.
pub const THEME_PACKAGE_SURFACE_RECORD_KIND: &str = "shell_m5_theme_package_surface_binding_record";

/// Stable record kind for [`ThemePackageManifest`] payloads.
pub const THEME_PACKAGE_MANIFEST_RECORD_KIND: &str = "shell_m5_theme_package_manifest_record";

/// Stable record kind for [`ThemePackageSupportExport`] payloads.
pub const THEME_PACKAGE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_theme_package_manifest_support_export_record";

/// Stable report id quoted across surfaces.
pub const THEME_PACKAGE_REPORT_ID: &str = "shell:m5_theme_packages:audit:v1";

/// Stable support-export id quoted in the published wrapper.
pub const THEME_PACKAGE_SUPPORT_EXPORT_ID: &str = "support-export:m5-theme-packages:001";

/// Source schema ref for the canonical theme-package manifest-audit contract.
pub const THEME_PACKAGE_SOURCE_SCHEMA_REF: &str =
    "schemas/ux/m5-theme-package-manifest.schema.json";

/// Schema ref for the canonical theme-package manifest object this lane
/// re-exports by reference instead of re-declaring.
pub const THEME_PACKAGE_CANONICAL_MANIFEST_SCHEMA_REF: &str =
    "schemas/ux/theme_package_manifest.schema.json";

/// Path of the published markdown audit artifact.
pub const THEME_PACKAGE_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md";

/// Path of the published companion doc.
pub const THEME_PACKAGE_PUBLISHED_DOC_REF: &str = "docs/m5/theme-package-manifests.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Color theme mode a package supports. Re-exported from the canonical
/// theme-package `theme_class` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeModeClass {
    /// Dark reference theme.
    DarkReference,
    /// Light parity theme.
    LightParity,
    /// High-contrast dark theme.
    HighContrastDark,
    /// High-contrast light theme.
    HighContrastLight,
}

impl ThemeModeClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DarkReference => "dark_reference",
            Self::LightParity => "light_parity",
            Self::HighContrastDark => "high_contrast_dark",
            Self::HighContrastLight => "high_contrast_light",
        }
    }
}

/// Density mode a package supports. Re-exported from the canonical
/// theme-package `density_class` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DensityClass {
    /// Compact density.
    Compact,
    /// Standard density.
    Standard,
    /// Comfortable density.
    Comfortable,
}

impl DensityClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Standard => "standard",
            Self::Comfortable => "comfortable",
        }
    }
}

/// Motion / accessibility posture a package supports. Re-exported from the
/// canonical theme-package `accessibility_posture_class` vocabulary without
/// modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotionPostureClass {
    /// Standard motion.
    MotionStandard,
    /// Reduced motion.
    MotionReduced,
    /// Low-motion treatment.
    MotionLowMotion,
    /// Power-saver motion treatment.
    MotionPowerSaver,
    /// Critical hot-path motion treatment.
    MotionCriticalHotPath,
}

impl MotionPostureClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MotionStandard => "motion_standard",
            Self::MotionReduced => "motion_reduced",
            Self::MotionLowMotion => "motion_low_motion",
            Self::MotionPowerSaver => "motion_power_saver",
            Self::MotionCriticalHotPath => "motion_critical_hot_path",
        }
    }
}

/// Where a theme package came from. Re-exported from the canonical
/// theme-package `theme_package_distribution_class` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClass {
    /// Built into the product.
    BuiltInWithProduct,
    /// Contributed by an extension.
    ExtensionContributed,
    /// Supplied by the community.
    CommunitySupplied,
    /// Imported and translated from another editor's theme format.
    ImportedTranslated,
    /// Authored for an air-gapped / offline deployment.
    AirGappedOffline,
}

impl ProvenanceClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInWithProduct => "built_in_with_product",
            Self::ExtensionContributed => "extension_contributed",
            Self::CommunitySupplied => "community_supplied",
            Self::ImportedTranslated => "imported_translated",
            Self::AirGappedOffline => "air_gapped_offline",
        }
    }

    /// `true` for the first-party distribution that must cover the full token
    /// and mode set the product already claims.
    pub const fn is_first_party(self) -> bool {
        matches!(self, Self::BuiltInWithProduct)
    }
}

/// Signature / trust state of a theme package. Re-exported from the canonical
/// theme-package `theme_package_signature_state` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureState {
    /// Signed and verified.
    SignedVerified,
    /// Signed but the signature was not verified.
    SignedUnverified,
    /// Unsigned, rendering accepted under an explicit decision row.
    UnsignedExplicitAcceptance,
    /// Signature verification failed; the package MUST NOT render.
    SignatureFailedBlocked,
    /// Built-in; signatures do not apply.
    NotApplicableBuiltIn,
}

impl SignatureState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedVerified => "signed_verified",
            Self::SignedUnverified => "signed_unverified",
            Self::UnsignedExplicitAcceptance => "unsigned_explicit_acceptance",
            Self::SignatureFailedBlocked => "signature_failed_blocked",
            Self::NotApplicableBuiltIn => "not_applicable_built_in",
        }
    }
}

/// Build-compatibility state of a package against the running build.
/// Re-exported from the canonical `version_match_state` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityState {
    /// Exact build match.
    ExactBuildMatch,
    /// Compatible minor drift.
    CompatibleMinorDrift,
    /// Incompatible drift detected.
    IncompatibleDriftDetected,
    /// Pre-release, unverified.
    PreReleaseUnverified,
    /// Unknown target build.
    UnknownTargetBuild,
}

impl CompatibilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }
}

/// Kind of token set a theme package contributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenSetKind {
    /// Semantic tokens (trust, severity, lifecycle meaning).
    Semantic,
    /// Component tokens (surface, control, chrome roles).
    Component,
    /// Syntax tokens (editor / notebook highlighting roles).
    Syntax,
}

impl TokenSetKind {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::Component => "component",
            Self::Syntax => "syntax",
        }
    }

    /// The three token-set kinds a first-party package must carry.
    pub const fn required_kinds() -> [Self; 3] {
        [Self::Semantic, Self::Component, Self::Syntax]
    }
}

/// One appearance axis a theme package expects consuming surfaces to inherit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InheritanceAxis {
    /// Theme (color mode) inheritance.
    Theme,
    /// Contrast inheritance.
    Contrast,
    /// Density inheritance.
    Density,
    /// Focus-visibility inheritance.
    Focus,
    /// Reduced-motion inheritance.
    ReducedMotion,
}

impl InheritanceAxis {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::Contrast => "contrast",
            Self::Density => "density",
            Self::Focus => "focus",
            Self::ReducedMotion => "reduced_motion",
        }
    }
}

/// Inheritance posture a surface reports against its active package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InheritancePosture {
    /// The surface inherits every axis the package expects.
    FullyInherited,
    /// The surface inherits part of the appearance posture and discloses the
    /// gaps for the axes it does not inherit.
    PartialInheritanceDisclosed,
    /// The surface does not inherit the package and discloses that posture
    /// (e.g. an embedded third-party surface that paints its own theme).
    DoesNotInheritDisclosed,
}

impl InheritancePosture {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyInherited => "fully_inherited",
            Self::PartialInheritanceDisclosed => "partial_inheritance_disclosed",
            Self::DoesNotInheritDisclosed => "does_not_inherit_disclosed",
        }
    }
}

/// Evidence state a surface discloses for its active package.
///
/// A non-`Current` state is honest **only** when it is disclosed. A disclosed
/// stale-or-disabled state still qualifies the surface; an undisclosed one is a
/// blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageEvidenceState {
    /// Fully supported, fresh evidence.
    Current,
    /// Captured appearance evidence has aged out.
    StaleEvidence,
    /// The package is disabled (e.g. signature failed or the author revoked it)
    /// and the surface falls back to the default package.
    DisabledPackage,
}

impl PackageEvidenceState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleEvidence => "stale_evidence",
            Self::DisabledPackage => "disabled_package",
        }
    }

    /// `true` for the states that represent a disclosed downgrade.
    pub const fn is_downgrade(self) -> bool {
        matches!(self, Self::StaleEvidence | Self::DisabledPackage)
    }
}

/// How much trust, lifecycle, or severity meaning a surface conveys.
///
/// A high-salience surface must keep that meaning legible across every theme
/// package and mode, so the audit requires its inheritance posture and
/// provenance to be disclosed rather than implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticSalience {
    /// Purely decorative; carries no semantic meaning.
    DecorativeOnly,
    /// Informational only; no trust, lifecycle, or severity meaning.
    Informational,
    /// Conveys lifecycle state (preview, stale, pending).
    LifecycleBearing,
    /// Conveys trust or identity (companion presence, boundary).
    TrustBearing,
    /// Conveys severity or risk (blocked, destructive, failed).
    SeverityBearing,
}

impl SemanticSalience {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DecorativeOnly => "decorative_only",
            Self::Informational => "informational",
            Self::LifecycleBearing => "lifecycle_bearing",
            Self::TrustBearing => "trust_bearing",
            Self::SeverityBearing => "severity_bearing",
        }
    }

    /// `true` for salience classes that must never hide their meaning.
    pub const fn is_high_salience(self) -> bool {
        matches!(
            self,
            Self::LifecycleBearing | Self::TrustBearing | Self::SeverityBearing
        )
    }
}

/// M5 surface family whose active theme package the audit makes inspectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemePackageSurfaceFamily {
    /// Notebook cell chrome and output frames.
    Notebook,
    /// Data / API result grids.
    ResultGrid,
    /// Profiler and trace timelines.
    ProfilerTimeline,
    /// Preview / embedded browser panes.
    PreviewBrowserPane,
    /// Docs / help panes.
    DocsHelpPane,
    /// Companion / cross-device surfaces.
    CompanionSurface,
    /// Extension-backed themed panels.
    ExtensionBackedSurface,
}

impl ThemePackageSurfaceFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::ResultGrid => "result_grid",
            Self::ProfilerTimeline => "profiler_timeline",
            Self::PreviewBrowserPane => "preview_browser_pane",
            Self::DocsHelpPane => "docs_help_pane",
            Self::CompanionSurface => "companion_surface",
            Self::ExtensionBackedSurface => "extension_backed_surface",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Notebook => "Notebook",
            Self::ResultGrid => "Result grid",
            Self::ProfilerTimeline => "Profiler timeline",
            Self::PreviewBrowserPane => "Preview / browser pane",
            Self::DocsHelpPane => "Docs / help pane",
            Self::CompanionSurface => "Companion surface",
            Self::ExtensionBackedSurface => "Extension-backed surface",
        }
    }
}

/// Contrast metadata captured on a theme-package manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageContrastMetadata {
    /// Opaque ref to the captured contrast evidence (raw ratios never cross
    /// this boundary).
    pub contrast_evidence_ref: String,
    /// `true` when normal-text contrast meets WCAG AA.
    pub meets_aa_normal_text: bool,
    /// `true` when normal-text contrast meets WCAG AAA.
    pub meets_aaa_normal_text: bool,
    /// `true` when safety-critical meaning survives forced-colors mode.
    pub forced_colors_preserved: bool,
}

/// One token set a theme package contributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageTokenSet {
    /// Token-set kind (semantic / component / syntax).
    pub kind: TokenSetKind,
    /// Opaque ref to the token-set rows (raw token tables never cross this
    /// boundary).
    pub token_set_ref: String,
    /// Count of tokens the set defines. Used for inspection, not raw values.
    pub token_count: u32,
}

/// A versioned theme-package manifest.
///
/// This is the report-level projection of the canonical
/// `theme_package_manifest_record`. It carries identity, version, provenance,
/// supported modes, token sets, contrast metadata, the design-token schema
/// range, build-compatibility state, and the inheritance axes the package
/// expects consuming surfaces to honour.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageManifest {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Schema ref for the canonical theme-package manifest object.
    pub canonical_manifest_schema_ref: String,
    /// Opaque, stable theme-package id. Frozen on first publication.
    pub package_id: String,
    /// Human-readable version label (e.g. `aureline-default-1.4.0`).
    pub package_version_label: String,
    /// Opaque revision ref for this manifest emission.
    pub package_revision_ref: String,
    /// Where the package came from.
    pub provenance_class: ProvenanceClass,
    /// Signature / trust state.
    pub signature_state: SignatureState,
    /// Theme modes the package supports, in canonical order.
    pub supported_theme_modes: Vec<ThemeModeClass>,
    /// Density modes the package supports, in canonical order.
    pub supported_density_classes: Vec<DensityClass>,
    /// Motion postures the package supports, in canonical order.
    pub supported_motion_postures: Vec<MotionPostureClass>,
    /// Token sets the package contributes, in canonical kind order.
    pub token_sets: Vec<ThemePackageTokenSet>,
    /// Contrast metadata captured for the package.
    pub contrast_metadata: ThemePackageContrastMetadata,
    /// Minimum design-token schema version the package supports.
    pub min_design_token_schema_version: u32,
    /// Maximum design-token schema version the package supports.
    pub max_design_token_schema_version: u32,
    /// Build-compatibility state against the running build.
    pub compatibility_state: CompatibilityState,
    /// Appearance axes the package expects consuming surfaces to inherit.
    pub inheritance_expectations: Vec<InheritanceAxis>,
    /// Optional opaque ref to the import-mapping report (required when the
    /// package is `imported_translated`).
    pub import_mapping_report_ref: Option<String>,
    /// Short privacy-safe provenance note.
    pub provenance_note: String,
    /// Timestamp the manifest was minted.
    pub minted_at: String,
}

impl ThemePackageManifest {
    /// `true` when the package supports the given theme mode.
    pub fn supports_theme_mode(&self, mode: ThemeModeClass) -> bool {
        self.supported_theme_modes.contains(&mode)
    }

    /// `true` when the package supports the given density class.
    pub fn supports_density(&self, density: DensityClass) -> bool {
        self.supported_density_classes.contains(&density)
    }

    /// `true` when the package supports the given motion posture.
    pub fn supports_motion(&self, posture: MotionPostureClass) -> bool {
        self.supported_motion_postures.contains(&posture)
    }

    /// `true` when the package carries the given token-set kind.
    pub fn has_token_set(&self, kind: TokenSetKind) -> bool {
        self.token_sets.iter().any(|set| set.kind == kind)
    }

    /// `true` when the package expects consuming surfaces to inherit `axis`.
    pub fn expects_inheritance(&self, axis: InheritanceAxis) -> bool {
        self.inheritance_expectations.contains(&axis)
    }
}

/// Canonical descriptor for one M5 surface's theme-package binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageSurfaceDescriptor {
    /// Stable surface id (e.g. `surface:notebook.cell_chrome`).
    pub surface_id: String,
    /// Surface family the descriptor belongs to.
    pub surface_family: ThemePackageSurfaceFamily,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical appearance anchor ref the audit can reopen the surface's
    /// design-QA entry from.
    pub appearance_anchor_ref: String,
    /// Accessibility note retained on the descriptor. MUST be non-empty.
    pub accessibility_note: String,
    /// Pinned semantic salience.
    pub semantic_salience: SemanticSalience,
    /// `true` when the surface is marketed on desktop appearance rows and
    /// therefore must keep fresh, disclosed appearance evidence.
    pub marketed_on_desktop_rows: bool,
    /// `true` once the surface rides the shared appearance-session model and
    /// does not paint its own appearance. MUST be `true`.
    pub registered_on_appearance_session: bool,
}

impl ThemePackageSurfaceDescriptor {
    /// `true` when this surface's salience makes it high-salience for the
    /// audit.
    pub const fn is_high_salience(&self) -> bool {
        self.semantic_salience.is_high_salience()
    }
}

/// One M5 surface's active-package binding: which versioned theme package it
/// rides, which modes it honours, and what it inherits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageSurfaceBinding {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Canonical surface descriptor.
    pub descriptor: ThemePackageSurfaceDescriptor,
    /// Active theme-package id the surface rides. MUST resolve to a manifest.
    pub active_package_id: String,
    /// Theme modes the surface honours (a subset of the package's support).
    pub honored_theme_modes: Vec<ThemeModeClass>,
    /// Density modes the surface honours (a subset of the package's support).
    pub honored_density_classes: Vec<DensityClass>,
    /// Motion postures the surface honours (a subset of the package's support).
    pub honored_motion_postures: Vec<MotionPostureClass>,
    /// Inheritance posture the surface reports.
    pub inheritance_posture: InheritancePosture,
    /// Appearance axes the surface actually inherits.
    pub inherited_axes: Vec<InheritanceAxis>,
    /// Appearance axes the surface discloses it does **not** inherit.
    pub disclosed_inheritance_gaps: Vec<InheritanceAxis>,
    /// `true` once the surface discloses the package provenance in product,
    /// export, and diagnostics. MUST be `true`.
    pub provenance_disclosed: bool,
    /// Evidence state the surface discloses for its active package.
    pub evidence_state: PackageEvidenceState,
    /// Opaque ref to the captured appearance evidence for this binding.
    pub evidence_ref: String,
    /// `true` when the surface conveys trust, lifecycle, or severity meaning.
    pub high_salience: bool,
    /// `true` when the surface is marketed on desktop appearance rows.
    pub marketed: bool,
    /// Blocking findings detected for this surface.
    pub blocking_findings: Vec<ThemePackageBlockingFinding>,
}

/// A blocking finding the validator emits for a surface binding or a manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ThemePackageBlockingFinding {
    /// A surface names an active package that is not in the registry.
    ActivePackageUnknown {
        /// Surface id that named the unknown package.
        surface_id: String,
        /// The unknown package id.
        package_id: String,
    },
    /// A surface claims to honour a mode the active package does not support.
    UnsupportedModeClaimed {
        /// Surface id.
        surface_id: String,
        /// The unsupported mode token.
        mode: String,
    },
    /// A surface does not inherit an axis the package expects, and does not
    /// disclose the gap. Always a blocker (a hidden appearance downgrade).
    InheritanceGapHidden {
        /// Surface id.
        surface_id: String,
        /// The undisclosed axis.
        axis: String,
    },
    /// A surface does not disclose its package provenance.
    ProvenanceNotDisclosed {
        /// Surface id.
        surface_id: String,
    },
    /// A marketed surface carries stale appearance evidence.
    StaleEvidenceOnMarketedSurface {
        /// Surface id.
        surface_id: String,
    },
    /// A surface rides a disabled package without disclosing the disabled
    /// state. Always a blocker.
    DisabledPackageRenderingUndisclosed {
        /// Surface id.
        surface_id: String,
        /// The disabled package id.
        package_id: String,
    },
    /// A surface paints its own appearance outside the shared
    /// appearance-session model.
    SurfaceNotOnAppearanceSession {
        /// Surface id.
        surface_id: String,
    },
    /// A surface descriptor carries no appearance anchor.
    DescriptorMissingAppearanceAnchor {
        /// Surface id.
        surface_id: String,
    },
    /// A surface descriptor carries no accessibility note.
    MissingAccessibilityNote {
        /// Surface id.
        surface_id: String,
    },
    /// A surface's inheritance posture disagrees with its disclosed gaps.
    InheritancePostureMismatch {
        /// Surface id.
        surface_id: String,
    },
    /// A first-party manifest is missing a required token set.
    ManifestTokenSetIncomplete {
        /// Package id.
        package_id: String,
        /// The missing token-set kind.
        token_set_kind: String,
    },
    /// A first-party manifest is missing a mode the product already claims.
    ManifestMissingRequiredMode {
        /// Package id.
        package_id: String,
        /// The missing mode token.
        mode: String,
    },
    /// A manifest whose signature failed is still registered for rendering.
    ManifestSignatureFailedStillRegistered {
        /// Package id.
        package_id: String,
    },
}

impl ThemePackageBlockingFinding {
    /// Stable finding-class token.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::ActivePackageUnknown { .. } => "active_package_unknown",
            Self::UnsupportedModeClaimed { .. } => "unsupported_mode_claimed",
            Self::InheritanceGapHidden { .. } => "inheritance_gap_hidden",
            Self::ProvenanceNotDisclosed { .. } => "provenance_not_disclosed",
            Self::StaleEvidenceOnMarketedSurface { .. } => "stale_evidence_on_marketed_surface",
            Self::DisabledPackageRenderingUndisclosed { .. } => {
                "disabled_package_rendering_undisclosed"
            }
            Self::SurfaceNotOnAppearanceSession { .. } => "surface_not_on_appearance_session",
            Self::DescriptorMissingAppearanceAnchor { .. } => {
                "descriptor_missing_appearance_anchor"
            }
            Self::MissingAccessibilityNote { .. } => "missing_accessibility_note",
            Self::InheritancePostureMismatch { .. } => "inheritance_posture_mismatch",
            Self::ManifestTokenSetIncomplete { .. } => "manifest_token_set_incomplete",
            Self::ManifestMissingRequiredMode { .. } => "manifest_missing_required_mode",
            Self::ManifestSignatureFailedStillRegistered { .. } => {
                "manifest_signature_failed_still_registered"
            }
        }
    }

    /// The surface id the finding is scoped to, if any.
    pub fn surface_id(&self) -> Option<&str> {
        match self {
            Self::ActivePackageUnknown { surface_id, .. }
            | Self::UnsupportedModeClaimed { surface_id, .. }
            | Self::InheritanceGapHidden { surface_id, .. }
            | Self::ProvenanceNotDisclosed { surface_id }
            | Self::StaleEvidenceOnMarketedSurface { surface_id }
            | Self::DisabledPackageRenderingUndisclosed { surface_id, .. }
            | Self::SurfaceNotOnAppearanceSession { surface_id }
            | Self::DescriptorMissingAppearanceAnchor { surface_id }
            | Self::MissingAccessibilityNote { surface_id }
            | Self::InheritancePostureMismatch { surface_id } => Some(surface_id),
            Self::ManifestTokenSetIncomplete { .. }
            | Self::ManifestMissingRequiredMode { .. }
            | Self::ManifestSignatureFailedStillRegistered { .. } => None,
        }
    }

    /// The package id the finding is scoped to, if any.
    pub fn package_id(&self) -> Option<&str> {
        match self {
            Self::ActivePackageUnknown { package_id, .. }
            | Self::DisabledPackageRenderingUndisclosed { package_id, .. }
            | Self::ManifestTokenSetIncomplete { package_id, .. }
            | Self::ManifestMissingRequiredMode { package_id, .. }
            | Self::ManifestSignatureFailedStillRegistered { package_id } => Some(package_id),
            _ => None,
        }
    }
}

/// Per-class blocking-finding tally.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageFindingSummary {
    /// `active_package_unknown` count.
    pub active_package_unknown: usize,
    /// `unsupported_mode_claimed` count.
    pub unsupported_mode_claimed: usize,
    /// `inheritance_gap_hidden` count.
    pub inheritance_gap_hidden: usize,
    /// `provenance_not_disclosed` count.
    pub provenance_not_disclosed: usize,
    /// `stale_evidence_on_marketed_surface` count.
    pub stale_evidence_on_marketed_surface: usize,
    /// `disabled_package_rendering_undisclosed` count.
    pub disabled_package_rendering_undisclosed: usize,
    /// `surface_not_on_appearance_session` count.
    pub surface_not_on_appearance_session: usize,
    /// `descriptor_missing_appearance_anchor` count.
    pub descriptor_missing_appearance_anchor: usize,
    /// `missing_accessibility_note` count.
    pub missing_accessibility_note: usize,
    /// `inheritance_posture_mismatch` count.
    pub inheritance_posture_mismatch: usize,
    /// `manifest_token_set_incomplete` count.
    pub manifest_token_set_incomplete: usize,
    /// `manifest_missing_required_mode` count.
    pub manifest_missing_required_mode: usize,
    /// `manifest_signature_failed_still_registered` count.
    pub manifest_signature_failed_still_registered: usize,
    /// Total blocking findings across all classes.
    pub total_blocking_findings: usize,
}

impl ThemePackageFindingSummary {
    /// Records one finding into the tally.
    fn record(&mut self, finding: &ThemePackageBlockingFinding) {
        match finding {
            ThemePackageBlockingFinding::ActivePackageUnknown { .. } => {
                self.active_package_unknown += 1;
            }
            ThemePackageBlockingFinding::UnsupportedModeClaimed { .. } => {
                self.unsupported_mode_claimed += 1;
            }
            ThemePackageBlockingFinding::InheritanceGapHidden { .. } => {
                self.inheritance_gap_hidden += 1;
            }
            ThemePackageBlockingFinding::ProvenanceNotDisclosed { .. } => {
                self.provenance_not_disclosed += 1;
            }
            ThemePackageBlockingFinding::StaleEvidenceOnMarketedSurface { .. } => {
                self.stale_evidence_on_marketed_surface += 1;
            }
            ThemePackageBlockingFinding::DisabledPackageRenderingUndisclosed { .. } => {
                self.disabled_package_rendering_undisclosed += 1;
            }
            ThemePackageBlockingFinding::SurfaceNotOnAppearanceSession { .. } => {
                self.surface_not_on_appearance_session += 1;
            }
            ThemePackageBlockingFinding::DescriptorMissingAppearanceAnchor { .. } => {
                self.descriptor_missing_appearance_anchor += 1;
            }
            ThemePackageBlockingFinding::MissingAccessibilityNote { .. } => {
                self.missing_accessibility_note += 1;
            }
            ThemePackageBlockingFinding::InheritancePostureMismatch { .. } => {
                self.inheritance_posture_mismatch += 1;
            }
            ThemePackageBlockingFinding::ManifestTokenSetIncomplete { .. } => {
                self.manifest_token_set_incomplete += 1;
            }
            ThemePackageBlockingFinding::ManifestMissingRequiredMode { .. } => {
                self.manifest_missing_required_mode += 1;
            }
            ThemePackageBlockingFinding::ManifestSignatureFailedStillRegistered { .. } => {
                self.manifest_signature_failed_still_registered += 1;
            }
        }
        self.total_blocking_findings += 1;
    }
}

/// One row of the per-package coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageCoverageSummary {
    /// Package id.
    pub package_id: String,
    /// Provenance class.
    pub provenance_class: ProvenanceClass,
    /// Number of surfaces riding this package.
    pub surfaces_using: usize,
    /// Number of those surfaces that are marketed on desktop rows.
    pub marketed_surfaces_using: usize,
}

/// One entry of the provenance index: how a package's provenance and evidence
/// read to support, About/help, and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageProvenanceEntry {
    /// Package id.
    pub package_id: String,
    /// Human-readable version label.
    pub package_version_label: String,
    /// Provenance class.
    pub provenance_class: ProvenanceClass,
    /// Signature / trust state.
    pub signature_state: SignatureState,
    /// Build-compatibility state.
    pub compatibility_state: CompatibilityState,
    /// The most degraded evidence state any surface discloses for the package
    /// (`current` when every binding is fresh).
    pub disclosed_evidence_state: PackageEvidenceState,
}

/// One marketed surface release tooling should narrow because its appearance
/// evidence is stale or its package is disabled without a fresh path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageNarrowableSurface {
    /// Surface id that must narrow.
    pub surface_id: String,
    /// Active package id.
    pub package_id: String,
    /// Stable reason the surface is narrowable.
    pub reason: String,
}

/// M5 theme-package manifest audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageManifestReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Schema ref for the canonical theme-package manifest object.
    pub canonical_manifest_schema_ref: String,
    /// Registered theme-package manifests, sorted by `package_id`.
    pub manifests: Vec<ThemePackageManifest>,
    /// Per-surface active-package bindings, sorted by `descriptor.surface_id`.
    pub surfaces: Vec<ThemePackageSurfaceBinding>,
    /// Per-package coverage summary, sorted by `package_id`.
    pub package_coverage: Vec<ThemePackageCoverageSummary>,
    /// Provenance index, sorted by `package_id`.
    pub provenance_index: Vec<ThemePackageProvenanceEntry>,
    /// Per-class blocking-finding summary.
    pub findings_summary: ThemePackageFindingSummary,
    /// Number of registered theme packages.
    pub manifest_count: usize,
    /// Number of registered surfaces.
    pub registered_surface_count: usize,
    /// Number of high-salience surfaces.
    pub high_salience_surface_count: usize,
    /// Number of surfaces marketed on desktop appearance rows.
    pub marketed_surface_count: usize,
    /// Marketed surfaces release tooling should narrow.
    pub narrowable_marketed_surfaces: Vec<ThemePackageNarrowableSurface>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl ThemePackageManifestReport {
    /// Returns the manifest registered for `package_id`, if any.
    pub fn manifest(&self, package_id: &str) -> Option<&ThemePackageManifest> {
        self.manifests
            .iter()
            .find(|manifest| manifest.package_id == package_id)
    }

    /// Returns `true` when every registered surface resolves its active
    /// package to a manifest.
    pub fn every_surface_package_resolved(&self) -> bool {
        self.surfaces
            .iter()
            .all(|surface| self.manifest(&surface.active_package_id).is_some())
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: manifests={}, surfaces={}, high_salience={}, marketed={}, blocking={}, clean={}",
            self.manifest_count,
            self.registered_surface_count,
            self.high_salience_surface_count,
            self.marketed_surface_count,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.package_coverage {
            lines.push(format!(
                "package: {} -- provenance={}, surfaces={}, marketed={}",
                coverage.package_id,
                coverage.provenance_class.as_str(),
                coverage.surfaces_using,
                coverage.marketed_surfaces_using,
            ));
        }
        for entry in &self.provenance_index {
            lines.push(format!(
                "provenance: {} -- signature={}, compatibility={}, evidence={}",
                entry.package_id,
                entry.signature_state.as_str(),
                entry.compatibility_state.as_str(),
                entry.disclosed_evidence_state.as_str(),
            ));
        }
        for surface in &self.surfaces {
            for finding in &surface.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {}",
                    finding.class_token(),
                    finding.surface_id().unwrap_or("surface"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_surfaces {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.surface_id, narrowable.package_id, narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 theme-package manifest audit\n\n");
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::theme_packages`](../../../../crates/aureline-shell/src/theme_packages/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- report-md > \\\n  artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Canonical manifest schema: `{}`\n",
            self.canonical_manifest_schema_ref
        ));
        out.push_str(&format!(
            "- Registered theme packages: `{}`\n",
            self.manifest_count
        ));
        out.push_str(&format!(
            "- Registered M5 surfaces: `{}`\n",
            self.registered_surface_count
        ));
        out.push_str(&format!(
            "- High-salience surfaces: `{}`\n",
            self.high_salience_surface_count
        ));
        out.push_str(&format!(
            "- Marketed surfaces: `{}`\n",
            self.marketed_surface_count
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed surfaces: `{}`\n",
            self.narrowable_marketed_surfaces.len()
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

        out.push_str("## Theme packages\n\n");
        out.push_str(
            "| Package | Version | Provenance | Signature | Modes | Densities | Motion | Compatibility |\n\
             | ------- | ------- | ---------- | --------- | ----- | --------- | ------ | ------------- |\n",
        );
        for manifest in &self.manifests {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | {} | {} | {} | `{}` |\n",
                manifest.package_id,
                manifest.package_version_label,
                manifest.provenance_class.as_str(),
                manifest.signature_state.as_str(),
                manifest.supported_theme_modes.len(),
                manifest.supported_density_classes.len(),
                manifest.supported_motion_postures.len(),
                manifest.compatibility_state.as_str(),
            ));
        }
        out.push('\n');

        out.push_str("## Provenance index\n\n");
        out.push_str(
            "| Package | Provenance | Signature | Compatibility | Evidence |\n\
             | ------- | ---------- | --------- | ------------- | -------- |\n",
        );
        for entry in &self.provenance_index {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                entry.package_id,
                entry.provenance_class.as_str(),
                entry.signature_state.as_str(),
                entry.compatibility_state.as_str(),
                entry.disclosed_evidence_state.as_str(),
            ));
        }
        out.push('\n');

        out.push_str("## Per-package coverage\n\n");
        out.push_str(
            "| Package | Provenance | Surfaces | Marketed |\n| ------- | ---------- | -------: | -------: |\n",
        );
        for coverage in &self.package_coverage {
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} |\n",
                coverage.package_id,
                coverage.provenance_class.as_str(),
                coverage.surfaces_using,
                coverage.marketed_surfaces_using,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `active_package_unknown` | {} |\n",
            self.findings_summary.active_package_unknown
        ));
        out.push_str(&format!(
            "| `unsupported_mode_claimed` | {} |\n",
            self.findings_summary.unsupported_mode_claimed
        ));
        out.push_str(&format!(
            "| `inheritance_gap_hidden` | {} |\n",
            self.findings_summary.inheritance_gap_hidden
        ));
        out.push_str(&format!(
            "| `provenance_not_disclosed` | {} |\n",
            self.findings_summary.provenance_not_disclosed
        ));
        out.push_str(&format!(
            "| `stale_evidence_on_marketed_surface` | {} |\n",
            self.findings_summary.stale_evidence_on_marketed_surface
        ));
        out.push_str(&format!(
            "| `disabled_package_rendering_undisclosed` | {} |\n",
            self.findings_summary.disabled_package_rendering_undisclosed
        ));
        out.push_str(&format!(
            "| `surface_not_on_appearance_session` | {} |\n",
            self.findings_summary.surface_not_on_appearance_session
        ));
        out.push_str(&format!(
            "| `descriptor_missing_appearance_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_appearance_anchor
        ));
        out.push_str(&format!(
            "| `missing_accessibility_note` | {} |\n",
            self.findings_summary.missing_accessibility_note
        ));
        out.push_str(&format!(
            "| `inheritance_posture_mismatch` | {} |\n",
            self.findings_summary.inheritance_posture_mismatch
        ));
        out.push_str(&format!(
            "| `manifest_token_set_incomplete` | {} |\n",
            self.findings_summary.manifest_token_set_incomplete
        ));
        out.push_str(&format!(
            "| `manifest_missing_required_mode` | {} |\n",
            self.findings_summary.manifest_missing_required_mode
        ));
        out.push_str(&format!(
            "| `manifest_signature_failed_still_registered` | {} |\n\n",
            self.findings_summary
                .manifest_signature_failed_still_registered
        ));

        out.push_str("## Per-surface bindings\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "### `{}` ({})\n\n",
                surface.descriptor.surface_id,
                surface.descriptor.surface_family.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                surface.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Active package: `{}`\n",
                surface.active_package_id
            ));
            out.push_str(&format!(
                "- Semantic salience: `{}`\n",
                surface.descriptor.semantic_salience.as_str()
            ));
            out.push_str(&format!(
                "- Appearance anchor: `{}`\n",
                surface.descriptor.appearance_anchor_ref
            ));
            out.push_str(&format!(
                "- Inheritance posture: `{}`\n",
                surface.inheritance_posture.as_str()
            ));
            out.push_str(&format!(
                "- Provenance disclosed: `{}`\n",
                if surface.provenance_disclosed {
                    "yes"
                } else {
                    "no"
                }
            ));
            out.push_str(&format!(
                "- Evidence: `{}`\n",
                surface.evidence_state.as_str()
            ));
            out.push_str(&format!(
                "- Marketed on desktop rows: `{}`\n",
                if surface.marketed { "yes" } else { "no" }
            ));
            let honored_modes: Vec<&str> = surface
                .honored_theme_modes
                .iter()
                .map(|mode| mode.as_str())
                .collect();
            out.push_str(&format!(
                "- Honored theme modes: `{}`\n",
                honored_modes.join(", ")
            ));
            let gaps: Vec<&str> = surface
                .disclosed_inheritance_gaps
                .iter()
                .map(|axis| axis.as_str())
                .collect();
            out.push_str(&format!(
                "- Disclosed inheritance gaps: `{}`\n\n",
                if gaps.is_empty() {
                    "none".to_owned()
                } else {
                    gaps.join(", ")
                }
            ));

            if surface.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &surface.blocking_findings {
                    out.push_str(&format!("- `{}`\n", finding.class_token()));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_theme_package_fixtures\n");
        out.push_str("python3 tools/ci/m5/theme_package_manifest_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 theme-package manifest audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePackageSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: ThemePackageManifestReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ThemePackageSupportExport {
    /// Builds the support-export wrapper for an audit report.
    ///
    /// Every report id, package id, surface id, and descriptor revision is
    /// quoted as a case id so a support reviewer can pivot from a case to the
    /// surface or package that flagged a stale or disabled appearance state.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: ThemePackageManifestReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for manifest in &report.manifests {
            case_ids.push(manifest.package_id.clone());
            case_ids.push(manifest.package_revision_ref.clone());
        }
        for surface in &report.surfaces {
            case_ids.push(surface.descriptor.surface_id.clone());
            case_ids.push(surface.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: THEME_PACKAGE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: THEME_PACKAGE_SCHEMA_VERSION,
            shared_contract_ref: THEME_PACKAGE_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the blocking findings for one surface against its active package.
fn compute_surface_findings(
    descriptor: &ThemePackageSurfaceDescriptor,
    active_package_id: &str,
    honored_theme_modes: &[ThemeModeClass],
    honored_density_classes: &[DensityClass],
    honored_motion_postures: &[MotionPostureClass],
    inheritance_posture: InheritancePosture,
    inherited_axes: &[InheritanceAxis],
    disclosed_inheritance_gaps: &[InheritanceAxis],
    provenance_disclosed: bool,
    evidence_state: PackageEvidenceState,
    marketed: bool,
    manifest: Option<&ThemePackageManifest>,
) -> Vec<ThemePackageBlockingFinding> {
    let mut findings = Vec::new();
    let surface_id = descriptor.surface_id.clone();

    // Descriptor-level (surface-scoped) findings.
    if descriptor.appearance_anchor_ref.trim().is_empty() {
        findings.push(
            ThemePackageBlockingFinding::DescriptorMissingAppearanceAnchor {
                surface_id: surface_id.clone(),
            },
        );
    }
    if descriptor.accessibility_note.trim().is_empty() {
        findings.push(ThemePackageBlockingFinding::MissingAccessibilityNote {
            surface_id: surface_id.clone(),
        });
    }
    if !descriptor.registered_on_appearance_session {
        findings.push(ThemePackageBlockingFinding::SurfaceNotOnAppearanceSession {
            surface_id: surface_id.clone(),
        });
    }
    if !provenance_disclosed {
        findings.push(ThemePackageBlockingFinding::ProvenanceNotDisclosed {
            surface_id: surface_id.clone(),
        });
    }

    // The active package must resolve. Mode and inheritance checks below need
    // the manifest, so they only run when it resolves.
    let Some(manifest) = manifest else {
        findings.push(ThemePackageBlockingFinding::ActivePackageUnknown {
            surface_id,
            package_id: active_package_id.to_owned(),
        });
        return findings;
    };

    // Every honored mode must be one the package supports.
    for mode in honored_theme_modes {
        if !manifest.supports_theme_mode(*mode) {
            findings.push(ThemePackageBlockingFinding::UnsupportedModeClaimed {
                surface_id: surface_id.clone(),
                mode: mode.as_str().to_owned(),
            });
        }
    }
    for density in honored_density_classes {
        if !manifest.supports_density(*density) {
            findings.push(ThemePackageBlockingFinding::UnsupportedModeClaimed {
                surface_id: surface_id.clone(),
                mode: density.as_str().to_owned(),
            });
        }
    }
    for posture in honored_motion_postures {
        if !manifest.supports_motion(*posture) {
            findings.push(ThemePackageBlockingFinding::UnsupportedModeClaimed {
                surface_id: surface_id.clone(),
                mode: posture.as_str().to_owned(),
            });
        }
    }

    // Every inheritance axis the package expects must be inherited or disclosed
    // as a gap; an undisclosed gap is a hidden downgrade.
    for axis in &manifest.inheritance_expectations {
        let inherited = inherited_axes.contains(axis);
        let disclosed = disclosed_inheritance_gaps.contains(axis);
        if !inherited && !disclosed {
            findings.push(ThemePackageBlockingFinding::InheritanceGapHidden {
                surface_id: surface_id.clone(),
                axis: axis.as_str().to_owned(),
            });
        }
    }

    // The inheritance posture must agree with the disclosed gaps.
    let has_gaps = !disclosed_inheritance_gaps.is_empty();
    let posture_consistent = match inheritance_posture {
        InheritancePosture::FullyInherited => !has_gaps,
        InheritancePosture::PartialInheritanceDisclosed => has_gaps,
        InheritancePosture::DoesNotInheritDisclosed => true,
    };
    if !posture_consistent {
        findings.push(ThemePackageBlockingFinding::InheritancePostureMismatch {
            surface_id: surface_id.clone(),
        });
    }

    // A disabled package must not keep rendering as the active package without
    // disclosure (a fresh fallback path is disclosed through the evidence
    // state); a marketed surface must keep fresh evidence.
    match evidence_state {
        PackageEvidenceState::DisabledPackage => {
            if !provenance_disclosed {
                findings.push(
                    ThemePackageBlockingFinding::DisabledPackageRenderingUndisclosed {
                        surface_id: surface_id.clone(),
                        package_id: active_package_id.to_owned(),
                    },
                );
            }
        }
        PackageEvidenceState::StaleEvidence => {
            if marketed {
                findings.push(
                    ThemePackageBlockingFinding::StaleEvidenceOnMarketedSurface {
                        surface_id: surface_id.clone(),
                    },
                );
            }
        }
        PackageEvidenceState::Current => {}
    }

    findings
}

/// Computes the manifest-scoped blocking findings.
fn compute_manifest_findings(manifest: &ThemePackageManifest) -> Vec<ThemePackageBlockingFinding> {
    let mut findings = Vec::new();

    if manifest.signature_state == SignatureState::SignatureFailedBlocked {
        findings.push(
            ThemePackageBlockingFinding::ManifestSignatureFailedStillRegistered {
                package_id: manifest.package_id.clone(),
            },
        );
    }

    if manifest.provenance_class.is_first_party() {
        for kind in TokenSetKind::required_kinds() {
            if !manifest.has_token_set(kind) {
                findings.push(ThemePackageBlockingFinding::ManifestTokenSetIncomplete {
                    package_id: manifest.package_id.clone(),
                    token_set_kind: kind.as_str().to_owned(),
                });
            }
        }
        for mode in [ThemeModeClass::DarkReference, ThemeModeClass::LightParity] {
            if !manifest.supports_theme_mode(mode) {
                findings.push(ThemePackageBlockingFinding::ManifestMissingRequiredMode {
                    package_id: manifest.package_id.clone(),
                    mode: mode.as_str().to_owned(),
                });
            }
        }
        if !manifest.supports_motion(MotionPostureClass::MotionReduced) {
            findings.push(ThemePackageBlockingFinding::ManifestMissingRequiredMode {
                package_id: manifest.package_id.clone(),
                mode: MotionPostureClass::MotionReduced.as_str().to_owned(),
            });
        }
    }

    findings
}

/// Builds a [`ThemePackageSurfaceBinding`] from a descriptor and its claimed
/// modes, computing the blocking findings against the manifest registry.
#[allow(clippy::too_many_arguments)]
pub fn build_theme_package_surface_binding(
    descriptor: ThemePackageSurfaceDescriptor,
    active_package_id: impl Into<String>,
    honored_theme_modes: Vec<ThemeModeClass>,
    honored_density_classes: Vec<DensityClass>,
    honored_motion_postures: Vec<MotionPostureClass>,
    inheritance_posture: InheritancePosture,
    inherited_axes: Vec<InheritanceAxis>,
    disclosed_inheritance_gaps: Vec<InheritanceAxis>,
    provenance_disclosed: bool,
    evidence_state: PackageEvidenceState,
    evidence_ref: impl Into<String>,
    marketed: bool,
    manifests: &[ThemePackageManifest],
) -> ThemePackageSurfaceBinding {
    let active_package_id = active_package_id.into();
    let manifest = manifests
        .iter()
        .find(|manifest| manifest.package_id == active_package_id);
    let high_salience = descriptor.is_high_salience();
    let blocking_findings = compute_surface_findings(
        &descriptor,
        &active_package_id,
        &honored_theme_modes,
        &honored_density_classes,
        &honored_motion_postures,
        inheritance_posture,
        &inherited_axes,
        &disclosed_inheritance_gaps,
        provenance_disclosed,
        evidence_state,
        marketed,
        manifest,
    );

    ThemePackageSurfaceBinding {
        record_kind: THEME_PACKAGE_SURFACE_RECORD_KIND.to_owned(),
        schema_version: THEME_PACKAGE_SCHEMA_VERSION,
        shared_contract_ref: THEME_PACKAGE_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        active_package_id,
        honored_theme_modes,
        honored_density_classes,
        honored_motion_postures,
        inheritance_posture,
        inherited_axes,
        disclosed_inheritance_gaps,
        provenance_disclosed,
        evidence_state,
        evidence_ref: evidence_ref.into(),
        high_salience,
        marketed,
        blocking_findings,
    }
}

/// Builds a full [`ThemePackageManifestReport`] from the manifest registry and
/// the per-surface bindings.
pub fn build_theme_package_manifest_audit(
    manifests: Vec<ThemePackageManifest>,
    surfaces: Vec<ThemePackageSurfaceBinding>,
) -> ThemePackageManifestReport {
    let mut manifests = manifests;
    manifests.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    let mut surfaces = surfaces;
    surfaces.sort_by(|left, right| left.descriptor.surface_id.cmp(&right.descriptor.surface_id));

    let manifest_count = manifests.len();
    let registered_surface_count = surfaces.len();
    let high_salience_surface_count = surfaces.iter().filter(|s| s.high_salience).count();
    let marketed_surface_count = surfaces.iter().filter(|s| s.marketed).count();

    let mut findings_summary = ThemePackageFindingSummary::default();
    for manifest in &manifests {
        for finding in compute_manifest_findings(manifest) {
            findings_summary.record(&finding);
        }
    }
    for surface in &surfaces {
        for finding in &surface.blocking_findings {
            findings_summary.record(finding);
        }
    }

    // Per-package coverage.
    let mut package_coverage: Vec<ThemePackageCoverageSummary> = manifests
        .iter()
        .map(|manifest| {
            let surfaces_using = surfaces
                .iter()
                .filter(|s| s.active_package_id == manifest.package_id)
                .count();
            let marketed_surfaces_using = surfaces
                .iter()
                .filter(|s| s.active_package_id == manifest.package_id && s.marketed)
                .count();
            ThemePackageCoverageSummary {
                package_id: manifest.package_id.clone(),
                provenance_class: manifest.provenance_class,
                surfaces_using,
                marketed_surfaces_using,
            }
        })
        .collect();
    package_coverage.sort_by(|left, right| left.package_id.cmp(&right.package_id));

    // Provenance index: the most degraded disclosed evidence state per package.
    let mut provenance_index: Vec<ThemePackageProvenanceEntry> = manifests
        .iter()
        .map(|manifest| {
            let disclosed_evidence_state = surfaces
                .iter()
                .filter(|s| s.active_package_id == manifest.package_id)
                .map(|s| s.evidence_state)
                .max_by_key(|state| match state {
                    PackageEvidenceState::Current => 0,
                    PackageEvidenceState::StaleEvidence => 1,
                    PackageEvidenceState::DisabledPackage => 2,
                })
                .unwrap_or(PackageEvidenceState::Current);
            ThemePackageProvenanceEntry {
                package_id: manifest.package_id.clone(),
                package_version_label: manifest.package_version_label.clone(),
                provenance_class: manifest.provenance_class,
                signature_state: manifest.signature_state,
                compatibility_state: manifest.compatibility_state,
                disclosed_evidence_state,
            }
        })
        .collect();
    provenance_index.sort_by(|left, right| left.package_id.cmp(&right.package_id));

    // Marketed surfaces release tooling should narrow.
    let mut narrowable_marketed_surfaces: Vec<ThemePackageNarrowableSurface> = surfaces
        .iter()
        .filter(|s| s.marketed && s.evidence_state.is_downgrade())
        .map(|s| ThemePackageNarrowableSurface {
            surface_id: s.descriptor.surface_id.clone(),
            package_id: s.active_package_id.clone(),
            reason: match s.evidence_state {
                PackageEvidenceState::StaleEvidence => "stale_appearance_evidence".to_owned(),
                PackageEvidenceState::DisabledPackage => "active_package_disabled".to_owned(),
                PackageEvidenceState::Current => "current".to_owned(),
            },
        })
        .collect();
    narrowable_marketed_surfaces.sort_by(|left, right| left.surface_id.cmp(&right.surface_id));

    let report_clean = findings_summary.total_blocking_findings == 0;

    ThemePackageManifestReport {
        record_kind: THEME_PACKAGE_REPORT_RECORD_KIND.to_owned(),
        schema_version: THEME_PACKAGE_SCHEMA_VERSION,
        shared_contract_ref: THEME_PACKAGE_SHARED_CONTRACT_REF.to_owned(),
        report_id: THEME_PACKAGE_REPORT_ID.to_owned(),
        source_schema_ref: THEME_PACKAGE_SOURCE_SCHEMA_REF.to_owned(),
        canonical_manifest_schema_ref: THEME_PACKAGE_CANONICAL_MANIFEST_SCHEMA_REF.to_owned(),
        manifests,
        surfaces,
        package_coverage,
        provenance_index,
        findings_summary,
        manifest_count,
        registered_surface_count,
        high_salience_surface_count,
        marketed_surface_count,
        narrowable_marketed_surfaces,
        report_clean,
        published_report_ref: THEME_PACKAGE_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: THEME_PACKAGE_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            THEME_PACKAGE_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/appearance-and-density-parity.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-theme-packages".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_theme_package_manifests`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ThemePackageValidationError {
    /// The audit has no registered theme packages.
    NoRegisteredManifests,
    /// The audit has no registered surfaces.
    NoRegisteredSurfaces,
    /// A surface's active package does not resolve to a manifest.
    SurfacePackageUnresolved {
        /// Surface id.
        surface_id: String,
        /// The unresolved package id.
        package_id: String,
    },
    /// A surface descriptor revision ref is empty.
    MissingDescriptorRevisionRef {
        /// Surface id.
        surface_id: String,
    },
    /// A blocking finding remains on a surface.
    SurfaceBlockingFindingPresent {
        /// Surface id.
        surface_id: String,
        /// Finding class.
        class: String,
    },
    /// A blocking finding remains on a manifest.
    ManifestBlockingFindingPresent {
        /// Package id.
        package_id: String,
        /// Finding class.
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates an audit report against the M5 theme-package acceptance
/// invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_theme_package_manifests(
    report: &ThemePackageManifestReport,
) -> Result<(), Vec<ThemePackageValidationError>> {
    let mut errors = Vec::new();

    if report.manifests.is_empty() {
        errors.push(ThemePackageValidationError::NoRegisteredManifests);
    }
    if report.surfaces.is_empty() {
        errors.push(ThemePackageValidationError::NoRegisteredSurfaces);
    }

    for manifest in &report.manifests {
        for finding in compute_manifest_findings(manifest) {
            errors.push(
                ThemePackageValidationError::ManifestBlockingFindingPresent {
                    package_id: manifest.package_id.clone(),
                    class: finding.class_token().to_owned(),
                },
            );
        }
    }

    for surface in &report.surfaces {
        if report.manifest(&surface.active_package_id).is_none() {
            errors.push(ThemePackageValidationError::SurfacePackageUnresolved {
                surface_id: surface.descriptor.surface_id.clone(),
                package_id: surface.active_package_id.clone(),
            });
        }
        if surface.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(ThemePackageValidationError::MissingDescriptorRevisionRef {
                surface_id: surface.descriptor.surface_id.clone(),
            });
        }
        for finding in &surface.blocking_findings {
            errors.push(ThemePackageValidationError::SurfaceBlockingFindingPresent {
                surface_id: surface.descriptor.surface_id.clone(),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(ThemePackageValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(ThemePackageValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Returns the canonical, deterministic theme-package manifest registry the
/// product already claims: a built-in default pack, a built-in high-contrast
/// pack, and a signed extension-contributed pack.
fn seeded_manifests() -> Vec<ThemePackageManifest> {
    let default_pack = ThemePackageManifest {
        record_kind: THEME_PACKAGE_MANIFEST_RECORD_KIND.to_owned(),
        schema_version: THEME_PACKAGE_SCHEMA_VERSION,
        canonical_manifest_schema_ref: THEME_PACKAGE_CANONICAL_MANIFEST_SCHEMA_REF.to_owned(),
        package_id: "theme-pkg:aureline-default".to_owned(),
        package_version_label: "aureline-default-1.4.0".to_owned(),
        package_revision_ref: "theme-rev:aureline-default:1.4.0".to_owned(),
        provenance_class: ProvenanceClass::BuiltInWithProduct,
        signature_state: SignatureState::NotApplicableBuiltIn,
        supported_theme_modes: vec![
            ThemeModeClass::DarkReference,
            ThemeModeClass::LightParity,
            ThemeModeClass::HighContrastDark,
            ThemeModeClass::HighContrastLight,
        ],
        supported_density_classes: vec![
            DensityClass::Compact,
            DensityClass::Standard,
            DensityClass::Comfortable,
        ],
        supported_motion_postures: vec![
            MotionPostureClass::MotionStandard,
            MotionPostureClass::MotionReduced,
            MotionPostureClass::MotionLowMotion,
            MotionPostureClass::MotionPowerSaver,
            MotionPostureClass::MotionCriticalHotPath,
        ],
        token_sets: vec![
            ThemePackageTokenSet {
                kind: TokenSetKind::Semantic,
                token_set_ref: "token-set:aureline-default:semantic".to_owned(),
                token_count: 96,
            },
            ThemePackageTokenSet {
                kind: TokenSetKind::Component,
                token_set_ref: "token-set:aureline-default:component".to_owned(),
                token_count: 184,
            },
            ThemePackageTokenSet {
                kind: TokenSetKind::Syntax,
                token_set_ref: "token-set:aureline-default:syntax".to_owned(),
                token_count: 72,
            },
        ],
        contrast_metadata: ThemePackageContrastMetadata {
            contrast_evidence_ref: "contrast-evidence:aureline-default".to_owned(),
            meets_aa_normal_text: true,
            meets_aaa_normal_text: true,
            forced_colors_preserved: true,
        },
        min_design_token_schema_version: 1,
        max_design_token_schema_version: 1,
        compatibility_state: CompatibilityState::ExactBuildMatch,
        inheritance_expectations: vec![
            InheritanceAxis::Theme,
            InheritanceAxis::Contrast,
            InheritanceAxis::Density,
            InheritanceAxis::Focus,
            InheritanceAxis::ReducedMotion,
        ],
        import_mapping_report_ref: None,
        provenance_note: "Built-in reference theme package shipped with the product.".to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    };

    let high_contrast_pack = ThemePackageManifest {
        record_kind: THEME_PACKAGE_MANIFEST_RECORD_KIND.to_owned(),
        schema_version: THEME_PACKAGE_SCHEMA_VERSION,
        canonical_manifest_schema_ref: THEME_PACKAGE_CANONICAL_MANIFEST_SCHEMA_REF.to_owned(),
        package_id: "theme-pkg:aureline-high-contrast".to_owned(),
        package_version_label: "aureline-high-contrast-1.4.0".to_owned(),
        package_revision_ref: "theme-rev:aureline-high-contrast:1.4.0".to_owned(),
        provenance_class: ProvenanceClass::BuiltInWithProduct,
        signature_state: SignatureState::NotApplicableBuiltIn,
        supported_theme_modes: vec![
            ThemeModeClass::DarkReference,
            ThemeModeClass::LightParity,
            ThemeModeClass::HighContrastDark,
            ThemeModeClass::HighContrastLight,
        ],
        supported_density_classes: vec![
            DensityClass::Compact,
            DensityClass::Standard,
            DensityClass::Comfortable,
        ],
        supported_motion_postures: vec![
            MotionPostureClass::MotionStandard,
            MotionPostureClass::MotionReduced,
            MotionPostureClass::MotionLowMotion,
        ],
        token_sets: vec![
            ThemePackageTokenSet {
                kind: TokenSetKind::Semantic,
                token_set_ref: "token-set:aureline-high-contrast:semantic".to_owned(),
                token_count: 96,
            },
            ThemePackageTokenSet {
                kind: TokenSetKind::Component,
                token_set_ref: "token-set:aureline-high-contrast:component".to_owned(),
                token_count: 184,
            },
            ThemePackageTokenSet {
                kind: TokenSetKind::Syntax,
                token_set_ref: "token-set:aureline-high-contrast:syntax".to_owned(),
                token_count: 72,
            },
        ],
        contrast_metadata: ThemePackageContrastMetadata {
            contrast_evidence_ref: "contrast-evidence:aureline-high-contrast".to_owned(),
            meets_aa_normal_text: true,
            meets_aaa_normal_text: true,
            forced_colors_preserved: true,
        },
        min_design_token_schema_version: 1,
        max_design_token_schema_version: 1,
        compatibility_state: CompatibilityState::ExactBuildMatch,
        inheritance_expectations: vec![
            InheritanceAxis::Theme,
            InheritanceAxis::Contrast,
            InheritanceAxis::Density,
            InheritanceAxis::Focus,
            InheritanceAxis::ReducedMotion,
        ],
        import_mapping_report_ref: None,
        provenance_note: "Built-in high-contrast theme package shipped with the product."
            .to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    };

    let partner_pack = ThemePackageManifest {
        record_kind: THEME_PACKAGE_MANIFEST_RECORD_KIND.to_owned(),
        schema_version: THEME_PACKAGE_SCHEMA_VERSION,
        canonical_manifest_schema_ref: THEME_PACKAGE_CANONICAL_MANIFEST_SCHEMA_REF.to_owned(),
        package_id: "theme-pkg:partner-dusk".to_owned(),
        package_version_label: "partner-dusk-2026.04".to_owned(),
        package_revision_ref: "theme-rev:partner-dusk:2026.04".to_owned(),
        provenance_class: ProvenanceClass::ExtensionContributed,
        signature_state: SignatureState::SignedVerified,
        supported_theme_modes: vec![
            ThemeModeClass::DarkReference,
            ThemeModeClass::HighContrastDark,
        ],
        supported_density_classes: vec![DensityClass::Standard, DensityClass::Comfortable],
        supported_motion_postures: vec![
            MotionPostureClass::MotionStandard,
            MotionPostureClass::MotionReduced,
        ],
        token_sets: vec![
            ThemePackageTokenSet {
                kind: TokenSetKind::Semantic,
                token_set_ref: "token-set:partner-dusk:semantic".to_owned(),
                token_count: 64,
            },
            ThemePackageTokenSet {
                kind: TokenSetKind::Component,
                token_set_ref: "token-set:partner-dusk:component".to_owned(),
                token_count: 120,
            },
        ],
        contrast_metadata: ThemePackageContrastMetadata {
            contrast_evidence_ref: "contrast-evidence:partner-dusk".to_owned(),
            meets_aa_normal_text: true,
            meets_aaa_normal_text: false,
            forced_colors_preserved: true,
        },
        min_design_token_schema_version: 1,
        max_design_token_schema_version: 1,
        compatibility_state: CompatibilityState::CompatibleMinorDrift,
        inheritance_expectations: vec![
            InheritanceAxis::Theme,
            InheritanceAxis::Contrast,
            InheritanceAxis::Density,
            InheritanceAxis::Focus,
        ],
        import_mapping_report_ref: None,
        provenance_note: "Signed extension-contributed theme package; narrows to dark modes."
            .to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    };

    vec![default_pack, high_contrast_pack, partner_pack]
}

/// Returns the seeded, deterministic M5 theme-package manifest audit.
///
/// The audit is the single mint-from-truth source for the fixtures under
/// `fixtures/ux/m5/theme-package-modes/` and the markdown artifact under
/// `artifacts/ux/m5/theme-manifest-audit/`.
pub fn seeded_theme_package_manifest_audit() -> ThemePackageManifestReport {
    let manifests = seeded_manifests();
    let default_pack = "theme-pkg:aureline-default";
    let all_themes = vec![
        ThemeModeClass::DarkReference,
        ThemeModeClass::LightParity,
        ThemeModeClass::HighContrastDark,
        ThemeModeClass::HighContrastLight,
    ];
    let all_densities = vec![
        DensityClass::Compact,
        DensityClass::Standard,
        DensityClass::Comfortable,
    ];
    let std_motion = vec![
        MotionPostureClass::MotionStandard,
        MotionPostureClass::MotionReduced,
    ];
    let all_axes = vec![
        InheritanceAxis::Theme,
        InheritanceAxis::Contrast,
        InheritanceAxis::Density,
        InheritanceAxis::Focus,
        InheritanceAxis::ReducedMotion,
    ];

    let notebook = build_theme_package_surface_binding(
        ThemePackageSurfaceDescriptor {
            surface_id: "surface:notebook.cell_chrome".to_owned(),
            surface_family: ThemePackageSurfaceFamily::Notebook,
            descriptor_revision_ref: "theme-binding-rev:notebook.cell_chrome:1".to_owned(),
            primary_label_ref: "label:notebook.cell_chrome".to_owned(),
            appearance_anchor_ref: "anchor:notebook.cell_chrome".to_owned(),
            accessibility_note:
                "Notebook cell run-state badges keep lifecycle meaning in every theme.".to_owned(),
            semantic_salience: SemanticSalience::LifecycleBearing,
            marketed_on_desktop_rows: true,
            registered_on_appearance_session: true,
        },
        default_pack,
        all_themes.clone(),
        all_densities.clone(),
        std_motion.clone(),
        InheritancePosture::FullyInherited,
        all_axes.clone(),
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:notebook.cell_chrome".to_owned(),
        true,
        &manifests,
    );

    let result_grid = build_theme_package_surface_binding(
        ThemePackageSurfaceDescriptor {
            surface_id: "surface:data.result_grid".to_owned(),
            surface_family: ThemePackageSurfaceFamily::ResultGrid,
            descriptor_revision_ref: "theme-binding-rev:data.result_grid:1".to_owned(),
            primary_label_ref: "label:data.result_grid".to_owned(),
            appearance_anchor_ref: "anchor:data.result_grid".to_owned(),
            accessibility_note:
                "Result-grid severity cells keep their non-color cues in every theme.".to_owned(),
            semantic_salience: SemanticSalience::SeverityBearing,
            marketed_on_desktop_rows: true,
            registered_on_appearance_session: true,
        },
        default_pack,
        all_themes.clone(),
        all_densities.clone(),
        std_motion.clone(),
        InheritancePosture::FullyInherited,
        all_axes.clone(),
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:data.result_grid".to_owned(),
        true,
        &manifests,
    );

    let profiler = build_theme_package_surface_binding(
        ThemePackageSurfaceDescriptor {
            surface_id: "surface:profiler.timeline".to_owned(),
            surface_family: ThemePackageSurfaceFamily::ProfilerTimeline,
            descriptor_revision_ref: "theme-binding-rev:profiler.timeline:1".to_owned(),
            primary_label_ref: "label:profiler.timeline".to_owned(),
            appearance_anchor_ref: "anchor:profiler.timeline".to_owned(),
            accessibility_note:
                "Profiler timeline lanes stay legible across density and motion modes.".to_owned(),
            semantic_salience: SemanticSalience::Informational,
            marketed_on_desktop_rows: true,
            registered_on_appearance_session: true,
        },
        default_pack,
        all_themes.clone(),
        all_densities.clone(),
        std_motion.clone(),
        InheritancePosture::FullyInherited,
        all_axes.clone(),
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:profiler.timeline".to_owned(),
        true,
        &manifests,
    );

    let preview = build_theme_package_surface_binding(
        ThemePackageSurfaceDescriptor {
            surface_id: "surface:preview.browser_pane".to_owned(),
            surface_family: ThemePackageSurfaceFamily::PreviewBrowserPane,
            descriptor_revision_ref: "theme-binding-rev:preview.browser_pane:1".to_owned(),
            primary_label_ref: "label:preview.browser_pane".to_owned(),
            appearance_anchor_ref: "anchor:preview.browser_pane".to_owned(),
            accessibility_note: "Preview-pane trust boundary cue stays present in every theme."
                .to_owned(),
            semantic_salience: SemanticSalience::TrustBearing,
            marketed_on_desktop_rows: true,
            registered_on_appearance_session: true,
        },
        default_pack,
        all_themes.clone(),
        all_densities.clone(),
        std_motion.clone(),
        InheritancePosture::FullyInherited,
        all_axes.clone(),
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:preview.browser_pane".to_owned(),
        true,
        &manifests,
    );

    let docs = build_theme_package_surface_binding(
        ThemePackageSurfaceDescriptor {
            surface_id: "surface:docs.help_pane".to_owned(),
            surface_family: ThemePackageSurfaceFamily::DocsHelpPane,
            descriptor_revision_ref: "theme-binding-rev:docs.help_pane:1".to_owned(),
            primary_label_ref: "label:docs.help_pane".to_owned(),
            appearance_anchor_ref: "anchor:docs.help_pane".to_owned(),
            accessibility_note:
                "Docs/help pane inherits the active package and quotes its provenance.".to_owned(),
            semantic_salience: SemanticSalience::Informational,
            marketed_on_desktop_rows: true,
            registered_on_appearance_session: true,
        },
        default_pack,
        all_themes.clone(),
        all_densities.clone(),
        std_motion.clone(),
        InheritancePosture::FullyInherited,
        all_axes.clone(),
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:docs.help_pane".to_owned(),
        true,
        &manifests,
    );

    let companion = build_theme_package_surface_binding(
        ThemePackageSurfaceDescriptor {
            surface_id: "surface:companion.handoff".to_owned(),
            surface_family: ThemePackageSurfaceFamily::CompanionSurface,
            descriptor_revision_ref: "theme-binding-rev:companion.handoff:1".to_owned(),
            primary_label_ref: "label:companion.handoff".to_owned(),
            appearance_anchor_ref: "anchor:companion.handoff".to_owned(),
            accessibility_note: "Companion handoff presence cue keeps trust meaning across themes."
                .to_owned(),
            semantic_salience: SemanticSalience::TrustBearing,
            marketed_on_desktop_rows: true,
            registered_on_appearance_session: true,
        },
        default_pack,
        all_themes.clone(),
        all_densities.clone(),
        std_motion.clone(),
        InheritancePosture::FullyInherited,
        all_axes.clone(),
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:companion.handoff".to_owned(),
        true,
        &manifests,
    );

    // Extension-backed surface rides a signed partner pack and discloses that
    // it does not inherit reduced motion (the partner pack does not expect it).
    let extension = build_theme_package_surface_binding(
        ThemePackageSurfaceDescriptor {
            surface_id: "surface:extension.themed_panel".to_owned(),
            surface_family: ThemePackageSurfaceFamily::ExtensionBackedSurface,
            descriptor_revision_ref: "theme-binding-rev:extension.themed_panel:1".to_owned(),
            primary_label_ref: "label:extension.themed_panel".to_owned(),
            appearance_anchor_ref: "anchor:extension.themed_panel".to_owned(),
            accessibility_note:
                "Extension panel discloses its partner package and the focus axis it owns."
                    .to_owned(),
            semantic_salience: SemanticSalience::LifecycleBearing,
            marketed_on_desktop_rows: false,
            registered_on_appearance_session: true,
        },
        "theme-pkg:partner-dusk",
        vec![
            ThemeModeClass::DarkReference,
            ThemeModeClass::HighContrastDark,
        ],
        vec![DensityClass::Standard, DensityClass::Comfortable],
        std_motion.clone(),
        InheritancePosture::PartialInheritanceDisclosed,
        vec![
            InheritanceAxis::Theme,
            InheritanceAxis::Contrast,
            InheritanceAxis::Density,
        ],
        vec![InheritanceAxis::Focus],
        true,
        PackageEvidenceState::Current,
        "evidence:extension.themed_panel".to_owned(),
        false,
        &manifests,
    );

    build_theme_package_manifest_audit(
        manifests,
        vec![
            notebook,
            result_grid,
            profiler,
            preview,
            docs,
            companion,
            extension,
        ],
    )
}
