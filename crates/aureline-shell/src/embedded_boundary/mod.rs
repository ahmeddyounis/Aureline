//! Reusable embedded-boundary toolkit for host-owned chrome and event logs.
//!
//! The toolkit consumes the beta embedded-boundary audit page and projects the
//! reusable row shape that shell surfaces can render around embedded docs/help,
//! extension webviews, marketplace/account content, service dashboards, and
//! auth handoff confirmations. It keeps owner/origin chrome, browser handoff,
//! native approval fences, and support-export rows on one contract.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::embedded::boundary_card::SurfaceFamily;
use crate::embedded_boundary_audit::{
    seeded_embedded_boundary_audit_page, validate_embedded_boundary_audit_page,
    EmbeddedBoundaryAuditDefect, EmbeddedBoundaryAuditPage, EmbeddedBoundaryAuditRow,
    EmbeddedBoundaryAuditSupportRow,
};

/// Schema version for embedded-boundary toolkit payloads.
pub const EMBEDDED_BOUNDARY_TOOLKIT_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref used by toolkit rows, event logs, and support export.
pub const EMBEDDED_BOUNDARY_TOOLKIT_SHARED_CONTRACT_REF: &str =
    "shell:embedded_boundary_toolkit:v1";

/// Record-kind tag for [`EmbeddedBoundaryToolkitPage`] payloads.
pub const EMBEDDED_BOUNDARY_TOOLKIT_PAGE_RECORD_KIND: &str =
    "shell_embedded_boundary_toolkit_page_record";

/// Record-kind tag for [`EmbeddedBoundaryToolkitRow`] payloads.
pub const EMBEDDED_BOUNDARY_TOOLKIT_ROW_RECORD_KIND: &str =
    "shell_embedded_boundary_toolkit_row_record";

/// Record-kind tag for [`EmbeddedBoundaryToolkitEvent`] payloads.
pub const EMBEDDED_BOUNDARY_TOOLKIT_EVENT_RECORD_KIND: &str =
    "shell_embedded_boundary_toolkit_event_record";

/// Record-kind tag for [`EmbeddedBoundaryToolkitSupportRow`] payloads.
pub const EMBEDDED_BOUNDARY_TOOLKIT_SUPPORT_ROW_RECORD_KIND: &str =
    "shell_embedded_boundary_toolkit_support_row_record";

/// Record-kind tag for [`EmbeddedBoundaryToolkitSupportExport`] payloads.
pub const EMBEDDED_BOUNDARY_TOOLKIT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_embedded_boundary_toolkit_support_export_record";

/// Record-kind tag for [`EmbeddedBoundaryToolkitDefect`] payloads.
pub const EMBEDDED_BOUNDARY_TOOLKIT_DEFECT_RECORD_KIND: &str =
    "shell_embedded_boundary_toolkit_defect_record";

/// Closed event vocabulary emitted by the toolkit event log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedBoundaryToolkitEventClass {
    /// Host-rendered owner/origin chrome was projected for the row.
    BoundaryChromeRendered,
    /// A typed browser handoff is available or policy-blocked on the row.
    BrowserHandoffDeclared,
    /// The auth row defaulted to system-browser or device-code handoff.
    AuthHandoffDefaultRecorded,
    /// Native high-risk approval surfaces stayed host-owned.
    NativeApprovalFenceConfirmed,
    /// The support-export row was projected with matching boundary vocabulary.
    SupportExportProjected,
}

impl EmbeddedBoundaryToolkitEventClass {
    /// Returns the stable schema token for the event class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundaryChromeRendered => "boundary_chrome_rendered",
            Self::BrowserHandoffDeclared => "browser_handoff_declared",
            Self::AuthHandoffDefaultRecorded => "auth_handoff_default_recorded",
            Self::NativeApprovalFenceConfirmed => "native_approval_fence_confirmed",
            Self::SupportExportProjected => "support_export_projected",
        }
    }
}

/// Closed defect vocabulary emitted by the toolkit validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedBoundaryToolkitDefectKind {
    /// The source audit page did not validate.
    SourceAuditDefectsPresent,
    /// A row is missing owner/origin chrome values.
    MissingOwnerOriginChrome,
    /// A row that offers a browser handoff lacks a matching event packet.
    MissingBrowserHandoffEvent,
    /// An auth row does not default to system browser or device-code fallback.
    IdentityRowNotSystemBrowserDefault,
    /// A row dropped a required native approval fence.
    MissingNativeApprovalFence,
    /// The toolkit support export dropped or drifted from a live row.
    SupportExportParityDrift,
    /// The event log cannot reconstruct content owner, native owner, and handoff path.
    EventLogReconstructionGap,
}

impl EmbeddedBoundaryToolkitDefectKind {
    /// Returns the stable schema token for the defect kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceAuditDefectsPresent => "source_audit_defects_present",
            Self::MissingOwnerOriginChrome => "missing_owner_origin_chrome",
            Self::MissingBrowserHandoffEvent => "missing_browser_handoff_event",
            Self::IdentityRowNotSystemBrowserDefault => "identity_row_not_system_browser_default",
            Self::MissingNativeApprovalFence => "missing_native_approval_fence",
            Self::SupportExportParityDrift => "support_export_parity_drift",
            Self::EventLogReconstructionGap => "event_log_reconstruction_gap",
        }
    }
}

