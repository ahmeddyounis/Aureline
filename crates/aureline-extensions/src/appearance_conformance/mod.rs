//! Extension appearance-conformance and inheritance-gap disclosure.
//!
//! Extension-contributed UI can render rich custom surfaces, but a marketplace
//! row, install sheet, or mirrored-catalog entry must not *imply* that the
//! contributed UI inherits the host appearance contract unless the runtime can
//! prove it. Until now appearance parity lived in ad hoc per-extension prose or
//! the webview-boundary disclosure axes; this module makes appearance
//! inheritance a first-class compatibility dimension that marketplace detail,
//! install review, side-load review, mirrored/offline bundle review, and
//! post-install diagnostics all read from the same record.
//!
//! Each extension UI surface declares an inheritance posture for the host
//! appearance axes — theme, density, focus ring, high contrast, reduced motion,
//! and design-system (host) tokens — plus any known unsupported states. The
//! host runs a conformance probe per axis and the module joins declaration with
//! proof:
//!
//! - a declared `inherits` axis is only badged [`AppearanceSupportClass::FullInheritance`]
//!   when the host probe proves it ([`AppearanceProofClass::ProvenInherits`]);
//! - a claim the host cannot prove is downgraded to
//!   [`AppearanceSupportClass::ReducedSupport`] and routed to review rather than
//!   silently badged compatible;
//! - a claim the host *contradicts* (declared `inherits`, probed unsupported) is
//!   an [`AppearanceConformanceDefectKind::OverclaimedInheritance`] defect and
//!   refuses the appearance claim;
//! - an undisclosed axis is an
//!   [`AppearanceConformanceDefectKind::AxisDisclosureMissing`] defect.
//!
//! Host-stable trust, severity, permission, and policy labels are carried on
//! every row and validated to stay host-rendered, so extension-local styling can
//! never hide them — even when the extension chrome cannot inherit the
//! design-system contract.
//!
//! The cross-tool schema is
//! [`/schemas/extensions/appearance_support.schema.json`](../../../../schemas/extensions/appearance_support.schema.json),
//! the reviewer-facing guide is
//! [`/docs/extensions/m3/appearance_conformance_beta.md`](../../../../docs/extensions/m3/appearance_conformance_beta.md),
//! and the checked fixture corpus lives under
//! [`/fixtures/extensions/m3/appearance_inheritance/`](../../../../fixtures/extensions/m3/appearance_inheritance/).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::conformance_reports::ReviewLifecycleClass;
use crate::webview_boundary::ExtensionInheritanceClass;

#[cfg(test)]
mod tests;

/// Record-kind tag carried by [`AppearanceConformanceRow`].
pub const EXTENSION_APPEARANCE_CONFORMANCE_ROW_RECORD_KIND: &str =
    "extension_appearance_conformance_row";

/// Record-kind tag carried by [`AppearanceConformanceSupportRow`].
pub const EXTENSION_APPEARANCE_CONFORMANCE_SUPPORT_ROW_RECORD_KIND: &str =
    "extension_appearance_conformance_support_row";

/// Record-kind tag carried by [`AppearanceConformanceDefect`].
pub const EXTENSION_APPEARANCE_CONFORMANCE_DEFECT_RECORD_KIND: &str =
    "extension_appearance_conformance_defect";

/// Record-kind tag carried by [`AppearanceConformancePacket`].
pub const EXTENSION_APPEARANCE_CONFORMANCE_PACKET_RECORD_KIND: &str =
    "extension_appearance_conformance_packet";

/// Record-kind tag carried by [`AppearanceConformanceSupportExport`].
pub const EXTENSION_APPEARANCE_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_appearance_conformance_support_export";

/// Schema version for extension appearance-conformance payloads.
pub const EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref used by rows, support rows, packets, docs, and artifacts.
pub const EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF: &str =
    "extensions:appearance_conformance_beta:v1";

// ---------------------------------------------------------------------------
// Axis, proof, and support vocabulary
// ---------------------------------------------------------------------------

/// Host appearance axis an extension UI surface can inherit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceAxisClass {
    /// Color/theme class (light, dark, and theme-package tokens).
    Theme,
    /// Density scale (comfortable, compact) tokens.
    Density,
    /// Keyboard focus-ring tokens and visible-focus behavior.
    FocusRing,
    /// High-contrast / forced-colors tokens.
    HighContrast,
    /// Reduced-motion tokens and animation suppression.
    ReducedMotion,
    /// Design-system host tokens beyond the named appearance axes.
    HostToken,
}

impl AppearanceAxisClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::Density => "density",
            Self::FocusRing => "focus_ring",
            Self::HighContrast => "high_contrast",
            Self::ReducedMotion => "reduced_motion",
            Self::HostToken => "host_token",
        }
    }

    /// Returns the short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Theme => "Theme",
            Self::Density => "Density",
            Self::FocusRing => "Focus ring",
            Self::HighContrast => "High contrast",
            Self::ReducedMotion => "Reduced motion",
            Self::HostToken => "Host tokens",
        }
    }
}

/// Every required appearance axis in canonical order.
pub const APPEARANCE_AXES: [AppearanceAxisClass; 6] = [
    AppearanceAxisClass::Theme,
    AppearanceAxisClass::Density,
    AppearanceAxisClass::FocusRing,
    AppearanceAxisClass::HighContrast,
    AppearanceAxisClass::ReducedMotion,
    AppearanceAxisClass::HostToken,
];

/// Outcome of a host-side conformance probe for one appearance axis.
///
/// This is the *proof* side of the contract. A declaration alone never earns a
/// full-inheritance badge; only a [`Self::ProvenInherits`] probe does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceProofClass {
    /// The host proved the surface inherits this axis.
    ProvenInherits,
    /// The host proved the surface inherits this axis only partially.
    ProvenReduced,
    /// The host proved the surface uses private styling for this axis.
    ProvenUnsupported,
    /// The host could not prove inheritance for this axis (no/inconclusive probe).
    Unproven,
}

impl AppearanceProofClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenInherits => "proven_inherits",
            Self::ProvenReduced => "proven_reduced",
            Self::ProvenUnsupported => "proven_unsupported",
            Self::Unproven => "unproven",
        }
    }
}

/// Effective appearance support for one axis after joining declaration and proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceSupportClass {
    /// Declared and proven to inherit the host axis.
    FullInheritance,
    /// Inherits some of the axis; carries a visible reduced-support caveat.
    ReducedSupport,
    /// Renders private styling that does not inherit the host axis.
    UnsupportedPrivateStyling,
    /// The axis posture was not disclosed and cannot be evaluated.
    UndisclosedGap,
}

impl AppearanceSupportClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullInheritance => "full_inheritance",
            Self::ReducedSupport => "reduced_support",
            Self::UnsupportedPrivateStyling => "unsupported_private_styling",
            Self::UndisclosedGap => "undisclosed_gap",
        }
    }

    /// Returns the short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullInheritance => "Inherits host appearance",
            Self::ReducedSupport => "Reduced appearance support",
            Self::UnsupportedPrivateStyling => "Private styling",
            Self::UndisclosedGap => "Appearance support undisclosed",
        }
    }

    /// Returns the worst-first severity rank used to roll axes into a row total.
    const fn severity_rank(self) -> u8 {
        match self {
            Self::FullInheritance => 0,
            Self::ReducedSupport => 1,
            Self::UnsupportedPrivateStyling => 2,
            Self::UndisclosedGap => 3,
        }
    }

    /// Returns `true` when this support class carries a visible appearance caveat.
    pub const fn carries_caveat(self) -> bool {
        !matches!(self, Self::FullInheritance)
    }
}

/// Marketplace and review surfaces that read appearance conformance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceSurfaceClass {
    /// Compact marketplace or mirrored-catalog result row.
    MarketplaceResultRow,
    /// Marketplace package detail page.
    MarketplaceDetailPage,
    /// Product-owned install review sheet.
    InstallReview,
    /// Side-load review sheet for a local or manual artifact.
    SideloadReview,
    /// Mirrored or offline bundle review sheet.
    MirroredBundleReview,
    /// Post-install diagnostics / help surface.
    PostInstallDiagnostics,
}

