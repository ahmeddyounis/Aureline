//! Canonical seed builders for the frozen M5 shared-component-state matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical shared-component-state matrix.
pub const M5_SHARED_COMPONENT_STATE_MATRIX_PACKET_ID: &str = "m5-shared-state-taxonomy:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every contract must be able to show.
fn mandatory_labels() -> Vec<M5ComponentStateRequiredLabel> {
    M5ComponentStateRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a contract carries.
fn labels_with(extra: &[M5ComponentStateRequiredLabel]) -> Vec<M5ComponentStateRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every contract filled in and every family-specific
/// vocabulary left empty for the caller to populate. The governed state subset is seeded
/// from the family's canonical partition.
fn base_row(
    component_family: M5SharedComponentStateFamily,
    qualification: M5ComponentStateQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5SharedComponentStateRow {
    M5SharedComponentStateRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComponentStateSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComponentStateDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        state_classes: component_family.governed_states().to_vec(),
        precedence_rules: vec![],
        disclosure_triggers: vec![],
        interaction_input_routes: vec![],
        lock_owner_classes: vec![],
        recovery_disclosure_classes: vec![],
        state_cause_classes: vec![],
        accessibility_routes: M5ComponentStateAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ComponentStateConsumerSurface::DesignSystemUi,
            M5ComponentStateConsumerSurface::SupportExport,
            M5ComponentStateConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5ComponentStateDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        collapses_current_and_selected: false,
        masks_lock_behind_disabled: false,
        presents_pending_as_generic_loading: false,
        omits_consequence_or_recovery_on_degraded: false,
    }
}

fn component_rows() -> Vec<M5SharedComponentStateRow> {
    use M5ComponentStateConsumerSurface as C;
    use M5ComponentStateDowngradeTrigger as D;
    use M5ComponentStateQualificationClass as Q;
    use M5ComponentStateRequiredLabel as L;
    use M5InteractionInputRoute as IR;
    use M5LockOwnerClass as LO;
    use M5RecoveryDisclosureClass as RD;
    use M5SharedComponentStateFamily as F;
    use M5StateCauseClass as SC;
    use M5StateDisclosureTrigger as DT;
    use M5StatePrecedenceRule as PR;

    let mut rows = Vec::new();

    // 1. Shared component-state taxonomy.
    let mut row = base_row(
        F::SharedComponentStateTaxonomy,
        Q::Stable,
        "Shared component-state taxonomy owner",
        "One shared component-state taxonomy naming the thirteen canonical states — default, hover, focus-visible, pressed/active, selected, current, disabled, read-only, loading, pending, warning/error, locked, and degraded — and freezing the precedence and distinctness rules (locked-over-disabled, read-only-over-disabled, current-vs-selected, pending-vs-loading) so every surface maps its local state machine back to one vocabulary and publishes cause, owner, block reason, or recovery instead of a silent style-only change",
        "evidence:m5-shared-component-state-taxonomy-parity:001",
        &[
            M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
            M5_SHARED_COMPONENT_STATE_COMPONENT_CONTRACT_REF,
        ],
    );
    row.precedence_rules = PR::ALL.to_vec();
    row.disclosure_triggers = DT::ALL.to_vec();
    row.required_labels = labels_with(&[L::StateCause, L::OwnerOrBlockReason, L::RecoveryAction]);
    row.consumer_surfaces = vec![
        C::DesignSystemUi,
        C::ShellUi,
        C::HelpUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PrecedenceRuleUnstated,
        D::DisclosureRequirementUnmet,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Interactive-state contract.
    let mut row = base_row(
        F::InteractiveState,
        Q::Stable,
        "Interactive-state contract owner",
        "One interactive-state contract naming the default, hover, focus-visible, and pressed/active states and the non-visual input routes each must be reachable and announced through, so no interactive state is hover-only, pointer-only, or encoded by color alone and focus stays visible for keyboard and assistive-tech operators",
        "evidence:m5-interactive-state-parity:001",
        &[
            M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
            M5_SHARED_COMPONENT_STATE_FOCUS_SELECTION_REF,
        ],
    );
    row.interaction_input_routes = IR::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::DesignSystemUi,
        C::ShellUi,
        C::CommandUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::KeyboardRouteMissing,
        D::ColorOnlyTreatment,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Selection-or-lock-state contract.
    let mut row = base_row(
        F::SelectionOrLockState,
        Q::Stable,
        "Selection-or-lock-state contract owner",
        "One selection-or-lock-state contract naming the selected, current, disabled, read-only, and locked states, who holds a lock (policy, trust, permission, ownership, source, or no lock), and why a state applies, so a disabled control never hides an explainable lock, a read-only control stays inspectable, and current and selected never collapse into one another",
        "evidence:m5-selection-or-lock-state-parity:001",
        &[
            M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
            M5_SHARED_COMPONENT_STATE_OPERATIONAL_STATE_REF,
        ],
    );
    row.lock_owner_classes = LO::ALL.to_vec();
    row.state_cause_classes = SC::ALL.to_vec();
    row.required_labels = labels_with(&[L::StateCause, L::OwnerOrBlockReason]);
    row.consumer_surfaces = vec![
        C::DesignSystemUi,
        C::ShellUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LockOwnerMasked,
        D::CurrentSelectedCollapsed,
        D::ReadOnlyInspectabilityLost,
        D::StateCauseUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Degraded-state-application contract.
    let mut row = base_row(
        F::DegradedStateApplication,
        Q::Stable,
        "Degraded-state-application contract owner",
        "One degraded-state-application contract naming the loading, pending, warning/error, and degraded states, what each degraded, warning, or error state must disclose (consequence, recovery action, freshness, retry path, fallback scope, or that no recovery is available), and why the state applies, so pending never masquerades as generic loading and a degraded, warning, or error surface always names its consequence and its recovery",
        "evidence:m5-degraded-state-application-parity:001",
        &[
            M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
            M5_SHARED_COMPONENT_STATE_RECOVERY_REF,
        ],
    );
    row.recovery_disclosure_classes = RD::ALL.to_vec();
    row.state_cause_classes = SC::ALL.to_vec();
    row.required_labels = labels_with(&[L::StateCause, L::RecoveryAction]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::CommandUi,
        C::HelpUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PendingShownAsLoading,
        D::ConsequenceOrRecoveryOmitted,
        D::StateCauseUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5SharedComponentStateGovernanceReview {
    M5SharedComponentStateGovernanceReview {
        taxonomy_names_all_thirteen_states: true,
        precedence_rules_named_once: true,
        disabled_never_hides_explainable_lock: true,
        read_only_preserves_inspectability: true,
        current_and_selected_never_collapse: true,
        pending_never_shown_as_generic_loading: true,
        degraded_warning_error_names_consequence_and_recovery: true,
        state_cause_owner_or_block_reason_always_disclosed: true,
        no_state_is_color_only: true,
        every_state_is_keyboard_visible: true,
        every_state_is_screen_reader_explainable: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        no_surface_invents_private_state_name: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5SharedComponentStateConsumerProjection {
    M5SharedComponentStateConsumerProjection {
        controls_consume_interactive_state_vocabulary: true,
        collections_consume_selection_lock_vocabulary: true,
        prompts_consume_state_cause_vocabulary: true,
        recovery_surfaces_consume_degraded_vocabulary: true,
        shell_status_progress_consume_shared_taxonomy: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5SharedComponentStateProofFreshness {
    M5SharedComponentStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SharedComponentStateReleasePosture {
    M5SharedComponentStateReleasePosture {
        proof_packet_ref: M5_SHARED_COMPONENT_STATE_ARTIFACT_REF.to_owned(),
        state_taxonomy_audit_ref: M5_SHARED_COMPONENT_STATE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
        M5_SHARED_COMPONENT_STATE_DOC_REF,
        M5_SHARED_COMPONENT_STATE_STATE_CLASS_REF,
        M5_SHARED_COMPONENT_STATE_RECOVERY_REF,
        M5_SHARED_COMPONENT_STATE_COMPONENT_CONTRACT_REF,
        M5_SHARED_COMPONENT_STATE_FOCUS_SELECTION_REF,
        M5_SHARED_COMPONENT_STATE_OPERATIONAL_STATE_REF,
    ])
}

/// Builds the canonical frozen M5 shared-component-state matrix packet.
pub fn seeded_m5_shared_component_state_matrix() -> M5SharedComponentStateMatrixPacket {
    M5SharedComponentStateMatrixPacket::new(M5SharedComponentStateMatrixPacketInput {
        packet_id: M5_SHARED_COMPONENT_STATE_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 shared-component-state-taxonomy, interactive-state, selection-or-lock-state, and degraded-state-application component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5SharedComponentStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the interactive-state contract is held at Beta because a slice of the
/// focus-visible ring does not yet round-trip across every host surface; every contract
/// stays visible.
pub fn seeded_m5_shared_component_state_matrix_interactive_state_beta_narrowed(
) -> M5SharedComponentStateMatrixPacket {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.packet_id = "m5-shared-state-taxonomy:interactive-state-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5SharedComponentStateFamily::InteractiveState)
        .expect("interactive-state row present");
    row.qualification = M5ComponentStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the degraded-state-application contract is narrowed to Preview pending
/// consequence-and-recovery parity proof across every recovery surface; every contract stays
/// visible.
pub fn seeded_m5_shared_component_state_matrix_degraded_state_application_preview_narrowed(
) -> M5SharedComponentStateMatrixPacket {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.packet_id =
        "m5-shared-state-taxonomy:degraded-state-application-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5SharedComponentStateFamily::DegradedStateApplication)
        .expect("degraded-state-application row present");
    row.qualification = M5ComponentStateQualificationClass::Preview;
    packet
}
