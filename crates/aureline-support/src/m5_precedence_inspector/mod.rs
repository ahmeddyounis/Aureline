//! Precedence inspectors that show the winning value, the overshadowed candidates, the affected
//! surfaces, and the source-of-truth lineage for every major M5 resolver family.
//!
//! Where the resolver families own *what value won* — the toolchain / execution-context resolver,
//! the effective-setting resolver, the policy resolver, the credential / auth resolver, and the
//! route / target resolver — this packet governs *how that win is explained so a silent fallback or
//! a hidden override is impossible to miss*. It is a registry of precedence inspectors, one per
//! resolver decision worth explaining, each carrying the winning value, the candidates it
//! overshadowed (and why), the source class, the policy-lock state, the affected surfaces, the
//! restart-or-reauth posture, and the source-of-truth lineage. It reuses the resolver truth by
//! reference — every candidate carries a `descriptor_ref` and every inspector a `source_of_truth_ref`
//! that projects from the existing effective-setting, launch-inspector, auth/credential, and
//! route-origin objects — rather than re-deriving any resolution of its own.
//!
//! The readiness analogue here is a fail-closed **precedence gate**. The guardrail the source set
//! treats as core supportability UX is that an inspector must never present a clean "this value won"
//! chip that hides what lost, why it lost, or that the win is a silent fallback, a hidden override, a
//! drift, a conflict, or a policy block. Each inspector therefore publishes an
//! [`InspectorPresentation`] that is the weaker of two ceilings: its [`ResolutionClass`] ceiling
//! (a clean resolution presents transparently; a fallback, override, drift, or conflict narrows it;
//! a policy-lock block caps it at blocked) and its [`ValueDisclosure`] ceiling (plain values present
//! transparently; secret- or identity-bearing values are narrowed to class / health / provenance,
//! never the raw value). An inspector can never claim a cleaner presentation than its inputs support:
//! a lower-precedence source that won without a fallback explanation, an override that hides the
//! suppressed value, or a credential shown raw all narrow or fail the gate automatically. The
//! recorded presentation, downgrade reasons, resolution path, and posture are all recomputed and
//! validated against the gate, so a clean win can never be asserted by hand over a degraded or
//! redacted resolution.
//!
//! Every inspector always carries its one-step `explain_entrypoint_ref` — the inspectable "Why did
//! this value win?" answer — and its `cli_object_ref`, the CLI / headless equivalent, so the same
//! precedence answer is reachable from the active surface, the Support Center, the CLI / headless
//! inspect path, and the support packets. Every required consumer surface binds to this one registry
//! via an [`InspectorConsumerBinding`] that must ingest it, preserve its precedence vocabulary and
//! object ids, and narrow with it, so route, credential, setting, policy, and toolchain precedence
//! share one supportability grammar across desktop, Support Center, CLI, and support exports.
//!
//! The packet is checked in at `artifacts/support/m5/m5-precedence-inspector.json` and embedded here.
//! It is metadata-only: every field is a typed state, a count, or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, or hidden policy payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported precedence-inspector schema version.
pub const M5_PRECEDENCE_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_PRECEDENCE_INSPECTOR_RECORD_KIND: &str = "m5_precedence_inspectors";

/// Repo-relative path to the checked-in packet.
pub const M5_PRECEDENCE_INSPECTOR_PATH: &str = "artifacts/support/m5/m5-precedence-inspector.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_PRECEDENCE_INSPECTOR_SCHEMA_REF: &str =
    "schemas/support/m5-precedence-inspector.schema.json";

/// Repo-relative path to the companion document.
pub const M5_PRECEDENCE_INSPECTOR_DOC_REF: &str = "docs/help/support/m5-precedence-inspection.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_PRECEDENCE_INSPECTOR_ARTIFACT_DOC_REF: &str =
    "artifacts/support/m5/m5-precedence-inspector.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_PRECEDENCE_INSPECTOR_FIXTURE_DIR: &str = "fixtures/support/m5/m5-precedence-inspector";

/// Repo-relative path to the shiproom review packet that renders this registry.
pub const M5_PRECEDENCE_INSPECTOR_REVIEW_PACKET_REF: &str =
    "artifacts/shiproom/m5-precedence-inspector-review-packet/precedence_inspector_review_packet.md";

/// Embedded checked-in packet JSON.
pub const M5_PRECEDENCE_INSPECTOR_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/m5/m5-precedence-inspector.json"
));

/// A major M5 resolver family whose precedence this registry explains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrecedenceFamily {
    /// Toolchain / execution-context resolution (interpreter, SDK, container).
    Toolchain,
    /// Effective-setting resolution across scopes.
    Setting,
    /// Policy resolution and policy-lock enforcement.
    Policy,
    /// Credential / auth resolution.
    Credential,
    /// Route / target resolution.
    Route,
}

impl PrecedenceFamily {
    /// Every resolver family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Toolchain,
        Self::Setting,
        Self::Policy,
        Self::Credential,
        Self::Route,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Toolchain => "toolchain",
            Self::Setting => "setting",
            Self::Policy => "policy",
            Self::Credential => "credential",
            Self::Route => "route",
        }
    }

    /// Whether this family resolves secret- or identity-bearing material that must never be dumped raw.
    pub const fn is_identity_bearing(self) -> bool {
        matches!(self, Self::Credential)
    }
}

/// The precedence scope a candidate value comes from, highest-precedence first.
///
/// This is the one unified precedence vocabulary every family projects into, so a policy lock, a
/// workspace override, a user default, a system-detected value, and a last-resort fallback rank the
/// same way whether the resolver is a setting, a toolchain, a credential, or a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrecedenceSource {
    /// An administrator- / policy-managed source; the strongest precedence.
    PolicyScoped,
    /// A workspace- / project-scoped source.
    ProjectScoped,
    /// A user- / personal-scoped source.
    UserScoped,
    /// A system- / host-detected source.
    SystemScoped,
    /// A fallback / last-resort source; the weakest precedence.
    FallbackScoped,
}

impl PrecedenceSource {
    /// Every precedence source, highest precedence first.
    pub const ALL: [Self; 5] = [
        Self::PolicyScoped,
        Self::ProjectScoped,
        Self::UserScoped,
        Self::SystemScoped,
        Self::FallbackScoped,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyScoped => "policy_scoped",
            Self::ProjectScoped => "project_scoped",
            Self::UserScoped => "user_scoped",
            Self::SystemScoped => "system_scoped",
            Self::FallbackScoped => "fallback_scoped",
        }
    }

    /// Precedence rank; higher outranks. Used to prove a winner genuinely out-precedes its candidates.
    pub const fn rank(self) -> u8 {
        match self {
            Self::PolicyScoped => 4,
            Self::ProjectScoped => 3,
            Self::UserScoped => 2,
            Self::SystemScoped => 1,
            Self::FallbackScoped => 0,
        }
    }
}

/// How a candidate's value is disclosed, after redaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueDisclosure {
    /// The values are non-sensitive and shown in full.
    PlainValues,
    /// The values are secret- or identity-bearing and shown by class / health / provenance only.
    MetadataOnly,
}

impl ValueDisclosure {
    /// Every disclosure level, in declaration order.
    pub const ALL: [Self; 2] = [Self::PlainValues, Self::MetadataOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainValues => "plain_values",
            Self::MetadataOnly => "metadata_only",
        }
    }

    /// Highest presentation this disclosure permits.
    pub const fn presentation_ceiling(self) -> InspectorPresentation {
        match self {
            Self::PlainValues => InspectorPresentation::Transparent,
            Self::MetadataOnly => InspectorPresentation::Narrowed,
        }
    }

    /// Whether the values are narrowed to metadata for redaction safety.
    pub const fn is_metadata_only(self) -> bool {
        matches!(self, Self::MetadataOnly)
    }
}

