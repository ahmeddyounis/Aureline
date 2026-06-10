//! Statement-safety classifier, write-mode bar, and protected-target step-up flow
//! qualification records.
//!
//! This module owns the typed records that keep SQL statement classification,
//! write-mode disclosure, and protected-target step-up gating inspectable and
//! attributable without depending on hidden shell shortcuts or ad hoc scripts.
//! The boundary schema is
//! [`/schemas/data/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.schema.json`](../../../schemas/data/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.json`](../../../artifacts/data/m5/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.json).
//!
//! Raw statement bodies, raw bind values, raw object names, raw secrets, and raw
//! connection strings do not belong in these records. They carry stable IDs,
//! closed posture vocabularies, and reviewable summaries that UI, CLI, export,
//! support, and public-proof surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for statement-safety qualification packets.
pub const STATEMENT_SAFETY_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`StatementSafetyQualificationPacket`].
pub const STATEMENT_SAFETY_QUALIFICATION_RECORD_KIND: &str =
    "add_the_statement_safety_classifier_write_mode_bar_and_protected_target_step_up_flows";

/// Repo-relative path to the checked-in statement-safety qualification packet.
pub const STATEMENT_SAFETY_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.json";

/// Embedded checked-in packet JSON.
pub const STATEMENT_SAFETY_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.json"
));

/// Qualification label shown on promoted statement-safety surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementSafetyQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl StatementSafetyQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Statement-safety surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementSafetySurfaceKind {
    /// Statement-safety classifier that labels SQL before execution.
    StatementSafetyClassifier,
    /// Write-mode bar disclosing transaction state and write posture.
    WriteModeBar,
    /// Protected-target step-up flow for mutation guard and consent.
    ProtectedTargetStepUp,
}

/// Fine-grained statement-safety class used by the classifier before execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementSafetyClass {
    /// Read-only SELECT or equivalent.
    ReadOnlyQuery,
    /// Read-only metadata introspection (e.g., INFORMATION_SCHEMA).
    ReadOnlyPureMetadataIntrospection,
    /// INSERT statement.
    DataManipulationInsert,
    /// UPDATE statement.
    DataManipulationUpdate,
    /// DELETE statement.
    DataManipulationDelete,
    /// MERGE or UPSERT statement.
    DataManipulationMergeOrUpsert,
    /// CREATE statement.
    DataDefinitionCreate,
    /// ALTER statement.
    DataDefinitionAlter,
    /// DROP statement.
    DataDefinitionDrop,
    /// TRUNCATE statement.
    DataDefinitionTruncate,
    /// GRANT or REVOKE statement.
    DataControlGrantOrRevoke,
    /// Session-level SET or equivalent.
    SessionSettingChange,
    /// BEGIN, COMMIT, ROLLBACK, or SAVEPOINT.
    TransactionControlStatement,
    /// EXPLAIN or PLAN without execution.
    ExplainOrPlanOnlyNoExecution,
    /// Stored procedure or function call with unknown side effects.
    StoredProcedureOrFunctionCallUnknownSideEffects,
    /// Multi-statement script containing mixed classes.
    MultiStatementScriptMixedClasses,
    /// Classifier cannot determine class; user review required.
    AmbiguousClassUserReviewRequired,
    /// Statement is blocked on the current connection.
    BlockedClassNotAdmissibleOnThisConnection,
}

impl StatementSafetyClass {
    /// Returns true when this class requires review or step-up before execution.
    pub const fn requires_review(self) -> bool {
        matches!(
            self,
            Self::DataManipulationInsert
                | Self::DataManipulationUpdate
                | Self::DataManipulationDelete
                | Self::DataManipulationMergeOrUpsert
                | Self::DataDefinitionCreate
                | Self::DataDefinitionAlter
                | Self::DataDefinitionDrop
                | Self::DataDefinitionTruncate
                | Self::DataControlGrantOrRevoke
                | Self::SessionSettingChange
                | Self::TransactionControlStatement
                | Self::StoredProcedureOrFunctionCallUnknownSideEffects
                | Self::MultiStatementScriptMixedClasses
                | Self::AmbiguousClassUserReviewRequired
                | Self::BlockedClassNotAdmissibleOnThisConnection
        )
    }

    /// Returns true when this class is a destructive DDL operation.
    pub const fn is_destructive_ddl(self) -> bool {
        matches!(self, Self::DataDefinitionDrop | Self::DataDefinitionTruncate)
    }

