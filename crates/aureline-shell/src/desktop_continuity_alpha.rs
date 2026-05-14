//! Native desktop continuity alpha packet and support projection.
//!
//! This module composes the existing native handoff, window-display,
//! restore-provenance, notification, and secret-broker contracts into one
//! reviewable alpha packet. It does not mint alternate OS-entry or credential
//! truth; it quotes the first consuming shell and auth records so support
//! export can reconstruct interruption cause, continuity state, recovery
//! choice, and resulting fidelity without scraping UI text.

use std::collections::BTreeSet;
use std::path::Path;

use aureline_auth::SecretBrokerAlphaPacket;
use aureline_commands::invocation::now_rfc3339;
use serde::{Deserialize, Serialize};

use crate::deeplink::native_handoff::{
    seeded_native_boundary_handoff_packet, NativeBoundaryHandoffPacket,
    NativeBoundaryHandoffReviewRecord, NativeFileHandoffReviewRecord, SourceSurfaceClass,
};

/// Stable record kind for [`DesktopContinuityAlphaPacket`] payloads.
pub const DESKTOP_CONTINUITY_ALPHA_PACKET_RECORD_KIND: &str =
    "desktop_continuity_alpha_packet_record";

/// Stable record kind for [`DesktopContinuitySupportExport`] payloads.
pub const DESKTOP_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "desktop_continuity_support_export_record";

/// Stable record kind for [`DesktopContinuitySupportExportRow`] payloads.
pub const DESKTOP_CONTINUITY_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "desktop_continuity_support_export_row_record";

/// Schema version for desktop-continuity alpha payloads.
pub const DESKTOP_CONTINUITY_ALPHA_SCHEMA_VERSION: u32 = 1;

/// OS-entry review row proving exact target and owner disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopEntryContinuityRow {
    /// Stable row id safe for logs, support export, and fixtures.
    pub row_id: String,
    /// Source surface token from the native handoff model.
    pub source_surface_token: String,
    /// Exact literal target label delivered by the OS surface.
    pub literal_target_label: String,
    /// Export-safe literal target ref.
    pub literal_target_ref: String,
    /// Resulting product mode or route token.
    pub resulting_mode_token: String,
    /// Command id selected by the product-owned review path.
    pub command_id_ref: String,
    /// Canonical object identity ref resolved from the literal target.
    pub object_identity_ref: String,
    /// Target availability token.
    pub availability_class_token: String,
    /// Target freshness token.
    pub freshness_class_token: String,
    /// Owning channel ref for side-by-side or handler ownership review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owning_channel_ref: Option<String>,
    /// Owner build ref for side-by-side or handler ownership review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_build_ref: Option<String>,
    /// Trust/profile boundary token visible before risky execution.
    pub trust_profile_boundary_token: String,
    /// Review surface selected for this entry.
    pub review_surface_token: String,
    /// True when the entry can execute after any required review.
    pub execution_allowed: bool,
    /// True when a summary-only OS affordance cannot execute directly.
    pub direct_os_execution_forbidden: bool,
    /// True when placeholder recovery is the resulting mode.
    pub placeholder_recovery_required: bool,
    /// Safe recovery actions surfaced for degraded or denied entries.
    pub recovery_action_tokens: Vec<String>,
    /// Source native handoff review row.
    pub native_review_ref: String,
    /// Support row id for this entry.
    pub support_export_ref: String,
}

impl DesktopEntryContinuityRow {
    fn from_handoff_review(
        source: SourceSurfaceClass,
        review: &NativeBoundaryHandoffReviewRecord,
    ) -> Self {
        let literal = literal_target_for(source, review);
        Self {
            row_id: format!("desktop-continuity.entry.{}", review.source_surface_token),
            source_surface_token: review.source_surface_token.clone(),
            literal_target_ref: literal.ref_id,
            literal_target_label: literal.label,
            resulting_mode_token: review.route_class_token.clone(),
            command_id_ref: review.command_id_ref.clone(),
            object_identity_ref: review.target.object_identity_ref.clone(),
            availability_class_token: review.target.availability_class_token.clone(),
            freshness_class_token: review.target.freshness_class_token.clone(),
            owning_channel_ref: review.owning_channel_ref.clone(),
            owner_build_ref: review.owner_build_ref.clone(),
            trust_profile_boundary_token: trust_profile_boundary_token(review),
            review_surface_token: review.review_surface_token.clone(),
            execution_allowed: review.execution_allowed,
            direct_os_execution_forbidden: review.direct_os_execution_forbidden,
            placeholder_recovery_required: review.placeholder_recovery_required,
            recovery_action_tokens: review
                .recovery_actions
                .iter()
                .map(|action| action.action_token.clone())
                .collect(),
            native_review_ref: review.review_id.clone(),
            support_export_ref: format!(
                "support.desktop_continuity.entry.{}",
                review.source_surface_token
            ),
        }
    }
}