/// The overall outcome of a resolver decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionClass {
    /// A clean win: the highest-precedence available source won.
    Resolved,
    /// A fallback won because the preferred, higher-precedence source was unavailable.
    Fallback,
    /// A higher-precedence source overrode a lower one (workspace-over-user, policy-over-user).
    Override,
    /// The resolved source drifted from the recorded / configured one.
    Drift,
    /// Two sources tie at the same precedence and must be reconciled.
    Conflict,
    /// A policy lock blocks the resolution; the lower-precedence value cannot take effect.
    Blocked,
}

impl ResolutionClass {
    /// Every resolution class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Resolved,
        Self::Fallback,
        Self::Override,
        Self::Drift,
        Self::Conflict,
        Self::Blocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Fallback => "fallback",
            Self::Override => "override",
            Self::Drift => "drift",
            Self::Conflict => "conflict",
            Self::Blocked => "blocked",
        }
    }

    /// Highest presentation this resolution permits.
    ///
    /// A clean resolution presents transparently; a fallback, override, drift, or conflict narrows
    /// the inspector; a policy-lock block caps it at blocked.
    pub const fn presentation_ceiling(self) -> InspectorPresentation {
        match self {
            Self::Resolved => InspectorPresentation::Transparent,
            Self::Fallback | Self::Override | Self::Drift | Self::Conflict => {
                InspectorPresentation::Narrowed
            }
            Self::Blocked => InspectorPresentation::Blocked,
        }
    }

    /// The resolution-driven downgrade reason, if any.
    const fn downgrade_reason(self) -> Option<InspectorDowngradeReason> {
        match self {
            Self::Resolved => None,
            Self::Fallback => Some(InspectorDowngradeReason::SilentFallbackEliminated),
            Self::Override => Some(InspectorDowngradeReason::HiddenOverride),
            Self::Drift => Some(InspectorDowngradeReason::SourceDrift),
            Self::Conflict => Some(InspectorDowngradeReason::UnreconciledConflict),
            Self::Blocked => Some(InspectorDowngradeReason::PolicyLockBlocked),
        }
    }

    /// Whether the resolution itself needs the user to act, beyond a redaction narrowing.
    pub const fn requires_resolution(self) -> bool {
        !matches!(self, Self::Resolved)
    }
}

/// The presentation the precedence gate publishes for an inspector, highest to lowest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorPresentation {
    /// The winning value and every overshadowed candidate are shown in full; precedence is clear.
    Transparent,
    /// The inspector is shown but narrowed: a fallback / override / drift / conflict needs attention,
    /// or the values are shown by class / health / provenance for redaction safety. What won and what
    /// lost stays visible.
    Narrowed,
    /// The resolution is blocked by a policy lock; the inspector warns before the value is used.
    Blocked,
}

impl InspectorPresentation {
    /// Every presentation, highest to lowest.
    pub const ALL: [Self; 3] = [Self::Transparent, Self::Narrowed, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transparent => "transparent",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// Rank for the fail-closed gate; higher is more revealing.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Transparent => 2,
            Self::Narrowed => 1,
            Self::Blocked => 0,
        }
    }

    /// Whether the gate narrowed or blocked the inspector below a fully transparent resolution.
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::Transparent)
    }

    /// Whether the inspector must warn before the resolved value is used.
    pub const fn warns_before_use(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// The weaker (lower-rank) of two presentations.
fn weaker(a: InspectorPresentation, b: InspectorPresentation) -> InspectorPresentation {
    if b.rank() < a.rank() {
        b
    } else {
        a
    }
}

/// A headline reason the precedence gate narrows or blocks an inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorDowngradeReason {
    /// A fallback won because the preferred source was unavailable; surfaced so it is not silent.
    SilentFallbackEliminated,
    /// A higher-precedence source overrode a lower one; surfaced so it is not hidden.
    HiddenOverride,
    /// The resolved source drifted from the recorded / configured one.
    SourceDrift,
    /// Two sources tie at the same precedence and are unreconciled.
    UnreconciledConflict,
    /// A policy lock blocks the resolution.
    PolicyLockBlocked,
    /// The values are secret- or identity-bearing and shown by class / health / provenance only.
    RedactionBoundary,
}

impl InspectorDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SilentFallbackEliminated,
        Self::HiddenOverride,
        Self::SourceDrift,
        Self::UnreconciledConflict,
        Self::PolicyLockBlocked,
        Self::RedactionBoundary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SilentFallbackEliminated => "silent_fallback_eliminated",
            Self::HiddenOverride => "hidden_override",
            Self::SourceDrift => "source_drift",
            Self::UnreconciledConflict => "unreconciled_conflict",
            Self::PolicyLockBlocked => "policy_lock_blocked",
            Self::RedactionBoundary => "redaction_boundary",
        }
    }
}

/// The resolution path surfaced when an inspector is narrowed or blocked for a resolution reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrecedenceResolutionPath {
    /// Restore or repair the preferred, higher-precedence source so the fallback is no longer needed.
    RestorePreferredSource,
    /// Review the higher-precedence override that suppressed the lower-precedence value.
    ReviewOverride,
    /// Request a policy change to unblock the locked resolution.
    RequestPolicyChange,
    /// Re-authenticate so the credential resolution is current.
    Reauthenticate,
    /// Reconnect or re-resolve the drifted source.
    ReconnectSource,
    /// Reconcile the conflicting, equal-precedence sources.
    ReconcileConflict,
    /// No resolution path is needed; only valid when the resolution is clean.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl PrecedenceResolutionPath {
    /// Every resolution path, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RestorePreferredSource,
        Self::ReviewOverride,
        Self::RequestPolicyChange,
        Self::Reauthenticate,
        Self::ReconnectSource,
        Self::ReconcileConflict,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestorePreferredSource => "restore_preferred_source",
            Self::ReviewOverride => "review_override",
            Self::RequestPolicyChange => "request_policy_change",
            Self::Reauthenticate => "reauthenticate",
            Self::ReconnectSource => "reconnect_source",
            Self::ReconcileConflict => "reconcile_conflict",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real path the user can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// What it takes to apply or refresh the resolved value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartReauthPosture {
    /// The resolved value is in effect immediately; nothing to apply.
    #[serde(rename = "none")]
    NoneNeeded,
    /// Applying or repairing the resolution requires a restart.
    RestartRequired,
    /// Refreshing the credential resolution requires re-authentication.
    ReauthRequired,
    /// Re-resolving the drifted route requires reconnecting the target.
    ReconnectRequired,
}

impl RestartReauthPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoneNeeded,
        Self::RestartRequired,
        Self::ReauthRequired,
        Self::ReconnectRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneNeeded => "none",
            Self::RestartRequired => "restart_required",
            Self::ReauthRequired => "reauth_required",
            Self::ReconnectRequired => "reconnect_required",
        }
    }
}

/// The policy-lock state governing whether a lower-precedence value may take effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyLockState {
    /// No policy lock applies; precedence resolves on scope alone.
    Unlocked,
    /// A policy lock pins the value; lower-precedence sources cannot take effect.
    Locked,
}

impl PolicyLockState {
    /// Every policy-lock state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Unlocked, Self::Locked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unlocked => "unlocked",
            Self::Locked => "locked",
        }
    }

    /// Whether a policy lock is in force.
    pub const fn is_locked(self) -> bool {
        matches!(self, Self::Locked)
    }
}

/// The role a candidate played in the resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateDisposition {
    /// This candidate won the resolution.
    Winner,
    /// This candidate lost to a higher-precedence source.
    Overshadowed,
    /// This candidate would have won but was unavailable, forcing a fallback.
    Unavailable,
    /// This candidate is blocked by a policy lock.
    Blocked,
    /// This candidate ties with another at the same precedence.
    Conflicting,
}

