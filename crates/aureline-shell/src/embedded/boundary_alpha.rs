//! Alpha projection for embedded-surface boundary chrome.
//!
//! The projection consumes [`EmbeddedBoundaryCardRecord`] payloads and produces
//! a compact, support-exportable row set that proves docs/help, extension
//! webview, and marketplace/account surfaces all disclose owner, origin,
//! profile or org scope, network posture, service boundary, browser fallback,
//! and host-owned approval boundaries with one shared vocabulary.

use std::path::Path;

use aureline_commands::invocation::now_rfc3339;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use super::boundary_card::{
    ActionPartitionRole, BoundaryActionId, BoundaryState, BrowserFallbackPostureClass,
    CapabilityLimitation, ChromeInheritanceAxis, DataBoundaryClass, EmbeddedBoundaryCardRecord,
    FallbackTargetClass, IdentityMode, NativeReservedSurface, OriginVerificationState,
    PermissionClass, ProviderHealthState, SurfaceFamily, TrustState,
};
use super::docs_help::seeded_docs_help_boundary_card;

/// Stable record-kind tag for [`EmbeddedBoundaryAlphaSnapshot`] payloads.
pub const EMBEDDED_BOUNDARY_ALPHA_SNAPSHOT_RECORD_KIND: &str =
    "embedded_boundary_alpha_snapshot_record";

/// Stable schema version for embedded boundary alpha snapshot payloads.
pub const EMBEDDED_BOUNDARY_ALPHA_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Exportable alpha snapshot of embedded-surface boundary chrome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAlphaSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Running build identity quoted into docs/help rows.
    pub build_identity_ref: String,
    /// Timestamp when the snapshot was projected.
    pub generated_at: String,
    /// Render-facing surface rows.
    pub surface_rows: Vec<EmbeddedBoundaryAlphaSurfaceRow>,
    /// Support/export-facing compact rows.
    pub support_rows: Vec<EmbeddedBoundaryAlphaSupportRow>,
}

impl EmbeddedBoundaryAlphaSnapshot {
    /// Returns the alpha surface row for `family`, if present.
    pub fn row_for_family(
        &self,
        family: SurfaceFamily,
    ) -> Option<&EmbeddedBoundaryAlphaSurfaceRow> {
        self.surface_rows
            .iter()
            .find(|row| row.surface_family == family)
    }

    /// Returns true when all rows keep native approval surfaces host-owned.
    pub fn all_high_risk_approval_host_owned(&self) -> bool {
        self.surface_rows
            .iter()
            .all(|row| row.high_risk_approval_host_owned)
    }
}

/// Render-facing alpha row for one embedded surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAlphaSurfaceRow {
    /// Stable card id.
    pub card_id: String,
    /// Surface id projected by the card.
    pub surface_id_ref: String,
    /// Embedded-surface family.
    pub surface_family: SurfaceFamily,
    /// Stable token for [`Self::surface_family`].
    pub surface_family_token: String,
    /// Owner label rendered in host chrome.
    pub owner_label: String,
    /// Publisher or service label rendered separately from owner.
    pub publisher_or_service_label: String,
    /// Origin label rendered in host chrome.
    pub origin_label: String,
    /// Host or domain label for the embedded content.
    pub host_or_domain_label: String,
    /// Origin verification token.
    pub origin_verification_token: String,
    /// Current profile, identity mode, or org/provider scope label.
    pub profile_or_org_scope_label: String,
    /// Network or provider health label.
    pub network_state_label: String,
    /// Service/data-boundary label rendered before the user acts.
    pub service_boundary_label: String,
    /// Data-boundary class token.
    pub data_boundary_class_token: String,
    /// Boundary-state token.
    pub boundary_state_token: String,
    /// Boundary-state label.
    pub boundary_state_label: String,
    /// Permission class token.
    pub permission_class_token: String,
    /// Browser-fallback posture token.
    pub browser_fallback_posture_token: String,
    /// Browser-fallback target token.
    pub fallback_target_class_token: String,
    /// True when a host-owned open-in-browser action is available.
    pub open_in_browser_available: bool,
    /// True when the fallback action preserves object identity.
    pub fallback_preserves_object_identity: bool,
    /// Host-owned native actions on the row.
    pub product_owned_native_action_tokens: Vec<String>,
    /// Host-owned handoff actions on the row.
    pub product_owned_handoff_action_tokens: Vec<String>,
    /// Embedded inspect/request-only actions on the row.
    pub embedded_limited_action_tokens: Vec<String>,
    /// Capability limitation tokens rendered on the card.
    pub capability_limitation_tokens: Vec<String>,
    /// Native-reserved high-risk surface tokens kept host-owned.
    pub native_reserved_surface_tokens: Vec<String>,
    /// Chrome inheritance tokens carried by the card.
    pub chrome_inheritance_tokens: Vec<String>,
    /// True when trust elevation, rollback, and AI apply are not delegated.
    pub high_risk_approval_host_owned: bool,
    /// Host-rendered plain-language summary.
    pub summary: String,
}

