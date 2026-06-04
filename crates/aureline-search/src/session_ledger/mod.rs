//! Durable search-session ledger, saved-query privacy, and export packets.
//!
//! This module is the search-owned runtime contract for turning a live
//! [`crate::SearchQuerySession`] and planner result set into durable local
//! continuity records, saved-query records, and redaction-safe packets for
//! support or documentation handoff. The implementation deliberately consumes
//! [`crate::SearchPlannerOutput`] and [`crate::PlannedResultSet`] so saved
//! queries and exports do not fork the planner's scope, readiness, source, or
//! result identity vocabulary.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::counts::SearchScopeCountsRecord;
use crate::lexical::ScopeClass;
use crate::planner::{
    PlannedResultSet, PlannerPathReadiness, SearchPlannerOutput, SEARCH_PLANNER_ALPHA_VERSION,
};
use crate::query_session::{stable_query_hash, QueryTextMode, SearchQuerySession, SearchSurface};

/// Schema version for the alpha saved-query and export-packet records.
pub const SAVED_QUERY_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Privacy posture applied when a query session becomes durable or exportable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryPrivacyClass {
    /// Raw query text may remain only in the local profile or session ledger.
    LocalOnlyPrivate,
    /// Raw text is removed at the workspace/share boundary and a hash may remain.
    WorkspaceSharedRedacted,
    /// Raw text is removed from support exports and a hash may remain.
    SupportExportRedacted,
    /// Raw text and derived hash material are withheld by policy.
    PolicyWithheld,
}

impl SavedQueryPrivacyClass {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyPrivate => "local_only_private",
            Self::WorkspaceSharedRedacted => "workspace_shared_redacted",
            Self::SupportExportRedacted => "support_export_redacted",
            Self::PolicyWithheld => "policy_withheld",
        }
    }

    /// True when serialized records may carry raw query text.
    pub const fn permits_raw_query_text(self) -> bool {
        matches!(self, Self::LocalOnlyPrivate)
    }

    fn for_destination(self, destination: SearchExportDestination) -> Self {
        match destination {
            SearchExportDestination::LocalReplay => self,
            SearchExportDestination::SupportBundle => match self {
                Self::PolicyWithheld => Self::PolicyWithheld,
                _ => Self::SupportExportRedacted,
            },
            SearchExportDestination::DocsHandoff => match self {
                Self::PolicyWithheld => Self::PolicyWithheld,
                _ => Self::WorkspaceSharedRedacted,
            },
        }
    }
}

/// Source class that explains who or what created a saved query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQuerySourceClass {
    /// Query was saved by the local user.
    UserAuthored,
    /// Query was committed or shipped by the repository.
    RepoProvided,
    /// Query was supplied by policy or an administrator.
    PolicyProvided,
    /// Query was shared by a team or workspace artifact.
    TeamShared,
    /// Query was captured from a support-safe evidence flow.
    SupportCaptured,
    /// Query was promoted from local query history.
    SessionHistory,
}

impl SavedQuerySourceClass {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthored => "user_authored",
            Self::RepoProvided => "repo_provided",
            Self::PolicyProvided => "policy_provided",
            Self::TeamShared => "team_shared",
            Self::SupportCaptured => "support_captured",
            Self::SessionHistory => "session_history",
        }
    }
}

/// Sharing policy attached to a durable saved query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQuerySharePolicy {
    /// Query remains local and is not shareable.
    LocalOnlyNoShare,
    /// Workspace sharing requires an explicit user action.
    WorkspaceShareExplicit,
    /// Only a redacted support packet may be exported.
    SupportExportRedactedOnly,
    /// Sharing is disabled by policy.
    ShareDisabledByPolicy,
}

impl SavedQuerySharePolicy {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyNoShare => "local_only_no_share",
            Self::WorkspaceShareExplicit => "workspace_share_explicit",
            Self::SupportExportRedactedOnly => "support_export_redacted_only",
            Self::ShareDisabledByPolicy => "share_disabled_by_policy",
        }
    }
}

/// How query material appears after privacy projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMaterialDisposition {
    /// Raw text remains in a local-only record.
    RawRetainedLocalOnly,
    /// Raw text was removed and a deterministic hash remains.
    RawRedactedHashRetained,
    /// Raw text and hash material were withheld by policy.
    RawWithheldByPolicy,
    /// The source session did not provide raw or hash material.
    NoQueryMaterialProvided,
}

impl QueryMaterialDisposition {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawRetainedLocalOnly => "raw_retained_local_only",
            Self::RawRedactedHashRetained => "raw_redacted_hash_retained",
            Self::RawWithheldByPolicy => "raw_withheld_by_policy",
            Self::NoQueryMaterialProvided => "no_query_material_provided",
        }
    }
}

/// Destination class for a search export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchExportDestination {
    /// Local replay or local-history continuity packet.
    LocalReplay,
    /// Support-bundle packet with redaction-safe query material.
    SupportBundle,
    /// Documentation/help handoff packet with redaction-safe query material.
    DocsHandoff,
}

impl SearchExportDestination {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReplay => "local_replay",
            Self::SupportBundle => "support_bundle",
            Self::DocsHandoff => "docs_handoff",
        }
    }

    /// True when the packet must not serialize raw query text.
    pub const fn requires_redacted_query_text(self) -> bool {
        !matches!(self, Self::LocalReplay)
    }
}

/// Redaction state attached to an exported search packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchPacketRedactionState {
    /// Raw query text is retained because the packet is local-only.
    RawQueryLocalOnly,
    /// Query text is represented by hash material only.
    QueryHashOnly,
    /// Query text and hash material are omitted by policy.
    QueryMaterialOmittedByPolicy,
}

impl SearchPacketRedactionState {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawQueryLocalOnly => "raw_query_local_only",
            Self::QueryHashOnly => "query_hash_only",
            Self::QueryMaterialOmittedByPolicy => "query_material_omitted_by_policy",
        }
    }
}

