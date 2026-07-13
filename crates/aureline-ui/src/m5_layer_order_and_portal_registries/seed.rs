//! Canonical seed builders for the M5 layer-order and portal registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures. The
//! headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean layer-tier and portal entries are built so the
//! canonical base / sticky / floating / menu / dialog / toast / critical z-tier ordering, the owning-surface
//! attachment and restore-safe portal semantics, and the single shared z-order model are proven across the
//! shell, dialog, panel, embedded, notification, and support surfaces without any hard-coded always-on-top
//! bypass, raw-z-index inlining, detached portal, or z-order fork.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_LAYER_PORTAL_REGISTRIES_PACKET_ID: &str =
    "m5-layer-order-and-portal-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn tier(input: M5LayerTierEntryResolutionInput) -> M5ResolvedLayerTierEntry {
    resolve_layer_tier_entry(input).expect("seed layer-tier entry resolves")
}

fn portal(input: M5PortalEntryResolutionInput) -> M5ResolvedPortalEntry {
    resolve_portal_entry(input).expect("seed portal entry resolves")
}

// -- Clean layer-tier entries (z-tier grammar across surfaces) -----------------------------------

fn clean_tier_base(
    entry_id: &str,
    token_name: &str,
    layer_order_role: M5LayerOrderRole,
    layer_tier: M5LayerTier,
    surface_context: M5LayerPortalSurfaceContext,
) -> M5LayerTierEntryResolutionInput {
    M5LayerTierEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualInteractionRole::Layer,
        layer_order_role,
        layer_tier,
        surface_context,
        hardcodes_always_on_top: false,
        stacks_under_shared_model: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn tier_base_clean() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:shell:base",
        "layer.base.workspace",
        M5LayerOrderRole::BaseContentTier,
        M5LayerTier::Base,
        M5LayerPortalSurfaceContext::Shell,
    ))
}

fn tier_sticky_clean() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:shell:sticky",
        "layer.sticky.affix",
        M5LayerOrderRole::BaseContentTier,
        M5LayerTier::Sticky,
        M5LayerPortalSurfaceContext::Shell,
    ))
}

fn tier_floating_clean() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:panel:floating",
        "layer.floating.hover_peek",
        M5LayerOrderRole::OverlayTier,
        M5LayerTier::Floating,
        M5LayerPortalSurfaceContext::Panel,
    ))
}

fn tier_menu_clean() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:shell:menu",
        "layer.menu.palette",
        M5LayerOrderRole::OverlayTier,
        M5LayerTier::Menu,
        M5LayerPortalSurfaceContext::Shell,
    ))
}

fn tier_dialog_clean() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:dialog:dialog",
        "layer.dialog.modal",
        M5LayerOrderRole::DialogTier,
        M5LayerTier::Dialog,
        M5LayerPortalSurfaceContext::Dialog,
    ))
}

fn tier_toast_clean() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:notification:toast",
        "layer.toast.transient",
        M5LayerOrderRole::NotificationTier,
        M5LayerTier::Toast,
        M5LayerPortalSurfaceContext::Notification,
    ))
}

fn tier_critical_clean() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:notification:critical",
        "layer.critical.credential",
        M5LayerOrderRole::DialogTier,
        M5LayerTier::Critical,
        M5LayerPortalSurfaceContext::Notification,
    ))
}

// -- Degraded layer-tier entries ----------------------------------------------------------------

/// Degraded layer-tier entry: a hard-coded always-on-top overlay bypasses the shared model.
fn tier_always_on_top() -> M5ResolvedLayerTierEntry {
    let mut input = clean_tier_base(
        "tier:shell:always-on-top",
        "layer.menu.palette",
        M5LayerOrderRole::OverlayTier,
        M5LayerTier::Menu,
        M5LayerPortalSurfaceContext::Shell,
    );
    input.hardcodes_always_on_top = true;
    tier(input)
}

/// Degraded layer-tier entry: a private layer bypasses the shared z-order model.
fn tier_private_bypass() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:settings:private-bypass",
        "layer.private.bypass",
        M5LayerOrderRole::PrivateLayerBypassDisallowed,
        M5LayerTier::Floating,
        M5LayerPortalSurfaceContext::Panel,
    ))
}

/// Degraded layer-tier entry: the tier does not stack under the shared z-order model.
fn tier_not_stacked() -> M5ResolvedLayerTierEntry {
    let mut input = clean_tier_base(
        "tier:editor:not-stacked",
        "layer.floating.detached",
        M5LayerOrderRole::OverlayTier,
        M5LayerTier::Floating,
        M5LayerPortalSurfaceContext::Panel,
    );
    input.stacks_under_shared_model = false;
    tier(input)
}

