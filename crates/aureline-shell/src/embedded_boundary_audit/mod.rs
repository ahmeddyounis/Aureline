//! Beta projection for the embedded-surface boundary audit.
//!
//! This module promotes the alpha embedded boundary card seed to a
//! page-level audit that proves, on every claimed embedded surface
//! (docs/help, extension webview, marketplace/account, service
//! dashboard, auth confirmation), that:
//!
//! 1. **Owner / origin / publisher disclosure.** Owner label, owner
//!    class, publisher/service label, origin host, origin class, and
//!    origin verification token all appear on the row — never hidden
//!    behind hover, scroll, or the embedded body.
//! 2. **Trust class disclosure.** Boundary state, data boundary class,
//!    permission class, identity mode, and trust state are quoted in
//!    the same vocabulary the live shell row paints.
//! 3. **Handoff rules disclosure.** Browser-fallback posture and
//!    fallback-target class are quoted so support and privacy review
//!    can see exactly which path the user takes when an embedded
//!    surface cannot be trusted.
//! 4. **System-browser baseline on identity & risky web flows.** Auth
//!    confirmation and risky web-owned surfaces (marketplace/account,
//!    extension webview, service dashboard) MUST quote a posture token
//!    from the closed safe-baseline set: `system_browser_first`,
//!    `device_code_fallback_offered`, `external_open_blocked_by_policy`,
//!    or `external_open_unavailable_offline`. The validator rejects
//!    `browser_fallback_not_applicable` on those rows.
//! 5. **Host-owned high-risk approvals.** All six native-reserved
//!    surfaces (security messaging, update verification, workspace
//!    trust elevation, rollback/restore confirmation, AI apply review,
//!    high-risk approval sheet) remain on the card's
//!    `reserved_native_surfaces_host_owned` set.
//! 6. **Support-export vocabulary parity.** The support row reuses the
//!    same surface family, boundary state, permission class, fallback
//!    posture, and handoff packet ref the live row paints. Drift is a
//!    contract bug.
//!
//! The page yields a typed [`EmbeddedBoundaryAuditDefect`] list. The
//! seeded page seeds zero defects; the validator and the headless
//! inspector are what surface a regression when a row drops a required
//! field, drifts vocabulary across the live row and the support row,
//! or weakens the system-browser baseline on a risky surface.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::embedded::boundary_alpha::{
    seeded_embedded_boundary_alpha_snapshot, EmbeddedBoundaryAlphaSnapshot,
    EmbeddedBoundaryAlphaSupportRow, EmbeddedBoundaryAlphaSurfaceRow,
};
use crate::embedded::boundary_card::{
    ActionPartitionRecord, ActionPartitionRole, AuthFlowClass, AuthHandoffCardRecord,
    BoundaryActionId, BoundaryState, BrowserFallbackPostureClass, BrowserFallbackRecord,
    CapabilityLimitation, ChromeInheritanceAxis, DataBoundaryClass, EmbeddedBoundaryCardRecord,
    FallbackTargetClass, IdentityMode, LayoutConstraintId, NativeReservedSurface, OriginClass,
    OriginIdentityRecord, OriginVerificationState, OwnerClass, OwnerIdentityRecord,
    PermissionClass, PermissionStateRecord, PolicyContext, ProviderActorClass, ProviderClass,
    ProviderHealthState, ProviderIdentityRecord, PublisherOrServiceClass,
    PublisherOrServiceIdentityRecord, RedactionClass, SurfaceFamily, TrustState,
};

/// Beta schema version exported with every record.
pub const EMBEDDED_BOUNDARY_AUDIT_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record on the page.
pub const EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF: &str =
    "shell:embedded_boundary_audit_beta:v1";

/// Stable record kind for [`EmbeddedBoundaryAuditPage`] payloads.
pub const EMBEDDED_BOUNDARY_AUDIT_BETA_PAGE_RECORD_KIND: &str =
    "shell_embedded_boundary_audit_beta_page_record";

/// Stable record kind for [`EmbeddedBoundaryAuditRow`] payloads.
pub const EMBEDDED_BOUNDARY_AUDIT_BETA_ROW_RECORD_KIND: &str =
    "shell_embedded_boundary_audit_beta_row_record";

/// Stable record kind for [`EmbeddedBoundaryAuditDefect`] payloads.
pub const EMBEDDED_BOUNDARY_AUDIT_BETA_DEFECT_RECORD_KIND: &str =
    "shell_embedded_boundary_audit_beta_defect_record";

/// Stable record kind for [`EmbeddedBoundaryAuditSupportRow`] payloads.
pub const EMBEDDED_BOUNDARY_AUDIT_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "shell_embedded_boundary_audit_beta_support_row_record";

/// Stable record kind for [`EmbeddedBoundaryAuditSupportExport`] payloads.
pub const EMBEDDED_BOUNDARY_AUDIT_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_embedded_boundary_audit_beta_support_export_record";

/// Closed semantic-axis vocabulary the audit verifies per row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedBoundaryAuditAxis {
    /// Owner label, owner class, publisher/service label, origin host,
    /// origin class, and origin verification token are all quoted.
    OwnerOriginPublisherDisclosed,
    /// Boundary state, data boundary class, permission class, identity
    /// mode, and trust state are quoted in the row.
    TrustClassDisclosed,
    /// Browser-fallback posture and target class are quoted; when a
    /// browser handoff is offered the packet ref is preserved.
    HandoffRulesDisclosed,
    /// Identity rows and risky web-owned rows quote a baseline posture
    /// from the safe set (system-browser first, device-code fallback,
    /// policy blocked, or offline blocked).
    SystemBrowserBaselineForIdentityOrRiskyWeb,
    /// All six native-reserved surfaces remain on the row's
    /// `reserved_native_surfaces_host_owned` set.
    HostOwnedHighRiskApproval,
    /// The support row reuses the same closed-vocabulary tokens as the
    /// live row.
    SupportExportVocabularyParity,
}

impl EmbeddedBoundaryAuditAxis {
    /// Stable schema token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOriginPublisherDisclosed => "owner_origin_publisher_disclosed",
            Self::TrustClassDisclosed => "trust_class_disclosed",
            Self::HandoffRulesDisclosed => "handoff_rules_disclosed",
            Self::SystemBrowserBaselineForIdentityOrRiskyWeb => {
                "system_browser_baseline_for_identity_or_risky_web"
            }
            Self::HostOwnedHighRiskApproval => "host_owned_high_risk_approval",
            Self::SupportExportVocabularyParity => "support_export_vocabulary_parity",
        }
    }
}

/// Closed defect vocabulary the audit emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedBoundaryAuditDefectKind {
    /// Row did not quote an owner label.
    MissingOwnerLabel,
    /// Row did not quote a publisher or service label.
    MissingPublisherOrServiceLabel,
    /// Row did not quote an origin host or domain label.
    MissingOriginHostLabel,
    /// Row did not quote a boundary-state token from the closed set.
    MissingBoundaryStateToken,
    /// Row did not quote a permission-class token from the closed set.
    MissingPermissionClassToken,
    /// Row did not quote a browser-fallback posture token.
    MissingBrowserFallbackPostureToken,
    /// Row offered a host-owned `open_in_system_browser` action but did
    /// not quote a handoff packet ref.
    MissingBrowserHandoffPacketRef,
    /// Identity row (auth confirmation, marketplace/account, extension
    /// webview, service dashboard) used the
    /// `browser_fallback_not_applicable` posture instead of one of the
    /// four safe-baseline postures.
    SystemBrowserNotBaselineOnIdentityOrRiskyWeb,
    /// Row dropped one of the six native-reserved high-risk approval
    /// surfaces from `reserved_native_surfaces_host_owned`.
    EmbeddedMintedNativeReservedSurface,
    /// Support row's surface family, boundary state, permission class,
    /// fallback posture, owner label, host or domain label, or handoff
    /// packet ref drifted from the live row.
    SupportRowVocabularyDrift,
    /// Row's `boundary_state` is `live_verified` while the origin
    /// verification token is anything other than `verified` (or vice
    /// versa for `policy_blocked` / `certificate_failed`).
    BoundaryStateInconsistentWithOriginVerification,
    /// Auth-confirmation row is missing an `auth_handoff` block or its
    /// flow_class is `not_applicable`.
    AuthConfirmationMissingFlowClass,
    /// Embedded auth-confirmation row used a flow class that mints
    /// credentials inside the embedded body
    /// (`embedded_password_exception`) without naming an exception ref.
    EmbeddedAuthExceptionMissingExceptionRef,
}

