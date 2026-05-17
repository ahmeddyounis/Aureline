//! Beta remote-helper capability negotiation, attach/reconnect lifecycle, and
//! supported skew-window enforcement.
//!
//! The alpha [`crate::capability_negotiation`] module owns the metadata-only
//! intersection of a client and helper capability manifest under a supplied
//! compatibility window. This beta layer promotes that primitive into the
//! attach- and reconnect-bound record that downstream remote, support, and
//! compatibility-report surfaces consume.
//!
//! Every beta record carries:
//!
//! - a closed [`RemoteHelperLifecyclePhaseClass`] (attach or reconnect);
//! - a typed [`RemoteHelperSkewVisibilityClass`] that captures the supported
//!   skew posture in user-visible terms;
//! - a typed [`RemoteHelperRepairPathClass`] that names the truthful recovery
//!   path for downgraded or refused negotiations;
//! - the same `(envelope_id, row_id)` rows compatibility reports and support
//!   exports reference, so users, reviewers, and support read one truth.
//!
//! The machine-readable boundary lives at
//! [`/schemas/providers/remote_capabilities.schema.json`](../../../../schemas/providers/remote_capabilities.schema.json).
//! The reviewer-facing landing page is
//! [`/docs/runtime/m3/remote_helper_skew_beta.md`](../../../../docs/runtime/m3/remote_helper_skew_beta.md).

use serde::{Deserialize, Serialize};

use crate::capability_negotiation::{
    CompatibilityWindow, CompatibilityWindowStatus, DroppedHelperCapability,
    EffectiveCapabilityPosture, HelperCapabilityResponse, NegotiationOutcome,
};

/// Schema version for the beta remote-helper capability records.
pub const REMOTE_HELPER_SKEW_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one beta remote-helper row.
pub const REMOTE_HELPER_SKEW_BETA_RECORD_KIND: &str = "remote_helper_skew_beta_record";

/// Stable record-kind tag for the beta support-export bundle.
pub const REMOTE_HELPER_SKEW_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_helper_skew_beta_support_export_record";

/// Stable record-kind tag for the compatibility-report row projected from the
/// same negotiated rows that support exports embed.
pub const REMOTE_HELPER_SKEW_BETA_COMPATIBILITY_ROW_RECORD_KIND: &str =
    "remote_helper_skew_beta_compatibility_row_record";

/// Closed vocabulary describing which remote-helper flow minted the record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteHelperLifecyclePhaseClass {
    /// Initial attach exchange; the helper or remote agent was newly started.
    Attach,
    /// Reconnect or rebind exchange after a session loss or version change.
    Reconnect,
}

impl RemoteHelperLifecyclePhaseClass {
    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attach => "attach",
            Self::Reconnect => "reconnect",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Attach => "Initial attach",
            Self::Reconnect => "Reconnect",
        }
    }
}

/// Closed user-visible skew posture for a negotiated remote-helper record.
///
/// This vocabulary is what status surfaces, support exports, and compatibility
/// reports show users. It is derived from the alpha
/// [`CompatibilityWindowStatus`] and the negotiated outcome so a single review
/// summary can communicate "what does this client/helper pair actually
/// support right now?" without leaking raw versions or endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteHelperSkewVisibilityClass {
    /// Client and helper are inside the declared adjacent skew window; full
    /// remote capability is admitted.
    AdjacentSupported,
    /// Helper is inside the supported window but the negotiated capability set
    /// is narrowed; mutation is paused until full support returns.
    NarrowedSupportedWindow,
    /// Helper pairing is untested or marked best-effort; a probe or reattach
    /// must complete before support is claimed.
    ProbeRequiredUntested,
    /// Helper pairing is outside the supported skew window; remote helper
    /// authority is refused until an upgrade or repin.
    OutsideSupportedWindow,
}

