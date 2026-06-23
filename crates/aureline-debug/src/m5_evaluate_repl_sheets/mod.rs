//! Typed evaluate/REPL review sheets and console emissions: the canonical M5 records
//! every debugger, notebook, replay, and support surface reads to show *whether an
//! expression is treated as pure, unknown, or may-mutate* before it is dispatched and
//! after a result returns, *whether evaluation was approved, withheld, denied, blocked,
//! or expired*, and *whether a console line is interactive user input or target output*
//! that is live or replayed.
//!
//! The [`m5_debug_contracts`](crate::m5_debug_contracts) matrix *names* the debugger
//! object families and freezes their vocabulary; the
//! [`m5_debug_session_descriptors`](crate::m5_debug_session_descriptors),
//! [`m5_breakpoint_specs`](crate::m5_breakpoint_specs), and
//! [`m5_frame_variable_snapshots`](crate::m5_frame_variable_snapshots) lanes materialize
//! the session, attach-target, breakpoint, frame-mapping, and variable/watch families.
//! This lane *materializes* the
//! [`DebugObjectClass::EvaluateRequestResult`](crate::m5_debug_contracts::DebugObjectClass::EvaluateRequestResult)
//! and
//! [`DebugObjectClass::ConsoleEmission`](crate::m5_debug_contracts::DebugObjectClass::ConsoleEmission)
//! families as concrete, serde-serializable [`EvaluateRecord`] and [`ConsoleEmission`]
//! records and freezes a canonical [`EvaluateReplSheetSet`].
//!
//! This module *refines* the matrix evaluate-purity vocabulary: the matrix's
//! `evaluate_side_effect_free` / `evaluate_unknown_side_effects` / `evaluate_mutating`
//! states are pinned here as the [`EvaluatePurityClass`] vocabulary (`pure`, `unknown`,
//! `may_mutate`), and the matrix's `evaluate_blocked_inspect_only` state maps to the
//! [`ApprovalDisposition::Blocked`] disposition. Purity and approval stay orthogonal so a
//! surface can name the side-effect class *and* the approval posture independently.
//!
//! Evaluate and console truth stays explicit, governed, and replay-safe:
//!
//! - **One posture pill, one purity vocabulary.** Every [`EvaluateRecord`] carries one
//!   [`EvaluatePosturePill`] pinning one [`EvaluatePurityClass`] (pure, unknown,
//!   may-mutate) and one [`ApprovalDisposition`]. The pill's
//!   [`approval_required`](EvaluatePosturePill::approval_required),
//!   [`discloses_side_effect_risk`](EvaluatePosturePill::discloses_side_effect_risk), and
//!   [`permits_dispatch`](EvaluatePosturePill::permits_dispatch) flags are *derived* from
//!   the purity, disposition, and context authority, so an unknown or mutating expression
//!   always discloses its risk and requires review before it runs.
//! - **Approval is never bypassed.** A pure expression needs no approval; an unknown or
//!   may-mutate expression requires it. A pending, denied, blocked, or expired evaluation
//!   never permits dispatch, and a [`withheld request`](EvaluateRecord::result) carries no
//!   result — so no surface can silently run an effectful evaluation under a
//!   harmless-inspect label.
//! - **Inspect-only contexts block effectful evaluation.** An effectful expression issued
//!   against an [`EvaluateContextAuthority::InspectOnly`] context (a core-file or replay
//!   target) is blocked rather than silently mutating a recording.
//! - **Actor lineage is preserved.** Every record names who requested the evaluation and
//!   their actor class; an approval-cleared evaluation names its reviewer.
//! - **Interactive input and target output stay separate.** Every [`ConsoleEmission`]
//!   carries one [`ConsoleEmissionPill`] pinning one [`ConsoleDirection`] (user input vs
//!   target output) derived from its [`ConsoleStreamClass`], one [`ConsoleLiveness`]
//!   (live vs replayed), and a redaction marker — so console history and export packets
//!   distinguish interactive input from target output, never present a replayed line as
//!   live, and preserve redaction review rather than flattening one transcript.
//!
//! [`m5_evaluate_repl_sheet_set`] is the canonical binding: it builds the set
//! deterministically and computes each [`EvaluateReplInvariant`]'s `holds` flag from the
//! built records, so the checked-in fixture and the freeze gate freeze the contract
//! byte-for-byte and an inconsistent edit flips an invariant and fails CI. The record
//! carries no raw expression source, value bodies, console bodies, raw paths, provider
//! payloads, URLs, hostnames, or credentials — only opaque object refs, stable tokens,
//! opaque digests, and short reviewable sentences — so it is safe for support export.
//!
//! The cross-tool boundary schema is at
//! [`/schemas/debug/m5_evaluate_repl_sheets.schema.json`](../../../schemas/debug/m5_evaluate_repl_sheets.schema.json).
//! The checked-in stable packet is at
//! [`/fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json`](../../../fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json).
//! The reviewer-facing contract is at
//! [`/docs/debug/m5_evaluate_repl_sheets.md`](../../../docs/debug/m5_evaluate_repl_sheets.md).

use serde::{Deserialize, Serialize};

use crate::m5_debug_contracts::DebugConsumer;

#[cfg(test)]
mod tests;

/// Schema version for the M5 evaluate/REPL sheet set.
pub const M5_EVALUATE_REPL_SHEETS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the M5 evaluate/REPL sheet set.
pub const M5_EVALUATE_REPL_SHEETS_SCHEMA_REF: &str =
    "schemas/debug/m5_evaluate_repl_sheets.schema.json";

/// Stable record-kind tag for the evaluate/REPL sheet set.
pub const M5_EVALUATE_REPL_SHEETS_RECORD_KIND: &str = "m5_evaluate_repl_sheet_set";

/// Stable id for the canonical evaluate/REPL sheet set.
pub const M5_EVALUATE_REPL_SHEETS_SET_ID: &str = "m5-evaluate-repl-sheets:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_EVALUATE_REPL_SHEETS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the evaluate/REPL sheet set current. Stable promotion runs
/// this gate; it fails when the in-code set drifts from the checked-in fixture or any
/// invariant flips.
pub const M5_EVALUATE_REPL_SHEETS_FREEZE_GATE_REF: &str =
    "crates/aureline-debug/tests/m5_evaluate_repl_sheets.rs";

/// The checked-in canonical evaluate/REPL sheet-set fixture.
pub const M5_EVALUATE_REPL_SHEETS_FIXTURE_REF: &str =
    "fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json";

/// The contract narrative document.
pub const M5_EVALUATE_REPL_SHEETS_DOC_REF: &str = "docs/debug/m5_evaluate_repl_sheets.md";

/// The human-readable evidence companion artifact.
pub const M5_EVALUATE_REPL_SHEETS_ARTIFACT_REF: &str = "artifacts/debug/m5_evaluate_repl_sheets.md";

// ---------------------------------------------------------------------------
// Evaluate purity.
// ---------------------------------------------------------------------------

/// How an evaluate / REPL expression is classified for side-effect risk.
///
/// Only [`EvaluatePurityClass::Pure`] is a harmless, read-only inspection. An `Unknown` or
/// `MayMutate` expression discloses its risk and requires approval before dispatch, so a
/// mutation-capable evaluation never hides inside debugger chrome. This refines the matrix
/// evaluate-purity family: `pure` ↔ `evaluate_side_effect_free`, `unknown` ↔
/// `evaluate_unknown_side_effects`, `may_mutate` ↔ `evaluate_mutating`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluatePurityClass {
    /// Read-only and free of side effects; harmless inspection.
    Pure,
    /// Side effects are unknown and cannot be proven absent; risk disclosed.
    Unknown,
    /// May mutate target state; side-effect risk disclosed.
    MayMutate,
}

impl EvaluatePurityClass {
    /// All purity classes, in canonical order.
    pub const ALL: [Self; 3] = [Self::Pure, Self::Unknown, Self::MayMutate];

    /// Stable snake_case token for this purity class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pure => "pure",
            Self::Unknown => "unknown",
            Self::MayMutate => "may_mutate",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pure => "Pure (read-only)",
            Self::Unknown => "Unknown side effects",
            Self::MayMutate => "May mutate",
        }
    }

    /// Short pill fragment.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Pure => "pure",
            Self::Unknown => "unknown",
            Self::MayMutate => "may mutate",
        }
    }

    /// Whether this class is a harmless, read-only inspection.
    pub const fn is_pure(self) -> bool {
        matches!(self, Self::Pure)
    }

    /// Whether this class requires approval before dispatch. Unknown and may-mutate do.
    pub const fn requires_approval(self) -> bool {
        !matches!(self, Self::Pure)
    }

    /// Whether this class discloses a side-effect risk to the user before it runs.
    pub const fn discloses_side_effect_risk(self) -> bool {
        !matches!(self, Self::Pure)
    }
}

// ---------------------------------------------------------------------------
// Approval disposition.
// ---------------------------------------------------------------------------

/// The approval state of an evaluate / REPL request, preserved across UI, CLI, and
/// support packets so a blocked, denied, or expired evaluation never reads as cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDisposition {
    /// No approval is required — the expression is pure.
    NotRequired,
    /// Approval is required and the request is awaiting review.
    Pending,
    /// A reviewer approved the request; effectful evaluation may proceed.
    Approved,
    /// A reviewer denied the request; it must not run.
    Denied,
    /// Blocked by policy (for example, an inspect-only session) before review.
    Blocked,
    /// Approval was granted but lapsed before dispatch.
    Expired,
}

