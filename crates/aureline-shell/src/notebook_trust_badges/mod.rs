//! Bounded notebook-trust-badge and representation-state wedge.
//!
//! ## What the wedge is for
//!
//! Notebook-like and rich-output surfaces collapse trust into a single
//! `trusted / not trusted` chip if nothing forces them to keep the axes
//! visibly separate. The four-axis trust posture in
//! [`docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`] —
//! workspace trust, document/notebook trust, kernel availability, output
//! trust, and widget trust — exists precisely to prevent that collapse.
//! This wedge is the first M1 surface that renders all of those axes as
//! distinct badge rows alongside the cell-level *representation state*
//! (raw / sanitized / sandboxed-active / tombstone-static-fallback /
//! blocked-metadata-only) on one bounded notebook-like preview row.
//!
//! ## Why a typed record rather than a static chip
//!
//! The protected walks (fully trusted local notebook with a code cell, and
//! a mixed-trust untrusted notebook with the same cell shape) and the
//! failure drill ("a buggy caller claims a widget cell will autoexecute on
//! open") both need a record that can refuse to flatten the axes. A static
//! "Trusted ✅" / "Untrusted ⚠️" badge cannot prove that the four axes were
//! kept distinct, that the wedge did not autoexecute active content on
//! notebook open, or that the safe-preview escape hatch was preserved
//! when the rendered representation could not be trusted. A typed
//! [`NotebookTrustBadgeCardRecord`] with a closed
//! [`NotebookTrustBadgeInvariantViolation`] vocabulary can.
//!
//! ## Reused vocabularies
//!
//! - [`NotebookTrustRung`] mirrors the frozen `notebook_trust_rung`
//!   vocabulary from
//!   [`docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`]
//!   verbatim. The wedge does not invent surface-local rung names.
//! - [`crate::state_cards::DegradedStateToken`] (Warming / Limited /
//!   PolicyBlocked / Offline / etc.) supplies the chrome chip on rows that
//!   cannot render full fidelity.
//! - [`RepresentationState`] is a superset of
//!   [`aureline_content_safety::RepresentationClass`] adapted to notebook /
//!   rich-output surfaces, so the wedge speaks the same raw / sanitized /
//!   escaped / sandboxed / blocked vocabulary the safe-preview wedge
//!   speaks, with the notebook-specific `tombstone_static_fallback`
//!   addition for widget downgrades.
//!
//! ## Bounded scope (deliberately)
//!
//! - Only one notebook-like preview row is the certified wedge in M1. The
//!   wedge does not stand up a notebook editor / runtime / kernel
//!   transport. The [`NotebookTrustBadgeClaimLimit::SingleBoundedWedgeOnly`]
//!   row is rendered under every card so this is explicit.
//! - The wedge never autoexecutes active content on notebook open. The
//!   `record_open_*` constructors set `will_autoexecute_on_open = false`
//!   on every row, and a buggy caller that tries to flip the bit lands
//!   the typed
//!   [`NotebookTrustBadgeInvariantViolation::AutoexecuteOnOpen`] failure.
//! - The wedge does not own the widget admission pipeline, diff/repair
//!   engines, or rich-output sandbox productization. It records the trust
//!   posture and representation state the chrome quotes verbatim.

use serde::{Deserialize, Serialize};

use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag carried in serialized [`NotebookTrustBadgeCardRecord`].
pub const NOTEBOOK_TRUST_BADGE_CARD_RECORD_KIND: &str = "notebook_trust_badge_card_record";

/// Schema version for the [`NotebookTrustBadgeCardRecord`] payload shape.
pub const NOTEBOOK_TRUST_BADGE_CARD_SCHEMA_VERSION: u32 = 1;

/// Prototype label carried on every card. Chrome quotes the token verbatim
/// and MUST NOT drop the chip even when the notebook is nominally trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// Bounded M1 prototype: notebook-trust badges and representation-state
    /// cues on one bounded certified wedge.
    M1PrototypeNotebookTrustBadgesAndRepresentationState,
}

impl PrototypeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeNotebookTrustBadgesAndRepresentationState => {
                "m1_prototype_notebook_trust_badges_and_representation_state"
            }
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeNotebookTrustBadgesAndRepresentationState => {
                "Prototype — notebook trust badges & representation state"
            }
        }
    }
}

/// Workspace trust posture — separate axis from notebook trust. Mirrored
/// from the workspace-trust vocabulary used across shell surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceTrustState {
    /// User or admin policy trusts the workspace.
    TrustedWorkspace,
    /// Restricted workspace; admission to elevated capabilities denied.
    RestrictedWorkspace,
    /// Workspace trust has not been resolved yet (warming / pending).
    UnknownWorkspace,
}

impl WorkspaceTrustState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedWorkspace => "trusted_workspace",
            Self::RestrictedWorkspace => "restricted_workspace",
            Self::UnknownWorkspace => "unknown_workspace",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::TrustedWorkspace => "Trusted workspace",
            Self::RestrictedWorkspace => "Restricted workspace",
            Self::UnknownWorkspace => "Workspace trust unknown",
        }
    }
}

