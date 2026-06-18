//! Governed appearance-inheritance descriptors for extension and embedded UI.
//!
//! An extension detail page, an embedded webview, a provider-backed panel, or a
//! contributed preview/docs/diagnostics pane can render rich custom UI. That is
//! fine — but a user must be able to tell whether such a surface actually
//! inherits Aureline's theme, focus, contrast, density, and reduced-motion
//! semantics or only approximates them. Without a descriptor the product can
//! inspect, that honesty lives in prose, support folklore, or manifest comments
//! the runtime cannot read, and a contributed surface can quietly drift from the
//! host appearance posture while marketing chrome still implies first-party
//! parity.
//!
//! This module makes appearance inheritance a governed, machine-readable
//! descriptor for the extension surfaces M5 touches. Each surface declares an
//! [`ExtensionAppearanceInput`] carrying a host id, a package id, and a posture
//! for each of the five host appearance axes, plus any known gaps and the
//! accessibility evidence backing a parity claim.
//! [`evaluate_extension_appearance_descriptor`] joins that declaration into an
//! [`ExtensionAppearanceDescriptor`] that:
//!
//! - derives a visible [`InheritanceBadge`]
//!   ([`InheritanceBadgeClass::FullInheritance`],
//!   [`InheritanceBadgeClass::PartialInheritance`],
//!   [`InheritanceBadgeClass::DoesNotInherit`], or
//!   [`InheritanceBadgeClass::Undisclosed`]) rendered in extension details,
//!   embedded panes, diagnostics, and support/export packets alike;
//! - resolves a [`ParityClaimStateClass`] that *blocks* a host-parity claim
//!   (`denied_claim`) unless every axis inherits, no gaps remain, and
//!   accessibility evidence backs the claim; and
//! - emits closed-vocabulary [`AppearanceDescriptorDefect`]s for an undisclosed
//!   axis, an overclaimed parity, a hidden inheritance gap, or a host badge the
//!   extension tried to suppress.
//!
//! The records are inspectable, serde-serializable truth packets that carry no
//! raw theme files, token values, screenshots, or user content — only opaque
//! refs, closed vocabulary, short labels, and counts. They are consumed by the
//! extension detail surface, embedded-pane chrome, post-install diagnostics, and
//! the support-export wrapper, and validated by the headless inspector
//! (`dump_extension_appearance_descriptor_records`), the gate
//! [`/tools/ci/m5/extension_appearance_descriptors_check.py`](../../../../tools/ci/m5/extension_appearance_descriptors_check.py),
//! and the contract test
//! [`/crates/aureline-extensions/tests/extension_appearance_descriptors_fixtures.rs`](../../../../crates/aureline-extensions/tests/extension_appearance_descriptors_fixtures.rs).
//!
//! The cross-tool schema is
//! [`/schemas/ux/extension-appearance-descriptor.schema.json`](../../../../schemas/ux/extension-appearance-descriptor.schema.json),
//! the reviewer-facing guide is
//! [`/docs/m5/extension-appearance-inheritance.md`](../../../../docs/m5/extension-appearance-inheritance.md),
//! and the checked fixture corpus lives under
//! [`/fixtures/ux/m5/extension-theme-inheritance/`](../../../../fixtures/ux/m5/extension-theme-inheritance/).
//!
//! The vocabulary is not minted here. The five inheritance axes
//! ([`AppearanceAxisClass`]) and the four parity-claim states
//! ([`ParityClaimStateClass`]) reuse the values already frozen in the canonical
//! appearance contracts — the design-side audited twin
//! `schemas/design/extension_ui_appearance_descriptor.schema.json` and the
//! user-facing `schemas/ux/appearance_checkpoint.schema.json`. The per-axis
//! posture reuses [`ExtensionInheritanceClass`] from the webview-boundary lane.
//! Only the visible badge and the closed defect vocabulary, which have no frozen
//! equivalent, are introduced by this runtime lane.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::conformance_reports::ReviewLifecycleClass;
use crate::webview_boundary::ExtensionInheritanceClass;

#[cfg(test)]
mod tests;

/// Schema version exported with every appearance-descriptor record.
pub const EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by detail, embedded, diagnostics, and support.
pub const EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF: &str =
    "extensions:m5_appearance_descriptor:v1";

/// Stable record kind for [`ExtensionAppearanceDescriptor`] payloads.
pub const EXTENSION_APPEARANCE_DESCRIPTOR_RECORD_KIND: &str =
    "extension_appearance_descriptor_record";

/// Stable record kind for [`ExtensionAppearanceAudit`] payloads.
pub const EXTENSION_APPEARANCE_AUDIT_RECORD_KIND: &str = "extension_appearance_audit_record";

/// Stable record kind for [`ExtensionAppearanceSupportRow`] payloads.
pub const EXTENSION_APPEARANCE_SUPPORT_ROW_RECORD_KIND: &str =
    "extension_appearance_support_row_record";

/// Stable record kind for [`ExtensionAppearanceSupportExport`] payloads.
pub const EXTENSION_APPEARANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_appearance_support_export_record";

/// Stable record kind for [`AppearanceDescriptorDefect`] payloads.
pub const EXTENSION_APPEARANCE_DESCRIPTOR_DEFECT_RECORD_KIND: &str =
    "extension_appearance_descriptor_defect_record";

/// Stable audit id surfaces pivot across.
pub const EXTENSION_APPEARANCE_AUDIT_ID: &str = "extensions:m5_appearance_descriptor:audit:v1";

/// Repo-relative ref to the boundary schema these records conform to.
pub const EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_REF: &str =
    "schemas/ux/extension-appearance-descriptor.schema.json";

/// Repo-relative ref to the frozen design-side audited descriptor contract this
/// runtime lane is the product-facing twin of.
pub const EXTENSION_APPEARANCE_DESIGN_CONTRACT_REF: &str =
    "schemas/design/extension_ui_appearance_descriptor.schema.json";

/// Published markdown artifact ref reviewers reopen the audit from.
pub const EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md";

/// Published companion doc ref.
pub const EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_DOC_REF: &str =
    "docs/m5/extension-appearance-inheritance.md";

/// Deterministic generated-at value carried by the seeded audit.
const GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Surface tokens every descriptor badge is rendered on, in stable order.
///
/// Used by the audit to prove the inheritance badge is visible in extension
/// details, embedded panes, diagnostics, and support/export packets rather than
/// hidden in docs-only notes.
pub const RENDERED_SURFACE_TOKENS: [&str; 4] = [
    "extension_detail",
    "embedded_pane",
    "diagnostics",
    "support_export",
];

// ---------------------------------------------------------------------------
// Vocabulary
// ---------------------------------------------------------------------------

