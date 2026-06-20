//! Command-graph and mutation-journal publication parity for mutating surfaces.
//!
//! Mutating product surfaces — AI apply, review actions, scaffold/update,
//! provider mutation, notebook/result mutation, and support repair — must
//! publish every
//! user-visible state change through the one canonical reactive path instead of
//! a private optimistic cache. The loophole this packet closes is the surface
//! that *looks* correct locally because it optimistically updated itself even
//! though the canonical command and mutation journal say otherwise.
//!
//! Four properties are frozen here:
//!
//! - **Publish only after commit.** A mutation becomes user-visible truth only
//!   after the command graph commits, the mutation journal commits, and the
//!   reactive graph republishes. No flow claims success before publication.
//! - **No private optimistic win.** Each surface declares how its optimistic
//!   path is handled — never present, removed, or quarantined behind the
//!   publication gate — so a local prediction can never outvote the canonical
//!   command, approval, or journal outcome.
//! - **Preserved lineage.** Every flow preserves actor, scope, command, and
//!   checkpoint lineage so diagnostics and support packets can reconstruct what
//!   the user saw and when.
//! - **Honest divergence.** When the canonical outcome diverges from what was
//!   shown the surface degrades or waits explicitly instead of taking a hidden
//!   cache win. [`ParityDrill`]s walk each surface from request through publish
//!   or honest divergence.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/reactive_command_parity.schema.json`](../../../../schemas/state/reactive_command_parity.schema.json)
//! - [`/docs/state/reactive_command_parity.md`](../../../../docs/state/reactive_command_parity.md)
//! - [`/artifacts/state/reactive_command_parity.json`](../../../../artifacts/state/reactive_command_parity.json)
//! - [`/artifacts/state/reactive_command_parity.md`](../../../../artifacts/state/reactive_command_parity.md)
//! - [`/artifacts/state/reactive_command_parity_drills.md`](../../../../artifacts/state/reactive_command_parity_drills.md)
//! - [`/fixtures/state/reactive_command_parity/`](../../../../fixtures/state/reactive_command_parity/)

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const REACTIVE_COMMAND_PARITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const REACTIVE_COMMAND_PARITY_PACKET_RECORD_KIND: &str =
    "reactive_command_parity_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const REACTIVE_COMMAND_PARITY_FIXTURE_RECORD_KIND: &str =
    "reactive_command_parity_fixture_record";

/// Repo-relative schema ref.
pub const REACTIVE_COMMAND_PARITY_SCHEMA_REF: &str =
    "schemas/state/reactive_command_parity.schema.json";

/// Repo-relative reviewer doc ref.
pub const REACTIVE_COMMAND_PARITY_DOC_REF: &str = "docs/state/reactive_command_parity.md";

/// Repo-relative machine-readable artifact packet.
pub const REACTIVE_COMMAND_PARITY_PACKET_REF: &str = "artifacts/state/reactive_command_parity.json";

/// Repo-relative reviewer artifact report.
pub const REACTIVE_COMMAND_PARITY_REPORT_REF: &str = "artifacts/state/reactive_command_parity.md";

/// Repo-relative reviewer drill report.
pub const REACTIVE_COMMAND_PARITY_DRILLS_REF: &str =
    "artifacts/state/reactive_command_parity_drills.md";

/// Repo-relative fixture directory.
pub const REACTIVE_COMMAND_PARITY_FIXTURE_DIR: &str = "fixtures/state/reactive_command_parity";

/// Repo-relative fixture manifest.
pub const REACTIVE_COMMAND_PARITY_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/reactive_command_parity/manifest.yaml";

/// Mutating product surface that must publish through the canonical reactive path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutatingSurface {
    /// AI scoped apply writing buffer and tree edits.
    AiApply,
    /// Review workspace approve, merge, and queue actions.
    ReviewAction,
    /// Scaffold and template update writing project files.
    ScaffoldUpdate,
    /// Connected-provider configuration and object mutation.
    ProviderMutation,
    /// Notebook cell execution and result mutation.
    NotebookResultMutation,
    /// Support-center repair that changes recoverable state.
    SupportRepair,
}

impl MutatingSurface {
    /// Returns the stable string vocabulary for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiApply => "ai_apply",
            Self::ReviewAction => "review_action",
            Self::ScaffoldUpdate => "scaffold_update",
            Self::ProviderMutation => "provider_mutation",
            Self::NotebookResultMutation => "notebook_result_mutation",
            Self::SupportRepair => "support_repair",
        }
    }
}

/// Kind of mutation a surface performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationKind {
    /// Apply an AI-proposed edit set.
    ApplyEdit,
    /// Approve, merge, or advance a review action.
    ApproveAction,
    /// Scaffold or update generated project artifacts.
    ScaffoldArtifact,
    /// Change connected-provider configuration.
    ProviderConfigChange,
    /// Execute a notebook cell and record its result.
    ExecuteCell,
    /// Repair recoverable state from the support center.
    RepairState,
}

impl MutationKind {
    /// Returns the stable string vocabulary for this mutation kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApplyEdit => "apply_edit",
            Self::ApproveAction => "approve_action",
            Self::ScaffoldArtifact => "scaffold_artifact",
            Self::ProviderConfigChange => "provider_config_change",
            Self::ExecuteCell => "execute_cell",
            Self::RepairState => "repair_state",
        }
    }
}

/// How a surface's feature-local optimistic state path is handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptimisticPosture {
    /// The surface never wrote a private optimistic cache; it waits for publish.
    NeverOptimistic,
    /// A former optimistic path was removed; the surface now shows pending.
    OptimisticRemoved,
    /// A local prediction is kept only as an explicit pending cue, gated so it
    /// can never become user-visible truth before the canonical path publishes.
    OptimisticQuarantined,
}

impl OptimisticPosture {
    /// Returns the stable string vocabulary for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeverOptimistic => "never_optimistic",
            Self::OptimisticRemoved => "optimistic_removed",
            Self::OptimisticQuarantined => "optimistic_quarantined",
        }
    }

    /// Returns true when the posture keeps a gated local prediction visible.
    pub const fn permits_local_prediction(self) -> bool {
        matches!(self, Self::OptimisticQuarantined)
    }

    /// Returns the pre-publish visibility this posture must present.
    ///
    /// A surface that never predicted waits; a removed or quarantined path shows
    /// an explicit pending cue. Neither ever shows published truth before the
    /// canonical path commits.
    pub const fn expected_pre_publish_state(self) -> StateVisibility {
        match self {
            Self::NeverOptimistic => StateVisibility::WaitingState,
            Self::OptimisticRemoved | Self::OptimisticQuarantined => StateVisibility::Pending,
        }
    }
}

