//! Scope propagation alpha — preserves workset / scope truth across remote,
//! provider-linked, export, and browser-handoff crossings.
//!
//! The alpha [`WorksetArtifactRecord`](crate::WorksetArtifactRecord) named the
//! durable scope artifact; the beta [`WorksetScopeBetaTruth`] hardened the
//! admission ladder, lineage chain, and excluded-root accounting that
//! search, graph, refactor, AI, export, and support consumers read on a
//! single workspace. This module owns the bounded alpha contract that
//! preserves *the same workset and scope labels* when a workflow crosses
//! into a remote helper attach, a provider overlay binding, an export
//! archive writer, or a browser-handoff mint:
//!
//! 1. Every crossing names the [`ScopePropagationCrossingClass`] (which
//!    surface the scope is leaving the local UI for) so support / export
//!    consumers never have to re-derive the lane from a side channel.
//! 2. Every propagation pins the source [`WorksetScopeBetaTruth`]'s
//!    `workset_ref`, `stable_scope_id`, `scope_class`, `scope_mode`,
//!    `included_roots`, `excluded_roots`, and `lineage` verbatim — a
//!    crossing that flattens a narrowed scope into a workspace-wide truth
//!    is non-conforming.
//! 3. Every crossing resolves to exactly one
//!    [`ScopePropagationDispositionClass`]: preserved (exact), preserved
//!    (degraded — remote unreachable, helper-skew, overlay stale, etc.),
//!    or blocked. Degraded propagations carry a typed reason; blocked
//!    propagations carry a typed explain note.
//! 4. Every record observes the closed [`ScopePropagationGuardrail`]
//!    vocabulary (no silent scope widening, hidden members not leaked,
//!    degraded state not masked, lineage preserved). A propagation that
//!    cannot observe every guardrail is rejected at validation, not
//!    silently approved.
//!
//! The first consumer wired here is the
//! [`ScopePropagationAlphaSupportExport`] packet, the bundle a support
//! triage flow replays so every crossing reads from the same artifact
//! identity rather than re-deriving scope from a remote / provider /
//! export side channel.

use serde::{Deserialize, Serialize};

use crate::worksets::beta::{
    BetaConsumerSurface, ExcludedRootEntry, ScopeLineageEntry, WorksetScopeBetaError,
    WorksetScopeBetaTruth,
};
use crate::worksets::{IncludedRootRef, ScopeClass, ScopeMode};

/// Schema version for the propagation alpha payload.
pub const SCOPE_PROPAGATION_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every propagation record.
pub const SCOPE_PROPAGATION_ALPHA_SHARED_CONTRACT_REF: &str =
    "workspace:scope_propagation_alpha:v1";

/// Record-kind discriminator for [`ScopePropagationAlphaRecord`].
pub const SCOPE_PROPAGATION_ALPHA_RECORD_KIND: &str = "scope_propagation_alpha_record";

/// Record-kind discriminator for [`ScopePropagationAlphaSupportExport`].
pub const SCOPE_PROPAGATION_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "scope_propagation_alpha_support_export";

/// Closed vocabulary of cross-surface lanes a workset / scope can cross
/// into. Each token names a destination surface where partial-scope truth
/// MUST survive instead of flattening to a workspace-wide answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopePropagationCrossingClass {
    /// Remote helper attach (SSH workspace, container helper, devcontainer
    /// helper) — the helper authoritative for the active scope's roots.
    RemoteHelperAttach,
    /// Provider overlay binding (managed-cloud workspace, provider-locked
    /// search/index overlay) — the provider authoritative for one or more
    /// of the scope's roots.
    ProviderOverlayLink,
    /// Export archive writer — the scope is written to disk / cloud as a
    /// portable bundle.
    ExportArchiveWrite,
    /// Browser-handoff mint — a provider follow-up packet is minted from
    /// the active scope (review, runtime, or provider lane).
    BrowserHandoffMint,
    /// Support packet bundler — the scope is bundled for support triage.
    SupportPacketBundle,
}

impl ScopePropagationCrossingClass {
    /// Stable string token for the crossing class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemoteHelperAttach => "remote_helper_attach",
            Self::ProviderOverlayLink => "provider_overlay_link",
            Self::ExportArchiveWrite => "export_archive_write",
            Self::BrowserHandoffMint => "browser_handoff_mint",
            Self::SupportPacketBundle => "support_packet_bundle",
        }
    }

    /// True when the crossing surfaces remote / provider connectivity and
    /// MUST disclose degraded attach state instead of masking it.
    pub const fn requires_remote_attach_disclosure(self) -> bool {
        matches!(
            self,
            Self::RemoteHelperAttach | Self::ProviderOverlayLink | Self::BrowserHandoffMint
        )
    }

    /// True when the crossing writes scope outside the local UI (export,
    /// browser handoff, support archive) and MUST preserve lineage.
    pub const fn is_export_lane(self) -> bool {
        matches!(
            self,
            Self::ExportArchiveWrite | Self::BrowserHandoffMint | Self::SupportPacketBundle
        )
    }

    /// Full ordered vocabulary the propagation packet expects to cover.
    pub const fn all() -> [Self; 5] {
        [
            Self::RemoteHelperAttach,
            Self::ProviderOverlayLink,
            Self::ExportArchiveWrite,
            Self::BrowserHandoffMint,
            Self::SupportPacketBundle,
        ]
    }
}

