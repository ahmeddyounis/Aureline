//! Workspace-trust gating lineage: the governed, export-safe projection
//! that finalizes workspace-trust gating across tasks, terminal, debug,
//! AI apply, and privileged extensions.
//!
//! The projection ingests a live [`TrustGatingInputs`] envelope verbatim
//! (one [`TrustSurfaceObservation`] per privileged workspace surface plus
//! the controlled inspection-hook table) and produces a stable-line
//! lineage record that proves the seven claims the workspace-trust lane
//! is anchored on:
//!
//! - **Surface coverage truth.** Every privileged workspace surface
//!   gated by workspace trust is bound to one closed
//!   [`TrustSurfaceKind`] (`tasks`, `terminal`, `debug`, `ai_apply`,
//!   `privileged_extension`), and the corpus seeds one row per kind so
//!   the trust gate is observable on every surface that can mutate or
//!   exfiltrate workspace state.
//! - **Trust-gating decision truth.** Every surface declares one
//!   [`GateDecisionClass`] drawn from a closed vocabulary, and the
//!   projection re-derives the gate from the captured workspace trust
//!   posture: a `Restricted` workspace cannot ship `allow_unconditional`
//!   and a `PendingEvaluation` workspace cannot ship anything other than
//!   a blocking / read-only gate.
//! - **No-silent-execution honesty.** Each surface declares whether it
//!   can fire silently. A surface that allows execution after an
//!   explicit grant must require explicit user action and reference a
//!   disclosure id so terminals, tasks, debuggers, AI apply, and
//!   privileged extensions never resume without review.
//! - **Override-route honesty.** Each surface declares its
//!   [`OverrideRouteClass`]; a non-`none` override must reference a
//!   disclosure id so the user can inspect what an override unlocks
//!   before it commits.
//! - **Trust-review hook honesty.** A controlled set of pre-execution
//!   inspection / repair hooks (`inspect_trust_grant`,
//!   `review_grant_scope`, `compare_workspace_trust`, `rollback_grant`,
//!   `export`, `repair`) is reachable so any destructive grant or
//!   privileged execution can be reviewed before it fires.
//! - **Support-export honesty.** Each surface's support-export
//!   projection preserves the surface kind, gate decision, override
//!   route, silent-execution posture, and disclosure id, while
//!   excluding raw secrets, approval tickets, delegated credentials,
//!   and live authority handles. Surfaces that touch credential stores
//!   must declare a non-`local_only` posture so support bundles can
//!   preserve the gating decision.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to the
//!   source corpus, the workspace, and the producer.
//!
//! In addition the record carries the producer attribution (producer
//! ref, schema version, capture timestamp, integrity hash) so replay
//! and support pipelines can pin the source before applying. When the
//! projection cannot prove a claim on the captured posture it
//! auto-narrows below Stable with a named
//! [`TrustGatingLineageNarrowReason`].

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`TrustGatingLineageRecord`].
pub const TRUST_GATING_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the trust-gating lineage record.
pub const TRUST_GATING_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/trust_gating_lineage.schema.json";

/// Stable record-kind tag for the trust-gating lineage record.
pub const TRUST_GATING_LINEAGE_RECORD_KIND: &str = "trust_gating_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the privileged workspace surfaces gated by
/// workspace trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustSurfaceKind {
    /// Task runners executing commands from workspace-bound task
    /// definitions.
    Tasks,
    /// Terminal sessions launched from the workspace shell.
    Terminal,
    /// Debug launches sourced from workspace-bound launch
    /// configurations.
    Debug,
    /// AI apply pipelines that mutate workspace files.
    AiApply,
    /// Extensions that request privileged workspace access.
    PrivilegedExtension,
}

impl TrustSurfaceKind {
    /// Returns the stable snake_case token for this surface kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tasks => "tasks",
            Self::Terminal => "terminal",
            Self::Debug => "debug",
            Self::AiApply => "ai_apply",
            Self::PrivilegedExtension => "privileged_extension",
        }
    }
}

/// Closed list of privileged surfaces every workspace-trust gate must
/// cover.
pub const REQUIRED_TRUST_SURFACES: [TrustSurfaceKind; 5] = [
    TrustSurfaceKind::Tasks,
    TrustSurfaceKind::Terminal,
    TrustSurfaceKind::Debug,
    TrustSurfaceKind::AiApply,
    TrustSurfaceKind::PrivilegedExtension,
];

/// Closed workspace-trust posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceTrustPosture {
    /// Workspace is fully trusted; privileged surfaces may run after
    /// inspection.
    Trusted,
    /// Workspace is restricted; privileged surfaces are blocked until
    /// repaired or read-only routes are taken.
    Restricted,
    /// Trust decision is pending; privileged surfaces are blocked
    /// until the user reviews the workspace.
    PendingEvaluation,
}

impl WorkspaceTrustPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::PendingEvaluation => "pending_evaluation",
        }
    }
}

/// Closed gate-decision-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateDecisionClass {
    /// Trusted workspace; the surface may run subject to its own
    /// review hooks.
    AllowUnconditional,
    /// The surface may run only after the user grants an explicit
    /// scope reviewed against a disclosure.
    AllowAfterExplicitGrant,
    /// The surface may run only in a non-mutating, inspect-only mode.
    AllowReadOnly,
    /// Trust decision is pending; the surface is blocked until the
    /// user resolves the trust review.
    BlockPendingTrustDecision,
    /// Workspace is restricted; the surface is blocked until trust is
    /// repaired.
    BlockUntilRepair,
}

