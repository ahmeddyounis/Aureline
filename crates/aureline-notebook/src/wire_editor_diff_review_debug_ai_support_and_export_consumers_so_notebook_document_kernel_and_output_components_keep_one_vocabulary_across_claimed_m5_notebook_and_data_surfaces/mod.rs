//! Shared editor, diff/review, debug, AI-context, CLI/headless, and support /
//! export + release-packet consumers for the frozen M5 notebook document /
//! kernel / output components.
//!
//! This module is the M05-1090 consumer-adoption lane over the frozen M5
//! notebook-kernel-output component matrix
//! ([`crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix`]).
//! Where the freeze matrix defines the eight reusable notebook-document header,
//! kernel-state strip, kernel-picker row, kernel-origin pill, output-trust banner,
//! output-provenance chip group, restart-consequence card, and kernel-recovery
//! card primitives — and the four B129 implement lanes wire their resolvers and
//! controls contracts — this lane proves those families are reusable *primitives*
//! rather than per-feature notebook chrome. It adopts them across the claimed M5
//! notebook / data consumer classes:
//!
//! 1. a notebook editor surface,
//! 2. a diff / review surface,
//! 3. a debug surface,
//! 4. an AI-context surface,
//! 5. a CLI / headless surface, and
//! 6. a support / export + release-packet lane (support packet + release
//!    evidence; AC2).
//!
//! Each [`NotebookConsumerRow`] points back to exactly one canonical component
//! family (its per-family matrix schema) and the one canonical controls contract
//! (schema + doc + release-proof artifact) its family group belongs to, instead
//! of cloning feature-local notebook chrome. Every consumer — even a read-only,
//! inspect-only, export-only, or support replay — keeps the identical
//! document-identity, kernel-state, kernel-selection, kernel-origin, output-trust,
//! output-provenance, restart-consequence, and recovery-continuity labels and the
//! identical frozen kernel/output disposition vocabulary. A narrower consumer
//! discloses the reduction with a reduced-capability banner (and, when it punts to
//! another surface, a desktop / kernel-manager / browser / support-packet note)
//! rather than renaming or dropping governed notebook truth, so editor, diff,
//! review, debug, AI, CLI, and support panes never fork notebook / kernel / output
//! vocabulary by surface. This is what makes the same document, kernel, output,
//! and recovery state render with one vocabulary and one component family across
//! every claimed consumer (AC1), and lets support / export / release packets drop
//! bespoke feature-local translation tables (AC2).
//!
//! The four spec guardrails are enforced per row and must all stay false: no
//! consumer lets a kernel recovery card imply a rerun; no consumer presents stale
//! output as live; no consumer hides its raw / sanitized / active trust class
//! behind a hover-only affordance; no consumer collapses local, SSH, container,
//! managed, or browser-bridge kernels into one unlabeled badge.
//!
//! The packet is metadata-only: raw kernel connection strings, credential
//! material, and bearer secrets never cross this boundary; the packet carries only
//! typed class tokens, opaque notebook-state refs, booleans, and redacted labels.
//!
//! The schema is
//! [`schemas/ui/m5-notebook-kernel-output-component-consumer.schema.json`](../../../../schemas/ui/m5-notebook-kernel-output-component-consumer.schema.json).
//! The contract doc is
//! [`docs/notebooks/m5_notebook_kernel_output_component_consumer_contract.md`](../../../../docs/notebooks/m5_notebook_kernel_output_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix as matrix;
use crate::implement_kernel_picker_rows_and_kernel_origin_pills_with_kernel_class_environment_identity_locality_trust_limits_exact_or_degraded_provenance_and_rerun_reattach_continuity_across_claimed_m5_notebook_surfaces as kernel_choice_controls;
use crate::implement_notebook_document_headers_and_kernel_state_strips_with_canonical_ipynb_source_selected_kernel_origin_busy_queued_offline_truth_and_no_kernel_edit_parity_across_claimed_m5_notebook_surfaces as document_kernel_controls;
use crate::implement_output_trust_banners_and_output_provenance_chip_groups_with_plaintext_sanitizedrich_trustedlocalactive_isolatedremoteactive_class_stale_output_honesty_and_copy_export_choice_across_claimed_m5_notebook_outputs as output_trust_controls;
use crate::implement_restart_consequence_cards_and_kernel_recovery_cards_with_preserved_state_lost_state_reconnect_restart_clean_choose_another_kernel_actions_and_no_hidden_rerun_truth_across_claimed_m5_notebook_restore_and_failure_flows as restart_recovery_controls;

pub use matrix::{
    M5NotebookKernelOutputComponentFamily, M5NotebookKernelOutputConsumerSurface,
    M5NotebookKernelOutputDisposition,
};

/// Schema version stamped on the M05-1090 consumer packet.
pub const NOTEBOOK_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`NotebookConsumerPacket`].
pub const NOTEBOOK_CONSUMER_RECORD_KIND: &str =
    "m5_notebook_kernel_output_component_consumer_packet";

/// Stable record-kind tag carried by each [`NotebookConsumerRow`].
pub const NOTEBOOK_CONSUMER_ROW_RECORD_KIND: &str =
    "m5_notebook_kernel_output_component_consumer_row";

/// Repo-relative path of the consumer schema.
pub const NOTEBOOK_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-notebook-kernel-output-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const NOTEBOOK_CONSUMER_DOC_REF: &str =
    "docs/notebooks/m5_notebook_kernel_output_component_consumer_contract.md";

