//! Boundary fallback validation for identity, provider, and embedded lanes.
//!
//! This module owns the shell-side alpha packet that verifies the first
//! system-browser auth rows, embedded boundary cards, native handoff review
//! rows, and provider approval-ticket reports stay coherent when a user leaves
//! Aureline for the system browser and returns through a callback or deep link.
//! The packet quotes those upstream records by ref and validates the boundary
//! claims without re-minting their lower-level authority state.

use std::collections::BTreeSet;

use aureline_auth::{
    ClaimedIdentityDefaultActionClass, EmbeddedFallbackPosture, SystemBrowserAlphaPacket,
};
use aureline_provider::{
    ApprovalActorClass, ApprovalAuthSourceClass, ApprovalTicketAlphaValidationReport,
    ApprovalTicketSupportAdminPacket, FindingSeverity,
};
use serde::{Deserialize, Serialize};

use super::boundary_alpha::EmbeddedBoundaryAlphaSnapshot;
use crate::deeplink::native_handoff::NativeBoundaryHandoffPacket;

/// Stable record kind for [`BoundaryFallbackAlphaPacket`] payloads.
pub const BOUNDARY_FALLBACK_ALPHA_PACKET_RECORD_KIND: &str = "boundary_fallback_alpha_packet";

/// Stable schema version for boundary fallback alpha packets.
pub const BOUNDARY_FALLBACK_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`BoundaryFallbackValidationReport`] payloads.
pub const BOUNDARY_FALLBACK_VALIDATION_REPORT_RECORD_KIND: &str =
    "boundary_fallback_validation_report";

/// Optional fixture metadata carried by protected examples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFallbackFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Redaction-safe scenario summary.
    pub scenario: String,
    /// Optional exercised axes documented by the fixture.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exercised_axes: Vec<String>,
}

/// Source artifacts this packet consumes by reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFallbackSourceRefs {
    /// Claimed identity packet ref from the auth lane.
    pub system_browser_alpha_packet_ref: String,
    /// Browser callback packet or fixture ref that proves callback origin truth.
    pub browser_callback_packet_ref: String,
    /// Embedded boundary alpha snapshot ref from the shell boundary lane.
    pub embedded_boundary_snapshot_ref: String,
    /// Native boundary handoff packet ref from the deep-link lane.
    pub native_boundary_handoff_packet_ref: String,
    /// Approval-ticket validation report ref from the provider lane.
    pub approval_ticket_report_ref: String,
    /// Approval-ticket support/admin projection ref from the provider lane.
    pub approval_ticket_support_projection_ref: String,
}

impl BoundaryFallbackSourceRefs {
    fn all_refs(&self) -> [&str; 6] {
        [
            &self.system_browser_alpha_packet_ref,
            &self.browser_callback_packet_ref,
            &self.embedded_boundary_snapshot_ref,
            &self.native_boundary_handoff_packet_ref,
            &self.approval_ticket_report_ref,
            &self.approval_ticket_support_projection_ref,
        ]
    }
}

/// Boundary surface class sampled by the validation packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryFallbackSurfaceClass {
    /// Claimed managed or self-hosted identity row.
    ClaimedIdentity,
    /// Provider-linked object, mutation, or account row.
    ProviderLinkedObject,
    /// Product docs or help pane.
    ProductDocsHelp,
    /// Extension-owned webview.
    ExtensionOwnedWebview,
    /// Marketplace or account content.
    MarketplaceOrAccountContent,
    /// Auth-handoff surface.
    AuthHandoff,
}

impl BoundaryFallbackSurfaceClass {
    /// Returns the stable token for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimedIdentity => "claimed_identity",
            Self::ProviderLinkedObject => "provider_linked_object",
            Self::ProductDocsHelp => "product_docs_help",
            Self::ExtensionOwnedWebview => "extension_owned_webview",
            Self::MarketplaceOrAccountContent => "marketplace_or_account_content",
            Self::AuthHandoff => "auth_handoff",
        }
    }

    const fn requires_auth_default_or_exception(self) -> bool {
        matches!(
            self,
            Self::ClaimedIdentity | Self::ProviderLinkedObject | Self::AuthHandoff
        )
    }

    const fn is_embedded_surface(self) -> bool {
        matches!(
            self,
            Self::ProductDocsHelp
                | Self::ExtensionOwnedWebview
                | Self::MarketplaceOrAccountContent
                | Self::AuthHandoff
        )
    }
}

/// Auth posture claimed for a validation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryAuthPostureClass {
    /// Auth defaults to a system-browser path.
    SystemBrowserDefault,
    /// A bounded embedded exception is named and visibly lower-trust.
    EmbeddedException,
    /// No auth is claimed for this row.
    NoAuthClaim,
    /// The row hands off to a provider-owned browser destination.
    BrowserOnlyProviderHandoff,
}

/// Browser or recovery target exposed by a boundary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryFallbackTargetClass {
    /// System-browser auth or open-external path.
    SystemBrowser,
    /// Device-code fallback.
    DeviceCode,
    /// Exact in-product reopen.
    ExactTargetReopen,
    /// Truthful placeholder recovery surface.
    TruthfulPlaceholder,
    /// Policy blocks external launch.
    PolicyBlocked,
    /// No fallback applies to the row.
    NotApplicable,
}

/// Owner class for native approvals near an embedded or provider row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeApprovalOwnerClass {
    /// Product-owned native review surface.
    ProductOwnedNative,
    /// Embedded content can request native review but cannot render it.
    EmbeddedRequestOnly,
    /// Embedded content attempted to own approval and must be denied.
    EmbeddedOwnedDenied,
}

/// Failure class used by failing-then-recovered examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryFailureClass {
    /// The origin was hidden or unverified.
    HiddenOrigin,
    /// The browser fallback was absent or not inspectable.
    MissingBrowserFallback,
    /// Embedded content attempted to own a high-risk approval.
    EmbeddedApprovalOwnership,
    /// A replayed callback was observed.
    CallbackReplay,
    /// Authority widened or changed while Aureline was out of focus.
    AuthorityDrift,
    /// Trust-store or certificate state changed while Aureline was out of focus.
    TrustStoreChanged,
    /// The session expired while Aureline was out of focus.
    SessionExpired,
}

/// Recovery state for a failing-then-recovered example.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryRecoveryStateClass {
    /// Failure is still open.
    Failed,
    /// Native review remediated the failed condition.
    RecoveredByNativeReview,
    /// Browser fallback remediated the failed condition.
    RecoveredByBrowserFallback,
    /// Placeholder recovery preserved the target without replay.
    RecoveredByPlaceholder,
}

