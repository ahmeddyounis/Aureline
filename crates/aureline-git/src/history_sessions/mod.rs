//! Durable history-surgery session objects and their first consumers.
//!
//! This module turns risky Git workflows into durable, serde-serializable
//! product objects instead of transient modal state or CLI folklore. It defines
//! one [`HistorySession`] descriptor per durable object — a conflict session, a
//! sequence-edit session, a stash/shelf entry, a publish/ref-update proposal, or
//! a recovery checkpoint — each carrying **explicit identity and lifecycle**:
//! repository and worktree refs, target refs, path scope, an unresolved count,
//! checkpoint lineage, and (where the source workflow requires exact order or
//! source-text inspection) refs to both the raw todo/patch text and the
//! structured cards derived from it.
//!
//! The same descriptor then *drives* every M5 history consumer through one
//! deterministic projection ([`HistorySession::project`]): the desktop surface,
//! review, search, AI context, CLI/headless result packets, redaction-safe
//! support export, and provider-overlay continuity all read the identical truth
//! instead of re-deriving it. Because each [`SessionConsumerBinding`] is
//! *derived* from its descriptor, a continue/abort/skip/apply/pop/drop/
//! create-branch/publish/restore action can never live only in local UI memory:
//! if a user can act on it, Aureline can explain and export it.
//!
//! Three guardrails are encoded in the projection rather than left to prose:
//!
//! * Apply, pop, drop, and create-branch from a stash stay **distinct verbs**;
//!   every surface discloses all four, and only mutation surfaces mark them
//!   actionable.
//! * A publish/ref-update proposal allows a network mutation only after its
//!   divergence is known, its affected approvals and checks are not invalidated,
//!   and its recovery lineage is present — never silently.
//! * Repository and worktree identity is preserved on every binding, so a
//!   session survives reopen, export, support, and provider degradation.
//!
//! The boundary schema is
//! [`schemas/git/history-session.schema.json`](../../../../schemas/git/history-session.schema.json).
//! The protected fixture corpus is
//! [`fixtures/git/m5/history-sessions/`](../../../../fixtures/git/m5/history-sessions/).
//! The checked-in canonical map is
//! [`artifacts/git/m5/history_sessions/history_session_first_consumers.json`](../../../../artifacts/git/m5/history_sessions/history_session_first_consumers.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth::{
    RISKY_VCS_APPROVAL_STATES, RISKY_VCS_CHECKPOINT_STATES, RISKY_VCS_CHECKPOINT_TRIGGER_KINDS,
    RISKY_VCS_CHECK_INVALIDATION_STATES, RISKY_VCS_CONFLICT_RESOLUTION_MODES,
    RISKY_VCS_CONFLICT_STATES, RISKY_VCS_DIVERGENCE_CLASSES, RISKY_VCS_PUBLISH_MODES,
    RISKY_VCS_REF_UPDATE_STATES, RISKY_VCS_RESTORE_OPTION_CLASSES, RISKY_VCS_SEQUENCE_STATES,
    RISKY_VCS_STASH_STATES,
};
use crate::freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix::HistorySurgerySession;

/// Schema version for [`HistorySessionConsumerMap`].
pub const HISTORY_SESSION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`HistorySessionConsumerMap`].
pub const HISTORY_SESSION_MAP_RECORD_KIND: &str = "git_history_session_map";

/// Stable record-kind tag carried by [`HistorySession`].
pub const HISTORY_SESSION_DESCRIPTOR_RECORD_KIND: &str = "git_history_session_descriptor";

/// Stable record-kind tag carried by [`SessionConsumerBinding`].
pub const HISTORY_SESSION_CONSUMER_BINDING_RECORD_KIND: &str =
    "git_history_session_consumer_binding";

/// Stable record-kind tag carried by [`HistorySessionSupportExport`].
pub const HISTORY_SESSION_SUPPORT_EXPORT_RECORD_KIND: &str = "git_history_session_support_export";

/// Repo-relative path of the boundary schema.
pub const HISTORY_SESSION_SCHEMA_REF: &str = "schemas/git/history-session.schema.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const HISTORY_SESSION_FIXTURE_DIR: &str = "fixtures/git/m5/history-sessions";

/// Repo-relative path of the checked-in canonical first-consumers map.
pub const HISTORY_SESSION_ARTIFACT_REF: &str =
    "artifacts/git/m5/history_sessions/history_session_first_consumers.json";

/// Identity fields a support export must retain after redaction so a session can
/// be reconstructed without leaking raw boundary material.
pub const HISTORY_SESSION_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 8] = [
    "session_kind",
    "repo_ref",
    "worktree_ref",
    "target_refs",
    "path_scope_tokens",
    "unresolved_count",
    "checkpoint_lineage_refs",
    "lifecycle_state",
];

/// Closed vocabulary of action verbs a history session can bind.
///
/// Each verb is a distinct action surface; in particular the four stash verbs
/// (`apply`, `pop`, `drop`, `create_branch`) never collapse into one.
pub const HISTORY_SESSION_ACTIONS: &[&str] = &[
    "continue",
    "abort",
    "skip",
    "edit_sequence",
    "apply",
    "pop",
    "drop",
    "create_branch",
    "publish",
    "withdraw",
    "restore",
    "prune",
];

/// The four distinct stash verbs that must stay separable on every surface.
pub const HISTORY_SESSION_STASH_VERBS: [&str; 4] = ["apply", "pop", "drop", "create_branch"];