/// Host appearance axis an extension surface can inherit.
///
/// Reuses the frozen `inheritance_axis_class` vocabulary from the canonical
/// appearance contracts (`schemas/design/extension_ui_appearance_descriptor`
/// and `schemas/ux/appearance_checkpoint`); this lane mints no parallel axis
/// values. `contrast` covers high-contrast and forced-colors modes; `focus`
/// covers the keyboard focus-ring and focus-token posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceAxisClass {
    /// Color/theme tokens (light, dark, and theme-package palettes).
    Theme,
    /// Keyboard focus-ring and focus-token posture.
    Focus,
    /// High-contrast / forced-colors tokens.
    Contrast,
    /// Density scale (compact, standard, comfortable) tokens.
    Density,
    /// Reduced-motion tokens and animation suppression.
    ReducedMotion,
}

impl AppearanceAxisClass {
    /// Returns the stable schema token for this axis.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::Focus => "focus",
            Self::Contrast => "contrast",
            Self::Density => "density",
            Self::ReducedMotion => "reduced_motion",
        }
    }

    /// Returns the short display label for this axis.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Theme => "Theme",
            Self::Focus => "Focus",
            Self::Contrast => "High contrast",
            Self::Density => "Density",
            Self::ReducedMotion => "Reduced motion",
        }
    }

    /// Returns the five axes in stable order.
    pub const fn all() -> [Self; 5] {
        [
            Self::Theme,
            Self::Focus,
            Self::Contrast,
            Self::Density,
            Self::ReducedMotion,
        ]
    }
}

/// Kind of extension-backed or embedded surface a descriptor describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKindClass {
    /// The extension detail page rendered inside host chrome.
    ExtensionDetailPane,
    /// An embedded webview contributed by an extension.
    EmbeddedWebview,
    /// A provider-backed panel (auth, sync, or external tool UI).
    ProviderPanel,
    /// A contributed preview pane.
    PreviewPane,
    /// An embedded docs/help pane.
    DocsHelpPane,
    /// A post-install diagnostics pane.
    DiagnosticsPane,
}

impl SurfaceKindClass {
    /// Returns the stable schema token for this surface kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionDetailPane => "extension_detail_pane",
            Self::EmbeddedWebview => "embedded_webview",
            Self::ProviderPanel => "provider_panel",
            Self::PreviewPane => "preview_pane",
            Self::DocsHelpPane => "docs_help_pane",
            Self::DiagnosticsPane => "diagnostics_pane",
        }
    }

    /// Returns the short display label for this surface kind.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExtensionDetailPane => "Extension detail",
            Self::EmbeddedWebview => "Embedded webview",
            Self::ProviderPanel => "Provider panel",
            Self::PreviewPane => "Preview pane",
            Self::DocsHelpPane => "Docs/help pane",
            Self::DiagnosticsPane => "Diagnostics pane",
        }
    }
}

/// Visible inheritance badge derived from the per-axis postures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InheritanceBadgeClass {
    /// Every axis inherits the host appearance contract.
    FullInheritance,
    /// Some axes inherit and at least one does not — disclosed partial parity.
    PartialInheritance,
    /// No axis inherits; the surface keeps private appearance logic.
    DoesNotInherit,
    /// At least one axis is undisclosed, so inheritance cannot be claimed.
    Undisclosed,
}

impl InheritanceBadgeClass {
    /// Returns the stable schema token for this badge.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullInheritance => "full_inheritance",
            Self::PartialInheritance => "partial_inheritance",
            Self::DoesNotInherit => "does_not_inherit",
            Self::Undisclosed => "undisclosed",
        }
    }

    /// Returns the short display label for this badge.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullInheritance => "Inherits Aureline appearance",
            Self::PartialInheritance => "Partly inherits appearance",
            Self::DoesNotInherit => "Does not inherit appearance",
            Self::Undisclosed => "Appearance inheritance undisclosed",
        }
    }

    /// Returns whether this badge is consistent with a host-parity claim.
    ///
    /// Only [`InheritanceBadgeClass::FullInheritance`] can back a host-parity
    /// claim.
    pub const fn permits_host_parity(self) -> bool {
        matches!(self, Self::FullInheritance)
    }
}

/// Resolved state of a surface's claim to first-party appearance parity.
///
/// Reuses the frozen `parity_claim_state_class` vocabulary from the canonical
/// appearance contracts; this lane mints no parallel parity values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityClaimStateClass {
    /// The surface makes no parity claim; its badge stands alone.
    NoParityClaim,
    /// The surface claims host parity and full inheritance plus evidence back it.
    ClaimsHostParity,
    /// The surface claims parity on the axes it inherits and discloses the gaps.
    PartialClaimWithGaps,
    /// The surface claimed parity it cannot back; the claim is denied.
    DeniedClaim,
}

impl ParityClaimStateClass {
    /// Returns the stable schema token for this parity state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoParityClaim => "no_parity_claim",
            Self::ClaimsHostParity => "claims_host_parity",
            Self::PartialClaimWithGaps => "partial_claim_with_gaps",
            Self::DeniedClaim => "denied_claim",
        }
    }

    /// Returns whether the parity claim was denied (blocked).
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::DeniedClaim)
    }
}

/// Closed defect vocabulary the gate refuses on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceDescriptorDefectKind {
    /// An axis posture is `not_disclosed`, so inheritance cannot be inspected.
    UndisclosedAxis,
    /// A host-parity claim is not backed by full inheritance + evidence.
    OverclaimedParity,
    /// A known gap or non-inheriting axis contradicts a full-inheritance badge.
    HiddenInheritanceGap,
    /// The host badge chrome is suppressed, hiding the posture from the user.
    HostBadgeChromeHidden,
    /// A support-export row disagrees with its descriptor.
    SupportExportParityDrift,
    /// Raw appearance material crossed the support-export boundary.
    RawAppearanceMaterialExported,
}

impl AppearanceDescriptorDefectKind {
    /// Returns the stable schema token for this defect kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UndisclosedAxis => "undisclosed_axis",
            Self::OverclaimedParity => "overclaimed_parity",
            Self::HiddenInheritanceGap => "hidden_inheritance_gap",
            Self::HostBadgeChromeHidden => "host_badge_chrome_hidden",
            Self::SupportExportParityDrift => "support_export_parity_drift",
            Self::RawAppearanceMaterialExported => "raw_appearance_material_exported",
        }
    }
}

// ---------------------------------------------------------------------------
// Input layer
// ---------------------------------------------------------------------------

/// A disclosed appearance gap on a named axis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceGap {
    /// Axis the gap applies to.
    pub axis: AppearanceAxisClass,
    /// Short, user-facing summary of what does not inherit.
    pub summary: String,
}

