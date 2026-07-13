//! Canonical seed builders for the M5 file-path-presentation and native-window / menu registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean file-path and window / menu entries are
//! built so the host-styled path separators (`/` on macOS / Linux, `\` on Windows), the platform-native
//! reveal verbs (Reveal in Finder / Show in Explorer / Open Containing Folder), the host-styled / canonical /
//! accessible presentation forms, and the stable-ID / in-product-surface / command reachability triple are
//! proven across the shell, settings, docs, onboarding, CLI, and support surfaces without any hand-copied
//! per-platform string, mislabeled path verb, lost canonical-path truth, presentation-form gap, or menu-only
//! action.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_FILE_PATH_REVEAL_REGISTRIES_PACKET_ID: &str =
    "m5-file-path-reveal-and-native-window-menu-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn path(input: M5FilePathPresentationEntryResolutionInput) -> M5ResolvedFilePathPresentationEntry {
    resolve_file_path_presentation_entry(input).expect("seed file-path entry resolves")
}

fn action(input: M5WindowMenuActionEntryResolutionInput) -> M5ResolvedWindowMenuActionEntry {
    resolve_window_menu_action_entry(input).expect("seed window / menu action entry resolves")
}

fn all_forms() -> Vec<M5PathPresentationForm> {
    M5PathPresentationForm::ALL.to_vec()
}

// -- Clean file-path entries (host-correct terminology bound to the shared registry) -------------

#[allow(clippy::too_many_arguments)]
fn clean_path_base(
    entry_id: &str,
    command_id: &str,
    token_name: &str,
    semantic_role: M5PlatformFitRole,
    path_role: M5FilePathRevealRole,
    host_platform: M5HostPlatform,
    surface_context: M5FilePathSurfaceContext,
    rendered_path: &str,
    reveal_verb: &str,
) -> M5FilePathPresentationEntryResolutionInput {
    M5FilePathPresentationEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        command_id: command_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        path_role,
        host_platform,
        surface_context,
        presentation_form_coverage: all_forms(),
        rendered_path: rendered_path.to_owned(),
        reveal_verb: reveal_verb.to_owned(),
        bound_to_registry: true,
        preserves_canonical_path_truth: true,
        reveal_target_unavailable: false,
        fallback_explained: true,
        proof_fresh: true,
    }
}

fn path_mac_open() -> M5ResolvedFilePathPresentationEntry {
    path(clean_path_base(
        "path:shell:open:macos",
        "command.file.open",
        "path.open.macos",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::FilePathTerminology,
        M5HostPlatform::Macos,
        M5FilePathSurfaceContext::FileOpenDialog,
        "/Users/ana/Documents",
        "Reveal in Finder",
    ))
}

fn path_win_save() -> M5ResolvedFilePathPresentationEntry {
    path(clean_path_base(
        "path:settings:save:windows",
        "command.file.save",
        "path.save.windows",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::SaveDialogTerminology,
        M5HostPlatform::Windows,
        M5FilePathSurfaceContext::SaveDialog,
        "C:\\Users\\ana\\Documents\\report.txt",
        "Show in Explorer",
    ))
}

fn path_linux_reveal() -> M5ResolvedFilePathPresentationEntry {
    path(clean_path_base(
        "path:cli:reveal:linux",
        "command.file.reveal",
        "path.reveal.linux",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::RevealVerb,
        M5HostPlatform::Linux,
        M5FilePathSurfaceContext::RevealMenu,
        "/home/ana/projects",
        "Open Containing Folder",
    ))
}

fn path_mac_breadcrumb() -> M5ResolvedFilePathPresentationEntry {
    path(clean_path_base(
        "path:shell:breadcrumb:macos",
        "command.workspace.locate",
        "path.breadcrumb.macos",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::HostMatchedSeparatorAndCase,
        M5HostPlatform::Macos,
        M5FilePathSurfaceContext::PathBreadcrumb,
        "/Users/ana/Projects/aureline",
        "Reveal in Finder",
    ))
}