/// Consumer surface that reuses a history-session descriptor instead of
/// re-deriving it from transient state.
///
/// These are the first real consumers this lane wires up; each one must read the
/// same durable object so a risky operation stays explainable and exportable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionConsumerSurface {
    /// Desktop shell chrome, activity center, and history-edit sheets.
    Desktop,
    /// Review diff, summary, publish, and history-edit rows.
    Review,
    /// Search result and zero-result rows over session state.
    Search,
    /// AI-context assembly and evidence inspectors.
    AiContext,
    /// CLI / headless replay or JSON result packets.
    CliHeadless,
    /// Redaction-safe support / export rows.
    SupportExport,
    /// Provider overlay (status, PR, checks) layered over local truth.
    ProviderOverlay,
}

impl SessionConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Desktop,
        Self::Review,
        Self::Search,
        Self::AiContext,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ProviderOverlay,
    ];

    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Review => "review",
            Self::Search => "search",
            Self::AiContext => "ai_context",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ProviderOverlay => "provider_overlay",
        }
    }

    /// Whether this surface can drive a session action (continue, apply, publish).
    ///
    /// Read and continuity surfaces still *disclose* the available verbs so the
    /// user can see them, but only a mutation surface marks them actionable.
    pub const fn is_mutation_surface(self) -> bool {
        matches!(self, Self::Desktop | Self::Review | Self::CliHeadless)
    }

    /// Whether this surface may hydrate a raw todo/patch body for inspection.
    ///
    /// Support export stays metadata-only, and a provider overlay never embeds
    /// local raw bodies, so both keep this `false`.
    pub const fn allows_body_export(self) -> bool {
        !matches!(self, Self::SupportExport | Self::ProviderOverlay)
    }
}

/// Durable history-surgery session object with explicit identity and lifecycle.
///
/// One descriptor exists per risky operation a user can continue or abort. The
/// structured fields are the canonical substrate; [`HistorySession::project`]
/// derives the truth a consumer renders, so every surface reads the same object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySession {
    /// Record-kind tag; must equal [`HISTORY_SESSION_DESCRIPTOR_RECORD_KIND`].
    pub record_kind: String,
    /// Which history-surgery object this descriptor represents.
    pub session_kind: HistorySurgerySession,
    /// Canonical record-kind of the underlying truth object; must equal
    /// [`HistorySurgerySession::canonical_record_kind`] for `session_kind`.
    pub canonical_record_kind: String,
    /// Stable session identity (referenced by bindings and support export).
    pub session_id: String,
    /// Redaction-safe repository ref the session belongs to.
    pub repo_ref: String,
    /// Redaction-safe worktree ref the session is rooted in.
    pub worktree_ref: String,
    /// Lifecycle state token from the closed vocabulary for `session_kind`.
    pub lifecycle_state: String,
    /// Target revision refs (base/ours/theirs, replay target, ref positions).
    pub target_refs: Vec<String>,
    /// Redaction-safe path-scope tokens; never raw paths.
    pub path_scope_tokens: Vec<String>,
    /// Count of unresolved conflict rows or blockers.
    pub unresolved_count: u32,
    /// Checkpoint lineage refs protecting this session's mutations.
    pub checkpoint_lineage_refs: Vec<String>,
    /// Ref to the exact raw todo/patch text, when the workflow preserves it.
    pub raw_source_text_ref: Option<String>,
    /// Ref to the structured cards derived from the same source text.
    pub structured_cards_ref: Option<String>,
    /// Distinct action verbs bound to this session, from
    /// [`HISTORY_SESSION_ACTIONS`].
    pub available_actions: Vec<String>,
    /// Conflict resolution mode, for conflict sessions.
    pub resolution_mode: Option<String>,
    /// Divergence class, for publish/ref-update proposals.
    pub divergence_class: Option<String>,
    /// Approval state, for publish/ref-update proposals.
    pub approval_state: Option<String>,
    /// Check-invalidation state, for publish/ref-update proposals.
    pub check_invalidation_state: Option<String>,
    /// Publish mode, for publish/ref-update proposals.
    pub publish_mode: Option<String>,
    /// Affected approval refs preserved before a network mutation.
    pub affected_approval_refs: Vec<String>,
    /// Affected check refs preserved before a network mutation.
    pub affected_check_refs: Vec<String>,
    /// Checkpoint trigger kind, for recovery checkpoints.
    pub trigger_kind: Option<String>,
    /// Restore option classes, for recovery checkpoints.
    pub restore_option_classes: Vec<String>,
    /// True when only a reflog-only fallback is available (no checkpoint).
    pub reflog_only_fallback: bool,
    /// Created timestamp (RFC 3339).
    pub created_at: String,
    /// Updated timestamp (RFC 3339).
    pub updated_at: String,
    /// Redaction-safe summary label.
    pub summary_label: String,
}

impl HistorySession {
    /// Whether the session retains exact repository and worktree identity.
    pub fn identity_preserved(&self) -> bool {
        !self.repo_ref.trim().is_empty() && !self.worktree_ref.trim().is_empty()
    }

    /// Whether the session can be reopened after a restart from durable truth.
    pub fn reopen_safe(&self) -> bool {
        !self.session_id.trim().is_empty()
            && !self.created_at.trim().is_empty()
            && !self.updated_at.trim().is_empty()
    }