impl AppearanceSurfaceClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarketplaceResultRow => "marketplace_result_row",
            Self::MarketplaceDetailPage => "marketplace_detail_page",
            Self::InstallReview => "install_review",
            Self::SideloadReview => "sideload_review",
            Self::MirroredBundleReview => "mirrored_bundle_review",
            Self::PostInstallDiagnostics => "post_install_diagnostics",
        }
    }

    /// Returns the per-surface phrasing prefix for a caveat line.
    const fn caveat_prefix(self) -> &'static str {
        match self {
            Self::MarketplaceResultRow => "",
            Self::MarketplaceDetailPage => "Before install: ",
            Self::InstallReview => "Install review: ",
            Self::SideloadReview => "Side-load review: ",
            Self::MirroredBundleReview => "Mirrored bundle: ",
            Self::PostInstallDiagnostics => "After install (diagnostics): ",
        }
    }

    /// Returns `true` when the surface keeps the caveat visible after enable.
    const fn persists_after_install(self) -> bool {
        matches!(self, Self::PostInstallDiagnostics)
    }
}

/// Every surface that must render an appearance caveat, in canonical order.
pub const APPEARANCE_SURFACES: [AppearanceSurfaceClass; 6] = [
    AppearanceSurfaceClass::MarketplaceResultRow,
    AppearanceSurfaceClass::MarketplaceDetailPage,
    AppearanceSurfaceClass::InstallReview,
    AppearanceSurfaceClass::SideloadReview,
    AppearanceSurfaceClass::MirroredBundleReview,
    AppearanceSurfaceClass::PostInstallDiagnostics,
];

/// Row-level appearance decision after validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceConformanceDecisionClass {
    /// Disclosure is honest and complete; the badge reflects proven support.
    Conformant,
    /// A claim cannot be proven or an axis is undisclosed; review before badging.
    NeedsReview,
    /// A claim is contradicted or host-stable labels are hidden; claim refused.
    Refused,
}

impl AppearanceConformanceDecisionClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::NeedsReview => "needs_review",
            Self::Refused => "refused",
        }
    }
}

/// Typed reason paired with [`AppearanceConformanceDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceConformanceReasonClass {
    /// Every axis is declared and proven to inherit the host appearance.
    FullInheritanceProven,
    /// Reduced support is disclosed across one or more axes and proven.
    ReducedSupportDisclosed,
    /// Private styling is disclosed across one or more axes.
    UnsupportedPrivateStylingDisclosed,
    /// An inheritance claim could not be proven across its declared modes.
    NeedsVerificationBeforeBadge,
    /// One or more axes were not disclosed.
    DisclosureIncomplete,
    /// An inheritance claim was contradicted by a host probe.
    OverclaimedInheritanceRefused,
    /// Host-stable trust/severity labels were not host-rendered.
    HostStableLabelHiddenRefused,
}

impl AppearanceConformanceReasonClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullInheritanceProven => "full_inheritance_proven",
            Self::ReducedSupportDisclosed => "reduced_support_disclosed",
            Self::UnsupportedPrivateStylingDisclosed => "unsupported_private_styling_disclosed",
            Self::NeedsVerificationBeforeBadge => "needs_verification_before_badge",
            Self::DisclosureIncomplete => "disclosure_incomplete",
            Self::OverclaimedInheritanceRefused => "overclaimed_inheritance_refused",
            Self::HostStableLabelHiddenRefused => "host_stable_label_hidden_refused",
        }
    }
}

/// Defect vocabulary emitted by appearance-conformance validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceConformanceDefectKind {
    /// One of the required appearance axes is missing from the declaration.
    AxisCoverageIncomplete,
    /// An axis posture was not disclosed.
    AxisDisclosureMissing,
    /// A declared inheritance claim was contradicted by a host probe.
    OverclaimedInheritance,
    /// A surface caveat implied full inheritance without proof.
    InheritanceImpliedWithoutProof,
    /// A known-unsupported state contradicts a fully-inherited axis.
    UnsupportedStateInconsistent,
    /// Host-stable trust/severity/permission/policy labels are not host-rendered.
    HostStableLabelHidden,
    /// A support-export row drifted from the product row.
    SupportExportParityDrift,
    /// A support export contains raw private styling material.
    RawPrivateMaterialExported,
}

impl AppearanceConformanceDefectKind {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AxisCoverageIncomplete => "axis_coverage_incomplete",
            Self::AxisDisclosureMissing => "axis_disclosure_missing",
            Self::OverclaimedInheritance => "overclaimed_inheritance",
            Self::InheritanceImpliedWithoutProof => "inheritance_implied_without_proof",
            Self::UnsupportedStateInconsistent => "unsupported_state_inconsistent",
            Self::HostStableLabelHidden => "host_stable_label_hidden",
            Self::SupportExportParityDrift => "support_export_parity_drift",
            Self::RawPrivateMaterialExported => "raw_private_material_exported",
        }
    }
}

// ---------------------------------------------------------------------------
// Declaration + probe inputs
// ---------------------------------------------------------------------------

/// One axis posture declared in the extension manifest appearance block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceAxisDeclaration {
    /// Axis the posture applies to.
    pub axis: AppearanceAxisClass,
    /// Declared inheritance posture for the axis.
    pub declared_class: ExtensionInheritanceClass,
    /// Optional author note rendered with the axis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// One known unsupported appearance state declared by the extension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceUnsupportedState {
    /// Axis the unsupported state applies to.
    pub axis: AppearanceAxisClass,
    /// Stable label for the unsupported state (e.g. `forced_colors_dark`).
    pub state_label: String,
    /// Metadata-safe explanation rendered with the state.
    pub summary: String,
}

/// Declared appearance support block extending the extension manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSupportDeclaration {
    /// Per-axis declared inheritance postures.
    pub declared_axes: Vec<AppearanceAxisDeclaration>,
    /// Known unsupported appearance states.
    #[serde(default)]
    pub known_unsupported_states: Vec<AppearanceUnsupportedState>,
    /// Metadata-safe declared-summary line.
    pub declared_summary: String,
}

impl AppearanceSupportDeclaration {
    fn declared_class(&self, axis: AppearanceAxisClass) -> Option<ExtensionInheritanceClass> {
        self.declared_axes
            .iter()
            .find(|entry| entry.axis == axis)
            .map(|entry| entry.declared_class)
    }
}

/// One host-side conformance probe for an appearance axis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceAxisProbe {
    /// Axis the probe covers.
    pub axis: AppearanceAxisClass,
    /// Proof outcome from the host conformance check.
    pub proof_class: AppearanceProofClass,
    /// Optional host conformance-check evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    /// Optional probe note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Input row supplied by extension host, SDK, or fixture code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceConformanceInput {
    /// Stable row id.
    pub row_id: String,
    /// Stable extension id.
    pub extension_id: String,
    /// Human-readable extension name.
    pub extension_name: String,
    /// Human-readable publisher label.
    pub publisher_label: String,
    /// Stable contributed-surface id.
    pub surface_id: String,
    /// Human-readable contributed-surface label.
    pub surface_label: String,
    /// Lifecycle class shared with marketplace and conformance vocabularies.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Declared appearance support block.
    pub declaration: AppearanceSupportDeclaration,
    /// Host-side conformance probes.
    #[serde(default)]
    pub probes: Vec<AppearanceAxisProbe>,
    /// Host-stable trust label that must remain host-rendered.
    pub host_trust_label: String,
    /// Host-stable severity label that must remain host-rendered.
    pub host_severity_label: String,
    /// Host-stable permission label that must remain host-rendered.
    pub host_permission_label: String,
    /// Host-stable policy label that must remain host-rendered.
    pub host_policy_label: String,
    /// True when host chrome (not extension styling) renders the labels above.
    pub host_rendered_trust_and_severity: bool,
    /// Registry, runtime, or support refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Metadata-safe row caveat summary.
    pub caveat_summary: String,
    /// Generation timestamp.
    pub generated_at: String,
}

