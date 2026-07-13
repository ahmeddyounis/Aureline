//! Canonical seed builders for the M5 opacity / scrim and overlay-depth registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures. The
//! headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean scrim and overlay-depth entries are built so the
//! canonical overlay grammar, the blocking modal / sheet / confirm / wizard / credential depth classes, the
//! reduced-motion / power-saver / thermal clamp coverage, the contrast-and-orientation preservation, and the
//! shared-z-order stacking are proven across the shell, dialog, panel, embedded, notification, and support
//! surfaces without any orientation erasure, text-contrast loss, raw-opacity inlining, clamp gap, or private
//! z-order bypass.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_OVERLAY_REGISTRIES_PACKET_ID: &str =
    "m5-opacity-scrim-and-overlay-depth-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn scrim(input: M5ScrimEntryResolutionInput) -> M5ResolvedScrimEntry {
    resolve_scrim_entry(input).expect("seed scrim entry resolves")
}

fn depth(input: M5OverlayDepthEntryResolutionInput) -> M5ResolvedOverlayDepthEntry {
    resolve_overlay_depth_entry(input).expect("seed overlay-depth entry resolves")
}

fn all_clamps() -> Vec<M5OverlayRuntimeClamp> {
    M5OverlayRuntimeClamp::ALL.to_vec()
}

// -- Clean scrim entries (depth-class + scrim-role grammar across surfaces) -----------------------