    /// Whether a reachable recovery path is visible for this session.
    pub fn recovery_visible(&self) -> bool {
        self.session_kind == HistorySurgerySession::RecoveryCheckpoint
            || !self.checkpoint_lineage_refs.is_empty()
            || self.reflog_only_fallback
    }

    /// Whether a publish proposal has cleared every precondition for a network
    /// mutation: divergence known, approvals and checks not invalidated, recovery
    /// lineage present, and the proposal explicitly ready to publish.
    pub fn publish_preconditions_met(&self) -> bool {
        if self.session_kind != HistorySurgerySession::PublishRefUpdateProposal {
            return false;
        }
        self.lifecycle_state == "ready_to_publish"
            && self.divergence_class.as_deref() != Some("unknown_requires_refresh")
            && self.approval_state.as_deref() != Some("approval_invalidated_by_changes")
            && self.check_invalidation_state.as_deref() != Some("checks_invalidated_blocks_publish")
            && !self.checkpoint_lineage_refs.is_empty()
    }

    /// Projects this descriptor onto one consumer surface, producing the binding
    /// that surface renders. The projection is deterministic, so a stored binding
    /// can be re-derived and verified against its descriptor.
    pub fn project(
        &self,
        surface: SessionConsumerSurface,
        binding_id: impl Into<String>,
    ) -> SessionConsumerBinding {
        let actionable_verbs = if surface.is_mutation_surface() {
            self.available_actions.clone()
        } else {
            Vec::new()
        };

        let network_mutation_allowed =
            surface.is_mutation_surface() && self.publish_preconditions_met();

        SessionConsumerBinding {
            record_kind: HISTORY_SESSION_CONSUMER_BINDING_RECORD_KIND.to_owned(),
            binding_id: binding_id.into(),
            surface,
            session_ref: self.session_id.clone(),
            session_kind: self.session_kind,
            repo_ref: self.repo_ref.clone(),
            worktree_ref: self.worktree_ref.clone(),
            lifecycle_state: self.lifecycle_state.clone(),
            identity_preserved: self.identity_preserved(),
            reopen_safe: self.reopen_safe(),
            disclosed_verbs: self.available_actions.clone(),
            actionable_verbs,
            network_mutation_allowed,
            recovery_visible: self.recovery_visible(),
            raw_body_export_allowed: surface.allows_body_export()
                && self.raw_source_text_ref.is_some(),
        }
    }
}

/// One consumer-surface binding derived from a [`HistorySession`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionConsumerBinding {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable binding id.
    pub binding_id: String,
    /// Surface that renders this binding.
    pub surface: SessionConsumerSurface,
    /// Referenced [`HistorySession::session_id`].
    pub session_ref: String,
    /// Session kind carried for surface routing.
    pub session_kind: HistorySurgerySession,
    /// Repository ref carried so identity survives every projection.
    pub repo_ref: String,
    /// Worktree ref carried so identity survives every projection.
    pub worktree_ref: String,
    /// Lifecycle state carried onto the surface.
    pub lifecycle_state: String,
    /// True when exact repo/worktree identity is preserved on this surface.
    pub identity_preserved: bool,
    /// True when the session can be reopened from durable truth.
    pub reopen_safe: bool,
    /// Verbs disclosed to the user, even on read-only surfaces.
    pub disclosed_verbs: Vec<String>,
    /// Verbs this surface may actually invoke (empty on read-only surfaces).
    pub actionable_verbs: Vec<String>,
    /// True only when this surface may run the publish network mutation now.
    pub network_mutation_allowed: bool,
    /// True when a reachable recovery path is visible on this surface.
    pub recovery_visible: bool,
    /// True when the surface may hydrate the raw todo/patch body.
    pub raw_body_export_allowed: bool,
}

/// Redaction-safe support-export projection for a history-session map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySessionSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Session ids included in the export.
    pub session_refs: Vec<String>,
    /// Binding ids included in the export.
    pub binding_refs: Vec<String>,
    /// Identity fields retained after redaction.
    pub reconstruction_fields: Vec<String>,
    /// True when no raw paths are embedded.
    pub raw_paths_redacted: bool,
    /// True when no raw patch/todo bodies are embedded.
    pub raw_patch_bodies_redacted: bool,
    /// True when no raw provider payloads are embedded.
    pub raw_provider_payloads_redacted: bool,
}

/// Top-level canonical map binding durable history sessions to first consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySessionConsumerMap {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable map id.
    pub map_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Repository ref every session in this map belongs to.
    pub repo_ref: String,
    /// Primary worktree ref for the map.
    pub worktree_ref: String,
    /// Durable history-surgery session descriptors.
    pub sessions: Vec<HistorySession>,
    /// Per-surface bindings derived from the descriptors.
    pub consumer_bindings: Vec<SessionConsumerBinding>,
    /// Redaction-safe support-export projection.
    pub support_export: HistorySessionSupportExport,
}

impl HistorySessionConsumerMap {
    /// Parses a map from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`HistorySessionError`] when the JSON is invalid or the parsed map
    /// violates the history-session contract.
    pub fn parse_json(input: &str) -> Result<Self, HistorySessionError> {
        let map: Self = serde_json::from_str(input).map_err(HistorySessionError::Json)?;
        let violations = map.validate();
        if violations.is_empty() {
            Ok(map)
        } else {
            Err(HistorySessionError::Validation(violations))
        }
    }