/// Confirm or reject action class rendered by native review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryReviewActionClass {
    /// Confirm through native review.
    ConfirmNativeReview,
    /// Reject and cancel without side effects.
    RejectCancel,
    /// Restart the browser handoff.
    RestartBrowserHandoff,
    /// Continue local-only.
    ContinueLocalOnly,
    /// Reapprove through a native surface.
    ReapproveNative,
    /// Open the exact target.
    OpenExactTarget,
    /// Open a truthful placeholder.
    OpenTruthfulPlaceholder,
}

/// Interruption class observed for a browser callback or deep-link return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackInterruptionClass {
    /// No interruption was observed.
    None,
    /// Sleep or wake occurred before return.
    SleepWake,
    /// The session expired before return.
    ExpiredSession,
    /// A consumed callback was replayed.
    CallbackReplay,
    /// The OS or enterprise trust store changed before return.
    TrustStoreChange,
}

/// Bounded embedded-auth exception disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedExceptionDisclosure {
    /// Exception register ref.
    pub exception_ref: String,
    /// Provider or IdP domain visible on the exception row.
    pub provider_domain_label: String,
    /// Why system-browser auth is unavailable for this bounded path.
    pub justification_label: String,
    /// Fallback target offered when the embedded path cannot continue.
    pub fallback_target_class: BoundaryFallbackTargetClass,
    /// Browser or device-code handoff packet ref.
    pub fallback_handoff_ref: String,
    /// True when the row is rendered visibly lower-trust than native chrome.
    pub lower_trust_cue_visible: bool,
}

/// Open-in-browser or exact-reopen disclosure for a boundary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserFallbackDisclosure {
    /// True when the user can leave to the browser or equivalent fallback.
    pub available: bool,
    /// True when policy blocks the external launch.
    #[serde(default)]
    pub policy_blocked: bool,
    /// Fallback target class.
    pub fallback_target_class: BoundaryFallbackTargetClass,
    /// True when the fallback preserves the object identity.
    pub preserves_object_identity: bool,
    /// True when the exact target can reopen.
    pub exact_target_reopen: bool,
    /// True when a placeholder honestly preserves unavailable target state.
    #[serde(default)]
    pub truthful_placeholder_recovery: bool,
    /// Handoff packet or native review ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handoff_ref: Option<String>,
}

/// Native-approval ownership disclosure for a boundary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeApprovalDisclosure {
    /// Owner class for high-risk approval near this row.
    pub owner_class: NativeApprovalOwnerClass,
    /// True when high-risk approvals remain product-owned.
    pub high_risk_approval_product_owned: bool,
    /// True when destructive confirmations remain product-owned.
    pub destructive_confirmation_product_owned: bool,
    /// True when trust elevation remains product-owned.
    pub trust_elevation_product_owned: bool,
    /// Optional approval ticket or reviewed-scope ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_or_scope_ref: Option<String>,
    /// Optional native reapproval route ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_reapproval_route_ref: Option<String>,
}

/// Theme, zoom, and focus continuity disclosure for embedded chrome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChromeContinuityDisclosure {
    /// True when theme follows host chrome.
    pub theme_continuity: bool,
    /// True when zoom follows host chrome.
    pub zoom_continuity: bool,
    /// True when focus return follows host chrome.
    pub focus_continuity: bool,
    /// True when reduced-motion preference follows host chrome.
    pub reduced_motion_continuity: bool,
}

impl ChromeContinuityDisclosure {
    fn all_required_axes_hold(&self) -> bool {
        self.theme_continuity
            && self.zoom_continuity
            && self.focus_continuity
            && self.reduced_motion_continuity
    }
}

/// Privacy-safe OS/browser handoff disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyHandoffDisclosure {
    /// True when raw URLs, tokens, cookies, and query strings are absent.
    pub metadata_safe_handoff: bool,
    /// True when raw URL or token material appears on the row.
    pub raw_url_or_token_exposed: bool,
    /// True when support export can reconstruct origin and target by ref.
    pub support_export_origin_reconstructable: bool,
    /// Opaque support reconstruction ref.
    pub support_reconstruction_ref: String,
}

/// One claimed boundary row validated by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFallbackClaimRow {
    /// Stable row id.
    pub row_id: String,
    /// Boundary surface class.
    pub surface_class: BoundaryFallbackSurfaceClass,
    /// Upstream row, card, or review ref.
    pub source_ref: String,
    /// Provider or surface label.
    pub provider_label: String,
    /// Provider or origin domain label.
    pub provider_domain_label: String,
    /// Profile, org, workspace, or provider scope label.
    pub profile_or_org_scope_label: String,
    /// Origin disclosure label.
    pub origin_label: String,
    /// True when origin was verified or intentionally withheld by policy.
    pub origin_visible_and_verified: bool,
    /// Auth posture for this row.
    pub auth_posture: BoundaryAuthPostureClass,
    /// True when claimed auth defaults to a system-browser path.
    pub system_browser_default: bool,
    /// Optional bounded embedded-auth exception.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedded_exception: Option<EmbeddedExceptionDisclosure>,
    /// Browser fallback disclosure.
    pub browser_fallback: BrowserFallbackDisclosure,
    /// Native approval ownership disclosure.
    pub native_approval: NativeApprovalDisclosure,
    /// Theme, zoom, focus, and motion continuity.
    pub chrome_continuity: ChromeContinuityDisclosure,
    /// Privacy-safe OS/browser handoff disclosure.
    pub privacy_handoff: PrivacyHandoffDisclosure,
    /// Redaction-safe summary.
    pub support_summary: String,
}

