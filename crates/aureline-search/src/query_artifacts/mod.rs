//! Versioned saved-query, history, deep-link, and export-snapshot artifacts.
//!
//! This module is the beta artifact layer above the planner-backed search
//! session and result-set contracts. It keeps saved queries, query history,
//! deep links, scope bindings, and exported result snapshots on one local-first
//! privacy vocabulary so durable search state does not become hidden telemetry
//! or a back door for widening data access.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::counts::SearchScopeCountsRecord;
use crate::lexical::ScopeClass;
use crate::planner::PlannedResultSet;
use crate::query_session::{QueryTextMode, SearchQuerySession, SearchSurface};
use crate::session_ledger::{
    SavedQueryPrivacyClass, SavedQueryRecord, SavedQueryRecordInputs, SavedQuerySharePolicy,
    SavedQuerySourceClass, SearchExportDestination, SearchExportError, SearchExportPacket,
    SearchExportPacketInputs, SearchExportSnapshotTruth, SearchPacketCountSummary,
};

/// Integer schema version for saved-query artifact records.
pub const SEARCH_QUERY_ARTIFACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the saved-query artifact schema.
pub const SAVED_QUERY_SCHEMA_REF: &str = "schemas/search/saved_query.schema.json";

/// Repo-relative path of the query-history artifact schema.
pub const QUERY_HISTORY_SCHEMA_REF: &str = "schemas/search/query_history.schema.json";

/// Repo-relative path of the search-export snapshot schema.
pub const SEARCH_EXPORT_SNAPSHOT_SCHEMA_REF: &str =
    "schemas/search/search_export_snapshot.schema.json";

/// Repo-relative path of the saved-query privacy reviewer doc.
pub const SAVED_QUERY_EXPORT_PRIVACY_DOC_REF: &str =
    "docs/search/m3/saved_query_and_export_privacy_beta.md";

/// Repo-relative path of the protected saved-query privacy fixture corpus.
pub const SAVED_QUERY_PRIVACY_FIXTURE_DIR: &str = "fixtures/search/m3/saved_query_privacy";

/// Stable record-kind tag for [`SavedQuery`].
pub const SAVED_QUERY_RECORD_KIND: &str = "saved_query";

/// Stable record-kind tag for [`QueryHistoryEntry`].
pub const QUERY_HISTORY_ENTRY_RECORD_KIND: &str = "query_history_entry";

/// Stable record-kind tag for [`ScopePackBinding`].
pub const SCOPE_PACK_BINDING_RECORD_KIND: &str = "scope_pack_binding";

/// Stable record-kind tag for [`SearchDeepLink`].
pub const SEARCH_DEEP_LINK_RECORD_KIND: &str = "search_deep_link";

/// Stable record-kind tag for [`SearchCollectionSnapshot`].
pub const SEARCH_COLLECTION_SNAPSHOT_RECORD_KIND: &str = "search_collection_snapshot";

/// Retention posture applied to durable search artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchRetentionMode {
    /// Artifact remains in the local profile and is not remotely retained.
    LocalOnlyDefault,
    /// Artifact is local and expires as ephemeral history.
    LocalOnlyEphemeral,
    /// Artifact may be shared into a workspace only after explicit admission.
    WorkspaceSharedExplicit,
    /// Artifact is read from repository state and follows repository retention.
    RepoProvidedReadOnly,
    /// Artifact is owned by managed policy or an administrator.
    PolicyOwnedManaged,
    /// Artifact is retained only as a redacted support export artifact.
    SupportExportBounded,
}

impl SearchRetentionMode {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyDefault => "local_only_default",
            Self::LocalOnlyEphemeral => "local_only_ephemeral",
            Self::WorkspaceSharedExplicit => "workspace_shared_explicit",
            Self::RepoProvidedReadOnly => "repo_provided_read_only",
            Self::PolicyOwnedManaged => "policy_owned_managed",
            Self::SupportExportBounded => "support_export_bounded",
        }
    }

    /// True when this retention mode leaves the local-only default boundary.
    pub const fn widens_local_default(self) -> bool {
        !matches!(self, Self::LocalOnlyDefault | Self::LocalOnlyEphemeral)
    }

    /// True when literal query text can remain under this retention mode.
    pub const fn permits_local_literal_text(self) -> bool {
        matches!(self, Self::LocalOnlyDefault | Self::LocalOnlyEphemeral)
    }

    /// Returns the default retention mode for a source and privacy posture.
    pub const fn default_for(
        source_class: SavedQuerySourceClass,
        privacy_class: SavedQueryPrivacyClass,
    ) -> Self {
        if matches!(privacy_class, SavedQueryPrivacyClass::PolicyWithheld) {
            return Self::PolicyOwnedManaged;
        }

        match source_class {
            SavedQuerySourceClass::UserAuthored | SavedQuerySourceClass::SessionHistory => {
                Self::LocalOnlyDefault
            }
            SavedQuerySourceClass::RepoProvided => Self::RepoProvidedReadOnly,
            SavedQuerySourceClass::PolicyProvided => Self::PolicyOwnedManaged,
            SavedQuerySourceClass::TeamShared => Self::WorkspaceSharedExplicit,
            SavedQuerySourceClass::SupportCaptured => Self::SupportExportBounded,
        }
    }
}

/// Sync posture applied to durable search artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchSyncClass {
    /// Artifact stays on the local device.
    LocalOnly,
    /// User explicitly admitted the artifact to sync.
    ExplicitUserSync,
    /// Artifact is shared through workspace state.
    WorkspaceShared,
    /// Artifact is provided by repository state and not user-synced.
    RepoProvided,
    /// Artifact is owned and distributed by policy.
    PolicyManaged,
    /// Artifact travels only inside a redacted support export.
    SupportExportOnly,
}

impl SearchSyncClass {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ExplicitUserSync => "explicit_user_sync",
            Self::WorkspaceShared => "workspace_shared",
            Self::RepoProvided => "repo_provided",
            Self::PolicyManaged => "policy_managed",
            Self::SupportExportOnly => "support_export_only",
        }
    }

    /// True when this sync class leaves the local-only default boundary.
    pub const fn widens_local_default(self) -> bool {
        !matches!(self, Self::LocalOnly)
    }

    /// Returns the default sync class for a source and privacy posture.
    pub const fn default_for(
        source_class: SavedQuerySourceClass,
        privacy_class: SavedQueryPrivacyClass,
    ) -> Self {
        if matches!(privacy_class, SavedQueryPrivacyClass::PolicyWithheld) {
            return Self::PolicyManaged;
        }

        match source_class {
            SavedQuerySourceClass::UserAuthored | SavedQuerySourceClass::SessionHistory => {
                Self::LocalOnly
            }
            SavedQuerySourceClass::RepoProvided => Self::RepoProvided,
            SavedQuerySourceClass::PolicyProvided => Self::PolicyManaged,
            SavedQuerySourceClass::TeamShared => Self::WorkspaceShared,
            SavedQuerySourceClass::SupportCaptured => Self::SupportExportOnly,
        }
    }
}

