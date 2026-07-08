//! Keyboard / screen-reader / CLI / export parity and honest auto-narrowing for
//! the six M5 badge families.
//!
//! This module is the M05-946 accessibility-and-auto-narrowing capstone over the
//! frozen M5 badge-family matrix
//! (`crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`).
//! Where the freeze matrix defines the six controlled badge families — support class,
//! evidence freshness, lifecycle, channel, deployment scope, and compatibility state —
//! with their explanation drawers, downgrade triggers, and consumer surfaces, and the
//! 941-944 implementation lanes resolve each family's per-surface truth, this lane
//! certifies — per family — that badge claims stay **keyboard-complete,
//! assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than
//! presenting a stale, limited, imported, or policy-blocked posture as still the badge's
//! full claim:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path into
//!   the same badge label, axis name, current value, explanation drawer, evidence
//!   source, and any downgrade reason the rich surface shows — never a hover-only,
//!   pointer-only, or color-encoded pill that strands assistive-tech or headless users.
//!   Hierarchy-heavy families (the compatibility-state badge's nested reconciliation
//!   detail — gap class, residual capability, repair action) additionally bind their
//!   tree to a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each
//!   badge's meaning from stable typed enums plus explanation / downgrade fields without
//!   a screenshot, preserving the same axis, value, and posture shown in-product.
//! - **Honest auto-narrowing.** When a family's support, freshness, lifecycle,
//!   deployment, or compatibility truth becomes stale, limited, imported, or
//!   policy-blocked, the badge's claim auto-narrows from its full claim to a limited /
//!   provisional / imported / policy-blocked ceiling, discloses the narrowing with a
//!   precise trigger and binding dimension, and preserves the canonical badge identity /
//!   axis rather than silently dropping it. A family with every dimension intact must NOT
//!   carry a spurious narrowing, and Certified never implies Fresh.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the marketplace,
//!   help, settings, onboarding, diagnostics, exported evidence, runtime, and CLI so
//!   badge truth stays aligned wherever it is viewed — a badge claim can never outrun
//!   the proof it is being viewed away from.
//!
//! Each [`BadgeAccessibilityRow`] keys on one reused frozen `M5BadgeFamily` and reuses
//! that frozen family vocabulary plus the frozen [`M5BadgeRequiredLabel`],
//! [`M5BadgeDowngradeTrigger`], and [`M5BadgeConsumerSurface`] rather than minting
//! parallel synonyms, so the certified labels stay byte-identical to the matrix and the
//! sibling badge lanes.
//!
//! The packet is metadata-only: raw evidence, signing keys, and credentials never cross
//! this boundary; the packet carries only typed class tokens, opaque summary / evidence
//! refs, booleans, and redacted labels so support and diagnostics exports can
//! reconstruct exactly what an accessible fallback would have shown without leaking
//! release material.
//!
//! The boundary schema is
//! [`schemas/ui/m5-badge-family-accessibility-fallback.schema.json`](../../../../schemas/ui/m5-badge-family-accessibility-fallback.schema.json).
//! The contract doc is
//! [`docs/release/m5_badge_family_accessibility_fallback.md`](../../../../docs/release/m5_badge_family_accessibility_fallback.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen badge vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel
// ones.
use crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix::{
    M5BadgeConsumerSurface, M5BadgeDowngradeTrigger, M5BadgeFamily, M5BadgeRequiredLabel,
    M5_BADGE_FAMILY_DOC_REF, M5_BADGE_FAMILY_SCHEMA_REF,
};

/// Schema version stamped on the M05-946 badge-family accessibility fallback packet.
pub const BADGE_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`BadgeAccessibilityPacket`].
pub const BADGE_A11Y_FALLBACK_RECORD_KIND: &str = "m5_badge_family_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`BadgeAccessibilityRow`].
pub const BADGE_A11Y_FALLBACK_ROW_RECORD_KIND: &str = "m5_badge_family_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const BADGE_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-badge-family-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const BADGE_A11Y_FALLBACK_DOC_REF: &str =
    "docs/release/m5_badge_family_accessibility_fallback.md";

