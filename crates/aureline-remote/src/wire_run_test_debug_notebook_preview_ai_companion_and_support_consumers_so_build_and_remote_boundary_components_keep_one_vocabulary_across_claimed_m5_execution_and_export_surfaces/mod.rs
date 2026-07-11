//! Shared run/test/debug, notebook, preview, AI tool-routing, companion-handoff,
//! and support / export + release-packet consumers for the frozen M5
//! build/remote/managed-workspace boundary components.
//!
//! This module is the M05-1082 consumer-adoption lane over the frozen M5
//! build/remote-boundary component matrix
//! ([`crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix`]).
//! Where the freeze matrix defines the eight reusable adapter-confidence chip,
//! discovery-diff card, host-boundary strip, execution-origin receipt row,
//! managed-workspace lifecycle card, suspend/resume/rebuild review sheet,
//! workspace-expiry banner, and local-safe continuation card primitives — and the
//! four B128 implement lanes wire their resolvers and controls contracts — this
//! lane proves those families are reusable *primitives* rather than per-feature
//! build/remote chrome. It adopts them across the claimed M5 execution / export
//! consumer classes:
//!
//! 1. a run / test / debug surface,
//! 2. a notebook surface,
//! 3. a preview surface,
//! 4. an AI tool-routing surface,
//! 5. a companion-handoff surface, and
//! 6. a support / export + release-packet lane (incident / diagnostics + export;
//!    AC2).
//!
//! Each [`BuildRemoteConsumerRow`] points back to exactly one canonical component
//! family (its per-family matrix schema) and the one canonical controls contract
//! (schema + doc + release-proof artifact) its family group belongs to, instead
//! of cloning feature-local build/remote chrome. Every consumer — even a
//! read-only, inspect-only, export-only, or incident replay — keeps the identical
//! adapter-confidence, discovery-drift, host-boundary, execution-origin,
//! lifecycle-state, changed-persistence, continuity, expiry-timing, and
//! local-safe-continuation labels and the identical frozen boundary-disposition
//! vocabulary. A narrower consumer discloses the reduction with a
//! reduced-capability banner (and, when it punts to another surface, a desktop /
//! companion / browser / support-packet note) rather than renaming or dropping
//! governed boundary truth, so run/test/debug, notebook, preview, AI, companion,
//! and support panes never fork build/remote-boundary vocabulary by surface. This
//! is what makes the same host, confidence, lifecycle, and continuation state
//! render with one vocabulary and one component family across every claimed
//! consumer (AC1), and lets support / export / release packets drop bespoke
//! feature-local translation tables (AC2).
//!
//! The three spec guardrails are enforced per row and must all stay false: no
//! consumer implies exact continuity after target identity, image, template, or
//! persistence class changed materially; no consumer hides local-safe
//! continuation or browser/companion handoff behind overflow-only affordances; no
//! consumer lets lower-confidence discovery overwrite higher-confidence resolved
//! target truth without an explicit review state.
//!
//! The packet is metadata-only: raw provider tokens, credential material, and
//! bearer secrets never cross this boundary; the packet carries only typed class
//! tokens, opaque boundary-state refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-build-remote-boundary-component-consumer.schema.json`](../../../../schemas/ui/m5-build-remote-boundary-component-consumer.schema.json).
//! The contract doc is
//! [`docs/remote/m5_build_remote_boundary_component_consumer_contract.md`](../../../../docs/remote/m5_build_remote_boundary_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix as matrix;
use crate::implement_the_m5_adapter_confidence_chip_and_discovery_diff_card_adapter_source_class_confidence_band_discovery_mode_downgrade_reason_target_identity_drift_changed_certainty_review_before_switch_and_no_higher_confidence_overwrite_primitive as adapter_discovery_controls;
use crate::implement_the_m5_host_boundary_strip_and_execution_origin_receipt_row_locality_class_target_label_owning_runtime_service_lane_reconnect_degraded_state_action_class_resolved_target_identity_provenance_and_export_safe_lineage_primitive as host_origin_controls;
use crate::implement_the_m5_managed_workspace_lifecycle_card_and_suspend_resume_rebuild_review_sheet_lifecycle_state_persistence_class_continuity_class_template_image_provenance_changed_persistence_preserved_vs_lost_state_and_local_safe_continuation_primitive as managed_lifecycle_controls;
use crate::implement_the_m5_workspace_expiry_banner_and_local_safe_continuation_card_expiry_timing_triggering_owner_source_affected_capabilities_export_before_loss_preserved_files_context_lost_live_state_and_no_exact_continuity_overclaim_primitive as expiry_continuation_controls;

pub use matrix::{
    M5BuildRemoteBoundaryComponentFamily, M5BuildRemoteBoundaryDisposition,
    M5BuildRemoteConsumerSurface,
};

/// Schema version stamped on the M05-1082 consumer packet.
pub const BUILD_REMOTE_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`BuildRemoteConsumerPacket`].
pub const BUILD_REMOTE_CONSUMER_RECORD_KIND: &str =
    "m5_build_remote_boundary_component_consumer_packet";

/// Stable record-kind tag carried by each [`BuildRemoteConsumerRow`].
pub const BUILD_REMOTE_CONSUMER_ROW_RECORD_KIND: &str =
    "m5_build_remote_boundary_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const BUILD_REMOTE_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-build-remote-boundary-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const BUILD_REMOTE_CONSUMER_DOC_REF: &str =
    "docs/remote/m5_build_remote_boundary_component_consumer_contract.md";

/// Repo-relative path of the frozen build/remote-boundary component matrix release
/// proof these consumers adopt.
pub const BUILD_REMOTE_CONSUMER_MATRIX_REF: &str =
    matrix::M5_BUILD_REMOTE_BOUNDARY_COMPONENT_ARTIFACT_REF;

/// Repo-relative path of the shared frozen component-matrix schema.
pub const BUILD_REMOTE_CONSUMER_SHARED_SCHEMA_REF: &str =
    matrix::M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const BUILD_REMOTE_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const BUILD_REMOTE_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const BUILD_REMOTE_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-consumer-proof/report.md";

/// Repo-relative path of the checked consumer-fixture directory.
pub const BUILD_REMOTE_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-build-remote-boundary-component-consumers";

/// The controlled label families a consumer must preserve identically across
/// every M5 execution / export surface. These are the track-invariant truth
/// pillars: adapter confidence, discovery drift, host boundary, execution origin,
/// lifecycle state, the changed persistence class, continuity, expiry timing, and
/// local-safe continuation. The union of every row's `preserved_label_families`
/// must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 9] = [
    "adapter_confidence",
    "discovery_drift",
    "host_boundary",
    "execution_origin",
    "lifecycle_state",
    "persistence_class",
    "continuity",
    "expiry_timing",
    "local_safe_continuation",
];