/// Redaction profile applied to query material and exported result snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchRedactionProfile {
    /// Literal query text may exist only in local storage.
    LiteralLocalOnly,
    /// Artifact carries hashes, scope summaries, result refs, and counts.
    HashesScopeAndResultRefs,
    /// Artifact carries metadata only and no query material.
    MetadataOnlyNoQueryMaterial,
    /// Policy withholds literal and hash query material.
    PolicyWithheld,
    /// Literal query text was explicitly consented for a higher-trust export.
    ExplicitLiteralConsent,
}

impl SearchRedactionProfile {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiteralLocalOnly => "literal_local_only",
            Self::HashesScopeAndResultRefs => "hashes_scope_and_result_refs",
            Self::MetadataOnlyNoQueryMaterial => "metadata_only_no_query_material",
            Self::PolicyWithheld => "policy_withheld",
            Self::ExplicitLiteralConsent => "explicit_literal_consent",
        }
    }

    /// True when this profile can carry literal query text.
    pub const fn permits_literal_query_text(self) -> bool {
        matches!(self, Self::LiteralLocalOnly | Self::ExplicitLiteralConsent)
    }

    /// Returns the default redaction profile for a privacy posture.
    pub const fn default_for(privacy_class: SavedQueryPrivacyClass) -> Self {
        match privacy_class {
            SavedQueryPrivacyClass::LocalOnlyPrivate => Self::LiteralLocalOnly,
            SavedQueryPrivacyClass::WorkspaceSharedRedacted
            | SavedQueryPrivacyClass::SupportExportRedacted => Self::HashesScopeAndResultRefs,
            SavedQueryPrivacyClass::PolicyWithheld => Self::PolicyWithheld,
        }
    }
}

/// Reason a durable search artifact is allowed to leave local-only defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchRetentionWideningBasis {
    /// Artifact uses the local-only default and needs no widening basis.
    NotWidenedLocalDefault,
    /// User explicitly opted into broader retention or sync.
    ExplicitUserOptIn,
    /// Managed policy owns the broader retention posture.
    PolicyOwned,
    /// Repository state owns the artifact.
    RepoProvidedArtifact,
    /// Team or workspace sharing owns the artifact.
    TeamSharedArtifact,
    /// Redacted support export owns the bounded retention copy.
    SupportCaseExport,
}

impl SearchRetentionWideningBasis {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotWidenedLocalDefault => "not_widened_local_default",
            Self::ExplicitUserOptIn => "explicit_user_opt_in",
            Self::PolicyOwned => "policy_owned",
            Self::RepoProvidedArtifact => "repo_provided_artifact",
            Self::TeamSharedArtifact => "team_shared_artifact",
            Self::SupportCaseExport => "support_case_export",
        }
    }

    /// Returns the default widening basis for a source and effective posture.
    pub const fn default_for(
        source_class: SavedQuerySourceClass,
        retention_mode: SearchRetentionMode,
        sync_class: SearchSyncClass,
    ) -> Self {
        if !retention_mode.widens_local_default() && !sync_class.widens_local_default() {
            return Self::NotWidenedLocalDefault;
        }

        match source_class {
            SavedQuerySourceClass::UserAuthored | SavedQuerySourceClass::SessionHistory => {
                Self::ExplicitUserOptIn
            }
            SavedQuerySourceClass::RepoProvided => Self::RepoProvidedArtifact,
            SavedQuerySourceClass::PolicyProvided => Self::PolicyOwned,
            SavedQuerySourceClass::TeamShared => Self::TeamSharedArtifact,
            SavedQuerySourceClass::SupportCaptured => Self::SupportCaseExport,
        }
    }
}

/// Whether an artifact represents current results or captured history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchResultSemantics {
    /// Results were re-resolved against current scope and permissions.
    CurrentLiveResults,
    /// Results are a captured snapshot of a prior planner pass.
    CapturedSnapshot,
    /// The artifact stores intent and must rerun before claiming current truth.
    LiveRerunRequired,
    /// Current scope differs from the captured scope.
    ScopeChangedSinceCapture,
    /// Re-resolution produced no rows because the current scope changed.
    EmptyBecauseScopeChanged,
}

impl SearchResultSemantics {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLiveResults => "current_live_results",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::LiveRerunRequired => "live_rerun_required",
            Self::ScopeChangedSinceCapture => "scope_changed_since_capture",
            Self::EmptyBecauseScopeChanged => "empty_because_scope_changed",
        }
    }
}

/// Scope-honesty state attached to reopenable search artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchScopeHonestyState {
    /// Current scope still matches the captured scope.
    CapturedScopeStillCurrent,
    /// Recipient must resolve the artifact under their own current permissions.
    RecipientMustReResolve,
    /// Current scope is wider and must narrow to the captured scope.
    CurrentScopeWiderNarrowedToCaptured,
    /// Current scope is narrower and the reduction is disclosed.
    CurrentScopeNarrowerDisclosed,
    /// Current scope changed laterally and requires explicit rebinding.
    CurrentScopeChangedRebindRequired,
}

impl SearchScopeHonestyState {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedScopeStillCurrent => "captured_scope_still_current",
            Self::RecipientMustReResolve => "recipient_must_re_resolve",
            Self::CurrentScopeWiderNarrowedToCaptured => "current_scope_wider_narrowed_to_captured",
            Self::CurrentScopeNarrowerDisclosed => "current_scope_narrower_disclosed",
            Self::CurrentScopeChangedRebindRequired => "current_scope_changed_rebind_required",
        }
    }
}

/// Explicit migration posture for versioned saved-query and deep-link grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchArtifactMigrationState {
    /// Artifact is current for this schema version.
    Current,
    /// Artifact was migrated from an older schema version.
    MigratedFromPreviousVersion,
    /// Artifact must migrate before replay or sharing.
    MigrationRequired,
}

impl SearchArtifactMigrationState {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::MigratedFromPreviousVersion => "migrated_from_previous_version",
            Self::MigrationRequired => "migration_required",
        }
    }
}

