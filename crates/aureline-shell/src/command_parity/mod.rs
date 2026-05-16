//! Beta-grade command-parity diff projection.
//!
//! This module turns the one-command-graph promise into a checked diff
//! report that compares the canonical descriptor for every claimed beta
//! command against the projection that each high-risk command surface
//! reports. The report is consumed by:
//!
//! - the live shell (so the in-product parity inspector quotes the same
//!   per-row blocking findings the CLI prints);
//! - the headless inspector (`aureline_shell_command_parity`), which is
//!   the only mint-from-truth path for the JSON fixtures checked in
//!   under `fixtures/commands/m3/command_parity/`;
//! - the support-export wrapper that lets a reviewer pivot from a
//!   support case to the row that flagged a blocker;
//! - the markdown report under
//!   `artifacts/ux/m3/command_parity_diff_report.md` (rendered from the
//!   same seed); and
//! - the CI gate `tools/ci/m3/command_parity_check.py`, which fails
//!   release if any blocking finding remains visible.
//!
//! The report covers exactly the five surface families the M3 row
//! claims switching parity for:
//!
//! - `command_palette`
//! - `menu_or_button`
//! - `keybinding_help`
//! - `cli_headless`
//! - `ai_tool_surface`
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every claimed beta command must declare a coverage row for each
//!    of the five required surface families.
//! 2. A high-risk command (any non-`no_preview_required` preview class
//!    or any non-`reversible_local_*` capability scope) with an
//!    `unknown` gap on any required surface is a blocker. Explicit
//!    narrowing rows are not blockers.
//! 3. Disabled reasons, lifecycle labels, preview classes, and required
//!    evidence refs MUST come from the same canonical descriptor across
//!    every claimed surface; a drift is a blocker.
//! 4. Each row must quote the canonical docs/help anchor and the
//!    descriptor revision the projection was produced against, so the
//!    report can be reused in release-truth and docs-truth checks.
//! 5. At least one beta command must claim each of the five surface
//!    families so the report cannot regress into a single-surface view.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/commands/m3/command_parity/`
//! are bit-for-bit equal to the seeded report returned by
//! [`seeded_command_parity_diff_report`].

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version exported with every command-parity record.
pub const COMMAND_PARITY_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta command-parity row.
pub const COMMAND_PARITY_SHARED_CONTRACT_REF: &str = "shell:command_parity_beta:v1";

/// Stable record kind for [`BetaCommandParityDiffReport`] payloads.
pub const COMMAND_PARITY_REPORT_RECORD_KIND: &str = "shell_command_parity_beta_diff_report_record";

/// Stable record kind for [`BetaCommandParityRow`] payloads.
pub const COMMAND_PARITY_ROW_RECORD_KIND: &str = "shell_command_parity_beta_row_record";

/// Stable record kind for [`BetaCommandParitySupportExport`] payloads.
pub const COMMAND_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_command_parity_beta_support_export_record";

/// Stable report id consumed across surfaces.
pub const COMMAND_PARITY_REPORT_ID: &str = "shell:command_parity_beta:diff:v1";

/// Stable support-export id quoted in the published wrapper.
pub const COMMAND_PARITY_SUPPORT_EXPORT_ID: &str = "support-export:command-parity:beta:001";

/// Path of the published markdown report.
pub const COMMAND_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m3/command_parity_diff_report.md";

/// Path of the published companion doc.
pub const COMMAND_PARITY_PUBLISHED_DOC_REF: &str = "docs/ux/m3/command_parity_diff_report.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-05-15T00:00:00Z";

/// One of the five surface families the beta parity report covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaSurfaceFamily {
    /// Command palette result rows.
    CommandPalette,
    /// Global menus, context menus, and toolbar buttons.
    MenuOrButton,
    /// Keybinding help surfaces and conflict resolution.
    KeybindingHelp,
    /// CLI / headless help and dispatch rows.
    CliHeadless,
    /// AI tool surfaces invoked by stable command identity.
    AiToolSurface,
}

impl BetaSurfaceFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandPalette => "command_palette",
            Self::MenuOrButton => "menu_or_button",
            Self::KeybindingHelp => "keybinding_help",
            Self::CliHeadless => "cli_headless",
            Self::AiToolSurface => "ai_tool_surface",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::CommandPalette => "Command palette",
            Self::MenuOrButton => "Menus and buttons",
            Self::KeybindingHelp => "Keybinding help",
            Self::CliHeadless => "CLI / headless",
            Self::AiToolSurface => "AI tool surface",
        }
    }

    /// Returns the five required surface families in canonical order.
    pub const fn required_surfaces() -> [Self; 5] {
        [
            Self::CommandPalette,
            Self::MenuOrButton,
            Self::KeybindingHelp,
            Self::CliHeadless,
            Self::AiToolSurface,
        ]
    }
}

/// Lifecycle label retained on the canonical command descriptor.
///
/// Surfaces MUST project the same label; a divergent surface label is
/// a blocking finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaLifecycleLabel {
    /// Generally available; default for stable commands.
    Stable,
    /// Beta lane; visibility, narrowing, and disabled reasons can change.
    Beta,
    /// Deprecated; surfaces must show the replacement command id.
    Deprecated,
}

impl BetaLifecycleLabel {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Preview class the canonical descriptor pins for the command.
///
/// Every surface projection MUST quote the same value; a divergent
/// preview class is a blocking finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaPreviewClass {
    /// Command is safe to invoke without a preview.
    NoPreviewRequired,
    /// Command must show a structured diff preview before apply.
    StructuredDiffPreview,
    /// Command crosses a destructive bulk-mutation boundary.
    DestructiveBulkMutationPreview,
    /// Command writes a policy or waiver that must be authored first.
    PolicyAuthoringOrWaiverPreview,
    /// Command publishes irreversibly (push, release, share).
    IrreversiblePublishPreview,
}

impl BetaPreviewClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPreviewRequired => "no_preview_required",
            Self::StructuredDiffPreview => "structured_diff_preview",
            Self::DestructiveBulkMutationPreview => "destructive_bulk_mutation_preview",
            Self::PolicyAuthoringOrWaiverPreview => "policy_authoring_or_waiver_preview",
            Self::IrreversiblePublishPreview => "irreversible_publish_preview",
        }
    }

    /// `true` when the preview class requires explicit pre-apply review.
    pub const fn is_high_risk(self) -> bool {
        !matches!(self, Self::NoPreviewRequired)
    }
}