/// Repo-relative path of the frozen badge-family matrix this lane certifies.
pub const BADGE_A11Y_FALLBACK_MATRIX_REF: &str = M5_BADGE_FAMILY_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const BADGE_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-badge-family-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const BADGE_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-badge-family-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const BADGE_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-badge-family-accessibility-fallback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const BADGE_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-badge-family-accessibility-fallback-proof/report.md";

/// The reusable badge families that render a non-linear hierarchy (the
/// compatibility-state badge's nested reconciliation detail — gap class, residual
/// capability, repair action) and therefore MUST bind their tree to an equivalent flat
/// list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5BadgeFamily) -> bool {
    matches!(family, M5BadgeFamily::CompatibilityState)
}

/// The badge dimension whose weakening a family primarily discloses. Every row must
/// model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(family: M5BadgeFamily) -> M5BadgeClaimDimension {
    match family {
        M5BadgeFamily::SupportClass => M5BadgeClaimDimension::SupportClassPosture,
        M5BadgeFamily::EvidenceFreshness => M5BadgeClaimDimension::EvidenceFreshness,
        M5BadgeFamily::Lifecycle => M5BadgeClaimDimension::LifecycleStage,
        M5BadgeFamily::Channel => M5BadgeClaimDimension::ChannelPosture,
        M5BadgeFamily::DeploymentScope => M5BadgeClaimDimension::DeploymentScope,
        M5BadgeFamily::CompatibilityState => M5BadgeClaimDimension::CompatibilityState,
    }
}

/// A rendered fallback modality for a badge family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeFallbackModality {
    /// A rich, structured (nested reconciliation / grouped drawer) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5BadgeFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich,
    /// structured surface (i.e. a keyboard / screen-reader / headless path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface:
/// the same badge may render at desktop-full capability or narrow to a companion,
/// read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / admin / evaluation export.
    SupportExport,
}

impl M5BadgeRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::HandoffPacket => "handoff_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a badge's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A hover-only / pointer-only surface that traps keyboard / assistive-tech /
    /// headless users (red).
    ViewOnlyTrap,
}

impl BadgeNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech /
    /// headless users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the badge meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeExportSummaryState {
    /// The badge meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl BadgeExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl BadgeNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The badge-claim ceiling a family asserts: how strong a posture it lets a surface
/// present. Auto-narrowing lowers this ceiling when a badge dimension weakens so a
/// stale, limited, imported, or policy-blocked posture can never keep the family's
/// full claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeSupportClaim {
    /// Full claim: a fully proven, current, evidence-verified badge posture — the
    /// strongest claim (e.g. Certified support class or Compatible state).
    FullClaim,
    /// Supported: a resolved, self-sufficient badge posture (a stated lifecycle stage or
    /// channel) that is not itself a fully certified claim.
    Supported,
    /// Limited: usable, but with a disclosed reduction in scope or confidence.
    Limited,
    /// Provisional: the underlying evidence is stale and being re-established; state is
    /// last-known, not current.
    Provisional,
    /// Imported: the badge posture is reconstructed from imported (not locally proven)
    /// evidence.
    Imported,
    /// Policy-blocked: a required entitlement / policy dependency is unmet.
    PolicyBlocked,
}