/// Typed validation defect emitted by [`validate_embedded_boundary_toolkit_page`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryToolkitDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Toolkit schema version.
    pub schema_version: u32,
    /// Shared toolkit contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Closed defect kind.
    pub defect_kind: EmbeddedBoundaryToolkitDefectKind,
    /// Stable defect token.
    pub defect_kind_token: String,
    /// Affected toolkit row id.
    pub toolkit_row_id: String,
    /// Affected field or logical section.
    pub field: String,
    /// Reviewer-facing defect note.
    pub note: String,
}

impl EmbeddedBoundaryToolkitDefect {
    fn new(
        defect_kind: EmbeddedBoundaryToolkitDefectKind,
        toolkit_row_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let toolkit_row_id = toolkit_row_id.into();
        Self {
            record_kind: EMBEDDED_BOUNDARY_TOOLKIT_DEFECT_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_TOOLKIT_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_TOOLKIT_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "ux:defect:embedded-boundary-toolkit:{}:{}",
                defect_kind.as_str(),
                toolkit_row_id
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            toolkit_row_id,
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Renderable toolkit row for one embedded surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryToolkitRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Toolkit schema version.
    pub schema_version: u32,
    /// Shared toolkit contract ref.
    pub shared_contract_ref: String,
    /// Stable toolkit row id.
    pub toolkit_row_id: String,
    /// Source audit row id.
    pub audit_row_id_ref: String,
    /// Matching support row id.
    pub support_row_id_ref: String,
    /// Embedded surface family.
    pub surface_family: SurfaceFamily,
    /// Stable surface-family token.
    pub surface_family_token: String,
    /// Human owner rendered by host chrome.
    pub owner_label: String,
    /// Publisher, provider, or service identity rendered separately from owner.
    pub publisher_or_service_label: String,
    /// Origin label rendered by host chrome.
    pub origin_label: String,
    /// Origin host or domain.
    pub host_or_domain_label: String,
    /// Data boundary class token.
    pub data_boundary_class_token: String,
    /// Data boundary label.
    pub data_boundary_label: String,
    /// Boundary state token.
    pub boundary_state_token: String,
    /// Permission class token.
    pub permission_class_token: String,
    /// Identity mode token.
    pub identity_mode_token: String,
    /// Trust state token.
    pub trust_state_token: String,
    /// Source, version, freshness, or provider freshness label.
    pub source_version_freshness_label: String,
    /// Network, offline, stale, policy, or provider posture label.
    pub network_or_offline_posture_label: String,
    /// Browser fallback posture token.
    pub browser_fallback_posture_token: String,
    /// Browser fallback target token.
    pub fallback_target_class_token: String,
    /// Browser handoff packet ref, when a handoff exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Return target label for browser/device-code handoff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_target_label: Option<String>,
    /// Provider scope label, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_scope_label: Option<String>,
    /// Provider health token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_health_token: Option<String>,
    /// Auth flow class token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_flow_class_token: Option<String>,
    /// Auth provider or domain label, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_provider_domain_label: Option<String>,
    /// Auth exception record ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_exception_id_ref: Option<String>,
    /// Reason-code token for the auth path.
    pub auth_reason_code: String,
    /// True when the row defaults to system browser or device-code auth.
    pub system_browser_auth_default: bool,
    /// Native owner token for high-risk approvals.
    pub native_approval_owner_token: String,
    /// Native approval surfaces that remain host-owned.
    pub native_approval_surface_tokens: Vec<String>,
    /// True when the row has every required native approval fence.
    pub native_approval_fence_active: bool,
    /// True when support export uses the same row identity and vocabulary.
    pub support_export_parity_checked: bool,
    /// Event ids emitted for this row.
    pub boundary_event_ids: Vec<String>,
    /// Host-rendered reviewer summary.
    pub plain_language_summary: String,
    /// Redaction class token.
    pub redaction_class_token: String,
}

impl EmbeddedBoundaryToolkitRow {
    fn from_audit_row(
        row: &EmbeddedBoundaryAuditRow,
        support: &EmbeddedBoundaryAuditSupportRow,
    ) -> Self {
        let toolkit_row_id = format!("ux:embedded-boundary-toolkit:{}", row.case_id);
        let browser_handoff_packet_ref = row.browser_handoff_packet_ref.clone();
        let native_approval_surface_tokens = row.native_reserved_surface_tokens.clone();
        let native_approval_fence_active =
            required_native_approval_surfaces().iter().all(|required| {
                native_approval_surface_tokens
                    .iter()
                    .any(|token| token == required)
            });
        let support_export_parity_checked = support_matches_row(row, support);
        let auth_reason_code = auth_reason_code(row);
        let system_browser_auth_default = auth_defaults_to_system_browser(row);
        let boundary_event_ids = event_ids_for_row(
            &toolkit_row_id,
            row.surface_family,
            browser_handoff_packet_ref.as_deref(),
            row.browser_fallback_posture_token.as_str(),
            row.auth_flow_class_token.as_deref(),
        );

        Self {
            record_kind: EMBEDDED_BOUNDARY_TOOLKIT_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_TOOLKIT_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_TOOLKIT_SHARED_CONTRACT_REF.to_owned(),
            toolkit_row_id,
            audit_row_id_ref: row.row_id.clone(),
            support_row_id_ref: support.row_id.clone(),
            surface_family: row.surface_family,
            surface_family_token: row.surface_family_token.clone(),
            owner_label: row.owner_label.clone(),
            publisher_or_service_label: row.publisher_or_service_label.clone(),
            origin_label: row.origin_label.clone(),
            host_or_domain_label: row.host_or_domain_label.clone(),
            data_boundary_class_token: row.data_boundary_class_token.clone(),
            data_boundary_label: row.data_boundary_label.clone(),
            boundary_state_token: row.boundary_state_token.clone(),
            permission_class_token: row.permission_class_token.clone(),
            identity_mode_token: row.identity_mode_token.clone(),
            trust_state_token: row.trust_state_token.clone(),
            source_version_freshness_label: source_version_freshness_label(row),
            network_or_offline_posture_label: network_or_offline_posture_label(row),
            browser_fallback_posture_token: row.browser_fallback_posture_token.clone(),
            fallback_target_class_token: row.fallback_target_class_token.clone(),
            browser_handoff_packet_ref,
            return_target_label: row.return_target_label.clone(),
            provider_scope_label: row.provider_scope_label.clone(),
            provider_health_token: row.provider_health_token.clone(),
            auth_flow_class_token: row.auth_flow_class_token.clone(),
            auth_provider_domain_label: row.auth_provider_domain_label.clone(),
            auth_exception_id_ref: row.auth_exception_id_ref.clone(),
            auth_reason_code,
            system_browser_auth_default,
            native_approval_owner_token: "host_product_native".to_owned(),
            native_approval_surface_tokens,
            native_approval_fence_active,
            support_export_parity_checked,
            boundary_event_ids,
            plain_language_summary: row.plain_language_summary.clone(),
            redaction_class_token: row.redaction_class_token.clone(),
        }
    }
}

/// Event-log row for reconstructing embedded boundary incidents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryToolkitEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Toolkit schema version.
    pub schema_version: u32,
    /// Shared toolkit contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub event_id: String,
    /// Closed event class.
    pub event_class: EmbeddedBoundaryToolkitEventClass,
    /// Stable event class token.
    pub event_class_token: String,
    /// Toolkit row id that emitted this event.
    pub toolkit_row_id_ref: String,
    /// Surface family token.
    pub surface_family_token: String,
    /// Embedded content owner.
    pub embedded_content_owner_label: String,
    /// Embedded origin host or domain.
    pub embedded_origin_label: String,
    /// Native owner for approval surfaces.
    pub native_approval_owner_token: String,
    /// Native approval surfaces that remained host-owned.
    pub native_approval_surface_tokens: Vec<String>,
    /// Browser handoff packet ref when this event crosses to a browser.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Support-export row id that can reconstruct this event.
    pub support_row_id_ref: String,
    /// Privacy consequence or redaction cue.
    pub privacy_consequence_label: String,
    /// Event timestamp used by seeded fixtures.
    pub emitted_at: String,
}

impl EmbeddedBoundaryToolkitEvent {
    fn from_row(
        row: &EmbeddedBoundaryToolkitRow,
        event_class: EmbeddedBoundaryToolkitEventClass,
        emitted_at: &str,
    ) -> Self {
        let browser_handoff_packet_ref = match event_class {
            EmbeddedBoundaryToolkitEventClass::BrowserHandoffDeclared
            | EmbeddedBoundaryToolkitEventClass::AuthHandoffDefaultRecorded => {
                row.browser_handoff_packet_ref.clone()
            }
            _ => None,
        };
        Self {
            record_kind: EMBEDDED_BOUNDARY_TOOLKIT_EVENT_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_TOOLKIT_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_TOOLKIT_SHARED_CONTRACT_REF.to_owned(),
            event_id: event_id_for_class(&row.toolkit_row_id, event_class),
            event_class,
            event_class_token: event_class.as_str().to_owned(),
            toolkit_row_id_ref: row.toolkit_row_id.clone(),
            surface_family_token: row.surface_family_token.clone(),
            embedded_content_owner_label: row.owner_label.clone(),
            embedded_origin_label: row.host_or_domain_label.clone(),
            native_approval_owner_token: row.native_approval_owner_token.clone(),
            native_approval_surface_tokens: row.native_approval_surface_tokens.clone(),
            browser_handoff_packet_ref,
            support_row_id_ref: row.support_row_id_ref.clone(),
            privacy_consequence_label: privacy_consequence_label(row).to_owned(),
            emitted_at: emitted_at.to_owned(),
        }
    }
}

/// Export-safe toolkit support row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryToolkitSupportRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Toolkit schema version.
    pub schema_version: u32,
    /// Shared toolkit contract ref.
    pub shared_contract_ref: String,
    /// Toolkit row id.
    pub toolkit_row_id_ref: String,
    /// Source audit row id.
    pub audit_row_id_ref: String,
    /// Source support row id.
    pub support_row_id_ref: String,
    /// Surface family token.
    pub surface_family_token: String,
    /// Embedded content owner label.
    pub owner_label: String,
    /// Embedded origin host or domain.
    pub host_or_domain_label: String,
    /// Data boundary class token.
    pub data_boundary_class_token: String,
    /// Boundary state token.
    pub boundary_state_token: String,
    /// Browser fallback posture token.
    pub browser_fallback_posture_token: String,
    /// Browser handoff packet ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Native approval owner token.
    pub native_approval_owner_token: String,
    /// Native approval surfaces that stayed host-owned.
    pub native_approval_surface_tokens: Vec<String>,
    /// Event ids included in the support export.
    pub boundary_event_ids: Vec<String>,
    /// Redaction class token.
    pub redaction_class_token: String,
}