impl CandidateDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Winner,
        Self::Overshadowed,
        Self::Unavailable,
        Self::Blocked,
        Self::Conflicting,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Winner => "winner",
            Self::Overshadowed => "overshadowed",
            Self::Unavailable => "unavailable",
            Self::Blocked => "blocked",
            Self::Conflicting => "conflicting",
        }
    }

    /// Whether this disposition is a candidate the winner suppressed or could not reconcile.
    pub const fn is_overshadowed(self) -> bool {
        !matches!(self, Self::Winner)
    }
}

/// A product surface a resolution affects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectedSurface {
    /// The run / launch surface.
    Run,
    /// The test surface.
    Test,
    /// The debug surface.
    Debug,
    /// The notebook surface.
    Notebook,
    /// The API request surface.
    Request,
    /// The database surface.
    Database,
    /// The preview / runtime surface.
    Preview,
    /// The pipeline / build surface.
    Pipeline,
    /// The editor surface.
    Editor,
    /// The integrated terminal surface.
    Terminal,
}

impl AffectedSurface {
    /// Every affected surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::Run,
        Self::Test,
        Self::Debug,
        Self::Notebook,
        Self::Request,
        Self::Database,
        Self::Preview,
        Self::Pipeline,
        Self::Editor,
        Self::Terminal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Test => "test",
            Self::Debug => "debug",
            Self::Notebook => "notebook",
            Self::Request => "request",
            Self::Database => "database",
            Self::Preview => "preview",
            Self::Pipeline => "pipeline",
            Self::Editor => "editor",
            Self::Terminal => "terminal",
        }
    }
}

/// A downstream surface that must ingest this registry and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorConsumerSurface {
    /// The active run-capable surface where the resolution is in effect.
    ActiveSurface,
    /// The Support Center's precedence-inspection views.
    SupportCenter,
    /// The support export of the precedence inspection.
    SupportExport,
    /// The issue-report / crash-intake packet.
    IssueReportPacket,
    /// The CLI / headless precedence inspect path.
    CliHeadless,
}

impl InspectorConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::ActiveSurface,
        Self::SupportCenter,
        Self::SupportExport,
        Self::IssueReportPacket,
        Self::CliHeadless,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveSurface => "active_surface",
            Self::SupportCenter => "support_center",
            Self::SupportExport => "support_export",
            Self::IssueReportPacket => "issue_report_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// One candidate value the resolver considered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrecedenceCandidate {
    /// Precedence scope this candidate comes from.
    pub source_class: PrecedenceSource,
    /// Redaction-safe value label; for a metadata-only inspector this is a class / health /
    /// provenance label, never the raw value.
    pub value_label: String,
    /// The role this candidate played in the resolution.
    pub disposition: CandidateDisposition,
    /// Why this candidate won or lost.
    pub reason: String,
    /// Ref to the resolver truth this candidate projects (source-of-truth lineage).
    pub descriptor_ref: String,
}

impl PrecedenceCandidate {
    /// Whether the candidate carries the non-empty value label, reason, and descriptor ref it requires.
    pub fn is_well_formed(&self) -> bool {
        !self.value_label.trim().is_empty()
            && !self.reason.trim().is_empty()
            && !self.descriptor_ref.trim().is_empty()
    }

    /// Whether this candidate won the resolution.
    pub fn is_winner(&self) -> bool {
        self.disposition == CandidateDisposition::Winner
    }
}

/// One precedence inspector: the winning value, the overshadowed candidates, and the lineage for a
/// single resolver decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrecedenceInspector {
    /// Stable inspector id.
    pub inspector_id: String,
    /// Resolver family this inspector explains.
    pub family: PrecedenceFamily,
    /// Human-readable label for the inspector (e.g. "Interpreter precedence").
    pub title: String,
    /// Redaction-safe label of the winning value; for a conflict, the unresolved marker.
    pub winning_value_label: String,
    /// Precedence scope the winning value came from.
    pub source_class: PrecedenceSource,
    /// How the values are disclosed after redaction.
    pub value_disclosure: ValueDisclosure,
    /// Policy-lock state governing whether a lower-precedence value may take effect.
    pub policy_lock_state: PolicyLockState,
    /// Overall outcome of the resolution.
    pub resolution_class: ResolutionClass,
    /// Presentation actually published after the gate; must equal the recomputed decision.
    pub presentation: InspectorPresentation,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<InspectorDowngradeReason>,
    /// Resolution path surfaced when the resolution needs attention.
    pub resolution_path: PrecedenceResolutionPath,
    /// What it takes to apply or refresh the resolved value.
    pub restart_reauth_posture: RestartReauthPosture,
    /// True when the inspector warns before the resolved value is used; required iff blocked.
    pub blocked_before_use: bool,
    /// Attestation that no raw secret bodies or hidden policy payloads are carried; always true.
    pub raw_material_excluded: bool,
    /// Candidate values the resolver considered; at least one winner (or, for a conflict, the
    /// conflicting set).
    #[serde(default)]
    pub candidates: Vec<PrecedenceCandidate>,
    /// Surfaces this resolution affects; at least one.
    #[serde(default)]
    pub affected_surfaces: Vec<AffectedSurface>,
    /// Caveats attached to a narrowed or blocked inspector.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// The source(s) that drove the downgrade (overridden, unavailable, drifted, blocked, conflicting).
    #[serde(default)]
    pub unmet_or_blocked_sources: Vec<String>,
    /// Ref to the resolver truth object this inspector projects.
    pub source_of_truth_ref: String,
    /// One-step "Why did this value win?" entrypoint; always present.
    pub explain_entrypoint_ref: String,
    /// The equivalent CLI / headless object id; always present.
    pub cli_object_ref: String,
    /// Ref to the conformance suite backing the inspector.
    pub conformance_ref: String,
    /// Ref to the inspector's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the inspector answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Ref to the machine-readable inspector receipt.
    pub inspector_receipt_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl PrecedenceInspector {
    /// The winning candidate, if exactly one is declared.
    pub fn winner(&self) -> Option<&PrecedenceCandidate> {
        let mut winners = self.candidates.iter().filter(|c| c.is_winner());
        let first = winners.next()?;
        if winners.next().is_some() {
            None
        } else {
            Some(first)
        }
    }

    /// The candidates the winner overshadowed, could not reconcile, or that were blocked.
    pub fn overshadowed_candidates(&self) -> impl Iterator<Item = &PrecedenceCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.disposition.is_overshadowed())
    }

    /// The number of distinct surfaces this resolution affects.
    pub fn affected_surface_count(&self) -> usize {
        self.affected_surfaces.iter().collect::<BTreeSet<_>>().len()
    }

    /// Highest presentation the resolution permits.
    pub fn resolution_ceiling(&self) -> InspectorPresentation {
        self.resolution_class.presentation_ceiling()
    }

    /// Highest presentation the value disclosure permits.
    pub fn disclosure_ceiling(&self) -> InspectorPresentation {
        self.value_disclosure.presentation_ceiling()
    }

    /// The presentation the gate permits this inspector to publish.
    ///
    /// Lowers the clean baseline to the weaker of the resolution ceiling and the disclosure ceiling,
    /// so a fallback, override, drift, conflict, policy block, or redaction boundary can never present
    /// a fuller claim than the inputs support.
    pub fn effective_presentation(&self) -> InspectorPresentation {
        weaker(self.resolution_ceiling(), self.disclosure_ceiling())
    }

    /// The headline downgrade reasons recomputed from the inspector's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<InspectorDowngradeReason> {
        InspectorDowngradeReason::ALL
            .into_iter()
            .filter(|reason| match reason {
                InspectorDowngradeReason::RedactionBoundary => {
                    self.value_disclosure.is_metadata_only()
                }
                other => self.resolution_class.downgrade_reason() == Some(*other),
            })
            .collect()
    }

    /// The resolution path the gate must record, derived from the resolution class and family.
    pub fn computed_resolution_path(&self) -> PrecedenceResolutionPath {
        match self.resolution_class {
            ResolutionClass::Resolved => PrecedenceResolutionPath::NoneNeeded,
            ResolutionClass::Fallback => PrecedenceResolutionPath::RestorePreferredSource,
            ResolutionClass::Override => PrecedenceResolutionPath::ReviewOverride,
            ResolutionClass::Blocked => PrecedenceResolutionPath::RequestPolicyChange,
            ResolutionClass::Conflict => PrecedenceResolutionPath::ReconcileConflict,
            ResolutionClass::Drift => {
                if self.family == PrecedenceFamily::Credential {
                    PrecedenceResolutionPath::Reauthenticate
                } else {
                    PrecedenceResolutionPath::ReconnectSource
                }
            }
        }
    }

    /// Whether the inspector presents a fully transparent resolution.
    pub fn is_transparent(&self) -> bool {
        self.effective_presentation() == InspectorPresentation::Transparent
    }

    /// Whether the inspector carries its own non-empty one-step explain and CLI-equivalent refs.
    pub fn has_one_step_explainability(&self) -> bool {
        !self.explain_entrypoint_ref.trim().is_empty() && !self.cli_object_ref.trim().is_empty()
    }

    /// Whether the inspector carries its own non-empty lineage, conformance, evidence, scope, and
    /// receipt refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.source_of_truth_ref.trim().is_empty()
            && !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
            && !self.inspector_receipt_ref.trim().is_empty()
    }

    /// Whether the recorded presentation, reasons, path, posture, and blocked flag agree with the gate.
    pub fn gate_consistent(&self) -> bool {
        let effective = self.effective_presentation();
        self.presentation == effective
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.resolution_path == self.computed_resolution_path()
            && self.blocked_before_use == effective.warns_before_use()
    }
}

