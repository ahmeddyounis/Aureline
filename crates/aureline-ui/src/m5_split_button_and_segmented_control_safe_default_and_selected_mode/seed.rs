//! Canonical seed builders for the M5 split-button / segmented-control controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean split
//! buttons and segmented controls are built so the shared safe-default and selected-mode grammar is
//! proven across forms, settings, search, review, support, and product surfaces without any riskier
//! alternate default, hidden alternate, undisclosed scope, stealth-navigation misuse, oversized set,
//! hidden lock, or broken command trace.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_PACKET_ID: &str =
    "m5-split-button-segmented-control-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn split(input: M5SplitButtonResolutionInput) -> M5ResolvedSplitButton {
    resolve_split_button(input).expect("seed split-button input resolves")
}

fn segmented(input: M5SegmentedControlResolutionInput) -> M5ResolvedSegmentedControl {
    resolve_segmented_control(input).expect("seed segmented-control input resolves")
}

// -- Clean split-button examples (safe default posture grammar across states) ------------------

#[allow(clippy::too_many_arguments)]
fn clean_split_base(
    split_button_id: &str,
    primary_action_label: &str,
    default_posture: M5SplitDefaultPosture,
    default_emphasis: M5ButtonEmphasis,
    disposition: M5CoreControlDisposition,
    surface: M5SplitSegmentedSurfaceContext,
    alternate_visibility: M5SplitAlternateVisibility,
    scope_impact: M5SplitScopeImpact,
    command_id: &str,
) -> M5SplitButtonResolutionInput {
    M5SplitButtonResolutionInput {
        split_button_id: split_button_id.to_owned(),
        primary_action_label: primary_action_label.to_owned(),
        default_posture,
        default_emphasis,
        emphasis_stated: true,
        disposition,
        surface_context: surface,
        alternate_visibility,
        scope_impact,
        scope_disclosed: true,
        stale_state_promoted_riskier_alternate: false,
        blocked_state_distinct: true,
        command_id: command_id.to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

/// Clean split button whose safe default click submits, with alternates in the adjacent menu.
fn split_primary_safe_clean() -> M5ResolvedSplitButton {
    split(clean_split_base(
        "split:forms:submit",
        "Submit request",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:forms.submit",
    ))
}

/// Clean split button whose alternates are reachable only by explicit selection.
fn split_explicit_alternate_clean() -> M5ResolvedSplitButton {
    split(clean_split_base(
        "split:search:save-search",
        "Save search",
        M5SplitDefaultPosture::ExplicitAlternate,
        M5ButtonEmphasis::Secondary,
        M5CoreControlDisposition::FocusVisible,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitAlternateVisibility::DisclosedOnExpand,
        M5SplitScopeImpact::CurrentSelection,
        "command:search.save",
    ))
}

/// Clean split button in a review sheet whose default is safe while a destructive alternate is guarded,
/// with a disclosed batch scope.
fn split_destructive_guarded_clean() -> M5ResolvedSplitButton {
    split(clean_split_base(
        "split:review:merge",
        "Merge selected",
        M5SplitDefaultPosture::DestructiveGuarded,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::WholeBatch,
        "command:review.merge",
    ))
}

/// Clean split button in a settings row whose default requires an explicit confirm.
fn split_confirm_clean() -> M5ResolvedSplitButton {
    split(clean_split_base(
        "split:settings:apply",
        "Apply and restart",
        M5SplitDefaultPosture::ConfirmRequired,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::SettingsRow,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:settings.apply",
    ))
}

/// Clean ghost split button locked distinctly (never behind generic disabled chrome).
fn split_locked_clean() -> M5ResolvedSplitButton {
    split(clean_split_base(
        "split:settings:policy",
        "Apply policy",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Ghost,
        M5CoreControlDisposition::Locked,
        M5SplitSegmentedSurfaceContext::SettingsRow,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:settings.policy",
    ))
}

/// Clean split button in a support flow.
fn split_support_clean() -> M5ResolvedSplitButton {
    split(clean_split_base(
        "split:support:export",
        "Export support bundle",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::SupportFlow,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:support.export",
    ))
}

/// Clean split button in a product pane header.
fn split_product_clean() -> M5ResolvedSplitButton {
    split(clean_split_base(
        "split:product:run",
        "Run task",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:product.run",
    ))
}

// -- Degraded split-button examples ------------------------------------------------------------

/// Degraded split button: the primary action label is unstated.
fn split_primary_unstated() -> M5ResolvedSplitButton {
    let mut input = clean_split_base(
        "split:forms:no-label",
        "  ",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:forms.submit",
    );
    input.primary_action_label = "  ".to_owned();
    split(input)
}

/// Degraded split button: stale state promoted a riskier alternate to the default click.
fn split_riskier_default() -> M5ResolvedSplitButton {
    let mut input = clean_split_base(
        "split:review:riskier-default",
        "Merge selected",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:review.merge",
    );
    input.stale_state_promoted_riskier_alternate = true;
    split(input)
}

/// Degraded split button: an alternate is hidden behind the default click.
fn split_alternate_hidden() -> M5ResolvedSplitButton {
    let mut input = clean_split_base(
        "split:search:alternate-hidden",
        "Save search",
        M5SplitDefaultPosture::ExplicitAlternate,
        M5ButtonEmphasis::Secondary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitAlternateVisibility::AlternateHidden,
        M5SplitScopeImpact::SingleTarget,
        "command:search.save",
    );
    input.alternate_visibility = M5SplitAlternateVisibility::AlternateHidden;
    split(input)
}

/// Degraded split button: a broadened batch scope is left undisclosed.
fn split_scope_undisclosed() -> M5ResolvedSplitButton {
    let mut input = clean_split_base(
        "split:review:scope-undisclosed",
        "Merge selected",
        M5SplitDefaultPosture::DestructiveGuarded,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::WholeBatch,
        "command:review.merge",
    );
    input.scope_disclosed = false;
    split(input)
}

/// Degraded split button: a locked state hides behind generic disabled chrome.
fn split_locked_hidden() -> M5ResolvedSplitButton {
    let mut input = clean_split_base(
        "split:settings:locked-hidden",
        "Apply policy",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Ghost,
        M5CoreControlDisposition::Locked,
        M5SplitSegmentedSurfaceContext::SettingsRow,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:settings.policy",
    );
    input.blocked_state_distinct = false;
    split(input)
}

/// Degraded split button: the emphasis is encoded by color alone.
fn split_emphasis_color_only() -> M5ResolvedSplitButton {
    let mut input = clean_split_base(
        "split:product:color-only",
        "Run task",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:product.run",
    );
    input.emphasis_stated = false;
    split(input)
}

/// Degraded split button: the canonical command binding is unstated.
fn split_command_unstated() -> M5ResolvedSplitButton {
    let mut input = clean_split_base(
        "split:support:no-command",
        "Export support bundle",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::SupportFlow,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:support.export",
    );
    input.command_id = "  ".to_owned();
    split(input)
}

/// Degraded split button: no command-backed path to inspect the action is reachable.
fn split_trace_missing() -> M5ResolvedSplitButton {
    let mut input = clean_split_base(
        "split:product:trace-missing",
        "Run task",
        M5SplitDefaultPosture::PrimaryDefaultSafe,
        M5ButtonEmphasis::Primary,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitAlternateVisibility::AdjacentMenuVisible,
        M5SplitScopeImpact::SingleTarget,
        "command:product.run",
    );
    input.command_route_available = false;
    split(input)
}

// -- Clean segmented-control examples ----------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_segmented_base(
    segmented_control_id: &str,
    group_label: &str,
    selected_segment_label: &str,
    mode: M5SegmentedMode,
    disposition: M5CoreControlDisposition,
    surface: M5SplitSegmentedSurfaceContext,
    scope_impact: M5SplitScopeImpact,
    command_id: &str,
) -> M5SegmentedControlResolutionInput {
    M5SegmentedControlResolutionInput {
        segmented_control_id: segmented_control_id.to_owned(),
        group_label: group_label.to_owned(),
        selected_segment_label: selected_segment_label.to_owned(),
        mode,
        disposition,
        surface_context: surface,
        selected_state_explicit: true,
        keyboard_cycling_available: true,
        oversized_segment_set: false,
        masquerades_as_navigation: false,
        scope_impact,
        scope_disclosed: true,
        blocked_state_distinct: true,
        command_id: command_id.to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

/// Clean segmented control toggling a compact mode with explicit selected-mode truth.
fn segmented_mode_toggle_clean() -> M5ResolvedSegmentedControl {
    segmented(clean_segmented_base(
        "segmented:forms:layout",
        "Layout mode",
        "Comfortable",
        M5SegmentedMode::ModeToggle,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitScopeImpact::SingleTarget,
        "command:forms.layout",
    ))
}

/// Clean segmented control switching a view of the same content.
fn segmented_view_switch_clean() -> M5ResolvedSegmentedControl {
    segmented(clean_segmented_base(
        "segmented:search:view",
        "Results view",
        "List",
        M5SegmentedMode::ViewSwitch,
        M5CoreControlDisposition::FocusVisible,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitScopeImpact::CurrentSelection,
        "command:search.view",
    ))
}

/// Clean segmented control as a single-select over a small set.
fn segmented_single_select_clean() -> M5ResolvedSegmentedControl {
    segmented(clean_segmented_base(
        "segmented:settings:density",
        "Density",
        "Compact",
        M5SegmentedMode::SingleSelectSmallSet,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::SettingsRow,
        M5SplitScopeImpact::SingleTarget,
        "command:settings.density",
    ))
}

/// Clean segmented control locked distinctly (never behind generic disabled chrome).
fn segmented_locked_clean() -> M5ResolvedSegmentedControl {
    segmented(clean_segmented_base(
        "segmented:settings:locked",
        "Sync mode",
        "Manual",
        M5SegmentedMode::ModeToggle,
        M5CoreControlDisposition::Locked,
        M5SplitSegmentedSurfaceContext::SettingsRow,
        M5SplitScopeImpact::SingleTarget,
        "command:settings.sync",
    ))
}

/// Clean segmented control in a support flow.
fn segmented_support_clean() -> M5ResolvedSegmentedControl {
    segmented(clean_segmented_base(
        "segmented:support:detail",
        "Detail level",
        "Summary",
        M5SegmentedMode::ViewSwitch,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::SupportFlow,
        M5SplitScopeImpact::SingleTarget,
        "command:support.detail",
    ))
}

/// Clean segmented control in a product pane header.
fn segmented_product_clean() -> M5ResolvedSegmentedControl {
    segmented(clean_segmented_base(
        "segmented:product:mode",
        "Editor mode",
        "Preview",
        M5SegmentedMode::ModeToggle,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitScopeImpact::SingleTarget,
        "command:product.mode",
    ))
}

// -- Degraded segmented-control examples -------------------------------------------------------

/// Degraded segmented control: the group label is unstated.
fn segmented_group_unstated() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:forms:no-group",
        "  ",
        "Comfortable",
        M5SegmentedMode::ModeToggle,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitScopeImpact::SingleTarget,
        "command:forms.layout",
    );
    input.group_label = "  ".to_owned();
    segmented(input)
}

/// Degraded segmented control: the selected-segment label is unstated.
fn segmented_selected_unstated() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:support:no-selected",
        "Detail level",
        "  ",
        M5SegmentedMode::ViewSwitch,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::SupportFlow,
        M5SplitScopeImpact::SingleTarget,
        "command:support.detail",
    );
    input.selected_segment_label = "  ".to_owned();
    segmented(input)
}

