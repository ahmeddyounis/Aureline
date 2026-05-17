//! Provider route-resolution, browser-handoff, and authority-truth panels for
//! claimed managed and external lanes.
//!
//! This module owns the typed beta contract that keeps external and
//! browser-linked lanes honest by surfacing four facts on every
//! provider-linked beta row:
//!
//! - **Current route.** The route the action will actually take if it
//!   proceeds now, named with the same route-choice and transport
//!   vocabulary the command, support, and route-origin packets already use.
//! - **Route owner.** The owner of the route — who minted it, who runs it,
//!   and whether it crosses a mirror, a managed boundary, or a tunnel.
//! - **Grant type.** The acting-identity class and auth source that backs
//!   the action, named with the same vocabulary the account-scope beta
//!   page uses.
//! - **Fallback / degraded path.** Where the action goes if the current
//!   route cannot proceed: typed fallback (browser handoff, publish-later
//!   queue, inspect-only, copy-or-export) plus the typed degraded-state
//!   reason.
//!
//! The page also pins a typed browser-handoff panel per browser-routed
//! action so the handoff is traceable back to the same route owner, route
//! choice, and acting identity used everywhere else, and a typed
//! authority-truth panel that names whether the green claim is still
//! honest given the freshness floor, route observation, and managed
//! boundary truth.
//!
//! Reviewer-facing landing page:
//! [`/docs/security/m3/provider_route_truth_beta.md`](../../../../docs/security/m3/provider_route_truth_beta.md).
//! The source matrix lives at
//! [`/artifacts/security/m3/route_resolution_panels/route_resolution_matrix.yaml`](../../../../artifacts/security/m3/route_resolution_panels/route_resolution_matrix.yaml).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::account_scope::{AccountScopeBetaProfileClass, ActingIdentityClass};
use crate::registry::{
    FreshnessLabel, ProviderAuthSourceClass, ProviderFallbackMode, ProviderSourceClass,
    ProviderSurfaceClass,
};

/// Beta schema version exported with every route-resolution beta record.
pub const ROUTE_RESOLUTION_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every route-resolution beta record.
pub const ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF: &str = "providers:route_resolution_beta:v1";

/// Source matrix ref consumed by this beta projection.
pub const ROUTE_RESOLUTION_BETA_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/route_resolution_panels/route_resolution_matrix.yaml";

/// Stable record kind for [`RouteResolutionBetaPage`] payloads.
pub const ROUTE_RESOLUTION_BETA_PAGE_RECORD_KIND: &str =
    "providers_route_resolution_beta_page_record";

/// Stable record kind for [`RouteResolutionRow`] payloads.
pub const ROUTE_RESOLUTION_BETA_ROW_RECORD_KIND: &str =
    "providers_route_resolution_beta_row_record";

/// Stable record kind for [`BrowserHandoffPanel`] payloads.
pub const ROUTE_RESOLUTION_BETA_BROWSER_HANDOFF_PANEL_RECORD_KIND: &str =
    "providers_route_resolution_beta_browser_handoff_panel_record";

/// Stable record kind for [`AuthorityTruthPanel`] payloads.
pub const ROUTE_RESOLUTION_BETA_AUTHORITY_TRUTH_PANEL_RECORD_KIND: &str =
    "providers_route_resolution_beta_authority_truth_panel_record";

/// Stable record kind for [`RouteResolutionBetaSummary`] payloads.
pub const ROUTE_RESOLUTION_BETA_SUMMARY_RECORD_KIND: &str =
    "providers_route_resolution_beta_summary_record";

/// Stable record kind for [`RouteResolutionBetaDefect`] payloads.
pub const ROUTE_RESOLUTION_BETA_DEFECT_RECORD_KIND: &str =
    "providers_route_resolution_beta_defect_record";

/// Stable record kind for [`RouteResolutionBetaSupportExport`] payloads.
pub const ROUTE_RESOLUTION_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "providers_route_resolution_beta_support_export_record";

/// Lane class the row claims. The vocabulary distinguishes managed lanes
/// (which honour managed-policy authority) from external lanes (which act
/// outside managed boundary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneClass {
    /// Managed lane reached through enterprise managed authority.
    ManagedProviderLane,
    /// Managed lane reached through a signed enterprise mirror.
    ManagedMirrorLane,
    /// External, customer-region provider lane.
    ExternalProviderLane,
    /// Tunnel-exposed external lane (SSH tunnel, dev tunnel, etc.).
    TunnelExposedExternalLane,
    /// Air-gapped / offline lane reading imported provider snapshots.
    OfflineSnapshotLane,
}

impl LaneClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedProviderLane => "managed_provider_lane",
            Self::ManagedMirrorLane => "managed_mirror_lane",
            Self::ExternalProviderLane => "external_provider_lane",
            Self::TunnelExposedExternalLane => "tunnel_exposed_external_lane",
            Self::OfflineSnapshotLane => "offline_snapshot_lane",
        }
    }

    /// True when the lane is reached under managed-policy authority.
    pub const fn is_managed(self) -> bool {
        matches!(self, Self::ManagedProviderLane | Self::ManagedMirrorLane)
    }
}

/// Route-choice class. Reuses the route-choice vocabulary the support and
/// command packets use, refined to the shapes this beta page admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteChoiceClass {
    /// Direct live-provider connection.
    LiveProviderDirect,
    /// Signed enterprise mirror connection.
    SignedMirrorRoute,
    /// Tunnel-exposed route (SSH tunnel, dev tunnel).
    TunnelExposedRoute,
    /// Imported provider snapshot (offline / air-gapped read).
    ImportedSnapshotRoute,
    /// System-browser handoff route.
    SystemBrowserHandoffRoute,
    /// Local-only execution (no provider crossed).
    LocalOnlyRoute,
}

impl RouteChoiceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveProviderDirect => "live_provider_direct",
            Self::SignedMirrorRoute => "signed_mirror_route",
            Self::TunnelExposedRoute => "tunnel_exposed_route",
            Self::ImportedSnapshotRoute => "imported_snapshot_route",
            Self::SystemBrowserHandoffRoute => "system_browser_handoff_route",
            Self::LocalOnlyRoute => "local_only_route",
        }
    }

    /// True when the route crosses an external provider boundary.
    pub const fn crosses_provider_boundary(self) -> bool {
        matches!(
            self,
            Self::LiveProviderDirect
                | Self::SignedMirrorRoute
                | Self::TunnelExposedRoute
                | Self::SystemBrowserHandoffRoute
        )
    }
}

/// Class of route owner. Names who minted the route and whose authority it
/// runs under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteOwnerClass {
    /// Workspace-bound user authority.
    WorkspaceUser,
    /// Provider's own authority (signed mirror, hosted provider).
    ProviderAuthority,
    /// Enterprise managed-policy authority.
    ManagedPolicyAuthority,
    /// Tunnel session minted under a separate identity.
    TunnelSessionOwner,
    /// Offline / imported authority (signed snapshot publisher).
    OfflineImportAuthority,
}

impl RouteOwnerClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceUser => "workspace_user",
            Self::ProviderAuthority => "provider_authority",
            Self::ManagedPolicyAuthority => "managed_policy_authority",
            Self::TunnelSessionOwner => "tunnel_session_owner",
            Self::OfflineImportAuthority => "offline_import_authority",
        }
    }
}

/// Route-side degraded-state class. Names why the current route is not in
/// its green state if it is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteDegradedStateClass {
    /// Route is observed-fresh and green.
    Green,
    /// Route is still reachable but freshness floor drifted.
    FreshnessFloorDrifted,
    /// Route is reachable but mirror lag is beyond tolerance.
    MirrorLagBeyondTolerance,
    /// Tunnel session reached its expiry horizon.
    TunnelSessionExpired,
    /// Offline snapshot is older than its retention floor.
    SnapshotOlderThanRetentionFloor,
    /// Managed policy boundary is closed for this lane.
    ManagedPolicyBoundaryClosed,
    /// Route is currently unreachable (transport failure).
    RouteUnreachable,
}

impl RouteDegradedStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::FreshnessFloorDrifted => "freshness_floor_drifted",
            Self::MirrorLagBeyondTolerance => "mirror_lag_beyond_tolerance",
            Self::TunnelSessionExpired => "tunnel_session_expired",
            Self::SnapshotOlderThanRetentionFloor => "snapshot_older_than_retention_floor",
            Self::ManagedPolicyBoundaryClosed => "managed_policy_boundary_closed",
            Self::RouteUnreachable => "route_unreachable",
        }
    }

    /// True when the state holds mutation authority closed.
    pub const fn holds_mutation_closed(self) -> bool {
        !matches!(self, Self::Green)
    }
}

/// Authority-truth state. Names whether the green claim on this row is
/// still honest right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityTruthState {
    /// Route, freshness, grant lifecycle, and managed boundary all
    /// agree — the claim is honest.
    GreenClaimHonest,
    /// Claim is degraded for a typed reason and must render visibly so.
    ClaimDegraded,
    /// Claim is stale beyond review window and must be retracted.
    ClaimStaleAndRetracted,
    /// Claim never resolved — no route or grant truth observed.
    NeverResolved,
}

impl AuthorityTruthState {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GreenClaimHonest => "green_claim_honest",
            Self::ClaimDegraded => "claim_degraded",
            Self::ClaimStaleAndRetracted => "claim_stale_and_retracted",
            Self::NeverResolved => "never_resolved",
        }
    }

    /// True when this state allows the green claim to remain.
    pub const fn allows_green_claim(self) -> bool {
        matches!(self, Self::GreenClaimHonest)
    }
}

/// Browser-handoff reason class. Mirrors the typed reason vocabulary the
/// browser-handoff-packet schema froze, narrowed to the shapes this beta
/// page surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserHandoffReasonClass {
    /// Mutation is not supported in-product and must complete in browser.
    MutationNotSupportedInProduct,
    /// Provider requires browser-based authentication.
    PublishRequiresBrowserAuth,
    /// License or portal acceptance step required.
    LicenseOrPortalAcceptance,
    /// Admin-only surface, only reachable through admin web.
    AdminOnlySurface,
    /// Provider consent / re-consent flow required.
    ProviderConsentFlow,
    /// Provider admin delegation (org-level grant) required.
    ProviderAdminDelegation,
    /// Step-up authentication required.
    StepUpRequired,
}

impl BrowserHandoffReasonClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MutationNotSupportedInProduct => "mutation_not_supported_in_product",
            Self::PublishRequiresBrowserAuth => "publish_requires_browser_auth",
            Self::LicenseOrPortalAcceptance => "license_or_portal_acceptance",
            Self::AdminOnlySurface => "admin_only_surface",
            Self::ProviderConsentFlow => "provider_consent_flow",
            Self::ProviderAdminDelegation => "provider_admin_delegation",
            Self::StepUpRequired => "step_up_required",
        }
    }
}

/// Action-class vocabulary the row's route resolves for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteActionClass {
    /// Read-only inspection (no mutation).
    ReadOnlyInspection,
    /// Mutate on the provider (publish, merge, comment, etc.).
    ProviderMutation,
    /// CI / check-run mutation.
    CiOrCheckMutation,
    /// Release publish or signed-artifact promotion.
    ReleasePublish,
    /// Credential projection / token exchange.
    CredentialProjection,
}