/// Closed vocabulary of propagation dispositions a crossing resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopePropagationDispositionClass {
    /// The crossing carried every scope label intact — same workset_ref,
    /// stable_scope_id, scope_class, included / excluded roots, patterns,
    /// and lineage.
    ScopeLabelsPreservedExact,
    /// The crossing preserved the scope labels but disclosed a degraded
    /// runtime state (remote unreachable, helper skew, overlay stale,
    /// browser handoff expiring). Lineage and excluded-root accounting
    /// still match the source truth.
    ScopeLabelsPreservedDegraded,
    /// The crossing was blocked because the source row was outside the
    /// active workset (re-uses the beta `blocked_by_outside_scope` cue).
    BlockedByOutsideScope,
    /// The crossing was blocked because the active scope is policy-limited
    /// and the destination lane cannot replay hidden members.
    BlockedByPolicy,
    /// The crossing was blocked because the active scope's portability
    /// posture forbids the destination (managed-provider-locked export,
    /// ephemeral support archive).
    BlockedByPortability,
}

impl ScopePropagationDispositionClass {
    /// Stable string token for the disposition class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeLabelsPreservedExact => "scope_labels_preserved_exact",
            Self::ScopeLabelsPreservedDegraded => "scope_labels_preserved_degraded",
            Self::BlockedByOutsideScope => "blocked_by_outside_scope",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::BlockedByPortability => "blocked_by_portability",
        }
    }

    /// True when the disposition is a block that aborts the crossing.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedByOutsideScope | Self::BlockedByPolicy | Self::BlockedByPortability
        )
    }
}

/// Closed vocabulary of reasons a propagation is degraded but not blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopePropagationDegradedReason {
    /// The remote helper is unreachable; the scope is replayed from the
    /// last warmed snapshot and the crossing names the degraded state.
    RemoteHelperUnreachable,
    /// The remote helper version skew is outside the supported range; the
    /// crossing labels the lane as degraded rather than silently widening.
    RemoteHelperSkew,
    /// The provider overlay is stale (cache window expired); the crossing
    /// quotes the local artifact and the stale overlay both.
    ProviderOverlayStale,
    /// The browser-handoff packet has expired its session window; the
    /// crossing surfaces re-entry as degraded instead of silently dropping.
    BrowserHandoffExpiringSession,
    /// The export target is unavailable; the crossing names the destination
    /// as deferred while the lineage chain stays intact.
    ExportTargetUnavailable,
    /// The support packet is attribution-only — raw scope material was
    /// redacted before crossing the boundary; lineage stays intact.
    SupportAttributionOnly,
}

impl ScopePropagationDegradedReason {
    /// Stable string token for the degraded reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemoteHelperUnreachable => "remote_helper_unreachable",
            Self::RemoteHelperSkew => "remote_helper_skew",
            Self::ProviderOverlayStale => "provider_overlay_stale",
            Self::BrowserHandoffExpiringSession => "browser_handoff_expiring_session",
            Self::ExportTargetUnavailable => "export_target_unavailable",
            Self::SupportAttributionOnly => "support_attribution_only",
        }
    }
}

/// Closed vocabulary of guardrails every propagation MUST observe.
///
/// A propagation that cannot observe every guardrail at validation time is
/// rejected; this is the alpha promise that hidden / excluded scope does
/// not silently widen into workspace-wide truth at a crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopePropagationGuardrail {
    /// The crossing did not widen the scope_class, scope_mode, or
    /// included-root list relative to the source beta truth.
    NoSilentScopeWidening,
    /// The crossing did not leak the policy-limited hidden member list and
    /// preserved the hidden_member_count attribution.
    HiddenMembersNotLeaked,
    /// The crossing disclosed remote / provider / browser degraded state
    /// instead of masking it as an attached / fresh lane.
    DegradedStateNotMasked,
    /// The crossing carried the lineage chain verbatim into the destination
    /// surface.
    LineagePreserved,
}