impl GateDecisionClass {
    /// Returns the stable snake_case token for this gate decision.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowUnconditional => "allow_unconditional",
            Self::AllowAfterExplicitGrant => "allow_after_explicit_grant",
            Self::AllowReadOnly => "allow_read_only",
            Self::BlockPendingTrustDecision => "block_pending_trust_decision",
            Self::BlockUntilRepair => "block_until_repair",
        }
    }

    /// True when the decision allows the surface to run in some form.
    pub const fn allows_execution(self) -> bool {
        matches!(
            self,
            Self::AllowUnconditional | Self::AllowAfterExplicitGrant | Self::AllowReadOnly
        )
    }

    /// True when the decision allows mutating execution.
    pub const fn allows_mutation(self) -> bool {
        matches!(
            self,
            Self::AllowUnconditional | Self::AllowAfterExplicitGrant
        )
    }
}

/// Closed silent-execution-posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SilentExecutionPosture {
    /// Surface cannot fire silently in any mode.
    CannotFireSilently,
    /// Surface requires an explicit user action before any execution.
    ExplicitUserActionRequired,
    /// Surface is read-only; no mutation may occur.
    ReadOnlyNoMutation,
}

impl SilentExecutionPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CannotFireSilently => "cannot_fire_silently",
            Self::ExplicitUserActionRequired => "explicit_user_action_required",
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
        }
    }
}

/// Closed override-route-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideRouteClass {
    /// No override route is offered.
    None,
    /// A disclosed override available for a single invocation.
    DisclosedOneTime,
    /// A disclosed override available for the current session.
    DisclosedSession,
    /// A disclosed override that emits a durable audit record.
    DisclosedWithAudit,
}

impl OverrideRouteClass {
    /// Returns the stable snake_case token for this override route.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DisclosedOneTime => "disclosed_one_time",
            Self::DisclosedSession => "disclosed_session",
            Self::DisclosedWithAudit => "disclosed_with_audit",
        }
    }

    /// True when the override route is non-`none`.
    pub const fn has_override(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Closed support-export-posture vocabulary (mirrors the workspace
/// state-package vocabulary so support bundles can share posture
/// classifications across lineages).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustSupportExportPosture {
    /// Surface stays local-only; the support packet redacts the
    /// surface's gating state entirely.
    LocalOnly,
    /// Surface ships a metadata-safe projection of its gating state
    /// in the support packet.
    MetadataSafeExport,
    /// Surface withholds the gating state from the support packet
    /// until a manual export reviews it.
    HeldRecord,
}

impl TrustSupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// Class of pre-execution inspection / repair hook available before a
/// privileged surface runs or a trust grant is committed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustInspectionHookClass {
    /// Open the trust-grant inspector for the workspace.
    InspectTrustGrant,
    /// Open the grant-scope review sheet before any privileged action.
    ReviewGrantScope,
    /// Compare the workspace trust posture across recent observations.
    CompareWorkspaceTrust,
    /// Capture a one-step rollback before committing a trust grant.
    RollbackGrant,
    /// Export the lineage record (support-safe, no raw secrets).
    Export,
    /// Open the repair sheet for a restricted workspace.
    Repair,
}

impl TrustInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectTrustGrant => "inspect_trust_grant",
            Self::ReviewGrantScope => "review_grant_scope",
            Self::CompareWorkspaceTrust => "compare_workspace_trust",
            Self::RollbackGrant => "rollback_grant",
            Self::Export => "export",
            Self::Repair => "repair",
        }
    }
}

/// One pre-execution inspection / repair hook offered before the
/// surface commits to a privileged execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustInspectionHook {
    /// Hook class.
    pub hook_class: TrustInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable on this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-execution inspection / repair hook table.