    /// Validates every descriptor, binding, and support-export invariant.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<HistorySessionValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != HISTORY_SESSION_MAP_RECORD_KIND {
            errors.push(HistorySessionValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != HISTORY_SESSION_SCHEMA_VERSION {
            errors.push(HistorySessionValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.map_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.repo_ref.trim().is_empty()
            || self.worktree_ref.trim().is_empty()
        {
            errors.push(HistorySessionValidationError::MissingIdentity);
        }
        if self.sessions.is_empty() {
            errors.push(HistorySessionValidationError::NoSessions);
        }

        let mut session_ids: HashSet<&str> = HashSet::new();
        for session in &self.sessions {
            if !session_ids.insert(session.session_id.as_str()) {
                errors.push(HistorySessionValidationError::DuplicateSessionId {
                    session_id: session.session_id.clone(),
                });
            }
            validate_session(session, &self.repo_ref, &mut errors);
        }

        let mut binding_ids: HashSet<&str> = HashSet::new();
        for binding in &self.consumer_bindings {
            if !binding_ids.insert(binding.binding_id.as_str()) {
                errors.push(HistorySessionValidationError::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            validate_binding(binding, &self.sessions, &mut errors);
        }

        validate_support_export(self, &session_ids, &binding_ids, &mut errors);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("history session map serializes"),
        ) {
            errors.push(HistorySessionValidationError::RawBoundaryMaterialInExport);
        }

        errors
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only map fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("history session map serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# History-Surgery Sessions and First Consumers\n\n");
        out.push_str(&format!("- Map: `{}`\n", self.map_id));
        out.push_str(&format!("- Repository: `{}`\n", self.repo_ref));
        out.push_str(&format!(
            "- Sessions: {} / Consumer bindings: {}\n",
            self.sessions.len(),
            self.consumer_bindings.len()
        ));

        out.push_str("\n## Sessions\n\n");
        for session in &self.sessions {
            out.push_str(&format!(
                "- **{}** (`{}`): state `{}`, unresolved {}, actions [{}]\n",
                session.session_kind.as_str(),
                session.session_id,
                session.lifecycle_state,
                session.unresolved_count,
                session.available_actions.join(", "),
            ));
        }

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** → `{}`: actionable [{}], network_mutation {}, recovery_visible {}\n",
                binding.surface.as_str(),
                binding.session_ref,
                binding.actionable_verbs.join(", "),
                binding.network_mutation_allowed,
                binding.recovery_visible,
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical first-consumers map.
///
/// # Errors
///
/// Returns [`HistorySessionError`] when the checked-in map fails to parse or
/// violates the history-session contract.
pub fn current_history_session_first_consumers_map(
) -> Result<HistorySessionConsumerMap, HistorySessionError> {
    HistorySessionConsumerMap::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/history_sessions/history_session_first_consumers.json"
    )))
}

/// Closed lifecycle vocabulary for one session kind.
fn lifecycle_vocab(kind: HistorySurgerySession) -> &'static [&'static str] {
    match kind {
        HistorySurgerySession::ConflictSession => RISKY_VCS_CONFLICT_STATES,
        HistorySurgerySession::SequenceEditSession => RISKY_VCS_SEQUENCE_STATES,
        HistorySurgerySession::StashShelfEntry => RISKY_VCS_STASH_STATES,
        HistorySurgerySession::PublishRefUpdateProposal => RISKY_VCS_REF_UPDATE_STATES,
        HistorySurgerySession::RecoveryCheckpoint => RISKY_VCS_CHECKPOINT_STATES,
    }
}

/// Action verbs that must be bound to one session kind for it to be actionable.
fn required_actions(kind: HistorySurgerySession) -> &'static [&'static str] {
    match kind {
        HistorySurgerySession::ConflictSession => &["continue", "abort"],
        HistorySurgerySession::SequenceEditSession => &["continue", "abort"],
        HistorySurgerySession::StashShelfEntry => &HISTORY_SESSION_STASH_VERBS,
        HistorySurgerySession::PublishRefUpdateProposal => &["publish"],
        HistorySurgerySession::RecoveryCheckpoint => &["restore"],
    }
}

fn validate_session(
    session: &HistorySession,
    map_repo_ref: &str,
    errors: &mut Vec<HistorySessionValidationError>,
) {
    let session_id = session.session_id.clone();

    if session.record_kind != HISTORY_SESSION_DESCRIPTOR_RECORD_KIND {
        errors.push(HistorySessionValidationError::WrongRecordKind {
            observed: session.record_kind.clone(),
        });
    }
    if session.canonical_record_kind != session.session_kind.canonical_record_kind() {
        errors.push(HistorySessionValidationError::CanonicalRecordKindMismatch {
            session_id: session_id.clone(),
        });
    }
    if session.session_id.trim().is_empty()
        || session.created_at.trim().is_empty()
        || session.updated_at.trim().is_empty()
        || session.summary_label.trim().is_empty()
    {
        errors.push(HistorySessionValidationError::SessionMissingIdentity {
            session_id: session_id.clone(),
        });
    }
    if !session.identity_preserved() || session.repo_ref != map_repo_ref {
        errors.push(HistorySessionValidationError::SessionIdentityNotPreserved {
            session_id: session_id.clone(),
        });
    }

    if !lifecycle_vocab(session.session_kind).contains(&session.lifecycle_state.as_str()) {
        errors.push(HistorySessionValidationError::LifecycleOutOfVocabulary {
            session_id: session_id.clone(),
        });
    }

    validate_actions(session, &session_id, errors);
    validate_recovery_lineage(session, &session_id, errors);
    validate_kind_specific(session, &session_id, errors);
}