impl EmbeddedBoundaryToolkitSupportRow {
    fn from_row(row: &EmbeddedBoundaryToolkitRow) -> Self {
        Self {
            record_kind: EMBEDDED_BOUNDARY_TOOLKIT_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_TOOLKIT_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_TOOLKIT_SHARED_CONTRACT_REF.to_owned(),
            toolkit_row_id_ref: row.toolkit_row_id.clone(),
            audit_row_id_ref: row.audit_row_id_ref.clone(),
            support_row_id_ref: row.support_row_id_ref.clone(),
            surface_family_token: row.surface_family_token.clone(),
            owner_label: row.owner_label.clone(),
            host_or_domain_label: row.host_or_domain_label.clone(),
            data_boundary_class_token: row.data_boundary_class_token.clone(),
            boundary_state_token: row.boundary_state_token.clone(),
            browser_fallback_posture_token: row.browser_fallback_posture_token.clone(),
            browser_handoff_packet_ref: row.browser_handoff_packet_ref.clone(),
            native_approval_owner_token: row.native_approval_owner_token.clone(),
            native_approval_surface_tokens: row.native_approval_surface_tokens.clone(),
            boundary_event_ids: row.boundary_event_ids.clone(),
            redaction_class_token: row.redaction_class_token.clone(),
        }
    }
}

