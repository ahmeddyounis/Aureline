//! Frozen M5 notebook-document-header, kernel-state-strip, kernel-picker-row,
//! kernel-origin-pill, output-trust-banner, output-provenance-chip-group,
//! restart-consequence-card, and kernel-recovery-card component matrix.
//!
//! This module locks Aureline's reusable notebook kernel / output components into one
//! export-safe packet. Every notebook surface M5 claims that still drifts too easily on
//! document, runtime, trust, and recovery language — the notebook document header, the kernel
//! state strip, the kernel picker row, the kernel origin pill, the output trust banner, the
//! output provenance chip group, the restart consequence card, and the kernel recovery card —
//! is named once here and constrained by the same canonical `.ipynb` identity, selected kernel
//! origin / class, execution and output trust state, stale-versus-live output honesty,
//! restart / reconnect consequences, preserved-versus-lost state, and choose-another-kernel
//! recovery vocabulary regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families; the one controlled disposition vocabulary every consumer binds
//! (`no_kernel`, `queued`, `busy`, `ready`, `disconnected`, `managed`, `remote`,
//! `stale_output`, `sanitized`, `active`, `reconnect`, `restart_clean`,
//! `choose_another_kernel`); the document source classes and identity states the document
//! header binds; the kernel execution states and connection states the kernel state strip
//! binds; the kernel candidate kinds and selection states the kernel picker row binds; the
//! kernel origin classes and origin trust states the kernel origin pill binds; the output
//! trust classes and freshness states the output trust banner binds; the output provenance
//! kinds and provenance states the provenance chip group binds; the restart action classes
//! and consequence states the restart consequence card binds; the recovery action classes and
//! recovery states the kernel recovery card binds; the deployment lines every component must
//! survive; the non-visual accessibility routes; and the mandatory labels every component must
//! be able to show. It does not re-architect the `.ipynb` document model, kernel transports,
//! output-trust classes, or restart / recovery records that already own those systems — it is
//! the shared notebook-component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 notebook, kernel,
//! output, debug, review, or CLI surface may publish a notebook document header, a kernel
//! state strip, a kernel picker row, a kernel origin pill, an output trust banner, an output
//! provenance chip group, a restart consequence card, or a kernel recovery card. Notebook,
//! kernel-manager, output-viewer, debugger, AI-context, review, and support consumers all read
//! this packet so one document header names its canonical identity and source, one kernel state
//! strip names where a kernel stands, one picker row names its candidates, one origin pill
//! never collapses local, SSH, container, managed, or browser-bridge kernels into one
//! unlabeled badge, one output trust banner never presents stale output as live and never
//! hides its raw / sanitized / active trust class behind a hover-only affordance, one
//! provenance chip group names an output's producing run, one restart consequence card names
//! preserved-versus-lost state, and one kernel recovery card offers reconnect, restart-clean,
//! or choose-another-kernel recovery without ever implying a rerun. No M5 lane invents a second
//! notebook grammar or an alternate label for a governed document, kernel, output-trust, or
//! recovery state.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5NotebookKernelOutputComponentVocabularySet`] rather than minted per surface. Raw
//! notebook JSON, raw cell source, raw output bytes, raw kernel-protocol frames, and private
//! endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_notebook_kernel_output_component_matrix,
    seeded_m5_notebook_kernel_output_component_matrix_kernel_recovery_card_beta_narrowed,
    seeded_m5_notebook_kernel_output_component_matrix_output_trust_banner_preview_narrowed,
    M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5NotebookKernelOutputComponentMatrixPacket`].
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix";

/// Schema version for M5 notebook-kernel-output component-matrix records.
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined notebook-kernel-output component schema.
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-notebook-kernel-output-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF: &str =
    "docs/notebooks/m5_notebook_kernel_output_component_matrix.md";

/// Repo-relative path of the per-component notebook-document-header schema.
pub const M5_NOTEBOOK_DOCUMENT_HEADER_SCHEMA_REF: &str =
    "schemas/ui/m5-notebook-document-header.schema.json";

/// Repo-relative path of the per-component kernel-state-strip schema.
pub const M5_KERNEL_STATE_STRIP_SCHEMA_REF: &str = "schemas/ui/m5-kernel-state-strip.schema.json";

/// Repo-relative path of the per-component kernel-picker-row schema.
pub const M5_KERNEL_PICKER_ROW_SCHEMA_REF: &str = "schemas/ui/m5-kernel-picker-row.schema.json";

/// Repo-relative path of the per-component kernel-origin-pill schema.
pub const M5_KERNEL_ORIGIN_PILL_SCHEMA_REF: &str = "schemas/ui/m5-kernel-origin-pill.schema.json";

/// Repo-relative path of the per-component output-trust-banner schema.
pub const M5_OUTPUT_TRUST_BANNER_SCHEMA_REF: &str = "schemas/ui/m5-output-trust-banner.schema.json";

/// Repo-relative path of the per-component output-provenance-chip-group schema.
pub const M5_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF: &str =
    "schemas/ui/m5-output-provenance-chip-group.schema.json";

/// Repo-relative path of the per-component restart-consequence-card schema.
pub const M5_RESTART_CONSEQUENCE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-restart-consequence-card.schema.json";

/// Repo-relative path of the per-component kernel-recovery-card schema.
pub const M5_KERNEL_RECOVERY_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-kernel-recovery-card.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-notebook-kernel-output-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-notebook-kernel-output-component-matrix.md";

/// One of the eight governed notebook-kernel-output component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputComponentFamily {
    /// A notebook document header carrying its document source class and identity state.
    NotebookDocumentHeader,
    /// A kernel state strip carrying its execution state and connection state.
    KernelStateStrip,
    /// A kernel picker row carrying its candidate kind and selection state.
    KernelPickerRow,
    /// A kernel origin pill carrying its origin class and origin trust state.
    KernelOriginPill,
    /// An output trust banner carrying its output trust class and freshness state.
    OutputTrustBanner,
    /// An output provenance chip group carrying its provenance kind and provenance state.
    OutputProvenanceChipGroup,
    /// A restart consequence card carrying its restart action class and consequence state.
    RestartConsequenceCard,
    /// A kernel recovery card carrying its recovery action class and recovery state.
    KernelRecoveryCard,
}

impl M5NotebookKernelOutputComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotebookDocumentHeader,
        Self::KernelStateStrip,
        Self::KernelPickerRow,
        Self::KernelOriginPill,
        Self::OutputTrustBanner,
        Self::OutputProvenanceChipGroup,
        Self::RestartConsequenceCard,
        Self::KernelRecoveryCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookDocumentHeader => "notebook_document_header",
            Self::KernelStateStrip => "kernel_state_strip",
            Self::KernelPickerRow => "kernel_picker_row",
            Self::KernelOriginPill => "kernel_origin_pill",
            Self::OutputTrustBanner => "output_trust_banner",
            Self::OutputProvenanceChipGroup => "output_provenance_chip_group",
            Self::RestartConsequenceCard => "restart_consequence_card",
            Self::KernelRecoveryCard => "kernel_recovery_card",
        }
    }

    /// `true` when this family is a notebook document header and must therefore declare its
    /// document source classes and identity states.
    pub const fn is_notebook_document_header(self) -> bool {
        matches!(self, Self::NotebookDocumentHeader)
    }

    /// `true` when this family is a kernel state strip and must therefore declare its kernel
    /// execution states and connection states.
    pub const fn is_kernel_state_strip(self) -> bool {
        matches!(self, Self::KernelStateStrip)
    }

    /// `true` when this family is a kernel picker row and must therefore declare its kernel
    /// candidate kinds and selection states.
    pub const fn is_kernel_picker_row(self) -> bool {
        matches!(self, Self::KernelPickerRow)
    }

    /// `true` when this family is a kernel origin pill and must therefore declare its kernel
    /// origin classes and origin trust states.
    pub const fn is_kernel_origin_pill(self) -> bool {
        matches!(self, Self::KernelOriginPill)
    }

    /// `true` when this family is an output trust banner and must therefore declare its output
    /// trust classes and freshness states.
    pub const fn is_output_trust_banner(self) -> bool {
        matches!(self, Self::OutputTrustBanner)
    }

    /// `true` when this family is an output provenance chip group and must therefore declare
    /// its output provenance kinds and provenance states.
    pub const fn is_output_provenance_chip_group(self) -> bool {
        matches!(self, Self::OutputProvenanceChipGroup)
    }

    /// `true` when this family is a restart consequence card and must therefore declare its
    /// restart action classes and consequence states.
    pub const fn is_restart_consequence_card(self) -> bool {
        matches!(self, Self::RestartConsequenceCard)
    }

    /// `true` when this family is a kernel recovery card and must therefore declare its kernel
    /// recovery action classes and recovery states.
    pub const fn is_kernel_recovery_card(self) -> bool {
        matches!(self, Self::KernelRecoveryCard)
    }
}

/// The one controlled disposition vocabulary every notebook-kernel-output component consumer
/// binds. These are the exact acceptance-criteria labels so no surface invents a parallel word
/// for a kernel-free, queued, busy, ready, disconnected, managed, remote, stale-output,
/// sanitized, active, reconnect, restart-clean, or choose-another-kernel state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputDisposition {
    /// No kernel is attached.
    NoKernel,
    /// Execution is queued.
    Queued,
    /// The kernel is busy.
    Busy,
    /// The kernel is ready.
    Ready,
    /// The kernel is disconnected.
    Disconnected,
    /// A managed kernel / workspace.
    Managed,
    /// A remote kernel.
    Remote,
    /// The output is stale.
    StaleOutput,
    /// The output is sanitized.
    Sanitized,
    /// The output is active / raw.
    Active,
    /// A reconnect is offered.
    Reconnect,
    /// A clean restart is offered.
    RestartClean,
    /// Choose-another-kernel recovery is offered.
    ChooseAnotherKernel,
}

impl M5NotebookKernelOutputDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::NoKernel,
        Self::Queued,
        Self::Busy,
        Self::Ready,
        Self::Disconnected,
        Self::Managed,
        Self::Remote,
        Self::StaleOutput,
        Self::Sanitized,
        Self::Active,
        Self::Reconnect,
        Self::RestartClean,
        Self::ChooseAnotherKernel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoKernel => "no_kernel",
            Self::Queued => "queued",
            Self::Busy => "busy",
            Self::Ready => "ready",
            Self::Disconnected => "disconnected",
            Self::Managed => "managed",
            Self::Remote => "remote",
            Self::StaleOutput => "stale_output",
            Self::Sanitized => "sanitized",
            Self::Active => "active",
            Self::Reconnect => "reconnect",
            Self::RestartClean => "restart_clean",
            Self::ChooseAnotherKernel => "choose_another_kernel",
        }
    }
}

/// Controlled document source class — where a notebook document header's `.ipynb` came from, so
/// a header never leaves its canonical local / remote / managed / imported source implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookDocumentSourceClass {
    /// A local `.ipynb`.
    LocalIpynb,
    /// A remote `.ipynb`.
    RemoteIpynb,
    /// A managed-workspace `.ipynb`.
    ManagedWorkspaceIpynb,
    /// An imported `.ipynb`.
    ImportedIpynb,
    /// An unsaved / untitled scratch notebook.
    ScratchUntitled,
    /// An unknown source.
    UnknownSource,
}

impl M5NotebookDocumentSourceClass {
    /// Every document source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalIpynb,
        Self::RemoteIpynb,
        Self::ManagedWorkspaceIpynb,
        Self::ImportedIpynb,
        Self::ScratchUntitled,
        Self::UnknownSource,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalIpynb => "local_ipynb",
            Self::RemoteIpynb => "remote_ipynb",
            Self::ManagedWorkspaceIpynb => "managed_workspace_ipynb",
            Self::ImportedIpynb => "imported_ipynb",
            Self::ScratchUntitled => "scratch_untitled",
            Self::UnknownSource => "unknown_source",
        }
    }
}

/// Controlled document identity state — where a notebook document header's canonical identity
/// stands, so a header never hides that a notebook is unsaved, conflicted, read-only, or
/// recovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookDocumentIdentityState {
    /// Saved and clean.
    SavedClean,
    /// Unsaved changes.
    UnsavedChanges,
    /// Autosaved.
    Autosaved,
    /// Conflicted.
    Conflicted,
    /// Read-only.
    ReadOnly,
    /// Recovered.
    Recovered,
}

impl M5NotebookDocumentIdentityState {
    /// Every document identity state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SavedClean,
        Self::UnsavedChanges,
        Self::Autosaved,
        Self::Conflicted,
        Self::ReadOnly,
        Self::Recovered,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SavedClean => "saved_clean",
            Self::UnsavedChanges => "unsaved_changes",
            Self::Autosaved => "autosaved",
            Self::Conflicted => "conflicted",
            Self::ReadOnly => "read_only",
            Self::Recovered => "recovered",
        }
    }
}

/// Controlled kernel execution state — where a kernel state strip's kernel stands in execution,
/// so a strip never leaves no-kernel, queued, busy, interrupted, dead, or reconnecting implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KernelExecutionState {
    /// Idle and ready.
    IdleReady,
    /// Queued / pending.
    QueuedPending,
    /// Busy / running.
    BusyRunning,
    /// Interrupted.
    Interrupted,
    /// Dead / no kernel.
    DeadNoKernel,
    /// Disconnected / reconnecting.
    DisconnectedReconnecting,
}

impl M5KernelExecutionState {
    /// Every kernel execution state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IdleReady,
        Self::QueuedPending,
        Self::BusyRunning,
        Self::Interrupted,
        Self::DeadNoKernel,
        Self::DisconnectedReconnecting,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdleReady => "idle_ready",
            Self::QueuedPending => "queued_pending",
            Self::BusyRunning => "busy_running",
            Self::Interrupted => "interrupted",
            Self::DeadNoKernel => "dead_no_kernel",
            Self::DisconnectedReconnecting => "disconnected_reconnecting",
        }
    }
}

/// Controlled kernel connection state — how a kernel state strip's kernel is connected, so a
/// strip never collapses a local, remote, reconnecting, or never-connected link into one label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KernelConnectionState {
    /// Connected to a local kernel.
    ConnectedLocal,
    /// Connected to a remote kernel.
    ConnectedRemote,
    /// Reconnecting.
    Reconnecting,
    /// Disconnected.
    Disconnected,
    /// Connection lost.
    ConnectionLost,
    /// Never connected.
    NeverConnected,
}

impl M5KernelConnectionState {
    /// Every kernel connection state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ConnectedLocal,
        Self::ConnectedRemote,
        Self::Reconnecting,
        Self::Disconnected,
        Self::ConnectionLost,
        Self::NeverConnected,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConnectedLocal => "connected_local",
            Self::ConnectedRemote => "connected_remote",
            Self::Reconnecting => "reconnecting",
            Self::Disconnected => "disconnected",
            Self::ConnectionLost => "connection_lost",
            Self::NeverConnected => "never_connected",
        }
    }
}

/// Controlled kernel candidate kind — what kind of kernel a kernel picker row offers, so a row
/// never leaves an interpreter, virtual env, conda env, container, remote, or managed candidate
/// unlabeled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KernelCandidateKind {
    /// A local interpreter.
    LocalInterpreter,
    /// A virtual environment.
    VirtualEnv,
    /// A conda environment.
    CondaEnv,
    /// A container kernel.
    ContainerKernel,
    /// A remote kernel.
    RemoteKernel,
    /// A managed kernel.
    ManagedKernel,
}

impl M5KernelCandidateKind {
    /// Every kernel candidate kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalInterpreter,
        Self::VirtualEnv,
        Self::CondaEnv,
        Self::ContainerKernel,
        Self::RemoteKernel,
        Self::ManagedKernel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInterpreter => "local_interpreter",
            Self::VirtualEnv => "virtual_env",
            Self::CondaEnv => "conda_env",
            Self::ContainerKernel => "container_kernel",
            Self::RemoteKernel => "remote_kernel",
            Self::ManagedKernel => "managed_kernel",
        }
    }
}

/// Controlled kernel selection state — where a kernel picker row's candidate stands, so a row
/// never hides that a candidate is incompatible, unavailable, or needs install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KernelSelectionState {
    /// Selected.
    Selected,
    /// Available.
    Available,
    /// Recommended.
    Recommended,
    /// Incompatible.
    Incompatible,
    /// Unavailable.
    Unavailable,
    /// Needs install.
    NeedsInstall,
}

impl M5KernelSelectionState {
    /// Every kernel selection state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Selected,
        Self::Available,
        Self::Recommended,
        Self::Incompatible,
        Self::Unavailable,
        Self::NeedsInstall,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Selected => "selected",
            Self::Available => "available",
            Self::Recommended => "recommended",
            Self::Incompatible => "incompatible",
            Self::Unavailable => "unavailable",
            Self::NeedsInstall => "needs_install",
        }
    }
}

/// Controlled kernel origin class — where a kernel origin pill's kernel physically runs, so a
/// pill never collapses local, SSH, container, devcontainer, managed, or browser-bridge kernels
/// into one unlabeled badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KernelOriginClass {
    /// A local host.
    LocalHost,
    /// An SSH remote.
    SshRemote,
    /// A container.
    Container,
    /// A devcontainer.
    Devcontainer,
    /// A managed workspace.
    ManagedWorkspace,
    /// A browser bridge.
    BrowserBridge,
}

impl M5KernelOriginClass {
    /// Every kernel origin class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalHost,
        Self::SshRemote,
        Self::Container,
        Self::Devcontainer,
        Self::ManagedWorkspace,
        Self::BrowserBridge,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHost => "local_host",
            Self::SshRemote => "ssh_remote",
            Self::Container => "container",
            Self::Devcontainer => "devcontainer",
            Self::ManagedWorkspace => "managed_workspace",
            Self::BrowserBridge => "browser_bridge",
        }
    }
}

/// Controlled kernel origin trust state — how trusted a kernel origin pill's origin is, so a
/// pill never leaves a third-party, unverified, or restricted origin implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KernelOriginTrustState {
    /// A trusted origin.
    TrustedOrigin,
    /// A first-party origin.
    FirstParty,
    /// A third-party origin.
    ThirdParty,
    /// An unverified origin.
    UnverifiedOrigin,
    /// A restricted origin.
    RestrictedOrigin,
    /// An unknown origin.
    UnknownOrigin,
}

impl M5KernelOriginTrustState {
    /// Every kernel origin trust state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TrustedOrigin,
        Self::FirstParty,
        Self::ThirdParty,
        Self::UnverifiedOrigin,
        Self::RestrictedOrigin,
        Self::UnknownOrigin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedOrigin => "trusted_origin",
            Self::FirstParty => "first_party",
            Self::ThirdParty => "third_party",
            Self::UnverifiedOrigin => "unverified_origin",
            Self::RestrictedOrigin => "restricted_origin",
            Self::UnknownOrigin => "unknown_origin",
        }
    }
}

/// Controlled output trust class — how an output trust banner classes its output, so a banner
/// never hides a raw / sanitized / active trust class behind a hover-only affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OutputTrustClass {
    /// A trusted output.
    TrustedOutput,
    /// A sanitized output.
    SanitizedOutput,
    /// A sandboxed output.
    SandboxedOutput,
    /// A raw / active output.
    RawActiveOutput,
    /// A blocked output.
    BlockedOutput,
    /// An unknown trust class.
    UnknownTrust,
}

impl M5OutputTrustClass {
    /// Every output trust class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TrustedOutput,
        Self::SanitizedOutput,
        Self::SandboxedOutput,
        Self::RawActiveOutput,
        Self::BlockedOutput,
        Self::UnknownTrust,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedOutput => "trusted_output",
            Self::SanitizedOutput => "sanitized_output",
            Self::SandboxedOutput => "sandboxed_output",
            Self::RawActiveOutput => "raw_active_output",
            Self::BlockedOutput => "blocked_output",
            Self::UnknownTrust => "unknown_trust",
        }
    }
}

/// Controlled output freshness state — whether an output trust banner's output is live or
/// stale, so a banner never presents stale output as live truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OutputFreshnessState {
    /// Live output.
    LiveOutput,
    /// Stale output.
    StaleOutput,
    /// Cached output.
    CachedOutput,
    /// Cleared output.
    ClearedOutput,
    /// Superseded output.
    SupersededOutput,
    /// No output.
    NoOutput,
}

impl M5OutputFreshnessState {
    /// Every output freshness state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveOutput,
        Self::StaleOutput,
        Self::CachedOutput,
        Self::ClearedOutput,
        Self::SupersededOutput,
        Self::NoOutput,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveOutput => "live_output",
            Self::StaleOutput => "stale_output",
            Self::CachedOutput => "cached_output",
            Self::ClearedOutput => "cleared_output",
            Self::SupersededOutput => "superseded_output",
            Self::NoOutput => "no_output",
        }
    }
}

/// Controlled output provenance kind — what produced an output an output provenance chip group
/// tracks, so a chip group never leaves an output's producing run or import implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OutputProvenanceKind {
    /// Produced by a cell.
    ProducedByCell,
    /// Produced by a run.
    ProducedByRun,
    /// An imported output.
    ImportedOutput,
    /// A restored output.
    RestoredOutput,
    /// An external output.
    ExternalOutput,
    /// An unknown provenance.
    UnknownProvenance,
}

impl M5OutputProvenanceKind {
    /// Every output provenance kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProducedByCell,
        Self::ProducedByRun,
        Self::ImportedOutput,
        Self::RestoredOutput,
        Self::ExternalOutput,
        Self::UnknownProvenance,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProducedByCell => "produced_by_cell",
            Self::ProducedByRun => "produced_by_run",
            Self::ImportedOutput => "imported_output",
            Self::RestoredOutput => "restored_output",
            Self::ExternalOutput => "external_output",
            Self::UnknownProvenance => "unknown_provenance",
        }
    }
}

/// Controlled output provenance state — how completely an output provenance chip group resolves
/// an output's execution lineage, so a chip group never hides a drifted execution count or a
/// missing provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OutputProvenanceState {
    /// Provenance complete.
    ProvenanceComplete,
    /// Provenance partial.
    ProvenancePartial,
    /// Provenance missing.
    ProvenanceMissing,
    /// Execution count pinned.
    ExecutionCountPinned,
    /// Execution count drifted.
    ExecutionCountDrifted,
    /// Provenance stale.
    ProvenanceStale,
}

impl M5OutputProvenanceState {
    /// Every output provenance state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProvenanceComplete,
        Self::ProvenancePartial,
        Self::ProvenanceMissing,
        Self::ExecutionCountPinned,
        Self::ExecutionCountDrifted,
        Self::ProvenanceStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceComplete => "provenance_complete",
            Self::ProvenancePartial => "provenance_partial",
            Self::ProvenanceMissing => "provenance_missing",
            Self::ExecutionCountPinned => "execution_count_pinned",
            Self::ExecutionCountDrifted => "execution_count_drifted",
            Self::ProvenanceStale => "provenance_stale",
        }
    }
}

/// Controlled restart action class — which restart / interrupt action a restart consequence
/// card describes, so a card never leaves what a restart, interrupt, or shutdown actually does
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestartActionClass {
    /// Restart the kernel.
    RestartKernel,
    /// Restart and run all.
    RestartAndRunAll,
    /// Interrupt the kernel.
    InterruptKernel,
    /// Shut down the kernel.
    ShutdownKernel,
    /// Reconnect the kernel.
    ReconnectKernel,
    /// Clear outputs.
    ClearOutputs,
}

impl M5RestartActionClass {
    /// Every restart action class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RestartKernel,
        Self::RestartAndRunAll,
        Self::InterruptKernel,
        Self::ShutdownKernel,
        Self::ReconnectKernel,
        Self::ClearOutputs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestartKernel => "restart_kernel",
            Self::RestartAndRunAll => "restart_and_run_all",
            Self::InterruptKernel => "interrupt_kernel",
            Self::ShutdownKernel => "shutdown_kernel",
            Self::ReconnectKernel => "reconnect_kernel",
            Self::ClearOutputs => "clear_outputs",
        }
    }
}

/// Controlled restart consequence state — what a restart consequence card says survives a
/// restart, so a card never hides that variables or outputs will be lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestartConsequenceState {
    /// State preserved.
    StatePreserved,
    /// State lost.
    StateLost,
    /// Variables cleared.
    VariablesCleared,
    /// Outputs retained.
    OutputsRetained,
    /// Outputs cleared.
    OutputsCleared,
    /// No consequence.
    NoConsequence,
}

impl M5RestartConsequenceState {
    /// Every restart consequence state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StatePreserved,
        Self::StateLost,
        Self::VariablesCleared,
        Self::OutputsRetained,
        Self::OutputsCleared,
        Self::NoConsequence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePreserved => "state_preserved",
            Self::StateLost => "state_lost",
            Self::VariablesCleared => "variables_cleared",
            Self::OutputsRetained => "outputs_retained",
            Self::OutputsCleared => "outputs_cleared",
            Self::NoConsequence => "no_consequence",
        }
    }
}

/// Controlled kernel recovery action class — which recovery action a kernel recovery card
/// offers, so a card offers reconnect, restart-clean, or choose-another-kernel recovery without
/// ever implying a rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KernelRecoveryActionClass {
    /// Reconnect.
    Reconnect,
    /// Restart clean.
    RestartClean,
    /// Choose another kernel.
    ChooseAnotherKernel,
    /// Reattach the session.
    ReattachSession,
    /// Start a local fallback.
    StartLocalFallback,
    /// Wait for a managed kernel.
    WaitForManaged,
}

impl M5KernelRecoveryActionClass {
    /// Every kernel recovery action class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Reconnect,
        Self::RestartClean,
        Self::ChooseAnotherKernel,
        Self::ReattachSession,
        Self::StartLocalFallback,
        Self::WaitForManaged,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reconnect => "reconnect",
            Self::RestartClean => "restart_clean",
            Self::ChooseAnotherKernel => "choose_another_kernel",
            Self::ReattachSession => "reattach_session",
            Self::StartLocalFallback => "start_local_fallback",
            Self::WaitForManaged => "wait_for_managed",
        }
    }
}

/// Controlled kernel recovery state — where a kernel recovery card's recovery stands, so a card
/// never leaves whether a kernel is recoverable, needs a restart, or has no kernel available
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KernelRecoveryState {
    /// Recoverable.
    Recoverable,
    /// Reconnect available.
    ReconnectAvailable,
    /// Restart required.
    RestartRequired,
    /// No kernel available.
    NoKernelAvailable,
    /// Recovery blocked.
    RecoveryBlocked,
    /// Recovered.
    Recovered,
}

impl M5KernelRecoveryState {
    /// Every kernel recovery state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Recoverable,
        Self::ReconnectAvailable,
        Self::RestartRequired,
        Self::NoKernelAvailable,
        Self::RecoveryBlocked,
        Self::Recovered,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recoverable => "recoverable",
            Self::ReconnectAvailable => "reconnect_available",
            Self::RestartRequired => "restart_required",
            Self::NoKernelAvailable => "no_kernel_available",
            Self::RecoveryBlocked => "recovery_blocked",
            Self::Recovered => "recovered",
        }
    }
}

/// Claimed M5 notebook surface family that renders / consumes a notebook-kernel-output
/// component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputSurfaceFamily {
    /// The notebook surface.
    NotebookSurface,
    /// The kernel manager surface.
    KernelManagerSurface,
    /// The output viewer surface.
    OutputViewerSurface,
    /// The debug surface.
    DebugSurface,
    /// The review surface.
    ReviewSurface,
    /// The CLI surface.
    CliSurface,
}

impl M5NotebookKernelOutputSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotebookSurface,
        Self::KernelManagerSurface,
        Self::OutputViewerSurface,
        Self::DebugSurface,
        Self::ReviewSurface,
        Self::CliSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookSurface => "notebook_surface",
            Self::KernelManagerSurface => "kernel_manager_surface",
            Self::OutputViewerSurface => "output_viewer_surface",
            Self::DebugSurface => "debug_surface",
            Self::ReviewSurface => "review_surface",
            Self::CliSurface => "cli_surface",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's document,
/// kernel, output-trust, or recovery truth never silently narrows or widens between deployment
/// shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5NotebookKernelOutputDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputConsumerSurface {
    /// The notebook UI.
    NotebookUi,
    /// The kernel-manager UI.
    KernelManagerUi,
    /// The output-viewer UI.
    OutputViewerUi,
    /// The debugger UI.
    DebuggerUi,
    /// The AI-context UI.
    AiContextUi,
    /// The review UI.
    ReviewUi,
    /// The CLI surface.
    CliSurface,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5NotebookKernelOutputConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::NotebookUi,
        Self::KernelManagerUi,
        Self::OutputViewerUi,
        Self::DebuggerUi,
        Self::AiContextUi,
        Self::ReviewUi,
        Self::CliSurface,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookUi => "notebook_ui",
            Self::KernelManagerUi => "kernel_manager_ui",
            Self::OutputViewerUi => "output_viewer_ui",
            Self::DebuggerUi => "debugger_ui",
            Self::AiContextUi => "ai_context_ui",
            Self::ReviewUi => "review_ui",
            Self::CliSurface => "cli_surface",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no notebook, kernel, or
/// output truth is hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5NotebookKernelOutputAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed notebook-kernel-output component must be able to show. The first
/// three are hard requirements on every component; the remaining three close the
/// acceptance-criteria ambiguity about kernel origin / class, output trust / freshness, and
/// restart / recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The kernel origin and class behind the component.
    KernelOriginAndClass,
    /// The output trust class and freshness of the component.
    OutputTrustAndFreshness,
    /// The restart consequence and recovery posture of the component.
    RestartAndRecovery,
}

impl M5NotebookKernelOutputRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::KernelOriginAndClass,
        Self::OutputTrustAndFreshness,
        Self::RestartAndRecovery,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::KernelOriginAndClass => "kernel_origin_and_class",
            Self::OutputTrustAndFreshness => "output_trust_and_freshness",
            Self::RestartAndRecovery => "restart_and_recovery",
        }
    }
}

/// Qualification class for an M5 notebook-kernel-output component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5NotebookKernelOutputQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a notebook-kernel-output component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookKernelOutputDowngradeTrigger {
    /// A component left its kernel origin unstated.
    KernelOriginUnstated,
    /// A component collapsed kernel classes into one unlabeled badge.
    KernelClassCollapsed,
    /// A component left its document identity unstated.
    DocumentIdentityUnstated,
    /// A component left its output trust class unstated.
    OutputTrustUnstated,
    /// A component presented stale output as live.
    StaleOutputShownAsLive,
    /// A component hid a trust class behind a hover-only affordance.
    TrustClassHoverOnly,
    /// A component severed an output's canonical provenance.
    ProvenanceSevered,
    /// A restart consequence card implied a rerun.
    RestartConsequenceImpliedRerun,
    /// A recovery card overclaimed continuity after recovery.
    RecoveryOverclaimed,
    /// A component showed a reconnect as fresh live state.
    ReconnectShownAsFresh,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5NotebookKernelOutputDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::KernelOriginUnstated,
        Self::KernelClassCollapsed,
        Self::DocumentIdentityUnstated,
        Self::OutputTrustUnstated,
        Self::StaleOutputShownAsLive,
        Self::TrustClassHoverOnly,
        Self::ProvenanceSevered,
        Self::RestartConsequenceImpliedRerun,
        Self::RecoveryOverclaimed,
        Self::ReconnectShownAsFresh,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KernelOriginUnstated => "kernel_origin_unstated",
            Self::KernelClassCollapsed => "kernel_class_collapsed",
            Self::DocumentIdentityUnstated => "document_identity_unstated",
            Self::OutputTrustUnstated => "output_trust_unstated",
            Self::StaleOutputShownAsLive => "stale_output_shown_as_live",
            Self::TrustClassHoverOnly => "trust_class_hover_only",
            Self::ProvenanceSevered => "provenance_severed",
            Self::RestartConsequenceImpliedRerun => "restart_consequence_implied_rerun",
            Self::RecoveryOverclaimed => "recovery_overclaimed",
            Self::ReconnectShownAsFresh => "reconnect_shown_as_fresh",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed notebook-kernel-output component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotebookKernelOutputComponentRow {
    /// Governed component family.
    pub component_family: M5NotebookKernelOutputComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5NotebookKernelOutputQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 notebook surface families that render / consume this component.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5NotebookKernelOutputRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Controlled dispositions this component binds (must be non-empty; drawn from the one
    /// shared [`M5NotebookKernelOutputDisposition`] vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Document source classes this component names (notebook-document-header only).
    pub document_source_classes: Vec<M5NotebookDocumentSourceClass>,
    /// Document identity states this component names (notebook-document-header only).
    pub document_identity_states: Vec<M5NotebookDocumentIdentityState>,
    /// Kernel execution states this component names (kernel-state-strip only).
    pub kernel_execution_states: Vec<M5KernelExecutionState>,
    /// Kernel connection states this component names (kernel-state-strip only).
    pub kernel_connection_states: Vec<M5KernelConnectionState>,
    /// Kernel candidate kinds this component names (kernel-picker-row only).
    pub kernel_candidate_kinds: Vec<M5KernelCandidateKind>,
    /// Kernel selection states this component names (kernel-picker-row only).
    pub kernel_selection_states: Vec<M5KernelSelectionState>,
    /// Kernel origin classes this component names (kernel-origin-pill only).
    pub kernel_origin_classes: Vec<M5KernelOriginClass>,
    /// Kernel origin trust states this component names (kernel-origin-pill only).
    pub kernel_origin_trust_states: Vec<M5KernelOriginTrustState>,
    /// Output trust classes this component names (output-trust-banner only).
    pub output_trust_classes: Vec<M5OutputTrustClass>,
    /// Output freshness states this component names (output-trust-banner only).
    pub output_freshness_states: Vec<M5OutputFreshnessState>,
    /// Output provenance kinds this component names (output-provenance-chip-group only).
    pub output_provenance_kinds: Vec<M5OutputProvenanceKind>,
    /// Output provenance states this component names (output-provenance-chip-group only).
    pub output_provenance_states: Vec<M5OutputProvenanceState>,
    /// Restart action classes this component names (restart-consequence-card only).
    pub restart_action_classes: Vec<M5RestartActionClass>,
    /// Restart consequence states this component names (restart-consequence-card only).
    pub restart_consequence_states: Vec<M5RestartConsequenceState>,
    /// Kernel recovery action classes this component names (kernel-recovery-card only).
    pub kernel_recovery_action_classes: Vec<M5KernelRecoveryActionClass>,
    /// Kernel recovery states this component names (kernel-recovery-card only).
    pub kernel_recovery_states: Vec<M5KernelRecoveryState>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never lets a kernel recovery card imply a rerun. MUST be
    /// `false`.
    pub recovery_card_implies_rerun: bool,
    /// Hard invariant: this component never presents stale output as live. MUST be `false`.
    pub presents_stale_output_as_live: bool,
    /// Hard invariant: this component never hides its raw / sanitized / active trust class
    /// behind a hover-only affordance. MUST be `false`.
    pub hides_trust_class_behind_hover_only: bool,
    /// Hard invariant: this component never collapses local, SSH, container, managed, or
    /// browser-bridge kernels into one unlabeled badge. MUST be `false`.
    pub collapses_kernel_origins_into_one_badge: bool,
}

impl M5NotebookKernelOutputComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.recovery_card_implies_rerun
            && !self.presents_stale_output_as_live
            && !self.hides_trust_class_behind_hover_only
            && !self.collapses_kernel_origins_into_one_badge
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotebookKernelOutputComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Disposition tokens (the one shared consumer vocabulary).
    pub dispositions: Vec<String>,
    /// Document-source-class tokens.
    pub document_source_classes: Vec<String>,
    /// Document-identity-state tokens.
    pub document_identity_states: Vec<String>,
    /// Kernel-execution-state tokens.
    pub kernel_execution_states: Vec<String>,
    /// Kernel-connection-state tokens.
    pub kernel_connection_states: Vec<String>,
    /// Kernel-candidate-kind tokens.
    pub kernel_candidate_kinds: Vec<String>,
    /// Kernel-selection-state tokens.
    pub kernel_selection_states: Vec<String>,
    /// Kernel-origin-class tokens.
    pub kernel_origin_classes: Vec<String>,
    /// Kernel-origin-trust-state tokens.
    pub kernel_origin_trust_states: Vec<String>,
    /// Output-trust-class tokens.
    pub output_trust_classes: Vec<String>,
    /// Output-freshness-state tokens.
    pub output_freshness_states: Vec<String>,
    /// Output-provenance-kind tokens.
    pub output_provenance_kinds: Vec<String>,
    /// Output-provenance-state tokens.
    pub output_provenance_states: Vec<String>,
    /// Restart-action-class tokens.
    pub restart_action_classes: Vec<String>,
    /// Restart-consequence-state tokens.
    pub restart_consequence_states: Vec<String>,
    /// Kernel-recovery-action-class tokens.
    pub kernel_recovery_action_classes: Vec<String>,
    /// Kernel-recovery-state tokens.
    pub kernel_recovery_states: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5NotebookKernelOutputComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5NotebookKernelOutputComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5NotebookKernelOutputDisposition::ALL, |v| v.as_str()),
            document_source_classes: tokens(&M5NotebookDocumentSourceClass::ALL, |v| v.as_str()),
            document_identity_states: tokens(&M5NotebookDocumentIdentityState::ALL, |v| v.as_str()),
            kernel_execution_states: tokens(&M5KernelExecutionState::ALL, |v| v.as_str()),
            kernel_connection_states: tokens(&M5KernelConnectionState::ALL, |v| v.as_str()),
            kernel_candidate_kinds: tokens(&M5KernelCandidateKind::ALL, |v| v.as_str()),
            kernel_selection_states: tokens(&M5KernelSelectionState::ALL, |v| v.as_str()),
            kernel_origin_classes: tokens(&M5KernelOriginClass::ALL, |v| v.as_str()),
            kernel_origin_trust_states: tokens(&M5KernelOriginTrustState::ALL, |v| v.as_str()),
            output_trust_classes: tokens(&M5OutputTrustClass::ALL, |v| v.as_str()),
            output_freshness_states: tokens(&M5OutputFreshnessState::ALL, |v| v.as_str()),
            output_provenance_kinds: tokens(&M5OutputProvenanceKind::ALL, |v| v.as_str()),
            output_provenance_states: tokens(&M5OutputProvenanceState::ALL, |v| v.as_str()),
            restart_action_classes: tokens(&M5RestartActionClass::ALL, |v| v.as_str()),
            restart_consequence_states: tokens(&M5RestartConsequenceState::ALL, |v| v.as_str()),
            kernel_recovery_action_classes: tokens(&M5KernelRecoveryActionClass::ALL, |v| {
                v.as_str()
            }),
            kernel_recovery_states: tokens(&M5KernelRecoveryState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5NotebookKernelOutputSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5NotebookKernelOutputDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5NotebookKernelOutputConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5NotebookKernelOutputAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            required_labels: tokens(&M5NotebookKernelOutputRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotebookKernelOutputComponentGovernanceReview {
    /// The notebook document header shows its canonical identity and source.
    pub document_header_shows_identity_and_source: bool,
    /// The kernel state strip shows its execution and connection state.
    pub kernel_state_strip_shows_execution_and_connection: bool,
    /// The kernel picker row shows its candidates and selection state.
    pub kernel_picker_row_shows_candidates_and_selection: bool,
    /// The kernel origin pill shows its origin and class.
    pub kernel_origin_pill_shows_origin_and_class: bool,
    /// The output trust banner shows its trust class and freshness.
    pub output_trust_banner_shows_trust_and_freshness: bool,
    /// The output provenance chip group shows its producing run.
    pub output_provenance_chip_group_shows_provenance: bool,
    /// The restart consequence card shows preserved-versus-lost state.
    pub restart_consequence_card_shows_preserved_and_lost: bool,
    /// The kernel recovery card shows recovery without implying a rerun.
    pub kernel_recovery_card_shows_recovery_without_implying_rerun: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// No output banner presents stale output as live.
    pub stale_output_never_presented_as_live: bool,
    /// No output banner hides its trust class behind a hover-only affordance.
    pub trust_class_never_hover_only: bool,
    /// No origin pill collapses kernel origins into one unlabeled badge.
    pub kernel_origins_never_collapsed_into_one_badge: bool,
    /// Kernel origin and class stay explicit.
    pub kernel_origin_and_class_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel notebook vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotebookKernelOutputComponentConsumerProjection {
    /// Notebook surfaces consume the shared document and kernel vocabulary.
    pub notebook_surfaces_consume_document_and_kernel_vocabulary: bool,
    /// Kernel surfaces consume the origin and recovery vocabulary.
    pub kernel_surfaces_consume_origin_and_recovery_vocabulary: bool,
    /// Output surfaces consume the trust and provenance vocabulary.
    pub output_surfaces_consume_trust_and_provenance_vocabulary: bool,
    /// Debug surfaces consume the restart and consequence vocabulary.
    pub debug_surfaces_consume_restart_and_consequence_vocabulary: bool,
    /// Recovery surfaces consume the recovery and reconnect vocabulary.
    pub recovery_surfaces_consume_recovery_and_reconnect_vocabulary: bool,
    /// Support / export reads a single canonical notebook-component source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotebookKernelOutputComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the notebook-kernel-output component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotebookKernelOutputComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting notebook-component audit for the lane.
    pub notebook_component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5NotebookKernelOutputComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5NotebookKernelOutputComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5NotebookKernelOutputComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5NotebookKernelOutputComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5NotebookKernelOutputComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5NotebookKernelOutputComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5NotebookKernelOutputComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5NotebookKernelOutputComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 notebook-kernel-output component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotebookKernelOutputComponentMatrixPacket {
    /// Record kind; must equal [`M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5NotebookKernelOutputComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5NotebookKernelOutputComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5NotebookKernelOutputComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5NotebookKernelOutputComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5NotebookKernelOutputComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5NotebookKernelOutputComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5NotebookKernelOutputComponentMatrixPacket {
    /// Builds an M5 notebook-kernel-output component matrix packet from stable-lane input.
    pub fn new(input: M5NotebookKernelOutputComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 notebook-kernel-output component matrix invariants.
    pub fn validate(&self) -> Vec<M5NotebookKernelOutputComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 notebook kernel output component matrix packet serializes"),
        ) {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 notebook kernel output component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,dispositions,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.dispositions, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Notebook-Document-Header, Kernel-State-Strip, Kernel-Picker-Row, Kernel-Origin-Pill, Output-Trust-Banner, Output-Provenance-Chip-Group, Restart-Consequence-Card, and Kernel-Recovery-Card Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Dispositions: {}\n",
                row.dispositions
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 notebook-kernel-output matrix export.
#[derive(Debug)]
pub enum M5NotebookKernelOutputComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5NotebookKernelOutputComponentMatrixViolation>),
}