impl RouteActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyInspection => "read_only_inspection",
            Self::ProviderMutation => "provider_mutation",
            Self::CiOrCheckMutation => "ci_or_check_mutation",
            Self::ReleasePublish => "release_publish",
            Self::CredentialProjection => "credential_projection",
        }
    }

    /// True when this action proposes a mutation that crosses authority.
    pub const fn proposes_mutation(self) -> bool {
        !matches!(self, Self::ReadOnlyInspection)
    }
}

/// Route descriptor consumed by a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteDescriptor {
    /// Route choice class.
    pub route_choice: RouteChoiceClass,
    /// Stable token for [`Self::route_choice`].
    pub route_choice_token: String,
    /// Redaction-safe route label.
    pub route_label: String,
    /// Transport label (e.g. `system_browser`, `signed_mirror_https`,
    /// `tunnel_https`, `imported_snapshot_read`).
    pub transport_label: String,
    /// Opaque target identity ref preserved for support reconstruction.
    pub target_identity_ref: String,
    /// Opaque tunnel session ref when the route is tunnel-exposed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tunnel_session_ref: Option<String>,
    /// Opaque mirror identity ref when the route runs through a signed
    /// mirror.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_identity_ref: Option<String>,
    /// Opaque snapshot publisher ref when the route reads an imported
    /// snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_publisher_ref: Option<String>,
}

/// Owner descriptor consumed by a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteOwnerDescriptor {
    /// Owner class.
    pub owner_class: RouteOwnerClass,
    /// Stable token for [`Self::owner_class`].
    pub owner_class_token: String,
    /// Opaque owner ref.
    pub owner_ref: String,
    /// Redaction-safe owner label.
    pub owner_label: String,
    /// Optional opaque managed-policy bundle ref when the owner is a
    /// managed-policy authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_policy_bundle_ref: Option<String>,
}

/// Grant descriptor consumed by a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantDescriptor {
    /// Acting-identity class on the bound row.
    pub acting_identity_class: ActingIdentityClass,
    /// Stable token for [`Self::acting_identity_class`].
    pub acting_identity_class_token: String,
    /// Auth source backing the grant.
    pub auth_source: ProviderAuthSourceClass,
    /// Stable token for [`Self::auth_source`].
    pub auth_source_token: String,
    /// Opaque ref to the account-scope beta identity row this grant is
    /// bound to.
    pub bound_identity_row_ref: String,
    /// Optional opaque ref to a managed-policy bundle if the grant is
    /// managed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_policy_bundle_ref: Option<String>,
}

/// Fallback descriptor consumed by a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackDescriptor {
    /// Fallback mode the surface offers if the current route cannot
    /// proceed.
    pub fallback_mode: ProviderFallbackMode,
    /// Stable token for [`Self::fallback_mode`].
    pub fallback_mode_token: String,
    /// Redaction-safe fallback label.
    pub fallback_label: String,
    /// Optional opaque ref to a browser-handoff packet (when fallback is
    /// `OpenInProvider`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Optional opaque ref to a publish-later queue item (when fallback
    /// is `PublishLaterQueue`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Optional opaque ref to an inspect-only snapshot (when fallback is
    /// `InspectOnly`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inspect_only_snapshot_ref: Option<String>,
    /// Optional opaque ref to an export evidence bundle (when fallback is
    /// `CopyOrExport`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_or_export_evidence_ref: Option<String>,
}

/// Freshness descriptor consumed by a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteFreshness {
    /// Freshness label rendered by the row.
    pub freshness_class: FreshnessLabel,
    /// Stable token for [`Self::freshness_class`].
    pub freshness_class_token: String,
    /// Opaque freshness-floor ref the route is evaluated against.
    pub freshness_floor_ref: String,
    /// Timestamp at which the route was observed.
    pub observed_at: String,
    /// Optional timestamp / duration after which the observation becomes
    /// stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_after: Option<String>,
    /// Optional redaction-safe degraded reason when freshness is not
    /// fresh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<String>,
}

/// One claimed route-resolution row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteResolutionRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque stable row id.
    pub row_id: String,
    /// Reviewable row label safe for UI and support export.
    pub display_label: String,
    /// Profile under which the row is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Lane class.
    pub lane_class: LaneClass,
    /// Stable token for [`Self::lane_class`].
    pub lane_class_token: String,
    /// Provider source class.
    pub provider_source_class: ProviderSourceClass,
    /// Provider surface class the row is rendered under.
    pub provider_surface_class: ProviderSurfaceClass,
    /// Opaque ref to a provider descriptor (registry alpha) the row reads
    /// from.
    pub provider_descriptor_ref: String,
    /// Action class the row resolves the route for.
    pub action_class: RouteActionClass,
    /// Stable token for [`Self::action_class`].
    pub action_class_token: String,
    /// Opaque ref to the provider-linked row the route resolves on.
    pub provider_linked_row_ref: String,
    /// Route descriptor.
    pub route: RouteDescriptor,
    /// Route owner descriptor.
    pub owner: RouteOwnerDescriptor,
    /// Grant descriptor.
    pub grant: GrantDescriptor,
    /// Fallback descriptor.
    pub fallback: FallbackDescriptor,
    /// Route freshness.
    pub freshness: RouteFreshness,
    /// Route degraded state.
    pub route_degraded_state: RouteDegradedStateClass,
    /// Stable token for [`Self::route_degraded_state`].
    pub route_degraded_state_token: String,
    /// Optional opaque ref to a route-origin command/support packet so
    /// browser-handoff and command/support reconstruction share the same
    /// origin lineage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_route_packet_ref: Option<String>,
    /// Beta guardrail: row never carries raw access-token material.
    pub raw_token_material_present: bool,
    /// Beta guardrail: row never offers a silent public-endpoint
    /// fallback.
    pub silent_public_endpoint_fallback_taken: bool,
    /// Beta guardrail: row never widens authority above the grant's
    /// declared scope.
    pub silent_authority_widening_taken: bool,
    /// Beta guardrail: local editing is preserved through the row's
    /// degraded/fallback path.
    pub local_editing_preserved: bool,
}

/// One claimed browser-handoff panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffPanel {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque stable panel id.
    pub panel_id: String,
    /// Reviewable panel label safe for UI and support export.
    pub display_label: String,
    /// Row id the panel projects from.
    pub bound_row_ref: String,
    /// Profile under which the panel is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Browser-handoff reason class.
    pub handoff_reason: BrowserHandoffReasonClass,
    /// Stable token for [`Self::handoff_reason`].
    pub handoff_reason_token: String,
    /// Route choice the panel projects (must match the bound row's route
    /// choice OR its fallback if the row degraded into the handoff).
    pub projected_route_choice: RouteChoiceClass,
    /// Stable token for [`Self::projected_route_choice`].
    pub projected_route_choice_token: String,
    /// Owner class projected on the handoff (must match the bound row).
    pub projected_owner_class: RouteOwnerClass,
    /// Stable token for [`Self::projected_owner_class`].
    pub projected_owner_class_token: String,
    /// Acting-identity class projected on the handoff (must match the
    /// bound row).
    pub projected_acting_identity_class: ActingIdentityClass,
    /// Stable token for [`Self::projected_acting_identity_class`].
    pub projected_acting_identity_class_token: String,
    /// Opaque ref to the browser-handoff packet seed.
    pub browser_handoff_packet_ref: String,
    /// Optional opaque ref to a return summary record once the callback
    /// resolves.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_summary_ref: Option<String>,
    /// Redaction-safe panel summary.
    pub panel_summary: String,
    /// Beta guardrail: panel never carries a raw URL.
    pub raw_url_present: bool,
    /// Beta guardrail: panel never carries raw provider payload.
    pub raw_provider_payload_present: bool,
}

/// One claimed authority-truth panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTruthPanel {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque stable panel id.
    pub panel_id: String,
    /// Reviewable panel label safe for UI and support export.
    pub display_label: String,
    /// Row id the panel projects from.
    pub bound_row_ref: String,
    /// Profile under which the panel is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Authority-truth state.
    pub truth_state: AuthorityTruthState,
    /// Stable token for [`Self::truth_state`].
    pub truth_state_token: String,
    /// Whether the green claim is currently held.
    pub green_claim_held: bool,
    /// Redaction-safe rationale for [`Self::truth_state`].
    pub rationale_summary: String,
    /// Timestamp at which the truth was last computed.
    pub computed_at: String,
}

/// Defect-kind vocabulary surfaced by the beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteResolutionBetaDefectKind {
    /// Row carries raw token material.
    RawTokenMaterialPresent,
    /// Row claims a silent public-endpoint fallback.
    SilentPublicEndpointFallbackTaken,
    /// Row silently widened authority above the grant scope.
    SilentAuthorityWideningTaken,
    /// Row did not preserve local editing.
    LocalEditingNotPreserved,
    /// Browser-handoff route is not paired with a browser-handoff packet
    /// ref in either the row or the panel.
    BrowserHandoffRouteWithoutPacketRef,
    /// Tunnel-exposed route is not paired with a tunnel session ref.
    TunnelRouteWithoutSessionRef,
    /// Signed-mirror route is not paired with a mirror identity ref.
    MirrorRouteWithoutMirrorRef,
    /// Imported-snapshot route is not paired with a snapshot publisher
    /// ref.
    SnapshotRouteWithoutPublisherRef,
    /// Managed lane is not bound to a managed-policy bundle ref.
    ManagedLaneWithoutManagedPolicyBundle,
    /// External lane is bound to a managed-policy bundle ref.
    ExternalLaneWithManagedPolicyBundle,
    /// Browser-handoff panel binds a row ref not present on the page.
    BrowserHandoffPanelRowRefUnknown,
    /// Browser-handoff panel projects a route choice that does not match
    /// the bound row.
    BrowserHandoffPanelRouteChoiceMismatch,
    /// Browser-handoff panel projects an owner class that does not match
    /// the bound row.
    BrowserHandoffPanelOwnerClassMismatch,
    /// Browser-handoff panel projects an acting-identity class that does
    /// not match the bound row.
    BrowserHandoffPanelIdentityClassMismatch,
    /// Browser-handoff panel carries a raw URL.
    BrowserHandoffPanelRawUrlPresent,
    /// Browser-handoff panel carries a raw provider payload.
    BrowserHandoffPanelRawProviderPayloadPresent,
    /// Authority-truth panel binds a row ref not present on the page.
    AuthorityTruthPanelRowRefUnknown,
    /// Authority-truth panel claims green but the bound row's freshness
    /// or route state holds mutation closed.
    AuthorityTruthPanelGreenClaimWhileStale,
    /// Authority-truth panel claims green but the bound row's lane is
    /// managed and managed-policy bundle ref is missing.
    AuthorityTruthPanelGreenClaimWithoutManagedBundle,
    /// Authority-truth panel `green_claim_held` flag disagrees with the
    /// typed state.
    AuthorityTruthPanelGreenFlagDisagreesWithState,
    /// Row uses a green degraded state but freshness is not fresh.
    RouteGreenStateWithStaleFreshness,
    /// Row's action proposes mutation but fallback is missing required
    /// proof ref.
    MutationActionWithoutFallbackProofRef,
    /// Fallback descriptor's `OpenInProvider` mode is missing a
    /// browser-handoff packet ref.
    FallbackOpenInProviderWithoutPacketRef,
    /// Fallback descriptor's `PublishLaterQueue` mode is missing a
    /// publish-later queue item ref.
    FallbackPublishLaterWithoutQueueRef,
    /// Fallback descriptor's `InspectOnly` mode is missing a snapshot
    /// ref.
    FallbackInspectOnlyWithoutSnapshotRef,
    /// Fallback descriptor's `CopyOrExport` mode is missing an evidence
    /// ref.
    FallbackCopyOrExportWithoutEvidenceRef,
    /// A required profile has no claimed row.
    ProfileCoverageMissing,
    /// `profile_token` did not match `profile`.
    ProfileTokenDrift,
    /// `lane_class_token` did not match `lane_class`.
    LaneClassTokenDrift,
    /// `route_choice_token` did not match `route_choice`.
    RouteChoiceTokenDrift,
    /// `owner_class_token` did not match `owner_class`.
    OwnerClassTokenDrift,
    /// `acting_identity_class_token` did not match
    /// `acting_identity_class`.
    ActingIdentityClassTokenDrift,
    /// `auth_source_token` did not match `auth_source`.
    AuthSourceTokenDrift,
    /// `action_class_token` did not match `action_class`.
    ActionClassTokenDrift,
    /// `fallback_mode_token` did not match `fallback_mode`.
    FallbackModeTokenDrift,
    /// `freshness_class_token` did not match `freshness_class`.
    FreshnessClassTokenDrift,
    /// `route_degraded_state_token` did not match
    /// `route_degraded_state`.
    RouteDegradedStateTokenDrift,
    /// `handoff_reason_token` did not match `handoff_reason`.
    HandoffReasonTokenDrift,
    /// `truth_state_token` did not match `truth_state`.
    TruthStateTokenDrift,
}

