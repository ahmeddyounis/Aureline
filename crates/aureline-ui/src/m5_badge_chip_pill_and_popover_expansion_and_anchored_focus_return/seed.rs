//! Canonical seed builders for the M5 badge / popover controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean badges
//! and popovers are built so the shared badge-taxonomy, plain-language-expansion, and popover
//! focus-return grammar is proven across help, settings, review, marketplace, repair, and support
//! surfaces without any color-only meaning, hover-only truth gap, taxonomy drift, popover that carries
//! the only critical instruction, or popover that loses focus return.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_BADGE_POPOVER_CONTROLS_PACKET_ID: &str =
    "m5-badge-chip-pill-and-popover-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn badge(input: M5BadgeResolutionInput) -> M5ResolvedBadge {
    resolve_badge(input).expect("seed badge input resolves")
}

fn popover(input: M5PopoverResolutionInput) -> M5ResolvedPopover {
    resolve_popover(input).expect("seed popover input resolves")
}

// -- Clean badge examples (expression / taxonomy grammar across surfaces) -----------------------

#[allow(clippy::too_many_arguments)]
fn clean_badge_base(
    badge_id: &str,
    label: &str,
    expression: M5BadgeExpression,
    disposition: M5DecisionFeedbackDisposition,
    taxonomy: M5BadgeMeaningTaxonomy,
    overflow: M5BadgeOverflowBehavior,
    route: M5BadgeExpansionRoute,
    surface: M5DecisionSurfaceContext,
) -> M5BadgeResolutionInput {
    M5BadgeResolutionInput {
        badge_id: badge_id.to_owned(),
        badge_label: label.to_owned(),
        expression,
        disposition,
        meaning_taxonomy: taxonomy,
        overflow_behavior: overflow,
        expansion_route: route,
        surface_context: surface,
        meaning_stated_non_color_only: true,
        plain_language_explanation_present: true,
        explanation_reachable_by_keyboard_sr_export: true,
        taxonomy_stable_across_surfaces: true,
        proof_fresh: true,
    }
}

/// Clean lifecycle status badge in a help panel.
fn badge_lifecycle_clean() -> M5ResolvedBadge {
    badge(clean_badge_base(
        "badge:help:lifecycle",
        "Beta",
        M5BadgeExpression::StatusWord,
        M5DecisionFeedbackDisposition::Info,
        M5BadgeMeaningTaxonomy::LifecycleState,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::DisclosureDrawer,
        M5DecisionSurfaceContext::HelpPanel,
    ))
}

/// Clean support-class badge in a settings row.
fn badge_support_clean() -> M5ResolvedBadge {
    badge(clean_badge_base(
        "badge:settings:support",
        "Supported",
        M5BadgeExpression::IconWithText,
        M5DecisionFeedbackDisposition::Success,
        M5BadgeMeaningTaxonomy::SupportClass,
        M5BadgeOverflowBehavior::TruncatesWithExpansion,
        M5BadgeExpansionRoute::InlineTextExpansion,
        M5DecisionSurfaceContext::SettingsRow,
    ))
}

/// Clean policy-source badge in a settings row.
fn badge_policy_clean() -> M5ResolvedBadge {
    badge(clean_badge_base(
        "badge:settings:policy",
        "Set by policy",
        M5BadgeExpression::TextLabel,
        M5DecisionFeedbackDisposition::Info,
        M5BadgeMeaningTaxonomy::PolicySource,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::HelpReference,
        M5DecisionSurfaceContext::SettingsRow,
    ))
}

/// Clean provider-origin badge in a marketplace listing.
fn badge_provider_clean() -> M5ResolvedBadge {
    badge(clean_badge_base(
        "badge:marketplace:provider",
        "Verified publisher",
        M5BadgeExpression::IconWithText,
        M5DecisionFeedbackDisposition::Success,
        M5BadgeMeaningTaxonomy::ProviderOrigin,
        M5BadgeOverflowBehavior::CollapsesToCountWithExpansion,
        M5BadgeExpansionRoute::LinkedDetailPopover,
        M5DecisionSurfaceContext::MarketplaceListing,
    ))
}