fn path_win_docs() -> M5ResolvedFilePathPresentationEntry {
    path(clean_path_base(
        "path:docs:help:windows",
        "command.file.reveal",
        "path.help.windows",
        M5PlatformFitRole::CommandStability,
        M5FilePathRevealRole::BoundToPathRegistry,
        M5HostPlatform::Windows,
        M5FilePathSurfaceContext::DocsHelp,
        "C:\\Program Files\\Aureline",
        "Show in Explorer",
    ))
}

// -- Degraded file-path entries -----------------------------------------------------------------

/// Degraded path entry: the terminology is a hand-copied per-platform string instead of tracing to the
/// registry.
fn path_hand_copied() -> M5ResolvedFilePathPresentationEntry {
    let mut input = clean_path_base(
        "path:shell:hand-copied",
        "command.file.open",
        "path.open.macos",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::MislabeledPathVerbDisallowed,
        M5HostPlatform::Macos,
        M5FilePathSurfaceContext::FileOpenDialog,
        "/Users/ana/Documents",
        "Reveal in Finder",
    );
    input.bound_to_registry = false;
    path(input)
}

/// Degraded path entry: a Windows entry rendered with a forward-slash path is mislabeled for its host.
fn path_mislabeled() -> M5ResolvedFilePathPresentationEntry {
    path(clean_path_base(
        "path:settings:mislabeled:windows",
        "command.file.save",
        "path.save.windows",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::SaveDialogTerminology,
        M5HostPlatform::Windows,
        // A Windows surface rendered with a forward-slash separator mislabels the path for its host.
        M5FilePathSurfaceContext::SaveDialog,
        "C:/Users/ana/Documents/report.txt",
        "Show in Explorer",
    ))
}

/// Degraded path entry: the entry drops the literal / canonical path truth.
fn path_canonical_lost() -> M5ResolvedFilePathPresentationEntry {
    let mut input = clean_path_base(
        "path:onboarding:canonical-lost:macos",
        "command.workspace.locate",
        "path.breadcrumb.macos",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::HostMatchedSeparatorAndCase,
        M5HostPlatform::Macos,
        M5FilePathSurfaceContext::RevealMenu,
        "/Users/ana/Projects/aureline",
        "Reveal in Finder",
    );
    input.preserves_canonical_path_truth = false;
    path(input)
}

/// Degraded path entry: the host-styled / canonical / accessible presentation-form coverage is incomplete.
fn path_form_incomplete() -> M5ResolvedFilePathPresentationEntry {
    let mut input = clean_path_base(
        "path:docs:form-incomplete:windows",
        "command.file.reveal",
        "path.help.windows",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::BoundToPathRegistry,
        M5HostPlatform::Windows,
        M5FilePathSurfaceContext::DocsHelp,
        "C:\\Program Files\\Aureline",
        "Show in Explorer",
    );
    input.presentation_form_coverage = vec![M5PathPresentationForm::HostStyledDisplay];
    path(input)
}

/// Degraded path entry: reveal-in-shell is unavailable on this surface and no fallback vocabulary is
/// explained.
fn path_reveal_unhandled() -> M5ResolvedFilePathPresentationEntry {
    let mut input = clean_path_base(
        "path:cli:reveal-unhandled:linux",
        "command.file.reveal",
        "path.reveal.linux",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::RevealVerb,
        M5HostPlatform::Linux,
        M5FilePathSurfaceContext::RevealMenu,
        "/home/ana/projects",
        "Open Containing Folder",
    );
    input.reveal_target_unavailable = true;
    input.fallback_explained = false;
    path(input)
}

/// Degraded path entry: the canonical registry token name is unstated.
fn path_token_unstated() -> M5ResolvedFilePathPresentationEntry {
    let mut input = clean_path_base(
        "path:support:token-unstated:macos",
        "command.file.open",
        "  ",
        M5PlatformFitRole::PathTerminology,
        M5FilePathRevealRole::FilePathTerminology,
        M5HostPlatform::Macos,
        M5FilePathSurfaceContext::FileOpenDialog,
        "/Users/ana/Documents",
        "Reveal in Finder",
    );
    input.token_name = "  ".to_owned();
    path(input)
}