pub fn default_trust_gating_inspection_hooks() -> Vec<TrustInspectionHook> {
    vec![
        TrustInspectionHook {
            hook_class: TrustInspectionHookClass::InspectTrustGrant,
            action_id: "trust_gating.inspect_trust_grant".to_owned(),
            label: "Inspect workspace trust grant".to_owned(),
            available: true,
            disclosure:
                "Opens the trust-grant inspector with the workspace posture, recent grant history, and which privileged surfaces are gated."
                    .to_owned(),
        },
        TrustInspectionHook {
            hook_class: TrustInspectionHookClass::ReviewGrantScope,
            action_id: "trust_gating.review_grant_scope".to_owned(),
            label: "Review grant scope".to_owned(),
            available: true,
            disclosure:
                "Opens the grant-scope review sheet so any privileged execution can be reviewed before it fires."
                    .to_owned(),
        },
        TrustInspectionHook {
            hook_class: TrustInspectionHookClass::CompareWorkspaceTrust,
            action_id: "trust_gating.compare_workspace_trust".to_owned(),
            label: "Compare workspace trust history".to_owned(),
            available: true,
            disclosure:
                "Produces a reviewable diff between the prior trust posture and the current grant so the user can see what changed before applying."
                    .to_owned(),
        },
        TrustInspectionHook {
            hook_class: TrustInspectionHookClass::RollbackGrant,
            action_id: "trust_gating.rollback_grant".to_owned(),
            label: "Rollback trust grant".to_owned(),
            available: true,
            disclosure:
                "Captures a one-step rollback so the user can revert a trust grant if a privileged surface misbehaves."
                    .to_owned(),
        },
        TrustInspectionHook {
            hook_class: TrustInspectionHookClass::Export,
            action_id: "trust_gating.export".to_owned(),
            label: "Export trust-gating lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this trust-gating lineage record for support without raw secrets, approval tickets, or delegated credentials."
                    .to_owned(),
        },
        TrustInspectionHook {
            hook_class: TrustInspectionHookClass::Repair,
            action_id: "trust_gating.repair".to_owned(),
            label: "Open repair sheet".to_owned(),
            available: true,
            disclosure:
                "Opens the repair sheet for a restricted workspace and surfaces the manual remediation steps."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for a privileged
/// surface's trust-gating row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustSupportExportInputs {
    pub posture: TrustSupportExportPosture,
    pub includes_surface_kind: bool,
    pub includes_gate_decision: bool,
    pub includes_override_route: bool,
    pub includes_silent_execution_posture: bool,
    pub includes_disclosure_id: bool,
    pub raw_secrets_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
}

impl TrustSupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: TrustSupportExportPosture) -> Self {
        Self {
            posture,
            includes_surface_kind: true,
            includes_gate_decision: true,
            includes_override_route: true,
            includes_silent_execution_posture: true,
            includes_disclosure_id: true,
            raw_secrets_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
        }
    }
}

/// One observation of a privileged workspace surface at a captured
/// moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustSurfaceObservation {
    /// Stable surface id (route-style, e.g. `tasks.run`).
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// Closed surface kind.
    pub surface_kind: TrustSurfaceKind,
    /// Declared gate decision.
    pub declared_gate_decision: GateDecisionClass,
    /// Declared silent-execution posture.
    pub silent_execution_posture: SilentExecutionPosture,
    /// True when the surface requires an explicit user action before
    /// firing.
    pub explicit_user_action_required: bool,
    /// Stable disclosure id presented to the user; empty when no
    /// disclosure is required.
    pub disclosure_id: String,
    /// Declared override route.
    pub override_route: OverrideRouteClass,
    /// Stable override action id; empty when `override_route` is
    /// `none`.
    pub override_action_id: String,
    /// Stable override disclosure id; empty when `override_route` is
    /// `none`.
    pub override_disclosure_id: String,
    /// Whether the surface touches a credential store (and therefore
    /// must ship a non-local-only support-export posture).
    pub touches_credential_store: bool,
    /// Support-export projection.
    pub support_export: TrustSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustGatingInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Captured workspace trust posture.
    pub workspace_trust_posture: WorkspaceTrustPosture,
    /// Captured privileged-surface observations.
    pub surfaces: Vec<TrustSurfaceObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a trust-gating lineage record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustGatingLineageNarrowReason {
    /// The captured input had no surface observations.
    CorpusEmpty,
    /// A required privileged surface is missing from the corpus.
    RequiredTrustSurfaceMissing,
    /// A `restricted` workspace declared `allow_unconditional` on a
    /// surface.
    RestrictedWorkspaceAllowsUnconditional,
    /// A `pending_evaluation` workspace declared a decision other than
    /// blocking / read-only on a surface.
    PendingWorkspaceAllowsExecution,
    /// A surface declared `allow_unconditional` while the workspace
    /// trust posture was not `trusted`.
    UnconditionalAllowWithoutTrustedPosture,
    /// A surface declared `allow_after_explicit_grant` but did not
    /// require explicit user action or omitted a disclosure id.
    SilentGrantWithoutDisclosure,
    /// A surface declared a non-`none` override route but did not
    /// reference a disclosure id.
    OverrideRouteUndisclosed,
    /// A surface declared `allow_read_only` but did not declare the
    /// `read_only_no_mutation` silent-execution posture.
    ReadOnlyMissingPosture,
    /// A required pre-execution inspection / repair hook is
    /// unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required gating field.
    SupportExportFieldsDropped,
    /// Raw secrets, approval tickets, delegated credentials, or live
    /// authority handles slipped into a support-export projection.
    SupportExportRedactionUnsafe,
    /// A credential-touching surface declared `local_only` support
    /// export.
    SupportExportPostureUnsafe,
    /// Producer attribution is incomplete (producer ref / captured-at).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl TrustGatingLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredTrustSurfaceMissing => "required_trust_surface_missing",
            Self::RestrictedWorkspaceAllowsUnconditional => {
                "restricted_workspace_allows_unconditional"
            }
            Self::PendingWorkspaceAllowsExecution => "pending_workspace_allows_execution",
            Self::UnconditionalAllowWithoutTrustedPosture => {
                "unconditional_allow_without_trusted_posture"
            }
            Self::SilentGrantWithoutDisclosure => "silent_grant_without_disclosure",
            Self::OverrideRouteUndisclosed => "override_route_undisclosed",
            Self::ReadOnlyMissingPosture => "read_only_missing_posture",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::SupportExportPostureUnsafe => "support_export_posture_unsafe",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a trust-gating lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustGatingLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<TrustGatingLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One trust-gating surface row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustGatingSurfaceRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Privileged-surface kind.
    pub surface_kind: TrustSurfaceKind,
    /// Re-derived gate decision (worst-case for the workspace posture).
    pub derived_gate_decision: GateDecisionClass,
    /// Gate decision declared on the captured row.
    pub declared_gate_decision: GateDecisionClass,
    /// True when the declared decision matches the re-derived decision
    /// or is a stricter blocking / read-only decision.
    pub gate_decision_matches: bool,
    /// Silent-execution posture declared on the captured row.
    pub silent_execution_posture: SilentExecutionPosture,
    /// True when the surface requires an explicit user action.
    pub explicit_user_action_required: bool,
    /// True when a disclosure id was provided.
    pub has_disclosure: bool,
    /// True when an override route is offered.
    pub has_override_route: bool,
    /// True when the override route carries a disclosure.
    pub override_has_disclosure: bool,
    /// True when the surface touches a credential store.
    pub touches_credential_store: bool,
    /// Support-export posture for this surface.
    pub support_export_posture: TrustSupportExportPosture,
}

