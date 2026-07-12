//! Canonical seed builders for the frozen M5 core-action-input component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical core-action-input component matrix.
pub const M5_CORE_CONTROL_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-core-action-input-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every control must be able to show.
fn mandatory_labels() -> Vec<M5CoreControlRequiredLabel> {
    M5CoreControlRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a control carries.
fn labels_with(extra: &[M5CoreControlRequiredLabel]) -> Vec<M5CoreControlRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every control filled in and every family-specific vocabulary
/// left empty for the caller to populate.
fn base_row(
    component_family: M5CoreControlFamily,
    qualification: M5CoreControlQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5CoreControlComponentRow {
    M5CoreControlComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5CoreControlSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CoreControlDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: vec![],
        button_emphases: vec![],
        icon_label_modes: vec![],
        split_postures: vec![],
        field_label_modes: vec![],
        field_validations: vec![],
        search_affordances: vec![],
        combobox_value_sources: vec![],
        toggle_semantics: vec![],
        segmented_modes: vec![],
        degraded_reasons: M5CoreControlDegradedReason::ALL.to_vec(),
        accessibility_routes: M5CoreControlAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5CoreControlConsumerSurface::SupportExport,
            M5CoreControlConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5CoreControlDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        lets_placeholder_text_replace_the_label: false,
        lets_loading_relabel_the_action_or_lose_attribution: false,
        leaves_icon_only_destructive_action_unlabeled: false,
        blurs_switch_with_deferred_checkbox: false,
        lets_split_button_default_to_riskier_alternate: false,
        hides_locked_or_degraded_semantics_behind_generic_disabled: false,
    }
}

fn component_rows() -> Vec<M5CoreControlComponentRow> {
    use M5ButtonEmphasis as BE;
    use M5ComboboxValueSource as CV;
    use M5CoreControlConsumerSurface as C;
    use M5CoreControlDisposition as ST;
    use M5CoreControlDowngradeTrigger as D;
    use M5CoreControlFamily as F;
    use M5CoreControlQualificationClass as Q;
    use M5CoreControlRequiredLabel as L;
    use M5FieldLabelMode as FL;
    use M5FieldValidationState as FV;
    use M5IconLabelMode as IL;
    use M5SearchFieldAffordance as SA;
    use M5SegmentedMode as SM;
    use M5SplitDefaultPosture as SP;
    use M5ToggleSemantics as TG;

    let mut rows = Vec::new();

    // 1. Button.
    let mut row = base_row(
        F::Button,
        Q::Stable,
        "Design-system control owner",
        "One text-button model naming a permanent label, a stable emphasis (primary, secondary, quiet, destructive, ghost, link), and the shared interaction state (default, hover, focus-visible, pressed, loading, disabled), so a loading button never relabels the action or loses attribution and emphasis is never encoded by color alone",
        "evidence:m5-button-parity:001",
        &[M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_BUTTON_SCHEMA_REF],
    );
    row.button_emphases = BE::ALL.to_vec();
    row.dispositions = vec![
        ST::Default,
        ST::Hover,
        ST::FocusVisible,
        ST::Pressed,
        ST::Loading,
        ST::Disabled,
    ];
    row.required_labels = labels_with(&[L::CommandBinding]);
    row.consumer_surfaces = vec![
        C::FormsUi,
        C::SettingsUi,
        C::EntryUi,
        C::ReviewUi,
        C::RepairUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LoadingRelabeledOrResized,
        D::CommandBindingUnstated,
        D::StateTaxonomyDrifted,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Icon button.
    let mut row = base_row(
        F::IconButton,
        Q::Stable,
        "Design-system control owner",
        "One icon-button model naming how an icon-only control carries its accessible name (labeled-visible, accessible-name-only, tooltip-labeled, text-with-icon) and its emphasis, so an icon-only destructive action is never left unlabeled and a decorative glyph is never mistaken for a control",
        "evidence:m5-icon-button-parity:001",
        &[M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_ICON_BUTTON_SCHEMA_REF],
    );
    row.icon_label_modes = IL::ALL.to_vec();
    row.button_emphases = vec![BE::Primary, BE::Secondary, BE::Quiet, BE::Destructive];
    row.dispositions = vec![ST::Default, ST::FocusVisible, ST::Pressed, ST::Disabled];
    row.required_labels = labels_with(&[L::CommandBinding]);
    row.consumer_surfaces = vec![
        C::FormsUi,
        C::SettingsUi,
        C::ReviewUi,
        C::RepairUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::IconOnlyDestructiveUnlabeled,
        D::CommandBindingUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Split button.
    let mut row = base_row(
        F::SplitButton,
        Q::Stable,
        "Design-system control owner",
        "One split-button model naming the default (primary-click) posture (primary-default-safe, explicit-alternate, confirm-required, destructive-guarded, all-disabled) and the emphasis, so a split button never defaults to a riskier alternate and a guarded destructive alternate stays behind a distinct labeled step",
        "evidence:m5-split-button-parity:001",
        &[M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_SPLIT_BUTTON_SCHEMA_REF],
    );
    row.split_postures = SP::ALL.to_vec();
    row.button_emphases = vec![BE::Primary, BE::Secondary, BE::Destructive];
    row.dispositions = vec![ST::Default, ST::Pressed, ST::Loading, ST::Disabled];
    row.required_labels = labels_with(&[L::CommandBinding]);
    row.consumer_surfaces = vec![
        C::FormsUi,
        C::ReviewUi,
        C::RepairUi,
        C::EntryUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SplitDefaultedToRiskierAlternate,
        D::CommandBindingUnstated,
        D::LoadingRelabeledOrResized,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Text field.
    let mut row = base_row(
        F::TextField,
        Q::Stable,
        "Forms surface owner",
        "One text-field model naming a permanent label mode (persistent, floating, label-plus-placeholder, aria-label-only) and the validation truth (valid, invalid-blocking, warning, pending-async, not-validated), so placeholder text never replaces the label and an invalid or unvalidated value never reads as valid",
        "evidence:m5-text-field-parity:001",
        &[M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_TEXT_FIELD_SCHEMA_REF],
    );
    row.field_label_modes = FL::ALL.to_vec();
    row.field_validations = FV::ALL.to_vec();
    row.dispositions = vec![
        ST::Default,
        ST::FocusVisible,
        ST::Disabled,
        ST::ReadOnly,
        ST::Degraded,
    ];
    row.required_labels = labels_with(&[L::ValueSource, L::ValidationAndConstraints]);
    row.consumer_surfaces = vec![
        C::FormsUi,
        C::SettingsUi,
        C::EntryUi,
        C::ReviewUi,
        C::RepairUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PlaceholderUsedAsLabel,
        D::ValidationStateUnstated,
        D::LockedOrDegradedHiddenBehindDisabled,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Search field.
    let mut row = base_row(
        F::SearchField,
        Q::Stable,
        "Search surface owner",
        "One search-field model naming a permanent label mode and the clear / submit / privacy affordances (clearable, submit-explicit, submit-as-you-type, history-private, scoped-search), so a search field never hides whether it clears, how it submits, or whether its history is private",
        "evidence:m5-search-field-parity:001",
        &[M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_SEARCH_FIELD_SCHEMA_REF],
    );
    row.field_label_modes = vec![FL::PersistentLabel, FL::FloatingLabel, FL::AriaLabelOnly];
    row.field_validations = vec![FV::Valid, FV::InvalidBlocking, FV::NotValidated];
    row.search_affordances = SA::ALL.to_vec();
    row.dispositions = vec![ST::Default, ST::FocusVisible, ST::Loading, ST::Disabled];
    row.required_labels = labels_with(&[L::ValueSource]);
    row.consumer_surfaces = vec![
        C::SearchUi,
        C::EntryUi,
        C::ReviewUi,
        C::RepairUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PlaceholderUsedAsLabel,
        D::ValueSourceUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Combobox.
    let mut row = base_row(
        F::Combobox,
        Q::Stable,
        "Forms surface owner",
        "One combobox model naming a permanent label mode, the validation truth, and the source of the committed value (canonical-option, filtered-subset, free-text-allowed, remote-backed, custom-unverified), so a free-text or unverified value is never presented as a canonical option and filterability stays honest",
        "evidence:m5-combobox-parity:001",
        &[M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_COMBOBOX_SCHEMA_REF],
    );
    row.field_label_modes = vec![
        FL::PersistentLabel,
        FL::FloatingLabel,
        FL::LabelPlusPlaceholder,
    ];
    row.field_validations = vec![
        FV::Valid,
        FV::InvalidBlocking,
        FV::PendingAsync,
        FV::NotValidated,
    ];
    row.combobox_value_sources = CV::ALL.to_vec();
    row.dispositions = vec![
        ST::Default,
        ST::FocusVisible,
        ST::Loading,
        ST::Disabled,
        ST::ReadOnly,
    ];
    row.required_labels = labels_with(&[L::ValueSource, L::ValidationAndConstraints]);
    row.consumer_surfaces = vec![
        C::FormsUi,
        C::SettingsUi,
        C::SearchUi,
        C::EntryUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ValueSourceUnstated,
        D::ValidationStateUnstated,
        D::PlaceholderUsedAsLabel,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Toggle control (checkbox / radio / switch).
    let mut row = base_row(
        F::ToggleControl,
        Q::Stable,
        "Design-system control owner",
        "One checkbox / radio / switch model naming which boolean control it actually is (checkbox-immediate, checkbox-deferred, radio-exclusive, switch-immediate, tristate-indeterminate) and whether its change is immediate or deferred, so a switch is never blurred with a deferred checkbox and a radio's exclusivity is never lost",
        "evidence:m5-toggle-control-parity:001",
        &[M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_TOGGLE_CONTROL_SCHEMA_REF],
    );
    row.toggle_semantics = TG::ALL.to_vec();
    row.dispositions = vec![
        ST::Default,
        ST::FocusVisible,
        ST::Pressed,
        ST::Disabled,
        ST::Locked,
    ];
    row.required_labels = labels_with(&[L::CommandBinding, L::ValueSource]);
    row.consumer_surfaces = vec![
        C::SettingsUi,
        C::FormsUi,
        C::EntryUi,
        C::ReviewUi,
        C::RepairUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SwitchAndDeferredCheckboxBlurred,
        D::LockedOrDegradedHiddenBehindDisabled,
        D::CommandBindingUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Segmented control.
    let mut row = base_row(
        F::SegmentedControl,
        Q::Stable,
        "Design-system control owner",
        "One segmented-control model naming what it does (mode-toggle, view-switch, single-select-small-set, exclusive-options, not-navigation), so a segmented control stays a small mode / view toggle and is never used as stealth top-level navigation",
        "evidence:m5-segmented-control-parity:001",
        &[M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_SEGMENTED_CONTROL_SCHEMA_REF],
    );
    row.segmented_modes = SM::ALL.to_vec();
    row.dispositions = vec![
        ST::Default,
        ST::Hover,
        ST::FocusVisible,
        ST::Pressed,
        ST::Disabled,
    ];
    row.required_labels = labels_with(&[L::CommandBinding]);
    row.consumer_surfaces = vec![
        C::ReviewUi,
        C::SettingsUi,
        C::SearchUi,
        C::EntryUi,
        C::RepairUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StateTaxonomyDrifted,
        D::CommandBindingUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5CoreControlGovernanceReview {
    M5CoreControlGovernanceReview {
        button_states_stay_semantically_stable: true,
        icon_button_never_unlabeled_when_destructive: true,
        split_button_default_stays_safe: true,
        text_field_labels_are_permanent_not_placeholder_only: true,
        search_field_preserves_clear_submit_and_privacy_truth: true,
        combobox_preserves_filterability_and_value_source_truth: true,
        toggle_control_semantics_stay_distinct: true,
        segmented_control_stays_mode_toggle_not_navigation: true,
        state_taxonomy_means_the_same_everywhere: true,
        loading_never_relabels_or_loses_attribution: true,
        placeholder_never_replaces_label: true,
        locked_and_degraded_never_hidden_behind_disabled: true,
        every_control_binds_to_one_command_or_value_source: true,
        every_control_declares_deployment_lines: true,
        every_control_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_control_vocabulary: true,
    }
}

fn consumer_projection() -> M5CoreControlConsumerProjection {
    M5CoreControlConsumerProjection {
        forms_and_settings_consume_shared_control_vocabulary: true,
        search_and_entry_consume_shared_field_vocabulary: true,
        review_consumes_shared_action_and_value_vocabulary: true,
        repair_consumes_shared_control_vocabulary: true,
        boolean_controls_consume_shared_toggle_vocabulary: true,
        support_export_reads_single_control_source: true,
    }
}

fn proof_freshness() -> M5CoreControlProofFreshness {
    M5CoreControlProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CoreControlReleasePosture {
    M5CoreControlReleasePosture {
        proof_packet_ref: M5_CORE_CONTROL_COMPONENT_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_CORE_CONTROL_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_BUTTON_SCHEMA_REF,
        M5_ICON_BUTTON_SCHEMA_REF,
        M5_SPLIT_BUTTON_SCHEMA_REF,
        M5_TEXT_FIELD_SCHEMA_REF,
        M5_SEARCH_FIELD_SCHEMA_REF,
        M5_COMBOBOX_SCHEMA_REF,
        M5_TOGGLE_CONTROL_SCHEMA_REF,
        M5_SEGMENTED_CONTROL_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 core-action-input component matrix packet.
pub fn seeded_m5_core_action_input_component_matrix() -> M5CoreControlComponentMatrixPacket {
    M5CoreControlComponentMatrixPacket::new(M5CoreControlComponentMatrixPacketInput {
        packet_id: M5_CORE_CONTROL_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 button, icon-button, split-button, text-field, search-field, combobox, toggle-control, and segmented-control component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5CoreControlVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the combobox is held at Beta because remote-backed value round-trips are not yet
/// proven across every deployment line; every control stays visible.
pub fn seeded_m5_core_action_input_component_matrix_combobox_beta_narrowed(
) -> M5CoreControlComponentMatrixPacket {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.packet_id = "m5-core-action-input-components:combobox-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::Combobox)
        .expect("combobox row present");
    row.qualification = M5CoreControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the segmented control is narrowed to Preview pending mode-versus-view parity across
/// every deployment line; every control stays visible.
pub fn seeded_m5_core_action_input_component_matrix_segmented_control_preview_narrowed(
) -> M5CoreControlComponentMatrixPacket {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.packet_id = "m5-core-action-input-components:segmented-control-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::SegmentedControl)
        .expect("segmented-control row present");
    row.qualification = M5CoreControlQualificationClass::Preview;
    packet
}