fn validate_actions(
    session: &HistorySession,
    session_id: &str,
    errors: &mut Vec<HistorySessionValidationError>,
) {
    let mut seen: HashSet<&str> = HashSet::new();
    for action in &session.available_actions {
        if !HISTORY_SESSION_ACTIONS.contains(&action.as_str()) {
            errors.push(HistorySessionValidationError::ActionOutOfVocabulary {
                session_id: session_id.to_owned(),
                action: action.clone(),
            });
        }
        if !seen.insert(action.as_str()) {
            errors.push(HistorySessionValidationError::DuplicateAction {
                session_id: session_id.to_owned(),
                action: action.clone(),
            });
        }
    }
    for required in required_actions(session.session_kind) {
        if !seen.contains(required) {
            errors.push(HistorySessionValidationError::MissingRequiredAction {
                session_id: session_id.to_owned(),
                action: (*required).to_owned(),
            });
        }
    }
}

fn validate_recovery_lineage(
    session: &HistorySession,
    session_id: &str,
    errors: &mut Vec<HistorySessionValidationError>,
) {
    // Every object that drives a mutation must keep a reachable recovery path
    // before it runs: an explicit checkpoint, or an acknowledged reflog-only
    // fallback. The recovery checkpoint is itself the recovery surface.
    let mutating = !matches!(
        session.session_kind,
        HistorySurgerySession::RecoveryCheckpoint
    );
    if mutating && session.checkpoint_lineage_refs.is_empty() && !session.reflog_only_fallback {
        errors.push(HistorySessionValidationError::MutationMissingRecovery {
            session_id: session_id.to_owned(),
        });
    }
}

fn validate_kind_specific(
    session: &HistorySession,
    session_id: &str,
    errors: &mut Vec<HistorySessionValidationError>,
) {
    match session.session_kind {
        HistorySurgerySession::ConflictSession => {
            require_source_text(session, session_id, errors);
            require_in_vocab(
                session.resolution_mode.as_deref(),
                RISKY_VCS_CONFLICT_RESOLUTION_MODES,
                session_id,
                "resolution_mode",
                errors,
            );
            if session.target_refs.len() < 3 {
                // base / ours / theirs provenance must survive reopen.
                errors.push(
                    HistorySessionValidationError::ConflictProvenanceIncomplete {
                        session_id: session_id.to_owned(),
                    },
                );
            }
            if matches!(
                session.lifecycle_state.as_str(),
                "completed_committed" | "completed_handed_off"
            ) && session.unresolved_count != 0
            {
                errors.push(HistorySessionValidationError::CompletedConflictUnresolved {
                    session_id: session_id.to_owned(),
                });
            }
        }
        HistorySurgerySession::SequenceEditSession => {
            require_source_text(session, session_id, errors);
            if session.target_refs.is_empty() {
                errors.push(HistorySessionValidationError::SequenceMissingTarget {
                    session_id: session_id.to_owned(),
                });
            }
        }
        HistorySurgerySession::StashShelfEntry => {
            if session.path_scope_tokens.is_empty() {
                errors.push(HistorySessionValidationError::StashScopeMissing {
                    session_id: session_id.to_owned(),
                });
            }
            for verb in HISTORY_SESSION_STASH_VERBS {
                if !session
                    .available_actions
                    .iter()
                    .any(|action| action == verb)
                {
                    errors.push(HistorySessionValidationError::StashVerbsNotDistinct {
                        session_id: session_id.to_owned(),
                    });
                    break;
                }
            }
        }
        HistorySurgerySession::PublishRefUpdateProposal => {
            require_in_vocab(
                session.divergence_class.as_deref(),
                RISKY_VCS_DIVERGENCE_CLASSES,
                session_id,
                "divergence_class",
                errors,
            );
            require_in_vocab(
                session.approval_state.as_deref(),
                RISKY_VCS_APPROVAL_STATES,
                session_id,
                "approval_state",
                errors,
            );
            require_in_vocab(
                session.check_invalidation_state.as_deref(),
                RISKY_VCS_CHECK_INVALIDATION_STATES,
                session_id,
                "check_invalidation_state",
                errors,
            );
            require_in_vocab(
                session.publish_mode.as_deref(),
                RISKY_VCS_PUBLISH_MODES,
                session_id,
                "publish_mode",
                errors,
            );
            if session.target_refs.is_empty() {
                errors.push(HistorySessionValidationError::PublishMissingTarget {
                    session_id: session_id.to_owned(),
                });
            }
            // A proposal that claims to be ready must not hide invalidated
            // approvals/checks, unknown divergence, or absent recovery lineage.
            if session.lifecycle_state == "ready_to_publish" && !session.publish_preconditions_met()
            {
                errors.push(
                    HistorySessionValidationError::PublishReadyWithoutPreconditions {
                        session_id: session_id.to_owned(),
                    },
                );
            }
        }
        HistorySurgerySession::RecoveryCheckpoint => {
            require_in_vocab(
                session.trigger_kind.as_deref(),
                RISKY_VCS_CHECKPOINT_TRIGGER_KINDS,
                session_id,
                "trigger_kind",
                errors,
            );
            if session.restore_option_classes.is_empty() {
                errors.push(
                    HistorySessionValidationError::RecoveryMissingRestoreOptions {
                        session_id: session_id.to_owned(),
                    },
                );
            }
            for option in &session.restore_option_classes {
                if !RISKY_VCS_RESTORE_OPTION_CLASSES.contains(&option.as_str()) {
                    errors.push(
                        HistorySessionValidationError::RecoveryMissingRestoreOptions {
                            session_id: session_id.to_owned(),
                        },
                    );
                    break;
                }
            }
        }
    }
}

