//! Beta drift-repair guidance, upgrade/downgrade prompts, and exportable
//! diagnostics for remote-helper attach and reconnect flows.
//!
//! The beta [`crate::remote_helper_skew_beta`] module owns the negotiated
//! attach/reconnect record (lifecycle phase, visible skew posture, typed repair
//! path). This module promotes that record into a guidance lane that:
//!
//! - explains drift as a *typed* combination of version, capability, auth,
//!   route, and target mismatches drawn from a closed vocabulary;
//! - names the bounded recovery actions (upgrade, downgrade, reconnect,
//!   continue-local, run-probe, contact-support) and the authority impact each
//!   one carries, so authority never widens silently;
//! - exports the same guidance packet to the support / diagnostics surface,
//!   so what reviewers and support read matches what users see in-product.
//!
//! The machine-readable boundary lives at
//! [`/schemas/workspace/remote_drift_repair.schema.json`](../../../../schemas/workspace/remote_drift_repair.schema.json).
//! The reviewer-facing landing page is
//! [`/docs/runtime/m3/remote_drift_repair_beta.md`](../../../../docs/runtime/m3/remote_drift_repair_beta.md).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::capability_negotiation::{
    EffectiveCapabilityPosture, MissingCapabilityReasonClass, NegotiationOutcome,
};
use crate::remote_helper_skew_beta::{
    RemoteHelperBetaRecord, RemoteHelperLifecyclePhaseClass, RemoteHelperRepairPathClass,
    RemoteHelperSkewVisibilityClass, RemoteHelperVisibleVersionState,
};

/// Schema version for the drift-repair guidance records.
pub const REMOTE_DRIFT_REPAIR_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one drift-repair guidance record.
pub const REMOTE_DRIFT_REPAIR_BETA_GUIDANCE_RECORD_KIND: &str =
    "remote_drift_repair_beta_guidance_record";

/// Stable record-kind tag for the exportable diagnostics packet.
pub const REMOTE_DRIFT_REPAIR_BETA_DIAGNOSTICS_PACKET_RECORD_KIND: &str =
    "remote_drift_repair_beta_diagnostics_packet";

/// Closed vocabulary describing *which kind* of drift the user is looking at.
///
/// A single record may carry multiple reasons (for example, a refused attach
/// where helper version is too old *and* the requested capability is no longer
/// admitted). The vocabulary is closed so reviewers, support, and diagnostics
/// consumers never invent free-form reason copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftReasonClass {
    /// Client and helper version tokens disagree outside the supported window.
    VersionMismatch,
    /// One or more capabilities cannot be admitted under the current pairing.
    CapabilityMismatch,
    /// Trust or credential verification did not admit the helper boundary.
    AuthMismatch,
    /// The selected attach route or transport is no longer valid for the pair.
    RouteMismatch,
    /// The target identity or workspace target the record refers to has
    /// drifted from what was bound at attach time.
    TargetMismatch,
}

impl DriftReasonClass {
    /// All drift-reason classes.
    pub const ALL: [Self; 5] = [
        Self::VersionMismatch,
        Self::CapabilityMismatch,
        Self::AuthMismatch,
        Self::RouteMismatch,
        Self::TargetMismatch,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VersionMismatch => "version_mismatch",
            Self::CapabilityMismatch => "capability_mismatch",
            Self::AuthMismatch => "auth_mismatch",
            Self::RouteMismatch => "route_mismatch",
            Self::TargetMismatch => "target_mismatch",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::VersionMismatch => "Version mismatch",
            Self::CapabilityMismatch => "Capability mismatch",
            Self::AuthMismatch => "Auth mismatch",
            Self::RouteMismatch => "Route mismatch",
            Self::TargetMismatch => "Target mismatch",
        }
    }
}

/// Closed vocabulary for the bounded recovery action a guidance record offers.
///
/// `Upgrade` and `Downgrade` are the two repair shapes the spec calls out by
/// name. The other classes preserve the truthful continuation options the
/// negotiation already exposes (reconnect, continue-local, probe, contact
/// support) so the in-product surface and the diagnostics packet share copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftRepairActionClass {
    /// No repair required; the negotiation already admits full remote.
    NoRepairRequired,
    /// Continue in the negotiated narrowed posture; mutation stays off until
    /// full support returns.
    ContinueNarrowedPosture,
    /// Run a drift probe or reattach to resolve an untested pairing.
    RunDriftProbe,
    /// Reconnect the same client/helper pair (for example, after a transient
    /// route or auth blip) before any other action.
    Reconnect,
    /// Upgrade the client or helper, or repin the protocol selection, to
    /// restore supported skew.
    Upgrade,
    /// Downgrade the requested capability set or the protocol selection to
    /// match the helper, restoring a supported narrowed posture.
    Downgrade,
    /// Continue locally only; the remote helper is refused but local work
    /// remains available.
    ContinueLocalOnly,
    /// Escalate to administrator or support; the lane cannot self-repair.
    ContactAdminOrSupport,
}