/// Degraded segmented control: masquerades as top-level / stealth navigation.
fn segmented_stealth_nav() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:search:stealth-nav",
        "Results view",
        "List",
        M5SegmentedMode::ViewSwitch,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitScopeImpact::SingleTarget,
        "command:search.view",
    );
    input.masquerades_as_navigation = true;
    segmented(input)
}

/// Degraded segmented control: the selected state is encoded by color alone.
fn segmented_selected_color_only() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:product:color-only",
        "Editor mode",
        "Preview",
        M5SegmentedMode::ModeToggle,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitScopeImpact::SingleTarget,
        "command:product.mode",
    );
    input.selected_state_explicit = false;
    segmented(input)
}

/// Degraded segmented control: keyboard cycling is missing.
fn segmented_keyboard_missing() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:review:keyboard-missing",
        "Diff mode",
        "Inline",
        M5SegmentedMode::ViewSwitch,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitScopeImpact::SingleTarget,
        "command:review.diff",
    );
    input.keyboard_cycling_available = false;
    segmented(input)
}

/// Degraded segmented control: the segment set is oversized, reading as navigation.
fn segmented_oversized() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:product:oversized",
        "Editor mode",
        "Preview",
        M5SegmentedMode::SingleSelectSmallSet,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitScopeImpact::SingleTarget,
        "command:product.mode",
    );
    input.oversized_segment_set = true;
    segmented(input)
}