// ---------------------------------------------------------------------------
// Evaluated records
// ---------------------------------------------------------------------------

/// Per-axis appearance conformance after joining declaration and proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceAxisConformance {
    /// Axis the row covers.
    pub axis: AppearanceAxisClass,
    /// Declared inheritance posture.
    pub declared_class: ExtensionInheritanceClass,
    /// Host probe outcome.
    pub proof_class: AppearanceProofClass,
    /// Effective support class after joining declaration and proof.
    pub support_class: AppearanceSupportClass,
    /// True when the host probe contradicts the declared inheritance claim.
    pub overclaimed: bool,
    /// True when a declared claim needs host proof before it can be badged.
    pub requires_verification: bool,
    /// Optional host conformance-check evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    /// Metadata-safe per-axis caveat.
    pub caveat: String,
}

/// Per-surface appearance caveat rendered on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSurfaceCaveat {
    /// Surface the caveat is rendered on.
    pub surface_class: AppearanceSurfaceClass,
    /// Short badge label for the surface.
    pub badge_label: String,
    /// True only when the row fully inherits and the badge may imply it.
    pub implies_full_inheritance: bool,
    /// True when the surface keeps the caveat visible after enable.
    pub persists_after_install: bool,
    /// Metadata-safe caveat line.
    pub caveat_line: String,
    /// Host-stable labels echoed to prove they persist.
    pub host_labels_line: String,
}

/// Audited appearance-conformance product row for one contributed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceConformanceRow {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for the row.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Stable extension id.
    pub extension_id: String,
    /// Human-readable extension name.
    pub extension_name: String,
    /// Human-readable publisher label.
    pub publisher_label: String,
    /// Stable contributed-surface id.
    pub surface_id: String,
    /// Human-readable contributed-surface label.
    pub surface_label: String,
    /// Lifecycle class shared with marketplace and conformance vocabularies.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Per-axis appearance conformance.
    pub axes: Vec<AppearanceAxisConformance>,
    /// Known unsupported appearance states copied from the declaration.
    pub known_unsupported_states: Vec<AppearanceUnsupportedState>,
    /// Per-surface caveats rendered across marketplace and review surfaces.
    pub surface_caveats: Vec<AppearanceSurfaceCaveat>,
    /// Worst-axis appearance support rolled into a row total.
    pub overall_support_class: AppearanceSupportClass,
    /// Row-level appearance decision.
    pub decision_class: AppearanceConformanceDecisionClass,
    /// Typed decision reason.
    pub reason_class: AppearanceConformanceReasonClass,
    /// Host-stable trust label.
    pub host_trust_label: String,
    /// Host-stable severity label.
    pub host_severity_label: String,
    /// Host-stable permission label.
    pub host_permission_label: String,
    /// Host-stable policy label.
    pub host_policy_label: String,
    /// True when host chrome renders the host-stable labels.
    pub host_rendered_trust_and_severity: bool,
    /// Registry, runtime, or support refs cited by the row.
    pub evidence_refs: Vec<String>,
    /// Metadata-safe row caveat summary.
    pub caveat_summary: String,
    /// Defect kind tokens found on this row before support-export parity checks.
    pub row_defect_kind_tokens: Vec<String>,
    /// Generation timestamp.
    pub generated_at: String,
}

/// Export-safe support row paired with an audited product row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceConformanceSupportRow {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for the support row.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable support row id.
    pub support_row_id: String,
    /// Product row this support row mirrors.
    pub row_ref: String,
    /// Stable extension id.
    pub extension_id: String,
    /// Human-readable extension name.
    pub extension_name: String,
    /// Human-readable publisher label.
    pub publisher_label: String,
    /// Human-readable contributed-surface label.
    pub surface_label: String,
    /// Lifecycle class shared with marketplace and conformance vocabularies.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Per-axis support classes keyed by axis token.
    pub axis_support_by_token: BTreeMap<String, String>,
    /// Worst-axis appearance support rolled into a row total.
    pub overall_support_class: AppearanceSupportClass,
    /// Row-level appearance decision.
    pub decision_class: AppearanceConformanceDecisionClass,
    /// Typed decision reason.
    pub reason_class: AppearanceConformanceReasonClass,
    /// Host-stable trust label.
    pub host_trust_label: String,
    /// Host-stable severity label.
    pub host_severity_label: String,
    /// True when host chrome renders the host-stable labels.
    pub host_rendered_trust_and_severity: bool,
    /// Defect kind tokens mirrored from the product row.
    pub row_defect_kind_tokens: Vec<String>,
    /// Metadata-safe row caveat summary.
    pub caveat_summary: String,
}

/// Typed defect emitted by appearance-conformance validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceConformanceDefect {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this defect.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Defect kind from the closed vocabulary.
    pub defect_kind: AppearanceConformanceDefectKind,
    /// Row that emitted the defect.
    pub row_ref: String,
    /// Field or group that failed validation.
    pub field: String,
    /// Export-safe validation message.
    pub message: String,
    /// True when the product row can show the same defect.
    pub visible_in_product: bool,
    /// True when the defect can be included in support export.
    pub support_export_safe: bool,
}

impl AppearanceConformanceDefect {
    fn new(
        row_ref: impl Into<String>,
        defect_kind: AppearanceConformanceDefectKind,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let row_ref = row_ref.into();
        Self {
            record_kind: EXTENSION_APPEARANCE_CONFORMANCE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION,
            shared_contract_ref: EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "extension-appearance-conformance:defect:{}:{}",
                defect_kind.as_str(),
                row_ref
            ),
            defect_kind,
            row_ref,
            field: field.into(),
            message: message.into(),
            visible_in_product: true,
            support_export_safe: true,
        }
    }
}

/// Aggregate summary for an appearance-conformance packet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AppearanceConformanceSummary {
    /// Number of audited product rows.
    pub row_count: usize,
    /// Number of support rows.
    pub support_row_count: usize,
    /// Number of rows whose decision is conformant.
    pub conformant_row_count: usize,
    /// Number of rows whose decision is needs-review.
    pub needs_review_row_count: usize,
    /// Number of rows whose decision is refused.
    pub refused_row_count: usize,
    /// Number of rows that fully inherit every host appearance axis.
    pub fully_inherited_row_count: usize,
    /// Number of rows whose worst axis is reduced support.
    pub reduced_support_row_count: usize,
    /// Number of rows whose worst axis is unsupported private styling.
    pub unsupported_row_count: usize,
    /// Number of rows whose worst axis is an undisclosed gap.
    pub undisclosed_row_count: usize,
    /// Number of rows with at least one overclaimed axis.
    pub overclaimed_row_count: usize,
    /// Number of emitted validation defects.
    pub defect_count: usize,
    /// Axes present in the packet.
    pub axes_present: Vec<AppearanceAxisClass>,
    /// Surfaces covered by the packet's caveats.
    pub surfaces_present: Vec<AppearanceSurfaceClass>,
}

impl AppearanceConformanceSummary {
    fn from_rows(
        rows: &[AppearanceConformanceRow],
        support_rows: &[AppearanceConformanceSupportRow],
        defects: &[AppearanceConformanceDefect],
    ) -> Self {
        let mut axes_present = Vec::new();
        let mut surfaces_present = Vec::new();
        for row in rows {
            for axis in &row.axes {
                push_unique(&mut axes_present, axis.axis);
            }
            for caveat in &row.surface_caveats {
                push_unique(&mut surfaces_present, caveat.surface_class);
            }
        }
        axes_present.sort();
        surfaces_present.sort();

        Self {
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            conformant_row_count: count_decision(
                rows,
                AppearanceConformanceDecisionClass::Conformant,
            ),
            needs_review_row_count: count_decision(
                rows,
                AppearanceConformanceDecisionClass::NeedsReview,
            ),
            refused_row_count: count_decision(rows, AppearanceConformanceDecisionClass::Refused),
            fully_inherited_row_count: count_overall(rows, AppearanceSupportClass::FullInheritance),
            reduced_support_row_count: count_overall(rows, AppearanceSupportClass::ReducedSupport),
            unsupported_row_count: count_overall(
                rows,
                AppearanceSupportClass::UnsupportedPrivateStyling,
            ),
            undisclosed_row_count: count_overall(rows, AppearanceSupportClass::UndisclosedGap),
            overclaimed_row_count: rows
                .iter()
                .filter(|row| row.axes.iter().any(|axis| axis.overclaimed))
                .count(),
            defect_count: defects.len(),
            axes_present,
            surfaces_present,
        }
    }
}

