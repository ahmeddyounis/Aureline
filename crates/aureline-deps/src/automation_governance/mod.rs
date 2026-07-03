//! AI, recipe, and CLI/headless package-mutation governance with
//! manifest/lockfile impact previews, validation-task selection, and
//! no-unsafe-fallback execution across claimed ecosystems.
//!
//! Where [`crate::reviewed_mutation_flows`] owns the canonical, preview-first
//! mutation review sheet a single operation renders, this module binds package
//! mutation into the command/automation/AI governance model: every AI-generated
//! proposal, recipe step, and CLI/headless invocation reuses that same reviewed
//! contract and **cannot become a bypass lane**. One [`GovernedMutationProposal`]
//! is the object the AI composer, a recipe/automation step, the CLI dry run, and
//! the desktop baseline all produce, so convenience never outruns lockfile-safe
//! review.
//!
//! Each proposal makes three governance facts explicit before anything executes:
//!
//! - **Manifest/lockfile impact preview** — a [`ReviewedSheetBinding`] carries
//!   the same manifest scope, script/native-build label, resolver identity,
//!   lockfile diff class, and registry/auth posture the direct review sheet
//!   would, bound back to the canonical sheet by `sheet_ref`.
//! - **Selected validation tasks** — a [`ValidationTaskSelection`] names the
//!   build/test/lint/typecheck/audit/license/lockfile checks chosen for the
//!   proposal; a proposal that proceeds may not leave a required check
//!   unselected.
//! - **No unsafe fallback** — an [`EcosystemCapabilityPosture`] records whether
//!   the claimed ecosystem can actually deliver the promised review preview,
//!   deterministic resolver, durable rollback, and validation execution. When
//!   any promised capability is missing, the [`ExecutionDecision`] **must**
//!   narrow to inspect-only, export, or a browser/CLI handoff — it can never
//!   silently proceed.
//!
//! A [`ParityAttestation`] proves the governed proposal preserves the same
//! review, script/native-build disclosure, registry/auth posture, validation
//! selection, and rollback packet as a direct UI-driven operation, never widens
//! scope silently, and never turns package mutation into hidden scripting. The
//! commit gate refuses a `CommittedReviewed` result while any intrinsic safety
//! blocker (script/native-build block, unsatisfied auth, divergent lockfile,
//! unconfirmed whole-workspace scope, or a capability gap) holds, while a
//! required validation task stays unselected, while parity is broken, or while
//! the rollback handle is not a durable recovery path. The result packet and
//! rollback handle a [`GovernedMutationProposal`] surfaces are identical across
//! the desktop, CLI, AI, and support/export surfaces; only the per-surface write
//! authority differs.
//!
//! The packet reuses the frozen
//! [`crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix`]
//! vocabulary and the [`crate::reviewed_mutation_flows`] flow/script/lockfile
//! vocabulary, and binds every label it surfaces back to a frozen state row
//! through `references_matrix_id`.
//!
//! The checked-in packet lives at `artifacts/deps/m5/automation-governance.json`
//! and is embedded here so Rust consumers, CLI/headless output, support exports,
//! and release evidence all validate against one source of truth. The model is
//! metadata-only: it carries no credential bodies, registry tokens, raw provider
//! payloads, or private registry URLs.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::{
    current_m5_package_state_matrix, AuthMode, LockfileAuthority, ManifestScopeClass,
    PackageStateLabel, PackageSurface, RegistrySourceAuthority, ResolverIdentityClass, RollbackClass,
    SurfaceWriteAuthority,
};
use crate::package_state_descriptors::EcosystemKind;
use crate::reviewed_mutation_flows::{
    LockfileDiffClass, ManifestDiffApplyAction, ManifestDiffCard,
    ManifestDiffCheckpointState, ManifestDiffPreviewState, ManifestDiffRollbackState,
    MutationFlowClass, ProposalSource, RecoveryActionKind, ScriptBuildLabel,
};

/// Supported automation-governance packet schema version.
pub const AUTOMATION_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const AUTOMATION_GOVERNANCE_RECORD_KIND: &str = "automation_governance";

/// Repo-relative path to the checked-in packet.
pub const AUTOMATION_GOVERNANCE_PATH: &str = "artifacts/deps/m5/automation-governance.json";

/// Embedded checked-in packet JSON.
pub const AUTOMATION_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/automation-governance.json"
));

/// A validation task the governance layer can require before a mutation commits.
///
/// The selection of which tasks run is a reviewed decision: an AI, recipe, or
/// CLI proposal cannot proceed while a required validation task stays
/// unselected, so automation never skips validation a direct operation would
/// run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationTaskKind {
    /// Compile / build the affected packages.
    Build,
    /// Run the affected test suites.
    Test,
    /// Run lint checks.
    Lint,
    /// Run type checks.
    Typecheck,
    /// Run a security/advisory audit over the resolved set.
    SecurityAudit,
    /// Run a license-review check over the resolved set.
    LicenseReview,
    /// Verify the lockfile is consistent and restorable.
    LockfileVerify,
}

impl ValidationTaskKind {
    /// Every validation-task kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Build,
        Self::Test,
        Self::Lint,
        Self::Typecheck,
        Self::SecurityAudit,
        Self::LicenseReview,
        Self::LockfileVerify,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Test => "test",
            Self::Lint => "lint",
            Self::Typecheck => "typecheck",
            Self::SecurityAudit => "security_audit",
            Self::LicenseReview => "license_review",
            Self::LockfileVerify => "lockfile_verify",
        }
    }
}

/// The governed execution decision for a proposal.
///
/// Exactly one of these is recorded. Only [`ExecutionDecision::ProceedAfterReview`]
/// allows a mutation to commit; every other variant is a **safe narrowing** that
/// keeps the proposal out of unsafe execution when an ecosystem or registry/auth
/// state cannot deliver the promised review, resolver, or rollback posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionDecision {
    /// All promised capabilities hold; the mutation may commit after review.
    ProceedAfterReview,
    /// Narrowed to inspect-only: no mutation, review/preview only.
    NarrowToInspectOnly,
    /// Narrowed to a redaction-safe export rather than execution.
    NarrowToExportOnly,
    /// Handed off to the provider's browser flow rather than executing in-product.
    HandoffToBrowser,
    /// Handed off to a CLI/headless flow rather than executing in-product.
    HandoffToCli,
    /// No safe execution path exists; the proposal is blocked.
    BlockedNoSafePath,
}

