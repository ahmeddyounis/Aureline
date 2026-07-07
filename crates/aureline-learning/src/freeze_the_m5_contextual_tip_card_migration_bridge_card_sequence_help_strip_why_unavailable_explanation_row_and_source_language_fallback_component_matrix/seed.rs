//! Canonical seed builders for the frozen M5 contextual-teaching component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical contextual-teaching component matrix.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-contextual-teaching-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5TeachingRequiredLabel> {
    M5TeachingRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5TeachingRequiredLabel]) -> Vec<M5TeachingRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5ContextualTeachingComponentFamily,
    qualification: M5TeachingQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5ContextualTeachingComponentRow {
    M5ContextualTeachingComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TeachingSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TeachingDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        tip_trigger_classes: vec![],
        tip_dismissal_states: vec![],
        migration_mapping_classes: vec![],
        source_tool_classes: vec![],
        sequence_help_states: vec![],
        sequence_step_kinds: vec![],
        command_backing_states: vec![],
        blocked_action_owners: vec![],
        unavailable_reason_classes: vec![],
        next_safe_action_classes: vec![],
        source_language_classes: vec![],
        fallback_state_classes: vec![],
        accessibility_routes: M5TeachingAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5TeachingConsumerSurface::OnboardingUi,
            M5TeachingConsumerSurface::SupportExport,
            M5TeachingConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5TeachingDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_command_binding_or_migration_mapping: false,
        hides_blocked_action_owner_or_reason: false,
        invents_alternate_state_label: false,
        severs_source_language_citation: false,
    }
}