/// Position of a mutation within the canonical publication pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationStage {
    /// The user requested the mutation; nothing has committed yet.
    ActionRequested,
    /// The command graph committed the mutation.
    CommandCommitted,
    /// The mutation journal committed the change with its lineage.
    JournalCommitted,
    /// The reactive graph republished the new state.
    ReactivePublished,
    /// The canonical outcome diverged from the requested mutation.
    Diverged,
}

impl PublicationStage {
    /// Returns the stable string vocabulary for this stage.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionRequested => "action_requested",
            Self::CommandCommitted => "command_committed",
            Self::JournalCommitted => "journal_committed",
            Self::ReactivePublished => "reactive_published",
            Self::Diverged => "diverged",
        }
    }

    /// Returns true once the reactive graph has republished the change.
    pub const fn is_published(self) -> bool {
        matches!(self, Self::ReactivePublished)
    }

    /// Returns true once the mutation journal has committed.
    pub const fn journal_committed(self) -> bool {
        matches!(self, Self::JournalCommitted | Self::ReactivePublished)
    }
}

/// What the user-visible surface claims while a mutation is in flight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateVisibility {
    /// An explicit in-flight cue; the change is not yet truth.
    Pending,
    /// The canonically published state is shown as current truth.
    PublishedTruth,
    /// The canonical outcome diverged; an explicit degraded state is shown.
    DegradedState,
    /// The surface waits for the canonical path; no truth is claimed.
    WaitingState,
}

impl StateVisibility {
    /// Returns the stable string vocabulary for this visibility.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::PublishedTruth => "published_truth",
            Self::DegradedState => "degraded_state",
            Self::WaitingState => "waiting_state",
        }
    }

    /// Returns true only when the surface claims current published truth.
    pub const fn claims_current_truth(self) -> bool {
        matches!(self, Self::PublishedTruth)
    }
}

/// How a flow resolves a divergence between the optimistic view and canon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DivergenceResolution {
    /// Degrade the surface to an explicit failed-change state.
    DegradeSurface,
    /// Hold an explicit waiting state until canonical truth arrives.
    HoldAndWait,
    /// Drop the local prediction and adopt the canonically published state.
    RevertToCanonical,
}

impl DivergenceResolution {
    /// Returns the stable string vocabulary for this resolution.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DegradeSurface => "degrade_surface",
            Self::HoldAndWait => "hold_and_wait",
            Self::RevertToCanonical => "revert_to_canonical",
        }
    }

    /// Returns the honest pre-recovery visibility this resolution presents.
    pub const fn divergent_visibility(self) -> StateVisibility {
        match self {
            Self::DegradeSurface => StateVisibility::DegradedState,
            Self::HoldAndWait => StateVisibility::WaitingState,
            // Reverting still adopts the published canonical state as truth.
            Self::RevertToCanonical => StateVisibility::PublishedTruth,
        }
    }
}

/// Lineage a published mutation preserves for later reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageField {
    /// The actor that requested the mutation.
    Actor,
    /// The scope the mutation applied to.
    Scope,
    /// The canonical command that drove the mutation.
    Command,
    /// The checkpoint the mutation can be reconstructed against.
    Checkpoint,
    /// The mutation-journal entry that recorded the change.
    JournalEntry,
    /// The reactive epoch at which the change was published.
    Epoch,
}

impl LineageField {
    /// Returns the stable string vocabulary for this lineage field.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Actor => "actor",
            Self::Scope => "scope",
            Self::Command => "command",
            Self::Checkpoint => "checkpoint",
            Self::JournalEntry => "journal_entry",
            Self::Epoch => "epoch",
        }
    }
}

/// Phase of a parity drill step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillPhase {
    /// The user requests the mutation.
    Request,
    /// The surface shows an explicit pending or waiting state.
    Pending,
    /// The command graph commits.
    CommandCommit,
    /// The mutation journal commits.
    JournalCommit,
    /// The reactive graph publishes the new state.
    Publish,
    /// The canonical outcome diverges from the request.
    Diverge,
    /// The resulting posture is verified honest and lineage-correlatable.
    Verify,
}

impl DrillPhase {
    /// Returns the stable string vocabulary for this phase.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Request => "request",
            Self::Pending => "pending",
            Self::CommandCommit => "command_commit",
            Self::JournalCommit => "journal_commit",
            Self::Publish => "publish",
            Self::Diverge => "diverge",
            Self::Verify => "verify",
        }
    }
}

/// One parity-flow row keyed by mutating surface and mutation kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityFlowRow {
    /// Stable flow id.
    pub flow_id: String,
    /// Mutating surface this flow governs.
    pub mutating_surface: MutatingSurface,
    /// Kind of mutation the surface performs.
    pub mutation_kind: MutationKind,
    /// How the surface's optimistic path is handled.
    pub optimistic_posture: OptimisticPosture,
    /// Visibility shown before the canonical path publishes.
    pub state_before_publish: StateVisibility,
    /// True only when publication waits for the command graph to commit.
    pub publishes_after_command_commit: bool,
    /// True only when publication waits for the mutation journal to commit.
    pub publishes_after_journal_commit: bool,
    /// True only when the change is published via the reactive graph.
    pub publishes_via_reactive_graph: bool,
    /// True only when the surface claims success before publication.
    pub claims_success_before_publish: bool,
    /// How the flow resolves a canonical divergence.
    pub divergence_resolution: DivergenceResolution,
    /// Lineage the published mutation preserves.
    pub preserved_lineage: Vec<LineageField>,
    /// True when the published state is support-correlatable with lineage.
    pub support_correlatable: bool,
    /// Support-safe summary of how the surface publishes.
    pub publication_summary: String,
    /// Support-safe summary of why the parity posture is honest.
    pub parity_rationale: String,
    /// Contract or module refs that anchor the flow.
    pub source_contract_refs: Vec<String>,
    /// Product consumers that quote the flow directly.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One ordered step inside a parity drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityDrillStep {
    /// Phase of this step.
    pub phase: DrillPhase,
    /// Publication stage reached at this step.
    pub publication_stage: PublicationStage,
    /// Visibility the surface presents at this step.
    pub state_visibility: StateVisibility,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One parity drill walking a surface from request to publish or divergence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Surface exercised by the drill.
    pub mutating_surface: MutatingSurface,
    /// Flow row exercised by the drill.
    pub exercised_flow_id: String,
    /// Ordered drill steps.
    pub steps: Vec<ParityDrillStep>,
    /// True when the drill proves no optimistic truth appears before publish.
    pub asserts_no_optimistic_truth_before_publish: bool,
    /// True when the drill proves the published state is lineage-correlatable.
    pub asserts_lineage_correlatable: bool,
    /// Publication stage the drill ends on.
    pub expected_final_publication_stage: PublicationStage,
    /// Visibility the drill ends on.
    pub expected_final_state_visibility: StateVisibility,
    /// Short reviewer note.
    pub notes: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Packet ref.
    pub packet_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// Drill report ref.
    pub drills_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet freezing the command-graph and mutation-journal parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveCommandParityPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// Parity-flow rows.
    pub flows: Vec<ParityFlowRow>,
    /// Parity drills.
    pub drills: Vec<ParityDrill>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// Fixture pinning one parity flow to its expected posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveCommandParityFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Parity flow under test.
    pub expected_flow_id: String,
    /// Mutating surface under test.
    pub mutating_surface: MutatingSurface,
    /// Mutation kind under test.
    pub mutation_kind: MutationKind,
    /// Expected optimistic posture.
    pub expected_optimistic_posture: OptimisticPosture,
    /// Expected divergence resolution.
    pub expected_divergence_resolution: DivergenceResolution,
    /// Expected pre-publish visibility.
    pub expected_state_before_publish: StateVisibility,
    /// Expected claim-before-publish flag.
    pub expected_claims_success_before_publish: bool,
    /// One consumer that would quote this scenario.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "reactive-command-parity validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