/// Repo-relative path of the frozen notebook-kernel-output component matrix release
/// proof these consumers adopt.
pub const NOTEBOOK_CONSUMER_MATRIX_REF: &str =
    matrix::M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_ARTIFACT_REF;

/// Repo-relative path of the shared frozen component-matrix schema.
pub const NOTEBOOK_CONSUMER_SHARED_SCHEMA_REF: &str =
    matrix::M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const NOTEBOOK_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const NOTEBOOK_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const NOTEBOOK_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-consumer-proof/report.md";

/// Repo-relative path of the checked consumer-fixture directory.
pub const NOTEBOOK_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-notebook-kernel-output-component-consumers";

/// The controlled label families a consumer must preserve identically across
/// every M5 notebook / data surface. These are the track-invariant truth pillars:
/// canonical `.ipynb` document identity, kernel state, kernel selection, kernel
/// origin / class, output trust class / freshness, output provenance, restart
/// consequence, and kernel-recovery continuity. The union of every row's
/// `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 8] = [
    "document_identity",
    "kernel_state",
    "kernel_selection",
    "kernel_origin",
    "output_trust",
    "output_provenance",
    "restart_consequence",
    "recovery_continuity",
];

/// The canonical kernel/output disposition vocabulary every consumer keeps visible
/// even when narrowed or export-only — the frozen
/// [`M5NotebookKernelOutputDisposition`] set (no-kernel / queued / busy / ready /
/// disconnected / managed / remote / stale-output / sanitized / active / reconnect
/// / restart-clean / choose-another-kernel). Every consumer renders the same
/// kernel / output / recovery state with these exact tokens rather than
/// feature-local phrasing (AC1).
pub fn canonical_notebook_disposition_vocab() -> Vec<String> {
    M5NotebookKernelOutputDisposition::ALL
        .iter()
        .map(|d| d.as_str().to_owned())
        .collect()
}

/// Whether a token is one of the frozen kernel/output disposition tokens.
pub fn is_canonical_notebook_disposition(token: &str) -> bool {
    M5NotebookKernelOutputDisposition::ALL
        .iter()
        .any(|d| d.as_str() == token)
}

/// The canonical per-family matrix schema that defines a family's contract.
pub const fn canonical_family_schema_ref_for(
    family: M5NotebookKernelOutputComponentFamily,
) -> &'static str {
    use M5NotebookKernelOutputComponentFamily::*;
    match family {
        NotebookDocumentHeader => matrix::M5_NOTEBOOK_DOCUMENT_HEADER_SCHEMA_REF,
        KernelStateStrip => matrix::M5_KERNEL_STATE_STRIP_SCHEMA_REF,
        KernelPickerRow => matrix::M5_KERNEL_PICKER_ROW_SCHEMA_REF,
        KernelOriginPill => matrix::M5_KERNEL_ORIGIN_PILL_SCHEMA_REF,
        OutputTrustBanner => matrix::M5_OUTPUT_TRUST_BANNER_SCHEMA_REF,
        OutputProvenanceChipGroup => matrix::M5_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF,
        RestartConsequenceCard => matrix::M5_RESTART_CONSEQUENCE_CARD_SCHEMA_REF,
        KernelRecoveryCard => matrix::M5_KERNEL_RECOVERY_CARD_SCHEMA_REF,
    }
}

/// The single primary notebook label family a component family must always
/// preserve — the notebook / kernel / output axis it exists to name. A consumer
/// may narrow authority, but it must never drop this label, so the family's core
/// document-identity, kernel-state, kernel-selection, kernel-origin, output-trust,
/// output-provenance, restart-consequence, or recovery-continuity truth is never
/// silently lost.
pub const fn family_primary_label(family: M5NotebookKernelOutputComponentFamily) -> &'static str {
    use M5NotebookKernelOutputComponentFamily::*;
    match family {
        NotebookDocumentHeader => "document_identity",
        KernelStateStrip => "kernel_state",
        KernelPickerRow => "kernel_selection",
        KernelOriginPill => "kernel_origin",
        OutputTrustBanner => "output_trust",
        OutputProvenanceChipGroup => "output_provenance",
        RestartConsequenceCard => "restart_consequence",
        KernelRecoveryCard => "recovery_continuity",
    }
}

/// The four B129 controls contracts the eight component families group into. A
/// consumer must point at the one canonical controls contract for its family's
/// lane rather than inventing a feature-local one — this is the heart of the
/// "notebook / data surfaces no longer fork notebook vocabulary" acceptance
/// criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookControlsLane {
    /// Notebook-document header + kernel-state strip controls (M05-1085 lane).
    DocumentKernel,
    /// Kernel-picker row + kernel-origin pill controls (M05-1086 lane).
    KernelChoice,
    /// Output-trust banner + output-provenance chip group controls (M05-1087
    /// lane).
    OutputTrust,
    /// Restart-consequence card + kernel-recovery card controls (M05-1088 lane).
    RestartRecovery,
}