/// Notebook trust ladder rung. Mirrors `notebook_trust_rung` from
/// `docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookTrustRung {
    UntrustedTainted,
    UntrustedQuarantinedForReview,
    StructuralOnlyTrusted,
    SelectiveCellTrust,
    FullyTrustedUser,
    FullyTrustedWorkspacePolicy,
    TrustRevokedPendingReview,
}

impl NotebookTrustRung {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UntrustedTainted => "untrusted_tainted",
            Self::UntrustedQuarantinedForReview => "untrusted_quarantined_for_review",
            Self::StructuralOnlyTrusted => "structural_only_trusted",
            Self::SelectiveCellTrust => "selective_cell_trust",
            Self::FullyTrustedUser => "fully_trusted_user",
            Self::FullyTrustedWorkspacePolicy => "fully_trusted_workspace_policy",
            Self::TrustRevokedPendingReview => "trust_revoked_pending_review",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::UntrustedTainted => "Untrusted (tainted)",
            Self::UntrustedQuarantinedForReview => "Untrusted (quarantined for review)",
            Self::StructuralOnlyTrusted => "Structural-only trusted",
            Self::SelectiveCellTrust => "Selective cell trust",
            Self::FullyTrustedUser => "Fully trusted (user)",
            Self::FullyTrustedWorkspacePolicy => "Fully trusted (workspace policy)",
            Self::TrustRevokedPendingReview => "Trust revoked — pending review",
        }
    }

    /// Returns `true` when a rung explicitly grants elevated trust.
    pub const fn is_fully_trusted(self) -> bool {
        matches!(
            self,
            Self::FullyTrustedUser | Self::FullyTrustedWorkspacePolicy
        )
    }

    /// Returns `true` when active content (code cell execution, widget live
    /// binding) MUST stay denied at this rung — even on user-initiated
    /// actions like notebook open. The chrome MAY surface explicit
    /// admit/elevate affordances at higher rungs but never auto-runs.
    pub const fn denies_active_content_by_default(self) -> bool {
        !self.is_fully_trusted()
    }
}

/// Kernel availability — separate axis from notebook/document trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelAvailability {
    /// Surface does not own a kernel (e.g. markdown-only notebook).
    NotApplicable,
    /// Local managed-toolchain kernel is available.
    LocalManagedAvailable,
    /// Local managed-toolchain kernel is not currently available.
    LocalManagedUnavailable,
    /// Remote managed kernel is available.
    RemoteManagedAvailable,
    /// Remote managed kernel is not currently available.
    RemoteManagedUnavailable,
    /// Org or trust policy denies kernel attachment in this context.
    KernelDeniedByPolicy,
}

impl KernelAvailability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::LocalManagedAvailable => "local_managed_available",
            Self::LocalManagedUnavailable => "local_managed_unavailable",
            Self::RemoteManagedAvailable => "remote_managed_available",
            Self::RemoteManagedUnavailable => "remote_managed_unavailable",
            Self::KernelDeniedByPolicy => "kernel_denied_by_policy",
        }
    }

    pub const fn is_available(self) -> bool {
        matches!(
            self,
            Self::LocalManagedAvailable | Self::RemoteManagedAvailable
        )
    }
}

/// Output trust — separate axis from notebook/document trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputTrustState {
    /// No outputs are part of this preview row.
    NotApplicable,
    /// Outputs are live from the current kernel session.
    LiveFromCurrentSession,
    /// Outputs are captured evidence from a prior kernel session.
    CapturedFromPriorSession,
    /// Outputs are replayed from a saved snapshot.
    ReplayedFromSnapshot,
    /// Outputs exist but have no producing session anchor (orphaned).
    OrphanedOutput,
    /// Output rendering is gated behind explicit widget admission.
    WidgetGated,
}

impl OutputTrustState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::LiveFromCurrentSession => "live_from_current_session",
            Self::CapturedFromPriorSession => "captured_from_prior_session",
            Self::ReplayedFromSnapshot => "replayed_from_snapshot",
            Self::OrphanedOutput => "orphaned_output",
            Self::WidgetGated => "widget_gated",
        }
    }
}

/// Widget trust — separate axis from notebook/document/output trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetTrustState {
    /// Surface does not host widgets.
    NotApplicable,
    /// Widget live binding is denied by default at the current trust rung.
    WidgetDeniedByDefault,
    /// User explicitly admitted widget live binding after a preview.
    WidgetAdmittedAfterPreview,
    /// Widget binding suppressed by org or local trust policy.
    WidgetSuppressedByPolicy,
    /// The widget's declared content class is blocked.
    WidgetContentClassBlocked,
    /// Widget runtime (extension / kernel-side) is unavailable.
    WidgetRuntimeUnavailable,
}