/// Declared appearance posture for one extension or embedded surface.
///
/// This is the raw declaration the host joins into an
/// [`ExtensionAppearanceDescriptor`]; it carries the per-axis posture flags the
/// spec requires (theme, contrast, density, focus, reduced motion) plus known
/// gaps, accessibility evidence, and the surface's own parity claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionAppearanceInput {
    /// Stable descriptor id (`extension-appearance:<package>:<surface>`).
    pub descriptor_id: String,
    /// Host pane/surface id this surface is embedded in.
    pub host_id: String,
    /// Extension/package id contributing the surface.
    pub package_id: String,
    /// Human-readable package name.
    pub package_name: String,
    /// Publisher label as rendered by the host.
    pub publisher_label: String,
    /// Kind of surface being described.
    pub surface_kind: SurfaceKindClass,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Review-lifecycle posture of the contributing extension.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Theme inheritance posture.
    pub inherit_theme: ExtensionInheritanceClass,
    /// Focus-token inheritance posture.
    pub inherit_focus: ExtensionInheritanceClass,
    /// High-contrast inheritance posture.
    pub inherit_contrast: ExtensionInheritanceClass,
    /// Density inheritance posture.
    pub inherit_density: ExtensionInheritanceClass,
    /// Reduced-motion inheritance posture.
    pub inherit_reduced_motion: ExtensionInheritanceClass,
    /// Disclosed appearance gaps, if any.
    #[serde(default)]
    pub known_gaps: Vec<AppearanceGap>,
    /// Accessibility evidence refs backing a parity claim.
    #[serde(default)]
    pub accessibility_evidence_refs: Vec<String>,
    /// Whether the surface claims first-party appearance parity.
    pub claims_first_party_parity: bool,
    /// Whether the host renders the appearance badge chrome (never suppressed).
    pub host_rendered_appearance_badge: bool,
    /// Short user-facing caveat summary.
    pub caveat_summary: String,
    /// Deterministic generated-at timestamp.
    pub generated_at: String,
}

// ---------------------------------------------------------------------------
// Output layer
// ---------------------------------------------------------------------------

/// Per-axis posture pairing on a derived descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceAxisPosture {
    /// Axis the posture applies to.
    pub axis: AppearanceAxisClass,
    /// Declared inheritance posture for the axis.
    pub posture: ExtensionInheritanceClass,
    /// Whether the user must see this posture disclosed (partial / does-not /
    /// undisclosed always require disclosure).
    pub user_visible_disclosure_required: bool,
}

/// Visible inheritance badge rendered on every consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InheritanceBadge {
    /// Derived badge class.
    pub badge_class: InheritanceBadgeClass,
    /// Short badge label.
    pub badge_label: String,
    /// One-line caveat shown beneath the badge.
    pub caveat_line: String,
    /// Whether the badge is consistent with a host-parity claim.
    pub implies_host_parity: bool,
}

/// Fully derived appearance descriptor for one extension or embedded surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionAppearanceDescriptor {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// Host pane/surface id.
    pub host_id: String,
    /// Extension/package id.
    pub package_id: String,
    /// Human-readable package name.
    pub package_name: String,
    /// Publisher label.
    pub publisher_label: String,
    /// Kind of surface.
    pub surface_kind: SurfaceKindClass,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Review-lifecycle posture.
    pub lifecycle_class: ReviewLifecycleClass,
    /// The five governed posture axes, in stable order.
    pub axes: Vec<AppearanceAxisPosture>,
    /// Disclosed appearance gaps.
    pub known_gaps: Vec<AppearanceGap>,
    /// Accessibility evidence refs.
    pub accessibility_evidence_refs: Vec<String>,
    /// Derived visible badge.
    pub badge: InheritanceBadge,
    /// Whether the surface claims first-party parity.
    pub claims_first_party_parity: bool,
    /// Resolved parity-claim state.
    pub parity_claim_state: ParityClaimStateClass,
    /// Plain-language reason for the parity state.
    pub parity_reason: String,
    /// Whether the host renders the badge chrome.
    pub host_rendered_appearance_badge: bool,
    /// Surfaces the badge is rendered on (extension detail, embedded, etc.).
    pub rendered_on_surfaces: Vec<String>,
    /// Caveat summary.
    pub caveat_summary: String,
    /// Defect-kind tokens detected on this descriptor (empty when clean).
    pub defect_kind_tokens: Vec<String>,
    /// Deterministic generated-at timestamp.
    pub generated_at: String,
}

impl ExtensionAppearanceDescriptor {
    /// Returns the posture declared for `axis`, if present.
    pub fn posture(&self, axis: AppearanceAxisClass) -> Option<ExtensionInheritanceClass> {
        self.axes.iter().find(|p| p.axis == axis).map(|p| p.posture)
    }

    /// Returns whether this descriptor carries no defects.
    pub fn is_clean(&self) -> bool {
        self.defect_kind_tokens.is_empty()
    }
}

/// Metadata-safe support-export projection of a descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionAppearanceSupportRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Descriptor this row projects.
    pub descriptor_ref: String,
    /// Host pane/surface id.
    pub host_id: String,
    /// Extension/package id.
    pub package_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Review-lifecycle posture.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Per-axis posture tokens keyed by axis token.
    pub posture_by_axis: BTreeMap<String, String>,
    /// Badge token.
    pub badge_token: String,
    /// Parity-claim-state token.
    pub parity_claim_token: String,
    /// Whether the host renders the badge chrome.
    pub host_rendered_appearance_badge: bool,
    /// Defect-kind tokens detected on the descriptor.
    pub defect_kind_tokens: Vec<String>,
}

/// One blocking defect emitted by descriptor validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceDescriptorDefect {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Defect kind.
    pub defect_kind: AppearanceDescriptorDefectKind,
    /// Descriptor the defect applies to.
    pub descriptor_ref: String,
    /// Field or axis the defect targets.
    pub field: String,
    /// Human-readable message.
    pub message: String,
    /// Whether the defect is visible in-product (not support-only).
    pub visible_in_product: bool,
}

/// Aggregate summary recomputed over an audit's descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionAppearanceSummary {
    /// Number of descriptors.
    pub descriptor_count: usize,
    /// Descriptors badged full inheritance.
    pub full_inheritance_count: usize,
    /// Descriptors badged partial inheritance.
    pub partial_inheritance_count: usize,
    /// Descriptors badged does-not-inherit.
    pub does_not_inherit_count: usize,
    /// Descriptors badged undisclosed.
    pub undisclosed_count: usize,
    /// Descriptors granted a host-parity claim.
    pub host_parity_claim_count: usize,
    /// Descriptors making an honest partial parity claim with disclosed gaps.
    pub partial_parity_claim_count: usize,
    /// Descriptors whose parity claim was denied.
    pub denied_parity_claim_count: usize,
    /// Total defects across descriptors.
    pub defect_count: usize,
}

/// Top-level audit packet bundling descriptors, summary, and defects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionAppearanceAudit {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable audit id.
    pub audit_id: String,
    /// Deterministic generated-at timestamp.
    pub generated_at: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Frozen design-side audited-twin contract ref.
    pub design_contract_ref: String,
    /// Published doc ref.
    pub docs_ref: String,
    /// Published markdown report ref.
    pub report_ref: String,
    /// Surfaces every descriptor badge is rendered on.
    pub rendered_surface_tokens: Vec<String>,
    /// Recomputed summary.
    pub summary: ExtensionAppearanceSummary,
    /// All descriptors.
    pub descriptors: Vec<ExtensionAppearanceDescriptor>,
    /// All defects (empty when the audit is clean).
    pub defects: Vec<AppearanceDescriptorDefect>,
}

