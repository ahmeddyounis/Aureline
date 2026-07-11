//! Canonical seed builders for the notebook-document-header / kernel-state-strip controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code components,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical notebook-document-header / kernel-state-strip packet.
pub const NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_PACKET_ID: &str =
    "m5-notebook-document-header-kernel-state-strip-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn header_source_refs() -> Vec<String> {
    strings(&[
        M5_NOTEBOOK_DOCUMENT_HEADER_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    ])
}

fn strip_source_refs() -> Vec<String> {
    strings(&[
        M5_KERNEL_STATE_STRIP_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    ])
}

fn header_downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::DocumentIdentityUnstated,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

fn strip_downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::KernelOriginUnstated,
        M5NotebookKernelOutputDowngradeTrigger::KernelClassCollapsed,
        M5NotebookKernelOutputDowngradeTrigger::ReconnectShownAsFresh,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

/// Builds a notebook document header, deriving the origin class, the canonical claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn header(
    header_id: &str,
    header_label: &str,
    source_class: M5NotebookDocumentSourceClass,
    identity_state: M5NotebookDocumentIdentityState,
    notebook_identity_label: &str,
    export_state_label: &str,
    target_context_label: &str,
    source_of_truth_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    header_actions: Vec<DocumentHeaderAction>,
    dispositions: Vec<M5NotebookKernelOutputDisposition>,
) -> NotebookDocumentHeader {
    let disclosure = resolve_document_header(source_class, identity_state);
    NotebookDocumentHeader {
        component: M5NotebookKernelOutputComponentFamily::NotebookDocumentHeader,
        header_id: header_id.to_owned(),
        header_label: header_label.to_owned(),
        source_class,
        identity_state,
        origin_class: disclosure.origin_class,
        claims_canonical_source: disclosure.is_canonical_source,
        imported_note: if disclosure.needs_imported_note {
            "Imported notebook; provenance is only as complete as the import".to_owned()
        } else {
            String::new()
        },
        scratch_note: if disclosure.needs_scratch_note {
            "Unsaved scratch notebook; it has no settled canonical file on disk yet".to_owned()
        } else {
            String::new()
        },
        unknown_source_note: if disclosure.needs_unknown_source_note {
            "Source could not be resolved; do not treat it as a settled canonical notebook"
                .to_owned()
        } else {
            String::new()
        },
        unsaved_note: if disclosure.needs_unsaved_note {
            "Unsaved changes; the on-disk notebook differs from what you see".to_owned()
        } else {
            String::new()
        },
        conflict_note: if disclosure.needs_conflict_note {
            "Conflicted; another writer changed this notebook, resolve before trusting it"
                .to_owned()
        } else {
            String::new()
        },
        readonly_note: if disclosure.needs_readonly_note {
            "Read-only; you can review and search but not edit this notebook".to_owned()
        } else {
            String::new()
        },
        recovered_note: if disclosure.needs_recovered_note {
            "Recovered from autosave; confirm it against the canonical notebook".to_owned()
        } else {
            String::new()
        },
        notebook_identity_label: notebook_identity_label.to_owned(),
        export_state_label: export_state_label.to_owned(),
        target_context_label: target_context_label.to_owned(),
        source_of_truth_note: source_of_truth_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        header_actions,
        dispositions,
        downgrade_triggers: header_downgrade_triggers(),
        required_labels: M5NotebookKernelOutputRequiredLabel::ALL.to_vec(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "notebook_identity_label",
            "source_class",
            "identity_state",
            "origin_class",
            "export_state_label",
            "target_context_label",
            "source_of_truth_note",
            "deep_link_kind",
        ]),
        source_contract_refs: header_source_refs(),
        pretends_kernel_free_is_live: false,
        collapses_kernel_origins_into_one_badge: false,
        conflates_document_and_runtime_truth: false,
        hides_state_behind_hover_only: false,
    }
}