/// Degraded segmented control: a broadened mode scope breaks review-state continuity.
fn segmented_scope_continuity() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:review:scope-continuity",
        "Diff mode",
        "Inline",
        M5SegmentedMode::ViewSwitch,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::ReviewSheet,
        M5SplitScopeImpact::CrossSurface,
        "command:review.diff",
    );
    input.scope_disclosed = false;
    segmented(input)
}

/// Degraded segmented control: a locked state hides behind generic disabled chrome.
fn segmented_locked_hidden() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:settings:locked-hidden",
        "Sync mode",
        "Manual",
        M5SegmentedMode::ModeToggle,
        M5CoreControlDisposition::Locked,
        M5SplitSegmentedSurfaceContext::SettingsRow,
        M5SplitScopeImpact::SingleTarget,
        "command:settings.sync",
    );
    input.blocked_state_distinct = false;
    segmented(input)
}

/// Degraded segmented control: no command-backed path to inspect the mode toggle is reachable.
fn segmented_trace_missing() -> M5ResolvedSegmentedControl {
    let mut input = clean_segmented_base(
        "segmented:product:trace-missing",
        "Editor mode",
        "Preview",
        M5SegmentedMode::ModeToggle,
        M5CoreControlDisposition::Default,
        M5SplitSegmentedSurfaceContext::PaneHeader,
        M5SplitScopeImpact::SingleTarget,
        "command:product.mode",
    );
    input.command_route_available = false;
    segmented(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SplitSegmentedConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    split_button_examples: Vec<M5ResolvedSplitButton>,
    segmented_control_examples: Vec<M5ResolvedSegmentedControl>,
) -> M5SplitButtonSegmentedControlControlsRow {
    M5SplitButtonSegmentedControlControlsRow {
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
        anatomy_parts: M5SplitSegmentedAnatomyPart::ALL.to_vec(),
        export_fields: M5SplitSegmentedExportField::ALL.to_vec(),
        downgrade_triggers,
        split_button_examples,
        segmented_control_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_SCHEMA_REF,
            M5_SPLIT_BUTTON_SCHEMA_REF,
            M5_SEGMENTED_CONTROL_SCHEMA_REF,
        ]),
        split_buttons_default_to_riskier_alternate: false,
        alternate_actions_hidden_behind_default: false,
        segmented_controls_masquerade_as_navigation: false,
        locked_or_degraded_semantics_hidden_behind_disabled: false,
    }
}