    /// Returns true when this class is read-only or explain-only.
    pub const fn is_read_only(self) -> bool {
        matches!(
            self,
            Self::ReadOnlyQuery
                | Self::ReadOnlyPureMetadataIntrospection
                | Self::ExplainOrPlanOnlyNoExecution
        )
    }
}

/// Transaction context class disclosed by the write-mode bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionContextClass {
    /// No explicit transaction; autocommit is active.
    ImplicitAutocommitNoTransaction,
    /// An explicit transaction is currently open.
    ExplicitTransactionOpen,
    /// Inside a savepoint within a larger transaction.
    SavepointWithinTransaction,
    /// This statement will open a new transaction.
    TransactionWillOpenForThisStatement,
    /// This statement will commit the transaction.
    TransactionWillCommitAfterThisStatement,
    /// This statement will roll back the transaction.
    TransactionWillRollbackAfterThisStatement,
    /// Transaction context is unknown and requires review.
    TransactionContextUnknownRequiresReview,
    /// Not applicable for explain-only statements.
    TransactionContextNotApplicableExplainOnly,
}

/// Object-impact class used for schema-refresh and review decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectImpactClass {
    /// No schema or row impact; read-only.
    NoObjectImpactReadOnly,
    /// Affects rows only; no schema change.
    RowsOnlyNoSchemaChange,
    /// Changes tables or views.
    SchemaChangeTableOrView,
    /// Changes indexes or constraints.
    SchemaChangeIndexOrConstraint,
    /// Changes roles or grants.
    SchemaChangeRoleOrGrant,
    /// Changes extensions or databases.
    SchemaChangeExtensionOrDatabase,
    /// Object impact is unknown and requires review.
    ObjectImpactUnknownRequiresReview,
    /// Not applicable for explain-only statements.
    ObjectImpactNotApplicableExplainOnly,
}

/// Multi-statement posture disclosed by the classifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultiStatementPostureClass {
    /// Single statement payload.
    SingleStatement,
    /// Multi-statement script; all statements are read-only.
    MultiStatementScriptHomogeneousReadOnly,
    /// Multi-statement script; all statements are DML.
    MultiStatementScriptHomogeneousDml,
    /// Multi-statement script; all statements are DDL.
    MultiStatementScriptHomogeneousDdl,
    /// Multi-statement script with mixed classes.
    MultiStatementScriptMixedClasses,
    /// Multi-statement script of unknown composition.
    MultiStatementScriptUnknownRequiresReview,
}

/// Reason for ambiguity when the classifier cannot determine safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmbiguityReasonClass {
    /// No ambiguity; classification is confident.
    NoAmbiguityClassificationConfident,
    /// Dialect-specific construct could not be parsed.
    DialectSpecificConstructUnparsed,
    /// Dynamic SQL or string concatenation observed.
    DynamicSqlOrStringConcatenationObserved,
    /// Stored procedure body is not visible to the classifier.
    StoredProcedureBodyNotVisibleToClassifier,
    /// User-defined function with unknown side effects.
    UserDefinedFunctionWithUnknownSideEffects,
    /// Comment-only or empty payload.
    CommentOnlyOrEmptyPayload,
    /// Ambiguity reason is unknown and requires review.
    AmbiguityReasonUnknownRequiresReview,
}

/// Reason a statement is blocked on the current connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedReasonClass {
    /// Not blocked; statement is admissible.
    NotBlockedAdmissible,
    /// Write attempted on a read-only connection.
    BlockedWriteOnReadOnlyConnection,
    /// Destructive DDL without a consent ticket.
    BlockedDestructiveDdlWithoutConsentTicket,
    /// TRUNCATE or DROP on a production target with high blast radius.
    BlockedTruncateOrDropOnProductionBlastRadiusHigh,
    /// GRANT or REVOKE outside the admin console.
    BlockedGrantOrRevokeOutsideAdminConsole,
    /// Session setting change is locked by policy.
    BlockedSessionSettingChangeLockedByPolicy,
    /// Multi-statement mixed classes without explicit user admit.
    BlockedMultiStatementMixedClassesWithoutUserAdmit,
    /// Workspace trust is pending or unset.
    BlockedPendingWorkspaceTrust,
    /// Policy epoch is expired or pending re-evaluation.
    BlockedPendingPolicy,
    /// Unknown classification requires user review.
    BlockedUnknownClassificationRequiresUserReview,
}

/// Step-up mechanism kind for protected-target flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepUpKind {
    /// Platform authenticator (WebAuthn / passkey).
    PlatformAuthenticator,
    /// System browser OAuth re-authentication.
    SystemBrowserReauth,
    /// Enterprise policy-mandated second factor.
    EnterpriseMfa,
    /// Local session password re-prompt.
    LocalSessionPassword,
    /// Admin override ticket.
    AdminOverrideTicket,
}