#[allow(clippy::too_many_arguments)]
fn clean_scrim_base(
    entry_id: &str,
    token_name: &str,
    semantic_role: M5VisualInteractionRole,
    scrim_role: M5OpacityScrimRole,
    depth_class: M5OverlayDepthClass,
    contrast_treatment: M5ScrimContrastTreatment,
    surface_context: M5OverlaySurfaceContext,
) -> M5ScrimEntryResolutionInput {
    M5ScrimEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        scrim_role,
        depth_class,
        contrast_treatment,
        surface_context,
        clamp_coverage: all_clamps(),
        preserves_orientation: true,
        preserves_text_contrast: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn scrim_confirm_clean() -> M5ResolvedScrimEntry {
    scrim(clean_scrim_base(
        "scrim:shell:confirm",
        "scrim.blocking.confirm",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::OrientationPreserved,
        M5OverlayDepthClass::BlockingConfirmScrim,
        M5ScrimContrastTreatment::DimBackdropReadableText,
        M5OverlaySurfaceContext::Shell,
    ))
}

fn scrim_modal_clean() -> M5ResolvedScrimEntry {
    scrim(clean_scrim_base(
        "scrim:dialog:modal",
        "scrim.blocking.modal",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::ScrimLayer,
        M5OverlayDepthClass::BlockingModalDialog,
        M5ScrimContrastTreatment::DimBackdropReadableText,
        M5OverlaySurfaceContext::Dialog,
    ))
}

fn scrim_sheet_clean() -> M5ResolvedScrimEntry {
    scrim(clean_scrim_base(
        "scrim:panel:sheet",
        "scrim.blocking.sheet",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::DismissAffordance,
        M5OverlayDepthClass::BlockingSheet,
        M5ScrimContrastTreatment::SolidPanelBehindText,
        M5OverlaySurfaceContext::Panel,
    ))
}

fn scrim_wizard_clean() -> M5ResolvedScrimEntry {
    scrim(clean_scrim_base(
        "scrim:embedded:wizard",
        "scrim.blocking.wizard",
        M5VisualInteractionRole::Attention,
        M5OpacityScrimRole::ContrastPreserved,
        M5OverlayDepthClass::BlockingWizardStep,
        M5ScrimContrastTreatment::BlurWithContrastFloor,
        M5OverlaySurfaceContext::Embedded,
    ))
}

fn scrim_credential_clean() -> M5ResolvedScrimEntry {
    scrim(clean_scrim_base(
        "scrim:notification:credential",
        "scrim.blocking.credential",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::DismissAffordance,
        M5OverlayDepthClass::BlockingCredentialPrompt,
        M5ScrimContrastTreatment::HighContrastBorder,
        M5OverlaySurfaceContext::Notification,
    ))
}

fn scrim_popover_clean() -> M5ResolvedScrimEntry {
    scrim(clean_scrim_base(
        "scrim:shell:popover",
        "scrim.lightweight.popover",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::OpacityLevel,
        M5OverlayDepthClass::LightweightPopover,
        M5ScrimContrastTreatment::SolidPanelBehindText,
        M5OverlaySurfaceContext::Shell,
    ))
}

// -- Degraded scrim entries ---------------------------------------------------------------------

/// Degraded scrim entry: the scrim erases workspace orientation.
fn scrim_orientation_erased() -> M5ResolvedScrimEntry {
    let mut input = clean_scrim_base(
        "scrim:shell:orientation-erased",
        "scrim.blocking.confirm",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::OrientationPreserved,
        M5OverlayDepthClass::BlockingConfirmScrim,
        M5ScrimContrastTreatment::DimBackdropReadableText,
        M5OverlaySurfaceContext::Shell,
    );
    input.preserves_orientation = false;
    scrim(input)
}

/// Degraded scrim entry: the reduced-motion / power-saver / thermal clamp coverage is incomplete.
fn scrim_clamp_incomplete() -> M5ResolvedScrimEntry {
    let mut input = clean_scrim_base(
        "scrim:panel:clamp-incomplete",
        "scrim.lightweight.drawer",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::OpacityLevel,
        M5OverlayDepthClass::InlineDrawer,
        M5ScrimContrastTreatment::SolidPanelBehindText,
        M5OverlaySurfaceContext::Panel,
    );
    input.clamp_coverage = vec![
        M5OverlayRuntimeClamp::ReducedMotion,
        M5OverlayRuntimeClamp::PowerSaver,
    ];
    scrim(input)
}

/// Degraded scrim entry: no contrast treatment is paired with the scrim.
fn scrim_contrast_cue_missing() -> M5ResolvedScrimEntry {
    let mut input = clean_scrim_base(
        "scrim:embedded:contrast-missing",
        "scrim.blocking.wizard",
        M5VisualInteractionRole::Attention,
        M5OpacityScrimRole::ContrastPreserved,
        M5OverlayDepthClass::BlockingWizardStep,
        M5ScrimContrastTreatment::BlurWithContrastFloor,
        M5OverlaySurfaceContext::Embedded,
    );
    input.contrast_treatment = M5ScrimContrastTreatment::NoneDisallowed;
    scrim(input)
}

/// Degraded scrim entry: a raw opacity value is inlined instead of tracing to a canonical token.
fn scrim_raw_opacity() -> M5ResolvedScrimEntry {
    let mut input = clean_scrim_base(
        "scrim:dialog:raw-opacity",
        "scrim.blocking.modal",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::ScrimLayer,
        M5OverlayDepthClass::BlockingModalDialog,
        M5ScrimContrastTreatment::DimBackdropReadableText,
        M5OverlaySurfaceContext::Dialog,
    );
    input.references_canonical_token = false;
    scrim(input)
}

/// Degraded scrim entry: the overlay depth class is unclassified.
fn scrim_unclassified() -> M5ResolvedScrimEntry {
    scrim(clean_scrim_base(
        "scrim:notification:unclassified",
        "scrim.unknown.depth",
        M5VisualInteractionRole::Overlay,
        M5OpacityScrimRole::OpacityLevel,
        M5OverlayDepthClass::DepthClassUnclassified,
        M5ScrimContrastTreatment::HighContrastBorder,
        M5OverlaySurfaceContext::Notification,
    ))
}

/// Degraded scrim entry: the scrim drops text contrast beneath the overlay.
fn scrim_text_contrast_lost() -> M5ResolvedScrimEntry {
    let mut input = clean_scrim_base(
        "scrim:support:text-contrast-lost",
        "scrim.lightweight.toast",
        M5VisualInteractionRole::Attention,
        M5OpacityScrimRole::ContrastPreserved,
        M5OverlayDepthClass::TransientToast,
        M5ScrimContrastTreatment::DimBackdropReadableText,
        M5OverlaySurfaceContext::Shell,
    );
    input.preserves_text_contrast = false;
    scrim(input)
}

// -- Clean overlay-depth entries ----------------------------------------------------------------

fn clean_depth_base(
    entry_id: &str,
    token_name: &str,
    layer_order_role: M5LayerOrderRole,
    semantic_role: M5VisualInteractionRole,
    depth_class: M5OverlayDepthClass,
    surface_context: M5OverlaySurfaceContext,
) -> M5OverlayDepthEntryResolutionInput {
    M5OverlayDepthEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        layer_order_role,
        semantic_role,
        depth_class,
        surface_context,
        clamp_coverage: all_clamps(),
        references_canonical_token: true,
        stacks_under_shared_model: true,
        proof_fresh: true,
    }
}