/// Export-safe alpha row for support bundles and release evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAlphaSupportRow {
    /// Stable card id.
    pub card_id: String,
    /// Surface family token.
    pub surface_family_token: String,
    /// Owner label.
    pub owner_label: String,
    /// Origin host/domain label.
    pub host_or_domain_label: String,
    /// Profile, identity, or org/provider scope label.
    pub profile_or_org_scope_label: String,
    /// Boundary state token.
    pub boundary_state_token: String,
    /// Network or provider health label.
    pub network_state_label: String,
    /// Browser-fallback posture token.
    pub browser_fallback_posture_token: String,
    /// Browser handoff packet ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// True when high-risk approval remains host-owned.
    pub high_risk_approval_host_owned: bool,
}

/// Builds the seeded alpha snapshot consumed by shell/runtime evidence.
pub fn seeded_embedded_boundary_alpha_snapshot(
    build_identity_ref: impl Into<String>,
) -> EmbeddedBoundaryAlphaSnapshot {
    let build_identity_ref = build_identity_ref.into();
    let cards = vec![
        seeded_docs_help_boundary_card(build_identity_ref.clone()),
        extension_webview_alpha_card_fixture(),
        marketplace_account_alpha_card_fixture(),
    ];
    materialize_embedded_boundary_alpha_snapshot(
        "embedded-boundary-alpha:seed",
        build_identity_ref,
        cards,
    )
}

/// Builds an alpha snapshot from concrete boundary cards.
pub fn materialize_embedded_boundary_alpha_snapshot(
    snapshot_id: impl Into<String>,
    build_identity_ref: impl Into<String>,
    cards: Vec<EmbeddedBoundaryCardRecord>,
) -> EmbeddedBoundaryAlphaSnapshot {
    let surface_rows: Vec<_> = cards
        .iter()
        .map(EmbeddedBoundaryAlphaSurfaceRow::from_card)
        .collect();
    let support_rows = surface_rows
        .iter()
        .zip(cards.iter())
        .map(|(row, card)| EmbeddedBoundaryAlphaSupportRow::from_row_and_card(row, card))
        .collect();
    EmbeddedBoundaryAlphaSnapshot {
        record_kind: EMBEDDED_BOUNDARY_ALPHA_SNAPSHOT_RECORD_KIND.to_owned(),
        schema_version: EMBEDDED_BOUNDARY_ALPHA_SNAPSHOT_SCHEMA_VERSION,
        snapshot_id: snapshot_id.into(),
        build_identity_ref: build_identity_ref.into(),
        generated_at: now_rfc3339(),
        surface_rows,
        support_rows,
    }
}