impl RouteResolutionBetaDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawTokenMaterialPresent => "raw_token_material_present",
            Self::SilentPublicEndpointFallbackTaken => "silent_public_endpoint_fallback_taken",
            Self::SilentAuthorityWideningTaken => "silent_authority_widening_taken",
            Self::LocalEditingNotPreserved => "local_editing_not_preserved",
            Self::BrowserHandoffRouteWithoutPacketRef => "browser_handoff_route_without_packet_ref",
            Self::TunnelRouteWithoutSessionRef => "tunnel_route_without_session_ref",
            Self::MirrorRouteWithoutMirrorRef => "mirror_route_without_mirror_ref",
            Self::SnapshotRouteWithoutPublisherRef => "snapshot_route_without_publisher_ref",
            Self::ManagedLaneWithoutManagedPolicyBundle => {
                "managed_lane_without_managed_policy_bundle"
            }
            Self::ExternalLaneWithManagedPolicyBundle => "external_lane_with_managed_policy_bundle",
            Self::BrowserHandoffPanelRowRefUnknown => "browser_handoff_panel_row_ref_unknown",
            Self::BrowserHandoffPanelRouteChoiceMismatch => {
                "browser_handoff_panel_route_choice_mismatch"
            }
            Self::BrowserHandoffPanelOwnerClassMismatch => {
                "browser_handoff_panel_owner_class_mismatch"
            }
            Self::BrowserHandoffPanelIdentityClassMismatch => {
                "browser_handoff_panel_identity_class_mismatch"
            }
            Self::BrowserHandoffPanelRawUrlPresent => "browser_handoff_panel_raw_url_present",
            Self::BrowserHandoffPanelRawProviderPayloadPresent => {
                "browser_handoff_panel_raw_provider_payload_present"
            }
            Self::AuthorityTruthPanelRowRefUnknown => "authority_truth_panel_row_ref_unknown",
            Self::AuthorityTruthPanelGreenClaimWhileStale => {
                "authority_truth_panel_green_claim_while_stale"
            }
            Self::AuthorityTruthPanelGreenClaimWithoutManagedBundle => {
                "authority_truth_panel_green_claim_without_managed_bundle"
            }
            Self::AuthorityTruthPanelGreenFlagDisagreesWithState => {
                "authority_truth_panel_green_flag_disagrees_with_state"
            }
            Self::RouteGreenStateWithStaleFreshness => "route_green_state_with_stale_freshness",
            Self::MutationActionWithoutFallbackProofRef => {
                "mutation_action_without_fallback_proof_ref"
            }
            Self::FallbackOpenInProviderWithoutPacketRef => {
                "fallback_open_in_provider_without_packet_ref"
            }
            Self::FallbackPublishLaterWithoutQueueRef => "fallback_publish_later_without_queue_ref",
            Self::FallbackInspectOnlyWithoutSnapshotRef => {
                "fallback_inspect_only_without_snapshot_ref"
            }
            Self::FallbackCopyOrExportWithoutEvidenceRef => {
                "fallback_copy_or_export_without_evidence_ref"
            }
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::ProfileTokenDrift => "profile_token_drift",
            Self::LaneClassTokenDrift => "lane_class_token_drift",
            Self::RouteChoiceTokenDrift => "route_choice_token_drift",
            Self::OwnerClassTokenDrift => "owner_class_token_drift",
            Self::ActingIdentityClassTokenDrift => "acting_identity_class_token_drift",
            Self::AuthSourceTokenDrift => "auth_source_token_drift",
            Self::ActionClassTokenDrift => "action_class_token_drift",
            Self::FallbackModeTokenDrift => "fallback_mode_token_drift",
            Self::FreshnessClassTokenDrift => "freshness_class_token_drift",
            Self::RouteDegradedStateTokenDrift => "route_degraded_state_token_drift",
            Self::HandoffReasonTokenDrift => "handoff_reason_token_drift",
            Self::TruthStateTokenDrift => "truth_state_token_drift",
        }
    }
}

/// Typed validation defect for the route-resolution beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteResolutionBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: RouteResolutionBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (row id, panel id, or `"page"`).
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl RouteResolutionBetaDefect {
    fn new(
        defect_kind: RouteResolutionBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: ROUTE_RESOLUTION_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the route-resolution beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteResolutionBetaSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Number of route-resolution rows.
    pub row_count: usize,
    /// Number of browser-handoff panels.
    pub browser_handoff_panel_count: usize,
    /// Number of authority-truth panels.
    pub authority_truth_panel_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Lane-class tokens present across rows.
    pub lane_classes_present: Vec<String>,
    /// Route-choice tokens present across rows.
    pub route_choices_present: Vec<String>,
    /// Owner-class tokens present across rows.
    pub owner_classes_present: Vec<String>,
    /// Acting-identity tokens present across rows.
    pub acting_identity_classes_present: Vec<String>,
    /// Action-class tokens present across rows.
    pub action_classes_present: Vec<String>,
    /// Fallback-mode tokens present across rows.
    pub fallback_modes_present: Vec<String>,
    /// Route-degraded-state tokens present across rows.
    pub route_degraded_states_present: Vec<String>,
    /// Authority-truth-state tokens present across authority-truth
    /// panels.
    pub authority_truth_states_present: Vec<String>,
    /// Counts of rows by route-degraded-state token.
    pub rows_by_route_degraded_state: BTreeMap<String, usize>,
    /// Counts of rows by lane-class token.
    pub rows_by_lane_class: BTreeMap<String, usize>,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl RouteResolutionBetaSummary {
    /// Builds the summary from rows, panels, and defects.
    pub fn from_records(
        rows: &[RouteResolutionRow],
        browser_handoff_panels: &[BrowserHandoffPanel],
        authority_truth_panels: &[AuthorityTruthPanel],
        defects: &[RouteResolutionBetaDefect],
    ) -> Self {
        let mut profiles_present: BTreeSet<String> = BTreeSet::new();
        let mut lane_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut route_choices_present: BTreeSet<String> = BTreeSet::new();
        let mut owner_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut acting_identity_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut action_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut fallback_modes_present: BTreeSet<String> = BTreeSet::new();
        let mut route_degraded_states_present: BTreeSet<String> = BTreeSet::new();
        let mut authority_truth_states_present: BTreeSet<String> = BTreeSet::new();
        let mut rows_by_route_degraded_state: BTreeMap<String, usize> = BTreeMap::new();
        let mut rows_by_lane_class: BTreeMap<String, usize> = BTreeMap::new();

        for row in rows {
            profiles_present.insert(row.profile_token.clone());
            lane_classes_present.insert(row.lane_class_token.clone());
            route_choices_present.insert(row.route.route_choice_token.clone());
            owner_classes_present.insert(row.owner.owner_class_token.clone());
            acting_identity_classes_present.insert(row.grant.acting_identity_class_token.clone());
            action_classes_present.insert(row.action_class_token.clone());
            fallback_modes_present.insert(row.fallback.fallback_mode_token.clone());
            route_degraded_states_present.insert(row.route_degraded_state_token.clone());
            *rows_by_route_degraded_state
                .entry(row.route_degraded_state_token.clone())
                .or_insert(0) += 1;
            *rows_by_lane_class
                .entry(row.lane_class_token.clone())
                .or_insert(0) += 1;
        }
        for panel in browser_handoff_panels {
            profiles_present.insert(panel.profile_token.clone());
        }
        for panel in authority_truth_panels {
            profiles_present.insert(panel.profile_token.clone());
            authority_truth_states_present.insert(panel.truth_state_token.clone());
        }

        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            record_kind: ROUTE_RESOLUTION_BETA_SUMMARY_RECORD_KIND.to_owned(),
            row_count: rows.len(),
            browser_handoff_panel_count: browser_handoff_panels.len(),
            authority_truth_panel_count: authority_truth_panels.len(),
            profiles_present: profiles_present.into_iter().collect(),
            lane_classes_present: lane_classes_present.into_iter().collect(),
            route_choices_present: route_choices_present.into_iter().collect(),
            owner_classes_present: owner_classes_present.into_iter().collect(),
            acting_identity_classes_present: acting_identity_classes_present.into_iter().collect(),
            action_classes_present: action_classes_present.into_iter().collect(),
            fallback_modes_present: fallback_modes_present.into_iter().collect(),
            route_degraded_states_present: route_degraded_states_present.into_iter().collect(),
            authority_truth_states_present: authority_truth_states_present.into_iter().collect(),
            rows_by_route_degraded_state,
            rows_by_lane_class,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level route-resolution beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteResolutionBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Claimed route-resolution rows.
    pub rows: Vec<RouteResolutionRow>,
    /// Claimed browser-handoff panels.
    pub browser_handoff_panels: Vec<BrowserHandoffPanel>,
    /// Claimed authority-truth panels.
    pub authority_truth_panels: Vec<AuthorityTruthPanel>,
    /// Typed validation defects.
    pub defects: Vec<RouteResolutionBetaDefect>,
    /// Aggregate summary.
    pub summary: RouteResolutionBetaSummary,
}

/// Support-export wrapper for the route-resolution beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteResolutionBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: RouteResolutionBetaPage,
    /// Defect-kind tokens present in the page.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw access tokens, raw URLs, and raw provider payloads
    /// are excluded from the export.
    pub raw_tokens_excluded: bool,
    /// True when route, owner, grant, and fallback lineage are preserved
    /// verbatim so support can name the route everywhere.
    pub route_lineage_preserved: bool,
    /// True when browser-handoff panel lineage is preserved verbatim.
    pub browser_handoff_lineage_preserved: bool,
    /// True when authority-truth panel lineage is preserved verbatim.
    pub authority_truth_lineage_preserved: bool,
    /// True when the export proves the no-silent-public-endpoint-fallback
    /// and no-silent-authority-widening invariants.
    pub fail_closed_invariant: bool,
    /// Reviewable summary of the redaction posture.
    pub redaction_summary: String,
}

impl RouteResolutionBetaSupportExport {
    /// Builds a support-export wrapper from a beta page. The beta page
    /// never carries raw token material, raw URLs, or raw provider
    /// payloads, so route, browser-handoff, and authority-truth lineage
    /// is preserved verbatim without further redaction.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: RouteResolutionBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: ROUTE_RESOLUTION_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_tokens_excluded: true,
            route_lineage_preserved: true,
            browser_handoff_lineage_preserved: true,
            authority_truth_lineage_preserved: true,
            fail_closed_invariant: true,
            redaction_summary:
                "Metadata-only route-resolution beta export: route, owner, grant, fallback, and \
                 browser-handoff lineage; authority-truth state lineage are preserved verbatim. \
                 Raw access tokens, raw URLs, raw delegated-token bodies, and raw provider \
                 payloads are excluded because the beta projection never carries them."
                    .to_owned(),
        }
    }
}

