//! Canonical seed for the core-control shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export,
//! matrix CSV, Markdown summary, and narrowed fixtures. Every binding is derived from one
//! per-object [`CoreControlStateFacetValues`] so the same control object always carries the
//! same vocabulary across surfaces, and every narrowed representation derives its
//! disclosure from [`resolve_core_control_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    state: &str,
    command_binding: &str,
    value_source: &str,
    validation: &str,
    lock_policy: &str,
) -> CoreControlStateFacetValues {
    CoreControlStateFacetValues {
        state_word: state.to_owned(),
        command_binding_word: command_binding.to_owned(),
        value_source_word: value_source.to_owned(),
        validation_word: validation.to_owned(),
        lock_policy_word: lock_policy.to_owned(),
    }
}

fn preserved_note_for(reason: CoreControlNarrowReason) -> String {
    match reason {
        CoreControlNarrowReason::CompactionNarrowed => {
            "state, command, value-source, validation, and lock/policy words preserved; only disclosure depth compacted"
        }
        CoreControlNarrowReason::RemoteProjectionNarrowed => {
            "all control vocabulary preserved; the value is projected from the remote source of truth"
        }
        CoreControlNarrowReason::ExportRedactionNarrowed => {
            "all control vocabulary preserved; only value detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: CoreControlNarrowNextAction) -> String {
    match action {
        CoreControlNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        CoreControlNarrowNextAction::OpenRemoteSource => "Open the remote source",
        CoreControlNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(component: M5CoreControlFamily) -> Vec<String> {
    vec![
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF.to_owned(),
        component.canonical_component_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    control_object_id: &str,
    control_object_label: &str,
    component: M5CoreControlFamily,
    consumer: M5CoreControlConsumerSurface,
    representation: CoreControlRepresentation,
    state_facets: CoreControlStateFacetValues,
) -> CoreControlConsumerBinding {
    let disclosure = resolve_core_control_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        CoreControlNarrowNote {
            reason,
            preserved_vocabulary_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let remote_source_note = if disclosure.needs_remote_source_note {
        "projected from the remote source of truth; the source stays remote".to_owned()
    } else {
        String::new()
    };
    let export_detail_note = if disclosure.needs_export_detail_note {
        "value detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };

    CoreControlConsumerBinding {
        binding_id: binding_id.to_owned(),
        control_object_id: control_object_id.to_owned(),
        control_object_label: control_object_label.to_owned(),
        component,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        lets_placeholder_text_replace_the_label: false,
        lets_loading_relabel_the_action_or_lose_attribution: false,
        leaves_icon_only_destructive_action_unlabeled: false,
        blurs_switch_with_deferred_checkbox: false,
        lets_split_button_default_to_riskier_alternate: false,
        hides_locked_or_degraded_semantics_behind_generic_disabled: false,
        source_contract_refs: binding_refs(component),
    }
}

/// One consumer-surface adoption of a control object, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5CoreControlConsumerSurface,
    representation: CoreControlRepresentation,
}

/// One control object rendered across several consumer surfaces at one vocabulary.
struct ObjectSpec {
    object_id: &'static str,
    object_label: &'static str,
    component: M5CoreControlFamily,
    facets: CoreControlStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    object_id: &'static str,
    object_label: &'static str,
    component: M5CoreControlFamily,
    facets: CoreControlStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> ObjectSpec {
    ObjectSpec {
        object_id,
        object_label,
        component,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5CoreControlConsumerSurface,
    representation: CoreControlRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The eight control objects — one per B134 control family — and the surfaces that adopt
/// each, drawn from the first claimed settings, request, package-install, provider-account,
/// template-starter, admin-policy, repair, and entry consumers.
fn object_specs() -> Vec<ObjectSpec> {
    use CoreControlRepresentation::*;
    use M5CoreControlConsumerSurface::*;
    use M5CoreControlFamily::*;

    vec![
        spec(
            "settings/apply-changes-button",
            "Settings apply-changes button",
            Button,
            facets(
                "default",
                "settings.apply_changes",
                "not_applicable",
                "not_applicable",
                "unlocked",
            ),
            vec![
                bs("cc-button-settings", SettingsUi, DesktopFull),
                bs("cc-button-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "request/delete-row-icon-button",
            "Request delete-row icon button",
            IconButton,
            facets(
                "default",
                "request.delete_row",
                "not_applicable",
                "not_applicable",
                "unlocked",
            ),
            vec![
                bs("cc-iconbutton-forms", FormsUi, DesktopFull),
                bs("cc-iconbutton-review", ReviewUi, CompactNarrowed),
            ],
        ),
        spec(
            "package/install-split-button",
            "Package install split button",
            SplitButton,
            facets(
                "default",
                "package.install_default",
                "not_applicable",
                "not_applicable",
                "unlocked",
            ),
            vec![
                bs("cc-split-product", ProductUi, DesktopFull),
                bs("cc-split-repair", RepairUi, CompactNarrowed),
                bs("cc-split-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "provider/account-display-name-field",
            "Provider account display-name text field",
            TextField,
            facets(
                "default",
                "provider.rename_account",
                "user_override",
                "valid",
                "unlocked",
            ),
            vec![
                bs("cc-textfield-settings", SettingsUi, DesktopFull),
                bs("cc-textfield-forms", FormsUi, CompactNarrowed),
            ],
        ),
        spec(
            "admin/policy-search-field",
            "Admin policy search field",
            SearchField,
            facets(
                "default",
                "admin.search_policies",
                "not_applicable",
                "not_validated",
                "unlocked",
            ),
            vec![
                bs("cc-search-search", SearchUi, DesktopFull),
                bs("cc-search-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "entry/starter-template-combobox",
            "Entry starter-template combobox",
            Combobox,
            facets(
                "default",
                "entry.pick_starter_template",
                "canonical_option",
                "valid",
                "unlocked",
            ),
            vec![
                bs("cc-combobox-entry", EntryUi, DesktopFull),
                bs("cc-combobox-forms", FormsUi, RemoteProjected),
                bs("cc-combobox-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "admin/policy-enforcement-toggle",
            "Admin policy-enforcement toggle",
            ToggleControl,
            facets(
                "locked",
                "admin.set_policy_enforcement",
                "policy_enforced",
                "not_applicable",
                "policy_locked",
            ),
            vec![
                bs("cc-toggle-settings", SettingsUi, DesktopFull),
                bs("cc-toggle-product", ProductUi, CompactNarrowed),
                bs("cc-toggle-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "entry/start-center-mode-segmented",
            "Start-center entry-mode segmented control",
            SegmentedControl,
            facets(
                "default",
                "entry.switch_mode",
                "not_applicable",
                "not_applicable",
                "unlocked",
            ),
            vec![
                bs("cc-segmented-entry", EntryUi, DesktopFull),
                bs("cc-segmented-review", ReviewUi, RemoteProjected),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<CoreControlConsumerBinding>
where
    F: Fn(&str, CoreControlRepresentation) -> CoreControlRepresentation,
{
    let mut bindings = Vec::new();
    for object in object_specs() {
        for spec in &object.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                object.object_id,
                object.object_label,
                object.component,
                spec.consumer,
                representation,
                object.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> CoreControlSharedConsumersTrustReview {
    CoreControlSharedConsumersTrustReview {
        control_reuse_proven_by_fixtures: true,
        same_object_same_vocabulary_across_surfaces: true,
        state_words_stay_in_frozen_vocabulary: true,
        placeholder_never_replaces_label: true,
        loading_never_relabels_or_loses_attribution: true,
        icon_destructive_never_unlabeled: true,
        switch_never_blurred_with_deferred_checkbox: true,
        split_never_defaults_to_riskier_alternate: true,
        locked_and_degraded_stay_distinct_from_disabled: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> CoreControlSharedConsumersProjection {
    CoreControlSharedConsumersProjection {
        forms_ui_reuses_shared_controls: true,
        settings_ui_reuses_shared_controls: true,
        search_ui_reuses_shared_controls: true,
        entry_ui_reuses_shared_controls: true,
        review_ui_reuses_shared_controls: true,
        repair_ui_reuses_shared_controls: true,
        support_export_reuses_shared_controls: true,
        every_control_adopted_by_two_or_more_consumers: true,
        vocabulary_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_contract_family: true,
    }
}

fn proof_freshness() -> CoreControlSharedConsumersProofFreshness {
    CoreControlSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_CORE_CONTROL_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_CORE_CONTROL_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_CORE_CONTROL_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF.to_owned(),
        M5_CORE_CONTROL_COMPONENT_DOC_REF.to_owned(),
    ];
    for family in M5CoreControlFamily::ALL {
        refs.push(family.canonical_component_schema_ref().to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    consumer_bindings: Vec<CoreControlConsumerBinding>,
) -> M5CoreControlSharedConsumersPacket {
    M5CoreControlSharedConsumersPacket::new(M5CoreControlSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: CoreControlSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5CoreControlConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in core-control shared-consumer parity packet.
pub fn seeded_m5_core_action_input_shared_consumers() -> M5CoreControlSharedConsumersPacket {
    packet_from_bindings(
        M5_CORE_CONTROL_SHARED_CONSUMERS_PACKET_ID,
        "M5 core action / input shared consumers",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same objects with two more settings surfaces narrowed to compact and remote
/// representations, proving state changes propagate across compact and remote forms.
pub fn seeded_m5_core_action_input_shared_consumers_compact_remote_narrowed(
) -> M5CoreControlSharedConsumersPacket {
    packet_from_bindings(
        "m5-core-action-input-shared-consumers:compact-remote:0001",
        "M5 core action / input shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "cc-button-settings" => CoreControlRepresentation::CompactNarrowed,
            "cc-toggle-settings" => CoreControlRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same objects with two more surfaces narrowed to exported, export-safe
/// representations, proving state changes propagate into exported forms.
pub fn seeded_m5_core_action_input_shared_consumers_exported_redaction_narrowed(
) -> M5CoreControlSharedConsumersPacket {
    packet_from_bindings(
        "m5-core-action-input-shared-consumers:exported-redaction:0001",
        "M5 core action / input shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "cc-textfield-settings" => CoreControlRepresentation::ExportedRedacted,
            "cc-segmented-entry" => CoreControlRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