/// Clean source-freshness badge in a review sheet.
fn badge_freshness_clean() -> M5ResolvedBadge {
    badge(clean_badge_base(
        "badge:review:freshness",
        "3 sources · updated today",
        M5BadgeExpression::CountWithLabel,
        M5DecisionFeedbackDisposition::Info,
        M5BadgeMeaningTaxonomy::SourceFreshness,
        M5BadgeOverflowBehavior::WrapsToPlainLanguage,
        M5BadgeExpansionRoute::ScreenReaderDescription,
        M5DecisionSurfaceContext::ReviewSheet,
    ))
}

/// Clean removable lifecycle chip in a help panel.
fn badge_removable_clean() -> M5ResolvedBadge {
    badge(clean_badge_base(
        "badge:help:removable",
        "Deprecated",
        M5BadgeExpression::RemovableChip,
        M5DecisionFeedbackDisposition::Warning,
        M5BadgeMeaningTaxonomy::LifecycleState,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::DisclosureDrawer,
        M5DecisionSurfaceContext::HelpPanel,
    ))
}

/// Clean support-class badge in a repair flow (used by the support-export row).
fn badge_support_repair_clean() -> M5ResolvedBadge {
    badge(clean_badge_base(
        "badge:support:evidence",
        "Evidence fresh",
        M5BadgeExpression::IconWithText,
        M5DecisionFeedbackDisposition::Success,
        M5BadgeMeaningTaxonomy::SupportClass,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::InlineTextExpansion,
        M5DecisionSurfaceContext::RepairFlow,
    ))
}

// -- Degraded badge examples --------------------------------------------------------------------

/// Degraded badge: the meaning is encoded by color alone.
fn badge_color_only() -> M5ResolvedBadge {
    let mut input = clean_badge_base(
        "badge:help:color-only",
        "Status",
        M5BadgeExpression::StatusWord,
        M5DecisionFeedbackDisposition::Warning,
        M5BadgeMeaningTaxonomy::LifecycleState,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::DisclosureDrawer,
        M5DecisionSurfaceContext::HelpPanel,
    );
    input.meaning_stated_non_color_only = false;
    badge(input)
}

/// Degraded badge: the plain-language explanation is reachable only on hover.
fn badge_hover_only() -> M5ResolvedBadge {
    let mut input = clean_badge_base(
        "badge:settings:hover-only",
        "Managed",
        M5BadgeExpression::TextLabel,
        M5DecisionFeedbackDisposition::Info,
        M5BadgeMeaningTaxonomy::PolicySource,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::InlineTextExpansion,
        M5DecisionSurfaceContext::SettingsRow,
    );
    input.explanation_reachable_by_keyboard_sr_export = false;
    badge(input)
}

/// Degraded badge: the plain-language explanation is missing.
fn badge_plain_missing() -> M5ResolvedBadge {
    let mut input = clean_badge_base(
        "badge:support:plain-missing",
        "Held",
        M5BadgeExpression::StatusWord,
        M5DecisionFeedbackDisposition::Blocked,
        M5BadgeMeaningTaxonomy::SupportClass,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::RouteUnknown,
        M5DecisionSurfaceContext::RepairFlow,
    );
    input.expansion_route = M5BadgeExpansionRoute::HelpReference;
    input.plain_language_explanation_present = false;
    badge(input)
}

/// Degraded badge: the meaning taxonomy drifted across surfaces.
fn badge_taxonomy_drift() -> M5ResolvedBadge {
    let mut input = clean_badge_base(
        "badge:review:taxonomy-drift",
        "Fresh",
        M5BadgeExpression::TextLabel,
        M5DecisionFeedbackDisposition::Info,
        M5BadgeMeaningTaxonomy::SourceFreshness,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::DisclosureDrawer,
        M5DecisionSurfaceContext::ReviewSheet,
    );
    input.taxonomy_stable_across_surfaces = false;
    badge(input)
}

/// Degraded badge: the concise label is unstated.
fn badge_label_unstated() -> M5ResolvedBadge {
    let mut input = clean_badge_base(
        "badge:marketplace:no-label",
        "  ",
        M5BadgeExpression::IconWithText,
        M5DecisionFeedbackDisposition::Info,
        M5BadgeMeaningTaxonomy::ProviderOrigin,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::LinkedDetailPopover,
        M5DecisionSurfaceContext::MarketplaceListing,
    );
    input.badge_label = "  ".to_owned();
    badge(input)
}

