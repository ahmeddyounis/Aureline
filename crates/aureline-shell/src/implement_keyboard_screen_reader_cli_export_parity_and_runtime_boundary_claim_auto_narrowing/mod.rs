//! Keyboard / screen-reader / CLI / export parity and honest auto-narrowing for
//! the M5 runtime-boundary components.
//!
//! This module is the M05-858 accessibility-and-auto-narrowing capstone over the
//! frozen M5 runtime-boundary component matrix
//! ([`crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix`]).
//! Where the freeze matrix defines the reusable terminal tab / header strip, remote
//! target pill, environment status strip, toolchain pin row, presence avatar stack,
//! and repair action card primitives, and the 853-857 implementation lanes resolve
//! their per-surface truth, this lane certifies — per component family — that
//! runtime-boundary and repair claims stay **keyboard-complete, assistive-tech-reachable,
//! CLI/export-safe, and self-narrowing** rather than presenting a stale, partial,
//! reconnecting, restored, or policy-blocked runtime state as still `Ready` or `Live`:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path
//!   into the same session title, host boundary, shell-integration quality, winning
//!   runtime/toolchain, collaboration role/follow state, and repair blast-radius /
//!   reversibility the rich surface shows — never a view-only card that strands
//!   assistive-tech or headless users. Hierarchy-heavy families (the toolchain pin
//!   row's precedence inspector with its ordered shadowed layers) additionally bind
//!   their tree to a flat list / textual path.
//! - **Export parity.** The support / release export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the
//!   same host boundaries, runtime sources, roles, and reversal classes shown
//!   in-product.
//! - **Honest auto-narrowing.** When host identity, shell-integration confidence,
//!   context precedence, collaboration role, or repair reversibility becomes stale,
//!   partial, reconnecting, restored, or policy-blocked, the component's
//!   runtime-support claim auto-narrows from `Live` / `Ready` to degraded /
//!   reconnecting / restored / policy-blocked, discloses the narrowing with a precise
//!   trigger and binding dimension, and preserves the canonical host/runtime/role
//!   identity rather than silently dropping it. A component with every dimension
//!   intact must NOT carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in shell chrome,
//!   side panels, docs/help, headless CLI, and support/admin exports so claim
//!   publication and field triage stay aligned on runtime-boundary downgrade behavior.
//!
//! Each [`RuntimeAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix::M5RuntimeBoundaryComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5RuntimeBoundaryRequiredLabel`] and [`M5RuntimeBoundaryDowngradeTrigger`] and the
//! shared [`M5ShellConsumerSurface`] consumer surfaces rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the
//! sibling primitive packets.
//!
//! The packet is metadata-only: raw terminal buffers, credentials, connection
//! secrets, and provider cursors never cross this boundary; the packet carries only
//! typed class tokens, opaque summary / evidence refs, booleans, and redacted labels
//! so support and diagnostics exports can reconstruct exactly what an accessible
//! fallback would have shown without leaking runtime state.
//!
//! The boundary schema is
//! [`schemas/ui/m5-runtime-boundary-component-accessibility-fallback.schema.json`](../../../../schemas/ui/m5-runtime-boundary-component-accessibility-fallback.schema.json).
//! The contract doc is
//! [`docs/runtime/m5_runtime_boundary_component_accessibility_fallback.md`](../../../../docs/runtime/m5_runtime_boundary_component_accessibility_fallback.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's
// families, required labels, and downgrade triggers rather than mint parallel ones.
use crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix::{
    M5RuntimeBoundaryComponentFamily, M5RuntimeBoundaryDowngradeTrigger,
    M5RuntimeBoundaryRequiredLabel,
};
// Reused consumer-surface family already minted by the shell-zone matrix: the same
// shell-frame / status-bar / docs-help / release-proof / support-export / product-ui
// surfaces ingest this accessibility fallback, so no parallel surface vocabulary is
// coined.
use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::M5ShellConsumerSurface;