impl RemoteHelperSkewVisibilityClass {
    /// All beta visibility classes.
    pub const ALL: [Self; 4] = [
        Self::AdjacentSupported,
        Self::NarrowedSupportedWindow,
        Self::ProbeRequiredUntested,
        Self::OutsideSupportedWindow,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdjacentSupported => "adjacent_supported",
            Self::NarrowedSupportedWindow => "narrowed_supported_window",
            Self::ProbeRequiredUntested => "probe_required_untested",
            Self::OutsideSupportedWindow => "outside_supported_window",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AdjacentSupported => "Adjacent supported",
            Self::NarrowedSupportedWindow => "Narrowed inside supported window",
            Self::ProbeRequiredUntested => "Probe required (untested)",
            Self::OutsideSupportedWindow => "Outside supported window",
        }
    }

    /// True when this visibility class fails closed for mutating remote work.
    pub const fn fails_closed(self) -> bool {
        matches!(
            self,
            Self::ProbeRequiredUntested | Self::OutsideSupportedWindow
        )
    }

    /// Derives the visibility class from the alpha compatibility-window status
    /// and the negotiated outcome.
    pub fn derive(status: CompatibilityWindowStatus, outcome: NegotiationOutcome) -> Self {
        match (status, outcome) {
            (CompatibilityWindowStatus::Unsupported, _) => Self::OutsideSupportedWindow,
            (CompatibilityWindowStatus::Untested, _) => Self::ProbeRequiredUntested,
            (CompatibilityWindowStatus::BestEffort, _) => Self::NarrowedSupportedWindow,
            (CompatibilityWindowStatus::Supported, NegotiationOutcome::Match) => {
                Self::AdjacentSupported
            }
            (CompatibilityWindowStatus::Supported, NegotiationOutcome::Downgrade) => {
                Self::NarrowedSupportedWindow
            }
            (CompatibilityWindowStatus::Supported, NegotiationOutcome::Refuse) => {
                Self::OutsideSupportedWindow
            }
        }
    }
}

/// Closed vocabulary for the truthful recovery path offered to the user when
/// the negotiated record is not full remote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteHelperRepairPathClass {
    /// No repair required; the record is fully supported.
    NoRepairRequired,
    /// Continue in the narrowed posture; surfaces honestly disclose what is
    /// disabled until the full capability set returns.
    ContinueNarrowedPosture,
    /// Run a drift probe or reattach to resolve an untested pairing.
    RunDriftProbeOrReattach,
    /// Upgrade or repin the client or helper to restore supported skew.
    UpgradeOrRepin,
    /// Continue locally only; the remote helper is refused but local work
    /// remains available.
    ContinueLocalOnly,
    /// Escalate to administrator or support; the lane cannot self-repair.
    ContactAdminOrSupport,
}

impl RemoteHelperRepairPathClass {
    /// All beta repair-path classes.
    pub const ALL: [Self; 6] = [
        Self::NoRepairRequired,
        Self::ContinueNarrowedPosture,
        Self::RunDriftProbeOrReattach,
        Self::UpgradeOrRepin,
        Self::ContinueLocalOnly,
        Self::ContactAdminOrSupport,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRepairRequired => "no_repair_required",
            Self::ContinueNarrowedPosture => "continue_narrowed_posture",
            Self::RunDriftProbeOrReattach => "run_drift_probe_or_reattach",
            Self::UpgradeOrRepin => "upgrade_or_repin",
            Self::ContinueLocalOnly => "continue_local_only",
            Self::ContactAdminOrSupport => "contact_admin_or_support",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoRepairRequired => "No repair required",
            Self::ContinueNarrowedPosture => "Continue in narrowed posture",
            Self::RunDriftProbeOrReattach => "Run drift probe or reattach",
            Self::UpgradeOrRepin => "Upgrade or repin",
            Self::ContinueLocalOnly => "Continue local only",
            Self::ContactAdminOrSupport => "Contact admin or support",
        }
    }

    /// Derives the repair-path class from a visibility class plus the
    /// effective posture and lifecycle phase. The derivation is intentionally
    /// total so adding a visibility class is a build-time signal.
    pub fn derive(
        visibility: RemoteHelperSkewVisibilityClass,
        posture: EffectiveCapabilityPosture,
        phase: RemoteHelperLifecyclePhaseClass,
    ) -> Self {
        match visibility {
            RemoteHelperSkewVisibilityClass::AdjacentSupported => Self::NoRepairRequired,
            RemoteHelperSkewVisibilityClass::NarrowedSupportedWindow => {
                if matches!(posture, EffectiveCapabilityPosture::LocalOnly) {
                    Self::ContinueLocalOnly
                } else {
                    Self::ContinueNarrowedPosture
                }
            }
            RemoteHelperSkewVisibilityClass::ProbeRequiredUntested => match phase {
                RemoteHelperLifecyclePhaseClass::Attach => Self::RunDriftProbeOrReattach,
                RemoteHelperLifecyclePhaseClass::Reconnect => Self::RunDriftProbeOrReattach,
            },
            RemoteHelperSkewVisibilityClass::OutsideSupportedWindow => match posture {
                EffectiveCapabilityPosture::Blocked => Self::ContactAdminOrSupport,
                EffectiveCapabilityPosture::LocalOnly => Self::ContinueLocalOnly,
                _ => Self::UpgradeOrRepin,
            },
        }
    }
}

