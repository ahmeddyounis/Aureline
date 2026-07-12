//! Canonical seed builders for the M5 combobox / checkbox-radio-switch controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean
//! comboboxes and toggles are built so the shared value-source, filterability, immediate-versus-
//! deferred, provenance-disclosure, and locked / read-only grammar is proven across the claimed M5
//! settings, provider, admin, request, and entry surfaces without any undisclosed value, unresolved
//! or undisclosed source, unverified-untagged value, blurred switch, ambiguous arity, hidden lock, or
//! broken command trace.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_PACKET_ID: &str =
    "m5-combobox-checkbox-radio-switch-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn combobox(input: M5ComboboxResolutionInput) -> M5ResolvedCombobox {
    resolve_combobox(input).expect("seed combobox input resolves")
}

fn toggle(input: M5ToggleResolutionInput) -> M5ResolvedToggle {
    resolve_toggle(input).expect("seed toggle input resolves")
}

// -- Clean combobox examples (value-source + provenance honesty across states) ------------------

#[allow(clippy::too_many_arguments)]
fn clean_combobox_base(
    combobox_id: &str,
    label: &str,
    selected_value: &str,
    value_source: M5ComboboxValueSource,
    value_provenance: M5ControlValueProvenance,
    disposition: M5CoreControlDisposition,
    surface: M5ControlSurfaceContext,
    command_id: &str,
) -> M5ComboboxResolutionInput {
    M5ComboboxResolutionInput {
        combobox_id: combobox_id.to_owned(),
        label: label.to_owned(),
        selected_value: selected_value.to_owned(),
        selected_value_disclosed: true,
        value_source,
        support_class_tag: String::new(),
        support_class_tagged: true,
        requires_filter: false,
        filter_offered: true,
        value_provenance,
        provenance_disclosed: true,
        keyboard_navigation_stable: true,
        disposition,
        blocked_state_distinct: true,
        surface_context: surface,
        command_id: command_id.to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

/// Clean combobox with a canonical option chosen and a user-override provenance.
fn combobox_canonical_clean() -> M5ResolvedCombobox {
    combobox(clean_combobox_base(
        "combobox:forms:tier",
        "Plan tier",
        "Standard tier",
        M5ComboboxValueSource::CanonicalOption,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::SettingsRow,
        "command:forms.plan_tier",
    ))
}

/// Clean combobox with a filtered subset and a disclosed detected provenance.
fn combobox_filtered_clean() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:settings:model",
        "Model",
        "Fast preview model",
        M5ComboboxValueSource::FilteredSubset,
        M5ControlValueProvenance::Detected,
        M5CoreControlDisposition::FocusVisible,
        M5ControlSurfaceContext::ProviderRow,
        "command:settings.model",
    );
    input.requires_filter = true;
    input.filter_offered = true;
    combobox(input)
}

/// Clean combobox with the applied default and no disclosure obligation.
fn combobox_default_clean() -> M5ResolvedCombobox {
    combobox(clean_combobox_base(
        "combobox:entry:region",
        "Region",
        "Auto (nearest)",
        M5ComboboxValueSource::CanonicalOption,
        M5ControlValueProvenance::DefaultApplied,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::EntryField,
        "command:entry.region",
    ))
}

/// Clean remote-backed combobox that carries its support-class tag and discloses imported provenance.
fn combobox_remote_tagged_clean() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:review:catalog",
        "Catalog entry",
        "Imported entry A",
        M5ComboboxValueSource::RemoteBacked,
        M5ControlValueProvenance::Imported,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::AdminRow,
        "command:review.catalog",
    );
    input.support_class_tag = "Remote catalog".to_owned();
    input.support_class_tagged = true;
    combobox(input)
}

/// Clean policy-locked combobox kept distinct from generic disabled chrome.
fn combobox_policy_locked_clean() -> M5ResolvedCombobox {
    combobox(clean_combobox_base(
        "combobox:support:retention",
        "Retention window",
        "90 days (policy)",
        M5ComboboxValueSource::CanonicalOption,
        M5ControlValueProvenance::PolicyEnforced,
        M5CoreControlDisposition::Locked,
        M5ControlSurfaceContext::AdminRow,
        "command:support.retention",
    ))
}

