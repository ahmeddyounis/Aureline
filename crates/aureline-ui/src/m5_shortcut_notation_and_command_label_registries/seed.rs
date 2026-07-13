//! Canonical seed builders for the M5 shortcut-notation and command-label registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean shortcut-notation and command-label
//! entries are built so the platform-native macOS glyph notation (⌘ ⌥ ⌃ ⇧), the Windows / Linux modifier
//! names (Ctrl / Alt / Shift), the visual / spoken / searchable notation forms, and the stable-ID /
//! human-label / shortcut-text discovery triple are proven across the shell, settings, docs, onboarding,
//! CLI, and support surfaces without any hand-copied per-platform string, mislabeled notation, unstable
//! command identity, notation-form gap, or missing reserved-key fallback.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SHORTCUT_NOTATION_REGISTRIES_PACKET_ID: &str =
    "m5-shortcut-notation-and-command-label-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn notation(input: M5ShortcutNotationEntryResolutionInput) -> M5ResolvedShortcutNotationEntry {
    resolve_shortcut_notation_entry(input).expect("seed shortcut-notation entry resolves")
}

fn label(input: M5CommandLabelMappingResolutionInput) -> M5ResolvedCommandLabelMappingEntry {
    resolve_command_label_mapping_entry(input).expect("seed command-label entry resolves")
}

fn all_forms() -> Vec<M5ShortcutNotationForm> {
    M5ShortcutNotationForm::ALL.to_vec()
}

// -- Clean shortcut-notation entries (platform-native notation bound to the shared registry) -------

#[allow(clippy::too_many_arguments)]
fn clean_notation_base(
    entry_id: &str,
    command_id: &str,
    token_name: &str,
    semantic_role: M5PlatformFitRole,
    notation_role: M5ShortcutNotationRole,
    host_platform: M5HostPlatform,
    surface_context: M5ShortcutSurfaceContext,
    rendered_notation: &str,
) -> M5ShortcutNotationEntryResolutionInput {
    M5ShortcutNotationEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        command_id: command_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        notation_role,
        host_platform,
        surface_context,
        notation_form_coverage: all_forms(),
        rendered_notation: rendered_notation.to_owned(),
        bound_to_registry: true,
        preserves_command_id: true,
        reserved_by_os: false,
        fallback_explained: true,
        proof_fresh: true,
    }
}

fn notation_mac_save_menu() -> M5ResolvedShortcutNotationEntry {
    notation(clean_notation_base(
        "notation:shell:save:macos",
        "command.file.save",
        "shortcut.file.save.macos",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::ModifierGlyphNotation,
        M5HostPlatform::Macos,
        M5ShortcutSurfaceContext::MenuBar,
        "⌘S",
    ))
}

fn notation_win_palette() -> M5ResolvedShortcutNotationEntry {
    notation(clean_notation_base(
        "notation:settings:palette:windows",
        "command.palette.open",
        "shortcut.palette.open.windows",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::AcceleratorLabel,
        M5HostPlatform::Windows,
        M5ShortcutSurfaceContext::CommandPalette,
        "Ctrl+Shift+P",
    ))
}

fn notation_linux_inspector() -> M5ResolvedShortcutNotationEntry {
    notation(clean_notation_base(
        "notation:cli:chord:linux",
        "command.keybinding.chord",
        "shortcut.keybinding.chord.linux",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::ChordSequence,
        M5HostPlatform::Linux,
        M5ShortcutSurfaceContext::KeybindingInspector,
        "Ctrl+K Ctrl+S",
    ))
}

fn notation_mac_help() -> M5ResolvedShortcutNotationEntry {
    notation(clean_notation_base(
        "notation:docs:help:macos",
        "command.help.open",
        "shortcut.help.open.macos",
        M5PlatformFitRole::CommandStability,
        M5ShortcutNotationRole::StableCommandIdBinding,
        M5HostPlatform::Macos,
        M5ShortcutSurfaceContext::HelpDoc,
        "⌘⇧/",
    ))
}

fn notation_win_onboarding() -> M5ResolvedShortcutNotationEntry {
    notation(clean_notation_base(
        "notation:onboarding:new:windows",
        "command.file.new",
        "shortcut.file.new.windows",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::PlatformAdaptiveNotation,
        M5HostPlatform::Windows,
        M5ShortcutSurfaceContext::Onboarding,
        "Ctrl+N",
    ))
}

// -- Degraded shortcut-notation entries ---------------------------------------------------------

/// Degraded notation entry: the notation is a hand-copied per-platform string instead of tracing to the
/// registry.
fn notation_hand_copied() -> M5ResolvedShortcutNotationEntry {
    let mut input = clean_notation_base(
        "notation:shell:hand-copied",
        "command.file.save",
        "shortcut.file.save.macos",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::HardcodedPlatformNotationDisallowed,
        M5HostPlatform::Macos,
        M5ShortcutSurfaceContext::MenuBar,
        "⌘S",
    );
    input.bound_to_registry = false;
    notation(input)
}

