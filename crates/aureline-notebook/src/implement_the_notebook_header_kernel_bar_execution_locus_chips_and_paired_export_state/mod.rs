//! Notebook header, kernel bar, execution-locus chips, and paired-export state.
//!
//! This module materializes the composed UI surface that the notebook chrome
//! consumes to render the header, kernel bar, execution-locus chips, and
//! paired-export state. It reuses the closed vocabularies and backing records
//! already frozen in the [`crate::runtime_truth`] module and adds the
//! [`ExecutionLocusChip`] record that compactly communicates execution locus
//! to the user.
//!
//! The module exposes:
//!
//! - the [`ExecutionLocusChip`] record that carries chip class, state, target
//!   name, tooltip, and boundary-cue visibility;
//! - the [`NotebookHeaderKernelBarState`] composed record that binds the
//!   notebook header block, kernel-selection state, kernel origin, available
//!   actions, last-successful-run summary, execution-locus chips, and
//!   paired-export posture into one UI-consumable state;
//! - the [`NotebookHeaderKernelBarPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`ExecutionLocusChip`] payloads.
pub const EXECUTION_LOCUS_CHIP_RECORD_KIND: &str = "notebook_execution_locus_chip";

/// Stable record-kind tag for serialized [`NotebookHeaderKernelBarState`] payloads.
pub const NOTEBOOK_HEADER_KERNEL_BAR_STATE_RECORD_KIND: &str = "notebook_header_kernel_bar_state";

/// Stable record-kind tag for the checked-in [`NotebookHeaderKernelBarPacket`].
pub const NOTEBOOK_HEADER_KERNEL_BAR_PACKET_RECORD_KIND: &str = "notebook_header_kernel_bar_packet";

/// Repo-relative path to the checked-in header-kernel-bar packet JSON.
pub const NOTEBOOK_HEADER_KERNEL_BAR_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.json";

/// Embedded checked-in header-kernel-bar packet JSON.
pub const NOTEBOOK_HEADER_KERNEL_BAR_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Execution-locus chip class. Mirrors the host-identity vocabulary from
    /// the UX spec so the chip never invents ad hoc locus labels.
    ExecutionLocusChipClass {
        LocalHost => "local_host",
        LocalContainer => "local_container",
        SshRemote => "ssh_remote",
        ManagedWorkspace => "managed_workspace",
        BrowserBridge => "browser_bridge",
        ServicePlane => "service_plane",
        NoKernel => "no_kernel",
    }
);

impl ExecutionLocusChipClass {
    /// True for any chip class that denotes execution off the local host.
    /// The header MUST render the local-vs-remote boundary cue whenever this
    /// returns true.
    pub const fn is_remote_boundary(self) -> bool {
        matches!(
            self,
            Self::SshRemote | Self::ManagedWorkspace | Self::BrowserBridge | Self::ServicePlane
        )
    }

    /// True when there is no resolvable execution locus.
    pub const fn is_no_kernel(self) -> bool {
        matches!(self, Self::NoKernel)
    }
}