/// Missing target or external-path recovery card row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopTargetRecoveryRow {
    /// Stable row id safe for logs, support export, and fixtures.
    pub row_id: String,
    /// Target kind token.
    pub target_kind_token: String,
    /// Literal target label or last-known root label.
    pub literal_target_label: String,
    /// Canonical target identity ref.
    pub object_identity_ref: String,
    /// Availability token that forced recovery.
    pub availability_class_token: String,
    /// Freshness token carried into the placeholder.
    pub freshness_class_token: String,
    /// Review or placeholder surface token.
    pub recovery_surface_token: String,
    /// Safe recovery action tokens.
    pub recovery_action_tokens: Vec<String>,
    /// True when the original source and target identity survive recovery.
    pub preserves_user_intent: bool,
    /// Source review row or fixture ref.
    pub source_ref: String,
    /// Support row id for this recovery row.
    pub support_export_ref: String,
}

/// Display-topology and mixed-DPI continuity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopTopologyContinuityRow {
    /// Stable row id safe for logs, support export, and fixtures.
    pub row_id: String,
    /// Fixture or verification-case ref backing this row.
    pub source_fixture_ref: String,
    /// Event class token.
    pub event_class_token: String,
    /// Display context token.
    pub display_context_token: String,
    /// Topology change tokens recorded for diagnostics and support.
    pub topology_change_tokens: Vec<String>,
    /// Adjustment tokens recorded for diagnostics and support.
    pub adjustment_tokens: Vec<String>,
    /// Resulting continuity state tokens.
    pub continuity_state_tokens: Vec<String>,
    /// Resulting restore fidelity token.
    pub resulting_fidelity_token: String,
    /// True when placement was downgraded from exact geometry.
    pub topology_adjustment_downgraded_fidelity: bool,
    /// True when windows remain visible after the adjustment.
    pub visible_bounds_preserved: bool,
    /// True when the dominant pane or focus target intent survives.
    pub focus_intent_preserved: bool,
    /// Safe recovery action tokens.
    pub recovery_action_tokens: Vec<String>,
    /// Support row id for this topology row.
    pub support_export_ref: String,
}

/// Sleep, wake, network-transition, or removable-root interruption row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopLifecycleInterruptionRow {
    /// Stable row id safe for logs, support export, and fixtures.
    pub row_id: String,
    /// Fixture ref backing this row.
    pub source_fixture_ref: String,
    /// Lifecycle event token.
    pub lifecycle_event_token: String,
    /// Target identity ref affected by the interruption.
    pub object_identity_ref: String,
    /// Trust or profile boundary token.
    pub trust_profile_boundary_token: String,
    /// Visible state tokens, such as reconnecting or local fallback.
    pub continuity_state_tokens: Vec<String>,
    /// Resulting restore fidelity token.
    pub resulting_fidelity_token: String,
    /// True when local non-credential work remains available.
    pub local_work_continues: bool,
    /// True when privileged or mutating work is held for review.
    pub privileged_or_mutating_work_paused: bool,
    /// True when rerun or authority reacquisition is forbidden silently.
    pub no_silent_rerun_or_authority_reuse: bool,
    /// Safe recovery action tokens.
    pub recovery_action_tokens: Vec<String>,
    /// Support row id for this lifecycle row.
    pub support_export_ref: String,
}

/// Credential-store interruption row projected from the secret broker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStoreInterruptionRow {
    /// Stable row id safe for logs, support export, and fixtures.
    pub row_id: String,
    /// Source secret-broker row ref.
    pub secret_broker_row_ref: String,
    /// Affected capability tokens.
    pub affected_capability_tokens: Vec<String>,
    /// Store class token.
    pub trust_store_class_token: String,
    /// Unlock state token.
    pub unlock_state_token: String,
    /// Continuity state token.
    pub continuity_state_token: String,
    /// Denial reason token, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_token: Option<String>,
    /// Local-continuation token.
    pub local_continuation_token: String,
    /// True when local non-credential work remains available.
    pub local_work_continues: bool,
    /// True when credentialed actions are paused.
    pub credentialed_actions_paused: bool,
    /// True when plaintext downgrade is forbidden.
    pub plaintext_downgrade_forbidden: bool,
    /// True when stale authority reuse is forbidden.
    pub stale_authority_reuse_forbidden: bool,
    /// Safe recovery action tokens.
    pub recovery_action_tokens: Vec<String>,
    /// Source support-export row ref.
    pub source_support_export_ref: String,
    /// Support row id for this credential interruption row.
    pub support_export_ref: String,
}

/// Canonical packet for the bounded native desktop continuity alpha wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopContinuityAlphaPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Running build identity ref.
    pub build_identity_ref: String,
    /// Export-safe timestamp.
    pub generated_at: String,
    /// Source native-boundary handoff packet id.
    pub native_boundary_handoff_packet_ref: String,
    /// Source secret-broker packet id.
    pub secret_broker_packet_ref: String,
    /// OS-entry continuity rows.
    pub entry_rows: Vec<DesktopEntryContinuityRow>,
    /// Missing target and external-path recovery rows.
    pub target_recovery_rows: Vec<DesktopTargetRecoveryRow>,
    /// Display topology and mixed-DPI rows.
    pub topology_rows: Vec<DesktopTopologyContinuityRow>,
    /// Sleep, wake, network-transition, and removable-root rows.
    pub lifecycle_rows: Vec<DesktopLifecycleInterruptionRow>,
    /// Credential-store interruption rows.
    pub credential_store_rows: Vec<CredentialStoreInterruptionRow>,
}