/// Degraded notation entry: a macOS entry rendered with Windows modifier names is mislabeled for its host.
fn notation_mislabeled() -> M5ResolvedShortcutNotationEntry {
    notation(clean_notation_base(
        "notation:settings:mislabeled:macos",
        "command.file.save",
        "shortcut.file.save.macos",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::ModifierGlyphNotation,
        M5HostPlatform::Macos,
        // A macOS surface rendered with the Windows modifier name mislabels the notation for its host.
        M5ShortcutSurfaceContext::CommandPalette,
        "Ctrl+S",
    ))
}

/// Degraded notation entry: the visual / spoken / searchable notation-form coverage is incomplete.
fn notation_form_incomplete() -> M5ResolvedShortcutNotationEntry {
    let mut input = clean_notation_base(
        "notation:docs:form-incomplete:macos",
        "command.help.open",
        "shortcut.help.open.macos",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::ModifierGlyphNotation,
        M5HostPlatform::Macos,
        M5ShortcutSurfaceContext::HelpDoc,
        "⌘⇧/",
    );
    input.notation_form_coverage = vec![M5ShortcutNotationForm::VisualNotation];
    notation(input)
}

/// Degraded notation entry: the rendered notation does not preserve the stable command ID.
fn notation_command_identity_unstable() -> M5ResolvedShortcutNotationEntry {
    let mut input = clean_notation_base(
        "notation:onboarding:identity-unstable:windows",
        "command.file.new",
        "shortcut.file.new.windows",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::PlatformAdaptiveNotation,
        M5HostPlatform::Windows,
        M5ShortcutSurfaceContext::Onboarding,
        "Ctrl+N",
    );
    input.preserves_command_id = false;
    notation(input)
}

/// Degraded notation entry: the shortcut is reserved by the OS and no fallback vocabulary is explained.
fn notation_reserved_unhandled() -> M5ResolvedShortcutNotationEntry {
    let mut input = clean_notation_base(
        "notation:cli:reserved-unhandled:linux",
        "command.window.tile",
        "shortcut.window.tile.linux",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::PlatformAdaptiveNotation,
        M5HostPlatform::Linux,
        M5ShortcutSurfaceContext::KeybindingInspector,
        "Ctrl+Alt+Left",
    );
    input.reserved_by_os = true;
    input.fallback_explained = false;
    notation(input)
}

/// Degraded notation entry: the canonical registry token name is unstated.
fn notation_token_unstated() -> M5ResolvedShortcutNotationEntry {
    let mut input = clean_notation_base(
        "notation:support:token-unstated:macos",
        "command.file.save",
        "  ",
        M5PlatformFitRole::Shortcut,
        M5ShortcutNotationRole::ModifierGlyphNotation,
        M5HostPlatform::Macos,
        M5ShortcutSurfaceContext::MenuBar,
        "⌘S",
    );
    input.token_name = "  ".to_owned();
    notation(input)
}

// -- Clean command-label mapping entries --------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_label_base(
    entry_id: &str,
    command_id: &str,
    token_name: &str,
    mapping_role: M5ShortcutNotationRole,
    label_kind: M5CommandLabelKind,
    surface_context: M5ShortcutSurfaceContext,
    human_label: &str,
    shortcut_text: &str,
) -> M5CommandLabelMappingResolutionInput {
    M5CommandLabelMappingResolutionInput {
        entry_id: entry_id.to_owned(),
        command_id: command_id.to_owned(),
        token_name: token_name.to_owned(),
        mapping_role,
        semantic_role: M5PlatformFitRole::CommandStability,
        label_kind,
        surface_context,
        notation_form_coverage: all_forms(),
        human_label: human_label.to_owned(),
        shortcut_text: shortcut_text.to_owned(),
        discoverable_by_id_label_and_shortcut: true,
        proof_fresh: true,
    }
}

fn label_menu_clean() -> M5ResolvedCommandLabelMappingEntry {
    label(clean_label_base(
        "label:shell:save:menu",
        "command.file.save",
        "label.file.save.menu",
        M5ShortcutNotationRole::AcceleratorLabel,
        M5CommandLabelKind::MenuLabel,
        M5ShortcutSurfaceContext::MenuBar,
        "Save",
        "Ctrl+S",
    ))
}

fn label_palette_clean() -> M5ResolvedCommandLabelMappingEntry {
    label(clean_label_base(
        "label:settings:save:palette",
        "command.file.save",
        "label.file.save.palette",
        M5ShortcutNotationRole::StableCommandIdBinding,
        M5CommandLabelKind::PaletteLabel,
        M5ShortcutSurfaceContext::CommandPalette,
        "Save File",
        "Ctrl+S",
    ))
}