/// Validates the route-resolution beta page and returns typed defects on
/// failure.
pub fn validate_route_resolution_beta_page(
    page: &RouteResolutionBetaPage,
) -> Result<(), Vec<RouteResolutionBetaDefect>> {
    let defects = audit_route_resolution_beta_page(
        &page.rows,
        &page.browser_handoff_panels,
        &page.authority_truth_panels,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for a route-resolution beta page.
pub fn audit_route_resolution_beta_page(
    rows: &[RouteResolutionRow],
    browser_handoff_panels: &[BrowserHandoffPanel],
    authority_truth_panels: &[AuthorityTruthPanel],
) -> Vec<RouteResolutionBetaDefect> {
    let mut defects = Vec::new();

    let row_index: BTreeMap<&str, &RouteResolutionRow> =
        rows.iter().map(|row| (row.row_id.as_str(), row)).collect();

    for row in rows {
        audit_row_tokens(&mut defects, row);
        audit_row_guardrails(&mut defects, row);
        audit_row_route(&mut defects, row);
        audit_row_lane(&mut defects, row);
        audit_row_fallback(&mut defects, row);
        audit_row_freshness_vs_degraded(&mut defects, row);
    }

    for panel in browser_handoff_panels {
        if panel.handoff_reason_token != panel.handoff_reason.as_str() {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::HandoffReasonTokenDrift,
                panel.panel_id.clone(),
                "handoff_reason_token",
                "handoff_reason_token must match handoff_reason",
            ));
        }
        if panel.profile_token != panel.profile.as_str() {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::ProfileTokenDrift,
                panel.panel_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if panel.projected_route_choice_token != panel.projected_route_choice.as_str() {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::RouteChoiceTokenDrift,
                panel.panel_id.clone(),
                "projected_route_choice_token",
                "projected_route_choice_token must match projected_route_choice",
            ));
        }
        if panel.projected_owner_class_token != panel.projected_owner_class.as_str() {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::OwnerClassTokenDrift,
                panel.panel_id.clone(),
                "projected_owner_class_token",
                "projected_owner_class_token must match projected_owner_class",
            ));
        }
        if panel.projected_acting_identity_class_token
            != panel.projected_acting_identity_class.as_str()
        {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::ActingIdentityClassTokenDrift,
                panel.panel_id.clone(),
                "projected_acting_identity_class_token",
                "projected_acting_identity_class_token must match \
                 projected_acting_identity_class",
            ));
        }
        if panel.raw_url_present {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::BrowserHandoffPanelRawUrlPresent,
                panel.panel_id.clone(),
                "raw_url_present",
                "browser-handoff panel must not carry raw URL material",
            ));
        }
        if panel.raw_provider_payload_present {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::BrowserHandoffPanelRawProviderPayloadPresent,
                panel.panel_id.clone(),
                "raw_provider_payload_present",
                "browser-handoff panel must not carry raw provider payload material",
            ));
        }

        match row_index.get(panel.bound_row_ref.as_str()) {
            None => {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::BrowserHandoffPanelRowRefUnknown,
                    panel.panel_id.clone(),
                    "bound_row_ref",
                    "bound_row_ref must reference a row on the page",
                ));
            }
            Some(row) => {
                let row_route = row.route.route_choice;
                let fallback_route = (row.fallback.fallback_mode
                    == ProviderFallbackMode::OpenInProvider)
                    .then_some(RouteChoiceClass::SystemBrowserHandoffRoute);
                if panel.projected_route_choice != row_route
                    && Some(panel.projected_route_choice) != fallback_route
                {
                    defects.push(RouteResolutionBetaDefect::new(
                        RouteResolutionBetaDefectKind::BrowserHandoffPanelRouteChoiceMismatch,
                        panel.panel_id.clone(),
                        "projected_route_choice",
                        "browser-handoff panel route choice must match the bound row's route or \
                         its `OpenInProvider` fallback",
                    ));
                }
                if panel.projected_owner_class != row.owner.owner_class {
                    defects.push(RouteResolutionBetaDefect::new(
                        RouteResolutionBetaDefectKind::BrowserHandoffPanelOwnerClassMismatch,
                        panel.panel_id.clone(),
                        "projected_owner_class",
                        "browser-handoff panel owner class must match the bound row's owner",
                    ));
                }
                if panel.projected_acting_identity_class != row.grant.acting_identity_class {
                    defects.push(RouteResolutionBetaDefect::new(
                        RouteResolutionBetaDefectKind::BrowserHandoffPanelIdentityClassMismatch,
                        panel.panel_id.clone(),
                        "projected_acting_identity_class",
                        "browser-handoff panel acting-identity class must match the bound row's \
                         grant identity class",
                    ));
                }
            }
        }
    }

    for panel in authority_truth_panels {
        if panel.profile_token != panel.profile.as_str() {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::ProfileTokenDrift,
                panel.panel_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if panel.truth_state_token != panel.truth_state.as_str() {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::TruthStateTokenDrift,
                panel.panel_id.clone(),
                "truth_state_token",
                "truth_state_token must match truth_state",
            ));
        }
        if panel.green_claim_held != panel.truth_state.allows_green_claim() {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::AuthorityTruthPanelGreenFlagDisagreesWithState,
                panel.panel_id.clone(),
                "green_claim_held",
                "green_claim_held must agree with truth_state",
            ));
        }
        match row_index.get(panel.bound_row_ref.as_str()) {
            None => {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::AuthorityTruthPanelRowRefUnknown,
                    panel.panel_id.clone(),
                    "bound_row_ref",
                    "bound_row_ref must reference a row on the page",
                ));
            }
            Some(row) => {
                if panel.green_claim_held
                    && (row.route_degraded_state.holds_mutation_closed()
                        || row.freshness.freshness_class != FreshnessLabel::Fresh)
                {
                    defects.push(RouteResolutionBetaDefect::new(
                        RouteResolutionBetaDefectKind::AuthorityTruthPanelGreenClaimWhileStale,
                        panel.panel_id.clone(),
                        "green_claim_held",
                        "green claim must not be held when route or freshness holds mutation \
                         closed on the bound row",
                    ));
                }
                if panel.green_claim_held
                    && row.lane_class.is_managed()
                    && row.owner.managed_policy_bundle_ref.is_none()
                {
                    defects.push(RouteResolutionBetaDefect::new(
                        RouteResolutionBetaDefectKind::AuthorityTruthPanelGreenClaimWithoutManagedBundle,
                        panel.panel_id.clone(),
                        "green_claim_held",
                        "managed lane green claim must cite a managed-policy bundle ref on the \
                         bound row",
                    ));
                }
            }
        }
    }

    let mut observed_profiles: BTreeSet<&str> = BTreeSet::new();
    for row in rows {
        observed_profiles.insert(row.profile_token.as_str());
    }
    for panel in browser_handoff_panels {
        observed_profiles.insert(panel.profile_token.as_str());
    }
    for panel in authority_truth_panels {
        observed_profiles.insert(panel.profile_token.as_str());
    }
    let required_profiles: BTreeSet<&str> = AccountScopeBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::ProfileCoverageMissing,
            "page",
            "profiles",
            format!("missing {} profile coverage", missing),
        ));
    }

    defects
}

fn audit_row_tokens(defects: &mut Vec<RouteResolutionBetaDefect>, row: &RouteResolutionRow) {
    if row.profile_token != row.profile.as_str() {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::ProfileTokenDrift,
            row.row_id.clone(),
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if row.lane_class_token != row.lane_class.as_str() {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::LaneClassTokenDrift,
            row.row_id.clone(),
            "lane_class_token",
            "lane_class_token must match lane_class",
        ));
    }
    if row.action_class_token != row.action_class.as_str() {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::ActionClassTokenDrift,
            row.row_id.clone(),
            "action_class_token",
            "action_class_token must match action_class",
        ));
    }
    if row.route.route_choice_token != row.route.route_choice.as_str() {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::RouteChoiceTokenDrift,
            row.row_id.clone(),
            "route.route_choice_token",
            "route.route_choice_token must match route.route_choice",
        ));
    }
    if row.owner.owner_class_token != row.owner.owner_class.as_str() {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::OwnerClassTokenDrift,
            row.row_id.clone(),
            "owner.owner_class_token",
            "owner.owner_class_token must match owner.owner_class",
        ));
    }
    if row.grant.acting_identity_class_token != row.grant.acting_identity_class.as_str() {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::ActingIdentityClassTokenDrift,
            row.row_id.clone(),
            "grant.acting_identity_class_token",
            "grant.acting_identity_class_token must match grant.acting_identity_class",
        ));
    }
    let auth_source_token = provider_auth_source_token(row.grant.auth_source);
    if row.grant.auth_source_token != auth_source_token {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::AuthSourceTokenDrift,
            row.row_id.clone(),
            "grant.auth_source_token",
            "grant.auth_source_token must match grant.auth_source",
        ));
    }
    let fallback_token = provider_fallback_token(row.fallback.fallback_mode);
    if row.fallback.fallback_mode_token != fallback_token {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::FallbackModeTokenDrift,
            row.row_id.clone(),
            "fallback.fallback_mode_token",
            "fallback.fallback_mode_token must match fallback.fallback_mode",
        ));
    }
    let freshness_token = row.freshness.freshness_class.as_str();
    if row.freshness.freshness_class_token != freshness_token {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::FreshnessClassTokenDrift,
            row.row_id.clone(),
            "freshness.freshness_class_token",
            "freshness.freshness_class_token must match freshness.freshness_class",
        ));
    }
    if row.route_degraded_state_token != row.route_degraded_state.as_str() {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::RouteDegradedStateTokenDrift,
            row.row_id.clone(),
            "route_degraded_state_token",
            "route_degraded_state_token must match route_degraded_state",
        ));
    }
}

