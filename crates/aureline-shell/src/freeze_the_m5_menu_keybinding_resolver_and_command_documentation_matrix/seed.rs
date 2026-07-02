//! Canonical seed builders for the frozen M5 menu-affordance,
//! keybinding-resolver, and command-documentation matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical discoverability matrix.
pub const M5_DISCOVERABILITY_MATRIX_PACKET_ID: &str = "m5-discoverability-affordances:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Descriptor revision every surface projects against.
const DESCRIPTOR_REVISION_REF: &str = "commands:m5_command_descriptor:v1";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn binding(
    lifecycle_label: M5LifecycleLabel,
    preview_class: M5PreviewClass,
    disabled_reason_mode: M5DisabledReasonMode,
    help_anchor_ref: &str,
) -> M5CanonicalCommandBinding {
    M5CanonicalCommandBinding {
        command_id_field: "command_id".to_owned(),
        primary_label_ref: "label:command.primary_label_ref".to_owned(),
        help_anchor_ref: help_anchor_ref.to_owned(),
        descriptor_revision_ref: DESCRIPTOR_REVISION_REF.to_owned(),
        lifecycle_label,
        preview_class,
        disabled_reason_mode,
    }
}

#[allow(clippy::too_many_arguments)]
fn surface_row(
    surface_family: M5CommandSurfaceFamily,
    qualification: M5SurfaceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    canonical_command_binding: M5CanonicalCommandBinding,
    required_labels: Vec<M5RequiredLabel>,
    shortcut_source_classes: Vec<M5ShortcutSourceClass>,
    conflict_reasons: Vec<M5ConflictReason>,
    import_translation_states: Vec<M5ImportTranslationState>,
    stale_target_states: Vec<M5StaleTargetState>,
    unavailable_reasons: Vec<M5UnavailableReason>,
    feature_families: Vec<M5FeatureFamily>,
    consumer_surfaces: Vec<M5DiscoveryChannel>,
    downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    required_proof_packet_refs: &[&str],
) -> M5DiscoverabilitySurfaceRow {
    M5DiscoverabilitySurfaceRow {
        surface_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        canonical_command_binding,
        required_labels,
        shortcut_source_classes,
        conflict_reasons,
        import_translation_states,
        stale_target_states,
        unavailable_reasons,
        feature_families,
        parity_surfaces: M5ParitySurface::ALL.to_vec(),
        consumer_surfaces,
        downgrade_triggers,
        required_proof_packet_refs: strings(required_proof_packet_refs),
        source_contract_refs: strings(&[
            M5_DISCOVERABILITY_SCHEMA_REF,
            M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF,
        ]),
        invents_alternate_label: false,
        masks_preview_or_approval: false,
        widens_authority: false,
        hides_disabled_reason: false,
    }
}

/// The four mandatory labels plus the primary label, shared by most surfaces.
fn labels_with_primary() -> Vec<M5RequiredLabel> {
    vec![
        M5RequiredLabel::CommandId,
        M5RequiredLabel::SourceLayer,
        M5RequiredLabel::DisabledReason,
        M5RequiredLabel::LifecycleOrDeprecation,
        M5RequiredLabel::PrimaryLabel,
    ]
}

/// The mandatory labels plus the preview/approval label, for surfaces that
/// invoke high-risk commands.
fn labels_with_preview() -> Vec<M5RequiredLabel> {
    vec![
        M5RequiredLabel::CommandId,
        M5RequiredLabel::SourceLayer,
        M5RequiredLabel::DisabledReason,
        M5RequiredLabel::LifecycleOrDeprecation,
        M5RequiredLabel::PrimaryLabel,
        M5RequiredLabel::PreviewOrApproval,
    ]
}