/// Visible version disclosure shared across status, support, and compatibility
/// surfaces. The disclosure intentionally uses opaque tokens (component
/// version, protocol version, schema epoch) and never raw hostnames, endpoints,
/// or credentials.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteHelperVisibleVersionState {
    /// Opaque client version token.
    pub client_version: String,
    /// Opaque helper or remote-agent version token.
    pub helper_version: String,
    /// Negotiated protocol version token from the alpha negotiation result.
    pub selected_protocol_version: String,
    /// Skew-case ref from the version-skew register.
    pub skew_case_ref: String,
    /// Skew-window declaration ref from the skew-windows manifest.
    pub skew_window_declaration_ref: String,
    /// Compatibility-row ref from the qualification matrix.
    pub compatibility_row_ref: String,
}

impl RemoteHelperVisibleVersionState {
    /// Builds the disclosure from a [`HelperCapabilityResponse`] plus explicit
    /// client/helper version tokens.
    pub fn from_response(
        response: &HelperCapabilityResponse,
        client_version: impl Into<String>,
        helper_version: impl Into<String>,
    ) -> Self {
        let CompatibilityWindow {
            compatibility_row_ref,
            skew_case_ref,
            skew_window_declaration_ref,
            ..
        } = response.compatibility_window.clone();
        Self {
            client_version: client_version.into(),
            helper_version: helper_version.into(),
            selected_protocol_version: response.selected_protocol_ref.clone(),
            skew_case_ref,
            skew_window_declaration_ref,
            compatibility_row_ref,
        }
    }
}

/// One beta record for a remote-helper attach or reconnect exchange.
///
/// The record bundles the alpha negotiation result with the lifecycle phase,
/// derived visibility class, derived repair-path class, version disclosure, and
/// attach-session ref. The same `(envelope_id, row_id)` pair is referenced by
/// both compatibility reports and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteHelperBetaRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the beta record family.
    pub schema_version: u32,
    /// Stable row id shared with compatibility reports and support exports.
    pub row_id: String,
    /// Envelope id from the alpha negotiation.
    pub envelope_id: String,
    /// Lifecycle phase that minted this exchange.
    pub lifecycle_phase: RemoteHelperLifecyclePhaseClass,
    /// Stable lifecycle-phase token.
    pub lifecycle_phase_token: String,
    /// Reviewer-facing lifecycle-phase label.
    pub lifecycle_phase_label: String,
    /// Reconnect attempt counter; zero for initial attach.
    pub reconnect_attempt: u32,
    /// Opaque attach-session ref from `remote_attach_session_record`.
    pub attach_session_ref: String,
    /// Negotiation outcome from the alpha module.
    pub negotiation_outcome: NegotiationOutcome,
    /// Stable negotiation-outcome token.
    pub negotiation_outcome_token: String,
    /// Effective posture after negotiation.
    pub effective_posture: EffectiveCapabilityPosture,
    /// Stable effective-posture token.
    pub effective_posture_token: String,
    /// True when remote mutation authority is admitted.
    pub mutation_allowed: bool,
    /// Derived visible skew posture.
    pub skew_visibility: RemoteHelperSkewVisibilityClass,
    /// Stable visibility token.
    pub skew_visibility_token: String,
    /// Reviewer-facing visibility label.
    pub skew_visibility_label: String,
    /// Derived repair path.
    pub repair_path: RemoteHelperRepairPathClass,
    /// Stable repair-path token.
    pub repair_path_token: String,
    /// Reviewer-facing repair-path label.
    pub repair_path_label: String,
    /// Visible version disclosure.
    pub visible_version_state: RemoteHelperVisibleVersionState,
    /// Capability intersection admitted for this record.
    pub negotiated_capabilities: Vec<String>,
    /// Requested capabilities that were not admitted.
    pub dropped_capabilities: Vec<DroppedHelperCapability>,
    /// Redaction-safe visible summary lifted from the alpha negotiation.
    pub visible_summary: String,
    /// Redaction-safe safe-continuation summary lifted from the alpha
    /// negotiation.
    pub safe_continuation: String,
    /// Compatibility-row, schema, fixture, doc, and support refs.
    pub source_refs: Vec<String>,
    /// Redaction-safe support packet refs the record can be embedded under.
    pub support_packet_refs: Vec<String>,
    /// Compatibility-report row refs the record contributes to.
    pub compatibility_report_row_refs: Vec<String>,
    /// True because raw tokens, endpoints, paths, and secrets are excluded.
    pub redaction_safe: bool,
}

