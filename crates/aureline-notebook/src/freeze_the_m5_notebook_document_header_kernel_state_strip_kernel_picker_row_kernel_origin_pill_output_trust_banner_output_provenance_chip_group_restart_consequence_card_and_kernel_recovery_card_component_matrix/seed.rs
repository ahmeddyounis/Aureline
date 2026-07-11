//! Canonical seed builders for the frozen M5 notebook-kernel-output component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical notebook-kernel-output component matrix.
pub const M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-notebook-kernel-output-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5NotebookKernelOutputRequiredLabel> {
    M5NotebookKernelOutputRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(
    extra: &[M5NotebookKernelOutputRequiredLabel],
) -> Vec<M5NotebookKernelOutputRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5NotebookKernelOutputComponentFamily,
    qualification: M5NotebookKernelOutputQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5NotebookKernelOutputComponentRow {
    M5NotebookKernelOutputComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: vec![],
        document_source_classes: vec![],
        document_identity_states: vec![],
        kernel_execution_states: vec![],
        kernel_connection_states: vec![],
        kernel_candidate_kinds: vec![],
        kernel_selection_states: vec![],
        kernel_origin_classes: vec![],
        kernel_origin_trust_states: vec![],
        output_trust_classes: vec![],
        output_freshness_states: vec![],
        output_provenance_kinds: vec![],
        output_provenance_states: vec![],
        restart_action_classes: vec![],
        restart_consequence_states: vec![],
        kernel_recovery_action_classes: vec![],
        kernel_recovery_states: vec![],
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5NotebookKernelOutputConsumerSurface::NotebookUi,
            M5NotebookKernelOutputConsumerSurface::SupportExport,
            M5NotebookKernelOutputConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5NotebookKernelOutputDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        recovery_card_implies_rerun: false,
        presents_stale_output_as_live: false,
        hides_trust_class_behind_hover_only: false,
        collapses_kernel_origins_into_one_badge: false,
    }
}