/// Clean product combobox reusing the filterable-set grammar with a user-override provenance.
fn combobox_product_clean() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:product:sort",
        "Sort by",
        "Recently updated",
        M5ComboboxValueSource::FilteredSubset,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::RequestFlow,
        "command:product.sort",
    );
    input.requires_filter = true;
    input.filter_offered = true;
    combobox(input)
}

// -- Degraded combobox examples ----------------------------------------------------------------

/// Degraded combobox: the value source is unresolved.
fn combobox_value_source_unresolved() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:forms:source-unknown",
        "Plan tier",
        "Standard tier",
        M5ComboboxValueSource::SourceUnknown,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::SettingsRow,
        "command:forms.plan_tier",
    );
    input.value_source = M5ComboboxValueSource::SourceUnknown;
    combobox(input)
}

/// Degraded combobox: a claimed filterable set does not offer filtering.
fn combobox_filterability_missing() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:settings:no-filter",
        "Model",
        "Fast preview model",
        M5ComboboxValueSource::FilteredSubset,
        M5ControlValueProvenance::Detected,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::ProviderRow,
        "command:settings.model",
    );
    input.requires_filter = true;
    input.filter_offered = false;
    combobox(input)
}

/// Degraded combobox: a policy provenance that materially changes trust is left undisclosed.
fn combobox_provenance_undisclosed() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:entry:provenance-hidden",
        "Region",
        "eu-west (policy)",
        M5ComboboxValueSource::CanonicalOption,
        M5ControlValueProvenance::PolicyEnforced,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::EntryField,
        "command:entry.region",
    );
    input.provenance_disclosed = false;
    combobox(input)
}

/// Degraded combobox: an unverified value is presented as a canonical option without a support-class tag.
fn combobox_unverified_untagged() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:review:unverified",
        "Catalog entry",
        "custom-value",
        M5ComboboxValueSource::CustomUnverified,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::AdminRow,
        "command:review.catalog",
    );
    input.support_class_tag = String::new();
    input.support_class_tagged = false;
    combobox(input)
}

/// Degraded combobox: keyboard / screen-reader navigation is unstable.
fn combobox_keyboard_unstable() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:support:keyboard-unstable",
        "Retention window",
        "30 days",
        M5ComboboxValueSource::CanonicalOption,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::AdminRow,
        "command:support.retention",
    );
    input.keyboard_navigation_stable = false;
    combobox(input)
}

/// Degraded combobox: no command-backed path to inspect the control is reachable.
fn combobox_trace_missing() -> M5ResolvedCombobox {
    let mut input = clean_combobox_base(
        "combobox:product:trace-missing",
        "Sort by",
        "Recently updated",
        M5ComboboxValueSource::FilteredSubset,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::RequestFlow,
        "command:product.sort",
    );
    input.requires_filter = true;
    input.filter_offered = true;
    input.command_route_available = false;
    combobox(input)
}

// -- Clean toggle examples (semantics + immediate-versus-deferred grammar) ----------------------