impl ExecutionDecision {
    /// Every execution decision, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProceedAfterReview,
        Self::NarrowToInspectOnly,
        Self::NarrowToExportOnly,
        Self::HandoffToBrowser,
        Self::HandoffToCli,
        Self::BlockedNoSafePath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedAfterReview => "proceed_after_review",
            Self::NarrowToInspectOnly => "narrow_to_inspect_only",
            Self::NarrowToExportOnly => "narrow_to_export_only",
            Self::HandoffToBrowser => "handoff_to_browser",
            Self::HandoffToCli => "handoff_to_cli",
            Self::BlockedNoSafePath => "blocked_no_safe_path",
        }
    }

    /// Whether this decision permits the mutation to execute and commit.
    pub const fn permits_execution(self) -> bool {
        matches!(self, Self::ProceedAfterReview)
    }

    /// Whether this decision is a safe narrowing away from execution.
    pub const fn is_safe_narrowing(self) -> bool {
        !self.permits_execution()
    }
}

/// The cross-surface-identical result class of a governed proposal.
///
/// This class is projected verbatim to the desktop, CLI, AI, and support
/// surfaces; only each surface's write authority differs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedResultClass {
    /// Preview produced; awaiting review.
    PreviewPending,
    /// Reviewed and ready to commit, not yet committed.
    ReviewedReady,
    /// Blocked: no safe execution path.
    BlockedUnsafe,
    /// Narrowed to inspect-only.
    NarrowedInspectOnly,
    /// Handed off to a browser or CLI flow.
    HandedOff,
    /// Committed after review.
    CommittedReviewed,
    /// Committed then rolled back from a durable checkpoint.
    RolledBack,
}

impl GovernedResultClass {
    /// Every result class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PreviewPending,
        Self::ReviewedReady,
        Self::BlockedUnsafe,
        Self::NarrowedInspectOnly,
        Self::HandedOff,
        Self::CommittedReviewed,
        Self::RolledBack,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewPending => "preview_pending",
            Self::ReviewedReady => "reviewed_ready",
            Self::BlockedUnsafe => "blocked_unsafe",
            Self::NarrowedInspectOnly => "narrowed_inspect_only",
            Self::HandedOff => "handed_off",
            Self::CommittedReviewed => "committed_reviewed",
            Self::RolledBack => "rolled_back",
        }
    }

    /// Whether this result represents a committed (post-write) state.
    pub const fn is_committed(self) -> bool {
        matches!(self, Self::CommittedReviewed | Self::RolledBack)
    }
}

/// Whether an execution decision and a result class are coherent.
///
/// A committed/rolled-back result requires a proceed decision; a narrowed result
/// requires the matching narrowing; a handed-off result requires a handoff or an
/// export narrowing; a blocked result requires the blocked decision.
const fn decision_allows_result(decision: ExecutionDecision, result: GovernedResultClass) -> bool {
    match result {
        GovernedResultClass::PreviewPending
        | GovernedResultClass::ReviewedReady
        | GovernedResultClass::CommittedReviewed
        | GovernedResultClass::RolledBack => {
            matches!(decision, ExecutionDecision::ProceedAfterReview)
        }
        GovernedResultClass::BlockedUnsafe => {
            matches!(decision, ExecutionDecision::BlockedNoSafePath)
        }
        GovernedResultClass::NarrowedInspectOnly => {
            matches!(decision, ExecutionDecision::NarrowToInspectOnly)
        }
        GovernedResultClass::HandedOff => matches!(
            decision,
            ExecutionDecision::HandoffToBrowser
                | ExecutionDecision::HandoffToCli
                | ExecutionDecision::NarrowToExportOnly
        ),
    }
}

/// Stable surface contract: the surfaces that share this object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutomationGovernanceSurfaceContract {
    /// AI-proposal surface.
    pub ai_proposal_surface: String,
    /// Recipe/automation surface.
    pub recipe_surface: String,
    /// CLI/headless surface.
    pub cli_headless_surface: String,
    /// Desktop baseline surface that automation must match.
    pub desktop_baseline_surface: String,
    /// Reviewed-sheet contract this packet reuses.
    pub reviewed_sheet_surface: String,
    /// Help page describing the packet.
    pub help_page: String,
    /// Support-export channel.
    pub support_export_surface: String,
}

/// Which surfaces a proposal's result is mirrored to identically.
///
/// The governed result packet and rollback handle must read identically across
/// desktop, the CLI/headless flow, the AI surface, and support/export; every
/// flag must stay `true`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceSurfaceParity {
    /// Reproduced on the desktop surface.
    pub desktop: bool,
    /// Reproduced by the CLI/headless flow.
    pub cli_headless: bool,
    /// Reproduced on the AI surface.
    pub ai_context: bool,
    /// Reproduced in support/export artifacts.
    pub support_export: bool,
}

impl GovernanceSurfaceParity {
    /// Whether the result is mirrored to every claimed surface.
    pub const fn is_consistent(&self) -> bool {
        self.desktop && self.cli_headless && self.ai_context && self.support_export
    }
}

/// The reviewed-sheet essentials a governed proposal binds to.
///
/// This is the manifest/lockfile impact preview the proposal reuses from the
/// canonical [`crate::reviewed_mutation_flows`] sheet, bound back to it by
/// `sheet_ref`. It keeps the same manifest scope, script/native-build label,
/// resolver identity, lockfile diff class, and registry/auth posture so a
/// governed proposal never re-derives a weaker preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewedSheetBinding {
    /// Canonical review-sheet id this proposal governs.
    pub sheet_ref: String,
    /// Mutation flow.
    pub flow_class: MutationFlowClass,
    /// Ecosystem the mutation targets.
    pub ecosystem: EcosystemKind,
    /// Frozen manifest-scope class.
    pub manifest_scope_class: ManifestScopeClass,
    /// Whether a whole-workspace scope was explicitly confirmed.
    #[serde(default)]
    pub scope_confirmed: bool,
    /// Redacted manifest path; never a raw URL.
    pub redacted_manifest_path: String,
    /// Script / native-build label.
    pub script_label: ScriptBuildLabel,
    /// Reviewer-facing script/native-build disclosure note (required when risky).
    pub script_disclosure_note: String,
    /// Resolver identity class that produced the resolved set.
    pub resolver_class: ResolverIdentityClass,
    /// Resolver version disclosed to review.
    pub resolver_version: String,
    /// Registry or mirror source authority.
    pub registry_source: RegistrySourceAuthority,
    /// Auth mode used to reach the source.
    pub auth_mode: AuthMode,
    /// Lockfile diff class.
    pub lockfile_diff: LockfileDiffClass,
    /// Lockfile authority governing the resolved set.
    pub lockfile_authority: LockfileAuthority,
}