/// Builds a kernel-state strip, deriving the live class, the live claim, and the required notes
/// from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn strip(
    strip_id: &str,
    strip_label: &str,
    execution_state: M5KernelExecutionState,
    connection_state: M5KernelConnectionState,
    kernel_origin_label: &str,
    kernel_state_summary: &str,
    execution_context_label: &str,
    kernel_free_edit_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    strip_actions: Vec<KernelStripAction>,
    dispositions: Vec<M5NotebookKernelOutputDisposition>,
) -> KernelStateStrip {
    let disclosure = resolve_kernel_state(execution_state, connection_state);
    KernelStateStrip {
        component: M5NotebookKernelOutputComponentFamily::KernelStateStrip,
        strip_id: strip_id.to_owned(),
        strip_label: strip_label.to_owned(),
        execution_state,
        connection_state,
        live_class: disclosure.live_class,
        claims_live: disclosure.is_live,
        no_kernel_note: if disclosure.needs_no_kernel_note {
            "No kernel attached; you can still edit, search, and review this notebook".to_owned()
        } else {
            String::new()
        },
        reconnect_note: if disclosure.needs_reconnect_note {
            "Kernel disconnected; reconnect to resume — any outputs shown are from before the drop"
                .to_owned()
        } else {
            String::new()
        },
        inspect_only_note: if disclosure.needs_inspect_only_note {
            "Kernel interrupted; state is inspect-only until you restart or continue".to_owned()
        } else {
            String::new()
        },
        kernel_origin_label: kernel_origin_label.to_owned(),
        kernel_state_summary: kernel_state_summary.to_owned(),
        execution_context_label: execution_context_label.to_owned(),
        kernel_free_edit_note: kernel_free_edit_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        strip_actions,
        dispositions,
        downgrade_triggers: strip_downgrade_triggers(),
        required_labels: M5NotebookKernelOutputRequiredLabel::ALL.to_vec(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "kernel_origin_label",
            "execution_state",
            "connection_state",
            "live_class",
            "kernel_state_summary",
            "execution_context_label",
            "kernel_free_edit_note",
            "deep_link_kind",
        ]),
        source_contract_refs: strip_source_refs(),
        pretends_kernel_free_is_live: false,
        collapses_kernel_origins_into_one_badge: false,
        conflates_document_and_runtime_truth: false,
        hides_state_behind_hover_only: false,
    }
}