impl M5BadgeSupportClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::FullClaim,
        Self::Supported,
        Self::Limited,
        Self::Provisional,
        Self::Imported,
        Self::PolicyBlocked,
    ];

    /// Capability rank; a higher rank asserts a stronger badge posture. Narrowing lowers
    /// rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullClaim => 5,
            Self::Supported => 4,
            Self::Limited => 3,
            Self::Provisional => 2,
            Self::Imported => 1,
            Self::PolicyBlocked => 0,
        }
    }

    /// Returns true when this claim asserts a fully proven, full badge posture.
    pub const fn asserts_full_claim(self) -> bool {
        matches!(self, Self::FullClaim)
    }

    /// Returns true when this claim asserts a fully self-sufficient (full or supported)
    /// posture.
    pub const fn asserts_trustworthy_posture(self) -> bool {
        matches!(self, Self::FullClaim | Self::Supported)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullClaim => "full_claim",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Provisional => "provisional",
            Self::Imported => "imported",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The badge dimension whose state governs how far a family may claim to be current,
/// supported, or authoritative. Each dimension maps 1:1 to a frozen badge family so
/// every family carries an honest narrowing path and no axis is implied from another
/// (Certified never implies Fresh).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeClaimDimension {
    /// Support-class posture: is the support class fully certified, or narrowed?
    SupportClassPosture,
    /// Evidence freshness: is the proof behind the badge current, or stale / imported?
    EvidenceFreshness,
    /// Lifecycle stage: is the lifecycle stage stated and current?
    LifecycleStage,
    /// Channel posture: is the release channel stated and current?
    ChannelPosture,
    /// Deployment scope: is where-it-runs fully proven, or imported / mirror-only?
    DeploymentScope,
    /// Compatibility state: is host compatibility proven, or limited / policy-blocked?
    CompatibilityState,
}

impl M5BadgeClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SupportClassPosture,
        Self::EvidenceFreshness,
        Self::LifecycleStage,
        Self::ChannelPosture,
        Self::DeploymentScope,
        Self::CompatibilityState,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a
    /// narrowing. Each dimension maps to the on-topic frozen trigger the freeze matrix
    /// already governs, so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5BadgeDowngradeTrigger {
        match self {
            Self::SupportClassPosture => M5BadgeDowngradeTrigger::SupportClassValueUnstated,
            Self::EvidenceFreshness => M5BadgeDowngradeTrigger::EvidenceFreshnessHidden,
            Self::LifecycleStage => M5BadgeDowngradeTrigger::LifecycleValueUnstated,
            Self::ChannelPosture => M5BadgeDowngradeTrigger::ChannelValueUnstated,
            Self::DeploymentScope => M5BadgeDowngradeTrigger::DeploymentScopeUnstated,
            Self::CompatibilityState => M5BadgeDowngradeTrigger::CompatibilityStateUnstated,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportClassPosture => "support_class_posture",
            Self::EvidenceFreshness => "evidence_freshness",
            Self::LifecycleStage => "lifecycle_stage",
            Self::ChannelPosture => "channel_posture",
            Self::DeploymentScope => "deployment_scope",
            Self::CompatibilityState => "compatibility_state",
        }
    }
}

/// The observed condition of one badge dimension. Anything weaker than
/// [`Self::Current`] imposes a narrowing ceiling on the family's claim — exactly the
/// four weakening conditions the spec requires auto-narrowing on: stale, limited,
/// imported, or policy-blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeConditionState {
    /// Fully current / proven — imposes no ceiling.
    Current,
    /// Limited — scope or confidence is reduced; the claim drops to limited.
    Limited,
    /// Stale — the proof aged out and is re-establishing; the claim drops to provisional.
    Stale,
    /// Imported — the posture rests on imported (not locally proven) evidence; the claim
    /// drops to imported.
    Imported,
    /// Policy-blocked — a required entitlement / policy dependency is unmet; the claim
    /// drops to policy-blocked.
    PolicyBlocked,
}

impl M5BadgeConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Limited,
        Self::Stale,
        Self::Imported,
        Self::PolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than current and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Current)
    }

    /// The strongest badge claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5BadgeSupportClaim {
        match self {
            Self::Current => M5BadgeSupportClaim::FullClaim,
            Self::Limited => M5BadgeSupportClaim::Limited,
            Self::Stale => M5BadgeSupportClaim::Provisional,
            Self::Imported => M5BadgeSupportClaim::Imported,
            Self::PolicyBlocked => M5BadgeSupportClaim::PolicyBlocked,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Limited => "limited",
            Self::Stale => "stale",
            Self::Imported => "imported",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One badge dimension's observed condition on a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5BadgeClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5BadgeConditionState,
}