impl WidgetTrustState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::WidgetDeniedByDefault => "widget_denied_by_default",
            Self::WidgetAdmittedAfterPreview => "widget_admitted_after_preview",
            Self::WidgetSuppressedByPolicy => "widget_suppressed_by_policy",
            Self::WidgetContentClassBlocked => "widget_content_class_blocked",
            Self::WidgetRuntimeUnavailable => "widget_runtime_unavailable",
        }
    }
}

/// Cell / output content class for the per-row badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CellContentClass {
    /// Markdown cell (no active content).
    MarkdownCell,
    /// Code cell (potentially active content).
    CodeCell,
    /// Rich text / image / table output (no live binding).
    RichOutput,
    /// Widget output (potentially live binding).
    WidgetOutput,
}

impl CellContentClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarkdownCell => "markdown_cell",
            Self::CodeCell => "code_cell",
            Self::RichOutput => "rich_output",
            Self::WidgetOutput => "widget_output",
        }
    }

    /// Returns `true` for content classes that can carry active content
    /// (code execution, widget live binding). These rows MUST render with
    /// raw / escaped / sandboxed-active / tombstone representations rather
    /// than auto-running on open.
    pub const fn can_carry_active_content(self) -> bool {
        matches!(self, Self::CodeCell | Self::WidgetOutput)
    }
}

/// Representation state — what the user is currently seeing on the row.
///
/// This is the notebook-surface equivalent of
/// [`aureline_preview::CurrentlyVisibleRepresentation`] from the
/// safe-preview wedge, plus the notebook-specific
/// `tombstone_static_fallback` case for widgets that were downgraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationState {
    /// Raw cell source / output bytes rendered as plain text.
    Raw,
    /// Sanitized rendered view; active content removed.
    Sanitized,
    /// Suspicious codepoints escaped for inspection.
    Escaped,
    /// Active content rendered inside an explicit sandbox boundary.
    SandboxedActive,
    /// Widget / live output downgraded to a static fallback chip.
    TombstoneStaticFallback,
    /// Body withheld; only typed metadata is shown.
    BlockedMetadataOnly,
}

impl RepresentationState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::Sanitized => "sanitized",
            Self::Escaped => "escaped",
            Self::SandboxedActive => "sandboxed_active",
            Self::TombstoneStaticFallback => "tombstone_static_fallback",
            Self::BlockedMetadataOnly => "blocked_metadata_only",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Raw => "Raw source",
            Self::Sanitized => "Sanitized rendered",
            Self::Escaped => "Escaped (safe inspection)",
            Self::SandboxedActive => "Sandboxed active content",
            Self::TombstoneStaticFallback => "Static fallback (tombstone)",
            Self::BlockedMetadataOnly => "Metadata only — body withheld",
        }
    }

    /// Returns `true` when this representation lets the user view active
    /// content live. Only `SandboxedActive` does.
    pub const fn renders_active_content(self) -> bool {
        matches!(self, Self::SandboxedActive)
    }
}

/// Closed escape-hatch vocabulary. The chrome MUST offer at least one
/// safe-preview escape hatch on any row whose representation is not
/// trusted-rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscapeHatch {
    /// Safe-preview path on the bounded copy/export card.
    SafePreview,
    /// Hand off to the user's default browser for rich rendering.
    OpenInBrowser,
    /// Hand off to a separate desktop viewer for rich rendering.
    OpenInDesktop,
    /// Export an exact-source copy for offline inspection.
    ExportRawSource,
    /// Keep the current static-fallback rendering without elevating trust.
    KeepStaticFallback,
}

impl EscapeHatch {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafePreview => "safe_preview",
            Self::OpenInBrowser => "open_in_browser",
            Self::OpenInDesktop => "open_in_desktop",
            Self::ExportRawSource => "export_raw_source",
            Self::KeepStaticFallback => "keep_static_fallback",
        }
    }
}

/// Frozen claim-limit vocabulary the chrome quotes verbatim under every
/// card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookTrustBadgeClaimLimit {
    /// One bounded wedge only.
    SingleBoundedWedgeOnly,
    /// Notebook open MUST NOT autoexecute active content.
    NoAutoexecuteOnOpen,
    /// Trust axes (workspace / notebook / kernel / output / widget) MUST
    /// remain visibly distinct.
    TrustAxesRemainDistinct,
    /// Wedge does not own kernel runtime, attachment, or transport.
    NoKernelOrTransportOrchestration,
    /// Wedge does not own widget admission, diff/repair engines, or
    /// rich-output sandbox productization.
    NoWidgetAdmissionPipeline,
}