/// The canonical boundary-disposition vocabulary every consumer keeps visible
/// even when narrowed or export-only — the frozen `M5BuildRemoteBoundaryDisposition`
/// set (local / SSH / container / devcontainer / managed / browser-bridge /
/// service-plane execution, suspended / rebuilt / recreated / expired,
/// local-safe-continuation, not-evaluated). Every consumer renders the same host /
/// lifecycle / continuity state with these exact tokens rather than feature-local
/// phrasing (AC1).
pub fn canonical_boundary_disposition_vocab() -> Vec<String> {
    M5BuildRemoteBoundaryDisposition::ALL
        .iter()
        .map(|d| d.as_str().to_owned())
        .collect()
}

/// Whether a token is one of the frozen boundary-disposition tokens.
pub fn is_canonical_boundary_disposition(token: &str) -> bool {
    M5BuildRemoteBoundaryDisposition::ALL
        .iter()
        .any(|d| d.as_str() == token)
}

/// The canonical per-family matrix schema that defines a family's contract.
pub fn canonical_family_schema_ref_for(
    family: M5BuildRemoteBoundaryComponentFamily,
) -> &'static str {
    family.canonical_component_schema_ref()
}

/// The single primary boundary label family a component family must always
/// preserve — the boundary axis it exists to name. A consumer may narrow
/// authority, but it must never drop this label, so the family's core adapter
/// confidence, discovery drift, host boundary, execution origin, lifecycle state,
/// continuity, expiry timing, or local-safe continuation truth is never silently
/// lost.
pub const fn family_primary_label(family: M5BuildRemoteBoundaryComponentFamily) -> &'static str {
    use M5BuildRemoteBoundaryComponentFamily::*;
    match family {
        AdapterConfidenceChip => "adapter_confidence",
        DiscoveryDiffCard => "discovery_drift",
        HostBoundaryStrip => "host_boundary",
        ExecutionOriginReceiptRow => "execution_origin",
        ManagedWorkspaceLifecycleCard => "lifecycle_state",
        SuspendResumeRebuildReviewSheet => "continuity",
        WorkspaceExpiryBanner => "expiry_timing",
        LocalSafeContinuationCard => "local_safe_continuation",
    }
}

/// The four B128 controls contracts the eight component families group into. A
/// consumer must point at the one canonical controls contract for its family's
/// lane rather than inventing a feature-local one — this is the heart of the
/// "execution surfaces no longer fork build/remote vocabulary" acceptance
/// criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteControlsLane {
    /// Adapter-confidence chip + discovery-diff card controls (M05-1077 lane).
    AdapterDiscovery,
    /// Host-boundary strip + execution-origin receipt row controls (M05-1078
    /// lane).
    HostOrigin,
    /// Managed-workspace lifecycle card + suspend/resume/rebuild review sheet
    /// controls (M05-1079 lane).
    ManagedLifecycle,
    /// Workspace-expiry banner + local-safe continuation card controls (M05-1080
    /// lane).
    ExpiryContinuation,
}

impl M5BuildRemoteControlsLane {
    /// Every controls lane, in declaration order.
    pub const ALL: [M5BuildRemoteControlsLane; 4] = [
        M5BuildRemoteControlsLane::AdapterDiscovery,
        M5BuildRemoteControlsLane::HostOrigin,
        M5BuildRemoteControlsLane::ManagedLifecycle,
        M5BuildRemoteControlsLane::ExpiryContinuation,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterDiscovery => "adapter_discovery",
            Self::HostOrigin => "host_origin",
            Self::ManagedLifecycle => "managed_lifecycle",
            Self::ExpiryContinuation => "expiry_continuation",
        }
    }

    /// The canonical controls schema every surface reuses for this lane.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::AdapterDiscovery => {
                adapter_discovery_controls::M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_REF
            }
            Self::HostOrigin => host_origin_controls::M5_HOST_ORIGIN_CONTROLS_SCHEMA_REF,
            Self::ManagedLifecycle => {
                managed_lifecycle_controls::M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_REF
            }
            Self::ExpiryContinuation => {
                expiry_continuation_controls::M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_REF
            }
        }
    }

    /// The canonical controls contract doc for this lane.
    pub const fn canonical_doc_ref(self) -> &'static str {
        match self {
            Self::AdapterDiscovery => {
                adapter_discovery_controls::M5_ADAPTER_DISCOVERY_CONTROLS_DOC_REF
            }
            Self::HostOrigin => host_origin_controls::M5_HOST_ORIGIN_CONTROLS_DOC_REF,
            Self::ManagedLifecycle => {
                managed_lifecycle_controls::M5_MANAGED_LIFECYCLE_CONTROLS_DOC_REF
            }
            Self::ExpiryContinuation => {
                expiry_continuation_controls::M5_EXPIRY_CONTINUATION_CONTROLS_DOC_REF
            }
        }
    }

    /// The canonical controls release-proof artifact every consumer points back
    /// to as the first-resolved truth for this lane.
    pub const fn canonical_artifact_ref(self) -> &'static str {
        match self {
            Self::AdapterDiscovery => {
                adapter_discovery_controls::M5_ADAPTER_DISCOVERY_CONTROLS_ARTIFACT_REF
            }
            Self::HostOrigin => host_origin_controls::M5_HOST_ORIGIN_CONTROLS_ARTIFACT_REF,
            Self::ManagedLifecycle => {
                managed_lifecycle_controls::M5_MANAGED_LIFECYCLE_CONTROLS_ARTIFACT_REF
            }
            Self::ExpiryContinuation => {
                expiry_continuation_controls::M5_EXPIRY_CONTINUATION_CONTROLS_ARTIFACT_REF
            }
        }
    }
}

/// The one controls lane a component family belongs to. The eight frozen families
/// group into the four B128 controls contracts; a consumer must reuse the lane's
/// canonical contract rather than forking it per surface.
pub const fn controls_lane_for(
    family: M5BuildRemoteBoundaryComponentFamily,
) -> M5BuildRemoteControlsLane {
    use M5BuildRemoteBoundaryComponentFamily::*;
    match family {
        AdapterConfidenceChip | DiscoveryDiffCard => M5BuildRemoteControlsLane::AdapterDiscovery,
        HostBoundaryStrip | ExecutionOriginReceiptRow => M5BuildRemoteControlsLane::HostOrigin,
        ManagedWorkspaceLifecycleCard | SuspendResumeRebuildReviewSheet => {
            M5BuildRemoteControlsLane::ManagedLifecycle
        }
        WorkspaceExpiryBanner | LocalSafeContinuationCard => {
            M5BuildRemoteControlsLane::ExpiryContinuation
        }
    }
}