/// Step-up state shown in the protected-target flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepUpState {
    /// No step-up required for this statement and target.
    NotRequired,
    /// Step-up is required but not yet satisfied.
    RequiredPending,
    /// Step-up is in progress (e.g., authenticator prompt active).
    InProgress,
    /// Step-up was satisfied; statement may proceed.
    Satisfied,
    /// Step-up was denied or timed out.
    Denied,
    /// Step-up is blocked by policy.
    BlockedByPolicy,
}

/// Write authority state shown across classifier, write-mode bar, and step-up surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementSafetyWritePosture {
    /// Engine/session is constrained to read-only work.
    ReadOnly,
    /// Writes are possible after guardrails and review.
    WriteCapable,
    /// Policy blocks execution or export.
    PolicyBlocked,
}

/// Execution origin class for attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementSafetyExecutionOrigin {
    /// Desktop SQL editor.
    DesktopSqlEditor,
    /// CLI runner.
    CliRunner,
    /// AI tool review surface.
    AiToolReviewSurface,
    /// Automation run review surface.
    AutomationRunReviewSurface,
    /// Extension host runner.
    ExtensionHostRunner,
    /// Support export reader.
    SupportExportReader,
    /// Admin audit reader.
    AdminAuditReader,
    /// Hosted review reader.
    HostedReviewReader,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatementSafetyQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatementSafetySurfaceGuardSet {
    /// Statement-safety class is visible before execution.
    pub statement_safety_visible: bool,
    /// Transaction context is visible on the write-mode bar.
    pub transaction_context_visible: bool,
    /// Write posture is visible before execution.
    pub write_posture_visible: bool,
    /// Object impact is visible when available.
    pub object_impact_visible: bool,
    /// Multi-statement posture is visible for scripts.
    pub multi_statement_posture_visible: bool,
    /// Ambiguity reason is visible when classification is uncertain.
    pub ambiguity_reason_visible: bool,
    /// Blocked reason is visible when statement is blocked.
    pub blocked_reason_visible: bool,
    /// Step-up state is visible for protected targets.
    pub step_up_state_visible: bool,
    /// Consent ticket ref is visible for destructive DDL.
    pub consent_ticket_visible: bool,
}

impl StatementSafetySurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.statement_safety_visible
            && self.transaction_context_visible
            && self.write_posture_visible
            && self.object_impact_visible
            && self.multi_statement_posture_visible
            && self.ambiguity_reason_visible
            && self.blocked_reason_visible
            && self.step_up_state_visible
            && self.consent_ticket_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatementSafetySurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: StatementSafetySurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: StatementSafetyQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: StatementSafetyQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<StatementSafetyQualificationProof>,
    /// Visible guard set.
    pub guards: StatementSafetySurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// Per-statement class descriptor for multi-statement scripts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerStatementClassDescriptor {
    /// Zero-based ordinal of this sub-statement inside the script.
    pub ordinal: u32,
    /// Classified statement-safety class.
    pub statement_safety_class: StatementSafetyClass,
    /// Object-impact class for this sub-statement.
    pub object_impact_class: Option<ObjectImpactClass>,
    /// Opaque ref into the per-script literal-label registry.
    pub body_label_opaque_ref: Option<String>,
}

/// Object-impact envelope for a classified statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectImpactEnvelope {
    /// Object-impact class.
    pub object_impact_class: ObjectImpactClass,
    /// Coarse bucket count of distinct schema objects expected to be touched.
    pub affected_object_count_bucket: u32,
    /// Opaque refs into a per-classification object-label registry.
    pub affected_object_label_refs: Vec<String>,
    /// UX-visible row-estimate bucket.
    pub row_estimate_bucket: Option<String>,
    /// Reviewable sentence describing object impact.
    pub object_impact_disclosure: String,
}

/// Transaction envelope for a classified statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionEnvelope {
    /// Transaction context class.
    pub transaction_context_class: TransactionContextClass,
    /// Opaque transaction id ref.
    pub transaction_id_ref: Option<String>,
    /// Opaque savepoint id ref.
    pub savepoint_id_ref: Option<String>,
    /// Reviewable sentence describing transaction context.
    pub transaction_disclosure: String,
}