/// Schema version stamped on the M05-858 runtime-boundary accessibility fallback
/// packet.
pub const RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`RuntimeAccessibilityPacket`].
pub const RUNTIME_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_runtime_boundary_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`RuntimeAccessibilityRow`].
pub const RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_runtime_boundary_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const RUNTIME_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-runtime-boundary-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const RUNTIME_A11Y_FALLBACK_DOC_REF: &str =
    "docs/runtime/m5_runtime_boundary_component_accessibility_fallback.md";

/// Repo-relative path of the frozen runtime-boundary component matrix this lane
/// certifies.
pub const RUNTIME_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-runtime-boundary-components.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const RUNTIME_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-runtime-boundary-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const RUNTIME_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const RUNTIME_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const RUNTIME_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/report.md";

/// The reusable component families that render a non-linear hierarchy (the toolchain
/// pin row's precedence inspector with its ordered shadowed layers) and therefore
/// MUST bind their tree to an equivalent flat list / textual path so the hierarchy is
/// navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5RuntimeBoundaryComponentFamily) -> bool {
    matches!(family, M5RuntimeBoundaryComponentFamily::ToolchainPinRow)
}

/// The runtime-boundary dimension whose weakening a family primarily discloses. Every
/// row must model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5RuntimeBoundaryComponentFamily,
) -> M5RuntimeClaimDimension {
    match family {
        M5RuntimeBoundaryComponentFamily::TerminalTab => {
            M5RuntimeClaimDimension::ShellIntegrationConfidence
        }
        M5RuntimeBoundaryComponentFamily::RemoteTargetPill => M5RuntimeClaimDimension::HostIdentity,
        M5RuntimeBoundaryComponentFamily::EnvironmentStatusStrip
        | M5RuntimeBoundaryComponentFamily::ToolchainPinRow => {
            M5RuntimeClaimDimension::ContextPrecedence
        }
        M5RuntimeBoundaryComponentFamily::PresenceAvatarStack => {
            M5RuntimeClaimDimension::CollaborationRole
        }
        M5RuntimeBoundaryComponentFamily::RepairActionCard => {
            M5RuntimeClaimDimension::RepairReversibility
        }
    }
}

/// A rendered fallback modality for a runtime-boundary component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeFallbackModality {
    /// A rich, structured (precedence tree / grouped inventory) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5RuntimeFallbackModality {
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
/// the same component may render at desktop-full capability or narrow to a companion,
/// read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeRenderingSurface {
    /// The full-capability desktop shell surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / admin export.
    SupportExport,
}

impl M5RuntimeRenderingSurface {
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

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl RuntimeNonVisualReachState {
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

/// Whether an export-safe summary preserves the component meaning without a
/// screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl RuntimeExportSummaryState {
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
pub enum RuntimeNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl RuntimeNarrowingDisclosureState {
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

/// The runtime-support claim ceiling a component asserts: how strong a runtime
/// posture it lets a surface present. Auto-narrowing lowers this ceiling when a
/// runtime dimension weakens so a stale, partial, reconnecting, restored, or
/// policy-blocked runtime can never keep an old `Live` or `Ready` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeSupportClaim {
    /// Live and current: an actively connected, authoritative runtime session — the
    /// strongest claim.
    Live,
    /// Ready: a resolved, self-sufficient runtime that is not itself a live stream
    /// (e.g. a resolved environment / toolchain selection).
    Ready,
    /// Degraded: usable, but with a disclosed reduction in scope or confidence.
    Degraded,
    /// Reconnecting: the link dropped and is re-establishing; state is last-known,
    /// not live.
    Reconnecting,
    /// Restored: reconstructed from a transcript / snapshot, not a live session.
    Restored,
    /// Policy-blocked: a required entitlement / policy dependency is unmet.
    PolicyBlocked,
}

impl M5RuntimeSupportClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::Live,
        Self::Ready,
        Self::Degraded,
        Self::Reconnecting,
        Self::Restored,
        Self::PolicyBlocked,
    ];