impl EmbeddedBoundaryAuditDefectKind {
    /// Stable schema token recorded on the defect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingOwnerLabel => "missing_owner_label",
            Self::MissingPublisherOrServiceLabel => "missing_publisher_or_service_label",
            Self::MissingOriginHostLabel => "missing_origin_host_label",
            Self::MissingBoundaryStateToken => "missing_boundary_state_token",
            Self::MissingPermissionClassToken => "missing_permission_class_token",
            Self::MissingBrowserFallbackPostureToken => "missing_browser_fallback_posture_token",
            Self::MissingBrowserHandoffPacketRef => "missing_browser_handoff_packet_ref",
            Self::SystemBrowserNotBaselineOnIdentityOrRiskyWeb => {
                "system_browser_not_baseline_on_identity_or_risky_web"
            }
            Self::EmbeddedMintedNativeReservedSurface => {
                "embedded_minted_native_reserved_surface"
            }
            Self::SupportRowVocabularyDrift => "support_row_vocabulary_drift",
            Self::BoundaryStateInconsistentWithOriginVerification => {
                "boundary_state_inconsistent_with_origin_verification"
            }
            Self::AuthConfirmationMissingFlowClass => "auth_confirmation_missing_flow_class",
            Self::EmbeddedAuthExceptionMissingExceptionRef => {
                "embedded_auth_exception_missing_exception_ref"
            }
        }
    }
}

/// Typed defect emitted by the audit validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAuditDefect {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub defect_id: String,
    pub defect_kind: EmbeddedBoundaryAuditDefectKind,
    pub defect_kind_token: String,
    pub surface_family: SurfaceFamily,
    pub surface_family_token: String,
    pub row_id: String,
    pub field: String,
    pub note: String,
}

impl EmbeddedBoundaryAuditDefect {
    fn new(
        defect_kind: EmbeddedBoundaryAuditDefectKind,
        surface_family: SurfaceFamily,
        row_id: &str,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: EMBEDDED_BOUNDARY_AUDIT_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_AUDIT_BETA_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "ux:defect:embedded-boundary-audit:{}:{}",
                defect_kind.as_str(),
                row_id
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            surface_family,
            surface_family_token: surface_family_token(surface_family).to_owned(),
            row_id: row_id.to_owned(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Audited row for one embedded boundary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAuditRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub row_id: String,
    pub source_card_id: String,
    pub surface_family: SurfaceFamily,
    pub surface_family_token: String,

    // Owner / origin / publisher disclosure
    pub owner_label: String,
    pub owner_class_token: String,
    pub publisher_or_service_label: String,
    pub publisher_or_service_class_token: String,
    pub origin_label: String,
    pub host_or_domain_label: String,
    pub origin_class_token: String,
    pub origin_verification_token: String,

    // Trust class disclosure
    pub data_boundary_class_token: String,
    pub data_boundary_label: String,
    pub boundary_state_token: String,
    pub boundary_state_label: String,
    pub permission_class_token: String,
    pub permission_label: String,
    pub identity_mode_token: String,
    pub trust_state_token: String,

    // Handoff rules disclosure
    pub browser_fallback_posture_token: String,
    pub fallback_target_class_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_target_label: Option<String>,

    // Optional auth-handoff block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_flow_class_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_provider_domain_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_exception_id_ref: Option<String>,

    // Optional provider block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_class_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_health_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_scope_label: Option<String>,

    // Host-owned high-risk approval surfaces present on the row
    pub native_reserved_surface_tokens: Vec<String>,
    pub capability_limitation_tokens: Vec<String>,
    pub layout_constraint_tokens: Vec<String>,
    pub chrome_inheritance_tokens: Vec<String>,

    pub promised_audit_axes: Vec<EmbeddedBoundaryAuditAxis>,
    pub plain_language_summary: String,
    pub redaction_class_token: String,
}

/// Export-safe support row for the audit page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAuditSupportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub row_id: String,
    pub surface_family_token: String,
    pub owner_label: String,
    pub host_or_domain_label: String,
    pub data_boundary_class_token: String,
    pub boundary_state_token: String,
    pub permission_class_token: String,
    pub browser_fallback_posture_token: String,
    pub fallback_target_class_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    pub identity_mode_token: String,
    pub trust_state_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_flow_class_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_health_token: Option<String>,
    pub native_reserved_surface_tokens: Vec<String>,
    pub redaction_class_token: String,
}

/// Aggregate banner emitted with the page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAuditSummary {
    pub row_count: usize,
    pub support_row_count: usize,
    pub surface_family_count: usize,
    pub defect_count: usize,
    pub identity_or_risky_row_count: usize,
    pub system_browser_first_row_count: usize,
    pub host_owned_high_risk_approval_row_count: usize,
    pub surface_families_present: Vec<SurfaceFamily>,
    pub boundary_states_present: Vec<BoundaryState>,
    pub permission_classes_present: Vec<PermissionClass>,
    pub fallback_postures_present: Vec<BrowserFallbackPostureClass>,
}

impl EmbeddedBoundaryAuditSummary {
    fn from_rows(
        rows: &[EmbeddedBoundaryAuditRow],
        support_rows: &[EmbeddedBoundaryAuditSupportRow],
        defects: &[EmbeddedBoundaryAuditDefect],
    ) -> Self {
        let mut families: Vec<SurfaceFamily> = Vec::new();
        let mut states: Vec<BoundaryState> = Vec::new();
        let mut perms: Vec<PermissionClass> = Vec::new();
        let mut postures: Vec<BrowserFallbackPostureClass> = Vec::new();
        let mut identity_or_risky = 0usize;
        let mut system_first = 0usize;
        let mut host_owned = 0usize;
        for row in rows {
            if !families.contains(&row.surface_family) {
                families.push(row.surface_family);
            }
            if let Some(state) = boundary_state_from_token(&row.boundary_state_token) {
                if !states.contains(&state) {
                    states.push(state);
                }
            }
            if let Some(class) = permission_class_from_token(&row.permission_class_token) {
                if !perms.contains(&class) {
                    perms.push(class);
                }
            }
            if let Some(posture) =
                browser_fallback_posture_from_token(&row.browser_fallback_posture_token)
            {
                if !postures.contains(&posture) {
                    postures.push(posture);
                }
            }
            if surface_is_identity_or_risky(row.surface_family) {
                identity_or_risky += 1;
            }
            if row.browser_fallback_posture_token == "system_browser_first" {
                system_first += 1;
            }
            if row.native_reserved_surface_tokens.len() == required_native_reserved_count() {
                host_owned += 1;
            }
        }
        Self {
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            surface_family_count: families.len(),
            defect_count: defects.len(),
            identity_or_risky_row_count: identity_or_risky,
            system_browser_first_row_count: system_first,
            host_owned_high_risk_approval_row_count: host_owned,
            surface_families_present: families,
            boundary_states_present: states,
            permission_classes_present: perms,
            fallback_postures_present: postures,
        }
    }
}

/// Top-level beta audit page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAuditPage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_id: String,
    pub page_label: String,
    pub generated_at: String,
    pub summary: EmbeddedBoundaryAuditSummary,
    pub rows: Vec<EmbeddedBoundaryAuditRow>,
    pub support_rows: Vec<EmbeddedBoundaryAuditSupportRow>,
    pub defects: Vec<EmbeddedBoundaryAuditDefect>,
}

impl EmbeddedBoundaryAuditPage {
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<EmbeddedBoundaryAuditRow>,
        support_rows: Vec<EmbeddedBoundaryAuditSupportRow>,
    ) -> Self {
        let defects = audit_rows(&rows, &support_rows);
        let summary = EmbeddedBoundaryAuditSummary::from_rows(&rows, &support_rows, &defects);
        Self {
            record_kind: EMBEDDED_BOUNDARY_AUDIT_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_AUDIT_BETA_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            support_rows,
            defects,
        }
    }

    /// True when every claimed surface family has at least one row.
    pub fn covers_required_surface_families(&self) -> bool {
        [
            SurfaceFamily::EmbeddedDocsHelp,
            SurfaceFamily::ExtensionHostedSurface,
            SurfaceFamily::EmbeddedMarketplaceOrAccount,
            SurfaceFamily::EmbeddedServiceDashboard,
            SurfaceFamily::EmbeddedAuthConfirmation,
        ]
        .iter()
        .all(|f| self.summary.surface_families_present.contains(f))
    }
}

/// Support-export wrapper that quotes the audited page plus a
/// metadata-safe defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryAuditSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub page: EmbeddedBoundaryAuditPage,
    pub defect_kinds_present: Vec<EmbeddedBoundaryAuditDefectKind>,
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    pub raw_private_material_excluded: bool,
}