impl ReviewedSheetBinding {
    /// Whether this scope must be confirmed explicitly before a bulk mutation.
    pub const fn requires_scope_confirmation(&self) -> bool {
        self.manifest_scope_class.requires_explicit_confirmation()
    }

    /// Whether the confirmation requirement is satisfied.
    pub const fn scope_confirmation_satisfied(&self) -> bool {
        !self.requires_scope_confirmation() || self.scope_confirmed
    }

    /// Whether the script/native-build label blocks commit outright.
    pub const fn script_blocks_commit(&self) -> bool {
        self.script_label.blocks_commit()
    }

    /// Whether auth is required but unsatisfied.
    pub const fn auth_blocks(&self) -> bool {
        self.auth_mode.blocks_until_satisfied()
    }

    /// Whether the lockfile is divergent and blocks until reconciled.
    pub const fn lockfile_blocks(&self) -> bool {
        self.lockfile_authority.blocks_until_reconciled()
    }

    /// Whether the resolver re-resolves the set, so resolver version must be
    /// disclosed.
    pub const fn re_resolves(&self) -> bool {
        self.flow_class.re_resolves()
    }
}

/// One selected (or offered) validation task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationTaskRow {
    /// Validation-task kind.
    pub kind: ValidationTaskKind,
    /// Human-readable label.
    pub label: String,
    /// Whether this task must pass before the mutation commits.
    pub required_before_commit: bool,
    /// Whether this task was selected for the proposal.
    pub selected: bool,
    /// Why this task is (or is not) selected.
    pub rationale: String,
}

impl ValidationTaskRow {
    /// Whether this task's selection requirement is satisfied for commit.
    pub const fn satisfied_for_commit(&self) -> bool {
        !self.required_before_commit || self.selected
    }
}

/// The validation-task selection for a proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationTaskSelection {
    /// Stable selection id.
    pub selection_id: String,
    /// Selected and offered validation tasks.
    #[serde(default)]
    pub tasks: Vec<ValidationTaskRow>,
}

impl ValidationTaskSelection {
    /// Whether every required validation task is selected.
    pub fn all_required_selected(&self) -> bool {
        self.tasks.iter().all(|t| t.satisfied_for_commit())
    }

    /// Whether the proposal selected at least one validation task.
    pub fn has_selected_task(&self) -> bool {
        self.tasks.iter().any(|t| t.selected)
    }

    /// The required validation tasks that are not selected.
    pub fn required_unselected(&self) -> Vec<ValidationTaskKind> {
        self.tasks
            .iter()
            .filter(|t| !t.satisfied_for_commit())
            .map(|t| t.kind)
            .collect()
    }
}

/// Whether the claimed ecosystem can deliver each promised governance capability.
///
/// A missing capability forces a safe narrowing: the no-unsafe-fallback rule is
/// that a proposal whose ecosystem cannot provide the promised review preview,
/// deterministic resolver, durable rollback, or validation execution may never
/// proceed to execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcosystemCapabilityPosture {
    /// Ecosystem this posture describes.
    pub ecosystem: EcosystemKind,
    /// Whether the ecosystem can render a manifest/lockfile impact preview.
    pub provides_review_preview: bool,
    /// Whether the ecosystem exposes a deterministic, identifiable resolver.
    pub provides_deterministic_resolver: bool,
    /// Whether the ecosystem supports a durable rollback checkpoint.
    pub provides_durable_rollback: bool,
    /// Whether the ecosystem can run the selected validation tasks.
    pub provides_validation_execution: bool,
    /// Reviewer-facing posture note.
    pub note: String,
}

impl EcosystemCapabilityPosture {
    /// Whether every promised capability holds, so execution may proceed.
    pub const fn all_promised_met(&self) -> bool {
        self.provides_review_preview
            && self.provides_deterministic_resolver
            && self.provides_durable_rollback
            && self.provides_validation_execution
    }

    /// The promised capabilities the ecosystem cannot deliver.
    pub fn missing_capabilities(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if !self.provides_review_preview {
            missing.push("review_preview");
        }
        if !self.provides_deterministic_resolver {
            missing.push("deterministic_resolver");
        }
        if !self.provides_durable_rollback {
            missing.push("durable_rollback");
        }
        if !self.provides_validation_execution {
            missing.push("validation_execution");
        }
        missing
    }
}

/// The cross-surface-identical rollback handle a proposal carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackHandleRef {
    /// Checkpoint id or VCS ref the handle points at; never a raw URL.
    pub checkpoint_ref: String,
    /// Rollback class.
    pub rollback_class: RollbackClass,
    /// Whether the handle is durable.
    pub durable: bool,
    /// Revert / open-diff / export-patch recovery actions offered.
    #[serde(default)]
    pub recovery_actions: Vec<RecoveryActionKind>,
    /// Resulting package state the handle records.
    pub resulting_state: String,
}

impl RollbackHandleRef {
    /// Whether the handle offers revert, open-diff, and export-patch recovery.
    pub fn offers_all_recovery_actions(&self) -> bool {
        let kinds: BTreeSet<RecoveryActionKind> = self.recovery_actions.iter().copied().collect();
        RecoveryActionKind::ALL.iter().all(|k| kinds.contains(k))
    }

    /// Whether the rollback class is a real recovery path.
    pub const fn is_recoverable(&self) -> bool {
        matches!(
            self.rollback_class,
            RollbackClass::ReversibleCheckpointed
                | RollbackClass::ReversibleManifestOnly
                | RollbackClass::CompensatingOnly
        )
    }

    /// Whether the handle is a durable, complete, recoverable checkpoint.
    pub fn is_durable_recovery(&self) -> bool {
        self.durable && self.offers_all_recovery_actions() && self.is_recoverable()
    }
}

/// The parity attestation that a governed proposal is not a bypass lane.
///
/// Every field must hold for a proposal to proceed to execution: the proposal
/// must reuse the reviewed contract, preserve the script/native-build
/// disclosure, the registry/auth posture, the validation selection, and the
/// rollback packet, never become a bypass lane, never turn package mutation into
/// hidden scripting, and never silently broaden scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParityAttestation {
    /// Reuses the canonical reviewed-mutation contract.
    pub reuses_review_contract: bool,
    /// Preserves the script/native-build disclosure.
    pub preserves_script_disclosure: bool,
    /// Preserves the registry/auth posture.
    pub preserves_registry_auth_posture: bool,
    /// Preserves the validation-task selection.
    pub preserves_validation_selection: bool,
    /// Preserves the rollback packet.
    pub preserves_rollback_packet: bool,
    /// Is not a bypass lane around review.
    pub not_a_bypass_lane: bool,
    /// Does not turn package mutation into hidden scripting.
    pub no_hidden_scripting: bool,
    /// Does not silently broaden the manifest scope.
    pub no_silent_scope_broadening: bool,
}