fn surface_rows() -> Vec<M5DiscoverabilitySurfaceRow> {
    use M5CommandSurfaceFamily as F;
    use M5ConflictReason as X;
    use M5DiscoverabilityDowngradeTrigger as D;
    use M5DiscoveryChannel as C;
    use M5FeatureFamily as FF;
    use M5ImportTranslationState as I;
    use M5ShortcutSourceClass as SS;
    use M5StaleTargetState as T;
    use M5SurfaceQualificationClass as Q;
    use M5UnavailableReason as U;

    let default_triggers = || {
        vec![
            D::AlternateLabelInvented,
            D::CommandIdMissing,
            D::DisabledReasonHidden,
            D::LifecycleOrDeprecationHidden,
            D::ProofStale,
        ]
    };

    vec![
        surface_row(
            F::MenuItem,
            Q::Stable,
            "Shell/menu owner",
            "A single application- or menu-bar item that projects one canonical command; it shows the command's primary label, resolved shortcut source layer, lifecycle/deprecation truth, and a typed disabled reason instead of greying out silently",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::NoPreviewRequired,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/menus/menu-item",
            ),
            labels_with_primary(),
            vec![SS::DefaultKeymap, SS::UserKeybinding],
            vec![],
            vec![],
            vec![T::TargetLive, T::TargetRemovedUnavailable, T::TargetContextLost],
            vec![
                U::NoActiveSelection,
                U::FocusRequiredElsewhere,
                U::CapabilityMissing,
                U::DeprecatedUseReplacement,
            ],
            vec![FF::Notebook, FF::DataApi, FF::ReviewPipeline, FF::Preview],
            vec![C::CommandPalette, C::KeybindingHelp, C::HelpSearch],
            default_triggers(),
            &["evidence:m5-menu-affordance-parity:001"],
        ),
        surface_row(
            F::MenuGroup,
            Q::Stable,
            "Shell/menu owner",
            "A named menu section or submenu that groups related canonical commands under one heading without inventing group-local labels or reordering authority; disabled members keep their typed reasons",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::NoPreviewRequired,
                M5DisabledReasonMode::AlwaysInvokable,
                "help:commands/menus/menu-group",
            ),
            labels_with_primary(),
            vec![],
            vec![],
            vec![],
            vec![T::TargetLive, T::TargetContextLost, T::TargetReplacedByDeprecation],
            vec![
                U::NoActiveSelection,
                U::CapabilityMissing,
                U::ExperimentalNotClaimed,
            ],
            vec![FF::Notebook, FF::DocsBrowser, FF::ReviewPipeline],
            vec![C::CommandPalette, C::HelpSearch],
            default_triggers(),
            &["evidence:m5-menu-affordance-parity:001"],
        ),
        surface_row(
            F::ContextMenu,
            Q::Stable,
            "Shell/context-menu owner",
            "A right-click / long-press context menu that projects the same canonical commands as the menu bar for the focused object, invalidates entries whose target moved or was removed, and never hides preview/approval requirements",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::StructuredDiffPreview,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/menus/context-menu",
            ),
            labels_with_preview(),
            vec![SS::DefaultKeymap, SS::UserKeybinding],
            vec![],
            vec![],
            vec![
                T::TargetLive,
                T::TargetMovedRebound,
                T::TargetRemovedUnavailable,
                T::TargetContextLost,
            ],
            vec![
                U::NoActiveSelection,
                U::PreviewApprovalRequired,
                U::PolicyBlocked,
                U::HigherScopeRequired,
            ],
            vec![FF::Notebook, FF::DataApi, FF::ReviewPipeline, FF::Incident],
            vec![C::CommandPalette, C::KeybindingHelp],
            default_triggers(),
            &["evidence:m5-menu-affordance-parity:001"],
        ),
        surface_row(
            F::CommandBar,
            Q::Stable,
            "Shell/command-bar owner",
            "A contextual command / action bar that surfaces the highest-value canonical commands for the active surface with their resolved shortcuts and preview/approval posture, never widening authority beyond the descriptor",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::StructuredDiffPreview,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/command-bar",
            ),
            labels_with_preview(),
            vec![SS::DefaultKeymap, SS::WorkspaceKeybinding, SS::UserKeybinding],
            vec![],
            vec![],
            vec![T::TargetLive, T::TargetContextLost, T::TargetRemovedUnavailable],
            vec![
                U::FocusRequiredElsewhere,
                U::PreviewApprovalRequired,
                U::CapabilityMissing,
                U::HigherScopeRequired,
            ],
            vec![FF::ReviewPipeline, FF::Preview, FF::Incident, FF::Companion],
            vec![C::CommandPalette, C::KeybindingHelp, C::AiAutomation],
            default_triggers(),
            &["evidence:m5-command-bar-parity:001"],
        ),
        surface_row(
            F::KeybindingResolverLayer,
            Q::Stable,
            "Shell/keybinding owner",
            "The keybinding resolver inspector that names, for a chord, the winning source layer and every shadowed loser drawn from one shortcut-source set, with the resolved command id and its lifecycle truth",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::NoPreviewRequired,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/keybindings/resolver",
            ),
            labels_with_primary(),
            SS::ALL.to_vec(),
            vec![
                X::SameChordDifferentCommand,
                X::HigherLayerShadowed,
                X::ContextScopeOverlap,
                X::PlatformReservedChord,
            ],
            vec![],
            vec![T::TargetLive, T::TargetRemovedUnavailable, T::TargetReplacedByDeprecation],
            vec![
                U::CapabilityMissing,
                U::ExperimentalNotClaimed,
                U::DeprecatedUseReplacement,
            ],
            vec![FF::Notebook, FF::DataApi, FF::ReviewPipeline, FF::Preview],
            vec![C::KeybindingHelp, C::HelpSearch, C::CliHeadless],
            vec![
                D::SourceLayerHidden,
                D::ConflictWinnerAmbiguous,
                D::CommandIdMissing,
                D::ProofStale,
            ],
            &["evidence:m5-keybinding-resolver-snapshot:001"],
        ),
        surface_row(
            F::ConflictReviewSheet,
            Q::Stable,
            "Shell/keybinding owner",
            "The conflict review sheet a user opens when two bindings collide; it names each conflict with a controlled reason, names the winner and losers by source layer, and never leaves the resolved winner ambiguous",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::NoPreviewRequired,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/keybindings/conflicts",
            ),
            labels_with_primary(),
            SS::ALL.to_vec(),
            X::ALL.to_vec(),
            vec![],
            vec![T::TargetLive, T::TargetRemovedUnavailable, T::TargetContextLost],
            vec![
                U::CapabilityMissing,
                U::PolicyBlocked,
                U::DeprecatedUseReplacement,
            ],
            vec![FF::Notebook, FF::ReviewPipeline, FF::Incident],
            vec![C::KeybindingHelp, C::HelpSearch],
            vec![
                D::ConflictWinnerAmbiguous,
                D::SourceLayerHidden,
                D::CommandIdMissing,
                D::ProofStale,
            ],
            &["evidence:m5-keybinding-resolver-snapshot:001"],
        ),
        surface_row(
            F::ImportBridgeRow,
            Q::Stable,
            "Shell/import owner",
            "One row in the keymap import bridge translating a foreign binding to a native canonical command; it reports a controlled translation state, flags collisions and unmapped keys, and rejects authority-widening bindings rather than adopting them silently",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::PolicyAuthoringOrWaiverPreview,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/keybindings/import-bridge",
            ),
            labels_with_preview(),
            SS::ALL.to_vec(),
            vec![
                X::ImportedBindingCollision,
                X::SameChordDifferentCommand,
                X::PlatformReservedChord,
            ],
            I::ALL.to_vec(),
            vec![T::TargetLive, T::TargetRemovedUnavailable, T::TargetReplacedByDeprecation],
            vec![
                U::CapabilityMissing,
                U::HigherScopeRequired,
                U::DeprecatedUseReplacement,
            ],
            vec![FF::Notebook, FF::DataApi, FF::ReviewPipeline],
            vec![C::KeybindingHelp, C::HelpSearch, C::CliHeadless],
            vec![
                D::ImportTranslationUntruthful,
                D::AuthorityWidened,
                D::SourceLayerHidden,
                D::ProofStale,
            ],
            &["evidence:m5-import-bridge-corpus:001"],
        ),
        surface_row(
            F::DisabledCommandExplainer,
            Q::Stable,
            "Shell/command owner",
            "The why-unavailable explainer shown when a command is greyed out; it names one controlled reason, keeps the command id and lifecycle truth visible, and points to the recovery or requirement rather than hiding the command",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::StructuredDiffPreview,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/disabled-explainer",
            ),
            labels_with_preview(),
            vec![],
            vec![],
            vec![],
            vec![
                T::TargetLive,
                T::TargetRemovedUnavailable,
                T::TargetContextLost,
                T::TargetReplacedByDeprecation,
            ],
            U::ALL.to_vec(),
            vec![FF::Notebook, FF::DataApi, FF::ReviewPipeline, FF::Incident, FF::Companion],
            vec![C::CommandPalette, C::KeybindingHelp, C::HelpSearch, C::AiAutomation],
            vec![
                D::DisabledReasonHidden,
                D::PreviewApprovalMasked,
                D::LifecycleOrDeprecationHidden,
                D::ProofStale,
            ],
            &["evidence:m5-disabled-explainer-corpus:001"],
        ),
        surface_row(
            F::LeaderSequenceHelp,
            Q::Beta,
            "Shell/keybinding owner",
            "The leader / multi-key sequence help overlay that lists in-progress sequences and their next keys with the resolved command id and source layer; narrowed to Beta until sequence-prefix parity is proven across all claimed families",
            binding(
                M5LifecycleLabel::Beta,
                M5PreviewClass::NoPreviewRequired,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/keybindings/leader-overlay",
            ),
            labels_with_primary(),
            vec![SS::UserKeybinding, SS::WorkspaceKeybinding, SS::LeaderSequence],
            vec![X::SequencePrefixCollision, X::ContextScopeOverlap],
            vec![],
            vec![T::TargetLive, T::TargetContextLost, T::TargetRemovedUnavailable],
            vec![
                U::FocusRequiredElsewhere,
                U::CapabilityMissing,
                U::ExperimentalNotClaimed,
            ],
            vec![FF::Notebook, FF::ReviewPipeline, FF::Preview],
            vec![C::KeybindingHelp, C::HelpSearch, C::OnboardingTour],
            vec![
                D::ConflictWinnerAmbiguous,
                D::SourceLayerHidden,
                D::CommandIdMissing,
                D::ProofStale,
            ],
            &["evidence:m5-leader-overlay-parity:001"],
        ),
        surface_row(
            F::CommandDocumentationSurface,
            Q::Stable,
            "Docs/command-help owner",
            "The command-documentation / command-detail surface that renders the canonical descriptor's label, aliases, lifecycle/deprecation truth, preview/approval posture, and shortcut source layer so docs never invent a second naming system",
            binding(
                M5LifecycleLabel::Stable,
                M5PreviewClass::NoPreviewRequired,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "help:commands/documentation/command-detail",
            ),
            labels_with_preview(),
            vec![SS::DefaultKeymap, SS::UserKeybinding, SS::LeaderSequence],
            vec![],
            vec![],
            vec![T::TargetLive, T::TargetReplacedByDeprecation, T::TargetRemovedUnavailable],
            vec![
                U::CapabilityMissing,
                U::ExperimentalNotClaimed,
                U::DeprecatedUseReplacement,
                U::PolicyBlocked,
            ],
            vec![FF::Notebook, FF::DataApi, FF::DocsBrowser, FF::ReviewPipeline, FF::Infrastructure],
            vec![C::HelpSearch, C::OnboardingTour, C::CliHeadless, C::AiAutomation],
            vec![
                D::AlternateLabelInvented,
                D::LifecycleOrDeprecationHidden,
                D::PreviewApprovalMasked,
                D::ProofStale,
            ],
            &["evidence:m5-command-doc-packet:001"],
        ),
    ]
}