/// One statement-safety classifier row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatementSafetyClassifierRow {
    /// Stable classifier row id.
    pub classifier_id: String,
    /// Execution origin.
    pub execution_origin: StatementSafetyExecutionOrigin,
    /// Classified statement-safety class.
    pub statement_safety_class: StatementSafetyClass,
    /// Multi-statement posture.
    pub multi_statement_posture_class: MultiStatementPostureClass,
    /// Per-statement class set for multi-statement scripts.
    pub per_statement_class_set: Vec<PerStatementClassDescriptor>,
    /// Transaction envelope.
    pub transaction_context: TransactionEnvelope,
    /// Object-impact envelope.
    pub object_impact: ObjectImpactEnvelope,
    /// Ambiguity reason.
    pub ambiguity_reason_class: AmbiguityReasonClass,
    /// Blocked reason.
    pub blocked_reason_class: BlockedReasonClass,
    /// Write posture.
    pub write_posture: StatementSafetyWritePosture,
    /// Consent ticket ref for destructive DDL.
    pub consent_ticket_ref: Option<String>,
    /// Opaque body label ref.
    pub body_label_opaque_ref: Option<String>,
    /// Whether the classifier is visible in UI.
    pub visible_in_ui: bool,
}

/// One write-mode bar row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WriteModeBarRow {
    /// Stable bar row id.
    pub bar_id: String,
    /// Transaction context class.
    pub transaction_context_class: TransactionContextClass,
    /// Write posture.
    pub write_posture: StatementSafetyWritePosture,
    /// Whether autocommit risk is disclosed.
    pub autocommit_risk_disclosed: bool,
    /// Whether explicit transaction scope is shown.
    pub explicit_transaction_scope_shown: bool,
    /// Whether rollback posture is shown.
    pub rollback_posture_shown: bool,
    /// Whether the target is write-guarded.
    pub write_guarded_label_shown: bool,
    /// Whether affected schema is disclosed.
    pub affected_schema_disclosed: bool,
    /// Whether the open-mutation-review action is available.
    pub open_mutation_review_available: bool,
    /// Whether the bar is visible before execution.
    pub visible_before_execution: bool,
    /// Whether the bar is visible in UI.
    pub visible_in_ui: bool,
}

/// One protected-target step-up flow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedTargetStepUpRow {
    /// Stable step-up row id.
    pub step_up_id: String,
    /// Step-up mechanism kind.
    pub step_up_kind: StepUpKind,
    /// Step-up state.
    pub step_up_state: StepUpState,
    /// Whether the target is protected.
    pub protected_target: bool,
    /// Whether mutation-class statements trigger step-up.
    pub mutation_triggers_step_up: bool,
    /// Whether destructive DDL triggers step-up.
    pub destructive_ddl_triggers_step_up: bool,
    /// Whether ambiguous statements trigger step-up.
    pub ambiguous_triggers_step_up: bool,
    /// Whether multi-statement mixed scripts trigger step-up.
    pub multi_statement_mixed_triggers_step_up: bool,
    /// Whether the flow denies only the requested action (not the whole session).
    pub denies_action_not_session: bool,
    /// Whether audit events are emitted.
    pub audit_events_emitted: bool,
    /// Whether consent tickets are required for destructive DDL.
    pub consent_ticket_required_for_destructive_ddl: bool,
    /// Whether the flow is visible in UI.
    pub visible_in_ui: bool,
}

/// Summary counts for a statement-safety qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatementSafetyQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of classifier rows.
    pub classifier_row_count: usize,
    /// Number of write-mode bar rows.
    pub write_mode_bar_row_count: usize,
    /// Number of protected-target step-up rows.
    pub protected_target_step_up_row_count: usize,
}

/// Canonical statement-safety qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatementSafetyQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<StatementSafetySurfaceQualificationRow>,
    /// Statement-safety classifier rows.
    pub classifiers: Vec<StatementSafetyClassifierRow>,
    /// Write-mode bar rows.
    pub write_mode_bars: Vec<WriteModeBarRow>,
    /// Protected-target step-up flow rows.
    pub protected_target_step_ups: Vec<ProtectedTargetStepUpRow>,
    /// Summary counts.
    pub summary: StatementSafetyQualificationSummary,
}