impl ExtensionAppearanceAudit {
    /// Returns the descriptor with `descriptor_id`, if present.
    pub fn descriptor(&self, descriptor_id: &str) -> Option<&ExtensionAppearanceDescriptor> {
        self.descriptors
            .iter()
            .find(|d| d.descriptor_id == descriptor_id)
    }

    /// Returns whether the audit carries no defects.
    pub fn is_clean(&self) -> bool {
        self.defects.is_empty()
    }

    /// Renders one-line summary plus per-descriptor badge lines.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.descriptors.len() + 1);
        lines.push(format!(
            "extension-appearance audit: {} descriptors, {} full, {} partial, {} private, {} undisclosed, {} denied, {} defects",
            self.summary.descriptor_count,
            self.summary.full_inheritance_count,
            self.summary.partial_inheritance_count,
            self.summary.does_not_inherit_count,
            self.summary.undisclosed_count,
            self.summary.denied_parity_claim_count,
            self.summary.defect_count,
        ));
        for descriptor in &self.descriptors {
            lines.push(format!(
                "{} [{}] {} -> {} ({})",
                descriptor.surface_kind.as_str(),
                descriptor.badge.badge_class.as_str(),
                descriptor.package_id,
                descriptor.parity_claim_state.as_str(),
                descriptor.descriptor_id,
            ));
        }
        lines
    }

    /// Renders a reviewer-facing markdown report of the audit.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Extension appearance-inheritance audit\n\n");
        out.push_str(
            "Every extension-backed or embedded surface declares whether it inherits Aureline's \
             theme, focus, contrast, density, and reduced-motion semantics. The badge below is \
             rendered in extension details, embedded panes, diagnostics, and support/export \
             packets.\n\n",
        );
        out.push_str("## Summary\n\n");
        out.push_str("| Metric | Count |\n| ------ | ----- |\n");
        out.push_str(&format!(
            "| Descriptors | {} |\n",
            self.summary.descriptor_count
        ));
        out.push_str(&format!(
            "| Inherits appearance | {} |\n",
            self.summary.full_inheritance_count
        ));
        out.push_str(&format!(
            "| Partly inherits | {} |\n",
            self.summary.partial_inheritance_count
        ));
        out.push_str(&format!(
            "| Does not inherit | {} |\n",
            self.summary.does_not_inherit_count
        ));
        out.push_str(&format!(
            "| Undisclosed | {} |\n",
            self.summary.undisclosed_count
        ));
        out.push_str(&format!(
            "| Host-parity claims granted | {} |\n",
            self.summary.host_parity_claim_count
        ));
        out.push_str(&format!(
            "| Partial parity claims | {} |\n",
            self.summary.partial_parity_claim_count
        ));
        out.push_str(&format!(
            "| Parity claims denied | {} |\n",
            self.summary.denied_parity_claim_count
        ));
        out.push_str(&format!("| Defects | {} |\n\n", self.summary.defect_count));

        out.push_str("## Descriptors\n\n");
        out.push_str("| Surface | Package | Badge | Parity claim |\n");
        out.push_str("| ------- | ------- | ----- | ------------ |\n");
        for descriptor in &self.descriptors {
            out.push_str(&format!(
                "| {} ({}) | {} | {} | {} |\n",
                descriptor.surface_label,
                descriptor.surface_kind.label(),
                descriptor.package_id,
                descriptor.badge.badge_label,
                descriptor.parity_claim_state.as_str(),
            ));
        }
        out.push('\n');

        out.push_str("## Per-axis posture\n\n");
        out.push_str("| Surface | Theme | Focus | High contrast | Density | Reduced motion |\n");
        out.push_str("| ------- | ----- | ----- | ------------- | ------- | -------------- |\n");
        for descriptor in &self.descriptors {
            let cell = |axis: AppearanceAxisClass| {
                descriptor
                    .posture(axis)
                    .map(ExtensionInheritanceClass::as_str)
                    .unwrap_or("-")
            };
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                descriptor.surface_label,
                cell(AppearanceAxisClass::Theme),
                cell(AppearanceAxisClass::Focus),
                cell(AppearanceAxisClass::Contrast),
                cell(AppearanceAxisClass::Density),
                cell(AppearanceAxisClass::ReducedMotion),
            ));
        }
        out.push('\n');

        out.push_str("## Findings\n\n");
        if self.defects.is_empty() {
            out.push_str(
                "No defects: no surface overclaims parity, no axis is undisclosed, and the host \
                 renders every appearance badge.\n",
            );
        } else {
            for defect in &self.defects {
                out.push_str(&format!(
                    "- **{}** on `{}` ({}): {}\n",
                    defect.defect_kind.as_str(),
                    defect.descriptor_ref,
                    defect.field,
                    defect.message,
                ));
            }
        }
        out.push('\n');

        out.push_str(&format!(
            "Regenerate: `cargo run -q -p aureline-extensions --example \
             dump_extension_appearance_descriptor_records -- markdown > {}`\n",
            EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_REPORT_REF,
        ));
        out
    }
}

/// Support-export wrapper carrying metadata-safe rows and pivot ids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionAppearanceSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Deterministic generated-at timestamp.
    pub generated_at: String,
    /// Audit this export wraps.
    pub audit_ref: String,
    /// Published doc ref.
    pub docs_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Recomputed summary.
    pub summary: ExtensionAppearanceSummary,
    /// Metadata-safe per-descriptor rows.
    pub support_rows: Vec<ExtensionAppearanceSupportRow>,
    /// Pivot ids: audit id, every descriptor id, package id, and host id.
    pub case_ids: Vec<String>,
    /// Defect counts keyed by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// Whether raw appearance material was excluded from the export.
    pub raw_appearance_material_excluded: bool,
}

// ---------------------------------------------------------------------------
// Derivation and evaluation
// ---------------------------------------------------------------------------

/// Derives the visible badge from the per-axis postures.
fn derive_badge_class(axes: &[AppearanceAxisPosture]) -> InheritanceBadgeClass {
    let mut any_undisclosed = false;
    let mut all_inherit = true;
    let mut any_inherit = false;
    let mut any_partial = false;
    for posture in axes {
        match posture.posture {
            ExtensionInheritanceClass::Inherits => any_inherit = true,
            ExtensionInheritanceClass::Partial => {
                any_partial = true;
                all_inherit = false;
            }
            ExtensionInheritanceClass::DoesNotInherit => all_inherit = false,
            ExtensionInheritanceClass::NotDisclosed => {
                any_undisclosed = true;
                all_inherit = false;
            }
        }
    }
    if any_undisclosed {
        InheritanceBadgeClass::Undisclosed
    } else if all_inherit {
        InheritanceBadgeClass::FullInheritance
    } else if any_inherit || any_partial {
        InheritanceBadgeClass::PartialInheritance
    } else {
        InheritanceBadgeClass::DoesNotInherit
    }
}

