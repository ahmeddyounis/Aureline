//! Canonical seed builders for the M5 why-unavailable / source-language primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical why-unavailable / source-language primitive packet.
pub const M5_BLOCKED_LOCALIZED_ROW_PACKET_ID: &str =
    "m5-why-unavailable-source-language-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked why-unavailable-row resolution case.
#[allow(clippy::too_many_arguments)]
fn why_case(
    blocked_action_ref: &str,
    unavailable_reason: M5UnavailableReasonClass,
    blocking_owner: M5BlockedActionOwner,
    next_safe_action: M5NextSafeActionClass,
    next_safe_action_ref: Option<&str>,
    deeper_docs_ref: &str,
    screen_reader_announcement: &str,
    row_identity_ref: &str,
) -> M5WhyUnavailableRowResolutionCase {
    M5WhyUnavailableRowResolutionCase::resolved(M5WhyUnavailableRowResolutionInput {
        blocked_action_ref: blocked_action_ref.to_owned(),
        unavailable_reason,
        blocking_owner,
        next_safe_action,
        next_safe_action_ref: next_safe_action_ref.map(str::to_owned),
        deeper_docs_ref: deeper_docs_ref.to_owned(),
        screen_reader_announcement: screen_reader_announcement.to_owned(),
        row_identity_ref: row_identity_ref.to_owned(),
    })
}

/// Builds a worked source-language-fallback resolution case.
#[allow(clippy::too_many_arguments)]
fn source_case(
    source_language_class: M5SourceLanguageClass,
    fallback_state: M5FallbackStateClass,
    display_locale: &str,
    stable_id_ref: &str,
    canonical_citation_ref: &str,
    source_language_text_ref: Option<&str>,
    screen_reader_announcement: &str,
    row_identity_ref: &str,
) -> M5SourceLanguageFallbackResolutionCase {
    M5SourceLanguageFallbackResolutionCase::resolved(M5SourceLanguageFallbackResolutionInput {
        source_language_class,
        fallback_state,
        display_locale: display_locale.to_owned(),
        stable_id_ref: stable_id_ref.to_owned(),
        canonical_citation_ref: canonical_citation_ref.to_owned(),
        source_language_text_ref: source_language_text_ref.map(str::to_owned),
        screen_reader_announcement: screen_reader_announcement.to_owned(),
        row_identity_ref: row_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full why-unavailable / source-language
/// anatomy, owner, reason, next-safe-action, posture, action, export-field, and accessibility
/// parity every consumer carries.
fn base_row(
    consumer_surface: M5BlockedLocalizedConsumerSurface,
    qualification: M5TeachingQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    why_unavailable_examples: Vec<M5WhyUnavailableRowResolutionCase>,
    source_language_examples: Vec<M5SourceLanguageFallbackResolutionCase>,
) -> M5BlockedLocalizedConsumerRow {
    M5BlockedLocalizedConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TeachingSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TeachingDeploymentLine::ALL.to_vec(),
        why_unavailable_anatomy_parts: M5WhyUnavailableAnatomyPart::ALL.to_vec(),
        source_language_anatomy_parts: M5SourceLanguageAnatomyPart::ALL.to_vec(),
        blocked_action_owners: M5BlockedActionOwner::ALL.to_vec(),
        unavailable_reason_classes: M5UnavailableReasonClass::ALL.to_vec(),
        next_safe_action_classes: M5NextSafeActionClass::ALL.to_vec(),
        failure_domains: M5UnavailableFailureDomain::ALL.to_vec(),
        why_unavailable_postures: M5WhyUnavailablePosture::ALL.to_vec(),
        why_unavailable_actions: M5WhyUnavailableAction::ALL.to_vec(),
        source_language_classes: M5SourceLanguageClass::ALL.to_vec(),
        fallback_state_classes: M5FallbackStateClass::ALL.to_vec(),
        source_language_postures: M5SourceLanguagePosture::ALL.to_vec(),
        source_language_actions: M5SourceLanguageAction::ALL.to_vec(),
        why_unavailable_export_fields: M5WhyUnavailableExportField::ALL.to_vec(),
        source_language_export_fields: M5SourceLanguageExportField::ALL.to_vec(),
        accessibility_routes: M5TeachingAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TeachingConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TeachingDowngradeTrigger::BlockedActionOwnerUnstated,
            M5TeachingDowngradeTrigger::UnavailableReasonUnstated,
            M5TeachingDowngradeTrigger::NextSafeActionMissing,
            M5TeachingDowngradeTrigger::SourceLanguageFallbackUnstated,
            M5TeachingDowngradeTrigger::CitationSevered,
            M5TeachingDowngradeTrigger::AlternateStateLabelInvented,
            M5TeachingDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF,
            M5_BLOCKED_LOCALIZED_ROW_FEATURE_AVAILABILITY_REF,
            M5_BLOCKED_LOCALIZED_ROW_LOCALE_FALLBACK_REF,
        ]),
        why_unavailable_examples,
        source_language_examples,
        collapses_into_generic_disabled_state: false,
        hides_blocking_owner_or_reason: false,
        severs_canonical_citation_or_id: false,
        drifts_into_unsourced_paraphrase: false,
    }
}