/// One callback or protocol-handoff row validated by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryCallbackReviewRow {
    /// Stable callback review id.
    pub callback_id: String,
    /// Upstream native handoff review or callback packet ref.
    pub source_ref: String,
    /// Source surface token such as `default_browser_callback`.
    pub source_surface_token: String,
    /// Origin class token such as `system_default_browser`.
    pub origin_class_token: String,
    /// Plain origin label.
    pub origin_label: String,
    /// True when origin verification passed or failed closed with disclosure.
    pub origin_visible_and_verified: bool,
    /// Requested action token such as `auth_return`.
    pub requested_action_token: String,
    /// Target scope or object ref.
    pub target_scope_ref: String,
    /// Owning channel ref.
    pub channel_owner_ref: String,
    /// Owning build ref.
    pub build_owner_ref: String,
    /// Confirm action class.
    pub confirm_action: BoundaryReviewActionClass,
    /// Reject action class.
    pub reject_action: BoundaryReviewActionClass,
    /// True when this is broader than a plain local open.
    pub broader_than_plain_local_open: bool,
    /// True when native review is required before acting.
    pub native_review_required: bool,
    /// True when exact target reopen is available.
    pub exact_target_reopen: bool,
    /// True when a truthful placeholder is available instead of silent widen.
    pub truthful_placeholder_recovery: bool,
    /// Interruption class observed on return.
    pub interruption_class: CallbackInterruptionClass,
    /// True when stale or replayed callback state is denied.
    pub stale_or_replayed_callback_denied: bool,
    /// True when authority or target changed while out of focus.
    pub authority_or_target_changed_while_unfocused: bool,
    /// True when native reapproval is required after authority or target drift.
    pub native_reapproval_required: bool,
    /// Ref that reconstructs the exact origin explanation.
    pub exact_origin_explanation_ref: String,
    /// Support/export ref for the row.
    pub support_export_ref: String,
}

/// Actor-class truth row for provider and auth handoffs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryActorTruthRow {
    /// Stable actor row id.
    pub actor_row_id: String,
    /// Actor class rendered on the row.
    pub actor_class: ApprovalActorClass,
    /// Auth source class rendered on the row.
    pub auth_source_class: ApprovalAuthSourceClass,
    /// Source authority ref from approval or credential state.
    pub source_authority_ref: String,
    /// Display label shown to users.
    pub display_label: String,
    /// Support/export ref.
    pub support_export_ref: String,
    /// True when the row avoids collapsing to a generic signed-in state.
    pub generic_signed_in_collapse_prevented: bool,
}

/// Failing-then-recovered example carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFallbackRecoveryExample {
    /// Stable example id.
    pub example_id: String,
    /// Failure class.
    pub failure_class: BoundaryFailureClass,
    /// Source ref that failed validation.
    pub failed_source_ref: String,
    /// Redaction-safe failed state label.
    pub failed_state_label: String,
    /// Native remediation action shown after failure.
    pub native_remediation_action: BoundaryReviewActionClass,
    /// Source ref that proves recovery.
    pub recovered_source_ref: String,
    /// Recovery state.
    pub recovered_state: BoundaryRecoveryStateClass,
    /// True when remediation uses product-owned native chrome.
    pub recovered_by_native_surface: bool,
    /// Support/export ref that reconstructs the failure and recovery.
    pub support_reconstruction_ref: String,
}

/// Canonical boundary fallback alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFallbackAlphaPacket {
    /// Optional protected fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<BoundaryFallbackFixtureMetadata>,
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Source artifacts consumed by this validation packet.
    pub source_refs: BoundaryFallbackSourceRefs,
    /// Boundary rows sampled across identity, provider, docs, extension, marketplace, and auth lanes.
    pub claimed_rows: Vec<BoundaryFallbackClaimRow>,
    /// Callback or protocol-handoff rows sampled by the packet.
    pub callback_rows: Vec<BoundaryCallbackReviewRow>,
    /// Actor-class truth rows sampled by the packet.
    pub actor_truth_rows: Vec<BoundaryActorTruthRow>,
    /// Failing-then-recovered examples proving native remediation.
    pub failing_then_recovered_examples: Vec<BoundaryFallbackRecoveryExample>,
    /// Timestamp when the packet was minted.
    pub minted_at: String,
}

impl BoundaryFallbackAlphaPacket {
    /// Validates the packet without external source artifacts.
    pub fn validate(&self) -> BoundaryFallbackValidationReport {
        let mut validator = BoundaryFallbackValidator::new(self);
        validator.validate_packet();
        validator.finish()
    }

    /// Validates the packet against the upstream source artifacts it quotes.
    pub fn validate_against_sources(
        &self,
        sources: BoundaryFallbackSourceEvidence<'_>,
    ) -> BoundaryFallbackValidationReport {
        let mut validator = BoundaryFallbackValidator::new(self);
        validator.validate_packet();
        validator.validate_sources(sources);
        validator.finish()
    }
}

/// Source evidence used for cross-checking quoted refs.
#[derive(Debug, Clone, Copy)]
pub struct BoundaryFallbackSourceEvidence<'a> {
    /// Claimed identity packet.
    pub system_browser_packet: &'a SystemBrowserAlphaPacket,
    /// Embedded boundary snapshot.
    pub embedded_boundary_snapshot: &'a EmbeddedBoundaryAlphaSnapshot,
    /// Native boundary handoff packet.
    pub native_boundary_handoff_packet: &'a NativeBoundaryHandoffPacket,
    /// Approval-ticket validation report.
    pub approval_ticket_report: &'a ApprovalTicketAlphaValidationReport,
    /// Approval-ticket support/admin projection.
    pub approval_ticket_support_projection: &'a ApprovalTicketSupportAdminPacket,
}

/// Validation report emitted by the first shell consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFallbackValidationReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id under validation.
    pub packet_id: String,
    /// True when no findings were emitted.
    pub passed: bool,
    /// Coverage observed during validation.
    pub coverage: BoundaryFallbackCoverage,
    /// Validation findings.
    pub findings: Vec<BoundaryFallbackValidationFinding>,
}

/// Coverage observed during boundary fallback validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BoundaryFallbackCoverage {
    /// Boundary surface classes covered by claimed rows.
    pub surface_classes: BTreeSet<BoundaryFallbackSurfaceClass>,
    /// Actor classes covered by actor truth rows.
    pub actor_classes: BTreeSet<ApprovalActorClass>,
    /// Callback interruption classes covered by callback rows.
    pub callback_interruption_classes: BTreeSet<CallbackInterruptionClass>,
    /// Failure classes covered by failing-then-recovered examples.
    pub failure_classes: BTreeSet<BoundaryFailureClass>,
    /// True when at least one row proves the system-browser default.
    pub has_system_browser_default: bool,
    /// True when at least one bounded embedded exception is sampled.
    pub has_embedded_exception: bool,
    /// True when at least one row proves native reapproval after drift or denial.
    pub has_native_reapproval: bool,
}

/// Validation finding emitted by the first shell consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFallbackValidationFinding {
    /// Severity of the finding.
    pub severity: FindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe finding message.
    pub message: String,
}

struct BoundaryFallbackValidator<'a> {
    packet: &'a BoundaryFallbackAlphaPacket,
    coverage: BoundaryFallbackCoverage,
    findings: Vec<BoundaryFallbackValidationFinding>,
}