/// Capability-scope class the canonical descriptor pins for the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaCapabilityScope {
    /// Inert metadata route (no state change).
    InertMetadataOnly,
    /// Reversible local read.
    ReversibleLocalRead,
    /// Reversible local mutation that the user can undo without rollback.
    ReversibleLocalMutation,
    /// Recoverable durable mutation that requires a rollback handle.
    RecoverableDurableMutation,
    /// Destructive bulk mutation (multi-file, multi-record).
    DestructiveBulkMutation,
    /// Irreversible publish / network mutation.
    IrreversiblePublish,
}

impl BetaCapabilityScope {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertMetadataOnly => "inert_metadata_only",
            Self::ReversibleLocalRead => "reversible_local_read",
            Self::ReversibleLocalMutation => "reversible_local_mutation",
            Self::RecoverableDurableMutation => "recoverable_durable_mutation",
            Self::DestructiveBulkMutation => "destructive_bulk_mutation",
            Self::IrreversiblePublish => "irreversible_publish",
        }
    }

    /// `true` for capability scopes that contribute to high-risk status.
    pub const fn is_high_risk(self) -> bool {
        matches!(
            self,
            Self::RecoverableDurableMutation
                | Self::DestructiveBulkMutation
                | Self::IrreversiblePublish
        )
    }
}

/// Disabled-reason mode the canonical descriptor pins for the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaDisabledReasonMode {
    /// Command is always invokable; no disabled-reason path is required.
    AlwaysInvokable,
    /// Command MUST surface a typed disabled reason when unavailable.
    TypedReasonRequiredWhenUnavailable,
}

impl BetaDisabledReasonMode {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvokable => "always_invokable",
            Self::TypedReasonRequiredWhenUnavailable => "typed_reason_required_when_unavailable",
        }
    }
}

/// Coverage status reported by a surface row.
///
/// Only `Claimed` rows are subject to projection-vs-descriptor drift
/// checks. `ExplicitlyNarrowed`, `DiscoverableOnly`,
/// `BrowserHandoffOnly`, `VoiceAddressable`, and
/// `NotSurfacedOnThisClient` rows are accepted as long as they carry a
/// `narrowing_reason`. `UnknownHighRiskGap` is a blocking finding for
/// any high-risk command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaCoverageStatus {
    /// Surface claims a first-class projection of the command.
    Claimed,
    /// Surface explicitly narrows; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// Surface lists the command for discoverability only (no dispatch).
    DiscoverableOnly,
    /// Surface routes to browser handoff only.
    BrowserHandoffOnly,
    /// Surface is voice-addressable only; the real route is elsewhere.
    VoiceAddressable,
    /// Client scope excludes this surface (e.g. CLI cannot open a UI route).
    NotSurfacedOnThisClient,
    /// Surface family is required but the projection is missing or unknown.
    /// Always a blocker for high-risk commands.
    UnknownHighRiskGap,
}

impl BetaCoverageStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Claimed => "claimed",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::DiscoverableOnly => "discoverable_only",
            Self::BrowserHandoffOnly => "browser_handoff_only",
            Self::VoiceAddressable => "voice_addressable",
            Self::NotSurfacedOnThisClient => "not_surfaced_on_this_client",
            Self::UnknownHighRiskGap => "unknown_high_risk_gap",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyNarrowed
                | Self::DiscoverableOnly
                | Self::BrowserHandoffOnly
                | Self::VoiceAddressable
                | Self::NotSurfacedOnThisClient
        )
    }

    /// `true` for statuses that are projected from the descriptor and
    /// therefore subject to descriptor-vs-projection drift checks.
    pub const fn projects_descriptor(self) -> bool {
        matches!(self, Self::Claimed)
    }
}

/// Canonical descriptor for one beta command.
///
/// Every surface row in the report quotes these fields verbatim; any
/// divergence is a blocking finding the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaCommandDescriptor {
    /// Stable command id (e.g. `cmd:workspace.open_folder`).
    pub command_id: String,
    /// Descriptor revision the report was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical docs/help anchor ref.
    pub docs_help_anchor_ref: String,
    /// Pinned lifecycle label.
    pub lifecycle_label: BetaLifecycleLabel,
    /// Pinned preview class.
    pub preview_class: BetaPreviewClass,
    /// Pinned capability scope class.
    pub capability_scope_class: BetaCapabilityScope,
    /// Pinned disabled-reason mode.
    pub disabled_reason_mode: BetaDisabledReasonMode,
    /// Canonical alias set the descriptor owns. Surfaces MUST NOT
    /// expose aliases outside this set.
    pub canonical_aliases: Vec<String>,
}

impl BetaCommandDescriptor {
    /// `true` when this command's pinned scope or preview class makes
    /// it high-risk for the parity report.
    pub const fn is_high_risk(&self) -> bool {
        self.preview_class.is_high_risk() || self.capability_scope_class.is_high_risk()
    }
}