fn label_help_clean() -> M5ResolvedCommandLabelMappingEntry {
    label(clean_label_base(
        "label:docs:save:help",
        "command.file.save",
        "label.file.save.help",
        M5ShortcutNotationRole::AcceleratorLabel,
        M5CommandLabelKind::HelpLabel,
        M5ShortcutSurfaceContext::HelpDoc,
        "Save the current file",
        "Ctrl+S",
    ))
}

// -- Degraded command-label mapping entries -----------------------------------------------------

/// Degraded command-label entry: the command is not discoverable by stable ID, label, and shortcut text.
fn label_discovery_incomplete() -> M5ResolvedCommandLabelMappingEntry {
    let mut input = clean_label_base(
        "label:shell:discovery-incomplete",
        "command.file.save",
        "label.file.save.menu",
        M5ShortcutNotationRole::AcceleratorLabel,
        M5CommandLabelKind::MenuLabel,
        M5ShortcutSurfaceContext::MenuBar,
        "Save",
        "Ctrl+S",
    );
    input.discoverable_by_id_label_and_shortcut = false;
    label(input)
}

/// Degraded command-label entry: the visual / spoken / searchable notation-form coverage is incomplete.
fn label_form_incomplete() -> M5ResolvedCommandLabelMappingEntry {
    let mut input = clean_label_base(
        "label:docs:form-incomplete",
        "command.file.save",
        "label.file.save.help",
        M5ShortcutNotationRole::AcceleratorLabel,
        M5CommandLabelKind::HelpLabel,
        M5ShortcutSurfaceContext::HelpDoc,
        "Save the current file",
        "Ctrl+S",
    );
    input.notation_form_coverage = vec![M5ShortcutNotationForm::VisualNotation];
    label(input)
}

/// Degraded command-label entry: the label kind is unclassified.
fn label_unclassified() -> M5ResolvedCommandLabelMappingEntry {
    label(clean_label_base(
        "label:onboarding:unclassified",
        "command.file.save",
        "label.file.save.unknown",
        M5ShortcutNotationRole::AcceleratorLabel,
        M5CommandLabelKind::LabelUnclassified,
        M5ShortcutSurfaceContext::Onboarding,
        "Save",
        "Ctrl+S",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ShortcutNotationRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    shortcut_notation_entries: Vec<M5ResolvedShortcutNotationEntry>,
    command_label_entries: Vec<M5ResolvedCommandLabelMappingEntry>,
) -> M5ShortcutNotationRegistriesRow {
    M5ShortcutNotationRegistriesRow {
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
            M5PlatformFitRequiredLabel::ShortcutNotation,
        ],
        accessibility_routes: M5PlatformFitAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ShortcutRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5ShortcutRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        shortcut_notation_entries,
        command_label_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SHORTCUT_NOTATION_REGISTRIES_SCHEMA_REF,
            M5_SHORTCUT_NOTATION_SCHEMA_REF,
        ]),
        notation_changes_command_or_permission_meaning: false,
        primary_command_hidden_only_in_os_chrome: false,
        notation_hardcoded_instead_of_registry: false,
        screenshot_or_docs_mislabels_shortcut: false,
    }
}

