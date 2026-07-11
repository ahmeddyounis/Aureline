//! Two reusable M5 notebook components — the notebook document header and the kernel-state
//! strip — so a user can orient to a notebook's canonical source and its live kernel state
//! before they run, debug, review, or trust any output: the document header names its canonical
//! `.ipynb` identity, where that `.ipynb` came from, where its saved / unsaved / conflicted /
//! read-only / recovered identity stands, its paired export state, and its current target /
//! workspace context, and offers first-class open / export / review actions; the kernel-state
//! strip names where the selected kernel stands in execution (no kernel, queued, busy, ready,
//! interrupted, disconnected / reconnecting) and how it is connected (local, remote,
//! reconnecting, disconnected, connection-lost, never-connected), and offers select / inspect /
//! continue-without-kernel actions so a kernel-free notebook stays explicitly editable,
//! searchable, and reviewable instead of forcing a setup-first blocker.
//!
//! Aureline's frozen notebook-kernel-output component matrix
//! ([`crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix`])
//! names the notebook document header and the kernel-state strip as two governed component
//! families and freezes their controlled vocabulary — the document source classes
//! (`local_ipynb`, `remote_ipynb`, `managed_workspace_ipynb`, `imported_ipynb`,
//! `scratch_untitled`, `unknown_source`) and document identity states (`saved_clean`,
//! `unsaved_changes`, `autosaved`, `conflicted`, `read_only`, `recovered`) a header binds; the
//! kernel execution states (`idle_ready`, `queued_pending`, `busy_running`, `interrupted`,
//! `dead_no_kernel`, `disconnected_reconnecting`) and kernel connection states
//! (`connected_local`, `connected_remote`, `reconnecting`, `disconnected`, `connection_lost`,
//! `never_connected`) a strip binds; the one controlled disposition vocabulary; the surface
//! families; the deployment lines; the consumer surfaces; the accessibility routes; the required
//! labels; and the downgrade triggers. This module *implements* that contract as two co-equal
//! component vectors so a claimed M5 notebook, kernel-manager, debug, review, or CLI surface can
//! project a document header and a kernel-state strip that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_document_header`] — takes a header's document source class and identity state and
//!    derives its origin class (local, remote, managed, imported, scratch, or unknown document),
//!    whether the header presents a canonical settled source of truth, and which notes the header
//!    must carry — so an imported, scratch, or unknown-source notebook can never read as a settled
//!    canonical source and a conflicted, read-only, or recovered document never hides its state.
//! 2. [`resolve_kernel_state`] — takes a strip's kernel execution state and connection state and
//!    derives its live class (ready, busy, queued, no-kernel-editable, disconnected-recoverable,
//!    or inspect-only), whether the kernel is actually live, whether kernel-free editing stays
//!    available, and which notes the strip must carry — so a kernel-free or disconnected notebook
//!    can never pretend to be live and a kernel-free notebook stays explicitly editable.
//!
//! A single controls packet — [`NotebookDocumentHeaderKernelStateStripControlsPacket`] — binds
//! one vector of document headers and one vector of kernel-state strips to the same source /
//! identity, kernel origin, execution / connection, deep-link, and non-visual accessibility
//! vocabulary, so notebook document truth and runtime kernel truth stay distinct and explicit
//! across notebook, kernel-manager, debug, review, headless / export, and support consumers.
//!
//! The component family ([`M5NotebookKernelOutputComponentFamily`]), document source class
//! ([`M5NotebookDocumentSourceClass`]), document identity state
//! ([`M5NotebookDocumentIdentityState`]), kernel execution state ([`M5KernelExecutionState`]),
//! kernel connection state ([`M5KernelConnectionState`]), disposition
//! ([`M5NotebookKernelOutputDisposition`]), surface family
//! ([`M5NotebookKernelOutputSurfaceFamily`]), deployment line
//! ([`M5NotebookKernelOutputDeploymentLine`]), consumer surface
//! ([`M5NotebookKernelOutputConsumerSurface`]), accessibility route
//! ([`M5NotebookKernelOutputAccessibilityRoute`]), required label
//! ([`M5NotebookKernelOutputRequiredLabel`]), and downgrade trigger
//! ([`M5NotebookKernelOutputDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: the derived origin and live classes, the bounded header and strip actions, and the
//! deep-link kinds. No M5 notebook surface invents a second document-header or kernel-strip
//! grammar.
//!
//! Raw notebook payloads, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every context line, deep-link reference, and component identity is carried
//! only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_notebook_document_header_kernel_state_strip_controls,
    seeded_notebook_document_header_kernel_state_strip_controls_document_header_scratch,
    seeded_notebook_document_header_kernel_state_strip_controls_kernel_state_strip_no_kernel,
    NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_PACKET_ID,
};