impl DesktopContinuityAlphaPacket {
    /// Validates that the packet covers the required alpha interruption rows.
    pub fn validate(&self) -> Result<(), DesktopContinuityValidationError> {
        if self.record_kind != DESKTOP_CONTINUITY_ALPHA_PACKET_RECORD_KIND {
            return Err(DesktopContinuityValidationError::InvalidRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if self.schema_version != DESKTOP_CONTINUITY_ALPHA_SCHEMA_VERSION {
            return Err(DesktopContinuityValidationError::InvalidSchemaVersion {
                actual: self.schema_version,
            });
        }

        let entry_sources = self
            .entry_rows
            .iter()
            .map(|row| row.source_surface_token.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "system_open",
            "file_association",
            "default_browser_callback",
            "dock_taskbar_recent",
            "dock_taskbar_jump_action",
        ] {
            if !entry_sources.contains(required) {
                return Err(DesktopContinuityValidationError::MissingEntrySurface {
                    surface: required.to_owned(),
                });
            }
        }

        if self
            .entry_rows
            .iter()
            .any(|row| row.literal_target_label.trim().is_empty())
        {
            return Err(DesktopContinuityValidationError::MissingLiteralTarget);
        }
        if self.target_recovery_rows.is_empty()
            || self.topology_rows.is_empty()
            || self.lifecycle_rows.is_empty()
            || self.credential_store_rows.is_empty()
        {
            return Err(DesktopContinuityValidationError::MissingInterruptionRows);
        }
        if self
            .topology_rows
            .iter()
            .any(|row| !row.visible_bounds_preserved || !row.focus_intent_preserved)
        {
            return Err(DesktopContinuityValidationError::UnsafeTopologyRestore);
        }
        if self
            .lifecycle_rows
            .iter()
            .any(|row| !row.no_silent_rerun_or_authority_reuse)
        {
            return Err(DesktopContinuityValidationError::SilentRerunAllowed);
        }
        if self
            .credential_store_rows
            .iter()
            .any(|row| !row.plaintext_downgrade_forbidden || !row.stale_authority_reuse_forbidden)
        {
            return Err(DesktopContinuityValidationError::CredentialDowngradeAllowed);
        }

        Ok(())
    }

    /// Builds a support/export projection for interruption reconstruction.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> DesktopContinuitySupportExport {
        let rows = self
            .entry_rows
            .iter()
            .map(DesktopContinuitySupportExportRow::from_entry)
            .chain(
                self.target_recovery_rows
                    .iter()
                    .map(DesktopContinuitySupportExportRow::from_target_recovery),
            )
            .chain(
                self.topology_rows
                    .iter()
                    .map(DesktopContinuitySupportExportRow::from_topology),
            )
            .chain(
                self.lifecycle_rows
                    .iter()
                    .map(DesktopContinuitySupportExportRow::from_lifecycle),
            )
            .chain(
                self.credential_store_rows
                    .iter()
                    .map(DesktopContinuitySupportExportRow::from_credential_store),
            )
            .collect::<Vec<_>>();

        DesktopContinuitySupportExport {
            record_kind: DESKTOP_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DESKTOP_CONTINUITY_ALPHA_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            packet_ref: self.packet_id.clone(),
            rows,
            reconstructs_interruption_cause: true,
            reconstructs_continuity_state: true,
            reconstructs_recovery_choice: true,
            reconstructs_resulting_fidelity: true,
            ui_text_scrape_required: false,
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
        }
    }
}

/// Metadata-only support/export projection for the desktop continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopContinuitySupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export-safe timestamp.
    pub generated_at: String,
    /// Source packet ref.
    pub packet_ref: String,
    /// Export rows.
    pub rows: Vec<DesktopContinuitySupportExportRow>,
    /// True when the cause can be reconstructed without UI text.
    pub reconstructs_interruption_cause: bool,
    /// True when the continuity state can be reconstructed without UI text.
    pub reconstructs_continuity_state: bool,
    /// True when the recovery choice can be reconstructed without UI text.
    pub reconstructs_recovery_choice: bool,
    /// True when fidelity can be reconstructed without UI text.
    pub reconstructs_resulting_fidelity: bool,
    /// True when support tooling must scrape rendered prose.
    pub ui_text_scrape_required: bool,
    /// Always false for this export.
    pub raw_secret_values_exported: bool,
    /// Always false for this export.
    pub raw_handle_ids_exported: bool,
}

impl DesktopContinuitySupportExport {
    /// True when the projection is safe for default support bundles.
    pub fn redaction_safe(&self) -> bool {
        !self.ui_text_scrape_required
            && !self.raw_secret_values_exported
            && !self.raw_handle_ids_exported
            && self
                .rows
                .iter()
                .all(|row| !row.raw_secret_values_exported && !row.raw_handle_ids_exported)
    }
}

/// One metadata-only support/export row for a continuity interruption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopContinuitySupportExportRow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable export row id.
    pub export_row_id: String,
    /// Source row id.
    pub source_row_ref: String,
    /// Row family token.
    pub row_family_token: String,
    /// Interruption cause token.
    pub interruption_cause_token: String,
    /// Continuity state tokens.
    pub continuity_state_tokens: Vec<String>,
    /// Recovery choice tokens.
    pub recovery_choice_tokens: Vec<String>,
    /// Resulting fidelity token.
    pub resulting_fidelity_token: String,
    /// Object or target identity ref.
    pub object_identity_ref: String,
    /// Source fixture or runtime packet ref.
    pub source_evidence_ref: String,
    /// Structured support fields used for reconstruction.
    pub support_field_refs: Vec<String>,
    /// Always false for this row.
    pub raw_secret_values_exported: bool,
    /// Always false for this row.
    pub raw_handle_ids_exported: bool,
}

