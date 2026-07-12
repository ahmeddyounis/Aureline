//! Canonical seed builders for the M5 button / icon-button controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean buttons
//! and icon buttons are built so the shared action-state and command-attribution grammar is proven
//! across forms, settings, review, entry (start-center), support, and product surfaces without any
//! feature-local style fork, loading relabel, hidden lock, brand-only affordance, unlabeled
//! destructive action, or broken command parity.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_PACKET_ID: &str =
    "m5-button-icon-button-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn button(input: M5ButtonResolutionInput) -> M5ResolvedButton {
    resolve_button(input).expect("seed button input resolves")
}

fn icon(input: M5IconButtonResolutionInput) -> M5ResolvedIconButton {
    resolve_icon_button(input).expect("seed icon-button input resolves")
}

// -- Clean button examples (primary / destructive / quiet emphasis grammar across states) --------

#[allow(clippy::too_many_arguments)]
fn clean_button_base(
    button_id: &str,
    label: &str,
    emphasis: M5ButtonEmphasis,
    disposition: M5CoreControlDisposition,
    surface: M5ActionSurfaceContext,
    loading_behavior: M5ButtonLoadingBehavior,
    command_id: &str,
) -> M5ButtonResolutionInput {
    M5ButtonResolutionInput {
        button_id: button_id.to_owned(),
        action_label: label.to_owned(),
        emphasis,
        emphasis_stated: true,
        disposition,
        surface_context: surface,
        loading_behavior,
        loading_preserves_label_and_width: true,
        blocked_state_distinct: true,
        command_id: command_id.to_owned(),
        forks_feature_local_style: false,
        command_route_available: true,
        proof_fresh: true,
    }
}

/// Clean primary button at rest in a pane header.
fn button_primary_clean() -> M5ResolvedButton {
    button(clean_button_base(
        "button:forms:submit",
        "Submit request",
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5ActionSurfaceContext::PaneHeader,
        M5ButtonLoadingBehavior::NotLoading,
        "command:forms.submit",
    ))
}

/// Clean destructive button holding keyboard focus in a review sheet.
fn button_destructive_clean() -> M5ResolvedButton {
    button(clean_button_base(
        "button:review:discard",
        "Discard changes",
        M5ButtonEmphasis::Destructive,
        M5CoreControlDisposition::FocusVisible,
        M5ActionSurfaceContext::ReviewSheet,
        M5ButtonLoadingBehavior::NotLoading,
        "command:review.discard",
    ))
}

/// Clean quiet button in a disabled state in a settings row.
fn button_quiet_disabled_clean() -> M5ResolvedButton {
    button(clean_button_base(
        "button:settings:reset",
        "Reset to default",
        M5ButtonEmphasis::Quiet,
        M5CoreControlDisposition::Disabled,
        M5ActionSurfaceContext::SettingsRow,
        M5ButtonLoadingBehavior::NotLoading,
        "command:settings.reset",
    ))
}

/// Clean secondary button in a loading state that preserves its label and width.
fn button_loading_clean() -> M5ResolvedButton {
    button(clean_button_base(
        "button:forms:save",
        "Save draft",
        M5ButtonEmphasis::Secondary,
        M5CoreControlDisposition::Loading,
        M5ActionSurfaceContext::PaneHeader,
        M5ButtonLoadingBehavior::LabelPreservedSpinnerLeading,
        "command:forms.save",
    ))
}

/// Clean ghost button locked distinctly (never behind generic disabled chrome).
fn button_locked_clean() -> M5ResolvedButton {
    button(clean_button_base(
        "button:settings:apply",
        "Apply policy",
        M5ButtonEmphasis::Ghost,
        M5CoreControlDisposition::Locked,
        M5ActionSurfaceContext::SettingsRow,
        M5ButtonLoadingBehavior::NotLoading,
        "command:settings.apply",
    ))
}

/// Clean primary button in a support flow.
fn button_support_clean() -> M5ResolvedButton {
    button(clean_button_base(
        "button:support:export",
        "Export support bundle",
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5ActionSurfaceContext::SupportFlow,
        M5ButtonLoadingBehavior::NotLoading,
        "command:support.export",
    ))
}

// -- Degraded button examples ------------------------------------------------------------------