/// Surface projection reported by one surface family for one command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaSurfaceProjection {
    /// Surface family this projection belongs to.
    pub surface_family: BetaSurfaceFamily,
    /// Coverage status the surface reports.
    pub coverage_status: BetaCoverageStatus,
    /// Projected command id (`None` for unknown gaps).
    pub projected_command_id: Option<String>,
    /// Projected primary label ref (`None` for unknown gaps).
    pub projected_label_ref: Option<String>,
    /// Projected lifecycle label (`None` for unknown gaps).
    pub projected_lifecycle_label: Option<BetaLifecycleLabel>,
    /// Projected preview class (`None` for unknown gaps).
    pub projected_preview_class: Option<BetaPreviewClass>,
    /// Projected disabled-reason mode (`None` for unknown gaps).
    pub projected_disabled_reason_mode: Option<BetaDisabledReasonMode>,
    /// Projected docs/help anchor ref (`None` for unknown gaps).
    pub projected_docs_help_anchor_ref: Option<String>,
    /// Aliases the surface exposes for this command. MUST be a subset
    /// of the canonical alias set on the descriptor.
    pub projected_aliases: Vec<String>,
    /// Narrowing reason set when `coverage_status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the row.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum BetaParityBlockingFinding {
    /// A high-risk command has no claim or explicit narrowing for a
    /// required surface family.
    UnknownHighRiskGap {
        /// Command that exposes the gap.
        command_id: String,
        /// Surface family that exposes the gap.
        surface_family: BetaSurfaceFamily,
    },
    /// A claimed surface carries a command id that disagrees with the
    /// descriptor.
    CommandIdDrift {
        /// Command id on the descriptor.
        command_id: String,
        /// Surface family that disagrees.
        surface_family: BetaSurfaceFamily,
        /// Projected command id on the surface.
        projected_command_id: String,
    },
    /// A claimed surface carries a label ref that disagrees with the
    /// descriptor.
    LabelDrift {
        command_id: String,
        surface_family: BetaSurfaceFamily,
        /// Projected label ref.
        projected_label_ref: String,
    },
    /// A claimed surface carries a lifecycle label that disagrees with
    /// the descriptor.
    LifecycleLabelDrift {
        command_id: String,
        surface_family: BetaSurfaceFamily,
        /// Projected lifecycle label.
        projected_lifecycle_label: BetaLifecycleLabel,
    },
    /// A claimed surface carries a preview class that disagrees with
    /// the descriptor.
    PreviewClassDrift {
        command_id: String,
        surface_family: BetaSurfaceFamily,
        /// Projected preview class.
        projected_preview_class: BetaPreviewClass,
    },
    /// A claimed surface drops typed disabled-reason disclosure.
    DisabledReasonDrift {
        command_id: String,
        surface_family: BetaSurfaceFamily,
        /// Projected disabled-reason mode.
        projected_disabled_reason_mode: BetaDisabledReasonMode,
    },
    /// A claimed surface cannot point back to the canonical docs/help
    /// anchor.
    MissingDocsHelpAnchor {
        command_id: String,
        surface_family: BetaSurfaceFamily,
    },
    /// A claimed surface exposes an alias outside the canonical alias
    /// set.
    AliasDrift {
        command_id: String,
        surface_family: BetaSurfaceFamily,
        /// First alias seen that the descriptor does not own.
        offending_alias: String,
    },
    /// A non-`Claimed` row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        command_id: String,
        surface_family: BetaSurfaceFamily,
        coverage_status: BetaCoverageStatus,
    },
    /// A row is missing a projection it requires (claimed but missing
    /// the descriptor mirror).
    MissingProjection {
        command_id: String,
        surface_family: BetaSurfaceFamily,
        /// Name of the missing projection field.
        field: String,
    },
}

impl BetaParityBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnknownHighRiskGap { .. } => "unknown_high_risk_gap",
            Self::CommandIdDrift { .. } => "command_id_drift",
            Self::LabelDrift { .. } => "label_drift",
            Self::LifecycleLabelDrift { .. } => "lifecycle_label_drift",
            Self::PreviewClassDrift { .. } => "preview_class_drift",
            Self::DisabledReasonDrift { .. } => "disabled_reason_drift",
            Self::MissingDocsHelpAnchor { .. } => "missing_docs_help_anchor",
            Self::AliasDrift { .. } => "alias_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
        }
    }

    /// Returns the command id this finding is attached to.
    pub fn command_id(&self) -> &str {
        match self {
            Self::UnknownHighRiskGap { command_id, .. }
            | Self::CommandIdDrift { command_id, .. }
            | Self::LabelDrift { command_id, .. }
            | Self::LifecycleLabelDrift { command_id, .. }
            | Self::PreviewClassDrift { command_id, .. }
            | Self::DisabledReasonDrift { command_id, .. }
            | Self::MissingDocsHelpAnchor { command_id, .. }
            | Self::AliasDrift { command_id, .. }
            | Self::MissingNarrowingReason { command_id, .. }
            | Self::MissingProjection { command_id, .. } => command_id,
        }
    }

    /// Returns the surface family this finding is attached to.
    pub fn surface_family(&self) -> BetaSurfaceFamily {
        match self {
            Self::UnknownHighRiskGap { surface_family, .. }
            | Self::CommandIdDrift { surface_family, .. }
            | Self::LabelDrift { surface_family, .. }
            | Self::LifecycleLabelDrift { surface_family, .. }
            | Self::PreviewClassDrift { surface_family, .. }
            | Self::DisabledReasonDrift { surface_family, .. }
            | Self::MissingDocsHelpAnchor { surface_family, .. }
            | Self::AliasDrift { surface_family, .. }
            | Self::MissingNarrowingReason { surface_family, .. }
            | Self::MissingProjection { surface_family, .. } => *surface_family,
        }
    }
}

/// One per-command parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaCommandParityRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the command.
    pub descriptor: BetaCommandDescriptor,
    /// Surface-by-surface projections, in canonical surface-family order.
    pub surfaces: Vec<BetaSurfaceProjection>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<BetaParityBlockingFinding>,
    /// `true` when the command's descriptor classifies it as high-risk
    /// for the parity report.
    pub high_risk: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaParityFindingSummary {
    /// Total blocking findings across the report.
    pub total_blocking_findings: usize,
    /// Number of `unknown_high_risk_gap` findings.
    pub unknown_high_risk_gap: usize,
    /// Number of `command_id_drift` findings.
    pub command_id_drift: usize,
    /// Number of `label_drift` findings.
    pub label_drift: usize,
    /// Number of `lifecycle_label_drift` findings.
    pub lifecycle_label_drift: usize,
    /// Number of `preview_class_drift` findings.
    pub preview_class_drift: usize,
    /// Number of `disabled_reason_drift` findings.
    pub disabled_reason_drift: usize,
    /// Number of `missing_docs_help_anchor` findings.
    pub missing_docs_help_anchor: usize,
    /// Number of `alias_drift` findings.
    pub alias_drift: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
}

impl BetaParityFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unknown_high_risk_gap: 0,
            command_id_drift: 0,
            label_drift: 0,
            lifecycle_label_drift: 0,
            preview_class_drift: 0,
            disabled_reason_drift: 0,
            missing_docs_help_anchor: 0,
            alias_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
        }
    }

    fn record(&mut self, finding: &BetaParityBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            BetaParityBlockingFinding::UnknownHighRiskGap { .. } => self.unknown_high_risk_gap += 1,
            BetaParityBlockingFinding::CommandIdDrift { .. } => self.command_id_drift += 1,
            BetaParityBlockingFinding::LabelDrift { .. } => self.label_drift += 1,
            BetaParityBlockingFinding::LifecycleLabelDrift { .. } => {
                self.lifecycle_label_drift += 1
            }
            BetaParityBlockingFinding::PreviewClassDrift { .. } => self.preview_class_drift += 1,
            BetaParityBlockingFinding::DisabledReasonDrift { .. } => {
                self.disabled_reason_drift += 1
            }
            BetaParityBlockingFinding::MissingDocsHelpAnchor { .. } => {
                self.missing_docs_help_anchor += 1
            }
            BetaParityBlockingFinding::AliasDrift { .. } => self.alias_drift += 1,
            BetaParityBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            BetaParityBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
        }
    }
}