impl NotebookTrustBadgeClaimLimit {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleBoundedWedgeOnly => "single_bounded_wedge_only",
            Self::NoAutoexecuteOnOpen => "no_autoexecute_on_open",
            Self::TrustAxesRemainDistinct => "trust_axes_remain_distinct",
            Self::NoKernelOrTransportOrchestration => "no_kernel_or_transport_orchestration",
            Self::NoWidgetAdmissionPipeline => "no_widget_admission_pipeline",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleBoundedWedgeOnly => {
                "One bounded notebook-like wedge only; not a notebook editor or runtime."
            }
            Self::NoAutoexecuteOnOpen => {
                "Notebook open never autoexecutes active content."
            }
            Self::TrustAxesRemainDistinct => {
                "Workspace, notebook, kernel, output, and widget trust remain visibly distinct."
            }
            Self::NoKernelOrTransportOrchestration => {
                "Does not own kernel runtime, attachment, or transport."
            }
            Self::NoWidgetAdmissionPipeline => {
                "Does not own widget admission, diff/repair engines, or rich-output sandbox productization."
            }
        }
    }

    /// Canonical M1 claim-limit set. Order is stable; chrome MUST render
    /// in this order.
    pub const fn canonical_set() -> [NotebookTrustBadgeClaimLimit; 5] {
        [
            Self::SingleBoundedWedgeOnly,
            Self::NoAutoexecuteOnOpen,
            Self::TrustAxesRemainDistinct,
            Self::NoKernelOrTransportOrchestration,
            Self::NoWidgetAdmissionPipeline,
        ]
    }
}

/// One claim-limit row on the serialized card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookTrustBadgeClaimLimitRow {
    pub token: String,
    pub label: String,
}

impl NotebookTrustBadgeClaimLimitRow {
    fn from_limit(limit: NotebookTrustBadgeClaimLimit) -> Self {
        Self {
            token: limit.as_str().to_owned(),
            label: limit.label().to_owned(),
        }
    }
}

/// Closed invariant-violation vocabulary surfaced on the card.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "violation")]
pub enum NotebookTrustBadgeInvariantViolation {
    /// The card is missing the prototype-label chip.
    MissingPrototypeLabel,
    /// The canonical claim-limit set is missing or out of order.
    ClaimLimitsMissingOrOutOfOrder,
    /// A row claims it will autoexecute on open. The wedge refuses every
    /// such claim — notebook open never runs active content.
    AutoexecuteOnOpen { row_id: String },
    /// A row hosts active content (code cell / widget output) on an
    /// untrusted / quarantined notebook rung but renders raw or
    /// sandboxed-active — the chrome would expose live content under an
    /// untrusted rung.
    ActiveContentOnUntrustedRung {
        row_id: String,
        representation: String,
        notebook_trust_rung: String,
    },
    /// A row hosts active content but has no safe-preview escape hatch.
    /// The chrome would have nowhere to send the user.
    MissingSafePreviewEscapeHatch { row_id: String },
    /// A widget-output row reports widget trust as
    /// `not_applicable`. Widgets MUST always carry a widget-trust value
    /// so the chrome cannot accidentally render live binding.
    WidgetTrustNotApplicableForWidget { row_id: String },
    /// Outputs claim to be live from the current session but no kernel is
    /// available. The chrome would render orphaned live outputs.
    LiveOutputsWithoutKernel {
        output_trust_state: String,
        kernel_availability: String,
    },
    /// All trust axes were collapsed onto the notebook rung — workspace,
    /// kernel, output, or widget state would mirror notebook trust
    /// without being independently set.
    TrustAxesCollapsed {
        notebook_trust_rung: String,
        collapsed_axes: Vec<String>,
    },
}

impl NotebookTrustBadgeInvariantViolation {
    pub fn token(&self) -> &'static str {
        match self {
            Self::MissingPrototypeLabel => "missing_prototype_label",
            Self::ClaimLimitsMissingOrOutOfOrder => "claim_limits_missing_or_out_of_order",
            Self::AutoexecuteOnOpen { .. } => "autoexecute_on_open",
            Self::ActiveContentOnUntrustedRung { .. } => "active_content_on_untrusted_rung",
            Self::MissingSafePreviewEscapeHatch { .. } => "missing_safe_preview_escape_hatch",
            Self::WidgetTrustNotApplicableForWidget { .. } => {
                "widget_trust_not_applicable_for_widget"
            }
            Self::LiveOutputsWithoutKernel { .. } => "live_outputs_without_kernel",
            Self::TrustAxesCollapsed { .. } => "trust_axes_collapsed",
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::MissingPrototypeLabel => "Prototype-label chip is missing.".to_owned(),
            Self::ClaimLimitsMissingOrOutOfOrder => {
                "Canonical claim-limit set is missing or out of order.".to_owned()
            }
            Self::AutoexecuteOnOpen { row_id } => format!(
                "Row {row_id} claims it will autoexecute active content on notebook open."
            ),
            Self::ActiveContentOnUntrustedRung {
                row_id,
                representation,
                notebook_trust_rung,
            } => format!(
                "Row {row_id} hosts active content with representation={representation} under \
                 notebook_trust_rung={notebook_trust_rung}; the row would expose live content \
                 on an untrusted rung."
            ),
            Self::MissingSafePreviewEscapeHatch { row_id } => format!(
                "Row {row_id} hosts active content but offers no safe-preview escape hatch."
            ),
            Self::WidgetTrustNotApplicableForWidget { row_id } => format!(
                "Row {row_id} is a widget output but widget_trust_state=not_applicable; widgets \
                 must always carry a widget-trust value."
            ),
            Self::LiveOutputsWithoutKernel {
                output_trust_state,
                kernel_availability,
            } => format!(
                "Outputs report {output_trust_state} but kernel availability is {kernel_availability}."
            ),
            Self::TrustAxesCollapsed {
                notebook_trust_rung,
                collapsed_axes,
            } => format!(
                "Trust axes collapsed onto notebook_trust_rung={notebook_trust_rung}: \
                 {axes}.",
                axes = collapsed_axes.join(",")
            ),
        }
    }
}