impl DriftRepairActionClass {
    /// All repair-action classes.
    pub const ALL: [Self; 8] = [
        Self::NoRepairRequired,
        Self::ContinueNarrowedPosture,
        Self::RunDriftProbe,
        Self::Reconnect,
        Self::Upgrade,
        Self::Downgrade,
        Self::ContinueLocalOnly,
        Self::ContactAdminOrSupport,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRepairRequired => "no_repair_required",
            Self::ContinueNarrowedPosture => "continue_narrowed_posture",
            Self::RunDriftProbe => "run_drift_probe",
            Self::Reconnect => "reconnect",
            Self::Upgrade => "upgrade",
            Self::Downgrade => "downgrade",
            Self::ContinueLocalOnly => "continue_local_only",
            Self::ContactAdminOrSupport => "contact_admin_or_support",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoRepairRequired => "No repair required",
            Self::ContinueNarrowedPosture => "Continue in narrowed posture",
            Self::RunDriftProbe => "Run drift probe or reattach",
            Self::Reconnect => "Reconnect helper",
            Self::Upgrade => "Upgrade or repin",
            Self::Downgrade => "Downgrade capability set",
            Self::ContinueLocalOnly => "Continue locally only",
            Self::ContactAdminOrSupport => "Contact admin or support",
        }
    }
}

/// Closed vocabulary describing how a repair action affects authority.
///
/// The spec's guardrail is that no path widens authority silently. Every
/// guidance record must therefore stamp the authority impact of its primary
/// action, and any action that would widen authority must be marked
/// [`Self::RequiresReapproval`] (never [`Self::WidensSilently`], which is
/// reserved as a forbidden value the schema rejects).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftRepairAuthorityImpactClass {
    /// The action neither widens nor narrows authority; the current posture
    /// continues.
    MaintainsCurrent,
    /// The action narrows authority (for example, from full-remote to
    /// review-only); user consent is implicit because authority shrinks.
    NarrowsAuthority,
    /// The action would widen authority and therefore requires an explicit
    /// re-approval step before it can run.
    RequiresReapproval,
}

impl DriftRepairAuthorityImpactClass {
    /// All authority-impact classes.
    pub const ALL: [Self; 3] = [
        Self::MaintainsCurrent,
        Self::NarrowsAuthority,
        Self::RequiresReapproval,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaintainsCurrent => "maintains_current",
            Self::NarrowsAuthority => "narrows_authority",
            Self::RequiresReapproval => "requires_reapproval",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MaintainsCurrent => "Maintains current authority",
            Self::NarrowsAuthority => "Narrows authority",
            Self::RequiresReapproval => "Requires explicit re-approval",
        }
    }

    /// True when running the action would widen authority. Surfaces honor this
    /// flag by routing the action through an approval prompt before it runs.
    pub const fn widens_authority(self) -> bool {
        matches!(self, Self::RequiresReapproval)
    }
}

/// One bounded recovery action attached to a drift-repair guidance record.
///
/// The action carries the closed action class, the authority impact, a stable
/// fails-closed flag, and a short reviewer-safe label so surfaces and
/// diagnostics packets can render it without re-deriving the impact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftRepairAction {
    /// Closed repair-action class.
    pub action_class: DriftRepairActionClass,
    /// Stable action-class token.
    pub action_class_token: String,
    /// Reviewer-facing action label.
    pub action_label: String,
    /// Closed authority-impact class for this action.
    pub authority_impact: DriftRepairAuthorityImpactClass,
    /// Stable authority-impact token.
    pub authority_impact_token: String,
    /// Reviewer-facing authority-impact label.
    pub authority_impact_label: String,
    /// True when the action keeps mutation off until the action completes.
    pub fails_closed: bool,
    /// True when the action would widen authority and therefore needs an
    /// explicit re-approval step; mirrors `authority_impact.widens_authority`.
    pub requires_reapproval: bool,
    /// Redaction-safe one-line summary for the action.
    pub visible_summary: String,
}