/// Per-surface coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaSurfaceCoverageSummary {
    /// Surface family this summary covers.
    pub surface_family: BetaSurfaceFamily,
    /// Number of `claimed` rows on this surface.
    pub claimed_rows: usize,
    /// Number of `explicitly_narrowed` rows on this surface.
    pub explicitly_narrowed_rows: usize,
    /// Number of `discoverable_only` rows on this surface.
    pub discoverable_only_rows: usize,
    /// Number of `browser_handoff_only` rows on this surface.
    pub browser_handoff_only_rows: usize,
    /// Number of `voice_addressable` rows on this surface.
    pub voice_addressable_rows: usize,
    /// Number of `not_surfaced_on_this_client` rows on this surface.
    pub not_surfaced_on_this_client_rows: usize,
    /// Number of `unknown_high_risk_gap` rows on this surface.
    pub unknown_high_risk_gap_rows: usize,
}

/// Beta command-parity diff report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaCommandParityDiffReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source descriptor schema ref for the canonical descriptors.
    pub source_descriptor_schema_ref: String,
    /// Required surface families, in canonical order.
    pub required_surface_families: Vec<BetaSurfaceFamily>,
    /// Per-command parity rows, sorted by `descriptor.command_id`.
    pub rows: Vec<BetaCommandParityRow>,
    /// Per-surface coverage summary, in canonical surface-family order.
    pub surface_coverage: Vec<BetaSurfaceCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: BetaParityFindingSummary,
    /// Number of claimed beta commands present.
    pub claimed_command_count: usize,
    /// Number of high-risk claimed beta commands present.
    pub high_risk_command_count: usize,
    /// Total surface rows checked.
    pub surface_rows_checked: usize,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this report is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the report can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the report can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the report was generated.
    pub generated_at: String,
}

impl BetaCommandParityDiffReport {
    /// Returns `true` when every required surface family is claimed by
    /// at least one beta command.
    pub fn every_required_surface_claimed(&self) -> bool {
        for family in BetaSurfaceFamily::required_surfaces() {
            let any_claimed = self.rows.iter().any(|row| {
                row.surfaces.iter().any(|projection| {
                    projection.surface_family == family
                        && projection.coverage_status == BetaCoverageStatus::Claimed
                })
            });
            if !any_claimed {
                return false;
            }
        }
        true
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: commands={}, high_risk={}, surface_rows={}, blocking={}, clean={}",
            self.claimed_command_count,
            self.high_risk_command_count,
            self.surface_rows_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for surface in &self.surface_coverage {
            lines.push(format!(
                "{}: claimed={}, narrowed={}, unknown_high_risk={}",
                surface.surface_family.display_label(),
                surface.claimed_rows,
                surface.explicitly_narrowed_rows
                    + surface.discoverable_only_rows
                    + surface.browser_handoff_only_rows
                    + surface.voice_addressable_rows
                    + surface.not_surfaced_on_this_client_rows,
                surface.unknown_high_risk_gap_rows,
            ));
        }
        for row in &self.rows {
            if !row.blocking_findings.is_empty() {
                for finding in &row.blocking_findings {
                    lines.push(format!(
                        "blocker: {} -- {} -- {}",
                        finding.class_token(),
                        finding.command_id(),
                        finding.surface_family().as_str(),
                    ));
                }
            }
        }
        lines
    }