/// One invariant row on the serialized card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookTrustBadgeInvariantRow {
    pub violation_token: String,
    pub violation_label: String,
    pub violation: NotebookTrustBadgeInvariantViolation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addressable_row_id: Option<String>,
}

impl NotebookTrustBadgeInvariantRow {
    fn from_violation(violation: NotebookTrustBadgeInvariantViolation) -> Self {
        let addressable_row_id = match &violation {
            NotebookTrustBadgeInvariantViolation::AutoexecuteOnOpen { row_id }
            | NotebookTrustBadgeInvariantViolation::ActiveContentOnUntrustedRung {
                row_id,
                ..
            }
            | NotebookTrustBadgeInvariantViolation::MissingSafePreviewEscapeHatch { row_id }
            | NotebookTrustBadgeInvariantViolation::WidgetTrustNotApplicableForWidget {
                row_id,
            } => Some(row_id.clone()),
            _ => None,
        };
        Self {
            violation_token: violation.token().to_owned(),
            violation_label: violation.label(),
            violation,
            addressable_row_id,
        }
    }
}

/// Per-row badge record carrying the cell- or output-level trust axes and
/// representation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookTrustBadgeRow {
    pub row_id: String,
    pub cell_or_output_ref: String,
    pub content_class: CellContentClass,
    pub content_class_token: String,
    pub representation_state: RepresentationState,
    pub representation_state_token: String,
    pub representation_state_label: String,
    /// True when the chrome surfaces a visible honesty marker on this row
    /// (e.g. "Escaped — review before copy" or "Static fallback").
    pub honesty_marker_present: bool,
    /// Per-row degraded chip mapped through the shared chrome vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    /// Escape hatches the chrome offers next to the row.
    pub escape_hatches: Vec<String>,
    /// Per-row override of `widget_trust_state` for widget outputs in a
    /// selectively trusted notebook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_trust_override_token: Option<String>,
    /// MUST be `false` on every protected walk: notebook open never auto-
    /// runs active content. A buggy caller that sets this to `true` lands
    /// the `autoexecute_on_open` invariant.
    pub will_autoexecute_on_open: bool,
}

/// Builder for one row.
#[derive(Debug, Clone)]
pub struct NotebookTrustBadgeRowBuilder {
    row_id: String,
    cell_or_output_ref: String,
    content_class: CellContentClass,
    representation_state: RepresentationState,
    honesty_marker_present: bool,
    degraded_token: Option<DegradedStateToken>,
    escape_hatches: Vec<EscapeHatch>,
    widget_trust_override: Option<WidgetTrustState>,
    will_autoexecute_on_open: bool,
}

impl NotebookTrustBadgeRowBuilder {
    pub fn new(
        row_id: impl Into<String>,
        cell_or_output_ref: impl Into<String>,
        content_class: CellContentClass,
        representation_state: RepresentationState,
    ) -> Self {
        Self {
            row_id: row_id.into(),
            cell_or_output_ref: cell_or_output_ref.into(),
            content_class,
            representation_state,
            honesty_marker_present: false,
            degraded_token: None,
            escape_hatches: Vec::new(),
            widget_trust_override: None,
            will_autoexecute_on_open: false,
        }
    }

    pub fn with_honesty_marker(mut self, present: bool) -> Self {
        self.honesty_marker_present = present;
        self
    }

    pub fn with_degraded(mut self, token: DegradedStateToken) -> Self {
        self.degraded_token = Some(token);
        self
    }

    pub fn with_escape_hatches(mut self, hatches: impl IntoIterator<Item = EscapeHatch>) -> Self {
        self.escape_hatches = hatches.into_iter().collect();
        self
    }

    pub fn with_widget_trust_override(mut self, state: WidgetTrustState) -> Self {
        self.widget_trust_override = Some(state);
        self
    }

    /// Used by the failure drill: a buggy caller flips this bit to claim
    /// the row will autoexecute on open. The wedge surfaces the typed
    /// `AutoexecuteOnOpen` invariant against the row.
    pub fn with_will_autoexecute_on_open(mut self, value: bool) -> Self {
        self.will_autoexecute_on_open = value;
        self
    }