/// Builds the visible badge for a descriptor.
fn build_badge(badge_class: InheritanceBadgeClass, caveat_summary: &str) -> InheritanceBadge {
    let caveat_line = match badge_class {
        InheritanceBadgeClass::FullInheritance => {
            "Inherits host theme, focus, contrast, density, and reduced motion.".to_owned()
        }
        InheritanceBadgeClass::PartialInheritance => {
            format!("Partly inherits host appearance — {caveat_summary}")
        }
        InheritanceBadgeClass::DoesNotInherit => {
            format!("Keeps private appearance logic — {caveat_summary}")
        }
        InheritanceBadgeClass::Undisclosed => {
            "Appearance inheritance is undisclosed for at least one axis.".to_owned()
        }
    };
    InheritanceBadge {
        badge_class,
        badge_label: badge_class.label().to_owned(),
        caveat_line,
        implies_host_parity: badge_class.permits_host_parity(),
    }
}

/// Returns whether every non-inheriting axis discloses at least one known gap.
fn every_gap_disclosed(axes: &[AppearanceAxisPosture], gaps: &[AppearanceGap]) -> bool {
    axes.iter()
        .filter(|p| {
            matches!(
                p.posture,
                ExtensionInheritanceClass::Partial | ExtensionInheritanceClass::DoesNotInherit
            )
        })
        .all(|p| gaps.iter().any(|gap| gap.axis == p.axis))
}

/// Resolves the parity-claim state and its plain-language reason.
fn resolve_parity_state(
    badge_class: InheritanceBadgeClass,
    claims_first_party_parity: bool,
    has_accessibility_evidence: bool,
    all_gaps_disclosed: bool,
) -> (ParityClaimStateClass, String) {
    if !claims_first_party_parity {
        return (
            ParityClaimStateClass::NoParityClaim,
            "Surface makes no first-party appearance-parity claim.".to_owned(),
        );
    }
    if badge_class.permits_host_parity() && has_accessibility_evidence {
        return (
            ParityClaimStateClass::ClaimsHostParity,
            "Host parity granted: every axis inherits and accessibility evidence backs the claim."
                .to_owned(),
        );
    }
    if badge_class == InheritanceBadgeClass::PartialInheritance
        && has_accessibility_evidence
        && all_gaps_disclosed
    {
        return (
            ParityClaimStateClass::PartialClaimWithGaps,
            "Partial parity claim accepted: inherited axes are backed and every gap is disclosed."
                .to_owned(),
        );
    }
    (
        ParityClaimStateClass::DeniedClaim,
        "Parity claim denied: full inheritance, accessibility evidence, or gap disclosure is missing.".to_owned(),
    )
}

/// Joins a declared appearance input into a fully derived descriptor.
///
/// Computes the visible badge, resolves the parity-claim state (denying an
/// unbacked first-party claim), and records the defect-kind tokens detected on
/// the descriptor so consuming surfaces never have to recompute honesty.
pub fn evaluate_extension_appearance_descriptor(
    input: ExtensionAppearanceInput,
) -> ExtensionAppearanceDescriptor {
    let axes: Vec<AppearanceAxisPosture> = [
        (AppearanceAxisClass::Theme, input.inherit_theme),
        (AppearanceAxisClass::Focus, input.inherit_focus),
        (AppearanceAxisClass::Contrast, input.inherit_contrast),
        (AppearanceAxisClass::Density, input.inherit_density),
        (
            AppearanceAxisClass::ReducedMotion,
            input.inherit_reduced_motion,
        ),
    ]
    .into_iter()
    .map(|(axis, posture)| AppearanceAxisPosture {
        axis,
        posture,
        user_visible_disclosure_required: !matches!(posture, ExtensionInheritanceClass::Inherits),
    })
    .collect();

    let badge_class = derive_badge_class(&axes);
    let badge = build_badge(badge_class, &input.caveat_summary);
    let has_accessibility_evidence = !input.accessibility_evidence_refs.is_empty();
    let all_gaps_disclosed = every_gap_disclosed(&axes, &input.known_gaps);
    let (parity_claim_state, parity_reason) = resolve_parity_state(
        badge_class,
        input.claims_first_party_parity,
        has_accessibility_evidence,
        all_gaps_disclosed,
    );

    let mut descriptor = ExtensionAppearanceDescriptor {
        record_kind: EXTENSION_APPEARANCE_DESCRIPTOR_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF.to_owned(),
        descriptor_id: input.descriptor_id,
        host_id: input.host_id,
        package_id: input.package_id,
        package_name: input.package_name,
        publisher_label: input.publisher_label,
        surface_kind: input.surface_kind,
        surface_label: input.surface_label,
        lifecycle_class: input.lifecycle_class,
        axes,
        known_gaps: input.known_gaps,
        accessibility_evidence_refs: input.accessibility_evidence_refs,
        badge,
        claims_first_party_parity: input.claims_first_party_parity,
        parity_claim_state,
        parity_reason,
        host_rendered_appearance_badge: input.host_rendered_appearance_badge,
        rendered_on_surfaces: RENDERED_SURFACE_TOKENS
            .iter()
            .map(|token| (*token).to_owned())
            .collect(),
        caveat_summary: input.caveat_summary,
        defect_kind_tokens: Vec::new(),
        generated_at: input.generated_at,
    };

    descriptor.defect_kind_tokens = validate_extension_appearance_descriptor(&descriptor)
        .iter()
        .map(|defect| defect.defect_kind.as_str().to_owned())
        .collect();
    descriptor.defect_kind_tokens.dedup();
    descriptor
}