impl ParityAttestation {
    /// Whether the attestation fully preserves the reviewed contract.
    pub const fn fully_preserves_contract(&self) -> bool {
        self.reuses_review_contract
            && self.preserves_script_disclosure
            && self.preserves_registry_auth_posture
            && self.preserves_validation_selection
            && self.preserves_rollback_packet
            && self.not_a_bypass_lane
            && self.no_hidden_scripting
            && self.no_silent_scope_broadening
    }
}

/// One governed package-mutation proposal — the object every surface produces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedMutationProposal {
    /// Stable proposal id.
    pub proposal_id: String,
    /// The automation surface that produced the proposal.
    pub automation_surface: ProposalSource,
    /// Human-readable proposal label.
    pub governed_label: String,
    /// Reviewed-sheet binding (the manifest/lockfile impact preview).
    pub reviewed_sheet: ReviewedSheetBinding,
    /// Validation-task selection.
    pub validation: ValidationTaskSelection,
    /// Ecosystem capability posture.
    pub capability: EcosystemCapabilityPosture,
    /// Governed execution decision.
    pub execution_decision: ExecutionDecision,
    /// Cross-surface-identical result class.
    pub result_class: GovernedResultClass,
    /// Parity attestation.
    pub parity: ParityAttestation,
    /// Cross-surface-identical rollback handle.
    pub rollback_handle: RollbackHandleRef,
    /// Surface parity flags.
    pub surface_parity: GovernanceSurfaceParity,
    /// Frozen package-state labels this proposal surfaces; each binds the matrix.
    #[serde(default)]
    pub applicable_labels: Vec<PackageStateLabel>,
    /// Reviewer-facing note.
    pub note: String,
}

impl GovernedMutationProposal {
    /// Whether any intrinsic safety blocker holds — the conditions under which a
    /// proposal can never proceed to execution.
    ///
    /// A proposal is unsafe to proceed when its script/native-build label blocks
    /// commit, auth is unsatisfied, the lockfile is divergent, a whole-workspace
    /// scope was not confirmed, or the ecosystem cannot deliver every promised
    /// capability.
    pub fn intrinsic_unsafe(&self) -> bool {
        self.reviewed_sheet.script_blocks_commit()
            || self.reviewed_sheet.auth_blocks()
            || self.reviewed_sheet.lockfile_blocks()
            || !self.reviewed_sheet.scope_confirmation_satisfied()
            || !self.capability.all_promised_met()
    }

    /// Whether the proposal must narrow away from execution.
    pub fn must_narrow(&self) -> bool {
        self.intrinsic_unsafe()
    }

    /// Whether the commit gate is blocked for this proposal.
    ///
    /// A mutation may only commit when its decision permits execution, no
    /// intrinsic safety blocker holds, every required validation task is
    /// selected, parity is fully preserved, and the rollback handle is a durable
    /// recovery path.
    pub fn commit_gate_blocked(&self) -> bool {
        !self.execution_decision.permits_execution()
            || self.intrinsic_unsafe()
            || !self.validation.all_required_selected()
            || !self.parity.fully_preserves_contract()
            || !self.rollback_handle.is_durable_recovery()
    }

    /// Whether the proposal discloses every field review must see.
    pub fn discloses_all_required(&self) -> bool {
        !self.governed_label.trim().is_empty()
            && !self.reviewed_sheet.sheet_ref.trim().is_empty()
            && !self.reviewed_sheet.redacted_manifest_path.trim().is_empty()
            && (!self.reviewed_sheet.re_resolves()
                || !self.reviewed_sheet.resolver_version.trim().is_empty())
            && (!self.reviewed_sheet.script_label.is_risky()
                || !self.reviewed_sheet.script_disclosure_note.trim().is_empty())
            && !self.rollback_handle.resulting_state.trim().is_empty()
    }

    /// The frozen labels this proposal surfaces.
    pub fn labels(&self) -> &[PackageStateLabel] {
        &self.applicable_labels
    }

    /// Builds the reusable manifest-diff card for this governed proposal.
    ///
    /// Automation cards carry the selected validation-task set, and they narrow
    /// honestly when the proposal cannot provide a preview or durable checkpoint.
    pub fn manifest_diff_card(&self) -> ManifestDiffCard {
        let preview_state = if self.capability.provides_review_preview {
            ManifestDiffPreviewState::Available
        } else {
            ManifestDiffPreviewState::NoPreview
        };
        let checkpoint_state = if self.capability.provides_durable_rollback {
            ManifestDiffCheckpointState::Available
        } else {
            ManifestDiffCheckpointState::NarrowedNoCheckpoint
        };
        let rollback_state = if self.rollback_handle.is_durable_recovery() {
            if self.rollback_handle.rollback_class == RollbackClass::CompensatingOnly {
                ManifestDiffRollbackState::CompensatingOnly
            } else {
                ManifestDiffRollbackState::Available
            }
        } else {
            ManifestDiffRollbackState::Unavailable
        };
        let apply_action = if self.commit_gate_blocked() {
            match self.execution_decision {
                ExecutionDecision::NarrowToInspectOnly
                | ExecutionDecision::NarrowToExportOnly
                | ExecutionDecision::HandoffToBrowser
                | ExecutionDecision::HandoffToCli => ManifestDiffApplyAction::InspectOnly,
                ExecutionDecision::BlockedNoSafePath | ExecutionDecision::ProceedAfterReview => {
                    ManifestDiffApplyAction::Blocked
                }
            }
        } else {
            ManifestDiffApplyAction::Apply
        };
        let selected_validation_tasks = self
            .validation
            .tasks
            .iter()
            .filter(|task| task.selected)
            .map(|task| task.kind.as_str().to_owned())
            .collect();
        let lockfile_touch_note = if self.reviewed_sheet.lockfile_diff.changes_lockfile() {
            format!(
                "{} lockfile preview under {} authority",
                self.reviewed_sheet.lockfile_diff.as_str(),
                self.reviewed_sheet.lockfile_authority.as_str()
            )
        } else {
            "No lockfile touched by this governed proposal.".to_owned()
        };
        ManifestDiffCard {
            card_id: format!("mdc:{}", self.proposal_id),
            sheet_ref: self.reviewed_sheet.sheet_ref.clone(),
            proposal_source: self.automation_surface,
            action_class: self.reviewed_sheet.flow_class.manifest_diff_action(),
            package_ref: self.proposal_id.clone(),
            affected_manifest_refs: vec![self.reviewed_sheet.redacted_manifest_path.clone()],
            affected_lockfile_refs: if self.reviewed_sheet.lockfile_diff.changes_lockfile() {
                vec![format!(
                    "lockfile:{}:{}",
                    self.reviewed_sheet.ecosystem.as_str(),
                    self.reviewed_sheet.lockfile_diff.as_str()
                )]
            } else {
                Vec::new()
            },
            lockfile_touch_note,
            scripts_hooks_note: self.reviewed_sheet.script_disclosure_note.clone(),
            peer_runtime_constraints_note: if self
                .reviewed_sheet
                .script_disclosure_note
                .contains("native")
            {
                "Runtime/toolchain constraints are reviewed with the selected validation tasks."
                    .to_owned()
            } else {
                "No peer/runtime constraint changes claimed by this governed proposal.".to_owned()
            },
            constraint_changes: Vec::new(),
            preview_state,
            checkpoint_state,
            rollback_state,
            checkpoint_ref: self.rollback_handle.checkpoint_ref.clone(),
            rollback_ref: self.rollback_handle.checkpoint_ref.clone(),
            validation_selection_ref: Some(self.validation.selection_id.clone()),
            selected_validation_tasks,
            apply_action,
            consumer_surfaces: vec![
                "package_manager".to_owned(),
                "review_pane".to_owned(),
                "ai_recipe_cli".to_owned(),
                "support_export".to_owned(),
                "release_proof".to_owned(),
            ],
        }
    }
}