/// Live-versus-captured truth carried by a search export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchExportSnapshotTruth {
    /// Packet represents a live rerun against the current scope.
    LiveRerun,
    /// Packet represents a captured snapshot of a prior result set.
    CapturedSnapshot,
    /// Packet was reopened after the captured scope changed.
    ScopeChangedSinceCapture,
}

impl SearchExportSnapshotTruth {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveRerun => "live_rerun",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::ScopeChangedSinceCapture => "scope_changed_since_capture",
        }
    }
}

/// Validation finding emitted by privacy and export checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryValidationFindingKind {
    /// Raw text exists on a record whose privacy class forbids it.
    RawTextPresentForExportablePrivacy,
    /// A policy-withheld record still carries query hash material.
    PolicyWithheldCarriesQueryHash,
    /// Sharing policy conflicts with the record privacy class.
    SharePolicyConflictsWithPrivacy,
    /// Scope identity is missing from a durable record.
    MissingScopeIdentity,
    /// Result set does not belong to the supplied query session.
    SessionResultSetMismatch,
    /// Export packet contains raw text for a non-local destination.
    UnsafeRawTextInExportPacket,
    /// Export packet lost omitted/truncated count disclosure.
    MissingOmittedOrTruncatedDisclosure,
}

impl SavedQueryValidationFindingKind {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawTextPresentForExportablePrivacy => "raw_text_present_for_exportable_privacy",
            Self::PolicyWithheldCarriesQueryHash => "policy_withheld_carries_query_hash",
            Self::SharePolicyConflictsWithPrivacy => "share_policy_conflicts_with_privacy",
            Self::MissingScopeIdentity => "missing_scope_identity",
            Self::SessionResultSetMismatch => "session_result_set_mismatch",
            Self::UnsafeRawTextInExportPacket => "unsafe_raw_text_in_export_packet",
            Self::MissingOmittedOrTruncatedDisclosure => "missing_omitted_or_truncated_disclosure",
        }
    }
}

/// Structured validation finding for a saved query or export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryValidationFinding {
    /// Stable finding kind.
    pub finding_kind: SavedQueryValidationFindingKind,
    /// Field or record section that failed validation.
    pub field: String,
    /// Short support-safe explanation of the validation failure.
    pub summary: String,
}

impl SavedQueryValidationFinding {
    fn new(
        finding_kind: SavedQueryValidationFindingKind,
        field: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            field: field.into(),
            summary: summary.into(),
        }
    }
}

/// Error returned when an export packet cannot be assembled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchExportError {
    /// The result set references a different query session.
    ResultSetSessionMismatch {
        /// Query-session id on the live session.
        query_session_id: String,
        /// Query-session id recorded on the result set.
        result_set_session_id: String,
    },
    /// A requested selected result id is not present in the result set.
    SelectedResultUnknown {
        /// Missing result id.
        result_id: String,
    },
}

impl fmt::Display for SearchExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResultSetSessionMismatch {
                query_session_id,
                result_set_session_id,
            } => write!(
                f,
                "result set belongs to {result_set_session_id}, not {query_session_id}"
            ),
            Self::SelectedResultUnknown { result_id } => {
                write!(
                    f,
                    "selected result {result_id} is not present in the result set"
                )
            }
        }
    }
}

impl Error for SearchExportError {}

/// Inputs for building a saved-query record from a live query session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryRecordInputs {
    /// Stable saved-query identity.
    pub saved_query_id: String,
    /// Source class for the saved query.
    pub source_class: SavedQuerySourceClass,
    /// Privacy posture for query material and exportability.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Sharing policy for this saved query.
    pub share_policy: SavedQuerySharePolicy,
    /// Live or sanitized query session to persist.
    pub query_session: SearchQuerySession,
    /// Policy epoch active when the saved query was minted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_epoch: Option<String>,
    /// Timestamp for record creation.
    pub created_at: String,
}

/// Durable saved-query record backed by a canonical query session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryRecord {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable saved-query identity.
    pub saved_query_id: String,
    /// Source class for the saved query.
    pub source_class: SavedQuerySourceClass,
    /// Privacy posture for query material and exportability.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Sharing policy for this saved query.
    pub share_policy: SavedQuerySharePolicy,
    /// Query-session id this saved query replays.
    pub query_session_id_ref: String,
    /// Surface family captured on the source session.
    pub surface: SearchSurface,
    /// Query-text retention mode after privacy projection.
    pub query_text_mode: QueryTextMode,
    /// Raw query text when local-only privacy permits it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    /// Deterministic query hash when privacy permits hash material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hash: Option<String>,
    /// Disposition that explains whether raw query material was kept or removed.
    pub query_material_disposition: QueryMaterialDisposition,
    /// Scope class captured when the query was saved.
    pub scope_class: ScopeClass,
    /// Stable scope identity captured when the query was saved.
    pub stable_scope_id: String,
    /// Scope mode token captured when the query was saved.
    pub scope_mode: String,
    /// Workset id captured when the query was workset-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_id: Option<String>,
    /// Scope chip label shown by the live search session.
    pub scope_label: String,
    /// Readiness token shown by the live search session.
    pub captured_readiness_state: String,
    /// Index epoch captured from the live session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_index_epoch: Option<String>,
    /// Graph epoch captured from the live session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_graph_epoch: Option<String>,
    /// Planner version captured from the live session.
    pub planner_version: String,
    /// Policy epoch active when the saved query was minted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_epoch: Option<String>,
    /// Timestamp for record creation.
    pub created_at: String,
    /// Timestamp for the most recent update.
    pub updated_at: String,
}

impl SavedQueryRecord {
    /// Stable record-kind tag carried in serialized saved-query records.
    pub const RECORD_KIND: &'static str = "saved_query_alpha_record";