/// Top-level appearance-conformance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceConformancePacket {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this packet.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Reviewer-facing docs page.
    pub docs_ref: String,
    /// Cross-tool JSON schema ref.
    pub schema_ref: String,
    /// Human-readable generated report ref.
    pub report_ref: String,
    /// Aggregate summary.
    pub summary: AppearanceConformanceSummary,
    /// Audited product rows.
    pub rows: Vec<AppearanceConformanceRow>,
    /// Support rows paired with the product rows.
    pub support_rows: Vec<AppearanceConformanceSupportRow>,
    /// Validation defects emitted by the packet.
    pub defects: Vec<AppearanceConformanceDefect>,
}

impl AppearanceConformancePacket {
    /// Builds a packet from already evaluated rows.
    pub fn from_rows(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<AppearanceConformanceRow>,
    ) -> Self {
        let support_rows: Vec<AppearanceConformanceSupportRow> = rows
            .iter()
            .map(project_appearance_conformance_support_row)
            .collect();
        let defects = audit_appearance_conformance_rows(&rows, &support_rows);
        let summary = AppearanceConformanceSummary::from_rows(&rows, &support_rows, &defects);
        Self {
            record_kind: EXTENSION_APPEARANCE_CONFORMANCE_PACKET_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION,
            shared_contract_ref: EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF.to_owned(),
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            docs_ref: "docs/extensions/m3/appearance_conformance_beta.md".to_owned(),
            schema_ref: "schemas/extensions/appearance_support.schema.json".to_owned(),
            report_ref: "artifacts/extensions/m3/appearance_gap_review.md".to_owned(),
            summary,
            rows,
            support_rows,
            defects,
        }
    }
}

/// Metadata-safe export projected from an appearance-conformance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceConformanceSupportExport {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this export.
    pub schema_version: u32,
    /// Shared contract ref used by every row and export.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Packet this export mirrors.
    pub packet_ref: String,
    /// Reviewer-facing docs page.
    pub docs_ref: String,
    /// Human-readable generated report ref.
    pub report_ref: String,
    /// Aggregate summary mirrored from the packet.
    pub summary: AppearanceConformanceSummary,
    /// Export-safe support rows.
    pub support_rows: Vec<AppearanceConformanceSupportRow>,
    /// Defect counts keyed by closed defect token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private styling material is excluded.
    pub raw_private_material_excluded: bool,
}

// ---------------------------------------------------------------------------
// Evaluation
// ---------------------------------------------------------------------------

/// Joins one axis declaration and probe into an effective support outcome.
///
/// Returns the effective support class, whether the host probe contradicts the
/// declared claim (overclaim), and whether the declared claim still needs proof
/// before it can be badged.
fn join_axis(
    declared: ExtensionInheritanceClass,
    proof: AppearanceProofClass,
) -> (AppearanceSupportClass, bool, bool) {
    use AppearanceProofClass as P;
    use AppearanceSupportClass as S;
    use ExtensionInheritanceClass as D;

    match declared {
        D::NotDisclosed => (S::UndisclosedGap, false, false),
        D::DoesNotInherit => (S::UnsupportedPrivateStyling, false, false),
        D::Inherits => match proof {
            P::ProvenInherits => (S::FullInheritance, false, false),
            P::ProvenReduced => (S::ReducedSupport, false, true),
            P::ProvenUnsupported => (S::UnsupportedPrivateStyling, true, false),
            P::Unproven => (S::ReducedSupport, false, true),
        },
        D::Partial => match proof {
            P::ProvenInherits => (S::ReducedSupport, false, false),
            P::ProvenReduced => (S::ReducedSupport, false, false),
            P::ProvenUnsupported => (S::UnsupportedPrivateStyling, true, false),
            P::Unproven => (S::ReducedSupport, false, true),
        },
    }
}

/// Evaluates one input row into an audited appearance-conformance row.
pub fn evaluate_appearance_conformance_row(
    input: AppearanceConformanceInput,
) -> AppearanceConformanceRow {
    let probe_index: BTreeMap<AppearanceAxisClass, &AppearanceAxisProbe> = input
        .probes
        .iter()
        .map(|probe| (probe.axis, probe))
        .collect();

    let mut axes = Vec::with_capacity(APPEARANCE_AXES.len());
    for axis in APPEARANCE_AXES {
        let declared = input
            .declaration
            .declared_class(axis)
            .unwrap_or(ExtensionInheritanceClass::NotDisclosed);
        let probe = probe_index.get(&axis);
        let proof = probe
            .map(|probe| probe.proof_class)
            .unwrap_or(AppearanceProofClass::Unproven);
        let (support_class, overclaimed, requires_verification) = join_axis(declared, proof);
        axes.push(AppearanceAxisConformance {
            axis,
            declared_class: declared,
            proof_class: proof,
            support_class,
            overclaimed,
            requires_verification,
            evidence_ref: probe.and_then(|probe| probe.evidence_ref.clone()),
            caveat: axis_caveat(axis, support_class, overclaimed, requires_verification),
        });
    }

    let overall_support_class = overall_support(&axes);
    let (decision_class, reason_class) = decide_row(&input, &axes, overall_support_class);
    let surface_caveats = build_surface_caveats(&input, overall_support_class, decision_class);

    let mut row = AppearanceConformanceRow {
        record_kind: EXTENSION_APPEARANCE_CONFORMANCE_ROW_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF.to_owned(),
        row_id: input.row_id,
        extension_id: input.extension_id,
        extension_name: input.extension_name,
        publisher_label: input.publisher_label,
        surface_id: input.surface_id,
        surface_label: input.surface_label,
        lifecycle_class: input.lifecycle_class,
        axes,
        known_unsupported_states: input.declaration.known_unsupported_states,
        surface_caveats,
        overall_support_class,
        decision_class,
        reason_class,
        host_trust_label: input.host_trust_label,
        host_severity_label: input.host_severity_label,
        host_permission_label: input.host_permission_label,
        host_policy_label: input.host_policy_label,
        host_rendered_trust_and_severity: input.host_rendered_trust_and_severity,
        evidence_refs: input.evidence_refs,
        caveat_summary: input.caveat_summary,
        row_defect_kind_tokens: Vec::new(),
        generated_at: input.generated_at,
    };

    let row_defects = validate_appearance_conformance_row(&row);
    row.row_defect_kind_tokens = defect_kind_tokens(&row_defects);
    row
}