    fn build(self) -> NotebookTrustBadgeRow {
        NotebookTrustBadgeRow {
            row_id: self.row_id,
            cell_or_output_ref: self.cell_or_output_ref,
            content_class_token: self.content_class.as_str().to_owned(),
            content_class: self.content_class,
            representation_state_token: self.representation_state.as_str().to_owned(),
            representation_state_label: self.representation_state.label().to_owned(),
            representation_state: self.representation_state,
            honesty_marker_present: self.honesty_marker_present,
            degraded_token: self.degraded_token.map(|t| t.token().to_owned()),
            escape_hatches: self
                .escape_hatches
                .iter()
                .map(|h| h.as_str().to_owned())
                .collect(),
            widget_trust_override_token: self
                .widget_trust_override
                .map(|w| w.as_str().to_owned()),
            will_autoexecute_on_open: self.will_autoexecute_on_open,
        }
    }
}

/// Serialized card record. Chrome quotes verbatim; export and proof flows
/// quote verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookTrustBadgeCardRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub prototype_label_token: String,
    pub prototype_label_display: String,
    pub workspace_id: String,
    pub notebook_ref: String,
    pub wedge_id: String,
    pub workspace_trust_state: WorkspaceTrustState,
    pub workspace_trust_state_token: String,
    pub workspace_trust_state_label: String,
    pub notebook_trust_rung: NotebookTrustRung,
    pub notebook_trust_rung_token: String,
    pub notebook_trust_rung_label: String,
    pub kernel_availability: KernelAvailability,
    pub kernel_availability_token: String,
    pub output_trust_state: OutputTrustState,
    pub output_trust_state_token: String,
    pub widget_trust_state: WidgetTrustState,
    pub widget_trust_state_token: String,
    pub rows: Vec<NotebookTrustBadgeRow>,
    pub claim_limits: Vec<NotebookTrustBadgeClaimLimitRow>,
    pub invariants: Vec<NotebookTrustBadgeInvariantRow>,
    pub has_invariant_violations: bool,
    /// True when any row claims it will autoexecute on open. The wedge
    /// refuses this; the chrome MUST surface the failure verbatim. The
    /// field is duplicated alongside the invariant list so a fixture can
    /// assert on it directly.
    pub any_row_claims_autoexecute_on_open: bool,
    pub summary_line: String,
}

impl NotebookTrustBadgeCardRecord {
    /// Deterministic plaintext block for support exports and proof
    /// captures. Stable across hosts; never bakes in wall-clock time.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {}\n",
            self.prototype_label_token, self.prototype_label_display,
        ));
        out.push_str(&format!(
            "wedge={} workspace={} notebook={}\n",
            self.wedge_id, self.workspace_id, self.notebook_ref,
        ));
        out.push_str("trust_axes:\n");
        out.push_str(&format!(
            "  workspace={} notebook={} kernel={} output={} widget={}\n",
            self.workspace_trust_state_token,
            self.notebook_trust_rung_token,
            self.kernel_availability_token,
            self.output_trust_state_token,
            self.widget_trust_state_token,
        ));
        out.push_str("rows:\n");
        for row in &self.rows {
            out.push_str(&format!(
                "  - id={} content={} representation={} autoexecute_on_open={}",
                row.row_id,
                row.content_class_token,
                row.representation_state_token,
                row.will_autoexecute_on_open,
            ));
            if let Some(token) = &row.degraded_token {
                out.push_str(&format!(" degraded={}", token));
            }
            if row.honesty_marker_present {
                out.push_str(" honesty_marker=true");
            }
            if !row.escape_hatches.is_empty() {
                out.push_str(&format!(" escape_hatches=[{}]", row.escape_hatches.join(",")));
            }
            if let Some(token) = &row.widget_trust_override_token {
                out.push_str(&format!(" widget_trust_override={}", token));
            }
            out.push('\n');
        }
        out.push_str("claim_limits:\n");
        for row in &self.claim_limits {
            out.push_str(&format!("  - {}: {}\n", row.token, row.label));
        }
        out.push_str("invariants:\n");
        if self.invariants.is_empty() {
            out.push_str("  - clean\n");
        } else {
            for row in &self.invariants {
                let suffix = row
                    .addressable_row_id
                    .as_deref()
                    .map(|id| format!(" (row={id})"))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "  - {}: {}{}\n",
                    row.violation_token, row.violation_label, suffix
                ));
            }
        }
        out.push_str(&format!("summary: {}\n", self.summary_line));
        out
    }

    /// True when the card represents a clean trusted-local protected walk:
    /// no invariant violations, no honesty markers on any row, no row
    /// claims autoexecute on open.
    pub fn is_clean_trusted_local(&self) -> bool {
        !self.has_invariant_violations
            && !self.any_row_claims_autoexecute_on_open
            && self.rows.iter().all(|row| !row.honesty_marker_present)
    }
}