impl DriftRepairAction {
    /// Builds an action from a closed class, authority impact, fails-closed
    /// flag, and a short reviewer-safe summary.
    pub fn new(
        action_class: DriftRepairActionClass,
        authority_impact: DriftRepairAuthorityImpactClass,
        fails_closed: bool,
        visible_summary: impl Into<String>,
    ) -> Self {
        Self {
            action_class,
            action_class_token: action_class.as_str().to_owned(),
            action_label: action_class.label().to_owned(),
            authority_impact,
            authority_impact_token: authority_impact.as_str().to_owned(),
            authority_impact_label: authority_impact.label().to_owned(),
            fails_closed,
            requires_reapproval: authority_impact.widens_authority(),
            visible_summary: visible_summary.into(),
        }
    }
}

/// One drift-repair guidance record.
///
/// The record is derived from a [`RemoteHelperBetaRecord`] and adds the typed
/// drift reasoning, the primary recovery action, the alternative actions, and
/// the authority impact that surfaces and diagnostics packets consume.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteDriftRepairGuidance {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the diagnostics packet.
    pub schema_version: u32,
    /// Stable guidance row id shared with the diagnostics packet.
    pub guidance_row_id: String,
    /// Source beta-record row id.
    pub source_record_row_id: String,
    /// Envelope id from the source beta record.
    pub envelope_id: String,
    /// Lifecycle phase the source record minted.
    pub lifecycle_phase: RemoteHelperLifecyclePhaseClass,
    /// Stable lifecycle-phase token.
    pub lifecycle_phase_token: String,
    /// Closed drift reasons that apply to this record.
    pub drift_reasons: Vec<DriftReasonClass>,
    /// Stable drift-reason tokens; preserves the order in `drift_reasons`.
    pub drift_reason_tokens: Vec<String>,
    /// Primary recovery action; surfaces render this as the default prompt.
    pub primary_action: DriftRepairAction,
    /// Alternative recovery actions; preserves a stable ordering so reviewers
    /// see the same options as users.
    pub alternative_actions: Vec<DriftRepairAction>,
    /// True when the source negotiation fails closed for mutating remote work.
    pub fails_closed_for_mutation: bool,
    /// True when *any* action in `primary_action` plus `alternative_actions`
    /// would widen authority; surfaces gate every such action behind an
    /// explicit re-approval prompt.
    pub any_action_requires_reapproval: bool,
    /// Visible version disclosure lifted from the source record.
    pub visible_version_state: RemoteHelperVisibleVersionState,
    /// Effective posture token from the source record.
    pub effective_posture_token: String,
    /// Negotiation-outcome token from the source record.
    pub negotiation_outcome_token: String,
    /// Visible skew-posture token from the source record.
    pub skew_visibility_token: String,
    /// Source-record repair-path token (the bounded skew classifier).
    pub source_repair_path_token: String,
    /// Redaction-safe summary lifted from the source record.
    pub visible_summary: String,
    /// Redaction-safe safe-continuation lifted from the source record.
    pub safe_continuation: String,
    /// Source refs imported from the source record.
    pub source_refs: Vec<String>,
    /// Support packet refs the diagnostics packet attaches to.
    pub support_packet_refs: Vec<String>,
    /// Compatibility-report row refs the source record contributes to.
    pub compatibility_report_row_refs: Vec<String>,
    /// True because raw versions, endpoints, paths, and secrets are excluded.
    pub redaction_safe: bool,
}