#[allow(clippy::too_many_arguments)]
fn clean_toggle_base(
    toggle_id: &str,
    label: &str,
    selected_state: &str,
    semantics: M5ToggleSemantics,
    timing: M5ToggleApplyTiming,
    value_provenance: M5ControlValueProvenance,
    disposition: M5CoreControlDisposition,
    surface: M5ControlSurfaceContext,
    command_id: &str,
) -> M5ToggleResolutionInput {
    M5ToggleResolutionInput {
        toggle_id: toggle_id.to_owned(),
        label: label.to_owned(),
        selected_state: selected_state.to_owned(),
        selected_state_disclosed: true,
        toggle_semantics: semantics,
        apply_timing: timing,
        selection_arity_explicit: true,
        group_exclusivity_enforced: true,
        value_provenance,
        provenance_disclosed: true,
        disposition,
        blocked_state_distinct: true,
        surface_context: surface,
        command_id: command_id.to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

/// Clean checkbox that applies immediately with a user-override provenance.
fn toggle_checkbox_immediate_clean() -> M5ResolvedToggle {
    toggle(clean_toggle_base(
        "toggle:forms:notify",
        "Email me on completion",
        "on",
        M5ToggleSemantics::CheckboxImmediate,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::SettingsRow,
        "command:forms.notify_on_completion",
    ))
}

/// Clean checkbox whose change is deferred until an explicit save.
fn toggle_checkbox_deferred_clean() -> M5ResolvedToggle {
    toggle(clean_toggle_base(
        "toggle:settings:experimental",
        "Enable experimental features",
        "off",
        M5ToggleSemantics::CheckboxDeferred,
        M5ToggleApplyTiming::DeferredUntilSave,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::SettingsRow,
        "command:settings.experimental",
    ))
}

/// Clean radio in an exclusive group that applies immediately with a default provenance.
fn toggle_radio_clean() -> M5ResolvedToggle {
    toggle(clean_toggle_base(
        "toggle:entry:theme",
        "Theme: System",
        "on",
        M5ToggleSemantics::RadioExclusive,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::DefaultApplied,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::EntryField,
        "command:entry.theme",
    ))
}

/// Clean switch that applies immediately with a disclosed detected provenance.
fn toggle_switch_immediate_clean() -> M5ResolvedToggle {
    toggle(clean_toggle_base(
        "toggle:review:streaming",
        "Stream responses",
        "on",
        M5ToggleSemantics::SwitchImmediate,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::Detected,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::ProviderRow,
        "command:review.streaming",
    ))
}

/// Clean policy-locked switch kept distinct from generic disabled chrome.
fn toggle_policy_locked_clean() -> M5ResolvedToggle {
    toggle(clean_toggle_base(
        "toggle:support:telemetry",
        "Share diagnostics",
        "off",
        M5ToggleSemantics::SwitchImmediate,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::PolicyEnforced,
        M5CoreControlDisposition::Locked,
        M5ControlSurfaceContext::AdminRow,
        "command:support.telemetry",
    ))
}

/// Clean tri-state checkbox whose change is deferred until save with a disclosed imported provenance.
fn toggle_tristate_clean() -> M5ResolvedToggle {
    toggle(clean_toggle_base(
        "toggle:product:select-all",
        "Select all items",
        "indeterminate",
        M5ToggleSemantics::TristateIndeterminate,
        M5ToggleApplyTiming::DeferredUntilSave,
        M5ControlValueProvenance::Imported,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::RequestFlow,
        "command:product.select_all",
    ))
}

// -- Degraded toggle examples ------------------------------------------------------------------

/// Degraded toggle: a switch is blurred with a deferred checkbox.
fn toggle_switch_deferred_blur() -> M5ResolvedToggle {
    let mut input = clean_toggle_base(
        "toggle:forms:switch-deferred",
        "Auto-save drafts",
        "on",
        M5ToggleSemantics::SwitchImmediate,
        M5ToggleApplyTiming::DeferredUntilSave,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::SettingsRow,
        "command:forms.auto_save",
    );
    input.apply_timing = M5ToggleApplyTiming::DeferredUntilSave;
    toggle(input)
}

/// Degraded toggle: a locked state hides behind generic disabled chrome.
fn toggle_locked_hidden() -> M5ResolvedToggle {
    let mut input = clean_toggle_base(
        "toggle:settings:locked-hidden",
        "Require review before apply",
        "on",
        M5ToggleSemantics::SwitchImmediate,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::PolicyEnforced,
        M5CoreControlDisposition::Locked,
        M5ControlSurfaceContext::SettingsRow,
        "command:settings.require_review",
    );
    input.blocked_state_distinct = false;
    toggle(input)
}

/// Degraded toggle: a radio group has lost its exclusivity.
fn toggle_exclusivity_lost() -> M5ResolvedToggle {
    let mut input = clean_toggle_base(
        "toggle:entry:exclusivity-lost",
        "Theme: Dark",
        "on",
        M5ToggleSemantics::RadioExclusive,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::DefaultApplied,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::EntryField,
        "command:entry.theme",
    );
    input.group_exclusivity_enforced = false;
    toggle(input)
}

/// Degraded toggle: the toggle semantics cannot be resolved.
fn toggle_semantics_unresolved() -> M5ResolvedToggle {
    let mut input = clean_toggle_base(
        "toggle:review:semantics-unknown",
        "Stream responses",
        "on",
        M5ToggleSemantics::SemanticsUnknown,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::Detected,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::ProviderRow,
        "command:review.streaming",
    );
    input.toggle_semantics = M5ToggleSemantics::SemanticsUnknown;
    toggle(input)
}

/// Degraded toggle: the selected on / off / indeterminate state is not disclosed.
fn toggle_state_unstated() -> M5ResolvedToggle {
    let mut input = clean_toggle_base(
        "toggle:support:state-unstated",
        "Share diagnostics",
        "off",
        M5ToggleSemantics::SwitchImmediate,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::PolicyEnforced,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::AdminRow,
        "command:support.telemetry",
    );
    input.selected_state_disclosed = false;
    toggle(input)
}

/// Degraded toggle: one-of-many versus multi-select behavior is ambiguous.
fn toggle_arity_ambiguous() -> M5ResolvedToggle {
    let mut input = clean_toggle_base(
        "toggle:product:arity-ambiguous",
        "Include archived",
        "off",
        M5ToggleSemantics::CheckboxImmediate,
        M5ToggleApplyTiming::AppliesImmediately,
        M5ControlValueProvenance::UserOverride,
        M5CoreControlDisposition::Default,
        M5ControlSurfaceContext::RequestFlow,
        "command:product.include_archived",
    );
    input.selection_arity_explicit = false;
    toggle(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ControlConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    combobox_examples: Vec<M5ResolvedCombobox>,
    toggle_examples: Vec<M5ResolvedToggle>,
) -> M5ComboboxToggleControlsRow {
    M5ComboboxToggleControlsRow {
        consumer_surface,
        qualification: M5CoreControlQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5CoreControlDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5CoreControlRequiredLabel::Identity,
            M5CoreControlRequiredLabel::State,
            M5CoreControlRequiredLabel::KeyboardRoute,
            M5CoreControlRequiredLabel::CommandBinding,
            M5CoreControlRequiredLabel::ValueSource,
        ],
        accessibility_routes: M5CoreControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ControlAnatomyPart::ALL.to_vec(),
        export_fields: M5ControlExportField::ALL.to_vec(),
        downgrade_triggers,
        combobox_examples,
        toggle_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_SCHEMA_REF,
            M5_COMBOBOX_SCHEMA_REF,
            M5_TOGGLE_CONTROL_SCHEMA_REF,
        ]),
        value_source_or_provenance_truth_dropped: false,
        switch_blurred_with_deferred_checkbox: false,
        one_of_many_versus_multi_select_blurred: false,
        locked_or_read_only_semantics_hidden_behind_disabled: false,
    }
}