/// Validates a single descriptor and returns its blocking defects.
///
/// A clean descriptor returns an empty vector. The closed defect vocabulary
/// refuses an undisclosed axis, an overclaimed parity, a hidden inheritance gap,
/// and a host badge the extension tried to suppress.
pub fn validate_extension_appearance_descriptor(
    descriptor: &ExtensionAppearanceDescriptor,
) -> Vec<AppearanceDescriptorDefect> {
    let mut defects = Vec::new();
    let mut push = |kind: AppearanceDescriptorDefectKind, field: &str, message: &str| {
        defects.push(AppearanceDescriptorDefect {
            record_kind: EXTENSION_APPEARANCE_DESCRIPTOR_DEFECT_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION,
            shared_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!("{}:{}", descriptor.descriptor_id, kind.as_str()),
            defect_kind: kind,
            descriptor_ref: descriptor.descriptor_id.clone(),
            field: field.to_owned(),
            message: message.to_owned(),
            visible_in_product: true,
        });
    };

    // Undisclosed axis: posture cannot be inspected.
    for posture in &descriptor.axes {
        if posture.posture == ExtensionInheritanceClass::NotDisclosed {
            push(
                AppearanceDescriptorDefectKind::UndisclosedAxis,
                posture.axis.as_str(),
                "Appearance axis posture is undisclosed; the user cannot tell whether it inherits.",
            );
        }
    }

    // Overclaimed parity: a first-party claim the descriptor cannot back.
    if descriptor.parity_claim_state.is_denied() {
        push(
            AppearanceDescriptorDefectKind::OverclaimedParity,
            "claims_first_party_parity",
            "First-party appearance parity is claimed without full inheritance, accessibility evidence, or gap disclosure.",
        );
    }

    // Hidden inheritance gap: a full-inheritance badge that hides a gap.
    if descriptor.badge.badge_class == InheritanceBadgeClass::FullInheritance
        && !descriptor.known_gaps.is_empty()
    {
        push(
            AppearanceDescriptorDefectKind::HiddenInheritanceGap,
            "known_gaps",
            "Badge claims full inheritance while disclosing appearance gaps.",
        );
    }

    // Host badge chrome must never be suppressed by the extension.
    if !descriptor.host_rendered_appearance_badge {
        push(
            AppearanceDescriptorDefectKind::HostBadgeChromeHidden,
            "host_rendered_appearance_badge",
            "The host appearance badge is suppressed; the posture would be hidden from the user.",
        );
    }

    defects
}

/// Projects a descriptor into a metadata-safe support row.
pub fn project_extension_appearance_support_row(
    descriptor: &ExtensionAppearanceDescriptor,
) -> ExtensionAppearanceSupportRow {
    let posture_by_axis = descriptor
        .axes
        .iter()
        .map(|p| (p.axis.as_str().to_owned(), p.posture.as_str().to_owned()))
        .collect();
    ExtensionAppearanceSupportRow {
        record_kind: EXTENSION_APPEARANCE_SUPPORT_ROW_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF.to_owned(),
        descriptor_ref: descriptor.descriptor_id.clone(),
        host_id: descriptor.host_id.clone(),
        package_id: descriptor.package_id.clone(),
        surface_label: descriptor.surface_label.clone(),
        lifecycle_class: descriptor.lifecycle_class,
        posture_by_axis,
        badge_token: descriptor.badge.badge_class.as_str().to_owned(),
        parity_claim_token: descriptor.parity_claim_state.as_str().to_owned(),
        host_rendered_appearance_badge: descriptor.host_rendered_appearance_badge,
        defect_kind_tokens: descriptor.defect_kind_tokens.clone(),
    }
}

/// Recomputes the aggregate summary over a descriptor slice.
pub fn summarize_descriptors(
    descriptors: &[ExtensionAppearanceDescriptor],
) -> ExtensionAppearanceSummary {
    let mut summary = ExtensionAppearanceSummary {
        descriptor_count: descriptors.len(),
        full_inheritance_count: 0,
        partial_inheritance_count: 0,
        does_not_inherit_count: 0,
        undisclosed_count: 0,
        host_parity_claim_count: 0,
        partial_parity_claim_count: 0,
        denied_parity_claim_count: 0,
        defect_count: 0,
    };
    for descriptor in descriptors {
        match descriptor.badge.badge_class {
            InheritanceBadgeClass::FullInheritance => summary.full_inheritance_count += 1,
            InheritanceBadgeClass::PartialInheritance => summary.partial_inheritance_count += 1,
            InheritanceBadgeClass::DoesNotInherit => summary.does_not_inherit_count += 1,
            InheritanceBadgeClass::Undisclosed => summary.undisclosed_count += 1,
        }
        match descriptor.parity_claim_state {
            ParityClaimStateClass::ClaimsHostParity => summary.host_parity_claim_count += 1,
            ParityClaimStateClass::PartialClaimWithGaps => summary.partial_parity_claim_count += 1,
            ParityClaimStateClass::DeniedClaim => summary.denied_parity_claim_count += 1,
            ParityClaimStateClass::NoParityClaim => {}
        }
        summary.defect_count += descriptor.defect_kind_tokens.len();
    }
    summary
}

/// Builds an audit packet from evaluated descriptors.
///
/// Recomputes the summary, runs per-descriptor validation, and collects every
/// defect so a consuming surface reads one fail-closed truth packet.
pub fn build_extension_appearance_audit(
    descriptors: Vec<ExtensionAppearanceDescriptor>,
) -> ExtensionAppearanceAudit {
    let summary = summarize_descriptors(&descriptors);
    let mut defects = Vec::new();
    for descriptor in &descriptors {
        defects.extend(validate_extension_appearance_descriptor(descriptor));
    }
    ExtensionAppearanceAudit {
        record_kind: EXTENSION_APPEARANCE_AUDIT_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF.to_owned(),
        audit_id: EXTENSION_APPEARANCE_AUDIT_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        schema_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_REF.to_owned(),
        design_contract_ref: EXTENSION_APPEARANCE_DESIGN_CONTRACT_REF.to_owned(),
        docs_ref: EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_DOC_REF.to_owned(),
        report_ref: EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_REPORT_REF.to_owned(),
        rendered_surface_tokens: RENDERED_SURFACE_TOKENS
            .iter()
            .map(|token| (*token).to_owned())
            .collect(),
        summary,
        descriptors,
        defects,
    }
}

/// Projects an audit into its support-export wrapper.
pub fn project_extension_appearance_support_export(
    audit: &ExtensionAppearanceAudit,
    export_id: impl Into<String>,
) -> ExtensionAppearanceSupportExport {
    let support_rows: Vec<ExtensionAppearanceSupportRow> = audit
        .descriptors
        .iter()
        .map(project_extension_appearance_support_row)
        .collect();

    let mut case_ids = Vec::with_capacity(audit.descriptors.len() * 3 + 1);
    case_ids.push(audit.audit_id.clone());
    for descriptor in &audit.descriptors {
        case_ids.push(descriptor.descriptor_id.clone());
        if !case_ids.contains(&descriptor.package_id) {
            case_ids.push(descriptor.package_id.clone());
        }
        if !case_ids.contains(&descriptor.host_id) {
            case_ids.push(descriptor.host_id.clone());
        }
    }

    let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
    for defect in &audit.defects {
        *defect_counts_by_kind
            .entry(defect.defect_kind.as_str().to_owned())
            .or_insert(0) += 1;
    }

    ExtensionAppearanceSupportExport {
        record_kind: EXTENSION_APPEARANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF.to_owned(),
        export_id: export_id.into(),
        generated_at: audit.generated_at.clone(),
        audit_ref: audit.audit_id.clone(),
        docs_ref: EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_DOC_REF.to_owned(),
        schema_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_REF.to_owned(),
        summary: audit.summary.clone(),
        support_rows,
        case_ids,
        defect_counts_by_kind,
        raw_appearance_material_excluded: true,
    }
}