    /// Builds a saved-query record from a live query session.
    pub fn from_session(inputs: SavedQueryRecordInputs) -> Self {
        let (query_session, disposition) =
            sanitize_session_for_privacy(inputs.query_session, inputs.privacy_class);
        let updated_at = inputs.created_at.clone();
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: SAVED_QUERY_ALPHA_SCHEMA_VERSION,
            saved_query_id: inputs.saved_query_id,
            source_class: inputs.source_class,
            privacy_class: inputs.privacy_class,
            share_policy: inputs.share_policy,
            query_session_id_ref: query_session.query_session_id,
            surface: query_session.surface,
            query_text_mode: query_session.query_text_mode,
            query_text: query_session.query_text,
            query_hash: query_session.query_hash,
            query_material_disposition: disposition,
            scope_class: query_session.scope_class,
            stable_scope_id: query_session.stable_scope_id,
            scope_mode: query_session.scope_mode,
            workset_id: query_session.workset_id,
            scope_label: query_session.scope_label,
            captured_readiness_state: query_session.readiness_state,
            captured_index_epoch: query_session.index_epoch,
            captured_graph_epoch: query_session.graph_epoch,
            planner_version: query_session.planner_version,
            policy_epoch: inputs.policy_epoch,
            created_at: inputs.created_at,
            updated_at,
        }
    }

    /// True when the serialized saved-query record contains raw query text.
    pub fn contains_raw_query_text(&self) -> bool {
        self.query_text.is_some()
    }

    /// Returns validation findings for privacy and share-policy invariants.
    pub fn validate_privacy(&self) -> Vec<SavedQueryValidationFinding> {
        let mut findings = Vec::new();
        if !self.privacy_class.permits_raw_query_text() && self.query_text.is_some() {
            findings.push(SavedQueryValidationFinding::new(
                SavedQueryValidationFindingKind::RawTextPresentForExportablePrivacy,
                "query_text",
                "raw query text is present for a non-local privacy class",
            ));
        }
        if self.privacy_class == SavedQueryPrivacyClass::PolicyWithheld && self.query_hash.is_some()
        {
            findings.push(SavedQueryValidationFinding::new(
                SavedQueryValidationFindingKind::PolicyWithheldCarriesQueryHash,
                "query_hash",
                "policy-withheld query material must not carry query hashes",
            ));
        }
        if self.stable_scope_id.trim().is_empty() {
            findings.push(SavedQueryValidationFinding::new(
                SavedQueryValidationFindingKind::MissingScopeIdentity,
                "stable_scope_id",
                "saved queries need a stable scope identity before they can reopen",
            ));
        }
        if self.privacy_class == SavedQueryPrivacyClass::LocalOnlyPrivate
            && self.share_policy != SavedQuerySharePolicy::LocalOnlyNoShare
        {
            findings.push(SavedQueryValidationFinding::new(
                SavedQueryValidationFindingKind::SharePolicyConflictsWithPrivacy,
                "share_policy",
                "local-only query material cannot be paired with a non-local share policy",
            ));
        }
        findings
    }

    /// Projects how this saved query would reopen against the current scope.
    pub fn reopen_against(&self, context: SavedQueryReopenContext) -> SavedQueryReopenProjection {
        let captured_rank = scope_rank(self.scope_class);
        let current_rank = scope_rank(context.current_scope_class);
        let query_material_available = self.query_text.is_some() || self.query_hash.is_some();

        let (scope_honesty_state, effective_scope_class, effective_stable_scope_id) =
            if context.current_stable_scope_id == self.stable_scope_id {
                (
                    SavedQueryReopenState::CapturedScopeStillCurrent,
                    context.current_scope_class,
                    context.current_stable_scope_id.clone(),
                )
            } else if current_rank > captured_rank {
                (
                    SavedQueryReopenState::CurrentScopeWiderNarrowedToSavedScope,
                    self.scope_class,
                    self.stable_scope_id.clone(),
                )
            } else if current_rank < captured_rank {
                (
                    SavedQueryReopenState::CurrentScopeNarrowerDisclosed,
                    context.current_scope_class,
                    context.current_stable_scope_id.clone(),
                )
            } else {
                (
                    SavedQueryReopenState::CurrentScopeChangedRebindRequired,
                    self.scope_class,
                    self.stable_scope_id.clone(),
                )
            };

        let mut rerun_allowed =
            scope_honesty_state != SavedQueryReopenState::CurrentScopeChangedRebindRequired;
        let mut denial_reason = None;
        if self.share_policy == SavedQuerySharePolicy::ShareDisabledByPolicy {
            rerun_allowed = false;
            denial_reason = Some(SavedQueryDenialReason::ShareDisabledByPolicy);
        }
        if self.privacy_class == SavedQueryPrivacyClass::PolicyWithheld || !query_material_available
        {
            rerun_allowed = false;
            denial_reason = Some(SavedQueryDenialReason::QueryMaterialWithheldByPolicy);
        }
        if scope_honesty_state == SavedQueryReopenState::CurrentScopeChangedRebindRequired
            && denial_reason.is_none()
        {
            denial_reason = Some(SavedQueryDenialReason::ScopeRequiresRebind);
        }

        let readiness_changed = context.current_readiness_state != self.captured_readiness_state;
        let index_epoch_changed = context.current_index_epoch != self.captured_index_epoch;
        let graph_epoch_changed = context.current_graph_epoch != self.captured_graph_epoch;

        SavedQueryReopenProjection {
            record_kind: SavedQueryReopenProjection::RECORD_KIND.to_string(),
            schema_version: SAVED_QUERY_ALPHA_SCHEMA_VERSION,
            saved_query_id_ref: self.saved_query_id.clone(),
            query_session_id_ref: self.query_session_id_ref.clone(),
            source_class: self.source_class,
            privacy_class: self.privacy_class,
            captured_scope_class: self.scope_class,
            current_scope_class: context.current_scope_class,
            effective_scope_class,
            captured_stable_scope_id: self.stable_scope_id.clone(),
            current_stable_scope_id: context.current_stable_scope_id,
            effective_stable_scope_id,
            captured_scope_label: self.scope_label.clone(),
            current_scope_label: context.current_scope_label,
            scope_honesty_state,
            captured_readiness_state: self.captured_readiness_state.clone(),
            current_readiness_state: context.current_readiness_state,
            readiness_changed,
            captured_index_epoch: self.captured_index_epoch.clone(),
            current_index_epoch: context.current_index_epoch,
            index_epoch_changed,
            captured_graph_epoch: self.captured_graph_epoch.clone(),
            current_graph_epoch: context.current_graph_epoch,
            graph_epoch_changed,
            rerun_allowed,
            denial_reason,
        }
    }
}