fn controls_rows() -> Vec<M5ComboboxToggleControlsRow> {
    use M5CoreControlConsumerSurface as C;
    use M5CoreControlDowngradeTrigger as D;

    vec![
        base_row(
            C::FormsUi,
            "Forms surface owner",
            "The forms surface renders a combobox that discloses its canonical selected value and a checkbox that applies immediately; both degrade honestly when the value source is unresolved or a switch is blurred with a deferred checkbox",
            "evidence:m5-combobox-toggle-forms-ui:001",
            vec![
                D::ValueSourceUnstated,
                D::SwitchAndDeferredCheckboxBlurred,
                D::ProofStale,
            ],
            vec![combobox_canonical_clean(), combobox_value_source_unresolved()],
            vec![toggle_checkbox_immediate_clean(), toggle_switch_deferred_blur()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface keeps a filterable model combobox filterable and a deferred-until-save checkbox distinct from an immediate switch, and keeps a locked toggle distinct rather than behind generic disabled chrome; both degrade honestly when the filter is missing or a lock hides behind disabled",
            "evidence:m5-combobox-toggle-settings-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::LockedOrDegradedHiddenBehindDisabled,
                D::ProofStale,
            ],
            vec![combobox_filtered_clean(), combobox_filterability_missing()],
            vec![toggle_checkbox_deferred_clean(), toggle_locked_hidden()],
        ),
        base_row(
            C::EntryUi,
            "Start-center entry owner",
            "The start-center entry surface offers a default-provenance region combobox and an exclusive theme radio; both degrade honestly when a policy provenance is undisclosed or a radio group loses its exclusivity",
            "evidence:m5-combobox-toggle-entry-ui:001",
            vec![
                D::ValueSourceUnstated,
                D::StateTaxonomyDrifted,
                D::ProofStale,
            ],
            vec![combobox_default_clean(), combobox_provenance_undisclosed()],
            vec![toggle_radio_clean(), toggle_exclusivity_lost()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface tags a remote-backed catalog combobox with its support class and keeps a switch's immediate semantics explicit; both degrade honestly when an unverified value is presented as canonical without a tag or the toggle semantics are unresolved",
            "evidence:m5-combobox-toggle-review-ui:001",
            vec![
                D::ValueSourceUnstated,
                D::StateTaxonomyDrifted,
                D::ProofStale,
            ],
            vec![combobox_remote_tagged_clean(), combobox_unverified_untagged()],
            vec![toggle_switch_immediate_clean(), toggle_semantics_unresolved()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved value source, provenance, lock, and apply-timing truth, so an unstable-keyboard combobox or an undisclosed toggle state is visible in evidence rather than hidden behind generic disabled chrome",
            "evidence:m5-combobox-toggle-support-export:001",
            vec![
                D::GenericChromeWordingUsed,
                D::LockedOrDegradedHiddenBehindDisabled,
                D::ProofStale,
            ],
            vec![combobox_policy_locked_clean(), combobox_keyboard_unstable()],
            vec![toggle_policy_locked_clean(), toggle_state_unstated()],
        ),
        base_row(
            C::ProductUi,
            "In-product control owner",
            "In-product surfaces reuse the same filterable-set, provenance, and immediate-versus-deferred grammar a user sees in settings and entry, always offering the command-backed detail path and degrading honestly when the trace path is missing or one-of-many versus multi-select is ambiguous",
            "evidence:m5-combobox-toggle-product-ui:001",
            vec![
                D::CommandBindingUnstated,
                D::StateTaxonomyDrifted,
                D::ProofStale,
            ],
            vec![combobox_product_clean(), combobox_trace_missing()],
            vec![toggle_tristate_clean(), toggle_arity_ambiguous()],
        ),
    ]
}

fn governance_review() -> M5ComboboxToggleGovernanceReview {
    M5ComboboxToggleGovernanceReview {
        combobox_discloses_selected_value_and_source: true,
        combobox_keeps_filterable_and_keyboard_stable: true,
        combobox_never_presents_unverified_as_canonical: true,
        toggle_names_immediate_versus_deferred_timing: true,
        switch_never_blurred_with_deferred_checkbox: true,
        one_of_many_versus_multi_select_unambiguous: true,
        provenance_carried_not_feature_local: true,
        material_provenance_always_disclosed: true,
        locked_and_read_only_never_hidden_behind_disabled: true,
        both_bind_canonical_command_with_trace: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ComboboxToggleConsumerProjection {
    M5ComboboxToggleConsumerProjection {
        settings_surfaces_consume_combobox_and_toggle_vocabulary: true,
        provider_admin_surfaces_consume_value_source_vocabulary: true,
        request_entry_surfaces_consume_toggle_vocabulary: true,
        value_source_lock_and_timing_trace_to_single_component_contract: true,
        support_export_reads_single_control_source: true,
        support_export_reconstructs_selection_and_editability: true,
    }
}

fn proof_freshness() -> M5ComboboxToggleProofFreshness {
    M5ComboboxToggleProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ComboboxToggleReleasePosture {
    M5ComboboxToggleReleasePosture {
        proof_packet_ref: M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_SCHEMA_REF,
        M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_COMBOBOX_SCHEMA_REF,
        M5_TOGGLE_CONTROL_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 combobox / checkbox-radio-switch controls packet.
pub fn seeded_m5_combobox_checkbox_radio_switch_controls() -> M5ComboboxToggleControlsPacket {
    M5ComboboxToggleControlsPacket::new(M5ComboboxToggleControlsPacketInput {
        packet_id: M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 combobox and checkbox-radio-switch controls with filterable selection, selected-value and source-of-value disclosure, explicit immediate-versus-deferred toggle semantics, provenance carried across surfaces, and locked / read-only truth aligned across settings, provider, admin, request, and entry surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5ComboboxToggleVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the settings-UI row is held at Beta pending locked-toggle and filterable-set parity
/// on every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_combobox_checkbox_radio_switch_controls_settings_ui_beta_narrowed(
) -> M5ComboboxToggleControlsPacket {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.packet_id =
        "m5-combobox-checkbox-radio-switch-controls:settings-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoreControlConsumerSurface::SettingsUi)
        .expect("settings-ui row present");
    row.qualification = M5CoreControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the entry-UI row is narrowed to Preview pending provenance-disclosure and radio-
/// exclusivity parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_combobox_checkbox_radio_switch_controls_entry_ui_preview_narrowed(
) -> M5ComboboxToggleControlsPacket {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.packet_id =
        "m5-combobox-checkbox-radio-switch-controls:entry-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoreControlConsumerSurface::EntryUi)
        .expect("entry-ui row present");
    row.qualification = M5CoreControlQualificationClass::Preview;
    packet
}