    /// Renders the markdown report artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Beta command-parity diff report\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded parity projection in\n\
             [`crate::command_parity`](../../../crates/aureline-shell/src/command_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- report-md > \\\n  artifacts/ux/m3/command_parity_diff_report.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Descriptor schema ref: `{}`\n",
            self.source_descriptor_schema_ref
        ));
        out.push_str(&format!(
            "- Claimed beta commands: `{}`\n",
            self.claimed_command_count
        ));
        out.push_str(&format!(
            "- High-risk beta commands: `{}`\n",
            self.high_risk_command_count
        ));
        out.push_str(&format!(
            "- Surface rows checked: `{}`\n",
            self.surface_rows_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
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

        out.push_str("## Per-surface coverage\n\n");
        out.push_str(
            "| Surface | Claimed | Narrowed | Unknown high-risk |\n\
             | ------- | ------: | -------: | ----------------: |\n",
        );
        for surface in &self.surface_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                surface.surface_family.display_label(),
                surface.claimed_rows,
                surface.explicitly_narrowed_rows
                    + surface.discoverable_only_rows
                    + surface.browser_handoff_only_rows
                    + surface.voice_addressable_rows
                    + surface.not_surfaced_on_this_client_rows,
                surface.unknown_high_risk_gap_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unknown_high_risk_gap` | {} |\n",
            self.findings_summary.unknown_high_risk_gap
        ));
        out.push_str(&format!(
            "| `command_id_drift` | {} |\n",
            self.findings_summary.command_id_drift
        ));
        out.push_str(&format!(
            "| `label_drift` | {} |\n",
            self.findings_summary.label_drift
        ));
        out.push_str(&format!(
            "| `lifecycle_label_drift` | {} |\n",
            self.findings_summary.lifecycle_label_drift
        ));
        out.push_str(&format!(
            "| `preview_class_drift` | {} |\n",
            self.findings_summary.preview_class_drift
        ));
        out.push_str(&format!(
            "| `disabled_reason_drift` | {} |\n",
            self.findings_summary.disabled_reason_drift
        ));
        out.push_str(&format!(
            "| `missing_docs_help_anchor` | {} |\n",
            self.findings_summary.missing_docs_help_anchor
        ));
        out.push_str(&format!(
            "| `alias_drift` | {} |\n",
            self.findings_summary.alias_drift
        ));
        out.push_str(&format!(
            "| `missing_narrowing_reason` | {} |\n",
            self.findings_summary.missing_narrowing_reason
        ));
        out.push_str(&format!(
            "| `missing_projection` | {} |\n\n",
            self.findings_summary.missing_projection
        ));

        out.push_str("## Per-command rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "### `{}` ({})\n\n",
                row.descriptor.command_id,
                row.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                row.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Preview class: `{}`\n",
                row.descriptor.preview_class.as_str()
            ));
            out.push_str(&format!(
                "- Capability scope: `{}`\n",
                row.descriptor.capability_scope_class.as_str()
            ));
            out.push_str(&format!(
                "- Disabled reason mode: `{}`\n",
                row.descriptor.disabled_reason_mode.as_str()
            ));
            out.push_str(&format!(
                "- Docs/help anchor: `{}`\n",
                row.descriptor.docs_help_anchor_ref
            ));
            out.push_str(&format!(
                "- High-risk: `{}`\n\n",
                if row.high_risk { "yes" } else { "no" }
            ));

            out.push_str(
                "| Surface | Status | Projected preview | Disabled reason mode | Narrowing reason |\n\
                 | ------- | ------ | ----------------- | -------------------- | ---------------- |\n",
            );
            for projection in &row.surfaces {
                let preview = projection
                    .projected_preview_class
                    .map(|class| class.as_str())
                    .unwrap_or("-");
                let mode = projection
                    .projected_disabled_reason_mode
                    .map(|mode| mode.as_str())
                    .unwrap_or("-");
                let narrowing = projection.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | {} |\n",
                    projection.surface_family.display_label(),
                    projection.coverage_status.as_str(),
                    preview,
                    mode,
                    narrowing,
                ));
            }
            out.push('\n');

            if row.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &row.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding.surface_family().as_str()
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test command_parity_fixtures\n");
        out.push_str("python3 tools/ci/m3/command_parity_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the parity diff report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaCommandParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Parity diff report quoted in full.
    pub report: BetaCommandParityDiffReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl BetaCommandParitySupportExport {
    /// Builds the support-export wrapper for a parity report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: BetaCommandParityDiffReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for row in &report.rows {
            case_ids.push(row.descriptor.command_id.clone());
            case_ids.push(row.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: COMMAND_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: COMMAND_PARITY_SCHEMA_VERSION,
            shared_contract_ref: COMMAND_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-row blocking findings from a descriptor and its
/// surface projections.
fn compute_row_findings(
    descriptor: &BetaCommandDescriptor,
    surfaces: &[BetaSurfaceProjection],
    high_risk: bool,
) -> Vec<BetaParityBlockingFinding> {
    let mut findings = Vec::new();
    let canonical_aliases: BTreeSet<&str> = descriptor
        .canonical_aliases
        .iter()
        .map(String::as_str)
        .collect();

    for projection in surfaces {
        let family = projection.surface_family;
        match projection.coverage_status {
            BetaCoverageStatus::UnknownHighRiskGap => {
                if high_risk {
                    findings.push(BetaParityBlockingFinding::UnknownHighRiskGap {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                    });
                }
            }
            BetaCoverageStatus::Claimed => {
                match &projection.projected_command_id {
                    Some(id) if id == &descriptor.command_id => {}
                    Some(id) => findings.push(BetaParityBlockingFinding::CommandIdDrift {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        projected_command_id: id.clone(),
                    }),
                    None => findings.push(BetaParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        field: "projected_command_id".to_owned(),
                    }),
                }

                match &projection.projected_label_ref {
                    Some(label) if label == &descriptor.primary_label_ref => {}
                    Some(label) => findings.push(BetaParityBlockingFinding::LabelDrift {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        projected_label_ref: label.clone(),
                    }),
                    None => findings.push(BetaParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        field: "projected_label_ref".to_owned(),
                    }),
                }

                match projection.projected_lifecycle_label {
                    Some(lifecycle) if lifecycle == descriptor.lifecycle_label => {}
                    Some(lifecycle) => {
                        findings.push(BetaParityBlockingFinding::LifecycleLabelDrift {
                            command_id: descriptor.command_id.clone(),
                            surface_family: family,
                            projected_lifecycle_label: lifecycle,
                        })
                    }
                    None => findings.push(BetaParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        field: "projected_lifecycle_label".to_owned(),
                    }),
                }

                match projection.projected_preview_class {
                    Some(preview) if preview == descriptor.preview_class => {}
                    Some(preview) => findings.push(BetaParityBlockingFinding::PreviewClassDrift {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        projected_preview_class: preview,
                    }),
                    None => findings.push(BetaParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        field: "projected_preview_class".to_owned(),
                    }),
                }

                match projection.projected_disabled_reason_mode {
                    Some(mode) if mode == descriptor.disabled_reason_mode => {}
                    Some(mode) => findings.push(BetaParityBlockingFinding::DisabledReasonDrift {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        projected_disabled_reason_mode: mode,
                    }),
                    None => findings.push(BetaParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        field: "projected_disabled_reason_mode".to_owned(),
                    }),
                }

                match &projection.projected_docs_help_anchor_ref {
                    Some(anchor) if anchor == &descriptor.docs_help_anchor_ref => {}
                    Some(_) | None => {
                        findings.push(BetaParityBlockingFinding::MissingDocsHelpAnchor {
                            command_id: descriptor.command_id.clone(),
                            surface_family: family,
                        });
                    }
                }

                for alias in &projection.projected_aliases {
                    if !canonical_aliases.contains(alias.as_str()) {
                        findings.push(BetaParityBlockingFinding::AliasDrift {
                            command_id: descriptor.command_id.clone(),
                            surface_family: family,
                            offending_alias: alias.clone(),
                        });
                    }
                }
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = projection
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(BetaParityBlockingFinding::MissingNarrowingReason {
                        command_id: descriptor.command_id.clone(),
                        surface_family: family,
                        coverage_status: status,
                    });
                }
            }
            _ => {}
        }
    }
    findings
}

/// Validates a finished report and returns the surface-coverage and
/// per-class findings summaries, plus the row-level findings list.
fn summarize_report(
    rows: &[BetaCommandParityRow],
) -> (Vec<BetaSurfaceCoverageSummary>, BetaParityFindingSummary) {
    let mut summary = BetaParityFindingSummary::empty();
    let mut coverage: Vec<BetaSurfaceCoverageSummary> = BetaSurfaceFamily::required_surfaces()
        .into_iter()
        .map(|family| BetaSurfaceCoverageSummary {
            surface_family: family,
            claimed_rows: 0,
            explicitly_narrowed_rows: 0,
            discoverable_only_rows: 0,
            browser_handoff_only_rows: 0,
            voice_addressable_rows: 0,
            not_surfaced_on_this_client_rows: 0,
            unknown_high_risk_gap_rows: 0,
        })
        .collect();

    for row in rows {
        for projection in &row.surfaces {
            if let Some(coverage_row) = coverage
                .iter_mut()
                .find(|coverage| coverage.surface_family == projection.surface_family)
            {
                match projection.coverage_status {
                    BetaCoverageStatus::Claimed => coverage_row.claimed_rows += 1,
                    BetaCoverageStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    BetaCoverageStatus::DiscoverableOnly => {
                        coverage_row.discoverable_only_rows += 1
                    }
                    BetaCoverageStatus::BrowserHandoffOnly => {
                        coverage_row.browser_handoff_only_rows += 1
                    }
                    BetaCoverageStatus::VoiceAddressable => {
                        coverage_row.voice_addressable_rows += 1
                    }
                    BetaCoverageStatus::NotSurfacedOnThisClient => {
                        coverage_row.not_surfaced_on_this_client_rows += 1
                    }
                    BetaCoverageStatus::UnknownHighRiskGap => {
                        coverage_row.unknown_high_risk_gap_rows += 1
                    }
                }
            }
        }
        for finding in &row.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Builds a [`BetaCommandParityRow`] from a descriptor and its surface
/// projections, computing the per-row blocking findings.
pub fn build_command_parity_row(
    descriptor: BetaCommandDescriptor,
    surfaces: Vec<BetaSurfaceProjection>,
) -> BetaCommandParityRow {
    let high_risk = descriptor.is_high_risk();
    let blocking_findings = compute_row_findings(&descriptor, &surfaces, high_risk);

    BetaCommandParityRow {
        record_kind: COMMAND_PARITY_ROW_RECORD_KIND.to_owned(),
        schema_version: COMMAND_PARITY_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_PARITY_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        surfaces,
        blocking_findings,
        high_risk,
    }
}

/// Builds a full [`BetaCommandParityDiffReport`] from per-command rows.
pub fn build_command_parity_diff_report(
    rows: Vec<BetaCommandParityRow>,
) -> BetaCommandParityDiffReport {
    let mut rows = rows;
    rows.sort_by(|left, right| left.descriptor.command_id.cmp(&right.descriptor.command_id));

    let claimed_command_count = rows.len();
    let high_risk_command_count = rows.iter().filter(|row| row.high_risk).count();
    let surface_rows_checked = rows.iter().map(|row| row.surfaces.len()).sum::<usize>();

    let (surface_coverage, findings_summary) = summarize_report(&rows);
    let report_clean = findings_summary.total_blocking_findings == 0;

    BetaCommandParityDiffReport {
        record_kind: COMMAND_PARITY_REPORT_RECORD_KIND.to_owned(),
        schema_version: COMMAND_PARITY_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_PARITY_SHARED_CONTRACT_REF.to_owned(),
        report_id: COMMAND_PARITY_REPORT_ID.to_owned(),
        source_descriptor_schema_ref: "schemas/commands/command_descriptor.schema.json".to_owned(),
        required_surface_families: BetaSurfaceFamily::required_surfaces().to_vec(),
        rows,
        surface_coverage,
        findings_summary,
        claimed_command_count,
        high_risk_command_count,
        surface_rows_checked,
        report_clean,
        published_report_ref: COMMAND_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: COMMAND_PARITY_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            COMMAND_PARITY_PUBLISHED_DOC_REF.to_owned(),
            "docs/commands/command_parity_diff.md".to_owned(),
        ],
        support_export_refs: vec!["support:command-parity:beta".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_command_parity_diff_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum BetaCommandParityValidationError {
    /// The report has no claimed commands.
    NoClaimedCommands,
    /// A required surface family has no claimed row.
    RequiredSurfaceNotClaimed { surface_family: String },
    /// A row is missing a required surface family from its projection set.
    MissingRequiredSurface {
        command_id: String,
        surface_family: String,
    },
    /// A blocking finding remains on the row.
    BlockingFindingPresent {
        command_id: String,
        surface_family: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A row's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { command_id: String },
}

/// Validates a parity diff report against the M3 acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_command_parity_diff_report(
    report: &BetaCommandParityDiffReport,
) -> Result<(), Vec<BetaCommandParityValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(BetaCommandParityValidationError::NoClaimedCommands);
    }

    for required in BetaSurfaceFamily::required_surfaces() {
        let any_claimed = report.rows.iter().any(|row| {
            row.surfaces.iter().any(|projection| {
                projection.surface_family == required
                    && projection.coverage_status == BetaCoverageStatus::Claimed
            })
        });
        if !any_claimed {
            errors.push(
                BetaCommandParityValidationError::RequiredSurfaceNotClaimed {
                    surface_family: required.as_str().to_owned(),
                },
            );
        }
    }

    for row in &report.rows {
        for required in BetaSurfaceFamily::required_surfaces() {
            if !row
                .surfaces
                .iter()
                .any(|projection| projection.surface_family == required)
            {
                errors.push(BetaCommandParityValidationError::MissingRequiredSurface {
                    command_id: row.descriptor.command_id.clone(),
                    surface_family: required.as_str().to_owned(),
                });
            }
        }
        if row.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(
                BetaCommandParityValidationError::MissingDescriptorRevisionRef {
                    command_id: row.descriptor.command_id.clone(),
                },
            );
        }
        for finding in &row.blocking_findings {
            errors.push(BetaCommandParityValidationError::BlockingFindingPresent {
                command_id: finding.command_id().to_owned(),
                surface_family: finding.surface_family().as_str().to_owned(),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(BetaCommandParityValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(BetaCommandParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_command_parity_diff_report`].
struct CommandSeed {
    command_id: &'static str,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    docs_help_anchor_ref: &'static str,
    lifecycle_label: BetaLifecycleLabel,
    preview_class: BetaPreviewClass,
    capability_scope_class: BetaCapabilityScope,
    disabled_reason_mode: BetaDisabledReasonMode,
    canonical_aliases: &'static [&'static str],
    surfaces: &'static [SurfaceSeed],
}