fn document_headers() -> Vec<NotebookDocumentHeader> {
    use DeepLinkKind as Link;
    use DocumentHeaderAction as Action;
    use M5NotebookDocumentIdentityState as Id;
    use M5NotebookDocumentSourceClass as Src;
    use M5NotebookKernelOutputDisposition as Disp;

    vec![
        // 1. Local .ipynb, saved and clean → canonical local document.
        header(
            "hdr-local-analysis",
            "analysis.ipynb (local)",
            Src::LocalIpynb,
            Id::SavedClean,
            "analysis.ipynb",
            "Paired HTML export is up to date",
            "Workspace: research (local)",
            "Canonical source: on-disk notebook in this workspace",
            "Document truth: where this notebook came from and where its identity stands",
            Link::NotebookLocation,
            "notebook:research/analysis.ipynb",
            vec![
                Action::OpenDocument,
                Action::ExportDocument,
                Action::ReviewDocument,
                Action::OpenDeepLink,
                Action::InspectSource,
            ],
            vec![Disp::Active],
        ),
        // 2. Remote .ipynb, autosaved → canonical remote document.
        header(
            "hdr-remote-featurize",
            "featurize.ipynb (remote)",
            Src::RemoteIpynb,
            Id::Autosaved,
            "featurize.ipynb",
            "No paired export for this notebook",
            "Workspace: shared-remote (remote host)",
            "Canonical source: remote notebook on the connected host",
            "Document truth: this is the remote canonical notebook, not a local copy",
            Link::NotebookLocation,
            "notebook:shared-remote/featurize.ipynb",
            vec![
                Action::OpenDocument,
                Action::ExportDocument,
                Action::ReviewDocument,
                Action::OpenDeepLink,
            ],
            vec![Disp::Remote],
        ),
        // 3. Managed-workspace .ipynb, unsaved changes → canonical managed document (needs
        //    unsaved note).
        header(
            "hdr-managed-retrain",
            "retrain.ipynb (managed)",
            Src::ManagedWorkspaceIpynb,
            Id::UnsavedChanges,
            "retrain.ipynb",
            "Paired report export is stale until you save",
            "Workspace: team-managed (managed workspace)",
            "Canonical source: managed-workspace notebook Aureline hosts",
            "Document truth: managed notebook with unsaved edits still in the buffer",
            Link::KernelManager,
            "kernel:managed/team-managed",
            vec![
                Action::OpenDocument,
                Action::ExportDocument,
                Action::ReviewDocument,
                Action::CopyDocumentPath,
                Action::OpenDeepLink,
            ],
            vec![Disp::Managed],
        ),
        // 4. Imported .ipynb, read-only → imported document (needs imported + read-only notes).
        header(
            "hdr-imported-baseline",
            "baseline.ipynb (imported)",
            Src::ImportedIpynb,
            Id::ReadOnly,
            "baseline.ipynb",
            "No paired export for an imported notebook",
            "Workspace: research (local, imported copy)",
            "Canonical source: imported file; the origin notebook lives elsewhere",
            "Document truth: an imported, read-only notebook — review and search only",
            Link::DocsAnchor,
            "docs:notebooks/import-and-provenance",
            vec![
                Action::OpenDocument,
                Action::ExportDocument,
                Action::ReviewDocument,
                Action::OpenDeepLink,
            ],
            vec![Disp::Active],
        ),
        // 5. Scratch / untitled, conflicted → scratch document (needs scratch + conflict notes).
        header(
            "hdr-scratch-untitled",
            "Untitled scratch notebook",
            Src::ScratchUntitled,
            Id::Conflicted,
            "Untitled-3 (unsaved)",
            "No paired export until the notebook is saved",
            "Workspace: research (local, unsaved)",
            "Canonical source: none yet — this scratch notebook is not on disk",
            "Document truth: an unsaved scratch notebook with a conflicting concurrent edit",
            Link::SupportBundle,
            "support:bundle/scratch-notebook-state",
            vec![
                Action::OpenDocument,
                Action::ExportDocument,
                Action::ReviewDocument,
            ],
            vec![Disp::Active],
        ),
        // 6. Unknown source, recovered → unknown document (needs unknown + recovered notes).
        header(
            "hdr-unknown-recovered",
            "Recovered notebook (unknown source)",
            Src::UnknownSource,
            Id::Recovered,
            "recovered-session.ipynb",
            "No paired export for an unresolved notebook",
            "Workspace: research (local, recovered)",
            "Canonical source: unresolved — recovered from an autosave buffer",
            "Document truth: recovered notebook whose original source is unknown",
            Link::NoDeepLink,
            "",
            vec![
                Action::OpenDocument,
                Action::ExportDocument,
                Action::ReviewDocument,
            ],
            vec![Disp::Active],
        ),
    ]
}