impl RemoteDriftRepairGuidance {
    /// Builds a guidance record from a beta remote-helper record.
    pub fn from_record(record: &RemoteHelperBetaRecord) -> Self {
        let reasons = derive_drift_reasons(record);
        let primary_action = derive_primary_action(record);
        let alternative_actions = derive_alternative_actions(record, primary_action.action_class);
        let any_action_requires_reapproval = primary_action.requires_reapproval
            || alternative_actions
                .iter()
                .any(|action| action.requires_reapproval);
        Self {
            record_kind: REMOTE_DRIFT_REPAIR_BETA_GUIDANCE_RECORD_KIND.to_owned(),
            schema_version: REMOTE_DRIFT_REPAIR_BETA_SCHEMA_VERSION,
            guidance_row_id: format!(
                "remote-drift-repair-row:{}",
                strip_row_prefix(&record.row_id)
            ),
            source_record_row_id: record.row_id.clone(),
            envelope_id: record.envelope_id.clone(),
            lifecycle_phase: record.lifecycle_phase,
            lifecycle_phase_token: record.lifecycle_phase_token.clone(),
            drift_reason_tokens: reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect(),
            drift_reasons: reasons,
            primary_action,
            alternative_actions,
            fails_closed_for_mutation: record.fails_closed_for_mutation(),
            any_action_requires_reapproval,
            visible_version_state: record.visible_version_state.clone(),
            effective_posture_token: record.effective_posture_token.clone(),
            negotiation_outcome_token: record.negotiation_outcome_token.clone(),
            skew_visibility_token: record.skew_visibility_token.clone(),
            source_repair_path_token: record.repair_path_token.clone(),
            visible_summary: record.visible_summary.clone(),
            safe_continuation: record.safe_continuation.clone(),
            source_refs: record.source_refs.clone(),
            support_packet_refs: record.support_packet_refs.clone(),
            compatibility_report_row_refs: record.compatibility_report_row_refs.clone(),
            redaction_safe: true,
        }
    }

    /// Returns one deterministic plaintext summary line.
    pub fn summary_line(&self) -> String {
        let reasons: Vec<&str> = self
            .drift_reason_tokens
            .iter()
            .map(String::as_str)
            .collect();
        format!(
            "row={}; phase={}; reasons=[{}]; primary={}; impact={}; fails_closed={}; requires_reapproval={}",
            self.guidance_row_id,
            self.lifecycle_phase_token,
            reasons.join(","),
            self.primary_action.action_class_token,
            self.primary_action.authority_impact_token,
            self.fails_closed_for_mutation,
            self.any_action_requires_reapproval,
        )
    }
}

/// Exportable diagnostics packet for the drift-repair guidance lane.
///
/// The packet bundles guidance records produced from a sequence of beta
/// records and preserves the same reasoning shown in-product so support
/// reviewers and CI/headless inspectors read one truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteDriftRepairDiagnosticsPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable diagnostics-packet id.
    pub diagnostics_packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Guidance records embedded in this packet.
    pub guidance_records: Vec<RemoteDriftRepairGuidance>,
    /// True when at least one guidance record fails closed for mutation.
    pub any_record_fails_closed_for_mutation: bool,
    /// True when at least one guidance record exposes an action that requires
    /// explicit re-approval; the packet itself never widens authority.
    pub any_record_requires_reapproval: bool,
    /// All drift-reason tokens that appear in the packet, deduplicated and
    /// sorted, so reviewers see coverage at a glance.
    pub drift_reason_summary_tokens: Vec<String>,
    /// All repair-action tokens that appear in the packet, deduplicated and
    /// sorted.
    pub repair_action_summary_tokens: Vec<String>,
    /// True because raw payloads, endpoints, paths, and secrets are excluded.
    pub redaction_safe: bool,
}

impl RemoteDriftRepairDiagnosticsPacket {
    /// Builds a diagnostics packet from a sequence of guidance records.
    pub fn from_guidance<'a>(
        diagnostics_packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        guidance: impl IntoIterator<Item = &'a RemoteDriftRepairGuidance>,
    ) -> Self {
        let guidance_records: Vec<RemoteDriftRepairGuidance> =
            guidance.into_iter().cloned().collect();
        let any_record_fails_closed_for_mutation = guidance_records
            .iter()
            .any(|record| record.fails_closed_for_mutation);
        let any_record_requires_reapproval = guidance_records
            .iter()
            .any(|record| record.any_action_requires_reapproval);
        let mut reason_set: BTreeSet<String> = BTreeSet::new();
        let mut action_set: BTreeSet<String> = BTreeSet::new();
        for record in &guidance_records {
            for token in &record.drift_reason_tokens {
                reason_set.insert(token.clone());
            }
            action_set.insert(record.primary_action.action_class_token.clone());
            for action in &record.alternative_actions {
                action_set.insert(action.action_class_token.clone());
            }
        }
        Self {
            record_kind: REMOTE_DRIFT_REPAIR_BETA_DIAGNOSTICS_PACKET_RECORD_KIND.to_owned(),
            schema_version: REMOTE_DRIFT_REPAIR_BETA_SCHEMA_VERSION,
            diagnostics_packet_id: diagnostics_packet_id.into(),
            generated_at: generated_at.into(),
            guidance_records,
            any_record_fails_closed_for_mutation,
            any_record_requires_reapproval,
            drift_reason_summary_tokens: reason_set.into_iter().collect(),
            repair_action_summary_tokens: action_set.into_iter().collect(),
            redaction_safe: true,
        }
    }

    /// Builds a diagnostics packet directly from a sequence of beta records.
    pub fn from_records<'a>(
        diagnostics_packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: impl IntoIterator<Item = &'a RemoteHelperBetaRecord>,
    ) -> Self {
        let guidance: Vec<RemoteDriftRepairGuidance> = records
            .into_iter()
            .map(RemoteDriftRepairGuidance::from_record)
            .collect();
        Self::from_guidance(diagnostics_packet_id, generated_at, guidance.iter())
    }
}