/// Degraded button: the action label is unstated.
fn button_label_unstated() -> M5ResolvedButton {
    let mut input = clean_button_base(
        "button:forms:no-label",
        "  ",
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5ActionSurfaceContext::PaneHeader,
        M5ButtonLoadingBehavior::NotLoading,
        "command:forms.submit",
    );
    input.action_label = "  ".to_owned();
    button(input)
}

/// Degraded button: a loading button relabeled the action or resized, losing attribution.
fn button_loading_relabeled() -> M5ResolvedButton {
    let mut input = clean_button_base(
        "button:forms:loading-relabel",
        "Save draft",
        M5ButtonEmphasis::Secondary,
        M5CoreControlDisposition::Loading,
        M5ActionSurfaceContext::PaneHeader,
        M5ButtonLoadingBehavior::WidthReservedLabelKept,
        "command:forms.save",
    );
    input.loading_preserves_label_and_width = false;
    button(input)
}

/// Degraded button: a locked state hides behind generic disabled chrome.
fn button_locked_hidden() -> M5ResolvedButton {
    let mut input = clean_button_base(
        "button:settings:locked-hidden",
        "Apply policy",
        M5ButtonEmphasis::Ghost,
        M5CoreControlDisposition::Locked,
        M5ActionSurfaceContext::SettingsRow,
        M5ButtonLoadingBehavior::NotLoading,
        "command:settings.apply",
    );
    input.blocked_state_distinct = false;
    button(input)
}

/// Degraded button: a feature-local style is forked instead of the shared emphasis grammar.
fn button_style_forked() -> M5ResolvedButton {
    let mut input = clean_button_base(
        "button:review:forked",
        "Approve",
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5ActionSurfaceContext::ReviewSheet,
        M5ButtonLoadingBehavior::NotLoading,
        "command:review.approve",
    );
    input.forks_feature_local_style = true;
    button(input)
}

/// Degraded button: the emphasis is encoded by color alone.
fn button_emphasis_color_only() -> M5ResolvedButton {
    let mut input = clean_button_base(
        "button:entry:color-only",
        "Open workspace",
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5ActionSurfaceContext::StartCenter,
        M5ButtonLoadingBehavior::NotLoading,
        "command:entry.open",
    );
    input.emphasis_stated = false;
    button(input)
}

/// Degraded button: the canonical command binding is unstated.
fn button_command_unstated() -> M5ResolvedButton {
    let mut input = clean_button_base(
        "button:support:no-command",
        "Restart service",
        M5ButtonEmphasis::Secondary,
        M5CoreControlDisposition::Default,
        M5ActionSurfaceContext::SupportFlow,
        M5ButtonLoadingBehavior::NotLoading,
        "command:support.restart",
    );
    input.command_id = "  ".to_owned();
    button(input)
}

/// Degraded button: no command-backed path to inspect the action is reachable.
fn button_trace_missing() -> M5ResolvedButton {
    let mut input = clean_button_base(
        "button:product:trace-missing",
        "Run task",
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5ActionSurfaceContext::PaneHeader,
        M5ButtonLoadingBehavior::NotLoading,
        "command:product.run",
    );
    input.command_route_available = false;
    button(input)
}

// -- Clean icon-button examples ----------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_icon_base(
    icon_button_id: &str,
    accessible_name: &str,
    label_mode: M5IconLabelMode,
    emphasis: M5ButtonEmphasis,
    surface: M5ActionSurfaceContext,
    command_surface: M5ActionCommandSurface,
    command_id: &str,
) -> M5IconButtonResolutionInput {
    M5IconButtonResolutionInput {
        icon_button_id: icon_button_id.to_owned(),
        accessible_name: accessible_name.to_owned(),
        label_mode,
        emphasis,
        disposition: M5CoreControlDisposition::Default,
        surface_context: surface,
        command_surface,
        tooltip_parity: true,
        command_id: command_id.to_owned(),
        command_parity_across_surfaces: true,
        invents_brand_only_affordance: false,
        command_route_available: true,
        proof_fresh: true,
    }
}

/// Clean icon button exposing an accessible name only, with command parity.
fn icon_named_clean() -> M5ResolvedIconButton {
    icon(clean_icon_base(
        "icon:forms:filter",
        "Filter results",
        M5IconLabelMode::AccessibleNameOnly,
        M5ButtonEmphasis::Quiet,
        M5ActionSurfaceContext::PaneHeader,
        M5ActionCommandSurface::InlineTrigger,
        "command:forms.filter",
    ))
}