/// Current scope and readiness facts used when reopening a saved query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryReopenContext {
    /// Current scope class at reopen time.
    pub current_scope_class: ScopeClass,
    /// Current stable scope identity at reopen time.
    pub current_stable_scope_id: String,
    /// Current scope chip label at reopen time.
    pub current_scope_label: String,
    /// Current readiness token at reopen time.
    pub current_readiness_state: String,
    /// Current index epoch at reopen time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_index_epoch: Option<String>,
    /// Current graph epoch at reopen time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_graph_epoch: Option<String>,
}

/// Scope honesty state emitted when a saved query reopens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryReopenState {
    /// Captured scope still matches the current scope.
    CapturedScopeStillCurrent,
    /// Current scope is wider, so the query remains narrowed to the saved scope.
    CurrentScopeWiderNarrowedToSavedScope,
    /// Current scope is narrower and the reduced scope is disclosed.
    CurrentScopeNarrowerDisclosed,
    /// Current scope has changed laterally and requires explicit rebinding.
    CurrentScopeChangedRebindRequired,
}

impl SavedQueryReopenState {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedScopeStillCurrent => "captured_scope_still_current",
            Self::CurrentScopeWiderNarrowedToSavedScope => {
                "current_scope_wider_narrowed_to_saved_scope"
            }
            Self::CurrentScopeNarrowerDisclosed => "current_scope_narrower_disclosed",
            Self::CurrentScopeChangedRebindRequired => "current_scope_changed_rebind_required",
        }
    }
}

/// Denial reason emitted when a saved query cannot reopen or rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryDenialReason {
    /// Query material was withheld and no rerunnable query remains.
    QueryMaterialWithheldByPolicy,
    /// Sharing or replay is disabled by policy.
    ShareDisabledByPolicy,
    /// Scope changed laterally and must be rebound before rerun.
    ScopeRequiresRebind,
}

impl SavedQueryDenialReason {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryMaterialWithheldByPolicy => "query_material_withheld_by_policy",
            Self::ShareDisabledByPolicy => "share_disabled_by_policy",
            Self::ScopeRequiresRebind => "scope_requires_rebind",
        }
    }
}

/// Reopen projection that proves saved queries do not silently widen scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryReopenProjection {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Saved-query id being reopened.
    pub saved_query_id_ref: String,
    /// Query-session id referenced by the saved query.
    pub query_session_id_ref: String,
    /// Source class of the saved query.
    pub source_class: SavedQuerySourceClass,
    /// Privacy class of the saved query.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Captured scope class on the saved query.
    pub captured_scope_class: ScopeClass,
    /// Current scope class at reopen time.
    pub current_scope_class: ScopeClass,
    /// Scope class that will actually be used for rerun.
    pub effective_scope_class: ScopeClass,
    /// Stable captured scope identity.
    pub captured_stable_scope_id: String,
    /// Stable current scope identity.
    pub current_stable_scope_id: String,
    /// Stable scope identity that will actually be used for rerun.
    pub effective_stable_scope_id: String,
    /// Captured scope chip label.
    pub captured_scope_label: String,
    /// Current scope chip label.
    pub current_scope_label: String,
    /// Scope honesty state for this reopen.
    pub scope_honesty_state: SavedQueryReopenState,
    /// Readiness token captured on the saved query.
    pub captured_readiness_state: String,
    /// Current readiness token at reopen time.
    pub current_readiness_state: String,
    /// True when the current readiness differs from the captured session.
    pub readiness_changed: bool,
    /// Captured index epoch on the saved query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_index_epoch: Option<String>,
    /// Current index epoch at reopen time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_index_epoch: Option<String>,
    /// True when the index epoch differs from the captured session.
    pub index_epoch_changed: bool,
    /// Captured graph epoch on the saved query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_graph_epoch: Option<String>,
    /// Current graph epoch at reopen time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_graph_epoch: Option<String>,
    /// True when the graph epoch differs from the captured session.
    pub graph_epoch_changed: bool,
    /// True when the query can safely rerun under the effective scope.
    pub rerun_allowed: bool,
    /// Denial reason when rerun is not allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<SavedQueryDenialReason>,
}

impl SavedQueryReopenProjection {
    /// Stable record-kind tag carried in serialized reopen projections.
    pub const RECORD_KIND: &'static str = "saved_query_reopen_projection";
}

/// One ledger entry that joins a query session, planner pass, and result set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuerySessionLedgerEntry {
    /// Query session after ledger privacy projection.
    pub query_session: SearchQuerySession,
    /// Privacy class used when this entry was written.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Planner pass id that answered this query session.
    pub planner_pass_id_ref: String,
    /// Result set id emitted by the planner pass.
    pub result_set_id_ref: String,
    /// Readiness token copied from the live session.
    pub readiness_state: String,
    /// Saved-query ids linked to this session.
    #[serde(default)]
    pub saved_query_id_refs: Vec<String>,
    /// Export packet ids linked to this session.
    #[serde(default)]
    pub export_packet_id_refs: Vec<String>,
    /// Timestamp for this ledger entry.
    pub recorded_at: String,
}