fn strip_row_prefix(row_id: &str) -> String {
    row_id
        .strip_prefix("remote-helper-beta-row:")
        .unwrap_or(row_id)
        .to_owned()
}

fn derive_drift_reasons(record: &RemoteHelperBetaRecord) -> Vec<DriftReasonClass> {
    let mut seen: BTreeSet<DriftReasonClass> = BTreeSet::new();
    let mut ordered: Vec<DriftReasonClass> = Vec::new();
    let push = |reason: DriftReasonClass, ordered: &mut Vec<_>, seen: &mut BTreeSet<_>| {
        if seen.insert(reason) {
            ordered.push(reason);
        }
    };

    let drops_imply_version = record.dropped_capabilities.iter().any(|drop| {
        matches!(
            drop.reason_class,
            MissingCapabilityReasonClass::OutsideSkewWindow
                | MissingCapabilityReasonClass::ProtocolFloorMismatch
        )
    });
    let drops_imply_auth = record.dropped_capabilities.iter().any(|drop| {
        matches!(
            drop.reason_class,
            MissingCapabilityReasonClass::TrustNotVerified
        )
    });

    let target_scoped_posture = matches!(
        record.effective_posture,
        EffectiveCapabilityPosture::LocalOnly | EffectiveCapabilityPosture::Blocked
    );

    if matches!(
        record.skew_visibility,
        RemoteHelperSkewVisibilityClass::ProbeRequiredUntested
    ) {
        push(DriftReasonClass::VersionMismatch, &mut ordered, &mut seen);
    }

    if matches!(
        record.skew_visibility,
        RemoteHelperSkewVisibilityClass::OutsideSupportedWindow
    ) {
        if drops_imply_version {
            push(DriftReasonClass::VersionMismatch, &mut ordered, &mut seen);
        } else if record.dropped_capabilities.is_empty() && !target_scoped_posture {
            push(DriftReasonClass::VersionMismatch, &mut ordered, &mut seen);
        }
    }

    for dropped in &record.dropped_capabilities {
        match dropped.reason_class {
            MissingCapabilityReasonClass::HelperDoesNotOffer
            | MissingCapabilityReasonClass::ClientRequiresUnknownFeature
            | MissingCapabilityReasonClass::PolicyNarrowed
            | MissingCapabilityReasonClass::ProtocolFloorMismatch => {
                push(
                    DriftReasonClass::CapabilityMismatch,
                    &mut ordered,
                    &mut seen,
                );
            }
            MissingCapabilityReasonClass::OutsideSkewWindow => {
                push(DriftReasonClass::VersionMismatch, &mut ordered, &mut seen);
                push(
                    DriftReasonClass::CapabilityMismatch,
                    &mut ordered,
                    &mut seen,
                );
            }
            MissingCapabilityReasonClass::TrustNotVerified => {
                push(DriftReasonClass::AuthMismatch, &mut ordered, &mut seen);
            }
            MissingCapabilityReasonClass::ProbeRequired => {
                push(
                    DriftReasonClass::CapabilityMismatch,
                    &mut ordered,
                    &mut seen,
                );
            }
        }
    }

    if matches!(
        record.lifecycle_phase,
        RemoteHelperLifecyclePhaseClass::Reconnect
    ) && matches!(
        record.skew_visibility,
        RemoteHelperSkewVisibilityClass::ProbeRequiredUntested
    ) {
        push(DriftReasonClass::RouteMismatch, &mut ordered, &mut seen);
    }

    if target_scoped_posture
        && record.dropped_capabilities.is_empty()
        && matches!(
            record.skew_visibility,
            RemoteHelperSkewVisibilityClass::OutsideSupportedWindow
                | RemoteHelperSkewVisibilityClass::NarrowedSupportedWindow
        )
    {
        push(DriftReasonClass::TargetMismatch, &mut ordered, &mut seen);
    } else if matches!(
        record.effective_posture,
        EffectiveCapabilityPosture::Blocked
    ) && !drops_imply_auth
        && !drops_imply_version
        && record.dropped_capabilities.is_empty()
    {
        push(DriftReasonClass::TargetMismatch, &mut ordered, &mut seen);
    }

    if matches!(record.negotiation_outcome, NegotiationOutcome::Refuse)
        && record.dropped_capabilities.is_empty()
        && !matches!(
            record.skew_visibility,
            RemoteHelperSkewVisibilityClass::AdjacentSupported
                | RemoteHelperSkewVisibilityClass::NarrowedSupportedWindow
        )
        && ordered.is_empty()
    {
        push(DriftReasonClass::VersionMismatch, &mut ordered, &mut seen);
    }

    ordered
}