fn component_rows() -> Vec<M5ContextualTeachingComponentRow> {
    use M5BlockedActionOwner as BO;
    use M5CommandBackingState as CB;
    use M5ContextualTeachingComponentFamily as F;
    use M5FallbackStateClass as FS;
    use M5MigrationMappingClass as MM;
    use M5NextSafeActionClass as NA;
    use M5SequenceHelpState as SH;
    use M5SequenceStepKind as SK;
    use M5SourceLanguageClass as SL;
    use M5SourceToolClass as ST;
    use M5TeachingConsumerSurface as C;
    use M5TeachingDowngradeTrigger as D;
    use M5TeachingQualificationClass as Q;
    use M5TeachingRequiredLabel as L;
    use M5TipDismissalState as TD;
    use M5TipTriggerClass as TT;
    use M5UnavailableReasonClass as UR;

    let mut rows = Vec::new();

    // 1. Contextual tip card.
    let mut row = base_row(
        F::ContextualTipCard,
        Q::Stable,
        "Contextual tip card owner",
        "One contextual-tip-card model naming why a teaching tip appears (first encounter, feature discovery, error recovery, mode change, idle hint, or contextual follow-up), the stable command that backs it, and how it can be dismissed, so teaching stays contextual, dismissible, and command-backed and never blocks the user or suggests an action it cannot invoke",
        "evidence:m5-contextual-tip-card-parity:001",
        &[
            M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
            M5_CONTEXTUAL_TEACHING_COMPONENT_COMMAND_DESCRIPTOR_REF,
        ],
    );
    row.tip_trigger_classes = TT::ALL.to_vec();
    row.tip_dismissal_states = TD::ALL.to_vec();
    row.command_backing_states = CB::ALL.to_vec();
    row.required_labels = labels_with(&[L::CommandBinding]);
    row.consumer_surfaces = vec![
        C::OnboardingUi,
        C::InlineTipUi,
        C::TourOverlayUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::TipCommandBindingUnstated,
        D::CommandBackingHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Migration bridge card.
    let mut row = base_row(
        F::MigrationBridgeCard,
        Q::Stable,
        "Migration bridge card owner",
        "One migration-bridge-card model naming how an imported behavior maps onto Aureline — exact, native, bridge, shimmed, partial, or unsupported — and the source tool it came from (a legacy editor, a rival IDE, a modal editor, an imported keymap, a migrated workflow config, or an unknown source), so migrated behavior discloses its exact/native/bridge/partial state and imported behavior is never overstated or given an alternate label",
        "evidence:m5-migration-bridge-card-parity:001",
        &[
            M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
            M5_CONTEXTUAL_TEACHING_COMPONENT_IMPORTER_OUTCOME_REF,
        ],
    );
    row.migration_mapping_classes = MM::ALL.to_vec();
    row.source_tool_classes = ST::ALL.to_vec();
    row.required_labels = labels_with(&[L::MigrationAndSourceLanguage]);
    row.consumer_surfaces = vec![
        C::MigrationReportUi,
        C::OnboardingUi,
        C::HelpPanelUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::MigrationMappingUnstated,
        D::SourceToolUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Sequence-help strip.
    let mut row = base_row(
        F::SequenceHelpStrip,
        Q::Stable,
        "Sequence-help strip owner",
        "One sequence-help-strip model naming the state of a keyboard command sequence — ready, awaiting the next key, a partial match, no binding, a conflicting binding, or disabled in context — the step kinds it names, and the stable command that backs it, so command-language help stays keyboard-first and never invents an alternate label for a partial or blocked sequence",
        "evidence:m5-sequence-help-strip-parity:001",
        &[
            M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
            M5_CONTEXTUAL_TEACHING_COMPONENT_KEYBINDING_RESOLVER_REF,
        ],
    );
    row.sequence_help_states = SH::ALL.to_vec();
    row.sequence_step_kinds = SK::ALL.to_vec();
    row.command_backing_states = CB::ALL.to_vec();
    row.required_labels = labels_with(&[L::CommandBinding]);
    row.consumer_surfaces = vec![
        C::CommandPaletteUi,
        C::InlineTipUi,
        C::CliHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SequenceHelpStateUnstated,
        D::CommandBackingHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Why-unavailable explanation row.
    let mut row = base_row(
        F::WhyUnavailableExplanationRow,
        Q::Stable,
        "Why-unavailable explanation row owner",
        "One why-unavailable-explanation-row model naming who owns a blocked action (a policy owner, a workspace admin, a provider service, an upstream dependency, the current user's own scope, or an unknown owner), why it is blocked (policy, missing permission, unmet precondition, feature flag off, offline, or unsupported target), and the next safe action, so a blocked action always names owner, reason, and next safe action and never leaves any of them implicit",
        "evidence:m5-why-unavailable-explanation-row-parity:001",
        &[
            M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
            M5_CONTEXTUAL_TEACHING_COMPONENT_FEATURE_AVAILABILITY_REF,
        ],
    );
    row.blocked_action_owners = BO::ALL.to_vec();
    row.unavailable_reason_classes = UR::ALL.to_vec();
    row.next_safe_action_classes = NA::ALL.to_vec();
    row.required_labels = labels_with(&[L::OwnerReasonAndNextAction]);
    row.consumer_surfaces = vec![
        C::CommandPaletteUi,
        C::HelpPanelUi,
        C::CliHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::BlockedActionOwnerUnstated,
        D::UnavailableReasonUnstated,
        D::NextSafeActionMissing,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Source-language fallback.
    let mut row = base_row(
        F::SourceLanguageFallback,
        Q::Stable,
        "Source-language fallback owner",
        "One source-language-fallback model naming the localization state of the help shown — authored in locale, translated, machine-translated, falling back to source, mixed locale, or untranslated source — and how it preserves canonical IDs and citations while showing fallback content, so localized help never severs a canonical citation and never masquerades as authoritative when it is falling back",
        "evidence:m5-source-language-fallback-parity:001",
        &[
            M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
            M5_CONTEXTUAL_TEACHING_COMPONENT_LOCALE_FALLBACK_REF,
        ],
    );
    row.source_language_classes = SL::ALL.to_vec();
    row.fallback_state_classes = FS::ALL.to_vec();
    row.required_labels = labels_with(&[L::MigrationAndSourceLanguage]);
    row.consumer_surfaces = vec![
        C::HelpPanelUi,
        C::MigrationReportUi,
        C::CliHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SourceLanguageFallbackUnstated,
        D::CitationSevered,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ContextualTeachingComponentGovernanceReview {
    M5ContextualTeachingComponentGovernanceReview {
        tip_card_shows_command_binding_and_dismissal: true,
        migration_card_shows_mapping_class_and_source_tool: true,
        sequence_strip_shows_help_state_and_command_backing: true,
        unavailable_row_shows_owner_reason_and_next_action: true,
        fallback_shows_source_language_and_citation_preserved: true,
        no_surface_invents_alternate_state_label: true,
        migration_mapping_vocabulary_named_once: true,
        blocked_action_owner_and_sequence_help_named_once: true,
        next_safe_action_always_explicit: true,
        command_binding_always_explicit: true,
        dismissal_state_always_explicit: true,
        source_language_citation_never_severed: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ContextualTeachingComponentConsumerProjection {
    M5ContextualTeachingComponentConsumerProjection {
        onboarding_surfaces_consume_tip_vocabulary: true,
        migration_surfaces_consume_mapping_vocabulary: true,
        command_help_surfaces_consume_sequence_vocabulary: true,
        blocked_action_surfaces_consume_owner_reason_vocabulary: true,
        localized_help_surfaces_consume_fallback_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ContextualTeachingComponentProofFreshness {
    M5ContextualTeachingComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ContextualTeachingComponentReleasePosture {
    M5ContextualTeachingComponentReleasePosture {
        proof_packet_ref: M5_CONTEXTUAL_TEACHING_COMPONENT_ARTIFACT_REF.to_owned(),
        teaching_component_audit_ref: M5_CONTEXTUAL_TEACHING_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_DOC_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_COMMAND_DESCRIPTOR_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_IMPORTER_OUTCOME_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_KEYBINDING_RESOLVER_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_FEATURE_AVAILABILITY_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_LOCALE_FALLBACK_REF,
    ])
}

/// Builds the canonical frozen M5 contextual-teaching component matrix packet.
pub fn seeded_m5_contextual_teaching_component_matrix() -> M5ContextualTeachingComponentMatrixPacket
{
    M5ContextualTeachingComponentMatrixPacket::new(M5ContextualTeachingComponentMatrixPacketInput {
        packet_id: M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 contextual-tip-card, migration-bridge-card, sequence-help-strip, why-unavailable-explanation-row, and source-language-fallback component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5ContextualTeachingComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the migration bridge card is held at Beta because a slice of the
/// shimmed/bridge mapping does not yet round-trip across every migration surface; every
/// component stays visible.
pub fn seeded_m5_contextual_teaching_component_matrix_migration_bridge_card_beta_narrowed(
) -> M5ContextualTeachingComponentMatrixPacket {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.packet_id =
        "m5-contextual-teaching-components:migration-bridge-card-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5ContextualTeachingComponentFamily::MigrationBridgeCard
        })
        .expect("migration-bridge-card row present");
    row.qualification = M5TeachingQualificationClass::Beta;
    packet
}

/// Narrowed variant: the source-language fallback surface is narrowed to Preview pending
/// citation-preservation parity proof across every localized surface; every component stays
/// visible.
pub fn seeded_m5_contextual_teaching_component_matrix_source_language_fallback_preview_narrowed(
) -> M5ContextualTeachingComponentMatrixPacket {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.packet_id =
        "m5-contextual-teaching-components:source-language-fallback-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5ContextualTeachingComponentFamily::SourceLanguageFallback
        })
        .expect("source-language-fallback row present");
    row.qualification = M5TeachingQualificationClass::Preview;
    packet
}