/// Validation finding emitted by search artifact privacy checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchArtifactValidationFindingKind {
    /// Raw query text exists outside the local-only literal profile.
    RawQueryMaterialNotLocalOnly,
    /// Sync or managed retention widened without opt-in or policy ownership.
    SyncOrManagedRetentionWithoutWideningBasis,
    /// A deep link would widen access instead of forcing re-resolution.
    DeepLinkWouldWidenAccess,
    /// A scope-bound artifact is missing stable scope identity.
    MissingScopeIdentity,
    /// A partial export snapshot has no partiality reason.
    ExportSnapshotMissingPartialityReasons,
    /// A captured artifact claims current live results without re-resolution.
    CapturedArtifactClaimsCurrentResults,
}

impl SearchArtifactValidationFindingKind {
    /// Stable token used in records, fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawQueryMaterialNotLocalOnly => "raw_query_material_not_local_only",
            Self::SyncOrManagedRetentionWithoutWideningBasis => {
                "sync_or_managed_retention_without_widening_basis"
            }
            Self::DeepLinkWouldWidenAccess => "deep_link_would_widen_access",
            Self::MissingScopeIdentity => "missing_scope_identity",
            Self::ExportSnapshotMissingPartialityReasons => {
                "export_snapshot_missing_partiality_reasons"
            }
            Self::CapturedArtifactClaimsCurrentResults => {
                "captured_artifact_claims_current_results"
            }
        }
    }
}

/// Structured validation finding for durable search artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchArtifactValidationFinding {
    /// Stable finding kind.
    pub finding_kind: SearchArtifactValidationFindingKind,
    /// Field or record section that failed validation.
    pub field: String,
    /// Short support-safe explanation of the validation failure.
    pub summary: String,
}

impl SearchArtifactValidationFinding {
    fn new(
        finding_kind: SearchArtifactValidationFindingKind,
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

/// Durable binding between search intent and a captured scope pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopePackBinding {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this artifact.
    pub schema_version: u32,
    /// Stable scope-binding identity.
    pub scope_binding_id: String,
    /// Source class that created the binding.
    pub source_class: SavedQuerySourceClass,
    /// Privacy posture for the binding.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Retention mode for the binding.
    pub retention_mode: SearchRetentionMode,
    /// Sync posture for the binding.
    pub sync_class: SearchSyncClass,
    /// Redaction profile for scope summaries.
    pub redaction_profile: SearchRedactionProfile,
    /// Basis for any retention or sync widening.
    pub retention_widening_basis: SearchRetentionWideningBasis,
    /// Captured scope class.
    pub captured_scope_class: ScopeClass,
    /// Stable captured scope identity.
    pub captured_stable_scope_id: String,
    /// Captured scope chip label.
    pub captured_scope_label: String,
    /// Captured scope mode token.
    pub scope_mode: String,
    /// Captured workset id, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_id: Option<String>,
    /// Redacted include-rule summaries.
    #[serde(default)]
    pub include_rule_summaries: Vec<String>,
    /// Redacted exclude-rule summaries.
    #[serde(default)]
    pub exclude_rule_summaries: Vec<String>,
    /// Reasons a captured scope could not resolve on reopen.
    #[serde(default)]
    pub missing_scope_reasons: Vec<String>,
    /// Scope honesty state for the latest resolution.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// Timestamp when the binding resolved.
    pub resolved_at: String,
}

impl ScopePackBinding {
    /// Builds a scope binding from a planner-backed query session.
    // Keep this constructor field-shaped so callers cannot hide privacy,
    // retention, sync, redaction, and source evidence behind defaults.
    #[allow(clippy::too_many_arguments)]
    pub fn from_query_session(
        scope_binding_id: impl Into<String>,
        session: &SearchQuerySession,
        source_class: SavedQuerySourceClass,
        privacy_class: SavedQueryPrivacyClass,
        retention_mode: SearchRetentionMode,
        sync_class: SearchSyncClass,
        redaction_profile: SearchRedactionProfile,
        retention_widening_basis: SearchRetentionWideningBasis,
        resolved_at: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: SCOPE_PACK_BINDING_RECORD_KIND.to_string(),
            schema_version: SEARCH_QUERY_ARTIFACT_SCHEMA_VERSION,
            scope_binding_id: scope_binding_id.into(),
            source_class,
            privacy_class,
            retention_mode,
            sync_class,
            redaction_profile,
            retention_widening_basis,
            captured_scope_class: session.scope_class,
            captured_stable_scope_id: session.stable_scope_id.clone(),
            captured_scope_label: session.scope_label.clone(),
            scope_mode: session.scope_mode.clone(),
            workset_id: session.workset_id.clone(),
            include_rule_summaries: Vec::new(),
            exclude_rule_summaries: Vec::new(),
            missing_scope_reasons: Vec::new(),
            scope_honesty_state: SearchScopeHonestyState::CapturedScopeStillCurrent,
            resolved_at: resolved_at.into(),
        }
    }

    /// Returns validation findings for scope and retention invariants.
    pub fn validate(&self) -> Vec<SearchArtifactValidationFinding> {
        let mut findings = Vec::new();
        push_missing_scope_identity(
            &mut findings,
            "captured_stable_scope_id",
            &self.captured_stable_scope_id,
        );
        push_widening_findings(
            &mut findings,
            self.source_class,
            self.retention_mode,
            self.sync_class,
            self.retention_widening_basis,
            "scope_pack_binding",
        );
        findings
    }
}

/// Durable saved search query artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQuery {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this artifact.
    pub schema_version: u32,
    /// Stable saved-query identity.
    pub saved_query_id: String,
    /// Short reviewable label that is not raw query text.
    pub display_name: String,
    /// Source class for the saved query.
    pub source_class: SavedQuerySourceClass,
    /// Privacy posture for query material.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Retention mode for the saved query.
    pub retention_mode: SearchRetentionMode,
    /// Sync posture for the saved query.
    pub sync_class: SearchSyncClass,
    /// Redaction profile for query material.
    pub redaction_profile: SearchRedactionProfile,
    /// Basis for any retention or sync widening.
    pub retention_widening_basis: SearchRetentionWideningBasis,
    /// Sharing policy attached to the saved query.
    pub share_policy: SavedQuerySharePolicy,
    /// Explicit schema migration state.
    pub migration_state: SearchArtifactMigrationState,
    /// Default surface used when reopening the query.
    pub default_surface: SearchSurface,
    /// Query-session id this saved query replays.
    pub query_session_id_ref: String,
    /// Query-text retention mode after privacy projection.
    pub query_text_mode: QueryTextMode,
    /// Raw query text when local-only privacy permits it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    /// Deterministic query hash when privacy permits hash material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hash: Option<String>,
    /// Scope binding referenced by this saved query.
    pub scope_binding_id_ref: String,
    /// Captured scope class.
    pub scope_class: ScopeClass,
    /// Stable captured scope identity.
    pub stable_scope_id: String,
    /// Captured scope chip label.
    pub scope_label: String,
    /// Readiness token captured when the query was saved.
    pub captured_readiness_state: String,
    /// Planner version captured when the query was saved.
    pub planner_version: String,
    /// Live-vs-captured semantics shown by reopen surfaces.
    pub result_semantics: SearchResultSemantics,
    /// Scope honesty state shown by reopen surfaces.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// Timestamp for record creation.
    pub created_at: String,
    /// Timestamp for the most recent update.
    pub updated_at: String,
}