/// The six claimed M5 execution / export consumer classes that must each adopt at
/// least one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerClass {
    /// A run / test / debug surface.
    RunTestDebug,
    /// A notebook surface.
    Notebook,
    /// A preview surface.
    Preview,
    /// An AI tool-routing surface.
    AiToolRouting,
    /// A companion-handoff surface.
    CompanionHandoff,
    /// A support / export + release-packet lane (incident / diagnostics + export;
    /// AC2).
    SupportExport,
}

impl ConsumerClass {
    /// Every consumer class that must be present for cross-surface reuse.
    pub const ALL: [ConsumerClass; 6] = [
        ConsumerClass::RunTestDebug,
        ConsumerClass::Notebook,
        ConsumerClass::Preview,
        ConsumerClass::AiToolRouting,
        ConsumerClass::CompanionHandoff,
        ConsumerClass::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunTestDebug => "run_test_debug",
            Self::Notebook => "notebook",
            Self::Preview => "preview",
            Self::AiToolRouting => "ai_tool_routing",
            Self::CompanionHandoff => "companion_handoff",
            Self::SupportExport => "support_export",
        }
    }

    /// True when this class routes or executes work across a host / continuity
    /// boundary (run/test/debug, AI tool routing, or companion handoff) and
    /// therefore must never drop the adopted family's primary boundary label — the
    /// host-boundary, execution-origin, lifecycle, or continuity truth that says
    /// where the work ran and whether it kept exact continuity.
    pub const fn is_boundary_crossing(self) -> bool {
        matches!(
            self,
            Self::RunTestDebug | Self::AiToolRouting | Self::CompanionHandoff
        )
    }
}

/// The consumer class a concrete matrix consumer surface belongs to. Reuses the
/// matrix's own [`M5BuildRemoteConsumerSurface`] taxonomy rather than inventing a
/// parallel one.
pub const fn consumer_class_for(surface: M5BuildRemoteConsumerSurface) -> ConsumerClass {
    use M5BuildRemoteConsumerSurface::*;
    match surface {
        ShellUi | RunTestDebugUi => ConsumerClass::RunTestDebug,
        NotebookUi => ConsumerClass::Notebook,
        PreviewUi => ConsumerClass::Preview,
        ProductUi => ConsumerClass::AiToolRouting,
        CompanionUi => ConsumerClass::CompanionHandoff,
        IncidentUi | SupportExport => ConsumerClass::SupportExport,
    }
}

/// True when this surface is the run / test / debug execution surface — the first
/// claimed execution consumer whose canonical adoption AC1 anchors.
pub const fn is_run_test_debug_surface(surface: M5BuildRemoteConsumerSurface) -> bool {
    matches!(surface, M5BuildRemoteConsumerSurface::RunTestDebugUi)
}

/// True when this surface is the support / export + release-packet surface (AC2).
pub const fn is_support_export_surface(surface: M5BuildRemoteConsumerSurface) -> bool {
    matches!(surface, M5BuildRemoteConsumerSurface::SupportExport)
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, override-gated,
/// export-only, policy-blocked) but never rename or drop the governed boundary
/// truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Full-interactive control (act on the build/remote component directly).
    FullInteractive,
    /// Read-only projection of the component.
    ReadOnly,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Override-gated: the action is visible but staged behind an explicit review
    /// gate (e.g. review-before-switch or review-before-commit) before it applies.
    OverrideGated,
    /// Export-only: reconstruct the component from an export packet.
    ExportOnly,
    /// Policy-blocked: the component is visible but action is gated by policy.
    PolicyBlocked,
}

impl AuthorityMode {
    /// Every authority mode, in declaration order.
    pub const ALL: [AuthorityMode; 6] = [
        AuthorityMode::FullInteractive,
        AuthorityMode::ReadOnly,
        AuthorityMode::InspectOnly,
        AuthorityMode::OverrideGated,
        AuthorityMode::ExportOnly,
        AuthorityMode::PolicyBlocked,
    ];