fn require_source_text(
    session: &HistorySession,
    session_id: &str,
    errors: &mut Vec<HistorySessionValidationError>,
) {
    let raw_ok = session
        .raw_source_text_ref
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty());
    let cards_ok = session
        .structured_cards_ref
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty());
    if !raw_ok || !cards_ok {
        errors.push(HistorySessionValidationError::SourceTextNotPreserved {
            session_id: session_id.to_owned(),
        });
    }
}

fn require_in_vocab(
    value: Option<&str>,
    vocab: &[&str],
    session_id: &str,
    field: &str,
    errors: &mut Vec<HistorySessionValidationError>,
) {
    match value {
        Some(token) if vocab.contains(&token) => {}
        _ => errors.push(HistorySessionValidationError::FieldOutOfVocabulary {
            session_id: session_id.to_owned(),
            field: field.to_owned(),
        }),
    }
}

fn validate_binding(
    binding: &SessionConsumerBinding,
    sessions: &[HistorySession],
    errors: &mut Vec<HistorySessionValidationError>,
) {
    if binding.record_kind != HISTORY_SESSION_CONSUMER_BINDING_RECORD_KIND {
        errors.push(HistorySessionValidationError::WrongRecordKind {
            observed: binding.record_kind.clone(),
        });
    }
    let Some(session) = sessions
        .iter()
        .find(|session| session.session_id == binding.session_ref)
    else {
        errors.push(HistorySessionValidationError::UnknownBindingSession {
            binding_id: binding.binding_id.clone(),
            session_ref: binding.session_ref.clone(),
        });
        return;
    };

    // The binding must equal the deterministic projection of its descriptor;
    // this is what proves the same object drives every surface.
    let expected = session.project(binding.surface, binding.binding_id.clone());
    if &expected != binding {
        errors.push(
            HistorySessionValidationError::BindingDoesNotMatchDescriptor {
                binding_id: binding.binding_id.clone(),
            },
        );
    }

    // Guardrail: a read-only surface never marks a verb actionable and never
    // performs a network mutation.
    if !binding.surface.is_mutation_surface()
        && (!binding.actionable_verbs.is_empty() || binding.network_mutation_allowed)
    {
        errors.push(HistorySessionValidationError::ReadOnlySurfaceActionable {
            binding_id: binding.binding_id.clone(),
        });
    }

    // Guardrail: a network mutation is allowed only once publish preconditions
    // are met (divergence/approvals/checks/recovery), never silently.
    if binding.network_mutation_allowed && !session.publish_preconditions_met() {
        errors.push(
            HistorySessionValidationError::NetworkMutationWithoutPreconditions {
                binding_id: binding.binding_id.clone(),
            },
        );
    }

    // Guardrail: identity must survive every projection.
    if !binding.identity_preserved {
        errors.push(HistorySessionValidationError::BindingIdentityNotPreserved {
            binding_id: binding.binding_id.clone(),
        });
    }

    // Guardrail: support export and provider overlay never hydrate raw bodies.
    if !binding.surface.allows_body_export() && binding.raw_body_export_allowed {
        errors.push(HistorySessionValidationError::SupportExportEmbedsRawBody {
            binding_id: binding.binding_id.clone(),
        });
    }
}

fn validate_support_export(
    map: &HistorySessionConsumerMap,
    session_ids: &HashSet<&str>,
    binding_ids: &HashSet<&str>,
    errors: &mut Vec<HistorySessionValidationError>,
) {
    let export = &map.support_export;
    if export.record_kind != HISTORY_SESSION_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(HistorySessionValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for session_ref in &export.session_refs {
        if !session_ids.contains(session_ref.as_str()) {
            errors.push(HistorySessionValidationError::UnknownSupportSessionRef {
                session_ref: session_ref.clone(),
            });
        }
    }
    // Every durable session must be reconstructable from the support export.
    for session in &map.sessions {
        if !export
            .session_refs
            .iter()
            .any(|reference| reference == &session.session_id)
        {
            errors.push(HistorySessionValidationError::SupportExportMissingSession {
                session_id: session.session_id.clone(),
            });
        }
    }
    for binding_ref in &export.binding_refs {
        if !binding_ids.contains(binding_ref.as_str()) {
            errors.push(HistorySessionValidationError::UnknownSupportBindingRef {
                binding_ref: binding_ref.clone(),
            });
        }
    }
    for required in HISTORY_SESSION_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(HistorySessionValidationError::SupportExportMissingField {
                field: required.to_string(),
            });
        }
    }
    if !export.raw_paths_redacted
        || !export.raw_patch_bodies_redacted
        || !export.raw_provider_payloads_redacted
    {
        errors.push(HistorySessionValidationError::SupportExportEmbedsRawMaterial);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Error returned while parsing a history-session map.
#[derive(Debug)]
pub enum HistorySessionError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<HistorySessionValidationError>),
}