impl RemoteHelperBetaRecord {
    /// Builds a beta record from an alpha negotiation response.
    pub fn from_response(
        response: &HelperCapabilityResponse,
        phase: RemoteHelperLifecyclePhaseClass,
        reconnect_attempt: u32,
        attach_session_ref: impl Into<String>,
        client_version: impl Into<String>,
        helper_version: impl Into<String>,
        compatibility_report_row_refs: Vec<String>,
    ) -> Self {
        let visibility = RemoteHelperSkewVisibilityClass::derive(
            response.compatibility_window.status,
            response.outcome,
        );
        let repair =
            RemoteHelperRepairPathClass::derive(visibility, response.effective_posture, phase);
        let attach_session_ref = attach_session_ref.into();
        let visible_version_state = RemoteHelperVisibleVersionState::from_response(
            response,
            client_version,
            helper_version,
        );
        let row_id = format!("remote-helper-beta-row:{}", response.row_id);
        let mut source_refs = response.compatibility_window.source_refs.clone();
        source_refs.extend(response.source_refs.iter().cloned());
        Self {
            record_kind: REMOTE_HELPER_SKEW_BETA_RECORD_KIND.to_owned(),
            schema_version: REMOTE_HELPER_SKEW_BETA_SCHEMA_VERSION,
            row_id,
            envelope_id: response.request_id.clone(),
            lifecycle_phase: phase,
            lifecycle_phase_token: phase.as_str().to_owned(),
            lifecycle_phase_label: phase.label().to_owned(),
            reconnect_attempt,
            attach_session_ref,
            negotiation_outcome: response.outcome,
            negotiation_outcome_token: response.outcome.as_str().to_owned(),
            effective_posture: response.effective_posture,
            effective_posture_token: response.effective_posture.as_str().to_owned(),
            mutation_allowed: response.mutation_allowed,
            skew_visibility: visibility,
            skew_visibility_token: visibility.as_str().to_owned(),
            skew_visibility_label: visibility.label().to_owned(),
            repair_path: repair,
            repair_path_token: repair.as_str().to_owned(),
            repair_path_label: repair.label().to_owned(),
            visible_version_state,
            negotiated_capabilities: response.negotiated_capabilities.clone(),
            dropped_capabilities: response.dropped_capabilities.clone(),
            visible_summary: response.visible_summary.clone(),
            safe_continuation: response.safe_continuation.clone(),
            source_refs,
            support_packet_refs: response.support_packet_refs.clone(),
            compatibility_report_row_refs,
            redaction_safe: true,
        }
    }

    /// True when this record fails closed for mutating remote work.
    pub fn fails_closed_for_mutation(&self) -> bool {
        !self.mutation_allowed || self.skew_visibility.fails_closed()
    }

    /// Returns one deterministic plaintext summary line for status surfaces.
    pub fn summary_line(&self) -> String {
        format!(
            "row={}; phase={}; visibility={}; posture={}; repair={}; mutation_allowed={}",
            self.row_id,
            self.lifecycle_phase_token,
            self.skew_visibility_token,
            self.effective_posture_token,
            self.repair_path_token,
            self.mutation_allowed,
        )
    }
}

/// Compatibility-report row projected from a beta record. The row carries the
/// shared `(envelope_id, row_id)` pair plus the minimum truth a compatibility
/// report needs so the report and the support export reference one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteHelperBetaCompatibilityRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the beta record family.
    pub schema_version: u32,
    /// Stable row id shared with the beta record and support export.
    pub row_id: String,
    /// Envelope id shared with the beta record.
    pub envelope_id: String,
    /// Compatibility-row ref the row attaches to.
    pub compatibility_row_ref: String,
    /// Skew-case ref the row attaches to.
    pub skew_case_ref: String,
    /// Visible version disclosure (client/helper/protocol tokens).
    pub visible_version_state: RemoteHelperVisibleVersionState,
    /// Visibility class.
    pub skew_visibility_token: String,
    /// Repair-path class.
    pub repair_path_token: String,
    /// Effective posture token.
    pub effective_posture_token: String,
    /// Mutation flag from the negotiation.
    pub mutation_allowed: bool,
    /// True because raw tokens and endpoints are excluded.
    pub redaction_safe: bool,
}