    /// Returns true when the consumer narrows below full-interactive authority and
    /// therefore must disclose the reduction with a banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullInteractive)
    }

    /// The banner `capability_state` label this authority maps to.
    pub const fn capability_state(self) -> &'static str {
        match self {
            Self::FullInteractive => "full",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
            Self::OverrideGated => "override_gated",
            Self::ExportOnly => "export_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The surface a narrower consumer hands off to when it cannot act on the
/// build/remote component locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders and acts on the component in-place.
    None,
    /// Punt to the desktop shell to act on the boundary state.
    DesktopShell,
    /// Punt to the companion app.
    CompanionApp,
    /// Punt to a read-only browser surface (the browser bridge).
    BrowserReadonly,
    /// Punt to a portable support / export packet.
    SupportPacket,
    /// Punt to a headless CLI.
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore must
    /// carry a desktop / companion / browser / support note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DesktopShell => "desktop_shell",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::SupportPacket => "support_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full label parity across the boundary truth pillars.
    Preserved,
    /// Reduced interactivity, disclosed, but the labels are still preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable host /
/// confidence / lifecycle / continuity identity support and automation need to
/// reconstruct the boundary state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// The reduced-capability banner a narrower consumer shows to disclose the control
/// it drops relative to the full build/remote-boundary component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The capability state; must match the row's `authority_mode`.
    pub capability_state: String,
    /// The capabilities the narrowed surface is missing relative to full.
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// One consumer adopting one canonical build/remote-boundary component family on
/// one M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteConsumerRow {
    /// Record kind; must equal [`BUILD_REMOTE_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BUILD_REMOTE_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_class: ConsumerClass,
    /// The concrete surface; must belong to `consumer_class`.
    pub consumer_surface: M5BuildRemoteConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5BuildRemoteBoundaryComponentFamily,
    /// The controls lane the family belongs to; must equal
    /// `controls_lane_for(component_family)`.
    pub controls_lane: M5BuildRemoteControlsLane,
    /// The canonical per-family matrix schema. Must equal
    /// `canonical_family_schema_ref_for(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical controls schema for the lane. Must equal
    /// `controls_lane.canonical_schema_ref()`.
    pub canonical_controls_schema_ref: String,
    /// The canonical controls release-proof artifact(s) this consumer points back
    /// to. Must contain `controls_lane.canonical_artifact_ref()`.
    #[serde(default)]
    pub canonical_controls_artifact_refs: Vec<String>,
    /// True when the consumer references the canonical family + controls lane
    /// rather than cloning feature-local build/remote chrome.
    pub references_canonical_not_local_prose: bool,
    /// An opaque, redaction-safe ref to the host / lifecycle / continuity state the
    /// user saw, so support and automation can reconstruct it without leaking raw
    /// provider tokens, credential material, or bearer secrets.
    pub boundary_state_ref: String,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The frozen boundary-disposition vocabulary the consumer keeps visible even
    /// when narrowed.
    #[serde(default)]
    pub boundary_disposition_vocab: Vec<String>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The desktop / companion / browser / support note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Guardrail: the consumer implies exact continuity after target identity,
    /// image, template, or persistence class changed materially. Must be false.
    pub implies_exact_continuity_after_material_change: bool,
    /// Guardrail: the consumer hides local-safe continuation or browser/companion
    /// handoff behind overflow-only affordances. Must be false.
    pub hides_local_safe_or_companion_handoff_in_overflow_only: bool,
    /// Guardrail: the consumer lets lower-confidence discovery overwrite
    /// higher-confidence resolved target truth without an explicit review state.
    /// Must be false.
    pub lower_confidence_overwrites_resolved_target_without_review: bool,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl BuildRemoteConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// The surface's declared class matches the row's declared class.
    pub fn surface_class_consistent(&self) -> bool {
        consumer_class_for(self.consumer_surface) == self.consumer_class
    }

    /// AC (no fork): the consumer reuses the canonical controls contract for its
    /// family's lane rather than a feature-local one.
    pub fn controls_lane_is_canonical(&self) -> bool {
        self.controls_lane == controls_lane_for(self.component_family)
            && self.canonical_controls_schema_ref == self.controls_lane.canonical_schema_ref()
            && self
                .canonical_controls_artifact_refs
                .iter()
                .any(|r| r == self.controls_lane.canonical_artifact_ref())
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family —
    /// the declared matrix schema matches the family, a controls release-proof
    /// artifact is referenced, and no feature-local build/remote chrome is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_family_schema_ref_for(self.component_family)
            && self.controls_lane_is_canonical()
            && self.references_canonical_not_local_prose
    }

    /// AC1 (parity): the consumer preserves the family's controlled label families
    /// and frozen boundary-disposition vocabulary rather than renaming or omitting
    /// them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.boundary_disposition_vocab.is_empty()
            && self
                .boundary_disposition_vocab
                .iter()
                .all(|v| is_canonical_boundary_disposition(v))
    }

    /// AC (boundary truth): every row preserves the adopted family's primary
    /// boundary label, and a boundary-crossing consumer (run/test/debug, AI tool
    /// routing, or companion handoff) never drops it — so an execution surface
    /// never hides where the work ran or whether it kept exact continuity.
    pub fn preserves_primary_boundary_truth(&self) -> bool {
        let primary = family_primary_label(self.component_family);
        self.preserved_label_families.iter().any(|f| f == primary)
    }

    /// AC2: the row carries the opaque boundary-state ref and canonical controls
    /// contract support and automation reconstruct the seen state from.
    pub fn supports_state_reconstruction(&self) -> bool {
        !self.boundary_state_ref.trim().is_empty()
            && self.controls_lane_is_canonical()
            && self.copy_export.is_complete()
    }

    /// The three spec guardrails are all clear (false).
    pub fn guardrails_clear(&self) -> bool {
        self.first_failed_guardrail().is_none()
    }

    /// The first guardrail that is (wrongly) set, if any.
    pub fn first_failed_guardrail(&self) -> Option<&'static str> {
        if self.implies_exact_continuity_after_material_change {
            Some("implies_exact_continuity_after_material_change")
        } else if self.hides_local_safe_or_companion_handoff_in_overflow_only {
            Some("hides_local_safe_or_companion_handoff_in_overflow_only")
        } else if self.lower_confidence_overwrites_resolved_target_without_review {
            Some("lower_confidence_overwrites_resolved_target_without_review")
        } else {
            None
        }
    }

    /// AC (disclosure): a narrower consumer discloses the reduction with a
    /// reduced-capability banner whose state matches the authority mode, and
    /// carries a note whenever it punts to another surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            match &self.reduced_capability_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.capability_state != self.authority_mode.capability_state()
                        || banner.capability_state == "full"
                        || banner.missing_capabilities.is_empty()
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.reduced_capability_banner.is_some() {
            // A full-interactive consumer must not carry a spurious banner.
            return false;
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == BUILD_REMOTE_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == BUILD_REMOTE_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.boundary_state_ref.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_controls_schema_ref.trim().is_empty()
            && !self.canonical_controls_artifact_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} class={class} family={family} lane={lane} \
authority={authority} label_parity={label_parity} handoff={handoff}",
            surface = self.consumer_surface.as_str(),
            class = self.consumer_class.as_str(),
            family = self.component_family.as_str(),
            lane = self.controls_lane.as_str(),
            authority = self.authority_mode.capability_state(),
            label_parity = self.label_parity.as_str(),
            handoff = self.handoff_target.as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1082 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteConsumerSummary {
    pub row_count: usize,
    pub consumer_class_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub controls_lane_count: usize,
    pub boundary_disposition_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_use_canonical_controls_lane: bool,
    pub all_boundary_rows_preserve_primary_truth: bool,
    pub all_rows_reconstructable: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub all_rows_guardrails_clear: bool,
    pub controls_lanes_stable_across_surfaces: bool,
    pub run_test_debug_consumer_present: bool,
    pub notebook_consumer_present: bool,
    pub preview_consumer_present: bool,
    pub ai_tool_routing_consumer_present: bool,
    pub companion_handoff_consumer_present: bool,
    pub support_export_consumer_present: bool,
    pub run_test_debug_reference_present: bool,
    pub support_export_reference_present: bool,
    pub label_family_coverage_complete: bool,
    pub boundary_disposition_coverage_complete: bool,
    pub families_reused_across_classes: usize,
}

/// Constructor input for [`BuildRemoteConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildRemoteConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<BuildRemoteConsumerRow>,
}

/// Checked-in M05-1082 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<BuildRemoteConsumerRow>,
    pub summary: BuildRemoteConsumerSummary,
}