impl<'a> BoundaryFallbackValidator<'a> {
    fn new(packet: &'a BoundaryFallbackAlphaPacket) -> Self {
        Self {
            packet,
            coverage: BoundaryFallbackCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn validate_packet(&mut self) {
        self.expect(
            self.packet.record_kind == BOUNDARY_FALLBACK_ALPHA_PACKET_RECORD_KIND,
            "boundary_fallback.packet_record_kind",
            "packet record_kind must match the boundary fallback contract",
        );
        self.expect(
            self.packet.schema_version == BOUNDARY_FALLBACK_ALPHA_SCHEMA_VERSION,
            "boundary_fallback.packet_schema_version",
            "packet schema version must match the crate constant",
        );
        self.expect(
            non_empty(&self.packet.packet_id),
            "boundary_fallback.packet_id_missing",
            "packet id is required",
        );
        for source_ref in self.packet.source_refs.all_refs() {
            self.expect(
                non_empty(source_ref),
                "boundary_fallback.source_ref_missing",
                "every consumed source ref must be non-empty",
            );
        }
        self.validate_claimed_rows();
        self.validate_callback_rows();
        self.validate_actor_truth_rows();
        self.validate_recovery_examples();
        self.validate_required_coverage();
    }

    fn validate_sources(&mut self, sources: BoundaryFallbackSourceEvidence<'_>) {
        self.expect(
            sources.system_browser_packet.packet_id
                == self.packet.source_refs.system_browser_alpha_packet_ref,
            "boundary_fallback.system_browser_packet_ref_mismatch",
            "quoted system-browser packet ref must match the source packet id",
        );
        self.expect(
            sources.embedded_boundary_snapshot.snapshot_id
                == self.packet.source_refs.embedded_boundary_snapshot_ref,
            "boundary_fallback.embedded_snapshot_ref_mismatch",
            "quoted embedded-boundary snapshot ref must match the source snapshot id",
        );
        self.expect(
            sources.native_boundary_handoff_packet.packet_id
                == self.packet.source_refs.native_boundary_handoff_packet_ref,
            "boundary_fallback.native_handoff_packet_ref_mismatch",
            "quoted native handoff packet ref must match the source packet id",
        );
        self.expect(
            sources.approval_ticket_report.passed,
            "boundary_fallback.approval_ticket_report_failed",
            "approval-ticket source report must pass before boundary fallback validation can pass",
        );
        self.expect(
            sources.approval_ticket_report.packet_id
                == self.packet.source_refs.approval_ticket_report_ref,
            "boundary_fallback.approval_ticket_report_ref_mismatch",
            "quoted approval-ticket report ref must match the source report packet id",
        );
        self.expect(
            sources.approval_ticket_support_projection.packet_id
                == self
                    .packet
                    .source_refs
                    .approval_ticket_support_projection_ref,
            "boundary_fallback.approval_support_ref_mismatch",
            "quoted approval-ticket support ref must match the source projection packet id",
        );
        self.expect(
            sources.approval_ticket_report.packet_id
                == sources.approval_ticket_support_projection.packet_id,
            "boundary_fallback.approval_support_packet_mismatch",
            "approval-ticket report and support projection must quote the same packet",
        );
        self.cross_check_system_browser_rows(sources.system_browser_packet);
        self.cross_check_embedded_rows(sources.embedded_boundary_snapshot);
        self.cross_check_native_handoff_rows(sources.native_boundary_handoff_packet);
        self.cross_check_actor_rows(sources.approval_ticket_report);
    }

    fn validate_claimed_rows(&mut self) {
        self.expect(
            !self.packet.claimed_rows.is_empty(),
            "boundary_fallback.claimed_rows_missing",
            "at least one claimed boundary row is required",
        );
        for row in &self.packet.claimed_rows {
            self.coverage.surface_classes.insert(row.surface_class);
            self.coverage.has_system_browser_default |= row.system_browser_default;
            self.coverage.has_embedded_exception |= row.embedded_exception.is_some();

            self.expect(
                non_empty(&row.row_id) && non_empty(&row.source_ref),
                "boundary_fallback.claim_row_identity_missing",
                "claimed rows must carry row id and source ref",
            );
            self.expect(
                non_empty(&row.provider_label)
                    && non_empty(&row.provider_domain_label)
                    && non_empty(&row.profile_or_org_scope_label),
                "boundary_fallback.claim_row_provider_scope_missing",
                "claimed rows must name provider/domain and profile or org scope",
            );
            self.expect(
                row.origin_visible_and_verified && non_empty(&row.origin_label),
                "boundary_fallback.claim_row_origin_hidden",
                "claimed rows must disclose verified origin or fail closed",
            );
            if row.surface_class.requires_auth_default_or_exception() {
                self.expect(
                    row.system_browser_default || row.embedded_exception.is_some(),
                    "boundary_fallback.system_browser_default_missing",
                    "claimed identity/provider/auth rows must default to system-browser auth unless a bounded embedded exception is named",
                );
            }
            if let Some(exception) = &row.embedded_exception {
                self.validate_embedded_exception(row, exception);
            }
            self.validate_browser_fallback(row);
            self.validate_native_approval(row);
            self.validate_continuity(row);
            self.validate_privacy(row);
        }
    }

    fn validate_embedded_exception(
        &mut self,
        row: &BoundaryFallbackClaimRow,
        exception: &EmbeddedExceptionDisclosure,
    ) {
        self.expect(
            row.auth_posture == BoundaryAuthPostureClass::EmbeddedException,
            "boundary_fallback.embedded_exception_posture_mismatch",
            "embedded exception rows must declare embedded_exception auth posture",
        );
        self.expect(
            non_empty(&exception.exception_ref)
                && non_empty(&exception.provider_domain_label)
                && non_empty(&exception.justification_label)
                && non_empty(&exception.fallback_handoff_ref),
            "boundary_fallback.embedded_exception_fields_missing",
            "embedded auth exceptions must name register ref, provider domain, justification, and fallback",
        );
        self.expect(
            exception.lower_trust_cue_visible,
            "boundary_fallback.embedded_exception_lower_trust_cue_missing",
            "embedded auth exceptions must render visibly lower-trust cues",
        );
        self.expect(
            matches!(
                exception.fallback_target_class,
                BoundaryFallbackTargetClass::SystemBrowser
                    | BoundaryFallbackTargetClass::DeviceCode
                    | BoundaryFallbackTargetClass::TruthfulPlaceholder
            ),
            "boundary_fallback.embedded_exception_fallback_invalid",
            "embedded auth exceptions must provide browser, device-code, or truthful placeholder fallback",
        );
        self.expect(
            row.native_approval.high_risk_approval_product_owned,
            "boundary_fallback.embedded_exception_native_boundary_invalid",
            "embedded auth exceptions cannot own high-risk approval",
        );
    }

    fn validate_browser_fallback(&mut self, row: &BoundaryFallbackClaimRow) {
        if row.browser_fallback.policy_blocked {
            self.expect(
                row.browser_fallback.fallback_target_class
                    == BoundaryFallbackTargetClass::PolicyBlocked,
                "boundary_fallback.policy_blocked_fallback_mismatch",
                "policy-blocked browser fallback must say policy_blocked",
            );
            return;
        }

        let fallback_has_target = matches!(
            row.browser_fallback.fallback_target_class,
            BoundaryFallbackTargetClass::SystemBrowser
                | BoundaryFallbackTargetClass::DeviceCode
                | BoundaryFallbackTargetClass::ExactTargetReopen
                | BoundaryFallbackTargetClass::TruthfulPlaceholder
        );
        self.expect(
            row.browser_fallback.available && fallback_has_target,
            "boundary_fallback.browser_fallback_missing",
            "claimed rows must expose an honest browser, device-code, exact-reopen, or placeholder fallback",
        );
        self.expect(
            row.browser_fallback.preserves_object_identity,
            "boundary_fallback.browser_fallback_loses_identity",
            "browser fallback and exact reopen must preserve object identity",
        );
        self.expect(
            row.browser_fallback.exact_target_reopen
                || row.browser_fallback.truthful_placeholder_recovery,
            "boundary_fallback.exact_reopen_or_placeholder_missing",
            "fallback must offer exact-target reopen or truthful placeholder recovery",
        );
        if row.browser_fallback.available
            && row.browser_fallback.fallback_target_class
                == BoundaryFallbackTargetClass::SystemBrowser
        {
            self.expect(
                row.browser_fallback
                    .handoff_ref
                    .as_deref()
                    .is_some_and(non_empty),
                "boundary_fallback.browser_handoff_ref_missing",
                "system-browser fallback must quote a handoff packet ref",
            );
        }
    }

    fn validate_native_approval(&mut self, row: &BoundaryFallbackClaimRow) {
        self.expect(
            matches!(
                row.native_approval.owner_class,
                NativeApprovalOwnerClass::ProductOwnedNative
                    | NativeApprovalOwnerClass::EmbeddedRequestOnly
            ),
            "boundary_fallback.native_approval_owner_invalid",
            "embedded content must not own high-risk approval",
        );
        self.expect(
            row.native_approval.high_risk_approval_product_owned
                && row.native_approval.destructive_confirmation_product_owned
                && row.native_approval.trust_elevation_product_owned,
            "boundary_fallback.native_approval_not_product_owned",
            "high-risk approvals, destructive confirmations, and trust elevation must remain product-owned",
        );
        self.coverage.has_native_reapproval |= row
            .native_approval
            .native_reapproval_route_ref
            .as_deref()
            .is_some_and(non_empty);
    }

    fn validate_continuity(&mut self, row: &BoundaryFallbackClaimRow) {
        if row.surface_class.is_embedded_surface() {
            self.expect(
                row.chrome_continuity.all_required_axes_hold(),
                "boundary_fallback.chrome_continuity_missing",
                "embedded boundary rows must preserve theme, zoom, focus, and reduced-motion continuity",
            );
        }
    }

    fn validate_privacy(&mut self, row: &BoundaryFallbackClaimRow) {
        self.expect(
            row.privacy_handoff.metadata_safe_handoff
                && !row.privacy_handoff.raw_url_or_token_exposed
                && row.privacy_handoff.support_export_origin_reconstructable
                && non_empty(&row.privacy_handoff.support_reconstruction_ref),
            "boundary_fallback.privacy_handoff_invalid",
            "OS/browser handoff rows must stay metadata-safe and reconstructable without raw URLs or tokens",
        );
    }

    fn validate_callback_rows(&mut self) {
        self.expect(
            !self.packet.callback_rows.is_empty(),
            "boundary_fallback.callback_rows_missing",
            "at least one callback or protocol handoff row is required",
        );
        for row in &self.packet.callback_rows {
            self.coverage
                .callback_interruption_classes
                .insert(row.interruption_class);
            self.coverage.has_native_reapproval |= row.native_reapproval_required;
            self.expect(
                non_empty(&row.callback_id)
                    && non_empty(&row.source_ref)
                    && non_empty(&row.source_surface_token)
                    && non_empty(&row.origin_class_token)
                    && non_empty(&row.origin_label)
                    && non_empty(&row.requested_action_token)
                    && non_empty(&row.target_scope_ref)
                    && non_empty(&row.channel_owner_ref)
                    && non_empty(&row.build_owner_ref),
                "boundary_fallback.callback_required_field_missing",
                "callback rows must disclose source, origin, action, target scope, channel, and build owner",
            );
            self.expect(
                row.origin_visible_and_verified && non_empty(&row.exact_origin_explanation_ref),
                "boundary_fallback.callback_origin_not_reviewable",
                "callback origin must be visible, verified, and reconstructable",
            );
            if row.broader_than_plain_local_open {
                self.expect(
                    row.native_review_required,
                    "boundary_fallback.callback_native_review_missing",
                    "callbacks broader than a plain local open must route through native review",
                );
            }
            self.expect(
                row.exact_target_reopen || row.truthful_placeholder_recovery,
                "boundary_fallback.callback_reopen_or_placeholder_missing",
                "callbacks must offer exact-target reopen or truthful placeholder recovery",
            );
            if row.interruption_class != CallbackInterruptionClass::None {
                self.validate_interrupted_callback(row);
            }
        }
    }

    fn validate_interrupted_callback(&mut self, row: &BoundaryCallbackReviewRow) {
        self.expect(
            row.stale_or_replayed_callback_denied,
            "boundary_fallback.interrupted_callback_not_denied",
            "stale, replayed, or interrupted callback state must fail closed",
        );
        if row.authority_or_target_changed_while_unfocused
            || matches!(
                row.interruption_class,
                CallbackInterruptionClass::SleepWake
                    | CallbackInterruptionClass::ExpiredSession
                    | CallbackInterruptionClass::TrustStoreChange
            )
        {
            self.expect(
                row.native_reapproval_required,
                "boundary_fallback.interrupted_callback_reapproval_missing",
                "callback authority or target drift must require native reapproval",
            );
        }
    }

    fn validate_actor_truth_rows(&mut self) {
        self.expect(
            !self.packet.actor_truth_rows.is_empty(),
            "boundary_fallback.actor_rows_missing",
            "actor-class truth rows are required",
        );
        for row in &self.packet.actor_truth_rows {
            self.coverage.actor_classes.insert(row.actor_class);
            self.expect(
                row.actor_class != ApprovalActorClass::UnknownActorClass
                    && row.generic_signed_in_collapse_prevented
                    && non_empty(&row.actor_row_id)
                    && non_empty(&row.source_authority_ref)
                    && non_empty(&row.display_label)
                    && non_empty(&row.support_export_ref),
                "boundary_fallback.actor_truth_invalid",
                "actor rows must distinguish the acting class and avoid generic signed-in collapse",
            );
        }
    }

    fn validate_recovery_examples(&mut self) {
        self.expect(
            !self.packet.failing_then_recovered_examples.is_empty(),
            "boundary_fallback.recovery_examples_missing",
            "at least one failing-then-recovered example is required",
        );
        for example in &self.packet.failing_then_recovered_examples {
            self.coverage.failure_classes.insert(example.failure_class);
            self.expect(
                non_empty(&example.example_id)
                    && non_empty(&example.failed_source_ref)
                    && non_empty(&example.recovered_source_ref)
                    && non_empty(&example.support_reconstruction_ref),
                "boundary_fallback.recovery_example_ref_missing",
                "failing-then-recovered examples must carry source and reconstruction refs",
            );
            self.expect(
                example.recovered_state != BoundaryRecoveryStateClass::Failed
                    && example.recovered_by_native_surface,
                "boundary_fallback.recovery_example_not_native",
                "failure examples must show native remediation and a recovered state",
            );
        }
    }

    fn validate_required_coverage(&mut self) {
        for surface_class in [
            BoundaryFallbackSurfaceClass::ClaimedIdentity,
            BoundaryFallbackSurfaceClass::ProviderLinkedObject,
            BoundaryFallbackSurfaceClass::ProductDocsHelp,
            BoundaryFallbackSurfaceClass::ExtensionOwnedWebview,
            BoundaryFallbackSurfaceClass::MarketplaceOrAccountContent,
            BoundaryFallbackSurfaceClass::AuthHandoff,
        ] {
            self.expect(
                self.coverage.surface_classes.contains(&surface_class),
                "boundary_fallback.surface_class_coverage_missing",
                "boundary validation must sample identity, provider, docs/help, extension, marketplace/account, and auth-handoff rows",
            );
        }
        for actor_class in [
            ApprovalActorClass::HumanAccount,
            ApprovalActorClass::InstallationOrAppGrant,
            ApprovalActorClass::DelegatedCredential,
        ] {
            self.expect(
                self.coverage.actor_classes.contains(&actor_class),
                "boundary_fallback.actor_class_coverage_missing",
                "boundary validation must distinguish human, installation, and delegated credential actor classes",
            );
        }
        self.expect(
            self.coverage.has_system_browser_default,
            "boundary_fallback.system_browser_default_coverage_missing",
            "at least one claimed row must prove system-browser defaulting",
        );
        self.expect(
            self.coverage.has_native_reapproval,
            "boundary_fallback.native_reapproval_coverage_missing",
            "callback or provider drift must prove native reapproval",
        );
        self.expect(
            self.coverage
                .callback_interruption_classes
                .iter()
                .any(|class| *class != CallbackInterruptionClass::None),
            "boundary_fallback.callback_interruption_coverage_missing",
            "validation must include callback interruption coverage",
        );
        self.expect(
            self.coverage.failure_classes.contains(&BoundaryFailureClass::HiddenOrigin)
                || self
                    .coverage
                    .failure_classes
                    .contains(&BoundaryFailureClass::MissingBrowserFallback)
                || self
                    .coverage
                    .failure_classes
                    .contains(&BoundaryFailureClass::EmbeddedApprovalOwnership),
            "boundary_fallback.recovered_failure_coverage_missing",
            "validation must include a hidden-origin, missing-fallback, or embedded-approval failure recovered by native remediation",
        );
    }

    fn cross_check_system_browser_rows(&mut self, packet: &SystemBrowserAlphaPacket) {
        for row in self
            .packet
            .claimed_rows
            .iter()
            .filter(|row| row.surface_class == BoundaryFallbackSurfaceClass::ClaimedIdentity)
        {
            let upstream = packet
                .claimed_identity_rows
                .iter()
                .find(|candidate| candidate.row_id == row.source_ref);
            self.expect(
                upstream.is_some(),
                "boundary_fallback.system_browser_row_missing",
                "claimed identity row must quote an upstream system-browser row",
            );
            if let Some(upstream) = upstream {
                self.expect(
                    upstream.default_action == ClaimedIdentityDefaultActionClass::OpenSystemBrowser
                        && upstream.auth_policy.embedded_fallback_posture
                            == EmbeddedFallbackPosture::EmbeddedFallbackForbidden
                        && upstream.has_device_code_alternative()
                        && upstream.has_stay_local_alternative(),
                    "boundary_fallback.system_browser_row_truth_mismatch",
                    "upstream identity row must default to system browser and keep device-code plus stay-local fallbacks",
                );
            }
        }
    }

    fn cross_check_embedded_rows(&mut self, snapshot: &EmbeddedBoundaryAlphaSnapshot) {
        for row in self.packet.claimed_rows.iter().filter(|row| {
            matches!(
                row.surface_class,
                BoundaryFallbackSurfaceClass::ProductDocsHelp
                    | BoundaryFallbackSurfaceClass::ExtensionOwnedWebview
                    | BoundaryFallbackSurfaceClass::MarketplaceOrAccountContent
            )
        }) {
            let upstream = snapshot
                .surface_rows
                .iter()
                .find(|candidate| candidate.card_id == row.source_ref);
            self.expect(
                upstream.is_some(),
                "boundary_fallback.embedded_card_missing",
                "embedded boundary validation rows must quote an upstream boundary card",
            );
            if let Some(upstream) = upstream {
                self.expect(
                    upstream.high_risk_approval_host_owned
                        && upstream.fallback_preserves_object_identity
                        && !upstream.chrome_inheritance_tokens.is_empty(),
                    "boundary_fallback.embedded_card_truth_mismatch",
                    "upstream embedded boundary card must preserve native approvals, object identity, and chrome inheritance",
                );
            }
        }
    }

    fn cross_check_native_handoff_rows(&mut self, packet: &NativeBoundaryHandoffPacket) {
        for row in &self.packet.callback_rows {
            let upstream = packet
                .handoff_reviews
                .iter()
                .find(|candidate| candidate.review_id == row.source_ref);
            self.expect(
                upstream.is_some(),
                "boundary_fallback.native_handoff_row_missing",
                "callback rows must quote an upstream native handoff review",
            );
            if let Some(upstream) = upstream {
                self.expect(
                    upstream.origin_class_token == row.origin_class_token
                        && upstream.requested_action_token == row.requested_action_token
                        && upstream.direct_os_execution_forbidden == row.native_review_required,
                    "boundary_fallback.native_handoff_truth_mismatch",
                    "upstream native handoff row must agree on origin, action, and native-review requirement",
                );
            }
        }
    }

    fn cross_check_actor_rows(&mut self, report: &ApprovalTicketAlphaValidationReport) {
        for actor_class in [
            ApprovalActorClass::HumanAccount,
            ApprovalActorClass::InstallationOrAppGrant,
            ApprovalActorClass::DelegatedCredential,
        ] {
            self.expect(
                report.coverage.actor_classes.contains(&actor_class)
                    && self.coverage.actor_classes.contains(&actor_class),
                "boundary_fallback.approval_actor_truth_missing",
                "approval-ticket source coverage and boundary packet must both distinguish required actor classes",
            );
        }
    }

    fn finish(self) -> BoundaryFallbackValidationReport {
        BoundaryFallbackValidationReport {
            record_kind: BOUNDARY_FALLBACK_VALIDATION_REPORT_RECORD_KIND.to_owned(),
            schema_version: BOUNDARY_FALLBACK_ALPHA_SCHEMA_VERSION,
            packet_id: self.packet.packet_id.clone(),
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(BoundaryFallbackValidationFinding {
                severity: FindingSeverity::Error,
                check_id: check_id.to_owned(),
                message: message.to_owned(),
            });
        }
    }
}

fn non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_packet() -> BoundaryFallbackAlphaPacket {
        BoundaryFallbackAlphaPacket {
            fixture_metadata: None,
            record_kind: BOUNDARY_FALLBACK_ALPHA_PACKET_RECORD_KIND.to_owned(),
            schema_version: BOUNDARY_FALLBACK_ALPHA_SCHEMA_VERSION,
            packet_id: "boundary-fallback-alpha:test".to_owned(),
            source_refs: BoundaryFallbackSourceRefs {
                system_browser_alpha_packet_ref: "system-browser-alpha:managed-claim:payments-prod"
                    .to_owned(),
                browser_callback_packet_ref:
                    "browser_callback_packet.managed_sign_in.outbound.0001".to_owned(),
                embedded_boundary_snapshot_ref: "embedded-boundary-alpha:seed".to_owned(),
                native_boundary_handoff_packet_ref: "native-boundary-handoff:alpha:seed".to_owned(),
                approval_ticket_report_ref: "approval-ticket-report:provider-alpha".to_owned(),
                approval_ticket_support_projection_ref: "approval-ticket-support:provider-alpha"
                    .to_owned(),
            },
            claimed_rows: vec![
                claim_row(
                    "boundary-row:identity",
                    BoundaryFallbackSurfaceClass::ClaimedIdentity,
                    "claimed-identity:managed:payments-prod",
                    BoundaryAuthPostureClass::SystemBrowserDefault,
                    true,
                ),
                claim_row(
                    "boundary-row:provider",
                    BoundaryFallbackSurfaceClass::ProviderLinkedObject,
                    "approval_ticket.alpha.code_host.comment.human.0001",
                    BoundaryAuthPostureClass::BrowserOnlyProviderHandoff,
                    true,
                ),
                claim_row(
                    "boundary-row:docs",
                    BoundaryFallbackSurfaceClass::ProductDocsHelp,
                    "ux:card:embedded-boundary-alpha:docs-help",
                    BoundaryAuthPostureClass::NoAuthClaim,
                    false,
                ),
                claim_row(
                    "boundary-row:extension",
                    BoundaryFallbackSurfaceClass::ExtensionOwnedWebview,
                    "ux:card:embedded-boundary-alpha:extension-webview",
                    BoundaryAuthPostureClass::NoAuthClaim,
                    false,
                ),
                claim_row(
                    "boundary-row:marketplace",
                    BoundaryFallbackSurfaceClass::MarketplaceOrAccountContent,
                    "ux:card:embedded-boundary-alpha:marketplace-account",
                    BoundaryAuthPostureClass::NoAuthClaim,
                    false,
                ),
                auth_exception_row(),
            ],
            callback_rows: vec![callback_row()],
            actor_truth_rows: vec![
                actor_row(
                    "actor:human",
                    ApprovalActorClass::HumanAccount,
                    ApprovalAuthSourceClass::HumanSession,
                ),
                actor_row(
                    "actor:install",
                    ApprovalActorClass::InstallationOrAppGrant,
                    ApprovalAuthSourceClass::InstallationGrant,
                ),
                actor_row(
                    "actor:delegated",
                    ApprovalActorClass::DelegatedCredential,
                    ApprovalAuthSourceClass::DelegatedCredential,
                ),
            ],
            failing_then_recovered_examples: vec![BoundaryFallbackRecoveryExample {
                example_id: "recovery:hidden-origin".to_owned(),
                failure_class: BoundaryFailureClass::HiddenOrigin,
                failed_source_ref: "boundary-row:auth-handoff:hidden-origin".to_owned(),
                failed_state_label: "Hidden callback origin blocked before launch.".to_owned(),
                native_remediation_action: BoundaryReviewActionClass::ReapproveNative,
                recovered_source_ref:
                    "native-handoff-review:platform:intent:exact-reopen:auth-callback:consumed:01"
                        .to_owned(),
                recovered_state: BoundaryRecoveryStateClass::RecoveredByNativeReview,
                recovered_by_native_surface: true,
                support_reconstruction_ref: "support:boundary:hidden-origin".to_owned(),
            }],
            minted_at: "2026-05-14T03:20:00Z".to_owned(),
        }
    }

    fn claim_row(
        row_id: &str,
        surface_class: BoundaryFallbackSurfaceClass,
        source_ref: &str,
        auth_posture: BoundaryAuthPostureClass,
        system_browser_default: bool,
    ) -> BoundaryFallbackClaimRow {
        BoundaryFallbackClaimRow {
            row_id: row_id.to_owned(),
            surface_class,
            source_ref: source_ref.to_owned(),
            provider_label: "Acme identity".to_owned(),
            provider_domain_label: "login.acme.example".to_owned(),
            profile_or_org_scope_label: "payments-prod tenant".to_owned(),
            origin_label: "https://login.acme.example/authorize".to_owned(),
            origin_visible_and_verified: true,
            auth_posture,
            system_browser_default,
            embedded_exception: None,
            browser_fallback: BrowserFallbackDisclosure {
                available: true,
                policy_blocked: false,
                fallback_target_class: BoundaryFallbackTargetClass::SystemBrowser,
                preserves_object_identity: true,
                exact_target_reopen: true,
                truthful_placeholder_recovery: false,
                handoff_ref: Some("browser-handoff:managed:payments-prod".to_owned()),
            },
            native_approval: NativeApprovalDisclosure {
                owner_class: NativeApprovalOwnerClass::ProductOwnedNative,
                high_risk_approval_product_owned: true,
                destructive_confirmation_product_owned: true,
                trust_elevation_product_owned: true,
                approval_ticket_or_scope_ref: Some(
                    "approval_ticket.alpha.code_host.comment.human.0001".to_owned(),
                ),
                native_reapproval_route_ref: Some(
                    "native-reapproval:auth:payments-prod".to_owned(),
                ),
            },
            chrome_continuity: ChromeContinuityDisclosure {
                theme_continuity: true,
                zoom_continuity: true,
                focus_continuity: true,
                reduced_motion_continuity: true,
            },
            privacy_handoff: PrivacyHandoffDisclosure {
                metadata_safe_handoff: true,
                raw_url_or_token_exposed: false,
                support_export_origin_reconstructable: true,
                support_reconstruction_ref: "support:boundary:row".to_owned(),
            },
            support_summary: "Boundary row stays product-owned and browser-fallback safe."
                .to_owned(),
        }
    }

    fn auth_exception_row() -> BoundaryFallbackClaimRow {
        let mut row = claim_row(
            "boundary-row:auth-handoff",
            BoundaryFallbackSurfaceClass::AuthHandoff,
            "auth-handoff:embedded-session-refresh:exception",
            BoundaryAuthPostureClass::EmbeddedException,
            false,
        );
        row.embedded_exception = Some(EmbeddedExceptionDisclosure {
            exception_ref: "embedded-auth-exception:session-refresh:legacy-idp".to_owned(),
            provider_domain_label: "login.legacy-idp.example".to_owned(),
            justification_label:
                "Legacy IdP session refresh cannot leave the managed browser profile.".to_owned(),
            fallback_target_class: BoundaryFallbackTargetClass::DeviceCode,
            fallback_handoff_ref: "device-code:legacy-idp:refresh".to_owned(),
            lower_trust_cue_visible: true,
        });
        row.browser_fallback.fallback_target_class = BoundaryFallbackTargetClass::DeviceCode;
        row.browser_fallback.handoff_ref = Some("device-code:legacy-idp:refresh".to_owned());
        row
    }

    fn callback_row() -> BoundaryCallbackReviewRow {
        BoundaryCallbackReviewRow {
            callback_id: "callback-review:auth-replay".to_owned(),
            source_ref:
                "native-handoff-review:platform:intent:exact-reopen:auth-callback:consumed:01"
                    .to_owned(),
            source_surface_token: "default_browser_callback".to_owned(),
            origin_class_token: "system_default_browser".to_owned(),
            origin_label: "System default browser return from login.acme.example".to_owned(),
            origin_visible_and_verified: true,
            requested_action_token: "auth_return".to_owned(),
            target_scope_ref: "scope:auth:oidc".to_owned(),
            channel_owner_ref: "channel:stable".to_owned(),
            build_owner_ref: "build:aureline:stable:2026-05-01".to_owned(),
            confirm_action: BoundaryReviewActionClass::RestartBrowserHandoff,
            reject_action: BoundaryReviewActionClass::RejectCancel,
            broader_than_plain_local_open: true,
            native_review_required: true,
            exact_target_reopen: false,
            truthful_placeholder_recovery: true,
            interruption_class: CallbackInterruptionClass::CallbackReplay,
            stale_or_replayed_callback_denied: true,
            authority_or_target_changed_while_unfocused: true,
            native_reapproval_required: true,
            exact_origin_explanation_ref: "origin-explain:auth-callback:replay".to_owned(),
            support_export_ref: "support:auth-callback:replay".to_owned(),
        }
    }

    fn actor_row(
        actor_row_id: &str,
        actor_class: ApprovalActorClass,
        auth_source_class: ApprovalAuthSourceClass,
    ) -> BoundaryActorTruthRow {
        BoundaryActorTruthRow {
            actor_row_id: actor_row_id.to_owned(),
            actor_class,
            auth_source_class,
            source_authority_ref: format!("authority:{actor_row_id}"),
            display_label: format!("display:{actor_row_id}"),
            support_export_ref: format!("support:{actor_row_id}"),
            generic_signed_in_collapse_prevented: true,
        }
    }

    #[test]
    fn valid_packet_covers_boundary_fallback_requirements() {
        let packet = valid_packet();
        let report = packet.validate();
        assert!(report.passed, "{:#?}", report.findings);
        assert!(report.coverage.has_system_browser_default);
        assert!(report.coverage.has_native_reapproval);
        assert!(report
            .coverage
            .surface_classes
            .contains(&BoundaryFallbackSurfaceClass::AuthHandoff));
        assert!(report
            .coverage
            .actor_classes
            .contains(&ApprovalActorClass::DelegatedCredential));
    }

    #[test]
    fn embedded_approval_ownership_is_rejected() {
        let mut packet = valid_packet();
        packet.claimed_rows[0].native_approval.owner_class =
            NativeApprovalOwnerClass::EmbeddedOwnedDenied;
        packet.claimed_rows[0]
            .native_approval
            .high_risk_approval_product_owned = false;
        let report = packet.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "boundary_fallback.native_approval_owner_invalid"
        }));
    }

    #[test]
    fn interrupted_callback_without_reapproval_is_rejected() {
        let mut packet = valid_packet();
        packet.callback_rows[0].native_reapproval_required = false;
        let report = packet.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "boundary_fallback.interrupted_callback_reapproval_missing"
        }));
    }

    #[test]
    fn claimed_auth_row_without_system_browser_or_exception_is_rejected() {
        let mut packet = valid_packet();
        packet.claimed_rows[0].system_browser_default = false;
        packet.claimed_rows[0].embedded_exception = None;
        let report = packet.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "boundary_fallback.system_browser_default_missing"
        }));
    }
}