impl ApprovalDisposition {
    /// All dispositions, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::NotRequired,
        Self::Pending,
        Self::Approved,
        Self::Denied,
        Self::Blocked,
        Self::Expired,
    ];

    /// Stable snake_case token for this disposition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Denied => "denied",
            Self::Blocked => "blocked",
            Self::Expired => "expired",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotRequired => "No approval required",
            Self::Pending => "Approval pending",
            Self::Approved => "Approved",
            Self::Denied => "Denied",
            Self::Blocked => "Blocked",
            Self::Expired => "Approval expired",
        }
    }

    /// Short pill fragment used in a posture label.
    pub const fn pill_fragment(self) -> &'static str {
        match self {
            Self::NotRequired => "no approval needed",
            Self::Pending => "approval pending",
            Self::Approved => "approved",
            Self::Denied => "approval denied",
            Self::Blocked => "blocked",
            Self::Expired => "approval expired",
        }
    }

    /// Whether this disposition permits dispatch. Only an unrequired or approved request
    /// may run.
    pub const fn permits_dispatch(self) -> bool {
        matches!(self, Self::NotRequired | Self::Approved)
    }

    /// Whether this disposition is a terminal block — denied, blocked, or expired.
    pub const fn is_terminal_block(self) -> bool {
        matches!(self, Self::Denied | Self::Blocked | Self::Expired)
    }
}

// ---------------------------------------------------------------------------
// Expression context.
// ---------------------------------------------------------------------------

/// The scope an expression evaluates against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluateContextScope {
    /// Evaluated against a specific stack frame.
    Frame,
    /// Evaluated against a thread without a selected frame.
    Thread,
    /// Evaluated against a module-level / global scope.
    GlobalScope,
    /// Evaluated against the session without a frame (a top-level REPL turn).
    Session,
    /// A REPL / console expression turn.
    Repl,
}

impl EvaluateContextScope {
    /// All scopes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Frame,
        Self::Thread,
        Self::GlobalScope,
        Self::Session,
        Self::Repl,
    ];

    /// Stable snake_case token for this scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Frame => "frame",
            Self::Thread => "thread",
            Self::GlobalScope => "global_scope",
            Self::Session => "session",
            Self::Repl => "repl",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Frame => "Frame",
            Self::Thread => "Thread",
            Self::GlobalScope => "Global scope",
            Self::Session => "Session",
            Self::Repl => "REPL",
        }
    }
}

/// Whether the context an expression runs against can be mutated, or is a recorded /
/// inspect-only target. Drives the inspect-only blocking rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluateContextAuthority {
    /// A live, mutable target (a launched or attached session).
    LiveMutable,
    /// An inspect-only target — a core file or a replay capture — that must not be mutated.
    InspectOnly,
}

impl EvaluateContextAuthority {
    /// All authorities, in canonical order.
    pub const ALL: [Self; 2] = [Self::LiveMutable, Self::InspectOnly];

    /// Stable snake_case token for this authority.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveMutable => "live_mutable",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveMutable => "Live (mutable)",
            Self::InspectOnly => "Inspect-only",
        }
    }

    /// Whether this authority allows effectful evaluation.
    pub const fn allows_mutation(self) -> bool {
        matches!(self, Self::LiveMutable)
    }
}

/// Where an expression evaluates: the session/thread/frame, its scope, the context
/// authority, and the notebook or replay surface it belongs to. No raw paths cross this
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionContext {
    /// Stable session id the expression runs in.
    pub session_id: String,
    /// Stable thread id, when the expression is bound to a thread.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// Stable frame id the expression runs against, tying it to a frame mapping.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<String>,
    /// The scope the expression evaluates against.
    pub scope: EvaluateContextScope,
    /// Stable token for the scope.
    pub scope_token: String,
    /// Whether the context is live/mutable or inspect-only.
    pub authority: EvaluateContextAuthority,
    /// Stable token for the authority.
    pub authority_token: String,
    /// Stable notebook cell ref, when evaluation happens in a notebook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notebook_cell_ref: Option<String>,
    /// Stable replay capture ref, when evaluation happens against a recorded capture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_capture_ref: Option<String>,
}

impl ExpressionContext {
    /// Builds an expression context, deriving the scope and authority tokens.
    pub fn build(
        session_id: impl Into<String>,
        thread_id: Option<&str>,
        frame_id: Option<&str>,
        scope: EvaluateContextScope,
        authority: EvaluateContextAuthority,
        notebook_cell_ref: Option<&str>,
        replay_capture_ref: Option<&str>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            thread_id: thread_id.map(str::to_owned),
            frame_id: frame_id.map(str::to_owned),
            scope,
            scope_token: scope.as_str().to_owned(),
            authority,
            authority_token: authority.as_str().to_owned(),
            notebook_cell_ref: notebook_cell_ref.map(str::to_owned),
            replay_capture_ref: replay_capture_ref.map(str::to_owned),
        }
    }

    /// Whether the carried tokens agree with their enums.
    pub fn is_consistent(&self) -> bool {
        self.scope_token == self.scope.as_str() && self.authority_token == self.authority.as_str()
    }
}

// ---------------------------------------------------------------------------
// Actor lineage.
// ---------------------------------------------------------------------------

/// The class of actor that requested an evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluateActorClass {
    /// A human user.
    Human,
    /// An AI agent acting on a user's behalf.
    AiAgent,
    /// An automation / scripted runner.
    Automation,
}

impl EvaluateActorClass {
    /// All actor classes, in canonical order.
    pub const ALL: [Self; 3] = [Self::Human, Self::AiAgent, Self::Automation];

    /// Stable snake_case token for this actor class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::AiAgent => "ai_agent",
            Self::Automation => "automation",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Human => "Human",
            Self::AiAgent => "AI agent",
            Self::Automation => "Automation",
        }
    }
}

/// Who requested and reviewed an evaluation, and from which surface — opaque actor refs
/// only, never names, emails, or credentials.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorLineage {
    /// Opaque ref of the actor that requested the evaluation.
    pub requested_by_ref: String,
    /// The class of the requesting actor.
    pub actor_class: EvaluateActorClass,
    /// Stable token for the actor class.
    pub actor_class_token: String,
    /// The surface the evaluation was requested from.
    pub origin_surface: DebugConsumer,
    /// Opaque ref of the reviewer that cleared the evaluation, when one did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_by_ref: Option<String>,
    /// Opaque ref of the principal an agent or automation acts on behalf of, when one is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_ref: Option<String>,
}

impl ActorLineage {
    /// Builds an actor lineage, deriving the actor-class token.
    pub fn build(
        requested_by_ref: impl Into<String>,
        actor_class: EvaluateActorClass,
        origin_surface: DebugConsumer,
        reviewed_by_ref: Option<&str>,
        on_behalf_of_ref: Option<&str>,
    ) -> Self {
        Self {
            requested_by_ref: requested_by_ref.into(),
            actor_class,
            actor_class_token: actor_class.as_str().to_owned(),
            origin_surface,
            reviewed_by_ref: reviewed_by_ref.map(str::to_owned),
            on_behalf_of_ref: on_behalf_of_ref.map(str::to_owned),
        }
    }

    /// Whether the carried token agrees with the actor class.
    pub fn is_consistent(&self) -> bool {
        self.actor_class_token == self.actor_class.as_str()
    }
}

// ---------------------------------------------------------------------------
// Redaction.
// ---------------------------------------------------------------------------

/// Why an expression, result, or console body is withheld. Shared by evaluate requests,
/// evaluate results, and console emissions so redaction reads one way everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluateRedactionClass {
    /// Not redacted; the body (a digest) is present.
    NotRedacted,
    /// Withheld because it matched a secret / credential class.
    SecretRedacted,
    /// Withheld because it matched a personal-data class.
    PiiRedacted,
    /// Withheld by an explicit policy rule on this surface.
    PolicyWithheld,
}

impl EvaluateRedactionClass {
    /// All redaction classes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::NotRedacted,
        Self::SecretRedacted,
        Self::PiiRedacted,
        Self::PolicyWithheld,
    ];

    /// Stable snake_case token for this redaction class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRedacted => "not_redacted",
            Self::SecretRedacted => "secret_redacted",
            Self::PiiRedacted => "pii_redacted",
            Self::PolicyWithheld => "policy_withheld",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotRedacted => "Not redacted",
            Self::SecretRedacted => "Secret (redacted)",
            Self::PiiRedacted => "Personal data (redacted)",
            Self::PolicyWithheld => "Policy-withheld",
        }
    }

    /// Whether this class withholds the body.
    pub const fn is_redacted(self) -> bool {
        !matches!(self, Self::NotRedacted)
    }
}

// ---------------------------------------------------------------------------
// Evaluate posture pill.
// ---------------------------------------------------------------------------

/// The single canonical posture pill every evaluate / REPL surface renders — one purity
/// class, one approval disposition, with every governance flag derived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluatePosturePill {
    /// The evaluation purity class.
    pub purity: EvaluatePurityClass,
    /// Stable token for the purity class.
    pub purity_token: String,
    /// The approval disposition.
    pub disposition: ApprovalDisposition,
    /// Stable token for the disposition.
    pub disposition_token: String,
    /// The context authority the expression runs against.
    pub context_authority: EvaluateContextAuthority,
    /// Stable token for the context authority.
    pub context_authority_token: String,
    /// One reviewable label combining purity, disposition, and inspect-only blocking.
    pub label: String,
    /// Whether the expression requires approval before dispatch.
    pub approval_required: bool,
    /// Whether the expression discloses a side-effect risk before it runs.
    pub discloses_side_effect_risk: bool,
    /// Whether the request may be dispatched in its current disposition.
    pub permits_dispatch: bool,
    /// Whether a review / approval affordance must be shown.
    pub requires_review_affordance: bool,
    /// Whether the request is blocked.
    pub is_blocked: bool,
    /// Whether the request is blocked specifically because its context is inspect-only.
    pub blocked_by_inspect_only: bool,
}

impl EvaluatePosturePill {
    /// Builds the canonical posture pill, deriving every flag and the label from the
    /// purity, disposition, and context authority so the pill cannot disagree with itself.
    pub fn derive(
        purity: EvaluatePurityClass,
        disposition: ApprovalDisposition,
        context_authority: EvaluateContextAuthority,
    ) -> Self {
        let approval_required = purity.requires_approval();
        let discloses_side_effect_risk = purity.discloses_side_effect_risk();
        let permits_dispatch = disposition.permits_dispatch();
        let requires_review_affordance =
            approval_required && disposition != ApprovalDisposition::Approved;
        let is_blocked = disposition == ApprovalDisposition::Blocked;
        let blocked_by_inspect_only = is_blocked && !context_authority.allows_mutation();

        let mut label = purity.label().to_owned();
        label.push_str(" · ");
        label.push_str(disposition.pill_fragment());
        if blocked_by_inspect_only {
            label.push_str(" · inspect-only");
        }

        Self {
            purity,
            purity_token: purity.as_str().to_owned(),
            disposition,
            disposition_token: disposition.as_str().to_owned(),
            context_authority,
            context_authority_token: context_authority.as_str().to_owned(),
            label,
            approval_required,
            discloses_side_effect_risk,
            permits_dispatch,
            requires_review_affordance,
            is_blocked,
            blocked_by_inspect_only,
        }
    }