/// Writes an embedded-boundary alpha snapshot to
/// `<evidence_root>/embedded_boundary_alpha_latest.json`.
pub fn write_embedded_boundary_alpha_log(
    evidence_root: &Path,
    snapshot: &EmbeddedBoundaryAlphaSnapshot,
) -> Result<(), String> {
    std::fs::create_dir_all(evidence_root)
        .map_err(|err| format!("create embedded boundary evidence root failed: {err}"))?;
    let path = evidence_root.join("embedded_boundary_alpha_latest.json");
    let json = serde_json::to_string_pretty(snapshot)
        .map_err(|err| format!("serialize embedded boundary alpha snapshot failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))
}

impl EmbeddedBoundaryAlphaSurfaceRow {
    fn from_card(card: &EmbeddedBoundaryCardRecord) -> Self {
        let product_owned_native_action_tokens =
            action_tokens_for_role(card, ActionPartitionRole::ProductOwnedNative);
        let product_owned_handoff_action_tokens =
            action_tokens_for_role(card, ActionPartitionRole::ProductOwnedHandoff);
        let embedded_limited_action_tokens = card
            .action_partition
            .iter()
            .filter(|action| {
                matches!(
                    action.partition_role,
                    ActionPartitionRole::EmbeddedInspectOnly
                        | ActionPartitionRole::EmbeddedRequestOnly
                )
            })
            .map(|action| boundary_action_token(action.action_id).to_owned())
            .collect();
        let open_in_browser = card.open_in_browser_action();
        let fallback_preserves_object_identity = open_in_browser
            .map(|action| action.preserves_object_identity)
            .unwrap_or_else(|| {
                card.action_partition
                    .iter()
                    .filter(|action| {
                        matches!(
                            action.partition_role,
                            ActionPartitionRole::ProductOwnedHandoff
                                | ActionPartitionRole::ProductOwnedNative
                        )
                    })
                    .all(|action| action.preserves_object_identity)
            });
        let native_reserved_surface_tokens: Vec<_> = card
            .reserved_native_surfaces_host_owned
            .iter()
            .copied()
            .map(native_reserved_surface_token)
            .map(str::to_owned)
            .collect();
        let high_risk_approval_host_owned =
            required_native_reserved_surfaces().iter().all(|required| {
                card.reserved_native_surfaces_host_owned
                    .iter()
                    .any(|candidate| candidate == required)
            });

        Self {
            card_id: card.card_id.clone(),
            surface_id_ref: card.surface_id_ref.clone(),
            surface_family: card.surface_family,
            surface_family_token: surface_family_token(card.surface_family).to_owned(),
            owner_label: card.owner_identity.label.clone(),
            publisher_or_service_label: card.publisher_or_service_identity.label.clone(),
            origin_label: card.origin_identity.origin_label.clone(),
            host_or_domain_label: card.origin_identity.host_or_domain_label.clone(),
            origin_verification_token: origin_verification_token(
                card.origin_identity.verification_state,
            )
            .to_owned(),
            profile_or_org_scope_label: profile_or_org_scope_label(card),
            network_state_label: network_state_label(card),
            service_boundary_label: card.data_boundary_label.clone(),
            data_boundary_class_token: data_boundary_class_token(card.data_boundary_class)
                .to_owned(),
            boundary_state_token: boundary_state_token(card.boundary_state).to_owned(),
            boundary_state_label: card.boundary_state_label.clone(),
            permission_class_token: permission_class_token(card.permission_state.permission_class)
                .to_owned(),
            browser_fallback_posture_token: browser_fallback_posture_token(
                card.browser_fallback.posture_class,
            )
            .to_owned(),
            fallback_target_class_token: fallback_target_class_token(
                card.browser_fallback.fallback_target_class,
            )
            .to_owned(),
            open_in_browser_available: open_in_browser
                .and_then(|action| action.browser_handoff_packet_ref.as_deref())
                .map(|packet| !packet.is_empty())
                .unwrap_or(false),
            fallback_preserves_object_identity,
            product_owned_native_action_tokens,
            product_owned_handoff_action_tokens,
            embedded_limited_action_tokens,
            capability_limitation_tokens: card
                .capability_limitations
                .iter()
                .copied()
                .map(capability_limitation_token)
                .map(str::to_owned)
                .collect(),
            native_reserved_surface_tokens,
            chrome_inheritance_tokens: card
                .chrome_inheritance_axes
                .iter()
                .copied()
                .map(chrome_inheritance_token)
                .map(str::to_owned)
                .collect(),
            high_risk_approval_host_owned,
            summary: card.plain_language_summary.clone(),
        }
    }
}

impl EmbeddedBoundaryAlphaSupportRow {
    fn from_row_and_card(
        row: &EmbeddedBoundaryAlphaSurfaceRow,
        card: &EmbeddedBoundaryCardRecord,
    ) -> Self {
        Self {
            card_id: row.card_id.clone(),
            surface_family_token: row.surface_family_token.clone(),
            owner_label: row.owner_label.clone(),
            host_or_domain_label: row.host_or_domain_label.clone(),
            profile_or_org_scope_label: row.profile_or_org_scope_label.clone(),
            boundary_state_token: row.boundary_state_token.clone(),
            network_state_label: row.network_state_label.clone(),
            browser_fallback_posture_token: row.browser_fallback_posture_token.clone(),
            browser_handoff_packet_ref: card
                .open_in_browser_action()
                .and_then(|action| action.browser_handoff_packet_ref.clone())
                .or_else(|| card.browser_fallback.browser_handoff_packet_ref.clone()),
            high_risk_approval_host_owned: row.high_risk_approval_host_owned,
        }
    }
}

fn action_tokens_for_role(
    card: &EmbeddedBoundaryCardRecord,
    role: ActionPartitionRole,
) -> Vec<String> {
    card.action_partition
        .iter()
        .filter(|action| action.partition_role == role)
        .map(|action| boundary_action_token(action.action_id).to_owned())
        .collect()
}

fn required_native_reserved_surfaces() -> [NativeReservedSurface; 6] {
    [
        NativeReservedSurface::ProductSecurityMessaging,
        NativeReservedSurface::UpdateVerification,
        NativeReservedSurface::WorkspaceTrustElevation,
        NativeReservedSurface::RollbackOrRestoreConfirmation,
        NativeReservedSurface::AiApplyReview,
        NativeReservedSurface::HighRiskApprovalSheet,
    ]
}

fn profile_or_org_scope_label(card: &EmbeddedBoundaryCardRecord) -> String {
    if let Some(provider) = card.provider_identity.as_ref() {
        return provider.provider_scope_label.clone();
    }
    let mode = identity_mode_token(card.policy_context.identity_mode);
    match card.policy_context.execution_context_id.as_deref() {
        Some(context) => format!("{mode} / {context}"),
        None => mode.to_owned(),
    }
}

fn network_state_label(card: &EmbeddedBoundaryCardRecord) -> String {
    if let Some(provider) = card.provider_identity.as_ref() {
        if let Some(summary) = provider.health_summary_label.as_deref() {
            return summary.to_owned();
        }
        return provider_health_state_token(provider.health_state).to_owned();
    }

    match card.boundary_state {
        BoundaryState::LiveVerified => "live verified".to_owned(),
        BoundaryState::StaleSnapshot => "stale snapshot".to_owned(),
        BoundaryState::PolicyBlocked => "policy blocked".to_owned(),
        BoundaryState::CertificateFailed => "certificate failed".to_owned(),
        BoundaryState::CrossOriginLimited => "cross-origin limited".to_owned(),
        BoundaryState::OfflineSnapshot => "offline snapshot".to_owned(),
        BoundaryState::ExternalOpenOnly => "external open only".to_owned(),
    }
}

fn extension_webview_alpha_card_fixture() -> EmbeddedBoundaryCardRecord {
    parse_card_fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ux/embedded_boundary_alpha/extension_webview_alpha_card.json"
    )))
}