impl M5NotebookControlsLane {
    /// Every controls lane, in declaration order.
    pub const ALL: [M5NotebookControlsLane; 4] = [
        M5NotebookControlsLane::DocumentKernel,
        M5NotebookControlsLane::KernelChoice,
        M5NotebookControlsLane::OutputTrust,
        M5NotebookControlsLane::RestartRecovery,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocumentKernel => "document_kernel",
            Self::KernelChoice => "kernel_choice",
            Self::OutputTrust => "output_trust",
            Self::RestartRecovery => "restart_recovery",
        }
    }

    /// The canonical controls schema every surface reuses for this lane.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::DocumentKernel => {
                document_kernel_controls::NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_REF
            }
            Self::KernelChoice => {
                kernel_choice_controls::KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_REF
            }
            Self::OutputTrust => {
                output_trust_controls::OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF
            }
            Self::RestartRecovery => {
                restart_recovery_controls::RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_REF
            }
        }
    }

    /// The canonical controls contract doc for this lane.
    pub const fn canonical_doc_ref(self) -> &'static str {
        match self {
            Self::DocumentKernel => {
                document_kernel_controls::NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_DOC_REF
            }
            Self::KernelChoice => {
                kernel_choice_controls::KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_DOC_REF
            }
            Self::OutputTrust => {
                output_trust_controls::OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_DOC_REF
            }
            Self::RestartRecovery => {
                restart_recovery_controls::RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_DOC_REF
            }
        }
    }

    /// The canonical controls release-proof artifact every consumer points back to
    /// as the first-resolved truth for this lane.
    pub const fn canonical_artifact_ref(self) -> &'static str {
        match self {
            Self::DocumentKernel => {
                document_kernel_controls::NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_ARTIFACT_REF
            }
            Self::KernelChoice => {
                kernel_choice_controls::KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_ARTIFACT_REF
            }
            Self::OutputTrust => {
                output_trust_controls::OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_ARTIFACT_REF
            }
            Self::RestartRecovery => {
                restart_recovery_controls::RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_ARTIFACT_REF
            }
        }
    }
}

/// The one controls lane a component family belongs to. The eight frozen families
/// group into the four B129 controls contracts; a consumer must reuse the lane's
/// canonical contract rather than forking it per surface.
pub const fn controls_lane_for(
    family: M5NotebookKernelOutputComponentFamily,
) -> M5NotebookControlsLane {
    use M5NotebookKernelOutputComponentFamily::*;
    match family {
        NotebookDocumentHeader | KernelStateStrip => M5NotebookControlsLane::DocumentKernel,
        KernelPickerRow | KernelOriginPill => M5NotebookControlsLane::KernelChoice,
        OutputTrustBanner | OutputProvenanceChipGroup => M5NotebookControlsLane::OutputTrust,
        RestartConsequenceCard | KernelRecoveryCard => M5NotebookControlsLane::RestartRecovery,
    }
}

/// The six claimed M5 notebook / data consumer classes that must each adopt at
/// least one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerClass {
    /// A notebook editor surface.
    NotebookEditor,
    /// A diff / review surface.
    DiffReview,
    /// A debug surface.
    Debug,
    /// An AI-context surface.
    AiContext,
    /// A CLI / headless surface.
    Cli,
    /// A support / export + release-packet lane (support packet + release
    /// evidence; AC2).
    SupportExport,
}

impl ConsumerClass {
    /// Every consumer class that must be present for cross-surface reuse.
    pub const ALL: [ConsumerClass; 6] = [
        ConsumerClass::NotebookEditor,
        ConsumerClass::DiffReview,
        ConsumerClass::Debug,
        ConsumerClass::AiContext,
        ConsumerClass::Cli,
        ConsumerClass::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookEditor => "notebook_editor",
            Self::DiffReview => "diff_review",
            Self::Debug => "debug",
            Self::AiContext => "ai_context",
            Self::Cli => "cli",
            Self::SupportExport => "support_export",
        }
    }

    /// True when this class renders live kernel / output truth (notebook editor,
    /// debug, or AI context) and therefore must never drop the adopted family's
    /// primary label or present stale kernel / output state as live — the truth
    /// that says whether the kernel is live, the output is fresh, and continuity
    /// survived recovery.
    pub const fn is_runtime_bearing(self) -> bool {
        matches!(self, Self::NotebookEditor | Self::Debug | Self::AiContext)
    }
}

/// The consumer class a concrete matrix consumer surface belongs to. Reuses the
/// matrix's own [`M5NotebookKernelOutputConsumerSurface`] taxonomy rather than
/// inventing a parallel one.
pub const fn consumer_class_for(surface: M5NotebookKernelOutputConsumerSurface) -> ConsumerClass {
    use M5NotebookKernelOutputConsumerSurface::*;
    match surface {
        NotebookUi | KernelManagerUi | OutputViewerUi => ConsumerClass::NotebookEditor,
        ReviewUi => ConsumerClass::DiffReview,
        DebuggerUi => ConsumerClass::Debug,
        AiContextUi | ProductUi => ConsumerClass::AiContext,
        CliSurface => ConsumerClass::Cli,
        SupportExport => ConsumerClass::SupportExport,
    }
}

/// True when this surface is the notebook editor surface — the first claimed
/// editor consumer whose canonical adoption AC1 anchors.
pub const fn is_editor_surface(surface: M5NotebookKernelOutputConsumerSurface) -> bool {
    matches!(surface, M5NotebookKernelOutputConsumerSurface::NotebookUi)
}