fn registry_rows() -> Vec<M5ShortcutNotationRegistriesRow> {
    use M5PlatformFitConsumerSurface as C;
    use M5PlatformFitDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell renders the macOS ⌘S menu accelerator and the Save menu label from the shared keybinding registry; a hand-copied per-platform notation and a command-label mapping that is not discoverable by ID, label, and shortcut degrade honestly instead of reading as a clean pass",
            "evidence:m5-shortcut-shell-ui:001",
            vec![
                D::ShortcutNotationDriftedByPlatform,
                D::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
                D::ProofStale,
            ],
            vec![notation_mac_save_menu(), notation_hand_copied()],
            vec![label_menu_clean(), label_discovery_incomplete()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The keybinding inspector renders the Windows Ctrl+Shift+P palette accelerator and the palette label from the registry; a macOS entry rendered with a Windows modifier name is caught as mislabeled for its host",
            "evidence:m5-shortcut-settings-ui:001",
            vec![
                D::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
                D::ShortcutNotationDriftedByPlatform,
                D::ProofStale,
            ],
            vec![notation_win_palette(), notation_mislabeled()],
            vec![label_palette_clean()],
        ),
        base_row(
            C::DocsHelp,
            "Docs/help surface owner",
            "Docs and help render the macOS ⌘⇧/ help accelerator and the help label across the visual, spoken, and searchable notation forms; a notation and a label that omit a notation form degrade honestly so a screenshot cannot reintroduce incorrect notation",
            "evidence:m5-shortcut-docs-help:001",
            vec![
                D::ShortcutNotationUnstated,
                D::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
                D::ProofStale,
            ],
            vec![notation_mac_help(), notation_form_incomplete()],
            vec![label_help_clean(), label_form_incomplete()],
        ),
        base_row(
            C::Onboarding,
            "Onboarding surface owner",
            "Onboarding renders the Windows Ctrl+N new-file accelerator from the registry while keeping the command ID stable; a notation that would change command identity and a label with an unclassified kind degrade honestly",
            "evidence:m5-shortcut-onboarding:001",
            vec![
                D::PlatformWordingChangedCommandOrPermissionMeaning,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![notation_win_onboarding(), notation_command_identity_unstable()],
            vec![label_unclassified()],
        ),
        base_row(
            C::CliExport,
            "CLI/export owner",
            "The CLI export renders the Linux Ctrl+K Ctrl+S chord from the keybinding inspector registry and the palette label; a shortcut reserved by the OS without an explained fallback degrades honestly instead of silently dropping the action",
            "evidence:m5-shortcut-cli-export:001",
            vec![
                D::PrimaryActionHiddenOnlyInOsChrome,
                D::ShortcutNotationUnstated,
                D::ProofStale,
            ],
            vec![notation_linux_inspector(), notation_reserved_unhandled()],
            vec![label_palette_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved shortcut-notation and command-label truth, so a hand-copied constant or an unstated registry token is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-shortcut-support-export:001",
            vec![
                D::ShortcutNotationUnstated,
                D::HostPlatformUnstated,
                D::ProofStale,
            ],
            vec![notation_mac_save_menu(), notation_token_unstated()],
            vec![label_menu_clean()],
        ),
    ]
}

fn governance_review() -> M5ShortcutNotationRegistriesGovernanceReview {
    M5ShortcutNotationRegistriesGovernanceReview {
        notation_registry_names_token_role_and_platform: true,
        platform_native_notation_rendered_from_shared_registry: true,
        command_discoverable_by_id_label_and_shortcut: true,
        command_ids_stable_while_notation_adapts: true,
        macos_glyphs_windows_linux_names_and_fallbacks_supported: true,
        every_entry_covers_all_notation_forms: true,
        notation_bound_to_single_registry_not_hand_copied: true,
        docs_help_and_screenshots_generated_from_registry: true,
        notation_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ShortcutNotationRegistriesConsumerProjection {
    M5ShortcutNotationRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        settings_consumes_shared_registries: true,
        docs_help_consumes_shared_registries: true,
        onboarding_and_cli_consume_shared_registries: true,
        notation_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ShortcutNotationRegistriesProofFreshness {
    M5ShortcutNotationRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ShortcutNotationRegistriesReleasePosture {
    M5ShortcutNotationRegistriesReleasePosture {
        proof_packet_ref: M5_SHORTCUT_NOTATION_REGISTRIES_ARTIFACT_REF.to_owned(),
        platform_fit_audit_ref: M5_SHORTCUT_NOTATION_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SHORTCUT_NOTATION_REGISTRIES_SCHEMA_REF,
        M5_SHORTCUT_NOTATION_REGISTRIES_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_SHORTCUT_NOTATION_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 shortcut-notation and command-label registries packet.
pub fn seeded_m5_shortcut_notation_command_label_registries() -> M5ShortcutNotationRegistriesPacket
{
    M5ShortcutNotationRegistriesPacket::new(M5ShortcutNotationRegistriesPacketInput {
        packet_id: M5_SHORTCUT_NOTATION_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 shortcut-notation and command-label registries with platform-native macOS glyph notation, Windows / Linux modifier names, explicit reserved-key fallbacks, visual / spoken / searchable notation-form coverage, and stable-command-ID / human-label / shortcut-text discovery across shell, settings, docs, onboarding, CLI, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5ShortcutNotationRegistriesVocabularySet::canonical(),
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
pub fn seeded_m5_shortcut_notation_command_label_registries_docs_help_beta_narrowed(
) -> M5ShortcutNotationRegistriesPacket {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.packet_id =
        "m5-shortcut-notation-and-command-label-registries:docs-help-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PlatformFitConsumerSurface::DocsHelp)
        .expect("docs-help row present");
    row.qualification = M5PlatformFitQualificationClass::Beta;
    packet
}

/// Narrowed variant: the onboarding row is narrowed to Preview pending reserved-key fallback parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_shortcut_notation_command_label_registries_onboarding_preview_narrowed(
) -> M5ShortcutNotationRegistriesPacket {
    let mut packet = seeded_m5_shortcut_notation_command_label_registries();
    packet.packet_id =
        "m5-shortcut-notation-and-command-label-registries:onboarding-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PlatformFitConsumerSurface::Onboarding)
        .expect("onboarding row present");
    row.qualification = M5PlatformFitQualificationClass::Preview;
    packet
}