impl QuerySessionLedgerEntry {
    fn from_planner_output(
        output: &SearchPlannerOutput,
        privacy_class: SavedQueryPrivacyClass,
        recorded_at: impl Into<String>,
    ) -> Self {
        let (query_session, _) =
            sanitize_session_for_privacy(output.query_session.clone(), privacy_class);
        Self {
            readiness_state: query_session.readiness_state.clone(),
            query_session,
            privacy_class,
            planner_pass_id_ref: output.planner_pass.planner_pass_id.clone(),
            result_set_id_ref: output.result_set.result_set_id.clone(),
            saved_query_id_refs: Vec::new(),
            export_packet_id_refs: Vec::new(),
            recorded_at: recorded_at.into(),
        }
    }
}

/// Durable ledger of query sessions and their saved/exported projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuerySessionLedgerRecord {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable ledger identity.
    pub ledger_id: String,
    /// Planner version whose outputs this ledger records.
    pub planner_version: String,
    /// Ordered query-session entries.
    pub entries: Vec<QuerySessionLedgerEntry>,
    /// Timestamp for ledger creation.
    pub created_at: String,
    /// Timestamp for the most recent ledger update.
    pub updated_at: String,
}

impl QuerySessionLedgerRecord {
    /// Stable record-kind tag carried in serialized ledger records.
    pub const RECORD_KIND: &'static str = "query_session_ledger_record";

    /// Creates an empty query-session ledger.
    pub fn new(ledger_id: impl Into<String>, created_at: impl Into<String>) -> Self {
        let created_at = created_at.into();
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: SAVED_QUERY_ALPHA_SCHEMA_VERSION,
            ledger_id: ledger_id.into(),
            planner_version: SEARCH_PLANNER_ALPHA_VERSION.to_string(),
            entries: Vec::new(),
            created_at: created_at.clone(),
            updated_at: created_at,
        }
    }

    /// Creates a ledger with one planner output entry.
    pub fn from_planner_output(
        ledger_id: impl Into<String>,
        output: &SearchPlannerOutput,
        privacy_class: SavedQueryPrivacyClass,
        recorded_at: impl Into<String>,
    ) -> Self {
        let recorded_at = recorded_at.into();
        let mut ledger = Self::new(ledger_id, recorded_at.clone());
        ledger.append_planner_output(output, privacy_class, recorded_at);
        ledger
    }

    /// Appends a planner output to the ledger.
    pub fn append_planner_output(
        &mut self,
        output: &SearchPlannerOutput,
        privacy_class: SavedQueryPrivacyClass,
        recorded_at: impl Into<String>,
    ) {
        let recorded_at = recorded_at.into();
        self.entries
            .push(QuerySessionLedgerEntry::from_planner_output(
                output,
                privacy_class,
                recorded_at.clone(),
            ));
        self.updated_at = recorded_at;
    }

    /// Links a saved query to an existing query-session entry.
    pub fn link_saved_query(
        &mut self,
        query_session_id: &str,
        saved_query_id: impl Into<String>,
        updated_at: impl Into<String>,
    ) -> bool {
        let Some(entry) = self.entry_mut(query_session_id) else {
            return false;
        };
        let saved_query_id = saved_query_id.into();
        if !entry.saved_query_id_refs.contains(&saved_query_id) {
            entry.saved_query_id_refs.push(saved_query_id);
        }
        self.updated_at = updated_at.into();
        true
    }

    /// Links an export packet to an existing query-session entry.
    pub fn link_export_packet(
        &mut self,
        query_session_id: &str,
        packet_id: impl Into<String>,
        updated_at: impl Into<String>,
    ) -> bool {
        let Some(entry) = self.entry_mut(query_session_id) else {
            return false;
        };
        let packet_id = packet_id.into();
        if !entry.export_packet_id_refs.contains(&packet_id) {
            entry.export_packet_id_refs.push(packet_id);
        }
        self.updated_at = updated_at.into();
        true
    }

    /// Returns a ledger entry by query-session id.
    pub fn entry(&self, query_session_id: &str) -> Option<&QuerySessionLedgerEntry> {
        self.entries
            .iter()
            .find(|entry| entry.query_session.query_session_id == query_session_id)
    }

    fn entry_mut(&mut self, query_session_id: &str) -> Option<&mut QuerySessionLedgerEntry> {
        self.entries
            .iter_mut()
            .find(|entry| entry.query_session.query_session_id == query_session_id)
    }
}

/// Count summary embedded in export-safe search packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchPacketCountSummary {
    /// Rows visible in the source result set or count projection.
    pub visible_rows: u64,
    /// Rows selected for export.
    pub selected_rows: u64,
    /// Rows included in the packet.
    pub included_rows: u64,
    /// Rows omitted because they were not selected.
    pub omitted_result_count: u64,
    /// Rows hidden by the current scope.
    pub hidden_by_current_scope_rows: u64,
    /// Rows hidden or blocked by policy.
    pub hidden_by_policy_rows: u64,
    /// Rows hidden behind a remote cache boundary.
    pub hidden_by_remote_cache_rows: u64,
    /// True when counts are partial, hidden, or otherwise not globally exact.
    pub count_is_partial: bool,
}

impl SearchPacketCountSummary {
    fn from_result_set(
        result_set: &PlannedResultSet,
        included_rows: usize,
        omitted_result_count: usize,
        scope_counts: Option<&SearchScopeCountsRecord>,
    ) -> Self {
        let visible_rows = scope_counts
            .map(|counts| counts.visible_rows)
            .unwrap_or(result_set.rows.len() as u64);
        let hidden_by_current_scope_rows = scope_counts
            .map(|counts| counts.hidden_by_current_scope_rows)
            .unwrap_or_default();
        let hidden_by_policy_rows = scope_counts
            .map(|counts| counts.hidden_by_policy_rows)
            .unwrap_or_default();
        let hidden_by_remote_cache_rows = scope_counts
            .map(|counts| counts.hidden_by_remote_cache_rows)
            .unwrap_or_default();
        let count_is_partial = scope_counts
            .map(|counts| counts.counts_class_token != "globally_authoritative")
            .unwrap_or_else(|| result_set.readiness_state != PlannerPathReadiness::Ready)
            || hidden_by_current_scope_rows > 0
            || hidden_by_policy_rows > 0
            || hidden_by_remote_cache_rows > 0;
        Self {
            visible_rows,
            selected_rows: included_rows as u64,
            included_rows: included_rows as u64,
            omitted_result_count: omitted_result_count as u64,
            hidden_by_current_scope_rows,
            hidden_by_policy_rows,
            hidden_by_remote_cache_rows,
            count_is_partial,
        }
    }
}