fn depth_confirm_clean() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:shell:confirm",
        "layer.dialog.confirm",
        M5LayerOrderRole::DialogTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::BlockingConfirmScrim,
        M5OverlaySurfaceContext::Shell,
    ))
}

fn depth_modal_clean() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:dialog:modal",
        "layer.dialog.modal",
        M5LayerOrderRole::DialogTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::BlockingModalDialog,
        M5OverlaySurfaceContext::Dialog,
    ))
}

fn depth_sheet_clean() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:panel:sheet",
        "layer.dialog.sheet",
        M5LayerOrderRole::DialogTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::BlockingSheet,
        M5OverlaySurfaceContext::Panel,
    ))
}

fn depth_wizard_clean() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:embedded:wizard",
        "layer.dialog.wizard",
        M5LayerOrderRole::DialogTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::BlockingWizardStep,
        M5OverlaySurfaceContext::Embedded,
    ))
}

fn depth_credential_clean() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:notification:credential",
        "layer.dialog.credential",
        M5LayerOrderRole::DialogTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::BlockingCredentialPrompt,
        M5OverlaySurfaceContext::Notification,
    ))
}

fn depth_toast_clean() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:notification:toast",
        "layer.notification.toast",
        M5LayerOrderRole::NotificationTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::TransientToast,
        M5OverlaySurfaceContext::Notification,
    ))
}

fn depth_popover_clean() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:shell:popover",
        "layer.overlay.popover",
        M5LayerOrderRole::OverlayTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::LightweightPopover,
        M5OverlaySurfaceContext::Shell,
    ))
}

// -- Degraded overlay-depth entries -------------------------------------------------------------

/// Degraded overlay-depth entry: a private layer bypasses the shared z-order model.
fn depth_private_bypass() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:shell:private-bypass",
        "layer.private.bypass",
        M5LayerOrderRole::PrivateLayerBypassDisallowed,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::LightweightPopover,
        M5OverlaySurfaceContext::Shell,
    ))
}

/// Degraded overlay-depth entry: the overlay does not stack under the shared z-order model.
fn depth_not_stacked() -> M5ResolvedOverlayDepthEntry {
    let mut input = clean_depth_base(
        "depth:panel:not-stacked",
        "layer.overlay.detached",
        M5LayerOrderRole::OverlayTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::SidePanel,
        M5OverlaySurfaceContext::Panel,
    );
    input.stacks_under_shared_model = false;
    depth(input)
}

/// Degraded overlay-depth entry: the reduced-motion / power-saver / thermal clamp coverage is incomplete.
fn depth_clamp_incomplete() -> M5ResolvedOverlayDepthEntry {
    let mut input = clean_depth_base(
        "depth:embedded:clamp-incomplete",
        "layer.overlay.embedded",
        M5LayerOrderRole::OverlayTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::ContextMenu,
        M5OverlaySurfaceContext::Embedded,
    );
    input.clamp_coverage = vec![M5OverlayRuntimeClamp::ReducedMotion];
    depth(input)
}

/// Degraded overlay-depth entry: the overlay depth class is unclassified.
fn depth_unclassified() -> M5ResolvedOverlayDepthEntry {
    depth(clean_depth_base(
        "depth:dialog:unclassified",
        "layer.unknown.depth",
        M5LayerOrderRole::OverlayTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::DepthClassUnclassified,
        M5OverlaySurfaceContext::Dialog,
    ))
}