struct SurfaceSeed {
    surface_family: BetaSurfaceFamily,
    coverage_status: BetaCoverageStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
    projected_aliases: &'static [&'static str],
}

const COMMAND_SEEDS: &[CommandSeed] = &[
    CommandSeed {
        command_id: "cmd:workspace.open_folder",
        descriptor_revision_ref: "cmd-rev:workspace.open_folder:2026.04.21-01",
        primary_label_ref: "label:workspace.open_folder:primary",
        docs_help_anchor_ref: "docs:anchor:workspace:open_folder_overview",
        lifecycle_label: BetaLifecycleLabel::Stable,
        preview_class: BetaPreviewClass::NoPreviewRequired,
        capability_scope_class: BetaCapabilityScope::ReversibleLocalMutation,
        disabled_reason_mode: BetaDisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        canonical_aliases: &[
            "alias:workspace.open_folder:cli_open",
            "alias:workspace.open_folder:legacy_file_open_folder",
        ],
        surfaces: &[
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CommandPalette,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::MenuOrButton,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::KeybindingHelp,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CliHeadless,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:workspace.open_folder:cli_open"],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::AiToolSurface,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
        ],
    },
    CommandSeed {
        command_id: "cmd:workspace.clone_repository",
        descriptor_revision_ref: "cmd-rev:workspace.clone_repository:2026.04.22-01",
        primary_label_ref: "label:workspace.clone_repository:primary",
        docs_help_anchor_ref: "docs:anchor:workspace:clone_repository_overview",
        lifecycle_label: BetaLifecycleLabel::Stable,
        preview_class: BetaPreviewClass::StructuredDiffPreview,
        capability_scope_class: BetaCapabilityScope::RecoverableDurableMutation,
        disabled_reason_mode: BetaDisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        canonical_aliases: &["alias:workspace.clone_repository:cli_clone"],
        surfaces: &[
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CommandPalette,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::MenuOrButton,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::KeybindingHelp,
                coverage_status: BetaCoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("keybinding_unassigned_at_beta"),
                note: Some("Keybinding help lists the command without a default chord."),
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CliHeadless,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:workspace.clone_repository:cli_clone"],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::AiToolSurface,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
        ],
    },
    CommandSeed {
        command_id: "cmd:workspace.import_profile",
        descriptor_revision_ref: "cmd-rev:workspace.import_profile:2026.04.22-01",
        primary_label_ref: "label:workspace.import_profile:primary",
        docs_help_anchor_ref: "docs:anchor:migration:import_profile_overview",
        lifecycle_label: BetaLifecycleLabel::Beta,
        preview_class: BetaPreviewClass::StructuredDiffPreview,
        capability_scope_class: BetaCapabilityScope::RecoverableDurableMutation,
        disabled_reason_mode: BetaDisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        canonical_aliases: &["alias:workspace.import_profile:cli_import_profile"],
        surfaces: &[
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CommandPalette,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::MenuOrButton,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::KeybindingHelp,
                coverage_status: BetaCoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("keybinding_unassigned_at_beta"),
                note: Some(
                    "Keybinding help shows the command in the migration category without a default chord.",
                ),
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CliHeadless,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:workspace.import_profile:cli_import_profile"],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::AiToolSurface,
                coverage_status: BetaCoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required"),
                note: Some(
                    "AI may draft the import plan but cannot apply without the same preview and confirmation.",
                ),
                projected_aliases: &[],
            },
        ],
    },
    CommandSeed {
        command_id: "cmd:workspace.restore_from_checkpoint",
        descriptor_revision_ref: "cmd-rev:workspace.restore_from_checkpoint:2026.04.22-01",
        primary_label_ref: "label:workspace.restore_from_checkpoint:primary",
        docs_help_anchor_ref: "docs:anchor:workspace:restore_from_checkpoint_overview",
        lifecycle_label: BetaLifecycleLabel::Beta,
        preview_class: BetaPreviewClass::DestructiveBulkMutationPreview,
        capability_scope_class: BetaCapabilityScope::DestructiveBulkMutation,
        disabled_reason_mode: BetaDisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        canonical_aliases: &["alias:workspace.restore_from_checkpoint:cli_restore"],
        surfaces: &[
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CommandPalette,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::MenuOrButton,
                coverage_status: BetaCoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required"),
                note: Some(
                    "Menu surfaces route to the restore review sheet rather than running the command directly.",
                ),
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::KeybindingHelp,
                coverage_status: BetaCoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("keybinding_unassigned_at_beta"),
                note: Some("Keybinding help lists the command without a default chord."),
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CliHeadless,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:workspace.restore_from_checkpoint:cli_restore"],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::AiToolSurface,
                coverage_status: BetaCoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required"),
                note: Some("AI may locate candidate checkpoint refs but cannot apply."),
                projected_aliases: &[],
            },
        ],
    },
    CommandSeed {
        command_id: "cmd:command_palette.open",
        descriptor_revision_ref: "cmd-rev:command_palette.open:2026.04.22-01",
        primary_label_ref: "label:command_palette.open:primary",
        docs_help_anchor_ref: "docs:anchor:command_palette:open_overview",
        lifecycle_label: BetaLifecycleLabel::Stable,
        preview_class: BetaPreviewClass::NoPreviewRequired,
        capability_scope_class: BetaCapabilityScope::InertMetadataOnly,
        disabled_reason_mode: BetaDisabledReasonMode::AlwaysInvokable,
        canonical_aliases: &["alias:command_palette.open:vscode_show_all_commands"],
        surfaces: &[
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CommandPalette,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::MenuOrButton,
                coverage_status: BetaCoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("learnability_only_route"),
                note: Some("Menu surfaces surface the chord; opening is a chord-only affordance."),
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::KeybindingHelp,
                coverage_status: BetaCoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::CliHeadless,
                coverage_status: BetaCoverageStatus::NotSurfacedOnThisClient,
                narrowing_reason: Some("client_scope_excludes_surface"),
                note: Some("Not surfaced on this client; CLI inspects the descriptor only."),
                projected_aliases: &[],
            },
            SurfaceSeed {
                surface_family: BetaSurfaceFamily::AiToolSurface,
                coverage_status: BetaCoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("ui_only_route"),
                note: Some("AI surfaces describe the chord; AI does not open palettes for the user."),
                projected_aliases: &[],
            },
        ],
    },
];