/// Degraded layer-tier entry: a raw z-index value is inlined instead of a canonical token.
fn tier_raw_z_order() -> M5ResolvedLayerTierEntry {
    let mut input = clean_tier_base(
        "tier:marketplace:raw-z-order",
        "layer.critical.credential",
        M5LayerOrderRole::DialogTier,
        M5LayerTier::Critical,
        M5LayerPortalSurfaceContext::Embedded,
    );
    input.references_canonical_token = false;
    tier(input)
}

/// Degraded layer-tier entry: the z-tier is unclassified.
fn tier_unclassified() -> M5ResolvedLayerTierEntry {
    tier(clean_tier_base(
        "tier:onboarding:unclassified",
        "layer.unknown.tier",
        M5LayerOrderRole::OverlayTier,
        M5LayerTier::TierUnclassified,
        M5LayerPortalSurfaceContext::Dialog,
    ))
}

// -- Clean portal entries ------------------------------------------------------------------------

fn clean_portal_base(
    entry_id: &str,
    token_name: &str,
    portal_ownership_role: M5PortalOwnershipRole,
    layer_tier: M5LayerTier,
    attachment_mode: M5PortalAttachmentMode,
    surface_context: M5LayerPortalSurfaceContext,
) -> M5PortalEntryResolutionInput {
    M5PortalEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        portal_ownership_role,
        semantic_role: M5VisualInteractionRole::Portal,
        layer_tier,
        attachment_mode,
        surface_context,
        attaches_to_owning_surface: true,
        restore_safe: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn portal_shell_clean() -> M5ResolvedPortalEntry {
    portal(clean_portal_base(
        "portal:shell:palette",
        "portal.shell.palette",
        M5PortalOwnershipRole::OwningSurfaceAttachment,
        M5LayerTier::Menu,
        M5PortalAttachmentMode::OwningWindowAnchored,
        M5LayerPortalSurfaceContext::Shell,
    ))
}

fn portal_dialog_clean() -> M5ResolvedPortalEntry {
    portal(clean_portal_base(
        "portal:dialog:modal",
        "portal.dialog.modal",
        M5PortalOwnershipRole::FocusScopeContainment,
        M5LayerTier::Dialog,
        M5PortalAttachmentMode::FocusScopeContained,
        M5LayerPortalSurfaceContext::Dialog,
    ))
}

fn portal_panel_clean() -> M5ResolvedPortalEntry {
    portal(clean_portal_base(
        "portal:panel:hover-peek",
        "portal.panel.hover_peek",
        M5PortalOwnershipRole::OwnerDrivenDismissal,
        M5LayerTier::Floating,
        M5PortalAttachmentMode::OwnerDrivenTeardown,
        M5LayerPortalSurfaceContext::Panel,
    ))
}

fn portal_embedded_clean() -> M5ResolvedPortalEntry {
    portal(clean_portal_base(
        "portal:embedded:boundary",
        "portal.embedded.boundary",
        M5PortalOwnershipRole::ExtensionPortalGoverned,
        M5LayerTier::Dialog,
        M5PortalAttachmentMode::RestoreSafeReparent,
        M5LayerPortalSurfaceContext::Embedded,
    ))
}

fn portal_notification_clean() -> M5ResolvedPortalEntry {
    portal(clean_portal_base(
        "portal:notification:toast",
        "portal.notification.toast",
        M5PortalOwnershipRole::StacksUnderSharedModel,
        M5LayerTier::Toast,
        M5PortalAttachmentMode::OwningWindowAnchored,
        M5LayerPortalSurfaceContext::Notification,
    ))
}

// -- Degraded portal entries --------------------------------------------------------------------

/// Degraded portal entry: the portal detaches from its owning surface.
fn portal_detached() -> M5ResolvedPortalEntry {
    let mut input = clean_portal_base(
        "portal:shell:detached",
        "portal.shell.detached",
        M5PortalOwnershipRole::OwningSurfaceAttachment,
        M5LayerTier::Menu,
        M5PortalAttachmentMode::OwningWindowAnchored,
        M5LayerPortalSurfaceContext::Shell,
    );
    input.attaches_to_owning_surface = false;
    portal(input)
}

/// Degraded portal entry: a disallowed detached-portal role strands an orphaned overlay.
fn portal_role_detached() -> M5ResolvedPortalEntry {
    portal(clean_portal_base(
        "portal:marketplace:role-detached",
        "portal.marketplace.orphaned",
        M5PortalOwnershipRole::DetachedPortalDisallowed,
        M5LayerTier::Dialog,
        M5PortalAttachmentMode::OwningWindowAnchored,
        M5LayerPortalSurfaceContext::Embedded,
    ))
}

/// Degraded portal entry: the portal does not restore safely when its owner changes.
fn portal_restore_unsafe() -> M5ResolvedPortalEntry {
    let mut input = clean_portal_base(
        "portal:editor:restore-unsafe",
        "portal.editor.reparent",
        M5PortalOwnershipRole::OwnerDrivenDismissal,
        M5LayerTier::Floating,
        M5PortalAttachmentMode::RestoreSafeReparent,
        M5LayerPortalSurfaceContext::Panel,
    );
    input.restore_safe = false;
    portal(input)
}

/// Degraded portal entry: no attachment mode is paired with the portal.
fn portal_attachment_missing() -> M5ResolvedPortalEntry {
    let mut input = clean_portal_base(
        "portal:onboarding:attachment-missing",
        "portal.onboarding.step",
        M5PortalOwnershipRole::OwningSurfaceAttachment,
        M5LayerTier::Dialog,
        M5PortalAttachmentMode::RestoreSafeReparent,
        M5LayerPortalSurfaceContext::Embedded,
    );
    input.attachment_mode = M5PortalAttachmentMode::NoneDisallowed;
    portal(input)
}

/// Degraded portal entry: the canonical token name is unstated.
fn portal_token_unstated() -> M5ResolvedPortalEntry {
    let mut input = clean_portal_base(
        "portal:settings:token-unstated",
        "  ",
        M5PortalOwnershipRole::StacksUnderSharedModel,
        M5LayerTier::Toast,
        M5PortalAttachmentMode::OwningWindowAnchored,
        M5LayerPortalSurfaceContext::Notification,
    );
    input.token_name = "  ".to_owned();
    portal(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5LayerPortalRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    layer_tier_entries: Vec<M5ResolvedLayerTierEntry>,
    portal_entries: Vec<M5ResolvedPortalEntry>,
) -> M5LayerPortalRegistriesRow {
    M5LayerPortalRegistriesRow {
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
        anatomy_parts: M5LayerPortalRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5LayerPortalRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        layer_tier_entries,
        portal_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_LAYER_PORTAL_REGISTRIES_SCHEMA_REF,
            M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
        ]),
        overlay_hardcodes_always_on_top: false,
        portal_detaches_from_owning_surface: false,
        raw_z_order_value_inlined_instead_of_token: false,
        layer_order_bypasses_shared_z_order_model: false,
    }
}