/// Clean tooltip-labeled icon button with command parity across the command palette.
fn icon_tooltip_labeled_clean() -> M5ResolvedIconButton {
    icon(clean_icon_base(
        "icon:settings:info",
        "Show setting details",
        M5IconLabelMode::TooltipLabeled,
        M5ButtonEmphasis::Ghost,
        M5ActionSurfaceContext::SettingsRow,
        M5ActionCommandSurface::CommandPalette,
        "command:settings.info",
    ))
}

/// Clean icon-only destructive button that stays labeled (tooltip + accessible name).
fn icon_destructive_labeled_clean() -> M5ResolvedIconButton {
    icon(clean_icon_base(
        "icon:review:delete",
        "Delete comment",
        M5IconLabelMode::TooltipLabeled,
        M5ButtonEmphasis::Destructive,
        M5ActionSurfaceContext::ReviewSheet,
        M5ActionCommandSurface::ContextMenu,
        "command:review.delete",
    ))
}

/// Clean icon button in the start center with help-reference command parity.
fn icon_entry_clean() -> M5ResolvedIconButton {
    icon(clean_icon_base(
        "icon:entry:recent",
        "Open recent workspace",
        M5IconLabelMode::AccessibleNameOnly,
        M5ButtonEmphasis::Quiet,
        M5ActionSurfaceContext::StartCenter,
        M5ActionCommandSurface::HelpReference,
        "command:entry.recent",
    ))
}

/// Clean icon button in a support flow.
fn icon_support_clean() -> M5ResolvedIconButton {
    icon(clean_icon_base(
        "icon:support:copy",
        "Copy diagnostics",
        M5IconLabelMode::AccessibleNameOnly,
        M5ButtonEmphasis::Quiet,
        M5ActionSurfaceContext::SupportFlow,
        M5ActionCommandSurface::KeyboardShortcut,
        "command:support.copy",
    ))
}

/// Clean icon button in a product pane header.
fn icon_product_clean() -> M5ResolvedIconButton {
    icon(clean_icon_base(
        "icon:product:refresh",
        "Refresh view",
        M5IconLabelMode::AccessibleNameOnly,
        M5ButtonEmphasis::Quiet,
        M5ActionSurfaceContext::PaneHeader,
        M5ActionCommandSurface::InlineTrigger,
        "command:product.refresh",
    ))
}

// -- Degraded icon-button examples -------------------------------------------------------------

/// Degraded icon button: the accessible name is unstated.
fn icon_name_unstated() -> M5ResolvedIconButton {
    let mut input = clean_icon_base(
        "icon:forms:no-name",
        "  ",
        M5IconLabelMode::AccessibleNameOnly,
        M5ButtonEmphasis::Quiet,
        M5ActionSurfaceContext::PaneHeader,
        M5ActionCommandSurface::InlineTrigger,
        "command:forms.filter",
    );
    input.accessible_name = "  ".to_owned();
    icon(input)
}

/// Degraded icon button: the tooltip does not match the accessible name.
fn icon_tooltip_parity_missing() -> M5ResolvedIconButton {
    let mut input = clean_icon_base(
        "icon:settings:tooltip-drift",
        "Show setting details",
        M5IconLabelMode::TooltipLabeled,
        M5ButtonEmphasis::Ghost,
        M5ActionSurfaceContext::SettingsRow,
        M5ActionCommandSurface::CommandPalette,
        "command:settings.info",
    );
    input.tooltip_parity = false;
    icon(input)
}

/// Degraded icon button: an icon-only destructive action is left unlabeled.
fn icon_destructive_unlabeled() -> M5ResolvedIconButton {
    icon(clean_icon_base(
        "icon:review:delete-unlabeled",
        "Delete comment",
        M5IconLabelMode::DecorativeOnly,
        M5ButtonEmphasis::Destructive,
        M5ActionSurfaceContext::ReviewSheet,
        M5ActionCommandSurface::ContextMenu,
        "command:review.delete",
    ))
}