    /// Whether this pill equals the canonical derivation for the given inputs.
    pub fn matches_derivation(
        &self,
        purity: EvaluatePurityClass,
        disposition: ApprovalDisposition,
        context_authority: EvaluateContextAuthority,
    ) -> bool {
        *self == Self::derive(purity, disposition, context_authority)
    }
}

// ---------------------------------------------------------------------------
// Evaluate result.
// ---------------------------------------------------------------------------

/// The outcome of a dispatched evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluateOutcome {
    /// Completed and returned a value.
    Completed,
    /// Completed with no value (a void / statement evaluation).
    NoValue,
    /// Raised an error during evaluation.
    RaisedError,
}

impl EvaluateOutcome {
    /// All outcomes, in canonical order.
    pub const ALL: [Self; 3] = [Self::Completed, Self::NoValue, Self::RaisedError];

    /// Stable snake_case token for this outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::NoValue => "no_value",
            Self::RaisedError => "raised_error",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Completed => "Completed",
            Self::NoValue => "No value",
            Self::RaisedError => "Raised error",
        }
    }

    /// Whether this outcome carries a value body.
    pub const fn carries_value(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Whether this outcome is an error.
    pub const fn is_error(self) -> bool {
        matches!(self, Self::RaisedError)
    }
}

/// The result of a dispatched evaluation: the outcome, a reviewable result summary, an
/// explicit side-effect note, and a redaction marker. Present only when a request was
/// permitted to dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluateResult {
    /// Stable, namespaced result id.
    pub result_id: String,
    /// The outcome of the evaluation.
    pub outcome: EvaluateOutcome,
    /// Stable token for the outcome.
    pub outcome_token: String,
    /// One reviewable export-safe sentence summarizing the result. Never a raw value body.
    pub result_summary: String,
    /// Opaque digest of the result value; present only when a value body is present
    /// (a completed, non-redacted outcome), never the raw value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_repr_digest: Option<String>,
    /// One reviewable sentence describing the observed or declared side effects.
    pub side_effect_note: String,
    /// Whether a mutation was actually observed during evaluation.
    pub observed_mutation: bool,
    /// The redaction class applied to the result body.
    pub redaction: EvaluateRedactionClass,
    /// Stable token for the redaction class.
    pub redaction_token: String,
    /// Whether the result body is withheld by redaction.
    pub is_redacted: bool,
    /// Whether a result value body (digest) is present.
    pub result_body_present: bool,
    /// Timestamp the result returned at.
    pub returned_as_of: String,
}

impl EvaluateResult {
    /// Builds an evaluate result, deriving every computed token and body-presence flag.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        result_id: impl Into<String>,
        outcome: EvaluateOutcome,
        result_summary: impl Into<String>,
        result_repr_digest: Option<&str>,
        side_effect_note: impl Into<String>,
        observed_mutation: bool,
        redaction: EvaluateRedactionClass,
        returned_as_of: impl Into<String>,
    ) -> Self {
        let is_redacted = redaction.is_redacted();
        let result_body_present = outcome.carries_value() && !is_redacted;
        Self {
            result_id: result_id.into(),
            outcome,
            outcome_token: outcome.as_str().to_owned(),
            result_summary: result_summary.into(),
            result_repr_digest: result_repr_digest.map(str::to_owned),
            side_effect_note: side_effect_note.into(),
            observed_mutation,
            redaction,
            redaction_token: redaction.as_str().to_owned(),
            is_redacted,
            result_body_present,
            returned_as_of: returned_as_of.into(),
        }
    }

    /// Whether the carried tokens and flags agree with the outcome and redaction.
    pub fn is_consistent(&self) -> bool {
        self.outcome_token == self.outcome.as_str()
            && self.redaction_token == self.redaction.as_str()
            && self.is_redacted == self.redaction.is_redacted()
            && self.result_body_present
                == (self.outcome.carries_value() && !self.redaction.is_redacted())
            && self.result_repr_digest.is_some() == self.result_body_present
    }
}

// ---------------------------------------------------------------------------
// Evaluate record.
// ---------------------------------------------------------------------------

/// A typed evaluate / REPL review sheet: the canonical record every debugger evaluate
/// pane, REPL, notebook console, replay inspector, and exported support packet reads to
/// show one expression, how its side-effect risk is classified, what approval posture it
/// carries, who requested it, and — when dispatch was permitted — its result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluateRecord {
    /// Stable, namespaced evaluate id.
    pub evaluate_id: String,
    /// Opaque digest of the expression source; never the raw expression text.
    pub expression_digest: String,
    /// The redaction class applied to the expression itself.
    pub expression_redaction: EvaluateRedactionClass,
    /// Stable token for the expression redaction class.
    pub expression_redaction_token: String,
    /// The context the expression evaluates against.
    pub context: ExpressionContext,
    /// The evaluation purity class.
    pub purity: EvaluatePurityClass,
    /// Stable token for the purity class.
    pub purity_token: String,
    /// The approval disposition.
    pub disposition: ApprovalDisposition,
    /// Stable token for the disposition.
    pub disposition_token: String,
    /// The canonical purity + approval posture pill every surface renders.
    pub posture: EvaluatePosturePill,
    /// Who requested and reviewed the evaluation.
    pub actor: ActorLineage,
    /// The result, present only when the request was permitted to dispatch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<EvaluateResult>,
    /// The proof packet that keeps this record current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the evaluation.
    pub summary: String,
}

impl EvaluateRecord {
    /// Builds an evaluate record, deriving every computed token and the posture pill from
    /// the typed inputs so the record cannot disagree with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        evaluate_id: impl Into<String>,
        expression_digest: impl Into<String>,
        expression_redaction: EvaluateRedactionClass,
        context: ExpressionContext,
        purity: EvaluatePurityClass,
        disposition: ApprovalDisposition,
        actor: ActorLineage,
        result: Option<EvaluateResult>,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        let posture = EvaluatePosturePill::derive(purity, disposition, context.authority);
        Self {
            evaluate_id: evaluate_id.into(),
            expression_digest: expression_digest.into(),
            expression_redaction,
            expression_redaction_token: expression_redaction.as_str().to_owned(),
            context,
            purity,
            purity_token: purity.as_str().to_owned(),
            disposition,
            disposition_token: disposition.as_str().to_owned(),
            posture,
            actor,
            result,
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// The evaluation purity class.
    pub const fn purity(&self) -> EvaluatePurityClass {
        self.purity
    }

    /// The approval disposition.
    pub const fn disposition(&self) -> ApprovalDisposition {
        self.disposition
    }

    /// Whether the request may be dispatched in its current disposition.
    pub const fn permits_dispatch(&self) -> bool {
        self.posture.permits_dispatch
    }
}

// ---------------------------------------------------------------------------
// Console emission.
// ---------------------------------------------------------------------------

/// Whether a console emission is interactive user input or target output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsoleDirection {
    /// Interactive input the user (or an actor) typed into the console / REPL.
    UserInput,
    /// Output the target program or debugger emitted.
    TargetOutput,
}

impl ConsoleDirection {
    /// All directions, in canonical order.
    pub const ALL: [Self; 2] = [Self::UserInput, Self::TargetOutput];

    /// Stable snake_case token for this direction.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserInput => "user_input",
            Self::TargetOutput => "target_output",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UserInput => "User input",
            Self::TargetOutput => "Target output",
        }
    }

    /// Whether this direction is interactive user input.
    pub const fn is_user_input(self) -> bool {
        matches!(self, Self::UserInput)
    }
}

/// The stream class of a console emission. Each stream class belongs unambiguously to one
/// [`ConsoleDirection`], so input and output can never be flattened into one transcript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsoleStreamClass {
    /// Standard input the user typed.
    Stdin,
    /// An evaluate / REPL expression the user submitted.
    EvaluateInput,
    /// Standard output from the target.
    Stdout,
    /// Standard error from the target.
    Stderr,
    /// A debugger / debug-console emission.
    DebugConsole,
    /// The result of an evaluate / REPL expression echoed back.
    EvaluateResult,
}

impl ConsoleStreamClass {
    /// All stream classes, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Stdin,
        Self::EvaluateInput,
        Self::Stdout,
        Self::Stderr,
        Self::DebugConsole,
        Self::EvaluateResult,
    ];

    /// Stable snake_case token for this stream class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stdin => "stdin",
            Self::EvaluateInput => "evaluate_input",
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
            Self::DebugConsole => "debug_console",
            Self::EvaluateResult => "evaluate_result",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Stdin => "stdin",
            Self::EvaluateInput => "evaluate input",
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
            Self::DebugConsole => "debug console",
            Self::EvaluateResult => "evaluate result",
        }
    }

    /// The direction this stream class belongs to.
    pub const fn direction(self) -> ConsoleDirection {
        match self {
            Self::Stdin | Self::EvaluateInput => ConsoleDirection::UserInput,
            Self::Stdout | Self::Stderr | Self::DebugConsole | Self::EvaluateResult => {
                ConsoleDirection::TargetOutput
            }
        }
    }
}

/// Whether a console emission is live or replayed from a captured session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsoleLiveness {
    /// Emitted live by the running session.
    Live,
    /// Replayed from a captured / recorded session.
    ReplayedCapture,
}

impl ConsoleLiveness {
    /// All liveness states, in canonical order.
    pub const ALL: [Self; 2] = [Self::Live, Self::ReplayedCapture];

    /// Stable snake_case token for this liveness state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::ReplayedCapture => "replayed_capture",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::ReplayedCapture => "Replayed (captured)",
        }
    }

    /// Whether this emission is live.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }

    /// Whether this emission must render with a visible caveat. A replayed line discloses.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// The single canonical pill every console surface renders — one direction, one stream