impl SavedQuery {
    /// Builds a durable saved-query artifact from the alpha saved-query record.
    pub fn from_saved_query_record(
        saved_query: &SavedQueryRecord,
        display_name: impl Into<String>,
        scope_binding_id_ref: impl Into<String>,
        retention_mode: SearchRetentionMode,
        sync_class: SearchSyncClass,
        redaction_profile: SearchRedactionProfile,
        retention_widening_basis: SearchRetentionWideningBasis,
    ) -> Self {
        Self {
            record_kind: SAVED_QUERY_RECORD_KIND.to_string(),
            schema_version: SEARCH_QUERY_ARTIFACT_SCHEMA_VERSION,
            saved_query_id: saved_query.saved_query_id.clone(),
            display_name: display_name.into(),
            source_class: saved_query.source_class,
            privacy_class: saved_query.privacy_class,
            retention_mode,
            sync_class,
            redaction_profile,
            retention_widening_basis,
            share_policy: saved_query.share_policy,
            migration_state: SearchArtifactMigrationState::Current,
            default_surface: saved_query.surface,
            query_session_id_ref: saved_query.query_session_id_ref.clone(),
            query_text_mode: saved_query.query_text_mode,
            query_text: saved_query.query_text.clone(),
            query_hash: saved_query.query_hash.clone(),
            scope_binding_id_ref: scope_binding_id_ref.into(),
            scope_class: saved_query.scope_class,
            stable_scope_id: saved_query.stable_scope_id.clone(),
            scope_label: saved_query.scope_label.clone(),
            captured_readiness_state: saved_query.captured_readiness_state.clone(),
            planner_version: saved_query.planner_version.clone(),
            result_semantics: SearchResultSemantics::LiveRerunRequired,
            scope_honesty_state: SearchScopeHonestyState::CapturedScopeStillCurrent,
            created_at: saved_query.created_at.clone(),
            updated_at: saved_query.updated_at.clone(),
        }
    }

    /// True when the serialized saved-query artifact contains raw query text.
    pub fn contains_raw_query_text(&self) -> bool {
        self.query_text.is_some()
    }

    /// Returns validation findings for saved-query privacy invariants.
    pub fn validate(&self) -> Vec<SearchArtifactValidationFinding> {
        let mut findings = Vec::new();
        push_missing_scope_identity(&mut findings, "stable_scope_id", &self.stable_scope_id);
        if self.query_text.is_some()
            && !literal_query_text_allowed(
                self.privacy_class,
                self.retention_mode,
                self.sync_class,
                self.redaction_profile,
                self.retention_widening_basis,
            )
        {
            findings.push(SearchArtifactValidationFinding::new(
                SearchArtifactValidationFindingKind::RawQueryMaterialNotLocalOnly,
                "query_text",
                "raw query text is present outside the local-only literal profile",
            ));
        }
        push_widening_findings(
            &mut findings,
            self.source_class,
            self.retention_mode,
            self.sync_class,
            self.retention_widening_basis,
            "saved_query",
        );
        if matches!(
            self.result_semantics,
            SearchResultSemantics::CurrentLiveResults
        ) {
            findings.push(SearchArtifactValidationFinding::new(
                SearchArtifactValidationFindingKind::CapturedArtifactClaimsCurrentResults,
                "result_semantics",
                "saved queries must rerun before claiming current live result truth",
            ));
        }
        findings
    }
}

/// Durable query-history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryHistoryEntry {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this artifact.
    pub schema_version: u32,
    /// Stable query-history identity.
    pub history_id: String,
    /// Source class for the history row.
    pub source_class: SavedQuerySourceClass,
    /// Privacy posture for query material.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Retention mode for the history row.
    pub retention_mode: SearchRetentionMode,
    /// Sync posture for the history row.
    pub sync_class: SearchSyncClass,
    /// Redaction profile for query material.
    pub redaction_profile: SearchRedactionProfile,
    /// Basis for any retention or sync widening.
    pub retention_widening_basis: SearchRetentionWideningBasis,
    /// Query-session id this history row references.
    pub query_session_id_ref: String,
    /// Saved-query id this entry was opened from, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_query_id_ref: Option<String>,
    /// Surface that produced the history entry.
    pub surface: SearchSurface,
    /// Stored text mode for the history row.
    pub stored_text_mode: QueryTextMode,
    /// Deterministic query hash when retained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hash: Option<String>,
    /// Scope binding referenced by this history row.
    pub scope_binding_id_ref: String,
    /// Captured scope class.
    pub scope_class: ScopeClass,
    /// Stable captured scope identity.
    pub stable_scope_id: String,
    /// Live-vs-captured semantics shown by history surfaces.
    pub result_semantics: SearchResultSemantics,
    /// Scope honesty state shown by history surfaces.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// Timestamp when the history row was last used.
    pub last_used_at: String,
    /// Timestamp when the history row expires, when bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl QueryHistoryEntry {
    /// Builds a query-history entry from a durable saved-query artifact.
    pub fn from_saved_query(
        history_id: impl Into<String>,
        saved_query: &SavedQuery,
        last_used_at: impl Into<String>,
        expires_at: Option<String>,
    ) -> Self {
        Self {
            record_kind: QUERY_HISTORY_ENTRY_RECORD_KIND.to_string(),
            schema_version: SEARCH_QUERY_ARTIFACT_SCHEMA_VERSION,
            history_id: history_id.into(),
            source_class: saved_query.source_class,
            privacy_class: saved_query.privacy_class,
            retention_mode: saved_query.retention_mode,
            sync_class: saved_query.sync_class,
            redaction_profile: saved_query.redaction_profile,
            retention_widening_basis: saved_query.retention_widening_basis,
            query_session_id_ref: saved_query.query_session_id_ref.clone(),
            saved_query_id_ref: Some(saved_query.saved_query_id.clone()),
            surface: saved_query.default_surface,
            stored_text_mode: saved_query.query_text_mode,
            query_hash: saved_query.query_hash.clone(),
            scope_binding_id_ref: saved_query.scope_binding_id_ref.clone(),
            scope_class: saved_query.scope_class,
            stable_scope_id: saved_query.stable_scope_id.clone(),
            result_semantics: SearchResultSemantics::LiveRerunRequired,
            scope_honesty_state: saved_query.scope_honesty_state,
            last_used_at: last_used_at.into(),
            expires_at,
        }
    }

    /// Returns validation findings for query-history privacy invariants.
    pub fn validate(&self) -> Vec<SearchArtifactValidationFinding> {
        let mut findings = Vec::new();
        push_missing_scope_identity(&mut findings, "stable_scope_id", &self.stable_scope_id);
        push_widening_findings(
            &mut findings,
            self.source_class,
            self.retention_mode,
            self.sync_class,
            self.retention_widening_basis,
            "query_history_entry",
        );
        findings
    }
}