impl EmbeddedBoundaryAuditSupportExport {
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: EmbeddedBoundaryAuditPage,
    ) -> Self {
        let mut kinds: Vec<EmbeddedBoundaryAuditDefectKind> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !kinds.contains(&defect.defect_kind) {
                kinds.push(defect.defect_kind);
            }
            *counts.entry(defect.defect_kind_token.clone()).or_insert(0) += 1;
        }
        kinds.sort();
        Self {
            record_kind: EMBEDDED_BOUNDARY_AUDIT_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_AUDIT_BETA_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            defect_kinds_present: kinds,
            defect_counts_by_kind: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Closed safe-baseline posture set for identity & risky web rows.
fn safe_baseline_posture_tokens() -> [&'static str; 4] {
    [
        "system_browser_first",
        "device_code_fallback_offered",
        "external_open_blocked_by_policy",
        "external_open_unavailable_offline",
    ]
}

fn surface_is_identity_or_risky(family: SurfaceFamily) -> bool {
    matches!(
        family,
        SurfaceFamily::EmbeddedAuthConfirmation
            | SurfaceFamily::EmbeddedMarketplaceOrAccount
            | SurfaceFamily::EmbeddedServiceDashboard
            | SurfaceFamily::ExtensionHostedSurface
    )
}

fn required_native_reserved_count() -> usize {
    6
}

/// Runs the audit validator over the page rows and support rows.
pub fn audit_rows(
    rows: &[EmbeddedBoundaryAuditRow],
    support_rows: &[EmbeddedBoundaryAuditSupportRow],
) -> Vec<EmbeddedBoundaryAuditDefect> {
    let mut defects: Vec<EmbeddedBoundaryAuditDefect> = Vec::new();
    let support_index: BTreeMap<&str, &EmbeddedBoundaryAuditSupportRow> = support_rows
        .iter()
        .map(|row| (row.row_id.as_str(), row))
        .collect();

    for row in rows {
        if row.owner_label.trim().is_empty() {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::MissingOwnerLabel,
                row.surface_family,
                &row.row_id,
                "owner_label",
                "row did not quote an owner label",
            ));
        }
        if row.publisher_or_service_label.trim().is_empty() {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::MissingPublisherOrServiceLabel,
                row.surface_family,
                &row.row_id,
                "publisher_or_service_label",
                "row did not quote a publisher or service label",
            ));
        }
        if row.host_or_domain_label.trim().is_empty() {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::MissingOriginHostLabel,
                row.surface_family,
                &row.row_id,
                "host_or_domain_label",
                "row did not quote an origin host or domain label",
            ));
        }
        if boundary_state_from_token(&row.boundary_state_token).is_none() {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::MissingBoundaryStateToken,
                row.surface_family,
                &row.row_id,
                "boundary_state_token",
                "row did not quote a boundary-state token from the closed set",
            ));
        }
        if permission_class_from_token(&row.permission_class_token).is_none() {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::MissingPermissionClassToken,
                row.surface_family,
                &row.row_id,
                "permission_class_token",
                "row did not quote a permission-class token from the closed set",
            ));
        }
        if browser_fallback_posture_from_token(&row.browser_fallback_posture_token).is_none() {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::MissingBrowserFallbackPostureToken,
                row.surface_family,
                &row.row_id,
                "browser_fallback_posture_token",
                "row did not quote a browser-fallback posture token from the closed set",
            ));
        }

        // Identity or risky-web baseline.
        if surface_is_identity_or_risky(row.surface_family) {
            let baseline = safe_baseline_posture_tokens();
            if !baseline.contains(&row.browser_fallback_posture_token.as_str()) {
                defects.push(EmbeddedBoundaryAuditDefect::new(
                    EmbeddedBoundaryAuditDefectKind::SystemBrowserNotBaselineOnIdentityOrRiskyWeb,
                    row.surface_family,
                    &row.row_id,
                    "browser_fallback_posture_token",
                    "identity or risky web row did not quote a safe-baseline posture token",
                ));
            }
        }

        // Browser handoff packet ref must accompany system_browser_first when offered.
        if row.browser_fallback_posture_token == "system_browser_first"
            && row
                .browser_handoff_packet_ref
                .as_deref()
                .map(str::is_empty)
                .unwrap_or(true)
        {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::MissingBrowserHandoffPacketRef,
                row.surface_family,
                &row.row_id,
                "browser_handoff_packet_ref",
                "system_browser_first posture must quote a browser handoff packet ref",
            ));
        }

        // Native reserved high-risk surfaces must remain host-owned.
        let required_set = required_native_reserved_surface_tokens();
        for required in required_set {
            if !row.native_reserved_surface_tokens.iter().any(|t| t == required) {
                defects.push(EmbeddedBoundaryAuditDefect::new(
                    EmbeddedBoundaryAuditDefectKind::EmbeddedMintedNativeReservedSurface,
                    row.surface_family,
                    &row.row_id,
                    "native_reserved_surface_tokens",
                    format!(
                        "row dropped required native-reserved surface `{required}` from host-owned set"
                    ),
                ));
                break;
            }
        }

        // Boundary state vs origin verification consistency.
        let inconsistent = match (
            row.boundary_state_token.as_str(),
            row.origin_verification_token.as_str(),
        ) {
            ("live_verified", v) if v != "verified" => true,
            ("policy_blocked", v) if v != "policy_blocked" => true,
            ("certificate_failed", v) if v != "certificate_failed" => true,
            ("cross_origin_limited", v) if v != "cross_origin_limited" => true,
            _ => false,
        };
        if inconsistent {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::BoundaryStateInconsistentWithOriginVerification,
                row.surface_family,
                &row.row_id,
                "boundary_state_token",
                "boundary state disagrees with the origin verification token",
            ));
        }

        // Auth-confirmation rows must declare an auth-handoff flow class.
        if row.surface_family == SurfaceFamily::EmbeddedAuthConfirmation {
            match row.auth_flow_class_token.as_deref() {
                None | Some("not_applicable") | Some("") => {
                    defects.push(EmbeddedBoundaryAuditDefect::new(
                        EmbeddedBoundaryAuditDefectKind::AuthConfirmationMissingFlowClass,
                        row.surface_family,
                        &row.row_id,
                        "auth_flow_class_token",
                        "auth-confirmation row must declare an auth_handoff flow class",
                    ));
                }
                Some("embedded_password_exception") => {
                    if row
                        .auth_exception_id_ref
                        .as_deref()
                        .map(str::is_empty)
                        .unwrap_or(true)
                    {
                        defects.push(EmbeddedBoundaryAuditDefect::new(
                            EmbeddedBoundaryAuditDefectKind::EmbeddedAuthExceptionMissingExceptionRef,
                            row.surface_family,
                            &row.row_id,
                            "auth_exception_id_ref",
                            "embedded_password_exception flow must name an exception_id_ref",
                        ));
                    }
                }
                _ => {}
            }
        }

        // Support-row vocabulary parity.
        if let Some(support) = support_index.get(row.row_id.as_str()) {
            let drift = support.surface_family_token != row.surface_family_token
                || support.boundary_state_token != row.boundary_state_token
                || support.permission_class_token != row.permission_class_token
                || support.browser_fallback_posture_token != row.browser_fallback_posture_token
                || support.fallback_target_class_token != row.fallback_target_class_token
                || support.owner_label != row.owner_label
                || support.host_or_domain_label != row.host_or_domain_label
                || support.identity_mode_token != row.identity_mode_token
                || support.trust_state_token != row.trust_state_token
                || support.browser_handoff_packet_ref != row.browser_handoff_packet_ref;
            if drift {
                defects.push(EmbeddedBoundaryAuditDefect::new(
                    EmbeddedBoundaryAuditDefectKind::SupportRowVocabularyDrift,
                    row.surface_family,
                    &row.row_id,
                    "support_row",
                    "support row vocabulary drifted from the live row",
                ));
            }
        } else {
            defects.push(EmbeddedBoundaryAuditDefect::new(
                EmbeddedBoundaryAuditDefectKind::SupportRowVocabularyDrift,
                row.surface_family,
                &row.row_id,
                "support_row",
                "no support row matched the live row id",
            ));
        }
    }

    defects
}

