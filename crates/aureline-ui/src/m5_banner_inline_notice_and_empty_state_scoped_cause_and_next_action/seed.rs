//! Canonical seed builders for the M5 banner / empty-state controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code controls, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean banners and empty states are built so
//! the shared scope, cause, what-still-works, next-action, degraded-state-variant, and empty-state
//! purpose grammar is proven across review, settings, update/install, support, shell, and support-export
//! surfaces without any unscoped / color-only notice, generic failure language, missing next action,
//! blank pane, decorative filler, or non-reconstructable explanation.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_BANNER_EMPTY_STATE_CONTROLS_PACKET_ID: &str =
    "m5-banner-inline-notice-and-empty-state-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn banner(input: M5BannerResolutionInput) -> M5ResolvedBanner {
    resolve_banner(input).expect("seed banner input resolves")
}

fn empty_state(input: M5EmptyStateResolutionInput) -> M5ResolvedEmptyState {
    resolve_empty_state(input).expect("seed empty-state input resolves")
}

// -- Clean banner examples (scope grammar and variant coverage across surfaces) ------------------

#[allow(clippy::too_many_arguments)]
fn clean_banner_base(
    banner_id: &str,
    label: &str,
    notice_scope: M5NoticeScope,
    disposition: M5DecisionFeedbackDisposition,
    surface: M5DecisionStateSurfaceContext,
    degraded_variant: M5DegradedStateVariant,
    action_posture: M5BannerActionPosture,
) -> M5BannerResolutionInput {
    M5BannerResolutionInput {
        banner_id: banner_id.to_owned(),
        banner_label: label.to_owned(),
        notice_scope,
        disposition,
        surface_context: surface,
        degraded_variant,
        action_posture,
        cause_named: true,
        what_still_works_stated: true,
        primary_next_action_present: true,
        support_or_help_backlink_present: true,
        avoids_generic_failure_language: true,
        reconstructable_from_export: true,
        proof_fresh: true,
    }
}

/// Clean review banner, page-scoped with a partial-capability variant.
fn banner_review_page() -> M5ResolvedBanner {
    banner(clean_banner_base(
        "banner:review:sync",
        "Review index is showing cached results while the fresh scan finishes",
        M5NoticeScope::PageScoped,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::ReviewWorkspace,
        M5DegradedStateVariant::PartialCapability,
        M5BannerActionPosture::PrimaryNextAction,
    ))
}

/// Clean settings banner, section-scoped with a blocked-by-policy variant.
fn banner_settings_section() -> M5ResolvedBanner {
    banner(clean_banner_base(
        "banner:settings:policy",
        "This capability is blocked by your organization's policy",
        M5NoticeScope::SectionScoped,
        M5DecisionFeedbackDisposition::Blocked,
        M5DecisionStateSurfaceContext::SettingsArea,
        M5DegradedStateVariant::BlockedByPolicy,
        M5BannerActionPosture::SupportBackLink,
    ))
}

/// Clean updates banner, actionable-with-next-step with a stale-data variant.
fn banner_updates_actionable() -> M5ResolvedBanner {
    banner(clean_banner_base(
        "banner:updates:stale",
        "The update catalog is stale; the last refresh failed",
        M5NoticeScope::ActionableWithNextStep,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::UpdatesArea,
        M5DegradedStateVariant::StaleData,
        M5BannerActionPosture::RetryInline,
    ))
}

/// Clean support banner, field-inline with an offline variant.
fn banner_support_field() -> M5ResolvedBanner {
    banner(clean_banner_base(
        "banner:support:offline",
        "Support diagnostics are offline; queued reports will send when reconnected",
        M5NoticeScope::FieldInline,
        M5DecisionFeedbackDisposition::Degraded,
        M5DecisionStateSurfaceContext::SupportArea,
        M5DegradedStateVariant::Offline,
        M5BannerActionPosture::HelpReference,
    ))
}

/// Clean shell banner, global-system with a restricted-access variant.
fn banner_shell_global() -> M5ResolvedBanner {
    banner(clean_banner_base(
        "banner:shell:restricted",
        "Some workspace actions are restricted for this account",
        M5NoticeScope::GlobalSystem,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::EntryStartCenter,
        M5DegradedStateVariant::RestrictedAccess,
        M5BannerActionPosture::DismissAndContinue,
    ))
}

