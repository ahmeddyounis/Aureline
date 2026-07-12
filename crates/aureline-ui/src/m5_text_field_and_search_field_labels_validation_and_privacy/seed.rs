//! Canonical seed builders for the M5 text-field / search-field controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean text and
//! search fields are built so the shared permanent-label, validation-honesty, clear/submit, and
//! retention/privacy grammar is proven across forms, settings, search, entry, support, and product
//! surfaces without any placeholder-only label, vague validation copy, missing reveal, undisclosed
//! privacy cue, hidden blocked state, dropped clear, or broken command trace.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_PACKET_ID: &str =
    "m5-text-field-search-field-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn text(input: M5TextFieldResolutionInput) -> M5ResolvedTextField {
    resolve_text_field(input).expect("seed text-field input resolves")
}

fn search(input: M5SearchFieldResolutionInput) -> M5ResolvedSearchField {
    resolve_search_field(input).expect("seed search-field input resolves")
}

// -- Clean text-field examples (permanent labels + validation honesty across states) -----------

#[allow(clippy::too_many_arguments)]
fn clean_text_base(
    text_field_id: &str,
    label: &str,
    label_mode: M5FieldLabelMode,
    validation: M5FieldValidationState,
    disposition: M5CoreControlDisposition,
    surface: M5FieldSurfaceContext,
    command_id: &str,
) -> M5TextFieldResolutionInput {
    M5TextFieldResolutionInput {
        text_field_id: text_field_id.to_owned(),
        label: label.to_owned(),
        label_mode,
        validation,
        validation_message_specific: true,
        disposition,
        surface_context: surface,
        focus_visible_offered: true,
        requires_reveal: false,
        reveal_offered: false,
        blocked_state_distinct: true,
        draft_preserved_across_interruption: true,
        validation_anchor_preserved: true,
        command_id: command_id.to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

/// Clean text field with a persistent visible label and a valid value.
fn text_persistent_clean() -> M5ResolvedTextField {
    text(clean_text_base(
        "text:forms:name",
        "Full name",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        "command:forms.name",
    ))
}

/// Clean text field with a floating label and a specific non-blocking warning.
fn text_floating_warning_clean() -> M5ResolvedTextField {
    text(clean_text_base(
        "text:settings:endpoint-name",
        "Workspace name",
        M5FieldLabelMode::FloatingLabel,
        M5FieldValidationState::WarningNonblocking,
        M5CoreControlDisposition::FocusVisible,
        M5FieldSurfaceContext::SettingsRow,
        "command:settings.workspace_name",
    ))
}

/// Clean text field with an accessible label only and a not-yet-validated value.
fn text_aria_clean() -> M5ResolvedTextField {
    text(clean_text_base(
        "text:entry:project",
        "Project title",
        M5FieldLabelMode::AriaLabelOnly,
        M5FieldValidationState::NotValidated,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::EntryField,
        "command:entry.project_title",
    ))
}

/// Clean sensitive text field that requires and offers a reveal control.
fn text_reveal_clean() -> M5ResolvedTextField {
    let mut input = clean_text_base(
        "text:entry:token-name",
        "Access token name",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::EntryField,
        "command:entry.token_name",
    );
    input.requires_reveal = true;
    input.reveal_offered = true;
    text(input)
}

/// Clean read-only text field kept distinct from generic disabled chrome.
fn text_readonly_clean() -> M5ResolvedTextField {
    text(clean_text_base(
        "text:support:bundle-id",
        "Support bundle ID",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::ReadOnly,
        M5FieldSurfaceContext::SupportFlow,
        "command:support.bundle_id",
    ))
}

/// Clean locked text field kept distinct from generic disabled chrome.
fn text_locked_clean() -> M5ResolvedTextField {
    text(clean_text_base(
        "text:settings:policy-name",
        "Policy name",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Locked,
        M5FieldSurfaceContext::SettingsRow,
        "command:settings.policy_name",
    ))
}

/// Clean text field in a product surface.
fn text_product_clean() -> M5ResolvedTextField {
    text(clean_text_base(
        "text:product:rename",
        "Rename item",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        "command:product.rename",
    ))
}

// -- Degraded text-field examples --------------------------------------------------------------

/// Degraded text field: the label is placeholder-only.
fn text_placeholder_only() -> M5ResolvedTextField {
    let mut input = clean_text_base(
        "text:forms:placeholder-only",
        "Search…",
        M5FieldLabelMode::PlaceholderOnlyDisallowed,
        M5FieldValidationState::NotValidated,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        "command:forms.name",
    );
    input.label_mode = M5FieldLabelMode::PlaceholderOnlyDisallowed;
    text(input)
}

/// Degraded text field: an invalid value carries only vague validation copy.
fn text_vague_validation() -> M5ResolvedTextField {
    let mut input = clean_text_base(
        "text:search:filter-vague",
        "Filter expression",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::InvalidBlocking,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SearchBar,
        "command:search.filter",
    );
    input.validation_message_specific = false;
    text(input)
}

/// Degraded text field: a sensitive value is missing its reveal control.
fn text_reveal_missing() -> M5ResolvedTextField {
    let mut input = clean_text_base(
        "text:entry:token-no-reveal",
        "Access token name",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::EntryField,
        "command:entry.token_name",
    );
    input.requires_reveal = true;
    input.reveal_offered = false;
    text(input)
}

/// Degraded text field: a locked state hides behind generic disabled chrome.
fn text_locked_hidden() -> M5ResolvedTextField {
    let mut input = clean_text_base(
        "text:settings:locked-hidden",
        "Policy name",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Locked,
        M5FieldSurfaceContext::SettingsRow,
        "command:settings.policy_name",
    );
    input.blocked_state_distinct = false;
    text(input)
}

/// Degraded text field: draft state was not preserved across the first interruption.
fn text_draft_lost() -> M5ResolvedTextField {
    let mut input = clean_text_base(
        "text:product:draft-lost",
        "Rename item",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        "command:product.rename",
    );
    input.draft_preserved_across_interruption = false;
    text(input)
}

/// Degraded text field: an exact validation anchor was lost across recovery.
fn text_anchor_lost() -> M5ResolvedTextField {
    let mut input = clean_text_base(
        "text:product:anchor-lost",
        "Rename item",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::InvalidBlocking,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        "command:product.rename",
    );
    input.validation_anchor_preserved = false;
    text(input)
}

/// Degraded text field: the canonical command binding is unstated.
fn text_command_unstated() -> M5ResolvedTextField {
    let mut input = clean_text_base(
        "text:support:no-command",
        "Support note",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SupportFlow,
        "command:support.note",
    );
    input.command_id = "  ".to_owned();
    text(input)
}

// -- Clean search-field examples (clear/submit/privacy grammar across retention postures) -------

#[allow(clippy::too_many_arguments)]
fn clean_search_base(
    search_field_id: &str,
    label: &str,
    label_mode: M5FieldLabelMode,
    validation: M5FieldValidationState,
    disposition: M5CoreControlDisposition,
    surface: M5FieldSurfaceContext,
    submit_model: M5SearchSubmitModel,
    retention_posture: M5SearchRetentionPosture,
    command_id: &str,
) -> M5SearchFieldResolutionInput {
    M5SearchFieldResolutionInput {
        search_field_id: search_field_id.to_owned(),
        label: label.to_owned(),
        label_mode,
        validation,
        validation_message_specific: true,
        disposition,
        surface_context: surface,
        offers_search_icon: true,
        offers_clear: true,
        submit_model,
        scope_label: String::new(),
        retention_posture,
        privacy_disclosed: true,
        blocked_state_distinct: true,
        draft_preserved_across_interruption: true,
        command_id: command_id.to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

/// Clean live search: submits as-you-type, not retained, with clear and search icon.
fn search_live_clean() -> M5ResolvedSearchField {
    search(clean_search_base(
        "search:search-bar:live",
        "Search results",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SearchBar,
        M5SearchSubmitModel::SubmitAsYouType,
        M5SearchRetentionPosture::LiveNotRetained,
        "command:search.run",
    ))
}

/// Clean provider-backed search: explicit submit, provider scope disclosed.
fn search_provider_clean() -> M5ResolvedSearchField {
    search(clean_search_base(
        "search:search-bar:provider",
        "Search the web index",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SearchBar,
        M5SearchSubmitModel::SubmitExplicit,
        M5SearchRetentionPosture::ProviderBackedRemote,
        "command:search.provider",
    ))
}

/// Clean export-sensitive search: scoped submit within a named scope, export handling disclosed.
fn search_export_clean() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:support:export",
        "Search support bundle",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SupportFlow,
        M5SearchSubmitModel::SubmitScoped,
        M5SearchRetentionPosture::ExportSensitive,
        "command:support.search",
    );
    input.scope_label = "Current bundle".to_owned();
    search(input)
}

/// Clean cached search: cached results disclosed, debounced submit.
fn search_cached_clean() -> M5ResolvedSearchField {
    search(clean_search_base(
        "search:settings:cached",
        "Search settings",
        M5FieldLabelMode::FloatingLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SettingsRow,
        M5SearchSubmitModel::SubmitDebounced,
        M5SearchRetentionPosture::CachedResultsDisclosed,
        "command:settings.search",
    ))
}

/// Clean private-history search: history kept private, explicit submit.
fn search_private_clean() -> M5ResolvedSearchField {
    search(clean_search_base(
        "search:entry:private",
        "Search recents",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::EntryField,
        M5SearchSubmitModel::SubmitExplicit,
        M5SearchRetentionPosture::HistoryPrivate,
        "command:entry.search_recents",
    ))
}

/// Clean blocked search: submission blocked by policy, shown distinctly.
fn search_blocked_clean() -> M5ResolvedSearchField {
    search(clean_search_base(
        "search:product:blocked",
        "Search catalog",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        M5SearchSubmitModel::SubmitBlocked,
        M5SearchRetentionPosture::LiveNotRetained,
        "command:product.search_catalog",
    ))
}

// -- Degraded search-field examples ------------------------------------------------------------

/// Degraded search field: the label is placeholder-only.
fn search_placeholder_only() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:forms:placeholder-only",
        "Type to search…",
        M5FieldLabelMode::PlaceholderOnlyDisallowed,
        M5FieldValidationState::NotValidated,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        M5SearchSubmitModel::SubmitAsYouType,
        M5SearchRetentionPosture::LiveNotRetained,
        "command:forms.search",
    );
    input.label_mode = M5FieldLabelMode::PlaceholderOnlyDisallowed;
    search(input)
}

