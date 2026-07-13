//! Canonical seed builders for the M5 iconography and illustration registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures. The
//! headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean icon and illustration entries are built so the
//! canonical shell / action / status / navigation / file-type / trust-overlay meaning classes, the
//! accessible-label parity, and the secondary, non-impersonating illustration boundary are proven across the
//! shell, explorer, tab, result-row, onboarding, and support surfaces without any unlabeled icon,
//! boundary-collapse, private grammar, or illustration standing in for operational or security truth.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_ICON_ILLUSTRATION_REGISTRIES_PACKET_ID: &str =
    "m5-iconography-and-illustration-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn icon(input: M5IconEntryResolutionInput) -> M5ResolvedIconEntry {
    resolve_icon_entry(input).expect("seed icon entry resolves")
}

fn illustration(input: M5IllustrationEntryResolutionInput) -> M5ResolvedIllustrationEntry {
    resolve_illustration_entry(input).expect("seed illustration entry resolves")
}

// -- Clean icon entries (semantic, labeled grammar across surfaces) ------------------------------

fn clean_icon_base(
    entry_id: &str,
    token_name: &str,
    iconography_role: M5IconographyRole,
    meaning_class: M5IconMeaningClass,
    surface_context: M5IconIllustrationSurfaceContext,
) -> M5IconEntryResolutionInput {
    M5IconEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualInteractionRole::Icon,
        iconography_role,
        meaning_class,
        surface_context,
        has_accessible_text_equivalent: true,
        reuses_stable_metaphor: true,
        boundary_distinct: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn icon_shell_clean() -> M5ResolvedIconEntry {
    icon(clean_icon_base(
        "icon:shell:chrome",
        "icon.shell.workspace",
        M5IconographyRole::SemanticNotDecorative,
        M5IconMeaningClass::ShellIcon,
        M5IconIllustrationSurfaceContext::Shell,
    ))
}

fn icon_action_clean() -> M5ResolvedIconEntry {
    icon(clean_icon_base(
        "icon:editor:save",
        "icon.action.save",
        M5IconographyRole::ActionIcon,
        M5IconMeaningClass::ActionIcon,
        M5IconIllustrationSurfaceContext::Tab,
    ))
}

fn icon_status_clean() -> M5ResolvedIconEntry {
    icon(clean_icon_base(
        "icon:result:status",
        "icon.status.warning",
        M5IconographyRole::StatusIcon,
        M5IconMeaningClass::StatusIcon,
        M5IconIllustrationSurfaceContext::ResultRow,
    ))
}

fn icon_navigation_clean() -> M5ResolvedIconEntry {
    icon(clean_icon_base(
        "icon:shell:nav",
        "icon.navigation.back",
        M5IconographyRole::NavigationIcon,
        M5IconMeaningClass::NavigationIcon,
        M5IconIllustrationSurfaceContext::Shell,
    ))
}

fn icon_file_type_clean() -> M5ResolvedIconEntry {
    icon(clean_icon_base(
        "icon:explorer:file",
        "icon.file_type.rust_source",
        M5IconographyRole::SemanticNotDecorative,
        M5IconMeaningClass::FileTypeIcon,
        M5IconIllustrationSurfaceContext::Explorer,
    ))
}

fn icon_trust_overlay_clean() -> M5ResolvedIconEntry {
    icon(clean_icon_base(
        "icon:result:trust",
        "icon.trust.verified_signature",
        M5IconographyRole::StatusIcon,
        M5IconMeaningClass::TrustStatusOverlay,
        M5IconIllustrationSurfaceContext::ResultRow,
    ))
}

// -- Degraded icon entries ----------------------------------------------------------------------

/// Degraded icon entry: an uncommon or destructive action uses an unlabeled icon with no text equivalent.
fn icon_unlabeled() -> M5ResolvedIconEntry {
    let mut input = clean_icon_base(
        "icon:shell:unlabeled-destructive",
        "icon.action.purge",
        M5IconographyRole::LabeledForUncommonOrDestructive,
        M5IconMeaningClass::ActionIcon,
        M5IconIllustrationSurfaceContext::Shell,
    );
    input.has_accessible_text_equivalent = false;
    icon(input)
}

/// Degraded icon entry: file-type and shell / status meaning collapse together in a dense surface.
fn icon_boundary_collapsed() -> M5ResolvedIconEntry {
    let mut input = clean_icon_base(
        "icon:explorer:boundary-collapsed",
        "icon.file_type.ambiguous",
        M5IconographyRole::SemanticNotDecorative,
        M5IconMeaningClass::FileTypeIcon,
        M5IconIllustrationSurfaceContext::Explorer,
    );
    input.boundary_distinct = false;
    icon(input)
}

/// Degraded icon entry: the icon meaning class is unclassified.
fn icon_meaning_unclassified() -> M5ResolvedIconEntry {
    icon(clean_icon_base(
        "icon:onboarding:unclassified",
        "icon.unknown.glyph",
        M5IconographyRole::SemanticNotDecorative,
        M5IconMeaningClass::MeaningUnclassified,
        M5IconIllustrationSurfaceContext::Onboarding,
    ))
}

/// Degraded icon entry: a private icon grammar is used instead of a canonical token.
fn icon_private_grammar() -> M5ResolvedIconEntry {
    let mut input = clean_icon_base(
        "icon:marketplace:private-grammar",
        "icon.action.extension_private",
        M5IconographyRole::ActionIcon,
        M5IconMeaningClass::ActionIcon,
        M5IconIllustrationSurfaceContext::Explorer,
    );
    input.references_canonical_token = false;
    icon(input)
}

/// Degraded icon entry: the icon metaphor is not reused stably across commands and surfaces.
fn icon_metaphor_unstable() -> M5ResolvedIconEntry {
    let mut input = clean_icon_base(
        "icon:settings:metaphor-unstable",
        "icon.navigation.forked",
        M5IconographyRole::NavigationIcon,
        M5IconMeaningClass::NavigationIcon,
        M5IconIllustrationSurfaceContext::Shell,
    );
    input.reuses_stable_metaphor = false;
    icon(input)
}

// -- Clean illustration entries ------------------------------------------------------------------

fn clean_illustration_base(
    entry_id: &str,
    token_name: &str,
    illustration_role: M5IllustrationRole,
    placement: M5IllustrationPlacement,
    surface_context: M5IconIllustrationSurfaceContext,
) -> M5IllustrationEntryResolutionInput {
    M5IllustrationEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        illustration_role,
        semantic_role: M5VisualInteractionRole::Illustration,
        placement,
        surface_context,
        stays_secondary_to_content: true,
        never_impersonates_operational_or_security_truth: true,
        replaces_operational_messaging: false,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn illustration_shell_clean() -> M5ResolvedIllustrationEntry {
    illustration(clean_illustration_base(
        "illustration:shell:decorative",
        "illustration.decorative.shell_accent",
        M5IllustrationRole::DecorativeAccent,
        M5IllustrationPlacement::DecorativeAccent,
        M5IconIllustrationSurfaceContext::Shell,
    ))
}

fn illustration_tab_clean() -> M5ResolvedIllustrationEntry {
    illustration(clean_illustration_base(
        "illustration:tab:calm",
        "illustration.decorative.tab_accent",
        M5IllustrationRole::DecorativeAccent,
        M5IllustrationPlacement::CalmNonAnthropomorphic,
        M5IconIllustrationSurfaceContext::Tab,
    ))
}

fn illustration_onboarding_clean() -> M5ResolvedIllustrationEntry {
    illustration(clean_illustration_base(
        "illustration:onboarding:welcome",
        "illustration.onboarding.welcome",
        M5IllustrationRole::OnboardingIllustration,
        M5IllustrationPlacement::OnboardingSecondary,
        M5IconIllustrationSurfaceContext::Onboarding,
    ))
}

fn illustration_explorer_clean() -> M5ResolvedIllustrationEntry {
    illustration(clean_illustration_base(
        "illustration:explorer:empty",
        "illustration.empty_state.explorer",
        M5IllustrationRole::EmptyStateIllustration,
        M5IllustrationPlacement::EmptyStateSecondary,
        M5IconIllustrationSurfaceContext::Explorer,
    ))
}

fn illustration_result_clean() -> M5ResolvedIllustrationEntry {
    illustration(clean_illustration_base(
        "illustration:result:empty",
        "illustration.empty_state.results",
        M5IllustrationRole::EmptyStateIllustration,
        M5IllustrationPlacement::SubordinateToMessaging,
        M5IconIllustrationSurfaceContext::ResultRow,
    ))
}

// -- Degraded illustration entries --------------------------------------------------------------

/// Degraded illustration entry: an illustration impersonates operational or security truth.
fn illustration_impersonates() -> M5ResolvedIllustrationEntry {
    let mut input = clean_illustration_base(
        "illustration:shell:impersonates",
        "illustration.decorative.fake_shield",
        M5IllustrationRole::DecorativeAccent,
        M5IllustrationPlacement::DecorativeAccent,
        M5IconIllustrationSurfaceContext::Shell,
    );
    input.never_impersonates_operational_or_security_truth = false;
    illustration(input)
}

/// Degraded illustration entry: a disallowed operational-truth role stands in for state.
fn illustration_role_operational_truth() -> M5ResolvedIllustrationEntry {
    illustration(clean_illustration_base(
        "illustration:marketplace:operational-truth",
        "illustration.onboarding.fake_status",
        M5IllustrationRole::IllustrationAsOperationalTruthDisallowed,
        M5IllustrationPlacement::OnboardingSecondary,
        M5IconIllustrationSurfaceContext::Onboarding,
    ))
}

/// Degraded illustration entry: an illustration replaces the operational messaging / a trust explanation.
fn illustration_replaces_messaging() -> M5ResolvedIllustrationEntry {
    let mut input = clean_illustration_base(
        "illustration:explorer:replaces-messaging",
        "illustration.empty_state.stands_in",
        M5IllustrationRole::EmptyStateIllustration,
        M5IllustrationPlacement::EmptyStateSecondary,
        M5IconIllustrationSurfaceContext::Explorer,
    );
    input.replaces_operational_messaging = true;
    illustration(input)
}

/// Degraded illustration entry: the illustration is not kept secondary to content.
fn illustration_not_secondary() -> M5ResolvedIllustrationEntry {
    let mut input = clean_illustration_base(
        "illustration:tab:primary",
        "illustration.decorative.overgrown",
        M5IllustrationRole::DecorativeAccent,
        M5IllustrationPlacement::CalmNonAnthropomorphic,
        M5IconIllustrationSurfaceContext::Tab,
    );
    input.stays_secondary_to_content = false;
    illustration(input)
}

/// Degraded illustration entry: no placement is paired with the illustration.
fn illustration_placement_missing() -> M5ResolvedIllustrationEntry {
    let mut input = clean_illustration_base(
        "illustration:onboarding:placement-missing",
        "illustration.onboarding.floating",
        M5IllustrationRole::OnboardingIllustration,
        M5IllustrationPlacement::OnboardingSecondary,
        M5IconIllustrationSurfaceContext::Onboarding,
    );
    input.placement = M5IllustrationPlacement::NoneDisallowed;
    illustration(input)
}

/// Degraded illustration entry: the canonical token name is unstated.
fn illustration_token_unstated() -> M5ResolvedIllustrationEntry {
    let mut input = clean_illustration_base(
        "illustration:result:token-unstated",
        "  ",
        M5IllustrationRole::EmptyStateIllustration,
        M5IllustrationPlacement::SubordinateToMessaging,
        M5IconIllustrationSurfaceContext::ResultRow,
    );
    input.token_name = "  ".to_owned();
    illustration(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5IconIllustrationRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    icon_entries: Vec<M5ResolvedIconEntry>,
    illustration_entries: Vec<M5ResolvedIllustrationEntry>,
) -> M5IconIllustrationRegistriesRow {
    M5IconIllustrationRegistriesRow {
        consumer_surface,
        qualification: M5VisualInteractionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5VisualInteractionDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5VisualInteractionRequiredLabel::Identity,
            M5VisualInteractionRequiredLabel::SemanticRole,
            M5VisualInteractionRequiredLabel::TokenReference,
            M5VisualInteractionRequiredLabel::AccessibleFallback,
        ],
        accessibility_routes: M5VisualInteractionAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5IconIllustrationRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5IconIllustrationRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        icon_entries,
        illustration_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ICON_ILLUSTRATION_REGISTRIES_SCHEMA_REF,
            M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF,
        ]),
        icon_uses_unlabeled_symbol_for_uncommon_or_destructive_action: false,
        file_type_and_shell_status_meaning_collapsed: false,
        illustration_impersonates_operational_or_security_truth: false,
        private_icon_or_illustration_grammar_instead_of_token: false,
    }
}