/// The four lineage fields every flow must preserve.
const REQUIRED_LINEAGE: [LineageField; 4] = [
    LineageField::Actor,
    LineageField::Scope,
    LineageField::Command,
    LineageField::Checkpoint,
];

// One row carries the full parity contract; a builder struct would obscure the
// seed more than the argument list does.
#[allow(clippy::too_many_arguments)]
fn flow(
    flow_id: &str,
    mutating_surface: MutatingSurface,
    mutation_kind: MutationKind,
    optimistic_posture: OptimisticPosture,
    divergence_resolution: DivergenceResolution,
    preserved_lineage: Vec<LineageField>,
    publication_summary: &str,
    parity_rationale: &str,
    source_contract_refs: Vec<&str>,
    consumer_refs: Vec<&str>,
    notes: &str,
) -> ParityFlowRow {
    ParityFlowRow {
        flow_id: flow_id.to_owned(),
        mutating_surface,
        mutation_kind,
        optimistic_posture,
        state_before_publish: optimistic_posture.expected_pre_publish_state(),
        publishes_after_command_commit: true,
        publishes_after_journal_commit: true,
        publishes_via_reactive_graph: true,
        claims_success_before_publish: false,
        divergence_resolution,
        preserved_lineage,
        support_correlatable: true,
        publication_summary: publication_summary.to_owned(),
        parity_rationale: parity_rationale.to_owned(),
        source_contract_refs: source_contract_refs
            .into_iter()
            .map(str::to_owned)
            .collect(),
        consumer_refs: consumer_refs.into_iter().map(str::to_owned).collect(),
        notes: notes.to_owned(),
    }
}

fn step(
    phase: DrillPhase,
    publication_stage: PublicationStage,
    state_visibility: StateVisibility,
    narration: &str,
) -> ParityDrillStep {
    ParityDrillStep {
        phase,
        publication_stage,
        state_visibility,
        narration: narration.to_owned(),
    }
}

const ADR_REF: &str = "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md";
const SUBSCRIPTION_SCHEMA_REF: &str = "schemas/runtime/subscription_envelope.schema.json";
const MUTATION_JOURNAL_SCHEMA_REF: &str = "schemas/state/mutation_journal.schema.json";
const COMMAND_DESCRIPTOR_SCHEMA_REF: &str = "schemas/commands/command_descriptor.schema.json";
const MUTATION_JOURNAL_MODULE_REF: &str =
    "crates/aureline-reactive-state/src/mutation_journal/mod.rs";

fn canonical_refs() -> Vec<&'static str> {
    vec![
        ADR_REF,
        SUBSCRIPTION_SCHEMA_REF,
        MUTATION_JOURNAL_SCHEMA_REF,
        COMMAND_DESCRIPTOR_SCHEMA_REF,
        MUTATION_JOURNAL_MODULE_REF,
    ]
}