/// Degraded overlay-depth entry: the canonical token name is unstated.
fn depth_token_unstated() -> M5ResolvedOverlayDepthEntry {
    let mut input = clean_depth_base(
        "depth:support:token-unstated",
        "  ",
        M5LayerOrderRole::NotificationTier,
        M5VisualInteractionRole::Layer,
        M5OverlayDepthClass::StatusHud,
        M5OverlaySurfaceContext::Notification,
    );
    input.token_name = "  ".to_owned();
    depth(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5OverlayRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    scrim_entries: Vec<M5ResolvedScrimEntry>,
    overlay_depth_entries: Vec<M5ResolvedOverlayDepthEntry>,
) -> M5OverlayRegistriesRow {
    M5OverlayRegistriesRow {
        consumer_surface,
        qualification: M5VisualInteractionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5VisualInteractionDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5VisualInteractionRequiredLabel::Identity,
            M5VisualInteractionRequiredLabel::SemanticRole,
            M5VisualInteractionRequiredLabel::TokenReference,
            M5VisualInteractionRequiredLabel::LayerTier,
        ],
        accessibility_routes: M5VisualInteractionAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5OverlayRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5OverlayRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        scrim_entries,
        overlay_depth_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_OVERLAY_REGISTRIES_SCHEMA_REF,
            M5_OPACITY_SCRIM_SCHEMA_REF,
            M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
        ]),
        scrim_erases_orientation_or_contrast: false,
        raw_opacity_value_inlined_instead_of_token: false,
        overlay_bypasses_shared_z_order: false,
        runtime_clamp_coverage_incomplete: false,
    }
}

