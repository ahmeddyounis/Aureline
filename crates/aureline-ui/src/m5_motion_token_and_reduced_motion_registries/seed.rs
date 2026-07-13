//! Canonical seed builders for the M5 motion-token and reduced-motion registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean motion and reduced-motion entries are
//! built so the canonical motion grammar, the protected command-palette / menu / typing / inline-editor /
//! diagnostic surface classes, the reduced-motion / power-saver / thermal clamp coverage, and the
//! static-fallback behavior are proven across the shell, dialog, panel, embedded, notification, and support
//! surfaces without any protected-path delay, layout shift, raw-duration inlining, clamp gap, or
//! motion-only meaning.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_MOTION_REGISTRIES_PACKET_ID: &str =
    "m5-motion-token-and-reduced-motion-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn motion(input: M5MotionEntryResolutionInput) -> M5ResolvedMotionEntry {
    resolve_motion_entry(input).expect("seed motion entry resolves")
}

fn reduced(input: M5ReducedMotionEntryResolutionInput) -> M5ResolvedReducedMotionEntry {
    resolve_reduced_motion_entry(input).expect("seed reduced-motion entry resolves")
}

fn all_clamps() -> Vec<M5MotionClamp> {
    M5MotionClamp::ALL.to_vec()
}

// -- Clean motion entries (surface-class + motion-role grammar across surfaces) ------------------

#[allow(clippy::too_many_arguments)]
fn clean_motion_base(
    entry_id: &str,
    token_name: &str,
    semantic_role: M5VisualInteractionRole,
    motion_role: M5MotionTokenRole,
    surface_class: M5MotionSurfaceClass,
    reduced_motion_fallback: M5ReducedMotionFallback,
    surface_context: M5MotionSurfaceContext,
) -> M5MotionEntryResolutionInput {
    M5MotionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        motion_role,
        surface_class,
        reduced_motion_fallback,
        surface_context,
        clamp_coverage: all_clamps(),
        respects_input_priority: true,
        preserves_no_layout_shift: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn motion_command_palette_clean() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:shell:command-palette",
        "motion.instant.palette",
        M5VisualInteractionRole::Attention,
        M5MotionTokenRole::RespectsInputPriority,
        M5MotionSurfaceClass::CommandPaletteInput,
        M5ReducedMotionFallback::InstantStateChange,
        M5MotionSurfaceContext::Shell,
    ))
}

fn motion_menu_clean() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:shell:menu",
        "motion.instant.menu",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::RespectsInputPriority,
        M5MotionSurfaceClass::MenuNavigation,
        M5ReducedMotionFallback::InstantStateChange,
        M5MotionSurfaceContext::Shell,
    ))
}

fn motion_typing_clean() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:panel:typing-caret",
        "motion.instant.caret",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::RespectsInputPriority,
        M5MotionSurfaceClass::TypingCaret,
        M5ReducedMotionFallback::InstantStateChange,
        M5MotionSurfaceContext::Panel,
    ))
}

fn motion_inline_editor_clean() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:panel:inline-editor",
        "motion.instant.inline",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::RespectsInputPriority,
        M5MotionSurfaceClass::InlineEditor,
        M5ReducedMotionFallback::StaticIndicator,
        M5MotionSurfaceContext::Panel,
    ))
}

fn motion_diagnostic_clean() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:embedded:diagnostic",
        "motion.instant.diagnostic",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::RespectsInputPriority,
        M5MotionSurfaceClass::DiagnosticSurface,
        M5ReducedMotionFallback::TextualStatus,
        M5MotionSurfaceContext::Embedded,
    ))
}

fn motion_dialog_clean() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:dialog:entrance",
        "motion.duration.dialog",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::OriginContinuityCue,
        M5MotionSurfaceClass::DialogEntrance,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Dialog,
    ))
}

fn motion_notification_clean() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:notification:entrance",
        "motion.duration.notification",
        M5VisualInteractionRole::Attention,
        M5MotionTokenRole::CompletionCue,
        M5MotionSurfaceClass::NotificationEntrance,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Notification,
    ))
}

fn motion_onboarding_clean() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:embedded:onboarding",
        "motion.duration.onboarding",
        M5VisualInteractionRole::Attention,
        M5MotionTokenRole::OriginContinuityCue,
        M5MotionSurfaceClass::OnboardingSequence,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Embedded,
    ))
}

// -- Degraded motion entries --------------------------------------------------------------------

/// Degraded motion entry: the motion delays input on a protected path.
fn motion_delays_protected() -> M5ResolvedMotionEntry {
    let mut input = clean_motion_base(
        "motion:shell:delays-protected",
        "motion.instant.palette",
        M5VisualInteractionRole::Attention,
        M5MotionTokenRole::RespectsInputPriority,
        M5MotionSurfaceClass::CommandPaletteInput,
        M5ReducedMotionFallback::InstantStateChange,
        M5MotionSurfaceContext::Shell,
    );
    input.respects_input_priority = false;
    motion(input)
}