fn marketplace_account_alpha_card_fixture() -> EmbeddedBoundaryCardRecord {
    parse_card_fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ux/embedded_boundary_alpha/marketplace_account_alpha_card.json"
    )))
}

fn parse_card_fixture(payload: &str) -> EmbeddedBoundaryCardRecord {
    let value: JsonValue =
        serde_json::from_str(payload).expect("embedded boundary alpha fixture must parse");
    serde_json::from_value(value).expect("embedded boundary alpha fixture must match card shape")
}

fn surface_family_token(value: SurfaceFamily) -> &'static str {
    match value {
        SurfaceFamily::EmbeddedDocsHelp => "embedded_docs_help",
        SurfaceFamily::EmbeddedMarketplaceOrAccount => "embedded_marketplace_or_account",
        SurfaceFamily::EmbeddedServiceDashboard => "embedded_service_dashboard",
        SurfaceFamily::EmbeddedAuthConfirmation => "embedded_auth_confirmation",
        SurfaceFamily::ExtensionHostedSurface => "extension_hosted_surface",
    }
}

fn origin_verification_token(value: OriginVerificationState) -> &'static str {
    match value {
        OriginVerificationState::Verified => "verified",
        OriginVerificationState::Unverified => "unverified",
        OriginVerificationState::CertificateFailed => "certificate_failed",
        OriginVerificationState::PolicyBlocked => "policy_blocked",
        OriginVerificationState::CrossOriginLimited => "cross_origin_limited",
        OriginVerificationState::OfflineCached => "offline_cached",
    }
}