fn registry_rows() -> Vec<M5OverlayRegistriesRow> {
    use M5VisualInteractionConsumerSurface as C;
    use M5VisualInteractionDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the confirm scrim through the canonical blocking grammar and keeps the workspace orientable; a scrim that erases orientation and a private overlay that bypasses the shared z-order degrade honestly instead of reading as a clean pass",
            "evidence:m5-overlay-shell-ui:001",
            vec![
                D::ScrimErasedOrientationOrContrast,
                D::OverlayBypassedSharedZOrder,
                D::ProofStale,
            ],
            vec![scrim_confirm_clean(), scrim_orientation_erased()],
            vec![depth_confirm_clean(), depth_private_bypass()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor renders blocking sheets and lightweight popovers with a solid panel behind text and full clamp coverage; a clamp-incomplete scrim and a detached overlay that does not stack under the shared model both degrade honestly",
            "evidence:m5-overlay-editor-ui:001",
            vec![
                D::ScrimErasedOrientationOrContrast,
                D::OverlayBypassedSharedZOrder,
                D::ProofStale,
            ],
            vec![
                scrim_sheet_clean(),
                scrim_popover_clean(),
                scrim_clamp_incomplete(),
            ],
            vec![depth_sheet_clean(), depth_not_stacked()],
        ),
        base_row(
            C::OnboardingUi,
            "Onboarding surface owner",
            "The onboarding wizard blurs its backdrop with a contrast floor while keeping orientation and tracing each token to the canonical scrim system; a missing contrast treatment and a clamp-incomplete overlay depth degrade honestly",
            "evidence:m5-overlay-onboarding-ui:001",
            vec![
                D::ScrimErasedOrientationOrContrast,
                D::MotionMeaningLostUnderReducedMotion,
                D::ProofStale,
            ],
            vec![scrim_wizard_clean(), scrim_contrast_cue_missing()],
            vec![depth_wizard_clean(), depth_clamp_incomplete()],
        ),
        base_row(
            C::MarketplaceUi,
            "Marketplace / embedded surface owner",
            "The embedded dialog surface consumes the canonical blocking modal scrim and traces every token to the scrim system; a raw-opacity scrim and an unclassified overlay depth degrade honestly",
            "evidence:m5-overlay-marketplace-ui:001",
            vec![
                D::TokenReferenceUnstated,
                D::LayerTierUnstated,
                D::ProofStale,
            ],
            vec![scrim_modal_clean(), scrim_raw_opacity()],
            vec![depth_modal_clean(), depth_unclassified()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings and notification surfaces render the credential prompt and transient toast under the shared z-order model with a high-contrast border; an unclassified scrim depth and an unstated overlay token degrade honestly instead of stacking outside the grammar",
            "evidence:m5-overlay-settings-ui:001",
            vec![
                D::LayerTierUnstated,
                D::OverlayBypassedSharedZOrder,
                D::ProofStale,
            ],
            vec![scrim_credential_clean(), scrim_unclassified()],
            vec![
                depth_credential_clean(),
                depth_toast_clean(),
                depth_token_unstated(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved scrim and overlay-depth truth, so an orientation erasure or a private z-order bypass is visible in evidence rather than hidden behind an opacity value",
            "evidence:m5-overlay-support-export:001",
            vec![
                D::ScrimErasedOrientationOrContrast,
                D::OverlayBypassedSharedZOrder,
                D::ProofStale,
            ],
            vec![scrim_popover_clean(), scrim_text_contrast_lost()],
            vec![depth_popover_clean(), depth_not_stacked()],
        ),
    ]
}

fn governance_review() -> M5OverlayRegistriesGovernanceReview {
    M5OverlayRegistriesGovernanceReview {
        scrim_registry_names_token_role_and_depth_class: true,
        opacity_scrim_classes_distinguish_lightweight_from_blocking: true,
        scrim_never_erases_orientation: true,
        every_entry_covers_all_runtime_clamps: true,
        runtime_clamps_narrow_overlay_behavior_honestly: true,
        scrims_name_contrast_treatment_not_unreadable_backdrop: true,
        overlays_stack_under_one_shared_z_order_model: true,
        blocking_versus_nonblocking_depth_truth_caught_before_release: true,
        first_consumers_use_canonical_overlay_grammar: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5OverlayRegistriesConsumerProjection {
    M5OverlayRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        dialog_consumes_shared_registries: true,
        panel_consumes_shared_registries: true,
        embedded_and_notification_consume_shared_registries: true,
        overlay_meaning_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5OverlayRegistriesProofFreshness {
    M5OverlayRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5OverlayRegistriesReleasePosture {
    M5OverlayRegistriesReleasePosture {
        proof_packet_ref: M5_OVERLAY_REGISTRIES_ARTIFACT_REF.to_owned(),
        interaction_audit_ref: M5_OVERLAY_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_OVERLAY_REGISTRIES_SCHEMA_REF,
        M5_OVERLAY_REGISTRIES_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_OPACITY_SCRIM_SCHEMA_REF,
        M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 opacity / scrim and overlay-depth registries packet.
pub fn seeded_m5_opacity_scrim_overlay_depth_registries() -> M5OverlayRegistriesPacket {
    M5OverlayRegistriesPacket::new(M5OverlayRegistriesPacketInput {
        packet_id: M5_OVERLAY_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 opacity / scrim and overlay-depth registries with canonical lightweight versus blocking depth classes, reduced-motion / power-saver / thermal clamp coverage, orientation-and-text-contrast preservation for blocking modal / sheet / confirm / wizard / credential surfaces, and one shared z-order model no private overlay bypasses across shell, dialog, panel, embedded, notification, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5OverlayRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shell-UI row is held at Beta pending scrim-orientation proof on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_opacity_scrim_overlay_depth_registries_shell_ui_beta_narrowed(
) -> M5OverlayRegistriesPacket {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.packet_id =
        "m5-opacity-scrim-and-overlay-depth-registries:shell-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualInteractionConsumerSurface::ShellUi)
        .expect("shell-ui row present");
    row.qualification = M5VisualInteractionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the onboarding-UI row is narrowed to Preview pending blocking-depth parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_opacity_scrim_overlay_depth_registries_onboarding_ui_preview_narrowed(
) -> M5OverlayRegistriesPacket {
    let mut packet = seeded_m5_opacity_scrim_overlay_depth_registries();
    packet.packet_id =
        "m5-opacity-scrim-and-overlay-depth-registries:onboarding-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualInteractionConsumerSurface::OnboardingUi)
        .expect("onboarding-ui row present");
    row.qualification = M5VisualInteractionQualificationClass::Preview;
    packet
}