impl DesktopContinuitySupportExportRow {
    fn from_entry(row: &DesktopEntryContinuityRow) -> Self {
        Self {
            record_kind: DESKTOP_CONTINUITY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            export_row_id: row.support_export_ref.clone(),
            source_row_ref: row.row_id.clone(),
            row_family_token: "os_entry".to_owned(),
            interruption_cause_token: row.source_surface_token.clone(),
            continuity_state_tokens: vec![
                row.availability_class_token.clone(),
                row.freshness_class_token.clone(),
                row.review_surface_token.clone(),
            ],
            recovery_choice_tokens: row.recovery_action_tokens.clone(),
            resulting_fidelity_token: if row.placeholder_recovery_required {
                "layout_only".to_owned()
            } else if row.execution_allowed {
                "exact_restore".to_owned()
            } else {
                "evidence_only".to_owned()
            },
            object_identity_ref: row.object_identity_ref.clone(),
            source_evidence_ref: row.native_review_ref.clone(),
            support_field_refs: vec![
                "desktop_continuity.entry.literal_target".to_owned(),
                "desktop_continuity.entry.owner_build_channel".to_owned(),
                "desktop_continuity.entry.trust_profile_boundary".to_owned(),
            ],
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
        }
    }

    fn from_target_recovery(row: &DesktopTargetRecoveryRow) -> Self {
        Self {
            record_kind: DESKTOP_CONTINUITY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            export_row_id: row.support_export_ref.clone(),
            source_row_ref: row.row_id.clone(),
            row_family_token: "target_recovery".to_owned(),
            interruption_cause_token: row.availability_class_token.clone(),
            continuity_state_tokens: vec![
                row.availability_class_token.clone(),
                row.freshness_class_token.clone(),
                row.recovery_surface_token.clone(),
            ],
            recovery_choice_tokens: row.recovery_action_tokens.clone(),
            resulting_fidelity_token: "layout_only".to_owned(),
            object_identity_ref: row.object_identity_ref.clone(),
            source_evidence_ref: row.source_ref.clone(),
            support_field_refs: vec![
                "desktop_continuity.target.identity".to_owned(),
                "desktop_continuity.target.safe_recovery_actions".to_owned(),
            ],
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
        }
    }

    fn from_topology(row: &DesktopTopologyContinuityRow) -> Self {
        Self {
            record_kind: DESKTOP_CONTINUITY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            export_row_id: row.support_export_ref.clone(),
            source_row_ref: row.row_id.clone(),
            row_family_token: "display_topology".to_owned(),
            interruption_cause_token: row.event_class_token.clone(),
            continuity_state_tokens: row.continuity_state_tokens.clone(),
            recovery_choice_tokens: row.recovery_action_tokens.clone(),
            resulting_fidelity_token: row.resulting_fidelity_token.clone(),
            object_identity_ref: row.display_context_token.clone(),
            source_evidence_ref: row.source_fixture_ref.clone(),
            support_field_refs: vec![
                "desktop_continuity.topology.change_tokens".to_owned(),
                "desktop_continuity.topology.adjustment_tokens".to_owned(),
                "desktop_continuity.topology.focus_visibility".to_owned(),
            ],
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
        }
    }

    fn from_lifecycle(row: &DesktopLifecycleInterruptionRow) -> Self {
        Self {
            record_kind: DESKTOP_CONTINUITY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            export_row_id: row.support_export_ref.clone(),
            source_row_ref: row.row_id.clone(),
            row_family_token: "lifecycle_interruption".to_owned(),
            interruption_cause_token: row.lifecycle_event_token.clone(),
            continuity_state_tokens: row.continuity_state_tokens.clone(),
            recovery_choice_tokens: row.recovery_action_tokens.clone(),
            resulting_fidelity_token: row.resulting_fidelity_token.clone(),
            object_identity_ref: row.object_identity_ref.clone(),
            source_evidence_ref: row.source_fixture_ref.clone(),
            support_field_refs: vec![
                "desktop_continuity.lifecycle.state_tokens".to_owned(),
                "desktop_continuity.lifecycle.no_silent_rerun".to_owned(),
                "desktop_continuity.lifecycle.safe_actions".to_owned(),
            ],
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
        }
    }

    fn from_credential_store(row: &CredentialStoreInterruptionRow) -> Self {
        Self {
            record_kind: DESKTOP_CONTINUITY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            export_row_id: row.support_export_ref.clone(),
            source_row_ref: row.row_id.clone(),
            row_family_token: "credential_store_interruption".to_owned(),
            interruption_cause_token: row
                .denial_reason_token
                .clone()
                .unwrap_or_else(|| row.continuity_state_token.clone()),
            continuity_state_tokens: vec![
                row.continuity_state_token.clone(),
                row.unlock_state_token.clone(),
                row.local_continuation_token.clone(),
            ],
            recovery_choice_tokens: row.recovery_action_tokens.clone(),
            resulting_fidelity_token: "local_only_fallback".to_owned(),
            object_identity_ref: row.secret_broker_row_ref.clone(),
            source_evidence_ref: row.source_support_export_ref.clone(),
            support_field_refs: vec![
                "desktop_continuity.credential_store.affected_capabilities".to_owned(),
                "desktop_continuity.credential_store.continuity_state".to_owned(),
                "desktop_continuity.credential_store.recovery_actions".to_owned(),
            ],
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
        }
    }
}