// The document source classes and identity states, the kernel execution and connection states,
// the disposition vocabulary, and the surface / deployment / consumer / accessibility / label /
// downgrade vocabularies are frozen once, in the notebook-kernel-output component matrix. This
// lane reuses them verbatim so it never invents a parallel document-header or kernel-strip
// vocabulary.
pub use crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix::{
    M5KernelConnectionState, M5KernelExecutionState, M5NotebookDocumentIdentityState,
    M5NotebookDocumentSourceClass, M5NotebookKernelOutputAccessibilityRoute,
    M5NotebookKernelOutputComponentFamily, M5NotebookKernelOutputConsumerSurface,
    M5NotebookKernelOutputDeploymentLine, M5NotebookKernelOutputDisposition,
    M5NotebookKernelOutputDowngradeTrigger, M5NotebookKernelOutputRequiredLabel,
    M5NotebookKernelOutputSurfaceFamily, M5_KERNEL_STATE_STRIP_SCHEMA_REF,
    M5_NOTEBOOK_DOCUMENT_HEADER_SCHEMA_REF, M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
    M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by
/// [`NotebookDocumentHeaderKernelStateStripControlsPacket`].
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_RECORD_KIND: &str =
    "implement_notebook_document_headers_and_kernel_state_strips_with_canonical_ipynb_source_selected_kernel_origin_busy_queued_offline_truth_and_no_kernel_edit_parity_across_claimed_m5_notebook_surfaces";

/// Schema version for M5 notebook-document-header / kernel-state-strip control records.
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_REF: &str =
    "schemas/ui/m5-notebook-document-header-kernel-state-strip-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_DOC_REF: &str =
    "docs/notebooks/m5_notebook_document_header_kernel_state_strip_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_FIXTURE_DIR: &str =
    "fixtures/ui/m5-notebook-document-header-kernel-state-strip-controls";

/// Repo-relative path of the checked support-export artifact.
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_ARTIFACT_REF: &str =
    "artifacts/release/m5-notebook-document-header-kernel-state-strip-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_CSV_REF: &str =
    "artifacts/release/m5-notebook-document-header-kernel-state-strip-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_REPORT_REF: &str =
    "artifacts/design/m5-notebook-document-header-kernel-state-strip.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a notebook component binds its next step against, so a document
/// header or kernel-state strip never routes through an ephemeral overlay — every next step is a
/// stable notebook location, kernel-manager, docs, or support-bundle reference the user can
/// reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable notebook / cell location.
    NotebookLocation,
    /// A stable kernel-manager reference.
    KernelManager,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable support-bundle anchor.
    SupportBundle,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotebookLocation,
        Self::KernelManager,
        Self::DocsAnchor,
        Self::SupportBundle,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookLocation => "notebook_location",
            Self::KernelManager => "kernel_manager",
            Self::DocsAnchor => "docs_anchor",
            Self::SupportBundle => "support_bundle",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- notebook-document-header vocabulary --------------------------------

/// Derived origin class a notebook document header may present.
///
/// This is the header honesty axis: the class is derived from the frozen document source class,
/// never asserted, so an imported, scratch, or unknown-source notebook can never present as a
/// settled canonical source and a user can always tell whether they are looking at a local,
/// remote, managed, imported, scratch, or unknown-source document before trusting it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentOriginClass {
    /// A canonical local `.ipynb`.
    LocalDocument,
    /// A canonical remote `.ipynb`.
    RemoteDocument,
    /// A canonical managed-workspace `.ipynb`.
    ManagedDocument,
    /// A notebook imported from elsewhere.
    ImportedDocument,
    /// An unsaved / untitled scratch notebook.
    ScratchDocument,
    /// A notebook whose source could not be resolved.
    UnknownDocument,
}

impl DocumentOriginClass {
    /// Every origin class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalDocument,
        Self::RemoteDocument,
        Self::ManagedDocument,
        Self::ImportedDocument,
        Self::ScratchDocument,
        Self::UnknownDocument,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDocument => "local_document",
            Self::RemoteDocument => "remote_document",
            Self::ManagedDocument => "managed_document",
            Self::ImportedDocument => "imported_document",
            Self::ScratchDocument => "scratch_document",
            Self::UnknownDocument => "unknown_document",
        }
    }

    /// True when the header presents a settled canonical source of truth (a real local, remote,
    /// or managed `.ipynb`), as opposed to an imported, scratch, or unknown-source notebook.
    pub const fn is_canonical_source(self) -> bool {
        matches!(
            self,
            Self::LocalDocument | Self::RemoteDocument | Self::ManagedDocument
        )
    }
}

/// One keyboard-complete default action a notebook document header offers, so a header never
/// hides its open / export / review affordance behind a pointer-only gesture. `OpenDocument`,
/// `ExportDocument`, and `ReviewDocument` are always offered so a notebook stays editable,
/// exportable, and reviewable — even before any kernel is selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentHeaderAction {
    /// Open / focus the canonical notebook document (always available).
    OpenDocument,
    /// Export the notebook / its paired export (always available).
    ExportDocument,
    /// Review / search the notebook without a kernel (always available).
    ReviewDocument,
    /// Copy the stable document path / identity.
    CopyDocumentPath,
    /// Open the stable notebook / kernel-manager / docs / support deep link.
    OpenDeepLink,
    /// Inspect the canonical source details.
    InspectSource,
}

impl DocumentHeaderAction {
    /// Every header action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDocument,
        Self::ExportDocument,
        Self::ReviewDocument,
        Self::CopyDocumentPath,
        Self::OpenDeepLink,
        Self::InspectSource,
    ];

    /// The default actions every keyboard-complete header must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenDocument,
        Self::ExportDocument,
        Self::ReviewDocument,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDocument => "open_document",
            Self::ExportDocument => "export_document",
            Self::ReviewDocument => "review_document",
            Self::CopyDocumentPath => "copy_document_path",
            Self::OpenDeepLink => "open_deep_link",
            Self::InspectSource => "inspect_source",
        }
    }
}

/// Disclosures a notebook document header must carry, derived from the source class and identity
/// state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentHeaderDisclosure {
    /// The derived origin class this header may present.
    pub origin_class: DocumentOriginClass,
    /// Whether the header presents a settled canonical source of truth.
    pub is_canonical_source: bool,
    /// Whether the header must carry an explicit imported-source note.
    pub needs_imported_note: bool,
    /// Whether the header must carry an explicit scratch / untitled note.
    pub needs_scratch_note: bool,
    /// Whether the header must carry an explicit unknown-source note.
    pub needs_unknown_source_note: bool,
    /// Whether the header must carry an explicit unsaved-changes note.
    pub needs_unsaved_note: bool,
    /// Whether the header must carry an explicit conflicted note.
    pub needs_conflict_note: bool,
    /// Whether the header must carry an explicit read-only note.
    pub needs_readonly_note: bool,
    /// Whether the header must carry an explicit recovered note.
    pub needs_recovered_note: bool,
}

/// Resolves the source and identity truth a notebook document header may present.
///
/// A `local_ipynb` / `remote_ipynb` / `managed_workspace_ipynb` source is a canonical document.
/// An `imported_ipynb` is imported, a `scratch_untitled` is scratch, and an `unknown_source` is
/// unknown, so a notebook Aureline did not settle to a canonical `.ipynb` can never read as a
/// settled source of truth. A `conflicted`, `read_only`, `recovered`, or `unsaved_changes`
/// identity always carries its own note so document state never hides behind a clean-looking
/// header.
pub fn resolve_document_header(
    source: M5NotebookDocumentSourceClass,
    identity: M5NotebookDocumentIdentityState,
) -> DocumentHeaderDisclosure {
    use DocumentOriginClass as Class;
    use M5NotebookDocumentIdentityState as Id;
    use M5NotebookDocumentSourceClass as Src;

    let origin_class = match source {
        Src::LocalIpynb => Class::LocalDocument,
        Src::RemoteIpynb => Class::RemoteDocument,
        Src::ManagedWorkspaceIpynb => Class::ManagedDocument,
        Src::ImportedIpynb => Class::ImportedDocument,
        Src::ScratchUntitled => Class::ScratchDocument,
        Src::UnknownSource => Class::UnknownDocument,
    };

    DocumentHeaderDisclosure {
        origin_class,
        is_canonical_source: origin_class.is_canonical_source(),
        needs_imported_note: matches!(origin_class, Class::ImportedDocument),
        needs_scratch_note: matches!(origin_class, Class::ScratchDocument),
        needs_unknown_source_note: matches!(origin_class, Class::UnknownDocument),
        needs_unsaved_note: matches!(identity, Id::UnsavedChanges),
        needs_conflict_note: matches!(identity, Id::Conflicted),
        needs_readonly_note: matches!(identity, Id::ReadOnly),
        needs_recovered_note: matches!(identity, Id::Recovered),
    }
}