fn decide_row(
    input: &AppearanceConformanceInput,
    axes: &[AppearanceAxisConformance],
    overall: AppearanceSupportClass,
) -> (
    AppearanceConformanceDecisionClass,
    AppearanceConformanceReasonClass,
) {
    let host_labels_hidden = !input.host_rendered_trust_and_severity
        || input.host_trust_label.trim().is_empty()
        || input.host_severity_label.trim().is_empty()
        || input.host_permission_label.trim().is_empty()
        || input.host_policy_label.trim().is_empty();
    if host_labels_hidden {
        return (
            AppearanceConformanceDecisionClass::Refused,
            AppearanceConformanceReasonClass::HostStableLabelHiddenRefused,
        );
    }
    if axes.iter().any(|axis| axis.overclaimed) {
        return (
            AppearanceConformanceDecisionClass::Refused,
            AppearanceConformanceReasonClass::OverclaimedInheritanceRefused,
        );
    }
    let coverage_complete = APPEARANCE_AXES
        .iter()
        .all(|axis| input.declaration.declared_class(*axis).is_some());
    if !coverage_complete
        || axes
            .iter()
            .any(|axis| axis.support_class == AppearanceSupportClass::UndisclosedGap)
    {
        return (
            AppearanceConformanceDecisionClass::NeedsReview,
            AppearanceConformanceReasonClass::DisclosureIncomplete,
        );
    }
    if axes.iter().any(|axis| axis.requires_verification) {
        return (
            AppearanceConformanceDecisionClass::NeedsReview,
            AppearanceConformanceReasonClass::NeedsVerificationBeforeBadge,
        );
    }
    match overall {
        AppearanceSupportClass::FullInheritance => (
            AppearanceConformanceDecisionClass::Conformant,
            AppearanceConformanceReasonClass::FullInheritanceProven,
        ),
        AppearanceSupportClass::ReducedSupport => (
            AppearanceConformanceDecisionClass::Conformant,
            AppearanceConformanceReasonClass::ReducedSupportDisclosed,
        ),
        AppearanceSupportClass::UnsupportedPrivateStyling => (
            AppearanceConformanceDecisionClass::Conformant,
            AppearanceConformanceReasonClass::UnsupportedPrivateStylingDisclosed,
        ),
        // Unreachable in practice: an undisclosed worst axis is handled above.
        AppearanceSupportClass::UndisclosedGap => (
            AppearanceConformanceDecisionClass::NeedsReview,
            AppearanceConformanceReasonClass::DisclosureIncomplete,
        ),
    }
}

fn overall_support(axes: &[AppearanceAxisConformance]) -> AppearanceSupportClass {
    axes.iter()
        .map(|axis| axis.support_class)
        .max_by_key(|class| class.severity_rank())
        .unwrap_or(AppearanceSupportClass::UndisclosedGap)
}

fn build_surface_caveats(
    input: &AppearanceConformanceInput,
    overall: AppearanceSupportClass,
    decision: AppearanceConformanceDecisionClass,
) -> Vec<AppearanceSurfaceCaveat> {
    let badge = row_badge_label(overall, decision);
    let base = base_caveat(overall, decision);
    let host_labels_line = host_labels_line(input);
    let implies_full = overall == AppearanceSupportClass::FullInheritance
        && decision == AppearanceConformanceDecisionClass::Conformant;

    APPEARANCE_SURFACES
        .iter()
        .map(|surface| AppearanceSurfaceCaveat {
            surface_class: *surface,
            badge_label: badge.to_owned(),
            implies_full_inheritance: implies_full,
            persists_after_install: surface.persists_after_install(),
            caveat_line: format!("{}{}", surface.caveat_prefix(), base),
            host_labels_line: host_labels_line.clone(),
        })
        .collect()
}

fn row_badge_label(
    overall: AppearanceSupportClass,
    decision: AppearanceConformanceDecisionClass,
) -> &'static str {
    match decision {
        AppearanceConformanceDecisionClass::Refused => "Appearance claim refused",
        AppearanceConformanceDecisionClass::NeedsReview => match overall {
            AppearanceSupportClass::UndisclosedGap => "Appearance support undisclosed",
            _ => "Appearance support unverified",
        },
        AppearanceConformanceDecisionClass::Conformant => match overall {
            AppearanceSupportClass::FullInheritance => "Inherits host appearance",
            AppearanceSupportClass::ReducedSupport => "Reduced appearance support",
            AppearanceSupportClass::UnsupportedPrivateStyling => {
                "Private styling (no host inheritance)"
            }
            AppearanceSupportClass::UndisclosedGap => "Appearance support undisclosed",
        },
    }
}

fn base_caveat(
    overall: AppearanceSupportClass,
    decision: AppearanceConformanceDecisionClass,
) -> &'static str {
    match decision {
        AppearanceConformanceDecisionClass::Refused => {
            "this extension UI made an appearance claim the host could not honor; the claim is refused."
        }
        AppearanceConformanceDecisionClass::NeedsReview => match overall {
            AppearanceSupportClass::UndisclosedGap => {
                "this extension UI has not disclosed how it inherits host appearance; treat as unverified."
            }
            _ => {
                "this extension UI claims host appearance inheritance the host has not yet proven across its declared modes."
            }
        },
        AppearanceConformanceDecisionClass::Conformant => match overall {
            AppearanceSupportClass::FullInheritance => {
                "this extension UI inherits host theme, density, focus ring, high contrast, reduced motion, and design-system tokens."
            }
            AppearanceSupportClass::ReducedSupport => {
                "this extension UI inherits some host appearance axes and declares reduced support for others; see per-axis rows."
            }
            AppearanceSupportClass::UnsupportedPrivateStyling => {
                "this extension UI renders private styling and does not inherit host appearance tokens."
            }
            AppearanceSupportClass::UndisclosedGap => {
                "this extension UI has not disclosed how it inherits host appearance; treat as unverified."
            }
        },
    }
}

fn host_labels_line(input: &AppearanceConformanceInput) -> String {
    format!(
        "Host-stable labels remain host-rendered: trust={}, severity={}, permission={}, policy={}.",
        input.host_trust_label,
        input.host_severity_label,
        input.host_permission_label,
        input.host_policy_label,
    )
}