/// Validation error for a desktop continuity alpha packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopContinuityValidationError {
    /// The packet carried an unexpected record kind.
    InvalidRecordKind { actual: String },
    /// The packet carried an unexpected schema version.
    InvalidSchemaVersion { actual: u32 },
    /// A required OS-entry surface is missing.
    MissingEntrySurface { surface: String },
    /// An OS-entry row omitted literal target disclosure.
    MissingLiteralTarget,
    /// One or more interruption families are missing.
    MissingInterruptionRows,
    /// A topology row failed visible-bounds or focus preservation.
    UnsafeTopologyRestore,
    /// A lifecycle row allows hidden rerun or authority reuse.
    SilentRerunAllowed,
    /// A credential-store row allows plaintext downgrade or stale authority.
    CredentialDowngradeAllowed,
}

impl std::fmt::Display for DesktopContinuityValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRecordKind { actual } => {
                write!(
                    f,
                    "desktop continuity packet has invalid record kind {actual}"
                )
            }
            Self::InvalidSchemaVersion { actual } => write!(
                f,
                "desktop continuity packet has invalid schema version {actual}"
            ),
            Self::MissingEntrySurface { surface } => {
                write!(f, "desktop continuity packet is missing {surface}")
            }
            Self::MissingLiteralTarget => {
                write!(f, "desktop continuity entry row omitted literal target")
            }
            Self::MissingInterruptionRows => {
                write!(f, "desktop continuity packet is missing interruption rows")
            }
            Self::UnsafeTopologyRestore => {
                write!(
                    f,
                    "desktop continuity topology row did not preserve visibility"
                )
            }
            Self::SilentRerunAllowed => {
                write!(f, "desktop continuity lifecycle row allowed silent rerun")
            }
            Self::CredentialDowngradeAllowed => write!(
                f,
                "desktop continuity credential-store row allowed an unsafe downgrade"
            ),
        }
    }
}

impl std::error::Error for DesktopContinuityValidationError {}

/// Builds the seeded native desktop continuity alpha packet.
pub fn seeded_desktop_continuity_alpha_packet(
    build_identity_ref: impl Into<String>,
) -> DesktopContinuityAlphaPacket {
    let build_identity_ref = build_identity_ref.into();
    let native_packet = seeded_native_boundary_handoff_packet(build_identity_ref.clone());
    let secret_packet = credential_store_interruption_packet();
    let secret_support_export = secret_packet.support_export(
        "secret-support-export:desktop-continuity",
        "2026-05-14T00:10:00Z",
    );

    DesktopContinuityAlphaPacket {
        record_kind: DESKTOP_CONTINUITY_ALPHA_PACKET_RECORD_KIND.to_owned(),
        schema_version: DESKTOP_CONTINUITY_ALPHA_SCHEMA_VERSION,
        packet_id: "desktop-continuity-alpha:claimed-wedge".to_owned(),
        build_identity_ref,
        generated_at: now_rfc3339(),
        native_boundary_handoff_packet_ref: native_packet.packet_id.clone(),
        secret_broker_packet_ref: secret_packet.packet_id.clone(),
        entry_rows: entry_rows(&native_packet),
        target_recovery_rows: target_recovery_rows(&native_packet),
        topology_rows: topology_rows(),
        lifecycle_rows: lifecycle_rows(),
        credential_store_rows: credential_store_rows(&secret_packet, &secret_support_export),
    }
}