/// Validates that the seeded page seeds zero defects.
pub fn validate_embedded_boundary_audit_page(
    page: &EmbeddedBoundaryAuditPage,
) -> Result<(), Vec<EmbeddedBoundaryAuditDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Builds the seeded beta audit page.
///
/// Loads the alpha embedded boundary snapshot for docs/help, extension
/// webview, and marketplace/account, then layers two additional rows
/// (service dashboard policy-blocked, auth confirmation system-browser
/// first) so all five surface families are covered.
pub fn seeded_embedded_boundary_audit_page() -> EmbeddedBoundaryAuditPage {
    let build_id = "id:build:embedded-boundary-audit-beta:seed";
    let alpha = seeded_embedded_boundary_alpha_snapshot(build_id);

    let mut audited_cards: Vec<(String, EmbeddedBoundaryCardRecord, EmbeddedBoundaryAlphaSurfaceRow, EmbeddedBoundaryAlphaSupportRow)> = Vec::new();
    let alpha_cards = alpha_card_rebuild(&alpha);
    for ((surface_row, support_row), card) in alpha
        .surface_rows
        .iter()
        .cloned()
        .zip(alpha.support_rows.iter().cloned())
        .zip(alpha_cards.into_iter())
    {
        let case_id = match surface_row.surface_family {
            SurfaceFamily::EmbeddedDocsHelp => {
                "embedded_boundary_audit_beta:docs_help_live_verified"
            }
            SurfaceFamily::ExtensionHostedSurface => {
                "embedded_boundary_audit_beta:extension_webview_cross_origin_limited"
            }
            SurfaceFamily::EmbeddedMarketplaceOrAccount => {
                "embedded_boundary_audit_beta:marketplace_account_stale_session"
            }
            SurfaceFamily::EmbeddedServiceDashboard => {
                "embedded_boundary_audit_beta:service_dashboard"
            }
            SurfaceFamily::EmbeddedAuthConfirmation => {
                "embedded_boundary_audit_beta:auth_confirmation"
            }
        };
        audited_cards.push((case_id.to_owned(), card, surface_row, support_row));
    }

    // Service dashboard (policy blocked) and auth confirmation
    // (system-browser first) cards are inline-built so the audit covers
    // all five claimed embedded surface families.
    audited_cards.push(seed_service_dashboard_policy_blocked_audit_row());
    audited_cards.push(seed_auth_confirmation_system_browser_first_audit_row());

    let mut rows: Vec<EmbeddedBoundaryAuditRow> = Vec::new();
    let mut support_rows: Vec<EmbeddedBoundaryAuditSupportRow> = Vec::new();

    for (case_id, card, surface_row, support_row) in audited_cards {
        let row_id = format!("ux:embedded-boundary-audit:beta:{}", case_id);
        let audited = build_audit_row(case_id.clone(), row_id.clone(), &card, &surface_row);
        let audit_support = build_support_row(case_id, row_id, &card, &support_row);
        rows.push(audited);
        support_rows.push(audit_support);
    }

    EmbeddedBoundaryAuditPage::new(
        "shell:embedded-boundary-audit:beta:page:default",
        "Embedded boundary audit (beta): owner / origin / trust / handoff disclosure across docs/help, extension webviews, marketplace/account, service dashboards, and auth-handoff surfaces",
        "2026-05-15T00:00:00Z",
        rows,
        support_rows,
    )
}

fn alpha_card_rebuild(alpha: &EmbeddedBoundaryAlphaSnapshot) -> Vec<EmbeddedBoundaryCardRecord> {
    // The alpha snapshot exposes derived rows but the boundary cards
    // themselves are still required for permission labels, action
    // partition introspection, and provider-identity passthrough used
    // by the audit row. We rebuild the cards from the seed by calling
    // the same fixture loaders the alpha snapshot used. Keeping this
    // local avoids re-parsing JSON on the audit hot path; the alpha
    // snapshot also exposes a small number of cards.
    use crate::embedded::docs_help::seeded_docs_help_boundary_card;
    let _ = alpha; // alpha is kept for symmetry; the fixture loaders are deterministic.
    let mut cards: Vec<EmbeddedBoundaryCardRecord> = Vec::new();
    cards.push(seeded_docs_help_boundary_card("id:build:embedded-boundary-audit-beta:seed"));
    cards.push(rebuild_extension_webview_alpha_card());
    cards.push(rebuild_marketplace_account_alpha_card());
    cards
}

fn rebuild_extension_webview_alpha_card() -> EmbeddedBoundaryCardRecord {
    parse_card_fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ux/embedded_boundary_alpha/extension_webview_alpha_card.json"
    )))
}

fn rebuild_marketplace_account_alpha_card() -> EmbeddedBoundaryCardRecord {
    parse_card_fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ux/embedded_boundary_alpha/marketplace_account_alpha_card.json"
    )))
}

fn parse_card_fixture(payload: &str) -> EmbeddedBoundaryCardRecord {
    let value: serde_json::Value =
        serde_json::from_str(payload).expect("embedded boundary audit fixture must parse");
    serde_json::from_value(value)
        .expect("embedded boundary audit fixture must match boundary card shape")
}

fn build_audit_row(
    case_id: String,
    row_id: String,
    card: &EmbeddedBoundaryCardRecord,
    surface_row: &EmbeddedBoundaryAlphaSurfaceRow,
) -> EmbeddedBoundaryAuditRow {
    let auth_block = card.auth_handoff.as_ref();
    let provider_block = card.provider_identity.as_ref();
    let promised = vec![
        EmbeddedBoundaryAuditAxis::OwnerOriginPublisherDisclosed,
        EmbeddedBoundaryAuditAxis::TrustClassDisclosed,
        EmbeddedBoundaryAuditAxis::HandoffRulesDisclosed,
        EmbeddedBoundaryAuditAxis::SystemBrowserBaselineForIdentityOrRiskyWeb,
        EmbeddedBoundaryAuditAxis::HostOwnedHighRiskApproval,
        EmbeddedBoundaryAuditAxis::SupportExportVocabularyParity,
    ];

    EmbeddedBoundaryAuditRow {
        record_kind: EMBEDDED_BOUNDARY_AUDIT_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: EMBEDDED_BOUNDARY_AUDIT_BETA_SCHEMA_VERSION,
        shared_contract_ref: EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id,
        row_id,
        source_card_id: card.card_id.clone(),
        surface_family: card.surface_family,
        surface_family_token: surface_row.surface_family_token.clone(),

        owner_label: card.owner_identity.label.clone(),
        owner_class_token: owner_class_token(card.owner_identity.class).to_owned(),
        publisher_or_service_label: card.publisher_or_service_identity.label.clone(),
        publisher_or_service_class_token: publisher_or_service_class_token(
            card.publisher_or_service_identity.class,
        )
        .to_owned(),
        origin_label: card.origin_identity.origin_label.clone(),
        host_or_domain_label: card.origin_identity.host_or_domain_label.clone(),
        origin_class_token: origin_class_token(card.origin_identity.origin_class).to_owned(),
        origin_verification_token: surface_row.origin_verification_token.clone(),

        data_boundary_class_token: surface_row.data_boundary_class_token.clone(),
        data_boundary_label: card.data_boundary_label.clone(),
        boundary_state_token: surface_row.boundary_state_token.clone(),
        boundary_state_label: card.boundary_state_label.clone(),
        permission_class_token: surface_row.permission_class_token.clone(),
        permission_label: card.permission_state.permission_label.clone(),
        identity_mode_token: identity_mode_token(card.policy_context.identity_mode).to_owned(),
        trust_state_token: trust_state_token(card.policy_context.trust_state).to_owned(),

        browser_fallback_posture_token: surface_row.browser_fallback_posture_token.clone(),
        fallback_target_class_token: surface_row.fallback_target_class_token.clone(),
        browser_handoff_packet_ref: card
            .open_in_browser_action()
            .and_then(|action| action.browser_handoff_packet_ref.clone())
            .or_else(|| card.browser_fallback.browser_handoff_packet_ref.clone()),
        return_target_label: card.browser_fallback.return_target_label.clone(),

        auth_flow_class_token: auth_block
            .map(|block| auth_flow_class_token(block.flow_class).to_owned()),
        auth_provider_domain_label: auth_block.map(|block| block.provider_domain_label.clone()),
        auth_exception_id_ref: auth_block.and_then(|block| block.exception_id_ref.clone()),

        provider_class_token: provider_block
            .map(|block| provider_class_token(block.provider_class).to_owned()),
        provider_health_token: provider_block
            .map(|block| provider_health_token(block.health_state).to_owned()),
        provider_scope_label: provider_block.map(|block| block.provider_scope_label.clone()),

        native_reserved_surface_tokens: surface_row.native_reserved_surface_tokens.clone(),
        capability_limitation_tokens: surface_row.capability_limitation_tokens.clone(),
        layout_constraint_tokens: card
            .layout_constraints
            .iter()
            .copied()
            .map(layout_constraint_token)
            .map(str::to_owned)
            .collect(),
        chrome_inheritance_tokens: surface_row.chrome_inheritance_tokens.clone(),

        promised_audit_axes: promised,
        plain_language_summary: card.plain_language_summary.clone(),
        redaction_class_token: redaction_class_token(card.redaction_class).to_owned(),
    }
}