fn controls_rows() -> Vec<M5SplitButtonSegmentedControlControlsRow> {
    use M5CoreControlConsumerSurface as C;
    use M5CoreControlDowngradeTrigger as D;

    vec![
        base_row(
            C::FormsUi,
            "Forms surface owner",
            "The forms surface offers a split button whose safe default click submits with alternates visible in the adjacent menu, and a segmented control toggling a compact layout mode with explicit selected-mode truth; both degrade honestly when the primary or group label is unstated",
            "evidence:m5-split-segmented-forms-ui:001",
            vec![
                D::PlaceholderUsedAsLabel,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![split_primary_safe_clean(), split_primary_unstated()],
            vec![segmented_mode_toggle_clean(), segmented_group_unstated()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface keeps confirm-required and locked split defaults distinct rather than behind generic disabled chrome, and keeps a single-select density toggle small and keyboard-cyclable; both degrade honestly when a lock hides behind disabled",
            "evidence:m5-split-segmented-settings-ui:001",
            vec![
                D::LockedOrDegradedHiddenBehindDisabled,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                split_confirm_clean(),
                split_locked_clean(),
                split_locked_hidden(),
            ],
            vec![
                segmented_single_select_clean(),
                segmented_locked_clean(),
                segmented_locked_hidden(),
            ],
        ),
        base_row(
            C::SearchUi,
            "Search surface owner",
            "The search surface keeps split-button alternates reachable only by explicit selection and never hides an alternate behind the default click, and keeps a results-view toggle a small view switch rather than stealth navigation; both degrade honestly when an alternate is hidden or a toggle masquerades as navigation",
            "evidence:m5-split-segmented-search-ui:001",
            vec![
                D::SplitDefaultedToRiskierAlternate,
                D::StateTaxonomyDrifted,
                D::ProofStale,
            ],
            vec![split_explicit_alternate_clean(), split_alternate_hidden()],
            vec![segmented_view_switch_clean(), segmented_stealth_nav()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review sheet keeps a destructive-guarded merge safe by default with any broadened batch scope disclosed, and keeps a diff-mode toggle keyboard-cyclable with mode-scope continuity preserved; both degrade honestly when stale state promotes a riskier default, a broadened scope is undisclosed, keyboard cycling is missing, or mode-scope continuity breaks",
            "evidence:m5-split-segmented-review-ui:001",
            vec![
                D::SplitDefaultedToRiskierAlternate,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                split_destructive_guarded_clean(),
                split_riskier_default(),
                split_scope_undisclosed(),
            ],
            vec![
                segmented_view_switch_clean(),
                segmented_keyboard_missing(),
                segmented_scope_continuity(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved default-action and selected-mode truth, so an unstated command binding or an unstated selected segment is visible in evidence rather than hidden behind generic chrome",
            "evidence:m5-split-segmented-support-export:001",
            vec![
                D::CommandBindingUnstated,
                D::StateTaxonomyDrifted,
                D::ProofStale,
            ],
            vec![split_support_clean(), split_command_unstated()],
            vec![segmented_support_clean(), segmented_selected_unstated()],
        ),
        base_row(
            C::ProductUi,
            "In-product control owner",
            "In-product surfaces reuse the same safe-default and selected-mode grammar a user sees in forms and settings, always offering the command-backed detail path and degrading honestly when the trace path is missing, an oversized set reads as navigation, or the selected state is color-only",
            "evidence:m5-split-segmented-product-ui:001",
            vec![
                D::CommandBindingUnstated,
                D::StateTaxonomyDrifted,
                D::ProofStale,
            ],
            vec![
                split_product_clean(),
                split_trace_missing(),
                split_emphasis_color_only(),
            ],
            vec![
                segmented_product_clean(),
                segmented_trace_missing(),
                segmented_oversized(),
                segmented_selected_color_only(),
            ],
        ),
    ]
}

fn governance_review() -> M5SplitButtonSegmentedControlGovernanceReview {
    M5SplitButtonSegmentedControlGovernanceReview {
        split_names_primary_label_and_posture: true,
        split_keeps_default_the_safe_action: true,
        split_keeps_alternates_visible_in_adjacent_menu: true,
        split_never_promotes_riskier_alternate_on_stale_state: true,
        segmented_stays_a_small_mode_or_view_toggle: true,
        segmented_never_masquerades_as_navigation: true,
        segmented_exposes_selected_mode_truth: true,
        segmented_offers_keyboard_cycling: true,
        broadened_scope_is_always_disclosed: true,
        locked_and_degraded_never_hidden_behind_disabled: true,
        both_bind_canonical_command_with_trace: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SplitButtonSegmentedControlConsumerProjection {
    M5SplitButtonSegmentedControlConsumerProjection {
        review_surfaces_consume_split_and_segmented_vocabulary: true,
        forms_surfaces_consume_split_vocabulary: true,
        settings_surfaces_consume_segmented_vocabulary: true,
        search_surfaces_consume_segmented_vocabulary: true,
        default_and_mode_facts_trace_to_single_component_contract: true,
        support_export_reads_single_control_source: true,
    }
}

fn proof_freshness() -> M5SplitButtonSegmentedControlProofFreshness {
    M5SplitButtonSegmentedControlProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SplitButtonSegmentedControlReleasePosture {
    M5SplitButtonSegmentedControlReleasePosture {
        proof_packet_ref: M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_SCHEMA_REF,
        M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_SPLIT_BUTTON_SCHEMA_REF,
        M5_SEGMENTED_CONTROL_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 split-button / segmented-control controls packet.
pub fn seeded_m5_split_button_segmented_control_controls(
) -> M5SplitButtonSegmentedControlControlsPacket {
    M5SplitButtonSegmentedControlControlsPacket::new(M5SplitButtonSegmentedControlControlsPacketInput {
        packet_id: M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 split-button and segmented-control controls with safe-by-default primary actions, visible adjacent-menu alternates that never widen risk on stale state, and small mode/view toggles with explicit selected-mode truth and keyboard cycling aligned across forms, settings, search, review, support, and product surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5SplitButtonSegmentedControlVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the review-UI row is held at Beta pending safe-default and scope-disclosure parity
/// on every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_split_button_segmented_control_controls_review_ui_beta_narrowed(
) -> M5SplitButtonSegmentedControlControlsPacket {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.packet_id = "m5-split-button-segmented-control-controls:review-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoreControlConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5CoreControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the search-UI row is narrowed to Preview pending alternate-visibility parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_split_button_segmented_control_controls_search_ui_preview_narrowed(
) -> M5SplitButtonSegmentedControlControlsPacket {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.packet_id =
        "m5-split-button-segmented-control-controls:search-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoreControlConsumerSurface::SearchUi)
        .expect("search-ui row present");
    row.qualification = M5CoreControlQualificationClass::Preview;
    packet
}