/// Returns the checked-in packet this lane freezes.
pub fn seeded_reactive_command_parity_packet() -> ReactiveCommandParityPacket {
    let flows = vec![
        flow(
            "ai_apply_edit",
            MutatingSurface::AiApply,
            MutationKind::ApplyEdit,
            OptimisticPosture::OptimisticQuarantined,
            DivergenceResolution::RevertToCanonical,
            vec![
                LineageField::Actor,
                LineageField::Scope,
                LineageField::Command,
                LineageField::Checkpoint,
                LineageField::JournalEntry,
                LineageField::Epoch,
            ],
            "An AI scoped apply shows its edit set as a quarantined pending preview and marks the change current only after the apply command and the mutation journal commit and the reactive graph republishes the edited buffers and tree.",
            "The inline preview is gated and never becomes user-visible truth on its own; if the apply command or journal disagree it reverts to the canonically published edit, so the surface cannot win with a private optimistic cache.",
            canonical_refs(),
            vec![
                "crates/aureline-ai/src/harden_ai_scoped_apply/mod.rs",
                "crates/aureline-ai/src/composer/mod.rs",
            ],
            "AI apply publishes through the command and journal path; the preview cannot outvote the canonical edit.",
        ),
        flow(
            "review_approve_action",
            MutatingSurface::ReviewAction,
            MutationKind::ApproveAction,
            OptimisticPosture::NeverOptimistic,
            DivergenceResolution::HoldAndWait,
            vec![
                LineageField::Actor,
                LineageField::Scope,
                LineageField::Command,
                LineageField::Checkpoint,
                LineageField::Epoch,
            ],
            "A review approve or merge never optimistically flips the workspace; it waits while the action command and the mutation journal commit, then shows the merge-queue state the reactive graph republishes.",
            "Approve and merge depend on exact current state; holding in an explicit waiting state until the canonical path publishes prevents a stale base or denied approval from being shown as an approved cache win.",
            canonical_refs(),
            vec!["crates/aureline-review/src/workspace/mod.rs"],
            "Review actions wait for canonical publish; a moved base or denied approval holds rather than faking approval.",
        ),
        flow(
            "scaffold_update_artifact",
            MutatingSurface::ScaffoldUpdate,
            MutationKind::ScaffoldArtifact,
            OptimisticPosture::OptimisticRemoved,
            DivergenceResolution::RevertToCanonical,
            vec![
                LineageField::Actor,
                LineageField::Scope,
                LineageField::Command,
                LineageField::Checkpoint,
                LineageField::JournalEntry,
            ],
            "A scaffold or template update shows a pending in-flight cue after its former optimistic tree write was removed, and reflects the new files only once the scaffold command and the mutation journal commit and the reactive tree republishes.",
            "The removed optimistic write can no longer diverge from disk; a divergent canonical outcome reverts to the published tree so the file explorer never shows files the journal did not record.",
            canonical_refs(),
            vec![
                "crates/aureline-scaffold/src/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/mod.rs",
                "crates/aureline-scaffold/src/stabilize_template_manifest_scaffold_lineage/mod.rs",
            ],
            "Scaffold/update reflects the canonically published tree only; the old optimistic write was removed.",
        ),
        flow(
            "provider_config_mutation",
            MutatingSurface::ProviderMutation,
            MutationKind::ProviderConfigChange,
            OptimisticPosture::OptimisticRemoved,
            DivergenceResolution::DegradeSurface,
            vec![
                LineageField::Actor,
                LineageField::Scope,
                LineageField::Command,
                LineageField::Checkpoint,
                LineageField::Epoch,
            ],
            "A connected-provider configuration change shows a pending cue and publishes the new provider state only after the mutation command and the mutation journal commit and the reactive provider surface republishes.",
            "A provider can reject a change; degrading to an explicit failed-change state instead of keeping a removed optimistic config prevents the surface from claiming a configuration the provider never accepted.",
            canonical_refs(),
            vec![
                "crates/aureline-provider/src/object_model/mod.rs",
                "crates/aureline-provider/src/approval_tickets/mod.rs",
            ],
            "Provider mutation publishes the accepted config; a rejection degrades rather than holding a stale optimistic value.",
        ),
        flow(
            "notebook_execute_cell",
            MutatingSurface::NotebookResultMutation,
            MutationKind::ExecuteCell,
            OptimisticPosture::OptimisticQuarantined,
            DivergenceResolution::HoldAndWait,
            vec![
                LineageField::Actor,
                LineageField::Scope,
                LineageField::Command,
                LineageField::Checkpoint,
                LineageField::JournalEntry,
                LineageField::Epoch,
            ],
            "A notebook cell run shows a quarantined running cue and presents the result as current only after the execution command and the mutation journal commit and the reactive graph republishes the cell output.",
            "A running cue is not a result; holding until the canonical result publishes keeps a cancelled or superseded run from leaving a stale optimistic output behind.",
            canonical_refs(),
            vec![
                "crates/aureline-notebook/src/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces/mod.rs",
                "crates/aureline-notebook/src/add_notebook_result_comparison_baseline_selection_and_confounder_visibility/mod.rs",
            ],
            "Notebook results publish through the command and journal path; a running cue never stands in for the result.",
        ),
        flow(
            "support_repair_state",
            MutatingSurface::SupportRepair,
            MutationKind::RepairState,
            OptimisticPosture::NeverOptimistic,
            DivergenceResolution::DegradeSurface,
            vec![
                LineageField::Actor,
                LineageField::Scope,
                LineageField::Command,
                LineageField::Checkpoint,
                LineageField::JournalEntry,
            ],
            "A support-center repair never optimistically reports recovered state; it waits while the repair command and the mutation journal commit, then shows the repaired state the reactive graph republishes.",
            "A repair can fail partway; degrading to an explicit failed-repair state rather than reporting success keeps the support surface from claiming a recovery the journal never recorded.",
            canonical_refs(),
            vec![
                "crates/aureline-support/src/m5_cache_repair/mod.rs",
                "crates/aureline-support/src/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets/mod.rs",
            ],
            "Support repair waits for canonical publish; a failed repair degrades rather than reporting a cache win.",
        ),
    ];

    let drills = vec![
        ParityDrill {
            drill_id: "drill.reactive_command_parity.ai_apply_publishes_after_commit".to_owned(),
            title: "AI apply becomes truth only after command and journal commit".to_owned(),
            mutating_surface: MutatingSurface::AiApply,
            exercised_flow_id: "ai_apply_edit".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Request,
                    PublicationStage::ActionRequested,
                    StateVisibility::Pending,
                    "The user requests a scoped apply; the edit set shows as a quarantined pending preview.",
                ),
                step(
                    DrillPhase::Pending,
                    PublicationStage::ActionRequested,
                    StateVisibility::Pending,
                    "The preview is gated; no buffer or tree node claims the edit as current truth.",
                ),
                step(
                    DrillPhase::CommandCommit,
                    PublicationStage::CommandCommitted,
                    StateVisibility::Pending,
                    "The apply command commits in the command graph.",
                ),
                step(
                    DrillPhase::JournalCommit,
                    PublicationStage::JournalCommitted,
                    StateVisibility::Pending,
                    "The mutation journal records the edit with actor, scope, command, and checkpoint lineage.",
                ),
                step(
                    DrillPhase::Publish,
                    PublicationStage::ReactivePublished,
                    StateVisibility::PublishedTruth,
                    "The reactive graph republishes the edited buffers and tree as current truth.",
                ),
                step(
                    DrillPhase::Verify,
                    PublicationStage::ReactivePublished,
                    StateVisibility::PublishedTruth,
                    "Diagnostics correlate the published edit with its command and journal lineage.",
                ),
            ],
            asserts_no_optimistic_truth_before_publish: true,
            asserts_lineage_correlatable: true,
            expected_final_publication_stage: PublicationStage::ReactivePublished,
            expected_final_state_visibility: StateVisibility::PublishedTruth,
            notes: "The preview never claimed truth before publish; the edit became current only after the journal committed.".to_owned(),
        },
        ParityDrill {
            drill_id: "drill.reactive_command_parity.review_action_holds_on_divergence".to_owned(),
            title: "Review action holds in waiting when the canonical outcome diverges".to_owned(),
            mutating_surface: MutatingSurface::ReviewAction,
            exercised_flow_id: "review_approve_action".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Request,
                    PublicationStage::ActionRequested,
                    StateVisibility::WaitingState,
                    "A reviewer requests approve; the workspace shows a waiting state, never an optimistic approved flip.",
                ),
                step(
                    DrillPhase::Pending,
                    PublicationStage::ActionRequested,
                    StateVisibility::WaitingState,
                    "Approve stays in waiting while the canonical path runs.",
                ),
                step(
                    DrillPhase::CommandCommit,
                    PublicationStage::CommandCommitted,
                    StateVisibility::WaitingState,
                    "The approve command is accepted, but the merge base moved underneath it.",
                ),
                step(
                    DrillPhase::Diverge,
                    PublicationStage::Diverged,
                    StateVisibility::WaitingState,
                    "The canonical merge-queue outcome diverges from the request; the workspace keeps waiting instead of claiming approval.",
                ),
                step(
                    DrillPhase::Verify,
                    PublicationStage::Diverged,
                    StateVisibility::WaitingState,
                    "The waiting state stays visible with its command lineage rather than taking a hidden cache win.",
                ),
            ],
            asserts_no_optimistic_truth_before_publish: true,
            asserts_lineage_correlatable: true,
            expected_final_publication_stage: PublicationStage::Diverged,
            expected_final_state_visibility: StateVisibility::WaitingState,
            notes: "The divergence resolved to an explicit waiting state; the workspace never showed an approval the canonical path did not publish.".to_owned(),
        },
        ParityDrill {
            drill_id: "drill.reactive_command_parity.scaffold_update_publishes_canonical_tree"
                .to_owned(),
            title: "Scaffold update reflects only the canonically published tree".to_owned(),
            mutating_surface: MutatingSurface::ScaffoldUpdate,
            exercised_flow_id: "scaffold_update_artifact".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Request,
                    PublicationStage::ActionRequested,
                    StateVisibility::Pending,
                    "A scaffold update is requested; the explorer shows a pending in-flight cue.",
                ),
                step(
                    DrillPhase::Pending,
                    PublicationStage::ActionRequested,
                    StateVisibility::Pending,
                    "With the old optimistic write removed, no file appears before the command commits.",
                ),
                step(
                    DrillPhase::CommandCommit,
                    PublicationStage::CommandCommitted,
                    StateVisibility::Pending,
                    "The scaffold command commits in the command graph.",
                ),
                step(
                    DrillPhase::JournalCommit,
                    PublicationStage::JournalCommitted,
                    StateVisibility::Pending,
                    "The mutation journal records the written files with scope and checkpoint lineage.",
                ),
                step(
                    DrillPhase::Publish,
                    PublicationStage::ReactivePublished,
                    StateVisibility::PublishedTruth,
                    "The reactive tree republishes the new files as current truth.",
                ),
                step(
                    DrillPhase::Verify,
                    PublicationStage::ReactivePublished,
                    StateVisibility::PublishedTruth,
                    "The explorer shows exactly the files the journal recorded; no optimistic node remains.",
                ),
            ],
            asserts_no_optimistic_truth_before_publish: true,
            asserts_lineage_correlatable: true,
            expected_final_publication_stage: PublicationStage::ReactivePublished,
            expected_final_state_visibility: StateVisibility::PublishedTruth,
            notes: "No file was shown before the journal committed; the explorer matched the canonical tree.".to_owned(),
        },
        ParityDrill {
            drill_id: "drill.reactive_command_parity.provider_mutation_degrades_on_reject"
                .to_owned(),
            title: "Provider mutation degrades when the provider rejects the change".to_owned(),
            mutating_surface: MutatingSurface::ProviderMutation,
            exercised_flow_id: "provider_config_mutation".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Request,
                    PublicationStage::ActionRequested,
                    StateVisibility::Pending,
                    "A provider config change is requested; the surface shows pending, not an applied config.",
                ),
                step(
                    DrillPhase::Pending,
                    PublicationStage::ActionRequested,
                    StateVisibility::Pending,
                    "No optimistic config value is shown while the command runs.",
                ),
                step(
                    DrillPhase::CommandCommit,
                    PublicationStage::CommandCommitted,
                    StateVisibility::Pending,
                    "The command commits, but the provider rejects the change.",
                ),
                step(
                    DrillPhase::Diverge,
                    PublicationStage::Diverged,
                    StateVisibility::DegradedState,
                    "The canonical outcome diverges; the provider surface degrades to an explicit failed-change state.",
                ),
                step(
                    DrillPhase::Verify,
                    PublicationStage::Diverged,
                    StateVisibility::DegradedState,
                    "The degraded state is support-correlatable with the command and journal lineage; no stale config remains.",
                ),
            ],
            asserts_no_optimistic_truth_before_publish: true,
            asserts_lineage_correlatable: true,
            expected_final_publication_stage: PublicationStage::Diverged,
            expected_final_state_visibility: StateVisibility::DegradedState,
            notes: "A rejected change degraded explicitly; the surface never claimed a configuration the provider refused.".to_owned(),
        },
        ParityDrill {
            drill_id: "drill.reactive_command_parity.notebook_result_publishes_after_commit"
                .to_owned(),
            title: "Notebook cell result becomes truth only after the journal commits".to_owned(),
            mutating_surface: MutatingSurface::NotebookResultMutation,
            exercised_flow_id: "notebook_execute_cell".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Request,
                    PublicationStage::ActionRequested,
                    StateVisibility::Pending,
                    "A cell run is requested; the cell shows a quarantined running cue.",
                ),
                step(
                    DrillPhase::Pending,
                    PublicationStage::ActionRequested,
                    StateVisibility::Pending,
                    "The running cue is gated; the prior output is not replaced with an optimistic result.",
                ),
                step(
                    DrillPhase::CommandCommit,
                    PublicationStage::CommandCommitted,
                    StateVisibility::Pending,
                    "The execution command commits in the command graph.",
                ),
                step(
                    DrillPhase::JournalCommit,
                    PublicationStage::JournalCommitted,
                    StateVisibility::Pending,
                    "The mutation journal records the execution result with actor, scope, and checkpoint lineage.",
                ),
                step(
                    DrillPhase::Publish,
                    PublicationStage::ReactivePublished,
                    StateVisibility::PublishedTruth,
                    "The reactive graph republishes the cell output as current truth.",
                ),
                step(
                    DrillPhase::Verify,
                    PublicationStage::ReactivePublished,
                    StateVisibility::PublishedTruth,
                    "Diagnostics correlate the published output with its command and journal lineage.",
                ),
            ],
            asserts_no_optimistic_truth_before_publish: true,
            asserts_lineage_correlatable: true,
            expected_final_publication_stage: PublicationStage::ReactivePublished,
            expected_final_state_visibility: StateVisibility::PublishedTruth,
            notes: "The running cue never stood in for the result; the output became current only after the journal committed.".to_owned(),
        },
        ParityDrill {
            drill_id: "drill.reactive_command_parity.support_repair_publishes_after_commit"
                .to_owned(),
            title: "Support repair reports recovery only after the journal commits".to_owned(),
            mutating_surface: MutatingSurface::SupportRepair,
            exercised_flow_id: "support_repair_state".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Request,
                    PublicationStage::ActionRequested,
                    StateVisibility::WaitingState,
                    "A support repair is requested; the surface shows waiting, never an optimistic repaired state.",
                ),
                step(
                    DrillPhase::Pending,
                    PublicationStage::ActionRequested,
                    StateVisibility::WaitingState,
                    "The repair stays in waiting while the canonical path runs.",
                ),
                step(
                    DrillPhase::CommandCommit,
                    PublicationStage::CommandCommitted,
                    StateVisibility::WaitingState,
                    "The repair command commits in the command graph.",
                ),
                step(
                    DrillPhase::JournalCommit,
                    PublicationStage::JournalCommitted,
                    StateVisibility::WaitingState,
                    "The mutation journal records the repair with actor, scope, command, and checkpoint lineage.",
                ),
                step(
                    DrillPhase::Publish,
                    PublicationStage::ReactivePublished,
                    StateVisibility::PublishedTruth,
                    "The reactive graph republishes the repaired state as current truth.",
                ),
                step(
                    DrillPhase::Verify,
                    PublicationStage::ReactivePublished,
                    StateVisibility::PublishedTruth,
                    "The support packet correlates the published recovery with its command and journal lineage.",
                ),
            ],
            asserts_no_optimistic_truth_before_publish: true,
            asserts_lineage_correlatable: true,
            expected_final_publication_stage: PublicationStage::ReactivePublished,
            expected_final_state_visibility: StateVisibility::PublishedTruth,
            notes: "The repair was reported only after the journal committed; waiting held until the recovery published.".to_owned(),
        },
    ];

    ReactiveCommandParityPacket {
        record_kind: REACTIVE_COMMAND_PARITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: REACTIVE_COMMAND_PARITY_SCHEMA_VERSION,
        packet_id: "state.reactive_command_parity.v1".to_owned(),
        title: "Command-graph and mutation-journal publication parity for mutating surfaces"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: REACTIVE_COMMAND_PARITY_DOC_REF.to_owned(),
            schema_ref: REACTIVE_COMMAND_PARITY_SCHEMA_REF.to_owned(),
            packet_ref: REACTIVE_COMMAND_PARITY_PACKET_REF.to_owned(),
            report_ref: REACTIVE_COMMAND_PARITY_REPORT_REF.to_owned(),
            drills_ref: REACTIVE_COMMAND_PARITY_DRILLS_REF.to_owned(),
            fixture_manifest_ref: REACTIVE_COMMAND_PARITY_FIXTURE_MANIFEST_REF.to_owned(),
        },
        flows,
        drills,
        invariants: vec![
            "User-visible state on a mutating surface is published only after the command and the mutation journal commit and the reactive graph republishes; no surface claims success before publication.".to_owned(),
            "No mutating surface keeps a private optimistic cache that can outvote the canonical command, approval, or journal outcome; optimistic paths are never offered as truth, removed, or quarantined behind the publication gate.".to_owned(),
            "Every published state preserves actor, scope, command, and checkpoint lineage so diagnostics and support packets can reconstruct what the user saw and when.".to_owned(),
            "Known divergence cases convert to an explicit degraded or waiting state instead of a hidden cache win.".to_owned(),
            "Each mutating surface publishes through the one canonical reactive path instead of inventing a private epoch or stale-state language.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixtures this lane freezes.
pub fn seeded_reactive_command_parity_fixtures() -> Vec<ReactiveCommandParityFixture> {
    seeded_reactive_command_parity_packet()
        .flows
        .iter()
        .map(|row| ReactiveCommandParityFixture {
            record_kind: REACTIVE_COMMAND_PARITY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: REACTIVE_COMMAND_PARITY_SCHEMA_VERSION,
            fixture_id: format!("fixture.reactive_command_parity.{}", row.flow_id),
            expected_flow_id: row.flow_id.clone(),
            mutating_surface: row.mutating_surface,
            mutation_kind: row.mutation_kind,
            expected_optimistic_posture: row.optimistic_posture,
            expected_divergence_resolution: row.divergence_resolution,
            expected_state_before_publish: row.state_before_publish,
            expected_claims_success_before_publish: row.claims_success_before_publish,
            consumer_ref: row.consumer_refs.first().cloned().unwrap_or_default(),
            notes: row.notes.clone(),
        })
        .collect()
}

/// Validates the checked-in packet contract.
pub fn validate_reactive_command_parity_packet(
    packet: &ReactiveCommandParityPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != REACTIVE_COMMAND_PARITY_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != REACTIVE_COMMAND_PARITY_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.source_contract_refs.doc_ref != REACTIVE_COMMAND_PARITY_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != REACTIVE_COMMAND_PARITY_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen JSON schema",
        );
    }
    if packet.source_contract_refs.packet_ref != REACTIVE_COMMAND_PARITY_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != REACTIVE_COMMAND_PARITY_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.drills_ref != REACTIVE_COMMAND_PARITY_DRILLS_REF {
        report.push(
            "packet.drills_ref",
            "drills_ref drifted from the frozen drill artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != REACTIVE_COMMAND_PARITY_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    let mut flow_ids = BTreeSet::new();
    let mut covered_surfaces = BTreeSet::new();
    let mut covered_kinds = BTreeSet::new();
    let mut covered_postures = BTreeSet::new();
    let mut covered_resolutions = BTreeSet::new();

    for row in &packet.flows {
        if !flow_ids.insert(row.flow_id.as_str()) {
            report.push(
                "flow.id_unique",
                format!("duplicate flow_id {}", row.flow_id),
            );
        }
        validate_flow_row(&mut report, row);

        covered_surfaces.insert(row.mutating_surface);
        covered_kinds.insert(row.mutation_kind);
        covered_postures.insert(row.optimistic_posture);
        covered_resolutions.insert(row.divergence_resolution);
    }

    for required in [
        MutatingSurface::AiApply,
        MutatingSurface::ReviewAction,
        MutatingSurface::ScaffoldUpdate,
        MutatingSurface::ProviderMutation,
        MutatingSurface::NotebookResultMutation,
        MutatingSurface::SupportRepair,
    ] {
        if !covered_surfaces.contains(&required) {
            report.push(
                "packet.covered_surface",
                format!("packet must cover mutating surface {}", required.as_str()),
            );
        }
    }
    for required in [
        MutationKind::ApplyEdit,
        MutationKind::ApproveAction,
        MutationKind::ScaffoldArtifact,
        MutationKind::ProviderConfigChange,
        MutationKind::ExecuteCell,
        MutationKind::RepairState,
    ] {
        if !covered_kinds.contains(&required) {
            report.push(
                "packet.covered_kind",
                format!("packet must cover mutation kind {}", required.as_str()),
            );
        }
    }
    for required in [
        OptimisticPosture::NeverOptimistic,
        OptimisticPosture::OptimisticRemoved,
        OptimisticPosture::OptimisticQuarantined,
    ] {
        if !covered_postures.contains(&required) {
            report.push(
                "packet.covered_posture",
                format!("packet must cover optimistic posture {}", required.as_str()),
            );
        }
    }
    for required in [
        DivergenceResolution::DegradeSurface,
        DivergenceResolution::HoldAndWait,
        DivergenceResolution::RevertToCanonical,
    ] {
        if !covered_resolutions.contains(&required) {
            report.push(
                "packet.covered_resolution",
                format!(
                    "packet must cover divergence resolution {}",
                    required.as_str()
                ),
            );
        }
    }

    validate_drills(&mut report, packet, &flow_ids);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_flow_row(report: &mut ValidationReport, row: &ParityFlowRow) {
    // The central guardrail: nothing is user-visible truth before publish, and
    // publish itself waits for the command and journal to commit.
    if !row.publishes_after_command_commit {
        report.push(
            "flow.publishes_after_command_commit",
            format!(
                "flow {} must publish only after the command graph commits",
                row.flow_id
            ),
        );
    }
    if !row.publishes_after_journal_commit {
        report.push(
            "flow.publishes_after_journal_commit",
            format!(
                "flow {} must publish only after the mutation journal commits",
                row.flow_id
            ),
        );
    }
    if !row.publishes_via_reactive_graph {
        report.push(
            "flow.publishes_via_reactive_graph",
            format!(
                "flow {} must publish through the reactive graph",
                row.flow_id
            ),
        );
    }
    if row.claims_success_before_publish {
        report.push(
            "flow.claims_success_before_publish",
            format!(
                "flow {} must not claim success before the canonical path publishes",
                row.flow_id
            ),
        );
    }
    if !row.support_correlatable {
        report.push(
            "flow.support_correlatable",
            format!(
                "flow {} must keep its published state support-correlatable",
                row.flow_id
            ),
        );
    }

    // Pre-publish visibility is never published truth, and agrees with posture.
    if row.state_before_publish.claims_current_truth() {
        report.push(
            "flow.state_before_publish",
            format!(
                "flow {} must not show published truth before the canonical path publishes",
                row.flow_id
            ),
        );
    }
    if row.state_before_publish != row.optimistic_posture.expected_pre_publish_state() {
        report.push(
            "flow.posture_state_agreement",
            format!(
                "flow {} optimistic posture {} expects pre-publish visibility {} but found {}",
                row.flow_id,
                row.optimistic_posture.as_str(),
                row.optimistic_posture.expected_pre_publish_state().as_str(),
                row.state_before_publish.as_str()
            ),
        );
    }

    // Lineage must let diagnostics reconstruct what the user saw and when.
    let lineage: BTreeSet<_> = row.preserved_lineage.iter().copied().collect();
    if lineage.len() != row.preserved_lineage.len() {
        report.push(
            "flow.preserved_lineage_unique",
            format!("flow {} repeats a preserved lineage field", row.flow_id),
        );
    }
    for required in REQUIRED_LINEAGE {
        if !lineage.contains(&required) {
            report.push(
                "flow.preserved_lineage",
                format!(
                    "flow {} must preserve {} lineage",
                    row.flow_id,
                    required.as_str()
                ),
            );
        }
    }

    if row.publication_summary.trim().is_empty() {
        report.push(
            "flow.publication_summary",
            format!("flow {} must explain how it publishes", row.flow_id),
        );
    }
    if row.parity_rationale.trim().is_empty() {
        report.push(
            "flow.parity_rationale",
            format!(
                "flow {} must explain why its parity posture is honest",
                row.flow_id
            ),
        );
    }
    if row.source_contract_refs.is_empty() {
        report.push(
            "flow.source_contract_refs",
            format!("flow {} must cite source contract refs", row.flow_id),
        );
    }
    if row.consumer_refs.is_empty() {
        report.push(
            "flow.consumer_refs",
            format!("flow {} must cite at least one consumer ref", row.flow_id),
        );
    }
    if row.notes.trim().is_empty() {
        report.push(
            "flow.notes",
            format!("flow {} must carry a reviewer note", row.flow_id),
        );
    }
}

fn validate_drills(
    report: &mut ValidationReport,
    packet: &ReactiveCommandParityPacket,
    flow_ids: &BTreeSet<&str>,
) {
    if packet.drills.is_empty() {
        report.push("packet.drills", "packet must declare parity drills");
    }

    let mut drill_ids = BTreeSet::new();
    let mut covered_surfaces = BTreeSet::new();
    let mut has_divergence_drill = false;
    let mut has_publish_drill = false;
    let flows_by_id: BTreeMap<_, _> = packet
        .flows
        .iter()
        .map(|row| (row.flow_id.as_str(), row))
        .collect();

    for drill in &packet.drills {
        if !drill_ids.insert(drill.drill_id.as_str()) {
            report.push(
                "drill.id_unique",
                format!("duplicate drill_id {}", drill.drill_id),
            );
        }
        if !flow_ids.contains(drill.exercised_flow_id.as_str()) {
            report.push(
                "drill.exercised_flow_id",
                format!(
                    "drill {} references unknown flow {}",
                    drill.drill_id, drill.exercised_flow_id
                ),
            );
        } else if let Some(flow_row) = flows_by_id.get(drill.exercised_flow_id.as_str()) {
            if flow_row.mutating_surface != drill.mutating_surface {
                report.push(
                    "drill.flow_surface_match",
                    format!(
                        "drill {} surface does not match flow {}",
                        drill.drill_id, drill.exercised_flow_id
                    ),
                );
            }
        }
        if !drill.asserts_no_optimistic_truth_before_publish {
            report.push(
                "drill.asserts_no_optimistic_truth_before_publish",
                format!(
                    "drill {} must assert no optimistic truth before publish",
                    drill.drill_id
                ),
            );
        }
        if !drill.asserts_lineage_correlatable {
            report.push(
                "drill.asserts_lineage_correlatable",
                format!(
                    "drill {} must assert the published state is lineage-correlatable",
                    drill.drill_id
                ),
            );
        }

        if drill.steps.is_empty() {
            report.push(
                "drill.steps",
                format!("drill {} must declare steps", drill.drill_id),
            );
            continue;
        }
        if drill.steps.first().map(|s| s.phase) != Some(DrillPhase::Request) {
            report.push(
                "drill.first_phase",
                format!("drill {} must begin with a request step", drill.drill_id),
            );
        }
        let last = drill.steps.last().expect("non-empty drill steps");
        if last.phase != DrillPhase::Verify {
            report.push(
                "drill.last_phase",
                format!("drill {} must end with a verify step", drill.drill_id),
            );
        }
        if drill.expected_final_publication_stage != last.publication_stage
            || drill.expected_final_state_visibility != last.state_visibility
        {
            report.push(
                "drill.final_posture",
                format!(
                    "drill {} expected-final posture must match its verify step",
                    drill.drill_id
                ),
            );
        }
        let has_pending = drill
            .steps
            .iter()
            .any(|s| s.phase == DrillPhase::Pending && !s.state_visibility.claims_current_truth());
        if !has_pending {
            report.push(
                "drill.pending_step",
                format!(
                    "drill {} must show a pending or waiting step before publishing",
                    drill.drill_id
                ),
            );
        }

        let mut reaches_publish = false;
        let mut diverges = false;
        for (index, drill_step) in drill.steps.iter().enumerate() {
            // The central invariant: no published truth before the reactive
            // graph publishes.
            if drill_step.state_visibility.claims_current_truth()
                && !drill_step.publication_stage.is_published()
            {
                report.push(
                    "drill.step_no_optimistic_truth",
                    format!(
                        "drill {} step {} claims published truth at stage {}",
                        drill.drill_id,
                        index,
                        drill_step.publication_stage.as_str()
                    ),
                );
            }
            // Phase and stage must agree so the publication path is legible.
            match drill_step.phase {
                DrillPhase::Publish => {
                    if drill_step.publication_stage != PublicationStage::ReactivePublished
                        || drill_step.state_visibility != StateVisibility::PublishedTruth
                    {
                        report.push(
                            "drill.publish_step",
                            format!(
                                "drill {} publish step {} must reach published truth",
                                drill.drill_id, index
                            ),
                        );
                    }
                    reaches_publish = true;
                }
                DrillPhase::Diverge => {
                    if drill_step.publication_stage != PublicationStage::Diverged
                        || !matches!(
                            drill_step.state_visibility,
                            StateVisibility::DegradedState | StateVisibility::WaitingState
                        )
                    {
                        report.push(
                            "drill.diverge_step",
                            format!(
                                "drill {} diverge step {} must degrade or wait, never claim truth",
                                drill.drill_id, index
                            ),
                        );
                    }
                    diverges = true;
                }
                DrillPhase::JournalCommit => {
                    if !drill_step.publication_stage.journal_committed() {
                        report.push(
                            "drill.journal_commit_step",
                            format!(
                                "drill {} journal-commit step {} must reach a journal-committed stage",
                                drill.drill_id, index
                            ),
                        );
                    }
                }
                _ => {}
            }
            if drill_step.narration.trim().is_empty() {
                report.push(
                    "drill.step_narration",
                    format!(
                        "drill {} step {} must narrate the step",
                        drill.drill_id, index
                    ),
                );
            }
        }
        has_publish_drill |= reaches_publish;
        has_divergence_drill |= diverges;

        covered_surfaces.insert(drill.mutating_surface);
    }

    // Every claimed mutating surface must be drilled at least once.
    for required in [
        MutatingSurface::AiApply,
        MutatingSurface::ReviewAction,
        MutatingSurface::ScaffoldUpdate,
        MutatingSurface::ProviderMutation,
        MutatingSurface::NotebookResultMutation,
        MutatingSurface::SupportRepair,
    ] {
        if !covered_surfaces.contains(&required) {
            report.push(
                "packet.drilled_surface",
                format!("packet must drill mutating surface {}", required.as_str()),
            );
        }
    }
    if !has_publish_drill {
        report.push(
            "packet.publish_drill",
            "packet must drill at least one canonical publish to published truth",
        );
    }
    if !has_divergence_drill {
        report.push(
            "packet.divergence_drill",
            "packet must drill at least one honest divergence into a degraded or waiting state",
        );
    }
}

/// Validates one checked-in fixture against the frozen packet.
pub fn validate_reactive_command_parity_fixture(
    packet: &ReactiveCommandParityPacket,
    fixture: &ReactiveCommandParityFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != REACTIVE_COMMAND_PARITY_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != REACTIVE_COMMAND_PARITY_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }

    let rows: BTreeMap<_, _> = packet
        .flows
        .iter()
        .map(|row| (row.flow_id.as_str(), row))
        .collect();
    let row = match rows.get(fixture.expected_flow_id.as_str()) {
        Some(row) => *row,
        None => {
            report.push(
                "fixture.expected_flow_id",
                format!("fixture {} references an unknown flow", fixture.fixture_id),
            );
            return Err(report);
        }
    };

    if row.mutating_surface != fixture.mutating_surface {
        report.push(
            "fixture.mutating_surface",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.mutation_kind != fixture.mutation_kind {
        report.push(
            "fixture.mutation_kind",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.optimistic_posture != fixture.expected_optimistic_posture {
        report.push(
            "fixture.optimistic_posture",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.divergence_resolution != fixture.expected_divergence_resolution {
        report.push(
            "fixture.divergence_resolution",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.state_before_publish != fixture.expected_state_before_publish {
        report.push(
            "fixture.state_before_publish",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.claims_success_before_publish != fixture.expected_claims_success_before_publish {
        report.push(
            "fixture.claims_success_before_publish",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if !row
        .consumer_refs
        .iter()
        .any(|reference| reference == &fixture.consumer_ref)
    {
        report.push(
            "fixture.consumer_ref",
            format!(
                "fixture {} cites a consumer_ref not declared by flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

#[cfg(test)]
mod tests;