/// Degraded badge: the meaning is unclassified (not in the preserved taxonomy).
fn badge_taxonomy_unclassified() -> M5ResolvedBadge {
    badge(clean_badge_base(
        "badge:support:unclassified",
        "New",
        M5BadgeExpression::StatusWord,
        M5DecisionFeedbackDisposition::Info,
        M5BadgeMeaningTaxonomy::TaxonomyUnclassified,
        M5BadgeOverflowBehavior::NoOverflow,
        M5BadgeExpansionRoute::DisclosureDrawer,
        M5DecisionSurfaceContext::RepairFlow,
    ))
}

// -- Clean popover examples ---------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_popover_base(
    popover_id: &str,
    accessible_name: &str,
    dismissal: M5PopoverDismissal,
    disposition: M5DecisionFeedbackDisposition,
    surface: M5DecisionSurfaceContext,
) -> M5PopoverResolutionInput {
    M5PopoverResolutionInput {
        popover_id: popover_id.to_owned(),
        accessible_name: accessible_name.to_owned(),
        dismissal,
        disposition,
        surface_context: surface,
        is_dismissible: true,
        keyboard_operable: true,
        focus_returns_to_trigger: true,
        carries_only_critical_instruction: false,
        critical_steps_available_elsewhere: true,
        is_non_modal_secondary: true,
        content_reachable_without_hover: true,
        proof_fresh: true,
    }
}

/// Clean help popover that dismisses on Escape and returns focus to its trigger.
fn popover_help_clean() -> M5ResolvedPopover {
    popover(clean_popover_base(
        "popover:help:glossary",
        "What does Beta mean?",
        M5PopoverDismissal::DismissOnEscape,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::HelpPanel,
    ))
}

/// Clean settings popover with an explicit close button.
fn popover_settings_clean() -> M5ResolvedPopover {
    popover(clean_popover_base(
        "popover:settings:policy-detail",
        "Why is this managed?",
        M5PopoverDismissal::ExplicitCloseButton,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::SettingsRow,
    ))
}

/// Clean review popover that returns focus to its trigger.
fn popover_review_clean() -> M5ResolvedPopover {
    popover(clean_popover_base(
        "popover:review:source-detail",
        "Source details",
        M5PopoverDismissal::FocusReturnsToTrigger,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::ReviewSheet,
    ))
}

/// Clean marketplace popover that stays a non-modal secondary surface.
fn popover_marketplace_clean() -> M5ResolvedPopover {
    popover(clean_popover_base(
        "popover:marketplace:publisher",
        "Publisher details",
        M5PopoverDismissal::NonModalSecondary,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::MarketplaceListing,
    ))
}

/// Clean repair popover that dismisses on an outside click.
fn popover_repair_clean() -> M5ResolvedPopover {
    popover(clean_popover_base(
        "popover:repair:hint",
        "How to retry",
        M5PopoverDismissal::DismissOnOutsideClick,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::RepairFlow,
    ))
}

/// Clean support popover used by the support-export row.
fn popover_support_clean() -> M5ResolvedPopover {
    popover(clean_popover_base(
        "popover:support:evidence",
        "Evidence freshness",
        M5PopoverDismissal::DismissOnEscape,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::RepairFlow,
    ))
}

// -- Degraded popover examples ------------------------------------------------------------------

/// Degraded popover: focus does not return to the trigger when closed.
fn popover_no_focus_return() -> M5ResolvedPopover {
    let mut input = clean_popover_base(
        "popover:help:no-focus",
        "What does Beta mean?",
        M5PopoverDismissal::DismissOnEscape,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::HelpPanel,
    );
    input.focus_returns_to_trigger = false;
    popover(input)
}

/// Degraded popover: it carries the only critical workflow instruction.
fn popover_carries_instruction() -> M5ResolvedPopover {
    let mut input = clean_popover_base(
        "popover:settings:only-instruction",
        "Enter recovery code",
        M5PopoverDismissal::ExplicitCloseButton,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionSurfaceContext::SettingsRow,
    );
    input.carries_only_critical_instruction = true;
    popover(input)
}

/// Degraded popover: critical workflow steps are trapped solely inside the popover.
fn popover_trapped_steps() -> M5ResolvedPopover {
    let mut input = clean_popover_base(
        "popover:review:trapped",
        "Approve steps",
        M5PopoverDismissal::FocusReturnsToTrigger,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionSurfaceContext::ReviewSheet,
    );
    input.critical_steps_available_elsewhere = false;
    popover(input)
}