/// Bounded notebook-trust-badge wedge.
///
/// Construct with [`NotebookTrustBadgeWedge::new`], configure the four
/// notebook-trust axes (workspace / notebook / kernel / output / widget),
/// add per-row badges, then call [`Self::card`] to materialise the
/// serialized record.
#[derive(Debug, Clone)]
pub struct NotebookTrustBadgeWedge {
    workspace_id: String,
    notebook_ref: String,
    wedge_id: String,
    workspace_trust_state: WorkspaceTrustState,
    notebook_trust_rung: NotebookTrustRung,
    kernel_availability: KernelAvailability,
    output_trust_state: OutputTrustState,
    widget_trust_state: WidgetTrustState,
    rows: Vec<NotebookTrustBadgeRow>,
}

impl NotebookTrustBadgeWedge {
    pub fn new(workspace_id: impl Into<String>, notebook_ref: impl Into<String>) -> Self {
        let ws = workspace_id.into();
        let nb = notebook_ref.into();
        let wedge_id = format!("notebook_trust_badge_wedge:{ws}:{nb}");
        Self {
            workspace_id: ws,
            notebook_ref: nb,
            wedge_id,
            workspace_trust_state: WorkspaceTrustState::UnknownWorkspace,
            notebook_trust_rung: NotebookTrustRung::UntrustedTainted,
            kernel_availability: KernelAvailability::NotApplicable,
            output_trust_state: OutputTrustState::NotApplicable,
            widget_trust_state: WidgetTrustState::NotApplicable,
            rows: Vec::new(),
        }
    }

    pub fn with_wedge_id(mut self, wedge_id: impl Into<String>) -> Self {
        self.wedge_id = wedge_id.into();
        self
    }

    pub fn with_workspace_trust(mut self, state: WorkspaceTrustState) -> Self {
        self.workspace_trust_state = state;
        self
    }

    pub fn with_notebook_trust_rung(mut self, rung: NotebookTrustRung) -> Self {
        self.notebook_trust_rung = rung;
        self
    }

    pub fn with_kernel_availability(mut self, ka: KernelAvailability) -> Self {
        self.kernel_availability = ka;
        self
    }

    pub fn with_output_trust(mut self, ots: OutputTrustState) -> Self {
        self.output_trust_state = ots;
        self
    }

    pub fn with_widget_trust(mut self, wts: WidgetTrustState) -> Self {
        self.widget_trust_state = wts;
        self
    }

    pub fn add_row(&mut self, row: NotebookTrustBadgeRowBuilder) -> &mut Self {
        self.rows.push(row.build());
        self
    }

    pub fn wedge_id(&self) -> &str {
        &self.wedge_id
    }

    pub fn rows(&self) -> &[NotebookTrustBadgeRow] {
        &self.rows
    }

    /// Materialise the current card.
    pub fn card(&self) -> NotebookTrustBadgeCardRecord {
        let label = PrototypeLabel::M1PrototypeNotebookTrustBadgesAndRepresentationState;
        let claim_limits: Vec<NotebookTrustBadgeClaimLimitRow> =
            NotebookTrustBadgeClaimLimit::canonical_set()
                .into_iter()
                .map(NotebookTrustBadgeClaimLimitRow::from_limit)
                .collect();
        let invariants_raw = self.validate_invariants();
        let invariants: Vec<NotebookTrustBadgeInvariantRow> = invariants_raw
            .into_iter()
            .map(NotebookTrustBadgeInvariantRow::from_violation)
            .collect();
        let has_invariant_violations = !invariants.is_empty();
        let any_row_claims_autoexecute_on_open =
            self.rows.iter().any(|row| row.will_autoexecute_on_open);
        let summary_line = self.summary_line(has_invariant_violations);
        NotebookTrustBadgeCardRecord {
            record_kind: NOTEBOOK_TRUST_BADGE_CARD_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_TRUST_BADGE_CARD_SCHEMA_VERSION,
            prototype_label_token: label.as_str().to_owned(),
            prototype_label_display: label.label().to_owned(),
            workspace_id: self.workspace_id.clone(),
            notebook_ref: self.notebook_ref.clone(),
            wedge_id: self.wedge_id.clone(),
            workspace_trust_state: self.workspace_trust_state,
            workspace_trust_state_token: self.workspace_trust_state.as_str().to_owned(),
            workspace_trust_state_label: self.workspace_trust_state.label().to_owned(),
            notebook_trust_rung: self.notebook_trust_rung,
            notebook_trust_rung_token: self.notebook_trust_rung.as_str().to_owned(),
            notebook_trust_rung_label: self.notebook_trust_rung.label().to_owned(),
            kernel_availability: self.kernel_availability,
            kernel_availability_token: self.kernel_availability.as_str().to_owned(),
            output_trust_state: self.output_trust_state,
            output_trust_state_token: self.output_trust_state.as_str().to_owned(),
            widget_trust_state: self.widget_trust_state,
            widget_trust_state_token: self.widget_trust_state.as_str().to_owned(),
            rows: self.rows.clone(),
            claim_limits,
            invariants,
            has_invariant_violations,
            any_row_claims_autoexecute_on_open,
            summary_line,
        }
    }