/// One binding wiring a downstream surface to this registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InspectorConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: InspectorConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Packet id this surface ingests.
    pub packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface ingests this registry rather than a parallel list.
    pub ingests_registry: bool,
    /// True when the surface preserves the precedence vocabulary verbatim.
    pub preserves_precedence_vocabulary: bool,
    /// True when the surface preserves the inspector and CLI object ids rather than reminting them.
    pub preserves_object_ids: bool,
    /// True when the surface narrows automatically as inspectors are narrowed or blocked.
    pub narrows_on_downgrade: bool,
    /// True when raw secret or hidden-policy material is excluded from the binding.
    pub raw_material_excluded: bool,
}

impl InspectorConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.ingests_registry
            && self.preserves_precedence_vocabulary
            && self.preserves_object_ids
            && self.narrows_on_downgrade
            && self.raw_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PrecedenceInspectorSummary {
    /// Total inspectors.
    pub total_inspectors: usize,
    /// Inspectors that present a fully transparent resolution.
    pub transparent_inspectors: usize,
    /// Inspectors the gate narrowed.
    pub narrowed_inspectors: usize,
    /// Inspectors the gate blocked.
    pub blocked_inspectors: usize,
    /// Inspectors carrying at least one downgrade reason.
    pub inspectors_with_downgrade_reasons: usize,
    /// Inspectors whose values are shown by class / health / provenance only.
    pub metadata_only_inspectors: usize,
    /// Inspectors that warn before the resolved value is used.
    pub blocked_before_use_inspectors: usize,
    /// Distinct resolver families covered.
    pub families_covered: usize,
    /// Total candidate values across all inspectors.
    pub total_candidates: usize,
    /// Total affected-surface links across all inspectors.
    pub total_affected_surface_links: usize,
}

/// A redaction-safe export row projected from an inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrecedenceInspectorExportRow {
    /// Inspector id.
    pub inspector_id: String,
    /// Family token.
    pub family: String,
    /// Winning-value label.
    pub winning_value_label: String,
    /// Winning source-class token.
    pub source_class: String,
    /// Value-disclosure token.
    pub value_disclosure: String,
    /// Policy-lock-state token.
    pub policy_lock_state: String,
    /// Resolution-class token.
    pub resolution_class: String,
    /// Published-presentation token.
    pub presentation: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Resolution-path token.
    pub resolution_path: String,
    /// Restart-or-reauth-posture token.
    pub restart_reauth_posture: String,
    /// Whether the inspector warns before the resolved value is used.
    pub blocked_before_use: bool,
    /// Overshadowed-candidate value labels.
    pub overshadowed_candidates: Vec<String>,
    /// Affected-surface tokens.
    pub affected_surfaces: Vec<String>,
    /// Number of distinct surfaces affected.
    pub affected_surface_count: usize,
    /// Source(s) that drove the downgrade.
    pub unmet_or_blocked_sources: Vec<String>,
    /// Source-of-truth lineage ref.
    pub source_of_truth_ref: String,
    /// One-step explain entrypoint ref.
    pub explain_entrypoint_ref: String,
    /// CLI / headless equivalent object id.
    pub cli_object_ref: String,
    /// Scope snapshot the inspector answered.
    pub scope_snapshot_ref: String,
    /// Inspector-receipt ref.
    pub inspector_receipt_ref: String,
    /// Whether the inspector presents transparently.
    pub transparent: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the registry — the canonical precedence index downstream
/// surfaces render instead of restating each resolution by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrecedenceInspectorExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5PrecedenceInspectorExportRow>,
    /// Whether every inspector's published presentation and decision agree with the gate.
    pub all_inspectors_gate_consistent: bool,
    /// Inspectors that present transparently.
    pub transparent_count: usize,
    /// Inspectors the gate narrowed.
    pub narrowed_count: usize,
    /// Inspectors the gate blocked.
    pub blocked_count: usize,
}

/// The typed precedence-inspector registry packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PrecedenceInspectors {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed resolver-family vocabulary.
    pub families: Vec<PrecedenceFamily>,
    /// Closed precedence-source vocabulary.
    pub source_classes: Vec<PrecedenceSource>,
    /// Closed value-disclosure vocabulary.
    pub value_disclosures: Vec<ValueDisclosure>,
    /// Closed resolution-class vocabulary.
    pub resolution_classes: Vec<ResolutionClass>,
    /// Closed presentation vocabulary.
    pub presentations: Vec<InspectorPresentation>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<InspectorDowngradeReason>,
    /// Closed resolution-path vocabulary.
    pub resolution_paths: Vec<PrecedenceResolutionPath>,
    /// Closed restart-or-reauth-posture vocabulary.
    pub restart_reauth_postures: Vec<RestartReauthPosture>,
    /// Closed policy-lock-state vocabulary.
    pub policy_lock_states: Vec<PolicyLockState>,
    /// Closed candidate-disposition vocabulary.
    pub candidate_dispositions: Vec<CandidateDisposition>,
    /// Closed affected-surface vocabulary.
    pub affected_surfaces: Vec<AffectedSurface>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<InspectorConsumerSurface>,
    /// Inspectors, one per resolver decision worth explaining.
    #[serde(default)]
    pub inspectors: Vec<PrecedenceInspector>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<InspectorConsumerBinding>,
    /// Summary counts.
    pub summary: M5PrecedenceInspectorSummary,
}

impl M5PrecedenceInspectors {
    /// Returns the inspector with the given id.
    pub fn inspector(&self, inspector_id: &str) -> Option<&PrecedenceInspector> {
        self.inspectors
            .iter()
            .find(|i| i.inspector_id == inspector_id)
    }