/// A notebook document header naming its canonical `.ipynb` identity, its source class and
/// derived origin class, its identity state, its paired export state, its current target /
/// workspace context, bounded open / export / review actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDocumentHeader {
    /// Frozen component this control implements; must be `notebook_document_header`.
    pub component: M5NotebookKernelOutputComponentFamily,
    /// Stable header id.
    pub header_id: String,
    /// Human-readable header label; required and non-empty.
    pub header_label: String,
    /// Document source class, reused from the frozen matrix.
    pub source_class: M5NotebookDocumentSourceClass,
    /// Document identity state, reused from the frozen matrix.
    pub identity_state: M5NotebookDocumentIdentityState,
    /// Derived origin class (must equal the resolved class).
    pub origin_class: DocumentOriginClass,
    /// Whether the header claims a settled canonical source (must equal the derived truth).
    pub claims_canonical_source: bool,
    /// Imported-source note; required when the notebook is imported.
    pub imported_note: String,
    /// Scratch / untitled note; required when the notebook is a scratch notebook.
    pub scratch_note: String,
    /// Unknown-source note; required when the notebook's source is unknown.
    pub unknown_source_note: String,
    /// Unsaved-changes note; required when the notebook has unsaved changes.
    pub unsaved_note: String,
    /// Conflicted note; required when the notebook is conflicted.
    pub conflict_note: String,
    /// Read-only note; required when the notebook is read-only.
    pub readonly_note: String,
    /// Recovered note; required when the notebook was recovered.
    pub recovered_note: String,
    /// Canonical `.ipynb` identity label; always required so document identity stays explicit.
    pub notebook_identity_label: String,
    /// Paired export state label; always required (names the paired export or that none exists).
    pub export_state_label: String,
    /// Current target / workspace context label; always required.
    pub target_context_label: String,
    /// Source-of-truth note; always required so the header names its canonical source cue.
    pub source_of_truth_note: String,
    /// Context note; always required so the header names what the document truth means here.
    pub context_note: String,
    /// Kind of stable deep link this header binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include open / export / review).
    pub header_actions: Vec<DocumentHeaderAction>,
    /// Dispositions this header binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Downgrade triggers this header can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Mandatory labels this header can show (must include the mandatory labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Claimed M5 surface families that render this header.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this header keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Non-visual accessibility routes this header offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Notebook subsystems that consume this header's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this header.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never presents a kernel-free notebook as live. MUST be `false`.
    pub pretends_kernel_free_is_live: bool,
    /// Hard invariant: never collapses local / remote / managed kernels into one badge. MUST be
    /// `false`.
    pub collapses_kernel_origins_into_one_badge: bool,
    /// Hard invariant: never conflates document truth with runtime truth. MUST be `false`.
    pub conflates_document_and_runtime_truth: bool,
    /// Hard invariant: never hides a governed state behind a hover-only affordance. MUST be
    /// `false`.
    pub hides_state_behind_hover_only: bool,
}

impl NotebookDocumentHeader {
    /// Source / identity disclosures this header must carry, derived from the frozen states.
    pub fn document_disclosure(&self) -> DocumentHeaderDisclosure {
        resolve_document_header(self.source_class, self.identity_state)
    }

    /// Whether the header offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<DocumentHeaderAction> = self.header_actions.iter().copied().collect();
        DocumentHeaderAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the header declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the header offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.header_actions
            .contains(&DocumentHeaderAction::OpenDeepLink)
    }
}

// ---- kernel-state-strip vocabulary --------------------------------------

/// Derived live class a kernel-state strip may present.
///
/// This is the strip honesty axis: the class is derived from the frozen kernel execution and
/// connection states, never asserted, so a kernel-free, disconnected, or interrupted notebook can
/// never present as live and a user can always tell whether the kernel is ready, busy, queued,
/// kernel-free, disconnected-recoverable, or inspect-only before trusting any output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelLiveClass {
    /// The kernel is attached and ready to run.
    ReadyLive,
    /// The kernel is attached and busy running.
    BusyLive,
    /// The kernel is attached with queued / pending work.
    QueuedLive,
    /// No kernel is attached; the notebook stays editable and searchable.
    NoKernelEditable,
    /// The kernel is disconnected but recoverable via reconnect.
    DisconnectedRecoverable,
    /// The kernel is interrupted / inspect-only.
    InspectOnly,
}

impl KernelLiveClass {
    /// Every live class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadyLive,
        Self::BusyLive,
        Self::QueuedLive,
        Self::NoKernelEditable,
        Self::DisconnectedRecoverable,
        Self::InspectOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyLive => "ready_live",
            Self::BusyLive => "busy_live",
            Self::QueuedLive => "queued_live",
            Self::NoKernelEditable => "no_kernel_editable",
            Self::DisconnectedRecoverable => "disconnected_recoverable",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// True when a kernel is actually attached and running / ready / queued (a live kernel).
    pub const fn is_live(self) -> bool {
        matches!(self, Self::ReadyLive | Self::BusyLive | Self::QueuedLive)
    }

    /// True when the notebook is explicitly kernel-free but still editable and searchable.
    pub const fn preserves_no_kernel_editing(self) -> bool {
        matches!(self, Self::NoKernelEditable)
    }
}