impl RemoteHelperBetaCompatibilityRow {
    /// Projects a compatibility row from a beta record.
    pub fn from_record(record: &RemoteHelperBetaRecord) -> Self {
        Self {
            record_kind: REMOTE_HELPER_SKEW_BETA_COMPATIBILITY_ROW_RECORD_KIND.to_owned(),
            schema_version: REMOTE_HELPER_SKEW_BETA_SCHEMA_VERSION,
            row_id: record.row_id.clone(),
            envelope_id: record.envelope_id.clone(),
            compatibility_row_ref: record.visible_version_state.compatibility_row_ref.clone(),
            skew_case_ref: record.visible_version_state.skew_case_ref.clone(),
            visible_version_state: record.visible_version_state.clone(),
            skew_visibility_token: record.skew_visibility_token.clone(),
            repair_path_token: record.repair_path_token.clone(),
            effective_posture_token: record.effective_posture_token.clone(),
            mutation_allowed: record.mutation_allowed,
            redaction_safe: true,
        }
    }
}

/// Support-export bundle for the beta remote-helper records. The bundle holds
/// the beta records and the projected compatibility rows together, so support
/// reviewers see the same `(envelope_id, row_id)` truth as the compatibility
/// report consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteHelperBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Beta records included in this bundle.
    pub records: Vec<RemoteHelperBetaRecord>,
    /// Compatibility-report rows projected from the same records.
    pub compatibility_rows: Vec<RemoteHelperBetaCompatibilityRow>,
    /// True when at least one record fails closed for mutating work.
    pub any_record_fails_closed_for_mutation: bool,
    /// True because raw payloads, endpoints, paths, and secrets are excluded.
    pub redaction_safe: bool,
}