/// Metadata-safe support export for the toolkit page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryToolkitSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Toolkit schema version.
    pub schema_version: u32,
    /// Shared toolkit contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Audit support export ref this toolkit wraps.
    pub audit_support_export_record_kind_ref: String,
    /// Export-safe toolkit rows.
    pub rows: Vec<EmbeddedBoundaryToolkitSupportRow>,
    /// Export-safe event log.
    pub event_log: Vec<EmbeddedBoundaryToolkitEvent>,
    /// Defect counts by toolkit defect token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True because raw embedded bodies, cookies, URLs, and secrets are excluded.
    pub raw_private_material_excluded: bool,
}

impl EmbeddedBoundaryToolkitSupportExport {
    fn from_rows(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        rows: &[EmbeddedBoundaryToolkitRow],
        event_log: &[EmbeddedBoundaryToolkitEvent],
        defects: &[EmbeddedBoundaryToolkitDefect],
    ) -> Self {
        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            record_kind: EMBEDDED_BOUNDARY_TOOLKIT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_TOOLKIT_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_TOOLKIT_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            audit_support_export_record_kind_ref:
                "shell_embedded_boundary_audit_beta_support_export_record".to_owned(),
            rows: rows
                .iter()
                .map(EmbeddedBoundaryToolkitSupportRow::from_row)
                .collect(),
            event_log: event_log.to_vec(),
            defect_counts_by_kind,
            raw_private_material_excluded: true,
        }
    }
}

/// Summary banner for the toolkit page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EmbeddedBoundaryToolkitSummary {
    /// Count of renderable toolkit rows.
    pub row_count: usize,
    /// Count of support-export rows.
    pub support_row_count: usize,
    /// Count of emitted boundary events.
    pub event_count: usize,
    /// Count of toolkit defects.
    pub defect_count: usize,
    /// Count of rows whose auth default is system-browser or device-code safe.
    pub system_browser_auth_default_row_count: usize,
    /// Count of rows whose native approval fence is active.
    pub native_approval_fence_active_row_count: usize,
    /// Count of rows with browser handoff packet refs.
    pub browser_handoff_row_count: usize,
    /// Surface-family tokens present in the page.
    pub surface_family_tokens_present: Vec<String>,
    /// Boundary-state tokens present in the page.
    pub boundary_state_tokens_present: Vec<String>,
    /// Browser fallback posture tokens present in the page.
    pub browser_fallback_postures_present: Vec<String>,
}

impl EmbeddedBoundaryToolkitSummary {
    fn from_rows(
        rows: &[EmbeddedBoundaryToolkitRow],
        support_export: &EmbeddedBoundaryToolkitSupportExport,
        defects: &[EmbeddedBoundaryToolkitDefect],
    ) -> Self {
        let mut surface_family_tokens_present = Vec::new();
        let mut boundary_state_tokens_present = Vec::new();
        let mut browser_fallback_postures_present = Vec::new();
        for row in rows {
            push_unique(
                &mut surface_family_tokens_present,
                row.surface_family_token.clone(),
            );
            push_unique(
                &mut boundary_state_tokens_present,
                row.boundary_state_token.clone(),
            );
            push_unique(
                &mut browser_fallback_postures_present,
                row.browser_fallback_posture_token.clone(),
            );
        }
        Self {
            row_count: rows.len(),
            support_row_count: support_export.rows.len(),
            event_count: support_export.event_log.len(),
            defect_count: defects.len(),
            system_browser_auth_default_row_count: rows
                .iter()
                .filter(|row| row.system_browser_auth_default)
                .count(),
            native_approval_fence_active_row_count: rows
                .iter()
                .filter(|row| row.native_approval_fence_active)
                .count(),
            browser_handoff_row_count: rows
                .iter()
                .filter(|row| row.browser_handoff_packet_ref.is_some())
                .count(),
            surface_family_tokens_present,
            boundary_state_tokens_present,
            browser_fallback_postures_present,
        }
    }
}