/// Degraded popover: it is not dismissible.
fn popover_not_dismissible() -> M5ResolvedPopover {
    let mut input = clean_popover_base(
        "popover:marketplace:sticky",
        "Publisher details",
        M5PopoverDismissal::NonModalSecondary,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::MarketplaceListing,
    );
    input.is_dismissible = false;
    popover(input)
}

/// Degraded popover: its content is reachable only on hover.
fn popover_hover_only() -> M5ResolvedPopover {
    let mut input = clean_popover_base(
        "popover:repair:hover-only",
        "How to retry",
        M5PopoverDismissal::DismissOnOutsideClick,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionSurfaceContext::RepairFlow,
    );
    input.content_reachable_without_hover = false;
    popover(input)
}

/// Degraded popover: it names the disallowed carries-only-instruction dismissal token.
fn popover_dismissal_disallowed() -> M5ResolvedPopover {
    popover(clean_popover_base(
        "popover:support:disallowed",
        "Evidence freshness",
        M5PopoverDismissal::CarriesOnlyInstructionDisallowed,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionSurfaceContext::RepairFlow,
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5BadgePopoverConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    badge_examples: Vec<M5ResolvedBadge>,
    popover_examples: Vec<M5ResolvedPopover>,
) -> M5BadgePopoverControlsRow {
    M5BadgePopoverControlsRow {
        consumer_surface,
        qualification: M5DecisionFeedbackQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5DecisionFeedbackDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5DecisionFeedbackRequiredLabel::Identity,
            M5DecisionFeedbackRequiredLabel::State,
            M5DecisionFeedbackRequiredLabel::KeyboardRoute,
            M5DecisionFeedbackRequiredLabel::Rationale,
        ],
        accessibility_routes: M5DecisionFeedbackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5BadgePopoverAnatomyPart::ALL.to_vec(),
        export_fields: M5BadgePopoverExportField::ALL.to_vec(),
        downgrade_triggers,
        badge_examples,
        popover_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BADGE_POPOVER_CONTROLS_SCHEMA_REF,
            M5_BADGE_CHIP_PILL_SCHEMA_REF,
            M5_POPOVER_SCHEMA_REF,
        ]),
        badge_meaning_relies_on_color_alone: false,
        badge_meaning_hidden_behind_hover_only: false,
        popover_carries_only_critical_instruction: false,
        popover_fails_to_return_focus_to_trigger: false,
    }
}