    fn summary_line(&self, has_invariant_violations: bool) -> String {
        let suffix = if has_invariant_violations {
            "INVARIANTS BLOCKED"
        } else if self.rows.iter().any(|row| row.honesty_marker_present) {
            "honesty markers present"
        } else {
            "clean"
        };
        format!(
            "{rows} row(s); workspace={ws} notebook={nb} kernel={kn} output={ot} widget={wt} — {suffix}",
            rows = self.rows.len(),
            ws = self.workspace_trust_state.as_str(),
            nb = self.notebook_trust_rung.as_str(),
            kn = self.kernel_availability.as_str(),
            ot = self.output_trust_state.as_str(),
            wt = self.widget_trust_state.as_str(),
            suffix = suffix,
        )
    }

    fn validate_invariants(&self) -> Vec<NotebookTrustBadgeInvariantViolation> {
        let mut out = Vec::new();

        if self.output_trust_state == OutputTrustState::LiveFromCurrentSession
            && !self.kernel_availability.is_available()
        {
            out.push(NotebookTrustBadgeInvariantViolation::LiveOutputsWithoutKernel {
                output_trust_state: self.output_trust_state.as_str().to_owned(),
                kernel_availability: self.kernel_availability.as_str().to_owned(),
            });
        }

        let collapsed_axes = self.collapsed_axes();
        if !collapsed_axes.is_empty() {
            out.push(NotebookTrustBadgeInvariantViolation::TrustAxesCollapsed {
                notebook_trust_rung: self.notebook_trust_rung.as_str().to_owned(),
                collapsed_axes,
            });
        }

        for row in &self.rows {
            if row.will_autoexecute_on_open {
                out.push(NotebookTrustBadgeInvariantViolation::AutoexecuteOnOpen {
                    row_id: row.row_id.clone(),
                });
            }
            if row.content_class.can_carry_active_content()
                && self.notebook_trust_rung.denies_active_content_by_default()
                && row.representation_state.renders_active_content()
            {
                out.push(
                    NotebookTrustBadgeInvariantViolation::ActiveContentOnUntrustedRung {
                        row_id: row.row_id.clone(),
                        representation: row.representation_state.as_str().to_owned(),
                        notebook_trust_rung: self.notebook_trust_rung.as_str().to_owned(),
                    },
                );
            }
            if row.content_class.can_carry_active_content() && row.escape_hatches.is_empty() {
                out.push(
                    NotebookTrustBadgeInvariantViolation::MissingSafePreviewEscapeHatch {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.content_class == CellContentClass::WidgetOutput {
                let effective_widget_trust = row
                    .widget_trust_override_token
                    .as_deref()
                    .unwrap_or(self.widget_trust_state.as_str());
                if effective_widget_trust == WidgetTrustState::NotApplicable.as_str() {
                    out.push(
                        NotebookTrustBadgeInvariantViolation::WidgetTrustNotApplicableForWidget {
                            row_id: row.row_id.clone(),
                        },
                    );
                }
            }
        }

        out
    }

    /// Returns the names of axes that mirror the notebook trust rung
    /// rather than carrying an independent value. Workspace trust collapse
    /// is flagged when the notebook is `fully_trusted_user` and workspace
    /// trust is left `unknown_workspace`; widget trust collapse is flagged
    /// when the notebook hosts widgets (any `widget_output` row) but
    /// `widget_trust_state = not_applicable`. Kernel and output collapse
    /// follow the same shape.
    fn collapsed_axes(&self) -> Vec<String> {
        let mut collapsed = Vec::new();
        let has_widget_rows = self
            .rows
            .iter()
            .any(|row| row.content_class == CellContentClass::WidgetOutput);
        let has_code_rows = self
            .rows
            .iter()
            .any(|row| row.content_class == CellContentClass::CodeCell);
        let has_outputs = !matches!(self.output_trust_state, OutputTrustState::NotApplicable);

        if self.notebook_trust_rung.is_fully_trusted()
            && self.workspace_trust_state == WorkspaceTrustState::UnknownWorkspace
        {
            collapsed.push("workspace_trust_state".to_owned());
        }
        if has_code_rows
            && matches!(self.kernel_availability, KernelAvailability::NotApplicable)
        {
            collapsed.push("kernel_availability".to_owned());
        }
        if has_outputs
            && self.notebook_trust_rung.is_fully_trusted()
            && matches!(self.output_trust_state, OutputTrustState::WidgetGated)
            && !has_widget_rows
        {
            // The notebook claims widget-gated outputs without any widget
            // row to gate. The axes are not independently meaningful.
            collapsed.push("output_trust_state".to_owned());
        }
        if has_widget_rows && matches!(self.widget_trust_state, WidgetTrustState::NotApplicable) {
            let already_addressed = self.rows.iter().all(|row| {
                row.content_class != CellContentClass::WidgetOutput
                    || row.widget_trust_override_token.is_some()
            });
            if !already_addressed {
                collapsed.push("widget_trust_state".to_owned());
            }
        }
        collapsed
    }
}

#[cfg(test)]
mod tests;