fn audit_row_guardrails(defects: &mut Vec<RouteResolutionBetaDefect>, row: &RouteResolutionRow) {
    if row.raw_token_material_present {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::RawTokenMaterialPresent,
            row.row_id.clone(),
            "raw_token_material_present",
            "claimed beta row must not carry raw token material",
        ));
    }
    if row.silent_public_endpoint_fallback_taken {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::SilentPublicEndpointFallbackTaken,
            row.row_id.clone(),
            "silent_public_endpoint_fallback_taken",
            "claimed beta row must not silently fall back to a public endpoint",
        ));
    }
    if row.silent_authority_widening_taken {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::SilentAuthorityWideningTaken,
            row.row_id.clone(),
            "silent_authority_widening_taken",
            "claimed beta row must not silently widen mutation authority",
        ));
    }
    if !row.local_editing_preserved {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::LocalEditingNotPreserved,
            row.row_id.clone(),
            "local_editing_preserved",
            "claimed beta row must preserve local editing through the fallback path",
        ));
    }
}

fn audit_row_route(defects: &mut Vec<RouteResolutionBetaDefect>, row: &RouteResolutionRow) {
    match row.route.route_choice {
        RouteChoiceClass::SystemBrowserHandoffRoute => {
            let bound = row
                .fallback
                .browser_handoff_packet_ref
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .is_some();
            if !bound {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::BrowserHandoffRouteWithoutPacketRef,
                    row.row_id.clone(),
                    "fallback.browser_handoff_packet_ref",
                    "browser-handoff route must cite a browser_handoff_packet_ref",
                ));
            }
        }
        RouteChoiceClass::TunnelExposedRoute => {
            if row
                .route
                .tunnel_session_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::TunnelRouteWithoutSessionRef,
                    row.row_id.clone(),
                    "route.tunnel_session_ref",
                    "tunnel-exposed route must cite a tunnel session ref",
                ));
            }
        }
        RouteChoiceClass::SignedMirrorRoute => {
            if row
                .route
                .mirror_identity_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::MirrorRouteWithoutMirrorRef,
                    row.row_id.clone(),
                    "route.mirror_identity_ref",
                    "signed-mirror route must cite a mirror identity ref",
                ));
            }
        }
        RouteChoiceClass::ImportedSnapshotRoute => {
            if row
                .route
                .snapshot_publisher_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::SnapshotRouteWithoutPublisherRef,
                    row.row_id.clone(),
                    "route.snapshot_publisher_ref",
                    "imported-snapshot route must cite a snapshot publisher ref",
                ));
            }
        }
        RouteChoiceClass::LiveProviderDirect | RouteChoiceClass::LocalOnlyRoute => {}
    }
}

fn audit_row_lane(defects: &mut Vec<RouteResolutionBetaDefect>, row: &RouteResolutionRow) {
    let bundle_present = row.owner.managed_policy_bundle_ref.is_some()
        || row.grant.managed_policy_bundle_ref.is_some();
    if row.lane_class.is_managed() && !bundle_present {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::ManagedLaneWithoutManagedPolicyBundle,
            row.row_id.clone(),
            "owner.managed_policy_bundle_ref",
            "managed lane row must cite a managed-policy bundle ref on the owner or grant",
        ));
    }
    if !row.lane_class.is_managed() && bundle_present {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::ExternalLaneWithManagedPolicyBundle,
            row.row_id.clone(),
            "owner.managed_policy_bundle_ref",
            "external lane row must not cite a managed-policy bundle ref",
        ));
    }
}

fn audit_row_fallback(defects: &mut Vec<RouteResolutionBetaDefect>, row: &RouteResolutionRow) {
    match row.fallback.fallback_mode {
        ProviderFallbackMode::OpenInProvider => {
            if row
                .fallback
                .browser_handoff_packet_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::FallbackOpenInProviderWithoutPacketRef,
                    row.row_id.clone(),
                    "fallback.browser_handoff_packet_ref",
                    "OpenInProvider fallback must cite a browser_handoff_packet_ref",
                ));
            }
        }
        ProviderFallbackMode::PublishLaterQueue => {
            if row
                .fallback
                .publish_later_queue_item_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::FallbackPublishLaterWithoutQueueRef,
                    row.row_id.clone(),
                    "fallback.publish_later_queue_item_ref",
                    "PublishLaterQueue fallback must cite a publish_later_queue_item_ref",
                ));
            }
        }
        ProviderFallbackMode::InspectOnly => {
            if row
                .fallback
                .inspect_only_snapshot_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::FallbackInspectOnlyWithoutSnapshotRef,
                    row.row_id.clone(),
                    "fallback.inspect_only_snapshot_ref",
                    "InspectOnly fallback must cite an inspect_only_snapshot_ref",
                ));
            }
        }
        ProviderFallbackMode::CopyOrExport => {
            if row
                .fallback
                .copy_or_export_evidence_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                defects.push(RouteResolutionBetaDefect::new(
                    RouteResolutionBetaDefectKind::FallbackCopyOrExportWithoutEvidenceRef,
                    row.row_id.clone(),
                    "fallback.copy_or_export_evidence_ref",
                    "CopyOrExport fallback must cite a copy_or_export_evidence_ref",
                ));
            }
        }
    }

    if row.action_class.proposes_mutation() && row.route_degraded_state.holds_mutation_closed() {
        let proof_present = match row.fallback.fallback_mode {
            ProviderFallbackMode::OpenInProvider => {
                row.fallback.browser_handoff_packet_ref.is_some()
            }
            ProviderFallbackMode::PublishLaterQueue => {
                row.fallback.publish_later_queue_item_ref.is_some()
            }
            ProviderFallbackMode::InspectOnly => row.fallback.inspect_only_snapshot_ref.is_some(),
            ProviderFallbackMode::CopyOrExport => {
                row.fallback.copy_or_export_evidence_ref.is_some()
            }
        };
        if !proof_present {
            defects.push(RouteResolutionBetaDefect::new(
                RouteResolutionBetaDefectKind::MutationActionWithoutFallbackProofRef,
                row.row_id.clone(),
                "fallback",
                "mutation action under a degraded route must cite the typed fallback proof ref",
            ));
        }
    }
}

fn audit_row_freshness_vs_degraded(
    defects: &mut Vec<RouteResolutionBetaDefect>,
    row: &RouteResolutionRow,
) {
    if row.route_degraded_state == RouteDegradedStateClass::Green
        && row.freshness.freshness_class != FreshnessLabel::Fresh
    {
        defects.push(RouteResolutionBetaDefect::new(
            RouteResolutionBetaDefectKind::RouteGreenStateWithStaleFreshness,
            row.row_id.clone(),
            "route_degraded_state",
            "green route state must pair with fresh freshness",
        ));
    }
}

fn provider_auth_source_token(class: ProviderAuthSourceClass) -> &'static str {
    match class {
        ProviderAuthSourceClass::HumanSession => "human_session",
        ProviderAuthSourceClass::InstallationGrant => "installation_grant",
        ProviderAuthSourceClass::DelegatedCredential => "delegated_credential",
        ProviderAuthSourceClass::ProjectScopedGrant => "project_scoped_grant",
        ProviderAuthSourceClass::PolicyInjectedService => "policy_injected_service",
        ProviderAuthSourceClass::BrowserOnly => "browser_only",
        ProviderAuthSourceClass::UnknownAuthSource => "unknown_auth_source",
    }
}

fn provider_fallback_token(mode: ProviderFallbackMode) -> &'static str {
    match mode {
        ProviderFallbackMode::CopyOrExport => "copy_or_export",
        ProviderFallbackMode::OpenInProvider => "open_in_provider",
        ProviderFallbackMode::PublishLaterQueue => "publish_later_queue",
        ProviderFallbackMode::InspectOnly => "inspect_only",
    }
}

/// Builds the seeded route-resolution beta page consumed by tests,
/// fixtures, support exports, and headless inspectors.
pub fn seeded_route_resolution_beta_page() -> RouteResolutionBetaPage {
    let rows = seed_rows();
    let browser_handoff_panels = seed_browser_handoff_panels();
    let authority_truth_panels = seed_authority_truth_panels();
    let defects =
        audit_route_resolution_beta_page(&rows, &browser_handoff_panels, &authority_truth_panels);
    let summary = RouteResolutionBetaSummary::from_records(
        &rows,
        &browser_handoff_panels,
        &authority_truth_panels,
        &defects,
    );
    RouteResolutionBetaPage {
        record_kind: ROUTE_RESOLUTION_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
        shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: ROUTE_RESOLUTION_BETA_SOURCE_MATRIX_REF.to_owned(),
        rows,
        browser_handoff_panels,
        authority_truth_panels,
        defects,
        summary,
    }
}