/// Writes the packet to `<evidence_root>/desktop_continuity_alpha_latest.json`.
pub fn write_desktop_continuity_alpha_log(
    evidence_root: &Path,
    packet: &DesktopContinuityAlphaPacket,
) -> Result<(), String> {
    std::fs::create_dir_all(evidence_root)
        .map_err(|err| format!("create desktop continuity evidence root failed: {err}"))?;
    let path = evidence_root.join("desktop_continuity_alpha_latest.json");
    let json = serde_json::to_string_pretty(packet)
        .map_err(|err| format!("serialize desktop continuity packet failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))
}

#[derive(Debug)]
struct LiteralTarget {
    label: String,
    ref_id: String,
}

fn literal_target_for(
    source: SourceSurfaceClass,
    review: &NativeBoundaryHandoffReviewRecord,
) -> LiteralTarget {
    let (label, ref_id) = match source {
        SourceSurfaceClass::SystemOpen => (
            "/Users/dev/workspaces/alpha-local".to_owned(),
            "literal:system-open:workspace-alpha-local".to_owned(),
        ),
        SourceSurfaceClass::FileAssociation => (
            "/Users/dev/src/demo/src/main.ts".to_owned(),
            "literal:file-association:demo-src-main".to_owned(),
        ),
        SourceSurfaceClass::DefaultBrowserCallback => (
            "aureline://auth/callback?session=obj:auth-session:oidc:01".to_owned(),
            "literal:auth-callback:oidc-session".to_owned(),
        ),
        SourceSurfaceClass::DockTaskbarRecent => (
            "dock/taskbar recent: payments workspace".to_owned(),
            "literal:dock-taskbar-recent:payments-workspace".to_owned(),
        ),
        SourceSurfaceClass::DockTaskbarJumpAction => (
            "dock/taskbar jump: reopen alpha workspace".to_owned(),
            "literal:dock-taskbar-jump:alpha-workspace".to_owned(),
        ),
        _ => (
            review.target.object_identity_ref.clone(),
            format!("literal:{}", review.target.object_identity_ref),
        ),
    };
    LiteralTarget { label, ref_id }
}

fn trust_profile_boundary_token(review: &NativeBoundaryHandoffReviewRecord) -> String {
    format!(
        "{}:{}",
        review.trust_state_token,
        review
            .tenant_or_workspace_scope_ref
            .as_deref()
            .unwrap_or("scope:unbound")
    )
}

fn entry_rows(packet: &NativeBoundaryHandoffPacket) -> Vec<DesktopEntryContinuityRow> {
    [
        SourceSurfaceClass::SystemOpen,
        SourceSurfaceClass::FileAssociation,
        SourceSurfaceClass::DefaultBrowserCallback,
        SourceSurfaceClass::DockTaskbarRecent,
        SourceSurfaceClass::DockTaskbarJumpAction,
    ]
    .into_iter()
    .filter_map(|source| {
        packet
            .handoff_for_source(source)
            .map(|review| DesktopEntryContinuityRow::from_handoff_review(source, review))
    })
    .collect()
}

fn target_recovery_rows(packet: &NativeBoundaryHandoffPacket) -> Vec<DesktopTargetRecoveryRow> {
    let mut rows = packet
        .handoff_reviews
        .iter()
        .filter(|review| review.placeholder_recovery_required)
        .map(target_recovery_from_handoff_review)
        .collect::<Vec<_>>();

    rows.extend(
        packet
            .native_file_reviews
            .iter()
            .filter(|review| {
                review.target_kind_token == "removable_volume"
                    || review.availability_class_token == "missing_or_unmounted"
            })
            .map(target_recovery_from_native_file_review),
    );
    rows
}

fn target_recovery_from_handoff_review(
    review: &NativeBoundaryHandoffReviewRecord,
) -> DesktopTargetRecoveryRow {
    DesktopTargetRecoveryRow {
        row_id: format!(
            "desktop-continuity.target-recovery.{}",
            review.source_surface_token
        ),
        target_kind_token: review.target.target_kind_token.clone(),
        literal_target_label: review.target.object_identity_ref.clone(),
        object_identity_ref: review.target.object_identity_ref.clone(),
        availability_class_token: review.target.availability_class_token.clone(),
        freshness_class_token: review.target.freshness_class_token.clone(),
        recovery_surface_token: review.review_surface_token.clone(),
        recovery_action_tokens: review
            .recovery_actions
            .iter()
            .map(|action| action.action_token.clone())
            .collect(),
        preserves_user_intent: true,
        source_ref: review.review_id.clone(),
        support_export_ref: format!(
            "support.desktop_continuity.target_recovery.{}",
            review.source_surface_token
        ),
    }
}

fn target_recovery_from_native_file_review(
    review: &NativeFileHandoffReviewRecord,
) -> DesktopTargetRecoveryRow {
    DesktopTargetRecoveryRow {
        row_id: format!("desktop-continuity.target-recovery.{}", review.case_id),
        target_kind_token: review.target_kind_token.clone(),
        literal_target_label: review.literal_target_label.clone(),
        object_identity_ref: review.object_identity_ref.clone(),
        availability_class_token: review.availability_class_token.clone(),
        freshness_class_token: "stale".to_owned(),
        recovery_surface_token: review.review_surface_token.clone(),
        recovery_action_tokens: review
            .recovery_actions
            .iter()
            .map(|action| action.action_token.clone())
            .collect(),
        preserves_user_intent: true,
        source_ref: review.review_id.clone(),
        support_export_ref: format!(
            "support.desktop_continuity.target_recovery.{}",
            review.case_id
        ),
    }
}

fn topology_rows() -> Vec<DesktopTopologyContinuityRow> {
    vec![
        DesktopTopologyContinuityRow {
            row_id: "desktop-continuity.topology.display-detach-safe-bounds".to_owned(),
            source_fixture_ref:
                "fixtures/platform/window_display_cases/display_detach_dock_safe_bounds.json"
                    .to_owned(),
            event_class_token: "display_topology_change".to_owned(),
            display_context_token: "monitor_topology_changed".to_owned(),
            topology_change_tokens: vec![
                "display_removed".to_owned(),
                "display_added".to_owned(),
                "safe_bounds_changed".to_owned(),
            ],
            adjustment_tokens: vec![
                "moved_to_primary_display".to_owned(),
                "snapped_to_safe_bounds".to_owned(),
                "dialog_recentered_to_owner".to_owned(),
                "stacking_repaired".to_owned(),
            ],
            continuity_state_tokens: vec![
                "layout_adjusted".to_owned(),
                "safe_bounds_restored".to_owned(),
            ],
            resulting_fidelity_token: "compatible_restore".to_owned(),
            topology_adjustment_downgraded_fidelity: true,
            visible_bounds_preserved: true,
            focus_intent_preserved: true,
            recovery_action_tokens: vec![
                "recenter_window".to_owned(),
                "review_layout".to_owned(),
                "restore_detached_window".to_owned(),
            ],
            support_export_ref: "support.desktop_continuity.topology.display_detach".to_owned(),
        },
        DesktopTopologyContinuityRow {
            row_id: "desktop-continuity.topology.mixed-dpi-cross-monitor".to_owned(),
            source_fixture_ref:
                "fixtures/platform/window_display_cases/mixed_dpi_cross_monitor_reflow.json"
                    .to_owned(),
            event_class_token: "mixed_dpi_cross_monitor_reflow".to_owned(),
            display_context_token: "mixed_dpi_multi_monitor".to_owned(),
            topology_change_tokens: vec![
                "display_moved".to_owned(),
                "scale_changed".to_owned(),
                "safe_bounds_changed".to_owned(),
            ],
            adjustment_tokens: vec![
                "scale_normalized".to_owned(),
                "snapped_to_safe_bounds".to_owned(),
                "native_chrome_reprojected".to_owned(),
            ],
            continuity_state_tokens: vec!["layout_adjusted".to_owned()],
            resulting_fidelity_token: "compatible_restore".to_owned(),
            topology_adjustment_downgraded_fidelity: true,
            visible_bounds_preserved: true,
            focus_intent_preserved: true,
            recovery_action_tokens: vec![
                "recenter_window".to_owned(),
                "review_layout".to_owned(),
                "keep_current_pane_visible".to_owned(),
            ],
            support_export_ref: "support.desktop_continuity.topology.mixed_dpi".to_owned(),
        },
        DesktopTopologyContinuityRow {
            row_id: "desktop-continuity.topology.fullscreen-snapped-restore".to_owned(),
            source_fixture_ref:
                "fixtures/platform/window_display_cases/fullscreen_snapped_restore_intent.json"
                    .to_owned(),
            event_class_token: "fullscreen_or_snapped_restore".to_owned(),
            display_context_token: "fullscreen_or_snap_state_narrowed".to_owned(),
            topology_change_tokens: vec![
                "app_reopen".to_owned(),
                "fullscreen_state_rewritten".to_owned(),
                "snap_or_tile_state_rewritten".to_owned(),
                "virtual_desktop_changed".to_owned(),
            ],
            adjustment_tokens: vec![
                "fullscreen_cleared".to_owned(),
                "snapped_layout_cleared".to_owned(),
                "virtual_desktop_fallback".to_owned(),
                "snapped_to_safe_bounds".to_owned(),
            ],
            continuity_state_tokens: vec!["layout_adjusted".to_owned()],
            resulting_fidelity_token: "compatible_restore".to_owned(),
            topology_adjustment_downgraded_fidelity: true,
            visible_bounds_preserved: true,
            focus_intent_preserved: true,
            recovery_action_tokens: vec![
                "exit_fullscreen_or_snapped_mode".to_owned(),
                "move_to_primary_display".to_owned(),
                "review_layout".to_owned(),
            ],
            support_export_ref: "support.desktop_continuity.topology.fullscreen_snapped".to_owned(),
        },
    ]
}

fn lifecycle_rows() -> Vec<DesktopLifecycleInterruptionRow> {
    vec![
        DesktopLifecycleInterruptionRow {
            row_id: "desktop-continuity.lifecycle.wake-from-sleep".to_owned(),
            source_fixture_ref:
                "fixtures/platform/native_lifecycle_cases/wake_from_sleep_local_context_preserved.yaml"
                    .to_owned(),
            lifecycle_event_token: "wake_from_sleep".to_owned(),
            object_identity_ref: "obj:managed-workspace:payments:dev:01".to_owned(),
            trust_profile_boundary_token: "restricted:scope:managed-workspace:payments:dev"
                .to_owned(),
            continuity_state_tokens: vec![
                "reconnecting".to_owned(),
                "resume_review_needed".to_owned(),
            ],
            resulting_fidelity_token: "layout_only".to_owned(),
            local_work_continues: true,
            privileged_or_mutating_work_paused: true,
            no_silent_rerun_or_authority_reuse: true,
            recovery_action_tokens: vec![
                "continue_local".to_owned(),
                "reconnect".to_owned(),
                "reauthenticate".to_owned(),
                "export_restore_details".to_owned(),
            ],
            support_export_ref: "support.desktop_continuity.lifecycle.wake_from_sleep".to_owned(),
        },
        DesktopLifecycleInterruptionRow {
            row_id: "desktop-continuity.lifecycle.network-transition".to_owned(),
            source_fixture_ref:
                "fixtures/platform/native_lifecycle_cases/network_transition_remote_unreachable.yaml"
                    .to_owned(),
            lifecycle_event_token: "network_transition".to_owned(),
            object_identity_ref: "obj:managed-workspace:payments:dev:01".to_owned(),
            trust_profile_boundary_token: "restricted:scope:managed-workspace:payments:dev"
                .to_owned(),
            continuity_state_tokens: vec![
                "reconnecting".to_owned(),
                "local_fallback".to_owned(),
                "stale".to_owned(),
            ],
            resulting_fidelity_token: "layout_only".to_owned(),
            local_work_continues: true,
            privileged_or_mutating_work_paused: true,
            no_silent_rerun_or_authority_reuse: true,
            recovery_action_tokens: vec![
                "continue_local".to_owned(),
                "reconnect".to_owned(),
                "reauthenticate".to_owned(),
                "export_restore_details".to_owned(),
            ],
            support_export_ref: "support.desktop_continuity.lifecycle.network_transition"
                .to_owned(),
        },
        DesktopLifecycleInterruptionRow {
            row_id: "desktop-continuity.lifecycle.removable-root-loss".to_owned(),
            source_fixture_ref:
                "fixtures/platform/native_lifecycle_cases/removable_volume_loss_root_unavailable.yaml"
                    .to_owned(),
            lifecycle_event_token: "removable_volume_loss".to_owned(),
            object_identity_ref: "obj:workspace-root:client-drive:payments:01".to_owned(),
            trust_profile_boundary_token: "restricted:scope:workspace:payments".to_owned(),
            continuity_state_tokens: vec![
                "root_unavailable".to_owned(),
                "stale".to_owned(),
                "cached_context_available".to_owned(),
            ],
            resulting_fidelity_token: "evidence_only".to_owned(),
            local_work_continues: true,
            privileged_or_mutating_work_paused: true,
            no_silent_rerun_or_authority_reuse: true,
            recovery_action_tokens: vec![
                "locate_target".to_owned(),
                "reconnect_volume".to_owned(),
                "open_cached_context".to_owned(),
                "export_context".to_owned(),
                "remove_root".to_owned(),
            ],
            support_export_ref: "support.desktop_continuity.lifecycle.removable_root_loss"
                .to_owned(),
        },
    ]
}

fn credential_store_interruption_packet() -> SecretBrokerAlphaPacket {
    let packet: SecretBrokerAlphaPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/auth/secret_broker_alpha/failure_locked_unavailable_trust_changed.json"
    )))
    .expect("credential-store interruption fixture must parse");
    packet
        .validate()
        .expect("credential-store interruption fixture must validate");
    packet
}