impl fmt::Display for HistorySessionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse history session map JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(formatter, "history session map has validation errors: ")?;
                for (index, error) in errors.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, "; ")?;
                    }
                    write!(formatter, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for HistorySessionError {}

/// Cross-row validation error for a history-session map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistorySessionValidationError {
    /// A record-kind tag does not match the stable contract.
    WrongRecordKind {
        /// Observed record-kind tag.
        observed: String,
    },
    /// The map schema version is unsupported.
    WrongSchemaVersion {
        /// Observed schema version.
        observed: u32,
    },
    /// A required map identity field is missing.
    MissingIdentity,
    /// The map carries no sessions.
    NoSessions,
    /// A session id is declared more than once.
    DuplicateSessionId {
        /// Duplicated session id.
        session_id: String,
    },
    /// A binding id is declared more than once.
    DuplicateBindingId {
        /// Duplicated binding id.
        binding_id: String,
    },
    /// A session's canonical record kind does not match its kind.
    CanonicalRecordKindMismatch {
        /// Session id.
        session_id: String,
    },
    /// A session is missing a required identity field.
    SessionMissingIdentity {
        /// Session id.
        session_id: String,
    },
    /// A session does not preserve exact repo/worktree identity.
    SessionIdentityNotPreserved {
        /// Session id.
        session_id: String,
    },
    /// A session lifecycle state is outside its closed vocabulary.
    LifecycleOutOfVocabulary {
        /// Session id.
        session_id: String,
    },
    /// A bound action verb is outside the closed vocabulary.
    ActionOutOfVocabulary {
        /// Session id.
        session_id: String,
        /// Offending action.
        action: String,
    },
    /// An action verb is bound more than once.
    DuplicateAction {
        /// Session id.
        session_id: String,
        /// Duplicated action.
        action: String,
    },
    /// A session is missing an action its kind requires.
    MissingRequiredAction {
        /// Session id.
        session_id: String,
        /// Missing action.
        action: String,
    },
    /// A mutating session keeps no reachable recovery path.
    MutationMissingRecovery {
        /// Session id.
        session_id: String,
    },
    /// A conflict/sequence session does not preserve raw + structured source text.
    SourceTextNotPreserved {
        /// Session id.
        session_id: String,
    },
    /// A conflict session does not preserve base/ours/theirs provenance.
    ConflictProvenanceIncomplete {
        /// Session id.
        session_id: String,
    },
    /// A completed conflict session still reports unresolved rows.
    CompletedConflictUnresolved {
        /// Session id.
        session_id: String,
    },
    /// A sequence-edit session names no replay target.
    SequenceMissingTarget {
        /// Session id.
        session_id: String,
    },
    /// A stash entry records no path scope.
    StashScopeMissing {
        /// Session id.
        session_id: String,
    },
    /// A stash entry does not keep apply/pop/drop/create-branch distinct.
    StashVerbsNotDistinct {
        /// Session id.
        session_id: String,
    },
    /// A publish proposal names no target ref.
    PublishMissingTarget {
        /// Session id.
        session_id: String,
    },
    /// A publish proposal claims ready without meeting its preconditions.
    PublishReadyWithoutPreconditions {
        /// Session id.
        session_id: String,
    },
    /// A recovery checkpoint lists no restore options.
    RecoveryMissingRestoreOptions {
        /// Session id.
        session_id: String,
    },
    /// A kind-specific field value is outside its closed vocabulary.
    FieldOutOfVocabulary {
        /// Session id.
        session_id: String,
        /// Field name.
        field: String,
    },
    /// A binding references an unknown session.
    UnknownBindingSession {
        /// Binding id.
        binding_id: String,
        /// Unknown session ref.
        session_ref: String,
    },
    /// A binding does not equal the projection of its descriptor.
    BindingDoesNotMatchDescriptor {
        /// Binding id.
        binding_id: String,
    },
    /// A read-only surface marks a verb actionable or runs a network mutation.
    ReadOnlySurfaceActionable {
        /// Binding id.
        binding_id: String,
    },
    /// A binding permits a network mutation without meeting publish preconditions.
    NetworkMutationWithoutPreconditions {
        /// Binding id.
        binding_id: String,
    },
    /// A binding drops exact repo/worktree identity.
    BindingIdentityNotPreserved {
        /// Binding id.
        binding_id: String,
    },
    /// A support-export/provider binding hydrates a raw body.
    SupportExportEmbedsRawBody {
        /// Binding id.
        binding_id: String,
    },
    /// A support-export session ref is unknown.
    UnknownSupportSessionRef {
        /// Unknown session ref.
        session_ref: String,
    },
    /// A support-export binding ref is unknown.
    UnknownSupportBindingRef {
        /// Unknown binding ref.
        binding_ref: String,
    },
    /// A durable session is missing from the support export lineage.
    SupportExportMissingSession {
        /// Session id.
        session_id: String,
    },
    /// The support export omits a required reconstruction field.
    SupportExportMissingField {
        /// Missing reconstruction field.
        field: String,
    },
    /// The support export embeds raw paths, bodies, or provider payloads.
    SupportExportEmbedsRawMaterial,
    /// The export contains obviously forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl fmt::Display for HistorySessionValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { observed } => {
                write!(formatter, "unexpected record kind {observed}")
            }
            Self::WrongSchemaVersion { observed } => {
                write!(formatter, "unsupported schema version {observed}")
            }
            Self::MissingIdentity => write!(formatter, "map is missing identity fields"),
            Self::NoSessions => write!(formatter, "map carries no sessions"),
            Self::DuplicateSessionId { session_id } => {
                write!(
                    formatter,
                    "session id {session_id} is declared more than once"
                )
            }
            Self::DuplicateBindingId { binding_id } => {
                write!(
                    formatter,
                    "binding id {binding_id} is declared more than once"
                )
            }
            Self::CanonicalRecordKindMismatch { session_id } => {
                write!(
                    formatter,
                    "session {session_id} canonical record kind mismatch"
                )
            }
            Self::SessionMissingIdentity { session_id } => {
                write!(formatter, "session {session_id} is missing identity fields")
            }
            Self::SessionIdentityNotPreserved { session_id } => {
                write!(
                    formatter,
                    "session {session_id} does not preserve repo/worktree identity"
                )
            }
            Self::LifecycleOutOfVocabulary { session_id } => {
                write!(
                    formatter,
                    "session {session_id} lifecycle state is out of vocabulary"
                )
            }
            Self::ActionOutOfVocabulary { session_id, action } => {
                write!(
                    formatter,
                    "session {session_id} action {action} is out of vocabulary"
                )
            }
            Self::DuplicateAction { session_id, action } => {
                write!(
                    formatter,
                    "session {session_id} action {action} is duplicated"
                )
            }
            Self::MissingRequiredAction { session_id, action } => {
                write!(
                    formatter,
                    "session {session_id} is missing required action {action}"
                )
            }
            Self::MutationMissingRecovery { session_id } => {
                write!(
                    formatter,
                    "mutating session {session_id} keeps no recovery path"
                )
            }
            Self::SourceTextNotPreserved { session_id } => {
                write!(
                    formatter,
                    "session {session_id} does not preserve raw + structured source text"
                )
            }
            Self::ConflictProvenanceIncomplete { session_id } => {
                write!(
                    formatter,
                    "conflict session {session_id} lacks base/ours/theirs provenance"
                )
            }
            Self::CompletedConflictUnresolved { session_id } => {
                write!(
                    formatter,
                    "completed conflict session {session_id} still has unresolved rows"
                )
            }
            Self::SequenceMissingTarget { session_id } => {
                write!(
                    formatter,
                    "sequence-edit session {session_id} names no replay target"
                )
            }
            Self::StashScopeMissing { session_id } => {
                write!(formatter, "stash entry {session_id} records no path scope")
            }
            Self::StashVerbsNotDistinct { session_id } => {
                write!(
                    formatter,
                    "stash entry {session_id} does not keep apply/pop/drop/create-branch distinct"
                )
            }
            Self::PublishMissingTarget { session_id } => {
                write!(
                    formatter,
                    "publish proposal {session_id} names no target ref"
                )
            }
            Self::PublishReadyWithoutPreconditions { session_id } => {
                write!(
                    formatter,
                    "publish proposal {session_id} is ready without meeting preconditions"
                )
            }
            Self::RecoveryMissingRestoreOptions { session_id } => {
                write!(
                    formatter,
                    "recovery checkpoint {session_id} lists no valid restore options"
                )
            }
            Self::FieldOutOfVocabulary { session_id, field } => {
                write!(
                    formatter,
                    "session {session_id} field {field} is out of vocabulary"
                )
            }
            Self::UnknownBindingSession {
                binding_id,
                session_ref,
            } => write!(
                formatter,
                "binding {binding_id} references unknown session {session_ref}"
            ),
            Self::BindingDoesNotMatchDescriptor { binding_id } => write!(
                formatter,
                "binding {binding_id} does not match its descriptor projection"
            ),
            Self::ReadOnlySurfaceActionable { binding_id } => {
                write!(
                    formatter,
                    "read-only binding {binding_id} marks a verb actionable"
                )
            }
            Self::NetworkMutationWithoutPreconditions { binding_id } => write!(
                formatter,
                "binding {binding_id} permits a network mutation without preconditions"
            ),
            Self::BindingIdentityNotPreserved { binding_id } => {
                write!(
                    formatter,
                    "binding {binding_id} drops repo/worktree identity"
                )
            }
            Self::SupportExportEmbedsRawBody { binding_id } => {
                write!(
                    formatter,
                    "support/provider binding {binding_id} hydrates a raw body"
                )
            }
            Self::UnknownSupportSessionRef { session_ref } => {
                write!(
                    formatter,
                    "support export references unknown session {session_ref}"
                )
            }
            Self::UnknownSupportBindingRef { binding_ref } => {
                write!(
                    formatter,
                    "support export references unknown binding {binding_ref}"
                )
            }
            Self::SupportExportMissingSession { session_id } => {
                write!(formatter, "support export omits session {session_id}")
            }
            Self::SupportExportMissingField { field } => {
                write!(
                    formatter,
                    "support export missing reconstruction field {field}"
                )
            }
            Self::SupportExportEmbedsRawMaterial => {
                write!(
                    formatter,
                    "support export embeds raw paths, bodies, or provider payloads"
                )
            }
            Self::RawBoundaryMaterialInExport => {
                write!(formatter, "export contains forbidden boundary material")
            }
        }
    }
}