/// Degraded search field: the clear affordance is missing.
fn search_clear_missing() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:search-bar:no-clear",
        "Search results",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SearchBar,
        M5SearchSubmitModel::SubmitAsYouType,
        M5SearchRetentionPosture::LiveNotRetained,
        "command:search.run",
    );
    input.offers_clear = false;
    search(input)
}

/// Degraded search field: the search icon cue is missing.
fn search_icon_missing() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:search-bar:no-icon",
        "Search results",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SearchBar,
        M5SearchSubmitModel::SubmitAsYouType,
        M5SearchRetentionPosture::LiveNotRetained,
        "command:search.run",
    );
    input.offers_search_icon = false;
    search(input)
}

/// Degraded search field: a material privacy cue is left undisclosed.
fn search_privacy_missing() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:settings:privacy-missing",
        "Search settings",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SettingsRow,
        M5SearchSubmitModel::SubmitDebounced,
        M5SearchRetentionPosture::CachedResultsDisclosed,
        "command:settings.search",
    );
    input.privacy_disclosed = false;
    search(input)
}

/// Degraded search field: the submit model is unresolved.
fn search_submit_unresolved() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:entry:submit-unknown",
        "Search recents",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::EntryField,
        M5SearchSubmitModel::SubmitUnknown,
        M5SearchRetentionPosture::HistoryPrivate,
        "command:entry.search_recents",
    );
    input.submit_model = M5SearchSubmitModel::SubmitUnknown;
    search(input)
}