fn data_boundary_class_token(value: DataBoundaryClass) -> &'static str {
    match value {
        DataBoundaryClass::LocalProductBoundary => "local_product_boundary",
        DataBoundaryClass::FirstPartyHostedServiceBoundary => "first_party_hosted_service_boundary",
        DataBoundaryClass::ConnectedProviderBoundary => "connected_provider_boundary",
        DataBoundaryClass::CustomerControlPlaneBoundary => "customer_control_plane_boundary",
        DataBoundaryClass::ExtensionPublisherBoundary => "extension_publisher_boundary",
        DataBoundaryClass::CrossOriginLimitedBoundary => "cross_origin_limited_boundary",
    }
}

fn boundary_state_token(value: BoundaryState) -> &'static str {
    match value {
        BoundaryState::LiveVerified => "live_verified",
        BoundaryState::StaleSnapshot => "stale_snapshot",
        BoundaryState::PolicyBlocked => "policy_blocked",
        BoundaryState::CertificateFailed => "certificate_failed",
        BoundaryState::CrossOriginLimited => "cross_origin_limited",
        BoundaryState::OfflineSnapshot => "offline_snapshot",
        BoundaryState::ExternalOpenOnly => "external_open_only",
    }
}

fn permission_class_token(value: PermissionClass) -> &'static str {
    match value {
        PermissionClass::HostOwnedFullAuthority => "host_owned_full_authority",
        PermissionClass::HostOwnedInspectOnly => "host_owned_inspect_only",
        PermissionClass::HostOwnedBrowserOnly => "host_owned_browser_only",
        PermissionClass::HostOwnedCopyExportOnly => "host_owned_copy_export_only",
        PermissionClass::HostOwnedWithNativeStepUpRequired => {
            "host_owned_with_native_step_up_required"
        }
        PermissionClass::EmbeddedLowerTrustSessionRefresh => "embedded_lower_trust_session_refresh",
        PermissionClass::EmbeddedLowerTrustPasswordException => {
            "embedded_lower_trust_password_exception"
        }
        PermissionClass::NoPermissionWithinProduct => "no_permission_within_product",
    }
}

fn browser_fallback_posture_token(value: BrowserFallbackPostureClass) -> &'static str {
    match value {
        BrowserFallbackPostureClass::SystemBrowserFirst => "system_browser_first",
        BrowserFallbackPostureClass::DeviceCodeFallbackOffered => "device_code_fallback_offered",
        BrowserFallbackPostureClass::ExternalOpenBlockedByPolicy => {
            "external_open_blocked_by_policy"
        }
        BrowserFallbackPostureClass::ExternalOpenUnavailableOffline => {
            "external_open_unavailable_offline"
        }
        BrowserFallbackPostureClass::BrowserFallbackNotApplicable => {
            "browser_fallback_not_applicable"
        }
    }
}

fn fallback_target_class_token(value: FallbackTargetClass) -> &'static str {
    match value {
        FallbackTargetClass::SystemBrowserHandoffPacket => "system_browser_handoff_packet",
        FallbackTargetClass::DeviceCodeCompanionCard => "device_code_companion_card",
        FallbackTargetClass::PlatformAuthenticatorNative => "platform_authenticator_native",
        FallbackTargetClass::HostNativeReviewOrApproval => "host_native_review_or_approval",
        FallbackTargetClass::LocalInspectOrExport => "local_inspect_or_export",
        FallbackTargetClass::NoFallbackAvailable => "no_fallback_available",
    }
}