    /// Capability rank; a higher rank asserts a stronger runtime posture. Narrowing
    /// lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::Live => 5,
            Self::Ready => 4,
            Self::Degraded => 3,
            Self::Reconnecting => 2,
            Self::Restored => 1,
            Self::PolicyBlocked => 0,
        }
    }

    /// Returns true when this claim asserts a live, actively connected session.
    pub const fn asserts_live(self) -> bool {
        matches!(self, Self::Live)
    }

    /// Returns true when this claim asserts a fully self-sufficient (live or resolved
    /// / current) posture.
    pub const fn asserts_full_self_sufficiency(self) -> bool {
        matches!(self, Self::Live | Self::Ready)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Reconnecting => "reconnecting",
            Self::Restored => "restored",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The runtime-boundary dimension whose state governs how far a component may claim
/// to be live, ready, or authoritative. These are exactly the five axes the spec
/// requires auto-narrowing on: host identity, shell-integration confidence, context
/// precedence, collaboration role, and repair reversibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeClaimDimension {
    /// Host identity: is the reported host / boundary verified and current?
    HostIdentity,
    /// Shell-integration confidence: is the terminal's shell-integration quality
    /// high, or heuristic / unknown / restored?
    ShellIntegrationConfidence,
    /// Context precedence: has the winning runtime / toolchain scope resolved without
    /// an unshadowed conflict?
    ContextPrecedence,
    /// Collaboration role: is the participant's role / follow state live, or
    /// last-known / reconnecting?
    CollaborationRole,
    /// Repair reversibility: is the repair's reversal class proven, or unverified /
    /// policy-gated?
    RepairReversibility,
}

impl M5RuntimeClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HostIdentity,
        Self::ShellIntegrationConfidence,
        Self::ContextPrecedence,
        Self::CollaborationRole,
        Self::RepairReversibility,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a
    /// narrowing. Each dimension maps to the on-topic frozen trigger the freeze matrix
    /// already governs, so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5RuntimeBoundaryDowngradeTrigger {
        match self {
            Self::HostIdentity => M5RuntimeBoundaryDowngradeTrigger::HostBoundaryMasked,
            Self::ShellIntegrationConfidence => {
                M5RuntimeBoundaryDowngradeTrigger::ShellIntegrationQualityHidden
            }
            Self::ContextPrecedence => M5RuntimeBoundaryDowngradeTrigger::RuntimeSourceUnexplained,
            Self::CollaborationRole => M5RuntimeBoundaryDowngradeTrigger::CollaborationRoleMasked,
            Self::RepairReversibility => M5RuntimeBoundaryDowngradeTrigger::ReversibilityOverstated,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostIdentity => "host_identity",
            Self::ShellIntegrationConfidence => "shell_integration_confidence",
            Self::ContextPrecedence => "context_precedence",
            Self::CollaborationRole => "collaboration_role",
            Self::RepairReversibility => "repair_reversibility",
        }
    }
}

/// The observed condition of one runtime-boundary dimension. Anything weaker than
/// [`Self::Intact`] imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeConditionState {
    /// Fully verified / current / live — imposes no ceiling.
    Intact,
    /// Partially resolved — scope or confidence is reduced; support drops to degraded.
    Partial,
    /// Reconnecting — the link dropped and is re-establishing; support drops to
    /// reconnecting.
    Reconnecting,
    /// Restored — reconstructed from a transcript / snapshot, not live; support drops
    /// to restored.
    Restored,
    /// Policy-blocked — a required entitlement / policy dependency is unmet; support
    /// drops to policy-blocked.
    PolicyBlocked,
}

impl M5RuntimeConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Intact,
        Self::Partial,
        Self::Reconnecting,
        Self::Restored,
        Self::PolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than intact and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Intact)
    }

    /// The strongest runtime-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5RuntimeSupportClaim {
        match self {
            Self::Intact => M5RuntimeSupportClaim::Live,
            Self::Partial => M5RuntimeSupportClaim::Degraded,
            Self::Reconnecting => M5RuntimeSupportClaim::Reconnecting,
            Self::Restored => M5RuntimeSupportClaim::Restored,
            Self::PolicyBlocked => M5RuntimeSupportClaim::PolicyBlocked,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::Partial => "partial",
            Self::Reconnecting => "reconnecting",
            Self::Restored => "restored",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One runtime-boundary dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5RuntimeClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5RuntimeConditionState,
}