/// Degraded motion entry: no reduced-motion static fallback is paired with the animation.
fn motion_fallback_missing() -> M5ResolvedMotionEntry {
    let mut input = clean_motion_base(
        "motion:dialog:fallback-missing",
        "motion.duration.dialog",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::OriginContinuityCue,
        M5MotionSurfaceClass::DialogEntrance,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Dialog,
    );
    input.reduced_motion_fallback = M5ReducedMotionFallback::NoneDisallowed;
    motion(input)
}

/// Degraded motion entry: the reduced-motion / power-saver / thermal clamp coverage is incomplete.
fn motion_clamp_incomplete() -> M5ResolvedMotionEntry {
    let mut input = clean_motion_base(
        "motion:panel:clamp-incomplete",
        "motion.duration.panel",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::EasingFamily,
        M5MotionSurfaceClass::PanelTransition,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Panel,
    );
    input.clamp_coverage = vec![M5MotionClamp::ReducedMotion, M5MotionClamp::PowerSaver];
    motion(input)
}

/// Degraded motion entry: the motion introduces a layout shift on a typing-adjacent surface.
fn motion_layout_shift() -> M5ResolvedMotionEntry {
    let mut input = clean_motion_base(
        "motion:dialog:layout-shift",
        "motion.duration.tooltip",
        M5VisualInteractionRole::Overlay,
        M5MotionTokenRole::EasingFamily,
        M5MotionSurfaceClass::TooltipReveal,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Dialog,
    );
    input.preserves_no_layout_shift = false;
    motion(input)
}

/// Degraded motion entry: a raw duration value is inlined instead of tracing to a canonical token.
fn motion_raw_inlined() -> M5ResolvedMotionEntry {
    let mut input = clean_motion_base(
        "motion:shell:raw-inlined",
        "motion.duration.focus",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::DurationFamily,
        M5MotionSurfaceClass::FocusTransition,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Shell,
    );
    input.references_canonical_token = false;
    motion(input)
}

/// Degraded motion entry: the motion surface class is unclassified.
fn motion_unclassified() -> M5ResolvedMotionEntry {
    motion(clean_motion_base(
        "motion:notification:unclassified",
        "motion.duration.unknown",
        M5VisualInteractionRole::Attention,
        M5MotionTokenRole::DurationFamily,
        M5MotionSurfaceClass::SurfaceClassUnclassified,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Notification,
    ))
}

/// Degraded motion entry: the canonical token name is unstated.
fn motion_token_unstated() -> M5ResolvedMotionEntry {
    let mut input = clean_motion_base(
        "motion:support:token-unstated",
        "  ",
        M5VisualInteractionRole::Motion,
        M5MotionTokenRole::DurationFamily,
        M5MotionSurfaceClass::ProgressIndicator,
        M5ReducedMotionFallback::OpacityCrossfade,
        M5MotionSurfaceContext::Shell,
    );
    input.token_name = "  ".to_owned();
    motion(input)
}

// -- Clean reduced-motion entries ---------------------------------------------------------------

fn clean_reduced_base(
    entry_id: &str,
    token_name: &str,
    reduced_motion_role: M5ReducedMotionRole,
    semantic_role: M5VisualInteractionRole,
    surface_context: M5MotionSurfaceContext,
) -> M5ReducedMotionEntryResolutionInput {
    M5ReducedMotionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        reduced_motion_role,
        semantic_role,
        surface_context,
        clamp_coverage: all_clamps(),
        references_canonical_token: true,
        static_fallback_preserves_meaning: true,
        proof_fresh: true,
    }
}

fn reduced_clamp_reduced_motion() -> M5ResolvedReducedMotionEntry {
    reduced(clean_reduced_base(
        "reduced:shell:reduced-motion",
        "reduced.clamp.reduced_motion",
        M5ReducedMotionRole::ReducedMotionClamp,
        M5VisualInteractionRole::Motion,
        M5MotionSurfaceContext::Shell,
    ))
}

fn reduced_clamp_power_saver() -> M5ResolvedReducedMotionEntry {
    reduced(clean_reduced_base(
        "reduced:dialog:power-saver",
        "reduced.clamp.power_saver",
        M5ReducedMotionRole::PowerSaverClamp,
        M5VisualInteractionRole::Motion,
        M5MotionSurfaceContext::Dialog,
    ))
}

fn reduced_clamp_thermal() -> M5ResolvedReducedMotionEntry {
    reduced(clean_reduced_base(
        "reduced:panel:thermal",
        "reduced.clamp.thermal",
        M5ReducedMotionRole::ThermalClamp,
        M5VisualInteractionRole::Motion,
        M5MotionSurfaceContext::Panel,
    ))
}