fn axis_caveat(
    axis: AppearanceAxisClass,
    support: AppearanceSupportClass,
    overclaimed: bool,
    requires_verification: bool,
) -> String {
    if overclaimed {
        return format!(
            "{} claimed inheritance but the host probe proved private styling.",
            axis.label()
        );
    }
    if requires_verification {
        return format!(
            "{} inheritance is claimed but not yet proven by a host probe.",
            axis.label()
        );
    }
    match support {
        AppearanceSupportClass::FullInheritance => {
            format!("{} inherits host tokens.", axis.label())
        }
        AppearanceSupportClass::ReducedSupport => {
            format!("{} inherits host tokens partially.", axis.label())
        }
        AppearanceSupportClass::UnsupportedPrivateStyling => {
            format!("{} uses private styling, not host tokens.", axis.label())
        }
        AppearanceSupportClass::UndisclosedGap => {
            format!("{} inheritance was not disclosed.", axis.label())
        }
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Projects one product row into a metadata-safe support row.
pub fn project_appearance_conformance_support_row(
    row: &AppearanceConformanceRow,
) -> AppearanceConformanceSupportRow {
    let axis_support_by_token = row
        .axes
        .iter()
        .map(|axis| {
            (
                axis.axis.as_str().to_owned(),
                axis.support_class.as_str().to_owned(),
            )
        })
        .collect();
    AppearanceConformanceSupportRow {
        record_kind: EXTENSION_APPEARANCE_CONFORMANCE_SUPPORT_ROW_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF.to_owned(),
        support_row_id: format!(
            "extension-appearance-conformance:support-row:{}",
            row.row_id
        ),
        row_ref: row.row_id.clone(),
        extension_id: row.extension_id.clone(),
        extension_name: row.extension_name.clone(),
        publisher_label: row.publisher_label.clone(),
        surface_label: row.surface_label.clone(),
        lifecycle_class: row.lifecycle_class,
        axis_support_by_token,
        overall_support_class: row.overall_support_class,
        decision_class: row.decision_class,
        reason_class: row.reason_class,
        host_trust_label: row.host_trust_label.clone(),
        host_severity_label: row.host_severity_label.clone(),
        host_rendered_trust_and_severity: row.host_rendered_trust_and_severity,
        row_defect_kind_tokens: row.row_defect_kind_tokens.clone(),
        caveat_summary: row.caveat_summary.clone(),
    }
}

/// Projects a metadata-safe support export from a packet.
pub fn project_appearance_conformance_support_export(
    packet: &AppearanceConformancePacket,
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> AppearanceConformanceSupportExport {
    let mut defect_counts_by_kind = BTreeMap::new();
    for defect in &packet.defects {
        *defect_counts_by_kind
            .entry(defect.defect_kind.as_str().to_owned())
            .or_insert(0) += 1;
    }

    AppearanceConformanceSupportExport {
        record_kind: EXTENSION_APPEARANCE_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF.to_owned(),
        export_id: export_id.into(),
        generated_at: generated_at.into(),
        packet_ref: packet.packet_id.clone(),
        docs_ref: packet.docs_ref.clone(),
        report_ref: packet.report_ref.clone(),
        summary: packet.summary.clone(),
        support_rows: packet.support_rows.clone(),
        defect_counts_by_kind,
        raw_private_material_excluded: true,
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Validates one product row without considering support-export parity.
pub fn validate_appearance_conformance_row(
    row: &AppearanceConformanceRow,
) -> Vec<AppearanceConformanceDefect> {
    let mut defects = Vec::new();

    for axis in APPEARANCE_AXES {
        if !row.axes.iter().any(|entry| entry.axis == axis) {
            defects.push(AppearanceConformanceDefect::new(
                &row.row_id,
                AppearanceConformanceDefectKind::AxisCoverageIncomplete,
                "axes",
                format!(
                    "appearance axis `{}` is missing from the row",
                    axis.as_str()
                ),
            ));
        }
    }

    for axis in &row.axes {
        if axis.declared_class == ExtensionInheritanceClass::NotDisclosed
            || axis.support_class == AppearanceSupportClass::UndisclosedGap
        {
            defects.push(AppearanceConformanceDefect::new(
                &row.row_id,
                AppearanceConformanceDefectKind::AxisDisclosureMissing,
                axis.axis.as_str(),
                format!(
                    "appearance axis `{}` posture must be disclosed",
                    axis.axis.as_str()
                ),
            ));
        }
        if axis.overclaimed {
            defects.push(AppearanceConformanceDefect::new(
                &row.row_id,
                AppearanceConformanceDefectKind::OverclaimedInheritance,
                axis.axis.as_str(),
                format!(
                    "appearance axis `{}` claimed inheritance but the host probe proved otherwise",
                    axis.axis.as_str()
                ),
            ));
        }
    }

    for state in &row.known_unsupported_states {
        if let Some(axis) = row.axes.iter().find(|entry| entry.axis == state.axis) {
            if axis.support_class == AppearanceSupportClass::FullInheritance {
                defects.push(AppearanceConformanceDefect::new(
                    &row.row_id,
                    AppearanceConformanceDefectKind::UnsupportedStateInconsistent,
                    state.axis.as_str(),
                    format!(
                        "axis `{}` is fully inherited but lists unsupported state `{}`",
                        state.axis.as_str(),
                        state.state_label
                    ),
                ));
            }
        }
    }

    if !row.host_rendered_trust_and_severity
        || row.host_trust_label.trim().is_empty()
        || row.host_severity_label.trim().is_empty()
        || row.host_permission_label.trim().is_empty()
        || row.host_policy_label.trim().is_empty()
    {
        defects.push(AppearanceConformanceDefect::new(
            &row.row_id,
            AppearanceConformanceDefectKind::HostStableLabelHidden,
            "host_rendered_trust_and_severity",
            "host-stable trust, severity, permission, and policy labels must stay host-rendered",
        ));
    }

    for surface in APPEARANCE_SURFACES {
        let Some(caveat) = row
            .surface_caveats
            .iter()
            .find(|caveat| caveat.surface_class == surface)
        else {
            defects.push(AppearanceConformanceDefect::new(
                &row.row_id,
                AppearanceConformanceDefectKind::InheritanceImpliedWithoutProof,
                surface.as_str(),
                format!(
                    "surface `{}` is missing its appearance caveat",
                    surface.as_str()
                ),
            ));
            continue;
        };
        let may_imply_full = row.overall_support_class == AppearanceSupportClass::FullInheritance
            && row.decision_class == AppearanceConformanceDecisionClass::Conformant;
        if caveat.implies_full_inheritance && !may_imply_full {
            defects.push(AppearanceConformanceDefect::new(
                &row.row_id,
                AppearanceConformanceDefectKind::InheritanceImpliedWithoutProof,
                surface.as_str(),
                format!(
                    "surface `{}` must not imply full inheritance without proven parity",
                    surface.as_str()
                ),
            ));
        }
    }

    defects
}

/// Audits product rows and support rows together.
pub fn audit_appearance_conformance_rows(
    rows: &[AppearanceConformanceRow],
    support_rows: &[AppearanceConformanceSupportRow],
) -> Vec<AppearanceConformanceDefect> {
    let mut defects = Vec::new();
    let support_index: BTreeMap<&str, &AppearanceConformanceSupportRow> = support_rows
        .iter()
        .map(|row| (row.row_ref.as_str(), row))
        .collect();

    for row in rows {
        defects.extend(validate_appearance_conformance_row(row));
        match support_index.get(row.row_id.as_str()) {
            Some(support) if support_row_matches_product_row(row, support) => {}
            Some(_) => defects.push(AppearanceConformanceDefect::new(
                &row.row_id,
                AppearanceConformanceDefectKind::SupportExportParityDrift,
                "support_row",
                "support export row drifted from the product row",
            )),
            None => defects.push(AppearanceConformanceDefect::new(
                &row.row_id,
                AppearanceConformanceDefectKind::SupportExportParityDrift,
                "support_row",
                "support export row is missing",
            )),
        }
    }

    defects
}

/// Validates that a packet has current constants and no defects.
pub fn validate_appearance_conformance_packet(
    packet: &AppearanceConformancePacket,
) -> Result<(), Vec<AppearanceConformanceDefect>> {
    let mut defects = Vec::new();
    if packet.record_kind != EXTENSION_APPEARANCE_CONFORMANCE_PACKET_RECORD_KIND {
        defects.push(packet_defect(
            AppearanceConformanceDefectKind::SupportExportParityDrift,
            "record_kind",
            "packet record kind is not current",
        ));
    }
    if packet.schema_version != EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION {
        defects.push(packet_defect(
            AppearanceConformanceDefectKind::SupportExportParityDrift,
            "schema_version",
            "packet schema version is not current",
        ));
    }
    if packet.shared_contract_ref != EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF {
        defects.push(packet_defect(
            AppearanceConformanceDefectKind::SupportExportParityDrift,
            "shared_contract_ref",
            "packet shared contract ref is not current",
        ));
    }
    defects.extend(audit_appearance_conformance_rows(
        &packet.rows,
        &packet.support_rows,
    ));
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Validates that a support export mirrors the packet and excludes private material.
pub fn validate_appearance_conformance_support_export(
    packet: &AppearanceConformancePacket,
    export: &AppearanceConformanceSupportExport,
) -> Result<(), Vec<AppearanceConformanceDefect>> {
    let mut defects = Vec::new();
    if export.record_kind != EXTENSION_APPEARANCE_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND {
        defects.push(packet_defect(
            AppearanceConformanceDefectKind::SupportExportParityDrift,
            "record_kind",
            "support export record kind is not current",
        ));
    }
    if export.schema_version != EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION {
        defects.push(packet_defect(
            AppearanceConformanceDefectKind::SupportExportParityDrift,
            "schema_version",
            "support export schema version is not current",
        ));
    }
    if export.shared_contract_ref != EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF {
        defects.push(packet_defect(
            AppearanceConformanceDefectKind::SupportExportParityDrift,
            "shared_contract_ref",
            "support export shared contract ref is not current",
        ));
    }
    if export.packet_ref != packet.packet_id
        || export.summary != packet.summary
        || export.support_rows != packet.support_rows
    {
        defects.push(packet_defect(
            AppearanceConformanceDefectKind::SupportExportParityDrift,
            "support_export",
            "support export no longer mirrors the packet",
        ));
    }
    if !export.raw_private_material_excluded {
        defects.push(packet_defect(
            AppearanceConformanceDefectKind::RawPrivateMaterialExported,
            "raw_private_material_excluded",
            "support export must exclude raw private styling material",
        ));
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

/// Builds the seeded appearance-conformance packet.
pub fn seeded_appearance_conformance_packet() -> AppearanceConformancePacket {
    let rows = seeded_appearance_conformance_inputs()
        .into_iter()
        .map(evaluate_appearance_conformance_row)
        .collect();
    AppearanceConformancePacket::from_rows(
        "extension-appearance-conformance:packet:beta:default",
        "2026-05-20T00:00:00Z",
        rows,
    )
}

/// Returns the fixture inputs used by the seeded packet.
pub fn seeded_appearance_conformance_inputs() -> Vec<AppearanceConformanceInput> {
    vec![
        full_inheritance_input(),
        reduced_support_input(),
        unverified_claim_input(),
        private_styling_input(),
    ]
}

fn full_inheritance_input() -> AppearanceConformanceInput {
    AppearanceConformanceInput {
        row_id: "extension-appearance-conformance:dev.aureline.samples/markdown-lens:preview-pane"
            .to_owned(),
        extension_id: "dev.aureline.samples/markdown-lens".to_owned(),
        extension_name: "Markdown Lens".to_owned(),
        publisher_label: "Aureline Samples".to_owned(),
        surface_id: "surface:markdown-lens:preview-pane".to_owned(),
        surface_label: "Markdown preview pane".to_owned(),
        lifecycle_class: ReviewLifecycleClass::Stable,
        declaration: AppearanceSupportDeclaration {
            declared_axes: all_axes_declared(ExtensionInheritanceClass::Inherits),
            known_unsupported_states: Vec::new(),
            declared_summary: "Renders host components and inherits every host appearance token."
                .to_owned(),
        },
        probes: all_axes_probed(AppearanceProofClass::ProvenInherits),
        host_trust_label: "Trusted publisher".to_owned(),
        host_severity_label: "No active warnings".to_owned(),
        host_permission_label: "Read-only workspace docs".to_owned(),
        host_policy_label: "Allowed by org policy".to_owned(),
        host_rendered_trust_and_severity: true,
        evidence_refs: vec![
            "conformance_probe:dev.aureline.samples/markdown-lens:appearance".to_owned(),
            "registry_descriptor:dev.aureline.samples/markdown-lens:2.1.0".to_owned(),
        ],
        caveat_summary:
            "Inherits host appearance across theme, density, focus, contrast, motion, and tokens."
                .to_owned(),
        generated_at: "2026-05-20T00:00:00Z".to_owned(),
    }
}

fn reduced_support_input() -> AppearanceConformanceInput {
    AppearanceConformanceInput {
        row_id: "extension-appearance-conformance:com.acme.dashboards:insights-panel".to_owned(),
        extension_id: "com.acme.dashboards".to_owned(),
        extension_name: "Acme Insights".to_owned(),
        publisher_label: "Acme Cloud, Inc.".to_owned(),
        surface_id: "surface:acme-dashboards:insights-panel".to_owned(),
        surface_label: "Insights dashboard panel".to_owned(),
        lifecycle_class: ReviewLifecycleClass::Beta,
        declaration: AppearanceSupportDeclaration {
            declared_axes: vec![
                axis_decl(AppearanceAxisClass::Theme, ExtensionInheritanceClass::Inherits),
                axis_decl(
                    AppearanceAxisClass::Density,
                    ExtensionInheritanceClass::Partial,
                ),
                axis_decl(
                    AppearanceAxisClass::FocusRing,
                    ExtensionInheritanceClass::Inherits,
                ),
                axis_decl(
                    AppearanceAxisClass::HighContrast,
                    ExtensionInheritanceClass::Partial,
                ),
                axis_decl(
                    AppearanceAxisClass::ReducedMotion,
                    ExtensionInheritanceClass::Inherits,
                ),
                axis_decl(
                    AppearanceAxisClass::HostToken,
                    ExtensionInheritanceClass::Inherits,
                ),
            ],
            known_unsupported_states: vec![
                AppearanceUnsupportedState {
                    axis: AppearanceAxisClass::Density,
                    state_label: "compact_density".to_owned(),
                    summary: "Compact density keeps a fixed chart row height.".to_owned(),
                },
                AppearanceUnsupportedState {
                    axis: AppearanceAxisClass::HighContrast,
                    state_label: "forced_colors_dark".to_owned(),
                    summary: "Chart series colors are fixed under forced-colors dark."
                        .to_owned(),
                },
            ],
            declared_summary:
                "Inherits theme, focus, motion, and tokens; reduced density and high-contrast support in charts."
                    .to_owned(),
        },
        probes: vec![
            axis_probe(
                AppearanceAxisClass::Theme,
                AppearanceProofClass::ProvenInherits,
            ),
            axis_probe(
                AppearanceAxisClass::Density,
                AppearanceProofClass::ProvenReduced,
            ),
            axis_probe(
                AppearanceAxisClass::FocusRing,
                AppearanceProofClass::ProvenInherits,
            ),
            axis_probe(
                AppearanceAxisClass::HighContrast,
                AppearanceProofClass::ProvenReduced,
            ),
            axis_probe(
                AppearanceAxisClass::ReducedMotion,
                AppearanceProofClass::ProvenInherits,
            ),
            axis_probe(
                AppearanceAxisClass::HostToken,
                AppearanceProofClass::ProvenInherits,
            ),
        ],
        host_trust_label: "Verified publisher".to_owned(),
        host_severity_label: "No active warnings".to_owned(),
        host_permission_label: "Workspace read + network status".to_owned(),
        host_policy_label: "Allowed by org policy".to_owned(),
        host_rendered_trust_and_severity: true,
        evidence_refs: vec![
            "conformance_probe:com.acme.dashboards:appearance".to_owned(),
            "registry_descriptor:com.acme.dashboards:4.3.0-beta.2".to_owned(),
        ],
        caveat_summary:
            "Inherits most host appearance; declares reduced density and high-contrast support in charts."
                .to_owned(),
        generated_at: "2026-05-20T00:00:00Z".to_owned(),
    }
}

fn unverified_claim_input() -> AppearanceConformanceInput {
    AppearanceConformanceInput {
        row_id: "extension-appearance-conformance:io.contrib.theme-extras:settings-surface"
            .to_owned(),
        extension_id: "io.contrib.theme-extras".to_owned(),
        extension_name: "Theme Extras".to_owned(),
        publisher_label: "Community Contributor".to_owned(),
        surface_id: "surface:theme-extras:settings-surface".to_owned(),
        surface_label: "Theme settings surface".to_owned(),
        lifecycle_class: ReviewLifecycleClass::Beta,
        declaration: AppearanceSupportDeclaration {
            declared_axes: all_axes_declared(ExtensionInheritanceClass::Inherits),
            known_unsupported_states: Vec::new(),
            declared_summary: "Claims full host appearance inheritance across all axes.".to_owned(),
        },
        probes: vec![
            axis_probe(
                AppearanceAxisClass::Theme,
                AppearanceProofClass::ProvenInherits,
            ),
            axis_probe(
                AppearanceAxisClass::Density,
                AppearanceProofClass::ProvenInherits,
            ),
            axis_probe(
                AppearanceAxisClass::FocusRing,
                AppearanceProofClass::ProvenInherits,
            ),
            // High-contrast parity has not been probed yet on this beta surface.
            axis_probe(
                AppearanceAxisClass::HighContrast,
                AppearanceProofClass::Unproven,
            ),
            axis_probe(
                AppearanceAxisClass::ReducedMotion,
                AppearanceProofClass::ProvenInherits,
            ),
            axis_probe(
                AppearanceAxisClass::HostToken,
                AppearanceProofClass::ProvenInherits,
            ),
        ],
        host_trust_label: "Community publisher".to_owned(),
        host_severity_label: "Beta surface".to_owned(),
        host_permission_label: "Settings read + write".to_owned(),
        host_policy_label: "Allowed by org policy".to_owned(),
        host_rendered_trust_and_severity: true,
        evidence_refs: vec![
            "conformance_probe:io.contrib.theme-extras:appearance".to_owned(),
            "registry_descriptor:io.contrib.theme-extras:0.9.0-beta.1".to_owned(),
        ],
        caveat_summary:
            "Claims full inheritance; high-contrast parity not yet proven, so the badge stays unverified."
                .to_owned(),
        generated_at: "2026-05-20T00:00:00Z".to_owned(),
    }
}

fn private_styling_input() -> AppearanceConformanceInput {
    AppearanceConformanceInput {
        row_id: "extension-appearance-conformance:net.legacy.toolbar:custom-toolbar".to_owned(),
        extension_id: "net.legacy.toolbar".to_owned(),
        extension_name: "Legacy Toolbar".to_owned(),
        publisher_label: "Legacy Tools (mirrored)".to_owned(),
        surface_id: "surface:legacy-toolbar:custom-toolbar".to_owned(),
        surface_label: "Custom toolbar surface".to_owned(),
        lifecycle_class: ReviewLifecycleClass::Limited,
        declaration: AppearanceSupportDeclaration {
            declared_axes: vec![
                axis_decl(AppearanceAxisClass::Theme, ExtensionInheritanceClass::Inherits),
                axis_decl(
                    AppearanceAxisClass::Density,
                    ExtensionInheritanceClass::DoesNotInherit,
                ),
                axis_decl(
                    AppearanceAxisClass::FocusRing,
                    ExtensionInheritanceClass::DoesNotInherit,
                ),
                axis_decl(
                    AppearanceAxisClass::HighContrast,
                    ExtensionInheritanceClass::DoesNotInherit,
                ),
                axis_decl(
                    AppearanceAxisClass::ReducedMotion,
                    ExtensionInheritanceClass::Partial,
                ),
                axis_decl(
                    AppearanceAxisClass::HostToken,
                    ExtensionInheritanceClass::DoesNotInherit,
                ),
            ],
            known_unsupported_states: vec![
                AppearanceUnsupportedState {
                    axis: AppearanceAxisClass::FocusRing,
                    state_label: "keyboard_focus_ring".to_owned(),
                    summary: "Toolbar buttons draw a custom focus outline, not the host ring."
                        .to_owned(),
                },
                AppearanceUnsupportedState {
                    axis: AppearanceAxisClass::HighContrast,
                    state_label: "forced_colors".to_owned(),
                    summary: "Toolbar icons keep fixed colors under forced-colors modes."
                        .to_owned(),
                },
                AppearanceUnsupportedState {
                    axis: AppearanceAxisClass::HostToken,
                    state_label: "spacing_tokens".to_owned(),
                    summary: "Toolbar spacing uses a private scale instead of host tokens."
                        .to_owned(),
                },
            ],
            declared_summary:
                "Inherits theme color only; density, focus, contrast, and tokens use private styling."
                    .to_owned(),
        },
        probes: vec![
            axis_probe(
                AppearanceAxisClass::Theme,
                AppearanceProofClass::ProvenInherits,
            ),
            axis_probe(
                AppearanceAxisClass::Density,
                AppearanceProofClass::ProvenUnsupported,
            ),
            axis_probe(
                AppearanceAxisClass::FocusRing,
                AppearanceProofClass::ProvenUnsupported,
            ),
            axis_probe(
                AppearanceAxisClass::HighContrast,
                AppearanceProofClass::ProvenUnsupported,
            ),
            axis_probe(
                AppearanceAxisClass::ReducedMotion,
                AppearanceProofClass::ProvenReduced,
            ),
            axis_probe(
                AppearanceAxisClass::HostToken,
                AppearanceProofClass::ProvenUnsupported,
            ),
        ],
        host_trust_label: "Mirrored — publisher continuity limited".to_owned(),
        host_severity_label: "Reduced appearance support".to_owned(),
        host_permission_label: "Workspace read".to_owned(),
        host_policy_label: "Allowed for mirrored catalog".to_owned(),
        host_rendered_trust_and_severity: true,
        evidence_refs: vec![
            "conformance_probe:net.legacy.toolbar:appearance".to_owned(),
            "mirror_import:net.legacy.toolbar:offline-bundle".to_owned(),
        ],
        caveat_summary:
            "Inherits theme color only; density, focus, contrast, and tokens use private styling."
                .to_owned(),
        generated_at: "2026-05-20T00:00:00Z".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn axis_decl(
    axis: AppearanceAxisClass,
    declared_class: ExtensionInheritanceClass,
) -> AppearanceAxisDeclaration {
    AppearanceAxisDeclaration {
        axis,
        declared_class,
        note: None,
    }
}

fn all_axes_declared(declared_class: ExtensionInheritanceClass) -> Vec<AppearanceAxisDeclaration> {
    APPEARANCE_AXES
        .iter()
        .map(|axis| axis_decl(*axis, declared_class))
        .collect()
}

fn axis_probe(axis: AppearanceAxisClass, proof_class: AppearanceProofClass) -> AppearanceAxisProbe {
    AppearanceAxisProbe {
        axis,
        proof_class,
        evidence_ref: Some(format!(
            "appearance_probe:{}:{}",
            axis.as_str(),
            proof_class.as_str()
        )),
        note: None,
    }
}

fn all_axes_probed(proof_class: AppearanceProofClass) -> Vec<AppearanceAxisProbe> {
    APPEARANCE_AXES
        .iter()
        .map(|axis| axis_probe(*axis, proof_class))
        .collect()
}

fn count_decision(
    rows: &[AppearanceConformanceRow],
    decision: AppearanceConformanceDecisionClass,
) -> usize {
    rows.iter()
        .filter(|row| row.decision_class == decision)
        .count()
}

fn count_overall(rows: &[AppearanceConformanceRow], class: AppearanceSupportClass) -> usize {
    rows.iter()
        .filter(|row| row.overall_support_class == class)
        .count()
}

fn defect_kind_tokens(defects: &[AppearanceConformanceDefect]) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    for defect in defects {
        let token = defect.defect_kind.as_str().to_owned();
        if !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    tokens.sort();
    tokens
}

fn support_row_matches_product_row(
    row: &AppearanceConformanceRow,
    support: &AppearanceConformanceSupportRow,
) -> bool {
    support.record_kind == EXTENSION_APPEARANCE_CONFORMANCE_SUPPORT_ROW_RECORD_KIND
        && support.schema_version == EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION
        && support.shared_contract_ref == EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF
        && support.extension_id == row.extension_id
        && support.extension_name == row.extension_name
        && support.publisher_label == row.publisher_label
        && support.surface_label == row.surface_label
        && support.lifecycle_class == row.lifecycle_class
        && support.overall_support_class == row.overall_support_class
        && support.decision_class == row.decision_class
        && support.reason_class == row.reason_class
        && support.host_trust_label == row.host_trust_label
        && support.host_severity_label == row.host_severity_label
        && support.host_rendered_trust_and_severity == row.host_rendered_trust_and_severity
        && support.row_defect_kind_tokens == row.row_defect_kind_tokens
        && support.caveat_summary == row.caveat_summary
        && support_axis_tokens_match(row, support)
}

fn support_axis_tokens_match(
    row: &AppearanceConformanceRow,
    support: &AppearanceConformanceSupportRow,
) -> bool {
    if support.axis_support_by_token.len() != row.axes.len() {
        return false;
    }
    row.axes.iter().all(|axis| {
        support
            .axis_support_by_token
            .get(axis.axis.as_str())
            .map(|token| token == axis.support_class.as_str())
            .unwrap_or(false)
    })
}

fn packet_defect(
    defect_kind: AppearanceConformanceDefectKind,
    field: impl Into<String>,
    message: impl Into<String>,
) -> AppearanceConformanceDefect {
    AppearanceConformanceDefect::new(
        "extension-appearance-conformance:packet",
        defect_kind,
        field,
        message,
    )
}

fn push_unique<T: Copy + PartialEq>(values: &mut Vec<T>, value: T) {
    if !values.contains(&value) {
        values.push(value);
    }
}