fn build_support_row(
    case_id: String,
    row_id: String,
    card: &EmbeddedBoundaryCardRecord,
    alpha_support: &EmbeddedBoundaryAlphaSupportRow,
) -> EmbeddedBoundaryAuditSupportRow {
    EmbeddedBoundaryAuditSupportRow {
        record_kind: EMBEDDED_BOUNDARY_AUDIT_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
        schema_version: EMBEDDED_BOUNDARY_AUDIT_BETA_SCHEMA_VERSION,
        shared_contract_ref: EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id,
        row_id,
        surface_family_token: alpha_support.surface_family_token.clone(),
        owner_label: alpha_support.owner_label.clone(),
        host_or_domain_label: alpha_support.host_or_domain_label.clone(),
        data_boundary_class_token: data_boundary_class_token(card.data_boundary_class).to_owned(),
        boundary_state_token: alpha_support.boundary_state_token.clone(),
        permission_class_token: permission_class_token(card.permission_state.permission_class)
            .to_owned(),
        browser_fallback_posture_token: alpha_support.browser_fallback_posture_token.clone(),
        fallback_target_class_token: fallback_target_class_token(
            card.browser_fallback.fallback_target_class,
        )
        .to_owned(),
        browser_handoff_packet_ref: alpha_support.browser_handoff_packet_ref.clone(),
        identity_mode_token: identity_mode_token(card.policy_context.identity_mode).to_owned(),
        trust_state_token: trust_state_token(card.policy_context.trust_state).to_owned(),
        auth_flow_class_token: card
            .auth_handoff
            .as_ref()
            .map(|block| auth_flow_class_token(block.flow_class).to_owned()),
        provider_health_token: card
            .provider_identity
            .as_ref()
            .map(|block| provider_health_token(block.health_state).to_owned()),
        native_reserved_surface_tokens: card
            .reserved_native_surfaces_host_owned
            .iter()
            .copied()
            .map(native_reserved_surface_token)
            .map(str::to_owned)
            .collect(),
        redaction_class_token: redaction_class_token(card.redaction_class).to_owned(),
    }
}

fn seed_service_dashboard_policy_blocked_audit_row() -> (
    String,
    EmbeddedBoundaryCardRecord,
    EmbeddedBoundaryAlphaSurfaceRow,
    EmbeddedBoundaryAlphaSupportRow,
) {
    let card = service_dashboard_policy_blocked_card();
    let surface_row = derive_surface_row(&card);
    let support_row = derive_support_row(&card, &surface_row);
    (
        "embedded_boundary_audit_beta:service_dashboard_policy_blocked".to_owned(),
        card,
        surface_row,
        support_row,
    )
}

fn seed_auth_confirmation_system_browser_first_audit_row() -> (
    String,
    EmbeddedBoundaryCardRecord,
    EmbeddedBoundaryAlphaSurfaceRow,
    EmbeddedBoundaryAlphaSupportRow,
) {
    let card = auth_confirmation_system_browser_first_card();
    let surface_row = derive_surface_row(&card);
    let support_row = derive_support_row(&card, &surface_row);
    (
        "embedded_boundary_audit_beta:auth_confirmation_system_browser_first".to_owned(),
        card,
        surface_row,
        support_row,
    )
}

fn derive_surface_row(card: &EmbeddedBoundaryCardRecord) -> EmbeddedBoundaryAlphaSurfaceRow {
    use crate::embedded::boundary_alpha::materialize_embedded_boundary_alpha_snapshot;
    let snapshot = materialize_embedded_boundary_alpha_snapshot(
        format!("derived:{}", card.card_id),
        "id:build:embedded-boundary-audit-beta:derived",
        vec![card.clone()],
    );
    snapshot.surface_rows.into_iter().next().unwrap()
}

fn derive_support_row(
    card: &EmbeddedBoundaryCardRecord,
    _surface_row: &EmbeddedBoundaryAlphaSurfaceRow,
) -> EmbeddedBoundaryAlphaSupportRow {
    use crate::embedded::boundary_alpha::materialize_embedded_boundary_alpha_snapshot;
    let snapshot = materialize_embedded_boundary_alpha_snapshot(
        format!("derived-support:{}", card.card_id),
        "id:build:embedded-boundary-audit-beta:derived",
        vec![card.clone()],
    );
    snapshot.support_rows.into_iter().next().unwrap()
}

fn service_dashboard_policy_blocked_card() -> EmbeddedBoundaryCardRecord {
    EmbeddedBoundaryCardRecord {
        record_kind: "embedded_boundary_card_record".to_owned(),
        embedded_boundary_card_schema_version: 1,
        card_id: "ux:card:embedded-boundary-audit:service-dashboard:policy-blocked".to_owned(),
        surface_id_ref: "ux:surface:embedded-boundary-audit:service-dashboard:policy-blocked"
            .to_owned(),
        surface_family: SurfaceFamily::EmbeddedServiceDashboard,
        owner_identity: OwnerIdentityRecord {
            label: "Payments dashboard".to_owned(),
            class: OwnerClass::CustomerServiceOwner,
        },
        publisher_or_service_identity: PublisherOrServiceIdentityRecord {
            label: "Acme Cloud".to_owned(),
            class: PublisherOrServiceClass::CustomerService,
        },
        origin_identity: OriginIdentityRecord {
            origin_class: OriginClass::CustomerOrEnterpriseHostedWeb,
            origin_label: "https://console.acme.example/payments-prod".to_owned(),
            verification_state: OriginVerificationState::PolicyBlocked,
            host_or_domain_label: "console.acme.example".to_owned(),
            origin_ref: Some("id:origin:payments-dashboard:policy-blocked".to_owned()),
        },
        data_boundary_class: DataBoundaryClass::CustomerControlPlaneBoundary,
        data_boundary_label:
            "Hosted dashboard. Embedded render disabled by managed-workspace policy.".to_owned(),
        boundary_state: BoundaryState::PolicyBlocked,
        boundary_state_label: "Policy blocked (in-product render disabled)".to_owned(),
        plain_language_summary:
            "Managed policy prevents Aureline from rendering this dashboard in-product. The card preserves owner, origin, and provider scope and routes recovery into the host-native review surface."
                .to_owned(),
        permission_state: PermissionStateRecord {
            permission_class: PermissionClass::HostOwnedBrowserOnly,
            permission_label:
                "Host-owned browser only. Embedded body disabled by managed policy.".to_owned(),
            host_native_step_up_required: Some(true),
            exception_id_ref: None,
            scope_narrowing_summary: Some(
                "Provider install, billing, and approval authority remain host-native and require the browser handoff before mutation."
                    .to_owned(),
            ),
        },
        action_partition: vec![
            ActionPartitionRecord {
                action_id: BoundaryActionId::InspectPolicyReason,
                partition_role: ActionPartitionRole::ProductOwnedNative,
                action_label: "Inspect policy reason".to_owned(),
                renders_in_host_chrome: true,
                preserves_object_identity: true,
                browser_handoff_packet_ref: None,
            },
            ActionPartitionRecord {
                action_id: BoundaryActionId::OpenInSystemBrowser,
                partition_role: ActionPartitionRole::ProductOwnedHandoff,
                action_label: "Review in browser".to_owned(),
                renders_in_host_chrome: true,
                preserves_object_identity: true,
                browser_handoff_packet_ref: Some(
                    "id:browser-handoff:service-dashboard:payments-prod-review".to_owned(),
                ),
            },
            ActionPartitionRecord {
                action_id: BoundaryActionId::OpenSupportEvidence,
                partition_role: ActionPartitionRole::ProductOwnedHandoff,
                action_label: "Show policy evidence".to_owned(),
                renders_in_host_chrome: true,
                preserves_object_identity: true,
                browser_handoff_packet_ref: None,
            },
        ],
        browser_fallback: BrowserFallbackRecord {
            posture_class: BrowserFallbackPostureClass::ExternalOpenBlockedByPolicy,
            fallback_target_class: FallbackTargetClass::HostNativeReviewOrApproval,
            summary_label:
                "External open blocked by managed policy; recovery routes through the host-native review surface."
                    .to_owned(),
            browser_handoff_packet_ref: Some(
                "id:browser-handoff:service-dashboard:payments-prod-review".to_owned(),
            ),
            device_code_ref: None,
            policy_reason_label: Some(
                "Managed-workspace policy disables embedded payments dashboards outside an approved review window."
                    .to_owned(),
            ),
            return_target_label: Some("Aureline desktop / Service dashboard pane".to_owned()),
        },
        capability_limitations: vec![
            CapabilityLimitation::CannotIssueNativeApproval,
            CapabilityLimitation::CannotPerformRollbackOrRestore,
            CapabilityLimitation::CookiesOrStorageOutsideProductBoundary,
            CapabilityLimitation::CrossOriginDomOrStorageHidden,
        ],
        reserved_native_surfaces_host_owned: vec![
            NativeReservedSurface::ProductSecurityMessaging,
            NativeReservedSurface::UpdateVerification,
            NativeReservedSurface::WorkspaceTrustElevation,
            NativeReservedSurface::RollbackOrRestoreConfirmation,
            NativeReservedSurface::AiApplyReview,
            NativeReservedSurface::HighRiskApprovalSheet,
        ],
        layout_constraints: vec![
            LayoutConstraintId::CardVisuallyDistinctFromEmbeddedBody,
            LayoutConstraintId::CardNotObscuredByEmbeddedBody,
            LayoutConstraintId::CardRequiredFieldsNeverHoverOnly,
            LayoutConstraintId::CardCompactLayoutPreservesRequiredFields,
            LayoutConstraintId::CardActionsRenderInHostChrome,
            LayoutConstraintId::EmbeddedBodyCannotOverlapCardActions,
            LayoutConstraintId::CardRemainsVisibleWhenEmbeddedBodyIsWithheld,
        ],
        chrome_inheritance_axes: vec![
            ChromeInheritanceAxis::ThemePaletteInheritsFromHost,
            ChromeInheritanceAxis::DensityClassInheritsFromHost,
            ChromeInheritanceAxis::ZoomLevelInheritsFromHost,
            ChromeInheritanceAxis::FocusRingInheritsFromHost,
            ChromeInheritanceAxis::ReducedMotionPostureInheritsFromHost,
            ChromeInheritanceAxis::HighContrastModeInheritsFromHost,
            ChromeInheritanceAxis::ForcedColorsModeInheritsFromHost,
        ],
        source_truth: None,
        provider_identity: Some(ProviderIdentityRecord {
            provider_class: ProviderClass::ManagedAdminProvider,
            provider_label: "Acme Cloud dashboard".to_owned(),
            provider_scope_label: "Cluster payments-prod".to_owned(),
            provider_actor_class: ProviderActorClass::DelegatedUserToken,
            health_state: ProviderHealthState::Suspended,
            connected_provider_record_id: Some(
                "id:provider:acme-cloud:payments-prod".to_owned(),
            ),
            health_summary_label: Some(
                "Embedded dashboard suspended by managed policy; provider session valid for browser handoff."
                    .to_owned(),
            ),
        }),
        auth_handoff: None,
        policy_context: PolicyContext {
            identity_mode: IdentityMode::ManagedWorkspace,
            policy_epoch: "epoch.embedded-boundary-audit:service-dashboard:beta".to_owned(),
            trust_state: TrustState::Restricted,
            execution_context_id: Some(
                "ctx:embedded-boundary-audit:service-dashboard:beta".to_owned(),
            ),
        },
        redaction_class: RedactionClass::InternalSupportRestricted,
        minted_at: "2026-05-15T00:00:00Z".to_owned(),
    }
}