// -- Clean window / menu action entries ---------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_action_base(
    entry_id: &str,
    command_id: &str,
    token_name: &str,
    action_role: M5PlatformConventionRole,
    action_surface: M5ProductActionSurface,
    surface_context: M5FilePathSurfaceContext,
    human_label: &str,
    in_product_route: &str,
) -> M5WindowMenuActionEntryResolutionInput {
    M5WindowMenuActionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        command_id: command_id.to_owned(),
        token_name: token_name.to_owned(),
        action_role,
        semantic_role: M5PlatformFitRole::WindowMenu,
        action_surface,
        surface_context,
        presentation_form_coverage: all_forms(),
        human_label: human_label.to_owned(),
        in_product_route: in_product_route.to_owned(),
        reachable_by_id_surface_and_command: true,
        proof_fresh: true,
    }
}

fn action_palette_clean() -> M5ResolvedWindowMenuActionEntry {
    action(clean_action_base(
        "action:shell:reveal:palette",
        "command.file.reveal",
        "action.file.reveal.palette",
        M5PlatformConventionRole::BoundToPlatformRegistry,
        M5ProductActionSurface::CommandPalette,
        M5FilePathSurfaceContext::RevealMenu,
        "Reveal in Finder",
        "command.file.reveal",
    ))
}

fn action_toolbar_clean() -> M5ResolvedWindowMenuActionEntry {
    action(clean_action_base(
        "action:settings:save:toolbar",
        "command.file.save",
        "action.file.save.toolbar",
        M5PlatformConventionRole::SystemChromeIntegration,
        M5ProductActionSurface::ProductToolbar,
        M5FilePathSurfaceContext::SaveDialog,
        "Save",
        "command.file.save",
    ))
}

fn action_command_list_clean() -> M5ResolvedWindowMenuActionEntry {
    action(clean_action_base(
        "action:docs:open:command-list",
        "command.file.open",
        "action.file.open.command_list",
        M5PlatformConventionRole::MenuBarBehavior,
        M5ProductActionSurface::CommandList,
        M5FilePathSurfaceContext::FileOpenDialog,
        "Open…",
        "command.file.open",
    ))
}

// -- Degraded window / menu action entries ------------------------------------------------------

/// Degraded action entry: the action is reachable only through OS chrome — not by stable ID, an in-product
/// surface, and a command.
fn action_os_chrome_only() -> M5ResolvedWindowMenuActionEntry {
    let mut input = clean_action_base(
        "action:shell:os-chrome-only",
        "command.file.reveal",
        "action.file.reveal.palette",
        M5PlatformConventionRole::BoundToPlatformRegistry,
        M5ProductActionSurface::CommandPalette,
        M5FilePathSurfaceContext::RevealMenu,
        "Reveal in Finder",
        "command.file.reveal",
    );
    input.reachable_by_id_surface_and_command = false;
    action(input)
}

/// Degraded action entry: the host-styled / canonical / accessible presentation-form coverage of the menu
/// phrasing is incomplete.
fn action_phrasing_incomplete() -> M5ResolvedWindowMenuActionEntry {
    let mut input = clean_action_base(
        "action:docs:phrasing-incomplete",
        "command.file.open",
        "action.file.open.command_list",
        M5PlatformConventionRole::MenuBarBehavior,
        M5ProductActionSurface::CommandList,
        M5FilePathSurfaceContext::FileOpenDialog,
        "Open…",
        "command.file.open",
    );
    input.presentation_form_coverage = vec![M5PathPresentationForm::HostStyledDisplay];
    action(input)
}