/// Clean support-export banner, page-scoped with a partial-capability variant.
fn banner_export_page() -> M5ResolvedBanner {
    banner(clean_banner_base(
        "banner:support:export",
        "This exported bundle captured partial capability; the reason is recorded",
        M5NoticeScope::PageScoped,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::SupportArea,
        M5DegradedStateVariant::PartialCapability,
        M5BannerActionPosture::SupportBackLink,
    ))
}

// -- Degraded banner examples --------------------------------------------------------------------

/// Degraded banner: it uses generic failure language.
fn banner_generic_failure() -> M5ResolvedBanner {
    let mut input = clean_banner_base(
        "banner:review:generic",
        "Something went wrong",
        M5NoticeScope::PageScoped,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::ReviewWorkspace,
        M5DegradedStateVariant::PartialCapability,
        M5BannerActionPosture::PrimaryNextAction,
    );
    input.avoids_generic_failure_language = false;
    banner(input)
}

/// Degraded banner: the notice scope is the disallowed unscoped / color-only token.
fn banner_unscoped() -> M5ResolvedBanner {
    banner(clean_banner_base(
        "banner:settings:unscoped",
        "A red bar with no scope or cause",
        M5NoticeScope::UnscopedColorOnlyDisallowed,
        M5DecisionFeedbackDisposition::Blocked,
        M5DecisionStateSurfaceContext::SettingsArea,
        M5DegradedStateVariant::BlockedByPolicy,
        M5BannerActionPosture::SupportBackLink,
    ))
}

/// Degraded banner: the degraded-state variant cannot be resolved.
fn banner_variant_unresolved() -> M5ResolvedBanner {
    banner(clean_banner_base(
        "banner:updates:no-variant",
        "The update catalog has a problem",
        M5NoticeScope::ActionableWithNextStep,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::UpdatesArea,
        M5DegradedStateVariant::VariantUnknown,
        M5BannerActionPosture::RetryInline,
    ))
}

/// Degraded banner: the primary next action is missing.
fn banner_next_action_missing() -> M5ResolvedBanner {
    let mut input = clean_banner_base(
        "banner:support:no-next-action",
        "Support diagnostics are offline",
        M5NoticeScope::FieldInline,
        M5DecisionFeedbackDisposition::Degraded,
        M5DecisionStateSurfaceContext::SupportArea,
        M5DegradedStateVariant::Offline,
        M5BannerActionPosture::HelpReference,
    );
    input.primary_next_action_present = false;
    banner(input)
}

/// Degraded banner: what still works is unstated.
fn banner_what_still_works_missing() -> M5ResolvedBanner {
    let mut input = clean_banner_base(
        "banner:shell:no-what-works",
        "Some workspace actions are restricted for this account",
        M5NoticeScope::GlobalSystem,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::EntryStartCenter,
        M5DegradedStateVariant::RestrictedAccess,
        M5BannerActionPosture::DismissAndContinue,
    );
    input.what_still_works_stated = false;
    banner(input)
}

/// Degraded banner: the explanation cannot be reconstructed from the export.
fn banner_not_reconstructable() -> M5ResolvedBanner {
    let mut input = clean_banner_base(
        "banner:support:screenshot-only",
        "This exported bundle captured partial capability",
        M5NoticeScope::PageScoped,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::SupportArea,
        M5DegradedStateVariant::PartialCapability,
        M5BannerActionPosture::SupportBackLink,
    );
    input.reconstructable_from_export = false;
    banner(input)
}

// -- Clean empty-state examples ------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_empty_base(
    empty_state_id: &str,
    label: &str,
    empty_purpose: M5EmptyStatePurpose,
    disposition: M5DecisionFeedbackDisposition,
    surface: M5DecisionStateSurfaceContext,
    degraded_variant: M5DegradedStateVariant,
    empty_reason: M5EmptyStateReason,
) -> M5EmptyStateResolutionInput {
    M5EmptyStateResolutionInput {
        empty_state_id: empty_state_id.to_owned(),
        empty_state_label: label.to_owned(),
        empty_purpose,
        disposition,
        surface_context: surface,
        degraded_variant,
        empty_reason,
        purpose_stated: true,
        emptiness_explained: true,
        best_next_action_present: true,
        avoids_decorative_filler: true,
        avoids_generic_failure_language: true,
        reconstructable_from_export: true,
        proof_fresh: true,
    }
}