fn rows() -> Vec<M5BlockedLocalizedConsumerRow> {
    use M5BlockedActionOwner as Owner;
    use M5BlockedLocalizedConsumerSurface as Surface;
    use M5FallbackStateClass as Fallback;
    use M5NextSafeActionClass as Next;
    use M5SourceLanguageClass as Lang;
    use M5TeachingQualificationClass as Qual;
    use M5UnavailableReasonClass as Reason;

    vec![
        // 1. Command-help row — a policy-blocked command and a permission-gated command, each
        //    naming the owning boundary and a request-access next step; plus a fully localized
        //    help string and a source-language fallback that preserves the source text and its
        //    canonical citation.
        base_row(
            Surface::CommandHelpRow,
            Qual::Stable,
            "Command-help row owner",
            "The command-help row renders both shared surfaces so a policy-blocked command names the policy owner, the exact reason, and a request-access next step, a permission-gated command names the workspace admin and the same safe next step, and a localized help string either reads as fully localized or falls back to the source language with its stable ID and canonical citation intact — never a generic disabled state and never an unsourced paraphrase",
            "evidence:m5-why-unavailable-source-language-command-help-row:001",
            vec![
                why_case(
                    "command:workspace.rotate-tokens",
                    Reason::PolicyBlocked,
                    Owner::PolicyOwner,
                    Next::RequestAccess,
                    Some("action:request-access.rotate-tokens"),
                    "docs:help/blocked-actions/policy",
                    "Rotate tokens is blocked by workspace policy, owned by the policy owner. Request access to proceed, or open the docs.",
                    "why:command-help:rotate-tokens-policy",
                ),
                why_case(
                    "command:project.delete",
                    Reason::MissingPermission,
                    Owner::WorkspaceAdmin,
                    Next::RequestAccess,
                    Some("action:request-access.project-delete"),
                    "docs:help/blocked-actions/permissions",
                    "Delete project needs a permission you do not have, owned by the workspace admin. Request access, or open the docs.",
                    "why:command-help:project-delete-permission",
                ),
            ],
            vec![
                source_case(
                    Lang::AuthoredLocale,
                    Fallback::LocalizedCurrent,
                    "fr-FR",
                    "stable:cmd.run-command",
                    "citation:docs/command-palette#run",
                    None,
                    "Run command help is fully localized in French and current with the canonical source.",
                    "loc:command-help:run-command-localized",
                ),
                source_case(
                    Lang::FallbackToSource,
                    Fallback::SourceLanguageShown,
                    "fr-FR",
                    "stable:cmd.open-settings",
                    "citation:docs/settings#open",
                    Some("text:source/en/open-settings"),
                    "Open settings help is not yet translated; the English source is shown with its canonical citation preserved.",
                    "loc:command-help:open-settings-fallback",
                ),
            ],
        ),
        // 2. Menu-and-action row — an unmet precondition that offers a satisfy-precondition retry
        //    and a disabled feature that points to settings; plus a partial translation and a
        //    stale translation, both keeping the source text and citation.
        base_row(
            Surface::MenuAndActionRow,
            Qual::Stable,
            "Menu-and-action row owner",
            "The menu-and-action row renders both shared surfaces so an unmet-precondition action names the precondition, offers a satisfy-precondition next step, and can retry when it clears, a feature-disabled action points to settings, a partial translation is shown honestly as partial with its source text, and a stale translation is shown as stale with its canonical citation intact",
            "evidence:m5-why-unavailable-source-language-menu-and-action-row:001",
            vec![
                why_case(
                    "action:merge.branch",
                    Reason::UnmetPrecondition,
                    Owner::CurrentUserScope,
                    Next::SatisfyPrecondition,
                    Some("action:resolve-conflicts.merge-branch"),
                    "docs:help/blocked-actions/preconditions",
                    "Merge branch cannot run until conflicts are resolved. Resolve the conflicts to satisfy the precondition, then retry, or open the docs.",
                    "why:menu-action:merge-precondition",
                ),
                why_case(
                    "action:ai.suggest",
                    Reason::FeatureFlagOff,
                    Owner::WorkspaceAdmin,
                    Next::OpenSettings,
                    Some("action:open-settings.ai-features"),
                    "docs:help/blocked-actions/feature-flags",
                    "AI suggestions are disabled by the workspace admin. Open settings to enable them, or open the docs.",
                    "why:menu-action:ai-feature-off",
                ),
            ],
            vec![
                source_case(
                    Lang::MixedLocale,
                    Fallback::PartialTranslation,
                    "de-DE",
                    "stable:menu.commit-changes",
                    "citation:docs/vcs#commit",
                    Some("text:source/en/commit-changes"),
                    "Commit changes help is partially translated to German; untranslated parts show the English source with the canonical citation.",
                    "loc:menu-action:commit-partial",
                ),
                source_case(
                    Lang::TranslatedLocale,
                    Fallback::StaleTranslation,
                    "de-DE",
                    "stable:menu.push-changes",
                    "citation:docs/vcs#push",
                    Some("text:source/en/push-changes"),
                    "Push changes help is translated to German but behind the canonical source; the English source and citation stay available.",
                    "loc:menu-action:push-stale",
                ),
            ],
        ),
        // 3. Inline-status row — an offline-unavailable action owned by the provider service with a
        //    switch-context next step, and an unsupported target that points to docs; plus a
        //    citation-preserved fallback and a fully untranslated surface.
        base_row(
            Surface::InlineStatusRow,
            Qual::Stable,
            "Inline-status row owner",
            "The inline-status row renders both shared surfaces so an offline-unavailable action names the provider-service boundary, offers a switch-context next step, and can retry once back online, an unsupported target points to docs, a citation-preserved fallback keeps its canonical link, and a fully untranslated surface shows the source text with a request-localization path",
            "evidence:m5-why-unavailable-source-language-inline-status-row:001",
            vec![
                why_case(
                    "action:sync.pull",
                    Reason::OfflineUnavailable,
                    Owner::ProviderService,
                    Next::SwitchContext,
                    Some("action:switch-context.work-offline"),
                    "docs:help/blocked-actions/offline",
                    "Pull from provider is unavailable while offline, owned by the provider service. Switch to an offline context, retry when back online, or open the docs.",
                    "why:inline-status:sync-pull-offline",
                ),
                why_case(
                    "action:preview.binary",
                    Reason::UnsupportedTarget,
                    Owner::UpstreamDependency,
                    Next::ReadDocs,
                    Some("action:read-docs.supported-targets"),
                    "docs:help/blocked-actions/unsupported",
                    "Preview is not supported for this binary target, limited by an upstream dependency. Read the docs for supported targets.",
                    "why:inline-status:preview-unsupported",
                ),
            ],
            vec![
                source_case(
                    Lang::FallbackToSource,
                    Fallback::CitationPreservedFallback,
                    "ja-JP",
                    "stable:inline.conflict-marker",
                    "citation:docs/vcs#conflicts",
                    Some("text:source/en/conflict-marker"),
                    "Conflict marker help falls back to the English source with its canonical citation preserved for Japanese readers.",
                    "loc:inline-status:conflict-citation-fallback",
                ),
                source_case(
                    Lang::UntranslatedSource,
                    Fallback::NoLocalization,
                    "ja-JP",
                    "stable:inline.lint-warning",
                    "citation:docs/lint#warnings",
                    Some("text:source/en/lint-warning"),
                    "Lint warning help has no Japanese localization; the English source is shown with a request-localization path and its canonical citation.",
                    "loc:inline-status:lint-no-localization",
                ),
            ],
        ),
        // 4. Settings-and-docs row — a feature-disabled setting pointing to settings, and a
        //    policy-blocked action with no safe action honestly named; plus a machine-translated
        //    partial and a fully localized current string.
        base_row(
            Surface::SettingsAndDocsRow,
            Qual::Stable,
            "Settings-and-docs row owner",
            "The settings-and-docs row renders both shared surfaces so a feature-disabled setting points to settings, a policy-blocked action with no safe next step is named honestly as having none (never a false promise), a machine-translated string is shown honestly as partial with its source, and a fully localized current string reads as current — every block names its owner and reason and every localized surface keeps its canonical citation",
            "evidence:m5-why-unavailable-source-language-settings-and-docs-row:001",
            vec![
                why_case(
                    "action:telemetry.enable",
                    Reason::FeatureFlagOff,
                    Owner::PolicyOwner,
                    Next::OpenSettings,
                    Some("action:open-settings.telemetry"),
                    "docs:help/blocked-actions/telemetry",
                    "Telemetry is disabled by policy. Open settings to review the policy, or open the docs.",
                    "why:settings-docs:telemetry-feature-off",
                ),
                why_case(
                    "action:export.customer-data",
                    Reason::PolicyBlocked,
                    Owner::UnknownOwner,
                    Next::NoSafeAction,
                    None,
                    "docs:help/blocked-actions/no-safe-action",
                    "Exporting customer data is blocked by policy and there is no safe action available here. Open the docs to understand the boundary.",
                    "why:settings-docs:export-no-safe-action",
                ),
            ],
            vec![
                source_case(
                    Lang::MachineTranslated,
                    Fallback::PartialTranslation,
                    "es-ES",
                    "stable:settings.keybindings",
                    "citation:docs/settings#keybindings",
                    Some("text:source/en/keybindings"),
                    "Keybindings settings help is machine-translated to Spanish and only partial; the English source and citation stay available.",
                    "loc:settings-docs:keybindings-machine-partial",
                ),
                source_case(
                    Lang::TranslatedLocale,
                    Fallback::LocalizedCurrent,
                    "es-ES",
                    "stable:settings.appearance",
                    "citation:docs/settings#appearance",
                    None,
                    "Appearance settings help is fully localized in Spanish and current with the canonical source.",
                    "loc:settings-docs:appearance-localized",
                ),
            ],
        ),
        // 5. Support explanation export — a permission-gated action owned by the provider service,
        //    and an unsupported target with no safe action; plus a source-language-shown fallback
        //    and a no-localization surface, all surviving the export with no raw material.
        base_row(
            Surface::SupportExplanationExport,
            Qual::Stable,
            "Support explanation export owner",
            "The support explanation export renders both shared surfaces so support can reconstruct exactly why an action was blocked — its owner, reason, and next safe action (or that there is none) — and exactly what localization state a help surface was in, with the source-language text, stable ID, and canonical citation intact and no raw error dump, stack trace, or endpoint crossing the boundary",
            "evidence:m5-why-unavailable-source-language-support-explanation-export:001",
            vec![
                why_case(
                    "action:billing.change-plan",
                    Reason::MissingPermission,
                    Owner::ProviderService,
                    Next::RequestAccess,
                    Some("action:request-access.billing"),
                    "docs:help/blocked-actions/billing",
                    "Change plan needs a billing permission gated by the provider service. Request access to proceed, or open the docs.",
                    "why:support-export:billing-permission",
                ),
                why_case(
                    "action:import.legacy-format",
                    Reason::UnsupportedTarget,
                    Owner::UnknownOwner,
                    Next::NoSafeAction,
                    None,
                    "docs:help/blocked-actions/legacy-import",
                    "Importing this legacy format is unsupported and there is no safe action available. Open the docs to understand the limitation.",
                    "why:support-export:legacy-import-no-safe-action",
                ),
            ],
            vec![
                source_case(
                    Lang::UntranslatedSource,
                    Fallback::SourceLanguageShown,
                    "pt-BR",
                    "stable:support.bundle-notes",
                    "citation:docs/support#bundle",
                    Some("text:source/en/bundle-notes"),
                    "Support bundle notes are shown in the English source for Portuguese readers, with the stable ID and canonical citation preserved.",
                    "loc:support-export:bundle-notes-source-shown",
                ),
                source_case(
                    Lang::UntranslatedSource,
                    Fallback::NoLocalization,
                    "pt-BR",
                    "stable:support.redaction-notice",
                    "citation:docs/support#redaction",
                    Some("text:source/en/redaction-notice"),
                    "The redaction notice has no Portuguese localization; the English source is shown with a request-localization path and its canonical citation.",
                    "loc:support-export:redaction-no-localization",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5BlockedLocalizedGovernanceReview {
    M5BlockedLocalizedGovernanceReview {
        row_names_blocked_action: true,
        row_names_exact_reason: true,
        row_names_owning_boundary: true,
        row_names_next_safe_action: true,
        row_links_deeper_docs: true,
        blocked_actions_never_collapse_into_generic_disabled: true,
        fallback_preserves_source_language_text: true,
        fallback_preserves_stable_id: true,
        fallback_preserves_canonical_citation: true,
        localized_flows_never_drift_into_unsourced_paraphrase: true,
        surfaces_never_require_pointer_hover: true,
        surfaces_provide_screen_reader_announcement: true,
        rows_stable_across_deployment_lines: true,
        rows_stable_across_consumer_surfaces: true,
        support_export_reconstructs_truth: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5BlockedLocalizedConsumerProjection {
    M5BlockedLocalizedConsumerProjection {
        surfaces_consume_shared_vocabulary: true,
        why_unavailable_reads_single_source: true,
        source_language_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5BlockedLocalizedProofFreshness {
    M5BlockedLocalizedProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BlockedLocalizedReleasePosture {
    M5BlockedLocalizedReleasePosture {
        release_packet_ref: M5_BLOCKED_LOCALIZED_ROW_ARTIFACT_REF.to_owned(),
        blocked_localized_audit_ref: M5_BLOCKED_LOCALIZED_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF,
        M5_BLOCKED_LOCALIZED_ROW_DOC_REF,
        M5_BLOCKED_LOCALIZED_ROW_COMPONENT_MATRIX_REF,
        M5_BLOCKED_LOCALIZED_ROW_FEATURE_AVAILABILITY_REF,
        M5_BLOCKED_LOCALIZED_ROW_LOCALE_FALLBACK_REF,
    ])
}

/// Builds the canonical M5 why-unavailable / source-language packet.
pub fn seeded_m5_blocked_localized_row_packet() -> M5BlockedLocalizedRowPacket {
    M5BlockedLocalizedRowPacket::new(M5BlockedLocalizedRowPacketInput {
        packet_id: M5_BLOCKED_LOCALIZED_ROW_PACKET_ID.to_owned(),
        matrix_label:
            "M5 why-unavailable / source-language primitive: blocked-action owner, unavailable reason, next-safe-action, failure domain, and deeper-docs path (why-unavailable) plus source-language class, fallback-state, stable ID, and citation-preserving link (source-language), with derived why-unavailable postures (blocked-by-policy/missing-permission/precondition-unmet/feature-disabled/offline-unavailable/unsupported-target), localization postures, and bounded actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5BlockedLocalizedVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the menu-and-action row consumer is held at Beta because a slice of menu
/// surfaces does not yet render the failure-domain cue on every profile; every consumer stays
/// visible.
pub fn seeded_m5_blocked_localized_menu_and_action_row_beta_narrowed() -> M5BlockedLocalizedRowPacket
{
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.packet_id =
        "m5-why-unavailable-source-language-primitive:menu-and-action-row-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BlockedLocalizedConsumerSurface::MenuAndActionRow)
        .expect("menu-and-action-row row present");
    row.qualification = M5TeachingQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support explanation export consumer is narrowed to Preview pending
/// citation-parity proof across every deployment; every consumer stays visible.
pub fn seeded_m5_blocked_localized_support_explanation_export_preview_narrowed(
) -> M5BlockedLocalizedRowPacket {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.packet_id =
        "m5-why-unavailable-source-language-primitive:support-explanation-export-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5BlockedLocalizedConsumerSurface::SupportExplanationExport
        })
        .expect("support-explanation-export row present");
    row.qualification = M5TeachingQualificationClass::Preview;
    packet
}