fn auth_confirmation_system_browser_first_card() -> EmbeddedBoundaryCardRecord {
    EmbeddedBoundaryCardRecord {
        record_kind: "embedded_boundary_card_record".to_owned(),
        embedded_boundary_card_schema_version: 1,
        card_id: "ux:card:embedded-boundary-audit:auth-confirmation:system-browser-first"
            .to_owned(),
        surface_id_ref:
            "ux:surface:embedded-boundary-audit:auth-confirmation:system-browser-first".to_owned(),
        surface_family: SurfaceFamily::EmbeddedAuthConfirmation,
        owner_identity: OwnerIdentityRecord {
            label: "Aureline auth handoff".to_owned(),
            class: OwnerClass::HostProduct,
        },
        publisher_or_service_identity: PublisherOrServiceIdentityRecord {
            label: "GitHub identity provider".to_owned(),
            class: PublisherOrServiceClass::IdentityProvider,
        },
        origin_identity: OriginIdentityRecord {
            origin_class: OriginClass::SystemBrowserReturn,
            origin_label: "System browser → Aureline desktop".to_owned(),
            verification_state: OriginVerificationState::Verified,
            host_or_domain_label: "github.com".to_owned(),
            origin_ref: Some("id:origin:auth:github-system-browser".to_owned()),
        },
        data_boundary_class: DataBoundaryClass::ConnectedProviderBoundary,
        data_boundary_label: "System browser session, returned to Aureline desktop.".to_owned(),
        boundary_state: BoundaryState::LiveVerified,
        boundary_state_label: "Live verified".to_owned(),
        plain_language_summary:
            "Sign in opens in your system browser. Aureline waits for the return packet; nothing is typed inside this card. Device-code is the auditable fallback if the browser cannot return."
                .to_owned(),
        permission_state: PermissionStateRecord {
            permission_class: PermissionClass::HostOwnedWithNativeStepUpRequired,
            permission_label:
                "Host-owned with native step-up gating any high-risk approval that follows."
                    .to_owned(),
            host_native_step_up_required: Some(true),
            exception_id_ref: None,
            scope_narrowing_summary: Some(
                "The card never collects passwords. Approval, trust, and apply surfaces remain host-native after sign-in."
                    .to_owned(),
            ),
        },
        action_partition: vec![
            ActionPartitionRecord {
                action_id: BoundaryActionId::OpenInSystemBrowser,
                partition_role: ActionPartitionRole::ProductOwnedHandoff,
                action_label: "Sign in with system browser".to_owned(),
                renders_in_host_chrome: true,
                preserves_object_identity: true,
                browser_handoff_packet_ref: Some("id:browser-handoff:auth:github".to_owned()),
            },
            ActionPartitionRecord {
                action_id: BoundaryActionId::SwitchToDeviceCode,
                partition_role: ActionPartitionRole::ProductOwnedHandoff,
                action_label: "Use device code instead".to_owned(),
                renders_in_host_chrome: true,
                preserves_object_identity: true,
                browser_handoff_packet_ref: None,
            },
            ActionPartitionRecord {
                action_id: BoundaryActionId::RetryAuthHandoff,
                partition_role: ActionPartitionRole::ProductOwnedNative,
                action_label: "Retry sign-in".to_owned(),
                renders_in_host_chrome: true,
                preserves_object_identity: true,
                browser_handoff_packet_ref: None,
            },
            ActionPartitionRecord {
                action_id: BoundaryActionId::OpenSupportEvidence,
                partition_role: ActionPartitionRole::ProductOwnedHandoff,
                action_label: "Show handoff evidence".to_owned(),
                renders_in_host_chrome: true,
                preserves_object_identity: true,
                browser_handoff_packet_ref: None,
            },
        ],
        browser_fallback: BrowserFallbackRecord {
            posture_class: BrowserFallbackPostureClass::SystemBrowserFirst,
            fallback_target_class: FallbackTargetClass::SystemBrowserHandoffPacket,
            summary_label: "System browser is the primary sign-in path.".to_owned(),
            browser_handoff_packet_ref: Some("id:browser-handoff:auth:github".to_owned()),
            device_code_ref: None,
            policy_reason_label: None,
            return_target_label: Some("Aureline desktop / Auth handoff sheet".to_owned()),
        },
        capability_limitations: vec![],
        reserved_native_surfaces_host_owned: vec![
            NativeReservedSurface::ProductSecurityMessaging,
            NativeReservedSurface::UpdateVerification,
            NativeReservedSurface::WorkspaceTrustElevation,
            NativeReservedSurface::RollbackOrRestoreConfirmation,
            NativeReservedSurface::AiApplyReview,
            NativeReservedSurface::HighRiskApprovalSheet,
        ],
        layout_constraints: vec![
            LayoutConstraintId::CardVisuallyDistinctFromEmbeddedBody,
            LayoutConstraintId::CardNotObscuredByEmbeddedBody,
            LayoutConstraintId::CardRequiredFieldsNeverHoverOnly,
            LayoutConstraintId::CardCompactLayoutPreservesRequiredFields,
            LayoutConstraintId::CardActionsRenderInHostChrome,
            LayoutConstraintId::EmbeddedBodyCannotOverlapCardActions,
            LayoutConstraintId::CardRemainsVisibleWhenEmbeddedBodyIsWithheld,
        ],
        chrome_inheritance_axes: vec![
            ChromeInheritanceAxis::ThemePaletteInheritsFromHost,
            ChromeInheritanceAxis::DensityClassInheritsFromHost,
            ChromeInheritanceAxis::ZoomLevelInheritsFromHost,
            ChromeInheritanceAxis::FocusRingInheritsFromHost,
            ChromeInheritanceAxis::ReducedMotionPostureInheritsFromHost,
            ChromeInheritanceAxis::HighContrastModeInheritsFromHost,
            ChromeInheritanceAxis::ForcedColorsModeInheritsFromHost,
        ],
        source_truth: None,
        provider_identity: None,
        auth_handoff: Some(AuthHandoffCardRecord {
            flow_class: AuthFlowClass::SystemBrowser,
            provider_domain_label: "github.com".to_owned(),
            reason_label: "Sign in with your GitHub account in the system browser.".to_owned(),
            return_target_label: "Aureline desktop / Auth handoff sheet".to_owned(),
            local_continuity_note: Some(
                "Your local workspace stays open while you sign in.".to_owned(),
            ),
            code_expiry_label: None,
            exception_id_ref: None,
        }),
        policy_context: PolicyContext {
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: "epoch.embedded-boundary-audit:auth-confirmation:beta".to_owned(),
            trust_state: TrustState::Trusted,
            execution_context_id: Some(
                "ctx:embedded-boundary-audit:auth-confirmation:beta".to_owned(),
            ),
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-15T00:00:00Z".to_owned(),
    }
}

// ----- Token helpers (closed-vocabulary mappings) ---------------------------

fn surface_family_token(value: SurfaceFamily) -> &'static str {
    match value {
        SurfaceFamily::EmbeddedDocsHelp => "embedded_docs_help",
        SurfaceFamily::EmbeddedMarketplaceOrAccount => "embedded_marketplace_or_account",
        SurfaceFamily::EmbeddedServiceDashboard => "embedded_service_dashboard",
        SurfaceFamily::EmbeddedAuthConfirmation => "embedded_auth_confirmation",
        SurfaceFamily::ExtensionHostedSurface => "extension_hosted_surface",
    }
}