fn boundary_action_token(value: BoundaryActionId) -> &'static str {
    match value {
        BoundaryActionId::ReloadEmbeddedSurface => "reload_embedded_surface",
        BoundaryActionId::OpenInSystemBrowser => "open_in_system_browser",
        BoundaryActionId::SwitchToDeviceCode => "switch_to_device_code",
        BoundaryActionId::CopyDeviceCode => "copy_device_code",
        BoundaryActionId::RetryAuthHandoff => "retry_auth_handoff",
        BoundaryActionId::InspectCertificateDetails => "inspect_certificate_details",
        BoundaryActionId::InspectPolicyReason => "inspect_policy_reason",
        BoundaryActionId::ContinueLocalWithoutSurface => "continue_local_without_surface",
        BoundaryActionId::OpenSupportEvidence => "open_support_evidence",
        BoundaryActionId::ViewAuthExceptionRecord => "view_auth_exception_record",
    }
}

fn capability_limitation_token(value: CapabilityLimitation) -> &'static str {
    match value {
        CapabilityLimitation::CannotIssueNativeApproval => "cannot_issue_native_approval",
        CapabilityLimitation::CannotVerifyUpdatesOrSignatures => {
            "cannot_verify_updates_or_signatures"
        }
        CapabilityLimitation::CannotRaiseWorkspaceTrust => "cannot_raise_workspace_trust",
        CapabilityLimitation::CannotPerformRollbackOrRestore => {
            "cannot_perform_rollback_or_restore"
        }
        CapabilityLimitation::CannotApplyAiChanges => "cannot_apply_ai_changes",
        CapabilityLimitation::CookiesOrStorageOutsideProductBoundary => {
            "cookies_or_storage_outside_product_boundary"
        }
        CapabilityLimitation::CrossOriginDomOrStorageHidden => "cross_origin_dom_or_storage_hidden",
        CapabilityLimitation::LiveNetworkMutationDisabledWhenOffline => {
            "live_network_mutation_disabled_when_offline"
        }
        CapabilityLimitation::ProviderScopeMayBeNarrowerThanPageClaims => {
            "provider_scope_may_be_narrower_than_page_claims"
        }
        CapabilityLimitation::EmbeddedAuthLowerTrust => "embedded_auth_lower_trust",
    }
}

fn native_reserved_surface_token(value: NativeReservedSurface) -> &'static str {
    match value {
        NativeReservedSurface::ProductSecurityMessaging => "product_security_messaging",
        NativeReservedSurface::UpdateVerification => "update_verification",
        NativeReservedSurface::WorkspaceTrustElevation => "workspace_trust_elevation",
        NativeReservedSurface::RollbackOrRestoreConfirmation => "rollback_or_restore_confirmation",
        NativeReservedSurface::AiApplyReview => "ai_apply_review",
        NativeReservedSurface::HighRiskApprovalSheet => "high_risk_approval_sheet",
    }
}

fn chrome_inheritance_token(value: ChromeInheritanceAxis) -> &'static str {
    match value {
        ChromeInheritanceAxis::ThemePaletteInheritsFromHost => "theme_palette_inherits_from_host",
        ChromeInheritanceAxis::DensityClassInheritsFromHost => "density_class_inherits_from_host",
        ChromeInheritanceAxis::ZoomLevelInheritsFromHost => "zoom_level_inherits_from_host",
        ChromeInheritanceAxis::FocusRingInheritsFromHost => "focus_ring_inherits_from_host",
        ChromeInheritanceAxis::ReducedMotionPostureInheritsFromHost => {
            "reduced_motion_posture_inherits_from_host"
        }
        ChromeInheritanceAxis::HighContrastModeInheritsFromHost => {
            "high_contrast_mode_inherits_from_host"
        }
        ChromeInheritanceAxis::ForcedColorsModeInheritsFromHost => {
            "forced_colors_mode_inherits_from_host"
        }
    }
}

fn provider_health_state_token(value: ProviderHealthState) -> &'static str {
    match value {
        ProviderHealthState::Healthy => "healthy",
        ProviderHealthState::Degraded => "degraded",
        ProviderHealthState::Unavailable => "unavailable",
        ProviderHealthState::Revoked => "revoked",
        ProviderHealthState::Suspended => "suspended",
        ProviderHealthState::Expired => "expired",
    }
}