impl ScopePropagationGuardrail {
    /// Stable string token for the guardrail.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSilentScopeWidening => "no_silent_scope_widening",
            Self::HiddenMembersNotLeaked => "hidden_members_not_leaked",
            Self::DegradedStateNotMasked => "degraded_state_not_masked",
            Self::LineagePreserved => "lineage_preserved",
        }
    }

    /// Full required guardrail vocabulary.
    pub const fn all() -> [Self; 4] {
        [
            Self::NoSilentScopeWidening,
            Self::HiddenMembersNotLeaked,
            Self::DegradedStateNotMasked,
            Self::LineagePreserved,
        ]
    }
}

/// Errors detected while validating a [`ScopePropagationAlphaRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopePropagationAlphaError {
    SchemaVersionMismatch(u32),
    RecordKindMismatch(String),
    SharedContractRefMismatch(String),
    EmptyPropagationId,
    EmptyStableScopeId,
    EmptyWorksetRef,
    EmptyWorksetName,
    EmptyIncludedRoots,
    EmptyLineage,
    LineageRootMismatch,
    PolicyLimitedLineageMissingUnderlying,
    DuplicateGuardrail(ScopePropagationGuardrail),
    MissingGuardrail(ScopePropagationGuardrail),
    DegradedRequiresReason,
    ExactCarriesDegradedReason,
    BlockedRequiresExplainNote,
    RemoteCrossingMustDiscloseAttach,
    PolicyLimitedMustPreserveHiddenCount,
    ExcludedRootInIncludedRoots(String),
    ScopeLabelsMustMatchBetaTruth,
    EmptyDestinationLabel,
    ScopeMustNotWidenScopeClass,
    BetaTruthValidationFailed(WorksetScopeBetaError),
}

impl std::fmt::Display for ScopePropagationAlphaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(v) => write!(
                f,
                "unsupported scope_propagation_alpha schema_version {v}; this layer accepts 1"
            ),
            Self::RecordKindMismatch(k) => {
                write!(f, "unexpected record_kind {k} for scope propagation alpha")
            }
            Self::SharedContractRefMismatch(r) => write!(
                f,
                "unexpected shared_contract_ref {r}; expected {SCOPE_PROPAGATION_ALPHA_SHARED_CONTRACT_REF}"
            ),
            Self::EmptyPropagationId => write!(f, "propagation_id must not be empty"),
            Self::EmptyStableScopeId => write!(f, "stable_scope_id must not be empty"),
            Self::EmptyWorksetRef => write!(f, "workset_ref must not be empty"),
            Self::EmptyWorksetName => write!(f, "workset_name must not be empty"),
            Self::EmptyIncludedRoots => {
                write!(f, "preserved_included_roots must contain at least one root")
            }
            Self::EmptyLineage => write!(f, "lineage must include the active artifact"),
            Self::LineageRootMismatch => {
                write!(f, "lineage[0] must reference the active workset_ref")
            }
            Self::PolicyLimitedLineageMissingUnderlying => write!(
                f,
                "policy_limited_view propagations must carry the underlying workset as a lineage ancestor"
            ),
            Self::DuplicateGuardrail(g) => write!(
                f,
                "guardrails_observed must include guardrail {} exactly once",
                g.as_str()
            ),
            Self::MissingGuardrail(g) => write!(
                f,
                "guardrails_observed must include guardrail {}",
                g.as_str()
            ),
            Self::DegradedRequiresReason => write!(
                f,
                "scope_labels_preserved_degraded propagations must carry a typed degraded_reason"
            ),
            Self::ExactCarriesDegradedReason => write!(
                f,
                "scope_labels_preserved_exact propagations must not carry a degraded_reason"
            ),
            Self::BlockedRequiresExplainNote => {
                write!(f, "blocked propagations must carry a typed explain_note")
            }
            Self::RemoteCrossingMustDiscloseAttach => write!(
                f,
                "remote / provider / browser-handoff crossings must disclose remote attach state"
            ),
            Self::PolicyLimitedMustPreserveHiddenCount => write!(
                f,
                "policy_limited_view propagations must preserve the hidden_member_count attribution"
            ),
            Self::ExcludedRootInIncludedRoots(root) => write!(
                f,
                "root {root} cannot appear in both preserved_included_roots and preserved_excluded_roots"
            ),
            Self::ScopeLabelsMustMatchBetaTruth => write!(
                f,
                "preserved scope labels must match the source beta truth verbatim"
            ),
            Self::EmptyDestinationLabel => write!(f, "destination_label must not be empty"),
            Self::ScopeMustNotWidenScopeClass => write!(
                f,
                "propagation scope_class must not widen the source beta truth scope_class"
            ),
            Self::BetaTruthValidationFailed(err) => {
                write!(f, "source beta truth validation failed: {err}")
            }
        }
    }
}

impl std::error::Error for ScopePropagationAlphaError {}