/// Degraded action entry: the product action surface is unclassified.
fn action_surface_unclassified() -> M5ResolvedWindowMenuActionEntry {
    action(clean_action_base(
        "action:onboarding:surface-unclassified",
        "command.file.save",
        "action.file.save.unknown",
        M5PlatformConventionRole::SystemChromeIntegration,
        M5ProductActionSurface::SurfaceUnclassified,
        M5FilePathSurfaceContext::SaveDialog,
        "Save",
        "command.file.save",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5FilePathRevealRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    file_path_presentation_entries: Vec<M5ResolvedFilePathPresentationEntry>,
    window_menu_action_entries: Vec<M5ResolvedWindowMenuActionEntry>,
) -> M5FilePathRevealRegistriesRow {
    M5FilePathRevealRegistriesRow {
        consumer_surface,
        qualification: M5PlatformFitQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5PlatformFitDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5PlatformFitRequiredLabel::Identity,
            M5PlatformFitRequiredLabel::SemanticRole,
            M5PlatformFitRequiredLabel::RegistryReference,
            M5PlatformFitRequiredLabel::HostPlatform,
            M5PlatformFitRequiredLabel::PathVerb,
        ],
        accessibility_routes: M5PlatformFitAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5FilePathRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5FilePathRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        file_path_presentation_entries,
        window_menu_action_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_FILE_PATH_REVEAL_REGISTRIES_SCHEMA_REF,
            M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
        ]),
        path_terminology_changes_command_or_permission_meaning: false,
        primary_action_reachable_only_in_os_chrome: false,
        terminology_hardcoded_instead_of_registry: false,
        screenshot_or_docs_mislabels_path_verb: false,
    }
}