/// class, one liveness, with every honesty flag derived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsoleEmissionPill {
    /// The emission direction.
    pub direction: ConsoleDirection,
    /// Stable token for the direction.
    pub direction_token: String,
    /// The stream class.
    pub stream: ConsoleStreamClass,
    /// Stable token for the stream class.
    pub stream_token: String,
    /// The liveness state.
    pub liveness: ConsoleLiveness,
    /// Stable token for the liveness state.
    pub liveness_token: String,
    /// One reviewable label combining stream, direction, liveness, and redaction.
    pub label: String,
    /// Whether this emission is interactive user input.
    pub is_user_input: bool,
    /// Whether this emission is target output.
    pub is_target_output: bool,
    /// Whether this emission is live.
    pub is_live: bool,
    /// Whether this emission is replayed from a capture.
    pub is_replayed: bool,
    /// Whether this emission must render with a visible caveat.
    pub requires_disclosure: bool,
    /// Whether the body is withheld by redaction.
    pub is_redacted: bool,
    /// Whether a body (digest) is present.
    pub body_present: bool,
}

impl ConsoleEmissionPill {
    /// Builds the canonical console pill, deriving every flag from the stream class,
    /// liveness, and redaction so the pill cannot disagree with itself.
    pub fn derive(
        stream: ConsoleStreamClass,
        liveness: ConsoleLiveness,
        redaction: EvaluateRedactionClass,
    ) -> Self {
        let direction = stream.direction();
        let is_user_input = direction.is_user_input();
        let is_live = liveness.is_live();
        let is_redacted = redaction.is_redacted();
        let body_present = !is_redacted;

        let mut label = format!("{} · {}", stream.label(), direction.label());
        if !is_live {
            label.push_str(" · replayed");
        }
        if is_redacted {
            label.push_str(" · ");
            label.push_str(redaction.label());
        }

        Self {
            direction,
            direction_token: direction.as_str().to_owned(),
            stream,
            stream_token: stream.as_str().to_owned(),
            liveness,
            liveness_token: liveness.as_str().to_owned(),
            label,
            is_user_input,
            is_target_output: !is_user_input,
            is_live,
            is_replayed: !is_live,
            requires_disclosure: liveness.requires_disclosure(),
            is_redacted,
            body_present,
        }
    }

    /// Whether this pill equals the canonical derivation for the given inputs.
    pub fn matches_derivation(
        &self,
        stream: ConsoleStreamClass,
        liveness: ConsoleLiveness,
        redaction: EvaluateRedactionClass,
    ) -> bool {
        *self == Self::derive(stream, liveness, redaction)
    }
}

/// A typed console emission: the canonical record every console pane, REPL transcript,
/// notebook output area, replay inspector, and exported support packet reads to show one
/// console line, whether it is interactive input or target output, whether it is live or
/// replayed, and whether its body is withheld by redaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsoleEmission {
    /// Stable, namespaced emission id.
    pub emission_id: String,
    /// Monotonic sequence number within the session transcript, for stable ordering.
    pub sequence: u64,
    /// The stream class.
    pub stream: ConsoleStreamClass,
    /// Stable token for the stream class.
    pub stream_token: String,
    /// The emission direction, derived from the stream class.
    pub direction: ConsoleDirection,
    /// Stable token for the direction.
    pub direction_token: String,
    /// The liveness state.
    pub liveness: ConsoleLiveness,
    /// Stable token for the liveness state.
    pub liveness_token: String,
    /// Stable session ref the emission belongs to.
    pub session_ref: String,
    /// Stable thread ref, when the emission is bound to a thread.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_ref: Option<String>,
    /// Stable frame ref, when the emission is bound to a frame.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_ref: Option<String>,
    /// Stable notebook cell ref, when the emission is shown in a notebook output area.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notebook_cell_ref: Option<String>,
    /// Stable replay capture ref, when the emission is replayed from a recorded capture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_capture_ref: Option<String>,
    /// The evaluate record this emission belongs to, when it is an evaluate input/result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_evaluate_id: Option<String>,
    /// Opaque digest of the emission body; present only when a body is present (not
    /// redacted), never the raw console body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_digest: Option<String>,
    /// The redaction class applied to the body.
    pub redaction: EvaluateRedactionClass,
    /// Stable token for the redaction class.
    pub redaction_token: String,
    /// Whether this emission is preserved for replay.
    pub replayable: bool,
    /// The canonical direction + liveness pill every surface renders.
    pub pill: ConsoleEmissionPill,
    /// The proof packet that keeps this emission current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the emission.
    pub summary: String,
}

impl ConsoleEmission {
    /// Builds a console emission, deriving every computed token and the pill from the
    /// typed inputs so the record cannot disagree with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        emission_id: impl Into<String>,
        sequence: u64,
        stream: ConsoleStreamClass,
        liveness: ConsoleLiveness,
        session_ref: impl Into<String>,
        thread_ref: Option<&str>,
        frame_ref: Option<&str>,
        notebook_cell_ref: Option<&str>,
        replay_capture_ref: Option<&str>,
        linked_evaluate_id: Option<&str>,
        body_digest: Option<&str>,
        redaction: EvaluateRedactionClass,
        replayable: bool,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        let direction = stream.direction();
        Self {
            emission_id: emission_id.into(),
            sequence,
            stream,
            stream_token: stream.as_str().to_owned(),
            direction,
            direction_token: direction.as_str().to_owned(),
            liveness,
            liveness_token: liveness.as_str().to_owned(),
            session_ref: session_ref.into(),
            thread_ref: thread_ref.map(str::to_owned),
            frame_ref: frame_ref.map(str::to_owned),
            notebook_cell_ref: notebook_cell_ref.map(str::to_owned),
            replay_capture_ref: replay_capture_ref.map(str::to_owned),
            linked_evaluate_id: linked_evaluate_id.map(str::to_owned),
            body_digest: body_digest.map(str::to_owned),
            redaction,
            redaction_token: redaction.as_str().to_owned(),
            replayable,
            pill: ConsoleEmissionPill::derive(stream, liveness, redaction),
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// The emission direction.
    pub const fn direction(&self) -> ConsoleDirection {
        self.direction
    }
}

// ---------------------------------------------------------------------------
// Invariants and set.
// ---------------------------------------------------------------------------

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluateReplInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen, typed M5 evaluate/REPL sheet set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluateReplSheetSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_evaluate_repl_sheets_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the set current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The boundary schemas this set binds as truth sources.
    pub source_schema_refs: Vec<String>,
    /// The crate modules that already produce this truth.
    pub producer_refs: Vec<String>,
    /// The surfaces that consume the evaluate records and console emissions.
    pub consumer_surfaces: Vec<DebugConsumer>,
    /// The evaluate / REPL records.
    pub evaluations: Vec<EvaluateRecord>,
    /// The console emissions.
    pub console: Vec<ConsoleEmission>,
    /// The computed invariants.
    pub invariants: Vec<EvaluateReplInvariant>,
    /// Whether raw expression/value/console bodies are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the evaluate/REPL sheet set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluateReplSheetSetValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for EvaluateReplSheetSetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m5 evaluate/REPL sheet set invalid: {}", self.reason)
    }
}

impl std::error::Error for EvaluateReplSheetSetValidationError {}

impl EvaluateReplSheetSet {
    /// Returns the evaluate record with the given id, if present.
    pub fn evaluation(&self, evaluate_id: &str) -> Option<&EvaluateRecord> {
        self.evaluations
            .iter()
            .find(|e| e.evaluate_id == evaluate_id)
    }

    /// Returns the console emission with the given id, if present.
    pub fn emission(&self, emission_id: &str) -> Option<&ConsoleEmission> {
        self.console.iter().find(|c| c.emission_id == emission_id)
    }

    /// Returns the first evaluation in the given purity class, if present.
    pub fn evaluation_in_purity(&self, purity: EvaluatePurityClass) -> Option<&EvaluateRecord> {
        self.evaluations.iter().find(|e| e.purity() == purity)
    }

    /// Returns the first evaluation in the given disposition, if present.
    pub fn evaluation_in_disposition(
        &self,
        disposition: ApprovalDisposition,
    ) -> Option<&EvaluateRecord> {
        self.evaluations
            .iter()
            .find(|e| e.disposition() == disposition)
    }