fn kernel_strips() -> Vec<KernelStateStrip> {
    use DeepLinkKind as Link;
    use KernelStripAction as Action;
    use M5KernelConnectionState as Conn;
    use M5KernelExecutionState as Exec;
    use M5NotebookKernelOutputDisposition as Disp;

    vec![
        // 1. Idle + connected local → ready-live.
        strip(
            "strip-ready-local",
            "Kernel ready (local)",
            Exec::IdleReady,
            Conn::ConnectedLocal,
            "Local kernel: CPython 3.11 on this workstation",
            "Idle and ready to run",
            "Executes on this workstation",
            "Editing, search, and review work whether or not the kernel is busy",
            "Runtime truth: where the kernel stands, kept separate from the document",
            Link::KernelManager,
            "kernel:local/cpython-311",
            vec![
                Action::SelectKernel,
                Action::InspectKernel,
                Action::ContinueWithoutKernel,
                Action::RestartKernel,
                Action::OpenDeepLink,
            ],
            vec![Disp::Ready],
        ),
        // 2. Busy + connected remote → busy-live.
        strip(
            "strip-busy-remote",
            "Kernel busy (remote)",
            Exec::BusyRunning,
            Conn::ConnectedRemote,
            "Remote kernel: managed GPU pool",
            "Busy running a cell",
            "Executes on the connected remote host",
            "You can keep editing and searching while the remote kernel runs",
            "Runtime truth: a remote kernel is busy — outputs are still being produced",
            Link::KernelManager,
            "kernel:remote/managed-gpu",
            vec![
                Action::SelectKernel,
                Action::InspectKernel,
                Action::ContinueWithoutKernel,
                Action::OpenDeepLink,
            ],
            vec![Disp::Busy],
        ),
        // 3. Queued + reconnecting → queued-live.
        strip(
            "strip-queued-container",
            "Kernel queued (container)",
            Exec::QueuedPending,
            Conn::Reconnecting,
            "Container kernel: devcontainer image py311",
            "Queued / pending execution",
            "Executes inside the project container",
            "Editing and search stay available while work is queued",
            "Runtime truth: work is queued on a container kernel that is reattaching",
            Link::KernelManager,
            "kernel:container/devcontainer-py311",
            vec![
                Action::SelectKernel,
                Action::InspectKernel,
                Action::ContinueWithoutKernel,
                Action::ReconnectKernel,
            ],
            vec![Disp::Queued],
        ),
        // 4. Disconnected/reconnecting + disconnected → disconnected-recoverable (needs reconnect
        //    note).
        strip(
            "strip-disconnected-ssh",
            "Kernel disconnected (SSH)",
            Exec::DisconnectedReconnecting,
            Conn::Disconnected,
            "SSH kernel: remote workstation over SSH",
            "Disconnected; reconnect available",
            "Executes on the SSH-connected remote workstation",
            "You can still edit and search this notebook while the kernel is disconnected",
            "Runtime truth: the SSH kernel dropped — outputs are from before the disconnect",
            Link::KernelManager,
            "kernel:ssh/remote-workstation",
            vec![
                Action::SelectKernel,
                Action::InspectKernel,
                Action::ContinueWithoutKernel,
                Action::ReconnectKernel,
                Action::OpenDeepLink,
            ],
            vec![Disp::Disconnected, Disp::Reconnect],
        ),
        // 5. Interrupted + connection lost → inspect-only (needs inspect-only note).
        strip(
            "strip-inspect-only",
            "Kernel interrupted (inspect only)",
            Exec::Interrupted,
            Conn::ConnectionLost,
            "Local kernel: CPython 3.11 on this workstation",
            "Interrupted; inspect-only until restart",
            "Executes on this workstation",
            "Editing and search still work while the kernel is inspect-only",
            "Runtime truth: an interrupted kernel — inspect state, do not treat it as live",
            Link::DocsAnchor,
            "docs:notebooks/kernel-interrupt-and-restart",
            vec![
                Action::SelectKernel,
                Action::InspectKernel,
                Action::ContinueWithoutKernel,
                Action::RestartKernel,
                Action::OpenDeepLink,
            ],
            vec![Disp::Disconnected],
        ),
        // 6. Dead / no kernel + never connected → no-kernel-editable (needs no-kernel note).
        strip(
            "strip-no-kernel",
            "No kernel selected",
            Exec::DeadNoKernel,
            Conn::NeverConnected,
            "No kernel: none selected yet",
            "No kernel attached",
            "No execution target selected",
            "This notebook is fully editable, searchable, and reviewable without a kernel",
            "Runtime truth: no kernel — nothing is live, but the notebook is not blocked",
            Link::NoDeepLink,
            "",
            vec![
                Action::SelectKernel,
                Action::InspectKernel,
                Action::ContinueWithoutKernel,
            ],
            vec![Disp::NoKernel],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::DocumentIdentityUnstated,
        M5NotebookKernelOutputDowngradeTrigger::KernelOriginUnstated,
        M5NotebookKernelOutputDowngradeTrigger::KernelClassCollapsed,
        M5NotebookKernelOutputDowngradeTrigger::ReconnectShownAsFresh,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

fn notebook_review() -> NotebookDocumentKernelReview {
    NotebookDocumentKernelReview {
        header_shows_canonical_source: true,
        header_shows_document_identity: true,
        header_offers_open_export_review: true,
        strip_shows_kernel_execution_state: true,
        strip_shows_kernel_connection_state: true,
        strip_offers_select_inspect_continue: true,
        document_and_runtime_truth_distinct: true,
        kernel_free_notebook_stays_editable: true,
        kernel_free_never_shown_as_live: true,
        no_kernel_origins_collapsed_into_one_badge: true,
        source_and_kernel_state_derived_never_asserted: true,
        every_next_step_names_stable_deep_link: true,
        no_state_hidden_behind_hover_only: true,
        header_and_strip_consistent_across_surfaces: true,
        no_component_widens_export_scope_or_exposes_raw_by_default: true,
        components_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        no_surface_invents_alternate_state_label: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> NotebookDocumentKernelConsumerProjection {
    NotebookDocumentKernelConsumerProjection {
        notebook_edit_surface_reads_single_source: true,
        kernel_manager_surface_reads_single_source: true,
        document_truth_visible_before_run: true,
        kernel_state_visible_before_trusting_output: true,
        support_export_shows_component_truth: true,
        help_docs_shows_component_truth: true,
    }
}

fn proof_freshness() -> NotebookDocumentKernelProofFreshness {
    NotebookDocumentKernelProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_REF,
        NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_DOC_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_NOTEBOOK_DOCUMENT_HEADER_SCHEMA_REF,
        M5_KERNEL_STATE_STRIP_SCHEMA_REF,
    ])
}

/// Builds the canonical notebook-document-header / kernel-state-strip controls packet.
pub fn seeded_notebook_document_header_kernel_state_strip_controls(
) -> NotebookDocumentHeaderKernelStateStripControlsPacket {
    NotebookDocumentHeaderKernelStateStripControlsPacket::new(
        NotebookDocumentHeaderKernelStateStripControlsPacketInput {
            packet_id: NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_PACKET_ID.to_owned(),
            surface_label:
                "M5 notebook document headers and kernel-state strips: canonical .ipynb source, selected kernel origin, busy/queued/offline truth, and no-kernel edit parity across claimed notebook surfaces"
                    .to_owned(),
            document_headers: document_headers(),
            kernel_strips: kernel_strips(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
            notebook_review: notebook_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a scratch / untitled document header that must never read as a
/// settled canonical source. Every source class, identity state, and origin class stays covered so
/// the fixture validates on its own.
pub fn seeded_notebook_document_header_kernel_state_strip_controls_document_header_scratch(
) -> NotebookDocumentHeaderKernelStateStripControlsPacket {
    let mut packet = seeded_notebook_document_header_kernel_state_strip_controls();
    packet.packet_id =
        "m5-notebook-document-header-kernel-state-strip-controls:fixture:document-header-scratch"
            .to_owned();
    packet.surface_label =
        "M5 notebook document headers: a scratch notebook never reads as a settled canonical source"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a kernel-free kernel-state strip that must never read as live and
/// must keep the notebook editable. Every execution state, connection state, and live class stays
/// covered so the fixture validates on its own.
pub fn seeded_notebook_document_header_kernel_state_strip_controls_kernel_state_strip_no_kernel(
) -> NotebookDocumentHeaderKernelStateStripControlsPacket {
    let mut packet = seeded_notebook_document_header_kernel_state_strip_controls();
    packet.packet_id =
        "m5-notebook-document-header-kernel-state-strip-controls:fixture:kernel-state-strip-no-kernel"
            .to_owned();
    packet.surface_label =
        "M5 kernel-state strips: a kernel-free notebook never reads as live and stays editable"
            .to_owned();
    packet
}