fn reduced_static_fallback() -> M5ResolvedReducedMotionEntry {
    reduced(clean_reduced_base(
        "reduced:embedded:static-fallback",
        "reduced.fallback.static",
        M5ReducedMotionRole::StaticFallbackEquivalent,
        M5VisualInteractionRole::Motion,
        M5MotionSurfaceContext::Embedded,
    ))
}

fn reduced_respects_pref() -> M5ResolvedReducedMotionEntry {
    reduced(clean_reduced_base(
        "reduced:notification:respects-pref",
        "reduced.pref.respect",
        M5ReducedMotionRole::RespectsUserPreference,
        M5VisualInteractionRole::Attention,
        M5MotionSurfaceContext::Notification,
    ))
}

// -- Degraded reduced-motion entries ------------------------------------------------------------

/// Degraded reduced-motion entry: meaning rides on motion alone with no static fallback.
fn reduced_motion_only() -> M5ResolvedReducedMotionEntry {
    reduced(clean_reduced_base(
        "reduced:shell:motion-only",
        "reduced.motion.only",
        M5ReducedMotionRole::MotionOnlyMeaningDisallowed,
        M5VisualInteractionRole::Motion,
        M5MotionSurfaceContext::Shell,
    ))
}

/// Degraded reduced-motion entry: the clamp coverage is incomplete.
fn reduced_clamp_incomplete() -> M5ResolvedReducedMotionEntry {
    let mut input = clean_reduced_base(
        "reduced:embedded:clamp-incomplete",
        "reduced.clamp.partial",
        M5ReducedMotionRole::ReducedMotionClamp,
        M5VisualInteractionRole::Motion,
        M5MotionSurfaceContext::Embedded,
    );
    input.clamp_coverage = vec![M5MotionClamp::ReducedMotion];
    reduced(input)
}

/// Degraded reduced-motion entry: the static fallback does not preserve meaning.
fn reduced_fallback_not_equiv() -> M5ResolvedReducedMotionEntry {
    let mut input = clean_reduced_base(
        "reduced:dialog:fallback-not-equiv",
        "reduced.fallback.partial",
        M5ReducedMotionRole::StaticFallbackEquivalent,
        M5VisualInteractionRole::Motion,
        M5MotionSurfaceContext::Dialog,
    );
    input.static_fallback_preserves_meaning = false;
    reduced(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5MotionRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    motion_entries: Vec<M5ResolvedMotionEntry>,
    reduced_motion_entries: Vec<M5ResolvedReducedMotionEntry>,
) -> M5MotionRegistriesRow {
    M5MotionRegistriesRow {
        consumer_surface,
        qualification: M5VisualInteractionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5VisualInteractionDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5VisualInteractionRequiredLabel::Identity,
            M5VisualInteractionRequiredLabel::SemanticRole,
            M5VisualInteractionRequiredLabel::TokenReference,
            M5VisualInteractionRequiredLabel::MotionProfile,
        ],
        accessibility_routes: M5VisualInteractionAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5MotionRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5MotionRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        motion_entries,
        reduced_motion_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MOTION_REGISTRIES_SCHEMA_REF,
            M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF,
        ]),
        motion_delays_protected_input: false,
        raw_duration_value_inlined_instead_of_token: false,
        layout_shift_on_protected_surface: false,
        clamp_coverage_incomplete: false,
    }
}