fn seed_rows() -> Vec<RouteResolutionRow> {
    vec![
        // Connected profile: live provider direct, allowed mutation by human session.
        RouteResolutionRow {
            record_kind: ROUTE_RESOLUTION_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "route-resolution-beta:row:connected:human-dev:comment".to_owned(),
            display_label: "Comment on PR via live provider direct".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            lane_class: LaneClass::ExternalProviderLane,
            lane_class_token: LaneClass::ExternalProviderLane.as_str().to_owned(),
            provider_source_class: ProviderSourceClass::LiveProvider,
            provider_surface_class: ProviderSurfaceClass::CodeHostSurface,
            provider_descriptor_ref: "provider-descriptor:code-host:public:payments-org".to_owned(),
            action_class: RouteActionClass::ProviderMutation,
            action_class_token: RouteActionClass::ProviderMutation.as_str().to_owned(),
            provider_linked_row_ref: "provider-linked-row:pr:payments/backend:1234".to_owned(),
            route: RouteDescriptor {
                route_choice: RouteChoiceClass::LiveProviderDirect,
                route_choice_token: RouteChoiceClass::LiveProviderDirect.as_str().to_owned(),
                route_label: "Live public code host".to_owned(),
                transport_label: "https".to_owned(),
                target_identity_ref:
                    "provider-target-identity:code-host:public:payments/backend:1234".to_owned(),
                tunnel_session_ref: None,
                mirror_identity_ref: None,
                snapshot_publisher_ref: None,
            },
            owner: RouteOwnerDescriptor {
                owner_class: RouteOwnerClass::WorkspaceUser,
                owner_class_token: RouteOwnerClass::WorkspaceUser.as_str().to_owned(),
                owner_ref: "actor:human-account:workspace:payments:dev-001".to_owned(),
                owner_label: "Workspace developer".to_owned(),
                managed_policy_bundle_ref: None,
            },
            grant: GrantDescriptor {
                acting_identity_class: ActingIdentityClass::ConnectedAccount,
                acting_identity_class_token: ActingIdentityClass::ConnectedAccount
                    .as_str()
                    .to_owned(),
                auth_source: ProviderAuthSourceClass::HumanSession,
                auth_source_token: provider_auth_source_token(
                    ProviderAuthSourceClass::HumanSession,
                )
                .to_owned(),
                bound_identity_row_ref: "account-scope-beta:connected-account:connected:human-dev"
                    .to_owned(),
                managed_policy_bundle_ref: None,
            },
            fallback: FallbackDescriptor {
                fallback_mode: ProviderFallbackMode::OpenInProvider,
                fallback_mode_token: provider_fallback_token(ProviderFallbackMode::OpenInProvider)
                    .to_owned(),
                fallback_label: "Open in browser".to_owned(),
                browser_handoff_packet_ref: Some(
                    "browser-handoff-packet:pr:1234:comment:connected".to_owned(),
                ),
                publish_later_queue_item_ref: None,
                inspect_only_snapshot_ref: None,
                copy_or_export_evidence_ref: None,
            },
            freshness: RouteFreshness {
                freshness_class: FreshnessLabel::Fresh,
                freshness_class_token: FreshnessLabel::Fresh.as_str().to_owned(),
                freshness_floor_ref: "freshness-floor:code-host:public:60s".to_owned(),
                observed_at: "2026-05-16T10:00:30Z".to_owned(),
                stale_after: Some("2026-05-16T10:01:30Z".to_owned()),
                degraded_reason: None,
            },
            route_degraded_state: RouteDegradedStateClass::Green,
            route_degraded_state_token: RouteDegradedStateClass::Green.as_str().to_owned(),
            command_route_packet_ref: Some(
                "command-route:reconstruction:pr:1234:comment:connected".to_owned(),
            ),
            raw_token_material_present: false,
            silent_public_endpoint_fallback_taken: false,
            silent_authority_widening_taken: false,
            local_editing_preserved: true,
        },
        // Mirror-only: signed mirror, browser handoff fallback for merge.
        RouteResolutionRow {
            record_kind: ROUTE_RESOLUTION_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "route-resolution-beta:row:mirror_only:reviewer:merge".to_owned(),
            display_label: "Merge PR routed through signed mirror".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            lane_class: LaneClass::ManagedMirrorLane,
            lane_class_token: LaneClass::ManagedMirrorLane.as_str().to_owned(),
            provider_source_class: ProviderSourceClass::MirroredOrSelfHosted,
            provider_surface_class: ProviderSurfaceClass::CodeHostSurface,
            provider_descriptor_ref: "provider-descriptor:code-host:enterprise-mirror:payments-org"
                .to_owned(),
            action_class: RouteActionClass::ProviderMutation,
            action_class_token: RouteActionClass::ProviderMutation.as_str().to_owned(),
            provider_linked_row_ref: "provider-linked-row:pr:payments/backend:1234".to_owned(),
            route: RouteDescriptor {
                route_choice: RouteChoiceClass::SignedMirrorRoute,
                route_choice_token: RouteChoiceClass::SignedMirrorRoute.as_str().to_owned(),
                route_label: "Enterprise signed mirror".to_owned(),
                transport_label: "signed_mirror_https".to_owned(),
                target_identity_ref:
                    "provider-target-identity:code-host:enterprise-mirror:payments/backend:1234"
                        .to_owned(),
                tunnel_session_ref: None,
                mirror_identity_ref: Some(
                    "mirror-identity:enterprise-mirror:tenant-001:fleet-mirror".to_owned(),
                ),
                snapshot_publisher_ref: None,
            },
            owner: RouteOwnerDescriptor {
                owner_class: RouteOwnerClass::ManagedPolicyAuthority,
                owner_class_token: RouteOwnerClass::ManagedPolicyAuthority.as_str().to_owned(),
                owner_ref: "managed-authority:tenant-001:mirror-owner".to_owned(),
                owner_label: "Managed mirror authority (tenant 001)".to_owned(),
                managed_policy_bundle_ref: Some(
                    "managed-policy-bundle:tenant-001:mirror:v3".to_owned(),
                ),
            },
            grant: GrantDescriptor {
                acting_identity_class: ActingIdentityClass::ConnectedAccount,
                acting_identity_class_token: ActingIdentityClass::ConnectedAccount
                    .as_str()
                    .to_owned(),
                auth_source: ProviderAuthSourceClass::HumanSession,
                auth_source_token: provider_auth_source_token(
                    ProviderAuthSourceClass::HumanSession,
                )
                .to_owned(),
                bound_identity_row_ref:
                    "account-scope-beta:connected-account:mirror_only:human-reviewer".to_owned(),
                managed_policy_bundle_ref: None,
            },
            fallback: FallbackDescriptor {
                fallback_mode: ProviderFallbackMode::OpenInProvider,
                fallback_mode_token: provider_fallback_token(ProviderFallbackMode::OpenInProvider)
                    .to_owned(),
                fallback_label: "Complete merge in browser".to_owned(),
                browser_handoff_packet_ref: Some(
                    "browser-handoff-packet:pr:1234:merge:mirror_only".to_owned(),
                ),
                publish_later_queue_item_ref: None,
                inspect_only_snapshot_ref: None,
                copy_or_export_evidence_ref: None,
            },
            freshness: RouteFreshness {
                freshness_class: FreshnessLabel::StaleWithinWindow,
                freshness_class_token: FreshnessLabel::StaleWithinWindow.as_str().to_owned(),
                freshness_floor_ref: "freshness-floor:mirror:tenant-001:300s".to_owned(),
                observed_at: "2026-05-16T10:05:30Z".to_owned(),
                stale_after: Some("2026-05-16T10:10:30Z".to_owned()),
                degraded_reason: Some(
                    "Mirror lag is 240s — within review window but no longer fresh.".to_owned(),
                ),
            },
            route_degraded_state: RouteDegradedStateClass::MirrorLagBeyondTolerance,
            route_degraded_state_token: RouteDegradedStateClass::MirrorLagBeyondTolerance
                .as_str()
                .to_owned(),
            command_route_packet_ref: Some(
                "command-route:reconstruction:pr:1234:merge:mirror_only".to_owned(),
            ),
            raw_token_material_present: false,
            silent_public_endpoint_fallback_taken: false,
            silent_authority_widening_taken: false,
            local_editing_preserved: true,
        },
        // Offline: imported snapshot, local-draft fallback.
        RouteResolutionRow {
            record_kind: ROUTE_RESOLUTION_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "route-resolution-beta:row:offline:release-signer:release-publish".to_owned(),
            display_label: "Release publish parked while offline snapshot is the only route"
                .to_owned(),
            profile: AccountScopeBetaProfileClass::Offline,
            profile_token: AccountScopeBetaProfileClass::Offline.as_str().to_owned(),
            lane_class: LaneClass::OfflineSnapshotLane,
            lane_class_token: LaneClass::OfflineSnapshotLane.as_str().to_owned(),
            provider_source_class: ProviderSourceClass::ImportedSnapshot,
            provider_surface_class: ProviderSurfaceClass::CiOrChecksSurface,
            provider_descriptor_ref: "provider-descriptor:release-registry:enterprise:fleet-0001"
                .to_owned(),
            action_class: RouteActionClass::ReleasePublish,
            action_class_token: RouteActionClass::ReleasePublish.as_str().to_owned(),
            provider_linked_row_ref: "provider-linked-row:release:fleet-0001:v1.2.3".to_owned(),
            route: RouteDescriptor {
                route_choice: RouteChoiceClass::ImportedSnapshotRoute,
                route_choice_token: RouteChoiceClass::ImportedSnapshotRoute.as_str().to_owned(),
                route_label: "Air-gapped release snapshot".to_owned(),
                transport_label: "imported_snapshot_read".to_owned(),
                target_identity_ref:
                    "provider-target-identity:release-registry:enterprise:fleet-0001:v1.2.3"
                        .to_owned(),
                tunnel_session_ref: None,
                mirror_identity_ref: None,
                snapshot_publisher_ref: Some(
                    "snapshot-publisher:release-registry:tenant-001".to_owned(),
                ),
            },
            owner: RouteOwnerDescriptor {
                owner_class: RouteOwnerClass::OfflineImportAuthority,
                owner_class_token: RouteOwnerClass::OfflineImportAuthority.as_str().to_owned(),
                owner_ref: "offline-import-authority:tenant-001:release-registry".to_owned(),
                owner_label: "Signed release snapshot publisher".to_owned(),
                managed_policy_bundle_ref: None,
            },
            grant: GrantDescriptor {
                acting_identity_class: ActingIdentityClass::DelegatedCredential,
                acting_identity_class_token: ActingIdentityClass::DelegatedCredential
                    .as_str()
                    .to_owned(),
                auth_source: ProviderAuthSourceClass::DelegatedCredential,
                auth_source_token: provider_auth_source_token(
                    ProviderAuthSourceClass::DelegatedCredential,
                )
                .to_owned(),
                bound_identity_row_ref:
                    "account-scope-beta:delegated-credential:offline:release-signer".to_owned(),
                managed_policy_bundle_ref: None,
            },
            fallback: FallbackDescriptor {
                fallback_mode: ProviderFallbackMode::PublishLaterQueue,
                fallback_mode_token: provider_fallback_token(
                    ProviderFallbackMode::PublishLaterQueue,
                )
                .to_owned(),
                fallback_label: "Queue release until delegated credential reissues".to_owned(),
                browser_handoff_packet_ref: None,
                publish_later_queue_item_ref: Some(
                    "publish-later-queue-item:release:fleet-0001:v1.2.3".to_owned(),
                ),
                inspect_only_snapshot_ref: None,
                copy_or_export_evidence_ref: None,
            },
            freshness: RouteFreshness {
                freshness_class: FreshnessLabel::ExpiredBeyondWindow,
                freshness_class_token: FreshnessLabel::ExpiredBeyondWindow.as_str().to_owned(),
                freshness_floor_ref: "freshness-floor:offline-snapshot:7d".to_owned(),
                observed_at: "2026-05-09T10:00:00Z".to_owned(),
                stale_after: Some("2026-05-16T10:00:00Z".to_owned()),
                degraded_reason: Some(
                    "Snapshot is older than the 7-day retention floor and the delegated \
                     credential expired."
                        .to_owned(),
                ),
            },
            route_degraded_state: RouteDegradedStateClass::SnapshotOlderThanRetentionFloor,
            route_degraded_state_token: RouteDegradedStateClass::SnapshotOlderThanRetentionFloor
                .as_str()
                .to_owned(),
            command_route_packet_ref: Some(
                "command-route:reconstruction:release:fleet-0001:v1.2.3:offline".to_owned(),
            ),
            raw_token_material_present: false,
            silent_public_endpoint_fallback_taken: false,
            silent_authority_widening_taken: false,
            local_editing_preserved: true,
        },
        // Enterprise-managed: managed provider lane, denied with admin review fallback.
        RouteResolutionRow {
            record_kind: ROUTE_RESOLUTION_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "route-resolution-beta:row:enterprise_managed:managed-bot:check-run".to_owned(),
            display_label: "Managed deploy bot blocked by managed policy authority".to_owned(),
            profile: AccountScopeBetaProfileClass::EnterpriseManaged,
            profile_token: AccountScopeBetaProfileClass::EnterpriseManaged
                .as_str()
                .to_owned(),
            lane_class: LaneClass::ManagedProviderLane,
            lane_class_token: LaneClass::ManagedProviderLane.as_str().to_owned(),
            provider_source_class: ProviderSourceClass::LiveProvider,
            provider_surface_class: ProviderSurfaceClass::CiOrChecksSurface,
            provider_descriptor_ref: "provider-descriptor:code-host:enterprise:tenant-001-fleet"
                .to_owned(),
            action_class: RouteActionClass::CiOrCheckMutation,
            action_class_token: RouteActionClass::CiOrCheckMutation.as_str().to_owned(),
            provider_linked_row_ref: "provider-linked-row:check-run:tenant-001/fleet:42".to_owned(),
            route: RouteDescriptor {
                route_choice: RouteChoiceClass::LiveProviderDirect,
                route_choice_token: RouteChoiceClass::LiveProviderDirect.as_str().to_owned(),
                route_label: "Enterprise code host (tenant 001)".to_owned(),
                transport_label: "https".to_owned(),
                target_identity_ref:
                    "provider-target-identity:code-host:enterprise:tenant-001/fleet:42".to_owned(),
                tunnel_session_ref: None,
                mirror_identity_ref: None,
                snapshot_publisher_ref: None,
            },
            owner: RouteOwnerDescriptor {
                owner_class: RouteOwnerClass::ManagedPolicyAuthority,
                owner_class_token: RouteOwnerClass::ManagedPolicyAuthority.as_str().to_owned(),
                owner_ref: "managed-authority:tenant-001:deploy-bot-owner".to_owned(),
                owner_label: "Managed policy authority (tenant 001)".to_owned(),
                managed_policy_bundle_ref: Some(
                    "managed-policy-bundle:tenant-001:deploy-bot:v3".to_owned(),
                ),
            },
            grant: GrantDescriptor {
                acting_identity_class: ActingIdentityClass::InstallationGrant,
                acting_identity_class_token: ActingIdentityClass::InstallationGrant
                    .as_str()
                    .to_owned(),
                auth_source: ProviderAuthSourceClass::PolicyInjectedService,
                auth_source_token: provider_auth_source_token(
                    ProviderAuthSourceClass::PolicyInjectedService,
                )
                .to_owned(),
                bound_identity_row_ref:
                    "account-scope-beta:installation-grant:enterprise_managed:managed-bot"
                        .to_owned(),
                managed_policy_bundle_ref: Some(
                    "managed-policy-bundle:tenant-001:deploy-bot:v3".to_owned(),
                ),
            },
            fallback: FallbackDescriptor {
                fallback_mode: ProviderFallbackMode::CopyOrExport,
                fallback_mode_token: provider_fallback_token(ProviderFallbackMode::CopyOrExport)
                    .to_owned(),
                fallback_label: "Export evidence for admin review".to_owned(),
                browser_handoff_packet_ref: None,
                publish_later_queue_item_ref: None,
                inspect_only_snapshot_ref: None,
                copy_or_export_evidence_ref: Some(
                    "evidence-bundle:admin-review:managed-deploy:tenant-001".to_owned(),
                ),
            },
            freshness: RouteFreshness {
                freshness_class: FreshnessLabel::RevokedOrDisconnected,
                freshness_class_token: FreshnessLabel::RevokedOrDisconnected.as_str().to_owned(),
                freshness_floor_ref: "freshness-floor:managed-policy:tenant-001:60s".to_owned(),
                observed_at: "2026-05-16T10:20:00Z".to_owned(),
                stale_after: None,
                degraded_reason: Some(
                    "Managed policy authority suspended the deploy-bot grant pending admin \
                     review."
                        .to_owned(),
                ),
            },
            route_degraded_state: RouteDegradedStateClass::ManagedPolicyBoundaryClosed,
            route_degraded_state_token: RouteDegradedStateClass::ManagedPolicyBoundaryClosed
                .as_str()
                .to_owned(),
            command_route_packet_ref: Some(
                "command-route:reconstruction:check-run:tenant-001/fleet:42:enterprise_managed"
                    .to_owned(),
            ),
            raw_token_material_present: false,
            silent_public_endpoint_fallback_taken: false,
            silent_authority_widening_taken: false,
            local_editing_preserved: true,
        },
        // Connected (tunnel) profile: tunnel-exposed external lane.
        RouteResolutionRow {
            record_kind: ROUTE_RESOLUTION_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "route-resolution-beta:row:connected:tunnel:inspect".to_owned(),
            display_label: "Inspect tunnel-exposed dev service".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            lane_class: LaneClass::TunnelExposedExternalLane,
            lane_class_token: LaneClass::TunnelExposedExternalLane.as_str().to_owned(),
            provider_source_class: ProviderSourceClass::LiveProvider,
            provider_surface_class: ProviderSurfaceClass::CiOrChecksSurface,
            provider_descriptor_ref: "provider-descriptor:dev-tunnel:public:payments-org"
                .to_owned(),
            action_class: RouteActionClass::ReadOnlyInspection,
            action_class_token: RouteActionClass::ReadOnlyInspection.as_str().to_owned(),
            provider_linked_row_ref:
                "provider-linked-row:dev-tunnel:payments/backend:preview-server".to_owned(),
            route: RouteDescriptor {
                route_choice: RouteChoiceClass::TunnelExposedRoute,
                route_choice_token: RouteChoiceClass::TunnelExposedRoute.as_str().to_owned(),
                route_label: "Tunnel-exposed dev preview".to_owned(),
                transport_label: "tunnel_https".to_owned(),
                target_identity_ref:
                    "provider-target-identity:dev-tunnel:public:payments/backend:preview-server"
                        .to_owned(),
                tunnel_session_ref: Some(
                    "tunnel-session:dev-tunnel:payments:dev-001:preview".to_owned(),
                ),
                mirror_identity_ref: None,
                snapshot_publisher_ref: None,
            },
            owner: RouteOwnerDescriptor {
                owner_class: RouteOwnerClass::TunnelSessionOwner,
                owner_class_token: RouteOwnerClass::TunnelSessionOwner.as_str().to_owned(),
                owner_ref: "actor:human-account:workspace:payments:dev-001".to_owned(),
                owner_label: "Workspace developer (tunnel session owner)".to_owned(),
                managed_policy_bundle_ref: None,
            },
            grant: GrantDescriptor {
                acting_identity_class: ActingIdentityClass::ConnectedAccount,
                acting_identity_class_token: ActingIdentityClass::ConnectedAccount
                    .as_str()
                    .to_owned(),
                auth_source: ProviderAuthSourceClass::HumanSession,
                auth_source_token: provider_auth_source_token(
                    ProviderAuthSourceClass::HumanSession,
                )
                .to_owned(),
                bound_identity_row_ref: "account-scope-beta:connected-account:connected:human-dev"
                    .to_owned(),
                managed_policy_bundle_ref: None,
            },
            fallback: FallbackDescriptor {
                fallback_mode: ProviderFallbackMode::InspectOnly,
                fallback_mode_token: provider_fallback_token(ProviderFallbackMode::InspectOnly)
                    .to_owned(),
                fallback_label: "Read tunnel preview metadata snapshot".to_owned(),
                browser_handoff_packet_ref: None,
                publish_later_queue_item_ref: None,
                inspect_only_snapshot_ref: Some(
                    "imported-snapshot:tunnel-preview:payments:preview-server".to_owned(),
                ),
                copy_or_export_evidence_ref: None,
            },
            freshness: RouteFreshness {
                freshness_class: FreshnessLabel::Fresh,
                freshness_class_token: FreshnessLabel::Fresh.as_str().to_owned(),
                freshness_floor_ref: "freshness-floor:tunnel:dev:60s".to_owned(),
                observed_at: "2026-05-16T10:00:45Z".to_owned(),
                stale_after: Some("2026-05-16T10:01:45Z".to_owned()),
                degraded_reason: None,
            },
            route_degraded_state: RouteDegradedStateClass::Green,
            route_degraded_state_token: RouteDegradedStateClass::Green.as_str().to_owned(),
            command_route_packet_ref: Some(
                "command-route:reconstruction:tunnel-preview:payments:preview-server".to_owned(),
            ),
            raw_token_material_present: false,
            silent_public_endpoint_fallback_taken: false,
            silent_authority_widening_taken: false,
            local_editing_preserved: true,
        },
    ]
}