closed_vocab!(
    /// Execution-locus chip state. Distinguishes active, degraded,
    /// disconnected, reconnecting, and policy-blocked states so the user
    /// never mistakes a disconnected kernel for an active one.
    ExecutionLocusChipState {
        Active => "active",
        Degraded => "degraded",
        Disconnected => "disconnected",
        Reconnecting => "reconnecting",
        PolicyBlocked => "policy_blocked",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeaderKernelBarFinding {
    /// Stable check id (e.g. `execution_locus_chip.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, chip id, state id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl HeaderKernelBarFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for an [`ExecutionLocusChip`].
pub type ExecutionLocusChipFinding = HeaderKernelBarFinding;

/// Typed validation finding for a [`NotebookHeaderKernelBarState`].
pub type NotebookHeaderKernelBarStateFinding = HeaderKernelBarFinding;

/// Typed validation finding for a [`NotebookHeaderKernelBarPacket`].
pub type NotebookHeaderKernelBarPacketFinding = HeaderKernelBarFinding;

/// Canonical execution-locus chip record. Compact label that tells the user
/// where notebook code executes without exposing raw host details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLocusChip {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_header_kernel_bar_schema_version: u32,
    /// Stable opaque chip id.
    pub chip_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Execution-locus chip class.
    pub chip_class: ExecutionLocusChipClass,
    /// Execution-locus chip state.
    pub chip_state: ExecutionLocusChipState,
    /// Export-safe target name label (e.g. `devcontainer:ml-cuda`).
    pub target_name_label: String,
    /// Export-safe tooltip label.
    pub tooltip_label: String,
    /// Whether the local-vs-remote boundary cue is visible for this chip.
    pub boundary_cue_visible: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl ExecutionLocusChip {
    /// Returns typed truth-rule findings; an empty vector means the chip is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<ExecutionLocusChipFinding> {
        let mut findings = Vec::new();
        let subject = self.chip_id.as_str();

        if self.record_kind != EXECUTION_LOCUS_CHIP_RECORD_KIND {
            findings.push(ExecutionLocusChipFinding::new(
                "execution_locus_chip.record_kind",
                subject,
                format!(
                    "record_kind must be '{EXECUTION_LOCUS_CHIP_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_header_kernel_bar_schema_version
            != NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION
        {
            findings.push(ExecutionLocusChipFinding::new(
                "execution_locus_chip.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION}, found {}",
                    self.notebook_header_kernel_bar_schema_version
                ),
            ));
        }

        if self.chip_class.is_remote_boundary() && !self.boundary_cue_visible {
            findings.push(ExecutionLocusChipFinding::new(
                "execution_locus_chip.remote_boundary_cue",
                subject,
                "remote chip classes must render the boundary cue",
            ));
        }
        if !self.chip_class.is_remote_boundary() && self.boundary_cue_visible {
            findings.push(ExecutionLocusChipFinding::new(
                "execution_locus_chip.local_no_boundary_cue",
                subject,
                "local chip classes must not render the boundary cue",
            ));
        }

        if self.chip_class.is_no_kernel()
            && matches!(self.chip_state, ExecutionLocusChipState::Active)
        {
            findings.push(ExecutionLocusChipFinding::new(
                "execution_locus_chip.no_kernel_active",
                subject,
                "no_kernel chip class must not report active state",
            ));
        }

        findings
    }
}

/// Composed notebook header / kernel bar / execution-locus / paired-export
/// state record. This is the UI-consumable surface that backs the notebook
/// chrome header and kernel bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookHeaderKernelBarState {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_header_kernel_bar_schema_version: u32,
    /// Stable opaque state id.
    pub state_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque VFS path-identity token for the document.
    pub document_path_token_ref: String,
    /// Export-safe notebook title for chrome / audits / exports.
    pub document_title_label: String,
    /// Document trust class projected for the header chip.
    pub document_trust_class: crate::NotebookDocumentTrustClass,
    /// Header-visible dirty-state class.
    pub dirty_state_class: crate::NotebookDirtyStateClass,
    /// Kernel-selection state shown on the kernel bar.
    pub kernel_selection_state: crate::KernelSelectionState,
    /// Execution-origin class for the selected kernel.
    pub kernel_origin_class: crate::KernelOriginClass,
    /// Opaque kernel-session id; null when no kernel is selected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Opaque kernelspec id; null when no kernel is selected or no
    /// kernelspec is resolvable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernelspec_ref: Option<String>,
    /// Whether the row renders the local-vs-remote boundary cue. MUST be
    /// `true` whenever `kernel_origin_class.is_remote_boundary()` returns
    /// `true`.
    pub local_vs_remote_boundary_cue_visible: bool,
    /// Opaque target-identity witness ref; required for any remote kernel
    /// origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_identity_witness_ref: Option<String>,
    /// Opaque remote-agent session id ref; required for any remote kernel
    /// origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_agent_session_id_ref: Option<String>,
    /// Opaque execution-context root ref for the kernel session, when one
    /// is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_root_ref: Option<String>,
    /// Execution-locus chips rendered in the header / kernel bar.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub execution_locus_chips: Vec<ExecutionLocusChip>,
    /// Paired-text export posture for the header chip.
    pub paired_export_posture: crate::NotebookPairedExportPosture,
    /// Opaque paired-export ref; non-null only when the posture is a
    /// derived class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paired_export_ref: Option<String>,
    /// Kernel-bar actions exposed to the user. Always at least
    /// `select_kernel` or `change_kernel`.
    pub available_actions: Vec<crate::KernelBarActionClass>,
    /// Last-successful-run summary; absent when no successful run has been
    /// observed for this notebook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_run: Option<crate::NotebookLastSuccessfulRunSummary>,
    /// Whether auto-rerun is forbidden after any restart, reconnect, or
    /// no-kernel transition. MUST be `true`; the field exists to make the
    /// invariant explicit in the record.
    pub auto_rerun_forbidden: bool,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookHeaderKernelBarState {
    /// Returns typed truth-rule findings; an empty vector means the state is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookHeaderKernelBarStateFinding> {
        let mut findings = Vec::new();
        let subject = self.state_id.as_str();

        if self.record_kind != NOTEBOOK_HEADER_KERNEL_BAR_STATE_RECORD_KIND {
            findings.push(NotebookHeaderKernelBarStateFinding::new(
                "notebook_header_kernel_bar_state.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_HEADER_KERNEL_BAR_STATE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_header_kernel_bar_schema_version
            != NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION
        {
            findings.push(NotebookHeaderKernelBarStateFinding::new(
                "notebook_header_kernel_bar_state.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION}, found {}",
                    self.notebook_header_kernel_bar_schema_version
                ),
            ));
        }

        if !self.auto_rerun_forbidden {
            findings.push(NotebookHeaderKernelBarStateFinding::new(
                "notebook_header_kernel_bar_state.auto_rerun_forbidden",
                subject,
                "auto_rerun_forbidden must be true for every retained header-kernel-bar state",
            ));
        }

        if self.kernel_origin_class.is_remote_boundary() {
            if !self.local_vs_remote_boundary_cue_visible {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.remote_boundary_cue",
                    subject,
                    "remote kernel origins must render the local-vs-remote boundary cue",
                ));
            }
            if self.target_identity_witness_ref.is_none() {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.target_identity_witness_required",
                    subject,
                    "remote kernel origins must carry a target_identity_witness_ref",
                ));
            }
            if self.remote_agent_session_id_ref.is_none() {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.remote_agent_session_id_required",
                    subject,
                    "remote kernel origins must carry a remote_agent_session_id_ref",
                ));
            }
        } else {
            if self.target_identity_witness_ref.is_some() {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.local_no_target_identity_witness",
                    subject,
                    "local kernel origins must not carry a target_identity_witness_ref",
                ));
            }
            if self.remote_agent_session_id_ref.is_some() {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.local_no_remote_agent_session_id",
                    subject,
                    "local kernel origins must not carry a remote_agent_session_id_ref",
                ));
            }
        }

        if self.kernel_origin_class.is_no_kernel() {
            if self.kernel_session_id_ref.is_some() {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.no_kernel_session_id",
                    subject,
                    "no_kernel origins must not carry a kernel_session_id_ref",
                ));
            }
            let has_select = self
                .available_actions
                .iter()
                .any(|action| matches!(action, crate::KernelBarActionClass::SelectKernel));
            if !has_select {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.no_kernel_actions",
                    subject,
                    "no_kernel origins must expose the select_kernel action",
                ));
            }
            let claims_running_actions = self.available_actions.iter().any(|action| {
                matches!(
                    action,
                    crate::KernelBarActionClass::Interrupt | crate::KernelBarActionClass::Restart
                )
            });
            if claims_running_actions {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.no_kernel_actions_running",
                    subject,
                    "no_kernel origins must not expose restart or interrupt actions",
                ));
            }
            if matches!(
                self.kernel_selection_state,
                crate::KernelSelectionState::SelectedKernelResolved
            ) {
                findings.push(NotebookHeaderKernelBarStateFinding::new(
                    "notebook_header_kernel_bar_state.no_kernel_selection_mismatch",
                    subject,
                    "no_kernel origins must not declare selected_kernel_resolved",
                ));
            }
        }

        if matches!(
            self.kernel_selection_state,
            crate::KernelSelectionState::SelectedKernelResolved
        ) && self.kernel_origin_class.is_no_kernel()
        {
            findings.push(NotebookHeaderKernelBarStateFinding::new(
                "notebook_header_kernel_bar_state.selection_resolved_requires_kernel_origin",
                subject,
                "selected_kernel_resolved requires a non-no_kernel execution origin",
            ));
        }

        match self.paired_export_posture {
            crate::NotebookPairedExportPosture::NotApplicable => {
                if self.paired_export_ref.is_some() {
                    findings.push(NotebookHeaderKernelBarStateFinding::new(
                        "notebook_header_kernel_bar_state.paired_export_ref_not_applicable",
                        subject,
                        "paired_text_export_not_applicable must not carry a paired_export_ref",
                    ));
                }
            }
            crate::NotebookPairedExportPosture::DerivedNotebookToScript
            | crate::NotebookPairedExportPosture::DerivedNotebookToMarkdown => {
                if self.paired_export_ref.is_none() {
                    findings.push(NotebookHeaderKernelBarStateFinding::new(
                        "notebook_header_kernel_bar_state.paired_export_ref_required",
                        subject,
                        "derived paired-export postures must carry a paired_export_ref",
                    ));
                }
            }
        }

        if self.available_actions.is_empty() {
            findings.push(NotebookHeaderKernelBarStateFinding::new(
                "notebook_header_kernel_bar_state.available_actions_required",
                subject,
                "kernel bar must expose at least one action",
            ));
        }

        for chip in &self.execution_locus_chips {
            findings.extend(chip.validate().into_iter().map(|f| {
                NotebookHeaderKernelBarStateFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Checked-in header-kernel-bar packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookHeaderKernelBarPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: execution-locus chip classes.
    pub execution_locus_chip_classes: Vec<ExecutionLocusChipClass>,
    /// Closed vocabulary: execution-locus chip states.
    pub execution_locus_chip_states: Vec<ExecutionLocusChipState>,
    /// Worked example execution-locus chips.
    pub example_execution_locus_chips: Vec<ExecutionLocusChip>,
    /// Worked example header-kernel-bar states.
    pub example_header_kernel_bar_states: Vec<NotebookHeaderKernelBarState>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookHeaderKernelBarPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookHeaderKernelBarPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION {
            findings.push(NotebookHeaderKernelBarPacketFinding::new(
                "notebook_header_kernel_bar_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_HEADER_KERNEL_BAR_PACKET_RECORD_KIND {
            findings.push(NotebookHeaderKernelBarPacketFinding::new(
                "notebook_header_kernel_bar_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_HEADER_KERNEL_BAR_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.execution_locus_chip_classes.len() != ExecutionLocusChipClass::ALL.len() {
            findings.push(NotebookHeaderKernelBarPacketFinding::new(
                "notebook_header_kernel_bar_packet.chip_classes_coverage",
                subject,
                "execution_locus_chip_classes must list every variant",
            ));
        }
        if self.execution_locus_chip_states.len() != ExecutionLocusChipState::ALL.len() {
            findings.push(NotebookHeaderKernelBarPacketFinding::new(
                "notebook_header_kernel_bar_packet.chip_states_coverage",
                subject,
                "execution_locus_chip_states must list every variant",
            ));
        }

        for chip in &self.example_execution_locus_chips {
            findings.extend(chip.validate().into_iter().map(|f| {
                NotebookHeaderKernelBarPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for state in &self.example_header_kernel_bar_states {
            findings.extend(state.validate().into_iter().map(|f| {
                NotebookHeaderKernelBarPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl ExecutionLocusChipClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalHost,
        Self::LocalContainer,
        Self::SshRemote,
        Self::ManagedWorkspace,
        Self::BrowserBridge,
        Self::ServicePlane,
        Self::NoKernel,
    ];
}

impl ExecutionLocusChipState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Active,
        Self::Degraded,
        Self::Disconnected,
        Self::Reconnecting,
        Self::PolicyBlocked,
    ];
}

/// Parses the checked-in header-kernel-bar packet JSON.
pub fn current_notebook_header_kernel_bar_packet(
) -> Result<NotebookHeaderKernelBarPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_HEADER_KERNEL_BAR_PACKET_JSON)
}

#[cfg(test)]
mod tests;