/// A redaction-safe view of one proposal, projected for a surface.
///
/// The `result_class`, `execution_decision`, and `rollback_handle_ref` are
/// identical across every surface; only `write_authority`, `can_execute_here`,
/// and `redacted` change per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedProposalSurfaceProjection {
    /// Proposal id.
    pub proposal_id: String,
    /// Surface token.
    pub surface: String,
    /// Write authority this surface carries.
    pub write_authority: String,
    /// Whether this surface may execute an unblocked, proceedable proposal.
    pub can_execute_here: bool,
    /// Whether the projection is redacted for export.
    pub redacted: bool,
    /// Cross-surface-identical result-class token.
    pub result_class: String,
    /// Cross-surface-identical execution-decision token.
    pub execution_decision: String,
    /// Cross-surface-identical rollback handle ref.
    pub rollback_handle_ref: String,
}

/// Summary counts derived from the proposals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutomationGovernanceSummary {
    /// Total proposals.
    pub total_proposals: usize,
    /// AI proposals.
    pub ai_proposals: usize,
    /// Recipe proposals.
    pub recipe_proposals: usize,
    /// CLI/headless proposals.
    pub cli_proposals: usize,
    /// Desktop baseline proposals.
    pub desktop_proposals: usize,
    /// Proposals permitted to proceed to execution.
    pub proceed_proposals: usize,
    /// Proposals narrowed to inspect-only.
    pub narrowed_inspect_proposals: usize,
    /// Proposals handed off (browser/CLI/export).
    pub handoff_proposals: usize,
    /// Proposals blocked with no safe path.
    pub blocked_proposals: usize,
    /// Proposals committed after review.
    pub committed_proposals: usize,
    /// Proposals committed then rolled back.
    pub rolled_back_proposals: usize,
    /// Proposals whose ecosystem cannot deliver every promised capability.
    pub capability_gap_proposals: usize,
    /// Proposals whose commit gate is blocked.
    pub commit_blocked_proposals: usize,
}

/// One row of the redaction-safe export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationGovernanceExportRow {
    /// Proposal id.
    pub row_id: String,
    /// Automation-surface token.
    pub automation_surface: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Proposal label.
    pub label: String,
    /// Execution-decision token.
    pub execution_decision: String,
    /// Result-class token.
    pub result_class: String,
    /// Whether the row blocks commit.
    pub blocks_commit: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// Redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationGovernanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<AutomationGovernanceExportRow>,
    /// Whether any proposal blocks commit.
    pub blocks_any_commit: bool,
    /// Whether every capability-gap proposal narrows safely.
    pub all_gaps_narrowed: bool,
    /// Whether every proposal discloses all required disclosures.
    pub all_disclose_required: bool,
    /// Whether every proposal's labels bind to the frozen matrix.
    pub all_bind_matrix: bool,
}

/// Typed automation-governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutomationGovernance {
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
    /// The frozen matrix this packet binds to.
    pub references_matrix_id: String,
    /// The reviewed-mutation contract this packet reuses.
    pub references_reviewed_flows_id: String,
    /// Stable surface contract.
    pub surface_contract: AutomationGovernanceSurfaceContract,
    /// Closed automation-surface vocabulary.
    pub automation_surfaces: Vec<ProposalSource>,
    /// Closed validation-task-kind vocabulary.
    pub validation_task_kinds: Vec<ValidationTaskKind>,
    /// Closed execution-decision vocabulary.
    pub execution_decisions: Vec<ExecutionDecision>,
    /// Closed result-class vocabulary.
    pub result_classes: Vec<GovernedResultClass>,
    /// Governed mutation proposals.
    #[serde(default)]
    pub proposals: Vec<GovernedMutationProposal>,
    /// Summary counts.
    pub summary: AutomationGovernanceSummary,
}

impl AutomationGovernance {
    /// Returns the proposal for `proposal_id`.
    pub fn proposal(&self, proposal_id: &str) -> Option<&GovernedMutationProposal> {
        self.proposals
            .iter()
            .find(|row| row.proposal_id == proposal_id)
    }

    /// Whether any proposal blocks commit.
    pub fn blocks_any_commit(&self) -> bool {
        self.proposals.iter().any(|p| p.commit_gate_blocked())
    }

    /// Whether every capability-gap proposal narrows safely away from execution.
    ///
    /// This is the no-unsafe-fallback guarantee: no proposal whose ecosystem
    /// cannot deliver every promised capability — or that carries any intrinsic
    /// safety blocker — is permitted to proceed to execution.
    pub fn all_gaps_narrowed(&self) -> bool {
        self.proposals
            .iter()
            .all(|p| !p.intrinsic_unsafe() || p.execution_decision.is_safe_narrowing())
    }

    /// Whether every proposal discloses all required disclosures.
    pub fn all_disclose_required(&self) -> bool {
        self.proposals.iter().all(|p| p.discloses_all_required())
    }

    /// Whether the packet binds to the matrix and every label resolves in it.
    pub fn all_bind_matrix(&self) -> bool {
        let Ok(matrix) = current_m5_package_state_matrix() else {
            return false;
        };
        if self.references_matrix_id != matrix.packet_id {
            return false;
        }
        self.proposals
            .iter()
            .flat_map(|p| p.applicable_labels.iter())
            .all(|label| matrix.state(*label).is_some())
    }