/// Degraded search field: a blocked submission hides behind generic disabled chrome.
fn search_blocked_hidden() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:support:blocked-hidden",
        "Search support bundle",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::SupportFlow,
        M5SearchSubmitModel::SubmitBlocked,
        M5SearchRetentionPosture::LiveNotRetained,
        "command:support.search",
    );
    input.blocked_state_distinct = false;
    search(input)
}

/// Degraded search field: draft state was not preserved across the first interruption.
fn search_draft_lost() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:product:draft-lost",
        "Search catalog",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        M5SearchSubmitModel::SubmitExplicit,
        M5SearchRetentionPosture::LiveNotRetained,
        "command:product.search_catalog",
    );
    input.draft_preserved_across_interruption = false;
    search(input)
}

/// Degraded search field: no command-backed path to inspect the field is reachable.
fn search_trace_missing() -> M5ResolvedSearchField {
    let mut input = clean_search_base(
        "search:product:trace-missing",
        "Search catalog",
        M5FieldLabelMode::PersistentLabel,
        M5FieldValidationState::Valid,
        M5CoreControlDisposition::Default,
        M5FieldSurfaceContext::FormsSheet,
        M5SearchSubmitModel::SubmitExplicit,
        M5SearchRetentionPosture::LiveNotRetained,
        "command:product.search_catalog",
    );
    input.command_route_available = false;
    search(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5FieldConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    text_field_examples: Vec<M5ResolvedTextField>,
    search_field_examples: Vec<M5ResolvedSearchField>,
) -> M5TextFieldSearchFieldControlsRow {
    M5TextFieldSearchFieldControlsRow {
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
            M5CoreControlRequiredLabel::ValidationAndConstraints,
        ],
        accessibility_routes: M5CoreControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5FieldAnatomyPart::ALL.to_vec(),
        export_fields: M5FieldExportField::ALL.to_vec(),
        downgrade_triggers,
        text_field_examples,
        search_field_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_SCHEMA_REF,
            M5_TEXT_FIELD_SCHEMA_REF,
            M5_SEARCH_FIELD_SCHEMA_REF,
        ]),
        placeholder_text_replaces_label: false,
        vague_validation_copy_used: false,
        clear_submit_or_privacy_truth_dropped: false,
        locked_or_degraded_semantics_hidden_behind_disabled: false,
    }
}

