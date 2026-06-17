//! Canonical Support Center information architecture: the one layout contract that makes the Support
//! Center a coherent product home instead of a scatter of hidden pages and ad hoc entry points.
//!
//! Where the [`crate::m5_support_center_matrix`] packet binds each Support Center module to its
//! inspectors, data classes, redaction default, and export modes, this packet governs the *shell* the
//! user navigates: a three-region layout — a left-nav module rail, center diagnosis/recovery cards,
//! and one shared right-side build/policy/residency/export inspector that stays visible across every
//! module. It reuses the same module registry ([`SupportModule`]) so the desktop shell, CLI/headless
//! references, docs/help, and support exports all name the same supportability surfaces, and each
//! [`NavEntry`] defers per-module readiness to the matrix via [`NavEntry::matrix_row_ref`] rather than
//! restating it.
//!
//! The readiness analogue here is a fail-closed **presentation gate**. The Support Center's
//! accessibility invariants — keyboard-complete navigation ([`AccessibilityGuarantee::KeyboardComplete`]),
//! high-contrast parity ([`AccessibilityGuarantee::HighContrastParity`]), and reduced-motion-safe
//! transitions ([`AccessibilityGuarantee::ReducedMotionSafe`]) — are hard requirements: an entry that
//! cannot satisfy all three is **withheld**, never presented as an inaccessible surface. Each entry
//! also depends on a subset of the shared inspector's facets ([`InspectorFacet`]); a required facet
//! that is [`FacetAvailability::Degraded`] narrows the entry to a flagged, still-actionable surface,
//! and a required facet that is [`FacetAvailability::Unwired`] withholds it. The published
//! [`PresentationDecision`] is the weaker of those two ceilings, so an inaccessible surface or a
//! dropped inspector facet narrows or withholds the entry automatically rather than leaving it
//! navigable by inertia. The recorded decision, downgrade reasons, and recovery path are all
//! recomputed and validated against the gate so a narrowing can never be asserted or hidden by hand.
//!
//! Every required consumer surface — the desktop shell, CLI/headless output, docs/help, and the
//! support export — binds to this one registry via a [`LayoutConsumerBinding`] that must ingest it,
//! preserve its nav labels and shared inspector, and narrow with it, so a withheld module cannot stay
//! navigable on a downstream surface.
//!
//! The packet is checked in at `artifacts/support/m5/m5-support-center-ui.json` and embedded here. It
//! is metadata-only: every field is a typed state, a count, or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, live authority handles, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_support_center_matrix::SupportModule;

/// Supported Support Center layout schema version.
pub const M5_SUPPORT_CENTER_UI_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SUPPORT_CENTER_UI_RECORD_KIND: &str = "m5_support_center_ui";

/// Repo-relative path to the checked-in packet.
pub const M5_SUPPORT_CENTER_UI_PATH: &str = "artifacts/support/m5/m5-support-center-ui.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_SUPPORT_CENTER_UI_SCHEMA_REF: &str =
    "schemas/support/m5-support-center-layout.schema.json";

/// Repo-relative path to the companion document.
pub const M5_SUPPORT_CENTER_UI_DOC_REF: &str = "docs/help/support/m5-support-center-ui.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_SUPPORT_CENTER_UI_ARTIFACT_DOC_REF: &str =
    "artifacts/support/m5/m5-support-center-ui.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_SUPPORT_CENTER_UI_FIXTURE_DIR: &str = "fixtures/support/m5/m5-support-center-ui";

/// Repo-relative path to the shiproom review packet that renders this layout.
pub const M5_SUPPORT_CENTER_UI_REVIEW_PACKET_REF: &str =
    "artifacts/shiproom/m5-support-center-ui-review-packet/support_center_ui_review_packet.md";

/// Embedded checked-in packet JSON.
pub const M5_SUPPORT_CENTER_UI_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/m5/m5-support-center-ui.json"
));

/// One of the three layout regions the Support Center is composed of.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutRegion {
    /// The left-nav module rail listing every Support Center module.
    LeftNav,
    /// The center region rendering diagnosis, recovery, inspector, and intake/export cards.
    Center,
    /// The shared right-side build/policy/residency/export inspector.
    RightInspector,
}

impl LayoutRegion {
    /// Every layout region, in left-to-right reading order.
    pub const ALL: [Self; 3] = [Self::LeftNav, Self::Center, Self::RightInspector];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeftNav => "left_nav",
            Self::Center => "center",
            Self::RightInspector => "right_inspector",
        }
    }
}

/// A left-nav grouping that keeps related modules together without forking their vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavSection {
    /// Diagnosis entry: Project Doctor.
    Diagnose,
    /// Recovery actions: Safe mode and extension bisect.
    Recover,
    /// Read-mostly inspectors: performance, language, index, AI usage, crash, network, artifacts.
    Inspect,
    /// Issue-report / crash-intake and the support-bundle export preview.
    IntakeExport,
}

impl NavSection {
    /// Every nav section, in top-to-bottom rail order.
    pub const ALL: [Self; 4] = [
        Self::Diagnose,
        Self::Recover,
        Self::Inspect,
        Self::IntakeExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diagnose => "diagnose",
            Self::Recover => "recover",
            Self::Inspect => "inspect",
            Self::IntakeExport => "intake_export",
        }
    }
}

/// The kind of card the center region renders for a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CenterSurfaceKind {
    /// Probe-backed diagnosis cards with finding codes and guided-repair entry.
    DiagnosisCards,
    /// Bounded recovery actions.
    RecoveryActions,
    /// A read-only inspector readout.
    InspectorReadout,
    /// Issue-report / crash-intake routing and the export preview.
    IntakeAndExport,
}

impl CenterSurfaceKind {
    /// Every center-surface kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DiagnosisCards,
        Self::RecoveryActions,
        Self::InspectorReadout,
        Self::IntakeAndExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosisCards => "diagnosis_cards",
            Self::RecoveryActions => "recovery_actions",
            Self::InspectorReadout => "inspector_readout",
            Self::IntakeAndExport => "intake_and_export",
        }
    }
}