/// One keyboard-complete default action a kernel-state strip offers, so a strip never hides its
/// select / inspect / continue-without-kernel affordance behind a pointer-only gesture.
/// `SelectKernel`, `InspectKernel`, and `ContinueWithoutKernel` are always offered so a
/// kernel-free notebook stays editable and a kernel is always selectable without a setup-first
/// blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelStripAction {
    /// Select / attach a kernel (always available).
    SelectKernel,
    /// Inspect the current kernel state (always available).
    InspectKernel,
    /// Continue editing / searching without a kernel (always available).
    ContinueWithoutKernel,
    /// Reconnect a disconnected kernel.
    ReconnectKernel,
    /// Restart the kernel.
    RestartKernel,
    /// Open the stable notebook / kernel-manager / docs / support deep link.
    OpenDeepLink,
}

impl KernelStripAction {
    /// Every strip action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SelectKernel,
        Self::InspectKernel,
        Self::ContinueWithoutKernel,
        Self::ReconnectKernel,
        Self::RestartKernel,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete strip must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::SelectKernel,
        Self::InspectKernel,
        Self::ContinueWithoutKernel,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectKernel => "select_kernel",
            Self::InspectKernel => "inspect_kernel",
            Self::ContinueWithoutKernel => "continue_without_kernel",
            Self::ReconnectKernel => "reconnect_kernel",
            Self::RestartKernel => "restart_kernel",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a kernel-state strip must carry, derived from the execution and connection states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelStripDisclosure {
    /// The derived live class this strip may present.
    pub live_class: KernelLiveClass,
    /// Whether the kernel is actually live.
    pub is_live: bool,
    /// Whether kernel-free editing stays available.
    pub preserves_no_kernel_editing: bool,
    /// Whether the strip must carry an explicit no-kernel note.
    pub needs_no_kernel_note: bool,
    /// Whether the strip must carry an explicit reconnect note.
    pub needs_reconnect_note: bool,
    /// Whether the strip must carry an explicit inspect-only note.
    pub needs_inspect_only_note: bool,
}

/// Resolves the live truth a kernel-state strip may present.
///
/// A `dead_no_kernel` execution (or an `idle_ready` execution that was `never_connected`) is
/// kernel-free but editable. A `queued_pending` execution is queued-live, a `busy_running`
/// execution is busy-live, and an `idle_ready` execution over a live connection is ready-live. An
/// `interrupted` execution is inspect-only. A `disconnected_reconnecting` execution — or an
/// `idle_ready` execution whose connection is `reconnecting`, `disconnected`, or `connection_lost`
/// — is disconnected-recoverable, so a notebook without a live kernel can never read as live.
pub fn resolve_kernel_state(
    execution: M5KernelExecutionState,
    connection: M5KernelConnectionState,
) -> KernelStripDisclosure {
    use KernelLiveClass as Class;
    use M5KernelConnectionState as Conn;
    use M5KernelExecutionState as Exec;

    let live_class = match execution {
        Exec::DeadNoKernel => Class::NoKernelEditable,
        Exec::QueuedPending => Class::QueuedLive,
        Exec::BusyRunning => Class::BusyLive,
        Exec::Interrupted => Class::InspectOnly,
        Exec::DisconnectedReconnecting => Class::DisconnectedRecoverable,
        Exec::IdleReady => match connection {
            Conn::ConnectedLocal | Conn::ConnectedRemote => Class::ReadyLive,
            Conn::Reconnecting | Conn::Disconnected | Conn::ConnectionLost => {
                Class::DisconnectedRecoverable
            }
            Conn::NeverConnected => Class::NoKernelEditable,
        },
    };

    KernelStripDisclosure {
        live_class,
        is_live: live_class.is_live(),
        preserves_no_kernel_editing: live_class.preserves_no_kernel_editing(),
        needs_no_kernel_note: matches!(live_class, Class::NoKernelEditable),
        needs_reconnect_note: matches!(live_class, Class::DisconnectedRecoverable),
        needs_inspect_only_note: matches!(live_class, Class::InspectOnly),
    }
}

/// A kernel-state strip naming its selected kernel origin / class, its execution and connection
/// states, its derived live class, its execution context, its kernel-free edit parity, bounded
/// select / inspect / continue actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelStateStrip {
    /// Frozen component this control implements; must be `kernel_state_strip`.
    pub component: M5NotebookKernelOutputComponentFamily,
    /// Stable strip id.
    pub strip_id: String,
    /// Human-readable strip label; required and non-empty.
    pub strip_label: String,
    /// Kernel execution state, reused from the frozen matrix.
    pub execution_state: M5KernelExecutionState,
    /// Kernel connection state, reused from the frozen matrix.
    pub connection_state: M5KernelConnectionState,
    /// Derived live class (must equal the resolved class).
    pub live_class: KernelLiveClass,
    /// Whether the strip claims the kernel is live (must equal the derived truth).
    pub claims_live: bool,
    /// No-kernel note; required when the notebook is kernel-free.
    pub no_kernel_note: String,
    /// Reconnect note; required when the kernel is disconnected-recoverable.
    pub reconnect_note: String,
    /// Inspect-only note; required when the kernel is inspect-only.
    pub inspect_only_note: String,
    /// Selected kernel origin / class label; always required so the kernel origin stays explicit
    /// and local / remote / container / managed kernels never collapse into one badge.
    pub kernel_origin_label: String,
    /// Kernel state summary label; always required so the execution / connection state stays
    /// explicit.
    pub kernel_state_summary: String,
    /// Execution context label; always required so where the kernel executes stays explicit.
    pub execution_context_label: String,
    /// Kernel-free edit parity note; always required so a no-kernel notebook stays explicitly
    /// editable / searchable / reviewable.
    pub kernel_free_edit_note: String,
    /// Context note; always required so the strip names what the runtime truth means here.
    pub context_note: String,
    /// Kind of stable deep link this strip binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include select / inspect / continue).
    pub strip_actions: Vec<KernelStripAction>,
    /// Dispositions this strip binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Downgrade triggers this strip can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Mandatory labels this strip can show (must include the mandatory labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Claimed M5 surface families that render this strip.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this strip keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Non-visual accessibility routes this strip offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Notebook subsystems that consume this strip's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this strip.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never presents a kernel-free notebook as live. MUST be `false`.
    pub pretends_kernel_free_is_live: bool,
    /// Hard invariant: never collapses local / remote / managed kernels into one badge. MUST be
    /// `false`.
    pub collapses_kernel_origins_into_one_badge: bool,
    /// Hard invariant: never conflates document truth with runtime truth. MUST be `false`.
    pub conflates_document_and_runtime_truth: bool,
    /// Hard invariant: never hides a governed state behind a hover-only affordance. MUST be
    /// `false`.
    pub hides_state_behind_hover_only: bool,
}