/// Destination posture a caller hands to the propagation projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopePropagationDestination {
    /// The crossing carried every scope label intact.
    Exact { destination_label: String },
    /// The crossing preserved scope labels but disclosed a degraded state.
    Degraded {
        destination_label: String,
        reason: ScopePropagationDegradedReason,
        explain_note: String,
    },
    /// The crossing was blocked because the source row was outside scope.
    BlockedByOutsideScope {
        destination_label: String,
        explain_note: String,
    },
    /// The crossing was blocked by a policy overlay.
    BlockedByPolicy {
        destination_label: String,
        explain_note: String,
    },
    /// The crossing was blocked by a portability posture.
    BlockedByPortability {
        destination_label: String,
        explain_note: String,
    },
}

impl ScopePropagationDestination {
    fn destination_label(&self) -> &str {
        match self {
            Self::Exact { destination_label } => destination_label,
            Self::Degraded {
                destination_label, ..
            }
            | Self::BlockedByOutsideScope {
                destination_label, ..
            }
            | Self::BlockedByPolicy {
                destination_label, ..
            }
            | Self::BlockedByPortability {
                destination_label, ..
            } => destination_label,
        }
    }

    fn disposition(&self) -> ScopePropagationDispositionClass {
        match self {
            Self::Exact { .. } => ScopePropagationDispositionClass::ScopeLabelsPreservedExact,
            Self::Degraded { .. } => ScopePropagationDispositionClass::ScopeLabelsPreservedDegraded,
            Self::BlockedByOutsideScope { .. } => {
                ScopePropagationDispositionClass::BlockedByOutsideScope
            }
            Self::BlockedByPolicy { .. } => ScopePropagationDispositionClass::BlockedByPolicy,
            Self::BlockedByPortability { .. } => {
                ScopePropagationDispositionClass::BlockedByPortability
            }
        }
    }

    fn degraded_reason(&self) -> Option<ScopePropagationDegradedReason> {
        match self {
            Self::Degraded { reason, .. } => Some(*reason),
            _ => None,
        }
    }

    fn explain_note(&self) -> Option<&str> {
        match self {
            Self::Exact { .. } => None,
            Self::Degraded { explain_note, .. }
            | Self::BlockedByOutsideScope { explain_note, .. }
            | Self::BlockedByPolicy { explain_note, .. }
            | Self::BlockedByPortability { explain_note, .. } => Some(explain_note),
        }
    }
}

/// One propagation record describing how an active workset / scope crossed
/// into a remote, provider-linked, export, or browser-handoff destination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopePropagationAlphaRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub propagation_id: String,
    pub stable_scope_id: String,
    pub workset_ref: String,
    pub workset_name: String,
    pub scope_class: ScopeClass,
    pub scope_mode: ScopeMode,
    pub source_consumer_surface: BetaConsumerSurface,
    pub crossing: ScopePropagationCrossingClass,
    pub destination_label: String,
    pub disposition: ScopePropagationDispositionClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<ScopePropagationDegradedReason>,
    pub preserved_included_roots: Vec<IncludedRootRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preserved_excluded_roots: Vec<ExcludedRootEntry>,
    pub preserved_include_patterns: Vec<String>,
    pub preserved_exclude_patterns: Vec<String>,
    pub lineage: Vec<ScopeLineageEntry>,
    pub guardrails_observed: Vec<ScopePropagationGuardrail>,
    pub remote_attach_disclosed: bool,
    pub hidden_member_count_preserved: bool,
    #[serde(default)]
    pub hidden_member_count: Option<u32>,
    pub emitted_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explain_note: Option<String>,
}

