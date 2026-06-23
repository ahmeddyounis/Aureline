//! M5 admin-plane *decision-history timelines and audit-event explorers*: the
//! concrete, typed instances of the decision-history / audit surface that
//! Aureline shows on its claimed managed, self-hosted, sovereign/air-gapped, and
//! mirrored/offline profiles.
//!
//! Where [`m5_admin_plane`](crate::m5_admin_plane) *names and freezes the
//! contract* — including the
//! [`DecisionHistoryTimeline`](crate::m5_admin_plane::AdminSurfaceClass::DecisionHistoryTimeline)
//! surface family, its applicable states, the controlled vocabularies it binds,
//! and the proof packet that keeps it current — this lane *renders that surface*.
//! It turns recent material allow / deny / quota / force-disable / publish-scope
//! decisions into a first-class local product surface: a user or admin can read,
//! on the machine in front of them, what was decided, who or what decided it, the
//! policy epoch and affected scope it applied to, when it happened, and where to
//! read the full explanation — without scraping logs or opening a separate vendor
//! console.
//!
//! Each timeline binds back to the matrix. Every state an event or the coverage
//! posture shows must be one the matrix declares applicable for the
//! decision-history surface
//! ([`DecisionHistoryInvariant`] `decision_history.surface_states_within_matrix`),
//! and every owner and data-residency token it uses is a term the matrix's shared
//! vocabulary defines. So the render layer cannot drift from the frozen contract:
//! an edit that shows a state the matrix does not admit flips an invariant and
//! fails the freeze gate.
//!
//! The bundle holds one [`DecisionHistoryPacket`] per claimed managed-bearing
//! profile and computes each invariant's `holds` flag from the rendered data, so
//! the checked-in fixture freezes the rendered timelines byte-for-byte. Honesty
//! rules are enforced, not just described:
//!
//! - Every event carries a *specific* [`ActorClass`] — user action, admin action,
//!   policy evaluation, provider limitation, or client limitation — instead of
//!   collapsing into a generic blocked/error row
//!   (`decision_history.actor_classes_distinguished`).
//! - Every event names a stable decision code, a policy epoch, an affected scope,
//!   a time, and an export-safe summary (`decision_history.decision_truth`).
//! - The explorer offers a filter for each of the eight audit families and every
//!   event resolves to exactly one of them (`decision_history.explorer_filters_complete`).
//! - Every row is exportable both as a machine-readable summary and a
//!   plain-language support/admin handoff sentence (`decision_history.export_parity`).
//! - An event whose backing evidence is stale or offline is never shown as a
//!   confirmed-green decision (`decision_history.no_silent_green`).
//! - Self-hosted, sovereign, mirrored, and offline-capable profiles keep a
//!   locally inspectable, vendor-console-independent history
//!   (`decision_history.locally_inspectable_offline`).
//!
//! The record carries no endpoint URLs, hostnames, credentials, raw provider
//! payloads, raw policy bodies, or absolute paths — only opaque object refs,
//! stable tokens, rendered metadata-safe summaries, and short reviewable
//! sentences — so it is safe to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_admin_plane::{
    admin_plane_matrix, all_unique, is_export_safe_ref, AdminConsumerClass,
    AdminDeploymentProfileClass, AdminPathClass, AdminRedactionClass, AdminStateClass,
    AdminSurfaceClass, M5_ADMIN_PLANE_MATRIX_ID,
};
use crate::m5_admin_render::{DataResidencyClass, EvidenceAgeClass, OwnerEscalationRoleClass};

#[cfg(test)]
mod tests;

/// Schema version for the decision-history bundle.
pub const M5_DECISION_HISTORY_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the decision-history bundle.
pub const M5_DECISION_HISTORY_SCHEMA_REF: &str = "schemas/admin/m5-decision-history.schema.json";

/// Stable record-kind tag for the decision-history bundle.
pub const M5_DECISION_HISTORY_RECORD_KIND: &str = "m5_decision_history_bundle";

/// Stable id for the canonical decision-history bundle.
pub const M5_DECISION_HISTORY_BUNDLE_ID: &str = "m5-decision-history:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_DECISION_HISTORY_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The matrix this render layer binds back to.
pub const M5_DECISION_HISTORY_MATRIX_REF: &str =
    "fixtures/admin/m5-admin-plane/canonical_matrix.json";

/// The freeze gate that keeps the decision-history bundle current.
pub const M5_DECISION_HISTORY_FREEZE_GATE_REF: &str =
    "crates/aureline-policy/tests/m5_decision_history.rs";

// ---------------------------------------------------------------------------
// Decision-history token enums.
// ---------------------------------------------------------------------------

/// The actor class behind a decision — the spec's honesty requirement to
/// distinguish a user action, an admin action, a policy evaluation, a provider
/// limitation, and a client limitation rather than collapsing everything into a
/// generic blocked/error event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
    /// The local user initiated the action.
    UserAction,
    /// An admin (org/security/compliance) initiated or imposed the action.
    AdminAction,
    /// The policy engine evaluated a rule and resolved the outcome.
    PolicyEvaluation,
    /// An upstream provider limited, refused, or was unreachable for the action.
    ProviderLimitation,
    /// The local client could not perform the action (capability or offline
    /// limit), not a provider or policy denial.
    ClientLimitation,
}

impl ActorClass {
    /// All actor classes, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::UserAction,
        Self::AdminAction,
        Self::PolicyEvaluation,
        Self::ProviderLimitation,
        Self::ClientLimitation,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAction => "user_action",
            Self::AdminAction => "admin_action",
            Self::PolicyEvaluation => "policy_evaluation",
            Self::ProviderLimitation => "provider_limitation",
            Self::ClientLimitation => "client_limitation",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UserAction => "User action",
            Self::AdminAction => "Admin action",
            Self::PolicyEvaluation => "Policy evaluation",
            Self::ProviderLimitation => "Provider limitation",
            Self::ClientLimitation => "Client limitation",
        }
    }
}

/// The stable decision code for a material allow / deny / quota / force-disable /
/// publish-scope decision. The token set mirrors the `decision_class` vocabulary
/// already frozen in the audit-event explorer contract so the render layer and
/// the durable audit rows speak the same codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionCodeClass {
    /// The action was allowed.
    Allow,
    /// The action was denied.
    Deny,
    /// The action was allowed but narrowed in scope.
    Narrow,
    /// A capability was force-disabled by policy.
    ForceDisable,
    /// The action hit a quota / rate limit.
    QuotaLimit,
    /// The decision is deferred pending a fresh policy/entitlement refresh.
    DeferPendingRefresh,
    /// The decision is deferred pending an admin step.
    DeferPendingAdmin,
    /// The decision was escalated to an owner.
    Escalate,
    /// Only an export of user-owned data is permitted.
    ExportOnly,
    /// The action proceeds local-only with managed effects withheld.
    LocalOnlyContinue,
    /// A remote mutation was recorded.
    MutationRecorded,
    /// A user request was recorded to act on later.
    RequestRecorded,
    /// A rollback was recorded.
    RollbackRecorded,
    /// The outcome could not be confirmed while offline.
    UnknownOffline,
}