impl BuildRemoteConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: BuildRemoteConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: BUILD_REMOTE_CONSUMER_SCHEMA_VERSION,
            record_kind: BUILD_REMOTE_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: BuildRemoteConsumerSummary {
                row_count: 0,
                consumer_class_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                controls_lane_count: 0,
                boundary_disposition_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_use_canonical_controls_lane: false,
                all_boundary_rows_preserve_primary_truth: false,
                all_rows_reconstructable: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                all_rows_guardrails_clear: false,
                controls_lanes_stable_across_surfaces: false,
                run_test_debug_consumer_present: false,
                notebook_consumer_present: false,
                preview_consumer_present: false,
                ai_tool_routing_consumer_present: false,
                companion_handoff_consumer_present: false,
                support_export_consumer_present: false,
                run_test_debug_reference_present: false,
                support_export_reference_present: false,
                label_family_coverage_complete: false,
                boundary_disposition_coverage_complete: false,
                families_reused_across_classes: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5BuildRemoteBoundaryComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The union of every row's boundary-disposition vocabulary.
    pub fn covered_boundary_dispositions(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.boundary_disposition_vocab.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// classes — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_classes(&self) -> usize {
        M5BuildRemoteBoundaryComponentFamily::ALL
            .iter()
            .filter(|family| {
                let classes: BTreeSet<ConsumerClass> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_class)
                    .collect();
                classes.len() >= 2
            })
            .count()
    }

    /// Whether every family maps to exactly one controls lane across every surface
    /// — no surface forks the lane by consumer.
    pub fn controls_lanes_stable_across_surfaces(&self) -> bool {
        let mut per_family: BTreeMap<
            M5BuildRemoteBoundaryComponentFamily,
            BTreeSet<M5BuildRemoteControlsLane>,
        > = BTreeMap::new();
        for row in &self.rows {
            per_family
                .entry(row.component_family)
                .or_default()
                .insert(row.controls_lane);
        }
        per_family.values().all(|lanes| lanes.len() <= 1)
    }

    /// Whether some run / test / debug execution surface references the canonical
    /// families — the first-claimed-consumer half of AC1.
    pub fn has_run_test_debug_reference(&self) -> bool {
        self.rows.iter().any(|r| {
            is_run_test_debug_surface(r.consumer_surface) && r.references_canonical_not_local_prose
        })
    }

    /// Whether some support / export surface references the canonical families —
    /// the release-packet half of AC2.
    pub fn has_support_export_reference(&self) -> bool {
        self.rows.iter().any(|r| {
            is_support_export_surface(r.consumer_surface) && r.references_canonical_not_local_prose
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> BuildRemoteConsumerSummary {
        let mut classes = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        let mut lanes = BTreeSet::new();
        for row in &self.rows {
            classes.insert(row.consumer_class);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
            lanes.insert(row.controls_lane);
        }

        let has_class = |c: ConsumerClass| classes.contains(&c);
        let covered = self.covered_label_families();
        let covered_dispositions = self.covered_boundary_dispositions();

        BuildRemoteConsumerSummary {
            row_count: self.rows.len(),
            consumer_class_count: classes.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            controls_lane_count: lanes.len(),
            boundary_disposition_count: covered_dispositions.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(BuildRemoteConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self
                .rows
                .iter()
                .all(BuildRemoteConsumerRow::preserves_labels),
            all_rows_use_canonical_controls_lane: self
                .rows
                .iter()
                .all(BuildRemoteConsumerRow::controls_lane_is_canonical),
            all_boundary_rows_preserve_primary_truth: self
                .rows
                .iter()
                .all(BuildRemoteConsumerRow::preserves_primary_boundary_truth),
            all_rows_reconstructable: self
                .rows
                .iter()
                .all(BuildRemoteConsumerRow::supports_state_reconstruction),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(BuildRemoteConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            all_rows_guardrails_clear: self
                .rows
                .iter()
                .all(BuildRemoteConsumerRow::guardrails_clear),
            controls_lanes_stable_across_surfaces: self.controls_lanes_stable_across_surfaces(),
            run_test_debug_consumer_present: has_class(ConsumerClass::RunTestDebug),
            notebook_consumer_present: has_class(ConsumerClass::Notebook),
            preview_consumer_present: has_class(ConsumerClass::Preview),
            ai_tool_routing_consumer_present: has_class(ConsumerClass::AiToolRouting),
            companion_handoff_consumer_present: has_class(ConsumerClass::CompanionHandoff),
            support_export_consumer_present: has_class(ConsumerClass::SupportExport),
            run_test_debug_reference_present: self.has_run_test_debug_reference(),
            support_export_reference_present: self.has_support_export_reference(),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            boundary_disposition_coverage_complete: M5BuildRemoteBoundaryDisposition::ALL
                .iter()
                .all(|d| covered_dispositions.contains(d.as_str())),
            families_reused_across_classes: self.families_reused_across_classes(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<BuildRemoteConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BUILD_REMOTE_CONSUMER_SCHEMA_VERSION {
            violations.push(BuildRemoteConsumerViolation::SchemaVersion {
                expected: BUILD_REMOTE_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BUILD_REMOTE_CONSUMER_RECORD_KIND {
            violations.push(BuildRemoteConsumerViolation::RecordKind {
                expected: BUILD_REMOTE_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(BuildRemoteConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(BuildRemoteConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_classes.insert(row.consumer_class);

            if !row.is_complete() {
                violations.push(BuildRemoteConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer class.
            if !row.surface_class_consistent() {
                violations.push(BuildRemoteConsumerViolation::SurfaceClassMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned feature-local chrome.
            if !row.points_to_canonical_family() {
                violations.push(BuildRemoteConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC (no fork): canonical controls lane per family.
            if !row.controls_lane_is_canonical() {
                violations.push(BuildRemoteConsumerViolation::NonCanonicalControlsLane {
                    id: row.row_id.clone(),
                });
            }

            // AC1: controlled label families / boundary-disposition vocab preserved.
            if !row.preserves_labels() {
                violations.push(BuildRemoteConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC (boundary truth): the family's primary boundary label is kept.
            if !row.preserves_primary_boundary_truth() {
                violations.push(BuildRemoteConsumerViolation::PrimaryBoundaryTruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // AC2: boundary state is reconstructable from the opaque ref +
            // canonical controls contract.
            if !row.supports_state_reconstruction() {
                violations.push(BuildRemoteConsumerViolation::StateNotReconstructable {
                    id: row.row_id.clone(),
                });
            }

            // Disclosure: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(BuildRemoteConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(BuildRemoteConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }

            // Spec guardrails must all stay false.
            if let Some(guardrail) = row.first_failed_guardrail() {
                violations.push(BuildRemoteConsumerViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                    guardrail,
                });
            }
        }

        // Cross-surface reuse spans all six claimed consumer classes.
        for class in ConsumerClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(BuildRemoteConsumerViolation::MissingConsumerClass { class });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5BuildRemoteBoundaryComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(BuildRemoteConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer classes so
        // multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_classes() == 0 {
            violations.push(BuildRemoteConsumerViolation::NoFamilyReusedAcrossClasses);
        }

        // AC (no fork): families resolve to one stable controls lane per family.
        if !self.controls_lanes_stable_across_surfaces() {
            violations.push(BuildRemoteConsumerViolation::ControlsLaneForkedAcrossSurfaces);
        }

        // AC1: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(BuildRemoteConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC1: the frozen boundary-disposition vocabulary is collectively preserved.
        let covered_dispositions = self.covered_boundary_dispositions();
        for disposition in M5BuildRemoteBoundaryDisposition::ALL {
            if !covered_dispositions.contains(disposition.as_str()) {
                violations.push(BuildRemoteConsumerViolation::MissingBoundaryDisposition {
                    disposition: disposition.as_str().to_owned(),
                });
            }
        }

        // AC1: a run / test / debug execution consumer references the canonical
        // components rather than cloning feature-local build/remote chrome.
        if !self.has_run_test_debug_reference() {
            violations.push(BuildRemoteConsumerViolation::MissingRunTestDebugReference);
        }

        // AC2: a support / export + release-packet consumer references the
        // canonical components so release packets drop feature-local translation
        // tables.
        if !self.has_support_export_reference() {
            violations.push(BuildRemoteConsumerViolation::MissingSupportExportReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(BuildRemoteConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(BuildRemoteConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_class,consumer_surface,component_family,controls_lane,authority,label_parity,handoff\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{class},{surface},{family},{lane},{authority},{label_parity},{handoff}\n",
                id = row.row_id,
                class = row.consumer_class.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                lane = row.controls_lane.as_str(),
                authority = row.authority_mode.capability_state(),
                label_parity = row.label_parity.as_str(),
                handoff = row.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Build/Remote-Boundary Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer classes and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_class_count,
            self.represented_families().len(),
            M5BuildRemoteBoundaryComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Controls lanes adopted: {} / {}\n",
            self.summary.controls_lane_count,
            M5BuildRemoteControlsLane::ALL.len(),
        ));
        out.push_str(&format!(
            "- Boundary dispositions preserved: {} / {}\n",
            self.summary.boundary_disposition_count,
            M5BuildRemoteBoundaryDisposition::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across classes: {}\n",
            self.summary.families_reused_across_classes,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_build_remote_boundary_component_consumers_export(
) -> Result<BuildRemoteConsumerPacket, BuildRemoteConsumerArtifactError> {
    let packet: BuildRemoteConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-build-remote-boundary-component-consumer-proof/support_export.json"
    )))
    .map_err(BuildRemoteConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BuildRemoteConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum BuildRemoteConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BuildRemoteConsumerViolation>),
}

impl fmt::Display for BuildRemoteConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "consumer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "consumer export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for BuildRemoteConsumerArtifactError {}

/// Validation failure for M05-1082 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildRemoteConsumerViolation {
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
    SurfaceClassMismatch {
        id: String,
    },
    NotCanonicalFamily {
        id: String,
    },
    NonCanonicalControlsLane {
        id: String,
    },
    LabelParityBroken {
        id: String,
    },
    PrimaryBoundaryTruthDropped {
        id: String,
    },
    StateNotReconstructable {
        id: String,
    },
    NarrowedWithoutDisclosure {
        id: String,
    },
    MissingCopyExportParity {
        id: String,
    },
    GuardrailViolated {
        id: String,
        guardrail: &'static str,
    },
    MissingConsumerClass {
        class: ConsumerClass,
    },
    MissingFamilyCoverage {
        family: M5BuildRemoteBoundaryComponentFamily,
    },
    NoFamilyReusedAcrossClasses,
    ControlsLaneForkedAcrossSurfaces,
    MissingLabelFamily {
        family: String,
    },
    MissingBoundaryDisposition {
        disposition: String,
    },
    MissingRunTestDebugReference,
    MissingSupportExportReference,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for BuildRemoteConsumerViolation {
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
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceClassMismatch { id } => {
                write!(
                    f,
                    "row {id} declares a surface that does not belong to its consumer class"
                )
            }
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::NonCanonicalControlsLane { id } => {
                write!(
                    f,
                    "row {id} forks the controls lane instead of reusing the canonical contract"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical adapter-confidence, discovery-drift, \
host-boundary, execution-origin, lifecycle-state, changed-persistence, continuity, expiry-timing, \
or local-safe-continuation label"
                )
            }
            Self::PrimaryBoundaryTruthDropped { id } => {
                write!(
                    f,
                    "row {id} drops the adopted family's primary boundary label (adapter confidence, \
discovery drift, host boundary, execution origin, lifecycle state, continuity, expiry timing, or \
local-safe continuation)"
                )
            }
            Self::StateNotReconstructable { id } => {
                write!(
                    f,
                    "row {id} cannot be reconstructed from its boundary-state ref and controls contract"
                )
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(
                    f,
                    "row {id} narrows authority without a reduced-capability banner or handoff note"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text / JSON / Markdown copy-export parity"
                )
            }
            Self::GuardrailViolated { id, guardrail } => {
                write!(f, "row {id} violates guardrail {guardrail}")
            }
            Self::MissingConsumerClass { class } => {
                write!(f, "consumer class {class:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not adopted in the packet"
                )
            }
            Self::NoFamilyReusedAcrossClasses => write!(
                f,
                "no component family is adopted across two or more consumer classes"
            ),
            Self::ControlsLaneForkedAcrossSurfaces => write!(
                f,
                "a component family resolves to more than one controls lane across surfaces"
            ),
            Self::MissingLabelFamily { family } => {
                write!(
                    f,
                    "controlled label family {family} is not preserved anywhere"
                )
            }
            Self::MissingBoundaryDisposition { disposition } => {
                write!(
                    f,
                    "boundary-disposition token {disposition} is not preserved anywhere"
                )
            }
            Self::MissingRunTestDebugReference => write!(
                f,
                "no run / test / debug consumer references the canonical component families"
            ),
            Self::MissingSupportExportReference => write!(
                f,
                "no support / export consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for BuildRemoteConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
/// Adds the build / remote generic phrasings the spec forbids collapsing into
/// (offline, stale, blocked, expired, rebuilt, remote, managed) to the shared
/// generic-label blocklist. These are matched as *whole* labels rather than
/// substrings so a descriptive banner may still name "managed workspace expired"
/// or "container execution stale" as a boundary state without being flagged; only
/// a banner whose entire label collapses to the generic phrase is rejected.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    if lower.contains("get started") {
        return true;
    }
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
            | "read only"
            | "read-only"
            | "offline"
            | "stale"
            | "blocked"
            | "loading"
            | "content"
            | "expired"
            | "rebuilt"
            | "remote"
            | "managed"
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

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_build_remote_boundary_component_consumers_packet() -> BuildRemoteConsumerPacket {
    BuildRemoteConsumerPacket::new(BuildRemoteConsumerPacketInput {
        packet_id: "m5-build-remote-boundary-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: BUILD_REMOTE_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:build-remote-boundary-consumer:{id}")]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn banner(
    id: &str,
    label: &str,
    authority: AuthorityMode,
    missing: &[&str],
) -> ReducedCapabilityBanner {
    ReducedCapabilityBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        capability_state: authority.capability_state().to_owned(),
        missing_capabilities: missing.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: M5BuildRemoteConsumerSurface,
    component_family: M5BuildRemoteBoundaryComponentFamily,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> BuildRemoteConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    let controls_lane = controls_lane_for(component_family);
    BuildRemoteConsumerRow {
        record_kind: BUILD_REMOTE_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: BUILD_REMOTE_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_class: consumer_class_for(consumer_surface),
        consumer_surface,
        component_family,
        controls_lane,
        canonical_family_schema_ref: canonical_family_schema_ref_for(component_family).to_owned(),
        canonical_controls_schema_ref: controls_lane.canonical_schema_ref().to_owned(),
        canonical_controls_artifact_refs: vec![controls_lane.canonical_artifact_ref().to_owned()],
        references_canonical_not_local_prose: true,
        boundary_state_ref: format!("boundary-state:{row_id}"),
        authority_mode,
        preserved_label_families: labels(label_families),
        boundary_disposition_vocab: canonical_boundary_disposition_vocab(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        implies_exact_continuity_after_material_change: false,
        hides_local_safe_or_companion_handoff_in_overflow_only: false,
        lower_confidence_overwrites_resolved_target_without_review: false,
        source_refs: vec![
            BUILD_REMOTE_CONSUMER_MATRIX_REF.to_owned(),
            BUILD_REMOTE_CONSUMER_SHARED_SCHEMA_REF.to_owned(),
            controls_lane.canonical_doc_ref().to_owned(),
        ],
        observed_at: "2026-07-11T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<BuildRemoteConsumerRow> {
    use AuthorityMode::*;
    use HandoffTarget as H;
    use M5BuildRemoteBoundaryComponentFamily::*;
    use M5BuildRemoteConsumerSurface::*;

    vec![
        // --- Run / test / debug --------------------------------------------
        row(
            "consumer:run-test-debug:host-boundary-strip",
            RunTestDebugUi,
            HostBoundaryStrip,
            FullInteractive,
            &["host_boundary", "execution_origin", "adapter_confidence"],
            &[
                "host_boundary",
                "execution_origin",
                "adapter_confidence",
                "controls_lane",
            ],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:run-test-debug:execution-origin-receipt-row",
            RunTestDebugUi,
            ExecutionOriginReceiptRow,
            ReadOnly,
            &["execution_origin", "host_boundary", "continuity"],
            &[
                "execution_origin",
                "host_boundary",
                "continuity",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:run-test-debug:execution-origin-receipt-row",
                "Read-only execution-origin receipt: names the origin locus where the run actually executed, the host boundary it crossed, and the continuity relative to the prior runtime; re-running stays in the desktop shell",
                ReadOnly,
                &["rerun_here", "switch_execution_origin"],
            )),
        ),
        row(
            "consumer:run-test-debug:adapter-confidence-chip",
            ShellUi,
            AdapterConfidenceChip,
            ReadOnly,
            &["adapter_confidence", "host_boundary", "discovery_drift"],
            &[
                "adapter_confidence",
                "host_boundary",
                "discovery_drift",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:run-test-debug:adapter-confidence-chip",
                "Read-only adapter-confidence chip in shell status: names the build/runtime adapter's confidence in the resolved target and the claim ceiling it permits; switching the resolved target stays in the desktop shell",
                ReadOnly,
                &["switch_resolved_target", "override_adapter"],
            )),
        ),
        // --- Notebook ------------------------------------------------------
        row(
            "consumer:notebook:adapter-confidence-chip",
            NotebookUi,
            AdapterConfidenceChip,
            ReadOnly,
            &["adapter_confidence", "discovery_drift", "host_boundary"],
            &[
                "adapter_confidence",
                "discovery_drift",
                "host_boundary",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:notebook:adapter-confidence-chip",
                "Read-only adapter-confidence chip on the notebook kernel target: names the adapter confidence behind the resolved kernel and its claim ceiling; changing the kernel target stays in the desktop shell",
                ReadOnly,
                &["switch_resolved_target", "override_adapter"],
            )),
        ),
        row(
            "consumer:notebook:discovery-diff-card",
            NotebookUi,
            DiscoveryDiffCard,
            InspectOnly,
            &["discovery_drift", "adapter_confidence", "host_boundary"],
            &[
                "discovery_drift",
                "adapter_confidence",
                "host_boundary",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:notebook:discovery-diff-card",
                "Inspect-only discovery-diff card: names heuristic-vs-resolved kernel-target drift and the review state that governs it, so lower-confidence discovery never overwrites the resolved kernel without review",
                InspectOnly,
                &["accept_discovered_target", "switch_resolved_target"],
            )),
        ),
        // --- Preview -------------------------------------------------------
        row(
            "consumer:preview:managed-workspace-lifecycle-card",
            PreviewUi,
            ManagedWorkspaceLifecycleCard,
            ReadOnly,
            &["lifecycle_state", "continuity", "expiry_timing"],
            &[
                "lifecycle_state",
                "continuity",
                "expiry_timing",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:preview:managed-workspace-lifecycle-card",
                "Read-only managed-workspace lifecycle card behind the preview: names the workspace lifecycle state, the changed persistence class, and the continuity relative to the prior runtime; suspend/resume/rebuild stays in the desktop shell",
                ReadOnly,
                &["suspend_workspace", "rebuild_workspace"],
            )),
        ),
        row(
            "consumer:preview:workspace-expiry-banner",
            PreviewUi,
            WorkspaceExpiryBanner,
            ReadOnly,
            &["expiry_timing", "lifecycle_state", "continuity"],
            &[
                "expiry_timing",
                "lifecycle_state",
                "continuity",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:preview:workspace-expiry-banner",
                "Read-only workspace-expiry banner over the preview: names the expiry timing that governs the managed workspace and the live state that will be lost, so an expiring preview never reads as a durable local target",
                ReadOnly,
                &["renew_workspace", "export_before_loss"],
            )),
        ),
        // --- AI tool routing (product chrome) ------------------------------
        row(
            "consumer:ai-tool-routing:execution-origin-receipt-row",
            ProductUi,
            ExecutionOriginReceiptRow,
            InspectOnly,
            &["execution_origin", "host_boundary", "continuity"],
            &[
                "execution_origin",
                "host_boundary",
                "continuity",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:ai-tool-routing:execution-origin-receipt-row",
                "Inspect-only execution-origin receipt for AI tool routing: names the origin locus where the routed tool call actually ran and the host boundary it crossed, so a routed action never reads as local first-party execution",
                InspectOnly,
                &["rerun_here", "switch_execution_origin"],
            )),
        ),
        row(
            "consumer:ai-tool-routing:discovery-diff-card",
            ProductUi,
            DiscoveryDiffCard,
            ReadOnly,
            &["discovery_drift", "adapter_confidence", "execution_origin"],
            &[
                "discovery_drift",
                "adapter_confidence",
                "execution_origin",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:ai-tool-routing:discovery-diff-card",
                "Read-only discovery-diff card for AI tool routing: names heuristic-vs-resolved target drift for a routed tool, so lower-confidence discovery never silently overwrites the resolved target the model acts against",
                ReadOnly,
                &["accept_discovered_target", "switch_resolved_target"],
            )),
        ),
        // --- Companion handoff ---------------------------------------------
        row(
            "consumer:companion-handoff:local-safe-continuation-card",
            CompanionUi,
            LocalSafeContinuationCard,
            OverrideGated,
            &["local_safe_continuation", "continuity", "host_boundary"],
            &[
                "local_safe_continuation",
                "continuity",
                "host_boundary",
                "controls_lane",
            ],
            H::CompanionApp,
            "handoff:companion-handoff:local-safe-continuation-card-companion",
            Some(banner(
                "banner:companion-handoff:local-safe-continuation-card",
                "Review-gated local-safe continuation card in the companion: names the local-safe continuation offered when managed continuity is unavailable and its explicit caveats, never presented in an overflow-only menu and never implying exact continuity",
                OverrideGated,
                &["resume_managed_exact", "discard_local_mirror"],
            )),
        ),
        row(
            "consumer:companion-handoff:suspend-resume-rebuild-review-sheet",
            CompanionUi,
            SuspendResumeRebuildReviewSheet,
            ReadOnly,
            &["continuity", "persistence_class", "lifecycle_state"],
            &[
                "continuity",
                "persistence_class",
                "lifecycle_state",
                "controls_lane",
            ],
            H::CompanionApp,
            "handoff:companion-handoff:suspend-resume-rebuild-review-sheet-companion",
            Some(banner(
                "banner:companion-handoff:suspend-resume-rebuild-review-sheet",
                "Read-only suspend/resume/rebuild review sheet in the companion: names the lifecycle state, the changed persistence class, and the preserved-vs-lost state so a rebuilt or recreated workspace never reads as exact continuity",
                ReadOnly,
                &["commit_rebuild", "resume_workspace"],
            )),
        ),
        // --- Support / export + release packet (AC2) -----------------------
        row(
            "consumer:support-export:host-boundary-strip",
            SupportExport,
            HostBoundaryStrip,
            ExportOnly,
            &["host_boundary", "execution_origin", "adapter_confidence"],
            &[
                "host_boundary",
                "execution_origin",
                "adapter_confidence",
                "boundary_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:host-boundary-strip-support-packet",
            Some(banner(
                "banner:support-export:host-boundary-strip",
                "Export-only support replay: reconstruct the host boundary the work ran on, the execution origin, and the adapter confidence the user saw from the support packet",
                ExportOnly,
                &["rerun_here", "switch_execution_origin"],
            )),
        ),
        row(
            "consumer:support-export:managed-workspace-lifecycle-card",
            SupportExport,
            ManagedWorkspaceLifecycleCard,
            ExportOnly,
            &["lifecycle_state", "continuity", "persistence_class"],
            &[
                "lifecycle_state",
                "continuity",
                "persistence_class",
                "boundary_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:managed-workspace-lifecycle-card-support-packet",
            Some(banner(
                "banner:support-export:managed-workspace-lifecycle-card",
                "Export-only support replay: reconstruct the workspace lifecycle state, the changed persistence class, and the continuity relative to the prior runtime from the support packet",
                ExportOnly,
                &["suspend_workspace", "rebuild_workspace"],
            )),
        ),
        row(
            "consumer:support-export:suspend-resume-rebuild-review-sheet",
            SupportExport,
            SuspendResumeRebuildReviewSheet,
            ExportOnly,
            &["continuity", "persistence_class", "lifecycle_state"],
            &[
                "continuity",
                "persistence_class",
                "lifecycle_state",
                "boundary_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:suspend-resume-rebuild-review-sheet-support-packet",
            Some(banner(
                "banner:support-export:suspend-resume-rebuild-review-sheet",
                "Export-only support replay: reconstruct the suspend/resume/rebuild review state, the preserved-vs-lost state, and that a rebuilt or recreated workspace was never claimed as exact continuity from the support packet",
                ExportOnly,
                &["commit_rebuild", "resume_workspace"],
            )),
        ),
        // --- Incident / diagnostics (support class) ------------------------
        row(
            "consumer:incident:workspace-expiry-banner",
            IncidentUi,
            WorkspaceExpiryBanner,
            ReadOnly,
            &["expiry_timing", "lifecycle_state", "local_safe_continuation"],
            &[
                "expiry_timing",
                "lifecycle_state",
                "local_safe_continuation",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:incident:workspace-expiry-banner",
                "Read-only workspace-expiry banner in the incident timeline: names the expiry timing that triggered the incident and the local-safe continuation offered, so an expired workspace never reads as a generic disconnect",
                ReadOnly,
                &["renew_workspace", "export_before_loss"],
            )),
        ),
    ]
}