    /// Projects a proposal for a marketed surface with the write authority that
    /// surface carries, pinned from the frozen matrix. The result class,
    /// execution decision, and rollback handle are identical across surfaces.
    pub fn surface_projection(
        &self,
        proposal_id: &str,
        surface: PackageSurface,
    ) -> Option<GovernedProposalSurfaceProjection> {
        let proposal = self.proposal(proposal_id)?;
        let authority = surface.canonical_write_authority();
        let blocked = proposal.commit_gate_blocked();
        Some(GovernedProposalSurfaceProjection {
            proposal_id: proposal.proposal_id.clone(),
            surface: surface.as_str().to_owned(),
            write_authority: authority.as_str().to_owned(),
            can_execute_here: authority.can_mutate() && !blocked,
            redacted: matches!(authority, SurfaceWriteAuthority::RedactedExport),
            result_class: proposal.result_class.as_str().to_owned(),
            execution_decision: proposal.execution_decision.as_str().to_owned(),
            rollback_handle_ref: proposal.rollback_handle.checkpoint_ref.clone(),
        })
    }

    /// Recomputes the summary block from the proposals.
    pub fn computed_summary(&self) -> AutomationGovernanceSummary {
        let surface_count = |surface: ProposalSource| {
            self.proposals
                .iter()
                .filter(|p| p.automation_surface == surface)
                .count()
        };
        let decision_count = |decision: ExecutionDecision| {
            self.proposals
                .iter()
                .filter(|p| p.execution_decision == decision)
                .count()
        };
        let result_count = |result: GovernedResultClass| {
            self.proposals
                .iter()
                .filter(|p| p.result_class == result)
                .count()
        };
        AutomationGovernanceSummary {
            total_proposals: self.proposals.len(),
            ai_proposals: surface_count(ProposalSource::AiProposal),
            recipe_proposals: surface_count(ProposalSource::RecipeProposal),
            cli_proposals: surface_count(ProposalSource::CliHeadlessDryRun),
            desktop_proposals: surface_count(ProposalSource::DesktopManual),
            proceed_proposals: decision_count(ExecutionDecision::ProceedAfterReview),
            narrowed_inspect_proposals: decision_count(ExecutionDecision::NarrowToInspectOnly),
            handoff_proposals: self
                .proposals
                .iter()
                .filter(|p| {
                    matches!(
                        p.execution_decision,
                        ExecutionDecision::HandoffToBrowser
                            | ExecutionDecision::HandoffToCli
                            | ExecutionDecision::NarrowToExportOnly
                    )
                })
                .count(),
            blocked_proposals: decision_count(ExecutionDecision::BlockedNoSafePath),
            committed_proposals: result_count(GovernedResultClass::CommittedReviewed),
            rolled_back_proposals: result_count(GovernedResultClass::RolledBack),
            capability_gap_proposals: self
                .proposals
                .iter()
                .filter(|p| !p.capability.all_promised_met())
                .count(),
            commit_blocked_proposals: self
                .proposals
                .iter()
                .filter(|p| p.commit_gate_blocked())
                .count(),
        }
    }