/// Degraded icon button: a brand-only affordance is invented.
fn icon_brand_only() -> M5ResolvedIconButton {
    let mut input = clean_icon_base(
        "icon:entry:brand-only",
        "Open recent workspace",
        M5IconLabelMode::AccessibleNameOnly,
        M5ButtonEmphasis::Quiet,
        M5ActionSurfaceContext::StartCenter,
        M5ActionCommandSurface::HelpReference,
        "command:entry.recent",
    );
    input.invents_brand_only_affordance = true;
    icon(input)
}

/// Degraded icon button: command parity across menu / palette / help is broken.
fn icon_parity_broken() -> M5ResolvedIconButton {
    let mut input = clean_icon_base(
        "icon:support:parity-broken",
        "Copy diagnostics",
        M5IconLabelMode::AccessibleNameOnly,
        M5ButtonEmphasis::Quiet,
        M5ActionSurfaceContext::SupportFlow,
        M5ActionCommandSurface::KeyboardShortcut,
        "command:support.copy",
    );
    input.command_parity_across_surfaces = false;
    icon(input)
}

/// Degraded icon button: no command-backed path to inspect the action is reachable.
fn icon_trace_missing() -> M5ResolvedIconButton {
    let mut input = clean_icon_base(
        "icon:product:trace-missing",
        "Refresh view",
        M5IconLabelMode::AccessibleNameOnly,
        M5ButtonEmphasis::Quiet,
        M5ActionSurfaceContext::PaneHeader,
        M5ActionCommandSurface::InlineTrigger,
        "command:product.refresh",
    );
    input.command_route_available = false;
    icon(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ButtonIconButtonConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    button_examples: Vec<M5ResolvedButton>,
    icon_button_examples: Vec<M5ResolvedIconButton>,
) -> M5ButtonIconButtonControlsRow {
    M5ButtonIconButtonControlsRow {
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
        ],
        accessibility_routes: M5CoreControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ButtonIconButtonAnatomyPart::ALL.to_vec(),
        export_fields: M5ButtonIconButtonExportField::ALL.to_vec(),
        downgrade_triggers,
        button_examples,
        icon_button_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BUTTON_ICON_BUTTON_CONTROLS_SCHEMA_REF,
            M5_BUTTON_SCHEMA_REF,
            M5_ICON_BUTTON_SCHEMA_REF,
        ]),
        buttons_relabel_or_resize_when_loading: false,
        icon_only_destructive_actions_go_unlabeled: false,
        locked_or_degraded_semantics_hidden_behind_disabled: false,
        controls_fork_feature_local_styles: false,
    }
}