    /// Returns the first emission in the given direction, if present.
    pub fn emission_in_direction(&self, direction: ConsoleDirection) -> Option<&ConsoleEmission> {
        self.console.iter().find(|c| c.direction() == direction)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are excluded
    /// and every ref is a repo-relative object ref, never a URL, host, credential, or
    /// absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_set = self
            .source_schema_refs
            .iter()
            .map(String::as_str)
            .chain(self.producer_refs.iter().map(String::as_str))
            .chain(std::iter::once(self.freeze_gate_ref.as_str()));
        let from_evals = self.evaluations.iter().map(|e| e.proof_packet_ref.as_str());
        let from_console = self.console.iter().map(|c| c.proof_packet_ref.as_str());
        from_set.chain(from_evals).chain(from_console)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    ///
    /// # Errors
    ///
    /// Returns an [`EvaluateReplSheetSetValidationError`] when an identifier, a ref, a
    /// computed flag, a pill, a purity/approval rule, a console rule, a linkage, or an
    /// invariant is inconsistent.
    pub fn validate(&self) -> Result<(), EvaluateReplSheetSetValidationError> {
        let fail = |reason: String| Err(EvaluateReplSheetSetValidationError { reason });

        if self.record_kind != M5_EVALUATE_REPL_SHEETS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_EVALUATE_REPL_SHEETS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.m5_evaluate_repl_sheets_schema_version != M5_EVALUATE_REPL_SHEETS_SCHEMA_VERSION {
            return fail("unexpected schema version".to_owned());
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.evaluations.is_empty() {
            return fail("no evaluations".to_owned());
        }
        if self.console.is_empty() {
            return fail("no console emissions".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.evaluations.iter().map(|e| e.evaluate_id.as_str())) {
            return fail("evaluate ids are not unique".to_owned());
        }
        if !all_unique(self.console.iter().map(|c| c.emission_id.as_str())) {
            return fail("emission ids are not unique".to_owned());
        }

        // The full purity vocabulary is materialized.
        for purity in EvaluatePurityClass::ALL {
            if self.evaluation_in_purity(purity).is_none() {
                return fail(format!(
                    "purity class {} is not materialized",
                    purity.as_str()
                ));
            }
        }
        // The full disposition vocabulary is materialized.
        for disposition in ApprovalDisposition::ALL {
            if self.evaluation_in_disposition(disposition).is_none() {
                return fail(format!(
                    "disposition {} is not materialized",
                    disposition.as_str()
                ));
            }
        }
        // Both console directions are materialized.
        for direction in ConsoleDirection::ALL {
            if self.emission_in_direction(direction).is_none() {
                return fail(format!(
                    "console direction {} is not materialized",
                    direction.as_str()
                ));
            }
        }

        // Per-evaluation structural floor and governance rules.
        for ev in &self.evaluations {
            validate_evaluation(ev)
                .map_err(|reason| EvaluateReplSheetSetValidationError { reason })?;
        }
        // Per-emission structural floor and rules.
        for em in &self.console {
            validate_emission(em)
                .map_err(|reason| EvaluateReplSheetSetValidationError { reason })?;
        }
        // Every console emission linked to an evaluate resolves to one in the set.
        for em in &self.console {
            if let Some(eval_id) = &em.linked_evaluate_id {
                if self.evaluation(eval_id).is_none() {
                    return fail(format!(
                        "console emission {} links to missing evaluate id {eval_id}",
                        em.emission_id
                    ));
                }
            }
        }

        if !self.is_support_export_safe() {
            return fail("set is not support-export safe".to_owned());
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

fn validate_evaluation(ev: &EvaluateRecord) -> Result<(), String> {
    if ev.evaluate_id.is_empty() {
        return Err("evaluation has empty id".to_owned());
    }
    if ev.expression_digest.is_empty() {
        return Err(format!(
            "evaluation {} has empty expression digest",
            ev.evaluate_id
        ));
    }
    if ev.context.session_id.is_empty() {
        return Err(format!(
            "evaluation {} has empty session id",
            ev.evaluate_id
        ));
    }
    if ev.actor.requested_by_ref.is_empty() {
        return Err(format!(
            "evaluation {} has no requesting actor",
            ev.evaluate_id
        ));
    }
    if ev.proof_packet_ref.is_empty() {
        return Err(format!("evaluation {} has no proof packet", ev.evaluate_id));
    }
    if !evaluation_flags_consistent(ev) {
        return Err(format!(
            "evaluation {} computed flags or pill disagree with its enums",
            ev.evaluate_id
        ));
    }
    // Approval is required exactly when the purity is unknown or may-mutate.
    if ev.posture.approval_required != ev.purity.requires_approval() {
        return Err(format!(
            "evaluation {} approval requirement disagrees with its purity",
            ev.evaluate_id
        ));
    }
    // A pure expression carries no approval; an effectful one is never left unrequired.
    match ev.purity {
        EvaluatePurityClass::Pure => {
            if ev.disposition != ApprovalDisposition::NotRequired {
                return Err(format!(
                    "pure evaluation {} must carry a not-required disposition",
                    ev.evaluate_id
                ));
            }
        }
        EvaluatePurityClass::Unknown | EvaluatePurityClass::MayMutate => {
            if ev.disposition == ApprovalDisposition::NotRequired {
                return Err(format!(
                    "effectful evaluation {} must not carry a not-required disposition",
                    ev.evaluate_id
                ));
            }
        }
    }
    // A withheld request never carries a result; a result implies dispatch was permitted.
    if ev.result.is_some() && !ev.posture.permits_dispatch {
        return Err(format!(
            "evaluation {} carries a result but its disposition does not permit dispatch",
            ev.evaluate_id
        ));
    }
    if let Some(result) = &ev.result {
        if !result.is_consistent() {
            return Err(format!(
                "evaluation {} result flags disagree with its outcome/redaction",
                ev.evaluate_id
            ));
        }
        if result.result_id.is_empty() {
            return Err(format!(
                "evaluation {} has an empty result id",
                ev.evaluate_id
            ));
        }
    }
    // An effectful expression against an inspect-only context must never permit dispatch.
    if !ev.context.authority.allows_mutation()
        && ev.purity.requires_approval()
        && ev.posture.permits_dispatch
    {
        return Err(format!(
            "evaluation {} would run an effectful expression against an inspect-only context",
            ev.evaluate_id
        ));
    }
    // An approval-cleared effectful evaluation names its reviewer.
    if ev.purity.requires_approval()
        && ev.disposition == ApprovalDisposition::Approved
        && ev.actor.reviewed_by_ref.is_none()
    {
        return Err(format!(
            "approved evaluation {} must name its reviewer",
            ev.evaluate_id
        ));
    }
    Ok(())
}

fn validate_emission(em: &ConsoleEmission) -> Result<(), String> {
    if em.emission_id.is_empty() {
        return Err("emission has empty id".to_owned());
    }
    if em.session_ref.is_empty() {
        return Err(format!("emission {} has empty session ref", em.emission_id));
    }
    if em.proof_packet_ref.is_empty() {
        return Err(format!("emission {} has no proof packet", em.emission_id));
    }
    if !emission_flags_consistent(em) {
        return Err(format!(
            "emission {} computed flags or pill disagree with its enums",
            em.emission_id
        ));
    }
    // The direction is exactly the one its stream class belongs to.
    if em.direction != em.stream.direction() {
        return Err(format!(
            "emission {} direction disagrees with its stream class",
            em.emission_id
        ));
    }
    // A body digest is present exactly when the pill says a body is present.
    if em.pill.body_present != em.body_digest.is_some() {
        return Err(format!(
            "emission {} body presence disagrees with its digest",
            em.emission_id
        ));
    }
    // A redacted emission withholds its body.
    if em.redaction.is_redacted() && em.body_digest.is_some() {
        return Err(format!(
            "emission {} is redacted but still carries a body",
            em.emission_id
        ));
    }
    // A replayed emission always discloses; a live emission never claims replay.
    if em.pill.is_replayed != (em.liveness == ConsoleLiveness::ReplayedCapture) {
        return Err(format!(
            "emission {} replay flag disagrees with its liveness",
            em.emission_id
        ));
    }
    Ok(())
}

fn evaluation_flags_consistent(ev: &EvaluateRecord) -> bool {
    ev.purity_token == ev.purity.as_str()
        && ev.disposition_token == ev.disposition.as_str()
        && ev.expression_redaction_token == ev.expression_redaction.as_str()
        && ev.context.is_consistent()
        && ev.actor.is_consistent()
        && ev
            .posture
            .matches_derivation(ev.purity, ev.disposition, ev.context.authority)
}

fn emission_flags_consistent(em: &ConsoleEmission) -> bool {
    em.stream_token == em.stream.as_str()
        && em.direction_token == em.direction.as_str()
        && em.liveness_token == em.liveness.as_str()
        && em.redaction_token == em.redaction.as_str()
        && em
            .pill
            .matches_derivation(em.stream, em.liveness, em.redaction)
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque `aureline://`
/// handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical M5 evaluate/REPL sheet set.
///
/// Deterministic: the same bytes every call. Each invariant's `holds` flag is computed
/// from the built records, so an inconsistent edit flips an invariant rather than silently
/// passing.
pub fn m5_evaluate_repl_sheet_set() -> EvaluateReplSheetSet {
    let evaluations = build_evaluations();
    let console = build_console();
    let invariants = compute_invariants(&evaluations, &console);

    EvaluateReplSheetSet {
        record_kind: M5_EVALUATE_REPL_SHEETS_RECORD_KIND.to_owned(),
        m5_evaluate_repl_sheets_schema_version: M5_EVALUATE_REPL_SHEETS_SCHEMA_VERSION,
        schema_ref: M5_EVALUATE_REPL_SHEETS_SCHEMA_REF.to_owned(),
        set_id: M5_EVALUATE_REPL_SHEETS_SET_ID.to_owned(),
        as_of: M5_EVALUATE_REPL_SHEETS_AS_OF.to_owned(),
        freeze_gate_ref: M5_EVALUATE_REPL_SHEETS_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed set of M5 evaluate/REPL review sheets and console emissions. \
                  Every evaluation carries one posture pill that pins one purity class (pure, \
                  unknown, may-mutate) and one approval disposition (not-required, pending, \
                  approved, denied, blocked, expired), so the user is told whether an expression is \
                  harmless inspection, unknown, or mutating before dispatch and after a result \
                  returns: a pure expression needs no approval, an unknown or mutating expression \
                  discloses its risk and requires review, a pending/denied/blocked/expired \
                  evaluation never permits dispatch and carries no result, an effectful expression \
                  against an inspect-only context is blocked rather than silently mutating a \
                  recording, and actor lineage names who requested and who reviewed it. Every \
                  console emission carries one pill that pins one direction (user input vs target \
                  output) derived from its stream class, one liveness (live vs replayed), and a \
                  redaction marker, so console history and export packets distinguish interactive \
                  input from target output, never present a replayed line as live, and preserve \
                  redaction review rather than flattening one transcript."
            .to_owned(),
        source_schema_refs: strvec(&[
            "schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json",
            "schemas/runtime/console_event.schema.json",
            "schemas/debug/m5_debug_contracts.schema.json",
        ]),
        producer_refs: strvec(&[
            "crates/aureline-debug/src/m5_evaluate_repl_sheets/mod.rs",
            "crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/mod.rs",
            "crates/aureline-runtime/src/m5_task_event_envelope_bus/mod.rs",
            "crates/aureline-runtime/src/debug/records.rs",
        ]),
        consumer_surfaces: vec![
            DebugConsumer::CoreDebugger,
            DebugConsumer::NotebookDebug,
            DebugConsumer::Profiler,
            DebugConsumer::IncidentReview,
            DebugConsumer::SupportExport,
            DebugConsumer::AiContext,
            DebugConsumer::ReviewWorkspace,
            DebugConsumer::CliHeadless,
        ],
        evaluations,
        console,
        invariants,
        raw_payload_excluded: true,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

const EVAL_PROOF_REF: &str =
    "fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json";
const EVAL_UNKNOWN_PROOF_REF: &str =
    "fixtures/debug/mapping_cases/generated_source_spec_unknown.json";
const REPLAY_INSPECT_PROOF_REF: &str =
    "fixtures/runtime/m3/replay_packets/container_debug_layout_only_mutating.json";
const CONSOLE_LIVE_PROOF_REF: &str =
    "fixtures/runtime/browser_inspection_cases/console_live_exact_mapping.yaml";
const CONSOLE_REPLAY_PROOF_REF: &str =
    "fixtures/runtime/browser_inspection_cases/console_stale_source_map_disclosed.yaml";

const SESSION_MAIN: &str = "debug.session:local-launch:0001";
const SESSION_REPLAY: &str = "debug.session:replay-capture:0009";
const THREAD_MAIN: &str = "debug.thread:main:0001";
const FRAME_CURRENT: &str = "debug.frame:main_exact_current:0001";

const EVAL_PURE_FRAME: &str = "debug.evaluate:pure_frame_read:0001";
const EVAL_PURE_ERROR: &str = "debug.evaluate:pure_global_error:0002";
const EVAL_UNKNOWN_PENDING: &str = "debug.evaluate:unknown_session_pending:0003";
const EVAL_UNKNOWN_APPROVED: &str = "debug.evaluate:unknown_notebook_approved:0004";
const EVAL_MUTATE_APPROVED: &str = "debug.evaluate:mutate_frame_approved:0005";
const EVAL_MUTATE_DENIED: &str = "debug.evaluate:mutate_thread_denied:0006";
const EVAL_MUTATE_BLOCKED: &str = "debug.evaluate:mutate_replay_blocked:0007";
const EVAL_UNKNOWN_EXPIRED: &str = "debug.evaluate:unknown_repl_expired:0008";
const EVAL_MUTATE_REDACTED: &str = "debug.evaluate:mutate_frame_redacted:0009";

fn build_evaluations() -> Vec<EvaluateRecord> {
    use ApprovalDisposition::*;
    use EvaluateActorClass::*;
    use EvaluateContextAuthority::*;
    use EvaluateContextScope::*;
    use EvaluateOutcome::*;
    use EvaluatePurityClass::*;
    use EvaluateRedactionClass::*;

    vec![
        // 1. Pure read of a local on the current frame: harmless inspection, no approval,
        //    a clean live result.
        EvaluateRecord::build(
            EVAL_PURE_FRAME,
            "expr:digest:p1aa11",
            NotRedacted,
            ExpressionContext::build(
                SESSION_MAIN,
                Some(THREAD_MAIN),
                Some(FRAME_CURRENT),
                Frame,
                LiveMutable,
                None,
                None,
            ),
            Pure,
            NotRequired,
            ActorLineage::build(
                "actor:user:0001",
                Human,
                DebugConsumer::CoreDebugger,
                None,
                None,
            ),
            Some(EvaluateResult::build(
                "debug.evaluate.result:pure_frame_read:0001",
                Completed,
                "Returned the request id scalar from the current frame.",
                Some("value:digest:r1aa11"),
                "No side effects: a read-only field access.",
                false,
                NotRedacted,
                M5_EVALUATE_REPL_SHEETS_AS_OF,
            )),
            EVAL_PROOF_REF,
            "Pure read of a local on the current frame: classified pure, dispatched without \
             approval, and returned a live value — the clean harmless-inspection path.",
        ),
        // 2. Pure global read that raised an error: pure, no approval, an error result with
        //    no value body.
        EvaluateRecord::build(
            EVAL_PURE_ERROR,
            "expr:digest:p2bb22",
            NotRedacted,
            ExpressionContext::build(
                SESSION_MAIN,
                Some(THREAD_MAIN),
                None,
                GlobalScope,
                LiveMutable,
                None,
                None,
            ),
            Pure,
            NotRequired,
            ActorLineage::build(
                "actor:automation:ci-probe",
                Automation,
                DebugConsumer::CliHeadless,
                None,
                None,
            ),
            Some(EvaluateResult::build(
                "debug.evaluate.result:pure_global_error:0002",
                RaisedError,
                "Evaluation raised a name-resolution error; no value returned.",
                None,
                "No side effects: the error was raised before any mutation.",
                false,
                NotRedacted,
                M5_EVALUATE_REPL_SHEETS_AS_OF,
            )),
            EVAL_PROOF_REF,
            "Pure global read that raised a name-resolution error: classified pure, dispatched \
             without approval, and disclosed as an error with no value body.",
        ),
        // 3. Unknown-effect expression awaiting review: discloses risk, requires approval,
        //    pending — no result, never silently run.
        EvaluateRecord::build(
            EVAL_UNKNOWN_PENDING,
            "expr:digest:u3cc33",
            NotRedacted,
            ExpressionContext::build(
                SESSION_MAIN,
                Some(THREAD_MAIN),
                None,
                Session,
                LiveMutable,
                None,
                None,
            ),
            Unknown,
            Pending,
            ActorLineage::build(
                "actor:user:0001",
                Human,
                DebugConsumer::CoreDebugger,
                None,
                None,
            ),
            None,
            EVAL_UNKNOWN_PROOF_REF,
            "Unknown-effect expression awaiting review: classified unknown, side-effect risk \
             disclosed, approval pending, and held without dispatch — never silently run.",
        ),
        // 4. Unknown-effect expression in a notebook, approved by a reviewer: cleared and
        //    dispatched, with the reviewer named.
        EvaluateRecord::build(
            EVAL_UNKNOWN_APPROVED,
            "expr:digest:u4dd44",
            NotRedacted,
            ExpressionContext::build(
                SESSION_MAIN,
                Some(THREAD_MAIN),
                None,
                Repl,
                LiveMutable,
                Some("notebook:doc:analysis#cell-3"),
                None,
            ),
            Unknown,
            Approved,
            ActorLineage::build(
                "actor:ai-agent:composer",
                AiAgent,
                DebugConsumer::AiContext,
                Some("actor:user:0001"),
                Some("actor:user:0001"),
            ),
            Some(EvaluateResult::build(
                "debug.evaluate.result:unknown_notebook_approved:0004",
                Completed,
                "Returned the dataframe shape tuple after reviewer approval.",
                Some("value:digest:r4dd44"),
                "Declared unknown effects; reviewer approved; no mutation observed on run.",
                false,
                NotRedacted,
                M5_EVALUATE_REPL_SHEETS_AS_OF,
            )),
            EVAL_PROOF_REF,
            "Unknown-effect notebook REPL expression approved by a reviewer: side-effect risk \
             disclosed, approval cleared with the reviewer named, then dispatched.",
        ),
        // 5. May-mutate expression approved by a reviewer: ran and observed a mutation,
        //    returning no value.
        EvaluateRecord::build(
            EVAL_MUTATE_APPROVED,
            "expr:digest:m5ee55",
            NotRedacted,
            ExpressionContext::build(
                SESSION_MAIN,
                Some(THREAD_MAIN),
                Some(FRAME_CURRENT),
                Frame,
                LiveMutable,
                None,
                None,
            ),
            MayMutate,
            Approved,
            ActorLineage::build(
                "actor:user:0001",
                Human,
                DebugConsumer::CoreDebugger,
                Some("actor:user:0002"),
                None,
            ),
            Some(EvaluateResult::build(
                "debug.evaluate.result:mutate_frame_approved:0005",
                NoValue,
                "Assigned a new value to the local; no value returned.",
                None,
                "Mutated a local on the current frame; mutation observed and disclosed.",
                true,
                NotRedacted,
                M5_EVALUATE_REPL_SHEETS_AS_OF,
            )),
            EVAL_PROOF_REF,
            "May-mutate assignment approved by a reviewer: side-effect risk disclosed, approval \
             cleared, then dispatched with the observed mutation recorded.",
        ),
        // 6. May-mutate expression denied by a reviewer: never runs, no result; its
        //    expression was secret-redacted.
        EvaluateRecord::build(
            EVAL_MUTATE_DENIED,
            "expr:digest:m6ff66",
            SecretRedacted,
            ExpressionContext::build(
                SESSION_MAIN,
                Some(THREAD_MAIN),
                None,
                Thread,
                LiveMutable,
                None,
                None,
            ),
            MayMutate,
            Denied,
            ActorLineage::build(
                "actor:user:0001",
                Human,
                DebugConsumer::CoreDebugger,
                Some("actor:user:0002"),
                None,
            ),
            None,
            EVAL_PROOF_REF,
            "May-mutate expression denied by a reviewer: side-effect risk disclosed, the request \
             held without dispatch, and the secret-bearing expression withheld.",
        ),
        // 7. May-mutate expression against a replay (inspect-only) context: blocked, never
        //    mutating the recording.
        EvaluateRecord::build(
            EVAL_MUTATE_BLOCKED,
            "expr:digest:m7gg77",
            NotRedacted,
            ExpressionContext::build(
                SESSION_REPLAY,
                None,
                None,
                Session,
                InspectOnly,
                None,
                Some("replay:capture:task-run-42"),
            ),
            MayMutate,
            Blocked,
            ActorLineage::build(
                "actor:user:0001",
                Human,
                DebugConsumer::IncidentReview,
                None,
                None,
            ),
            None,
            REPLAY_INSPECT_PROOF_REF,
            "May-mutate expression issued against an inspect-only replay capture: blocked before \
             dispatch so it can never mutate the recording.",
        ),
        // 8. Unknown-effect expression whose approval expired before dispatch: never runs,
        //    no result.
        EvaluateRecord::build(
            EVAL_UNKNOWN_EXPIRED,
            "expr:digest:u8hh88",
            NotRedacted,
            ExpressionContext::build(
                SESSION_MAIN,
                Some(THREAD_MAIN),
                None,
                Repl,
                LiveMutable,
                None,
                None,
            ),
            Unknown,
            Expired,
            ActorLineage::build(
                "actor:user:0001",
                Human,
                DebugConsumer::CoreDebugger,
                Some("actor:user:0002"),
                None,
            ),
            None,
            EVAL_PROOF_REF,
            "Unknown-effect expression whose approval lapsed before dispatch: the expired state \
             is preserved and the request is held without dispatch.",
        ),
        // 9. May-mutate expression approved, with a secret-redacted result body: ran, but
        //    the result body is withheld.
        EvaluateRecord::build(
            EVAL_MUTATE_REDACTED,
            "expr:digest:m9ii99",
            NotRedacted,
            ExpressionContext::build(
                SESSION_MAIN,
                Some(THREAD_MAIN),
                Some(FRAME_CURRENT),
                Frame,
                LiveMutable,
                None,
                None,
            ),
            MayMutate,
            Approved,
            ActorLineage::build(
                "actor:ai-agent:composer",
                AiAgent,
                DebugConsumer::AiContext,
                Some("actor:user:0002"),
                Some("actor:user:0001"),
            ),
            Some(EvaluateResult::build(
                "debug.evaluate.result:mutate_frame_redacted:0009",
                Completed,
                "Rotated the credential and returned the new handle (body withheld).",
                None,
                "Mutated a credential field; mutation observed; result body withheld as secret.",
                true,
                SecretRedacted,
                M5_EVALUATE_REPL_SHEETS_AS_OF,
            )),
            EVAL_PROOF_REF,
            "May-mutate credential rotation approved by a reviewer: dispatched with the mutation \
             observed, but the secret result body is withheld and disclosed as redacted.",
        ),
    ]
}

fn build_console() -> Vec<ConsoleEmission> {
    use ConsoleLiveness::*;
    use ConsoleStreamClass::*;
    use EvaluateRedactionClass::*;

    vec![
        // 1. The user-typed evaluate input for the pure frame read.
        ConsoleEmission::build(
            "debug.console:eval_input_pure:0001",
            1,
            EvaluateInput,
            Live,
            SESSION_MAIN,
            Some(THREAD_MAIN),
            Some(FRAME_CURRENT),
            None,
            None,
            Some(EVAL_PURE_FRAME),
            Some("console:digest:c1aa11"),
            NotRedacted,
            true,
            CONSOLE_LIVE_PROOF_REF,
            "Interactive evaluate input the user typed for the pure frame read: classified as \
             user input, live, and preserved for replay.",
        ),
        // 2. The target-output result echoed back for the pure frame read.
        ConsoleEmission::build(
            "debug.console:eval_result_pure:0002",
            2,
            EvaluateResult,
            Live,
            SESSION_MAIN,
            Some(THREAD_MAIN),
            Some(FRAME_CURRENT),
            None,
            None,
            Some(EVAL_PURE_FRAME),
            Some("console:digest:c2bb22"),
            NotRedacted,
            true,
            CONSOLE_LIVE_PROOF_REF,
            "Target output echoing the pure frame read's result: classified as target output and \
             kept distinct from the interactive input that produced it.",
        ),
        // 3. Live stdin the user typed: interactive input.
        ConsoleEmission::build(
            "debug.console:stdin_live:0003",
            3,
            Stdin,
            Live,
            SESSION_MAIN,
            Some(THREAD_MAIN),
            None,
            None,
            None,
            None,
            Some("console:digest:c3cc33"),
            NotRedacted,
            true,
            CONSOLE_LIVE_PROOF_REF,
            "Live standard input the user typed: classified as interactive user input, never \
             flattened into the target's output stream.",
        ),
        // 4. Live stdout from the target: target output.
        ConsoleEmission::build(
            "debug.console:stdout_live:0004",
            4,
            Stdout,
            Live,
            SESSION_MAIN,
            Some(THREAD_MAIN),
            None,
            None,
            None,
            None,
            Some("console:digest:c4dd44"),
            NotRedacted,
            true,
            CONSOLE_LIVE_PROOF_REF,
            "Live standard output from the target program: classified as target output, distinct \
             from interactive input.",
        ),
        // 5. Live stderr carrying personal data: target output with the body withheld.
        ConsoleEmission::build(
            "debug.console:stderr_pii:0005",
            5,
            Stderr,
            Live,
            SESSION_MAIN,
            Some(THREAD_MAIN),
            None,
            None,
            None,
            None,
            None,
            PiiRedacted,
            true,
            CONSOLE_LIVE_PROOF_REF,
            "Live standard error matching a personal-data class: classified as target output with \
             its body withheld and the redaction preserved for review.",
        ),
        // 6. Replayed debug-console line from a capture carrying a secret: disclosed as
        //    replayed with the body withheld.
        ConsoleEmission::build(
            "debug.console:debug_replayed_secret:0006",
            6,
            DebugConsole,
            ReplayedCapture,
            SESSION_REPLAY,
            None,
            None,
            None,
            Some("replay:capture:task-run-42"),
            None,
            None,
            SecretRedacted,
            true,
            CONSOLE_REPLAY_PROOF_REF,
            "Replayed debug-console line from a captured session matching a secret class: \
             disclosed as replayed, never shown as live, with its body withheld.",
        ),
        // 7. Replayed stdout shown in a notebook output area, policy-withheld: disclosed as
        //    replayed with the body withheld.
        ConsoleEmission::build(
            "debug.console:stdout_replayed_notebook:0007",
            7,
            Stdout,
            ReplayedCapture,
            SESSION_REPLAY,
            None,
            None,
            Some("notebook:doc:analysis#cell-3"),
            Some("replay:capture:task-run-42"),
            None,
            None,
            PolicyWithheld,
            true,
            CONSOLE_REPLAY_PROOF_REF,
            "Replayed target output shown in a notebook output area, withheld by policy: disclosed \
             as replayed, never shown as live, with its body withheld for review.",
        ),
        // 8. The target-output result echoed for the approved may-mutate evaluation.
        ConsoleEmission::build(
            "debug.console:eval_result_mutate:0008",
            8,
            EvaluateResult,
            Live,
            SESSION_MAIN,
            Some(THREAD_MAIN),
            Some(FRAME_CURRENT),
            None,
            None,
            Some(EVAL_MUTATE_APPROVED),
            Some("console:digest:c8ee88"),
            NotRedacted,
            true,
            CONSOLE_LIVE_PROOF_REF,
            "Target output echoing the approved may-mutate evaluation's result: classified as \
             target output and linked back to the evaluate record that produced it.",
        ),
    ]
}

fn invariant(invariant_id: &str, statement: &str, holds: bool) -> EvaluateReplInvariant {
    EvaluateReplInvariant {
        invariant_id: invariant_id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    evaluations: &[EvaluateRecord],
    console: &[ConsoleEmission],
) -> Vec<EvaluateReplInvariant> {
    // Every evaluation carries one posture pill whose flags equal the derivation from its
    // purity, disposition, and context authority.
    let eval_one_canonical_pill = evaluations.iter().all(|e| {
        e.posture
            .matches_derivation(e.purity, e.disposition, e.context.authority)
            && e.posture.purity_token == e.posture.purity.as_str()
            && e.posture.disposition_token == e.posture.disposition.as_str()
    });

    // The full purity vocabulary is materialized.
    let purity_complete = EvaluatePurityClass::ALL
        .iter()
        .all(|p| evaluations.iter().any(|e| e.purity() == *p));

    // The full disposition vocabulary is materialized.
    let disposition_complete = ApprovalDisposition::ALL
        .iter()
        .all(|d| evaluations.iter().any(|e| e.disposition() == *d));

    // Every evaluation discloses its side-effect risk before dispatch: an unknown or
    // mutating expression discloses risk and requires approval; a pure one never claims a
    // risk, and at least one effectful evaluation exists.
    let side_effect_risk_disclosed = evaluations.iter().all(|e| {
        e.posture.discloses_side_effect_risk == e.purity.discloses_side_effect_risk()
            && e.posture.approval_required == e.purity.requires_approval()
            && e.posture.discloses_side_effect_risk == e.posture.approval_required
    }) && evaluations
        .iter()
        .any(|e| e.purity.discloses_side_effect_risk());

    // An unknown or mutating expression requires approval and never runs unless approved:
    // a record that requires approval and is not approved never permits dispatch.
    let approval_never_bypassed = evaluations.iter().all(|e| {
        if e.posture.approval_required && e.disposition != ApprovalDisposition::Approved {
            !e.posture.permits_dispatch
        } else {
            true
        }
    }) && evaluations
        .iter()
        .any(|e| e.posture.approval_required && e.posture.permits_dispatch);

    // A withheld request carries no result: a result implies dispatch was permitted, and a
    // withheld (pending/denied/blocked/expired) evaluation exists with no result.
    let withheld_has_no_result = evaluations
        .iter()
        .all(|e| e.result.is_none() || e.posture.permits_dispatch)
        && evaluations
            .iter()
            .any(|e| !e.posture.permits_dispatch && e.result.is_none());

    // The blocked, denied, and expired states are all materialized and none permit
    // dispatch, so a non-cleared approval state is never lost.
    let blocked_denied_expired_preserved = [
        ApprovalDisposition::Blocked,
        ApprovalDisposition::Denied,
        ApprovalDisposition::Expired,
    ]
    .iter()
    .all(|d| evaluations.iter().any(|e| e.disposition() == *d))
        && evaluations
            .iter()
            .filter(|e| e.disposition().is_terminal_block())
            .all(|e| !e.posture.permits_dispatch && e.result.is_none());

    // An effectful expression against an inspect-only context is blocked rather than run,
    // and such a case is materialized.
    let inspect_only_blocks_effectful = evaluations.iter().all(|e| {
        if !e.context.authority.allows_mutation() && e.purity.requires_approval() {
            !e.posture.permits_dispatch
        } else {
            true
        }
    }) && evaluations.iter().any(|e| {
        !e.context.authority.allows_mutation()
            && e.purity.requires_approval()
            && e.posture.blocked_by_inspect_only
    });

    // Every evaluation names its requesting actor and class; every approval-cleared
    // effectful evaluation names its reviewer.
    let actor_lineage_preserved = evaluations.iter().all(|e| {
        !e.actor.requested_by_ref.is_empty()
            && e.actor.is_consistent()
            && (!(e.purity.requires_approval() && e.disposition == ApprovalDisposition::Approved)
                || e.actor.reviewed_by_ref.is_some())
    });

    // No raw expression text crosses the boundary: every evaluation carries an opaque
    // expression digest, and every redacted result withholds its body.
    let no_raw_expression_or_value = evaluations.iter().all(|e| {
        !e.expression_digest.is_empty()
            && e.result
                .as_ref()
                .map(|r| !r.is_redacted || r.result_repr_digest.is_none())
                .unwrap_or(true)
    });

    // Console input and output are separated: both directions are materialized, each
    // emission's direction matches its stream class, and a user-input line is never
    // mislabeled as target output.
    let console_input_output_separated = ConsoleDirection::ALL
        .iter()
        .all(|d| console.iter().any(|c| c.direction() == *d))
        && console.iter().all(|c| {
            c.direction == c.stream.direction()
                && c.pill.is_user_input == c.direction.is_user_input()
        });

    // Every console emission carries one pill whose flags equal the derivation from its
    // stream class, liveness, and redaction.
    let console_one_canonical_pill = console.iter().all(|c| {
        c.pill.matches_derivation(c.stream, c.liveness, c.redaction)
            && c.pill.direction_token == c.pill.direction.as_str()
            && c.pill.stream_token == c.pill.stream.as_str()
            && c.pill.liveness_token == c.pill.liveness.as_str()
    });

    // A replayed console line always discloses and is never shown as live; a live line
    // never claims replay, and a replayed line is materialized.
    let replayed_never_shown_as_live = console.iter().all(|c| {
        c.pill.is_replayed == (c.liveness == ConsoleLiveness::ReplayedCapture)
            && c.pill.is_live == (c.liveness == ConsoleLiveness::Live)
            && (!c.pill.is_replayed || c.pill.requires_disclosure)
    }) && console.iter().any(|c| c.pill.is_replayed);

    // A redacted console emission withholds its body and is marked; redaction review is
    // preserved rather than flattened, and a redacted emission is materialized.
    let console_redaction_preserved = console.iter().all(|c| {
        c.redaction.is_redacted() == c.pill.is_redacted
            && c.pill.body_present != c.pill.is_redacted
            && c.pill.body_present == c.body_digest.is_some()
    }) && console.iter().any(|c| c.pill.is_redacted);

    // Every console emission carries its session linkage, and an evaluate-linked emission
    // resolves to an evaluation in the set.
    let console_session_linkage_preserved =
        console.iter().all(|c| {
            !c.session_ref.is_empty()
                && c.linked_evaluate_id
                    .as_ref()
                    .map(|id| evaluations.iter().any(|e| &e.evaluate_id == id))
                    .unwrap_or(true)
        }) && console.iter().any(|c| c.linked_evaluate_id.is_some());

    // The full redaction vocabulary is materialized across expressions, results, and
    // console bodies.
    let redaction_vocabulary_complete = EvaluateRedactionClass::ALL.iter().all(|class| {
        evaluations.iter().any(|e| {
            e.expression_redaction == *class
                || e.result
                    .as_ref()
                    .map(|r| r.redaction == *class)
                    .unwrap_or(false)
        }) || console.iter().any(|c| c.redaction == *class)
    });

    // Every evaluation and emission retains its typed tokens and cites an export-safe proof
    // packet, so export never flattens them into rendered chrome.
    let export_retains_state = evaluations.iter().all(|e| {
        !e.posture.purity_token.is_empty()
            && !e.posture.disposition_token.is_empty()
            && !e.proof_packet_ref.is_empty()
            && is_export_safe_ref(&e.proof_packet_ref)
    }) && console.iter().all(|c| {
        !c.pill.direction_token.is_empty()
            && !c.proof_packet_ref.is_empty()
            && is_export_safe_ref(&c.proof_packet_ref)
    });

    vec![
        invariant(
            "evaluate.one_canonical_posture_pill",
            "Every evaluation carries exactly one posture pill whose purity and disposition tokens \
             come from the frozen vocabulary and whose flags equal their derivation.",
            eval_one_canonical_pill,
        ),
        invariant(
            "evaluate.purity_vocabulary_complete",
            "Pure, unknown, and may-mutate are all materialized.",
            purity_complete,
        ),
        invariant(
            "evaluate.disposition_vocabulary_complete",
            "Not-required, pending, approved, denied, blocked, and expired are all materialized.",
            disposition_complete,
        ),
        invariant(
            "evaluate.side_effect_risk_disclosed_before_dispatch",
            "Every unknown or may-mutate expression discloses its side-effect risk and requires \
             approval before dispatch; a pure expression never claims a side-effect risk.",
            side_effect_risk_disclosed,
        ),
        invariant(
            "evaluate.unknown_or_mutating_never_runs_unless_approved",
            "An expression that requires approval and is not approved never permits dispatch, so no \
             surface can silently run unknown or mutating evaluation under a harmless-inspect label.",
            approval_never_bypassed,
        ),
        invariant(
            "evaluate.withheld_request_carries_no_result",
            "A result is present only when dispatch was permitted; a pending, denied, blocked, or \
             expired evaluation carries no result.",
            withheld_has_no_result,
        ),
        invariant(
            "evaluate.blocked_denied_expired_states_preserved",
            "The blocked, denied, and expired approval states are all materialized and none permit \
             dispatch, so a non-cleared approval state is never lost in UI, CLI, or support packets.",
            blocked_denied_expired_preserved,
        ),
        invariant(
            "evaluate.inspect_only_context_blocks_effectful_evaluation",
            "An effectful expression issued against an inspect-only context is blocked rather than \
             dispatched, so it can never silently mutate a core-file or replay target.",
            inspect_only_blocks_effectful,
        ),
        invariant(
            "evaluate.actor_lineage_preserved",
            "Every evaluation names its requesting actor and class, and every approval-cleared \
             effectful evaluation names its reviewer.",
            actor_lineage_preserved,
        ),
        invariant(
            "evaluate.no_raw_expression_or_value_body",
            "Every evaluation carries an opaque expression digest rather than raw source, and every \
             redacted result withholds its value body.",
            no_raw_expression_or_value,
        ),
        invariant(
            "console.interactive_input_and_target_output_separated",
            "Both directions are materialized, each emission's direction matches its stream class, \
             and a user-input line is never mislabeled as target output.",
            console_input_output_separated,
        ),
        invariant(
            "console.one_canonical_emission_pill",
            "Every console emission carries one pill whose direction, stream, and liveness tokens \
             come from the frozen vocabulary and whose flags equal their derivation.",
            console_one_canonical_pill,
        ),
        invariant(
            "console.replayed_never_shown_as_live",
            "A replayed console line always discloses and is never shown as live; a live line never \
             claims replay.",
            replayed_never_shown_as_live,
        ),
        invariant(
            "console.redaction_review_preserved",
            "Every redacted console emission withholds its body and is marked redacted, so redaction \
             review is preserved rather than flattened into one transcript.",
            console_redaction_preserved,
        ),
        invariant(
            "console.session_and_evaluate_linkage_preserved",
            "Every console emission carries its session linkage, and an evaluate-linked emission \
             resolves to an evaluation in the set.",
            console_session_linkage_preserved,
        ),
        invariant(
            "set.redaction_vocabulary_complete",
            "Not-redacted, secret, personal-data, and policy-withheld redaction classes are all \
             materialized across expressions, results, and console bodies.",
            redaction_vocabulary_complete,
        ),
        invariant(
            "set.export_retains_evaluate_and_console_state",
            "Every evaluation and console emission retains its typed purity/approval/direction tokens \
             and cites an export-safe proof packet, so support export never flattens it into chrome.",
            export_retains_state,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the evaluate/REPL sheet set as human-readable lines for CLI/headless and
/// support.
pub fn m5_evaluate_repl_sheet_lines(set: &EvaluateReplSheetSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "M5 evaluate/REPL sheets & console emissions — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Evaluations: {}  Console: {}  Invariants: {}",
        set.evaluations.len(),
        set.console.len(),
        set.invariants.len(),
    ));

    lines.push("Evaluations:".to_owned());
    for ev in &set.evaluations {
        lines.push(format!(
            "  - {} purity={} disposition={} posture={}",
            ev.evaluate_id, ev.purity_token, ev.disposition_token, ev.posture.label,
        ));
        lines.push(format!(
            "      approval_required={} discloses_side_effect_risk={} permits_dispatch={} review_affordance={} blocked_inspect_only={}",
            ev.posture.approval_required,
            ev.posture.discloses_side_effect_risk,
            ev.posture.permits_dispatch,
            ev.posture.requires_review_affordance,
            ev.posture.blocked_by_inspect_only,
        ));
        lines.push(format!(
            "      context=[{} scope={} authority={}] actor={} class={} reviewer={}",
            ev.context.session_id,
            ev.context.scope_token,
            ev.context.authority_token,
            ev.actor.requested_by_ref,
            ev.actor.actor_class_token,
            ev.actor.reviewed_by_ref.as_deref().unwrap_or("-"),
        ));
        match &ev.result {
            Some(r) => lines.push(format!(
                "      result={} outcome={} observed_mutation={} body_present={} redaction={}",
                r.result_id,
                r.outcome_token,
                r.observed_mutation,
                r.result_body_present,
                r.redaction_token,
            )),
            None => lines.push("      result=<withheld: not dispatched>".to_owned()),
        }
        lines.push(format!("      {}", ev.summary));
        lines.push(format!("      proof: {}", ev.proof_packet_ref));
    }

    lines.push("Console:".to_owned());
    for em in &set.console {
        lines.push(format!(
            "  - #{} {} [{}] stream={} liveness={} body_present={} redaction={} replayable={} link={}",
            em.sequence,
            em.emission_id,
            em.direction_token,
            em.stream_token,
            em.liveness_token,
            em.pill.body_present,
            em.redaction_token,
            em.replayable,
            em.linked_evaluate_id.as_deref().unwrap_or("-"),
        ));
        lines.push(format!("      {}", em.summary));
        lines.push(format!("      proof: {}", em.proof_packet_ref));
    }

    lines.push("Invariants:".to_owned());
    for i in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