/// An existing source a center surface reuses rather than duplicating.
///
/// The Support Center renders finding codes, crash IDs, install/advisory rows, and schema-registry
/// state from their owning lanes; an entry names which of these it reuses so the IA never mints a
/// second copy of supportability truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationSource {
    /// Project Doctor finding codes.
    FindingCodes,
    /// Crash store / incident-trail crash IDs.
    CrashIds,
    /// Install and advisory rows.
    InstallAdvisoryRows,
    /// Schema-registry state.
    SchemaRegistryState,
}

impl IntegrationSource {
    /// Every integration source, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FindingCodes,
        Self::CrashIds,
        Self::InstallAdvisoryRows,
        Self::SchemaRegistryState,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FindingCodes => "finding_codes",
            Self::CrashIds => "crash_ids",
            Self::InstallAdvisoryRows => "install_advisory_rows",
            Self::SchemaRegistryState => "schema_registry_state",
        }
    }
}

/// A facet of the one shared right-side inspector.
///
/// These are the four truth panes the spec names — build, policy, residency, and export — that stay
/// visible across every module so the user never loses the execution context that explains a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorFacet {
    /// Build facet: exact-build identity and release channel.
    Build,
    /// Policy facet: which config/policy layer won and what it shadowed.
    Policy,
    /// Residency facet: where the module's data is retained and under which residency rule.
    Residency,
    /// Export facet: redaction manifest and export-consent posture.
    Export,
}

impl InspectorFacet {
    /// Every inspector facet, in declaration order.
    pub const ALL: [Self; 4] = [Self::Build, Self::Policy, Self::Residency, Self::Export];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Policy => "policy",
            Self::Residency => "residency",
            Self::Export => "export",
        }
    }
}

/// How available a shared-inspector facet is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FacetAvailability {
    /// The facet is wired and current.
    Wired,
    /// The facet is wired but degraded; caps entries that require it at narrowed.
    Degraded,
    /// The facet is not wired; caps entries that require it at withheld.
    Unwired,
}

impl FacetAvailability {
    /// Every availability state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Wired, Self::Degraded, Self::Unwired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wired => "wired",
            Self::Degraded => "degraded",
            Self::Unwired => "unwired",
        }
    }

    /// Highest presentation this availability permits an entry that requires the facet.
    pub const fn presentation_ceiling(self) -> PresentationDecision {
        match self {
            Self::Wired => PresentationDecision::Presented,
            Self::Degraded => PresentationDecision::Narrowed,
            Self::Unwired => PresentationDecision::Withheld,
        }
    }
}

/// An accessibility invariant the Support Center must hold across every region and entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityGuarantee {
    /// Every action and navigation step is reachable by keyboard alone.
    KeyboardComplete,
    /// The surface renders at full fidelity in high-contrast mode.
    HighContrastParity,
    /// Transitions respect the reduced-motion preference.
    ReducedMotionSafe,
}

impl AccessibilityGuarantee {
    /// Every accessibility guarantee, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::KeyboardComplete,
        Self::HighContrastParity,
        Self::ReducedMotionSafe,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardComplete => "keyboard_complete",
            Self::HighContrastParity => "high_contrast_parity",
            Self::ReducedMotionSafe => "reduced_motion_safe",
        }
    }
}

/// Whether a set of accessibility guarantees covers every required invariant.
fn accessibility_complete(guarantees: &[AccessibilityGuarantee]) -> bool {
    AccessibilityGuarantee::ALL
        .iter()
        .all(|g| guarantees.contains(g))
}

/// The presentation the gate publishes for a nav entry, highest to lowest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationDecision {
    /// The entry is presented and fully navigable.
    Presented,
    /// The entry is presented but flagged and read-mostly; some actions are limited.
    Narrowed,
    /// The entry is withheld from the Support Center.
    Withheld,
}

impl PresentationDecision {
    /// Every presentation decision, highest to lowest.
    pub const ALL: [Self; 3] = [Self::Presented, Self::Narrowed, Self::Withheld];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presented => "presented",
            Self::Narrowed => "narrowed",
            Self::Withheld => "withheld",
        }
    }

    /// Rank for the fail-closed gate; higher is more capable.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Presented => 2,
            Self::Narrowed => 1,
            Self::Withheld => 0,
        }
    }

    /// Whether the gate narrowed or withheld the entry below a clean presentation.
    pub const fn requires_recovery(self) -> bool {
        !matches!(self, Self::Presented)
    }
}

/// The weaker (lower-rank) of two presentation decisions.
fn weaker(a: PresentationDecision, b: PresentationDecision) -> PresentationDecision {
    if b.rank() < a.rank() {
        b
    } else {
        a
    }
}

/// A headline reason the presentation gate narrows or withholds an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationDowngradeReason {
    /// The entry does not satisfy every accessibility invariant.
    AccessibilityUnmet,
    /// A required shared-inspector facet is degraded.
    InspectorFacetDegraded,
    /// A required shared-inspector facet is unwired.
    InspectorFacetUnwired,
}

impl PresentationDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::AccessibilityUnmet,
        Self::InspectorFacetDegraded,
        Self::InspectorFacetUnwired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccessibilityUnmet => "accessibility_unmet",
            Self::InspectorFacetDegraded => "inspector_facet_degraded",
            Self::InspectorFacetUnwired => "inspector_facet_unwired",
        }
    }
}

/// The recovery path surfaced when an entry is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationRecoveryPath {
    /// Restore the missing accessibility invariant.
    RestoreAccessibility,
    /// Wire or restore the degraded/unwired shared-inspector facet.
    RestoreInspectorFacet,
    /// No recovery is needed; only valid when the entry presents cleanly.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl PresentationRecoveryPath {
    /// Every recovery path, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RestoreAccessibility,
        Self::RestoreInspectorFacet,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreAccessibility => "restore_accessibility",
            Self::RestoreInspectorFacet => "restore_inspector_facet",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the owner can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A downstream surface that must ingest this registry and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutConsumerSurface {
    /// The desktop shell's Support Center pages.
    DesktopShell,
    /// CLI / headless support references.
    CliHeadless,
    /// Docs/help articles describing the Support Center.
    DocsHelp,
    /// Support export of the Support Center layout.
    SupportExport,
}