fn registry_rows() -> Vec<M5IconIllustrationRegistriesRow> {
    use M5VisualInteractionConsumerSurface as C;
    use M5VisualInteractionDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves its chrome and navigation icons through the canonical semantic-labeled grammar and keeps its decorative accent secondary; an unlabeled destructive icon and an illustration that impersonates a security shield both degrade honestly instead of reading as a clean pass",
            "evidence:m5-icon-illustration-shell-ui:001",
            vec![
                D::UnlabeledIconForUncommonOrDestructiveAction,
                D::IllustrationImpersonatedOperationalState,
                D::ProofStale,
            ],
            vec![icon_shell_clean(), icon_navigation_clean(), icon_unlabeled()],
            vec![illustration_shell_clean(), illustration_impersonates()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor renders its action icons in the tab strip with tooltip parity and keeps its tab accent calm and secondary; a file-type icon that collapses into shell / status meaning and an illustration that outgrows its secondary boundary both degrade honestly",
            "evidence:m5-icon-illustration-editor-ui:001",
            vec![
                D::IconSemanticsAmbiguous,
                D::IllustrationImpersonatedOperationalState,
                D::ProofStale,
            ],
            vec![icon_action_clean(), icon_boundary_collapsed()],
            vec![illustration_tab_clean(), illustration_not_secondary()],
        ),
        base_row(
            C::OnboardingUi,
            "Onboarding surface owner",
            "The onboarding wizard renders status icons in result rows and keeps its welcome illustration a secondary onboarding accent; an unclassified icon meaning class and an illustration that carries no placement both degrade honestly instead of standing in for state",
            "evidence:m5-icon-illustration-onboarding-ui:001",
            vec![
                D::IconSemanticsAmbiguous,
                D::IllustrationImpersonatedOperationalState,
                D::ProofStale,
            ],
            vec![icon_status_clean(), icon_meaning_unclassified()],
            vec![
                illustration_onboarding_clean(),
                illustration_placement_missing(),
            ],
        ),
        base_row(
            C::MarketplaceUi,
            "Marketplace / explorer surface owner",
            "The explorer renders distinct file-type icons and keeps its empty-state illustration a secondary accent; a private extension icon grammar inlined instead of a canonical token and an illustration that replaces the operational messaging both degrade honestly",
            "evidence:m5-icon-illustration-marketplace-ui:001",
            vec![
                D::TokenReferenceUnstated,
                D::IllustrationImpersonatedOperationalState,
                D::ProofStale,
            ],
            vec![icon_file_type_clean(), icon_private_grammar()],
            vec![
                illustration_explorer_clean(),
                illustration_replaces_messaging(),
            ],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings and result surfaces render trust / status overlays distinct from shell and file-type icons and keep the empty-results illustration subordinate to the messaging; an unstable icon metaphor and an illustration with an unstated token both degrade honestly",
            "evidence:m5-icon-illustration-settings-ui:001",
            vec![
                D::IconSemanticsAmbiguous,
                D::TokenReferenceUnstated,
                D::ProofStale,
            ],
            vec![icon_trust_overlay_clean(), icon_metaphor_unstable()],
            vec![illustration_result_clean(), illustration_token_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved icon and illustration truth, so an unlabeled icon, a boundary collapse, or an illustration standing in for operational truth is visible in evidence rather than hidden behind a bare glyph",
            "evidence:m5-icon-illustration-support-export:001",
            vec![
                D::IconSemanticsAmbiguous,
                D::IllustrationImpersonatedOperationalState,
                D::ProofStale,
            ],
            vec![icon_shell_clean(), icon_boundary_collapsed()],
            vec![
                illustration_onboarding_clean(),
                illustration_role_operational_truth(),
            ],
        ),
    ]
}