/// Inputs for building an export-safe search packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportPacketInputs {
    /// Stable export packet identity.
    pub packet_id: String,
    /// Destination class for this packet.
    pub destination: SearchExportDestination,
    /// Privacy class requested by the caller.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Query session that produced the result set.
    pub query_session: SearchQuerySession,
    /// Planned result set to export.
    pub result_set: PlannedResultSet,
    /// Optional selected result ids; empty means every current row is included.
    #[serde(default)]
    pub selected_result_ids: Vec<String>,
    /// Optional scope count projection to preserve hidden/partial counts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_counts: Option<SearchScopeCountsRecord>,
    /// Timestamp for packet creation.
    pub exported_at: String,
}

/// Export-safe packet for support, docs, or local replay flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportPacket {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable export packet identity.
    pub packet_id: String,
    /// Destination class for this packet.
    pub destination: SearchExportDestination,
    /// Query-session id this packet quotes.
    pub query_session_id_ref: String,
    /// Result-set id this packet quotes.
    pub result_set_id_ref: String,
    /// Planner-pass id this packet quotes.
    pub planner_pass_id_ref: String,
    /// Surface family copied from the live query session.
    pub surface: SearchSurface,
    /// Scope class copied from the live query session.
    pub scope_class: ScopeClass,
    /// Scope chip label copied from the live query session.
    pub scope_label: String,
    /// Stable scope identity copied from the live query session.
    pub stable_scope_id: String,
    /// Readiness token copied from the live query session.
    pub readiness_state: String,
    /// Index epoch copied from the live query session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_epoch: Option<String>,
    /// Graph epoch copied from the live query session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_epoch: Option<String>,
    /// Privacy class after destination projection.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Packet redaction state.
    pub redaction_state: SearchPacketRedactionState,
    /// Live-versus-captured truth for this packet.
    #[serde(default = "default_export_snapshot_truth")]
    pub snapshot_truth: SearchExportSnapshotTruth,
    /// Query-text retention mode after export projection.
    pub query_text_mode: QueryTextMode,
    /// Raw query text when local replay permits it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    /// Deterministic query hash when privacy permits hash material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hash: Option<String>,
    /// Result ids selected for the packet.
    pub selected_result_refs: Vec<String>,
    /// Result ids included in the packet in live rank order.
    pub included_result_refs: Vec<String>,
    /// Source labels contributed by included result rows.
    pub result_source_labels: Vec<String>,
    /// Partial-truth causes contributed by included result rows.
    pub partial_truth_causes: Vec<String>,
    /// Count summary preserving loaded, hidden, and omitted rows.
    pub count_summary: SearchPacketCountSummary,
    /// Export-safe flags for omitted or truncated content/classes.
    #[serde(default)]
    pub omitted_or_truncated_flags: Vec<String>,
    /// Evidence refs this packet can hand to support, docs, AI, or CLI consumers.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Timestamp for packet creation.
    pub exported_at: String,
}

impl SearchExportPacket {
    /// Stable record-kind tag carried in serialized export packets.
    pub const RECORD_KIND: &'static str = "search_export_packet";

    /// Builds an export-safe packet from a live session and planned result set.
    pub fn from_planned_result_set(
        inputs: SearchExportPacketInputs,
    ) -> Result<Self, SearchExportError> {
        if inputs.query_session.query_session_id != inputs.result_set.query_session_id_ref {
            return Err(SearchExportError::ResultSetSessionMismatch {
                query_session_id: inputs.query_session.query_session_id,
                result_set_session_id: inputs.result_set.query_session_id_ref,
            });
        }

        let result_ids: BTreeSet<&str> = inputs
            .result_set
            .rows
            .iter()
            .map(|row| row.result_id.as_str())
            .collect();
        for selected in &inputs.selected_result_ids {
            if !result_ids.contains(selected.as_str()) {
                return Err(SearchExportError::SelectedResultUnknown {
                    result_id: selected.clone(),
                });
            }
        }

        let included_result_refs: Vec<String> = if inputs.selected_result_ids.is_empty() {
            inputs
                .result_set
                .rows
                .iter()
                .map(|row| row.result_id.clone())
                .collect()
        } else {
            inputs
                .result_set
                .rows
                .iter()
                .filter(|row| inputs.selected_result_ids.contains(&row.result_id))
                .map(|row| row.result_id.clone())
                .collect()
        };
        let selected_result_refs = if inputs.selected_result_ids.is_empty() {
            included_result_refs.clone()
        } else {
            inputs.selected_result_ids.clone()
        };
        let included_set: BTreeSet<&str> =
            included_result_refs.iter().map(String::as_str).collect();
        let omitted_result_count = inputs
            .result_set
            .rows
            .len()
            .saturating_sub(included_result_refs.len());

        let result_source_labels = inputs
            .result_set
            .rows
            .iter()
            .filter(|row| included_set.contains(row.result_id.as_str()))
            .map(|row| row.answered_by.as_str().to_string())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let partial_truth_causes = inputs
            .result_set
            .rows
            .iter()
            .filter(|row| included_set.contains(row.result_id.as_str()))
            .flat_map(|row| row.partial_truth_causes.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let count_summary = SearchPacketCountSummary::from_result_set(
            &inputs.result_set,
            included_result_refs.len(),
            omitted_result_count,
            inputs.scope_counts.as_ref(),
        );

        let privacy_class = inputs.privacy_class.for_destination(inputs.destination);
        let (query_session, _) = sanitize_session_for_privacy(inputs.query_session, privacy_class);
        let redaction_state = packet_redaction_state(
            privacy_class,
            query_session.query_text.is_some(),
            query_session.query_hash.is_some(),
        );
        let omitted_or_truncated_flags = omitted_or_truncated_flags(&count_summary);
        let evidence_refs = evidence_refs_for_export(
            &query_session.query_session_id,
            &inputs.result_set.result_set_id,
            &inputs.result_set.planner_pass_id_ref,
        );

        Ok(Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: SAVED_QUERY_ALPHA_SCHEMA_VERSION,
            packet_id: inputs.packet_id,
            destination: inputs.destination,
            query_session_id_ref: query_session.query_session_id,
            result_set_id_ref: inputs.result_set.result_set_id,
            planner_pass_id_ref: inputs.result_set.planner_pass_id_ref,
            surface: query_session.surface,
            scope_class: query_session.scope_class,
            scope_label: query_session.scope_label,
            stable_scope_id: query_session.stable_scope_id,
            readiness_state: query_session.readiness_state,
            index_epoch: query_session.index_epoch,
            graph_epoch: query_session.graph_epoch,
            privacy_class,
            redaction_state,
            snapshot_truth: SearchExportSnapshotTruth::CapturedSnapshot,
            query_text_mode: query_session.query_text_mode,
            query_text: query_session.query_text,
            query_hash: query_session.query_hash,
            selected_result_refs,
            included_result_refs,
            result_source_labels,
            partial_truth_causes,
            count_summary,
            omitted_or_truncated_flags,
            evidence_refs,
            exported_at: inputs.exported_at,
        })
    }