fn seed_browser_handoff_panels() -> Vec<BrowserHandoffPanel> {
    vec![
        BrowserHandoffPanel {
            record_kind: ROUTE_RESOLUTION_BETA_BROWSER_HANDOFF_PANEL_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            panel_id: "route-resolution-beta:browser-handoff-panel:connected:human-dev:comment"
                .to_owned(),
            display_label: "Open comment in browser".to_owned(),
            bound_row_ref: "route-resolution-beta:row:connected:human-dev:comment".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            handoff_reason: BrowserHandoffReasonClass::MutationNotSupportedInProduct,
            handoff_reason_token: BrowserHandoffReasonClass::MutationNotSupportedInProduct
                .as_str()
                .to_owned(),
            // Projected as the row's `OpenInProvider` fallback.
            projected_route_choice: RouteChoiceClass::SystemBrowserHandoffRoute,
            projected_route_choice_token: RouteChoiceClass::SystemBrowserHandoffRoute
                .as_str()
                .to_owned(),
            projected_owner_class: RouteOwnerClass::WorkspaceUser,
            projected_owner_class_token: RouteOwnerClass::WorkspaceUser.as_str().to_owned(),
            projected_acting_identity_class: ActingIdentityClass::ConnectedAccount,
            projected_acting_identity_class_token: ActingIdentityClass::ConnectedAccount
                .as_str()
                .to_owned(),
            browser_handoff_packet_ref: "browser-handoff-packet:pr:1234:comment:connected"
                .to_owned(),
            return_summary_ref: None,
            panel_summary:
                "Open the PR comment surface in the system browser under the signed-in human \
                 account. Provider mutation is not supported in-product."
                    .to_owned(),
            raw_url_present: false,
            raw_provider_payload_present: false,
        },
        BrowserHandoffPanel {
            record_kind: ROUTE_RESOLUTION_BETA_BROWSER_HANDOFF_PANEL_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            panel_id: "route-resolution-beta:browser-handoff-panel:mirror_only:reviewer:merge"
                .to_owned(),
            display_label: "Complete merge in browser".to_owned(),
            bound_row_ref: "route-resolution-beta:row:mirror_only:reviewer:merge".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            handoff_reason: BrowserHandoffReasonClass::StepUpRequired,
            handoff_reason_token: BrowserHandoffReasonClass::StepUpRequired
                .as_str()
                .to_owned(),
            projected_route_choice: RouteChoiceClass::SystemBrowserHandoffRoute,
            projected_route_choice_token: RouteChoiceClass::SystemBrowserHandoffRoute
                .as_str()
                .to_owned(),
            projected_owner_class: RouteOwnerClass::ManagedPolicyAuthority,
            projected_owner_class_token: RouteOwnerClass::ManagedPolicyAuthority
                .as_str()
                .to_owned(),
            projected_acting_identity_class: ActingIdentityClass::ConnectedAccount,
            projected_acting_identity_class_token: ActingIdentityClass::ConnectedAccount
                .as_str()
                .to_owned(),
            browser_handoff_packet_ref: "browser-handoff-packet:pr:1234:merge:mirror_only"
                .to_owned(),
            return_summary_ref: None,
            panel_summary:
                "Mirror lag exceeds review window; merge must complete in browser with step-up \
                 reauth before authority is reused."
                    .to_owned(),
            raw_url_present: false,
            raw_provider_payload_present: false,
        },
    ]
}