/// True when this surface is the support / export + release-packet surface (AC2).
pub const fn is_support_export_surface(surface: M5NotebookKernelOutputConsumerSurface) -> bool {
    matches!(
        surface,
        M5NotebookKernelOutputConsumerSurface::SupportExport
    )
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, override-gated,
/// export-only, policy-blocked) but never rename or drop the governed notebook
/// truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Full-interactive control (act on the notebook component directly).
    FullInteractive,
    /// Read-only projection of the component.
    ReadOnly,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Override-gated: the action is visible but staged behind an explicit review
    /// gate (e.g. review-before-restart or review-before-switch) before it applies.
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

/// The surface a narrower consumer hands off to when it cannot act on the notebook
/// component locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders and acts on the component in-place.
    None,
    /// Punt to the desktop notebook editor to act on the kernel / output state.
    NotebookDesktop,
    /// Punt to the kernel-manager surface.
    KernelManager,
    /// Punt to a read-only browser surface (the browser bridge).
    BrowserReadonly,
    /// Punt to a portable support / export packet.
    SupportPacket,
    /// Punt to a headless CLI.
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore must
    /// carry a desktop / kernel-manager / browser / support note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NotebookDesktop => "notebook_desktop",
            Self::KernelManager => "kernel_manager",
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
    /// Full label parity across the notebook truth pillars.
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
/// screenshot-only export is prohibited (it would lose the machine-readable
/// document / kernel / output / recovery identity support and automation need to
/// reconstruct the notebook state).
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
/// it drops relative to the full notebook component.
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

/// One consumer adopting one canonical notebook-kernel-output component family on
/// one M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookConsumerRow {
    /// Record kind; must equal [`NOTEBOOK_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`NOTEBOOK_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_class: ConsumerClass,
    /// The concrete surface; must belong to `consumer_class`.
    pub consumer_surface: M5NotebookKernelOutputConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5NotebookKernelOutputComponentFamily,
    /// The controls lane the family belongs to; must equal
    /// `controls_lane_for(component_family)`.
    pub controls_lane: M5NotebookControlsLane,
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
    /// rather than cloning feature-local notebook chrome.
    pub references_canonical_not_local_prose: bool,
    /// An opaque, redaction-safe ref to the document / kernel / output state the
    /// user saw, so support and automation can reconstruct it without leaking raw
    /// kernel connection strings, credential material, or bearer secrets.
    pub notebook_state_ref: String,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The frozen kernel/output disposition vocabulary the consumer keeps visible
    /// even when narrowed.
    #[serde(default)]
    pub notebook_disposition_vocab: Vec<String>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The desktop / kernel-manager / browser / support note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Guardrail: the consumer lets a kernel recovery card imply a rerun. Must be
    /// false.
    pub recovery_card_implies_rerun: bool,
    /// Guardrail: the consumer presents stale output as live. Must be false.
    pub presents_stale_output_as_live: bool,
    /// Guardrail: the consumer hides a raw / sanitized / active trust class behind a
    /// hover-only affordance. Must be false.
    pub hides_trust_class_behind_hover_only: bool,
    /// Guardrail: the consumer collapses local, SSH, container, managed, or
    /// browser-bridge kernels into one unlabeled badge. Must be false.
    pub collapses_kernel_origins_into_one_badge: bool,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl NotebookConsumerRow {
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
    /// artifact is referenced, and no feature-local notebook chrome is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_family_schema_ref_for(self.component_family)
            && self.controls_lane_is_canonical()
            && self.references_canonical_not_local_prose
    }

    /// AC1 (parity): the consumer preserves the family's controlled label families
    /// and frozen kernel/output disposition vocabulary rather than renaming or
    /// omitting them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.notebook_disposition_vocab.is_empty()
            && self
                .notebook_disposition_vocab
                .iter()
                .all(|v| is_canonical_notebook_disposition(v))
    }

    /// AC (notebook truth): every row preserves the adopted family's primary
    /// notebook label, and a runtime-bearing consumer (notebook editor, debug, or
    /// AI context) never drops it — so a live surface never hides whether the
    /// kernel is live, the output is fresh, or continuity survived recovery.
    pub fn preserves_primary_notebook_truth(&self) -> bool {
        let primary = family_primary_label(self.component_family);
        self.preserved_label_families.iter().any(|f| f == primary)
    }

    /// AC2: the row carries the opaque notebook-state ref and canonical controls
    /// contract support and automation reconstruct the seen state from.
    pub fn supports_state_reconstruction(&self) -> bool {
        !self.notebook_state_ref.trim().is_empty()
            && self.controls_lane_is_canonical()
            && self.copy_export.is_complete()
    }

    /// The four spec guardrails are all clear (false).
    pub fn guardrails_clear(&self) -> bool {
        self.first_failed_guardrail().is_none()
    }

    /// The first guardrail that is (wrongly) set, if any.
    pub fn first_failed_guardrail(&self) -> Option<&'static str> {
        if self.recovery_card_implies_rerun {
            Some("recovery_card_implies_rerun")
        } else if self.presents_stale_output_as_live {
            Some("presents_stale_output_as_live")
        } else if self.hides_trust_class_behind_hover_only {
            Some("hides_trust_class_behind_hover_only")
        } else if self.collapses_kernel_origins_into_one_badge {
            Some("collapses_kernel_origins_into_one_badge")
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
        self.record_kind == NOTEBOOK_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == NOTEBOOK_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.notebook_state_ref.trim().is_empty()
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

/// Rolled-up summary of an M05-1090 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookConsumerSummary {
    pub row_count: usize,
    pub consumer_class_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub controls_lane_count: usize,
    pub disposition_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_use_canonical_controls_lane: bool,
    pub all_runtime_rows_preserve_primary_truth: bool,
    pub all_rows_reconstructable: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub all_rows_guardrails_clear: bool,
    pub controls_lanes_stable_across_surfaces: bool,
    pub notebook_editor_consumer_present: bool,
    pub diff_review_consumer_present: bool,
    pub debug_consumer_present: bool,
    pub ai_context_consumer_present: bool,
    pub cli_consumer_present: bool,
    pub support_export_consumer_present: bool,
    pub editor_reference_present: bool,
    pub support_export_reference_present: bool,
    pub label_family_coverage_complete: bool,
    pub disposition_coverage_complete: bool,
    pub families_reused_across_classes: usize,
}