fn controls_rows() -> Vec<M5TextFieldSearchFieldControlsRow> {
    use M5CoreControlConsumerSurface as C;
    use M5CoreControlDowngradeTrigger as D;

    vec![
        base_row(
            C::FormsUi,
            "Forms surface owner",
            "The forms surface renders a text field with a permanent label and a search field with a search icon, clear affordance, and submit model; both degrade honestly when the label is placeholder-only",
            "evidence:m5-text-search-forms-ui:001",
            vec![
                D::PlaceholderUsedAsLabel,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![text_persistent_clean(), text_placeholder_only()],
            vec![search_live_clean(), search_placeholder_only()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface keeps read-only and locked text fields distinct rather than behind generic disabled chrome, and discloses a cached search's retention cue; both degrade honestly when a lock hides behind disabled or a privacy cue is missing",
            "evidence:m5-text-search-settings-ui:001",
            vec![
                D::LockedOrDegradedHiddenBehindDisabled,
                D::ValueSourceUnstated,
                D::ProofStale,
            ],
            vec![
                text_floating_warning_clean(),
                text_locked_clean(),
                text_locked_hidden(),
            ],
            vec![search_cached_clean(), search_privacy_missing()],
        ),
        base_row(
            C::SearchUi,
            "Search surface owner",
            "The search surface keeps a provider-backed query's scope disclosed and its clear affordance present, and keeps text-field validation copy specific; both degrade honestly when validation copy is vague, the clear affordance is missing, or the search icon cue is missing",
            "evidence:m5-text-search-search-ui:001",
            vec![
                D::ValidationStateUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![text_aria_clean(), text_vague_validation()],
            vec![
                search_provider_clean(),
                search_clear_missing(),
                search_icon_missing(),
            ],
        ),
        base_row(
            C::EntryUi,
            "Start-center entry owner",
            "The start-center entry surface offers a sensitive text field with a reveal control and a private-history search with a resolved submit model; both degrade honestly when the reveal control is missing or the submit model is unresolved",
            "evidence:m5-text-search-entry-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::ValueSourceUnstated,
                D::ProofStale,
            ],
            vec![text_reveal_clean(), text_reveal_missing()],
            vec![search_private_clean(), search_submit_unresolved()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved label, validation, retention, and submit truth, so an unstated command binding or a blocked search hidden behind disabled is visible in evidence rather than hidden behind generic chrome",
            "evidence:m5-text-search-support-export:001",
            vec![
                D::CommandBindingUnstated,
                D::LockedOrDegradedHiddenBehindDisabled,
                D::ProofStale,
            ],
            vec![text_readonly_clean(), text_command_unstated()],
            vec![search_export_clean(), search_blocked_hidden()],
        ),
        base_row(
            C::ProductUi,
            "In-product control owner",
            "In-product surfaces reuse the same permanent-label, validation-anchor, and draft-continuity grammar a user sees in forms and settings, always offering the command-backed detail path and degrading honestly when draft continuity is lost, a validation anchor is lost, or the trace path is missing",
            "evidence:m5-text-search-product-ui:001",
            vec![
                D::ValidationStateUnstated,
                D::CommandBindingUnstated,
                D::ProofStale,
            ],
            vec![text_product_clean(), text_draft_lost(), text_anchor_lost()],
            vec![
                search_blocked_clean(),
                search_draft_lost(),
                search_trace_missing(),
            ],
        ),
    ]
}

fn governance_review() -> M5TextFieldSearchFieldGovernanceReview {
    M5TextFieldSearchFieldGovernanceReview {
        text_names_permanent_label_and_validation: true,
        text_never_uses_placeholder_as_label: true,
        text_keeps_validation_copy_specific: true,
        text_preserves_draft_and_validation_anchors: true,
        focus_visible_treatment_present: true,
        search_exposes_clear_and_submit_truth: true,
        search_discloses_retention_and_privacy_cues: true,
        search_keeps_blocked_state_distinct: true,
        search_preserves_draft_across_interruption: true,
        locked_and_degraded_never_hidden_behind_disabled: true,
        both_bind_canonical_command_with_trace: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5TextFieldSearchFieldConsumerProjection {
    M5TextFieldSearchFieldConsumerProjection {
        forms_surfaces_consume_text_and_search_vocabulary: true,
        settings_surfaces_consume_text_vocabulary: true,
        search_surfaces_consume_search_vocabulary: true,
        entry_surfaces_consume_field_vocabulary: true,
        label_validation_and_privacy_facts_trace_to_single_component_contract: true,
        support_export_reads_single_control_source: true,
    }
}

fn proof_freshness() -> M5TextFieldSearchFieldProofFreshness {
    M5TextFieldSearchFieldProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TextFieldSearchFieldReleasePosture {
    M5TextFieldSearchFieldReleasePosture {
        proof_packet_ref: M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_SCHEMA_REF,
        M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_TEXT_FIELD_SCHEMA_REF,
        M5_SEARCH_FIELD_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 text-field / search-field controls packet.
pub fn seeded_m5_text_field_search_field_controls() -> M5TextFieldSearchFieldControlsPacket {
    M5TextFieldSearchFieldControlsPacket::new(M5TextFieldSearchFieldControlsPacketInput {
        packet_id: M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 text-field and search-field controls with permanent labels, specific validation copy, focus-visible treatment, and reveal/clear/submit and retention/privacy truth aligned across forms, settings, search, entry, support, and product surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5TextFieldSearchFieldVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the settings-UI row is held at Beta pending locked-state and retention-cue parity on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_text_field_search_field_controls_settings_ui_beta_narrowed(
) -> M5TextFieldSearchFieldControlsPacket {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.packet_id = "m5-text-field-search-field-controls:settings-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoreControlConsumerSurface::SettingsUi)
        .expect("settings-ui row present");
    row.qualification = M5CoreControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the search-UI row is narrowed to Preview pending clear/submit/privacy parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_text_field_search_field_controls_search_ui_preview_narrowed(
) -> M5TextFieldSearchFieldControlsPacket {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.packet_id = "m5-text-field-search-field-controls:search-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoreControlConsumerSurface::SearchUi)
        .expect("search-ui row present");
    row.qualification = M5CoreControlQualificationClass::Preview;
    packet
}