    /// Returns validation findings for export-safety invariants.
    pub fn validate_export_safe(&self) -> Vec<SavedQueryValidationFinding> {
        let mut findings = Vec::new();
        if self.destination.requires_redacted_query_text() && self.query_text.is_some() {
            findings.push(SavedQueryValidationFinding::new(
                SavedQueryValidationFindingKind::UnsafeRawTextInExportPacket,
                "query_text",
                "non-local search export packets must not contain raw query text",
            ));
        }
        if !self.privacy_class.permits_raw_query_text() && self.query_text.is_some() {
            findings.push(SavedQueryValidationFinding::new(
                SavedQueryValidationFindingKind::RawTextPresentForExportablePrivacy,
                "query_text",
                "packet privacy class forbids raw query text",
            ));
        }
        if self.privacy_class == SavedQueryPrivacyClass::PolicyWithheld && self.query_hash.is_some()
        {
            findings.push(SavedQueryValidationFinding::new(
                SavedQueryValidationFindingKind::PolicyWithheldCarriesQueryHash,
                "query_hash",
                "policy-withheld export packets must not carry query hashes",
            ));
        }
        if (self.count_summary.omitted_result_count > 0 || self.count_summary.count_is_partial)
            && self.omitted_or_truncated_flags.is_empty()
        {
            findings.push(SavedQueryValidationFinding::new(
                SavedQueryValidationFindingKind::MissingOmittedOrTruncatedDisclosure,
                "omitted_or_truncated_flags",
                "partial or omitted export packets must preserve omitted/truncated flags",
            ));
        }
        findings
    }
}

fn omitted_or_truncated_flags(count_summary: &SearchPacketCountSummary) -> Vec<String> {
    let mut flags = BTreeSet::new();
    if count_summary.omitted_result_count > 0 {
        flags.insert("omitted_unselected_results".to_string());
    }
    if count_summary.hidden_by_current_scope_rows > 0 {
        flags.insert("hidden_by_current_scope".to_string());
    }
    if count_summary.hidden_by_policy_rows > 0 {
        flags.insert("hidden_by_policy".to_string());
    }
    if count_summary.hidden_by_remote_cache_rows > 0 {
        flags.insert("hidden_by_remote_cache".to_string());
    }
    if count_summary.count_is_partial {
        flags.insert("partial_counts".to_string());
    }
    flags.into_iter().collect()
}

fn evidence_refs_for_export(
    query_session_id: &str,
    result_set_id: &str,
    planner_pass_id: &str,
) -> Vec<String> {
    vec![
        format!("query_session:{query_session_id}"),
        format!("result_set:{result_set_id}"),
        format!("planner_pass:{planner_pass_id}"),
    ]
}

fn sanitize_session_for_privacy(
    mut session: SearchQuerySession,
    privacy_class: SavedQueryPrivacyClass,
) -> (SearchQuerySession, QueryMaterialDisposition) {
    match privacy_class {
        SavedQueryPrivacyClass::LocalOnlyPrivate => {
            if session.query_hash.is_none() {
                session.query_hash = session.query_text.as_deref().map(stable_query_hash);
            }
            let disposition = if session.query_text.is_some() {
                QueryMaterialDisposition::RawRetainedLocalOnly
            } else if session.query_hash.is_some() {
                QueryMaterialDisposition::RawRedactedHashRetained
            } else {
                QueryMaterialDisposition::NoQueryMaterialProvided
            };
            (session, disposition)
        }
        SavedQueryPrivacyClass::WorkspaceSharedRedacted
        | SavedQueryPrivacyClass::SupportExportRedacted => {
            if session.query_hash.is_none() {
                session.query_hash = session.query_text.as_deref().map(stable_query_hash);
            }
            session.query_text = None;
            session.query_text_mode = if session.query_hash.is_some() {
                QueryTextMode::HashOnly
            } else {
                QueryTextMode::OmittedByPolicy
            };
            let disposition = if session.query_hash.is_some() {
                QueryMaterialDisposition::RawRedactedHashRetained
            } else {
                QueryMaterialDisposition::NoQueryMaterialProvided
            };
            (session, disposition)
        }
        SavedQueryPrivacyClass::PolicyWithheld => {
            session.query_text = None;
            session.query_hash = None;
            session.query_text_mode = QueryTextMode::OmittedByPolicy;
            (session, QueryMaterialDisposition::RawWithheldByPolicy)
        }
    }
}