/// Constructor input for [`NotebookConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotebookConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<NotebookConsumerRow>,
}

/// Checked-in M05-1090 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<NotebookConsumerRow>,
    pub summary: NotebookConsumerSummary,
}

impl NotebookConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: NotebookConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: NOTEBOOK_CONSUMER_SCHEMA_VERSION,
            record_kind: NOTEBOOK_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: NotebookConsumerSummary {
                row_count: 0,
                consumer_class_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                controls_lane_count: 0,
                disposition_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_use_canonical_controls_lane: false,
                all_runtime_rows_preserve_primary_truth: false,
                all_rows_reconstructable: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                all_rows_guardrails_clear: false,
                controls_lanes_stable_across_surfaces: false,
                notebook_editor_consumer_present: false,
                diff_review_consumer_present: false,
                debug_consumer_present: false,
                ai_context_consumer_present: false,
                cli_consumer_present: false,
                support_export_consumer_present: false,
                editor_reference_present: false,
                support_export_reference_present: false,
                label_family_coverage_complete: false,
                disposition_coverage_complete: false,
                families_reused_across_classes: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5NotebookKernelOutputComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The union of every row's kernel/output disposition vocabulary.
    pub fn covered_dispositions(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.notebook_disposition_vocab.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// classes — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_classes(&self) -> usize {
        M5NotebookKernelOutputComponentFamily::ALL
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
            M5NotebookKernelOutputComponentFamily,
            BTreeSet<M5NotebookControlsLane>,
        > = BTreeMap::new();
        for row in &self.rows {
            per_family
                .entry(row.component_family)
                .or_default()
                .insert(row.controls_lane);
        }
        per_family.values().all(|lanes| lanes.len() <= 1)
    }

    /// Whether some notebook editor surface references the canonical families — the
    /// first-claimed-consumer half of AC1.
    pub fn has_editor_reference(&self) -> bool {
        self.rows.iter().any(|r| {
            is_editor_surface(r.consumer_surface) && r.references_canonical_not_local_prose
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
    pub fn computed_summary(&self) -> NotebookConsumerSummary {
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
        let covered_dispositions = self.covered_dispositions();

        NotebookConsumerSummary {
            row_count: self.rows.len(),
            consumer_class_count: classes.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            controls_lane_count: lanes.len(),
            disposition_count: covered_dispositions.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(NotebookConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self.rows.iter().all(NotebookConsumerRow::preserves_labels),
            all_rows_use_canonical_controls_lane: self
                .rows
                .iter()
                .all(NotebookConsumerRow::controls_lane_is_canonical),
            all_runtime_rows_preserve_primary_truth: self
                .rows
                .iter()
                .all(NotebookConsumerRow::preserves_primary_notebook_truth),
            all_rows_reconstructable: self
                .rows
                .iter()
                .all(NotebookConsumerRow::supports_state_reconstruction),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(NotebookConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            all_rows_guardrails_clear: self.rows.iter().all(NotebookConsumerRow::guardrails_clear),
            controls_lanes_stable_across_surfaces: self.controls_lanes_stable_across_surfaces(),
            notebook_editor_consumer_present: has_class(ConsumerClass::NotebookEditor),
            diff_review_consumer_present: has_class(ConsumerClass::DiffReview),
            debug_consumer_present: has_class(ConsumerClass::Debug),
            ai_context_consumer_present: has_class(ConsumerClass::AiContext),
            cli_consumer_present: has_class(ConsumerClass::Cli),
            support_export_consumer_present: has_class(ConsumerClass::SupportExport),
            editor_reference_present: self.has_editor_reference(),
            support_export_reference_present: self.has_support_export_reference(),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            disposition_coverage_complete: M5NotebookKernelOutputDisposition::ALL
                .iter()
                .all(|d| covered_dispositions.contains(d.as_str())),
            families_reused_across_classes: self.families_reused_across_classes(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<NotebookConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != NOTEBOOK_CONSUMER_SCHEMA_VERSION {
            violations.push(NotebookConsumerViolation::SchemaVersion {
                expected: NOTEBOOK_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != NOTEBOOK_CONSUMER_RECORD_KIND {
            violations.push(NotebookConsumerViolation::RecordKind {
                expected: NOTEBOOK_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(NotebookConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(NotebookConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_classes.insert(row.consumer_class);

            if !row.is_complete() {
                violations.push(NotebookConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer class.
            if !row.surface_class_consistent() {
                violations.push(NotebookConsumerViolation::SurfaceClassMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned feature-local chrome.
            if !row.points_to_canonical_family() {
                violations.push(NotebookConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC (no fork): canonical controls lane per family.
            if !row.controls_lane_is_canonical() {
                violations.push(NotebookConsumerViolation::NonCanonicalControlsLane {
                    id: row.row_id.clone(),
                });
            }

            // AC1: controlled label families / disposition vocab preserved.
            if !row.preserves_labels() {
                violations.push(NotebookConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC (notebook truth): the family's primary notebook label is kept.
            if !row.preserves_primary_notebook_truth() {
                violations.push(NotebookConsumerViolation::PrimaryNotebookTruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // AC2: state is reconstructable from the opaque ref + canonical controls
            // contract.
            if !row.supports_state_reconstruction() {
                violations.push(NotebookConsumerViolation::StateNotReconstructable {
                    id: row.row_id.clone(),
                });
            }

            // Disclosure: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(NotebookConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(NotebookConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }

            // Spec guardrails must all stay false.
            if let Some(guardrail) = row.first_failed_guardrail() {
                violations.push(NotebookConsumerViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                    guardrail,
                });
            }
        }

        // Cross-surface reuse spans all six claimed consumer classes.
        for class in ConsumerClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(NotebookConsumerViolation::MissingConsumerClass { class });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5NotebookKernelOutputComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(NotebookConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer classes so
        // multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_classes() == 0 {
            violations.push(NotebookConsumerViolation::NoFamilyReusedAcrossClasses);
        }

        // AC (no fork): families resolve to one stable controls lane per family.
        if !self.controls_lanes_stable_across_surfaces() {
            violations.push(NotebookConsumerViolation::ControlsLaneForkedAcrossSurfaces);
        }

        // AC1: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(NotebookConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC1: the frozen kernel/output disposition vocabulary is collectively
        // preserved.
        let covered_dispositions = self.covered_dispositions();
        for disposition in M5NotebookKernelOutputDisposition::ALL {
            if !covered_dispositions.contains(disposition.as_str()) {
                violations.push(NotebookConsumerViolation::MissingDisposition {
                    disposition: disposition.as_str().to_owned(),
                });
            }
        }

        // AC1: a notebook editor consumer references the canonical components rather
        // than cloning feature-local notebook chrome.
        if !self.has_editor_reference() {
            violations.push(NotebookConsumerViolation::MissingEditorReference);
        }

        // AC2: a support / export + release-packet consumer references the canonical
        // components so release packets drop feature-local translation tables.
        if !self.has_support_export_reference() {
            violations.push(NotebookConsumerViolation::MissingSupportExportReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(NotebookConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(NotebookConsumerViolation::RawNotebookMaterialInExport);
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
        out.push_str("# M5 Notebook-Kernel-Output Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer classes and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_class_count,
            self.represented_families().len(),
            M5NotebookKernelOutputComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Controls lanes adopted: {} / {}\n",
            self.summary.controls_lane_count,
            M5NotebookControlsLane::ALL.len(),
        ));
        out.push_str(&format!(
            "- Kernel/output dispositions preserved: {} / {}\n",
            self.summary.disposition_count,
            M5NotebookKernelOutputDisposition::ALL.len(),
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
pub fn current_m5_notebook_kernel_output_component_consumers_export(
) -> Result<NotebookConsumerPacket, NotebookConsumerArtifactError> {
    let packet: NotebookConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-notebook-kernel-output-component-consumer-proof/support_export.json"
    )))
    .map_err(NotebookConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NotebookConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum NotebookConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<NotebookConsumerViolation>),
}

impl fmt::Display for NotebookConsumerArtifactError {
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

impl Error for NotebookConsumerArtifactError {}

/// Validation failure for M05-1090 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotebookConsumerViolation {
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
    PrimaryNotebookTruthDropped {
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
        family: M5NotebookKernelOutputComponentFamily,
    },
    NoFamilyReusedAcrossClasses,
    ControlsLaneForkedAcrossSurfaces,
    MissingLabelFamily {
        family: String,
    },
    MissingDisposition {
        disposition: String,
    },
    MissingEditorReference,
    MissingSupportExportReference,
    SummaryMismatch,
    RawNotebookMaterialInExport,
}

impl fmt::Display for NotebookConsumerViolation {
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
                    "row {id} renames or drops a canonical document-identity, kernel-state, \
kernel-selection, kernel-origin, output-trust, output-provenance, restart-consequence, or \
recovery-continuity label"
                )
            }
            Self::PrimaryNotebookTruthDropped { id } => {
                write!(
                    f,
                    "row {id} drops the adopted family's primary notebook label (document identity, \
kernel state, kernel selection, kernel origin, output trust, output provenance, restart \
consequence, or recovery continuity)"
                )
            }
            Self::StateNotReconstructable { id } => {
                write!(
                    f,
                    "row {id} cannot be reconstructed from its notebook-state ref and controls contract"
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
            Self::MissingDisposition { disposition } => {
                write!(
                    f,
                    "kernel/output disposition token {disposition} is not preserved anywhere"
                )
            }
            Self::MissingEditorReference => write!(
                f,
                "no notebook editor consumer references the canonical component families"
            ),
            Self::MissingSupportExportReference => write!(
                f,
                "no support / export consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawNotebookMaterialInExport => {
                write!(f, "export contains raw notebook material")
            }
        }
    }
}

impl Error for NotebookConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
/// Adds the notebook / kernel / output generic phrasings the spec forbids
/// collapsing into (offline, stale, busy, queued, disconnected, remote, managed,
/// sanitized) to the shared generic-label blocklist. These are matched as *whole*
/// labels rather than substrings so a descriptive banner may still name "remote
/// kernel disconnected" or "sanitized rich output" as a state without being
/// flagged; only a banner whose entire label collapses to the generic phrase is
/// rejected.
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
            | "busy"
            | "queued"
            | "disconnected"
            | "remote"
            | "managed"
            | "sanitized"
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
pub fn seeded_m5_notebook_kernel_output_component_consumers_packet() -> NotebookConsumerPacket {
    NotebookConsumerPacket::new(NotebookConsumerPacketInput {
        packet_id: "m5-notebook-kernel-output-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: NOTEBOOK_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:notebook-kernel-output-consumer:{id}")]
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
    consumer_surface: M5NotebookKernelOutputConsumerSurface,
    component_family: M5NotebookKernelOutputComponentFamily,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> NotebookConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    let controls_lane = controls_lane_for(component_family);
    NotebookConsumerRow {
        record_kind: NOTEBOOK_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: NOTEBOOK_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_class: consumer_class_for(consumer_surface),
        consumer_surface,
        component_family,
        controls_lane,
        canonical_family_schema_ref: canonical_family_schema_ref_for(component_family).to_owned(),
        canonical_controls_schema_ref: controls_lane.canonical_schema_ref().to_owned(),
        canonical_controls_artifact_refs: vec![controls_lane.canonical_artifact_ref().to_owned()],
        references_canonical_not_local_prose: true,
        notebook_state_ref: format!("notebook-state:{row_id}"),
        authority_mode,
        preserved_label_families: labels(label_families),
        notebook_disposition_vocab: canonical_notebook_disposition_vocab(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        recovery_card_implies_rerun: false,
        presents_stale_output_as_live: false,
        hides_trust_class_behind_hover_only: false,
        collapses_kernel_origins_into_one_badge: false,
        source_refs: vec![
            NOTEBOOK_CONSUMER_MATRIX_REF.to_owned(),
            NOTEBOOK_CONSUMER_SHARED_SCHEMA_REF.to_owned(),
            controls_lane.canonical_doc_ref().to_owned(),
        ],
        observed_at: "2026-07-11T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<NotebookConsumerRow> {
    use AuthorityMode::*;
    use HandoffTarget as H;
    use M5NotebookKernelOutputComponentFamily::*;
    use M5NotebookKernelOutputConsumerSurface::*;

    vec![
        // --- Notebook editor -----------------------------------------------
        row(
            "consumer:notebook-editor:notebook-document-header",
            NotebookUi,
            NotebookDocumentHeader,
            FullInteractive,
            &["document_identity", "kernel_state"],
            &["document_identity", "kernel_state", "controls_lane"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:notebook-editor:kernel-state-strip",
            NotebookUi,
            KernelStateStrip,
            ReadOnly,
            &["kernel_state", "kernel_origin", "document_identity"],
            &["kernel_state", "kernel_origin", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:notebook-editor:kernel-state-strip",
                "Read-only kernel-state strip: names whether the selected kernel is no-kernel, queued, busy, ready, disconnected, managed, or remote; selecting or restarting the kernel stays in the desktop notebook",
                ReadOnly,
                &["select_kernel", "restart_kernel"],
            )),
        ),
        row(
            "consumer:notebook-editor:kernel-picker-row",
            KernelManagerUi,
            KernelPickerRow,
            OverrideGated,
            &["kernel_selection", "kernel_origin", "kernel_state"],
            &["kernel_selection", "kernel_origin", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:notebook-editor:kernel-picker-row",
                "Review-gated kernel-picker row in the kernel manager: names each candidate kernel's class, environment identity, and exact-or-degraded provenance before a switch applies, never overwriting the resolved kernel without review",
                OverrideGated,
                &["switch_kernel_immediately", "override_provenance"],
            )),
        ),
        row(
            "consumer:notebook-editor:kernel-origin-pill",
            KernelManagerUi,
            KernelOriginPill,
            ReadOnly,
            &["kernel_origin", "kernel_state", "document_identity"],
            &["kernel_origin", "kernel_state", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:notebook-editor:kernel-origin-pill",
                "Read-only kernel-origin pill: names the local, SSH, container, managed, or browser-bridge origin class and trust limits behind the selected kernel, never collapsing distinct kernel origins into one badge",
                ReadOnly,
                &["reattach_kernel", "edit_origin"],
            )),
        ),
        row(
            "consumer:notebook-editor:output-trust-banner",
            OutputViewerUi,
            OutputTrustBanner,
            ReadOnly,
            &["output_trust", "output_provenance", "kernel_state"],
            &["output_trust", "output_provenance", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:notebook-editor:output-trust-banner",
                "Read-only output-trust banner in the output viewer: names the plain-text, sanitized-rich, trusted-local-active, or isolated-remote-active trust class and whether the output is live or stale, never presenting stale output as live",
                ReadOnly,
                &["run_cell", "mark_output_trusted"],
            )),
        ),
        row(
            "consumer:notebook-editor:output-provenance-chip-group",
            OutputViewerUi,
            OutputProvenanceChipGroup,
            InspectOnly,
            &["output_provenance", "output_trust", "kernel_state"],
            &["output_provenance", "output_trust", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:notebook-editor:output-provenance-chip-group",
                "Inspect-only output-provenance chip group: names the producing kernel run identity and lineage behind each output so a rendered result never loses its canonical provenance",
                InspectOnly,
                &["rerun_producing_cell", "edit_provenance"],
            )),
        ),
        row(
            "consumer:notebook-editor:kernel-recovery-card",
            NotebookUi,
            KernelRecoveryCard,
            OverrideGated,
            &["recovery_continuity", "kernel_state", "restart_consequence"],
            &["recovery_continuity", "kernel_state", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:notebook-editor:kernel-recovery-card",
                "Review-gated kernel-recovery card: names reconnect, restart-clean, and choose-another-kernel recovery and the preserved-versus-lost state each keeps, never implying a hidden rerun on recovery",
                OverrideGated,
                &["auto_rerun_on_recovery", "recover_without_review"],
            )),
        ),
        // --- Diff / review -------------------------------------------------
        row(
            "consumer:diff-review:output-trust-banner",
            ReviewUi,
            OutputTrustBanner,
            ReadOnly,
            &["output_trust", "output_provenance", "document_identity"],
            &["output_trust", "output_provenance", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:diff-review:output-trust-banner",
                "Read-only output-trust banner in the diff / review pane: names the trust class and live-versus-stale freshness of each diffed output so a reviewer never reads a stale or sanitized output as live truth",
                ReadOnly,
                &["run_cell", "accept_output"],
            )),
        ),
        row(
            "consumer:diff-review:restart-consequence-card",
            ReviewUi,
            RestartConsequenceCard,
            InspectOnly,
            &["restart_consequence", "recovery_continuity", "kernel_state"],
            &["restart_consequence", "recovery_continuity", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:diff-review:restart-consequence-card",
                "Inspect-only restart-consequence card in review: names what a kernel restart-clean would clear and preserve so a reviewer sees the consequence without triggering it or implying a rerun",
                InspectOnly,
                &["restart_kernel", "apply_restart"],
            )),
        ),
        // --- Debug ---------------------------------------------------------
        row(
            "consumer:debug:kernel-state-strip",
            DebuggerUi,
            KernelStateStrip,
            InspectOnly,
            &["kernel_state", "kernel_origin", "restart_consequence"],
            &["kernel_state", "kernel_origin", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:debug:kernel-state-strip",
                "Inspect-only kernel-state strip in the debugger: names whether the debugged kernel is busy, queued, disconnected, managed, or remote so a debug session never runs against an anonymous kernel",
                InspectOnly,
                &["select_kernel", "restart_kernel"],
            )),
        ),
        row(
            "consumer:debug:restart-consequence-card",
            DebuggerUi,
            RestartConsequenceCard,
            OverrideGated,
            &["restart_consequence", "recovery_continuity", "kernel_state"],
            &["restart_consequence", "recovery_continuity", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:debug:restart-consequence-card",
                "Review-gated restart-consequence card in the debugger: names the debugger and session state a kernel restart-clean would end versus keep before it applies, never implying the outputs silently rerun",
                OverrideGated,
                &["restart_without_review", "auto_rerun_after_restart"],
            )),
        ),
        // --- AI context ----------------------------------------------------
        row(
            "consumer:ai-context:kernel-origin-pill",
            AiContextUi,
            KernelOriginPill,
            InspectOnly,
            &["kernel_origin", "kernel_state", "output_provenance"],
            &["kernel_origin", "kernel_state", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:ai-context:kernel-origin-pill",
                "Inspect-only kernel-origin pill for AI context: names the local, container, managed, or remote origin class and trust limits behind the kernel the model reasons about, so a remote kernel never reads as local first-party execution",
                InspectOnly,
                &["reattach_kernel", "edit_origin"],
            )),
        ),
        row(
            "consumer:ai-context:output-provenance-chip-group",
            AiContextUi,
            OutputProvenanceChipGroup,
            ReadOnly,
            &["output_provenance", "output_trust", "kernel_origin"],
            &["output_provenance", "output_trust", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:ai-context:output-provenance-chip-group",
                "Read-only output-provenance chip group for AI context: names the producing run identity and trust class behind each output the model ingests so retrieved evidence never loses its lineage or is treated as live when stale",
                ReadOnly,
                &["rerun_producing_cell", "trust_output"],
            )),
        ),
        // --- CLI / headless ------------------------------------------------
        row(
            "consumer:cli:output-trust-banner",
            CliSurface,
            OutputTrustBanner,
            ExportOnly,
            &["output_trust", "output_provenance", "kernel_state"],
            &["output_trust", "output_provenance", "notebook_state_ref", "controls_lane"],
            H::CliHeadless,
            "handoff:cli:output-trust-banner-cli-headless",
            Some(banner(
                "banner:cli:output-trust-banner",
                "Headless CLI output-trust line: reconstruct the plain-text, sanitized-rich, or isolated-remote-active trust class and the live-versus-stale freshness of each output from the exported packet without a rich viewer",
                ExportOnly,
                &["run_cell", "render_rich_output"],
            )),
        ),
        // --- Support / export + release packet (AC2) -----------------------
        row(
            "consumer:support-export:notebook-document-header",
            SupportExport,
            NotebookDocumentHeader,
            ExportOnly,
            &["document_identity", "kernel_state", "kernel_origin"],
            &["document_identity", "kernel_state", "notebook_state_ref", "controls_lane"],
            H::SupportPacket,
            "handoff:support-export:notebook-document-header-support-packet",
            Some(banner(
                "banner:support-export:notebook-document-header",
                "Export-only support replay: reconstruct the canonical .ipynb document identity, its source class, and the selected kernel origin the user saw from the support packet",
                ExportOnly,
                &["open_notebook", "edit_header"],
            )),
        ),
        row(
            "consumer:support-export:kernel-recovery-card",
            SupportExport,
            KernelRecoveryCard,
            ExportOnly,
            &["recovery_continuity", "restart_consequence", "kernel_state"],
            &["recovery_continuity", "restart_consequence", "notebook_state_ref", "controls_lane"],
            H::SupportPacket,
            "handoff:support-export:kernel-recovery-card-support-packet",
            Some(banner(
                "banner:support-export:kernel-recovery-card",
                "Export-only support replay: reconstruct the reconnect / restart-clean / choose-another-kernel recovery the user was offered and the preserved-versus-lost state, and that recovery never implied a hidden rerun, from the support packet",
                ExportOnly,
                &["reconnect_kernel", "auto_rerun_on_recovery"],
            )),
        ),
    ]
}