impl LayoutConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::DesktopShell,
        Self::CliHeadless,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopShell => "desktop_shell",
            Self::CliHeadless => "cli_headless",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// One layout region descriptor, with the accessibility invariants the chrome must satisfy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegionDescriptor {
    /// Layout region this descriptor governs.
    pub region: LayoutRegion,
    /// Reviewer-facing description of the region's role.
    pub role: String,
    /// Tab order of the region in keyboard-complete navigation; unique across regions.
    pub keyboard_order: u32,
    /// Accessibility guarantees the region's chrome satisfies; must cover every invariant.
    #[serde(default)]
    pub accessibility: Vec<AccessibilityGuarantee>,
}

impl RegionDescriptor {
    /// Whether the region's chrome satisfies every accessibility invariant.
    pub fn is_accessible(&self) -> bool {
        accessibility_complete(&self.accessibility)
    }
}

/// One facet binding on the shared inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InspectorFacetBinding {
    /// Facet this binding wires.
    pub facet: InspectorFacet,
    /// How available the facet is.
    pub availability: FacetAvailability,
    /// Ref to the canonical truth source this facet projects.
    pub descriptor_ref: String,
    /// Capture timestamp for the availability check.
    pub checked_at: String,
}

impl InspectorFacetBinding {
    /// Whether the binding carries the non-empty descriptor ref and timestamp it requires.
    pub fn is_well_formed(&self) -> bool {
        !self.descriptor_ref.trim().is_empty() && !self.checked_at.trim().is_empty()
    }
}

/// The one shared right-side inspector, kept visible across every module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedInspector {
    /// Reviewer-facing role of the inspector region.
    pub role: String,
    /// True when the inspector persists across module switches rather than re-mounting per module.
    pub persists_across_modules: bool,
    /// Facet bindings; must cover every [`InspectorFacet`] exactly once.
    #[serde(default)]
    pub facets: Vec<InspectorFacetBinding>,
}

impl SharedInspector {
    /// Returns the binding for the given facet, if present.
    pub fn facet(&self, facet: InspectorFacet) -> Option<&InspectorFacetBinding> {
        self.facets.iter().find(|b| b.facet == facet)
    }

    /// The availability of the given facet, treating an undeclared facet as unwired.
    pub fn facet_availability(&self, facet: InspectorFacet) -> FacetAvailability {
        self.facet(facet)
            .map(|b| b.availability)
            .unwrap_or(FacetAvailability::Unwired)
    }

    /// Whether the inspector declares every facet exactly once and each binding is well-formed.
    pub fn declares_all_facets(&self) -> bool {
        InspectorFacet::ALL
            .iter()
            .all(|facet| self.facets.iter().filter(|b| b.facet == *facet).count() == 1)
            && self
                .facets
                .iter()
                .all(InspectorFacetBinding::is_well_formed)
    }
}

/// One left-nav module entry in the Support Center registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavEntry {
    /// Stable nav-entry id.
    pub entry_id: String,
    /// Support Center module this entry surfaces; reuses the one module registry.
    pub module: SupportModule,
    /// Human-readable left-nav label.
    pub nav_label: String,
    /// Order of the entry in the left-nav rail; unique across entries.
    pub nav_order: u32,
    /// Left-nav section this entry groups under.
    pub section: NavSection,
    /// Kind of card the center region renders for this module.
    pub center_surface_kind: CenterSurfaceKind,
    /// Ref to the center surface implementation.
    pub center_surface_ref: String,
    /// Shared-inspector facets this entry's center surface depends on; at least one.
    #[serde(default)]
    pub required_facets: Vec<InspectorFacet>,
    /// Existing sources this entry reuses rather than duplicating; at least one.
    #[serde(default)]
    pub integration_sources: Vec<IntegrationSource>,
    /// Accessibility guarantees this entry's surface satisfies.
    #[serde(default)]
    pub accessibility: Vec<AccessibilityGuarantee>,
    /// Ref to the matrix row this entry defers per-module readiness to.
    pub matrix_row_ref: String,
    /// Presentation actually published after the gate; must equal the recomputed decision.
    pub presentation: PresentationDecision,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<PresentationDowngradeReason>,
    /// Recovery path surfaced when the entry is narrowed or withheld.
    pub recovery_path: PresentationRecoveryPath,
    /// Actions the entry still offers; empty when withheld.
    #[serde(default)]
    pub offered_actions: Vec<String>,
    /// Caveats attached to the presented entry.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Accessibility invariants unmet or inspector facets unwired/degraded for this entry.
    #[serde(default)]
    pub unmet_or_unwired_fields: Vec<String>,
    /// Ref to the conformance suite backing the entry.
    pub conformance_ref: String,
    /// Ref to the entry's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the layout answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Ref to the machine-readable layout receipt.
    pub layout_receipt_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl NavEntry {
    /// Whether the entry's surface satisfies every accessibility invariant.
    pub fn accessibility_complete(&self) -> bool {
        accessibility_complete(&self.accessibility)
    }

    /// Highest presentation the accessibility invariants permit.
    pub fn accessibility_ceiling(&self) -> PresentationDecision {
        if self.accessibility_complete() {
            PresentationDecision::Presented
        } else {
            PresentationDecision::Withheld
        }
    }

    /// Highest presentation the required inspector facets permit, the weakest across every facet.
    pub fn facet_ceiling(&self, inspector: &SharedInspector) -> PresentationDecision {
        let mut ceiling = PresentationDecision::Presented;
        for facet in &self.required_facets {
            ceiling = weaker(
                ceiling,
                inspector.facet_availability(*facet).presentation_ceiling(),
            );
        }
        ceiling
    }

    /// The presentation the gate permits this entry to publish.
    ///
    /// Lowers the clean baseline to the weaker of the accessibility ceiling and the required-facet
    /// ceiling, so an inaccessible surface or a dropped inspector facet can never present a fuller
    /// claim than the inputs support.
    pub fn effective_presentation(&self, inspector: &SharedInspector) -> PresentationDecision {
        weaker(self.accessibility_ceiling(), self.facet_ceiling(inspector))
    }

    /// Whether any required facet is degraded.
    pub fn has_degraded_facet(&self, inspector: &SharedInspector) -> bool {
        self.required_facets
            .iter()
            .any(|f| inspector.facet_availability(*f) == FacetAvailability::Degraded)
    }

    /// Whether any required facet is unwired.
    pub fn has_unwired_facet(&self, inspector: &SharedInspector) -> bool {
        self.required_facets
            .iter()
            .any(|f| inspector.facet_availability(*f) == FacetAvailability::Unwired)
    }

    /// The headline downgrade reasons recomputed from the entry's observed states.
    pub fn computed_downgrade_reasons(
        &self,
        inspector: &SharedInspector,
    ) -> Vec<PresentationDowngradeReason> {
        let mut reasons = Vec::new();
        if !self.accessibility_complete() {
            reasons.push(PresentationDowngradeReason::AccessibilityUnmet);
        }
        if self.has_degraded_facet(inspector) {
            reasons.push(PresentationDowngradeReason::InspectorFacetDegraded);
        }
        if self.has_unwired_facet(inspector) {
            reasons.push(PresentationDowngradeReason::InspectorFacetUnwired);
        }
        reasons
    }

    /// The recovery path the gate must record, derived from the entry's observed states.
    ///
    /// Accessibility is the harder invariant, so an inaccessible surface points at an accessibility
    /// restore before an inspector-facet restore.
    pub fn computed_recovery_path(&self, inspector: &SharedInspector) -> PresentationRecoveryPath {
        if !self.accessibility_complete() {
            PresentationRecoveryPath::RestoreAccessibility
        } else if self.facet_ceiling(inspector) != PresentationDecision::Presented {
            PresentationRecoveryPath::RestoreInspectorFacet
        } else {
            PresentationRecoveryPath::NoneNeeded
        }
    }

    /// Whether the entry presents cleanly with nothing narrowing it.
    pub fn is_presented(&self, inspector: &SharedInspector) -> bool {
        self.effective_presentation(inspector) == PresentationDecision::Presented
    }

    /// Whether the entry carries its own non-empty conformance, evidence, scope, and receipt refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
            && !self.layout_receipt_ref.trim().is_empty()
            && !self.matrix_row_ref.trim().is_empty()
    }

    /// Whether the recorded decision, reasons, and path all agree with the gate.
    pub fn gate_consistent(&self, inspector: &SharedInspector) -> bool {
        self.presentation == self.effective_presentation(inspector)
            && self.downgrade_reasons == self.computed_downgrade_reasons(inspector)
            && self.recovery_path == self.computed_recovery_path(inspector)
    }
}