fn controls_rows() -> Vec<M5ButtonIconButtonControlsRow> {
    use M5CoreControlConsumerSurface as C;
    use M5CoreControlDowngradeTrigger as D;

    vec![
        base_row(
            C::FormsUi,
            "Forms surface owner",
            "The forms surface names one permanent action label and stable primary/secondary emphasis, preserves the label and width while a submit or save is in flight, and offers a labeled icon button with command parity; both degrade honestly when the label is unstated or a loading button loses attribution",
            "evidence:m5-button-icon-button-forms-ui:001",
            vec![
                D::PlaceholderUsedAsLabel,
                D::LoadingRelabeledOrResized,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                button_primary_clean(),
                button_loading_clean(),
                button_loading_relabeled(),
                button_label_unstated(),
            ],
            vec![icon_named_clean(), icon_name_unstated()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface keeps quiet and ghost emphasis distinct in disabled and locked states, showing locked semantics distinctly rather than behind generic disabled chrome, and keeps icon tooltip parity; both degrade honestly when a lock hides behind disabled or tooltip parity drifts",
            "evidence:m5-button-icon-button-settings-ui:001",
            vec![
                D::LockedOrDegradedHiddenBehindDisabled,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                button_quiet_disabled_clean(),
                button_locked_clean(),
                button_locked_hidden(),
            ],
            vec![icon_tooltip_labeled_clean(), icon_tooltip_parity_missing()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review sheet keeps destructive triggers appropriately risky and always labeled, reusing the shared emphasis grammar rather than a feature-local style fork, and never leaves an icon-only destructive action unlabeled; both degrade honestly when a style is forked or a destructive icon goes unlabeled",
            "evidence:m5-button-icon-button-review-ui:001",
            vec![
                D::StateTaxonomyDrifted,
                D::IconOnlyDestructiveUnlabeled,
                D::ProofStale,
            ],
            vec![button_destructive_clean(), button_style_forked()],
            vec![icon_destructive_labeled_clean(), icon_destructive_unlabeled()],
        ),
        base_row(
            C::EntryUi,
            "Start-center entry owner",
            "The start center reuses the same primary emphasis and named icon affordances a user sees elsewhere, never encoding emphasis by color alone and never inventing a brand-only affordance; both degrade honestly when emphasis is color-only or a brand-only affordance is invented",
            "evidence:m5-button-icon-button-entry-ui:001",
            vec![
                D::StateTaxonomyDrifted,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![button_primary_clean(), button_emphasis_color_only()],
            vec![icon_entry_clean(), icon_brand_only()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved action and command truth, so an unstated command binding, a broken command parity, or a missing canonical command ID is visible in evidence rather than hidden behind generic chrome",
            "evidence:m5-button-icon-button-support-export:001",
            vec![
                D::CommandBindingUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![button_support_clean(), button_command_unstated()],
            vec![icon_support_clean(), icon_parity_broken()],
        ),
        base_row(
            C::ProductUi,
            "In-product action owner",
            "In-product surfaces reuse the same action label, emphasis, and command grammar a user sees in forms and settings, always offering the command-backed detail path and degrading honestly when the trace path is missing",
            "evidence:m5-button-icon-button-product-ui:001",
            vec![
                D::CommandBindingUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![button_primary_clean(), button_trace_missing()],
            vec![icon_product_clean(), icon_trace_missing()],
        ),
    ]
}

fn governance_review() -> M5ButtonIconButtonGovernanceReview {
    M5ButtonIconButtonGovernanceReview {
        button_names_label_and_emphasis: true,
        button_preserves_width_and_label_while_loading: true,
        button_never_forks_feature_local_style: true,
        icon_button_always_exposes_accessible_name: true,
        icon_button_keeps_tooltip_parity: true,
        icon_button_never_unlabeled_when_destructive: true,
        icon_button_binds_canonical_command_with_parity: true,
        locked_and_degraded_never_hidden_behind_disabled: true,
        emphasis_and_state_never_encoded_by_color_alone: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ButtonIconButtonConsumerProjection {
    M5ButtonIconButtonConsumerProjection {
        forms_surfaces_consume_button_vocabulary: true,
        settings_surfaces_consume_button_vocabulary: true,
        review_surfaces_consume_action_and_command_vocabulary: true,
        entry_surfaces_consume_button_vocabulary: true,
        action_facts_trace_to_single_component_contract: true,
        support_export_reads_single_control_source: true,
    }
}

fn proof_freshness() -> M5ButtonIconButtonProofFreshness {
    M5ButtonIconButtonProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ButtonIconButtonReleasePosture {
    M5ButtonIconButtonReleasePosture {
        proof_packet_ref: M5_BUTTON_ICON_BUTTON_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_BUTTON_ICON_BUTTON_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BUTTON_ICON_BUTTON_CONTROLS_SCHEMA_REF,
        M5_BUTTON_ICON_BUTTON_CONTROLS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_BUTTON_SCHEMA_REF,
        M5_ICON_BUTTON_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 button / icon-button controls packet.
pub fn seeded_m5_button_icon_button_controls() -> M5ButtonIconButtonControlsPacket {
    M5ButtonIconButtonControlsPacket::new(M5ButtonIconButtonControlsPacketInput {
        packet_id: M5_BUTTON_ICON_BUTTON_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 button and icon-button controls with primary/secondary/quiet/destructive/ghost emphasis, loading-attribution-preserving states, locked/degraded distinctness, and canonical command parity aligned across forms, settings, review, entry, support, and product surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5ButtonIconButtonVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the forms-UI row is held at Beta pending action-attribution parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_button_icon_button_controls_forms_ui_beta_narrowed(
) -> M5ButtonIconButtonControlsPacket {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.packet_id = "m5-button-icon-button-controls:forms-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoreControlConsumerSurface::FormsUi)
        .expect("forms-ui row present");
    row.qualification = M5CoreControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review-UI row is narrowed to Preview pending destructive-labeling parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_button_icon_button_controls_review_ui_preview_narrowed(
) -> M5ButtonIconButtonControlsPacket {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.packet_id = "m5-button-icon-button-controls:review-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoreControlConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5CoreControlQualificationClass::Preview;
    packet
}