fn component_rows() -> Vec<M5NotebookKernelOutputComponentRow> {
    use M5KernelCandidateKind as KC;
    use M5KernelConnectionState as KN;
    use M5KernelExecutionState as KE;
    use M5KernelOriginClass as KO;
    use M5KernelOriginTrustState as KT;
    use M5KernelRecoveryActionClass as RA;
    use M5KernelRecoveryState as RV;
    use M5KernelSelectionState as KS;
    use M5NotebookDocumentIdentityState as DI;
    use M5NotebookDocumentSourceClass as DS;
    use M5NotebookKernelOutputComponentFamily as F;
    use M5NotebookKernelOutputConsumerSurface as C;
    use M5NotebookKernelOutputDisposition as P;
    use M5NotebookKernelOutputDowngradeTrigger as D;
    use M5NotebookKernelOutputQualificationClass as Q;
    use M5NotebookKernelOutputRequiredLabel as L;
    use M5OutputFreshnessState as OF;
    use M5OutputProvenanceKind as PK;
    use M5OutputProvenanceState as PS;
    use M5OutputTrustClass as OT;
    use M5RestartActionClass as XA;
    use M5RestartConsequenceState as XC;

    let mut rows = Vec::new();

    // 1. Notebook document header.
    let mut row = base_row(
        F::NotebookDocumentHeader,
        Q::Stable,
        "Notebook document header owner",
        "One notebook-document-header model naming where a notebook came from (a local, remote, managed-workspace, imported, scratch/untitled, or unknown-source .ipynb) and where its canonical identity stands (saved clean, unsaved changes, autosaved, conflicted, read-only, or recovered), so a header never leaves a notebook's canonical .ipynb identity or its local / remote / managed source implicit",
        "evidence:m5-notebook-document-header-parity:001",
        &[
            M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
            M5_NOTEBOOK_DOCUMENT_HEADER_SCHEMA_REF,
        ],
    );
    row.dispositions = vec![P::Managed, P::Remote];
    row.document_source_classes = DS::ALL.to_vec();
    row.document_identity_states = DI::ALL.to_vec();
    row.required_labels = labels_with(&[L::KernelOriginAndClass]);
    row.consumer_surfaces = vec![
        C::NotebookUi,
        C::KernelManagerUi,
        C::ReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DocumentIdentityUnstated,
        D::KernelOriginUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Kernel state strip.
    let mut row = base_row(
        F::KernelStateStrip,
        Q::Stable,
        "Kernel state strip owner",
        "One kernel-state-strip model naming where a kernel stands in execution (idle ready, queued pending, busy running, interrupted, dead / no kernel, or disconnected / reconnecting) and how it is connected (connected local, connected remote, reconnecting, disconnected, connection lost, or never connected), so a strip never leaves no-kernel, busy, disconnected, or reconnecting execution state implicit",
        "evidence:m5-kernel-state-strip-parity:001",
        &[
            M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
            M5_KERNEL_STATE_STRIP_SCHEMA_REF,
        ],
    );
    row.dispositions = vec![P::NoKernel, P::Queued, P::Busy, P::Ready, P::Disconnected];
    row.kernel_execution_states = KE::ALL.to_vec();
    row.kernel_connection_states = KN::ALL.to_vec();
    row.required_labels = labels_with(&[L::KernelOriginAndClass]);
    row.consumer_surfaces = vec![
        C::NotebookUi,
        C::KernelManagerUi,
        C::DebuggerUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::KernelOriginUnstated,
        D::ReconnectShownAsFresh,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Kernel picker row.
    let mut row = base_row(
        F::KernelPickerRow,
        Q::Stable,
        "Kernel picker row owner",
        "One kernel-picker-row model naming what kind of kernel a candidate is (a local interpreter, a virtual env, a conda env, a container kernel, a remote kernel, or a managed kernel) and where its selection stands (selected, available, recommended, incompatible, unavailable, or needs install), so a picker never collapses kernel kinds into one badge and always offers choose-another-kernel recovery",
        "evidence:m5-kernel-picker-row-parity:001",
        &[
            M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
            M5_KERNEL_PICKER_ROW_SCHEMA_REF,
        ],
    );
    row.dispositions = vec![P::ChooseAnotherKernel, P::Ready];
    row.kernel_candidate_kinds = KC::ALL.to_vec();
    row.kernel_selection_states = KS::ALL.to_vec();
    row.required_labels = labels_with(&[L::KernelOriginAndClass]);
    row.consumer_surfaces = vec![
        C::KernelManagerUi,
        C::NotebookUi,
        C::CliSurface,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::KernelClassCollapsed,
        D::KernelOriginUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Kernel origin pill.
    let mut row = base_row(
        F::KernelOriginPill,
        Q::Stable,
        "Kernel origin pill owner",
        "One kernel-origin-pill model naming where a kernel physically runs (a local host, an SSH remote, a container, a devcontainer, a managed workspace, or a browser bridge) and how trusted that origin is (trusted, first-party, third-party, unverified, restricted, or unknown), so a pill never collapses local, SSH, container, managed, or browser-bridge kernels into one unlabeled badge",
        "evidence:m5-kernel-origin-pill-parity:001",
        &[
            M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
            M5_KERNEL_ORIGIN_PILL_SCHEMA_REF,
        ],
    );
    row.dispositions = vec![P::Managed, P::Remote];
    row.kernel_origin_classes = KO::ALL.to_vec();
    row.kernel_origin_trust_states = KT::ALL.to_vec();
    row.required_labels = labels_with(&[L::KernelOriginAndClass]);
    row.consumer_surfaces = vec![
        C::NotebookUi,
        C::KernelManagerUi,
        C::AiContextUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::KernelClassCollapsed,
        D::KernelOriginUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Output trust banner.
    let mut row = base_row(
        F::OutputTrustBanner,
        Q::Stable,
        "Output trust banner owner",
        "One output-trust-banner model naming an output's trust class (trusted, sanitized, sandboxed, raw / active, blocked, or unknown) and its freshness (live, stale, cached, cleared, superseded, or no output), so a banner never presents stale output as live and never hides its raw / sanitized / active trust class behind a hover-only affordance",
        "evidence:m5-output-trust-banner-parity:001",
        &[
            M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
            M5_OUTPUT_TRUST_BANNER_SCHEMA_REF,
        ],
    );
    row.dispositions = vec![P::StaleOutput, P::Sanitized, P::Active];
    row.output_trust_classes = OT::ALL.to_vec();
    row.output_freshness_states = OF::ALL.to_vec();
    row.required_labels = labels_with(&[L::OutputTrustAndFreshness]);
    row.consumer_surfaces = vec![
        C::OutputViewerUi,
        C::NotebookUi,
        C::AiContextUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StaleOutputShownAsLive,
        D::TrustClassHoverOnly,
        D::OutputTrustUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Output provenance chip group.
    let mut row = base_row(
        F::OutputProvenanceChipGroup,
        Q::Stable,
        "Output provenance chip group owner",
        "One output-provenance-chip-group model naming what produced an output (a cell, a run, an imported output, a restored output, an external output, or an unknown provenance) and how completely its execution lineage resolves (provenance complete, partial, missing, execution count pinned, execution count drifted, or provenance stale), so a chip group never severs an output's canonical provenance or hides a drifted execution count",
        "evidence:m5-output-provenance-chip-group-parity:001",
        &[
            M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
            M5_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF,
        ],
    );
    row.dispositions = vec![P::Active, P::StaleOutput];
    row.output_provenance_kinds = PK::ALL.to_vec();
    row.output_provenance_states = PS::ALL.to_vec();
    row.required_labels = labels_with(&[L::OutputTrustAndFreshness]);
    row.consumer_surfaces = vec![
        C::OutputViewerUi,
        C::NotebookUi,
        C::ReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ProvenanceSevered,
        D::StaleOutputShownAsLive,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Restart consequence card.
    let mut row = base_row(
        F::RestartConsequenceCard,
        Q::Stable,
        "Restart consequence card owner",
        "One restart-consequence-card model naming which restart / interrupt action it describes (restart kernel, restart and run all, interrupt kernel, shutdown kernel, reconnect kernel, or clear outputs) and what survives it (state preserved, state lost, variables cleared, outputs retained, outputs cleared, or no consequence), so a card never leaves restart / reconnect consequences or preserved-versus-lost state implicit",
        "evidence:m5-restart-consequence-card-parity:001",
        &[
            M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
            M5_RESTART_CONSEQUENCE_CARD_SCHEMA_REF,
        ],
    );
    row.dispositions = vec![P::RestartClean, P::Reconnect];
    row.restart_action_classes = XA::ALL.to_vec();
    row.restart_consequence_states = XC::ALL.to_vec();
    row.required_labels = labels_with(&[L::RestartAndRecovery]);
    row.consumer_surfaces = vec![
        C::NotebookUi,
        C::KernelManagerUi,
        C::DebuggerUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RestartConsequenceImpliedRerun,
        D::ReconnectShownAsFresh,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Kernel recovery card.
    let mut row = base_row(
        F::KernelRecoveryCard,
        Q::Stable,
        "Kernel recovery card owner",
        "One kernel-recovery-card model naming which recovery action it offers (reconnect, restart clean, choose another kernel, reattach session, start local fallback, or wait for managed) and where recovery stands (recoverable, reconnect available, restart required, no kernel available, recovery blocked, or recovered), so a card offers reconnect / restart-clean / choose-another-kernel recovery without ever implying a rerun",
        "evidence:m5-kernel-recovery-card-parity:001",
        &[
            M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
            M5_KERNEL_RECOVERY_CARD_SCHEMA_REF,
        ],
    );
    row.dispositions = vec![P::Reconnect, P::RestartClean, P::ChooseAnotherKernel];
    row.kernel_recovery_action_classes = RA::ALL.to_vec();
    row.kernel_recovery_states = RV::ALL.to_vec();
    row.required_labels = labels_with(&[L::RestartAndRecovery]);
    row.consumer_surfaces = vec![
        C::NotebookUi,
        C::KernelManagerUi,
        C::DebuggerUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RecoveryOverclaimed,
        D::RestartConsequenceImpliedRerun,
        D::KernelOriginUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5NotebookKernelOutputComponentGovernanceReview {
    M5NotebookKernelOutputComponentGovernanceReview {
        document_header_shows_identity_and_source: true,
        kernel_state_strip_shows_execution_and_connection: true,
        kernel_picker_row_shows_candidates_and_selection: true,
        kernel_origin_pill_shows_origin_and_class: true,
        output_trust_banner_shows_trust_and_freshness: true,
        output_provenance_chip_group_shows_provenance: true,
        restart_consequence_card_shows_preserved_and_lost: true,
        kernel_recovery_card_shows_recovery_without_implying_rerun: true,
        no_surface_invents_alternate_state_label: true,
        stale_output_never_presented_as_live: true,
        trust_class_never_hover_only: true,
        kernel_origins_never_collapsed_into_one_badge: true,
        kernel_origin_and_class_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5NotebookKernelOutputComponentConsumerProjection {
    M5NotebookKernelOutputComponentConsumerProjection {
        notebook_surfaces_consume_document_and_kernel_vocabulary: true,
        kernel_surfaces_consume_origin_and_recovery_vocabulary: true,
        output_surfaces_consume_trust_and_provenance_vocabulary: true,
        debug_surfaces_consume_restart_and_consequence_vocabulary: true,
        recovery_surfaces_consume_recovery_and_reconnect_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5NotebookKernelOutputComponentProofFreshness {
    M5NotebookKernelOutputComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5NotebookKernelOutputComponentReleasePosture {
    M5NotebookKernelOutputComponentReleasePosture {
        proof_packet_ref: M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_ARTIFACT_REF.to_owned(),
        notebook_component_audit_ref: M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
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
    ])
}

/// Builds the canonical frozen M5 notebook-kernel-output component matrix packet.
pub fn seeded_m5_notebook_kernel_output_component_matrix(
) -> M5NotebookKernelOutputComponentMatrixPacket {
    M5NotebookKernelOutputComponentMatrixPacket::new(M5NotebookKernelOutputComponentMatrixPacketInput {
        packet_id: M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 notebook-document-header, kernel-state-strip, kernel-picker-row, kernel-origin-pill, output-trust-banner, output-provenance-chip-group, restart-consequence-card, and kernel-recovery-card component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5NotebookKernelOutputComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the kernel recovery card is held at Beta because recovery parity for a
/// slice of the choose-another-kernel and reattach flows does not yet round-trip across every
/// notebook surface; every component stays visible.
pub fn seeded_m5_notebook_kernel_output_component_matrix_kernel_recovery_card_beta_narrowed(
) -> M5NotebookKernelOutputComponentMatrixPacket {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.packet_id =
        "m5-notebook-kernel-output-components:kernel-recovery-card-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5NotebookKernelOutputComponentFamily::KernelRecoveryCard
        })
        .expect("kernel-recovery-card row present");
    row.qualification = M5NotebookKernelOutputQualificationClass::Beta;
    packet
}

/// Narrowed variant: the output trust banner is narrowed to Preview pending stale-versus-live
/// and trust-class parity proof across every output surface; every component stays visible.
pub fn seeded_m5_notebook_kernel_output_component_matrix_output_trust_banner_preview_narrowed(
) -> M5NotebookKernelOutputComponentMatrixPacket {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.packet_id =
        "m5-notebook-kernel-output-components:output-trust-banner-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5NotebookKernelOutputComponentFamily::OutputTrustBanner
        })
        .expect("output-trust-banner row present");
    row.qualification = M5NotebookKernelOutputQualificationClass::Preview;
    packet
}