/// Durable search deep link that reopens search intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchDeepLink {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this artifact.
    pub schema_version: u32,
    /// Stable deep-link identity.
    pub deep_link_id: String,
    /// Source class for the deep link.
    pub source_class: SavedQuerySourceClass,
    /// Privacy posture for query material.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Retention mode for the deep link.
    pub retention_mode: SearchRetentionMode,
    /// Sync posture for the deep link.
    pub sync_class: SearchSyncClass,
    /// Redaction profile for link metadata.
    pub redaction_profile: SearchRedactionProfile,
    /// Basis for any retention or sync widening.
    pub retention_widening_basis: SearchRetentionWideningBasis,
    /// Explicit schema migration state.
    pub migration_state: SearchArtifactMigrationState,
    /// Target surface to reopen.
    pub target_surface: SearchSurface,
    /// Saved-query id this link opens, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_query_id_ref: Option<String>,
    /// Query-session id this link opens.
    pub query_session_id_ref: String,
    /// Scope binding referenced by the link.
    pub scope_binding_id_ref: String,
    /// Live-vs-captured semantics shown by the open sheet.
    pub result_semantics: SearchResultSemantics,
    /// Scope honesty state shown by the open sheet.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// True when a live rerun is required before presenting current truth.
    pub rerun_required: bool,
    /// True when recipients must resolve with their own scope and permissions.
    pub recipient_re_resolves_under_current_permissions: bool,
    /// True only for unsafe links that would widen access.
    pub access_widening_allowed: bool,
    /// Expiry timestamp, when the link is bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Return anchor for focus restoration, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_anchor_ref: Option<String>,
    /// Timestamp when the link was created.
    pub created_at: String,
}

impl SearchDeepLink {
    /// Builds a deep link that reopens search intent under current permissions.
    pub fn for_saved_query(
        deep_link_id: impl Into<String>,
        saved_query: &SavedQuery,
        expires_at: Option<String>,
        created_at: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: SEARCH_DEEP_LINK_RECORD_KIND.to_string(),
            schema_version: SEARCH_QUERY_ARTIFACT_SCHEMA_VERSION,
            deep_link_id: deep_link_id.into(),
            source_class: saved_query.source_class,
            privacy_class: saved_query.privacy_class,
            retention_mode: saved_query.retention_mode,
            sync_class: saved_query.sync_class,
            redaction_profile: saved_query.redaction_profile,
            retention_widening_basis: saved_query.retention_widening_basis,
            migration_state: SearchArtifactMigrationState::Current,
            target_surface: saved_query.default_surface,
            saved_query_id_ref: Some(saved_query.saved_query_id.clone()),
            query_session_id_ref: saved_query.query_session_id_ref.clone(),
            scope_binding_id_ref: saved_query.scope_binding_id_ref.clone(),
            result_semantics: SearchResultSemantics::LiveRerunRequired,
            scope_honesty_state: SearchScopeHonestyState::RecipientMustReResolve,
            rerun_required: true,
            recipient_re_resolves_under_current_permissions: true,
            access_widening_allowed: false,
            expires_at,
            return_anchor_ref: None,
            created_at: created_at.into(),
        }
    }

    /// Returns validation findings for deep-link privacy and sharing invariants.
    pub fn validate(&self) -> Vec<SearchArtifactValidationFinding> {
        let mut findings = Vec::new();
        push_widening_findings(
            &mut findings,
            self.source_class,
            self.retention_mode,
            self.sync_class,
            self.retention_widening_basis,
            "search_deep_link",
        );
        if self.access_widening_allowed || !self.recipient_re_resolves_under_current_permissions {
            findings.push(SearchArtifactValidationFinding::new(
                SearchArtifactValidationFindingKind::DeepLinkWouldWidenAccess,
                "recipient_re_resolves_under_current_permissions",
                "search deep links must re-resolve under the recipient's current permissions",
            ));
        }
        if matches!(
            self.result_semantics,
            SearchResultSemantics::CurrentLiveResults | SearchResultSemantics::CapturedSnapshot
        ) {
            findings.push(SearchArtifactValidationFinding::new(
                SearchArtifactValidationFindingKind::CapturedArtifactClaimsCurrentResults,
                "result_semantics",
                "deep links carry search intent and must not claim current or frozen result truth",
            ));
        }
        findings
    }
}

/// Captured export snapshot for a planner-backed search result collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchCollectionSnapshot {
    /// Stable record-kind tag for schema and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this artifact.
    pub schema_version: u32,
    /// Stable snapshot identity.
    pub snapshot_id: String,
    /// Source class for the snapshot.
    pub source_class: SavedQuerySourceClass,
    /// Privacy posture for query material.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Retention mode for the snapshot.
    pub retention_mode: SearchRetentionMode,
    /// Sync posture for the snapshot.
    pub sync_class: SearchSyncClass,
    /// Redaction profile for exported data.
    pub redaction_profile: SearchRedactionProfile,
    /// Basis for any retention or sync widening.
    pub retention_widening_basis: SearchRetentionWideningBasis,
    /// Destination class for this export snapshot.
    pub destination: SearchExportDestination,
    /// Query-session id this snapshot quotes.
    pub query_session_id_ref: String,
    /// Result-set id this snapshot quotes.
    pub result_set_id_ref: String,
    /// Planner-pass id this snapshot quotes.
    pub planner_pass_id_ref: String,
    /// Scope binding referenced by the snapshot.
    pub scope_binding_id_ref: String,
    /// Captured scope class.
    pub scope_class: ScopeClass,
    /// Captured scope label.
    pub scope_label: String,
    /// Stable captured scope identity.
    pub stable_scope_id: String,
    /// Result ids selected for export.
    pub selected_result_refs: Vec<String>,
    /// Result ids included in the snapshot.
    pub included_result_refs: Vec<String>,
    /// Source labels contributed by included result rows.
    pub result_source_labels: Vec<String>,
    /// Partiality or omission reasons preserved for review.
    pub partiality_reasons: Vec<String>,
    /// Count summary preserving visible, selected, hidden, and omitted rows.
    pub count_summary: SearchPacketCountSummary,
    /// Live-versus-captured truth copied from the export packet.
    #[serde(default = "default_export_snapshot_truth")]
    pub snapshot_truth: SearchExportSnapshotTruth,
    /// Export-safe flags for omitted or truncated content/classes.
    #[serde(default)]
    pub omitted_or_truncated_flags: Vec<String>,
    /// Evidence refs shared with support, docs, AI, CLI, and replay consumers.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Live-vs-captured semantics shown by export review.
    pub result_semantics: SearchResultSemantics,
    /// Scope honesty state shown by export review.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// True when a rerun is required before presenting current result truth.
    pub current_truth_requires_rerun: bool,
    /// True when literal query text is included in the snapshot.
    pub literal_query_text_included: bool,
    /// Deterministic query hash when retained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hash: Option<String>,
    /// Timestamp when the snapshot was created.
    pub created_at: String,
}