fn registry_rows() -> Vec<M5MotionRegistriesRow> {
    use M5VisualInteractionConsumerSurface as C;
    use M5VisualInteractionDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves command-palette and menu motion through the canonical instant grammar and never delays protected input; a motion entry that delays the palette and a reduced-motion entry that rides on motion alone degrade honestly instead of reading as a clean pass",
            "evidence:m5-motion-shell-ui:001",
            vec![
                D::MotionDelayedProtectedInput,
                D::MotionMeaningLostUnderReducedMotion,
                D::ProofStale,
            ],
            vec![
                motion_command_palette_clean(),
                motion_menu_clean(),
                motion_delays_protected(),
            ],
            vec![reduced_clamp_reduced_motion(), reduced_motion_only()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor keeps the typing caret and inline editor effectively instant with a static fallback across every clamp; a dropped fallback and an introduced layout shift both degrade honestly",
            "evidence:m5-motion-editor-ui:001",
            vec![
                D::MotionDelayedProtectedInput,
                D::MotionMeaningLostUnderReducedMotion,
                D::ProofStale,
            ],
            vec![
                motion_typing_clean(),
                motion_inline_editor_clean(),
                motion_fallback_missing(),
                motion_layout_shift(),
            ],
            vec![reduced_clamp_thermal()],
        ),
        base_row(
            C::OnboardingUi,
            "Onboarding surface owner",
            "The onboarding and diagnostic surfaces clarify origin and completion while keeping diagnostics instant and tracing each token to the canonical motion system; a clamp-incomplete motion entry and a clamp-incomplete reduced-motion entry degrade honestly",
            "evidence:m5-motion-onboarding-ui:001",
            vec![
                D::MotionMeaningLostUnderReducedMotion,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![
                motion_diagnostic_clean(),
                motion_onboarding_clean(),
                motion_clamp_incomplete(),
            ],
            vec![reduced_static_fallback(), reduced_clamp_incomplete()],
        ),
        base_row(
            C::MarketplaceUi,
            "Marketplace / embedded surface owner",
            "The embedded dialog surface consumes the canonical dialog entrance duration and traces every token to the motion system; a raw-duration motion entry and a non-equivalent static fallback degrade honestly",
            "evidence:m5-motion-marketplace-ui:001",
            vec![
                D::TokenReferenceUnstated,
                D::MotionMeaningLostUnderReducedMotion,
                D::ProofStale,
            ],
            vec![motion_dialog_clean(), motion_raw_inlined()],
            vec![reduced_clamp_power_saver(), reduced_fallback_not_equiv()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings and notification surfaces route attention with a completion cue and respect the user's reduced-motion preference; an unclassified surface class degrades honestly instead of animating outside the grammar",
            "evidence:m5-motion-settings-ui:001",
            vec![
                D::SemanticRoleUnstated,
                D::MotionMeaningLostUnderReducedMotion,
                D::ProofStale,
            ],
            vec![motion_notification_clean(), motion_unclassified()],
            vec![reduced_respects_pref()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved motion and reduced-motion truth, so a protected-path delay or an unstated token is visible in evidence rather than hidden behind an animation curve",
            "evidence:m5-motion-support-export:001",
            vec![
                D::TokenReferenceUnstated,
                D::MotionDelayedProtectedInput,
                D::ProofStale,
            ],
            vec![motion_menu_clean(), motion_token_unstated()],
            vec![reduced_clamp_reduced_motion()],
        ),
    ]
}

fn governance_review() -> M5MotionRegistriesGovernanceReview {
    M5MotionRegistriesGovernanceReview {
        motion_registry_names_token_role_and_surface_class: true,
        duration_easing_families_clarify_origin_and_completion: true,
        motion_never_delays_protected_input: true,
        every_motion_entry_covers_all_clamps: true,
        reduced_motion_power_saver_thermal_clamps_respected: true,
        reduced_motion_names_static_fallback_not_motion_only: true,
        motion_preserves_no_layout_shift: true,
        protected_path_animation_caught_before_release: true,
        first_consumers_use_canonical_motion_grammar: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5MotionRegistriesConsumerProjection {
    M5MotionRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        dialog_consumes_shared_registries: true,
        panel_consumes_shared_registries: true,
        embedded_and_notification_consume_shared_registries: true,
        motion_meaning_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5MotionRegistriesProofFreshness {
    M5MotionRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MotionRegistriesReleasePosture {
    M5MotionRegistriesReleasePosture {
        proof_packet_ref: M5_MOTION_REGISTRIES_ARTIFACT_REF.to_owned(),
        interaction_audit_ref: M5_MOTION_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MOTION_REGISTRIES_SCHEMA_REF,
        M5_MOTION_REGISTRIES_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 motion-token and reduced-motion registries packet.
pub fn seeded_m5_motion_reduced_motion_registries() -> M5MotionRegistriesPacket {
    M5MotionRegistriesPacket::new(M5MotionRegistriesPacketInput {
        packet_id: M5_MOTION_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 motion-token and reduced-motion registries with canonical duration / easing families, reduced-motion / power-saver / thermal clamp coverage, no-protected-path-delay and no-layout-shift guarantees for command-palette / menu / typing / inline-editor / diagnostic surfaces, static-fallback equivalence, and canonical-token tracing across shell, dialog, panel, embedded, notification, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5MotionRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shell-UI row is held at Beta pending protected-path-motion proof on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_motion_reduced_motion_registries_shell_ui_beta_narrowed(
) -> M5MotionRegistriesPacket {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.packet_id =
        "m5-motion-token-and-reduced-motion-registries:shell-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualInteractionConsumerSurface::ShellUi)
        .expect("shell-ui row present");
    row.qualification = M5VisualInteractionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the onboarding-UI row is narrowed to Preview pending reduced-motion-clamp parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_motion_reduced_motion_registries_onboarding_ui_preview_narrowed(
) -> M5MotionRegistriesPacket {
    let mut packet = seeded_m5_motion_reduced_motion_registries();
    packet.packet_id =
        "m5-motion-token-and-reduced-motion-registries:onboarding-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualInteractionConsumerSurface::OnboardingUi)
        .expect("onboarding-ui row present");
    row.qualification = M5VisualInteractionQualificationClass::Preview;
    packet
}