impl StatementSafetyQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> StatementSafetyQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        StatementSafetyQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            classifier_row_count: self.classifiers.len(),
            write_mode_bar_row_count: self.write_mode_bars.len(),
            protected_target_step_up_row_count: self.protected_target_step_ups.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<StatementSafetyQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != STATEMENT_SAFETY_QUALIFICATION_SCHEMA_VERSION {
            violations.push(StatementSafetyQualificationViolation::SchemaVersion {
                expected: STATEMENT_SAFETY_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != STATEMENT_SAFETY_QUALIFICATION_RECORD_KIND {
            violations.push(StatementSafetyQualificationViolation::RecordKind {
                expected: STATEMENT_SAFETY_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            StatementSafetyQualificationViolationKind::Surface,
        );
        collect_ids(
            self.classifiers
                .iter()
                .map(|row| row.classifier_id.as_str()),
            &mut violations,
            StatementSafetyQualificationViolationKind::Classifier,
        );
        collect_ids(
            self.write_mode_bars.iter().map(|row| row.bar_id.as_str()),
            &mut violations,
            StatementSafetyQualificationViolationKind::WriteModeBar,
        );
        collect_ids(
            self.protected_target_step_ups
                .iter()
                .map(|row| row.step_up_id.as_str()),
            &mut violations,
            StatementSafetyQualificationViolationKind::ProtectedTargetStepUp,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        StatementSafetyQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        StatementSafetyQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    StatementSafetyQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let safety_classes: BTreeSet<_> = self
            .classifiers
            .iter()
            .map(|row| row.statement_safety_class)
            .collect();
        for required_class in [
            StatementSafetyClass::ReadOnlyQuery,
            StatementSafetyClass::ReadOnlyPureMetadataIntrospection,
            StatementSafetyClass::DataManipulationInsert,
            StatementSafetyClass::DataManipulationUpdate,
            StatementSafetyClass::DataManipulationDelete,
            StatementSafetyClass::DataManipulationMergeOrUpsert,
            StatementSafetyClass::DataDefinitionCreate,
            StatementSafetyClass::DataDefinitionAlter,
            StatementSafetyClass::DataDefinitionDrop,
            StatementSafetyClass::DataDefinitionTruncate,
            StatementSafetyClass::DataControlGrantOrRevoke,
            StatementSafetyClass::SessionSettingChange,
            StatementSafetyClass::TransactionControlStatement,
            StatementSafetyClass::ExplainOrPlanOnlyNoExecution,
            StatementSafetyClass::StoredProcedureOrFunctionCallUnknownSideEffects,
            StatementSafetyClass::MultiStatementScriptMixedClasses,
            StatementSafetyClass::AmbiguousClassUserReviewRequired,
            StatementSafetyClass::BlockedClassNotAdmissibleOnThisConnection,
        ] {
            if !safety_classes.contains(&required_class) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingStatementSafetyClass {
                        statement_safety_class: required_class,
                    },
                );
            }
        }

        let transaction_contexts: BTreeSet<_> = self
            .classifiers
            .iter()
            .map(|row| row.transaction_context.transaction_context_class)
            .chain(self.write_mode_bars.iter().map(|row| row.transaction_context_class))
            .collect();
        for required_ctx in [
            TransactionContextClass::ImplicitAutocommitNoTransaction,
            TransactionContextClass::ExplicitTransactionOpen,
            TransactionContextClass::SavepointWithinTransaction,
            TransactionContextClass::TransactionWillOpenForThisStatement,
            TransactionContextClass::TransactionWillCommitAfterThisStatement,
            TransactionContextClass::TransactionWillRollbackAfterThisStatement,
            TransactionContextClass::TransactionContextUnknownRequiresReview,
            TransactionContextClass::TransactionContextNotApplicableExplainOnly,
        ] {
            if !transaction_contexts.contains(&required_ctx) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingTransactionContextClass {
                        transaction_context_class: required_ctx,
                    },
                );
            }
        }

        let object_impacts: BTreeSet<_> = self
            .classifiers
            .iter()
            .map(|row| row.object_impact.object_impact_class)
            .collect();
        for required_impact in [
            ObjectImpactClass::NoObjectImpactReadOnly,
            ObjectImpactClass::RowsOnlyNoSchemaChange,
            ObjectImpactClass::SchemaChangeTableOrView,
            ObjectImpactClass::SchemaChangeIndexOrConstraint,
            ObjectImpactClass::SchemaChangeRoleOrGrant,
            ObjectImpactClass::SchemaChangeExtensionOrDatabase,
            ObjectImpactClass::ObjectImpactUnknownRequiresReview,
            ObjectImpactClass::ObjectImpactNotApplicableExplainOnly,
        ] {
            if !object_impacts.contains(&required_impact) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingObjectImpactClass {
                        object_impact_class: required_impact,
                    },
                );
            }
        }

        let multi_statement_postures: BTreeSet<_> = self
            .classifiers
            .iter()
            .map(|row| row.multi_statement_posture_class)
            .collect();
        for required_posture in [
            MultiStatementPostureClass::SingleStatement,
            MultiStatementPostureClass::MultiStatementScriptHomogeneousReadOnly,
            MultiStatementPostureClass::MultiStatementScriptHomogeneousDml,
            MultiStatementPostureClass::MultiStatementScriptHomogeneousDdl,
            MultiStatementPostureClass::MultiStatementScriptMixedClasses,
            MultiStatementPostureClass::MultiStatementScriptUnknownRequiresReview,
        ] {
            if !multi_statement_postures.contains(&required_posture) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingMultiStatementPostureClass {
                        multi_statement_posture_class: required_posture,
                    },
                );
            }
        }

        let ambiguity_reasons: BTreeSet<_> = self
            .classifiers
            .iter()
            .map(|row| row.ambiguity_reason_class)
            .collect();
        for required_reason in [
            AmbiguityReasonClass::NoAmbiguityClassificationConfident,
            AmbiguityReasonClass::DialectSpecificConstructUnparsed,
            AmbiguityReasonClass::DynamicSqlOrStringConcatenationObserved,
            AmbiguityReasonClass::StoredProcedureBodyNotVisibleToClassifier,
            AmbiguityReasonClass::UserDefinedFunctionWithUnknownSideEffects,
            AmbiguityReasonClass::CommentOnlyOrEmptyPayload,
            AmbiguityReasonClass::AmbiguityReasonUnknownRequiresReview,
        ] {
            if !ambiguity_reasons.contains(&required_reason) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingAmbiguityReasonClass {
                        ambiguity_reason_class: required_reason,
                    },
                );
            }
        }

        let blocked_reasons: BTreeSet<_> = self
            .classifiers
            .iter()
            .map(|row| row.blocked_reason_class)
            .collect();
        for required_reason in [
            BlockedReasonClass::NotBlockedAdmissible,
            BlockedReasonClass::BlockedWriteOnReadOnlyConnection,
            BlockedReasonClass::BlockedDestructiveDdlWithoutConsentTicket,
            BlockedReasonClass::BlockedTruncateOrDropOnProductionBlastRadiusHigh,
            BlockedReasonClass::BlockedGrantOrRevokeOutsideAdminConsole,
            BlockedReasonClass::BlockedSessionSettingChangeLockedByPolicy,
            BlockedReasonClass::BlockedMultiStatementMixedClassesWithoutUserAdmit,
            BlockedReasonClass::BlockedPendingWorkspaceTrust,
            BlockedReasonClass::BlockedPendingPolicy,
            BlockedReasonClass::BlockedUnknownClassificationRequiresUserReview,
        ] {
            if !blocked_reasons.contains(&required_reason) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingBlockedReasonClass {
                        blocked_reason_class: required_reason,
                    },
                );
            }
        }

        let write_postures: BTreeSet<_> = self
            .classifiers
            .iter()
            .map(|row| row.write_posture)
            .chain(self.write_mode_bars.iter().map(|row| row.write_posture))
            .collect();
        for required_posture in [
            StatementSafetyWritePosture::ReadOnly,
            StatementSafetyWritePosture::WriteCapable,
            StatementSafetyWritePosture::PolicyBlocked,
        ] {
            if !write_postures.contains(&required_posture) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingWritePosture {
                        write_posture: required_posture,
                    },
                );
            }
        }

        let step_up_kinds: BTreeSet<_> = self
            .protected_target_step_ups
            .iter()
            .map(|row| row.step_up_kind)
            .collect();
        for required_kind in [
            StepUpKind::PlatformAuthenticator,
            StepUpKind::SystemBrowserReauth,
            StepUpKind::EnterpriseMfa,
            StepUpKind::LocalSessionPassword,
            StepUpKind::AdminOverrideTicket,
        ] {
            if !step_up_kinds.contains(&required_kind) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingStepUpKind {
                        step_up_kind: required_kind,
                    },
                );
            }
        }

        let step_up_states: BTreeSet<_> = self
            .protected_target_step_ups
            .iter()
            .map(|row| row.step_up_state)
            .collect();
        for required_state in [
            StepUpState::NotRequired,
            StepUpState::RequiredPending,
            StepUpState::InProgress,
            StepUpState::Satisfied,
            StepUpState::Denied,
            StepUpState::BlockedByPolicy,
        ] {
            if !step_up_states.contains(&required_state) {
                violations.push(
                    StatementSafetyQualificationViolation::MissingStepUpState {
                        step_up_state: required_state,
                    },
                );
            }
        }

        for row in &self.classifiers {
            if !row.visible_in_ui {
                violations.push(
                    StatementSafetyQualificationViolation::IncompleteClassifierProjection {
                        classifier_id: row.classifier_id.clone(),
                    },
                );
            }
            if row.statement_safety_class == StatementSafetyClass::MultiStatementScriptMixedClasses
                && row.per_statement_class_set.is_empty()
            {
                violations.push(
                    StatementSafetyQualificationViolation::MissingPerStatementClassSet {
                        classifier_id: row.classifier_id.clone(),
                    },
                );
            }
            if row.statement_safety_class == StatementSafetyClass::AmbiguousClassUserReviewRequired
                && row.ambiguity_reason_class == AmbiguityReasonClass::NoAmbiguityClassificationConfident
            {
                violations.push(
                    StatementSafetyQualificationViolation::AmbiguousWithoutReason {
                        classifier_id: row.classifier_id.clone(),
                    },
                );
            }
            if row.statement_safety_class == StatementSafetyClass::BlockedClassNotAdmissibleOnThisConnection
                && row.blocked_reason_class == BlockedReasonClass::NotBlockedAdmissible
            {
                violations.push(
                    StatementSafetyQualificationViolation::BlockedWithoutReason {
                        classifier_id: row.classifier_id.clone(),
                    },
                );
            }
            if row.statement_safety_class.is_destructive_ddl()
                && row.blocked_reason_class == BlockedReasonClass::NotBlockedAdmissible
                && row.consent_ticket_ref.is_none()
            {
                violations.push(
                    StatementSafetyQualificationViolation::DestructiveDdlWithoutConsentTicket {
                        classifier_id: row.classifier_id.clone(),
                    },
                );
            }
            if row.statement_safety_class.is_read_only()
                && row.object_impact.object_impact_class != ObjectImpactClass::NoObjectImpactReadOnly
                && row.object_impact.object_impact_class != ObjectImpactClass::ObjectImpactNotApplicableExplainOnly
            {
                violations.push(
                    StatementSafetyQualificationViolation::ReadOnlyWithUnexpectedObjectImpact {
                        classifier_id: row.classifier_id.clone(),
                    },
                );
            }
        }

        for row in &self.write_mode_bars {
            if !row.visible_in_ui || !row.visible_before_execution {
                violations.push(
                    StatementSafetyQualificationViolation::IncompleteWriteModeBarProjection {
                        bar_id: row.bar_id.clone(),
                    },
                );
            }
        }

        for row in &self.protected_target_step_ups {
            if !row.visible_in_ui {
                violations.push(
                    StatementSafetyQualificationViolation::IncompleteStepUpProjection {
                        step_up_id: row.step_up_id.clone(),
                    },
                );
            }
            if row.protected_target
                && !row.mutation_triggers_step_up
                && !row.destructive_ddl_triggers_step_up
                && !row.ambiguous_triggers_step_up
                && !row.multi_statement_mixed_triggers_step_up
            {
                violations.push(
                    StatementSafetyQualificationViolation::ProtectedTargetWithoutStepUpTrigger {
                        step_up_id: row.step_up_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(StatementSafetyQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in statement-safety qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_statement_safety_qualification(
) -> Result<StatementSafetyQualificationPacket, serde_json::Error> {
    serde_json::from_str(STATEMENT_SAFETY_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatementSafetyQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Classifier rows.
    Classifier,
    /// Write-mode bar rows.
    WriteModeBar,
    /// Protected-target step-up rows.
    ProtectedTargetStepUp,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<StatementSafetyQualificationViolation>,
    kind: StatementSafetyQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(StatementSafetyQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for statement-safety qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementSafetyQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: StatementSafetyQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required statement-safety class is missing.
    MissingStatementSafetyClass {
        statement_safety_class: StatementSafetyClass,
    },
    /// Required transaction context class is missing.
    MissingTransactionContextClass {
        transaction_context_class: TransactionContextClass,
    },
    /// Required object-impact class is missing.
    MissingObjectImpactClass {
        object_impact_class: ObjectImpactClass,
    },
    /// Required multi-statement posture class is missing.
    MissingMultiStatementPostureClass {
        multi_statement_posture_class: MultiStatementPostureClass,
    },
    /// Required ambiguity reason class is missing.
    MissingAmbiguityReasonClass {
        ambiguity_reason_class: AmbiguityReasonClass,
    },
    /// Required blocked reason class is missing.
    MissingBlockedReasonClass {
        blocked_reason_class: BlockedReasonClass,
    },
    /// Required write posture is missing.
    MissingWritePosture {
        write_posture: StatementSafetyWritePosture,
    },
    /// Required step-up kind is missing.
    MissingStepUpKind {
        step_up_kind: StepUpKind,
    },
    /// Required step-up state is missing.
    MissingStepUpState {
        step_up_state: StepUpState,
    },
    /// Classifier row does not project classification truth everywhere.
    IncompleteClassifierProjection { classifier_id: String },
    /// Multi-statement mixed class row lacks per-statement class set.
    MissingPerStatementClassSet { classifier_id: String },
    /// Ambiguous classification lacks a non-confident ambiguity reason.
    AmbiguousWithoutReason { classifier_id: String },
    /// Blocked classification lacks a non-admissible blocked reason.
    BlockedWithoutReason { classifier_id: String },
    /// Destructive DDL is admitted without a consent ticket.
    DestructiveDdlWithoutConsentTicket { classifier_id: String },
    /// Read-only class paired with unexpected object impact.
    ReadOnlyWithUnexpectedObjectImpact { classifier_id: String },
    /// Write-mode bar row does not project bar truth everywhere.
    IncompleteWriteModeBarProjection { bar_id: String },
    /// Step-up row does not project step-up truth everywhere.
    IncompleteStepUpProjection { step_up_id: String },
    /// Protected target with write capability lacks step-up trigger.
    ProtectedTargetWithoutStepUpTrigger { step_up_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for StatementSafetyQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingStatementSafetyClass { statement_safety_class } => {
                write!(
                    f,
                    "statement safety class {statement_safety_class:?} is not covered"
                )
            }
            Self::MissingTransactionContextClass { transaction_context_class } => {
                write!(
                    f,
                    "transaction context class {transaction_context_class:?} is not covered"
                )
            }
            Self::MissingObjectImpactClass { object_impact_class } => {
                write!(
                    f,
                    "object impact class {object_impact_class:?} is not covered"
                )
            }
            Self::MissingMultiStatementPostureClass { multi_statement_posture_class } => {
                write!(
                    f,
                    "multi-statement posture class {multi_statement_posture_class:?} is not covered"
                )
            }
            Self::MissingAmbiguityReasonClass { ambiguity_reason_class } => {
                write!(
                    f,
                    "ambiguity reason class {ambiguity_reason_class:?} is not covered"
                )
            }
            Self::MissingBlockedReasonClass { blocked_reason_class } => {
                write!(
                    f,
                    "blocked reason class {blocked_reason_class:?} is not covered"
                )
            }
            Self::MissingWritePosture { write_posture } => {
                write!(f, "write posture {write_posture:?} is not covered")
            }
            Self::MissingStepUpKind { step_up_kind } => {
                write!(f, "step-up kind {step_up_kind:?} is not covered")
            }
            Self::MissingStepUpState { step_up_state } => {
                write!(f, "step-up state {step_up_state:?} is not covered")
            }
            Self::IncompleteClassifierProjection { classifier_id } => {
                write!(
                    f,
                    "{classifier_id} does not project classifier truth everywhere"
                )
            }
            Self::MissingPerStatementClassSet { classifier_id } => {
                write!(
                    f,
                    "{classifier_id} is multi-statement mixed without per-statement class set"
                )
            }
            Self::AmbiguousWithoutReason { classifier_id } => {
                write!(
                    f,
                    "{classifier_id} is ambiguous without a non-confident ambiguity reason"
                )
            }
            Self::BlockedWithoutReason { classifier_id } => {
                write!(
                    f,
                    "{classifier_id} is blocked without a non-admissible blocked reason"
                )
            }
            Self::DestructiveDdlWithoutConsentTicket { classifier_id } => {
                write!(
                    f,
                    "{classifier_id} is destructive DDL without a consent ticket"
                )
            }
            Self::ReadOnlyWithUnexpectedObjectImpact { classifier_id } => {
                write!(
                    f,
                    "{classifier_id} is read-only with unexpected object impact"
                )
            }
            Self::IncompleteWriteModeBarProjection { bar_id } => {
                write!(f, "{bar_id} does not project write-mode bar truth everywhere")
            }
            Self::IncompleteStepUpProjection { step_up_id } => {
                write!(f, "{step_up_id} does not project step-up truth everywhere")
            }
            Self::ProtectedTargetWithoutStepUpTrigger { step_up_id } => {
                write!(
                    f,
                    "{step_up_id} is protected and write-capable without a step-up trigger"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for StatementSafetyQualificationViolation {}