/// Top-level toolkit page consumed by shell UI, docs, fixtures, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryToolkitPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Toolkit schema version.
    pub schema_version: u32,
    /// Shared toolkit contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// Timestamp used by seeded fixtures.
    pub generated_at: String,
    /// Source audit page id.
    pub audit_page_id_ref: String,
    /// Source audit shared contract ref.
    pub audit_shared_contract_ref: String,
    /// Summary banner.
    pub summary: EmbeddedBoundaryToolkitSummary,
    /// Renderable toolkit rows.
    pub rows: Vec<EmbeddedBoundaryToolkitRow>,
    /// Event log for incident reconstruction.
    pub event_log: Vec<EmbeddedBoundaryToolkitEvent>,
    /// Metadata-safe support export.
    pub support_export: EmbeddedBoundaryToolkitSupportExport,
    /// Toolkit defects.
    pub defects: Vec<EmbeddedBoundaryToolkitDefect>,
}

/// Builds the seeded embedded-boundary toolkit page.
pub fn seeded_embedded_boundary_toolkit_page() -> EmbeddedBoundaryToolkitPage {
    let generated_at = "2026-05-17T00:00:00Z";
    let audit_page = seeded_embedded_boundary_audit_page();
    EmbeddedBoundaryToolkitPage::from_audit_page(audit_page, generated_at)
}

impl EmbeddedBoundaryToolkitPage {
    /// Projects a toolkit page from the audited embedded-boundary page.
    pub fn from_audit_page(
        audit_page: EmbeddedBoundaryAuditPage,
        generated_at: impl Into<String>,
    ) -> Self {
        let generated_at = generated_at.into();
        let support_index: BTreeMap<&str, &EmbeddedBoundaryAuditSupportRow> = audit_page
            .support_rows
            .iter()
            .map(|row| (row.row_id.as_str(), row))
            .collect();
        let mut rows = Vec::new();
        let mut defects = audit_defects_as_toolkit_defects(&audit_page.defects);
        for row in &audit_page.rows {
            if let Some(support) = support_index.get(row.row_id.as_str()) {
                rows.push(EmbeddedBoundaryToolkitRow::from_audit_row(row, support));
            } else {
                defects.push(EmbeddedBoundaryToolkitDefect::new(
                    EmbeddedBoundaryToolkitDefectKind::SupportExportParityDrift,
                    row.row_id.clone(),
                    "support_row",
                    "audit page did not carry a support row for this embedded surface",
                ));
            }
        }

        let mut event_log = Vec::new();
        for row in &rows {
            event_log.extend(events_for_row(row, &generated_at));
        }

        let support_export = EmbeddedBoundaryToolkitSupportExport::from_rows(
            "support-export:embedded-boundary-toolkit:001",
            generated_at.clone(),
            &rows,
            &event_log,
            &defects,
        );

        defects.extend(validate_toolkit_parts(&rows, &event_log, &support_export));
        let support_export = EmbeddedBoundaryToolkitSupportExport::from_rows(
            "support-export:embedded-boundary-toolkit:001",
            generated_at.clone(),
            &rows,
            &event_log,
            &defects,
        );
        let summary = EmbeddedBoundaryToolkitSummary::from_rows(&rows, &support_export, &defects);

        Self {
            record_kind: EMBEDDED_BOUNDARY_TOOLKIT_PAGE_RECORD_KIND.to_owned(),
            schema_version: EMBEDDED_BOUNDARY_TOOLKIT_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_TOOLKIT_SHARED_CONTRACT_REF.to_owned(),
            page_id: "shell:embedded-boundary-toolkit:page:default".to_owned(),
            page_label: "Embedded boundary toolkit: owner/origin chrome, browser handoff, native approval fences, and support export".to_owned(),
            generated_at,
            audit_page_id_ref: audit_page.page_id,
            audit_shared_contract_ref: audit_page.shared_contract_ref,
            summary,
            rows,
            event_log,
            support_export,
            defects,
        }
    }

    /// Returns true when every claimed beta embedded surface family is present.
    pub fn covers_required_surface_families(&self) -> bool {
        let required = [
            "embedded_docs_help",
            "extension_hosted_surface",
            "embedded_marketplace_or_account",
            "embedded_service_dashboard",
            "embedded_auth_confirmation",
        ];
        required.iter().all(|token| {
            self.summary
                .surface_family_tokens_present
                .iter()
                .any(|v| v == token)
        })
    }
}

/// Validates that the toolkit page has no source-audit or toolkit defects.
pub fn validate_embedded_boundary_toolkit_page(
    page: &EmbeddedBoundaryToolkitPage,
) -> Result<(), Vec<EmbeddedBoundaryToolkitDefect>> {
    let mut defects = page.defects.clone();
    defects.extend(validate_toolkit_parts(
        &page.rows,
        &page.event_log,
        &page.support_export,
    ));
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

fn audit_defects_as_toolkit_defects(
    defects: &[EmbeddedBoundaryAuditDefect],
) -> Vec<EmbeddedBoundaryToolkitDefect> {
    defects
        .iter()
        .map(|defect| {
            EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::SourceAuditDefectsPresent,
                defect.row_id.clone(),
                defect.field.clone(),
                format!(
                    "source embedded-boundary audit defect {} must be fixed before toolkit projection is trusted",
                    defect.defect_kind_token
                ),
            )
        })
        .collect()
}