/// Clean review empty state explaining its purpose.
fn empty_review_purpose() -> M5ResolvedEmptyState {
    empty_state(clean_empty_base(
        "empty:review:none",
        "No review items yet — approved changes will appear here",
        M5EmptyStatePurpose::ExplainsPurpose,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::ReviewWorkspace,
        M5DegradedStateVariant::PartialCapability,
        M5EmptyStateReason::NeverPopulated,
    ))
}

/// Clean settings empty state offering a next action.
fn empty_settings_next() -> M5ResolvedEmptyState {
    empty_state(clean_empty_base(
        "empty:settings:cleared",
        "No saved capabilities — grant one to get started",
        M5EmptyStatePurpose::OffersNextAction,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::SettingsArea,
        M5DegradedStateVariant::BlockedByPolicy,
        M5EmptyStateReason::AllItemsCleared,
    ))
}

/// Clean updates empty state with first-run guidance.
fn empty_updates_firstrun() -> M5ResolvedEmptyState {
    empty_state(clean_empty_base(
        "empty:updates:first-run",
        "No updates configured yet — connect a catalog to begin",
        M5EmptyStatePurpose::FirstRunGuidance,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::UpdatesArea,
        M5DegradedStateVariant::StaleData,
        M5EmptyStateReason::AwaitingFirstRun,
    ))
}

/// Clean support empty state for a filtered no-results view.
fn empty_support_filtered() -> M5ResolvedEmptyState {
    empty_state(clean_empty_base(
        "empty:support:filtered",
        "No reports match this filter — clear the filter to see all",
        M5EmptyStatePurpose::FilteredNoResults,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::SupportArea,
        M5DegradedStateVariant::Offline,
        M5EmptyStateReason::FilterExcludesAll,
    ))
}

/// Clean shell empty state explaining why it is empty now.
fn empty_shell_emptiness() -> M5ResolvedEmptyState {
    empty_state(clean_empty_base(
        "empty:shell:blocked",
        "No workspaces available — access is blocked upstream",
        M5EmptyStatePurpose::ExplainsCurrentEmptiness,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::EntryStartCenter,
        M5DegradedStateVariant::RestrictedAccess,
        M5EmptyStateReason::BlockedUpstream,
    ))
}

/// Clean support-export empty state explaining its purpose.
fn empty_export_purpose() -> M5ResolvedEmptyState {
    empty_state(clean_empty_base(
        "empty:support:export",
        "No captured evidence in this bundle — the reason is recorded",
        M5EmptyStatePurpose::ExplainsPurpose,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::SupportArea,
        M5DegradedStateVariant::PartialCapability,
        M5EmptyStateReason::NeverPopulated,
    ))
}

// -- Degraded empty-state examples ---------------------------------------------------------------

/// Degraded empty state: it uses generic failure language.
fn empty_generic_failure() -> M5ResolvedEmptyState {
    let mut input = clean_empty_base(
        "empty:review:generic",
        "Something went wrong",
        M5EmptyStatePurpose::ExplainsPurpose,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::ReviewWorkspace,
        M5DegradedStateVariant::PartialCapability,
        M5EmptyStateReason::NeverPopulated,
    );
    input.avoids_generic_failure_language = false;
    empty_state(input)
}

/// Degraded empty state: what the area is for is unstated.
fn empty_purpose_missing() -> M5ResolvedEmptyState {
    let mut input = clean_empty_base(
        "empty:settings:no-purpose",
        "Nothing here",
        M5EmptyStatePurpose::OffersNextAction,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::SettingsArea,
        M5DegradedStateVariant::BlockedByPolicy,
        M5EmptyStateReason::AllItemsCleared,
    );
    input.purpose_stated = false;
    empty_state(input)
}

/// Degraded empty state: the best next action is missing.
fn empty_best_action_missing() -> M5ResolvedEmptyState {
    let mut input = clean_empty_base(
        "empty:updates:no-next-action",
        "No updates configured yet",
        M5EmptyStatePurpose::FirstRunGuidance,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::UpdatesArea,
        M5DegradedStateVariant::StaleData,
        M5EmptyStateReason::AwaitingFirstRun,
    );
    input.best_next_action_present = false;
    empty_state(input)
}

/// Degraded empty state: the emptiness reason cannot be resolved.
fn empty_reason_unresolved() -> M5ResolvedEmptyState {
    empty_state(clean_empty_base(
        "empty:support:no-reason",
        "No reports match this filter",
        M5EmptyStatePurpose::FilteredNoResults,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::SupportArea,
        M5DegradedStateVariant::Offline,
        M5EmptyStateReason::ReasonUnknown,
    ))
}