/// Validates an audit packet as a fail-closed gate.
///
/// Returns `Ok(())` only when the summary is internally consistent, every
/// descriptor is clean, and the badge is rendered on every required surface.
/// Otherwise returns every blocking defect.
pub fn validate_extension_appearance_audit(
    audit: &ExtensionAppearanceAudit,
) -> Result<(), Vec<AppearanceDescriptorDefect>> {
    let mut defects = Vec::new();

    let expected_summary = summarize_descriptors(&audit.descriptors);
    if audit.summary != expected_summary {
        defects.push(AppearanceDescriptorDefect {
            record_kind: EXTENSION_APPEARANCE_DESCRIPTOR_DEFECT_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION,
            shared_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!("{}:summary_stale", audit.audit_id),
            defect_kind: AppearanceDescriptorDefectKind::SupportExportParityDrift,
            descriptor_ref: audit.audit_id.clone(),
            field: "summary".to_owned(),
            message: "Audit summary does not match the descriptors.".to_owned(),
            visible_in_product: false,
        });
    }

    for descriptor in &audit.descriptors {
        // Every descriptor must render its badge on every required surface.
        for token in RENDERED_SURFACE_TOKENS {
            if !descriptor.rendered_on_surfaces.iter().any(|s| s == token) {
                defects.push(AppearanceDescriptorDefect {
                    record_kind: EXTENSION_APPEARANCE_DESCRIPTOR_DEFECT_RECORD_KIND.to_owned(),
                    schema_version: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION,
                    shared_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF
                        .to_owned(),
                    defect_id: format!("{}:badge_not_rendered:{token}", descriptor.descriptor_id),
                    defect_kind: AppearanceDescriptorDefectKind::HostBadgeChromeHidden,
                    descriptor_ref: descriptor.descriptor_id.clone(),
                    field: "rendered_on_surfaces".to_owned(),
                    message: format!("Inheritance badge is not rendered on the {token} surface."),
                    visible_in_product: true,
                });
            }
        }
        defects.extend(validate_extension_appearance_descriptor(descriptor));
    }

    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Builds an audit-scoped defect for a support-export or audit-level failure.
fn audit_scoped_defect(
    audit_id: &str,
    export_id: &str,
    kind: AppearanceDescriptorDefectKind,
    field: &str,
    message: &str,
) -> AppearanceDescriptorDefect {
    AppearanceDescriptorDefect {
        record_kind: EXTENSION_APPEARANCE_DESCRIPTOR_DEFECT_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_APPEARANCE_DESCRIPTOR_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF.to_owned(),
        defect_id: format!("{export_id}:{field}"),
        defect_kind: kind,
        descriptor_ref: audit_id.to_owned(),
        field: field.to_owned(),
        message: message.to_owned(),
        visible_in_product: false,
    }
}

/// Validates that a support export is in parity with its audit.
pub fn validate_extension_appearance_support_export(
    audit: &ExtensionAppearanceAudit,
    export: &ExtensionAppearanceSupportExport,
) -> Result<(), Vec<AppearanceDescriptorDefect>> {
    let mut defects = Vec::new();
    let drift = |field: &str, message: &str| {
        audit_scoped_defect(
            &audit.audit_id,
            &export.export_id,
            AppearanceDescriptorDefectKind::SupportExportParityDrift,
            field,
            message,
        )
    };

    if export.audit_ref != audit.audit_id {
        defects.push(drift(
            "audit_ref",
            "Support export does not quote the audit id.",
        ));
    }
    if export.summary != audit.summary {
        defects.push(drift(
            "summary",
            "Support export summary drifted from the audit.",
        ));
    }
    if !export.raw_appearance_material_excluded {
        defects.push(audit_scoped_defect(
            &audit.audit_id,
            &export.export_id,
            AppearanceDescriptorDefectKind::RawAppearanceMaterialExported,
            "raw_appearance_material_excluded",
            "Support export does not assert raw appearance material is excluded.",
        ));
    }

    let expected_rows: Vec<ExtensionAppearanceSupportRow> = audit
        .descriptors
        .iter()
        .map(project_extension_appearance_support_row)
        .collect();
    if export.support_rows != expected_rows {
        defects.push(drift(
            "support_rows",
            "Support export rows drifted from the descriptor projection.",
        ));
    }

    for descriptor in &audit.descriptors {
        if !export.case_ids.contains(&descriptor.descriptor_id) {
            defects.push(drift(
                "case_ids",
                "Support export must quote every descriptor id.",
            ));
            break;
        }
    }

    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

// ---------------------------------------------------------------------------
// Seeded corpus
// ---------------------------------------------------------------------------

/// Returns the deterministic seeded appearance inputs covering the honesty
/// spectrum: full inheritance with a granted host-parity claim, partial
/// inheritance, private styling, an honest partial parity claim with disclosed
/// gaps, and a partial diagnostics pane.
pub fn seeded_extension_appearance_inputs() -> Vec<ExtensionAppearanceInput> {
    vec![
        ExtensionAppearanceInput {
            descriptor_id: "extension-appearance:dev.aureline.samples/markdown-lens:preview-pane"
                .to_owned(),
            host_id: "host:editor-preview-dock".to_owned(),
            package_id: "dev.aureline.samples/markdown-lens".to_owned(),
            package_name: "Markdown Lens".to_owned(),
            publisher_label: "Aureline Samples".to_owned(),
            surface_kind: SurfaceKindClass::PreviewPane,
            surface_label: "Markdown preview".to_owned(),
            lifecycle_class: ReviewLifecycleClass::Stable,
            inherit_theme: ExtensionInheritanceClass::Inherits,
            inherit_focus: ExtensionInheritanceClass::Inherits,
            inherit_contrast: ExtensionInheritanceClass::Inherits,
            inherit_density: ExtensionInheritanceClass::Inherits,
            inherit_reduced_motion: ExtensionInheritanceClass::Inherits,
            known_gaps: Vec::new(),
            accessibility_evidence_refs: vec![
                "a11y_evidence:markdown-lens:contrast-audit".to_owned(),
                "a11y_evidence:markdown-lens:focus-traversal".to_owned(),
            ],
            claims_first_party_parity: true,
            host_rendered_appearance_badge: true,
            caveat_summary: "Renders host components and inherits every appearance token."
                .to_owned(),
            generated_at: GENERATED_AT.to_owned(),
        },
        ExtensionAppearanceInput {
            descriptor_id: "extension-appearance:com.acme.insights/analytics:dashboard".to_owned(),
            host_id: "host:embedded-panel-dock".to_owned(),
            package_id: "com.acme.insights/analytics".to_owned(),
            package_name: "Acme Insights".to_owned(),
            publisher_label: "Acme Analytics".to_owned(),
            surface_kind: SurfaceKindClass::EmbeddedWebview,
            surface_label: "Insights dashboard".to_owned(),
            lifecycle_class: ReviewLifecycleClass::Beta,
            inherit_theme: ExtensionInheritanceClass::Inherits,
            inherit_focus: ExtensionInheritanceClass::DoesNotInherit,
            inherit_contrast: ExtensionInheritanceClass::Partial,
            inherit_density: ExtensionInheritanceClass::Inherits,
            inherit_reduced_motion: ExtensionInheritanceClass::Partial,
            known_gaps: vec![
                AppearanceGap {
                    axis: AppearanceAxisClass::Focus,
                    summary: "Embedded charts draw a private focus ring.".to_owned(),
                },
                AppearanceGap {
                    axis: AppearanceAxisClass::Contrast,
                    summary: "Chart palettes only approximate forced-colors mode.".to_owned(),
                },
                AppearanceGap {
                    axis: AppearanceAxisClass::ReducedMotion,
                    summary: "Chart transitions shorten but do not fully stop.".to_owned(),
                },
            ],
            accessibility_evidence_refs: Vec::new(),
            claims_first_party_parity: false,
            host_rendered_appearance_badge: true,
            caveat_summary: "Charts keep a private focus ring and approximate high contrast."
                .to_owned(),
            generated_at: GENERATED_AT.to_owned(),
        },
        ExtensionAppearanceInput {
            descriptor_id: "extension-appearance:io.devtools.legacy/console:panel".to_owned(),
            host_id: "host:bottom-panel-dock".to_owned(),
            package_id: "io.devtools.legacy/console".to_owned(),
            package_name: "Legacy Console".to_owned(),
            publisher_label: "DevTools Legacy".to_owned(),
            surface_kind: SurfaceKindClass::ProviderPanel,
            surface_label: "Legacy console panel".to_owned(),
            lifecycle_class: ReviewLifecycleClass::Limited,
            inherit_theme: ExtensionInheritanceClass::DoesNotInherit,
            inherit_focus: ExtensionInheritanceClass::DoesNotInherit,
            inherit_contrast: ExtensionInheritanceClass::DoesNotInherit,
            inherit_density: ExtensionInheritanceClass::DoesNotInherit,
            inherit_reduced_motion: ExtensionInheritanceClass::DoesNotInherit,
            known_gaps: vec![
                AppearanceGap {
                    axis: AppearanceAxisClass::Theme,
                    summary: "Ships a fixed dark palette regardless of host theme.".to_owned(),
                },
                AppearanceGap {
                    axis: AppearanceAxisClass::Focus,
                    summary: "Draws its own focus outline.".to_owned(),
                },
                AppearanceGap {
                    axis: AppearanceAxisClass::Contrast,
                    summary: "No forced-colors support.".to_owned(),
                },
                AppearanceGap {
                    axis: AppearanceAxisClass::Density,
                    summary: "Fixed row height.".to_owned(),
                },
                AppearanceGap {
                    axis: AppearanceAxisClass::ReducedMotion,
                    summary: "Cursor blink ignores reduced motion.".to_owned(),
                },
            ],
            accessibility_evidence_refs: Vec::new(),
            claims_first_party_parity: false,
            host_rendered_appearance_badge: true,
            caveat_summary: "Keeps a fixed private palette and does not follow host appearance."
                .to_owned(),
            generated_at: GENERATED_AT.to_owned(),
        },
        ExtensionAppearanceInput {
            descriptor_id: "extension-appearance:dev.aureline.samples/api-docs:help-pane"
                .to_owned(),
            host_id: "host:help-dock".to_owned(),
            package_id: "dev.aureline.samples/api-docs".to_owned(),
            package_name: "API Docs".to_owned(),
            publisher_label: "Aureline Samples".to_owned(),
            surface_kind: SurfaceKindClass::DocsHelpPane,
            surface_label: "API reference".to_owned(),
            lifecycle_class: ReviewLifecycleClass::Stable,
            inherit_theme: ExtensionInheritanceClass::Inherits,
            inherit_focus: ExtensionInheritanceClass::Inherits,
            inherit_contrast: ExtensionInheritanceClass::Inherits,
            inherit_density: ExtensionInheritanceClass::Partial,
            inherit_reduced_motion: ExtensionInheritanceClass::Inherits,
            known_gaps: vec![AppearanceGap {
                axis: AppearanceAxisClass::Density,
                summary: "Code samples use fixed line spacing in compact density.".to_owned(),
            }],
            accessibility_evidence_refs: vec![
                "a11y_evidence:api-docs:contrast-audit".to_owned(),
                "a11y_evidence:api-docs:focus-traversal".to_owned(),
            ],
            claims_first_party_parity: true,
            host_rendered_appearance_badge: true,
            caveat_summary: "Inherits all axes except fixed line spacing in compact density."
                .to_owned(),
            generated_at: GENERATED_AT.to_owned(),
        },
        ExtensionAppearanceInput {
            descriptor_id: "extension-appearance:com.acme.insights/analytics:diagnostics"
                .to_owned(),
            host_id: "host:diagnostics-dock".to_owned(),
            package_id: "com.acme.insights/analytics".to_owned(),
            package_name: "Acme Insights".to_owned(),
            publisher_label: "Acme Analytics".to_owned(),
            surface_kind: SurfaceKindClass::DiagnosticsPane,
            surface_label: "Insights diagnostics".to_owned(),
            lifecycle_class: ReviewLifecycleClass::Beta,
            inherit_theme: ExtensionInheritanceClass::Inherits,
            inherit_focus: ExtensionInheritanceClass::Inherits,
            inherit_contrast: ExtensionInheritanceClass::Inherits,
            inherit_density: ExtensionInheritanceClass::Partial,
            inherit_reduced_motion: ExtensionInheritanceClass::Inherits,
            known_gaps: vec![AppearanceGap {
                axis: AppearanceAxisClass::Density,
                summary: "Diagnostics tables keep a fixed row height.".to_owned(),
            }],
            accessibility_evidence_refs: Vec::new(),
            claims_first_party_parity: false,
            host_rendered_appearance_badge: true,
            caveat_summary: "Diagnostics tables keep a fixed row height in compact density."
                .to_owned(),
            generated_at: GENERATED_AT.to_owned(),
        },
    ]
}

/// Returns the deterministic seeded audit minted from the seeded inputs.
///
/// The seeded corpus is clean (no defects) and covers the full, partial, and
/// does-not-inherit badge spectrum plus granted, honest-partial, and
/// no-claim parity states, so the checked-in fixtures are bit-for-bit equal to
/// this.
pub fn seeded_extension_appearance_audit() -> ExtensionAppearanceAudit {
    let descriptors = seeded_extension_appearance_inputs()
        .into_iter()
        .map(evaluate_extension_appearance_descriptor)
        .collect();
    build_extension_appearance_audit(descriptors)
}

/// Returns the deterministic seeded support export for the seeded audit.
pub fn seeded_extension_appearance_support_export() -> ExtensionAppearanceSupportExport {
    let audit = seeded_extension_appearance_audit();
    project_extension_appearance_support_export(
        &audit,
        "support-export:m5-extension-appearance:001",
    )
}