fn validate_toolkit_parts(
    rows: &[EmbeddedBoundaryToolkitRow],
    event_log: &[EmbeddedBoundaryToolkitEvent],
    support_export: &EmbeddedBoundaryToolkitSupportExport,
) -> Vec<EmbeddedBoundaryToolkitDefect> {
    let mut defects = Vec::new();
    let event_index: BTreeMap<&str, Vec<&EmbeddedBoundaryToolkitEvent>> = {
        let mut index: BTreeMap<&str, Vec<&EmbeddedBoundaryToolkitEvent>> = BTreeMap::new();
        for event in event_log {
            index
                .entry(event.toolkit_row_id_ref.as_str())
                .or_default()
                .push(event);
        }
        index
    };
    let support_index: BTreeMap<&str, &EmbeddedBoundaryToolkitSupportRow> = support_export
        .rows
        .iter()
        .map(|row| (row.toolkit_row_id_ref.as_str(), row))
        .collect();

    for row in rows {
        if row.owner_label.trim().is_empty()
            || row.host_or_domain_label.trim().is_empty()
            || row.origin_label.trim().is_empty()
        {
            defects.push(EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::MissingOwnerOriginChrome,
                row.toolkit_row_id.clone(),
                "owner_origin_chrome",
                "toolkit row must quote owner label, origin label, and host/domain",
            ));
        }

        let events = event_index
            .get(row.toolkit_row_id.as_str())
            .cloned()
            .unwrap_or_default();
        if !events.iter().any(|event| {
            event.event_class == EmbeddedBoundaryToolkitEventClass::NativeApprovalFenceConfirmed
        }) {
            defects.push(EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::MissingNativeApprovalFence,
                row.toolkit_row_id.clone(),
                "event_log",
                "row must emit a native approval fence confirmation event",
            ));
        }
        if row.browser_handoff_packet_ref.is_some()
            && !events.iter().any(|event| {
                event.event_class == EmbeddedBoundaryToolkitEventClass::BrowserHandoffDeclared
                    && event.browser_handoff_packet_ref == row.browser_handoff_packet_ref
            })
        {
            defects.push(EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::MissingBrowserHandoffEvent,
                row.toolkit_row_id.clone(),
                "event_log",
                "row with browser handoff packet ref must emit a matching handoff event",
            ));
        }
        if row.surface_family == SurfaceFamily::EmbeddedAuthConfirmation
            && !row.system_browser_auth_default
        {
            defects.push(EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::IdentityRowNotSystemBrowserDefault,
                row.toolkit_row_id.clone(),
                "system_browser_auth_default",
                "auth confirmation rows must default to system browser or device-code fallback",
            ));
        }
        if !row.native_approval_fence_active {
            defects.push(EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::MissingNativeApprovalFence,
                row.toolkit_row_id.clone(),
                "native_approval_surface_tokens",
                "row dropped at least one native high-risk approval surface",
            ));
        }
        if events.iter().any(|event| {
            event.embedded_content_owner_label.trim().is_empty()
                || event.embedded_origin_label.trim().is_empty()
                || event.native_approval_owner_token != "host_product_native"
        }) {
            defects.push(EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::EventLogReconstructionGap,
                row.toolkit_row_id.clone(),
                "event_log",
                "event log must reconstruct embedded owner, origin, and native approval owner",
            ));
        }

        match support_index.get(row.toolkit_row_id.as_str()) {
            Some(support)
                if support.owner_label == row.owner_label
                    && support.host_or_domain_label == row.host_or_domain_label
                    && support.boundary_state_token == row.boundary_state_token
                    && support.browser_fallback_posture_token
                        == row.browser_fallback_posture_token
                    && support.browser_handoff_packet_ref == row.browser_handoff_packet_ref
                    && support.native_approval_surface_tokens
                        == row.native_approval_surface_tokens
                    && support.boundary_event_ids == row.boundary_event_ids => {}
            Some(_) => defects.push(EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::SupportExportParityDrift,
                row.toolkit_row_id.clone(),
                "support_export",
                "toolkit support export row drifted from the live toolkit row",
            )),
            None => defects.push(EmbeddedBoundaryToolkitDefect::new(
                EmbeddedBoundaryToolkitDefectKind::SupportExportParityDrift,
                row.toolkit_row_id.clone(),
                "support_export",
                "toolkit support export is missing a row",
            )),
        }
    }

    defects
}

fn events_for_row(
    row: &EmbeddedBoundaryToolkitRow,
    emitted_at: &str,
) -> Vec<EmbeddedBoundaryToolkitEvent> {
    let mut classes = vec![
        EmbeddedBoundaryToolkitEventClass::BoundaryChromeRendered,
        EmbeddedBoundaryToolkitEventClass::NativeApprovalFenceConfirmed,
        EmbeddedBoundaryToolkitEventClass::SupportExportProjected,
    ];
    if row.browser_handoff_packet_ref.is_some()
        || row.browser_fallback_posture_token == "external_open_blocked_by_policy"
    {
        classes.push(EmbeddedBoundaryToolkitEventClass::BrowserHandoffDeclared);
    }
    if row.surface_family == SurfaceFamily::EmbeddedAuthConfirmation {
        classes.push(EmbeddedBoundaryToolkitEventClass::AuthHandoffDefaultRecorded);
    }
    classes
        .into_iter()
        .map(|class| EmbeddedBoundaryToolkitEvent::from_row(row, class, emitted_at))
        .collect()
}