    /// Inspectors for the given resolver family.
    pub fn inspectors_for(
        &self,
        family: PrecedenceFamily,
    ) -> impl Iterator<Item = &PrecedenceInspector> {
        self.inspectors.iter().filter(move |i| i.family == family)
    }

    /// Inspectors that present transparently.
    pub fn transparent_inspectors(&self) -> impl Iterator<Item = &PrecedenceInspector> {
        self.inspectors
            .iter()
            .filter(|i| i.effective_presentation() == InspectorPresentation::Transparent)
    }

    /// Inspectors the gate narrowed.
    pub fn narrowed_inspectors(&self) -> impl Iterator<Item = &PrecedenceInspector> {
        self.inspectors
            .iter()
            .filter(|i| i.effective_presentation() == InspectorPresentation::Narrowed)
    }

    /// Inspectors the gate blocked.
    pub fn blocked_inspectors(&self) -> impl Iterator<Item = &PrecedenceInspector> {
        self.inspectors
            .iter()
            .filter(|i| i.effective_presentation() == InspectorPresentation::Blocked)
    }

    /// Whether a consumer binding preserves this registry for the given surface.
    pub fn has_binding_for(&self, surface: InspectorConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every inspector's recorded decision agrees with the gate.
    pub fn all_inspectors_gate_consistent(&self) -> bool {
        self.inspectors
            .iter()
            .all(PrecedenceInspector::gate_consistent)
    }

    /// Recomputes the summary block from the inspectors.
    pub fn computed_summary(&self) -> M5PrecedenceInspectorSummary {
        let count_presentation = |decision: InspectorPresentation| {
            self.inspectors
                .iter()
                .filter(|i| i.effective_presentation() == decision)
                .count()
        };
        let mut families = BTreeSet::new();
        let mut total_candidates = 0usize;
        let mut total_links = 0usize;
        for inspector in &self.inspectors {
            families.insert(inspector.family);
            total_candidates += inspector.candidates.len();
            total_links += inspector.affected_surfaces.len();
        }
        M5PrecedenceInspectorSummary {
            total_inspectors: self.inspectors.len(),
            transparent_inspectors: count_presentation(InspectorPresentation::Transparent),
            narrowed_inspectors: count_presentation(InspectorPresentation::Narrowed),
            blocked_inspectors: count_presentation(InspectorPresentation::Blocked),
            inspectors_with_downgrade_reasons: self
                .inspectors
                .iter()
                .filter(|i| !i.downgrade_reasons.is_empty())
                .count(),
            metadata_only_inspectors: self
                .inspectors
                .iter()
                .filter(|i| i.value_disclosure.is_metadata_only())
                .count(),
            blocked_before_use_inspectors: self
                .inspectors
                .iter()
                .filter(|i| i.blocked_before_use)
                .count(),
            families_covered: families.len(),
            total_candidates,
            total_affected_surface_links: total_links,
        }
    }

    /// Produces the precedence index downstream surfaces render instead of restating each resolution
    /// by hand.
    pub fn export_projection(&self) -> M5PrecedenceInspectorExportProjection {
        let rows = self
            .inspectors
            .iter()
            .map(|i| M5PrecedenceInspectorExportRow {
                inspector_id: i.inspector_id.clone(),
                family: i.family.as_str().to_owned(),
                winning_value_label: i.winning_value_label.clone(),
                source_class: i.source_class.as_str().to_owned(),
                value_disclosure: i.value_disclosure.as_str().to_owned(),
                policy_lock_state: i.policy_lock_state.as_str().to_owned(),
                resolution_class: i.resolution_class.as_str().to_owned(),
                presentation: i.presentation.as_str().to_owned(),
                downgrade_reasons: i
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                resolution_path: i.resolution_path.as_str().to_owned(),
                restart_reauth_posture: i.restart_reauth_posture.as_str().to_owned(),
                blocked_before_use: i.blocked_before_use,
                overshadowed_candidates: i
                    .overshadowed_candidates()
                    .map(|c| c.value_label.clone())
                    .collect(),
                affected_surfaces: i
                    .affected_surfaces
                    .iter()
                    .map(|s| s.as_str().to_owned())
                    .collect(),
                affected_surface_count: i.affected_surface_count(),
                unmet_or_blocked_sources: i.unmet_or_blocked_sources.clone(),
                source_of_truth_ref: i.source_of_truth_ref.clone(),
                explain_entrypoint_ref: i.explain_entrypoint_ref.clone(),
                cli_object_ref: i.cli_object_ref.clone(),
                scope_snapshot_ref: i.scope_snapshot_ref.clone(),
                inspector_receipt_ref: i.inspector_receipt_ref.clone(),
                transparent: i.is_transparent(),
                summary: format!(
                    "{}: {} won ({}), resolution {}, presentation {}",
                    i.family.as_str(),
                    i.winning_value_label,
                    i.source_class.as_str(),
                    i.resolution_class.as_str(),
                    i.presentation.as_str()
                ),
            })
            .collect();
        M5PrecedenceInspectorExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_inspectors_gate_consistent: self.all_inspectors_gate_consistent(),
            transparent_count: self.transparent_inspectors().count(),
            narrowed_count: self.narrowed_inspectors().count(),
            blocked_count: self.blocked_inspectors().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact inspector registry.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5PrecedenceInspectorSupportExport {
        M5PrecedenceInspectorSupportExport {
            record_kind: M5_PRECEDENCE_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_PRECEDENCE_INSPECTOR_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_material_excluded: true,
            registry: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5PrecedenceInspectorViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for inspector in &self.inspectors {
            if !seen_ids.insert(inspector.inspector_id.clone()) {
                violations.push(M5PrecedenceInspectorViolation::DuplicateInspector {
                    inspector_id: inspector.inspector_id.clone(),
                });
            }
            seen_families.insert(inspector.family);
            self.validate_inspector(inspector, &mut violations);
        }

        // Every resolver family must carry at least one inspector, so route, credential, setting,
        // policy, and toolchain precedence all share this one supportability grammar.
        for family in PrecedenceFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(M5PrecedenceInspectorViolation::MissingFamily {
                    family: family.as_str(),
                });
            }
        }

        for surface in InspectorConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5PrecedenceInspectorViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5PrecedenceInspectorViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5PrecedenceInspectorViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5PrecedenceInspectorViolation>) {
        if self.schema_version != M5_PRECEDENCE_INSPECTOR_SCHEMA_VERSION {
            violations.push(M5PrecedenceInspectorViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_PRECEDENCE_INSPECTOR_RECORD_KIND {
            violations.push(M5PrecedenceInspectorViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PrecedenceInspectorViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("families", self.families == PrecedenceFamily::ALL.to_vec()),
            (
                "source_classes",
                self.source_classes == PrecedenceSource::ALL.to_vec(),
            ),
            (
                "value_disclosures",
                self.value_disclosures == ValueDisclosure::ALL.to_vec(),
            ),
            (
                "resolution_classes",
                self.resolution_classes == ResolutionClass::ALL.to_vec(),
            ),
            (
                "presentations",
                self.presentations == InspectorPresentation::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == InspectorDowngradeReason::ALL.to_vec(),
            ),
            (
                "resolution_paths",
                self.resolution_paths == PrecedenceResolutionPath::ALL.to_vec(),
            ),
            (
                "restart_reauth_postures",
                self.restart_reauth_postures == RestartReauthPosture::ALL.to_vec(),
            ),
            (
                "policy_lock_states",
                self.policy_lock_states == PolicyLockState::ALL.to_vec(),
            ),
            (
                "candidate_dispositions",
                self.candidate_dispositions == CandidateDisposition::ALL.to_vec(),
            ),
            (
                "affected_surfaces",
                self.affected_surfaces == AffectedSurface::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == InspectorConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5PrecedenceInspectorViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_inspector(
        &self,
        inspector: &PrecedenceInspector,
        violations: &mut Vec<M5PrecedenceInspectorViolation>,
    ) {
        for (field, value) in [
            ("inspector_id", &inspector.inspector_id),
            ("title", &inspector.title),
            ("winning_value_label", &inspector.winning_value_label),
            ("source_of_truth_ref", &inspector.source_of_truth_ref),
            ("explain_entrypoint_ref", &inspector.explain_entrypoint_ref),
            ("cli_object_ref", &inspector.cli_object_ref),
            ("conformance_ref", &inspector.conformance_ref),
            ("evidence_ref", &inspector.evidence_ref),
            ("scope_snapshot_ref", &inspector.scope_snapshot_ref),
            ("inspector_receipt_ref", &inspector.inspector_receipt_ref),
            ("note", &inspector.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PrecedenceInspectorViolation::EmptyField {
                    id: inspector.inspector_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every inspector must carry its one-step "Why did this value win?" entry and its CLI/headless
        // equivalent, so precedence is answerable from the active surface, Support Center, and CLI.
        if !inspector.has_one_step_explainability() {
            violations.push(
                M5PrecedenceInspectorViolation::MissingOneStepExplainability {
                    inspector_id: inspector.inspector_id.clone(),
                },
            );
        }

        // No raw secret bodies or hidden policy payloads may be carried, ever.
        if !inspector.raw_material_excluded {
            violations.push(M5PrecedenceInspectorViolation::RawMaterialNotExcluded {
                inspector_id: inspector.inspector_id.clone(),
            });
        }

        self.validate_candidates(inspector, violations);
        self.validate_affected_surfaces(inspector, violations);
        self.validate_redaction(inspector, violations);
        self.validate_posture(inspector, violations);
        self.validate_policy_lock(inspector, violations);
        self.validate_precedence(inspector, violations);
        self.validate_gate(inspector, violations);
    }

    fn validate_candidates(
        &self,
        inspector: &PrecedenceInspector,
        violations: &mut Vec<M5PrecedenceInspectorViolation>,
    ) {
        // Every inspector must show the candidates the winner overshadowed — the whole point is that
        // what lost stays visible, not only what won.
        if inspector.candidates.len() < 2 {
            violations.push(M5PrecedenceInspectorViolation::TooFewCandidates {
                inspector_id: inspector.inspector_id.clone(),
            });
        }
        for candidate in &inspector.candidates {
            if !candidate.is_well_formed() {
                violations.push(M5PrecedenceInspectorViolation::CandidateIncomplete {
                    inspector_id: inspector.inspector_id.clone(),
                });
            }
        }

        let winners = inspector
            .candidates
            .iter()
            .filter(|c| c.is_winner())
            .count();
        let conflicting = inspector
            .candidates
            .iter()
            .filter(|c| c.disposition == CandidateDisposition::Conflicting)
            .count();

        if inspector.resolution_class == ResolutionClass::Conflict {
            // A conflict has no single winner: at least two candidates tie at the same precedence.
            if winners != 0 {
                violations.push(M5PrecedenceInspectorViolation::ConflictHasWinner {
                    inspector_id: inspector.inspector_id.clone(),
                });
            }
            if conflicting < 2 {
                violations.push(M5PrecedenceInspectorViolation::ConflictUnderspecified {
                    inspector_id: inspector.inspector_id.clone(),
                });
            }
        } else {
            // Every non-conflict resolution names exactly one winning candidate.
            if winners != 1 {
                violations.push(M5PrecedenceInspectorViolation::WinnerCountInvalid {
                    inspector_id: inspector.inspector_id.clone(),
                    winners,
                });
            }
            if let Some(winner) = inspector.winner() {
                if winner.value_label != inspector.winning_value_label {
                    violations.push(M5PrecedenceInspectorViolation::WinningValueMismatch {
                        inspector_id: inspector.inspector_id.clone(),
                    });
                }
                if winner.source_class != inspector.source_class {
                    violations.push(M5PrecedenceInspectorViolation::WinningSourceMismatch {
                        inspector_id: inspector.inspector_id.clone(),
                    });
                }
            }
        }

        // Every inspector must show at least one overshadowed candidate, so the silent fallback or
        // hidden override is always visible.
        if inspector.overshadowed_candidates().next().is_none() {
            violations.push(M5PrecedenceInspectorViolation::NoOvershadowedCandidate {
                inspector_id: inspector.inspector_id.clone(),
            });
        }
    }

    fn validate_affected_surfaces(
        &self,
        inspector: &PrecedenceInspector,
        violations: &mut Vec<M5PrecedenceInspectorViolation>,
    ) {
        if inspector.affected_surfaces.is_empty() {
            violations.push(M5PrecedenceInspectorViolation::NoAffectedSurface {
                inspector_id: inspector.inspector_id.clone(),
            });
        }
        let mut seen = BTreeSet::new();
        for surface in &inspector.affected_surfaces {
            if !seen.insert(*surface) {
                violations.push(M5PrecedenceInspectorViolation::DuplicateAffectedSurface {
                    inspector_id: inspector.inspector_id.clone(),
                    surface: surface.as_str(),
                });
            }
        }
    }

    fn validate_redaction(
        &self,
        inspector: &PrecedenceInspector,
        violations: &mut Vec<M5PrecedenceInspectorViolation>,
    ) {
        let metadata_only = inspector.value_disclosure.is_metadata_only();
        let has_redaction_reason = inspector
            .downgrade_reasons
            .contains(&InspectorDowngradeReason::RedactionBoundary);

        // Metadata-only disclosure must be flagged with a redaction-boundary reason, and only an
        // identity-bearing family may use it — so a non-secret resolution can never hide behind
        // "metadata only" and a secret one can never present as plain values.
        if metadata_only != has_redaction_reason {
            violations.push(M5PrecedenceInspectorViolation::RedactionReasonMismatch {
                inspector_id: inspector.inspector_id.clone(),
            });
        }
        if metadata_only && !inspector.family.is_identity_bearing() {
            violations.push(M5PrecedenceInspectorViolation::MetadataOnlyWrongFamily {
                inspector_id: inspector.inspector_id.clone(),
                family: inspector.family.as_str(),
            });
        }
    }

    fn validate_posture(
        &self,
        inspector: &PrecedenceInspector,
        violations: &mut Vec<M5PrecedenceInspectorViolation>,
    ) {
        let family = inspector.family;
        let ok = match inspector.restart_reauth_posture {
            RestartReauthPosture::ReauthRequired => family == PrecedenceFamily::Credential,
            RestartReauthPosture::ReconnectRequired => family == PrecedenceFamily::Route,
            RestartReauthPosture::RestartRequired | RestartReauthPosture::NoneNeeded => true,
        };
        if !ok {
            violations.push(M5PrecedenceInspectorViolation::PostureFamilyMismatch {
                inspector_id: inspector.inspector_id.clone(),
                posture: inspector.restart_reauth_posture.as_str(),
                family: family.as_str(),
            });
        }
    }

    fn validate_policy_lock(
        &self,
        inspector: &PrecedenceInspector,
        violations: &mut Vec<M5PrecedenceInspectorViolation>,
    ) {
        // A policy lock and a policy-lock block stand or fall together: a locked resolution blocks the
        // lower-precedence value, and a blocked resolution is locked.
        let locked = inspector.policy_lock_state.is_locked();
        let blocked = inspector.resolution_class == ResolutionClass::Blocked;
        if locked != blocked {
            violations.push(M5PrecedenceInspectorViolation::PolicyLockMismatch {
                inspector_id: inspector.inspector_id.clone(),
            });
        }
    }

    fn validate_precedence(
        &self,
        inspector: &PrecedenceInspector,
        violations: &mut Vec<M5PrecedenceInspectorViolation>,
    ) {
        match inspector.resolution_class {
            // A clean win or an override must genuinely out-precede every candidate it overshadowed;
            // a lower-precedence value that wins without a fallback / drift / conflict explanation is
            // exactly the silent fallback this packet exists to catch.
            ResolutionClass::Resolved | ResolutionClass::Override => {
                if let Some(winner) = inspector.winner() {
                    let winner_rank = winner.source_class.rank();
                    let outranked = inspector
                        .overshadowed_candidates()
                        .all(|c| winner_rank >= c.source_class.rank());
                    if !outranked {
                        violations.push(M5PrecedenceInspectorViolation::WinnerDoesNotOutrank {
                            inspector_id: inspector.inspector_id.clone(),
                        });
                    }
                }
            }
            // A fallback's winner must be out-precedence-ed by an unavailable candidate — otherwise
            // nothing forced the fallback and the resolution should have been clean.
            ResolutionClass::Fallback => {
                if let Some(winner) = inspector.winner() {
                    let winner_rank = winner.source_class.rank();
                    let forced = inspector.candidates.iter().any(|c| {
                        c.disposition == CandidateDisposition::Unavailable
                            && c.source_class.rank() > winner_rank
                    });
                    if !forced {
                        violations.push(M5PrecedenceInspectorViolation::FallbackNotForced {
                            inspector_id: inspector.inspector_id.clone(),
                        });
                    }
                }
            }
            ResolutionClass::Drift | ResolutionClass::Conflict | ResolutionClass::Blocked => {}
        }
    }

    fn validate_gate(
        &self,
        inspector: &PrecedenceInspector,
        violations: &mut Vec<M5PrecedenceInspectorViolation>,
    ) {
        // The published presentation must equal the gate's recomputed decision, so a fallback, override,
        // drift, conflict, policy block, or redaction boundary can never read as a clean "this won" chip.
        let effective = inspector.effective_presentation();
        if inspector.presentation != effective {
            violations.push(M5PrecedenceInspectorViolation::OverstatedPresentation {
                inspector_id: inspector.inspector_id.clone(),
                published: inspector.presentation.as_str(),
                computed: effective.as_str(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &inspector.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5PrecedenceInspectorViolation::DuplicateDowngradeReason {
                    inspector_id: inspector.inspector_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }
        if inspector.downgrade_reasons != inspector.computed_downgrade_reasons() {
            violations.push(M5PrecedenceInspectorViolation::DowngradeReasonsMismatch {
                inspector_id: inspector.inspector_id.clone(),
            });
        }

        let computed_path = inspector.computed_resolution_path();
        if inspector.resolution_path != computed_path {
            violations.push(M5PrecedenceInspectorViolation::ResolutionPathMismatch {
                inspector_id: inspector.inspector_id.clone(),
                declared: inspector.resolution_path.as_str(),
                required: computed_path.as_str(),
            });
        }

        // A blocked inspector must warn before the resolved value is used, and a non-blocked one must
        // not claim it does.
        if inspector.blocked_before_use != effective.warns_before_use() {
            violations.push(M5PrecedenceInspectorViolation::BlockedBeforeUseMismatch {
                inspector_id: inspector.inspector_id.clone(),
            });
        }

        // A narrowed or blocked inspector always carries a caveat naming why precedence is not fully
        // transparent.
        if effective.requires_attention() && inspector.caveats.is_empty() {
            violations.push(M5PrecedenceInspectorViolation::EmptyField {
                id: inspector.inspector_id.clone(),
                field_name: "caveats",
            });
        }

        // A resolution that needs the user to act always names a real resolution path and the source
        // that drove the downgrade — a narrowing never drops its remediation or hides its cause.
        if inspector.resolution_class.requires_resolution() {
            if !inspector.resolution_path.is_offered() {
                violations.push(M5PrecedenceInspectorViolation::MissingResolutionPath {
                    inspector_id: inspector.inspector_id.clone(),
                });
            }
            if inspector.unmet_or_blocked_sources.is_empty() {
                violations.push(M5PrecedenceInspectorViolation::EmptyField {
                    id: inspector.inspector_id.clone(),
                    field_name: "unmet_or_blocked_sources",
                });
            }
        } else if inspector.resolution_path.is_offered() {
            // A clean resolution offers no resolution path.
            violations.push(M5PrecedenceInspectorViolation::ResolvedOffersPath {
                inspector_id: inspector.inspector_id.clone(),
            });
        }

        // A fully transparent inspector must be genuinely whole: a clean resolution, plain values,
        // nothing flagging it.
        if effective == InspectorPresentation::Transparent
            && (inspector.resolution_class != ResolutionClass::Resolved
                || inspector.value_disclosure != ValueDisclosure::PlainValues
                || !inspector.downgrade_reasons.is_empty()
                || !inspector.caveats.is_empty()
                || !inspector.unmet_or_blocked_sources.is_empty()
                || inspector.resolution_path.is_offered()
                || inspector.restart_reauth_posture != RestartReauthPosture::NoneNeeded
                || inspector.policy_lock_state.is_locked()
                || inspector.blocked_before_use)
        {
            violations.push(
                M5PrecedenceInspectorViolation::TransparentInspectorNotWhole {
                    inspector_id: inspector.inspector_id.clone(),
                },
            );
        }
    }
}

/// A validation violation for the precedence-inspector registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PrecedenceInspectorViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Inspector or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An inspector id appears more than once.
    DuplicateInspector {
        /// Duplicate inspector id.
        inspector_id: String,
    },
    /// A resolver family has no inspector.
    MissingFamily {
        /// Family token.
        family: &'static str,
    },
    /// An inspector is missing its one-step explain entry or CLI-equivalent object id.
    MissingOneStepExplainability {
        /// Inspector id.
        inspector_id: String,
    },
    /// An inspector does not attest that raw secret or hidden-policy material is excluded.
    RawMaterialNotExcluded {
        /// Inspector id.
        inspector_id: String,
    },
    /// An inspector lists fewer than two candidates.
    TooFewCandidates {
        /// Inspector id.
        inspector_id: String,
    },
    /// A candidate is missing its value label, reason, or descriptor ref.
    CandidateIncomplete {
        /// Inspector id.
        inspector_id: String,
    },
    /// A non-conflict inspector names a number of winners other than one.
    WinnerCountInvalid {
        /// Inspector id.
        inspector_id: String,
        /// Number of winners found.
        winners: usize,
    },
    /// A conflict inspector names a winner.
    ConflictHasWinner {
        /// Inspector id.
        inspector_id: String,
    },
    /// A conflict inspector names fewer than two conflicting candidates.
    ConflictUnderspecified {
        /// Inspector id.
        inspector_id: String,
    },
    /// The top-level winning value disagrees with the winning candidate.
    WinningValueMismatch {
        /// Inspector id.
        inspector_id: String,
    },
    /// The top-level winning source disagrees with the winning candidate.
    WinningSourceMismatch {
        /// Inspector id.
        inspector_id: String,
    },
    /// An inspector shows no overshadowed candidate.
    NoOvershadowedCandidate {
        /// Inspector id.
        inspector_id: String,
    },
    /// An inspector names no affected surface.
    NoAffectedSurface {
        /// Inspector id.
        inspector_id: String,
    },
    /// An inspector lists the same affected surface twice.
    DuplicateAffectedSurface {
        /// Inspector id.
        inspector_id: String,
        /// Surface token.
        surface: &'static str,
    },
    /// Metadata-only disclosure and the redaction-boundary reason disagree.
    RedactionReasonMismatch {
        /// Inspector id.
        inspector_id: String,
    },
    /// A non-identity-bearing family claims metadata-only disclosure.
    MetadataOnlyWrongFamily {
        /// Inspector id.
        inspector_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A restart-or-reauth posture does not match the inspector's family.
    PostureFamilyMismatch {
        /// Inspector id.
        inspector_id: String,
        /// Posture token.
        posture: &'static str,
        /// Family token.
        family: &'static str,
    },
    /// The policy-lock state and the blocked resolution disagree.
    PolicyLockMismatch {
        /// Inspector id.
        inspector_id: String,
    },
    /// A clean win or override does not out-precede the candidates it overshadowed.
    WinnerDoesNotOutrank {
        /// Inspector id.
        inspector_id: String,
    },
    /// A fallback is not forced by an unavailable, higher-precedence candidate.
    FallbackNotForced {
        /// Inspector id.
        inspector_id: String,
    },
    /// An inspector publishes a presentation cleaner than the gate computes.
    OverstatedPresentation {
        /// Inspector id.
        inspector_id: String,
        /// Published presentation token.
        published: &'static str,
        /// Computed effective presentation token.
        computed: &'static str,
    },
    /// An inspector lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Inspector id.
        inspector_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// An inspector's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Inspector id.
        inspector_id: String,
    },
    /// An inspector's resolution path disagrees with the recomputed path.
    ResolutionPathMismatch {
        /// Inspector id.
        inspector_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// An inspector's blocked-before-use flag disagrees with the gate.
    BlockedBeforeUseMismatch {
        /// Inspector id.
        inspector_id: String,
    },
    /// A narrowed or blocked inspector offers no resolution path.
    MissingResolutionPath {
        /// Inspector id.
        inspector_id: String,
    },
    /// A cleanly resolved inspector offers a resolution path.
    ResolvedOffersPath {
        /// Inspector id.
        inspector_id: String,
    },
    /// A transparent inspector flags a state or carries a reason.
    TransparentInspectorNotWhole {
        /// Inspector id.
        inspector_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints registry truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the inspectors.
    SummaryMismatch,
}

impl fmt::Display for M5PrecedenceInspectorViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateInspector { inspector_id } => {
                write!(f, "duplicate inspector id {inspector_id}")
            }
            Self::MissingFamily { family } => write!(f, "missing inspector for family {family}"),
            Self::MissingOneStepExplainability { inspector_id } => write!(
                f,
                "inspector {inspector_id} is missing its one-step explain entry or CLI-equivalent object id"
            ),
            Self::RawMaterialNotExcluded { inspector_id } => write!(
                f,
                "inspector {inspector_id} does not attest raw secret/policy material is excluded"
            ),
            Self::TooFewCandidates { inspector_id } => {
                write!(f, "inspector {inspector_id} lists fewer than two candidates")
            }
            Self::CandidateIncomplete { inspector_id } => write!(
                f,
                "inspector {inspector_id} has a candidate missing its value label, reason, or descriptor ref"
            ),
            Self::WinnerCountInvalid {
                inspector_id,
                winners,
            } => write!(
                f,
                "inspector {inspector_id} names {winners} winners (expected exactly one)"
            ),
            Self::ConflictHasWinner { inspector_id } => {
                write!(f, "conflict inspector {inspector_id} names a winner")
            }
            Self::ConflictUnderspecified { inspector_id } => write!(
                f,
                "conflict inspector {inspector_id} names fewer than two conflicting candidates"
            ),
            Self::WinningValueMismatch { inspector_id } => write!(
                f,
                "inspector {inspector_id} winning value disagrees with the winning candidate"
            ),
            Self::WinningSourceMismatch { inspector_id } => write!(
                f,
                "inspector {inspector_id} winning source disagrees with the winning candidate"
            ),
            Self::NoOvershadowedCandidate { inspector_id } => {
                write!(f, "inspector {inspector_id} shows no overshadowed candidate")
            }
            Self::NoAffectedSurface { inspector_id } => {
                write!(f, "inspector {inspector_id} names no affected surface")
            }
            Self::DuplicateAffectedSurface {
                inspector_id,
                surface,
            } => write!(
                f,
                "inspector {inspector_id} lists affected surface {surface} more than once"
            ),
            Self::RedactionReasonMismatch { inspector_id } => write!(
                f,
                "inspector {inspector_id} metadata-only disclosure and redaction-boundary reason disagree"
            ),
            Self::MetadataOnlyWrongFamily {
                inspector_id,
                family,
            } => write!(
                f,
                "inspector {inspector_id} family {family} cannot claim metadata-only disclosure"
            ),
            Self::PostureFamilyMismatch {
                inspector_id,
                posture,
                family,
            } => write!(
                f,
                "inspector {inspector_id} posture {posture} does not match family {family}"
            ),
            Self::PolicyLockMismatch { inspector_id } => write!(
                f,
                "inspector {inspector_id} policy-lock state and blocked resolution disagree"
            ),
            Self::WinnerDoesNotOutrank { inspector_id } => write!(
                f,
                "inspector {inspector_id} winner does not out-precede the candidates it overshadowed"
            ),
            Self::FallbackNotForced { inspector_id } => write!(
                f,
                "inspector {inspector_id} fallback is not forced by an unavailable higher-precedence candidate"
            ),
            Self::OverstatedPresentation {
                inspector_id,
                published,
                computed,
            } => write!(
                f,
                "inspector {inspector_id} publishes presentation {published} but the gate computes {computed}"
            ),
            Self::DuplicateDowngradeReason {
                inspector_id,
                reason,
            } => write!(
                f,
                "inspector {inspector_id} repeats downgrade reason {reason}"
            ),
            Self::DowngradeReasonsMismatch { inspector_id } => write!(
                f,
                "inspector {inspector_id} downgrade reasons disagree with the gate"
            ),
            Self::ResolutionPathMismatch {
                inspector_id,
                declared,
                required,
            } => write!(
                f,
                "inspector {inspector_id} records resolution {declared} but the gate requires {required}"
            ),
            Self::BlockedBeforeUseMismatch { inspector_id } => write!(
                f,
                "inspector {inspector_id} blocked-before-use flag disagrees with the gate"
            ),
            Self::MissingResolutionPath { inspector_id } => write!(
                f,
                "inspector {inspector_id} needs attention but offers no resolution path"
            ),
            Self::ResolvedOffersPath { inspector_id } => write!(
                f,
                "inspector {inspector_id} resolves cleanly but offers a resolution path"
            ),
            Self::TransparentInspectorNotWhole { inspector_id } => write!(
                f,
                "inspector {inspector_id} presents transparently but flags a state or carries a reason"
            ),
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve registry truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the inspectors"),
        }
    }
}

impl Error for M5PrecedenceInspectorViolation {}

/// Stable record-kind tag for [`M5PrecedenceInspectorSupportExport`].
pub const M5_PRECEDENCE_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_precedence_inspectors_support_export";

/// Support-export wrapper preserving the registry verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrecedenceInspectorSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw secret or hidden-policy material is excluded.
    pub raw_material_excluded: bool,
    /// Exact registry preserved by the export.
    pub registry: M5PrecedenceInspectors,
}

impl M5PrecedenceInspectorSupportExport {
    /// Whether the export preserves the same packet id and a clean registry.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_PRECEDENCE_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_PRECEDENCE_INSPECTOR_SCHEMA_VERSION
            && self.packet_id_ref == self.registry.packet_id
            && self.raw_material_excluded
            && self.registry.validate().is_empty()
    }
}

/// Loads the embedded precedence-inspector registry packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5PrecedenceInspectors`].
pub fn current_m5_precedence_inspectors() -> Result<M5PrecedenceInspectors, serde_json::Error> {
    serde_json::from_str(M5_PRECEDENCE_INSPECTOR_JSON)
}

#[cfg(test)]
mod tests;