fn owner_class_token(value: OwnerClass) -> &'static str {
    match value {
        OwnerClass::HostProduct => "host_product",
        OwnerClass::FirstPartyProject => "first_party_project",
        OwnerClass::ExtensionBundle => "extension_bundle",
        OwnerClass::ConnectedProvider => "connected_provider",
        OwnerClass::EnterpriseAdmin => "enterprise_admin",
        OwnerClass::CustomerServiceOwner => "customer_service_owner",
        OwnerClass::UnknownOwner => "unknown_owner",
    }
}

fn publisher_or_service_class_token(value: PublisherOrServiceClass) -> &'static str {
    match value {
        PublisherOrServiceClass::FirstPartyProject => "first_party_project",
        PublisherOrServiceClass::MarketplaceService => "marketplace_service",
        PublisherOrServiceClass::ConnectedProviderService => "connected_provider_service",
        PublisherOrServiceClass::CustomerService => "customer_service",
        PublisherOrServiceClass::ExtensionPublisher => "extension_publisher",
        PublisherOrServiceClass::IdentityProvider => "identity_provider",
        PublisherOrServiceClass::UnknownPublisherOrService => "unknown_publisher_or_service",
    }
}

fn origin_class_token(value: OriginClass) -> &'static str {
    match value {
        OriginClass::LocalPackOrArtifact => "local_pack_or_artifact",
        OriginClass::FirstPartyHostedWeb => "first_party_hosted_web",
        OriginClass::ConnectedProviderHostedWeb => "connected_provider_hosted_web",
        OriginClass::CustomerOrEnterpriseHostedWeb => "customer_or_enterprise_hosted_web",
        OriginClass::ExtensionPublisherHostedWeb => "extension_publisher_hosted_web",
        OriginClass::CrossOriginSubframe => "cross_origin_subframe",
        OriginClass::SystemBrowserReturn => "system_browser_return",
        OriginClass::UnknownOriginClass => "unknown_origin_class",
    }
}

fn boundary_state_from_token(value: &str) -> Option<BoundaryState> {
    match value {
        "live_verified" => Some(BoundaryState::LiveVerified),
        "stale_snapshot" => Some(BoundaryState::StaleSnapshot),
        "policy_blocked" => Some(BoundaryState::PolicyBlocked),
        "certificate_failed" => Some(BoundaryState::CertificateFailed),
        "cross_origin_limited" => Some(BoundaryState::CrossOriginLimited),
        "offline_snapshot" => Some(BoundaryState::OfflineSnapshot),
        "external_open_only" => Some(BoundaryState::ExternalOpenOnly),
        _ => None,
    }
}

fn permission_class_from_token(value: &str) -> Option<PermissionClass> {
    match value {
        "host_owned_full_authority" => Some(PermissionClass::HostOwnedFullAuthority),
        "host_owned_inspect_only" => Some(PermissionClass::HostOwnedInspectOnly),
        "host_owned_browser_only" => Some(PermissionClass::HostOwnedBrowserOnly),
        "host_owned_copy_export_only" => Some(PermissionClass::HostOwnedCopyExportOnly),
        "host_owned_with_native_step_up_required" => {
            Some(PermissionClass::HostOwnedWithNativeStepUpRequired)
        }
        "embedded_lower_trust_session_refresh" => {
            Some(PermissionClass::EmbeddedLowerTrustSessionRefresh)
        }
        "embedded_lower_trust_password_exception" => {
            Some(PermissionClass::EmbeddedLowerTrustPasswordException)
        }
        "no_permission_within_product" => Some(PermissionClass::NoPermissionWithinProduct),
        _ => None,
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
        PermissionClass::EmbeddedLowerTrustSessionRefresh => {
            "embedded_lower_trust_session_refresh"
        }
        PermissionClass::EmbeddedLowerTrustPasswordException => {
            "embedded_lower_trust_password_exception"
        }
        PermissionClass::NoPermissionWithinProduct => "no_permission_within_product",
    }
}

fn browser_fallback_posture_from_token(value: &str) -> Option<BrowserFallbackPostureClass> {
    match value {
        "system_browser_first" => Some(BrowserFallbackPostureClass::SystemBrowserFirst),
        "device_code_fallback_offered" => {
            Some(BrowserFallbackPostureClass::DeviceCodeFallbackOffered)
        }
        "external_open_blocked_by_policy" => {
            Some(BrowserFallbackPostureClass::ExternalOpenBlockedByPolicy)
        }
        "external_open_unavailable_offline" => {
            Some(BrowserFallbackPostureClass::ExternalOpenUnavailableOffline)
        }
        "browser_fallback_not_applicable" => {
            Some(BrowserFallbackPostureClass::BrowserFallbackNotApplicable)
        }
        _ => None,
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

fn identity_mode_token(value: IdentityMode) -> &'static str {
    match value {
        IdentityMode::AccountFreeLocal => "account_free_local",
        IdentityMode::SelfHostedOrg => "self_hosted_org",
        IdentityMode::ManagedWorkspace => "managed_workspace",
    }
}

fn trust_state_token(value: TrustState) -> &'static str {
    match value {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
    }
}

fn auth_flow_class_token(value: AuthFlowClass) -> &'static str {
    match value {
        AuthFlowClass::NotApplicable => "not_applicable",
        AuthFlowClass::SystemBrowser => "system_browser",
        AuthFlowClass::DeviceCode => "device_code",
        AuthFlowClass::PlatformAuthenticatorNative => "platform_authenticator_native",
        AuthFlowClass::EmbeddedSessionRefresh => "embedded_session_refresh",
        AuthFlowClass::EmbeddedPasswordException => "embedded_password_exception",
    }
}

fn provider_class_token(value: ProviderClass) -> &'static str {
    match value {
        ProviderClass::ReviewOrCodeHost => "review_or_code_host",
        ProviderClass::IssueOrPlanningTracker => "issue_or_planning_tracker",
        ProviderClass::CiOrCheckProvider => "ci_or_check_provider",
        ProviderClass::DocsOrPortalProvider => "docs_or_portal_provider",
        ProviderClass::IdentityOrEnterpriseProvider => "identity_or_enterprise_provider",
        ProviderClass::CallbackOrEventProvider => "callback_or_event_provider",
        ProviderClass::AiProvider => "ai_provider",
        ProviderClass::PackageRegistryProvider => "package_registry_provider",
        ProviderClass::ReleasePublisherProvider => "release_publisher_provider",
        ProviderClass::ManagedAdminProvider => "managed_admin_provider",
    }
}