/// An honest runtime-support-claim auto-narrow block. When a runtime dimension
/// weakens, the component's support claim lowers to the permitted ceiling, names the
/// binding dimension and frozen trigger, and preserves the canonical host / runtime /
/// role identity rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5RuntimeSupportClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the
    /// strongest ceiling constraint).
    pub binding_dimension: M5RuntimeClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5RuntimeBoundaryDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical host boundary, runtime source, collaboration role, and reversal
    /// class are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl RuntimeClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must
/// be copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl RuntimeCopyExportParity {
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
pub struct RuntimeRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5RuntimeRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: RuntimeNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a runtime-boundary accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims support, or drops
    /// state silently (red).
    Stranded,
}

impl RuntimeAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one runtime-boundary component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAccessibilityRow {
    /// Record kind; must equal [`RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5RuntimeBoundaryComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the runtime / session / target context this component acts on;
    /// stays visible on every surface, so this is never empty.
    pub runtime_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a
    /// non-visual (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5RuntimeFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical host boundary, runtime
    /// source, role, and reversal class as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: RuntimeNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: RuntimeNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: RuntimeNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: RuntimeExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: RuntimeCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5RuntimeSupportClaim,
    /// The observed condition of each modeled runtime dimension.
    #[serde(default)]
    pub claim_conditions: Vec<RuntimeClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below
    /// the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<RuntimeClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5RuntimeRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<RuntimeRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5RuntimeBoundaryRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl RuntimeAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to
    /// a flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Intact` when the row does
    /// not model that dimension.
    pub fn condition_for(&self, dimension: M5RuntimeClaimDimension) -> M5RuntimeConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5RuntimeConditionState::Intact)
    }

    /// Whether any modeled dimension is weaker than intact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled
    /// dimension's ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5RuntimeSupportClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak
    /// dimension narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5RuntimeClaimDimension> {
        let mut binding: Option<(M5RuntimeClaimDimension, u8)> = None;
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

    /// The support claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5RuntimeSupportClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC1 / auto-narrowing honesty: a stale, partial, reconnecting, restored, or
    /// policy-blocked runtime can no longer keep an old `Live` / `Ready` label. The
    /// effective claim never exceeds the permitted ceiling; when a dimension narrows
    /// below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity. When nothing narrows, no spurious
    /// narrow block is present.
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

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same
    /// canonical truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning
    /// without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.runtime_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component
    /// carries an honest claim narrow.
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
    /// reduced interactivity and keeps its labels, so claim publication and field
    /// triage stay aligned on the same narrowed state.
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
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible
    /// fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5RuntimeBoundaryRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> RuntimeAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return RuntimeAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            RuntimeAccessibilityStatus::NarrowedDisclosed
        } else {
            RuntimeAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.runtime_context_ref.trim().is_empty()
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
            family = self.component_family.as_str(),
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

/// Rolled-up summary of an M05-858 runtime-boundary accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAccessibilitySummary {
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

/// Constructor input for [`RuntimeAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<RuntimeAccessibilityRow>,
}

/// Checked-in M05-858 runtime-boundary accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<RuntimeAccessibilityRow>,
    pub summary: RuntimeAccessibilitySummary,
}