fn derive_primary_action(record: &RemoteHelperBetaRecord) -> DriftRepairAction {
    match record.repair_path {
        RemoteHelperRepairPathClass::NoRepairRequired => DriftRepairAction::new(
            DriftRepairActionClass::NoRepairRequired,
            DriftRepairAuthorityImpactClass::MaintainsCurrent,
            false,
            "Negotiated capability set is admitted; no repair required.",
        ),
        RemoteHelperRepairPathClass::ContinueNarrowedPosture => DriftRepairAction::new(
            DriftRepairActionClass::ContinueNarrowedPosture,
            DriftRepairAuthorityImpactClass::NarrowsAuthority,
            true,
            "Continue in the narrowed posture; mutation stays off until full support returns.",
        ),
        RemoteHelperRepairPathClass::RunDriftProbeOrReattach => DriftRepairAction::new(
            DriftRepairActionClass::RunDriftProbe,
            DriftRepairAuthorityImpactClass::MaintainsCurrent,
            true,
            "Run a drift probe or reattach to resolve the untested pairing.",
        ),
        RemoteHelperRepairPathClass::UpgradeOrRepin => DriftRepairAction::new(
            DriftRepairActionClass::Upgrade,
            DriftRepairAuthorityImpactClass::RequiresReapproval,
            true,
            "Upgrade or repin the client or helper to restore the supported skew window.",
        ),
        RemoteHelperRepairPathClass::ContinueLocalOnly => DriftRepairAction::new(
            DriftRepairActionClass::ContinueLocalOnly,
            DriftRepairAuthorityImpactClass::NarrowsAuthority,
            true,
            "Continue locally only; remote helper authority remains refused.",
        ),
        RemoteHelperRepairPathClass::ContactAdminOrSupport => DriftRepairAction::new(
            DriftRepairActionClass::ContactAdminOrSupport,
            DriftRepairAuthorityImpactClass::MaintainsCurrent,
            true,
            "Lane cannot self-repair; contact administrator or support.",
        ),
    }
}