impl DecisionCodeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Narrow => "narrow",
            Self::ForceDisable => "force_disable",
            Self::QuotaLimit => "quota_limit",
            Self::DeferPendingRefresh => "defer_pending_refresh",
            Self::DeferPendingAdmin => "defer_pending_admin",
            Self::Escalate => "escalate",
            Self::ExportOnly => "export_only",
            Self::LocalOnlyContinue => "local_only_continue",
            Self::MutationRecorded => "mutation_recorded",
            Self::RequestRecorded => "request_recorded",
            Self::RollbackRecorded => "rollback_recorded",
            Self::UnknownOffline => "unknown_offline",
        }
    }
}

/// The eight audit-event explorer filter families the spec requires: trust,
/// policy, auth, remote mutation, provider routing, collaboration control,
/// publish state, and managed-identity scope changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventFamilyClass {
    /// Trust-root, signer, and verification posture changes.
    TrustChange,
    /// Policy bundle / effective-policy changes.
    PolicyChange,
    /// Authentication and session lifecycle events.
    AuthSession,
    /// Remote mutation receipts (writes against a remote/managed target).
    RemoteMutation,
    /// AI/provider routing and network egress decisions.
    ProviderRouting,
    /// Collaboration-control grants and revocations.
    CollaborationControl,
    /// Publish-state / marketplace publication-scope changes.
    PublishState,
    /// Managed-identity scope changes (org switch, seat, directory scope).
    ManagedIdentityScope,
}

impl EventFamilyClass {
    /// All families, in explorer order.
    pub const ALL: [Self; 8] = [
        Self::TrustChange,
        Self::PolicyChange,
        Self::AuthSession,
        Self::RemoteMutation,
        Self::ProviderRouting,
        Self::CollaborationControl,
        Self::PublishState,
        Self::ManagedIdentityScope,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustChange => "trust_change",
            Self::PolicyChange => "policy_change",
            Self::AuthSession => "auth_session",
            Self::RemoteMutation => "remote_mutation",
            Self::ProviderRouting => "provider_routing",
            Self::CollaborationControl => "collaboration_control",
            Self::PublishState => "publish_state",
            Self::ManagedIdentityScope => "managed_identity_scope",
        }
    }

    /// The user-facing filter label shown in the explorer.
    pub const fn filter_label(self) -> &'static str {
        match self {
            Self::TrustChange => "Trust",
            Self::PolicyChange => "Policy",
            Self::AuthSession => "Auth",
            Self::RemoteMutation => "Remote mutation",
            Self::ProviderRouting => "Provider routing",
            Self::CollaborationControl => "Collaboration control",
            Self::PublishState => "Publish state",
            Self::ManagedIdentityScope => "Managed identity scope",
        }
    }
}

/// The scope kind an event affects — the controlled vocabulary the audit rows
/// use for the affected target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKindClass {
    /// The whole tenant / organization.
    TenantOrOrg,
    /// A deployment profile.
    DeploymentProfile,
    /// A workspace.
    Workspace,
    /// A group.
    Group,
    /// A seat.
    Seat,
    /// A device / install.
    Device,
    /// A capability scope.
    CapabilityScope,
    /// A session or command.
    SessionOrCommand,
}

impl ScopeKindClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TenantOrOrg => "tenant_or_org",
            Self::DeploymentProfile => "deployment_profile",
            Self::Workspace => "workspace",
            Self::Group => "group",
            Self::Seat => "seat",
            Self::Device => "device",
            Self::CapabilityScope => "capability_scope",
            Self::SessionOrCommand => "session_or_command",
        }
    }
}

/// How complete the rendered history window is — labeled honestly so the explorer
/// never implies a full history it does not have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessClass {
    /// The window is complete for its declared range.
    Complete,
    /// The live tail is missing because the managed/mirror source is offline.
    PartialOffline,
    /// The history is replayed from an imported snapshot with no live tail.
    PartialImported,
    /// Some rows are withheld by a redaction floor and the count says so.
    PartialRedacted,
}

impl CompletenessClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::PartialOffline => "partial_offline",
            Self::PartialImported => "partial_imported",
            Self::PartialRedacted => "partial_redacted",
        }
    }

    /// Whether this window is partial (gaps must be labeled, never implied
    /// complete).
    pub const fn is_partial(self) -> bool {
        !matches!(self, Self::Complete)
    }
}

/// The export forms a timeline offers. Both must be available so a row can be
/// copied or exported as a machine-readable summary and as a plain-language
/// support/admin handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormatClass {
    /// Machine-readable JSON summary objects.
    MachineReadableJson,
    /// Plain-language support / admin handoff packet.
    PlainLanguageHandoff,
}

impl ExportFormatClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MachineReadableJson => "machine_readable_json",
            Self::PlainLanguageHandoff => "plain_language_handoff",
        }
    }
}

// ---------------------------------------------------------------------------
// Decision event.
// ---------------------------------------------------------------------------

/// One material decision in a profile's decision-history timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEvent {
    /// Stable, opaque event id (deep-linkable, export-safe).
    pub event_id: String,
    /// The stable decision code.
    pub decision_code: DecisionCodeClass,
    /// The audit family this event belongs to (one of the eight explorer
    /// filters).
    pub event_family: EventFamilyClass,
    /// The actor class behind the decision.
    pub actor_class: ActorClass,
    /// One reviewable label for the actor (never a raw user identifier).
    pub actor_label: String,
    /// The affected target (opaque control/capability/object token).
    pub affected_target: String,
    /// The scope kind the target sits in.
    pub scope_kind: ScopeKindClass,
    /// One reviewable label for the affected scope.
    pub scope_label: String,
    /// The policy epoch under which the decision was evaluated (opaque token).
    pub policy_epoch: String,
    /// The entitlement epoch, when the decision is entitlement-bearing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_epoch: Option<String>,
    /// When the decision happened (ISO-8601 UTC).
    pub event_at: String,
    /// Monotonic sequence within the profile timeline.
    pub sequence: u32,
    /// The resolved outcome state (must be one the matrix admits for this
    /// surface).
    pub outcome_state: AdminStateClass,
    /// The freshness of the evidence backing the event.
    pub evidence_age: EvidenceAgeClass,
    /// Where this event's data lives.
    pub data_residency: DataResidencyClass,
    /// Who owns the decision or its next step.
    pub owner: OwnerEscalationRoleClass,
    /// The explanation this event links to (a locked-state explanation, effective
    /// control, or source packet), if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_ref: Option<String>,
    /// The export-safe machine-readable summary (stable tokens, never a secret).
    pub machine_summary: String,
    /// The plain-language support/admin handoff sentence.
    pub plain_language: String,
}