fn credential_store_rows(
    packet: &SecretBrokerAlphaPacket,
    support_export: &aureline_auth::SecretBrokerSupportExport,
) -> Vec<CredentialStoreInterruptionRow> {
    packet
        .rows
        .iter()
        .map(|row| {
            let support_row = support_export
                .rows
                .iter()
                .find(|support_row| support_row.secret_broker_row_ref == row.secret_broker_row_id)
                .expect("secret broker support row exists");
            CredentialStoreInterruptionRow {
                row_id: format!(
                    "desktop-continuity.credential-store.{}",
                    row.capability_class.as_str()
                ),
                secret_broker_row_ref: row.secret_broker_row_id.clone(),
                affected_capability_tokens: row
                    .continuity
                    .affected_capabilities
                    .iter()
                    .map(|capability| capability.as_str().to_owned())
                    .collect(),
                trust_store_class_token: row.storage.trust_store_class.as_str().to_owned(),
                unlock_state_token: row.storage.unlock_state.as_str().to_owned(),
                continuity_state_token: row.continuity.continuity_state.as_str().to_owned(),
                denial_reason_token: row
                    .continuity
                    .denial_reason
                    .map(|reason| reason.as_str().to_owned()),
                local_continuation_token: row.continuity.local_continuation.as_str().to_owned(),
                local_work_continues: row.continuity.local_work_continues,
                credentialed_actions_paused: row.continuity.credentialed_actions_paused,
                plaintext_downgrade_forbidden: !row.storage.plaintext_persistence_allowed
                    && !row.storage.silent_in_memory_promotion_allowed,
                stale_authority_reuse_forbidden: !row.storage.stale_ticket_reuse_allowed,
                recovery_action_tokens: row
                    .continuity
                    .recovery_actions
                    .iter()
                    .map(|action| action.as_str().to_owned())
                    .collect(),
                source_support_export_ref: support_row.export_row_id.clone(),
                support_export_ref: format!(
                    "support.desktop_continuity.credential_store.{}",
                    row.capability_class.as_str()
                ),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates_required_families() {
        let packet = seeded_desktop_continuity_alpha_packet("build:test");
        packet.validate().expect("packet validates");
        assert_eq!(packet.entry_rows.len(), 5);
        assert_eq!(packet.credential_store_rows.len(), 3);
    }

    #[test]
    fn support_export_reconstructs_without_ui_scrape() {
        let packet = seeded_desktop_continuity_alpha_packet("build:test");
        let export = packet.support_export(
            "support.desktop_continuity.alpha.unit",
            "2026-05-14T00:20:00Z",
        );

        assert!(export.redaction_safe());
        assert!(export.reconstructs_interruption_cause);
        assert!(export.reconstructs_continuity_state);
        assert!(export.reconstructs_recovery_choice);
        assert!(export.reconstructs_resulting_fidelity);
        assert!(export.rows.iter().all(|row| {
            !row.interruption_cause_token.is_empty()
                && !row.continuity_state_tokens.is_empty()
                && !row.support_field_refs.is_empty()
        }));
    }
}