fn governance_review() -> M5IconIllustrationRegistriesGovernanceReview {
    M5IconIllustrationRegistriesGovernanceReview {
        icon_registry_names_token_role_and_meaning_class: true,
        icon_registry_covers_canonical_meaning_classes: true,
        no_unlabeled_icon_for_uncommon_or_destructive_action: true,
        file_type_and_shell_status_meaning_stays_distinct: true,
        illustrations_stay_secondary_and_never_impersonate_truth: true,
        illustrations_name_placement_not_operational_stand_in: true,
        icons_and_illustrations_trace_to_canonical_tokens: true,
        icon_or_illustration_drift_caught_before_release: true,
        first_consumers_use_canonical_icon_grammar: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5IconIllustrationRegistriesConsumerProjection {
    M5IconIllustrationRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        explorer_consumes_shared_registries: true,
        tab_and_result_row_consume_shared_registries: true,
        onboarding_consumes_shared_registries: true,
        icon_meaning_traces_to_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5IconIllustrationRegistriesProofFreshness {
    M5IconIllustrationRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5IconIllustrationRegistriesReleasePosture {
    M5IconIllustrationRegistriesReleasePosture {
        proof_packet_ref: M5_ICON_ILLUSTRATION_REGISTRIES_ARTIFACT_REF.to_owned(),
        interaction_audit_ref: M5_ICON_ILLUSTRATION_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ICON_ILLUSTRATION_REGISTRIES_SCHEMA_REF,
        M5_ICON_ILLUSTRATION_REGISTRIES_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 iconography and illustration registries packet.
pub fn seeded_m5_iconography_and_illustration_registries() -> M5IconIllustrationRegistriesPacket {
    M5IconIllustrationRegistriesPacket::new(M5IconIllustrationRegistriesPacketInput {
        packet_id: M5_ICON_ILLUSTRATION_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 iconography and illustration registries with canonical shell / action / status / navigation / file-type / trust-overlay icon meaning classes, tooltip and accessible-label parity, stable metaphor reuse, distinct file-type-versus-shell/status boundaries, and secondary, non-anthropomorphic illustration that never impersonates operational or security truth across shell, explorer, tab, result-row, onboarding, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5IconIllustrationRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shell-UI row is held at Beta pending accessible-label parity proof on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_iconography_and_illustration_registries_shell_ui_beta_narrowed(
) -> M5IconIllustrationRegistriesPacket {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.packet_id = "m5-iconography-and-illustration-registries:shell-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualInteractionConsumerSurface::ShellUi)
        .expect("shell-ui row present");
    row.qualification = M5VisualInteractionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the onboarding-UI row is narrowed to Preview pending illustration-boundary parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_iconography_and_illustration_registries_onboarding_ui_preview_narrowed(
) -> M5IconIllustrationRegistriesPacket {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.packet_id =
        "m5-iconography-and-illustration-registries:onboarding-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualInteractionConsumerSurface::OnboardingUi)
        .expect("onboarding-ui row present");
    row.qualification = M5VisualInteractionQualificationClass::Preview;
    packet
}