/// One binding wiring a downstream surface to this registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: LayoutConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Layout packet id this surface ingests.
    pub layout_packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface ingests this registry rather than a parallel list.
    pub ingests_registry: bool,
    /// True when the surface preserves the nav labels and sections verbatim.
    pub preserves_nav_labels: bool,
    /// True when the surface preserves the shared inspector rather than forking it.
    pub preserves_shared_inspector: bool,
    /// True when the surface narrows automatically as entries are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl LayoutConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.layout_packet_id_ref == packet_id
            && self.ingests_registry
            && self.preserves_nav_labels
            && self.preserves_shared_inspector
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SupportCenterLayoutSummary {
    /// Total nav entries.
    pub total_modules: usize,
    /// Entries presented cleanly.
    pub presented_modules: usize,
    /// Entries the gate narrowed.
    pub narrowed_modules: usize,
    /// Entries the gate withheld.
    pub withheld_modules: usize,
    /// Entries carrying at least one downgrade reason.
    pub modules_with_downgrade_reasons: usize,
    /// Entries that do not satisfy every accessibility invariant.
    pub accessibility_gap_modules: usize,
    /// Entries that require at least one degraded or unwired facet.
    pub modules_with_imperfect_facets: usize,
    /// Distinct nav sections used.
    pub nav_sections_used: usize,
    /// Distinct integration sources reused.
    pub integration_sources_used: usize,
}

/// A redaction-safe export row projected from a nav entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportCenterLayoutExportRow {
    /// Nav-entry id.
    pub entry_id: String,
    /// Module token.
    pub module: String,
    /// Left-nav label.
    pub nav_label: String,
    /// Nav-section token.
    pub section: String,
    /// Center-surface-kind token.
    pub center_surface_kind: String,
    /// Required-facet tokens.
    pub required_facets: Vec<String>,
    /// Integration-source tokens.
    pub integration_sources: Vec<String>,
    /// Accessibility-guarantee tokens the entry satisfies.
    pub accessibility: Vec<String>,
    /// Matrix-row ref this entry defers readiness to.
    pub matrix_row_ref: String,
    /// Published-presentation token.
    pub presentation: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Recovery-path token.
    pub recovery_path: String,
    /// Actions the entry still offers.
    pub offered_actions: Vec<String>,
    /// Caveats attached to the entry.
    pub caveats: Vec<String>,
    /// Accessibility invariants unmet or facets unwired/degraded.
    pub unmet_or_unwired_fields: Vec<String>,
    /// Scope snapshot the layout answered.
    pub scope_snapshot_ref: String,
    /// Layout-receipt ref.
    pub layout_receipt_ref: String,
    /// Whether the entry presents cleanly.
    pub presented: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the layout — the canonical Support Center index downstream
/// surfaces render instead of restating each module's navigation by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportCenterLayoutExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5SupportCenterLayoutExportRow>,
    /// Whether every entry's published presentation and decision agree with the gate.
    pub all_entries_gate_consistent: bool,
    /// Entries presented cleanly.
    pub presented_count: usize,
    /// Entries the gate narrowed.
    pub narrowed_count: usize,
    /// Entries the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed Support Center layout packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SupportCenterLayout {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed Support Center module vocabulary.
    pub modules: Vec<SupportModule>,
    /// Closed layout-region vocabulary.
    pub layout_regions: Vec<LayoutRegion>,
    /// Closed nav-section vocabulary.
    pub nav_sections: Vec<NavSection>,
    /// Closed center-surface-kind vocabulary.
    pub center_surface_kinds: Vec<CenterSurfaceKind>,
    /// Closed integration-source vocabulary.
    pub integration_sources: Vec<IntegrationSource>,
    /// Closed inspector-facet vocabulary.
    pub inspector_facets: Vec<InspectorFacet>,
    /// Closed facet-availability vocabulary.
    pub facet_availabilities: Vec<FacetAvailability>,
    /// Closed accessibility-guarantee vocabulary.
    pub accessibility_guarantees: Vec<AccessibilityGuarantee>,
    /// Closed presentation vocabulary.
    pub presentations: Vec<PresentationDecision>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<PresentationDowngradeReason>,
    /// Closed recovery-path vocabulary.
    pub recovery_paths: Vec<PresentationRecoveryPath>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<LayoutConsumerSurface>,
    /// Region descriptors, one per layout region.
    #[serde(default)]
    pub regions: Vec<RegionDescriptor>,
    /// The one shared right-side inspector.
    pub shared_inspector: SharedInspector,
    /// Nav entries, one per module.
    #[serde(default)]
    pub nav_entries: Vec<NavEntry>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<LayoutConsumerBinding>,
    /// Summary counts.
    pub summary: M5SupportCenterLayoutSummary,
}