/// An honest badge-claim auto-narrow block. When a badge dimension weakens, the
/// family's claim lowers to the permitted ceiling, names the binding dimension and
/// frozen trigger, and preserves the canonical badge identity / axis rather than
/// silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeClaimAutoNarrow {
    /// The claim the badge is narrowed to.
    pub narrowed_to: M5BadgeSupportClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest
    /// ceiling constraint).
    pub binding_dimension: M5BadgeClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5BadgeDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical badge identity, axis name, and current value are preserved rather
    /// than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl BadgeClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a badge's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl BadgeCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited as
    /// the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5BadgeRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: BadgeNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a badge accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims a posture, or drops state
    /// silently (red).
    Stranded,
}

impl BadgeAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one M5 badge family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAccessibilityRow {
    /// Record kind; must equal [`BADGE_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BADGE_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen badge family this row certifies.
    pub badge_family: M5BadgeFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the object the badge labels; stays visible on every surface, so
    /// this is never empty.
    pub badge_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a
    /// non-visual (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5BadgeFallbackModality>,
    /// The non-visual / CLI path reaches the same badge label, axis, value, explanation,
    /// and downgrade truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: BadgeNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: BadgeNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: BadgeNonVisualReachState,
    /// Whether the export-safe summary preserves badge meaning.
    pub export_summary: BadgeExportSummaryState,
    /// Ref to the export-safe summary object for this badge.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: BadgeCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_support_claim: M5BadgeSupportClaim,
    /// The observed condition of each modeled badge dimension.
    #[serde(default)]
    pub claim_conditions: Vec<BadgeClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<BadgeClaimAutoNarrow>,
    /// Rendering surfaces this badge is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5BadgeRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<BadgeRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5BadgeRequiredLabel>,
    /// Semantic consumer surfaces this badge is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5BadgeConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl BadgeAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a
    /// flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.badge_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Current` when the row does
    /// not model that dimension.
    pub fn condition_for(&self, dimension: M5BadgeClaimDimension) -> M5BadgeConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5BadgeConditionState::Current)
    }

    /// Whether any modeled dimension is weaker than current.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5BadgeSupportClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5BadgeClaimDimension> {
        let mut binding: Option<(M5BadgeClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_support_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition.dimension, rank)),
            }
        }
        binding.map(|(dimension, _)| dimension)
    }

    /// The claim this badge effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5BadgeSupportClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale, limited, imported, or policy-blocked badge
    /// can no longer keep the family's full claim. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow
    /// block is present, narrows to exactly the permitted ceiling, binds to the
    /// ceiling-imposing dimension with its frozen trigger, and preserves canonical
    /// identity. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_dimension()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding
                    && narrow.trigger == binding.default_trigger()
                    && self.condition_for(binding).is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same badge
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.badge_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the badge meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the badge carries
    /// an honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its
    /// reduced interactivity and keeps its labels, so badge truth stays aligned on the
    /// same narrowed state wherever it is viewed.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed
        // surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.badge_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5BadgeRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> BadgeAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return BadgeAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            BadgeAccessibilityStatus::NarrowedDisclosed
        } else {
            BadgeAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == BADGE_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == BADGE_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.badge_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.badge_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_support_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-946 badge-family accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAccessibilitySummary {
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`BadgeAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<BadgeAccessibilityRow>,
}

/// Checked-in M05-946 badge-family accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<BadgeAccessibilityRow>,
    pub summary: BadgeAccessibilitySummary,
}

impl BadgeAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: BadgeAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: BADGE_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: BADGE_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: BadgeAccessibilitySummary {
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5BadgeFamily> {
        self.rows.iter().map(|r| r.badge_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5BadgeClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5BadgeSupportClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5BadgeConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> BadgeAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5BadgeConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&BadgeAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                BadgeAccessibilityStatus::Parity => green += 1,
                BadgeAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                BadgeAccessibilityStatus::Stranded => red += 1,
            }
        }

        BadgeAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(BadgeAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self.rows.iter().all(BadgeAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(BadgeAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(BadgeAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<BadgeAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BADGE_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(BadgeAccessibilityViolation::SchemaVersion {
                expected: BADGE_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BADGE_A11Y_FALLBACK_RECORD_KIND {
            violations.push(BadgeAccessibilityViolation::RecordKind {
                expected: BADGE_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(BadgeAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(BadgeAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.badge_family);

            if !row.is_complete() {
                violations.push(BadgeAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(BadgeAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.badge_family),
                });
            }

            // Each row must preserve every mandatory badge label.
            if !row.preserves_mandatory_labels() {
                violations.push(BadgeAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual
            // path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5BadgeFallbackModality::Structured)
            {
                violations.push(
                    BadgeAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: claim never over-asserts the family's full posture for a weakened one.
            if !row.claim_is_honest() {
                violations.push(BadgeAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same badge truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(BadgeAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(BadgeAccessibilityViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(BadgeAccessibilityViolation::NarrowingDropsContextSilently {
                    id: row.row_id.clone(),
                });
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(BadgeAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == BadgeAccessibilityStatus::Stranded {
                violations.push(BadgeAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5BadgeFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(BadgeAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5BadgeClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(BadgeAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing
        // spectrum (full → … → policy-blocked) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5BadgeSupportClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(BadgeAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Cross-surface: the same narrowed state must reach the marketplace, help,
        // settings, onboarding, diagnostics, docs, evaluation, support / admin exports,
        // CLI, and product UI — so every consumer surface is exercised at least once
        // across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5BadgeConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations
                    .push(BadgeAccessibilityViolation::MissingConsumerSurfaceCoverage { surface });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(BadgeAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("badge accessibility fallback packet serializes"),
        ) {
            violations.push(BadgeAccessibilityViolation::RawReleaseMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("badge accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,badge_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.badge_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_support_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Badge-Family Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5BadgeFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.badge_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_support_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in badge-family accessibility fallback export.
pub fn current_m5_badge_a11y_fallback_export(
) -> Result<BadgeAccessibilityPacket, BadgeAccessibilityArtifactError> {
    let packet: BadgeAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-badge-family-accessibility-fallback-proof/support_export.json"
    )))
    .map_err(BadgeAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BadgeAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in badge-family accessibility fallback
/// export.
#[derive(Debug)]
pub enum BadgeAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BadgeAccessibilityViolation>),
}

impl fmt::Display for BadgeAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "badge accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "badge accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for BadgeAccessibilityArtifactError {}

/// Validation failure for M05-946 badge-family accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5BadgeClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5BadgeFamily,
    },
    MissingDimensionCoverage {
        dimension: M5BadgeClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5BadgeSupportClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5BadgeConsumerSurface,
    },
    SummaryMismatch,
    RawReleaseMaterialInExport,
}

impl fmt::Display for BadgeAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory badge label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts its full badge posture for a weakened one, or narrows spuriously"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the badge truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(f, "badge family {family:?} is not certified in the packet")
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawReleaseMaterialInExport => write!(f, "export contains raw release material"),
        }
    }
}

impl Error for BadgeAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "limited"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in badge-family accessibility fallback packet. This is
/// the one source of truth shared by the tests, the example dump, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_badge_a11y_fallback_packet() -> BadgeAccessibilityPacket {
    BadgeAccessibilityPacket::new(BadgeAccessibilityPacketInput {
        packet_id: "m5-badge-family-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-08T00:00:00Z".to_owned(),
        matrix_ref: BADGE_A11Y_FALLBACK_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:badge-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5BadgeRequiredLabel> {
    M5BadgeRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> BadgeCopyExportParity {
    BadgeCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5BadgeClaimDimension,
    state: M5BadgeConditionState,
) -> BadgeClaimConditionEntry {
    BadgeClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / admin export and
/// CLI inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5BadgeConsumerSurface]) -> Vec<M5BadgeConsumerSurface> {
    let mut out = vec![
        M5BadgeConsumerSurface::SupportExport,
        M5BadgeConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity)
/// row keeps full label and summary parity on the narrower surfaces; a narrowed row
/// discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: BadgeNarrowingDisclosureState,
) -> Vec<BadgeRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        BadgeRenderingNarrowingDisclosure {
            rendering_surface: M5BadgeRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        BadgeRenderingNarrowingDisclosure {
            rendering_surface: M5BadgeRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label
/// and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<BadgeRenderingNarrowingDisclosure> {
    surface_disclosures(labels, BadgeNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<BadgeRenderingNarrowingDisclosure> {
    surface_disclosures(labels, BadgeNarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5BadgeRenderingSurface> {
    vec![
        M5BadgeRenderingSurface::DesktopFull,
        M5BadgeRenderingSurface::CliHeadless,
        M5BadgeRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<BadgeAccessibilityRow> {
    vec![
        // Support-class badge — the support class is fully certified and its backing is
        // intact, so the badge carries its full posture and is reachable on every
        // surface (green). Certified here never implies Fresh: freshness is a separate
        // badge with its own row.
        BadgeAccessibilityRow {
            record_kind: BADGE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BADGE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:support-class-badge".to_owned(),
            badge_family: M5BadgeFamily::SupportClass,
            source_family_schema_ref: BADGE_A11Y_FALLBACK_MATRIX_REF.to_owned(),
            badge_context_ref: "badge:support-class:0001".to_owned(),
            fallback_modalities: vec![
                M5BadgeFallbackModality::List,
                M5BadgeFallbackModality::Textual,
                M5BadgeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            cli_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            export_summary: BadgeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:support-class-badge:a11y".to_owned(),
            copy_export: copy_export(&["identity", "axis_name", "value_state", "explanation_drawer"]),
            full_support_claim: M5BadgeSupportClaim::FullClaim,
            claim_conditions: vec![condition(
                M5BadgeClaimDimension::SupportClassPosture,
                M5BadgeConditionState::Current,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["identity", "axis_name", "value_state"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BadgeConsumerSurface::MarketplaceUi,
                M5BadgeConsumerSurface::HelpAbout,
            ]),
            source_refs: vec![
                "UX Design System §16.21 capability / support-class / lifecycle badges".to_owned(),
                BADGE_A11Y_FALLBACK_DOC_REF.to_owned(),
                M5_BADGE_FAMILY_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-08T00:00:00Z".to_owned(),
            evidence_refs: ev("support-class-badge"),
        },
        // Lifecycle badge — the lifecycle stage is stated and current, so the badge
        // carries a supported, self-sufficient stage with no unstated lifecycle (green).
        BadgeAccessibilityRow {
            record_kind: BADGE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BADGE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:lifecycle-badge".to_owned(),
            badge_family: M5BadgeFamily::Lifecycle,
            source_family_schema_ref: BADGE_A11Y_FALLBACK_MATRIX_REF.to_owned(),
            badge_context_ref: "badge:lifecycle:0002".to_owned(),
            fallback_modalities: vec![
                M5BadgeFallbackModality::List,
                M5BadgeFallbackModality::Textual,
                M5BadgeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            cli_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            export_summary: BadgeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:lifecycle-badge:a11y".to_owned(),
            copy_export: copy_export(&["identity", "axis_name", "value_state", "filter_key"]),
            full_support_claim: M5BadgeSupportClaim::Supported,
            claim_conditions: vec![condition(
                M5BadgeClaimDimension::LifecycleStage,
                M5BadgeConditionState::Current,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["identity", "axis_name", "value_state"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BadgeConsumerSurface::SettingsUi,
                M5BadgeConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "Milestones v3.1 badge-and-notice stability".to_owned(),
                BADGE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-08T00:00:00Z".to_owned(),
            evidence_refs: ev("lifecycle-badge"),
        },
        // Evidence-freshness badge — the proof behind the claim aged out and is being
        // re-established, so the freshness badge auto-narrows to provisional and reads
        // from last-known evidence rather than a fresh reading (yellow).
        BadgeAccessibilityRow {
            record_kind: BADGE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BADGE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:evidence-freshness-badge".to_owned(),
            badge_family: M5BadgeFamily::EvidenceFreshness,
            source_family_schema_ref: BADGE_A11Y_FALLBACK_MATRIX_REF.to_owned(),
            badge_context_ref: "badge:evidence-freshness:0003".to_owned(),
            fallback_modalities: vec![
                M5BadgeFallbackModality::List,
                M5BadgeFallbackModality::Textual,
                M5BadgeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            cli_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            export_summary: BadgeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:evidence-freshness-badge:a11y".to_owned(),
            copy_export: copy_export(&["identity", "axis_name", "value_state", "evidence_source"]),
            full_support_claim: M5BadgeSupportClaim::FullClaim,
            claim_conditions: vec![condition(
                M5BadgeClaimDimension::EvidenceFreshness,
                M5BadgeConditionState::Stale,
            )],
            claim_narrow: Some(BadgeClaimAutoNarrow {
                narrowed_to: M5BadgeSupportClaim::Provisional,
                binding_dimension: M5BadgeClaimDimension::EvidenceFreshness,
                trigger: M5BadgeDowngradeTrigger::EvidenceFreshnessHidden,
                narrowed_label:
                    "Evidence stale — freshness shown from last-known proof until re-verification lands"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["identity", "axis_name", "value_state"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BadgeConsumerSurface::DiagnosticsSurface,
                M5BadgeConsumerSurface::DocsPortal,
            ]),
            source_refs: vec![
                "UI/UX Spec §10.7 qualification / freshness / support badges".to_owned(),
                BADGE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-08T00:00:00Z".to_owned(),
            evidence_refs: ev("evidence-freshness-badge"),
        },
        // Channel badge — the release channel context is only partially resolved (a
        // channel reassignment is mid-flight), so the channel badge auto-narrows to
        // limited until the channel settles (yellow).
        BadgeAccessibilityRow {
            record_kind: BADGE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BADGE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:channel-badge".to_owned(),
            badge_family: M5BadgeFamily::Channel,
            source_family_schema_ref: BADGE_A11Y_FALLBACK_MATRIX_REF.to_owned(),
            badge_context_ref: "badge:channel:0004".to_owned(),
            fallback_modalities: vec![
                M5BadgeFallbackModality::List,
                M5BadgeFallbackModality::Textual,
                M5BadgeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            cli_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            export_summary: BadgeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:channel-badge:a11y".to_owned(),
            copy_export: copy_export(&["identity", "axis_name", "value_state", "filter_key"]),
            full_support_claim: M5BadgeSupportClaim::FullClaim,
            claim_conditions: vec![condition(
                M5BadgeClaimDimension::ChannelPosture,
                M5BadgeConditionState::Limited,
            )],
            claim_narrow: Some(BadgeClaimAutoNarrow {
                narrowed_to: M5BadgeSupportClaim::Limited,
                binding_dimension: M5BadgeClaimDimension::ChannelPosture,
                trigger: M5BadgeDowngradeTrigger::ChannelValueUnstated,
                narrowed_label:
                    "Channel reassignment in flight — channel shown limited until the new channel settles"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["identity", "axis_name", "value_state"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BadgeConsumerSurface::OnboardingFlow,
                M5BadgeConsumerSurface::EvaluationPack,
            ]),
            source_refs: vec![
                "Milestones v3.1 claim-propagation governance".to_owned(),
                BADGE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-08T00:00:00Z".to_owned(),
            evidence_refs: ev("channel-badge"),
        },
        // Deployment-scope badge — where-it-runs rests on imported (mirror / offline)
        // evidence not locally proven on this host, so the deployment-scope badge
        // auto-narrows to imported and reads from the imported record (yellow).
        BadgeAccessibilityRow {
            record_kind: BADGE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BADGE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:deployment-scope-badge".to_owned(),
            badge_family: M5BadgeFamily::DeploymentScope,
            source_family_schema_ref: BADGE_A11Y_FALLBACK_MATRIX_REF.to_owned(),
            badge_context_ref: "badge:deployment-scope:0005".to_owned(),
            fallback_modalities: vec![
                M5BadgeFallbackModality::List,
                M5BadgeFallbackModality::Textual,
                M5BadgeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            cli_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            export_summary: BadgeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:deployment-scope-badge:a11y".to_owned(),
            copy_export: copy_export(&["identity", "axis_name", "value_state", "evidence_source"]),
            full_support_claim: M5BadgeSupportClaim::FullClaim,
            claim_conditions: vec![condition(
                M5BadgeClaimDimension::DeploymentScope,
                M5BadgeConditionState::Imported,
            )],
            claim_narrow: Some(BadgeClaimAutoNarrow {
                narrowed_to: M5BadgeSupportClaim::Imported,
                binding_dimension: M5BadgeClaimDimension::DeploymentScope,
                trigger: M5BadgeDowngradeTrigger::DeploymentScopeUnstated,
                narrowed_label:
                    "Deployment scope from imported mirror evidence — shown imported until locally re-proven"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["identity", "axis_name", "value_state"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BadgeConsumerSurface::MarketplaceUi,
                M5BadgeConsumerSurface::DiagnosticsSurface,
            ]),
            source_refs: vec![
                "TAD/TDD capability lifecycle metadata & deployment-scope truth".to_owned(),
                BADGE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-08T00:00:00Z".to_owned(),
            evidence_refs: ev("deployment-scope-badge"),
        },
        // Compatibility-state badge — hierarchy-heavy (nested reconciliation detail: gap
        // class, residual capability, repair action); host compatibility requires policy
        // approval that has not landed, so the compatibility badge auto-narrows to
        // policy-blocked and binds its reconciliation tree to a flat list / textual path
        // (yellow).
        BadgeAccessibilityRow {
            record_kind: BADGE_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: BADGE_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:compatibility-state-badge".to_owned(),
            badge_family: M5BadgeFamily::CompatibilityState,
            source_family_schema_ref: BADGE_A11Y_FALLBACK_MATRIX_REF.to_owned(),
            badge_context_ref: "badge:compatibility-state:0006".to_owned(),
            fallback_modalities: vec![
                M5BadgeFallbackModality::Structured,
                M5BadgeFallbackModality::List,
                M5BadgeFallbackModality::Textual,
                M5BadgeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BadgeNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: BadgeNonVisualReachState::ReachableAndLabeled,
            export_summary: BadgeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:compatibility-state-badge:a11y".to_owned(),
            copy_export: copy_export(&[
                "identity",
                "axis_name",
                "value_state",
                "explanation_drawer",
            ]),
            full_support_claim: M5BadgeSupportClaim::FullClaim,
            claim_conditions: vec![condition(
                M5BadgeClaimDimension::CompatibilityState,
                M5BadgeConditionState::PolicyBlocked,
            )],
            claim_narrow: Some(BadgeClaimAutoNarrow {
                narrowed_to: M5BadgeSupportClaim::PolicyBlocked,
                binding_dimension: M5BadgeClaimDimension::CompatibilityState,
                trigger: M5BadgeDowngradeTrigger::CompatibilityStateUnstated,
                narrowed_label:
                    "Compatibility blocked by policy — host compatibility not confirmable until an approver signs off"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["identity", "axis_name", "value_state"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5BadgeConsumerSurface::SettingsUi,
                M5BadgeConsumerSurface::DocsPortal,
            ]),
            source_refs: vec![
                "TAD/TDD compatibility & claim / report publication expectations".to_owned(),
                BADGE_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-08T00:00:00Z".to_owned(),
            evidence_refs: ev("compatibility-state-badge"),
        },
    ]
}