fn controls_rows() -> Vec<M5BadgePopoverControlsRow> {
    use M5DecisionFeedbackConsumerSurface as C;
    use M5DecisionFeedbackDowngradeTrigger as D;

    vec![
        base_row(
            C::HelpUi,
            "Help surface owner",
            "The help panel expands every lifecycle badge into a plain-language explanation reachable off-hover and keeps its glossary popover dismissible with anchored focus return; both degrade honestly when meaning is color-only or focus does not return to the trigger",
            "evidence:m5-badge-popover-help-ui:001",
            vec![
                D::ColorAloneUsedForMeaning,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![badge_lifecycle_clean(), badge_removable_clean(), badge_color_only()],
            vec![popover_help_clean(), popover_no_focus_return()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface keeps support-class and policy-source badges classified and reachable by keyboard and screen reader, and never lets a policy popover carry the only critical instruction; both degrade honestly when the explanation is hover-only or the popover carries the only instruction",
            "evidence:m5-badge-popover-settings-ui:001",
            vec![
                D::ColorAloneUsedForMeaning,
                D::PopoverCarriedOnlyCriticalInstruction,
                D::ProofStale,
            ],
            vec![badge_support_clean(), badge_policy_clean(), badge_hover_only()],
            vec![popover_settings_clean(), popover_carries_instruction()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review sheet keeps freshness badges classified and stable across surfaces and keeps its source popover a lightweight secondary control that never traps critical steps; both degrade honestly when the taxonomy drifts or critical steps are trapped inside the popover",
            "evidence:m5-badge-popover-review-ui:001",
            vec![
                D::StateTaxonomyDrifted,
                D::PopoverCarriedOnlyCriticalInstruction,
                D::ProofStale,
            ],
            vec![badge_freshness_clean(), badge_taxonomy_drift()],
            vec![popover_review_clean(), popover_trapped_steps()],
        ),
        base_row(
            C::UpdatesUi,
            "Marketplace / updates owner",
            "The marketplace listing keeps provider-origin badges legible with concise text and a named expansion path and keeps its publisher popover dismissible; both degrade honestly when the badge label is unstated or the popover is not dismissible",
            "evidence:m5-badge-popover-updates-ui:001",
            vec![
                D::RationaleUnstated,
                D::RecoveryPathUnstated,
                D::ProofStale,
            ],
            vec![badge_provider_clean(), badge_label_unstated()],
            vec![popover_marketplace_clean(), popover_not_dismissible()],
        ),
        base_row(
            C::SupportUi,
            "Repair / support surface owner",
            "The repair flow keeps support-class badges classified and reachable and keeps its hint popover reachable without hover; both degrade honestly when the meaning is unclassified or the popover content is hover-only",
            "evidence:m5-badge-popover-support-ui:001",
            vec![
                D::StateTaxonomyDrifted,
                D::ColorAloneUsedForMeaning,
                D::ProofStale,
            ],
            vec![badge_support_repair_clean(), badge_taxonomy_unclassified()],
            vec![popover_repair_clean(), popover_hover_only()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved badge and popover truth, so a missing plain-language explanation or a disallowed popover dismissal model is visible in evidence rather than hidden behind color or hover",
            "evidence:m5-badge-popover-support-export:001",
            vec![
                D::RationaleUnstated,
                D::PopoverCarriedOnlyCriticalInstruction,
                D::ProofStale,
            ],
            vec![badge_support_repair_clean(), badge_plain_missing()],
            vec![popover_support_clean(), popover_dismissal_disallowed()],
        ),
    ]
}

fn governance_review() -> M5BadgePopoverGovernanceReview {
    M5BadgePopoverGovernanceReview {
        badge_names_label_and_meaning: true,
        badge_expands_to_plain_language: true,
        badge_never_relies_on_color_alone: true,
        badge_meaning_reachable_by_keyboard_sr_export: true,
        badge_preserves_taxonomy_across_surfaces: true,
        popover_stays_lightweight_secondary: true,
        popover_is_dismissible_with_anchored_focus_return: true,
        popover_never_carries_only_critical_instruction: true,
        popover_content_reachable_without_hover: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5BadgePopoverConsumerProjection {
    M5BadgePopoverConsumerProjection {
        help_surfaces_consume_badge_vocabulary: true,
        settings_surfaces_consume_badge_vocabulary: true,
        review_surfaces_consume_badge_and_popover_vocabulary: true,
        marketplace_surfaces_consume_badge_vocabulary: true,
        badge_meaning_traces_to_single_component_contract: true,
        support_export_reads_single_badge_source: true,
    }
}

fn proof_freshness() -> M5BadgePopoverProofFreshness {
    M5BadgePopoverProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BadgePopoverReleasePosture {
    M5BadgePopoverReleasePosture {
        proof_packet_ref: M5_BADGE_POPOVER_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_BADGE_POPOVER_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BADGE_POPOVER_CONTROLS_SCHEMA_REF,
        M5_BADGE_POPOVER_CONTROLS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_BADGE_CHIP_PILL_SCHEMA_REF,
        M5_POPOVER_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 badge / popover controls packet.
pub fn seeded_m5_badge_popover_controls() -> M5BadgePopoverControlsPacket {
    M5BadgePopoverControlsPacket::new(M5BadgePopoverControlsPacketInput {
        packet_id: M5_BADGE_POPOVER_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 badge / chip / pill and popover controls with concise text, overflow rules, plain-language expansion off color and hover, preserved lifecycle/support/provider/policy/source/freshness taxonomy, and lightweight popovers with anchored focus return across help, settings, review, marketplace, repair, and support surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5BadgePopoverVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the help-UI row is held at Beta pending badge-taxonomy parity on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_badge_popover_controls_help_ui_beta_narrowed() -> M5BadgePopoverControlsPacket {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.packet_id = "m5-badge-chip-pill-and-popover-controls:help-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionFeedbackConsumerSurface::HelpUi)
        .expect("help-ui row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review-UI row is narrowed to Preview pending popover focus-return parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_badge_popover_controls_review_ui_preview_narrowed() -> M5BadgePopoverControlsPacket
{
    let mut packet = seeded_m5_badge_popover_controls();
    packet.packet_id = "m5-badge-chip-pill-and-popover-controls:review-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionFeedbackConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Preview;
    packet
}