impl M5SupportCenterLayout {
    /// Returns the nav entry for the given module.
    pub fn entry_for(&self, module: SupportModule) -> Option<&NavEntry> {
        self.nav_entries.iter().find(|e| e.module == module)
    }

    /// Returns the nav entry with the given id.
    pub fn entry(&self, entry_id: &str) -> Option<&NavEntry> {
        self.nav_entries.iter().find(|e| e.entry_id == entry_id)
    }

    /// Returns the descriptor for the given region.
    pub fn region_for(&self, region: LayoutRegion) -> Option<&RegionDescriptor> {
        self.regions.iter().find(|r| r.region == region)
    }

    /// Entries presented cleanly.
    pub fn presented_entries(&self) -> impl Iterator<Item = &NavEntry> {
        self.nav_entries.iter().filter(|e| {
            e.effective_presentation(&self.shared_inspector) == PresentationDecision::Presented
        })
    }

    /// Entries the gate narrowed.
    pub fn narrowed_entries(&self) -> impl Iterator<Item = &NavEntry> {
        self.nav_entries.iter().filter(|e| {
            e.effective_presentation(&self.shared_inspector) == PresentationDecision::Narrowed
        })
    }

    /// Entries the gate withheld entirely.
    pub fn withheld_entries(&self) -> impl Iterator<Item = &NavEntry> {
        self.nav_entries.iter().filter(|e| {
            e.effective_presentation(&self.shared_inspector) == PresentationDecision::Withheld
        })
    }