/// Surface coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustSurfaceCoverageSummary {
    /// All surface rows carried by the corpus.
    pub surface_rows: Vec<TrustGatingSurfaceRow>,
    /// True when every required privileged surface kind is present.
    pub all_required_surfaces_present: bool,
}

/// Gate-decision posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateDecisionTruthSummary {
    /// Number of allow-unconditional surfaces.
    pub allow_unconditional_count: usize,
    /// Number of allow-after-explicit-grant surfaces.
    pub allow_after_explicit_grant_count: usize,
    /// Number of allow-read-only surfaces.
    pub allow_read_only_count: usize,
    /// Number of block-pending-trust-decision surfaces.
    pub block_pending_trust_decision_count: usize,
    /// Number of block-until-repair surfaces.
    pub block_until_repair_count: usize,
    /// True when no surface declares `allow_unconditional` outside of
    /// a `trusted` workspace.
    pub no_unconditional_allow_outside_trusted: bool,
    /// True when a `restricted` workspace blocks or read-only-routes
    /// every surface.
    pub restricted_workspace_blocks_or_routes_every_surface: bool,
    /// True when a `pending_evaluation` workspace blocks or
    /// read-only-routes every surface.
    pub pending_workspace_blocks_or_routes_every_surface: bool,
}

/// Silent-execution posture honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SilentExecutionHonestySummary {
    /// True when every surface that allows execution after an
    /// explicit grant requires explicit user action.
    pub all_grant_surfaces_require_explicit_user_action: bool,
    /// True when every surface that allows execution after an
    /// explicit grant references a disclosure id.
    pub all_grant_surfaces_reference_disclosure: bool,
    /// True when every surface restricted to read-only mode declares
    /// the `read_only_no_mutation` posture.
    pub all_read_only_surfaces_declare_no_mutation_posture: bool,
}