fn event_ids_for_row(
    toolkit_row_id: &str,
    surface_family: SurfaceFamily,
    handoff_ref: Option<&str>,
    fallback_posture: &str,
    auth_flow: Option<&str>,
) -> Vec<String> {
    let mut classes = vec![
        EmbeddedBoundaryToolkitEventClass::BoundaryChromeRendered,
        EmbeddedBoundaryToolkitEventClass::NativeApprovalFenceConfirmed,
        EmbeddedBoundaryToolkitEventClass::SupportExportProjected,
    ];
    if handoff_ref.is_some() || fallback_posture == "external_open_blocked_by_policy" {
        classes.push(EmbeddedBoundaryToolkitEventClass::BrowserHandoffDeclared);
    }
    if surface_family == SurfaceFamily::EmbeddedAuthConfirmation && auth_flow.is_some() {
        classes.push(EmbeddedBoundaryToolkitEventClass::AuthHandoffDefaultRecorded);
    }
    classes
        .into_iter()
        .map(|class| event_id_for_class(toolkit_row_id, class))
        .collect()
}

fn event_id_for_class(row_id: &str, event_class: EmbeddedBoundaryToolkitEventClass) -> String {
    format!("event:{}:{}", event_class.as_str(), row_id)
}

fn source_version_freshness_label(row: &EmbeddedBoundaryAuditRow) -> String {
    match row.surface_family {
        SurfaceFamily::EmbeddedDocsHelp => {
            format!(
                "Docs/help source is {}; version and freshness are represented by {}.",
                row.data_boundary_label, row.boundary_state_label
            )
        }
        SurfaceFamily::ExtensionHostedSurface => {
            format!(
                "Extension publisher surface from {}; state {}.",
                row.publisher_or_service_label, row.boundary_state_label
            )
        }
        SurfaceFamily::EmbeddedMarketplaceOrAccount => {
            format!(
                "Marketplace/account scope {}; provider health {}.",
                row.provider_scope_label
                    .as_deref()
                    .unwrap_or("not declared"),
                row.provider_health_token
                    .as_deref()
                    .unwrap_or("not declared")
            )
        }
        SurfaceFamily::EmbeddedServiceDashboard => {
            format!(
                "Service dashboard scope {}; provider health {}.",
                row.provider_scope_label
                    .as_deref()
                    .unwrap_or("not declared"),
                row.provider_health_token
                    .as_deref()
                    .unwrap_or("not declared")
            )
        }
        SurfaceFamily::EmbeddedAuthConfirmation => {
            format!(
                "Auth handoff for {}; flow {}.",
                row.auth_provider_domain_label
                    .as_deref()
                    .unwrap_or(row.host_or_domain_label.as_str()),
                row.auth_flow_class_token
                    .as_deref()
                    .unwrap_or("not_declared")
            )
        }
    }
}

fn network_or_offline_posture_label(row: &EmbeddedBoundaryAuditRow) -> String {
    if let Some(provider_health) = row.provider_health_token.as_deref() {
        return format!(
            "{} via provider health {}; fallback {}.",
            row.boundary_state_label, provider_health, row.browser_fallback_posture_token
        );
    }
    format!(
        "{}; fallback {}.",
        row.boundary_state_label, row.browser_fallback_posture_token
    )
}

fn auth_reason_code(row: &EmbeddedBoundaryAuditRow) -> String {
    match row.auth_flow_class_token.as_deref() {
        Some("system_browser") => "system_browser_primary".to_owned(),
        Some("device_code") => "device_code_fallback".to_owned(),
        Some("platform_authenticator_native") => "platform_authenticator_native".to_owned(),
        Some("embedded_session_refresh") => "embedded_session_refresh_exception".to_owned(),
        Some("embedded_password_exception") => "embedded_password_exception".to_owned(),
        _ if row.surface_family == SurfaceFamily::EmbeddedAuthConfirmation => {
            "auth_flow_missing".to_owned()
        }
        _ => "not_auth_surface".to_owned(),
    }
}

fn auth_defaults_to_system_browser(row: &EmbeddedBoundaryAuditRow) -> bool {
    if row.surface_family != SurfaceFamily::EmbeddedAuthConfirmation {
        return false;
    }
    matches!(
        row.auth_flow_class_token.as_deref(),
        Some("system_browser") | Some("device_code") | Some("platform_authenticator_native")
    )
}

fn privacy_consequence_label(row: &EmbeddedBoundaryToolkitRow) -> &'static str {
    match row.redaction_class_token.as_str() {
        "metadata_safe_default" => "metadata only; raw embedded body, cookies, secrets, and raw URLs excluded",
        "internal_support_restricted" => {
            "internal support metadata only; raw embedded body, cookies, secrets, and raw URLs excluded"
        }
        "operator_only_restricted" => {
            "operator-only metadata; raw embedded body, cookies, secrets, and raw URLs excluded"
        }
        "signing_evidence_only" => "signing evidence only; raw embedded body excluded",
        _ => "metadata only; raw embedded body excluded",
    }
}