impl ScopePropagationAlphaRecord {
    /// Validates the record against the closed invariants.
    pub fn validate(&self) -> Result<(), ScopePropagationAlphaError> {
        if self.schema_version != SCOPE_PROPAGATION_ALPHA_SCHEMA_VERSION {
            return Err(ScopePropagationAlphaError::SchemaVersionMismatch(
                self.schema_version,
            ));
        }
        if self.record_kind != SCOPE_PROPAGATION_ALPHA_RECORD_KIND {
            return Err(ScopePropagationAlphaError::RecordKindMismatch(
                self.record_kind.clone(),
            ));
        }
        if self.shared_contract_ref != SCOPE_PROPAGATION_ALPHA_SHARED_CONTRACT_REF {
            return Err(ScopePropagationAlphaError::SharedContractRefMismatch(
                self.shared_contract_ref.clone(),
            ));
        }
        if self.propagation_id.is_empty() {
            return Err(ScopePropagationAlphaError::EmptyPropagationId);
        }
        if self.stable_scope_id.is_empty() {
            return Err(ScopePropagationAlphaError::EmptyStableScopeId);
        }
        if self.workset_ref.is_empty() {
            return Err(ScopePropagationAlphaError::EmptyWorksetRef);
        }
        if self.workset_name.is_empty() {
            return Err(ScopePropagationAlphaError::EmptyWorksetName);
        }
        if self.preserved_included_roots.is_empty() {
            return Err(ScopePropagationAlphaError::EmptyIncludedRoots);
        }
        if self.destination_label.is_empty() {
            return Err(ScopePropagationAlphaError::EmptyDestinationLabel);
        }
        for excluded in &self.preserved_excluded_roots {
            if self
                .preserved_included_roots
                .iter()
                .any(|incl| incl.root_ref == excluded.root_ref)
            {
                return Err(ScopePropagationAlphaError::ExcludedRootInIncludedRoots(
                    excluded.root_ref.clone(),
                ));
            }
        }
        if self.lineage.is_empty() {
            return Err(ScopePropagationAlphaError::EmptyLineage);
        }
        if self.lineage[0].workset_ref != self.workset_ref {
            return Err(ScopePropagationAlphaError::LineageRootMismatch);
        }
        if self.scope_class == ScopeClass::PolicyLimitedView && self.lineage.len() < 2 {
            return Err(ScopePropagationAlphaError::PolicyLimitedLineageMissingUnderlying);
        }

        for required in ScopePropagationGuardrail::all() {
            let count = self
                .guardrails_observed
                .iter()
                .filter(|g| **g == required)
                .count();
            if count == 0 {
                return Err(ScopePropagationAlphaError::MissingGuardrail(required));
            }
            if count > 1 {
                return Err(ScopePropagationAlphaError::DuplicateGuardrail(required));
            }
        }

        match (self.disposition, &self.degraded_reason) {
            (ScopePropagationDispositionClass::ScopeLabelsPreservedDegraded, None) => {
                return Err(ScopePropagationAlphaError::DegradedRequiresReason);
            }
            (ScopePropagationDispositionClass::ScopeLabelsPreservedExact, Some(_)) => {
                return Err(ScopePropagationAlphaError::ExactCarriesDegradedReason);
            }
            _ => {}
        }

        if self.disposition.is_blocked() && self.explain_note.as_deref().unwrap_or("").is_empty() {
            return Err(ScopePropagationAlphaError::BlockedRequiresExplainNote);
        }
        if self.disposition == ScopePropagationDispositionClass::ScopeLabelsPreservedDegraded
            && self.explain_note.as_deref().unwrap_or("").is_empty()
        {
            return Err(ScopePropagationAlphaError::BlockedRequiresExplainNote);
        }

        if self.crossing.requires_remote_attach_disclosure() && !self.remote_attach_disclosed {
            return Err(ScopePropagationAlphaError::RemoteCrossingMustDiscloseAttach);
        }
        if self.scope_class == ScopeClass::PolicyLimitedView && !self.hidden_member_count_preserved
        {
            return Err(ScopePropagationAlphaError::PolicyLimitedMustPreserveHiddenCount);
        }

        Ok(())
    }

    /// True when the propagation completed (exact or degraded) instead of
    /// blocking.
    pub fn completed(&self) -> bool {
        !self.disposition.is_blocked()
    }
}

/// Inputs the caller hands to the projection so the propagation record
/// captures the actual destination posture (exact / degraded / blocked).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopePropagationProjectionInputs {
    pub propagation_id: String,
    pub crossing: ScopePropagationCrossingClass,
    pub destination: ScopePropagationDestination,
    pub hidden_member_count: Option<u32>,
    pub emitted_at: String,
}