fn derive_alternative_actions(
    record: &RemoteHelperBetaRecord,
    primary: DriftRepairActionClass,
) -> Vec<DriftRepairAction> {
    let mut actions: Vec<DriftRepairAction> = Vec::new();
    let mut seen: BTreeSet<DriftRepairActionClass> = BTreeSet::new();
    seen.insert(primary);

    let push = |action: DriftRepairAction,
                actions: &mut Vec<DriftRepairAction>,
                seen: &mut BTreeSet<DriftRepairActionClass>| {
        if seen.insert(action.action_class) {
            actions.push(action);
        }
    };

    let has_downgradable_capability = record
        .dropped_capabilities
        .iter()
        .any(|drop| drop.retryable);

    match record.skew_visibility {
        RemoteHelperSkewVisibilityClass::AdjacentSupported => {}
        RemoteHelperSkewVisibilityClass::NarrowedSupportedWindow => {
            push(
                DriftRepairAction::new(
                    DriftRepairActionClass::Upgrade,
                    DriftRepairAuthorityImpactClass::RequiresReapproval,
                    true,
                    "Upgrade or repin the client or helper to restore the full capability set.",
                ),
                &mut actions,
                &mut seen,
            );
            push(
                DriftRepairAction::new(
                    DriftRepairActionClass::ContinueLocalOnly,
                    DriftRepairAuthorityImpactClass::NarrowsAuthority,
                    true,
                    "Continue locally only and leave remote work pending.",
                ),
                &mut actions,
                &mut seen,
            );
        }
        RemoteHelperSkewVisibilityClass::ProbeRequiredUntested => {
            if matches!(
                record.lifecycle_phase,
                RemoteHelperLifecyclePhaseClass::Reconnect
            ) {
                push(
                    DriftRepairAction::new(
                        DriftRepairActionClass::Reconnect,
                        DriftRepairAuthorityImpactClass::MaintainsCurrent,
                        true,
                        "Reconnect the helper before running a probe.",
                    ),
                    &mut actions,
                    &mut seen,
                );
            }
            push(
                DriftRepairAction::new(
                    DriftRepairActionClass::ContinueLocalOnly,
                    DriftRepairAuthorityImpactClass::NarrowsAuthority,
                    true,
                    "Continue locally only while the probe is scheduled.",
                ),
                &mut actions,
                &mut seen,
            );
        }
        RemoteHelperSkewVisibilityClass::OutsideSupportedWindow => {
            if has_downgradable_capability {
                push(
                    DriftRepairAction::new(
                        DriftRepairActionClass::Downgrade,
                        DriftRepairAuthorityImpactClass::NarrowsAuthority,
                        true,
                        "Downgrade the requested capability set to match the helper.",
                    ),
                    &mut actions,
                    &mut seen,
                );
            }
            push(
                DriftRepairAction::new(
                    DriftRepairActionClass::ContinueLocalOnly,
                    DriftRepairAuthorityImpactClass::NarrowsAuthority,
                    true,
                    "Continue locally only; remote helper authority remains refused.",
                ),
                &mut actions,
                &mut seen,
            );
            if matches!(
                record.effective_posture,
                EffectiveCapabilityPosture::Blocked
            ) {
                push(
                    DriftRepairAction::new(
                        DriftRepairActionClass::ContactAdminOrSupport,
                        DriftRepairAuthorityImpactClass::MaintainsCurrent,
                        true,
                        "Lane cannot self-repair; contact administrator or support.",
                    ),
                    &mut actions,
                    &mut seen,
                );
            }
        }
    }

    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability_negotiation::{
        CompatibilityWindow, CompatibilityWindowStatus, DroppedHelperCapability,
        EffectiveCapabilityPosture, HelperCapabilityResponse, MissingCapabilityReasonClass,
        NegotiationOutcome, HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
    };

    fn response(
        status: CompatibilityWindowStatus,
        outcome: NegotiationOutcome,
        posture: EffectiveCapabilityPosture,
        mutation_allowed: bool,
        dropped: Vec<DroppedHelperCapability>,
    ) -> HelperCapabilityResponse {
        HelperCapabilityResponse {
            schema_version: HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
            request_id: "helper_capability_envelope:test.case".to_owned(),
            row_id: "drift_truth.helper_agent.test.case".to_owned(),
            surface_ref: "remote_agent_attach".to_owned(),
            title: "test".to_owned(),
            outcome,
            selected_protocol_ref: "protocol@2026.05".to_owned(),
            negotiated_capabilities: Vec::new(),
            dropped_capabilities: dropped,
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
                source_refs: Vec::new(),
            },
        }
    }

    fn record_from(
        response: HelperCapabilityResponse,
        phase: RemoteHelperLifecyclePhaseClass,
    ) -> RemoteHelperBetaRecord {
        RemoteHelperBetaRecord::from_response(
            &response,
            phase,
            0,
            "attach_session:remote.test".to_owned(),
            "client@2026.05",
            "agent@2026.05",
            vec!["compat_report_row:remote.test".to_owned()],
        )
    }

    #[test]
    fn adjacent_supported_emits_no_repair_required() {
        let response = response(
            CompatibilityWindowStatus::Supported,
            NegotiationOutcome::Match,
            EffectiveCapabilityPosture::FullRemote,
            true,
            Vec::new(),
        );
        let record = record_from(response, RemoteHelperLifecyclePhaseClass::Attach);
        let guidance = RemoteDriftRepairGuidance::from_record(&record);
        assert!(guidance.drift_reasons.is_empty());
        assert_eq!(
            guidance.primary_action.action_class,
            DriftRepairActionClass::NoRepairRequired
        );
        assert!(guidance.alternative_actions.is_empty());
        assert!(!guidance.fails_closed_for_mutation);
        assert!(!guidance.any_action_requires_reapproval);
    }

    #[test]
    fn version_mismatch_yields_upgrade_primary_with_authority_widening_flag() {
        let response = response(
            CompatibilityWindowStatus::Unsupported,
            NegotiationOutcome::Refuse,
            EffectiveCapabilityPosture::ReviewOnly,
            false,
            vec![DroppedHelperCapability {
                capability: "remote.review.preview".to_owned(),
                reason_class: MissingCapabilityReasonClass::OutsideSkewWindow,
                visible_reason: "Helper outside supported window for preview".to_owned(),
                retryable: false,
            }],
        );
        let record = record_from(response, RemoteHelperLifecyclePhaseClass::Attach);
        let guidance = RemoteDriftRepairGuidance::from_record(&record);
        assert!(guidance
            .drift_reasons
            .contains(&DriftReasonClass::VersionMismatch));
        assert!(guidance
            .drift_reasons
            .contains(&DriftReasonClass::CapabilityMismatch));
        assert_eq!(
            guidance.primary_action.action_class,
            DriftRepairActionClass::Upgrade
        );
        assert!(guidance.primary_action.requires_reapproval);
        assert!(guidance.any_action_requires_reapproval);
        assert!(guidance.fails_closed_for_mutation);
    }

    #[test]
    fn untested_reconnect_emits_probe_with_reconnect_alternative() {
        let response = response(
            CompatibilityWindowStatus::Untested,
            NegotiationOutcome::Refuse,
            EffectiveCapabilityPosture::InspectOnly,
            false,
            vec![DroppedHelperCapability {
                capability: "remote.file.write".to_owned(),
                reason_class: MissingCapabilityReasonClass::ProbeRequired,
                visible_reason: "probe required".to_owned(),
                retryable: true,
            }],
        );
        let record = record_from(response, RemoteHelperLifecyclePhaseClass::Reconnect);
        let guidance = RemoteDriftRepairGuidance::from_record(&record);
        assert_eq!(
            guidance.primary_action.action_class,
            DriftRepairActionClass::RunDriftProbe
        );
        assert!(guidance
            .alternative_actions
            .iter()
            .any(|action| action.action_class == DriftRepairActionClass::Reconnect));
        assert!(guidance.fails_closed_for_mutation);
        assert!(guidance
            .drift_reasons
            .contains(&DriftReasonClass::RouteMismatch));
    }

    #[test]
    fn auth_trust_mismatch_is_classified_as_auth_reason() {
        let response = response(
            CompatibilityWindowStatus::Supported,
            NegotiationOutcome::Refuse,
            EffectiveCapabilityPosture::Blocked,
            false,
            vec![DroppedHelperCapability {
                capability: "remote.fs.read".to_owned(),
                reason_class: MissingCapabilityReasonClass::TrustNotVerified,
                visible_reason: "trust not verified".to_owned(),
                retryable: false,
            }],
        );
        let record = record_from(response, RemoteHelperLifecyclePhaseClass::Attach);
        let guidance = RemoteDriftRepairGuidance::from_record(&record);
        assert!(guidance
            .drift_reasons
            .contains(&DriftReasonClass::AuthMismatch));
    }

    #[test]
    fn diagnostics_packet_summarises_reasons_and_actions() {
        let supported = record_from(
            response(
                CompatibilityWindowStatus::Supported,
                NegotiationOutcome::Match,
                EffectiveCapabilityPosture::FullRemote,
                true,
                Vec::new(),
            ),
            RemoteHelperLifecyclePhaseClass::Attach,
        );
        let unsupported = record_from(
            response(
                CompatibilityWindowStatus::Unsupported,
                NegotiationOutcome::Refuse,
                EffectiveCapabilityPosture::ReviewOnly,
                false,
                vec![DroppedHelperCapability {
                    capability: "remote.review.preview".to_owned(),
                    reason_class: MissingCapabilityReasonClass::OutsideSkewWindow,
                    visible_reason: "outside window".to_owned(),
                    retryable: true,
                }],
            ),
            RemoteHelperLifecyclePhaseClass::Attach,
        );
        let packet = RemoteDriftRepairDiagnosticsPacket::from_records(
            "remote_drift_repair_packet:test",
            "2026-05-16T00:00:00Z",
            [&supported, &unsupported],
        );
        assert_eq!(packet.guidance_records.len(), 2);
        assert!(packet
            .drift_reason_summary_tokens
            .contains(&"version_mismatch".to_owned()));
        assert!(packet
            .drift_reason_summary_tokens
            .contains(&"capability_mismatch".to_owned()));
        assert!(packet
            .repair_action_summary_tokens
            .iter()
            .any(|token| token == "upgrade"));
        assert!(packet.any_record_fails_closed_for_mutation);
        assert!(packet.any_record_requires_reapproval);
    }
}