/// Override-route honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverrideRouteHonestySummary {
    /// True when every non-`none` override route references both an
    /// action id and a disclosure id.
    pub all_override_routes_disclosed: bool,
    /// Number of surfaces that ship a non-`none` override.
    pub override_route_count: usize,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustSupportExportHonestySummary {
    /// True when every surface's support-export projection preserves
    /// surface_kind, gate_decision, override_route, silent-execution
    /// posture, and disclosure id.
    pub all_surfaces_preserve_gating_fields: bool,
    /// True when every surface declares
    /// `raw_secrets_excluded = true`.
    pub all_surfaces_redact_raw_secrets: bool,
    /// True when every surface declares
    /// `approval_tickets_excluded = true`.
    pub all_surfaces_exclude_approval_tickets: bool,
    /// True when every surface declares
    /// `delegated_credentials_excluded = true`.
    pub all_surfaces_exclude_delegated_credentials: bool,
    /// True when every surface declares
    /// `live_authority_handles_excluded = true`.
    pub all_surfaces_exclude_live_authority_handles: bool,
    /// True when every credential-touching surface declares a
    /// non-`local_only` support-export posture.
    pub all_credential_surfaces_have_safe_posture: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustProducerAttributionSummary {
    /// Opaque producer build / instance ref.
    pub producer_ref: String,
    /// Schema version pinned by the input.
    pub schema_version: u32,
    /// Opaque integrity hash derived from the input surface identities.
    pub integrity_hash: String,
    /// Input capture timestamp.
    pub captured_at: String,
    /// True when producer attribution fields are non-empty.
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe trust-gating lineage record per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustGatingLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub trust_gating_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque ref to the corpus the projection ingested.
    pub corpus_ref: String,
    /// Captured workspace trust posture.
    pub workspace_trust_posture: WorkspaceTrustPosture,
    /// Producer attribution pillar.
    pub producer_attribution: TrustProducerAttributionSummary,
    /// Surface coverage pillar.
    pub surface_coverage: TrustSurfaceCoverageSummary,
    /// Gate-decision truth pillar.
    pub gate_decision_truth: GateDecisionTruthSummary,
    /// Silent-execution honesty pillar.
    pub silent_execution_honesty: SilentExecutionHonestySummary,
    /// Override-route honesty pillar.
    pub override_route_honesty: OverrideRouteHonestySummary,
    /// Support-export honesty pillar.
    pub support_export_honesty: TrustSupportExportHonestySummary,
    /// Pre-execution inspection / repair hooks.
    pub inspection_hooks: Vec<TrustInspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: TrustGatingLineageQualification,
    /// Whether the record is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl TrustGatingLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == TRUST_GATING_LINEAGE_SCHEMA_REF
            && self.record_kind == TRUST_GATING_LINEAGE_RECORD_KIND
            && !self.workspace_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the
    /// claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(&self, class: TrustInspectionHookClass) -> Option<&TrustInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed trust-gating lineage record from a live
/// [`TrustGatingInputs`] envelope using the default inspection-hook
/// set.
pub fn project_trust_gating_lineage(
    posture_id: impl Into<String>,
    inputs: &TrustGatingInputs,
) -> TrustGatingLineageRecord {
    project_trust_gating_lineage_with_hooks(
        posture_id,
        inputs,
        default_trust_gating_inspection_hooks(),
    )
}

/// Like [`project_trust_gating_lineage`] but with an explicit
/// inspection-hook set (for testing degraded-hook postures).
pub fn project_trust_gating_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &TrustGatingInputs,
    inspection_hooks: Vec<TrustInspectionHook>,
) -> TrustGatingLineageRecord {
    let posture_id: String = posture_id.into();

    let surface_coverage = project_surface_coverage(inputs);
    let gate_decision_truth =
        project_gate_decision_truth(&surface_coverage, inputs.workspace_trust_posture);
    let silent_execution_honesty = project_silent_execution_honesty(inputs);
    let override_route_honesty = project_override_route_honesty(inputs);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let mut narrow_reasons = Vec::new();

    if inputs.surfaces.is_empty() {
        narrow_reasons.push(TrustGatingLineageNarrowReason::CorpusEmpty);
    }
    if !surface_coverage.all_required_surfaces_present {
        narrow_reasons.push(TrustGatingLineageNarrowReason::RequiredTrustSurfaceMissing);
    }

    collect_gate_narrows(
        &gate_decision_truth,
        inputs.workspace_trust_posture,
        &mut narrow_reasons,
    );
    collect_silent_execution_narrows(&silent_execution_honesty, &mut narrow_reasons);
    collect_override_route_narrows(&override_route_honesty, &mut narrow_reasons);
    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    let required_hooks = [
        TrustInspectionHookClass::InspectTrustGrant,
        TrustInspectionHookClass::ReviewGrantScope,
        TrustInspectionHookClass::CompareWorkspaceTrust,
        TrustInspectionHookClass::RollbackGrant,
        TrustInspectionHookClass::Export,
        TrustInspectionHookClass::Repair,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(TrustGatingLineageNarrowReason::InspectionHookUnavailable);
    }

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons.push(TrustGatingLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(TrustGatingLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = TrustGatingLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        inputs.workspace_trust_posture,
        &surface_coverage,
        &gate_decision_truth,
        &stable_qualification,
    );

    TrustGatingLineageRecord {
        record_kind: TRUST_GATING_LINEAGE_RECORD_KIND.to_owned(),
        trust_gating_lineage_schema_version: TRUST_GATING_LINEAGE_SCHEMA_VERSION,
        schema_ref: TRUST_GATING_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        workspace_trust_posture: inputs.workspace_trust_posture,
        producer_attribution,
        surface_coverage,
        gate_decision_truth,
        silent_execution_honesty,
        override_route_honesty,
        support_export_honesty,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_surface_coverage(inputs: &TrustGatingInputs) -> TrustSurfaceCoverageSummary {
    let surface_rows: Vec<TrustGatingSurfaceRow> = inputs
        .surfaces
        .iter()
        .map(|surface| project_surface_row(surface, inputs.workspace_trust_posture))
        .collect();
    let observed: BTreeSet<_> = surface_rows.iter().map(|row| row.surface_kind).collect();
    let all_required_surfaces_present = REQUIRED_TRUST_SURFACES
        .iter()
        .all(|required| observed.contains(required));
    TrustSurfaceCoverageSummary {
        surface_rows,
        all_required_surfaces_present,
    }
}

fn project_surface_row(
    surface: &TrustSurfaceObservation,
    workspace_posture: WorkspaceTrustPosture,
) -> TrustGatingSurfaceRow {
    let derived = derive_gate_decision(workspace_posture, surface);
    let gate_decision_matches =
        decision_at_least_as_strict(surface.declared_gate_decision, derived, workspace_posture);
    TrustGatingSurfaceRow {
        surface_id: surface.surface_id.clone(),
        title: surface.title.clone(),
        surface_kind: surface.surface_kind,
        derived_gate_decision: derived,
        declared_gate_decision: surface.declared_gate_decision,
        gate_decision_matches,
        silent_execution_posture: surface.silent_execution_posture,
        explicit_user_action_required: surface.explicit_user_action_required,
        has_disclosure: !surface.disclosure_id.trim().is_empty(),
        has_override_route: surface.override_route.has_override(),
        override_has_disclosure: !surface.override_disclosure_id.trim().is_empty()
            && !surface.override_action_id.trim().is_empty(),
        touches_credential_store: surface.touches_credential_store,
        support_export_posture: surface.support_export.posture,
    }
}

fn derive_gate_decision(
    workspace_posture: WorkspaceTrustPosture,
    surface: &TrustSurfaceObservation,
) -> GateDecisionClass {
    match workspace_posture {
        WorkspaceTrustPosture::Trusted => surface.declared_gate_decision,
        WorkspaceTrustPosture::Restricted => match surface.declared_gate_decision {
            GateDecisionClass::AllowReadOnly => GateDecisionClass::AllowReadOnly,
            _ => GateDecisionClass::BlockUntilRepair,
        },
        WorkspaceTrustPosture::PendingEvaluation => match surface.declared_gate_decision {
            GateDecisionClass::AllowReadOnly => GateDecisionClass::AllowReadOnly,
            _ => GateDecisionClass::BlockPendingTrustDecision,
        },
    }
}

fn decision_at_least_as_strict(
    declared: GateDecisionClass,
    derived: GateDecisionClass,
    workspace_posture: WorkspaceTrustPosture,
) -> bool {
    match workspace_posture {
        WorkspaceTrustPosture::Trusted => declared == derived,
        WorkspaceTrustPosture::Restricted => matches!(
            declared,
            GateDecisionClass::AllowReadOnly | GateDecisionClass::BlockUntilRepair
        ),
        WorkspaceTrustPosture::PendingEvaluation => matches!(
            declared,
            GateDecisionClass::AllowReadOnly | GateDecisionClass::BlockPendingTrustDecision
        ),
    }
}

fn project_gate_decision_truth(
    coverage: &TrustSurfaceCoverageSummary,
    workspace_posture: WorkspaceTrustPosture,
) -> GateDecisionTruthSummary {
    let mut allow_unconditional_count = 0usize;
    let mut allow_after_explicit_grant_count = 0usize;
    let mut allow_read_only_count = 0usize;
    let mut block_pending_trust_decision_count = 0usize;
    let mut block_until_repair_count = 0usize;
    let mut no_unconditional_allow_outside_trusted = true;
    let mut restricted_workspace_blocks_or_routes_every_surface = true;
    let mut pending_workspace_blocks_or_routes_every_surface = true;

    for row in &coverage.surface_rows {
        match row.declared_gate_decision {
            GateDecisionClass::AllowUnconditional => allow_unconditional_count += 1,
            GateDecisionClass::AllowAfterExplicitGrant => allow_after_explicit_grant_count += 1,
            GateDecisionClass::AllowReadOnly => allow_read_only_count += 1,
            GateDecisionClass::BlockPendingTrustDecision => block_pending_trust_decision_count += 1,
            GateDecisionClass::BlockUntilRepair => block_until_repair_count += 1,
        }

        if row.declared_gate_decision == GateDecisionClass::AllowUnconditional
            && workspace_posture != WorkspaceTrustPosture::Trusted
        {
            no_unconditional_allow_outside_trusted = false;
        }

        if workspace_posture == WorkspaceTrustPosture::Restricted
            && !matches!(
                row.declared_gate_decision,
                GateDecisionClass::AllowReadOnly | GateDecisionClass::BlockUntilRepair
            )
        {
            restricted_workspace_blocks_or_routes_every_surface = false;
        }

        if workspace_posture == WorkspaceTrustPosture::PendingEvaluation
            && !matches!(
                row.declared_gate_decision,
                GateDecisionClass::AllowReadOnly | GateDecisionClass::BlockPendingTrustDecision
            )
        {
            pending_workspace_blocks_or_routes_every_surface = false;
        }
    }

    GateDecisionTruthSummary {
        allow_unconditional_count,
        allow_after_explicit_grant_count,
        allow_read_only_count,
        block_pending_trust_decision_count,
        block_until_repair_count,
        no_unconditional_allow_outside_trusted,
        restricted_workspace_blocks_or_routes_every_surface,
        pending_workspace_blocks_or_routes_every_surface,
    }
}

fn project_silent_execution_honesty(inputs: &TrustGatingInputs) -> SilentExecutionHonestySummary {
    let mut all_grant_surfaces_require_explicit_user_action = true;
    let mut all_grant_surfaces_reference_disclosure = true;
    let mut all_read_only_surfaces_declare_no_mutation_posture = true;

    for surface in &inputs.surfaces {
        if surface.declared_gate_decision == GateDecisionClass::AllowAfterExplicitGrant {
            if !surface.explicit_user_action_required {
                all_grant_surfaces_require_explicit_user_action = false;
            }
            if surface.disclosure_id.trim().is_empty() {
                all_grant_surfaces_reference_disclosure = false;
            }
        }
        if surface.declared_gate_decision == GateDecisionClass::AllowReadOnly
            && surface.silent_execution_posture != SilentExecutionPosture::ReadOnlyNoMutation
        {
            all_read_only_surfaces_declare_no_mutation_posture = false;
        }
    }

    SilentExecutionHonestySummary {
        all_grant_surfaces_require_explicit_user_action,
        all_grant_surfaces_reference_disclosure,
        all_read_only_surfaces_declare_no_mutation_posture,
    }
}

fn project_override_route_honesty(inputs: &TrustGatingInputs) -> OverrideRouteHonestySummary {
    let mut override_route_count = 0usize;
    let mut all_override_routes_disclosed = true;
    for surface in &inputs.surfaces {
        if surface.override_route.has_override() {
            override_route_count += 1;
            if surface.override_action_id.trim().is_empty()
                || surface.override_disclosure_id.trim().is_empty()
            {
                all_override_routes_disclosed = false;
            }
        }
    }
    OverrideRouteHonestySummary {
        all_override_routes_disclosed,
        override_route_count,
    }
}

fn project_support_export_honesty(inputs: &TrustGatingInputs) -> TrustSupportExportHonestySummary {
    let mut all_surfaces_preserve_gating_fields = true;
    let mut all_surfaces_redact_raw_secrets = true;
    let mut all_surfaces_exclude_approval_tickets = true;
    let mut all_surfaces_exclude_delegated_credentials = true;
    let mut all_surfaces_exclude_live_authority_handles = true;
    let mut all_credential_surfaces_have_safe_posture = true;

    for surface in &inputs.surfaces {
        let support = surface.support_export;
        if !(support.includes_surface_kind
            && support.includes_gate_decision
            && support.includes_override_route
            && support.includes_silent_execution_posture
            && support.includes_disclosure_id)
        {
            all_surfaces_preserve_gating_fields = false;
        }
        if !support.raw_secrets_excluded {
            all_surfaces_redact_raw_secrets = false;
        }
        if !support.approval_tickets_excluded {
            all_surfaces_exclude_approval_tickets = false;
        }
        if !support.delegated_credentials_excluded {
            all_surfaces_exclude_delegated_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            all_surfaces_exclude_live_authority_handles = false;
        }
        if surface.touches_credential_store
            && support.posture == TrustSupportExportPosture::LocalOnly
        {
            all_credential_surfaces_have_safe_posture = false;
        }
    }

    TrustSupportExportHonestySummary {
        all_surfaces_preserve_gating_fields,
        all_surfaces_redact_raw_secrets,
        all_surfaces_exclude_approval_tickets,
        all_surfaces_exclude_delegated_credentials,
        all_surfaces_exclude_live_authority_handles,
        all_credential_surfaces_have_safe_posture,
    }
}

fn project_producer_attribution(inputs: &TrustGatingInputs) -> TrustProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    TrustProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: TRUST_GATING_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_gate_narrows(
    summary: &GateDecisionTruthSummary,
    workspace_posture: WorkspaceTrustPosture,
    narrow_reasons: &mut Vec<TrustGatingLineageNarrowReason>,
) {
    if !summary.no_unconditional_allow_outside_trusted {
        narrow_reasons
            .push(TrustGatingLineageNarrowReason::UnconditionalAllowWithoutTrustedPosture);
    }
    if workspace_posture == WorkspaceTrustPosture::Restricted
        && !summary.restricted_workspace_blocks_or_routes_every_surface
    {
        narrow_reasons.push(TrustGatingLineageNarrowReason::RestrictedWorkspaceAllowsUnconditional);
    }
    if workspace_posture == WorkspaceTrustPosture::PendingEvaluation
        && !summary.pending_workspace_blocks_or_routes_every_surface
    {
        narrow_reasons.push(TrustGatingLineageNarrowReason::PendingWorkspaceAllowsExecution);
    }
}

fn collect_silent_execution_narrows(
    summary: &SilentExecutionHonestySummary,
    narrow_reasons: &mut Vec<TrustGatingLineageNarrowReason>,
) {
    if !(summary.all_grant_surfaces_require_explicit_user_action
        && summary.all_grant_surfaces_reference_disclosure)
    {
        narrow_reasons.push(TrustGatingLineageNarrowReason::SilentGrantWithoutDisclosure);
    }
    if !summary.all_read_only_surfaces_declare_no_mutation_posture {
        narrow_reasons.push(TrustGatingLineageNarrowReason::ReadOnlyMissingPosture);
    }
}

fn collect_override_route_narrows(
    summary: &OverrideRouteHonestySummary,
    narrow_reasons: &mut Vec<TrustGatingLineageNarrowReason>,
) {
    if !summary.all_override_routes_disclosed {
        narrow_reasons.push(TrustGatingLineageNarrowReason::OverrideRouteUndisclosed);
    }
}

fn collect_support_export_narrows(
    summary: &TrustSupportExportHonestySummary,
    narrow_reasons: &mut Vec<TrustGatingLineageNarrowReason>,
) {
    if !summary.all_surfaces_preserve_gating_fields {
        narrow_reasons.push(TrustGatingLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !summary.all_credential_surfaces_have_safe_posture {
        narrow_reasons.push(TrustGatingLineageNarrowReason::SupportExportPostureUnsafe);
    }
    if !(summary.all_surfaces_redact_raw_secrets
        && summary.all_surfaces_exclude_approval_tickets
        && summary.all_surfaces_exclude_delegated_credentials
        && summary.all_surfaces_exclude_live_authority_handles)
    {
        narrow_reasons.push(TrustGatingLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn compute_integrity_hash(inputs: &TrustGatingInputs) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let header = [
        inputs.workspace_ref.as_str(),
        inputs.producer_ref.as_str(),
        inputs.corpus_ref.as_str(),
        inputs.captured_at.as_str(),
        inputs.workspace_trust_posture.as_str(),
    ];
    for input in header {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for surface in &inputs.surfaces {
        for byte in surface.surface_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(surface.surface_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(surface.declared_gate_decision.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("tgl:{hash:016x}")
}

fn hook_available(hooks: &[TrustInspectionHook], class: TrustInspectionHookClass) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    workspace_posture: WorkspaceTrustPosture,
    coverage: &TrustSurfaceCoverageSummary,
    gate: &GateDecisionTruthSummary,
    qualification: &TrustGatingLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Trust-gating lineage proven Stable: workspace_trust={trust} surfaces={total} allow_unconditional={uncond} allow_after_grant={grant} allow_read_only={ro} block_pending={pending} block_until_repair={repair}.",
            trust = workspace_posture.as_str(),
            total = coverage.surface_rows.len(),
            uncond = gate.allow_unconditional_count,
            grant = gate.allow_after_explicit_grant_count,
            ro = gate.allow_read_only_count,
            pending = gate.block_pending_trust_decision_count,
            repair = gate.block_until_repair_count,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Trust-gating lineage narrowed below Stable (workspace_trust={trust}, surfaces={total}): {reasons}.",
            trust = workspace_posture.as_str(),
            total = coverage.surface_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a trust-gating lineage
/// record. The same projection is consumed by the workspace
/// trust-gating status surface, the headless CLI emitter, Help/About,
/// and support export.
pub fn trust_gating_lineage_lines(record: &TrustGatingLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Trust-gating lineage — {} ({})",
        record.posture_id, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} corpus={} producer={} integrity_hash={} captured_at={} workspace_trust={}",
        record.workspace_ref,
        record.corpus_ref,
        record.producer_attribution.producer_ref,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.captured_at,
        record.workspace_trust_posture.as_str(),
    ));
    lines.push(format!(
        "surface_coverage: surfaces={} required_surfaces_present={}",
        record.surface_coverage.surface_rows.len(),
        record.surface_coverage.all_required_surfaces_present,
    ));
    lines.push("Surface rows:".to_owned());
    for row in &record.surface_coverage.surface_rows {
        lines.push(format!(
            "  - {kind} {id} declared={declared} derived={derived} matches={matches} silent={silent} action_required={action} disclosure={disclosure} override={override_route} override_disclosed={override_disclosed} credential_store={credential} support_export={posture}",
            kind = row.surface_kind.as_str(),
            id = row.surface_id,
            declared = row.declared_gate_decision.as_str(),
            derived = row.derived_gate_decision.as_str(),
            matches = row.gate_decision_matches,
            silent = row.silent_execution_posture.as_str(),
            action = row.explicit_user_action_required,
            disclosure = row.has_disclosure,
            override_route = row.has_override_route,
            override_disclosed = row.override_has_disclosure,
            credential = row.touches_credential_store,
            posture = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "Gate decision truth: allow_unconditional={uncond} allow_after_grant={grant} allow_read_only={ro} block_pending={pending} block_repair={repair} no_uncond_outside_trusted={safe_trust} restricted_blocks_all={restricted} pending_blocks_all={pending_all}",
        uncond = record.gate_decision_truth.allow_unconditional_count,
        grant = record.gate_decision_truth.allow_after_explicit_grant_count,
        ro = record.gate_decision_truth.allow_read_only_count,
        pending = record.gate_decision_truth.block_pending_trust_decision_count,
        repair = record.gate_decision_truth.block_until_repair_count,
        safe_trust = record.gate_decision_truth.no_unconditional_allow_outside_trusted,
        restricted = record
            .gate_decision_truth
            .restricted_workspace_blocks_or_routes_every_surface,
        pending_all = record
            .gate_decision_truth
            .pending_workspace_blocks_or_routes_every_surface,
    ));
    lines.push(format!(
        "Silent execution honesty: grant_requires_action={action} grant_references_disclosure={disclosure} read_only_no_mutation={read_only}",
        action = record
            .silent_execution_honesty
            .all_grant_surfaces_require_explicit_user_action,
        disclosure = record
            .silent_execution_honesty
            .all_grant_surfaces_reference_disclosure,
        read_only = record
            .silent_execution_honesty
            .all_read_only_surfaces_declare_no_mutation_posture,
    ));
    lines.push(format!(
        "Override route honesty: overrides={count} all_disclosed={disclosed}",
        count = record.override_route_honesty.override_route_count,
        disclosed = record.override_route_honesty.all_override_routes_disclosed,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} redact_secrets={secrets} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority} credential_surfaces_safe={safe}",
        fields = record
            .support_export_honesty
            .all_surfaces_preserve_gating_fields,
        secrets = record
            .support_export_honesty
            .all_surfaces_redact_raw_secrets,
        approvals = record
            .support_export_honesty
            .all_surfaces_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_surfaces_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_surfaces_exclude_live_authority_handles,
        safe = record
            .support_export_honesty
            .all_credential_surfaces_have_safe_posture,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }
    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }
    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