impl RuntimeAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: RuntimeAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: RUNTIME_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: RuntimeAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5RuntimeBoundaryComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5RuntimeClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5RuntimeSupportClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> HashSet<M5ShellConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> RuntimeAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: HashSet<M5ShellConsumerSurface> = HashSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&RuntimeAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                RuntimeAccessibilityStatus::Parity => green += 1,
                RuntimeAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                RuntimeAccessibilityStatus::Stranded => red += 1,
            }
        }

        RuntimeAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(RuntimeAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(RuntimeAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(RuntimeAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(RuntimeAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<RuntimeAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(RuntimeAccessibilityViolation::SchemaVersion {
                expected: RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != RUNTIME_A11Y_FALLBACK_RECORD_KIND {
            violations.push(RuntimeAccessibilityViolation::RecordKind {
                expected: RUNTIME_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(RuntimeAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(RuntimeAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(RuntimeAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(RuntimeAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // Each row must preserve every mandatory runtime-boundary label.
            if !row.preserves_mandatory_labels() {
                violations.push(RuntimeAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a
            // non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5RuntimeFallbackModality::Structured)
            {
                violations.push(RuntimeAccessibilityViolation::HierarchyHeavyMissingStructured {
                    id: row.row_id.clone(),
                });
            }

            // AC1: claim never over-asserts a live / ready runtime for a weakened one.
            if !row.claim_is_honest() {
                violations.push(RuntimeAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(RuntimeAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(RuntimeAccessibilityViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(RuntimeAccessibilityViolation::NarrowingDropsContextSilently {
                    id: row.row_id.clone(),
                });
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(RuntimeAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == RuntimeAccessibilityStatus::Stranded {
                violations.push(RuntimeAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5RuntimeBoundaryComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(RuntimeAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5RuntimeClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(RuntimeAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the
        // full narrowing spectrum (live → … → policy-blocked) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5RuntimeSupportClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(RuntimeAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Cross-surface: the same narrowed state must reach shell chrome, docs/help,
        // release proof, and support/admin exports — so every consumer surface is
        // exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5_RUNTIME_A11Y_CONSUMER_SURFACES {
            if !consumers.contains(&surface) {
                violations
                    .push(RuntimeAccessibilityViolation::MissingConsumerSurfaceCoverage { surface });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(RuntimeAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("runtime accessibility fallback packet serializes"),
        ) {
            violations.push(RuntimeAccessibilityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("runtime accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
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
        out.push_str("# M5 Runtime-Boundary Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5RuntimeBoundaryComponentFamily::ALL.len(),
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
                row.component_family.as_str(),
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

/// Reads and validates the checked-in runtime-boundary accessibility fallback export.
pub fn current_m5_runtime_a11y_fallback_export(
) -> Result<RuntimeAccessibilityPacket, RuntimeAccessibilityArtifactError> {
    let packet: RuntimeAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/support_export.json"
    )))
    .map_err(RuntimeAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RuntimeAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in runtime-boundary accessibility fallback
/// export.
#[derive(Debug)]
pub enum RuntimeAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RuntimeAccessibilityViolation>),
}

impl fmt::Display for RuntimeAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "runtime accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "runtime accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for RuntimeAccessibilityArtifactError {}

/// The consumer surfaces this lane requires the packet to exercise. A subset of the
/// full [`M5ShellConsumerSurface`] set that always includes the two field-triage
/// surfaces (release proof and support export) plus the shell/docs/product surfaces
/// where a runtime-boundary component is embedded.
pub const M5_RUNTIME_A11Y_CONSUMER_SURFACES: [M5ShellConsumerSurface; 10] = [
    M5ShellConsumerSurface::ShellFrame,
    M5ShellConsumerSurface::Windowing,
    M5ShellConsumerSurface::Layout,
    M5ShellConsumerSurface::StatusBar,
    M5ShellConsumerSurface::AttentionRouter,
    M5ShellConsumerSurface::NotificationEnvelope,
    M5ShellConsumerSurface::DocsHelp,
    M5ShellConsumerSurface::ReleaseProof,
    M5ShellConsumerSurface::SupportExport,
    M5ShellConsumerSurface::ProductUi,
];

/// Validation failure for M05-858 runtime-boundary accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeAccessibilityViolation {
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
        dimension: M5RuntimeClaimDimension,
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
        family: M5RuntimeBoundaryComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5RuntimeClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5RuntimeSupportClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5ShellConsumerSurface,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for RuntimeAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema version mismatch: expected {expected}, got {actual}")
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
                write!(f, "row {id} drops a mandatory runtime-boundary label")
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
                    "row {id} over-asserts a live / ready runtime for a weakened one, or narrows spuriously"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(f, "row {id} narrows a rendering surface without disclosing it")
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(f, "component family {family:?} is not certified in the packet")
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
                    "support claim tier {} does not appear as an effective claim",
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
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for RuntimeAccessibilityViolation {}

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

/// Builds the canonical, checked-in runtime-boundary accessibility fallback packet.
/// This is the one source of truth shared by the tests, the example dump, and the
/// on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_runtime_a11y_fallback_packet() -> RuntimeAccessibilityPacket {
    RuntimeAccessibilityPacket::new(RuntimeAccessibilityPacketInput {
        packet_id: "m5-runtime-boundary-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: RUNTIME_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:runtime-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5RuntimeBoundaryRequiredLabel> {
    M5RuntimeBoundaryRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> RuntimeCopyExportParity {
    RuntimeCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5RuntimeClaimDimension,
    state: M5RuntimeConditionState,
) -> RuntimeClaimConditionEntry {
    RuntimeClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — release proof and
/// support/export replay — so the narrowed state always reaches field triage.
fn base_consumers(extra: &[M5ShellConsumerSurface]) -> Vec<M5ShellConsumerSurface> {
    let mut out = vec![
        M5ShellConsumerSurface::ReleaseProof,
        M5ShellConsumerSurface::SupportExport,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full
/// parity) row keeps full label and summary parity on the narrower surfaces; a
/// narrowed row discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: RuntimeNarrowingDisclosureState,
) -> Vec<RuntimeRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        RuntimeRenderingNarrowingDisclosure {
            rendering_surface: M5RuntimeRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        RuntimeRenderingNarrowingDisclosure {
            rendering_surface: M5RuntimeRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full
/// label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<RuntimeRenderingNarrowingDisclosure> {
    surface_disclosures(labels, RuntimeNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their
/// reduced interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<RuntimeRenderingNarrowingDisclosure> {
    surface_disclosures(labels, RuntimeNarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5RuntimeRenderingSurface> {
    vec![
        M5RuntimeRenderingSurface::DesktopFull,
        M5RuntimeRenderingSurface::CliHeadless,
        M5RuntimeRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<RuntimeAccessibilityRow> {
    vec![
        // Remote target pill — host identity verified and the connection is live; the
        // pill offers a fully live, authoritative remote target and is reachable on
        // every surface (green).
        RuntimeAccessibilityRow {
            record_kind: RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:remote-target-pill".to_owned(),
            component_family: M5RuntimeBoundaryComponentFamily::RemoteTargetPill,
            source_family_schema_ref: RUNTIME_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            runtime_context_ref: "target:ssh:0001".to_owned(),
            fallback_modalities: vec![
                M5RuntimeFallbackModality::List,
                M5RuntimeFallbackModality::Textual,
                M5RuntimeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            cli_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            export_summary: RuntimeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:remote-target-pill:a11y".to_owned(),
            copy_export: copy_export(&[
                "host_boundary",
                "connection_state",
                "target_identity",
                "runtime_source",
            ]),
            full_support_claim: M5RuntimeSupportClaim::Live,
            claim_conditions: vec![condition(
                M5RuntimeClaimDimension::HostIdentity,
                M5RuntimeConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "host_boundary",
                "connection_state",
                "target_identity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellConsumerSurface::ShellFrame,
                M5ShellConsumerSurface::StatusBar,
            ]),
            source_refs: vec![
                "UI/UX Spec execution-context strip".to_owned(),
                RUNTIME_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("remote-target-pill"),
        },
        // Environment status strip — context precedence fully resolved; the strip
        // reports a ready, self-sufficient winning runtime with no unshadowed conflict
        // (green).
        RuntimeAccessibilityRow {
            record_kind: RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:environment-status-strip".to_owned(),
            component_family: M5RuntimeBoundaryComponentFamily::EnvironmentStatusStrip,
            source_family_schema_ref: RUNTIME_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            runtime_context_ref: "runtime:env:0002".to_owned(),
            fallback_modalities: vec![
                M5RuntimeFallbackModality::List,
                M5RuntimeFallbackModality::Textual,
                M5RuntimeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            cli_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            export_summary: RuntimeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:environment-status-strip:a11y".to_owned(),
            copy_export: copy_export(&[
                "runtime_source",
                "winning_scope",
                "readiness",
                "why_context",
            ]),
            full_support_claim: M5RuntimeSupportClaim::Ready,
            claim_conditions: vec![condition(
                M5RuntimeClaimDimension::ContextPrecedence,
                M5RuntimeConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["runtime_source", "winning_scope", "readiness"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellConsumerSurface::Layout,
                M5ShellConsumerSurface::DocsHelp,
            ]),
            source_refs: vec![
                "UX Guide §16.62".to_owned(),
                RUNTIME_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("environment-status-strip"),
        },
        // Terminal tab — the tab is a restored transcript, not a live PTY, and its
        // shell-integration quality is unknown, so the claim auto-narrows to restored:
        // the tab stays labeled but no longer reads as a live session (yellow).
        RuntimeAccessibilityRow {
            record_kind: RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:terminal-tab".to_owned(),
            component_family: M5RuntimeBoundaryComponentFamily::TerminalTab,
            source_family_schema_ref: RUNTIME_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            runtime_context_ref: "session:terminal:0003".to_owned(),
            fallback_modalities: vec![
                M5RuntimeFallbackModality::List,
                M5RuntimeFallbackModality::Textual,
                M5RuntimeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            cli_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            export_summary: RuntimeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:terminal-tab:a11y".to_owned(),
            copy_export: copy_export(&[
                "session_title",
                "host_boundary",
                "shell_integration",
                "liveness",
            ]),
            full_support_claim: M5RuntimeSupportClaim::Live,
            claim_conditions: vec![condition(
                M5RuntimeClaimDimension::ShellIntegrationConfidence,
                M5RuntimeConditionState::Restored,
            )],
            claim_narrow: Some(RuntimeClaimAutoNarrow {
                narrowed_to: M5RuntimeSupportClaim::Restored,
                binding_dimension: M5RuntimeClaimDimension::ShellIntegrationConfidence,
                trigger: M5RuntimeBoundaryDowngradeTrigger::ShellIntegrationQualityHidden,
                narrowed_label:
                    "Restored transcript — shell integration unknown, not a live session"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "session_title",
                "host_boundary",
                "liveness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellConsumerSurface::ShellFrame,
                M5ShellConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec terminal UX".to_owned(),
                RUNTIME_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("terminal-tab"),
        },
        // Toolchain pin row — hierarchy-heavy (precedence inspector with ordered
        // shadowed layers); context precedence is only partially resolved (a workspace
        // pin still shadows a durable one), so the winning-toolchain claim auto-narrows
        // to degraded and binds the inspector tree to a flat list / textual path
        // (yellow).
        RuntimeAccessibilityRow {
            record_kind: RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:toolchain-pin-row".to_owned(),
            component_family: M5RuntimeBoundaryComponentFamily::ToolchainPinRow,
            source_family_schema_ref: RUNTIME_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            runtime_context_ref: "toolchain:pin:0004".to_owned(),
            fallback_modalities: vec![
                M5RuntimeFallbackModality::Structured,
                M5RuntimeFallbackModality::List,
                M5RuntimeFallbackModality::Textual,
                M5RuntimeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: RuntimeNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            export_summary: RuntimeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:toolchain-pin-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "winning_scope",
                "shadowed_layers",
                "pin_state",
                "switch_review",
            ]),
            full_support_claim: M5RuntimeSupportClaim::Ready,
            claim_conditions: vec![condition(
                M5RuntimeClaimDimension::ContextPrecedence,
                M5RuntimeConditionState::Partial,
            )],
            claim_narrow: Some(RuntimeClaimAutoNarrow {
                narrowed_to: M5RuntimeSupportClaim::Degraded,
                binding_dimension: M5RuntimeClaimDimension::ContextPrecedence,
                trigger: M5RuntimeBoundaryDowngradeTrigger::RuntimeSourceUnexplained,
                narrowed_label:
                    "Precedence partially resolved — winning toolchain shown degraded until the shadowing conflict clears"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "winning_scope",
                "shadowed_layers",
                "pin_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellConsumerSurface::ProductUi,
                M5ShellConsumerSurface::Windowing,
            ]),
            source_refs: vec![
                "UX Guide §16.62".to_owned(),
                RUNTIME_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("toolchain-pin-row"),
        },
        // Presence avatar stack — the collaboration link dropped and is reconnecting,
        // so the roles are shown from last-known state and the claim auto-narrows to
        // reconnecting rather than reading as a live shared session (yellow).
        RuntimeAccessibilityRow {
            record_kind: RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:presence-avatar-stack".to_owned(),
            component_family: M5RuntimeBoundaryComponentFamily::PresenceAvatarStack,
            source_family_schema_ref: RUNTIME_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            runtime_context_ref: "session:collab:0005".to_owned(),
            fallback_modalities: vec![
                M5RuntimeFallbackModality::List,
                M5RuntimeFallbackModality::Textual,
                M5RuntimeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            cli_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            export_summary: RuntimeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:presence-avatar-stack:a11y".to_owned(),
            copy_export: copy_export(&[
                "participant_roles",
                "presenter",
                "follow_state",
                "continuity_posture",
            ]),
            full_support_claim: M5RuntimeSupportClaim::Live,
            claim_conditions: vec![condition(
                M5RuntimeClaimDimension::CollaborationRole,
                M5RuntimeConditionState::Reconnecting,
            )],
            claim_narrow: Some(RuntimeClaimAutoNarrow {
                narrowed_to: M5RuntimeSupportClaim::Reconnecting,
                binding_dimension: M5RuntimeClaimDimension::CollaborationRole,
                trigger: M5RuntimeBoundaryDowngradeTrigger::CollaborationRoleMasked,
                narrowed_label:
                    "Collaboration link reconnecting — roles shown from last-known, not live"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "participant_roles",
                "presenter",
                "follow_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellConsumerSurface::AttentionRouter,
                M5ShellConsumerSurface::NotificationEnvelope,
            ]),
            source_refs: vec![
                "UI/UX Spec collaboration/session-state".to_owned(),
                RUNTIME_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("presence-avatar-stack"),
        },
        // Repair action card — the repair requires policy approval and its
        // reversibility cannot be guaranteed until approved, so the card auto-narrows
        // to policy-blocked rather than presenting a ready "Fix now" (yellow).
        RuntimeAccessibilityRow {
            record_kind: RUNTIME_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:repair-action-card".to_owned(),
            component_family: M5RuntimeBoundaryComponentFamily::RepairActionCard,
            source_family_schema_ref: RUNTIME_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            runtime_context_ref: "repair:transaction:0006".to_owned(),
            fallback_modalities: vec![
                M5RuntimeFallbackModality::List,
                M5RuntimeFallbackModality::Textual,
                M5RuntimeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            cli_reach: RuntimeNonVisualReachState::ReachableAndLabeled,
            export_summary: RuntimeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:repair-action-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "repair_class",
                "blast_radius",
                "target_boundary",
                "reversal_class",
            ]),
            full_support_claim: M5RuntimeSupportClaim::Ready,
            claim_conditions: vec![condition(
                M5RuntimeClaimDimension::RepairReversibility,
                M5RuntimeConditionState::PolicyBlocked,
            )],
            claim_narrow: Some(RuntimeClaimAutoNarrow {
                narrowed_to: M5RuntimeSupportClaim::PolicyBlocked,
                binding_dimension: M5RuntimeClaimDimension::RepairReversibility,
                trigger: M5RuntimeBoundaryDowngradeTrigger::ReversibilityOverstated,
                narrowed_label:
                    "Repair blocked by policy — reversal not exact until an approver signs off"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "repair_class",
                "blast_radius",
                "reversal_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellConsumerSurface::DocsHelp,
                M5ShellConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "TDD repair transaction/preview/rollback".to_owned(),
                RUNTIME_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("repair-action-card"),
        },
    ]
}