impl fmt::Display for M5NotebookKernelOutputComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 notebook kernel output component matrix export parse failed: {error}"
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
                    "m5 notebook kernel output component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5NotebookKernelOutputComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5NotebookKernelOutputComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5NotebookKernelOutputComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row declares no dispositions.
    DispositionsMissing,
    /// A notebook-document-header component declares no document source classes.
    DocumentSourceClassMissing,
    /// A notebook-document-header component declares no document identity states.
    DocumentIdentityStateMissing,
    /// A kernel-state-strip component declares no kernel execution states.
    KernelExecutionStateMissing,
    /// A kernel-state-strip component declares no kernel connection states.
    KernelConnectionStateMissing,
    /// A kernel-picker-row component declares no kernel candidate kinds.
    KernelCandidateKindMissing,
    /// A kernel-picker-row component declares no kernel selection states.
    KernelSelectionStateMissing,
    /// A kernel-origin-pill component declares no kernel origin classes.
    KernelOriginClassMissing,
    /// A kernel-origin-pill component declares no kernel origin trust states.
    KernelOriginTrustStateMissing,
    /// An output-trust-banner component declares no output trust classes.
    OutputTrustClassMissing,
    /// An output-trust-banner component declares no output freshness states.
    OutputFreshnessStateMissing,
    /// An output-provenance-chip-group component declares no output provenance kinds.
    OutputProvenanceKindMissing,
    /// An output-provenance-chip-group component declares no output provenance states.
    OutputProvenanceStateMissing,
    /// A restart-consequence-card component declares no restart action classes.
    RestartActionClassMissing,
    /// A restart-consequence-card component declares no restart consequence states.
    RestartConsequenceStateMissing,
    /// A kernel-recovery-card component declares no kernel recovery action classes.
    KernelRecoveryActionClassMissing,
    /// A kernel-recovery-card component declares no kernel recovery states.
    KernelRecoveryStateMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (recovery implies rerun, stale output shown as
    /// live, trust class hidden behind hover-only, or kernel origins collapsed into one badge).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5NotebookKernelOutputComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DocumentSourceClassMissing => "document_source_class_missing",
            Self::DocumentIdentityStateMissing => "document_identity_state_missing",
            Self::KernelExecutionStateMissing => "kernel_execution_state_missing",
            Self::KernelConnectionStateMissing => "kernel_connection_state_missing",
            Self::KernelCandidateKindMissing => "kernel_candidate_kind_missing",
            Self::KernelSelectionStateMissing => "kernel_selection_state_missing",
            Self::KernelOriginClassMissing => "kernel_origin_class_missing",
            Self::KernelOriginTrustStateMissing => "kernel_origin_trust_state_missing",
            Self::OutputTrustClassMissing => "output_trust_class_missing",
            Self::OutputFreshnessStateMissing => "output_freshness_state_missing",
            Self::OutputProvenanceKindMissing => "output_provenance_kind_missing",
            Self::OutputProvenanceStateMissing => "output_provenance_state_missing",
            Self::RestartActionClassMissing => "restart_action_class_missing",
            Self::RestartConsequenceStateMissing => "restart_consequence_state_missing",
            Self::KernelRecoveryActionClassMissing => "kernel_recovery_action_class_missing",
            Self::KernelRecoveryStateMissing => "kernel_recovery_state_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 notebook-kernel-output matrix export.
pub fn current_stable_m5_notebook_kernel_output_component_matrix_export() -> Result<
    M5NotebookKernelOutputComponentMatrixPacket,
    M5NotebookKernelOutputComponentMatrixArtifactError,
> {
    let packet: M5NotebookKernelOutputComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-notebook-kernel-output-proof/support_export.json"
        )))
        .map_err(M5NotebookKernelOutputComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5NotebookKernelOutputComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5NotebookKernelOutputComponentMatrixPacket,
    violations: &mut Vec<M5NotebookKernelOutputComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_NOTEBOOK_DOCUMENT_HEADER_SCHEMA_REF,
        M5_KERNEL_STATE_STRIP_SCHEMA_REF,
        M5_KERNEL_PICKER_ROW_SCHEMA_REF,
        M5_KERNEL_ORIGIN_PILL_SCHEMA_REF,
        M5_OUTPUT_TRUST_BANNER_SCHEMA_REF,
        M5_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF,
        M5_RESTART_CONSEQUENCE_CARD_SCHEMA_REF,
        M5_KERNEL_RECOVERY_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5NotebookKernelOutputComponentMatrixPacket,
    violations: &mut Vec<M5NotebookKernelOutputComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5NotebookKernelOutputComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5NotebookKernelOutputComponentMatrixPacket,
    violations: &mut Vec<M5NotebookKernelOutputComponentMatrixViolation>,
) {
    let present: BTreeSet<M5NotebookKernelOutputComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5NotebookKernelOutputComponentFamily::ALL {
        if !present.contains(&required) {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::MandatoryLabelMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::DispositionsMissing);
        }
        if family.is_notebook_document_header() && row.document_source_classes.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::DocumentSourceClassMissing);
        }
        if family.is_notebook_document_header() && row.document_identity_states.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::DocumentIdentityStateMissing);
        }
        if family.is_kernel_state_strip() && row.kernel_execution_states.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::KernelExecutionStateMissing);
        }
        if family.is_kernel_state_strip() && row.kernel_connection_states.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::KernelConnectionStateMissing);
        }
        if family.is_kernel_picker_row() && row.kernel_candidate_kinds.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::KernelCandidateKindMissing);
        }
        if family.is_kernel_picker_row() && row.kernel_selection_states.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::KernelSelectionStateMissing);
        }
        if family.is_kernel_origin_pill() && row.kernel_origin_classes.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::KernelOriginClassMissing);
        }
        if family.is_kernel_origin_pill() && row.kernel_origin_trust_states.is_empty() {
            violations.push(
                M5NotebookKernelOutputComponentMatrixViolation::KernelOriginTrustStateMissing,
            );
        }
        if family.is_output_trust_banner() && row.output_trust_classes.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::OutputTrustClassMissing);
        }
        if family.is_output_trust_banner() && row.output_freshness_states.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::OutputFreshnessStateMissing);
        }
        if family.is_output_provenance_chip_group() && row.output_provenance_kinds.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::OutputProvenanceKindMissing);
        }
        if family.is_output_provenance_chip_group() && row.output_provenance_states.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::OutputProvenanceStateMissing);
        }
        if family.is_restart_consequence_card() && row.restart_action_classes.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::RestartActionClassMissing);
        }
        if family.is_restart_consequence_card() && row.restart_consequence_states.is_empty() {
            violations.push(
                M5NotebookKernelOutputComponentMatrixViolation::RestartConsequenceStateMissing,
            );
        }
        if family.is_kernel_recovery_card() && row.kernel_recovery_action_classes.is_empty() {
            violations.push(
                M5NotebookKernelOutputComponentMatrixViolation::KernelRecoveryActionClassMissing,
            );
        }
        if family.is_kernel_recovery_card() && row.kernel_recovery_states.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::KernelRecoveryStateMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5NotebookKernelOutputComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5NotebookKernelOutputComponentMatrixPacket,
    violations: &mut Vec<M5NotebookKernelOutputComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.document_header_shows_identity_and_source,
        review.kernel_state_strip_shows_execution_and_connection,
        review.kernel_picker_row_shows_candidates_and_selection,
        review.kernel_origin_pill_shows_origin_and_class,
        review.output_trust_banner_shows_trust_and_freshness,
        review.output_provenance_chip_group_shows_provenance,
        review.restart_consequence_card_shows_preserved_and_lost,
        review.kernel_recovery_card_shows_recovery_without_implying_rerun,
        review.no_surface_invents_alternate_state_label,
        review.stale_output_never_presented_as_live,
        review.trust_class_never_hover_only,
        review.kernel_origins_never_collapsed_into_one_badge,
        review.kernel_origin_and_class_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5NotebookKernelOutputComponentMatrixPacket,
    violations: &mut Vec<M5NotebookKernelOutputComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.notebook_surfaces_consume_document_and_kernel_vocabulary,
        projection.kernel_surfaces_consume_origin_and_recovery_vocabulary,
        projection.output_surfaces_consume_trust_and_provenance_vocabulary,
        projection.debug_surfaces_consume_restart_and_consequence_vocabulary,
        projection.recovery_surfaces_consume_recovery_and_reconnect_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5NotebookKernelOutputComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5NotebookKernelOutputComponentMatrixPacket,
    violations: &mut Vec<M5NotebookKernelOutputComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5NotebookKernelOutputComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5NotebookKernelOutputComponentMatrixPacket,
    violations: &mut Vec<M5NotebookKernelOutputComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.notebook_component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5NotebookKernelOutputComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