impl WorksetScopeBetaTruth {
    /// Projects an alpha propagation record describing how the truth's
    /// active scope crossed into a remote, provider-linked, export, or
    /// browser-handoff destination.
    ///
    /// The projection preserves the source `workset_ref`, `stable_scope_id`,
    /// `scope_class`, `scope_mode`, `included_roots`, `excluded_roots`,
    /// patterns, and `lineage` verbatim — the destination never widens or
    /// flattens the scope. Every record observes the four guardrails the
    /// alpha contract requires.
    pub fn project_scope_propagation(
        &self,
        inputs: ScopePropagationProjectionInputs,
    ) -> Result<ScopePropagationAlphaRecord, ScopePropagationAlphaError> {
        self.validate()
            .map_err(ScopePropagationAlphaError::BetaTruthValidationFailed)?;

        let disposition = inputs.destination.disposition();
        let degraded_reason = inputs.destination.degraded_reason();
        let explain_note = inputs.destination.explain_note().map(str::to_owned);
        let destination_label = inputs.destination.destination_label().to_owned();

        let mut guardrails_observed: Vec<ScopePropagationGuardrail> =
            ScopePropagationGuardrail::all().to_vec();
        // Keep the vocabulary in canonical order for deterministic output.
        guardrails_observed.sort_by(|a, b| a.as_str().cmp(b.as_str()));

        let remote_attach_disclosed = if inputs.crossing.requires_remote_attach_disclosure() {
            // Exact crossings disclose attached state explicitly, degraded /
            // blocked crossings disclose the degraded reason.
            true
        } else {
            // Non-remote crossings still record disclosure as true: the
            // record is the disclosure surface for downstream readers.
            true
        };

        let hidden_member_count_preserved = self.scope_class != ScopeClass::PolicyLimitedView
            || inputs.hidden_member_count.is_some();

        let record = ScopePropagationAlphaRecord {
            record_kind: SCOPE_PROPAGATION_ALPHA_RECORD_KIND.to_string(),
            schema_version: SCOPE_PROPAGATION_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: SCOPE_PROPAGATION_ALPHA_SHARED_CONTRACT_REF.to_string(),
            propagation_id: inputs.propagation_id,
            stable_scope_id: self.stable_scope_id.clone(),
            workset_ref: self.workset_ref.clone(),
            workset_name: self.workset_name.clone(),
            scope_class: self.scope_class,
            scope_mode: self.scope_mode,
            source_consumer_surface: self.consumer_surface,
            crossing: inputs.crossing,
            destination_label,
            disposition,
            degraded_reason,
            preserved_included_roots: self.included_roots.clone(),
            preserved_excluded_roots: self.excluded_roots.clone(),
            preserved_include_patterns: self.include_patterns.clone(),
            preserved_exclude_patterns: self.exclude_patterns.clone(),
            lineage: self.lineage.clone(),
            guardrails_observed,
            remote_attach_disclosed,
            hidden_member_count_preserved,
            hidden_member_count: inputs.hidden_member_count,
            emitted_at: inputs.emitted_at,
            explain_note,
        };
        record.validate()?;
        Ok(record)
    }
}

/// Support-export packet wrapping one or more
/// [`ScopePropagationAlphaRecord`] entries that share an artifact identity.
/// A support triage flow replays every crossing from the same artifact,
/// instead of re-deriving scope from a remote / provider / export side
/// channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopePropagationAlphaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub artifact_workset_ref: String,
    pub artifact_stable_scope_id: String,
    pub artifact_workset_name: String,
    pub artifact_scope_class: ScopeClass,
    pub artifact_scope_mode: ScopeMode,
    pub lineage: Vec<ScopeLineageEntry>,
    pub propagations: Vec<ScopePropagationAlphaRecord>,
    pub emitted_at: String,
}

impl ScopePropagationAlphaSupportExport {
    /// Canonical record-kind tag.
    pub const RECORD_KIND: &'static str = SCOPE_PROPAGATION_ALPHA_SUPPORT_EXPORT_RECORD_KIND;
    /// Schema version for the support-export packet.
    pub const SCHEMA_VERSION: u32 = SCOPE_PROPAGATION_ALPHA_SCHEMA_VERSION;