impl KernelStateStrip {
    /// Live disclosures this strip must carry, derived from the execution and connection states.
    pub fn kernel_disclosure(&self) -> KernelStripDisclosure {
        resolve_kernel_state(self.execution_state, self.connection_state)
    }

    /// Whether the strip offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<KernelStripAction> = self.strip_actions.iter().copied().collect();
        KernelStripAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the strip declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the strip offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.strip_actions
            .contains(&KernelStripAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance notebook-component review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDocumentKernelReview {
    /// The document header names its canonical source.
    pub header_shows_canonical_source: bool,
    /// The document header names its document identity state.
    pub header_shows_document_identity: bool,
    /// The document header offers open, export, and review.
    pub header_offers_open_export_review: bool,
    /// The kernel-state strip names its kernel execution state.
    pub strip_shows_kernel_execution_state: bool,
    /// The kernel-state strip names its kernel connection state.
    pub strip_shows_kernel_connection_state: bool,
    /// The kernel-state strip offers select, inspect, and continue-without-kernel.
    pub strip_offers_select_inspect_continue: bool,
    /// Document truth and runtime truth stay distinct at a glance.
    pub document_and_runtime_truth_distinct: bool,
    /// A kernel-free notebook stays explicitly editable / searchable / reviewable.
    pub kernel_free_notebook_stays_editable: bool,
    /// A kernel-free or disconnected notebook is never shown as live.
    pub kernel_free_never_shown_as_live: bool,
    /// Local / remote / container / managed kernels never collapse into one unlabeled badge.
    pub no_kernel_origins_collapsed_into_one_badge: bool,
    /// Source and kernel state are derived from state, never asserted.
    pub source_and_kernel_state_derived_never_asserted: bool,
    /// Every next step names one stable notebook / kernel-manager / docs / support deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// No governed state is hidden behind a hover-only affordance.
    pub no_state_hidden_behind_hover_only: bool,
    /// Headers and strips stay consistent across edit, diff, debug, and support surfaces.
    pub header_and_strip_consistent_across_surfaces: bool,
    /// No component widens export scope or exposes raw payloads by default.
    pub no_component_widens_export_scope_or_exposes_raw_by_default: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl NotebookDocumentKernelReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.header_shows_canonical_source
            && self.header_shows_document_identity
            && self.header_offers_open_export_review
            && self.strip_shows_kernel_execution_state
            && self.strip_shows_kernel_connection_state
            && self.strip_offers_select_inspect_continue
            && self.document_and_runtime_truth_distinct
            && self.kernel_free_notebook_stays_editable
            && self.kernel_free_never_shown_as_live
            && self.no_kernel_origins_collapsed_into_one_badge
            && self.source_and_kernel_state_derived_never_asserted
            && self.every_next_step_names_stable_deep_link
            && self.no_state_hidden_behind_hover_only
            && self.header_and_strip_consistent_across_surfaces
            && self.no_component_widens_export_scope_or_exposes_raw_by_default
            && self.components_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.no_surface_invents_alternate_state_label
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDocumentKernelConsumerProjection {
    /// The notebook edit surface reads a single canonical source.
    pub notebook_edit_surface_reads_single_source: bool,
    /// The kernel-manager surface reads a single canonical source.
    pub kernel_manager_surface_reads_single_source: bool,
    /// Document truth is visible before a run.
    pub document_truth_visible_before_run: bool,
    /// Kernel state is visible before trusting an output.
    pub kernel_state_visible_before_trusting_output: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
    /// Help / docs shows component truth.
    pub help_docs_shows_component_truth: bool,
}

impl NotebookDocumentKernelConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.notebook_edit_surface_reads_single_source
            && self.kernel_manager_surface_reads_single_source
            && self.document_truth_visible_before_run
            && self.kernel_state_visible_before_trusting_output
            && self.support_export_shows_component_truth
            && self.help_docs_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDocumentKernelProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for
/// [`NotebookDocumentHeaderKernelStateStripControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotebookDocumentHeaderKernelStateStripControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Notebook document headers.
    pub document_headers: Vec<NotebookDocumentHeader>,
    /// Kernel-state strips.
    pub kernel_strips: Vec<KernelStateStrip>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Notebook review block.
    pub notebook_review: NotebookDocumentKernelReview,
    /// Consumer projection block.
    pub consumer_projection: NotebookDocumentKernelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: NotebookDocumentKernelProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe notebook-document-header / kernel-state-strip controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDocumentHeaderKernelStateStripControlsPacket {
    /// Record kind; must equal [`NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Notebook document headers.
    pub document_headers: Vec<NotebookDocumentHeader>,
    /// Kernel-state strips.
    pub kernel_strips: Vec<KernelStateStrip>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Notebook review block.
    pub notebook_review: NotebookDocumentKernelReview,
    /// Consumer projection block.
    pub consumer_projection: NotebookDocumentKernelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: NotebookDocumentKernelProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl NotebookDocumentHeaderKernelStateStripControlsPacket {
    /// Builds a notebook-document-header / kernel-state-strip controls packet from stable-lane
    /// input.
    pub fn new(input: NotebookDocumentHeaderKernelStateStripControlsPacketInput) -> Self {
        Self {
            record_kind: NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            document_headers: input.document_headers,
            kernel_strips: input.kernel_strips,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            notebook_review: input.notebook_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the notebook-document-header / kernel-state-strip control invariants.
    pub fn validate(&self) -> Vec<NotebookDocumentHeaderKernelStateStripViolation> {
        let mut violations = Vec::new();

        if self.record_kind != NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_RECORD_KIND {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::WrongRecordKind);
        }
        if self.schema_version != NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_VERSION {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_document_headers(self, &mut violations);
        validate_kernel_strips(self, &mut violations);

        if !self.notebook_review.all_hold() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::NotebookReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(
                NotebookDocumentHeaderKernelStateStripViolation::ConsumerProjectionIncomplete,
            );
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("notebook document header kernel strip packet serializes"),
        ) {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::RawMaterialInExport);
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
            .expect("notebook document header kernel strip packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("component,id,state_or_source,identity_or_connection,derived,live_or_canonical,deep_link_kind\n");
        for header in &self.document_headers {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "notebook_document_header",
                csv_field(&header.header_id),
                header.source_class.as_str(),
                header.identity_state.as_str(),
                header.document_disclosure().origin_class.as_str(),
                header.document_disclosure().is_canonical_source,
                header.deep_link_kind.as_str(),
            ));
        }
        for strip in &self.kernel_strips {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "kernel_state_strip",
                csv_field(&strip.strip_id),
                strip.execution_state.as_str(),
                strip.connection_state.as_str(),
                strip.kernel_disclosure().live_class.as_str(),
                strip.kernel_disclosure().is_live,
                strip.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let non_canonical = self
            .document_headers
            .iter()
            .filter(|header| !header.document_disclosure().is_canonical_source)
            .count();
        let not_live = self
            .kernel_strips
            .iter()
            .filter(|strip| !strip.kernel_disclosure().is_live)
            .count();

        let mut out = String::new();
        out.push_str("# Notebook document headers and kernel-state strips\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Notebook document headers: {} ({} not a settled canonical source)\n",
            self.document_headers.len(),
            non_canonical
        ));
        out.push_str(&format!(
            "- Kernel-state strips: {} ({} not a live kernel)\n",
            self.kernel_strips.len(),
            not_live
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Notebook document headers\n\n");
        for header in &self.document_headers {
            out.push_str(&format!(
                "- **{}** — source `{}`, identity `{}` → `{}`, export `{}`, deep link `{}`\n",
                header.header_label,
                header.source_class.as_str(),
                header.identity_state.as_str(),
                header.document_disclosure().origin_class.as_str(),
                header.export_state_label,
                header.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Kernel-state strips\n\n");
        for strip in &self.kernel_strips {
            out.push_str(&format!(
                "- **{}** — execution `{}`, connection `{}` → `{}`, deep link `{}`\n",
                strip.strip_label,
                strip.execution_state.as_str(),
                strip.connection_state.as_str(),
                strip.kernel_disclosure().live_class.as_str(),
                strip.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in notebook-document-header / kernel-state-strip
/// export.
#[derive(Debug)]
pub enum NotebookDocumentHeaderKernelStateStripArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<NotebookDocumentHeaderKernelStateStripViolation>),
}

impl fmt::Display for NotebookDocumentHeaderKernelStateStripArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "notebook document header kernel strip export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "notebook document header kernel strip export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for NotebookDocumentHeaderKernelStateStripArtifactError {}

/// Validation failures emitted by
/// [`NotebookDocumentHeaderKernelStateStripControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotebookDocumentHeaderKernelStateStripViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No notebook document headers are present.
    HeadersMissing,
    /// A notebook document header is incomplete.
    HeaderIncomplete,
    /// A notebook document header carries the wrong frozen component class.
    HeaderWrongComponentClass,
    /// A header misrepresents its derived origin class.
    DocumentSourceMisrepresented,
    /// An imported notebook does not name its imported source.
    ImportedNoteMissing,
    /// A scratch notebook does not name its scratch state.
    ScratchNoteMissing,
    /// An unknown-source notebook does not name its unknown source.
    UnknownSourceNoteMissing,
    /// An unsaved notebook does not name its unsaved changes.
    UnsavedNoteMissing,
    /// A conflicted notebook does not name its conflict.
    ConflictNoteMissing,
    /// A read-only notebook does not name its read-only state.
    ReadOnlyNoteMissing,
    /// A recovered notebook does not name its recovered state.
    RecoveredNoteMissing,
    /// A header does not name its canonical `.ipynb` identity.
    NotebookIdentityMissing,
    /// A header does not name its paired export state.
    ExportStateMissing,
    /// A header does not name its target / workspace context.
    TargetContextMissing,
    /// A header does not name its source-of-truth cue.
    SourceOfTruthNoteMissing,
    /// A header omits a mandatory open / export / review action.
    HeaderActionsIncomplete,
    /// The headers do not cover every document source class.
    DocumentSourceCoverageMissing,
    /// The headers do not cover every document identity state.
    DocumentIdentityCoverageMissing,
    /// The headers do not cover every derived origin class.
    DocumentOriginClassCoverageMissing,
    /// No kernel-state strips are present.
    StripsMissing,
    /// A kernel-state strip is incomplete.
    StripIncomplete,
    /// A kernel-state strip carries the wrong frozen component class.
    StripWrongComponentClass,
    /// A strip misrepresents its derived live class.
    KernelStateMisrepresented,
    /// A kernel-free strip does not name its no-kernel state.
    NoKernelNoteMissing,
    /// A disconnected-recoverable strip does not name its reconnect state.
    ReconnectNoteMissing,
    /// An inspect-only strip does not name its inspect-only state.
    InspectOnlyNoteMissing,
    /// A strip does not name its selected kernel origin / class.
    KernelOriginLabelMissing,
    /// A strip does not name its kernel state summary.
    KernelStateSummaryMissing,
    /// A strip does not name its execution context.
    ExecutionContextMissing,
    /// A strip does not name its kernel-free edit parity.
    KernelFreeEditNoteMissing,
    /// A strip omits a mandatory select / inspect / continue action.
    StripActionsIncomplete,
    /// The strips do not cover every kernel execution state.
    KernelExecutionStateCoverageMissing,
    /// The strips do not cover every kernel connection state.
    KernelConnectionStateCoverageMissing,
    /// The strips do not cover every derived live class.
    KernelLiveClassCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component presents a kernel-free notebook as live.
    KernelFreePresentedAsLive,
    /// A component collapses local / remote / managed kernels into one badge.
    KernelOriginsCollapsed,
    /// A component conflates document truth with runtime truth.
    DocumentAndRuntimeConflated,
    /// A component hides a governed state behind a hover-only affordance.
    StateHiddenBehindHoverOnly,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Notebook review does not satisfy required invariants.
    NotebookReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl NotebookDocumentHeaderKernelStateStripViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::HeadersMissing => "headers_missing",
            Self::HeaderIncomplete => "header_incomplete",
            Self::HeaderWrongComponentClass => "header_wrong_component_class",
            Self::DocumentSourceMisrepresented => "document_source_misrepresented",
            Self::ImportedNoteMissing => "imported_note_missing",
            Self::ScratchNoteMissing => "scratch_note_missing",
            Self::UnknownSourceNoteMissing => "unknown_source_note_missing",
            Self::UnsavedNoteMissing => "unsaved_note_missing",
            Self::ConflictNoteMissing => "conflict_note_missing",
            Self::ReadOnlyNoteMissing => "read_only_note_missing",
            Self::RecoveredNoteMissing => "recovered_note_missing",
            Self::NotebookIdentityMissing => "notebook_identity_missing",
            Self::ExportStateMissing => "export_state_missing",
            Self::TargetContextMissing => "target_context_missing",
            Self::SourceOfTruthNoteMissing => "source_of_truth_note_missing",
            Self::HeaderActionsIncomplete => "header_actions_incomplete",
            Self::DocumentSourceCoverageMissing => "document_source_coverage_missing",
            Self::DocumentIdentityCoverageMissing => "document_identity_coverage_missing",
            Self::DocumentOriginClassCoverageMissing => "document_origin_class_coverage_missing",
            Self::StripsMissing => "strips_missing",
            Self::StripIncomplete => "strip_incomplete",
            Self::StripWrongComponentClass => "strip_wrong_component_class",
            Self::KernelStateMisrepresented => "kernel_state_misrepresented",
            Self::NoKernelNoteMissing => "no_kernel_note_missing",
            Self::ReconnectNoteMissing => "reconnect_note_missing",
            Self::InspectOnlyNoteMissing => "inspect_only_note_missing",
            Self::KernelOriginLabelMissing => "kernel_origin_label_missing",
            Self::KernelStateSummaryMissing => "kernel_state_summary_missing",
            Self::ExecutionContextMissing => "execution_context_missing",
            Self::KernelFreeEditNoteMissing => "kernel_free_edit_note_missing",
            Self::StripActionsIncomplete => "strip_actions_incomplete",
            Self::KernelExecutionStateCoverageMissing => "kernel_execution_state_coverage_missing",
            Self::KernelConnectionStateCoverageMissing => {
                "kernel_connection_state_coverage_missing"
            }
            Self::KernelLiveClassCoverageMissing => "kernel_live_class_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::KernelFreePresentedAsLive => "kernel_free_presented_as_live",
            Self::KernelOriginsCollapsed => "kernel_origins_collapsed",
            Self::DocumentAndRuntimeConflated => "document_and_runtime_conflated",
            Self::StateHiddenBehindHoverOnly => "state_hidden_behind_hover_only",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::NotebookReviewIncomplete => "notebook_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable notebook-document-header / kernel-state-strip
/// export.
pub fn current_notebook_document_header_kernel_state_strip_export() -> Result<
    NotebookDocumentHeaderKernelStateStripControlsPacket,
    NotebookDocumentHeaderKernelStateStripArtifactError,
> {
    let packet: NotebookDocumentHeaderKernelStateStripControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-notebook-document-header-kernel-state-strip-proof/support_export.json"
        )))
        .map_err(NotebookDocumentHeaderKernelStateStripArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NotebookDocumentHeaderKernelStateStripArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &NotebookDocumentHeaderKernelStateStripControlsPacket,
    violations: &mut Vec<NotebookDocumentHeaderKernelStateStripViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_REF,
        NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_DOC_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_NOTEBOOK_DOCUMENT_HEADER_SCHEMA_REF,
        M5_KERNEL_STATE_STRIP_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_document_headers(
    packet: &NotebookDocumentHeaderKernelStateStripControlsPacket,
    violations: &mut Vec<NotebookDocumentHeaderKernelStateStripViolation>,
) {
    if packet.document_headers.is_empty() {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::HeadersMissing);
        return;
    }

    let mut origin_classes: BTreeSet<DocumentOriginClass> = BTreeSet::new();
    let mut sources: BTreeSet<M5NotebookDocumentSourceClass> = BTreeSet::new();
    let mut identities: BTreeSet<M5NotebookDocumentIdentityState> = BTreeSet::new();

    for header in &packet.document_headers {
        let disclosure = header.document_disclosure();
        origin_classes.insert(disclosure.origin_class);
        sources.insert(header.source_class);
        identities.insert(header.identity_state);

        if header.header_id.trim().is_empty()
            || header.header_label.trim().is_empty()
            || header.fields_shown.is_empty()
            || header.surface_families.is_empty()
            || header.deployment_lines.is_empty()
            || header.consumer_surfaces.is_empty()
            || header.source_contract_refs.is_empty()
        {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::HeaderIncomplete);
        }
        if header.component != M5NotebookKernelOutputComponentFamily::NotebookDocumentHeader {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::HeaderWrongComponentClass);
        }
        if header.origin_class != disclosure.origin_class
            || header.claims_canonical_source != disclosure.is_canonical_source
        {
            violations.push(
                NotebookDocumentHeaderKernelStateStripViolation::DocumentSourceMisrepresented,
            );
        }
        if disclosure.needs_imported_note && header.imported_note.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::ImportedNoteMissing);
        }
        if disclosure.needs_scratch_note && header.scratch_note.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::ScratchNoteMissing);
        }
        if disclosure.needs_unknown_source_note && header.unknown_source_note.trim().is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::UnknownSourceNoteMissing);
        }
        if disclosure.needs_unsaved_note && header.unsaved_note.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::UnsavedNoteMissing);
        }
        if disclosure.needs_conflict_note && header.conflict_note.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::ConflictNoteMissing);
        }
        if disclosure.needs_readonly_note && header.readonly_note.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::ReadOnlyNoteMissing);
        }
        if disclosure.needs_recovered_note && header.recovered_note.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::RecoveredNoteMissing);
        }
        if header.notebook_identity_label.trim().is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::NotebookIdentityMissing);
        }
        if header.export_state_label.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::ExportStateMissing);
        }
        if header.target_context_label.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::TargetContextMissing);
        }
        if header.source_of_truth_note.trim().is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::SourceOfTruthNoteMissing);
        }
        if !header.declares_mandatory_actions() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::HeaderActionsIncomplete);
        }
        validate_deep_link(
            header.offers_deep_link_action(),
            header.deep_link_kind,
            &header.deep_link_ref,
            &header.context_note,
            violations,
        );
        validate_common_control(
            &header.dispositions,
            &header.downgrade_triggers,
            header.declares_mandatory_labels(),
            &header.accessibility_routes,
            ControlInvariants {
                pretends_kernel_free_is_live: header.pretends_kernel_free_is_live,
                collapses_kernel_origins_into_one_badge: header
                    .collapses_kernel_origins_into_one_badge,
                conflates_document_and_runtime_truth: header.conflates_document_and_runtime_truth,
                hides_state_behind_hover_only: header.hides_state_behind_hover_only,
            },
            violations,
        );
    }

    for required in DocumentOriginClass::ALL {
        if !origin_classes.contains(&required) {
            violations.push(
                NotebookDocumentHeaderKernelStateStripViolation::DocumentOriginClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5NotebookDocumentSourceClass::ALL {
        if !sources.contains(&required) {
            violations.push(
                NotebookDocumentHeaderKernelStateStripViolation::DocumentSourceCoverageMissing,
            );
            break;
        }
    }
    for required in M5NotebookDocumentIdentityState::ALL {
        if !identities.contains(&required) {
            violations.push(
                NotebookDocumentHeaderKernelStateStripViolation::DocumentIdentityCoverageMissing,
            );
            break;
        }
    }
}

fn validate_kernel_strips(
    packet: &NotebookDocumentHeaderKernelStateStripControlsPacket,
    violations: &mut Vec<NotebookDocumentHeaderKernelStateStripViolation>,
) {
    if packet.kernel_strips.is_empty() {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::StripsMissing);
        return;
    }

    let mut live_classes: BTreeSet<KernelLiveClass> = BTreeSet::new();
    let mut executions: BTreeSet<M5KernelExecutionState> = BTreeSet::new();
    let mut connections: BTreeSet<M5KernelConnectionState> = BTreeSet::new();

    for strip in &packet.kernel_strips {
        let disclosure = strip.kernel_disclosure();
        live_classes.insert(disclosure.live_class);
        executions.insert(strip.execution_state);
        connections.insert(strip.connection_state);

        if strip.strip_id.trim().is_empty()
            || strip.strip_label.trim().is_empty()
            || strip.fields_shown.is_empty()
            || strip.surface_families.is_empty()
            || strip.deployment_lines.is_empty()
            || strip.consumer_surfaces.is_empty()
            || strip.source_contract_refs.is_empty()
        {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::StripIncomplete);
        }
        if strip.component != M5NotebookKernelOutputComponentFamily::KernelStateStrip {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::StripWrongComponentClass);
        }
        if strip.live_class != disclosure.live_class || strip.claims_live != disclosure.is_live {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::KernelStateMisrepresented);
        }
        if disclosure.needs_no_kernel_note && strip.no_kernel_note.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::NoKernelNoteMissing);
        }
        if disclosure.needs_reconnect_note && strip.reconnect_note.trim().is_empty() {
            violations.push(NotebookDocumentHeaderKernelStateStripViolation::ReconnectNoteMissing);
        }
        if disclosure.needs_inspect_only_note && strip.inspect_only_note.trim().is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::InspectOnlyNoteMissing);
        }
        if strip.kernel_origin_label.trim().is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::KernelOriginLabelMissing);
        }
        if strip.kernel_state_summary.trim().is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::KernelStateSummaryMissing);
        }
        if strip.execution_context_label.trim().is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::ExecutionContextMissing);
        }
        if strip.kernel_free_edit_note.trim().is_empty() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::KernelFreeEditNoteMissing);
        }
        if !strip.declares_mandatory_actions() {
            violations
                .push(NotebookDocumentHeaderKernelStateStripViolation::StripActionsIncomplete);
        }
        validate_deep_link(
            strip.offers_deep_link_action(),
            strip.deep_link_kind,
            &strip.deep_link_ref,
            &strip.context_note,
            violations,
        );
        validate_common_control(
            &strip.dispositions,
            &strip.downgrade_triggers,
            strip.declares_mandatory_labels(),
            &strip.accessibility_routes,
            ControlInvariants {
                pretends_kernel_free_is_live: strip.pretends_kernel_free_is_live,
                collapses_kernel_origins_into_one_badge: strip
                    .collapses_kernel_origins_into_one_badge,
                conflates_document_and_runtime_truth: strip.conflates_document_and_runtime_truth,
                hides_state_behind_hover_only: strip.hides_state_behind_hover_only,
            },
            violations,
        );
    }

    for required in KernelLiveClass::ALL {
        if !live_classes.contains(&required) {
            violations.push(
                NotebookDocumentHeaderKernelStateStripViolation::KernelLiveClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5KernelExecutionState::ALL {
        if !executions.contains(&required) {
            violations.push(
                NotebookDocumentHeaderKernelStateStripViolation::KernelExecutionStateCoverageMissing,
            );
            break;
        }
    }
    for required in M5KernelConnectionState::ALL {
        if !connections.contains(&required) {
            violations.push(
                NotebookDocumentHeaderKernelStateStripViolation::KernelConnectionStateCoverageMissing,
            );
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a component
/// that names a resolvable kind must carry its stable reference, and every component must name its
/// context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<NotebookDocumentHeaderKernelStateStripViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::DeepLinkRefMissing);
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    pretends_kernel_free_is_live: bool,
    collapses_kernel_origins_into_one_badge: bool,
    conflates_document_and_runtime_truth: bool,
    hides_state_behind_hover_only: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5NotebookKernelOutputDisposition],
    downgrade_triggers: &[M5NotebookKernelOutputDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5NotebookKernelOutputAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<NotebookDocumentHeaderKernelStateStripViolation>,
) {
    if dispositions.is_empty() {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes
            .contains(&M5NotebookKernelOutputAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::AccessibilityRouteMissing);
    }
    if invariants.pretends_kernel_free_is_live {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::KernelFreePresentedAsLive);
    }
    if invariants.collapses_kernel_origins_into_one_badge {
        violations.push(NotebookDocumentHeaderKernelStateStripViolation::KernelOriginsCollapsed);
    }
    if invariants.conflates_document_and_runtime_truth {
        violations
            .push(NotebookDocumentHeaderKernelStateStripViolation::DocumentAndRuntimeConflated);
    }
    if invariants.hides_state_behind_hover_only {
        violations
            .push(NotebookDocumentHeaderKernelStateStripViolation::StateHiddenBehindHoverOnly);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