/// Degraded empty state: decorative marketing filler was used.
fn empty_decorative_filler() -> M5ResolvedEmptyState {
    let mut input = clean_empty_base(
        "empty:shell:filler",
        "Welcome! Discover everything Aureline can do",
        M5EmptyStatePurpose::ExplainsCurrentEmptiness,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionStateSurfaceContext::EntryStartCenter,
        M5DegradedStateVariant::RestrictedAccess,
        M5EmptyStateReason::BlockedUpstream,
    );
    input.avoids_decorative_filler = false;
    empty_state(input)
}

/// Degraded empty state: the explanation cannot be reconstructed from the export.
fn empty_not_reconstructable() -> M5ResolvedEmptyState {
    let mut input = clean_empty_base(
        "empty:support:screenshot-only",
        "No captured evidence in this bundle",
        M5EmptyStatePurpose::ExplainsPurpose,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionStateSurfaceContext::SupportArea,
        M5DegradedStateVariant::PartialCapability,
        M5EmptyStateReason::NeverPopulated,
    );
    input.reconstructable_from_export = false;
    empty_state(input)
}

// -- Row builders --------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5BannerEmptyStateConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    banner_examples: Vec<M5ResolvedBanner>,
    empty_state_examples: Vec<M5ResolvedEmptyState>,
) -> M5BannerEmptyStateControlsRow {
    M5BannerEmptyStateControlsRow {
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
            M5DecisionFeedbackRequiredLabel::Scope,
            M5DecisionFeedbackRequiredLabel::RecoveryPath,
        ],
        accessibility_routes: M5DecisionFeedbackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5BannerEmptyStateAnatomyPart::ALL.to_vec(),
        export_fields: M5BannerEmptyStateExportField::ALL.to_vec(),
        downgrade_triggers,
        banner_examples,
        empty_state_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BANNER_EMPTY_STATE_CONTROLS_SCHEMA_REF,
            M5_BANNER_INLINE_NOTICE_SCHEMA_REF,
            M5_EMPTY_STATE_SCHEMA_REF,
        ]),
        banner_relies_on_color_alone_for_meaning: false,
        banner_uses_generic_failure_language: false,
        empty_state_blanks_pane_without_next_action: false,
        empty_state_uses_decorative_marketing_filler: false,
    }
}

fn controls_rows() -> Vec<M5BannerEmptyStateControlsRow> {
    use M5DecisionFeedbackConsumerSurface as C;
    use M5DecisionFeedbackDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review banner is page-scoped, names its cause and what still works, and exposes a primary next action, and its empty state explains its purpose; both degrade honestly when generic failure language is used",
            "evidence:m5-banner-empty-state-review-ui:001",
            vec![D::GenericChromeWordingUsed, D::RationaleUnstated, D::ProofStale],
            vec![banner_review_page(), banner_generic_failure()],
            vec![empty_review_purpose(), empty_generic_failure()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings banner is section-scoped with a blocked-by-policy variant and a support back-link, and its empty state offers a next action; both degrade honestly when the notice is unscoped / color-only or the purpose is unstated",
            "evidence:m5-banner-empty-state-settings-ui:001",
            vec![D::ColorAloneUsedForMeaning, D::RationaleUnstated, D::ProofStale],
            vec![banner_settings_section(), banner_unscoped()],
            vec![empty_settings_next(), empty_purpose_missing()],
        ),
        base_row(
            C::UpdatesUi,
            "Update / install owner",
            "The updates banner is actionable-with-next-step with a stale-data variant, and its empty state gives first-run guidance; both degrade honestly when the degraded-state variant cannot be resolved or the best next action is missing",
            "evidence:m5-banner-empty-state-updates-ui:001",
            vec![D::ScopeUnstated, D::RecoveryPathUnstated, D::ProofStale],
            vec![banner_updates_actionable(), banner_variant_unresolved()],
            vec![empty_updates_firstrun(), empty_best_action_missing()],
        ),
        base_row(
            C::SupportUi,
            "Support surface owner",
            "The support banner is field-inline with an offline variant and a help reference, and its empty state explains a filtered no-results view; both degrade honestly when the primary next action is missing or the emptiness reason cannot be resolved",
            "evidence:m5-banner-empty-state-support-ui:001",
            vec![D::RecoveryPathUnstated, D::RationaleUnstated, D::ProofStale],
            vec![banner_support_field(), banner_next_action_missing()],
            vec![empty_support_filtered(), empty_reason_unresolved()],
        ),
        base_row(
            C::ShellUi,
            "Shell / entry surface owner",
            "The shell banner is global-system with a restricted-access variant, and its empty state explains why it is empty now; both degrade honestly when what still works is unstated or decorative marketing filler is used",
            "evidence:m5-banner-empty-state-shell-ui:001",
            vec![D::ScopeUnstated, D::GenericChromeWordingUsed, D::ProofStale],
            vec![banner_shell_global(), banner_what_still_works_missing()],
            vec![empty_shell_emptiness(), empty_decorative_filler()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved banner and empty-state truth, so a screenshot-only banner or empty state is visible in evidence rather than hidden, and the reason a pane was empty or bannered can be reconstructed at capture time",
            "evidence:m5-banner-empty-state-support-export:001",
            vec![D::GenericChromeWordingUsed, D::RecoveryPathUnstated, D::ProofStale],
            vec![banner_export_page(), banner_not_reconstructable()],
            vec![empty_export_purpose(), empty_not_reconstructable()],
        ),
    ]
}