    /// Whether a consumer binding preserves this registry for the given surface.
    pub fn has_binding_for(&self, surface: LayoutConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every entry's recorded presentation, reasons, and path agree with the gate.
    pub fn all_entries_gate_consistent(&self) -> bool {
        self.nav_entries
            .iter()
            .all(|e| e.gate_consistent(&self.shared_inspector))
    }

    /// Recomputes the summary block from the entries.
    pub fn computed_summary(&self) -> M5SupportCenterLayoutSummary {
        let inspector = &self.shared_inspector;
        let count_presentation = |decision: PresentationDecision| {
            self.nav_entries
                .iter()
                .filter(|e| e.effective_presentation(inspector) == decision)
                .count()
        };
        let mut sections = BTreeSet::new();
        let mut sources = BTreeSet::new();
        for entry in &self.nav_entries {
            sections.insert(entry.section);
            for source in &entry.integration_sources {
                sources.insert(*source);
            }
        }
        M5SupportCenterLayoutSummary {
            total_modules: self.nav_entries.len(),
            presented_modules: count_presentation(PresentationDecision::Presented),
            narrowed_modules: count_presentation(PresentationDecision::Narrowed),
            withheld_modules: count_presentation(PresentationDecision::Withheld),
            modules_with_downgrade_reasons: self
                .nav_entries
                .iter()
                .filter(|e| !e.downgrade_reasons.is_empty())
                .count(),
            accessibility_gap_modules: self
                .nav_entries
                .iter()
                .filter(|e| !e.accessibility_complete())
                .count(),
            modules_with_imperfect_facets: self
                .nav_entries
                .iter()
                .filter(|e| e.has_degraded_facet(inspector) || e.has_unwired_facet(inspector))
                .count(),
            nav_sections_used: sections.len(),
            integration_sources_used: sources.len(),
        }
    }

    /// Produces the Support Center index downstream surfaces render instead of restating each
    /// module's navigation by hand.
    pub fn export_projection(&self) -> M5SupportCenterLayoutExportProjection {
        let inspector = &self.shared_inspector;
        let rows = self
            .nav_entries
            .iter()
            .map(|e| M5SupportCenterLayoutExportRow {
                entry_id: e.entry_id.clone(),
                module: e.module.as_str().to_owned(),
                nav_label: e.nav_label.clone(),
                section: e.section.as_str().to_owned(),
                center_surface_kind: e.center_surface_kind.as_str().to_owned(),
                required_facets: e
                    .required_facets
                    .iter()
                    .map(|f| f.as_str().to_owned())
                    .collect(),
                integration_sources: e
                    .integration_sources
                    .iter()
                    .map(|s| s.as_str().to_owned())
                    .collect(),
                accessibility: e
                    .accessibility
                    .iter()
                    .map(|a| a.as_str().to_owned())
                    .collect(),
                matrix_row_ref: e.matrix_row_ref.clone(),
                presentation: e.presentation.as_str().to_owned(),
                downgrade_reasons: e
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                recovery_path: e.recovery_path.as_str().to_owned(),
                offered_actions: e.offered_actions.clone(),
                caveats: e.caveats.clone(),
                unmet_or_unwired_fields: e.unmet_or_unwired_fields.clone(),
                scope_snapshot_ref: e.scope_snapshot_ref.clone(),
                layout_receipt_ref: e.layout_receipt_ref.clone(),
                presented: e.is_presented(inspector),
                summary: format!(
                    "{}: section {}, surface {}, presentation {}, recovery {}",
                    e.module.as_str(),
                    e.section.as_str(),
                    e.center_surface_kind.as_str(),
                    e.presentation.as_str(),
                    e.recovery_path.as_str()
                ),
            })
            .collect();
        M5SupportCenterLayoutExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_entries_gate_consistent: self.all_entries_gate_consistent(),
            presented_count: self.presented_entries().count(),
            narrowed_count: self.narrowed_entries().count(),
            withheld_count: self.withheld_entries().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact Support Center layout.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5SupportCenterLayoutSupportExport {
        M5SupportCenterLayoutSupportExport {
            record_kind: M5_SUPPORT_CENTER_UI_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORT_CENTER_UI_SCHEMA_VERSION,
            export_id: export_id.into(),
            layout_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            layout: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5SupportCenterLayoutViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_regions(&mut violations);
        self.validate_shared_inspector(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut seen_orders = BTreeSet::new();
        let mut covered_modules = BTreeSet::new();
        for entry in &self.nav_entries {
            if !seen_ids.insert(entry.entry_id.clone()) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateNavEntry {
                    entry_id: entry.entry_id.clone(),
                });
            }
            if !seen_orders.insert(entry.nav_order) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateNavOrder {
                    nav_order: entry.nav_order,
                });
            }
            if !covered_modules.insert(entry.module) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateModule {
                    module: entry.module.as_str(),
                });
            }
            self.validate_entry(entry, &mut violations);
        }

        // Every Support Center module must carry exactly one nav entry, so the registry the desktop
        // shell, CLI/headless, docs/help, and support export all read is the same one and complete.
        for module in SupportModule::ALL {
            if !covered_modules.contains(&module) {
                violations.push(M5SupportCenterLayoutViolation::MissingModule {
                    module: module.as_str(),
                });
            }
        }

        for surface in LayoutConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5SupportCenterLayoutViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5SupportCenterLayoutViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5SupportCenterLayoutViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5SupportCenterLayoutViolation>) {
        if self.schema_version != M5_SUPPORT_CENTER_UI_SCHEMA_VERSION {
            violations.push(M5SupportCenterLayoutViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_SUPPORT_CENTER_UI_RECORD_KIND {
            violations.push(M5SupportCenterLayoutViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SupportCenterLayoutViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("modules", self.modules == SupportModule::ALL.to_vec()),
            (
                "layout_regions",
                self.layout_regions == LayoutRegion::ALL.to_vec(),
            ),
            (
                "nav_sections",
                self.nav_sections == NavSection::ALL.to_vec(),
            ),
            (
                "center_surface_kinds",
                self.center_surface_kinds == CenterSurfaceKind::ALL.to_vec(),
            ),
            (
                "integration_sources",
                self.integration_sources == IntegrationSource::ALL.to_vec(),
            ),
            (
                "inspector_facets",
                self.inspector_facets == InspectorFacet::ALL.to_vec(),
            ),
            (
                "facet_availabilities",
                self.facet_availabilities == FacetAvailability::ALL.to_vec(),
            ),
            (
                "accessibility_guarantees",
                self.accessibility_guarantees == AccessibilityGuarantee::ALL.to_vec(),
            ),
            (
                "presentations",
                self.presentations == PresentationDecision::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == PresentationDowngradeReason::ALL.to_vec(),
            ),
            (
                "recovery_paths",
                self.recovery_paths == PresentationRecoveryPath::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == LayoutConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5SupportCenterLayoutViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_regions(&self, violations: &mut Vec<M5SupportCenterLayoutViolation>) {
        let mut covered = BTreeSet::new();
        let mut seen_orders = BTreeSet::new();
        for region in &self.regions {
            if !covered.insert(region.region) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateRegion {
                    region: region.region.as_str(),
                });
            }
            if !seen_orders.insert(region.keyboard_order) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateKeyboardOrder {
                    keyboard_order: region.keyboard_order,
                });
            }
            if region.role.trim().is_empty() {
                violations.push(M5SupportCenterLayoutViolation::EmptyField {
                    id: region.region.as_str().to_owned(),
                    field_name: "role",
                });
            }
            // Every region's chrome must satisfy every accessibility invariant: the Support Center
            // never ships an inaccessible rail, card stack, or inspector.
            if !region.is_accessible() {
                violations.push(M5SupportCenterLayoutViolation::RegionNotAccessible {
                    region: region.region.as_str(),
                });
            }
        }
        for region in LayoutRegion::ALL {
            if !covered.contains(&region) {
                violations.push(M5SupportCenterLayoutViolation::MissingRegion {
                    region: region.as_str(),
                });
            }
        }
    }

    fn validate_shared_inspector(&self, violations: &mut Vec<M5SupportCenterLayoutViolation>) {
        let inspector = &self.shared_inspector;
        if inspector.role.trim().is_empty() {
            violations.push(M5SupportCenterLayoutViolation::EmptyField {
                id: "<shared_inspector>".to_owned(),
                field_name: "role",
            });
        }
        // The inspector is shared: it persists across module switches rather than re-mounting per
        // module, so build/policy/residency/export truth stays visible as the user moves around.
        if !inspector.persists_across_modules {
            violations.push(M5SupportCenterLayoutViolation::InspectorNotShared);
        }
        let mut seen = BTreeSet::new();
        for binding in &inspector.facets {
            if !seen.insert(binding.facet) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateInspectorFacet {
                    facet: binding.facet.as_str(),
                });
            }
            if !binding.is_well_formed() {
                violations.push(
                    M5SupportCenterLayoutViolation::InspectorFacetBindingIncomplete {
                        facet: binding.facet.as_str(),
                    },
                );
            }
        }
        // The shared inspector must always declare all four facets so build, policy, residency, and
        // export truth are each a visible pane, even when a facet is degraded or unwired.
        for facet in InspectorFacet::ALL {
            if !seen.contains(&facet) {
                violations.push(M5SupportCenterLayoutViolation::MissingInspectorFacet {
                    facet: facet.as_str(),
                });
            }
        }
    }

    fn validate_entry(
        &self,
        entry: &NavEntry,
        violations: &mut Vec<M5SupportCenterLayoutViolation>,
    ) {
        let inspector = &self.shared_inspector;
        for (field, value) in [
            ("entry_id", &entry.entry_id),
            ("nav_label", &entry.nav_label),
            ("center_surface_ref", &entry.center_surface_ref),
            ("matrix_row_ref", &entry.matrix_row_ref),
            ("conformance_ref", &entry.conformance_ref),
            ("evidence_ref", &entry.evidence_ref),
            ("scope_snapshot_ref", &entry.scope_snapshot_ref),
            ("layout_receipt_ref", &entry.layout_receipt_ref),
            ("note", &entry.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SupportCenterLayoutViolation::EmptyField {
                    id: entry.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every entry must depend on at least one shared-inspector facet, so the registry never lists
        // a module that has dropped the shared inspector.
        if entry.required_facets.is_empty() {
            violations.push(M5SupportCenterLayoutViolation::NoRequiredFacets {
                entry_id: entry.entry_id.clone(),
            });
        }
        let mut seen_facets = BTreeSet::new();
        for facet in &entry.required_facets {
            if !seen_facets.insert(*facet) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateRequiredFacet {
                    entry_id: entry.entry_id.clone(),
                    facet: facet.as_str(),
                });
            }
        }

        // Every entry must reuse at least one existing source, so the center cards never duplicate
        // finding codes, crash IDs, install/advisory rows, or schema-registry state.
        if entry.integration_sources.is_empty() {
            violations.push(M5SupportCenterLayoutViolation::NoIntegrationSources {
                entry_id: entry.entry_id.clone(),
            });
        }
        let mut seen_sources = BTreeSet::new();
        for source in &entry.integration_sources {
            if !seen_sources.insert(*source) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateIntegrationSource {
                    entry_id: entry.entry_id.clone(),
                    source: source.as_str(),
                });
            }
        }

        let mut seen_a11y = BTreeSet::new();
        for guarantee in &entry.accessibility {
            if !seen_a11y.insert(*guarantee) {
                violations.push(
                    M5SupportCenterLayoutViolation::DuplicateAccessibilityGuarantee {
                        entry_id: entry.entry_id.clone(),
                        guarantee: guarantee.as_str(),
                    },
                );
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &entry.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5SupportCenterLayoutViolation::DuplicateDowngradeReason {
                    entry_id: entry.entry_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published presentation must equal the gate's recomputed decision, so an inaccessible
        // or facet-starved entry can never read as cleanly presented.
        let effective = entry.effective_presentation(inspector);
        if entry.presentation != effective {
            violations.push(M5SupportCenterLayoutViolation::OverstatedPresentation {
                entry_id: entry.entry_id.clone(),
                published: entry.presentation.as_str(),
                computed: effective.as_str(),
            });
        }

        let computed_reasons = entry.computed_downgrade_reasons(inspector);
        if entry.downgrade_reasons != computed_reasons {
            violations.push(M5SupportCenterLayoutViolation::DowngradeReasonsMismatch {
                entry_id: entry.entry_id.clone(),
            });
        }

        let computed_path = entry.computed_recovery_path(inspector);
        if entry.recovery_path != computed_path {
            violations.push(M5SupportCenterLayoutViolation::RecoveryPathMismatch {
                entry_id: entry.entry_id.clone(),
                declared: entry.recovery_path.as_str(),
                required: computed_path.as_str(),
            });
        }

        // A narrowed or withheld entry must offer a real recovery path, name a caveat, and list what
        // is unmet or unwired, so a narrowing never drops its recovery semantics or hides its cause.
        if entry.presentation.requires_recovery() {
            if !entry.recovery_path.is_offered() {
                violations.push(M5SupportCenterLayoutViolation::MissingRecoveryPath {
                    entry_id: entry.entry_id.clone(),
                });
            }
            if entry.caveats.is_empty() {
                violations.push(M5SupportCenterLayoutViolation::EmptyField {
                    id: entry.entry_id.clone(),
                    field_name: "caveats",
                });
            }
            if entry.unmet_or_unwired_fields.is_empty() {
                violations.push(M5SupportCenterLayoutViolation::EmptyField {
                    id: entry.entry_id.clone(),
                    field_name: "unmet_or_unwired_fields",
                });
            }
        }

        // A withheld entry offers nothing; a still-presented entry must name at least one action.
        if entry.presentation == PresentationDecision::Withheld {
            if !entry.offered_actions.is_empty() {
                violations.push(M5SupportCenterLayoutViolation::WithheldEntryOffersActions {
                    entry_id: entry.entry_id.clone(),
                });
            }
        } else if entry.offered_actions.is_empty() {
            violations.push(M5SupportCenterLayoutViolation::EmptyField {
                id: entry.entry_id.clone(),
                field_name: "offered_actions",
            });
        }

        // A cleanly presented entry must be genuinely whole: every accessibility invariant met, every
        // required facet wired, and nothing narrowing it.
        if effective == PresentationDecision::Presented
            && (!entry.accessibility_complete()
                || entry.facet_ceiling(inspector) != PresentationDecision::Presented
                || !entry.downgrade_reasons.is_empty()
                || !entry.caveats.is_empty()
                || !entry.unmet_or_unwired_fields.is_empty()
                || entry.recovery_path.is_offered())
        {
            violations.push(M5SupportCenterLayoutViolation::PresentedEntryNotWhole {
                entry_id: entry.entry_id.clone(),
            });
        }
    }
}

/// A validation violation for the Support Center layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SupportCenterLayoutViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Entry, region, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A layout region has no descriptor.
    MissingRegion {
        /// Region token.
        region: &'static str,
    },
    /// A layout region appears more than once.
    DuplicateRegion {
        /// Region token.
        region: &'static str,
    },
    /// Two regions share a keyboard order.
    DuplicateKeyboardOrder {
        /// Duplicate keyboard order.
        keyboard_order: u32,
    },
    /// A region's chrome does not satisfy every accessibility invariant.
    RegionNotAccessible {
        /// Region token.
        region: &'static str,
    },
    /// The shared inspector does not persist across module switches.
    InspectorNotShared,
    /// The shared inspector is missing a facet.
    MissingInspectorFacet {
        /// Facet token.
        facet: &'static str,
    },
    /// The shared inspector declares a facet more than once.
    DuplicateInspectorFacet {
        /// Facet token.
        facet: &'static str,
    },
    /// A shared-inspector facet binding is missing its descriptor ref or timestamp.
    InspectorFacetBindingIncomplete {
        /// Facet token.
        facet: &'static str,
    },
    /// A nav-entry id appears more than once.
    DuplicateNavEntry {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// Two entries share a nav order.
    DuplicateNavOrder {
        /// Duplicate nav order.
        nav_order: u32,
    },
    /// A module appears in more than one entry.
    DuplicateModule {
        /// Module token.
        module: &'static str,
    },
    /// A Support Center module has no nav entry.
    MissingModule {
        /// Module token.
        module: &'static str,
    },
    /// An entry depends on no shared-inspector facet.
    NoRequiredFacets {
        /// Entry id.
        entry_id: String,
    },
    /// An entry requires the same facet more than once.
    DuplicateRequiredFacet {
        /// Entry id.
        entry_id: String,
        /// Facet token.
        facet: &'static str,
    },
    /// An entry reuses no existing source.
    NoIntegrationSources {
        /// Entry id.
        entry_id: String,
    },
    /// An entry lists the same integration source more than once.
    DuplicateIntegrationSource {
        /// Entry id.
        entry_id: String,
        /// Integration-source token.
        source: &'static str,
    },
    /// An entry lists the same accessibility guarantee more than once.
    DuplicateAccessibilityGuarantee {
        /// Entry id.
        entry_id: String,
        /// Guarantee token.
        guarantee: &'static str,
    },
    /// An entry lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Entry id.
        entry_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// An entry publishes a presentation stronger than the gate computes.
    OverstatedPresentation {
        /// Entry id.
        entry_id: String,
        /// Published presentation token.
        published: &'static str,
        /// Computed effective presentation token.
        computed: &'static str,
    },
    /// An entry's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Entry id.
        entry_id: String,
    },
    /// An entry's recovery path disagrees with the recomputed path.
    RecoveryPathMismatch {
        /// Entry id.
        entry_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// A narrowed or withheld entry offers no recovery path.
    MissingRecoveryPath {
        /// Entry id.
        entry_id: String,
    },
    /// A withheld entry still offers actions.
    WithheldEntryOffersActions {
        /// Entry id.
        entry_id: String,
    },
    /// An entry presents cleanly but narrows a state or carries a reason.
    PresentedEntryNotWhole {
        /// Entry id.
        entry_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints registry truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the entries.
    SummaryMismatch,
}

impl fmt::Display for M5SupportCenterLayoutViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::MissingRegion { region } => write!(f, "missing descriptor for region {region}"),
            Self::DuplicateRegion { region } => {
                write!(f, "region {region} has more than one descriptor")
            }
            Self::DuplicateKeyboardOrder { keyboard_order } => {
                write!(f, "keyboard order {keyboard_order} is used by more than one region")
            }
            Self::RegionNotAccessible { region } => {
                write!(f, "region {region} does not satisfy every accessibility invariant")
            }
            Self::InspectorNotShared => {
                write!(f, "shared inspector does not persist across module switches")
            }
            Self::MissingInspectorFacet { facet } => {
                write!(f, "shared inspector is missing facet {facet}")
            }
            Self::DuplicateInspectorFacet { facet } => {
                write!(f, "shared inspector declares facet {facet} more than once")
            }
            Self::InspectorFacetBindingIncomplete { facet } => {
                write!(f, "shared inspector facet {facet} is missing its descriptor ref or timestamp")
            }
            Self::DuplicateNavEntry { entry_id } => write!(f, "duplicate nav entry id {entry_id}"),
            Self::DuplicateNavOrder { nav_order } => {
                write!(f, "nav order {nav_order} is used by more than one entry")
            }
            Self::DuplicateModule { module } => {
                write!(f, "module {module} has more than one nav entry")
            }
            Self::MissingModule { module } => write!(f, "missing nav entry for module {module}"),
            Self::NoRequiredFacets { entry_id } => {
                write!(f, "entry {entry_id} depends on no shared-inspector facet")
            }
            Self::DuplicateRequiredFacet { entry_id, facet } => {
                write!(f, "entry {entry_id} requires facet {facet} more than once")
            }
            Self::NoIntegrationSources { entry_id } => {
                write!(f, "entry {entry_id} reuses no existing source")
            }
            Self::DuplicateIntegrationSource { entry_id, source } => {
                write!(f, "entry {entry_id} reuses source {source} more than once")
            }
            Self::DuplicateAccessibilityGuarantee { entry_id, guarantee } => {
                write!(f, "entry {entry_id} lists guarantee {guarantee} more than once")
            }
            Self::DuplicateDowngradeReason { entry_id, reason } => {
                write!(f, "entry {entry_id} repeats downgrade reason {reason}")
            }
            Self::OverstatedPresentation {
                entry_id,
                published,
                computed,
            } => write!(
                f,
                "entry {entry_id} publishes presentation {published} but the gate computes {computed}"
            ),
            Self::DowngradeReasonsMismatch { entry_id } => {
                write!(f, "entry {entry_id} downgrade reasons disagree with the gate")
            }
            Self::RecoveryPathMismatch {
                entry_id,
                declared,
                required,
            } => write!(
                f,
                "entry {entry_id} records recovery {declared} but the gate requires {required}"
            ),
            Self::MissingRecoveryPath { entry_id } => {
                write!(f, "entry {entry_id} is narrowed or withheld but offers no recovery path")
            }
            Self::WithheldEntryOffersActions { entry_id } => {
                write!(f, "entry {entry_id} is withheld but still offers actions")
            }
            Self::PresentedEntryNotWhole { entry_id } => {
                write!(f, "entry {entry_id} presents cleanly but narrows a state or carries a reason")
            }
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve registry truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the entries"),
        }
    }
}

impl Error for M5SupportCenterLayoutViolation {}

/// Stable record-kind tag for [`M5SupportCenterLayoutSupportExport`].
pub const M5_SUPPORT_CENTER_UI_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_support_center_ui_support_export";

/// Support-export wrapper preserving the layout verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportCenterLayoutSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub layout_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact layout preserved by the export.
    pub layout: M5SupportCenterLayout,
}

impl M5SupportCenterLayoutSupportExport {
    /// Whether the export preserves the same packet id and a clean layout.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_SUPPORT_CENTER_UI_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_SUPPORT_CENTER_UI_SCHEMA_VERSION
            && self.layout_packet_id_ref == self.layout.packet_id
            && self.raw_private_material_excluded
            && self.layout.validate().is_empty()
    }
}

/// Loads the embedded Support Center layout packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5SupportCenterLayout`].
pub fn current_m5_support_center_layout() -> Result<M5SupportCenterLayout, serde_json::Error> {
    serde_json::from_str(M5_SUPPORT_CENTER_UI_JSON)
}

#[cfg(test)]
mod tests;