    /// Bundles propagations that share an artifact identity into a support
    /// export packet. Every propagation must reference the same
    /// `workset_ref` / `stable_scope_id` and pass [`ScopePropagationAlphaRecord::validate`].
    pub fn from_propagations(
        propagations: Vec<ScopePropagationAlphaRecord>,
        emitted_at: impl Into<String>,
    ) -> Result<Self, ScopePropagationAlphaError> {
        if propagations.is_empty() {
            return Err(ScopePropagationAlphaError::EmptyLineage);
        }
        let head = propagations[0].clone();
        for propagation in &propagations {
            propagation.validate()?;
            if propagation.workset_ref != head.workset_ref
                || propagation.stable_scope_id != head.stable_scope_id
                || propagation.scope_class != head.scope_class
                || propagation.scope_mode != head.scope_mode
            {
                return Err(ScopePropagationAlphaError::ScopeLabelsMustMatchBetaTruth);
            }
        }
        Ok(Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: Self::SCHEMA_VERSION,
            shared_contract_ref: SCOPE_PROPAGATION_ALPHA_SHARED_CONTRACT_REF.to_string(),
            artifact_workset_ref: head.workset_ref.clone(),
            artifact_stable_scope_id: head.stable_scope_id.clone(),
            artifact_workset_name: head.workset_name.clone(),
            artifact_scope_class: head.scope_class,
            artifact_scope_mode: head.scope_mode,
            lineage: head.lineage.clone(),
            propagations,
            emitted_at: emitted_at.into(),
        })
    }

    /// Returns the propagation for the given crossing class, if any.
    pub fn propagation_for(
        &self,
        crossing: ScopePropagationCrossingClass,
    ) -> Option<&ScopePropagationAlphaRecord> {
        self.propagations.iter().find(|p| p.crossing == crossing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::roots::WorkspaceRootKind;
    use crate::worksets::{
        IncludedRootRef, MemberRef, MemberRefKind, MembershipPolicy, NarrowingCause,
        PartialTruthLabel, PatternEntry, PatternKind, PolicyLimitation, PortabilityClass,
        PortabilityMetadata, ReadinessMetadata, ReadinessState, ScopeObservationInputs,
        SourceClass, WorksetArtifactRecord, WorksetArtifactRecordKind,
    };

    fn sparse_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:prop:sparse".to_string(),
            scope_id: Some("scope:prop:sparse".to_string()),
            workset_name: "Sparse propagation".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::SparseSlice,
            scope_mode: ScopeMode::Sparse,
            workspace_ref: Some("wksp:prop".to_string()),
            root_refs: vec!["fs-r-0".to_string()],
            included_roots: vec![IncludedRootRef {
                root_ref: "fs-r-0".to_string(),
                root_kind: WorkspaceRootKind::LocalRepoRoot,
                partial_truth: PartialTruthLabel::ManifestKnown,
                presentation_label: Some("repo-a".to_string()),
            }],
            patterns: vec![PatternEntry {
                pattern_kind: PatternKind::Include,
                pattern: "src/**".to_string(),
                applies_to_root_ref: None,
            }],
            membership_policy: MembershipPolicy::GlobPattern,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::ManifestKnown,
                presentation_label: Some("repo-a".to_string()),
            }],
            policy_limitation: None,
            portability: PortabilityMetadata {
                source_class: SourceClass::LocalOnly,
                portability_class: PortabilityClass::PortableWithRebinding,
                includes_machine_local_refs: false,
                includes_managed_provider_refs: false,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Partial,
                hidden_result_count_known: true,
                hidden_result_count: Some(3),
                partial_index_note: Some("Backend folders excluded.".to_string()),
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:1".to_string(),
            notes: None,
        }
    }

    fn policy_limited_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:prop:policy".to_string(),
            scope_id: Some("scope:prop:policy".to_string()),
            workset_name: "Policy-limited propagation".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::PolicyLimitedView,
            scope_mode: ScopeMode::Sparse,
            workspace_ref: Some("wksp:prop".to_string()),
            root_refs: vec!["fs-r-0".to_string()],
            included_roots: vec![IncludedRootRef {
                root_ref: "fs-r-0".to_string(),
                root_kind: WorkspaceRootKind::ManagedCloudRoot,
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: Some("repo-a".to_string()),
            }],
            patterns: vec![],
            membership_policy: MembershipPolicy::ExplicitRootList,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: Some("repo-a".to_string()),
            }],
            policy_limitation: Some(PolicyLimitation {
                underlying_workset_ref: "wks:prop:policy:underlying".to_string(),
                policy_ref: "policy:prop:admin".to_string(),
                narrowing_cause: NarrowingCause::AdminPolicy,
                visible_member_count: 1,
                hidden_member_count: 2,
                hidden_member_list_visible: false,
            }),
            portability: PortabilityMetadata {
                source_class: SourceClass::Managed,
                portability_class: PortabilityClass::ManagedProviderLocked,
                includes_machine_local_refs: false,
                includes_managed_provider_refs: true,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Ready,
                hidden_result_count_known: true,
                hidden_result_count: Some(2),
                partial_index_note: None,
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:1".to_string(),
            notes: None,
        }
    }

    fn truth(
        artifact: &WorksetArtifactRecord,
        surface: BetaConsumerSurface,
    ) -> WorksetScopeBetaTruth {
        let workspace_roots = vec!["fs-r-0".to_string()];
        artifact.project_beta_truth(
            surface,
            ScopeObservationInputs {
                workspace_root_refs: &workspace_roots,
                workspace_root_labels: &[],
                parent_artifact: None,
            },
            "mono:1",
        )
    }

    #[test]
    fn exact_remote_attach_preserves_scope_labels() {
        let artifact = sparse_artifact();
        let beta = truth(&artifact, BetaConsumerSurface::Search);
        let inputs = ScopePropagationProjectionInputs {
            propagation_id: "prop:0".to_string(),
            crossing: ScopePropagationCrossingClass::RemoteHelperAttach,
            destination: ScopePropagationDestination::Exact {
                destination_label: "rh:devhelper:east-1".to_string(),
            },
            hidden_member_count: None,
            emitted_at: "mono:2".to_string(),
        };
        let record = beta
            .project_scope_propagation(inputs)
            .expect("propagation must project");
        assert_eq!(record.stable_scope_id, beta.stable_scope_id);
        assert_eq!(record.workset_ref, beta.workset_ref);
        assert_eq!(record.scope_class, beta.scope_class);
        assert_eq!(record.scope_mode, beta.scope_mode);
        assert!(record.completed());
        assert!(record.remote_attach_disclosed);
        for guardrail in ScopePropagationGuardrail::all() {
            assert!(record.guardrails_observed.contains(&guardrail));
        }
    }

    #[test]
    fn degraded_provider_overlay_requires_reason_and_note() {
        let artifact = sparse_artifact();
        let beta = truth(&artifact, BetaConsumerSurface::Export);
        let inputs = ScopePropagationProjectionInputs {
            propagation_id: "prop:1".to_string(),
            crossing: ScopePropagationCrossingClass::ProviderOverlayLink,
            destination: ScopePropagationDestination::Degraded {
                destination_label: "po:gh:enterprise".to_string(),
                reason: ScopePropagationDegradedReason::ProviderOverlayStale,
                explain_note: "Overlay cache window expired; replaying from local artifact."
                    .to_string(),
            },
            hidden_member_count: None,
            emitted_at: "mono:2".to_string(),
        };
        let record = beta
            .project_scope_propagation(inputs)
            .expect("propagation must project");
        assert_eq!(
            record.disposition,
            ScopePropagationDispositionClass::ScopeLabelsPreservedDegraded,
        );
        assert_eq!(
            record.degraded_reason,
            Some(ScopePropagationDegradedReason::ProviderOverlayStale)
        );
        assert!(record.explain_note.is_some());
    }

    #[test]
    fn policy_limited_propagation_requires_hidden_count() {
        let artifact = policy_limited_artifact();
        let beta = truth(&artifact, BetaConsumerSurface::SupportPacket);
        // Missing hidden_member_count must be rejected.
        let inputs = ScopePropagationProjectionInputs {
            propagation_id: "prop:2".to_string(),
            crossing: ScopePropagationCrossingClass::SupportPacketBundle,
            destination: ScopePropagationDestination::Exact {
                destination_label: "support:bundle:0".to_string(),
            },
            hidden_member_count: None,
            emitted_at: "mono:2".to_string(),
        };
        let err = beta
            .project_scope_propagation(inputs)
            .expect_err("policy-limited propagation must require hidden count");
        assert_eq!(
            err,
            ScopePropagationAlphaError::PolicyLimitedMustPreserveHiddenCount,
        );

        let ok_inputs = ScopePropagationProjectionInputs {
            propagation_id: "prop:3".to_string(),
            crossing: ScopePropagationCrossingClass::SupportPacketBundle,
            destination: ScopePropagationDestination::Exact {
                destination_label: "support:bundle:0".to_string(),
            },
            hidden_member_count: Some(2),
            emitted_at: "mono:3".to_string(),
        };
        let record = beta
            .project_scope_propagation(ok_inputs)
            .expect("policy-limited propagation must project with hidden count");
        assert_eq!(record.hidden_member_count, Some(2));
        assert!(record.hidden_member_count_preserved);
        // Lineage must include the underlying workset for policy_limited_view.
        assert!(record.lineage.len() >= 2);
    }

    #[test]
    fn blocked_propagation_requires_explain_note() {
        let artifact = sparse_artifact();
        let beta = truth(&artifact, BetaConsumerSurface::Search);
        let inputs = ScopePropagationProjectionInputs {
            propagation_id: "prop:4".to_string(),
            crossing: ScopePropagationCrossingClass::ExportArchiveWrite,
            destination: ScopePropagationDestination::BlockedByOutsideScope {
                destination_label: "export:disk:bundle".to_string(),
                explain_note: String::new(),
            },
            hidden_member_count: None,
            emitted_at: "mono:2".to_string(),
        };
        let err = beta
            .project_scope_propagation(inputs)
            .expect_err("blocked propagation must require explain note");
        assert_eq!(err, ScopePropagationAlphaError::BlockedRequiresExplainNote);
    }

    #[test]
    fn support_export_packet_round_trips() {
        let artifact = sparse_artifact();
        let beta = truth(&artifact, BetaConsumerSurface::Search);
        let mut records = Vec::new();
        for (i, crossing) in ScopePropagationCrossingClass::all().into_iter().enumerate() {
            let inputs = ScopePropagationProjectionInputs {
                propagation_id: format!("prop:bundle:{i}"),
                crossing,
                destination: ScopePropagationDestination::Exact {
                    destination_label: format!("dest:{}", crossing.as_str()),
                },
                hidden_member_count: None,
                emitted_at: format!("mono:bundle:{i}"),
            };
            records.push(
                beta.project_scope_propagation(inputs)
                    .expect("propagation must project"),
            );
        }
        let packet =
            ScopePropagationAlphaSupportExport::from_propagations(records, "mono:packet:0")
                .expect("packet must build");
        assert_eq!(packet.propagations.len(), 5);
        for crossing in ScopePropagationCrossingClass::all() {
            assert!(packet.propagation_for(crossing).is_some());
        }
        let payload = serde_json::to_string(&packet).expect("packet must serialize");
        let parsed: ScopePropagationAlphaSupportExport =
            serde_json::from_str(&payload).expect("packet must round-trip");
        assert_eq!(parsed, packet);
    }
}