fn support_matches_row(
    row: &EmbeddedBoundaryAuditRow,
    support: &EmbeddedBoundaryAuditSupportRow,
) -> bool {
    support.surface_family_token == row.surface_family_token
        && support.owner_label == row.owner_label
        && support.host_or_domain_label == row.host_or_domain_label
        && support.data_boundary_class_token == row.data_boundary_class_token
        && support.boundary_state_token == row.boundary_state_token
        && support.permission_class_token == row.permission_class_token
        && support.browser_fallback_posture_token == row.browser_fallback_posture_token
        && support.fallback_target_class_token == row.fallback_target_class_token
        && support.browser_handoff_packet_ref == row.browser_handoff_packet_ref
        && support.identity_mode_token == row.identity_mode_token
        && support.trust_state_token == row.trust_state_token
}

fn required_native_approval_surfaces() -> [&'static str; 6] {
    [
        "product_security_messaging",
        "update_verification",
        "workspace_trust_elevation",
        "rollback_or_restore_confirmation",
        "ai_apply_review",
        "high_risk_approval_sheet",
    ]
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}

/// Validates the source audit page before building toolkit rows.
pub fn validate_source_audit_page(
    page: &EmbeddedBoundaryAuditPage,
) -> Result<(), Vec<EmbeddedBoundaryAuditDefect>> {
    validate_embedded_boundary_audit_page(page)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_toolkit_validates_and_covers_every_surface() {
        let page = seeded_embedded_boundary_toolkit_page();
        validate_embedded_boundary_toolkit_page(&page).expect("seeded toolkit validates");
        assert!(page.covers_required_surface_families());
        assert_eq!(page.summary.row_count, 5);
        assert_eq!(page.summary.defect_count, 0);
        assert_eq!(page.summary.native_approval_fence_active_row_count, 5);
        assert!(page.summary.event_count >= page.summary.row_count * 3);
    }

    #[test]
    fn auth_row_defaults_to_system_browser_and_records_handoff_event() {
        let page = seeded_embedded_boundary_toolkit_page();
        let row = page
            .rows
            .iter()
            .find(|row| row.surface_family == SurfaceFamily::EmbeddedAuthConfirmation)
            .expect("auth row");
        assert!(row.system_browser_auth_default);
        assert_eq!(row.auth_reason_code, "system_browser_primary");
        assert_eq!(
            row.browser_handoff_packet_ref.as_deref(),
            Some("id:browser-handoff:auth:github")
        );
        assert!(page.event_log.iter().any(|event| {
            event.toolkit_row_id_ref == row.toolkit_row_id
                && event.event_class
                    == EmbeddedBoundaryToolkitEventClass::AuthHandoffDefaultRecorded
                && event.browser_handoff_packet_ref == row.browser_handoff_packet_ref
        }));
    }

    #[test]
    fn event_log_reconstructs_owner_origin_native_owner_and_support_row() {
        let page = seeded_embedded_boundary_toolkit_page();
        for event in &page.event_log {
            assert!(!event.embedded_content_owner_label.is_empty());
            assert!(!event.embedded_origin_label.is_empty());
            assert_eq!(event.native_approval_owner_token, "host_product_native");
            assert!(page
                .support_export
                .rows
                .iter()
                .any(|row| row.support_row_id_ref == event.support_row_id_ref));
        }
    }

    #[test]
    fn validator_flags_auth_row_without_system_browser_default() {
        let mut page = seeded_embedded_boundary_toolkit_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.surface_family == SurfaceFamily::EmbeddedAuthConfirmation)
            .expect("auth row");
        row.system_browser_auth_default = false;
        let defects = validate_embedded_boundary_toolkit_page(&page).unwrap_err();
        assert!(defects.iter().any(|defect| {
            defect.defect_kind
                == EmbeddedBoundaryToolkitDefectKind::IdentityRowNotSystemBrowserDefault
        }));
    }

    #[test]
    fn validator_flags_missing_browser_handoff_event() {
        let mut page = seeded_embedded_boundary_toolkit_page();
        let row_id = page
            .rows
            .iter()
            .find(|row| row.browser_handoff_packet_ref.is_some())
            .expect("handoff row")
            .toolkit_row_id
            .clone();
        page.event_log.retain(|event| {
            !(event.toolkit_row_id_ref == row_id
                && event.event_class == EmbeddedBoundaryToolkitEventClass::BrowserHandoffDeclared)
        });
        page.support_export.event_log = page.event_log.clone();
        let defects = validate_embedded_boundary_toolkit_page(&page).unwrap_err();
        assert!(defects.iter().any(|defect| {
            defect.defect_kind == EmbeddedBoundaryToolkitDefectKind::MissingBrowserHandoffEvent
        }));
    }

    #[test]
    fn validator_flags_support_export_drift() {
        let mut page = seeded_embedded_boundary_toolkit_page();
        page.support_export.rows[0].owner_label = "different owner".to_owned();
        let defects = validate_embedded_boundary_toolkit_page(&page).unwrap_err();
        assert!(defects.iter().any(|defect| {
            defect.defect_kind == EmbeddedBoundaryToolkitDefectKind::SupportExportParityDrift
        }));
    }
}