fn build_projection_from_seed(
    descriptor: &BetaCommandDescriptor,
    seed: &SurfaceSeed,
) -> BetaSurfaceProjection {
    let projects_descriptor = seed.coverage_status.projects_descriptor();
    BetaSurfaceProjection {
        surface_family: seed.surface_family,
        coverage_status: seed.coverage_status,
        projected_command_id: projects_descriptor.then(|| descriptor.command_id.clone()),
        projected_label_ref: projects_descriptor.then(|| descriptor.primary_label_ref.clone()),
        projected_lifecycle_label: projects_descriptor.then_some(descriptor.lifecycle_label),
        projected_preview_class: projects_descriptor.then_some(descriptor.preview_class),
        projected_disabled_reason_mode: projects_descriptor
            .then_some(descriptor.disabled_reason_mode),
        projected_docs_help_anchor_ref: projects_descriptor
            .then(|| descriptor.docs_help_anchor_ref.clone()),
        projected_aliases: seed
            .projected_aliases
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        narrowing_reason: seed.narrowing_reason.map(str::to_owned),
        note: seed.note.map(str::to_owned),
    }
}

fn build_row_from_seed(seed: &CommandSeed) -> BetaCommandParityRow {
    let descriptor = BetaCommandDescriptor {
        command_id: seed.command_id.to_owned(),
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        docs_help_anchor_ref: seed.docs_help_anchor_ref.to_owned(),
        lifecycle_label: seed.lifecycle_label,
        preview_class: seed.preview_class,
        capability_scope_class: seed.capability_scope_class,
        disabled_reason_mode: seed.disabled_reason_mode,
        canonical_aliases: seed
            .canonical_aliases
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
    };
    let surfaces: Vec<BetaSurfaceProjection> = seed
        .surfaces
        .iter()
        .map(|surface_seed| build_projection_from_seed(&descriptor, surface_seed))
        .collect();
    build_command_parity_row(descriptor, surfaces)
}