fn governance_review() -> M5DiscoverabilityGovernanceReview {
    M5DiscoverabilityGovernanceReview {
        all_surfaces_project_one_command_record: true,
        no_surface_invents_alternate_label: true,
        no_surface_widens_authority: true,
        no_surface_hides_disabled_reason: true,
        no_surface_masks_preview_or_approval: true,
        every_surface_shows_mandatory_labels: true,
        keybinding_winners_and_losers_named: true,
        import_bridge_outcomes_controlled: true,
        stale_targets_invalidated: true,
        cross_modality_parity_preserved: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5DiscoverabilityConsumerProjection {
    M5DiscoverabilityConsumerProjection {
        command_palette_consumes_matrix: true,
        keybinding_help_shows_source_and_conflicts: true,
        help_search_uses_controlled_vocabulary: true,
        onboarding_tour_quotes_command_ids: true,
        cli_headless_explains_same_semantics: true,
        ai_automation_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DiscoverabilityProofFreshness {
    M5DiscoverabilityProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DiscoverabilityReleasePosture {
    M5DiscoverabilityReleasePosture {
        release_packet_ref:
            "artifacts/release/m5-discoverability-affordances-proof/support_export.json".to_owned(),
        command_parity_audit_ref:
            "artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md".to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DISCOVERABILITY_SCHEMA_REF,
        M5_DISCOVERABILITY_DOC_REF,
        M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF,
        M5_DISCOVERABILITY_KEYBINDING_RESOLVER_REF,
        M5_DISCOVERABILITY_MENU_ITEM_REF,
        M5_DISCOVERABILITY_LEADER_OVERLAY_REF,
    ])
}

/// Builds the canonical frozen M5 discoverability-affordance matrix packet.
pub fn seeded_m5_discoverability_matrix() -> M5DiscoverabilityMatrixPacket {
    M5DiscoverabilityMatrixPacket::new(M5DiscoverabilityMatrixPacketInput {
        packet_id: M5_DISCOVERABILITY_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 menu-affordance, keybinding-resolver, and command-documentation matrix"
            .to_owned(),
        surface_rows: surface_rows(),
        vocabulary_set: M5DiscoverabilityVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the import-bridge row is held at Beta because a slice of
/// imported bindings translate only approximately; every surface stays visible.
pub fn seeded_m5_discoverability_matrix_imported_keymap_approximated_narrowed(
) -> M5DiscoverabilityMatrixPacket {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.packet_id =
        "m5-discoverability-affordances:imported-keymap-approximated:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::ImportBridgeRow)
        .expect("import-bridge row present");
    row.qualification = M5SurfaceQualificationClass::Beta;
    row.canonical_command_binding.lifecycle_label = M5LifecycleLabel::Beta;
    packet
}

/// Narrowed variant: the leader/sequence help overlay is narrowed to Preview
/// pending sequence-prefix parity proof; every surface stays visible.
pub fn seeded_m5_discoverability_matrix_leader_sequence_help_preview_narrowed(
) -> M5DiscoverabilityMatrixPacket {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.packet_id =
        "m5-discoverability-affordances:leader-sequence-help-preview:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::LeaderSequenceHelp)
        .expect("leader-sequence-help row present");
    row.qualification = M5SurfaceQualificationClass::Preview;
    packet
}