fn packet_redaction_state(
    privacy_class: SavedQueryPrivacyClass,
    has_raw_query_text: bool,
    has_query_hash: bool,
) -> SearchPacketRedactionState {
    if privacy_class == SavedQueryPrivacyClass::PolicyWithheld {
        SearchPacketRedactionState::QueryMaterialOmittedByPolicy
    } else if has_raw_query_text {
        SearchPacketRedactionState::RawQueryLocalOnly
    } else if has_query_hash {
        SearchPacketRedactionState::QueryHashOnly
    } else {
        SearchPacketRedactionState::QueryMaterialOmittedByPolicy
    }
}

fn default_export_snapshot_truth() -> SearchExportSnapshotTruth {
    SearchExportSnapshotTruth::CapturedSnapshot
}

fn scope_rank(scope_class: ScopeClass) -> u8 {
    match scope_class {
        ScopeClass::PolicyLimitedView => 0,
        ScopeClass::CurrentRepo | ScopeClass::SelectedWorkset | ScopeClass::SparseSlice => 1,
        ScopeClass::FullWorkspace => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planner::{PlannedResultSet, SearchPlannerAlpha, SearchPlannerInputs};

    fn planner_input() -> SearchPlannerInputs {
        serde_json::from_value(serde_json::json!({
            "query_session": {
                "record_kind": "search_query_session",
                "schema_version": 1,
                "query_session_id": "search:session:ledger:unit",
                "surface": "file_search",
                "query_text_mode": "local_text",
                "query_text": "tokenized secret",
                "scope_class": "selected_workset",
                "stable_scope_id": "scope:workset:unit",
                "scope_mode": "sparse",
                "workset_id": "wks:unit",
                "scope_label": "Selected workset",
                "planner_version": "search-planner-alpha",
                "readiness_state": "warming",
                "index_epoch": "idx:unit:01",
                "observed_at": "mono:ledger:unit:01"
            },
            "planner_pass_id": "search:planner:ledger:unit",
            "result_set_id": "search:result_set:ledger:unit",
            "planner_version": "search-planner-alpha",
            "observed_at": "mono:ledger:unit:02",
            "path_snapshots": [{
                "path_kind": "lexical",
                "snapshot_id": "search:snapshot:ledger:lexical",
                "readiness": "partial",
                "freshness": "authoritative_live",
                "index_epoch": "idx:unit:01",
                "partial_truth_causes": ["indexing_in_progress"],
                "rows": [{
                    "candidate_id": "candidate:unit",
                    "canonical_id": "workspace:file:src/lib.rs",
                    "target_kind": "file",
                    "title": "lib.rs",
                    "relative_path": "src/lib.rs",
                    "ranking_reasons": ["lexical_prefix_match"]
                }]
            }]
        }))
        .expect("unit fixture must parse")
    }

    #[test]
    fn saved_query_redacts_query_text_for_support_privacy() {
        let output = SearchPlannerAlpha::plan(planner_input());
        let saved = SavedQueryRecord::from_session(SavedQueryRecordInputs {
            saved_query_id: "search:saved:unit".to_string(),
            source_class: SavedQuerySourceClass::SupportCaptured,
            privacy_class: SavedQueryPrivacyClass::SupportExportRedacted,
            share_policy: SavedQuerySharePolicy::SupportExportRedactedOnly,
            query_session: output.query_session,
            policy_epoch: Some("policy:unit".to_string()),
            created_at: "2026-05-13T10:00:00Z".to_string(),
        });

        assert!(!saved.contains_raw_query_text());
        assert_eq!(saved.query_text_mode, QueryTextMode::HashOnly);
        assert_eq!(
            saved.query_material_disposition,
            QueryMaterialDisposition::RawRedactedHashRetained
        );
        assert!(saved.validate_privacy().is_empty());
    }

    #[test]
    fn export_packet_refuses_mismatched_result_set() {
        let output = SearchPlannerAlpha::plan(planner_input());
        let mut result_set: PlannedResultSet = output.result_set.clone();
        result_set.query_session_id_ref = "search:session:different".to_string();

        let err = SearchExportPacket::from_planned_result_set(SearchExportPacketInputs {
            packet_id: "search:packet:unit".to_string(),
            destination: SearchExportDestination::SupportBundle,
            privacy_class: SavedQueryPrivacyClass::SupportExportRedacted,
            query_session: output.query_session,
            result_set,
            selected_result_ids: Vec::new(),
            scope_counts: None,
            exported_at: "2026-05-13T10:00:00Z".to_string(),
        })
        .expect_err("mismatched session must fail");

        assert!(matches!(
            err,
            SearchExportError::ResultSetSessionMismatch { .. }
        ));
    }

    #[test]
    fn export_packets_preserve_snapshot_truth_flags_and_evidence_refs() {
        let output = SearchPlannerAlpha::plan(planner_input());
        let packet = SearchExportPacket::from_planned_result_set(SearchExportPacketInputs {
            packet_id: "search:packet:evidence".to_string(),
            destination: SearchExportDestination::SupportBundle,
            privacy_class: SavedQueryPrivacyClass::SupportExportRedacted,
            query_session: output.query_session,
            result_set: output.result_set,
            selected_result_ids: Vec::new(),
            scope_counts: None,
            exported_at: "2026-05-13T10:00:00Z".to_string(),
        })
        .expect("packet materializes");

        assert_eq!(
            packet.snapshot_truth,
            SearchExportSnapshotTruth::CapturedSnapshot
        );
        assert!(packet
            .omitted_or_truncated_flags
            .contains(&"partial_counts".to_string()));
        assert!(packet
            .evidence_refs
            .iter()
            .any(|reference| reference.starts_with("query_session:")));
        assert!(packet.validate_export_safe().is_empty());
    }
}