/// Seeded report builder used by the headless inspector and the
/// integration test. The seed mirrors the JSON fixtures checked in
/// under `fixtures/commands/m3/command_parity/`.
pub fn seeded_command_parity_diff_report() -> BetaCommandParityDiffReport {
    let rows = COMMAND_SEEDS.iter().map(build_row_from_seed).collect();
    build_command_parity_diff_report(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_report_passes_validation() {
        let report = seeded_command_parity_diff_report();
        validate_command_parity_diff_report(&report).expect("seeded report must validate");
    }

    #[test]
    fn seeded_report_claims_every_required_surface() {
        let report = seeded_command_parity_diff_report();
        assert!(report.every_required_surface_claimed());
    }

    #[test]
    fn seeded_report_has_no_blocking_findings() {
        let report = seeded_command_parity_diff_report();
        assert!(report.report_clean);
        assert_eq!(report.findings_summary.total_blocking_findings, 0);
        for row in &report.rows {
            assert!(
                row.blocking_findings.is_empty(),
                "row {} has blockers",
                row.descriptor.command_id
            );
        }
    }

    #[test]
    fn high_risk_commands_are_marked() {
        let report = seeded_command_parity_diff_report();
        let restore_row = report
            .rows
            .iter()
            .find(|row| row.descriptor.command_id == "cmd:workspace.restore_from_checkpoint")
            .expect("restore row present");
        assert!(restore_row.high_risk);
        let palette_row = report
            .rows
            .iter()
            .find(|row| row.descriptor.command_id == "cmd:command_palette.open")
            .expect("palette row present");
        assert!(!palette_row.high_risk);
    }

    #[test]
    fn unknown_high_risk_gap_blocks_when_high_risk() {
        let mut report = seeded_command_parity_diff_report();
        let row = report
            .rows
            .iter_mut()
            .find(|row| row.descriptor.command_id == "cmd:workspace.restore_from_checkpoint")
            .expect("restore row present");
        let surface = row
            .surfaces
            .iter_mut()
            .find(|projection| projection.surface_family == BetaSurfaceFamily::CommandPalette)
            .expect("palette projection present");
        surface.coverage_status = BetaCoverageStatus::UnknownHighRiskGap;
        surface.narrowing_reason = None;
        surface.projected_command_id = None;
        surface.projected_label_ref = None;
        surface.projected_lifecycle_label = None;
        surface.projected_preview_class = None;
        surface.projected_disabled_reason_mode = None;
        surface.projected_docs_help_anchor_ref = None;
        row.blocking_findings = compute_row_findings(&row.descriptor, &row.surfaces, row.high_risk);
        let errors = validate_command_parity_diff_report(&report)
            .expect_err("must flag unknown high-risk gap");
        assert!(errors.iter().any(|err| matches!(
            err,
            BetaCommandParityValidationError::BlockingFindingPresent { class, .. }
                if class == "unknown_high_risk_gap"
        )));
    }

    #[test]
    fn preview_class_drift_blocks() {
        let mut report = seeded_command_parity_diff_report();
        let row = report
            .rows
            .iter_mut()
            .find(|row| row.descriptor.command_id == "cmd:workspace.clone_repository")
            .expect("clone row present");
        let surface = row
            .surfaces
            .iter_mut()
            .find(|projection| projection.surface_family == BetaSurfaceFamily::CommandPalette)
            .expect("palette projection present");
        surface.projected_preview_class = Some(BetaPreviewClass::NoPreviewRequired);
        row.blocking_findings = compute_row_findings(&row.descriptor, &row.surfaces, row.high_risk);
        let errors =
            validate_command_parity_diff_report(&report).expect_err("must flag preview drift");
        assert!(errors.iter().any(|err| matches!(
            err,
            BetaCommandParityValidationError::BlockingFindingPresent { class, .. }
                if class == "preview_class_drift"
        )));
    }

    #[test]
    fn missing_narrowing_reason_blocks() {
        let mut report = seeded_command_parity_diff_report();
        let row = report
            .rows
            .iter_mut()
            .find(|row| row.descriptor.command_id == "cmd:command_palette.open")
            .expect("palette row present");
        let surface = row
            .surfaces
            .iter_mut()
            .find(|projection| projection.surface_family == BetaSurfaceFamily::CliHeadless)
            .expect("cli projection present");
        surface.narrowing_reason = None;
        row.blocking_findings = compute_row_findings(&row.descriptor, &row.surfaces, row.high_risk);
        let errors = validate_command_parity_diff_report(&report)
            .expect_err("must flag missing narrowing reason");
        assert!(errors.iter().any(|err| matches!(
            err,
            BetaCommandParityValidationError::BlockingFindingPresent { class, .. }
                if class == "missing_narrowing_reason"
        )));
    }

    #[test]
    fn missing_required_surface_blocks() {
        let mut report = seeded_command_parity_diff_report();
        let row = report
            .rows
            .iter_mut()
            .find(|row| row.descriptor.command_id == "cmd:workspace.open_folder")
            .expect("open_folder row present");
        row.surfaces
            .retain(|projection| projection.surface_family != BetaSurfaceFamily::KeybindingHelp);
        let errors = validate_command_parity_diff_report(&report)
            .expect_err("must flag missing required surface");
        assert!(errors.iter().any(|err| matches!(
            err,
            BetaCommandParityValidationError::MissingRequiredSurface { surface_family, .. }
                if surface_family == "keybinding_help"
        )));
    }

    #[test]
    fn support_export_quotes_every_command_id() {
        let report = seeded_command_parity_diff_report();
        let export = BetaCommandParitySupportExport::from_report(
            COMMAND_PARITY_SUPPORT_EXPORT_ID,
            report.clone(),
        );
        assert_eq!(
            export.shared_contract_ref,
            COMMAND_PARITY_SHARED_CONTRACT_REF
        );
        for row in &report.rows {
            assert!(
                export.case_ids.contains(&row.descriptor.command_id),
                "case ids must quote {}",
                row.descriptor.command_id,
            );
            assert!(
                export
                    .case_ids
                    .contains(&row.descriptor.descriptor_revision_ref),
                "case ids must quote {}",
                row.descriptor.descriptor_revision_ref,
            );
        }
    }
}