fn registry_rows() -> Vec<M5LayerPortalRegistriesRow> {
    use M5VisualInteractionConsumerSurface as C;
    use M5VisualInteractionDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the base workspace and palette menu tier through the canonical z-tier grammar and anchors its palette portal to the owning window; a hard-coded always-on-top overlay and a detached portal degrade honestly instead of reading as a clean pass",
            "evidence:m5-layer-portal-shell-ui:001",
            vec![
                D::OverlayBypassedSharedZOrder,
                D::PortalDetachedFromOwningSurface,
                D::ProofStale,
            ],
            vec![tier_base_clean(), tier_menu_clean(), tier_always_on_top()],
            vec![portal_shell_clean(), portal_detached()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor renders the sticky affix and floating hover / peek tiers under the shared model and tears its peek portal down with its owner; a tier that stacks outside the shared model and a restore-unsafe portal both degrade honestly",
            "evidence:m5-layer-portal-editor-ui:001",
            vec![
                D::OverlayBypassedSharedZOrder,
                D::PortalDetachedFromOwningSurface,
                D::ProofStale,
            ],
            vec![tier_sticky_clean(), tier_floating_clean(), tier_not_stacked()],
            vec![portal_panel_clean(), portal_restore_unsafe()],
        ),
        base_row(
            C::OnboardingUi,
            "Onboarding surface owner",
            "The onboarding wizard renders the dialog tier and re-parents its embedded step portal restore-safe; an unclassified z-tier and a portal missing its attachment mode degrade honestly instead of stranding an orphaned overlay",
            "evidence:m5-layer-portal-onboarding-ui:001",
            vec![
                D::LayerTierUnstated,
                D::PortalDetachedFromOwningSurface,
                D::ProofStale,
            ],
            vec![tier_dialog_clean(), tier_unclassified()],
            vec![portal_embedded_clean(), portal_attachment_missing()],
        ),
        base_row(
            C::MarketplaceUi,
            "Marketplace / embedded surface owner",
            "The embedded marketplace surface renders the critical prompt tier and governs its extension portal under the shared model; a raw z-index inlined instead of a canonical token and a disallowed detached portal role degrade honestly",
            "evidence:m5-layer-portal-marketplace-ui:001",
            vec![
                D::TokenReferenceUnstated,
                D::PortalDetachedFromOwningSurface,
                D::ProofStale,
            ],
            vec![tier_critical_clean(), tier_raw_z_order()],
            vec![portal_dialog_clean(), portal_role_detached()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings and notification surfaces render the transient toast tier under the shared z-order model and anchor the notification portal to the owning window; a private layer that bypasses the shared model and an unstated portal token degrade honestly",
            "evidence:m5-layer-portal-settings-ui:001",
            vec![
                D::OverlayBypassedSharedZOrder,
                D::TokenReferenceUnstated,
                D::ProofStale,
            ],
            vec![tier_toast_clean(), tier_private_bypass()],
            vec![portal_notification_clean(), portal_token_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved layer-tier and portal truth, so a hard-coded always-on-top bypass or a detached portal is visible in evidence rather than hidden behind a raw z-index",
            "evidence:m5-layer-portal-support-export:001",
            vec![
                D::OverlayBypassedSharedZOrder,
                D::PortalDetachedFromOwningSurface,
                D::ProofStale,
            ],
            vec![tier_menu_clean(), tier_not_stacked()],
            vec![portal_notification_clean(), portal_detached()],
        ),
    ]
}

fn governance_review() -> M5LayerPortalRegistriesGovernanceReview {
    M5LayerPortalRegistriesGovernanceReview {
        layer_tier_registry_names_token_role_and_tier: true,
        z_tier_registry_covers_canonical_ordering: true,
        no_overlay_hardcodes_always_on_top: true,
        competing_tiers_stack_under_one_shared_model: true,
        portals_attach_to_owning_surface_and_restore_safely: true,
        portals_name_attachment_mode_not_orphaned_overlay: true,
        overlays_stack_under_one_shared_z_order_model: true,
        layer_order_drift_caught_before_release: true,
        first_consumers_use_canonical_layer_grammar: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5LayerPortalRegistriesConsumerProjection {
    M5LayerPortalRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        dialog_consumes_shared_registries: true,
        panel_consumes_shared_registries: true,
        embedded_and_notification_consume_shared_registries: true,
        layer_meaning_traces_to_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5LayerPortalRegistriesProofFreshness {
    M5LayerPortalRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5LayerPortalRegistriesReleasePosture {
    M5LayerPortalRegistriesReleasePosture {
        proof_packet_ref: M5_LAYER_PORTAL_REGISTRIES_ARTIFACT_REF.to_owned(),
        interaction_audit_ref: M5_LAYER_PORTAL_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LAYER_PORTAL_REGISTRIES_SCHEMA_REF,
        M5_LAYER_PORTAL_REGISTRIES_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 layer-order and portal registries packet.
pub fn seeded_m5_layer_order_and_portal_registries() -> M5LayerPortalRegistriesPacket {
    M5LayerPortalRegistriesPacket::new(M5LayerPortalRegistriesPacketInput {
        packet_id: M5_LAYER_PORTAL_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 layer-order and portal registries with a canonical base / sticky / floating / menu / dialog / toast / critical z-tier ordering, owning-surface attachment and restore-safe portal semantics, no hard-coded always-on-top bypass, and one shared z-order model no private overlay bypasses across shell, dialog, panel, embedded, notification, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5LayerPortalRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shell-UI row is held at Beta pending owning-window portal proof on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_layer_order_and_portal_registries_shell_ui_beta_narrowed(
) -> M5LayerPortalRegistriesPacket {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.packet_id = "m5-layer-order-and-portal-registries:shell-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualInteractionConsumerSurface::ShellUi)
        .expect("shell-ui row present");
    row.qualification = M5VisualInteractionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the onboarding-UI row is narrowed to Preview pending portal restore-safe parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_layer_order_and_portal_registries_onboarding_ui_preview_narrowed(
) -> M5LayerPortalRegistriesPacket {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.packet_id = "m5-layer-order-and-portal-registries:onboarding-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualInteractionConsumerSurface::OnboardingUi)
        .expect("onboarding-ui row present");
    row.qualification = M5VisualInteractionQualificationClass::Preview;
    packet
}