fn identity_mode_token(value: IdentityMode) -> &'static str {
    match value {
        IdentityMode::AccountFreeLocal => "account_free_local",
        IdentityMode::SelfHostedOrg => "self_hosted_org",
        IdentityMode::ManagedWorkspace => "managed_workspace",
    }
}

#[allow(dead_code)]
fn trust_state_token(value: TrustState) -> &'static str {
    match value {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_snapshot_covers_docs_extension_and_marketplace_boundary_truth() {
        let snapshot = seeded_embedded_boundary_alpha_snapshot("id:build:test:embedded-alpha");

        assert_eq!(
            snapshot.record_kind,
            EMBEDDED_BOUNDARY_ALPHA_SNAPSHOT_RECORD_KIND
        );
        assert_eq!(snapshot.surface_rows.len(), 3);
        assert!(snapshot
            .row_for_family(SurfaceFamily::EmbeddedDocsHelp)
            .is_some());
        let extension = snapshot
            .row_for_family(SurfaceFamily::ExtensionHostedSurface)
            .expect("extension-hosted surface row must exist");
        assert_eq!(extension.owner_label, "Acme Cloud extension panel");
        assert_eq!(extension.host_or_domain_label, "status.acme.example");
        assert!(extension
            .profile_or_org_scope_label
            .contains("ctx:extension"));
        assert_eq!(extension.network_state_label, "cross-origin limited");
        assert!(extension.open_in_browser_available);
        assert!(extension.fallback_preserves_object_identity);

        let marketplace = snapshot
            .row_for_family(SurfaceFamily::EmbeddedMarketplaceOrAccount)
            .expect("marketplace/account surface row must exist");
        assert_eq!(marketplace.host_or_domain_label, "marketplace.aureline.dev");
        assert!(marketplace
            .profile_or_org_scope_label
            .contains("Tenant acme-prod"));
        assert!(marketplace.network_state_label.contains("Session stale"));
        assert!(marketplace.open_in_browser_available);
        assert_eq!(
            marketplace.service_boundary_label,
            "Provider account surface (stale snapshot)."
        );
    }

    #[test]
    fn high_risk_native_approval_surfaces_remain_host_owned() {
        let snapshot = seeded_embedded_boundary_alpha_snapshot("id:build:test:embedded-alpha");
        assert!(snapshot.all_high_risk_approval_host_owned());
        for row in &snapshot.surface_rows {
            assert!(row
                .native_reserved_surface_tokens
                .contains(&"workspace_trust_elevation".to_owned()));
            assert!(row
                .native_reserved_surface_tokens
                .contains(&"ai_apply_review".to_owned()));
            assert!(row
                .native_reserved_surface_tokens
                .contains(&"high_risk_approval_sheet".to_owned()));
            assert!(!row
                .embedded_limited_action_tokens
                .contains(&"inspect_policy_reason".to_owned()));
        }
    }

    #[test]
    fn support_projection_preserves_owner_origin_scope_network_and_handoff() {
        let snapshot = seeded_embedded_boundary_alpha_snapshot("id:build:test:embedded-alpha");
        let extension = snapshot
            .support_rows
            .iter()
            .find(|row| row.surface_family_token == "extension_hosted_surface")
            .expect("extension support row must exist");
        assert_eq!(extension.owner_label, "Acme Cloud extension panel");
        assert_eq!(extension.host_or_domain_label, "status.acme.example");
        assert_eq!(
            extension.browser_handoff_packet_ref.as_deref(),
            Some("id:browser-handoff:extension:acme-status")
        );
        assert!(extension.high_risk_approval_host_owned);
    }

    #[test]
    fn fixture_cards_round_trip_through_card_contract() {
        let extension = extension_webview_alpha_card_fixture();
        let marketplace = marketplace_account_alpha_card_fixture();
        assert_eq!(
            extension.surface_family,
            SurfaceFamily::ExtensionHostedSurface
        );
        assert_eq!(
            marketplace.surface_family,
            SurfaceFamily::EmbeddedMarketplaceOrAccount
        );
        assert!(extension.open_in_browser_action().is_some());
        assert!(marketplace.open_in_browser_action().is_some());
    }
}