impl DecisionEvent {
    /// Whether the event asserts a currently-confirmed decision.
    pub fn is_confirmed(&self) -> bool {
        self.outcome_state == AdminStateClass::ActiveEnforced
    }

    /// Whether the event carries both export representations.
    pub fn has_export_parity(&self) -> bool {
        !self.machine_summary.is_empty() && !self.plain_language.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Explorer filter, export form, coverage.
// ---------------------------------------------------------------------------

/// One audit-event explorer filter, with the events it currently matches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerFilter {
    /// Stable filter id.
    pub filter_id: String,
    /// The family this filter selects.
    pub family: EventFamilyClass,
    /// The user-facing filter label.
    pub label: String,
    /// One reviewable sentence describing what the filter selects.
    pub description: String,
    /// The event ids in this timeline that match the filter.
    pub matched_event_ids: Vec<String>,
}

/// One export form the timeline offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportForm {
    /// The export format.
    pub format: ExportFormatClass,
    /// One reviewable label.
    pub label: String,
    /// The opaque artifact ref produced by this export.
    pub artifact_ref: String,
    /// The redaction rule applied on export.
    pub redaction: AdminRedactionClass,
    /// One reviewable sentence describing the export.
    pub description: String,
}

/// The coverage posture of a timeline: how complete it is, and whether it stays
/// locally inspectable without a vendor console.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoveragePosture {
    /// The coverage state (must be one the matrix admits for this surface).
    pub coverage_state: AdminStateClass,
    /// How complete the window is.
    pub completeness: CompletenessClass,
    /// One reviewable label for the coverage window.
    pub window_label: String,
    /// One reviewable sentence stating the coverage rule and any labeled gap.
    pub coverage_note: String,
    /// Whether the history is locally inspectable on this profile.
    pub locally_inspectable: bool,
    /// Whether the history is available without a vendor console / control plane.
    pub vendor_console_independent: bool,
}

// ---------------------------------------------------------------------------
// Timeline, per-profile packet, and the bundle.
// ---------------------------------------------------------------------------

/// The rendered decision-history timeline / audit-event explorer for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionHistoryTimeline {
    /// The surface family (always
    /// [`AdminSurfaceClass::DecisionHistoryTimeline`]).
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id from the matrix.
    pub surface_id: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The ordered decision events.
    pub events: Vec<DecisionEvent>,
    /// The audit-event explorer filters (one per family).
    pub filters: Vec<ExplorerFilter>,
    /// The export forms offered.
    pub export_forms: Vec<ExportForm>,
    /// The coverage posture of the timeline.
    pub coverage: CoveragePosture,
}

impl DecisionHistoryTimeline {
    /// Resolves the filter for a family, if present.
    pub fn filter(&self, family: EventFamilyClass) -> Option<&ExplorerFilter> {
        self.filters.iter().find(|f| f.family == family)
    }

    /// The distinct actor classes present in the timeline.
    pub fn actor_classes(&self) -> std::collections::BTreeSet<ActorClass> {
        self.events.iter().map(|e| e.actor_class).collect()
    }

    /// Whether the timeline offers a given export format.
    pub fn offers(&self, format: ExportFormatClass) -> bool {
        self.export_forms.iter().any(|f| f.format == format)
    }
}

/// The rendered decision-history surface for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionHistoryPacket {
    /// The admin path / profile this packet renders.
    pub profile: AdminPathClass,
    /// Stable, namespaced profile id from the matrix.
    pub profile_id: String,
    /// The deployment profile this maps to.
    pub deployment_profile: AdminDeploymentProfileClass,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The consumers that render this packet (identical bytes for each).
    pub consumers: Vec<AdminConsumerClass>,
    /// The decision-history timeline.
    pub timeline: DecisionHistoryTimeline,
}