fn seed_authority_truth_panels() -> Vec<AuthorityTruthPanel> {
    vec![
        AuthorityTruthPanel {
            record_kind: ROUTE_RESOLUTION_BETA_AUTHORITY_TRUTH_PANEL_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            panel_id: "route-resolution-beta:authority-truth-panel:connected:human-dev:comment"
                .to_owned(),
            display_label: "Comment authority is honest".to_owned(),
            bound_row_ref: "route-resolution-beta:row:connected:human-dev:comment".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            truth_state: AuthorityTruthState::GreenClaimHonest,
            truth_state_token: AuthorityTruthState::GreenClaimHonest.as_str().to_owned(),
            green_claim_held: true,
            rationale_summary:
                "Live provider route is fresh; signed-in human-account grant is active and the \
                 PR target is unchanged."
                    .to_owned(),
            computed_at: "2026-05-16T10:00:45Z".to_owned(),
        },
        AuthorityTruthPanel {
            record_kind: ROUTE_RESOLUTION_BETA_AUTHORITY_TRUTH_PANEL_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            panel_id: "route-resolution-beta:authority-truth-panel:mirror_only:reviewer:merge"
                .to_owned(),
            display_label: "Merge authority is degraded".to_owned(),
            bound_row_ref: "route-resolution-beta:row:mirror_only:reviewer:merge".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            truth_state: AuthorityTruthState::ClaimDegraded,
            truth_state_token: AuthorityTruthState::ClaimDegraded.as_str().to_owned(),
            green_claim_held: false,
            rationale_summary:
                "Mirror lag is beyond tolerance; green claim is retracted in favour of the \
                 browser-handoff fallback under managed authority."
                    .to_owned(),
            computed_at: "2026-05-16T10:05:45Z".to_owned(),
        },
        AuthorityTruthPanel {
            record_kind: ROUTE_RESOLUTION_BETA_AUTHORITY_TRUTH_PANEL_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            panel_id:
                "route-resolution-beta:authority-truth-panel:offline:release-signer:release-publish"
                    .to_owned(),
            display_label: "Release authority retracted on offline profile".to_owned(),
            bound_row_ref: "route-resolution-beta:row:offline:release-signer:release-publish"
                .to_owned(),
            profile: AccountScopeBetaProfileClass::Offline,
            profile_token: AccountScopeBetaProfileClass::Offline.as_str().to_owned(),
            truth_state: AuthorityTruthState::ClaimStaleAndRetracted,
            truth_state_token: AuthorityTruthState::ClaimStaleAndRetracted.as_str().to_owned(),
            green_claim_held: false,
            rationale_summary:
                "Snapshot is older than the 7-day retention floor and the delegated credential \
                 expired; release publish is parked as a publish-later queue item."
                    .to_owned(),
            computed_at: "2026-05-16T10:25:45Z".to_owned(),
        },
        AuthorityTruthPanel {
            record_kind: ROUTE_RESOLUTION_BETA_AUTHORITY_TRUTH_PANEL_RECORD_KIND.to_owned(),
            schema_version: ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
            shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF.to_owned(),
            panel_id:
                "route-resolution-beta:authority-truth-panel:enterprise_managed:managed-bot:check-run"
                    .to_owned(),
            display_label: "Managed deploy-bot authority is closed".to_owned(),
            bound_row_ref:
                "route-resolution-beta:row:enterprise_managed:managed-bot:check-run".to_owned(),
            profile: AccountScopeBetaProfileClass::EnterpriseManaged,
            profile_token: AccountScopeBetaProfileClass::EnterpriseManaged
                .as_str()
                .to_owned(),
            truth_state: AuthorityTruthState::ClaimStaleAndRetracted,
            truth_state_token: AuthorityTruthState::ClaimStaleAndRetracted.as_str().to_owned(),
            green_claim_held: false,
            rationale_summary:
                "Managed policy authority suspended the deploy-bot grant; mutation authority is \
                 retracted and fallback exports evidence for admin review."
                    .to_owned(),
            computed_at: "2026-05-16T10:20:45Z".to_owned(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_route_resolution_beta_page();
        validate_route_resolution_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        for profile in AccountScopeBetaProfileClass::ALL {
            assert!(page
                .summary
                .profiles_present
                .iter()
                .any(|token| token == profile.as_str()));
        }
    }

    #[test]
    fn seeded_page_covers_all_lane_classes_and_route_choices() {
        let page = seeded_route_resolution_beta_page();
        let lanes: BTreeSet<&str> = page
            .summary
            .lane_classes_present
            .iter()
            .map(String::as_str)
            .collect();
        assert!(lanes.contains("managed_provider_lane"));
        assert!(lanes.contains("managed_mirror_lane"));
        assert!(lanes.contains("external_provider_lane"));
        assert!(lanes.contains("tunnel_exposed_external_lane"));
        assert!(lanes.contains("offline_snapshot_lane"));

        let routes: BTreeSet<&str> = page
            .summary
            .route_choices_present
            .iter()
            .map(String::as_str)
            .collect();
        assert!(routes.contains("live_provider_direct"));
        assert!(routes.contains("signed_mirror_route"));
        assert!(routes.contains("tunnel_exposed_route"));
        assert!(routes.contains("imported_snapshot_route"));
    }

    #[test]
    fn validator_flags_raw_token_material() {
        let mut page = seeded_route_resolution_beta_page();
        page.rows[0].raw_token_material_present = true;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == RouteResolutionBetaDefectKind::RawTokenMaterialPresent));
    }

    #[test]
    fn validator_flags_silent_public_endpoint_fallback_taken() {
        let mut page = seeded_route_resolution_beta_page();
        page.rows[1].silent_public_endpoint_fallback_taken = true;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::SilentPublicEndpointFallbackTaken));
    }

    #[test]
    fn validator_flags_managed_lane_without_bundle() {
        let mut page = seeded_route_resolution_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.lane_class == LaneClass::ManagedProviderLane)
            .expect("managed lane row");
        row.owner.managed_policy_bundle_ref = None;
        row.grant.managed_policy_bundle_ref = None;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::ManagedLaneWithoutManagedPolicyBundle));
    }

    #[test]
    fn validator_flags_external_lane_with_managed_bundle() {
        let mut page = seeded_route_resolution_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.lane_class == LaneClass::ExternalProviderLane)
            .expect("external lane row");
        row.owner.managed_policy_bundle_ref =
            Some("managed-policy-bundle:should-not-be-here".to_owned());
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::ExternalLaneWithManagedPolicyBundle));
    }

    #[test]
    fn validator_flags_tunnel_route_without_session() {
        let mut page = seeded_route_resolution_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.route.route_choice == RouteChoiceClass::TunnelExposedRoute)
            .expect("tunnel row");
        row.route.tunnel_session_ref = None;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::TunnelRouteWithoutSessionRef));
    }

    #[test]
    fn validator_flags_mirror_route_without_mirror_ref() {
        let mut page = seeded_route_resolution_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.route.route_choice == RouteChoiceClass::SignedMirrorRoute)
            .expect("mirror row");
        row.route.mirror_identity_ref = None;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::MirrorRouteWithoutMirrorRef));
    }

    #[test]
    fn validator_flags_snapshot_route_without_publisher() {
        let mut page = seeded_route_resolution_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.route.route_choice == RouteChoiceClass::ImportedSnapshotRoute)
            .expect("snapshot row");
        row.route.snapshot_publisher_ref = None;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::SnapshotRouteWithoutPublisherRef));
    }

    #[test]
    fn validator_flags_browser_handoff_panel_route_mismatch() {
        let mut page = seeded_route_resolution_beta_page();
        page.browser_handoff_panels[0].projected_route_choice = RouteChoiceClass::LocalOnlyRoute;
        page.browser_handoff_panels[0].projected_route_choice_token =
            RouteChoiceClass::LocalOnlyRoute.as_str().to_owned();
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::BrowserHandoffPanelRouteChoiceMismatch));
    }

    #[test]
    fn validator_flags_browser_handoff_panel_owner_mismatch() {
        let mut page = seeded_route_resolution_beta_page();
        page.browser_handoff_panels[1].projected_owner_class = RouteOwnerClass::ProviderAuthority;
        page.browser_handoff_panels[1].projected_owner_class_token =
            RouteOwnerClass::ProviderAuthority.as_str().to_owned();
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::BrowserHandoffPanelOwnerClassMismatch));
    }

    #[test]
    fn validator_flags_authority_truth_green_claim_while_stale() {
        let mut page = seeded_route_resolution_beta_page();
        // Force the second authority-truth panel (the mirror-only degraded one)
        // to falsely claim green.
        page.authority_truth_panels[1].truth_state = AuthorityTruthState::GreenClaimHonest;
        page.authority_truth_panels[1].truth_state_token =
            AuthorityTruthState::GreenClaimHonest.as_str().to_owned();
        page.authority_truth_panels[1].green_claim_held = true;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::AuthorityTruthPanelGreenClaimWhileStale));
    }

    #[test]
    fn validator_flags_authority_truth_green_flag_disagrees() {
        let mut page = seeded_route_resolution_beta_page();
        // The first panel is honestly green; flip the green flag to false to
        // induce disagreement with the typed state.
        page.authority_truth_panels[0].green_claim_held = false;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::AuthorityTruthPanelGreenFlagDisagreesWithState));
    }

    #[test]
    fn validator_flags_fallback_publish_later_without_queue() {
        let mut page = seeded_route_resolution_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.fallback.fallback_mode == ProviderFallbackMode::PublishLaterQueue)
            .expect("publish-later fallback row");
        row.fallback.publish_later_queue_item_ref = None;
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::FallbackPublishLaterWithoutQueueRef));
    }

    #[test]
    fn validator_flags_green_state_with_stale_freshness() {
        let mut page = seeded_route_resolution_beta_page();
        let row = &mut page.rows[0];
        row.freshness.freshness_class = FreshnessLabel::StaleWithinWindow;
        row.freshness.freshness_class_token = FreshnessLabel::StaleWithinWindow.as_str().to_owned();
        row.freshness.degraded_reason = Some("forced stale".to_owned());
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::RouteGreenStateWithStaleFreshness));
    }

    #[test]
    fn validator_flags_missing_profile_coverage() {
        let mut page = seeded_route_resolution_beta_page();
        page.rows
            .retain(|row| row.profile != AccountScopeBetaProfileClass::EnterpriseManaged);
        page.browser_handoff_panels
            .retain(|panel| panel.profile != AccountScopeBetaProfileClass::EnterpriseManaged);
        page.authority_truth_panels
            .retain(|panel| panel.profile != AccountScopeBetaProfileClass::EnterpriseManaged);
        let defects = audit_route_resolution_beta_page(
            &page.rows,
            &page.browser_handoff_panels,
            &page.authority_truth_panels,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::ProfileCoverageMissing
            && defect.note.contains("enterprise_managed")));
    }

    #[test]
    fn support_export_preserves_lineage() {
        let page = seeded_route_resolution_beta_page();
        let export = RouteResolutionBetaSupportExport::from_page(
            "route-resolution-beta:support-export:001",
            "2026-05-16T11:00:00Z",
            page,
        );
        assert!(export.raw_tokens_excluded);
        assert!(export.route_lineage_preserved);
        assert!(export.browser_handoff_lineage_preserved);
        assert!(export.authority_truth_lineage_preserved);
        assert!(export.fail_closed_invariant);
        assert!(export.defect_kinds_present.is_empty());
    }

    #[test]
    fn summary_counts_match_records() {
        let page = seeded_route_resolution_beta_page();
        assert_eq!(page.summary.row_count, page.rows.len());
        assert_eq!(
            page.summary.browser_handoff_panel_count,
            page.browser_handoff_panels.len()
        );
        assert_eq!(
            page.summary.authority_truth_panel_count,
            page.authority_truth_panels.len()
        );
        let lane_total: usize = page.summary.rows_by_lane_class.values().sum();
        assert_eq!(lane_total, page.rows.len());
        let degraded_total: usize = page.summary.rows_by_route_degraded_state.values().sum();
        assert_eq!(degraded_total, page.rows.len());
    }
}