fn registry_rows() -> Vec<M5FilePathRevealRegistriesRow> {
    use M5PlatformFitConsumerSurface as C;
    use M5PlatformFitDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell renders the macOS /Users open-dialog path and the Reveal in Finder verb from the shared path registry, and keeps Reveal reachable from the command palette; a hand-copied per-platform verb and an action reachable only through OS chrome degrade honestly instead of reading as a clean pass",
            "evidence:m5-file-path-shell-ui:001",
            vec![
                D::ShortcutNotationDriftedByPlatform,
                D::PrimaryActionHiddenOnlyInOsChrome,
                D::ProofStale,
            ],
            vec![path_mac_open(), path_hand_copied()],
            vec![action_palette_clean(), action_os_chrome_only()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings path presentation renders the Windows C:\\ save-dialog path and the Show in Explorer verb from the registry, and keeps Save reachable from the toolbar; a Windows entry rendered with a forward-slash separator is caught as mislabeled for its host",
            "evidence:m5-file-path-settings-ui:001",
            vec![
                D::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
                D::PathVerbUnstated,
                D::ProofStale,
            ],
            vec![path_win_save(), path_mislabeled()],
            vec![action_toolbar_clean()],
        ),
        base_row(
            C::DocsHelp,
            "Docs/help surface owner",
            "Docs and help render the Windows Program Files reveal path across the host-styled, canonical, and accessible presentation forms, and keep Open reachable from the command list; a path and an action that omit a presentation form degrade honestly so a screenshot cannot reintroduce an incorrect path verb",
            "evidence:m5-file-path-docs-help:001",
            vec![
                D::PathVerbUnstated,
                D::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
                D::ProofStale,
            ],
            vec![path_win_docs(), path_form_incomplete()],
            vec![action_command_list_clean(), action_phrasing_incomplete()],
        ),
        base_row(
            C::Onboarding,
            "Onboarding surface owner",
            "Onboarding renders the macOS breadcrumb path from the registry while keeping the literal-versus-canonical path truth explicit; a path that drops canonical truth and an action with an unclassified product surface degrade honestly",
            "evidence:m5-file-path-onboarding:001",
            vec![
                D::PlatformWordingChangedCommandOrPermissionMeaning,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![path_mac_breadcrumb(), path_canonical_lost()],
            vec![action_surface_unclassified()],
        ),
        base_row(
            C::CliExport,
            "CLI/export owner",
            "The CLI export renders the Linux Open Containing Folder reveal verb from the path registry and keeps Reveal reachable from the command palette; a reveal target that is unavailable without an explained fallback degrades honestly instead of silently dropping the action",
            "evidence:m5-file-path-cli-export:001",
            vec![
                D::PrimaryActionHiddenOnlyInOsChrome,
                D::PathVerbUnstated,
                D::ProofStale,
            ],
            vec![path_linux_reveal(), path_reveal_unhandled()],
            vec![action_palette_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved file-path and window / menu truth, so a hand-copied constant or an unstated registry token is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-file-path-support-export:001",
            vec![
                D::PathVerbUnstated,
                D::HostPlatformUnstated,
                D::ProofStale,
            ],
            vec![path_mac_open(), path_token_unstated()],
            vec![action_toolbar_clean()],
        ),
    ]
}

fn governance_review() -> M5FilePathRevealRegistriesGovernanceReview {
    M5FilePathRevealRegistriesGovernanceReview {
        terminology_registry_names_token_role_and_platform: true,
        host_correct_terms_and_separators_rendered_from_shared_registry: true,
        literal_versus_canonical_path_truth_kept_explicit: true,
        reveal_and_save_terminology_match_host_platform: true,
        high_frequency_actions_reachable_from_product_surfaces: true,
        native_window_chrome_and_menu_phrasing_preserved: true,
        every_entry_covers_all_presentation_forms: true,
        terminology_bound_to_single_registry_not_hand_copied: true,
        docs_help_and_screenshots_generated_from_registry: true,
        path_or_chrome_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5FilePathRevealRegistriesConsumerProjection {
    M5FilePathRevealRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        settings_consumes_shared_registries: true,
        docs_help_consumes_shared_registries: true,
        onboarding_and_cli_consume_shared_registries: true,
        terminology_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5FilePathRevealRegistriesProofFreshness {
    M5FilePathRevealRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5FilePathRevealRegistriesReleasePosture {
    M5FilePathRevealRegistriesReleasePosture {
        proof_packet_ref: M5_FILE_PATH_REVEAL_REGISTRIES_ARTIFACT_REF.to_owned(),
        platform_fit_audit_ref: M5_FILE_PATH_REVEAL_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_FILE_PATH_REVEAL_REGISTRIES_SCHEMA_REF,
        M5_FILE_PATH_REVEAL_REGISTRIES_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 file-path-presentation and native-window / menu registries packet.
pub fn seeded_m5_file_path_reveal_and_native_window_menu_registries(
) -> M5FilePathRevealRegistriesPacket {
    M5FilePathRevealRegistriesPacket::new(M5FilePathRevealRegistriesPacketInput {
        packet_id: M5_FILE_PATH_REVEAL_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 file-path-presentation and native-window / menu registries with host-correct separators and reveal verbs (Reveal in Finder / Show in Explorer / Open Containing Folder), explicit literal-versus-canonical path truth, host-styled / canonical / accessible presentation-form coverage, and stable-ID / in-product-surface / command reachability across shell, settings, docs, onboarding, CLI, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5FilePathRevealRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the docs/help row is held at Beta pending screenshot-generation parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_file_path_reveal_and_native_window_menu_registries_docs_help_beta_narrowed(
) -> M5FilePathRevealRegistriesPacket {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.packet_id =
        "m5-file-path-reveal-and-native-window-menu-registries:docs-help-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PlatformFitConsumerSurface::DocsHelp)
        .expect("docs-help row present");
    row.qualification = M5PlatformFitQualificationClass::Beta;
    packet
}

/// Narrowed variant: the CLI/export reveal row is narrowed to Preview pending reveal-fallback parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_file_path_reveal_and_native_window_menu_registries_reveal_preview_narrowed(
) -> M5FilePathRevealRegistriesPacket {
    let mut packet = seeded_m5_file_path_reveal_and_native_window_menu_registries();
    packet.packet_id =
        "m5-file-path-reveal-and-native-window-menu-registries:reveal-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PlatformFitConsumerSurface::CliExport)
        .expect("cli-export row present");
    row.qualification = M5PlatformFitQualificationClass::Preview;
    packet
}