impl SearchCollectionSnapshot {
    /// Builds a captured snapshot from an export-safe search packet.
    pub fn from_export_packet(
        snapshot_id: impl Into<String>,
        packet: SearchExportPacket,
        source_class: SavedQuerySourceClass,
        retention_mode: SearchRetentionMode,
        sync_class: SearchSyncClass,
        redaction_profile: SearchRedactionProfile,
        retention_widening_basis: SearchRetentionWideningBasis,
        scope_binding_id_ref: impl Into<String>,
    ) -> Self {
        let partiality_reasons = partiality_reasons_for_packet(&packet);
        Self {
            record_kind: SEARCH_COLLECTION_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: SEARCH_QUERY_ARTIFACT_SCHEMA_VERSION,
            snapshot_id: snapshot_id.into(),
            source_class,
            privacy_class: packet.privacy_class,
            retention_mode,
            sync_class,
            redaction_profile,
            retention_widening_basis,
            destination: packet.destination,
            query_session_id_ref: packet.query_session_id_ref,
            result_set_id_ref: packet.result_set_id_ref,
            planner_pass_id_ref: packet.planner_pass_id_ref,
            scope_binding_id_ref: scope_binding_id_ref.into(),
            scope_class: packet.scope_class,
            scope_label: packet.scope_label,
            stable_scope_id: packet.stable_scope_id,
            selected_result_refs: packet.selected_result_refs,
            included_result_refs: packet.included_result_refs,
            result_source_labels: packet.result_source_labels,
            partiality_reasons,
            count_summary: packet.count_summary,
            snapshot_truth: packet.snapshot_truth,
            omitted_or_truncated_flags: packet.omitted_or_truncated_flags,
            evidence_refs: packet.evidence_refs,
            result_semantics: SearchResultSemantics::CapturedSnapshot,
            scope_honesty_state: SearchScopeHonestyState::CapturedScopeStillCurrent,
            current_truth_requires_rerun: true,
            literal_query_text_included: packet.query_text.is_some(),
            query_hash: packet.query_hash,
            created_at: packet.exported_at,
        }
    }

    /// True when the snapshot proves a support/docs export avoided raw query text.
    pub fn export_avoids_raw_query_by_default(&self) -> bool {
        self.destination.requires_redacted_query_text()
            && !self.literal_query_text_included
            && matches!(
                self.redaction_profile,
                SearchRedactionProfile::HashesScopeAndResultRefs
                    | SearchRedactionProfile::MetadataOnlyNoQueryMaterial
                    | SearchRedactionProfile::PolicyWithheld
            )
    }

    /// Returns validation findings for export snapshot privacy invariants.
    pub fn validate(&self) -> Vec<SearchArtifactValidationFinding> {
        let mut findings = Vec::new();
        push_missing_scope_identity(&mut findings, "stable_scope_id", &self.stable_scope_id);
        push_widening_findings(
            &mut findings,
            self.source_class,
            self.retention_mode,
            self.sync_class,
            self.retention_widening_basis,
            "search_collection_snapshot",
        );
        if self.literal_query_text_included
            && (!literal_query_text_allowed(
                self.privacy_class,
                self.retention_mode,
                self.sync_class,
                self.redaction_profile,
                self.retention_widening_basis,
            ) || self.destination.requires_redacted_query_text())
        {
            findings.push(SearchArtifactValidationFinding::new(
                SearchArtifactValidationFindingKind::RawQueryMaterialNotLocalOnly,
                "literal_query_text_included",
                "export snapshots outside local replay must not include literal query text",
            ));
        }
        if self.count_summary.count_is_partial && self.partiality_reasons.is_empty() {
            findings.push(SearchArtifactValidationFinding::new(
                SearchArtifactValidationFindingKind::ExportSnapshotMissingPartialityReasons,
                "partiality_reasons",
                "partial or hidden-count snapshots must preserve a partiality reason",
            ));
        }
        if matches!(
            self.result_semantics,
            SearchResultSemantics::CurrentLiveResults
        ) {
            findings.push(SearchArtifactValidationFinding::new(
                SearchArtifactValidationFindingKind::CapturedArtifactClaimsCurrentResults,
                "result_semantics",
                "export snapshots are captured history and must not claim current result truth",
            ));
        }
        findings
    }
}

/// Inputs for materializing the complete durable search artifact set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchArtifactMaterializationInput {
    /// Stable saved-query identity.
    pub saved_query_id: String,
    /// Stable query-history identity.
    pub history_id: String,
    /// Stable scope-binding identity.
    pub scope_binding_id: String,
    /// Stable deep-link identity.
    pub deep_link_id: String,
    /// Stable collection-snapshot identity.
    pub snapshot_id: String,
    /// Short reviewable saved-query label.
    pub display_name: String,
    /// Source class for the artifacts.
    pub source_class: SavedQuerySourceClass,
    /// Privacy posture requested by the caller.
    pub privacy_class: SavedQueryPrivacyClass,
    /// Sharing policy for the saved query.
    pub share_policy: SavedQuerySharePolicy,
    /// Destination for the export snapshot.
    pub destination: SearchExportDestination,
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
    /// Optional caller-provided retention mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_mode: Option<SearchRetentionMode>,
    /// Optional caller-provided sync class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_class: Option<SearchSyncClass>,
    /// Optional caller-provided redaction profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_profile: Option<SearchRedactionProfile>,
    /// Optional caller-provided retention widening basis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_widening_basis: Option<SearchRetentionWideningBasis>,
    /// Timestamp for saved-query creation.
    pub created_at: String,
    /// Timestamp for query-history use.
    pub last_used_at: String,
    /// Timestamp for snapshot export.
    pub exported_at: String,
    /// Expiry timestamp for history/deep link, when bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Complete set of durable artifacts emitted for one saved search workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchArtifactSet {
    /// Scope binding used by the saved query, history row, link, and snapshot.
    pub scope_binding: ScopePackBinding,
    /// Durable saved query.
    pub saved_query: SavedQuery,
    /// Query-history row.
    pub history_entry: QueryHistoryEntry,
    /// Search deep link that reopens intent.
    pub deep_link: SearchDeepLink,
    /// Captured search collection snapshot.
    pub collection_snapshot: SearchCollectionSnapshot,
}