fn governance_review() -> M5BannerEmptyStateGovernanceReview {
    M5BannerEmptyStateGovernanceReview {
        banner_states_scope_cause_and_what_still_works: true,
        banner_exposes_primary_next_action: true,
        banner_offers_support_or_help_backlink: true,
        banner_avoids_generic_failure_language: true,
        banner_meaning_never_color_only: true,
        empty_state_states_purpose_and_emptiness: true,
        empty_state_offers_best_next_action: true,
        empty_state_avoids_decorative_filler: true,
        empty_state_never_blank_without_explanation: true,
        both_reconstructable_from_export: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5BannerEmptyStateConsumerProjection {
    M5BannerEmptyStateConsumerProjection {
        review_surfaces_consume_banner_and_empty_state_vocabulary: true,
        settings_surfaces_consume_banner_and_empty_state_vocabulary: true,
        updates_surfaces_consume_banner_vocabulary: true,
        support_surfaces_consume_empty_state_vocabulary: true,
        banner_and_empty_state_trace_to_single_component_contract: true,
        support_export_reads_single_banner_empty_state_source: true,
    }
}

fn proof_freshness() -> M5BannerEmptyStateProofFreshness {
    M5BannerEmptyStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BannerEmptyStateReleasePosture {
    M5BannerEmptyStateReleasePosture {
        proof_packet_ref: M5_BANNER_EMPTY_STATE_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_BANNER_EMPTY_STATE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BANNER_EMPTY_STATE_CONTROLS_SCHEMA_REF,
        M5_BANNER_EMPTY_STATE_CONTROLS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_BANNER_INLINE_NOTICE_SCHEMA_REF,
        M5_EMPTY_STATE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 banner / empty-state controls packet.
pub fn seeded_m5_banner_empty_state_controls() -> M5BannerEmptyStateControlsPacket {
    M5BannerEmptyStateControlsPacket::new(M5BannerEmptyStateControlsPacketInput {
        packet_id: M5_BANNER_EMPTY_STATE_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 banner / inline-notice and empty-state controls with explicit scope, cause, what-still-works, primary next action, and support/help back-links, reusable empty-state cards that state purpose, current emptiness, and best next action, and shared blocked-by-policy / partial / stale / offline / restricted degraded-state variants across review, settings, update/install, support, shell, and support surfaces with no generic-something-went-wrong drift"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5BannerEmptyStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the review-UI row is held at Beta pending banner scope/cause parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_banner_empty_state_controls_review_ui_beta_narrowed(
) -> M5BannerEmptyStateControlsPacket {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.packet_id =
        "m5-banner-inline-notice-and-empty-state-controls:review-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionFeedbackConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the updates-UI row is narrowed to Preview pending empty-state purpose / next-action
/// parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_banner_empty_state_controls_updates_ui_preview_narrowed(
) -> M5BannerEmptyStateControlsPacket {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.packet_id =
        "m5-banner-inline-notice-and-empty-state-controls:updates-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionFeedbackConsumerSurface::UpdatesUi)
        .expect("updates-ui row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Preview;
    packet
}