fn provider_health_token(value: ProviderHealthState) -> &'static str {
    match value {
        ProviderHealthState::Healthy => "healthy",
        ProviderHealthState::Degraded => "degraded",
        ProviderHealthState::Unavailable => "unavailable",
        ProviderHealthState::Revoked => "revoked",
        ProviderHealthState::Suspended => "suspended",
        ProviderHealthState::Expired => "expired",
    }
}

fn redaction_class_token(value: RedactionClass) -> &'static str {
    match value {
        RedactionClass::MetadataSafeDefault => "metadata_safe_default",
        RedactionClass::OperatorOnlyRestricted => "operator_only_restricted",
        RedactionClass::InternalSupportRestricted => "internal_support_restricted",
        RedactionClass::SigningEvidenceOnly => "signing_evidence_only",
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

fn layout_constraint_token(value: LayoutConstraintId) -> &'static str {
    match value {
        LayoutConstraintId::CardVisuallyDistinctFromEmbeddedBody => {
            "card_visually_distinct_from_embedded_body"
        }
        LayoutConstraintId::CardNotObscuredByEmbeddedBody => "card_not_obscured_by_embedded_body",
        LayoutConstraintId::CardRequiredFieldsNeverHoverOnly => {
            "card_required_fields_never_hover_only"
        }
        LayoutConstraintId::CardCompactLayoutPreservesRequiredFields => {
            "card_compact_layout_preserves_required_fields"
        }
        LayoutConstraintId::CardActionsRenderInHostChrome => "card_actions_render_in_host_chrome",
        LayoutConstraintId::EmbeddedBodyCannotOverlapCardActions => {
            "embedded_body_cannot_overlap_card_actions"
        }
        LayoutConstraintId::CardRemainsVisibleWhenEmbeddedBodyIsWithheld => {
            "card_remains_visible_when_embedded_body_is_withheld"
        }
    }
}

fn required_native_reserved_surface_tokens() -> [&'static str; 6] {
    [
        "product_security_messaging",
        "update_verification",
        "workspace_trust_elevation",
        "rollback_or_restore_confirmation",
        "ai_apply_review",
        "high_risk_approval_sheet",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_seeds_zero_defects() {
        let page = seeded_embedded_boundary_audit_page();
        assert!(
            page.defects.is_empty(),
            "seeded page must seed zero defects: {:#?}",
            page.defects
        );
        validate_embedded_boundary_audit_page(&page).expect("seeded page must validate");
    }

    #[test]
    fn seeded_page_covers_every_embedded_surface_family() {
        let page = seeded_embedded_boundary_audit_page();
        assert!(page.covers_required_surface_families());
        assert_eq!(page.summary.surface_family_count, 5);
    }

    #[test]
    fn seeded_page_keeps_high_risk_approval_host_owned_on_every_row() {
        let page = seeded_embedded_boundary_audit_page();
        for row in &page.rows {
            for required in required_native_reserved_surface_tokens() {
                assert!(
                    row.native_reserved_surface_tokens.iter().any(|t| t == required),
                    "row {} must keep {} on host-owned set",
                    row.row_id,
                    required
                );
            }
        }
        assert_eq!(
            page.summary.host_owned_high_risk_approval_row_count,
            page.rows.len()
        );
    }

    #[test]
    fn seeded_page_keeps_safe_baseline_posture_on_every_identity_or_risky_row() {
        let page = seeded_embedded_boundary_audit_page();
        let baseline = safe_baseline_posture_tokens();
        for row in page
            .rows
            .iter()
            .filter(|r| surface_is_identity_or_risky(r.surface_family))
        {
            assert!(
                baseline.contains(&row.browser_fallback_posture_token.as_str()),
                "identity or risky row {} must quote a safe-baseline posture token (got {})",
                row.row_id,
                row.browser_fallback_posture_token
            );
        }
    }

    #[test]
    fn seeded_page_pairs_every_row_with_a_support_row() {
        let page = seeded_embedded_boundary_audit_page();
        assert_eq!(page.rows.len(), page.support_rows.len());
        for row in &page.rows {
            let support = page
                .support_rows
                .iter()
                .find(|s| s.row_id == row.row_id)
                .expect("support row must exist for live row");
            assert_eq!(support.surface_family_token, row.surface_family_token);
            assert_eq!(support.boundary_state_token, row.boundary_state_token);
            assert_eq!(support.permission_class_token, row.permission_class_token);
            assert_eq!(
                support.browser_fallback_posture_token,
                row.browser_fallback_posture_token
            );
        }
    }

    #[test]
    fn audit_flags_missing_owner_label() {
        let mut page = seeded_embedded_boundary_audit_page();
        page.rows[0].owner_label.clear();
        let defects = audit_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == EmbeddedBoundaryAuditDefectKind::MissingOwnerLabel));
    }

    #[test]
    fn audit_flags_missing_origin_host_label() {
        let mut page = seeded_embedded_boundary_audit_page();
        page.rows[0].host_or_domain_label.clear();
        let defects = audit_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == EmbeddedBoundaryAuditDefectKind::MissingOriginHostLabel));
    }

    #[test]
    fn audit_flags_missing_browser_fallback_posture_token() {
        let mut page = seeded_embedded_boundary_audit_page();
        page.rows[0].browser_fallback_posture_token = "not_a_real_posture".to_owned();
        let defects = audit_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(
            |d| d.defect_kind == EmbeddedBoundaryAuditDefectKind::MissingBrowserFallbackPostureToken
        ));
    }

    #[test]
    fn audit_flags_system_browser_not_baseline_on_identity_or_risky_row() {
        let mut page = seeded_embedded_boundary_audit_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.surface_family == SurfaceFamily::EmbeddedAuthConfirmation)
            .expect("auth row");
        row.browser_fallback_posture_token = "browser_fallback_not_applicable".to_owned();
        let defects = audit_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == EmbeddedBoundaryAuditDefectKind::SystemBrowserNotBaselineOnIdentityOrRiskyWeb));
    }

    #[test]
    fn audit_flags_dropped_native_reserved_surface() {
        let mut page = seeded_embedded_boundary_audit_page();
        page.rows[0]
            .native_reserved_surface_tokens
            .retain(|t| t != "workspace_trust_elevation");
        let defects = audit_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == EmbeddedBoundaryAuditDefectKind::EmbeddedMintedNativeReservedSurface));
    }

    #[test]
    fn audit_flags_support_row_vocabulary_drift() {
        let mut page = seeded_embedded_boundary_audit_page();
        page.support_rows[0].boundary_state_token = "stale_snapshot".to_owned();
        let defects = audit_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == EmbeddedBoundaryAuditDefectKind::SupportRowVocabularyDrift));
    }

    #[test]
    fn audit_flags_boundary_state_inconsistent_with_origin_verification() {
        let mut page = seeded_embedded_boundary_audit_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.boundary_state_token == "live_verified")
            .expect("seed has a live_verified row");
        row.origin_verification_token = "unverified".to_owned();
        let defects = audit_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == EmbeddedBoundaryAuditDefectKind::BoundaryStateInconsistentWithOriginVerification));
    }

    #[test]
    fn audit_flags_auth_confirmation_missing_flow_class() {
        let mut page = seeded_embedded_boundary_audit_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.surface_family == SurfaceFamily::EmbeddedAuthConfirmation)
            .expect("auth row");
        row.auth_flow_class_token = None;
        let defects = audit_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == EmbeddedBoundaryAuditDefectKind::AuthConfirmationMissingFlowClass));
    }

    #[test]
    fn support_export_quotes_page_and_summarises_defects() {
        let page = seeded_embedded_boundary_audit_page();
        let export = EmbeddedBoundaryAuditSupportExport::from_page(
            "support-export:embedded-boundary-audit-beta:001",
            "2026-05-15T00:00:00Z",
            page.clone(),
        );
        assert_eq!(
            export.shared_contract_ref,
            EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.defect_counts_by_kind.is_empty());
        assert_eq!(export.page, page);
    }

    #[test]
    fn support_export_summarises_defects_when_present() {
        let mut page = seeded_embedded_boundary_audit_page();
        page.rows[0].owner_label.clear();
        page.defects = audit_rows(&page.rows, &page.support_rows);
        let export = EmbeddedBoundaryAuditSupportExport::from_page(
            "support-export:embedded-boundary-audit-beta:002",
            "2026-05-15T00:00:00Z",
            page,
        );
        assert!(export
            .defect_kinds_present
            .contains(&EmbeddedBoundaryAuditDefectKind::MissingOwnerLabel));
        let count = export
            .defect_counts_by_kind
            .get(EmbeddedBoundaryAuditDefectKind::MissingOwnerLabel.as_str())
            .copied()
            .unwrap_or(0);
        assert_eq!(count, 1);
    }
}