impl SearchArtifactSet {
    /// Materializes saved-query, history, deep-link, scope, and snapshot artifacts.
    pub fn materialize(
        input: SearchArtifactMaterializationInput,
    ) -> Result<Self, SearchExportError> {
        let retention_mode = input.retention_mode.unwrap_or_else(|| {
            SearchRetentionMode::default_for(input.source_class, input.privacy_class)
        });
        let sync_class = input.sync_class.unwrap_or_else(|| {
            SearchSyncClass::default_for(input.source_class, input.privacy_class)
        });
        let redaction_profile = input
            .redaction_profile
            .unwrap_or_else(|| SearchRedactionProfile::default_for(input.privacy_class));
        let retention_widening_basis = input.retention_widening_basis.unwrap_or_else(|| {
            SearchRetentionWideningBasis::default_for(
                input.source_class,
                retention_mode,
                sync_class,
            )
        });

        let scope_binding = ScopePackBinding::from_query_session(
            input.scope_binding_id.clone(),
            &input.query_session,
            input.source_class,
            input.privacy_class,
            retention_mode,
            sync_class,
            redaction_profile,
            retention_widening_basis,
            input.created_at.clone(),
        );

        let alpha_saved_query = SavedQueryRecord::from_session(SavedQueryRecordInputs {
            saved_query_id: input.saved_query_id,
            source_class: input.source_class,
            privacy_class: input.privacy_class,
            share_policy: input.share_policy,
            query_session: input.query_session.clone(),
            policy_epoch: None,
            created_at: input.created_at.clone(),
        });
        let saved_query = SavedQuery::from_saved_query_record(
            &alpha_saved_query,
            input.display_name,
            scope_binding.scope_binding_id.clone(),
            retention_mode,
            sync_class,
            redaction_profile,
            retention_widening_basis,
        );

        let history_entry = QueryHistoryEntry::from_saved_query(
            input.history_id,
            &saved_query,
            input.last_used_at,
            input.expires_at.clone(),
        );

        let deep_link = SearchDeepLink::for_saved_query(
            input.deep_link_id,
            &saved_query,
            input.expires_at,
            input.created_at,
        );

        let export_packet =
            SearchExportPacket::from_planned_result_set(SearchExportPacketInputs {
                packet_id: input.snapshot_id.clone(),
                destination: input.destination,
                privacy_class: input.privacy_class,
                query_session: input.query_session,
                result_set: input.result_set,
                selected_result_ids: input.selected_result_ids,
                scope_counts: input.scope_counts,
                exported_at: input.exported_at,
            })?;
        let collection_snapshot = SearchCollectionSnapshot::from_export_packet(
            input.snapshot_id,
            export_packet,
            input.source_class,
            retention_mode,
            sync_class,
            redaction_profile,
            retention_widening_basis,
            scope_binding.scope_binding_id.clone(),
        );

        Ok(Self {
            scope_binding,
            saved_query,
            history_entry,
            deep_link,
            collection_snapshot,
        })
    }

    /// Returns validation findings for every artifact in the set.
    pub fn validate(&self) -> Vec<SearchArtifactValidationFinding> {
        let mut findings = Vec::new();
        findings.extend(self.scope_binding.validate());
        findings.extend(self.saved_query.validate());
        findings.extend(self.history_entry.validate());
        findings.extend(self.deep_link.validate());
        findings.extend(self.collection_snapshot.validate());
        findings
    }
}

fn literal_query_text_allowed(
    privacy_class: SavedQueryPrivacyClass,
    retention_mode: SearchRetentionMode,
    sync_class: SearchSyncClass,
    redaction_profile: SearchRedactionProfile,
    retention_widening_basis: SearchRetentionWideningBasis,
) -> bool {
    if !privacy_class.permits_raw_query_text() || !redaction_profile.permits_literal_query_text() {
        return false;
    }

    if retention_mode.permits_local_literal_text()
        && matches!(sync_class, SearchSyncClass::LocalOnly)
        && matches!(redaction_profile, SearchRedactionProfile::LiteralLocalOnly)
    {
        return true;
    }

    matches!(
        redaction_profile,
        SearchRedactionProfile::ExplicitLiteralConsent
    ) && matches!(
        retention_widening_basis,
        SearchRetentionWideningBasis::ExplicitUserOptIn | SearchRetentionWideningBasis::PolicyOwned
    )
}

fn push_missing_scope_identity(
    findings: &mut Vec<SearchArtifactValidationFinding>,
    field: &'static str,
    stable_scope_id: &str,
) {
    if stable_scope_id.trim().is_empty() {
        findings.push(SearchArtifactValidationFinding::new(
            SearchArtifactValidationFindingKind::MissingScopeIdentity,
            field,
            "durable search artifacts require a stable scope identity",
        ));
    }
}

fn push_widening_findings(
    findings: &mut Vec<SearchArtifactValidationFinding>,
    source_class: SavedQuerySourceClass,
    retention_mode: SearchRetentionMode,
    sync_class: SearchSyncClass,
    retention_widening_basis: SearchRetentionWideningBasis,
    field_prefix: &'static str,
) {
    let local_user_source = matches!(
        source_class,
        SavedQuerySourceClass::UserAuthored | SavedQuerySourceClass::SessionHistory
    );
    let widened = retention_mode.widens_local_default() || sync_class.widens_local_default();
    let explicit_basis = matches!(
        retention_widening_basis,
        SearchRetentionWideningBasis::ExplicitUserOptIn | SearchRetentionWideningBasis::PolicyOwned
    );

    if local_user_source && widened && !explicit_basis {
        findings.push(SearchArtifactValidationFinding::new(
            SearchArtifactValidationFindingKind::SyncOrManagedRetentionWithoutWideningBasis,
            format!("{field_prefix}.retention_widening_basis"),
            "user-authored or history search artifacts need explicit opt-in or policy ownership before retention or sync widens",
        ));
    }
}

fn partiality_reasons_for_packet(packet: &SearchExportPacket) -> Vec<String> {
    let mut reasons = packet
        .partial_truth_causes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if packet.count_summary.hidden_by_current_scope_rows > 0 {
        reasons.insert("hidden_by_current_scope".to_string());
    }
    if packet.count_summary.hidden_by_policy_rows > 0 {
        reasons.insert("hidden_by_policy".to_string());
    }
    if packet.count_summary.hidden_by_remote_cache_rows > 0 {
        reasons.insert("hidden_by_remote_cache".to_string());
    }
    if packet.count_summary.omitted_result_count > 0 {
        reasons.insert("omitted_unselected_results".to_string());
    }
    if packet.count_summary.count_is_partial && reasons.is_empty() {
        reasons.insert("partial_or_hidden_counts_disclosed".to_string());
    }
    reasons.into_iter().collect()
}