impl RemoteHelperBetaSupportExport {
    /// Builds the support-export bundle from a sequence of beta records.
    pub fn from_records<'a>(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: impl IntoIterator<Item = &'a RemoteHelperBetaRecord>,
    ) -> Self {
        let records: Vec<RemoteHelperBetaRecord> = records.into_iter().cloned().collect();
        let compatibility_rows: Vec<RemoteHelperBetaCompatibilityRow> = records
            .iter()
            .map(RemoteHelperBetaCompatibilityRow::from_record)
            .collect();
        let any_record_fails_closed_for_mutation = records
            .iter()
            .any(RemoteHelperBetaRecord::fails_closed_for_mutation);
        Self {
            record_kind: REMOTE_HELPER_SKEW_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REMOTE_HELPER_SKEW_BETA_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            records,
            compatibility_rows,
            any_record_fails_closed_for_mutation,
            redaction_safe: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability_negotiation::{
        CompatibilityWindow, CompatibilityWindowStatus, EffectiveCapabilityPosture,
        HelperCapabilityResponse, NegotiationOutcome, HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
    };

    fn response(
        status: CompatibilityWindowStatus,
        outcome: NegotiationOutcome,
        posture: EffectiveCapabilityPosture,
        mutation_allowed: bool,
    ) -> HelperCapabilityResponse {
        HelperCapabilityResponse {
            schema_version: HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
            request_id: "helper_capability_envelope:test.case".to_owned(),
            row_id: "drift_truth.helper_agent.test.case".to_owned(),
            surface_ref: "remote_agent_attach".to_owned(),
            title: "test".to_owned(),
            outcome,
            selected_protocol_ref: "protocol@2026.05".to_owned(),
            negotiated_capabilities: vec!["remote.fs.read".to_owned()],
            dropped_capabilities: Vec::new(),
            mutation_allowed,
            effective_posture: posture,
            visible_summary: "summary".to_owned(),
            safe_continuation: "continue".to_owned(),
            primary_recovery_ref: None,
            recovery_refs: Vec::new(),
            blocked_action_refs: Vec::new(),
            preserved_read_only_refs: Vec::new(),
            retry_ref: None,
            support_packet_refs: vec!["support_packet:test".to_owned()],
            review_packet_refs: Vec::new(),
            source_refs: Vec::new(),
            client_manifest_digest: "digest:client".to_owned(),
            helper_manifest_digest: "digest:helper".to_owned(),
            compatibility_window: CompatibilityWindow {
                boundary_family: "desktop_cli_and_remote_agent".to_owned(),
                compatibility_row_ref: "compat_row:remote.attach_envelope_and_drift".to_owned(),
                version_skew_register_ref: "version_skew_register:remote.attach".to_owned(),
                skew_case_ref: "skew_case:remote.attach_adjacent_window".to_owned(),
                skew_window_declaration_ref:
                    "skew_window:desktop_cli_and_remote_agent.declared_adjacent_window".to_owned(),
                status,
                selected_protocol_ref: "protocol@2026.05".to_owned(),
                source_refs: vec!["artifacts/compat/skew_windows.yaml".to_owned()],
            },
        }
    }

    #[test]
    fn adjacent_supported_attach_admits_full_remote() {
        let response = response(
            CompatibilityWindowStatus::Supported,
            NegotiationOutcome::Match,
            EffectiveCapabilityPosture::FullRemote,
            true,
        );
        let record = RemoteHelperBetaRecord::from_response(
            &response,
            RemoteHelperLifecyclePhaseClass::Attach,
            0,
            "attach_session:remote.alpha".to_owned(),
            "client@2026.05",
            "agent@2026.05",
            vec!["compat_report_row:remote.attach".to_owned()],
        );
        assert_eq!(
            record.skew_visibility,
            RemoteHelperSkewVisibilityClass::AdjacentSupported
        );
        assert_eq!(
            record.repair_path,
            RemoteHelperRepairPathClass::NoRepairRequired
        );
        assert!(record.mutation_allowed);
        assert!(!record.fails_closed_for_mutation());
    }

    #[test]
    fn unsupported_skew_attach_fails_closed_with_upgrade_repair() {
        let response = response(
            CompatibilityWindowStatus::Unsupported,
            NegotiationOutcome::Refuse,
            EffectiveCapabilityPosture::ReviewOnly,
            false,
        );
        let record = RemoteHelperBetaRecord::from_response(
            &response,
            RemoteHelperLifecyclePhaseClass::Attach,
            0,
            "attach_session:remote.unsupported".to_owned(),
            "client@2026.05",
            "agent@2025.09",
            Vec::new(),
        );
        assert_eq!(
            record.skew_visibility,
            RemoteHelperSkewVisibilityClass::OutsideSupportedWindow
        );
        assert_eq!(
            record.repair_path,
            RemoteHelperRepairPathClass::UpgradeOrRepin
        );
        assert!(!record.mutation_allowed);
        assert!(record.fails_closed_for_mutation());
    }

    #[test]
    fn untested_reconnect_routes_to_probe_repair() {
        let response = response(
            CompatibilityWindowStatus::Untested,
            NegotiationOutcome::Refuse,
            EffectiveCapabilityPosture::InspectOnly,
            false,
        );
        let record = RemoteHelperBetaRecord::from_response(
            &response,
            RemoteHelperLifecyclePhaseClass::Reconnect,
            2,
            "attach_session:remote.reconnect".to_owned(),
            "client@2026.05",
            "agent@2026.05",
            Vec::new(),
        );
        assert_eq!(
            record.skew_visibility,
            RemoteHelperSkewVisibilityClass::ProbeRequiredUntested
        );
        assert_eq!(
            record.repair_path,
            RemoteHelperRepairPathClass::RunDriftProbeOrReattach
        );
        assert_eq!(record.reconnect_attempt, 2);
    }

    #[test]
    fn compatibility_row_and_support_export_share_row_id() {
        let response = response(
            CompatibilityWindowStatus::BestEffort,
            NegotiationOutcome::Downgrade,
            EffectiveCapabilityPosture::ReviewOnly,
            false,
        );
        let record = RemoteHelperBetaRecord::from_response(
            &response,
            RemoteHelperLifecyclePhaseClass::Attach,
            0,
            "attach_session:remote.best_effort".to_owned(),
            "client@2026.05",
            "agent@2026.04",
            vec!["compat_report_row:remote.best_effort".to_owned()],
        );
        let compat = RemoteHelperBetaCompatibilityRow::from_record(&record);
        let export = RemoteHelperBetaSupportExport::from_records(
            "support_export:remote_helper_skew_beta.example",
            "2026-05-16T00:00:00Z",
            std::iter::once(&record),
        );
        assert_eq!(compat.row_id, record.row_id);
        assert_eq!(export.records.len(), 1);
        assert_eq!(export.compatibility_rows.len(), 1);
        assert_eq!(export.compatibility_rows[0].row_id, record.row_id);
        assert!(export.any_record_fails_closed_for_mutation);
    }
}