    /// Produces a redaction-safe export projection for UI, CLI, support, docs,
    /// release, and public-proof consumers.
    pub fn export_projection(&self) -> AutomationGovernanceExportProjection {
        let mut rows = Vec::new();
        for proposal in &self.proposals {
            rows.push(AutomationGovernanceExportRow {
                row_id: proposal.proposal_id.clone(),
                automation_surface: proposal.automation_surface.as_str().to_owned(),
                ecosystem: proposal.reviewed_sheet.ecosystem.as_str().to_owned(),
                label: proposal.governed_label.clone(),
                execution_decision: proposal.execution_decision.as_str().to_owned(),
                result_class: proposal.result_class.as_str().to_owned(),
                blocks_commit: proposal.commit_gate_blocked(),
                summary: format!(
                    "{} {} scope {} script {} lockfile {} resolver {} validation {}/{} decision {} -> {}",
                    proposal.reviewed_sheet.flow_class.as_str(),
                    proposal.reviewed_sheet.sheet_ref,
                    proposal.reviewed_sheet.manifest_scope_class.as_str(),
                    proposal.reviewed_sheet.script_label.as_str(),
                    proposal.reviewed_sheet.lockfile_diff.as_str(),
                    proposal.reviewed_sheet.resolver_class.as_str(),
                    proposal.validation.tasks.iter().filter(|t| t.selected).count(),
                    proposal.validation.tasks.len(),
                    proposal.execution_decision.as_str(),
                    proposal.result_class.as_str(),
                ),
            });
        }
        AutomationGovernanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            blocks_any_commit: self.blocks_any_commit(),
            all_gaps_narrowed: self.all_gaps_narrowed(),
            all_disclose_required: self.all_disclose_required(),
            all_bind_matrix: self.all_bind_matrix(),
        }
    }

    /// Returns the corpus-coverage gaps: any automation surface, validation-task
    /// kind, execution decision, or result class the proposals do not exercise.
    pub fn corpus_coverage_gaps(&self) -> Vec<AutomationGovernanceViolation> {
        let mut gaps = Vec::new();
        let surfaces: BTreeSet<ProposalSource> = self
            .proposals
            .iter()
            .map(|p| p.automation_surface)
            .collect();
        for required in ProposalSource::ALL {
            if !surfaces.contains(&required) {
                gaps.push(AutomationGovernanceViolation::MissingCorpusState {
                    field: "automation_surface",
                    state: required.as_str(),
                });
            }
        }
        let decisions: BTreeSet<ExecutionDecision> = self
            .proposals
            .iter()
            .map(|p| p.execution_decision)
            .collect();
        for required in ExecutionDecision::ALL {
            if !decisions.contains(&required) {
                gaps.push(AutomationGovernanceViolation::MissingCorpusState {
                    field: "execution_decision",
                    state: required.as_str(),
                });
            }
        }
        let results: BTreeSet<GovernedResultClass> =
            self.proposals.iter().map(|p| p.result_class).collect();
        for required in GovernedResultClass::ALL {
            if !results.contains(&required) {
                gaps.push(AutomationGovernanceViolation::MissingCorpusState {
                    field: "result_class",
                    state: required.as_str(),
                });
            }
        }
        let kinds: BTreeSet<ValidationTaskKind> = self
            .proposals
            .iter()
            .flat_map(|p| p.validation.tasks.iter().map(|t| t.kind))
            .collect();
        for required in ValidationTaskKind::ALL {
            if !kinds.contains(&required) {
                gaps.push(AutomationGovernanceViolation::MissingCorpusState {
                    field: "validation_task_kind",
                    state: required.as_str(),
                });
            }
        }
        gaps
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<AutomationGovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_proposals(&mut violations);
        if self.summary != self.computed_summary() {
            violations.push(AutomationGovernanceViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<AutomationGovernanceViolation>) {
        if self.schema_version != AUTOMATION_GOVERNANCE_SCHEMA_VERSION {
            violations.push(AutomationGovernanceViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != AUTOMATION_GOVERNANCE_RECORD_KIND {
            violations.push(AutomationGovernanceViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("references_matrix_id", &self.references_matrix_id),
            (
                "references_reviewed_flows_id",
                &self.references_reviewed_flows_id,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(AutomationGovernanceViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, value) in [
            (
                "ai_proposal_surface",
                &self.surface_contract.ai_proposal_surface,
            ),
            ("recipe_surface", &self.surface_contract.recipe_surface),
            (
                "cli_headless_surface",
                &self.surface_contract.cli_headless_surface,
            ),
            (
                "desktop_baseline_surface",
                &self.surface_contract.desktop_baseline_surface,
            ),
            (
                "reviewed_sheet_surface",
                &self.surface_contract.reviewed_sheet_surface,
            ),
            ("help_page", &self.surface_contract.help_page),
            (
                "support_export_surface",
                &self.surface_contract.support_export_surface,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(AutomationGovernanceViolation::EmptyField {
                    id: "<surface_contract>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab_checks: [(&'static str, bool); 4] = [
            (
                "automation_surfaces",
                self.automation_surfaces == ProposalSource::ALL.to_vec(),
            ),
            (
                "validation_task_kinds",
                self.validation_task_kinds == ValidationTaskKind::ALL.to_vec(),
            ),
            (
                "execution_decisions",
                self.execution_decisions == ExecutionDecision::ALL.to_vec(),
            ),
            (
                "result_classes",
                self.result_classes == GovernedResultClass::ALL.to_vec(),
            ),
        ];
        for (field, ok) in vocab_checks {
            if !ok {
                violations.push(AutomationGovernanceViolation::ClosedVocabularyMismatch { field });
            }
        }
        if let Ok(matrix) = current_m5_package_state_matrix() {
            if self.references_matrix_id != matrix.packet_id {
                violations.push(AutomationGovernanceViolation::MatrixBindingMismatch {
                    referenced: self.references_matrix_id.clone(),
                });
            }
        }
    }

    fn validate_proposals(&self, violations: &mut Vec<AutomationGovernanceViolation>) {
        let matrix = current_m5_package_state_matrix().ok();
        let mut seen = BTreeSet::new();
        for proposal in &self.proposals {
            if !seen.insert(proposal.proposal_id.clone()) {
                violations.push(AutomationGovernanceViolation::DuplicateRowId {
                    row_id: proposal.proposal_id.clone(),
                });
            }
            self.validate_proposal(proposal, matrix.as_ref(), violations);
        }
    }

    fn validate_proposal(
        &self,
        proposal: &GovernedMutationProposal,
        matrix: Option<&crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::M5PackageStateMatrix>,
        violations: &mut Vec<AutomationGovernanceViolation>,
    ) {
        for (field, value) in [
            ("proposal_id", &proposal.proposal_id),
            ("governed_label", &proposal.governed_label),
            ("sheet_ref", &proposal.reviewed_sheet.sheet_ref),
            (
                "redacted_manifest_path",
                &proposal.reviewed_sheet.redacted_manifest_path,
            ),
            ("selection_id", &proposal.validation.selection_id),
            ("checkpoint_ref", &proposal.rollback_handle.checkpoint_ref),
            ("resulting_state", &proposal.rollback_handle.resulting_state),
            ("note", &proposal.note),
        ] {
            if value.trim().is_empty() {
                violations.push(AutomationGovernanceViolation::EmptyField {
                    id: proposal.proposal_id.clone(),
                    field_name: field,
                });
            }
        }
        // Redaction: no field may leak a raw URL.
        for (field, value) in [
            (
                "redacted_manifest_path",
                &proposal.reviewed_sheet.redacted_manifest_path,
            ),
            ("checkpoint_ref", &proposal.rollback_handle.checkpoint_ref),
        ] {
            if value.contains("://") {
                violations.push(AutomationGovernanceViolation::RawUrlLeak {
                    id: proposal.proposal_id.clone(),
                    field_name: field,
                });
            }
        }
        // A re-resolving flow must disclose its resolver version.
        if proposal.reviewed_sheet.re_resolves()
            && proposal.reviewed_sheet.resolver_version.trim().is_empty()
        {
            violations.push(AutomationGovernanceViolation::MissingResolverVersion {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // A risky script/native-build label must carry a disclosure note.
        if proposal.reviewed_sheet.script_label.is_risky()
            && proposal
                .reviewed_sheet
                .script_disclosure_note
                .trim()
                .is_empty()
        {
            violations.push(AutomationGovernanceViolation::ScriptRiskUndisclosed {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // No proposal may turn package mutation into hidden scripting.
        if !proposal.parity.no_hidden_scripting {
            violations.push(AutomationGovernanceViolation::HiddenScriptingAllowed {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // No-unsafe-fallback: an unsafe proposal can never proceed to execution.
        if proposal.intrinsic_unsafe() && proposal.execution_decision.permits_execution() {
            violations.push(AutomationGovernanceViolation::UnsafeFallbackExecution {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // A proceeding proposal must select every required validation task.
        if proposal.execution_decision.permits_execution()
            && !proposal.validation.all_required_selected()
        {
            violations.push(
                AutomationGovernanceViolation::RequiredValidationUnselected {
                    proposal_id: proposal.proposal_id.clone(),
                },
            );
        }
        // A proceeding proposal must select at least one validation task.
        if proposal.execution_decision.permits_execution()
            && !proposal.validation.has_selected_task()
        {
            violations.push(AutomationGovernanceViolation::NoValidationSelected {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // A proceeding proposal must fully preserve the reviewed contract.
        if proposal.execution_decision.permits_execution()
            && !proposal.parity.fully_preserves_contract()
        {
            violations.push(AutomationGovernanceViolation::ParityContractBroken {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // A proceeding whole-workspace mutation may not silently broaden scope.
        if proposal.execution_decision.permits_execution()
            && proposal.reviewed_sheet.requires_scope_confirmation()
            && !proposal.reviewed_sheet.scope_confirmed
        {
            violations.push(AutomationGovernanceViolation::SilentScopeBroadening {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // A proceeding proposal must carry a durable rollback recovery handle.
        if proposal.execution_decision.permits_execution()
            && !proposal.rollback_handle.is_durable_recovery()
        {
            violations.push(AutomationGovernanceViolation::RollbackHandleNotDurable {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // The result class must be coherent with the execution decision.
        if !decision_allows_result(proposal.execution_decision, proposal.result_class) {
            violations.push(AutomationGovernanceViolation::ResultDecisionMismatch {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // The commit gate: a committed result may carry no live block reason.
        if proposal.result_class.is_committed() && proposal.commit_gate_blocked() {
            violations.push(AutomationGovernanceViolation::CommitGateViolated {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // The capability posture must describe the same ecosystem as the sheet.
        if proposal.capability.ecosystem != proposal.reviewed_sheet.ecosystem {
            violations.push(AutomationGovernanceViolation::CapabilityEcosystemMismatch {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // Surface parity must hold across desktop, CLI, AI, and support/export.
        if !proposal.surface_parity.is_consistent() {
            violations.push(AutomationGovernanceViolation::SurfaceParityBroken {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // Every surfaced label must resolve in the frozen matrix.
        if let Some(matrix) = matrix {
            for label in &proposal.applicable_labels {
                if matrix.state(*label).is_none() {
                    violations.push(AutomationGovernanceViolation::UnboundLabel {
                        proposal_id: proposal.proposal_id.clone(),
                        label: label.as_str(),
                    });
                }
            }
        }
    }
}

/// A validation violation for the automation-governance packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutomationGovernanceViolation {
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
        /// Row, section, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A proposal id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A required corpus state is missing.
    MissingCorpusState {
        /// Field that must exercise the state.
        field: &'static str,
        /// Missing state token.
        state: &'static str,
    },
    /// A re-resolving flow does not disclose its resolver version.
    MissingResolverVersion {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proposal discloses script/native-build risk without a disclosure note.
    ScriptRiskUndisclosed {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proposal would allow package mutation to become hidden scripting.
    HiddenScriptingAllowed {
        /// Proposal id.
        proposal_id: String,
    },
    /// An intrinsically unsafe proposal is allowed to proceed to execution.
    UnsafeFallbackExecution {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proceeding proposal leaves a required validation task unselected.
    RequiredValidationUnselected {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proceeding proposal selects no validation task at all.
    NoValidationSelected {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proceeding proposal does not fully preserve the reviewed contract.
    ParityContractBroken {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proceeding whole-workspace mutation silently broadens scope.
    SilentScopeBroadening {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proceeding proposal carries no durable rollback recovery handle.
    RollbackHandleNotDurable {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proposal's result class disagrees with its execution decision.
    ResultDecisionMismatch {
        /// Proposal id.
        proposal_id: String,
    },
    /// A committed result still carries a live block reason.
    CommitGateViolated {
        /// Proposal id.
        proposal_id: String,
    },
    /// A capability posture describes a different ecosystem than the sheet.
    CapabilityEcosystemMismatch {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proposal's result is not mirrored to every claimed surface.
    SurfaceParityBroken {
        /// Proposal id.
        proposal_id: String,
    },
    /// A surfaced label does not resolve in the frozen matrix.
    UnboundLabel {
        /// Proposal id.
        proposal_id: String,
        /// Unbound label token.
        label: &'static str,
    },
    /// A redacted field leaks a raw URL.
    RawUrlLeak {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// The packet binds to a matrix id other than the frozen matrix.
    MatrixBindingMismatch {
        /// Referenced matrix id.
        referenced: String,
    },
    /// Summary counts disagree with the proposals.
    SummaryMismatch,
}

impl fmt::Display for AutomationGovernanceViolation {
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
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate proposal row id {row_id}")
            }
            Self::MissingCorpusState { field, state } => {
                write!(f, "packet corpus does not exercise {field} state {state}")
            }
            Self::MissingResolverVersion { proposal_id } => write!(
                f,
                "proposal {proposal_id} re-resolves without a resolver version"
            ),
            Self::ScriptRiskUndisclosed { proposal_id } => write!(
                f,
                "proposal {proposal_id} discloses script/native-build risk without a note"
            ),
            Self::HiddenScriptingAllowed { proposal_id } => write!(
                f,
                "proposal {proposal_id} would allow package mutation as hidden scripting"
            ),
            Self::UnsafeFallbackExecution { proposal_id } => write!(
                f,
                "proposal {proposal_id} proceeds to execution while intrinsically unsafe"
            ),
            Self::RequiredValidationUnselected { proposal_id } => write!(
                f,
                "proposal {proposal_id} proceeds with a required validation task unselected"
            ),
            Self::NoValidationSelected { proposal_id } => write!(
                f,
                "proposal {proposal_id} proceeds without selecting any validation task"
            ),
            Self::ParityContractBroken { proposal_id } => write!(
                f,
                "proposal {proposal_id} proceeds without fully preserving the reviewed contract"
            ),
            Self::SilentScopeBroadening { proposal_id } => write!(
                f,
                "proposal {proposal_id} proceeds on an unconfirmed whole-workspace scope"
            ),
            Self::RollbackHandleNotDurable { proposal_id } => write!(
                f,
                "proposal {proposal_id} proceeds without a durable rollback handle"
            ),
            Self::ResultDecisionMismatch { proposal_id } => write!(
                f,
                "proposal {proposal_id} result class disagrees with its execution decision"
            ),
            Self::CommitGateViolated { proposal_id } => write!(
                f,
                "proposal {proposal_id} is committed while a block reason still holds"
            ),
            Self::CapabilityEcosystemMismatch { proposal_id } => write!(
                f,
                "proposal {proposal_id} capability posture names a different ecosystem than the sheet"
            ),
            Self::SurfaceParityBroken { proposal_id } => write!(
                f,
                "proposal {proposal_id} result is not mirrored to every claimed surface"
            ),
            Self::UnboundLabel { proposal_id, label } => {
                write!(f, "proposal {proposal_id} surfaces unbound label {label}")
            }
            Self::RawUrlLeak { id, field_name } => {
                write!(f, "{id} field {field_name} leaks a raw URL")
            }
            Self::MatrixBindingMismatch { referenced } => {
                write!(f, "packet binds to non-frozen matrix {referenced}")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the proposals"),
        }
    }
}

impl Error for AutomationGovernanceViolation {}

/// Loads the embedded automation-governance packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`AutomationGovernance`].
pub fn current_automation_governance() -> Result<AutomationGovernance, serde_json::Error> {
    serde_json::from_str(AUTOMATION_GOVERNANCE_JSON)
}

#[cfg(test)]
mod tests;