impl DecisionHistoryPacket {
    /// Resolves an event by id within this packet.
    pub fn event(&self, event_id: &str) -> Option<&DecisionEvent> {
        self.timeline.events.iter().find(|e| e.event_id == event_id)
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionHistoryInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the rendered bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen decision-history bundle: one packet per claimed managed-bearing
/// profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionHistoryBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_decision_history_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The matrix this render layer binds back to.
    pub matrix_ref: String,
    /// The matrix id this render layer binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps this bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The per-profile decision-history packets.
    pub profiles: Vec<DecisionHistoryPacket>,
    /// The computed invariants.
    pub invariants: Vec<DecisionHistoryInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionHistoryValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for DecisionHistoryValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "decision-history bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for DecisionHistoryValidationError {}

/// The profiles the decision-history bundle covers, in bundle order.
pub const HISTORY_PROFILES: [AdminPathClass; 4] = [
    AdminPathClass::ManagedCloud,
    AdminPathClass::SelfHosted,
    AdminPathClass::SovereignAirGapped,
    AdminPathClass::MirroredOffline,
];

impl DecisionHistoryBundle {
    /// Returns the packet for a profile, if present.
    pub fn packet(&self, profile: AdminPathClass) -> Option<&DecisionHistoryPacket> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.file_refs().into_iter().all(is_export_safe_ref)
            && self.token_ids().into_iter().all(is_safe_token)
    }

    /// The repo-relative file refs carried by the bundle, for export-safety
    /// auditing. Stable token ids are audited separately by [`is_safe_token`].
    fn file_refs(&self) -> [&str; 3] {
        [
            self.schema_ref.as_str(),
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
        ]
    }

    /// Every stable token id carried by the bundle, for export-safety auditing.
    fn token_ids(&self) -> Vec<&str> {
        let mut ids = Vec::new();
        for p in &self.profiles {
            ids.push(p.profile_id.as_str());
            ids.push(p.timeline.surface_id.as_str());
            for e in &p.timeline.events {
                ids.push(e.event_id.as_str());
                ids.push(e.affected_target.as_str());
                ids.push(e.policy_epoch.as_str());
                if let Some(reference) = &e.explanation_ref {
                    ids.push(reference.as_str());
                }
            }
            for f in &p.timeline.filters {
                ids.push(f.filter_id.as_str());
            }
            for x in &p.timeline.export_forms {
                ids.push(x.artifact_ref.as_str());
            }
        }
        ids
    }

    /// Re-checks structural consistency and returns an error on the first
    /// failure. Complements the computed [`DecisionHistoryInvariant`]s with the
    /// coverage and resolution checks a consumer relies on.
    pub fn validate(&self) -> Result<(), DecisionHistoryValidationError> {
        let fail = |reason: String| Err(DecisionHistoryValidationError { reason });

        if self.record_kind != M5_DECISION_HISTORY_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_DECISION_HISTORY_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.matrix_id != M5_ADMIN_PLANE_MATRIX_ID {
            return fail(format!("unexpected matrix_id {}", self.matrix_id));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        for profile in HISTORY_PROFILES {
            if self
                .profiles
                .iter()
                .filter(|p| p.profile == profile)
                .count()
                != 1
            {
                return fail(format!(
                    "profile {} not present exactly once",
                    profile.as_str()
                ));
            }
        }
        if !all_unique(self.profiles.iter().map(|p| p.profile_id.as_str())) {
            return fail("profile ids are not unique".to_owned());
        }

        for packet in &self.profiles {
            validate_packet(packet).map_err(|reason| DecisionHistoryValidationError { reason })?;
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

/// Whether a stable token id is safe to export: non-empty and carries no URL
/// scheme or absolute path.
fn is_safe_token(token: &str) -> bool {
    !token.is_empty() && !token.starts_with('/') && !token.contains("://")
}

/// Whether a state asserts a currently-confirmed decision, so stale evidence
/// under it would be a silent-green lie. Only the active/enforced state is a
/// confirmed headline on this surface; the other admitted states are explicit
/// non-confirmations.
fn requires_fresh_evidence(state: AdminStateClass) -> bool {
    matches!(state, AdminStateClass::ActiveEnforced)
}

/// Per-packet structural floor checks, shared by
/// [`DecisionHistoryBundle::validate`].
fn validate_packet(packet: &DecisionHistoryPacket) -> Result<(), String> {
    if packet.profile_id != packet.profile.path_id() {
        return Err(format!(
            "profile id mismatch for {}",
            packet.profile.as_str()
        ));
    }
    let timeline = &packet.timeline;
    if timeline.surface != AdminSurfaceClass::DecisionHistoryTimeline {
        return Err(format!(
            "{}: timeline is not the decision-history surface",
            packet.profile.as_str()
        ));
    }
    if timeline.events.is_empty() {
        return Err(format!("{}: no decision events", packet.profile.as_str()));
    }
    // The explorer offers every family exactly once.
    for family in EventFamilyClass::ALL {
        if timeline
            .filters
            .iter()
            .filter(|f| f.family == family)
            .count()
            != 1
        {
            return Err(format!(
                "{}: filter family {} not offered exactly once",
                packet.profile.as_str(),
                family.as_str()
            ));
        }
    }
    // Every event resolves to its family's filter and is listed there.
    for event in &timeline.events {
        let Some(filter) = timeline.filter(event.event_family) else {
            return Err(format!(
                "{}: event {} has no filter for family {}",
                packet.profile.as_str(),
                event.event_id,
                event.event_family.as_str()
            ));
        };
        if !filter.matched_event_ids.contains(&event.event_id) {
            return Err(format!(
                "{}: event {} not listed under its family filter",
                packet.profile.as_str(),
                event.event_id
            ));
        }
        if !event.has_export_parity() {
            return Err(format!(
                "{}: event {} lacks both export representations",
                packet.profile.as_str(),
                event.event_id
            ));
        }
    }
    // Both export forms are offered.
    if !timeline.offers(ExportFormatClass::MachineReadableJson)
        || !timeline.offers(ExportFormatClass::PlainLanguageHandoff)
    {
        return Err(format!(
            "{}: timeline does not offer both export forms",
            packet.profile.as_str()
        ));
    }
    // The history is locally inspectable without a vendor console.
    if !timeline.coverage.locally_inspectable || !timeline.coverage.vendor_console_independent {
        return Err(format!(
            "{}: history is not locally inspectable without a vendor console",
            packet.profile.as_str()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical decision-history bundle.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the rendered packets, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn decision_history_bundle() -> DecisionHistoryBundle {
    let profiles: Vec<DecisionHistoryPacket> = HISTORY_PROFILES
        .iter()
        .map(|p| history_packet(*p))
        .collect();
    let invariants = compute_invariants(&profiles);

    DecisionHistoryBundle {
        record_kind: M5_DECISION_HISTORY_RECORD_KIND.to_owned(),
        m5_decision_history_schema_version: M5_DECISION_HISTORY_SCHEMA_VERSION,
        schema_ref: M5_DECISION_HISTORY_SCHEMA_REF.to_owned(),
        bundle_id: M5_DECISION_HISTORY_BUNDLE_ID.to_owned(),
        as_of: M5_DECISION_HISTORY_AS_OF.to_owned(),
        matrix_ref: M5_DECISION_HISTORY_MATRIX_REF.to_owned(),
        matrix_id: M5_ADMIN_PLANE_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_DECISION_HISTORY_FREEZE_GATE_REF.to_owned(),
        summary: "Rendered decision-history timelines and audit-event explorers — material \
                  allow/deny/quota/force-disable/publish-scope decisions with stable decision \
                  codes, distinguished actor classes, policy epochs, affected scope, time, \
                  explanation links, and dual machine/plain-language export — bound back to the \
                  frozen admin-plane matrix and rendered identically for shell, CLI/headless, \
                  support export, procurement, and managed-service consumers across the \
                  managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles, \
                  each kept locally inspectable without a vendor console."
            .to_owned(),
        profiles,
        invariants,
        raw_payload_excluded: true,
    }
}

/// The consumers every packet must serve identically; mirrors the matrix's
/// declared consumers for the decision-history surface.
fn parity_consumers() -> Vec<AdminConsumerClass> {
    admin_plane_matrix()
        .surface(AdminSurfaceClass::DecisionHistoryTimeline)
        .map(|entry| entry.consumed_by.clone())
        .unwrap_or_default()
}

fn history_packet(profile: AdminPathClass) -> DecisionHistoryPacket {
    let (deployment_profile, summary) = match profile {
        AdminPathClass::ManagedCloud => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Managed-cloud profile: a live decision history of policy, routing, and remote-mutation \
             decisions, each confirmed against the managed control plane.",
        ),
        AdminPathClass::SelfHosted => (
            AdminDeploymentProfileClass::SelfHosted,
            "Self-hosted profile: the customer's own control plane records the full local decision \
             history; an unreachable external provider is shown as a provider limitation, not a \
             generic error.",
        ),
        AdminPathClass::SovereignAirGapped => (
            AdminDeploymentProfileClass::SovereignAirGapped,
            "Sovereign / air-gapped profile: decisions resolve from the signed offline bundle and \
             imported snapshots; the history is locally inspectable with no live control-plane tail.",
        ),
        AdminPathClass::MirroredOffline => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Mirrored / offline profile: the managed source is offline, so the history is the \
             last-synced tail labeled as such; queued requests are recorded to act on when the \
             mirror reconnects.",
        ),
        _ => (
            AdminDeploymentProfileClass::IndividualLocal,
            "Local profile.",
        ),
    };

    DecisionHistoryPacket {
        profile,
        profile_id: profile.path_id(),
        deployment_profile,
        summary: summary.to_owned(),
        consumers: parity_consumers(),
        timeline: render_timeline(profile),
    }
}

fn render_timeline(profile: AdminPathClass) -> DecisionHistoryTimeline {
    let surface = AdminSurfaceClass::DecisionHistoryTimeline;
    let events = build_events(profile);
    let filters = build_filters(&events);
    let export_forms = build_export_forms(profile);
    let coverage = build_coverage(profile);

    let summary = match profile {
        AdminPathClass::ManagedCloud => {
            "Each decision names its code, actor class, policy epoch, affected scope, and time, and \
             links to its explanation; every row exports as a machine summary and a plain-language \
             handoff."
        }
        AdminPathClass::SelfHosted => {
            "A complete local decision history from the self-hosted control plane; an external \
             provider's refusal is attributed as a provider limitation rather than a blocked event."
        }
        AdminPathClass::SovereignAirGapped => {
            "Decisions from the signed offline bundle and imported snapshots; the live tail is \
             absent and labeled, never implied complete."
        }
        AdminPathClass::MirroredOffline => {
            "The last-synced decision tail labeled as last known while the mirror is offline; \
             queued requests are recorded, not lost."
        }
        _ => "Decision history.",
    };

    DecisionHistoryTimeline {
        surface,
        surface_id: surface.surface_id(),
        summary: summary.to_owned(),
        events,
        filters,
        export_forms,
        coverage,
    }
}

/// One concise builder for a decision event, to keep the per-profile data dense
/// and reviewable.
#[allow(clippy::too_many_arguments)]
fn event(
    event_id: &str,
    decision_code: DecisionCodeClass,
    event_family: EventFamilyClass,
    actor_class: ActorClass,
    actor_label: &str,
    affected_target: &str,
    scope_kind: ScopeKindClass,
    scope_label: &str,
    policy_epoch: &str,
    entitlement_epoch: Option<&str>,
    event_at: &str,
    sequence: u32,
    outcome_state: AdminStateClass,
    evidence_age: EvidenceAgeClass,
    data_residency: DataResidencyClass,
    owner: OwnerEscalationRoleClass,
    explanation_ref: Option<&str>,
    machine_summary: &str,
    plain_language: &str,
) -> DecisionEvent {
    DecisionEvent {
        event_id: event_id.to_owned(),
        decision_code,
        event_family,
        actor_class,
        actor_label: actor_label.to_owned(),
        affected_target: affected_target.to_owned(),
        scope_kind,
        scope_label: scope_label.to_owned(),
        policy_epoch: policy_epoch.to_owned(),
        entitlement_epoch: entitlement_epoch.map(str::to_owned),
        event_at: event_at.to_owned(),
        sequence,
        outcome_state,
        evidence_age,
        data_residency,
        owner,
        explanation_ref: explanation_ref.map(str::to_owned),
        machine_summary: machine_summary.to_owned(),
        plain_language: plain_language.to_owned(),
    }
}

fn build_events(profile: AdminPathClass) -> Vec<DecisionEvent> {
    use ActorClass::*;
    use AdminStateClass::*;
    use DataResidencyClass::*;
    use DecisionCodeClass::*;
    use EventFamilyClass::*;
    use EvidenceAgeClass::*;
    use OwnerEscalationRoleClass::*;
    use ScopeKindClass::*;

    match profile {
        AdminPathClass::ManagedCloud => vec![
            event(
                "decision_history.event.managed_cloud.0001",
                ForceDisable,
                PolicyChange,
                AdminAction,
                "Organization admin",
                "ai.provider.allowed",
                TenantOrOrg,
                "Managed organization",
                "policy_epoch.managed.rev42",
                None,
                "2026-06-20T09:00:00Z",
                1,
                ActiveEnforced,
                Fresh,
                ManagedCopy,
                OrgAdmin,
                Some("admin_render.lock.managed_cloud.ai_provider"),
                "decision=force_disable family=policy_change actor=admin_action \
                 target=ai.provider.allowed epoch=policy_epoch.managed.rev42",
                "An organization admin locked AI providers to the approved managed list under \
                 policy epoch rev 42; local overrides no longer apply.",
            ),
            event(
                "decision_history.event.managed_cloud.0002",
                Narrow,
                ProviderRouting,
                PolicyEvaluation,
                "Policy engine",
                "ai.route.default",
                CapabilityScope,
                "AI routing",
                "policy_epoch.managed.rev42",
                None,
                "2026-06-20T09:05:00Z",
                2,
                ActiveEnforced,
                Fresh,
                ManagedCopy,
                OrgAdmin,
                None,
                "decision=narrow family=provider_routing actor=policy_evaluation \
                 target=ai.route.default epoch=policy_epoch.managed.rev42",
                "Policy narrowed default AI routing to the approved managed region; other routes \
                 were excluded for this request.",
            ),
            event(
                "decision_history.event.managed_cloud.0003",
                MutationRecorded,
                RemoteMutation,
                UserAction,
                "Local user",
                "remote.session.fs.write",
                SessionOrCommand,
                "Remote session",
                "policy_epoch.managed.rev42",
                None,
                "2026-06-20T09:10:00Z",
                3,
                ActiveEnforced,
                Fresh,
                ManagedCopy,
                LocalUser,
                None,
                "decision=mutation_recorded family=remote_mutation actor=user_action \
                 target=remote.session.fs.write epoch=policy_epoch.managed.rev42",
                "A user write to the remote session filesystem was recorded with a mutation receipt.",
            ),
            event(
                "decision_history.event.managed_cloud.0004",
                Allow,
                AuthSession,
                PolicyEvaluation,
                "Policy engine",
                "auth.session.start",
                SessionOrCommand,
                "Managed session",
                "policy_epoch.managed.rev42",
                Some("entitlement_epoch.managed.seat7"),
                "2026-06-20T08:55:00Z",
                4,
                ActiveEnforced,
                Fresh,
                ManagedCopy,
                OrgAdmin,
                None,
                "decision=allow family=auth_session actor=policy_evaluation \
                 target=auth.session.start epoch=policy_epoch.managed.rev42",
                "A managed session was allowed to start under the current entitlement; the seat is \
                 in good standing.",
            ),
        ],
        AdminPathClass::SelfHosted => vec![
            event(
                "decision_history.event.self_hosted.0001",
                ForceDisable,
                PolicyChange,
                AdminAction,
                "Security owner",
                "network.egress",
                TenantOrOrg,
                "Self-hosted organization",
                "policy_epoch.self_hosted.rev7",
                None,
                "2026-06-19T14:00:00Z",
                1,
                ActiveEnforced,
                Fresh,
                ManagedCopy,
                SecurityOwner,
                Some("admin_render.lock.self_hosted.network_egress"),
                "decision=force_disable family=policy_change actor=admin_action \
                 target=network.egress epoch=policy_epoch.self_hosted.rev7",
                "The security owner restricted network egress to self-hosted endpoints under policy \
                 epoch rev 7; other destinations are blocked.",
            ),
            event(
                "decision_history.event.self_hosted.0002",
                Allow,
                TrustChange,
                AdminAction,
                "Security owner",
                "trust.root.customer",
                DeploymentProfile,
                "Self-hosted deployment",
                "policy_epoch.self_hosted.rev7",
                None,
                "2026-06-19T14:02:00Z",
                2,
                ActiveEnforced,
                Recent,
                ManagedCopy,
                SecurityOwner,
                None,
                "decision=allow family=trust_change actor=admin_action \
                 target=trust.root.customer epoch=policy_epoch.self_hosted.rev7",
                "A new customer trust root was activated for the self-hosted deployment; signed \
                 bundles now verify against it.",
            ),
            event(
                "decision_history.event.self_hosted.0003",
                Deny,
                ProviderRouting,
                ProviderLimitation,
                "External AI provider",
                "ai.provider.external",
                CapabilityScope,
                "AI routing",
                "policy_epoch.self_hosted.rev7",
                None,
                "2026-06-19T14:30:00Z",
                3,
                UnconfirmedStale,
                Stale,
                LocalOnly,
                SecurityOwner,
                None,
                "decision=deny family=provider_routing actor=provider_limitation \
                 target=ai.provider.external epoch=policy_epoch.self_hosted.rev7",
                "An external AI provider was unreachable, so the routing decision is shown as a \
                 provider limitation and left unconfirmed rather than recorded as a policy denial.",
            ),
        ],
        AdminPathClass::SovereignAirGapped => vec![
            event(
                "decision_history.event.sovereign.0001",
                ForceDisable,
                PolicyChange,
                AdminAction,
                "Security owner",
                "ai.provider.allowed",
                TenantOrOrg,
                "Sovereign deployment",
                "policy_epoch.offline.seal_a1",
                None,
                "2026-06-10T10:00:00Z",
                1,
                ActiveEnforced,
                Recent,
                LocalOnly,
                SecurityOwner,
                Some("admin_render.lock.sovereign.ai_provider"),
                "decision=force_disable family=policy_change actor=admin_action \
                 target=ai.provider.allowed epoch=policy_epoch.offline.seal_a1",
                "The signed offline bundle (seal 0xA1) locked AI to on-device offline models; the \
                 decision verified against the pinned offline root.",
            ),
            event(
                "decision_history.event.sovereign.0002",
                LocalOnlyContinue,
                ManagedIdentityScope,
                PolicyEvaluation,
                "Policy engine",
                "identity.managed.scope",
                TenantOrOrg,
                "Sovereign deployment",
                "policy_epoch.offline.seal_a1",
                Some("entitlement_epoch.offline.seal_a1"),
                "2026-06-10T10:05:00Z",
                2,
                ImportedSnapshotNoLive,
                Stale,
                LocalOnly,
                ComplianceOwner,
                None,
                "decision=local_only_continue family=managed_identity_scope actor=policy_evaluation \
                 target=identity.managed.scope epoch=policy_epoch.offline.seal_a1",
                "Managed identity scope is read from an imported snapshot with no live control \
                 plane; the session continues local-only and the row is labeled imported.",
            ),
            event(
                "decision_history.event.sovereign.0003",
                ForceDisable,
                CollaborationControl,
                ClientLimitation,
                "Local client",
                "collab.share.external",
                CapabilityScope,
                "Collaboration",
                "policy_epoch.offline.seal_a1",
                None,
                "2026-06-10T10:08:00Z",
                3,
                ActiveEnforced,
                Fresh,
                LocalOnly,
                SecurityOwner,
                Some("decision_history.explain.sovereign.collab_unavailable"),
                "decision=force_disable family=collaboration_control actor=client_limitation \
                 target=collab.share.external epoch=policy_epoch.offline.seal_a1",
                "External collaboration is unavailable on this air-gapped install; the client \
                 cannot reach a sharing service, so the row is a client limitation, not a denial.",
            ),
        ],
        AdminPathClass::MirroredOffline => vec![
            event(
                "decision_history.event.mirrored.0001",
                ForceDisable,
                PolicyChange,
                AdminAction,
                "Organization admin",
                "ai.provider.allowed",
                TenantOrOrg,
                "Managed organization (mirrored)",
                "policy_epoch.mirror.rev42",
                None,
                "2026-06-18T11:00:00Z",
                1,
                MirrorOfflineLastKnown,
                Stale,
                MirroredCopy,
                OrgAdmin,
                Some("admin_render.lock.mirrored.ai_provider"),
                "decision=force_disable family=policy_change actor=admin_action \
                 target=ai.provider.allowed epoch=policy_epoch.mirror.rev42",
                "The last-synced mirror locks AI providers to the approved list; the decision is \
                 shown as last known while the mirror is offline.",
            ),
            event(
                "decision_history.event.mirrored.0002",
                RequestRecorded,
                PublishState,
                UserAction,
                "Local user",
                "marketplace.publish",
                Workspace,
                "Workspace publish",
                "policy_epoch.mirror.rev42",
                None,
                "2026-06-18T11:05:00Z",
                2,
                MirrorOfflineLastKnown,
                Recent,
                MirroredCopy,
                WorkspaceOwner,
                None,
                "decision=request_recorded family=publish_state actor=user_action \
                 target=marketplace.publish epoch=policy_epoch.mirror.rev42",
                "A publish request was recorded locally to publish when the mirror reconnects; \
                 nothing was lost.",
            ),
            event(
                "decision_history.event.mirrored.0003",
                DeferPendingRefresh,
                AuthSession,
                ClientLimitation,
                "Local client",
                "auth.session.refresh",
                SessionOrCommand,
                "Managed session",
                "policy_epoch.mirror.rev42",
                Some("entitlement_epoch.mirror.seat7"),
                "2026-06-18T11:10:00Z",
                3,
                UnconfirmedStale,
                Stale,
                MirroredCopy,
                OrgAdmin,
                None,
                "decision=defer_pending_refresh family=auth_session actor=client_limitation \
                 target=auth.session.refresh epoch=policy_epoch.mirror.rev42",
                "Session refresh is deferred while the mirror is offline; the session keeps its \
                 last-known entitlement and reconfirms on reconnect.",
            ),
        ],
        _ => Vec::new(),
    }
}

fn build_filters(events: &[DecisionEvent]) -> Vec<ExplorerFilter> {
    EventFamilyClass::ALL
        .iter()
        .map(|family| {
            let matched_event_ids = events
                .iter()
                .filter(|e| e.event_family == *family)
                .map(|e| e.event_id.clone())
                .collect();
            ExplorerFilter {
                filter_id: format!("decision_history.filter.{}", family.as_str()),
                family: *family,
                label: family.filter_label().to_owned(),
                description: format!(
                    "Show only {} events in this decision history.",
                    family.filter_label().to_lowercase()
                ),
                matched_event_ids,
            }
        })
        .collect()
}

fn build_export_forms(profile: AdminPathClass) -> Vec<ExportForm> {
    let profile_token = profile.as_str();
    vec![
        ExportForm {
            format: ExportFormatClass::MachineReadableJson,
            label: "Machine-readable summary".to_owned(),
            artifact_ref: format!("decision_history.export.{profile_token}.machine"),
            redaction: AdminRedactionClass::MetadataSafeDefault,
            description: "Each row's stable codes, actor class, policy epoch, scope, and time as \
                          JSON summary objects, copyable or exportable for tooling."
                .to_owned(),
        },
        ExportForm {
            format: ExportFormatClass::PlainLanguageHandoff,
            label: "Plain-language handoff packet".to_owned(),
            artifact_ref: format!("decision_history.export.{profile_token}.handoff"),
            redaction: AdminRedactionClass::InternalSupportRestricted,
            description: "The same rows as reviewable plain-language sentences for a support or \
                          admin handoff, with no raw payloads."
                .to_owned(),
        },
    ]
}

fn build_coverage(profile: AdminPathClass) -> CoveragePosture {
    use AdminStateClass::*;
    use CompletenessClass::*;

    match profile {
        AdminPathClass::ManagedCloud => CoveragePosture {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "Last 30 days — live".to_owned(),
            coverage_note: "The managed control plane is live; the window is complete for its \
                            declared range and refreshes continuously."
                .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
        AdminPathClass::SelfHosted => CoveragePosture {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "Last 30 days — self-hosted".to_owned(),
            coverage_note:
                "The customer's own control plane records the full local history; it is \
                            inspectable on this machine without any vendor console."
                    .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
        AdminPathClass::SovereignAirGapped => CoveragePosture {
            coverage_state: ImportedSnapshotNoLive,
            completeness: PartialImported,
            window_label: "Imported snapshot — no live tail".to_owned(),
            coverage_note: "The history is replayed from the last imported snapshot; there is no \
                            live tail and the gap is labeled rather than implied complete."
                .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
        AdminPathClass::MirroredOffline => CoveragePosture {
            coverage_state: MirrorOfflineLastKnown,
            completeness: PartialOffline,
            window_label: "Last synced — mirror offline".to_owned(),
            coverage_note:
                "The mirror is offline, so the tail beyond the last sync is missing and \
                            labeled; the recorded rows remain locally inspectable."
                    .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
        _ => CoveragePosture {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "Local".to_owned(),
            coverage_note: "Local history.".to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> DecisionHistoryInvariant {
    DecisionHistoryInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(profiles: &[DecisionHistoryPacket]) -> Vec<DecisionHistoryInvariant> {
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| -> bool {
        matrix
            .surface(AdminSurfaceClass::DecisionHistoryTimeline)
            .is_some_and(|entry| entry.applicable_states.contains(&state))
    };
    let declared_consumers = parity_consumers();

    let mut out = Vec::new();

    // Every rendered state is one the matrix admits for this surface.
    out.push(invariant(
        "decision_history.surface_states_within_matrix",
        "Every state an event or the coverage posture shows is one the frozen admin-plane matrix \
         declares applicable for the decision-history surface, so the render layer cannot drift \
         from the contract.",
        profiles.iter().all(|p| {
            p.timeline.events.iter().all(|e| admitted(e.outcome_state))
                && admitted(p.timeline.coverage.coverage_state)
        }),
    ));

    // Every event names a stable code, epoch, scope, time, and id; ids unique.
    out.push(invariant(
        "decision_history.decision_truth",
        "Every decision event carries a stable event id, decision code, policy epoch, affected \
         target and scope, and an event time, so each material allow/deny/quota/force-disable/ \
         publish-scope decision is attributable.",
        profiles.iter().all(|p| {
            all_unique(p.timeline.events.iter().map(|e| e.event_id.as_str()))
                && p.timeline.events.iter().all(|e| {
                    !e.event_id.is_empty()
                        && !e.policy_epoch.is_empty()
                        && !e.affected_target.is_empty()
                        && !e.scope_label.is_empty()
                        && !e.event_at.is_empty()
                })
        }),
    ));

    // Actor classes are distinguished, never collapsed into one generic class.
    out.push(invariant(
        "decision_history.actor_classes_distinguished",
        "Every event names a specific actor class — user action, admin action, policy evaluation, \
         provider limitation, or client limitation — and each timeline uses at least two distinct \
         classes, so decisions are not collapsed into a generic blocked/error event.",
        profiles
            .iter()
            .all(|p| p.timeline.actor_classes().len() >= 2),
    ));

    // The whole bundle exercises every actor class — proof the surface keeps the
    // five lanes distinct rather than folding limitations into denials.
    out.push(invariant(
        "decision_history.actor_classes_all_present",
        "Across the bundle every actor class appears at least once, so provider and client \
         limitations are surfaced as themselves rather than as policy or admin denials.",
        ActorClass::ALL.iter().all(|actor| {
            profiles
                .iter()
                .any(|p| p.timeline.events.iter().any(|e| e.actor_class == *actor))
        }),
    ));

    // The explorer offers every family and every event resolves to exactly one.
    out.push(invariant(
        "decision_history.explorer_filters_complete",
        "Every timeline offers a filter for each of the eight audit families (trust, policy, auth, \
         remote mutation, provider routing, collaboration control, publish state, managed identity \
         scope), and every event resolves to exactly one of them and is listed under it.",
        profiles.iter().all(|p| {
            EventFamilyClass::ALL
                .iter()
                .all(|family| p.timeline.filter(*family).is_some())
                && p.timeline.events.iter().all(|e| {
                    p.timeline
                        .filter(e.event_family)
                        .is_some_and(|f| f.matched_event_ids.contains(&e.event_id))
                })
        }),
    ));

    // Export parity: machine summary and plain-language handoff on every row and
    // both export forms offered.
    out.push(invariant(
        "decision_history.export_parity",
        "Every row carries both an export-safe machine-readable summary and a plain-language \
         support/admin handoff sentence, and every timeline offers both export forms.",
        profiles.iter().all(|p| {
            p.timeline
                .events
                .iter()
                .all(DecisionEvent::has_export_parity)
                && p.timeline.offers(ExportFormatClass::MachineReadableJson)
                && p.timeline.offers(ExportFormatClass::PlainLanguageHandoff)
        }),
    ));

    // No-silent-green: stale/offline evidence never sits under a confirmed state.
    out.push(invariant(
        "decision_history.no_silent_green",
        "An event whose backing evidence is stale is never shown under a confirmed \
         active/enforced state; offline and imported rows use an explicit non-confirmed state \
         instead.",
        profiles.iter().all(|p| {
            p.timeline
                .events
                .iter()
                .all(|e| !(e.evidence_age.is_stale() && requires_fresh_evidence(e.outcome_state)))
        }),
    ));

    // Locally inspectable without a vendor console on every profile.
    out.push(invariant(
        "decision_history.locally_inspectable_offline",
        "Every profile — including self-hosted, sovereign/air-gapped, and mirrored/offline — keeps \
         a locally inspectable audit history that does not require a vendor console or control \
         plane.",
        profiles.iter().all(|p| {
            p.timeline.coverage.locally_inspectable
                && p.timeline.coverage.vendor_console_independent
        }),
    ));

    // Partial windows are labeled, never implied complete.
    out.push(invariant(
        "decision_history.coverage_labeled",
        "A history window that is offline, imported, or redaction-limited is labeled with a \
         non-complete completeness class and a coverage note, so a partial history is never \
         presented as complete.",
        profiles.iter().all(|p| {
            let coverage = &p.timeline.coverage;
            !coverage.coverage_note.is_empty()
                && (!coverage.completeness.is_partial()
                    || coverage.coverage_state != AdminStateClass::ActiveEnforced)
        }),
    ));

    // Ownership stays visible: every event names an owner; locked decisions link
    // to an explanation.
    out.push(invariant(
        "decision_history.ownership_visible",
        "Every event names an owner, and every force-disable decision links to an explanation, so \
         the next step is always attributable.",
        profiles.iter().all(|p| {
            p.timeline.events.iter().all(|e| {
                e.decision_code != DecisionCodeClass::ForceDisable || e.explanation_ref.is_some()
            })
        }),
    ));

    // Cross-surface parity: one typed packet serves every declared consumer.
    out.push(invariant(
        "decision_history.consumer_parity",
        "Each profile is one typed packet consumed identically by every consumer the matrix \
         declares for the decision-history surface, so the timeline is identical across UI, CLI, \
         support export, procurement, and managed-service surfaces by construction.",
        !declared_consumers.is_empty()
            && profiles
                .iter()
                .all(|p| declared_consumers.iter().all(|c| p.consumers.contains(c))),
    ));

    // Every claimed managed-bearing profile is rendered.
    out.push(invariant(
        "decision_history.profiles_covered",
        "The bundle renders the managed-cloud, self-hosted, sovereign/air-gapped, and \
         mirrored/offline profiles.",
        HISTORY_PROFILES
            .iter()
            .all(|profile| profiles.iter().any(|p| p.profile == *profile)),
    ));

    // Export safety, surfaced as a computed invariant for release automation.
    out.push(invariant(
        "decision_history.export_safe",
        "Every stable surface, profile, event, target, epoch, filter, and export id is an opaque \
         token with no URL scheme or absolute path, so the bundle is safe to embed in a support \
         export verbatim.",
        profiles.iter().all(|p| {
            is_safe_token(p.profile_id.as_str())
                && is_safe_token(p.timeline.surface_id.as_str())
                && p.timeline.events.iter().all(|e| {
                    is_safe_token(e.event_id.as_str())
                        && is_safe_token(e.affected_target.as_str())
                        && is_safe_token(e.policy_epoch.as_str())
                        && e.explanation_ref
                            .as_deref()
                            .map(is_safe_token)
                            .unwrap_or(true)
                })
                && p.timeline
                    .filters
                    .iter()
                    .all(|f| is_safe_token(f.filter_id.as_str()))
                && p.timeline
                    .export_forms
                    .iter()
                    .all(|x| is_safe_token(x.artifact_ref.as_str()))
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn decision_history_lines(bundle: &DecisionHistoryBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Decision-history bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Profiles: {}  Invariants: {}  (binds matrix {})",
        bundle.profiles.len(),
        bundle.invariants.len(),
        bundle.matrix_id,
    ));

    for p in &bundle.profiles {
        lines.push(format!("Profile {} [{}]", p.profile.as_str(), p.profile_id));
        lines.push(format!("  {}", p.summary));
        let coverage = &p.timeline.coverage;
        lines.push(format!(
            "  Coverage: state={} completeness={} window={} local={} console_independent={}",
            coverage.coverage_state.as_str(),
            coverage.completeness.as_str(),
            coverage.window_label,
            coverage.locally_inspectable,
            coverage.vendor_console_independent,
        ));
        lines.push("  Decisions:".to_owned());
        for e in &p.timeline.events {
            lines.push(format!(
                "    - {} [{}] decision={} actor={} target={} epoch={} state={} age={}",
                e.event_id,
                e.event_family.as_str(),
                e.decision_code.as_str(),
                e.actor_class.as_str(),
                e.affected_target,
                e.policy_epoch,
                e.outcome_state.as_str(),
                e.evidence_age.as_str(),
            ));
            lines.push(format!("        {}", e.plain_language));
            if let Some(reference) = &e.explanation_ref {
                lines.push(format!("        explained → {reference}"));
            }
        }
        lines.push("  Filters:".to_owned());
        for f in &p.timeline.filters {
            lines.push(format!(
                "    - {} ({}) matches={}",
                f.label,
                f.family.as_str(),
                f.matched_event_ids.len(),
            ));
        }
        lines.push("  Export forms:".to_owned());
        for x in &p.timeline.export_forms {
            lines.push(format!("    - {} [{}]", x.label, x.format.as_str()));
        }
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