fn default_export_snapshot_truth() -> SearchExportSnapshotTruth {
    SearchExportSnapshotTruth::CapturedSnapshot
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planner::{PlannerRankingReason, SearchPlannerAlpha, SearchPlannerInputs};

    fn planner_input() -> SearchPlannerInputs {
        serde_json::from_value(serde_json::json!({
            "query_session": {
                "record_kind": "search_query_session",
                "schema_version": 1,
                "query_session_id": "search:session:query-artifacts:unit",
                "surface": "file_search",
                "query_text_mode": "local_text",
                "query_text": "customer 1234 retry",
                "scope_class": "selected_workset",
                "stable_scope_id": "scope:workset:auth",
                "scope_mode": "sparse",
                "workset_id": "wks:auth",
                "scope_label": "Selected workset",
                "planner_version": "search-planner-alpha",
                "readiness_state": "warming",
                "index_epoch": "idx:auth:01",
                "observed_at": "mono:query-artifacts:unit:01"
            },
            "planner_pass_id": "search:planner:query-artifacts:unit",
            "result_set_id": "search:result_set:query-artifacts:unit",
            "planner_version": "search-planner-alpha",
            "observed_at": "mono:query-artifacts:unit:02",
            "path_snapshots": [{
                "path_kind": "lexical",
                "snapshot_id": "search:snapshot:query-artifacts:lexical",
                "readiness": "partial",
                "freshness": "authoritative_live",
                "index_epoch": "idx:auth:01",
                "partial_truth_causes": ["indexing_in_progress"],
                "rows": [{
                    "candidate_id": "candidate:auth-policy",
                    "canonical_id": "workspace:file:services/auth/src/policy.rs",
                    "target_kind": "file",
                    "title": "policy.rs",
                    "relative_path": "services/auth/src/policy.rs",
                    "ranking_reasons": ["lexical_path_match"]
                }]
            }]
        }))
        .expect("unit fixture must parse")
    }

    #[test]
    fn support_artifacts_default_to_redacted_hash_and_refs() {
        let output = SearchPlannerAlpha::plan(planner_input());
        let artifacts = SearchArtifactSet::materialize(SearchArtifactMaterializationInput {
            saved_query_id: "search:saved:query-artifacts:unit".to_string(),
            history_id: "search:history:query-artifacts:unit".to_string(),
            scope_binding_id: "search:scope-binding:query-artifacts:unit".to_string(),
            deep_link_id: "search:deep-link:query-artifacts:unit".to_string(),
            snapshot_id: "search:snapshot:query-artifacts:unit".to_string(),
            display_name: "Auth retry investigation".to_string(),
            source_class: SavedQuerySourceClass::SupportCaptured,
            privacy_class: SavedQueryPrivacyClass::SupportExportRedacted,
            share_policy: SavedQuerySharePolicy::SupportExportRedactedOnly,
            destination: SearchExportDestination::SupportBundle,
            query_session: output.query_session,
            result_set: output.result_set,
            selected_result_ids: Vec::new(),
            scope_counts: None,
            retention_mode: None,
            sync_class: None,
            redaction_profile: None,
            retention_widening_basis: None,
            created_at: "2026-05-18T12:00:00Z".to_string(),
            last_used_at: "2026-05-18T12:01:00Z".to_string(),
            exported_at: "2026-05-18T12:02:00Z".to_string(),
            expires_at: None,
        })
        .expect("support artifacts materialize");

        assert!(!artifacts.saved_query.contains_raw_query_text());
        assert_eq!(
            artifacts.saved_query.query_text_mode,
            QueryTextMode::HashOnly
        );
        assert_eq!(
            artifacts.collection_snapshot.result_semantics,
            SearchResultSemantics::CapturedSnapshot
        );
        assert!(artifacts
            .collection_snapshot
            .export_avoids_raw_query_by_default());
        assert!(artifacts.validate().is_empty());
    }

    #[test]
    fn unsafe_deep_link_access_widening_is_reported() {
        let output = SearchPlannerAlpha::plan(planner_input());
        let mut artifacts = SearchArtifactSet::materialize(SearchArtifactMaterializationInput {
            saved_query_id: "search:saved:query-artifacts:link".to_string(),
            history_id: "search:history:query-artifacts:link".to_string(),
            scope_binding_id: "search:scope-binding:query-artifacts:link".to_string(),
            deep_link_id: "search:deep-link:query-artifacts:link".to_string(),
            snapshot_id: "search:snapshot:query-artifacts:link".to_string(),
            display_name: "Unsafe link drill".to_string(),
            source_class: SavedQuerySourceClass::TeamShared,
            privacy_class: SavedQueryPrivacyClass::WorkspaceSharedRedacted,
            share_policy: SavedQuerySharePolicy::WorkspaceShareExplicit,
            destination: SearchExportDestination::DocsHandoff,
            query_session: output.query_session,
            result_set: output.result_set,
            selected_result_ids: Vec::new(),
            scope_counts: None,
            retention_mode: None,
            sync_class: None,
            redaction_profile: None,
            retention_widening_basis: None,
            created_at: "2026-05-18T12:00:00Z".to_string(),
            last_used_at: "2026-05-18T12:01:00Z".to_string(),
            exported_at: "2026-05-18T12:02:00Z".to_string(),
            expires_at: None,
        })
        .expect("team-shared artifacts materialize");

        artifacts.deep_link.access_widening_allowed = true;
        let findings = artifacts.deep_link.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind == SearchArtifactValidationFindingKind::DeepLinkWouldWidenAccess
        }));
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            SearchRetentionMode::LocalOnlyDefault.as_str(),
            "local_only_default"
        );
        assert_eq!(SearchSyncClass::LocalOnly.as_str(), "local_only");
        assert_eq!(
            SearchRedactionProfile::HashesScopeAndResultRefs.as_str(),
            "hashes_scope_and_result_refs"
        );
        assert_eq!(
            SearchResultSemantics::LiveRerunRequired.as_str(),
            "live_rerun_required"
        );
        assert_eq!(
            SearchArtifactValidationFindingKind::RawQueryMaterialNotLocalOnly.as_str(),
            "raw_query_material_not_local_only"
        );
        assert_eq!(
            PlannerRankingReason::LexicalPathMatch.as_str(),
            "lexical_path_match"
        );
    }
}
