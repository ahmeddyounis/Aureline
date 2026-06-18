//! Reusable dry-run / explain preview object and its first consumers.
//!
//! The automation contract baseline in
//! [`crate::m5_automation_contract_baseline`] froze *what* a dry-run / explain
//! preview is ([`DryRunExplainPacket`]) and the reused aggregate-outcome and
//! safety-label vocabularies every surface reads. This module makes the preview
//! concrete: a live [`DryRunExplainPreview`] that explains each step a claimed
//! automation would take as a **side-effect-bearing action** — it carries the
//! predicted writes the step would make, whether the step launches a process,
//! calls a network service, or mutates a remote target, the trust and policy
//! blockers in its way, the artifact destinations its output would land in, and
//! an idempotence hint — *before* anything is applied.
//!
//! The preview never asserts safety; it derives it. [`DryRunExplainPreview::dry_run_outcome_class`]
//! derives the aggregate outcome from the per-action blockers and the preview's
//! posture, and [`DryRunExplainPreview::aggregate_safety_labels`] derives the
//! recipe-wide label union from each action's side-effect class — so a mutating
//! action cannot read as safe merely because its preview is compact. The live
//! preview projects back onto the frozen [`DryRunExplainPacket`] through
//! [`DryRunExplainPreview::to_packet_record`] so every consumer reads the same
//! outcome truth, and [`DryRunExplainPreview::to_run_history_row`] and
//! [`DryRunExplainPreview::export`] carry the preview result into run history,
//! support export, and approval/evidence packets so the chosen automation path
//! stays attributable after the dialog closes.
//!
//! [`DryRunExplainFirstConsumersPacket`] binds the first M5 automation families
//! that support a preview — notebook, task/test/debug, request/API, package,
//! incident, and the AI assistant — each to a seeded preview, and
//! [`DryRunExplainFirstConsumersPacket::validate`] enforces the freeze
//! mechanically: every entrypoint binds a non-empty preview, every predicted
//! write is declared, no mutating action hides as read-only, every trust/policy
//! blocker stays visible, and the frozen projection stays consistent with the
//! live actions. A dropped entrypoint, an undeclared write, a mislabeled
//! read-only mutation, an inconsistent outcome or label projection, or a violated
//! invariant *blocks stable*.
//!
//! The reviewer-facing landing page is
//! [`/docs/m5/dry-run-and-explain.md`]; the cross-tool boundary schema is
//! [`/schemas/automation/dry-run-explain.schema.json`]; the reused frozen-packet
//! schema is [`/schemas/automation/recipe-builder.schema.json`].
//!
//! [`/docs/m5/dry-run-and-explain.md`]: ../../../docs/m5/dry-run-and-explain.md
//! [`/schemas/automation/dry-run-explain.schema.json`]: ../../../schemas/automation/dry-run-explain.schema.json
//! [`/schemas/automation/recipe-builder.schema.json`]: ../../../schemas/automation/recipe-builder.schema.json

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::m5_automation_contract_baseline::{
    AutomationBaselinePromotionState, AutomationSafetyLabelId, DryRunExplainPacket,
    DryRunOutcomeClass, DryRunStepExplanation, RECIPE_BUILDER_SCHEMA_REF,
    RUN_HISTORY_ROW_SCHEMA_REF, RUN_RECORD_SCHEMA_REF,
};
use crate::recipe_builder::RecipeBuilderEntrypoint;

/// Stable record-kind tag for [`DryRunExplainFirstConsumersPacket`].
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_RECORD_KIND: &str =
    "m5_dry_run_explain_first_consumers_packet";

/// Stable record-kind tag for [`DryRunExplainFirstConsumersSupportExport`].
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_dry_run_explain_first_consumers_support_export";

/// Stable record-kind tag for [`DryRunExplainFirstConsumersCliHeadlessView`].
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_dry_run_explain_first_consumers_cli_headless";

/// Stable record-kind tag for [`DryRunExplainExport`].
pub const DRY_RUN_EXPLAIN_EXPORT_RECORD_KIND: &str = "dry_run_explain_export_record";

/// Stable record-kind tag for [`DryRunPreviewRunHistoryRow`].
pub const DRY_RUN_PREVIEW_RUN_HISTORY_ROW_RECORD_KIND: &str = "dry_run_preview_run_history_row";

/// Stable record-kind tag the preview mints for the frozen packet projection.
///
/// Identical to the record kind frozen in the automation contract baseline, so
/// the projection is the same `dry_run_explain_packet_record` every surface reads.
pub const DRY_RUN_EXPLAIN_PACKET_RECORD_KIND: &str = "dry_run_explain_packet_record";

/// Integer schema version for the dry-run/explain first-consumers family.
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the first-consumers boundary schema.
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_REF: &str =
    "schemas/automation/dry-run-explain.schema.json";

/// Repo-relative path of the frozen dry-run/explain-packet boundary schema.
pub const DRY_RUN_EXPLAIN_PACKET_SCHEMA_REF: &str = RECIPE_BUILDER_SCHEMA_REF;

/// Repo-relative path of the reviewer contract doc for the dry-run/explain lane.
pub const DRY_RUN_EXPLAIN_DOC_REF: &str = "docs/m5/dry-run-and-explain.md";

/// Repo-relative path of the checked-in first-consumers packet artifact.
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/automation/dry-run-explain/packet.json";

/// Repo-relative root the worked-example side-effect-preview fixtures live under.
pub const DRY_RUN_EXPLAIN_FIXTURE_DIR: &str = "fixtures/automation/m5/side-effect-preview";

/// Stable packet id minted by the seed.
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_ID: &str =
    "automation:m5:dry-run-explain-first-consumers:v1";

/// Stable support-export id minted by the seed inspector.
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SUPPORT_EXPORT_ID: &str =
    "support-export:automation:m5:dry-run-explain-first-consumers";

/// Stable CLI/headless view id minted by the seed inspector.
pub const DRY_RUN_EXPLAIN_FIRST_CONSUMERS_CLI_HEADLESS_ID: &str =
    "cli-headless:automation:m5:dry-run-explain-first-consumers";

/// Posture token: the surface can mint a safe preview before apply.
pub const PREVIEW_POSTURE_SUPPORTED: &str = "preview_supported";

/// Posture token: no safe preview exists; apply needs a superseding approval.
pub const PREVIEW_POSTURE_NO_SAFE_PREVIEW: &str = "no_safe_preview";

/// Posture token: apply needs no approval ticket.
pub const APPROVAL_POSTURE_NONE: &str = "no_approval_required";

/// Posture token: apply needs an approval ticket first.
pub const APPROVAL_POSTURE_REQUIRED: &str = "approval_required_before_apply";

// ---------------------------------------------------------------------------
// Side-effect class
// ---------------------------------------------------------------------------

/// The side-effect class of one previewed action.
///
/// This is the explicit answer to "what does this step actually do" before any
/// apply. A [`SideEffectClass::ReadOnlyInspection`] makes no mutation; the other
/// variants each map to a frozen [`AutomationSafetyLabelId`], so a mutating step
/// always projects the matching safety label and cannot read as safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// The step reads or inspects state and mutates nothing.
    ReadOnlyInspection,
    /// The step writes files or buffers in the workspace or on the device.
    PredictedWrite,
    /// The step launches or controls a process.
    ProcessLaunch,
    /// The step performs a network call.
    NetworkCall,
    /// The step mutates remote state.
    RemoteMutation,
}

impl SideEffectClass {
    /// Every side-effect class in canonical order.
    pub const ALL: [SideEffectClass; 5] = [
        SideEffectClass::ReadOnlyInspection,
        SideEffectClass::PredictedWrite,
        SideEffectClass::ProcessLaunch,
        SideEffectClass::NetworkCall,
        SideEffectClass::RemoteMutation,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            SideEffectClass::ReadOnlyInspection => "read_only_inspection",
            SideEffectClass::PredictedWrite => "predicted_write",
            SideEffectClass::ProcessLaunch => "process_launch",
            SideEffectClass::NetworkCall => "network_call",
            SideEffectClass::RemoteMutation => "remote_mutation",
        }
    }

    /// Whether the class mutates state (anything but a read-only inspection).
    pub fn is_mutating(self) -> bool {
        !matches!(self, SideEffectClass::ReadOnlyInspection)
    }

    /// The frozen safety label this side-effect class projects, if any.
    ///
    /// Returns `None` only for [`SideEffectClass::ReadOnlyInspection`], which
    /// projects no side-effect label.
    pub fn safety_label(self) -> Option<AutomationSafetyLabelId> {
        Some(match self {
            SideEffectClass::ReadOnlyInspection => return None,
            SideEffectClass::PredictedWrite => AutomationSafetyLabelId::WritesFiles,
            SideEffectClass::ProcessLaunch => AutomationSafetyLabelId::RunsProcess,
            SideEffectClass::NetworkCall => AutomationSafetyLabelId::NetworkCall,
            SideEffectClass::RemoteMutation => AutomationSafetyLabelId::RemoteMutation,
        })
    }
}

// ---------------------------------------------------------------------------
// Predicted write
// ---------------------------------------------------------------------------

/// The kind of a predicted write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteKind {
    /// A new workspace file would be created.
    CreateFile,
    /// An existing workspace file would be modified in place.
    ModifyFile,
    /// A workspace file would be deleted.
    DeleteFile,
    /// Content would be appended to an existing file.
    AppendFile,
    /// An open editor buffer would be edited.
    BufferEdit,
    /// A change would be staged into the VCS index.
    StageVcs,
}

impl WriteKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            WriteKind::CreateFile => "create_file",
            WriteKind::ModifyFile => "modify_file",
            WriteKind::DeleteFile => "delete_file",
            WriteKind::AppendFile => "append_file",
            WriteKind::BufferEdit => "buffer_edit",
            WriteKind::StageVcs => "stage_vcs",
        }
    }
}

/// One write a previewed action would make before apply.
///
/// The target is an opaque, workspace-relative reference; a raw absolute path
/// never appears here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictedWrite {
    /// The kind of write.
    pub write_kind: WriteKind,
    /// Opaque, workspace-relative target reference; never a raw absolute path.
    pub target_ref: String,
    /// Whether the write is reversible (e.g. undo, unstage, restore).
    pub reversible: bool,
    /// Reviewable summary of the write (never the raw content).
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Artifact destination
// ---------------------------------------------------------------------------

/// Where an action's output artifact would land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactDestinationClass {
    /// A file inside the workspace.
    WorkspaceFile,
    /// A path on the local device outside the workspace.
    DeviceLocalPath,
    /// A remote target (host, container, or managed workspace).
    RemoteTarget,
    /// A network endpoint reached by an outbound call.
    NetworkEndpoint,
    /// An external package or artifact registry.
    ExternalRegistry,
    /// A local support / evidence bundle.
    SupportBundle,
}

impl ArtifactDestinationClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ArtifactDestinationClass::WorkspaceFile => "workspace_file",
            ArtifactDestinationClass::DeviceLocalPath => "device_local_path",
            ArtifactDestinationClass::RemoteTarget => "remote_target",
            ArtifactDestinationClass::NetworkEndpoint => "network_endpoint",
            ArtifactDestinationClass::ExternalRegistry => "external_registry",
            ArtifactDestinationClass::SupportBundle => "support_bundle",
        }
    }

    /// Whether landing an artifact here is itself a mutation of durable state.
    ///
    /// A read-only inspection may still produce a local support bundle, so that
    /// destination is not mutating; the workspace, device, remote, and registry
    /// destinations are.
    pub fn is_mutating(self) -> bool {
        matches!(
            self,
            ArtifactDestinationClass::WorkspaceFile
                | ArtifactDestinationClass::DeviceLocalPath
                | ArtifactDestinationClass::RemoteTarget
                | ArtifactDestinationClass::ExternalRegistry
        )
    }
}

/// One destination an action's output artifact would land in.
///
/// The destination is an opaque reference; a raw path, URL, or host never appears
/// here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDestination {
    /// The destination class.
    pub destination_class: ArtifactDestinationClass,
    /// Opaque destination reference; never a raw path, URL, or host.
    pub destination_ref: String,
    /// Reviewable summary of the destination.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Trust / policy blocker
// ---------------------------------------------------------------------------

/// The class of a trust or policy blocker in an action's way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerClass {
    /// The action is blocked at a trust boundary (untrusted host or target).
    TrustGate,
    /// The action is blocked by an admin or organization policy.
    PolicyGate,
    /// The action requires a capability the target did not negotiate.
    CapabilityGate,
    /// The action requires an approval ticket before apply.
    ApprovalRequiredGate,
    /// The action requires a credential that is not present.
    MissingCredentialGate,
}

impl BlockerClass {
    /// Every blocker class in canonical order.
    pub const ALL: [BlockerClass; 5] = [
        BlockerClass::TrustGate,
        BlockerClass::PolicyGate,
        BlockerClass::CapabilityGate,
        BlockerClass::ApprovalRequiredGate,
        BlockerClass::MissingCredentialGate,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            BlockerClass::TrustGate => "trust_gate",
            BlockerClass::PolicyGate => "policy_gate",
            BlockerClass::CapabilityGate => "capability_gate",
            BlockerClass::ApprovalRequiredGate => "approval_required_gate",
            BlockerClass::MissingCredentialGate => "missing_credential_gate",
        }
    }

    /// Whether a blocking gate of this class denies apply outright.
    ///
    /// An approval gate does not deny — it gates apply behind an approval ticket;
    /// every other class denies until the gate clears.
    pub fn denies_when_blocking(self) -> bool {
        !matches!(self, BlockerClass::ApprovalRequiredGate)
    }
}

/// One trust or policy blocker an action discloses before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustPolicyBlocker {
    /// The blocker class.
    pub blocker_class: BlockerClass,
    /// Whether the blocker currently blocks apply.
    pub blocking: bool,
    /// Opaque policy, ticket, or capability reference, or `null`.
    pub policy_ref: Option<String>,
    /// Reviewable summary of the blocker.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Idempotence
// ---------------------------------------------------------------------------

/// An idempotence hint for a previewed action.
///
/// A reviewer can always tell whether re-running the action is safe: idempotent
/// (repeats converge), idempotent only under an explicit key, not idempotent
/// (repeats compound), or unknown (treated as not idempotent for review).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdempotenceClass {
    /// Re-running converges to the same state.
    Idempotent,
    /// Re-running is idempotent only when an explicit idempotency key is reused.
    IdempotentWithKey,
    /// Re-running compounds the effect.
    NotIdempotent,
    /// The idempotence is unknown; reviewed as not idempotent.
    UnknownIdempotence,
}

impl IdempotenceClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            IdempotenceClass::Idempotent => "idempotent",
            IdempotenceClass::IdempotentWithKey => "idempotent_with_key",
            IdempotenceClass::NotIdempotent => "not_idempotent",
            IdempotenceClass::UnknownIdempotence => "unknown_idempotence",
        }
    }
}

// ---------------------------------------------------------------------------
// Previewed action
// ---------------------------------------------------------------------------

/// One side-effect-bearing action in a dry-run / explain preview.
///
/// An action is the unit of the preview: it names what the step would do, its
/// side-effect class, the writes it would make, the artifact destinations its
/// output would land in, the trust/policy blockers in its way, an idempotence
/// hint, and whether the effect is reversible. The projected safety labels and
/// the approval requirement are derived, never asserted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewedAction {
    /// Opaque step id.
    pub step_id: String,
    /// Dotted canonical verb the step invokes.
    pub canonical_verb: String,
    /// Plain-language explanation of what the step would do.
    pub explanation: String,
    /// The side-effect class of the step.
    pub side_effect_class: SideEffectClass,
    /// The writes the step would make before apply.
    pub predicted_writes: Vec<PredictedWrite>,
    /// The artifact destinations the step's output would land in.
    pub artifact_destinations: Vec<ArtifactDestination>,
    /// The trust/policy blockers the step discloses.
    pub trust_policy_blockers: Vec<TrustPolicyBlocker>,
    /// The idempotence hint for re-running the step.
    pub idempotence_class: IdempotenceClass,
    /// Whether the step's effect is reversible.
    pub reversible: bool,
    /// Capability declarations the step quotes.
    pub capability_declarations: Vec<String>,
    /// Reviewable blast-radius summary.
    pub blast_radius_summary: String,
}

impl PreviewedAction {
    /// The safety labels this action projects, derived from its side effect.
    ///
    /// The side-effect class contributes its label and a blocking approval gate
    /// contributes [`AutomationSafetyLabelId::ApprovalRequired`]. Because the
    /// labels are derived, a mutating action cannot drop its label and read as
    /// safe.
    pub fn projected_safety_labels(&self) -> Vec<AutomationSafetyLabelId> {
        let mut set: BTreeSet<AutomationSafetyLabelId> = BTreeSet::new();
        if let Some(label) = self.side_effect_class.safety_label() {
            set.insert(label);
        }
        if self.requires_approval() {
            set.insert(AutomationSafetyLabelId::ApprovalRequired);
        }
        canonical_label_order(&set)
    }

    /// Whether the action mutates state.
    pub fn is_mutating(&self) -> bool {
        self.side_effect_class.is_mutating()
    }

    /// Whether a blocking gate on this action denies apply outright.
    pub fn has_blocking_denial(&self) -> bool {
        self.trust_policy_blockers
            .iter()
            .any(|blocker| blocker.blocking && blocker.blocker_class.denies_when_blocking())
    }

    /// Whether the action requires an approval ticket before apply.
    pub fn requires_approval(&self) -> bool {
        self.trust_policy_blockers.iter().any(|blocker| {
            blocker.blocking && blocker.blocker_class == BlockerClass::ApprovalRequiredGate
        })
    }

    /// Whether any blocker on the action is currently blocking.
    pub fn has_blocking_blocker(&self) -> bool {
        self.trust_policy_blockers
            .iter()
            .any(|blocker| blocker.blocking)
    }

    /// Whether the declared side-effect class matches the declared effects.
    ///
    /// A read-only inspection must declare no write, stay reversible and
    /// idempotent, and name no mutating destination; a predicted write must
    /// declare at least one write. This is the guardrail that stops a mutating
    /// action from hiding as read-only behind a compact preview.
    pub fn side_effect_consistent(&self) -> bool {
        match self.side_effect_class {
            SideEffectClass::ReadOnlyInspection => {
                self.predicted_writes.is_empty()
                    && self.reversible
                    && self.idempotence_class == IdempotenceClass::Idempotent
                    && !self
                        .artifact_destinations
                        .iter()
                        .any(|destination| destination.destination_class.is_mutating())
                    && !self.has_blocking_blocker()
            }
            SideEffectClass::PredictedWrite => !self.predicted_writes.is_empty(),
            SideEffectClass::ProcessLaunch
            | SideEffectClass::NetworkCall
            | SideEffectClass::RemoteMutation => true,
        }
    }

    /// Projects this action onto the frozen [`DryRunStepExplanation`].
    pub fn to_step_explanation(&self) -> DryRunStepExplanation {
        DryRunStepExplanation {
            step_id: self.step_id.clone(),
            canonical_verb: self.canonical_verb.clone(),
            explanation: self.explanation.clone(),
            capability_declarations: self.capability_declarations.clone(),
            projected_safety_labels: self.projected_safety_labels(),
            reversible: self.reversible,
            blast_radius_summary: self.blast_radius_summary.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Dry-run / explain preview
// ---------------------------------------------------------------------------

/// An error raised by a [`DryRunExplainPreview`] mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DryRunExplainError {
    /// No action with the given step id is present.
    ActionNotFound(String),
    /// An action with the given step id is already present.
    DuplicateStepId(String),
}

impl std::fmt::Display for DryRunExplainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DryRunExplainError::ActionNotFound(id) => {
                write!(formatter, "no action with step id {id} is present")
            }
            DryRunExplainError::DuplicateStepId(id) => {
                write!(formatter, "an action with step id {id} is already present")
            }
        }
    }
}

impl std::error::Error for DryRunExplainError {}

/// The live, mutable dry-run / explain preview authoring object.
///
/// The preview owns the ordered list of [`PreviewedAction`]s; it derives the
/// aggregate outcome and safety-label union, and projects the frozen
/// [`DryRunExplainPacket`] on demand. It asserts no safety: every projection
/// reads back through the actions, so a consumer reviewing side effects reuses
/// the same outcome truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainPreview {
    /// Opaque preview id.
    pub preview_id: String,
    /// The M5 automation family this preview explains.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Opaque builder session id this preview belongs to.
    pub builder_id: String,
    /// Opaque draft recipe revision ref this preview explains.
    pub draft_recipe_revision_ref: String,
    /// Reviewable title.
    pub title: String,
    /// Reviewable summary sentence.
    pub summary: String,
    /// Whether the surface can mint a safe preview before apply.
    pub preview_posture_class: String,
    /// Whether apply needs an approval ticket first.
    pub approval_posture_class: String,
    /// Recipe-wide portability labels (recipe-safe, headless-safe, or ui-only).
    pub portability_labels: Vec<AutomationSafetyLabelId>,
    /// Ordered previewed actions.
    pub actions: Vec<PreviewedAction>,
    /// Monotonic mint timestamp.
    pub minted_at: String,
}

impl DryRunExplainPreview {
    /// Opens an empty preview for one entrypoint.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        entrypoint: RecipeBuilderEntrypoint,
        preview_id: impl Into<String>,
        builder_id: impl Into<String>,
        draft_recipe_revision_ref: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        preview_posture_class: impl Into<String>,
        approval_posture_class: impl Into<String>,
        portability_labels: Vec<AutomationSafetyLabelId>,
        minted_at: impl Into<String>,
    ) -> Self {
        DryRunExplainPreview {
            preview_id: preview_id.into(),
            entrypoint,
            builder_id: builder_id.into(),
            draft_recipe_revision_ref: draft_recipe_revision_ref.into(),
            title: title.into(),
            summary: summary.into(),
            preview_posture_class: preview_posture_class.into(),
            approval_posture_class: approval_posture_class.into(),
            portability_labels,
            actions: Vec::new(),
            minted_at: minted_at.into(),
        }
    }

    /// Appends a previewed action to the preview.
    pub fn add_action(&mut self, action: PreviewedAction) -> Result<(), DryRunExplainError> {
        if self.action(&action.step_id).is_some() {
            return Err(DryRunExplainError::DuplicateStepId(action.step_id.clone()));
        }
        self.actions.push(action);
        Ok(())
    }

    /// The action with the given step id, if present.
    pub fn action(&self, step_id: &str) -> Option<&PreviewedAction> {
        self.actions.iter().find(|action| action.step_id == step_id)
    }

    /// The aggregate dry-run outcome, derived from the actions and posture.
    ///
    /// A blocking denial gate dominates; then a missing safe preview; then a
    /// required approval; otherwise the recipe would apply.
    pub fn dry_run_outcome_class(&self) -> DryRunOutcomeClass {
        derive_outcome(
            &self.actions,
            &self.preview_posture_class,
            &self.approval_posture_class,
        )
    }

    /// The recipe-wide safety-label union, derived from portability and actions.
    pub fn aggregate_safety_labels(&self) -> Vec<AutomationSafetyLabelId> {
        derive_aggregate_labels(&self.actions, &self.portability_labels)
    }

    /// Count of actions that write files or buffers.
    pub fn predicted_write_count(&self) -> u32 {
        self.count_side_effect(SideEffectClass::PredictedWrite)
    }

    /// Count of actions that launch a process.
    pub fn process_launch_count(&self) -> u32 {
        self.count_side_effect(SideEffectClass::ProcessLaunch)
    }

    /// Count of actions that call a network service.
    pub fn network_call_count(&self) -> u32 {
        self.count_side_effect(SideEffectClass::NetworkCall)
    }

    /// Count of actions that mutate a remote target.
    pub fn remote_mutation_count(&self) -> u32 {
        self.count_side_effect(SideEffectClass::RemoteMutation)
    }

    fn count_side_effect(&self, class: SideEffectClass) -> u32 {
        self.actions
            .iter()
            .filter(|action| action.side_effect_class == class)
            .count() as u32
    }

    /// Count of actions with a currently-blocking trust/policy blocker.
    pub fn blocking_blocker_count(&self) -> u32 {
        self.actions
            .iter()
            .filter(|action| action.has_blocking_blocker())
            .count() as u32
    }

    /// Count of actions whose effect is irreversible.
    pub fn irreversible_action_count(&self) -> u32 {
        self.actions
            .iter()
            .filter(|action| !action.reversible)
            .count() as u32
    }

    /// Whether the recipe would apply cleanly with no gate.
    pub fn is_apply_ready(&self) -> bool {
        self.dry_run_outcome_class() == DryRunOutcomeClass::WouldApply
    }

    /// Whether every action's declared side effect matches its declared effects.
    pub fn every_action_side_effect_consistent(&self) -> bool {
        self.actions
            .iter()
            .all(PreviewedAction::side_effect_consistent)
    }

    /// Per-action side-effect-class tokens, index-aligned with the actions.
    pub fn side_effect_tokens(&self) -> Vec<String> {
        self.actions
            .iter()
            .map(|action| action.side_effect_class.as_str().to_owned())
            .collect()
    }

    /// Per-action idempotence-class tokens, index-aligned with the actions.
    pub fn idempotence_tokens(&self) -> Vec<String> {
        self.actions
            .iter()
            .map(|action| action.idempotence_class.as_str().to_owned())
            .collect()
    }

    /// Projects the live preview onto the frozen dry-run/explain-packet record.
    ///
    /// This is the proof the preview reuses outcome truth: the emitted packet
    /// quotes the same aggregate outcome, label union, and per-step explanation
    /// the preview derives.
    pub fn to_packet_record(&self) -> DryRunExplainPacket {
        DryRunExplainPacket {
            record_kind: DRY_RUN_EXPLAIN_PACKET_RECORD_KIND.to_owned(),
            recipe_builder_schema_version: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_VERSION,
            packet_id: self.preview_id.clone(),
            builder_id: self.builder_id.clone(),
            draft_recipe_revision_ref: self.draft_recipe_revision_ref.clone(),
            dry_run_outcome_class: self.dry_run_outcome_class(),
            aggregate_safety_labels: self.aggregate_safety_labels(),
            step_explanations: self
                .actions
                .iter()
                .map(PreviewedAction::to_step_explanation)
                .collect(),
            preview_posture_class: self.preview_posture_class.clone(),
            approval_posture_class: self.approval_posture_class.clone(),
            run_record_schema_ref: RUN_RECORD_SCHEMA_REF.to_owned(),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Projects the preview onto an attributable run-history row.
    ///
    /// The row carries the chosen automation path's outcome, label union,
    /// side-effect counts, and preview digest into run history so the preview
    /// result does not disappear after the dialog closes.
    pub fn to_run_history_row(
        &self,
        row_id: impl Into<String>,
        recorded_at: impl Into<String>,
    ) -> DryRunPreviewRunHistoryRow {
        DryRunPreviewRunHistoryRow {
            record_kind: DRY_RUN_PREVIEW_RUN_HISTORY_ROW_RECORD_KIND.to_owned(),
            schema_version: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_VERSION,
            row_id: row_id.into(),
            recorded_at: recorded_at.into(),
            preview_id: self.preview_id.clone(),
            entrypoint: self.entrypoint,
            builder_id: self.builder_id.clone(),
            draft_recipe_revision_ref: self.draft_recipe_revision_ref.clone(),
            dry_run_outcome_class: self.dry_run_outcome_class(),
            aggregate_safety_labels: self.aggregate_safety_labels(),
            predicted_write_count: self.predicted_write_count(),
            process_launch_count: self.process_launch_count(),
            network_call_count: self.network_call_count(),
            remote_mutation_count: self.remote_mutation_count(),
            blocking_blocker_count: self.blocking_blocker_count(),
            irreversible_action_count: self.irreversible_action_count(),
            preview_digest: self.preview_digest(),
            run_record_schema_ref: RUN_RECORD_SCHEMA_REF.to_owned(),
            run_history_row_schema_ref: RUN_HISTORY_ROW_SCHEMA_REF.to_owned(),
        }
    }

    /// Exports the preview, carrying its frozen packet and run-history row.
    pub fn export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DryRunExplainExport {
        let exported_at = exported_at.into();
        DryRunExplainExport {
            record_kind: DRY_RUN_EXPLAIN_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.clone(),
            packet_record: self.to_packet_record(),
            run_history_row: self
                .to_run_history_row(format!("run-history:{}", self.preview_id), exported_at),
            preview: self.clone(),
            export_digest: self.preview_digest(),
        }
    }

    /// Order-stable digest over the preview's actions and side-effect classes.
    pub fn preview_digest(&self) -> String {
        fnv1a64(&self.digest_tokens())
    }

    fn digest_tokens(&self) -> Vec<String> {
        let mut tokens = vec![self.preview_id.clone(), self.builder_id.clone()];
        for action in &self.actions {
            tokens.push(action.step_id.clone());
            tokens.push(action.side_effect_class.as_str().to_owned());
            tokens.push(action.idempotence_class.as_str().to_owned());
            for write in &action.predicted_writes {
                tokens.push(write.write_kind.as_str().to_owned());
                tokens.push(write.target_ref.clone());
            }
        }
        tokens
    }
}

/// An attributable run-history row carrying a dry-run / explain preview result.
///
/// The row is the bridge from a preview into run history, support export, and
/// approval/evidence packets: it keeps the outcome, label union, side-effect
/// counts, and preview digest so the chosen automation path stays attributable
/// after the preview dialog closes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunPreviewRunHistoryRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque run-history row id.
    pub row_id: String,
    /// Monotonic record timestamp.
    pub recorded_at: String,
    /// Opaque preview id this row records.
    pub preview_id: String,
    /// The entrypoint the preview explains.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Opaque builder session id.
    pub builder_id: String,
    /// Opaque draft recipe revision ref.
    pub draft_recipe_revision_ref: String,
    /// Aggregate dry-run outcome.
    pub dry_run_outcome_class: DryRunOutcomeClass,
    /// Safety-label union over the recipe.
    pub aggregate_safety_labels: Vec<AutomationSafetyLabelId>,
    /// Count of predicted-write actions.
    pub predicted_write_count: u32,
    /// Count of process-launch actions.
    pub process_launch_count: u32,
    /// Count of network-call actions.
    pub network_call_count: u32,
    /// Count of remote-mutation actions.
    pub remote_mutation_count: u32,
    /// Count of actions with a blocking blocker.
    pub blocking_blocker_count: u32,
    /// Count of irreversible actions.
    pub irreversible_action_count: u32,
    /// Order-stable preview digest carried for verification.
    pub preview_digest: String,
    /// Schema each dispatch mints a run record against.
    pub run_record_schema_ref: String,
    /// Schema this row conforms to in run history.
    pub run_history_row_schema_ref: String,
}

/// A preview exported for rerun review, sharing, or support bundles.
///
/// The export nests the whole [`DryRunExplainPreview`] verbatim — so each
/// action's side-effect class, predicted writes, artifact destinations, blockers,
/// and idempotence hint all survive — alongside the derived frozen-packet
/// projection, the run-history row, and an order-stable digest.
/// [`DryRunExplainExport::import`] reconstructs the identical preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// The frozen packet projection consumers read.
    pub packet_record: DryRunExplainPacket,
    /// The attributable run-history row the preview mints.
    pub run_history_row: DryRunPreviewRunHistoryRow,
    /// The preview, preserved verbatim for round-trip import.
    pub preview: DryRunExplainPreview,
    /// Order-stable digest over the preview's actions.
    pub export_digest: String,
}

impl DryRunExplainExport {
    /// Reconstructs the preview from the export.
    pub fn import(&self) -> DryRunExplainPreview {
        self.preview.clone()
    }

    /// Whether the export preserves side-effect truth across the boundary.
    ///
    /// Every action must declare a consistent side effect, the frozen packet must
    /// project one step per action with the same outcome, and the run-history row
    /// must carry the same outcome and digest — so the preview result survives
    /// export, history, and support without losing a side effect.
    pub fn side_effects_preserved(&self) -> bool {
        !self.preview.actions.is_empty()
            && self.preview.every_action_side_effect_consistent()
            && self.packet_record.step_explanations.len() == self.preview.actions.len()
            && self.packet_record.dry_run_outcome_class == self.preview.dry_run_outcome_class()
            && self.run_history_row.dry_run_outcome_class == self.preview.dry_run_outcome_class()
            && self.run_history_row.preview_digest == self.export_digest
    }
}

// ---------------------------------------------------------------------------
// First-consumer bindings
// ---------------------------------------------------------------------------

/// One entrypoint binding: the seeded preview a first consumer explains.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainConsumerBinding {
    /// The entrypoint this binding describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// Opaque preview id.
    pub preview_id: String,
    /// Opaque builder id the preview belongs to.
    pub builder_id: String,
    /// Whether the surface can mint a safe preview before apply.
    pub preview_posture_class: String,
    /// Whether apply needs an approval ticket first.
    pub approval_posture_class: String,
    /// Recipe-wide portability labels.
    pub portability_labels: Vec<AutomationSafetyLabelId>,
    /// The frozen packet record the consumer reuses.
    pub packet_record: DryRunExplainPacket,
    /// The live previewed actions, carrying the full side-effect dimensions.
    pub previewed_actions: Vec<PreviewedAction>,
    /// The attributable run-history row the preview mints.
    pub run_history_row: DryRunPreviewRunHistoryRow,
    /// Aggregate dry-run outcome.
    pub dry_run_outcome_class: DryRunOutcomeClass,
    /// Safety-label union over the recipe.
    pub aggregate_safety_labels: Vec<AutomationSafetyLabelId>,
    /// Action count, carried for compact projections.
    pub action_count: u32,
    /// Count of predicted-write actions.
    pub predicted_write_count: u32,
    /// Count of process-launch actions.
    pub process_launch_count: u32,
    /// Count of network-call actions.
    pub network_call_count: u32,
    /// Count of remote-mutation actions.
    pub remote_mutation_count: u32,
    /// Count of actions with a blocking blocker.
    pub blocking_blocker_count: u32,
    /// Reviewable summary of what the consumer previews.
    pub entry_summary: String,
}

impl DryRunExplainConsumerBinding {
    /// Builds a binding from a consumer's authored preview.
    pub fn from_preview(preview: &DryRunExplainPreview) -> Self {
        DryRunExplainConsumerBinding {
            entrypoint: preview.entrypoint,
            title: preview.entrypoint.title().to_owned(),
            preview_id: preview.preview_id.clone(),
            builder_id: preview.builder_id.clone(),
            preview_posture_class: preview.preview_posture_class.clone(),
            approval_posture_class: preview.approval_posture_class.clone(),
            portability_labels: preview.portability_labels.clone(),
            packet_record: preview.to_packet_record(),
            previewed_actions: preview.actions.clone(),
            run_history_row: preview.to_run_history_row(
                format!("run-history:{}", preview.preview_id),
                preview.minted_at.clone(),
            ),
            dry_run_outcome_class: preview.dry_run_outcome_class(),
            aggregate_safety_labels: preview.aggregate_safety_labels(),
            action_count: preview.actions.len() as u32,
            predicted_write_count: preview.predicted_write_count(),
            process_launch_count: preview.process_launch_count(),
            network_call_count: preview.network_call_count(),
            remote_mutation_count: preview.remote_mutation_count(),
            blocking_blocker_count: preview.blocking_blocker_count(),
            entry_summary: preview.summary.clone(),
        }
    }

    /// Recomputes the aggregate outcome from this binding's live actions.
    pub fn recomputed_outcome(&self) -> DryRunOutcomeClass {
        derive_outcome(
            &self.previewed_actions,
            &self.preview_posture_class,
            &self.approval_posture_class,
        )
    }

    /// Recomputes the safety-label union from this binding's live actions.
    pub fn recomputed_labels(&self) -> Vec<AutomationSafetyLabelId> {
        derive_aggregate_labels(&self.previewed_actions, &self.portability_labels)
    }
}

// ---------------------------------------------------------------------------
// Invariants and findings
// ---------------------------------------------------------------------------

/// Frozen invariants the first-consumers packet pins as schema-level constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainInvariantsBlock {
    /// Every first-consumer entrypoint binds a non-empty preview.
    pub every_entrypoint_binds_a_preview: bool,
    /// Predicted writes are declared before apply, never implied.
    pub predicted_writes_are_explicit_before_apply: bool,
    /// Process, network, and remote actions are labeled, never read as safe.
    pub process_network_remote_actions_are_labeled: bool,
    /// Trust and policy blockers stay visible before apply.
    pub trust_and_policy_blockers_are_visible: bool,
    /// Artifact destinations are named, never left implicit.
    pub artifact_destinations_are_named: bool,
    /// Idempotence hints are present for every action.
    pub idempotence_hints_are_present: bool,
    /// Outcomes and labels reuse the frozen vocabulary, not parallel ones.
    pub outcome_and_labels_reuse_the_frozen_vocabulary: bool,
    /// Preview results survive export, run history, and support.
    pub preview_survives_export_history_and_support: bool,
}

impl DryRunExplainInvariantsBlock {
    /// The frozen all-true invariants block.
    pub fn frozen() -> Self {
        DryRunExplainInvariantsBlock {
            every_entrypoint_binds_a_preview: true,
            predicted_writes_are_explicit_before_apply: true,
            process_network_remote_actions_are_labeled: true,
            trust_and_policy_blockers_are_visible: true,
            artifact_destinations_are_named: true,
            idempotence_hints_are_present: true,
            outcome_and_labels_reuse_the_frozen_vocabulary: true,
            preview_survives_export_history_and_support: true,
        }
    }

    /// Returns the `(name, value)` pairs in declaration order.
    pub fn entries(&self) -> [(&'static str, bool); 8] {
        [
            (
                "every_entrypoint_binds_a_preview",
                self.every_entrypoint_binds_a_preview,
            ),
            (
                "predicted_writes_are_explicit_before_apply",
                self.predicted_writes_are_explicit_before_apply,
            ),
            (
                "process_network_remote_actions_are_labeled",
                self.process_network_remote_actions_are_labeled,
            ),
            (
                "trust_and_policy_blockers_are_visible",
                self.trust_and_policy_blockers_are_visible,
            ),
            (
                "artifact_destinations_are_named",
                self.artifact_destinations_are_named,
            ),
            (
                "idempotence_hints_are_present",
                self.idempotence_hints_are_present,
            ),
            (
                "outcome_and_labels_reuse_the_frozen_vocabulary",
                self.outcome_and_labels_reuse_the_frozen_vocabulary,
            ),
            (
                "preview_survives_export_history_and_support",
                self.preview_survives_export_history_and_support,
            ),
        ]
    }
}

/// Severity of a dry-run/explain validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DryRunExplainFindingSeverity {
    /// Blocks the packet from stable.
    Blocker,
    /// Narrows the packet below stable.
    Warning,
}

/// Kind of a dry-run/explain validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DryRunExplainFindingKind {
    /// A required first-consumer entrypoint is absent.
    MissingEntrypoint,
    /// An entrypoint binds a preview with no actions.
    EntrypointPreviewEmpty,
    /// A predicted-write action declares no write.
    PredictedWriteNotDeclared,
    /// A mutating action is mislabeled as a read-only inspection.
    MutatingActionMislabeledReadOnly,
    /// The frozen packet outcome or steps disagree with the live actions.
    OutcomeProjectionInconsistent,
    /// The frozen packet safety labels disagree with the live actions.
    SafetyLabelProjectionInconsistent,
    /// A frozen invariant is set false.
    InvariantViolated,
}

impl DryRunExplainFindingKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            DryRunExplainFindingKind::MissingEntrypoint => "missing_entrypoint",
            DryRunExplainFindingKind::EntrypointPreviewEmpty => "entrypoint_preview_empty",
            DryRunExplainFindingKind::PredictedWriteNotDeclared => "predicted_write_not_declared",
            DryRunExplainFindingKind::MutatingActionMislabeledReadOnly => {
                "mutating_action_mislabeled_read_only"
            }
            DryRunExplainFindingKind::OutcomeProjectionInconsistent => {
                "outcome_projection_inconsistent"
            }
            DryRunExplainFindingKind::SafetyLabelProjectionInconsistent => {
                "safety_label_projection_inconsistent"
            }
            DryRunExplainFindingKind::InvariantViolated => "invariant_violated",
        }
    }
}

/// One blocking or warning finding raised by the first-consumers gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainFinding {
    /// The finding kind.
    pub finding_kind: DryRunExplainFindingKind,
    /// Whether the finding blocks stable or narrows below stable.
    pub severity: DryRunExplainFindingSeverity,
    /// Optional subject the finding is about.
    pub subject: Option<String>,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl DryRunExplainFinding {
    fn blocker(
        finding_kind: DryRunExplainFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        DryRunExplainFinding {
            finding_kind,
            severity: DryRunExplainFindingSeverity::Blocker,
            subject,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// First-consumers packet
// ---------------------------------------------------------------------------

/// Mutable input the seed mints and the materializer freezes into a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainFirstConsumersInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<DryRunExplainConsumerBinding>,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Frozen invariants block.
    pub invariants: DryRunExplainInvariantsBlock,
}

/// Canonical M5 dry-run/explain first-consumers packet.
///
/// The packet binds every first-consumer entrypoint to a seeded preview and pins
/// the freeze invariants. [`DryRunExplainFirstConsumersPacket::validate`]
/// recomputes the findings so the fail-closed gate and the typed consumer agree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainFirstConsumersPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Boundary schema ref for this packet.
    pub schema_ref: String,
    /// Reused frozen-packet boundary schema ref.
    pub packet_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<DryRunExplainConsumerBinding>,
    /// Frozen invariants block.
    pub invariants: DryRunExplainInvariantsBlock,
    /// Findings raised against this packet.
    pub validation_findings: Vec<DryRunExplainFinding>,
    /// Promotion state derived from the findings.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Order-invariant digest over entrypoint and step tokens.
    pub packet_digest: String,
}

impl DryRunExplainFirstConsumersPacket {
    /// Freezes an input into a packet, computing findings, promotion, and digest.
    pub fn materialize(input: DryRunExplainFirstConsumersInput) -> Self {
        let findings = validate_parts(&input.consumer_bindings, &input.invariants);
        let promotion_state = promotion_state_for_findings(&findings);
        let packet_digest = packet_digest(&input.consumer_bindings);
        DryRunExplainFirstConsumersPacket {
            record_kind: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            schema_ref: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
            packet_schema_ref: DRY_RUN_EXPLAIN_PACKET_SCHEMA_REF.to_owned(),
            doc_ref: DRY_RUN_EXPLAIN_DOC_REF.to_owned(),
            reused_contract_refs: input.reused_contract_refs,
            consumer_bindings: input.consumer_bindings,
            invariants: input.invariants,
            validation_findings: findings,
            promotion_state,
            packet_digest,
        }
    }

    /// Re-validates the materialized packet.
    pub fn validate(&self) -> Vec<DryRunExplainFinding> {
        validate_parts(&self.consumer_bindings, &self.invariants)
    }

    /// Whether the packet promotes to stable.
    pub fn is_stable(&self) -> bool {
        self.promotion_state == AutomationBaselinePromotionState::Stable
    }

    /// The binding for one entrypoint, if present.
    pub fn binding(
        &self,
        entrypoint: RecipeBuilderEntrypoint,
    ) -> Option<&DryRunExplainConsumerBinding> {
        self.consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
    }

    /// Entrypoint tokens in the order the packet stores them.
    pub fn entrypoint_tokens(&self) -> Vec<&'static str> {
        self.consumer_bindings
            .iter()
            .map(|binding| binding.entrypoint.as_str())
            .collect()
    }

    /// Builds the redacted support-export projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DryRunExplainFirstConsumersSupportExport {
        DryRunExplainFirstConsumersSupportExport {
            record_kind: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id: self.packet_id.clone(),
            packet_digest: self.packet_digest.clone(),
            promotion_state: self.promotion_state,
            consumer_rows: self
                .consumer_bindings
                .iter()
                .map(DryRunExplainSupportConsumerRow::from_binding)
                .collect(),
            run_history_rows: self
                .consumer_bindings
                .iter()
                .map(|binding| binding.run_history_row.clone())
                .collect(),
            invariants: self.invariants.clone(),
            finding_kinds: self
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind)
                .collect(),
        }
    }

    /// Builds the compact CLI / headless projection.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> DryRunExplainFirstConsumersCliHeadlessView {
        DryRunExplainFirstConsumersCliHeadlessView {
            record_kind: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id: self.packet_id.clone(),
            promotion_state: self.promotion_state,
            consumer_lines: self
                .consumer_bindings
                .iter()
                .map(|binding| {
                    format!(
                        "{} preview={} outcome={} writes={} process={} net={} remote={} blockers={}",
                        binding.entrypoint.as_str(),
                        binding.preview_id,
                        binding.dry_run_outcome_class.as_str(),
                        binding.predicted_write_count,
                        binding.process_launch_count,
                        binding.network_call_count,
                        binding.remote_mutation_count,
                        binding.blocking_blocker_count,
                    )
                })
                .collect(),
        }
    }

    /// Compact text projection lines for `compact.txt`.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "packet {} schema_version={} promotion={} consumers={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.consumer_bindings.len(),
            self.packet_digest,
        )];
        for binding in &self.consumer_bindings {
            lines.push(format!(
                "consumer {} preview={} outcome={} actions={} writes={} process={} net={} remote={} blockers={}",
                binding.entrypoint.as_str(),
                binding.preview_id,
                binding.dry_run_outcome_class.as_str(),
                binding.action_count,
                binding.predicted_write_count,
                binding.process_launch_count,
                binding.network_call_count,
                binding.remote_mutation_count,
                binding.blocking_blocker_count,
            ));
            for action in &binding.previewed_actions {
                lines.push(format!(
                    "  action {} verb={} effect={} idempotence={} reversible={} writes={} dests={} blocking={}",
                    action.step_id,
                    action.canonical_verb,
                    action.side_effect_class.as_str(),
                    action.idempotence_class.as_str(),
                    action.reversible,
                    action.predicted_writes.len(),
                    action.artifact_destinations.len(),
                    action.has_blocking_blocker(),
                ));
            }
        }
        lines
    }
}

/// One redacted support-export action row (no raw path, URL, or content).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainSupportActionRow {
    /// Opaque step id.
    pub step_id: String,
    /// Dotted canonical verb.
    pub canonical_verb: String,
    /// Side-effect class.
    pub side_effect_class: SideEffectClass,
    /// Idempotence hint.
    pub idempotence_class: IdempotenceClass,
    /// Whether the effect is reversible.
    pub reversible: bool,
    /// Count of predicted writes.
    pub predicted_write_count: u32,
    /// Artifact destination classes (no raw destinations).
    pub artifact_destination_classes: Vec<ArtifactDestinationClass>,
    /// Trust/policy blocker classes (no raw policy bodies).
    pub blocker_classes: Vec<BlockerClass>,
    /// Whether any blocker is currently blocking.
    pub has_blocking_blocker: bool,
    /// Derived projected safety labels.
    pub projected_safety_labels: Vec<AutomationSafetyLabelId>,
}

impl DryRunExplainSupportActionRow {
    fn from_action(action: &PreviewedAction) -> Self {
        DryRunExplainSupportActionRow {
            step_id: action.step_id.clone(),
            canonical_verb: action.canonical_verb.clone(),
            side_effect_class: action.side_effect_class,
            idempotence_class: action.idempotence_class,
            reversible: action.reversible,
            predicted_write_count: action.predicted_writes.len() as u32,
            artifact_destination_classes: action
                .artifact_destinations
                .iter()
                .map(|destination| destination.destination_class)
                .collect(),
            blocker_classes: action
                .trust_policy_blockers
                .iter()
                .map(|blocker| blocker.blocker_class)
                .collect(),
            has_blocking_blocker: action.has_blocking_blocker(),
            projected_safety_labels: action.projected_safety_labels(),
        }
    }
}

/// One redacted support-export consumer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainSupportConsumerRow {
    /// The entrypoint this row describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// Opaque preview id.
    pub preview_id: String,
    /// Aggregate dry-run outcome.
    pub dry_run_outcome_class: DryRunOutcomeClass,
    /// Safety-label union over the recipe.
    pub aggregate_safety_labels: Vec<AutomationSafetyLabelId>,
    /// Action count.
    pub action_count: u32,
    /// Count of predicted-write actions.
    pub predicted_write_count: u32,
    /// Count of process-launch actions.
    pub process_launch_count: u32,
    /// Count of network-call actions.
    pub network_call_count: u32,
    /// Count of remote-mutation actions.
    pub remote_mutation_count: u32,
    /// Count of actions with a blocking blocker.
    pub blocking_blocker_count: u32,
    /// Per-action redacted rows.
    pub action_rows: Vec<DryRunExplainSupportActionRow>,
}

impl DryRunExplainSupportConsumerRow {
    fn from_binding(binding: &DryRunExplainConsumerBinding) -> Self {
        DryRunExplainSupportConsumerRow {
            entrypoint: binding.entrypoint,
            title: binding.title.clone(),
            preview_id: binding.preview_id.clone(),
            dry_run_outcome_class: binding.dry_run_outcome_class,
            aggregate_safety_labels: binding.aggregate_safety_labels.clone(),
            action_count: binding.action_count,
            predicted_write_count: binding.predicted_write_count,
            process_launch_count: binding.process_launch_count,
            network_call_count: binding.network_call_count,
            remote_mutation_count: binding.remote_mutation_count,
            blocking_blocker_count: binding.blocking_blocker_count,
            action_rows: binding
                .previewed_actions
                .iter()
                .map(DryRunExplainSupportActionRow::from_action)
                .collect(),
        }
    }
}

/// Redacted support-export projection of the first-consumers packet.
///
/// The export carries the per-action side-effect classes and counts and the
/// attributable run-history rows, so a preview result is reviewable in a support
/// bundle without a raw path, URL, or content ever crossing the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainFirstConsumersSupportExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// Packet id this export was minted from.
    pub packet_id: String,
    /// Packet digest carried for verification.
    pub packet_digest: String,
    /// Promotion state of the source packet.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Consumer rows.
    pub consumer_rows: Vec<DryRunExplainSupportConsumerRow>,
    /// Attributable run-history rows carried for support review.
    pub run_history_rows: Vec<DryRunPreviewRunHistoryRow>,
    /// Frozen invariants block.
    pub invariants: DryRunExplainInvariantsBlock,
    /// Finding kinds carried for support review.
    pub finding_kinds: Vec<DryRunExplainFindingKind>,
}

impl DryRunExplainFirstConsumersSupportExport {
    /// Whether the export is safe to cross a tenant or surface boundary.
    pub fn is_export_safe(&self) -> bool {
        !self.packet_id.is_empty()
            && !self.packet_digest.is_empty()
            && !self.consumer_rows.is_empty()
            && self.run_history_rows.len() == self.consumer_rows.len()
    }
}

/// Compact CLI / headless projection of the first-consumers packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainFirstConsumersCliHeadlessView {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Monotonic generation timestamp.
    pub generated_at: String,
    /// Packet id this view was minted from.
    pub packet_id: String,
    /// Promotion state.
    pub promotion_state: AutomationBaselinePromotionState,
    /// One line per consumer entrypoint.
    pub consumer_lines: Vec<String>,
}

impl DryRunExplainFirstConsumersCliHeadlessView {
    /// Whether the view explains every entrypoint.
    pub fn every_entrypoint_explained(&self) -> bool {
        self.consumer_lines.len() == RecipeBuilderEntrypoint::ALL.len()
    }
}

// ---------------------------------------------------------------------------
// Derivations
// ---------------------------------------------------------------------------

/// Derives the aggregate dry-run outcome from actions and posture.
fn derive_outcome(
    actions: &[PreviewedAction],
    preview_posture_class: &str,
    approval_posture_class: &str,
) -> DryRunOutcomeClass {
    if actions.iter().any(PreviewedAction::has_blocking_denial) {
        return DryRunOutcomeClass::WouldBeDeniedAtGate;
    }
    if preview_posture_class == PREVIEW_POSTURE_NO_SAFE_PREVIEW {
        return DryRunOutcomeClass::NoSafePreview;
    }
    let needs_approval = approval_posture_class == APPROVAL_POSTURE_REQUIRED
        || actions.iter().any(PreviewedAction::requires_approval);
    if needs_approval {
        return DryRunOutcomeClass::WouldApplyUnderApproval;
    }
    DryRunOutcomeClass::WouldApply
}

/// Derives the recipe-wide safety-label union from actions and portability.
fn derive_aggregate_labels(
    actions: &[PreviewedAction],
    portability_labels: &[AutomationSafetyLabelId],
) -> Vec<AutomationSafetyLabelId> {
    let mut set: BTreeSet<AutomationSafetyLabelId> = portability_labels.iter().copied().collect();
    for action in actions {
        for label in action.projected_safety_labels() {
            set.insert(label);
        }
    }
    canonical_label_order(&set)
}

/// Returns the labels present in `set` in canonical [`AutomationSafetyLabelId::ALL`] order.
fn canonical_label_order(set: &BTreeSet<AutomationSafetyLabelId>) -> Vec<AutomationSafetyLabelId> {
    AutomationSafetyLabelId::ALL
        .into_iter()
        .filter(|label| set.contains(label))
        .collect()
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_parts(
    consumer_bindings: &[DryRunExplainConsumerBinding],
    invariants: &DryRunExplainInvariantsBlock,
) -> Vec<DryRunExplainFinding> {
    let mut findings = Vec::new();

    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let Some(binding) = consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
        else {
            findings.push(DryRunExplainFinding::blocker(
                DryRunExplainFindingKind::MissingEntrypoint,
                Some(entrypoint.as_str().to_owned()),
                format!(
                    "the {} entrypoint binds no dry-run/explain preview",
                    entrypoint.as_str()
                ),
            ));
            continue;
        };
        validate_binding(binding, &mut findings);
    }

    for (name, value) in invariants.entries() {
        if !value {
            findings.push(DryRunExplainFinding::blocker(
                DryRunExplainFindingKind::InvariantViolated,
                Some(name.to_owned()),
                format!("the invariant {name} is set false"),
            ));
        }
    }

    findings
}

fn validate_binding(
    binding: &DryRunExplainConsumerBinding,
    findings: &mut Vec<DryRunExplainFinding>,
) {
    let entrypoint = binding.entrypoint.as_str();
    let actions = &binding.previewed_actions;

    if actions.is_empty() {
        findings.push(DryRunExplainFinding::blocker(
            DryRunExplainFindingKind::EntrypointPreviewEmpty,
            Some(entrypoint.to_owned()),
            format!("the {entrypoint} entrypoint binds a preview with no actions"),
        ));
        return;
    }

    // The frozen packet must project one step per live action.
    if binding.packet_record.step_explanations.len() != actions.len() {
        findings.push(DryRunExplainFinding::blocker(
            DryRunExplainFindingKind::OutcomeProjectionInconsistent,
            Some(entrypoint.to_owned()),
            format!(
                "the {entrypoint} preview projects {} steps for {} actions",
                binding.packet_record.step_explanations.len(),
                actions.len()
            ),
        ));
    }

    // The frozen outcome must quote the recomputed outcome.
    let recomputed_outcome = binding.recomputed_outcome();
    if binding.packet_record.dry_run_outcome_class != recomputed_outcome
        || binding.dry_run_outcome_class != recomputed_outcome
    {
        findings.push(DryRunExplainFinding::blocker(
            DryRunExplainFindingKind::OutcomeProjectionInconsistent,
            Some(entrypoint.to_owned()),
            format!(
                "the {entrypoint} preview reports outcome {}, recomputed {}",
                binding.packet_record.dry_run_outcome_class.as_str(),
                recomputed_outcome.as_str()
            ),
        ));
    }

    // The frozen label union must quote the recomputed union.
    let recomputed_labels = binding.recomputed_labels();
    if binding.packet_record.aggregate_safety_labels != recomputed_labels
        || binding.aggregate_safety_labels != recomputed_labels
    {
        findings.push(DryRunExplainFinding::blocker(
            DryRunExplainFindingKind::SafetyLabelProjectionInconsistent,
            Some(entrypoint.to_owned()),
            format!("the {entrypoint} preview projects a safety-label union that disagrees with its actions"),
        ));
    }

    for (index, action) in actions.iter().enumerate() {
        let subject = format!("{entrypoint}:{}", action.step_id);

        // A predicted write must declare what it writes.
        if action.side_effect_class == SideEffectClass::PredictedWrite
            && action.predicted_writes.is_empty()
        {
            findings.push(DryRunExplainFinding::blocker(
                DryRunExplainFindingKind::PredictedWriteNotDeclared,
                Some(subject.clone()),
                format!(
                    "action {} on {entrypoint} is a predicted write but declares no write",
                    action.step_id
                ),
            ));
        }

        // A mutating action must not hide as a read-only inspection.
        if action.side_effect_class == SideEffectClass::ReadOnlyInspection
            && !action.side_effect_consistent()
        {
            findings.push(DryRunExplainFinding::blocker(
                DryRunExplainFindingKind::MutatingActionMislabeledReadOnly,
                Some(subject.clone()),
                format!(
                    "action {} on {entrypoint} is labeled read-only but declares a side effect",
                    action.step_id
                ),
            ));
        }

        // The frozen step must quote the same projection as the live action.
        if let Some(step) = binding.packet_record.step_explanations.get(index) {
            let expected = action.to_step_explanation();
            if step != &expected {
                findings.push(DryRunExplainFinding::blocker(
                    DryRunExplainFindingKind::OutcomeProjectionInconsistent,
                    Some(subject.clone()),
                    format!(
                        "the projected step for {} on {entrypoint} disagrees with the action",
                        action.step_id
                    ),
                ));
            }
        }
    }
}

fn promotion_state_for_findings(
    findings: &[DryRunExplainFinding],
) -> AutomationBaselinePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == DryRunExplainFindingSeverity::Blocker)
    {
        AutomationBaselinePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == DryRunExplainFindingSeverity::Warning)
    {
        AutomationBaselinePromotionState::NarrowedBelowStable
    } else {
        AutomationBaselinePromotionState::Stable
    }
}

fn packet_digest(consumer_bindings: &[DryRunExplainConsumerBinding]) -> String {
    let mut tokens: Vec<String> = Vec::new();
    for binding in consumer_bindings {
        tokens.push(binding.entrypoint.as_str().to_owned());
        for action in &binding.previewed_actions {
            tokens.push(action.step_id.clone());
        }
    }
    tokens.sort_unstable();
    fnv1a64(&tokens)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of strings.
fn fnv1a64(items_in_order: &[String]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for item in items_in_order {
        for byte in item.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

// ---------------------------------------------------------------------------
// Seeds
// ---------------------------------------------------------------------------

fn s(value: &str) -> String {
    value.to_owned()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn write(
    write_kind: WriteKind,
    target_ref: &str,
    reversible: bool,
    summary: &str,
) -> PredictedWrite {
    PredictedWrite {
        write_kind,
        target_ref: s(target_ref),
        reversible,
        summary: s(summary),
    }
}

fn dest(
    destination_class: ArtifactDestinationClass,
    destination_ref: &str,
    summary: &str,
) -> ArtifactDestination {
    ArtifactDestination {
        destination_class,
        destination_ref: s(destination_ref),
        summary: s(summary),
    }
}

fn blocker(
    blocker_class: BlockerClass,
    blocking: bool,
    policy_ref: Option<&str>,
    summary: &str,
) -> TrustPolicyBlocker {
    TrustPolicyBlocker {
        blocker_class,
        blocking,
        policy_ref: policy_ref.map(s),
        summary: s(summary),
    }
}

#[allow(clippy::too_many_arguments)]
fn action(
    step_id: &str,
    canonical_verb: &str,
    explanation: &str,
    side_effect_class: SideEffectClass,
    idempotence_class: IdempotenceClass,
    reversible: bool,
    predicted_writes: Vec<PredictedWrite>,
    artifact_destinations: Vec<ArtifactDestination>,
    trust_policy_blockers: Vec<TrustPolicyBlocker>,
    capability_declarations: &[&str],
    blast_radius_summary: &str,
) -> PreviewedAction {
    PreviewedAction {
        step_id: s(step_id),
        canonical_verb: s(canonical_verb),
        explanation: s(explanation),
        side_effect_class,
        predicted_writes,
        artifact_destinations,
        trust_policy_blockers,
        idempotence_class,
        reversible,
        capability_declarations: strings(capability_declarations),
        blast_radius_summary: s(blast_radius_summary),
    }
}

/// The two portability labels every seeded preview carries.
fn recipe_and_headless_safe() -> Vec<AutomationSafetyLabelId> {
    vec![
        AutomationSafetyLabelId::RecipeSafe,
        AutomationSafetyLabelId::HeadlessSafe,
    ]
}

/// Existing contracts the first-consumers packet reuses instead of re-deciding.
pub fn canonical_reused_contract_refs() -> Vec<String> {
    strings(&[
        DRY_RUN_EXPLAIN_PACKET_SCHEMA_REF,
        "schemas/automation/automation-contract-baseline.schema.json",
        "schemas/automation/recipe-builder-first-consumers.schema.json",
        "schemas/automation/parameter-review.schema.json",
        "schemas/automation/run_record.schema.json",
        "schemas/automation/run_history_row.schema.json",
        "schemas/commands/command_descriptor.schema.json",
        "docs/m5/recipe-builder-and-macro-contract.md",
        "docs/automation/preview-and-lifecycle.md",
    ])
}

/// Builds the seeded preview one first consumer explains.
pub fn seeded_consumer_preview(entrypoint: RecipeBuilderEntrypoint) -> DryRunExplainPreview {
    use ArtifactDestinationClass::{
        ExternalRegistry, NetworkEndpoint, RemoteTarget, SupportBundle, WorkspaceFile,
    };
    use BlockerClass::{ApprovalRequiredGate, TrustGate};
    use IdempotenceClass::{Idempotent, IdempotentWithKey, NotIdempotent};
    use SideEffectClass::{
        NetworkCall, PredictedWrite, ProcessLaunch, ReadOnlyInspection, RemoteMutation,
    };
    use WriteKind::{CreateFile, ModifyFile, StageVcs};

    match entrypoint {
        RecipeBuilderEntrypoint::Notebook => {
            let mut preview = DryRunExplainPreview::new(
                entrypoint,
                "dry-run:notebook-run-and-export:v1",
                "builder:notebook:run-and-export:v1",
                "recipe-rev:notebook-run-and-export:1",
                "Preview notebook run-and-export side effects",
                "Explains the cells the run would execute and the export file it would write before the notebook runs.",
                PREVIEW_POSTURE_SUPPORTED,
                APPROVAL_POSTURE_NONE,
                recipe_and_headless_safe(),
                "2026-06-18T00:00:00Z",
            );
            preview
                .add_action(action(
                    "step:run-cells",
                    "notebook.run_all_cells",
                    "Runs every cell against the workspace kernel; mutates only in-memory kernel state.",
                    ReadOnlyInspection,
                    Idempotent,
                    true,
                    vec![],
                    vec![dest(
                        SupportBundle,
                        "artifact:notebook-run-log",
                        "a local run log captured for review",
                    )],
                    vec![],
                    &["read_only_kernel_execution"],
                    "the workspace kernel session only",
                ))
                .expect("add run-cells");
            preview
                .add_action(action(
                    "step:write-export",
                    "notebook.export_rendered",
                    "Writes the rendered notebook export to the workspace output directory.",
                    PredictedWrite,
                    Idempotent,
                    true,
                    vec![write(
                        CreateFile,
                        "path:exports/notebook.html",
                        true,
                        "creates the rendered export file",
                    )],
                    vec![dest(
                        WorkspaceFile,
                        "path:exports/",
                        "the workspace export directory",
                    )],
                    vec![],
                    &["reversible_workspace_filesystem_mutation"],
                    "the workspace export directory only",
                ))
                .expect("add write-export");
            preview
        }
        RecipeBuilderEntrypoint::TaskTestDebug => {
            let mut preview = DryRunExplainPreview::new(
                entrypoint,
                "dry-run:run-tests-and-report:v1",
                "builder:task:run-tests-and-report:v1",
                "recipe-rev:run-tests-and-report:1",
                "Preview test run side effects",
                "Explains the test process it would launch and the coverage report it would write before the run.",
                PREVIEW_POSTURE_SUPPORTED,
                APPROVAL_POSTURE_NONE,
                recipe_and_headless_safe(),
                "2026-06-18T00:00:00Z",
            );
            preview
                .add_action(action(
                    "step:launch-tests",
                    "test.run_selected",
                    "Launches the selected test process against the local toolchain.",
                    ProcessLaunch,
                    Idempotent,
                    true,
                    vec![],
                    vec![dest(
                        SupportBundle,
                        "artifact:test-run-events",
                        "a local test-event stream captured for review",
                    )],
                    vec![],
                    &["local_process_launch"],
                    "a local test process on the workspace toolchain",
                ))
                .expect("add launch-tests");
            preview
                .add_action(action(
                    "step:write-coverage",
                    "coverage.write_report",
                    "Writes the coverage report into the workspace coverage directory.",
                    PredictedWrite,
                    Idempotent,
                    true,
                    vec![write(
                        ModifyFile,
                        "path:coverage/report.json",
                        true,
                        "overwrites the coverage report",
                    )],
                    vec![dest(
                        WorkspaceFile,
                        "path:coverage/",
                        "the workspace coverage directory",
                    )],
                    vec![],
                    &["reversible_workspace_filesystem_mutation"],
                    "the workspace coverage directory only",
                ))
                .expect("add write-coverage");
            preview
        }
        RecipeBuilderEntrypoint::RequestApi => {
            let mut preview = DryRunExplainPreview::new(
                entrypoint,
                "dry-run:send-request-and-save:v1",
                "builder:request:send-and-save:v1",
                "recipe-rev:send-request-and-save:1",
                "Preview request send side effects",
                "Explains the outbound call it would make and the saved response it would write; apply needs approval.",
                PREVIEW_POSTURE_SUPPORTED,
                APPROVAL_POSTURE_REQUIRED,
                recipe_and_headless_safe(),
                "2026-06-18T00:00:00Z",
            );
            preview
                .add_action(action(
                    "step:send-request",
                    "request.send",
                    "Sends the configured request to the resolved environment endpoint.",
                    NetworkCall,
                    NotIdempotent,
                    false,
                    vec![],
                    vec![dest(
                        NetworkEndpoint,
                        "endpoint:env-profile-resolved",
                        "the resolved environment endpoint",
                    )],
                    vec![blocker(
                        ApprovalRequiredGate,
                        true,
                        Some("approval:request-send"),
                        "sending to a non-local endpoint requires an approval ticket",
                    )],
                    &["outbound_network_call"],
                    "one outbound request to the resolved endpoint",
                ))
                .expect("add send-request");
            preview
                .add_action(action(
                    "step:save-response",
                    "request.save_response",
                    "Writes the captured response body to the workspace responses directory.",
                    PredictedWrite,
                    Idempotent,
                    true,
                    vec![write(
                        CreateFile,
                        "path:responses/latest.json",
                        true,
                        "creates the saved response file",
                    )],
                    vec![dest(
                        WorkspaceFile,
                        "path:responses/",
                        "the workspace responses directory",
                    )],
                    vec![],
                    &["reversible_workspace_filesystem_mutation"],
                    "the workspace responses directory only",
                ))
                .expect("add save-response");
            preview
        }
        RecipeBuilderEntrypoint::Package => {
            let mut preview = DryRunExplainPreview::new(
                entrypoint,
                "dry-run:update-and-publish:v1",
                "builder:package:update-and-publish:v1",
                "recipe-rev:update-and-publish:1",
                "Preview package update and publish side effects",
                "Explains the lockfile write, registry call, and publish it would make; apply needs approval.",
                PREVIEW_POSTURE_SUPPORTED,
                APPROVAL_POSTURE_REQUIRED,
                recipe_and_headless_safe(),
                "2026-06-18T00:00:00Z",
            );
            preview
                .add_action(action(
                    "step:resolve-update",
                    "package.resolve_update",
                    "Resolves the update against the registry and writes the lockfile.",
                    PredictedWrite,
                    Idempotent,
                    true,
                    vec![write(
                        ModifyFile,
                        "path:lockfile",
                        true,
                        "rewrites the dependency lockfile",
                    )],
                    vec![dest(
                        WorkspaceFile,
                        "path:lockfile",
                        "the workspace dependency lockfile",
                    )],
                    vec![],
                    &["reversible_workspace_filesystem_mutation"],
                    "the workspace lockfile only",
                ))
                .expect("add resolve-update");
            preview
                .add_action(action(
                    "step:publish",
                    "package.publish",
                    "Publishes the built package to the external registry.",
                    RemoteMutation,
                    IdempotentWithKey,
                    false,
                    vec![],
                    vec![dest(
                        ExternalRegistry,
                        "registry:configured-publish-target",
                        "the configured external package registry",
                    )],
                    vec![blocker(
                        ApprovalRequiredGate,
                        true,
                        Some("approval:package-publish"),
                        "publishing to an external registry requires an approval ticket",
                    )],
                    &["external_registry_publish"],
                    "one published artifact on the external registry",
                ))
                .expect("add publish");
            preview
        }
        RecipeBuilderEntrypoint::Incident => {
            let mut preview = DryRunExplainPreview::new(
                entrypoint,
                "dry-run:run-remote-runbook:v1",
                "builder:incident:run-remote-runbook:v1",
                "recipe-rev:run-remote-runbook:1",
                "Preview incident runbook side effects",
                "Explains the bundle it would write and the remote runbook action it would attempt; the remote action is denied at a trust gate.",
                PREVIEW_POSTURE_SUPPORTED,
                APPROVAL_POSTURE_NONE,
                recipe_and_headless_safe(),
                "2026-06-18T00:00:00Z",
            );
            preview
                .add_action(action(
                    "step:write-bundle",
                    "incident.write_bundle",
                    "Writes the incident evidence bundle to the local support folder.",
                    PredictedWrite,
                    Idempotent,
                    true,
                    vec![write(
                        CreateFile,
                        "path:support/incident-bundle.json",
                        true,
                        "creates the incident evidence bundle",
                    )],
                    vec![dest(
                        SupportBundle,
                        "artifact:incident-bundle",
                        "the local incident evidence bundle",
                    )],
                    vec![],
                    &["reversible_workspace_filesystem_mutation"],
                    "the local support folder only",
                ))
                .expect("add write-bundle");
            preview
                .add_action(action(
                    "step:remote-runbook",
                    "incident.run_remote_runbook",
                    "Attempts the runbook action on the remote target; blocked at the trust gate.",
                    RemoteMutation,
                    NotIdempotent,
                    false,
                    vec![],
                    vec![dest(
                        RemoteTarget,
                        "remote:incident-target",
                        "the remote incident target",
                    )],
                    vec![blocker(
                        TrustGate,
                        true,
                        Some("policy:remote-runbook-trust"),
                        "the remote target is untrusted, so the runbook action is denied",
                    )],
                    &["remote_target_mutation"],
                    "one runbook action on the remote target",
                ))
                .expect("add remote-runbook");
            preview
        }
        RecipeBuilderEntrypoint::AiAssistant => {
            let mut preview = DryRunExplainPreview::new(
                entrypoint,
                "dry-run:apply-proposed-fix:v1",
                "builder:ai:apply-proposed-fix:v1",
                "recipe-rev:apply-proposed-fix:1",
                "Preview AI-proposed fix side effects",
                "Explains the edits the proposed fix would apply and stage; apply needs approval.",
                PREVIEW_POSTURE_SUPPORTED,
                APPROVAL_POSTURE_REQUIRED,
                recipe_and_headless_safe(),
                "2026-06-18T00:00:00Z",
            );
            preview
                .add_action(action(
                    "step:apply-fix",
                    "ai.apply_proposed_edits",
                    "Applies the AI-proposed edits to the changed files in the workspace.",
                    PredictedWrite,
                    Idempotent,
                    true,
                    vec![write(
                        ModifyFile,
                        "path:changed-files",
                        true,
                        "modifies the changed files in place",
                    )],
                    vec![dest(
                        WorkspaceFile,
                        "path:changed-files",
                        "the changed files in the workspace",
                    )],
                    vec![blocker(
                        ApprovalRequiredGate,
                        true,
                        Some("approval:ai-apply-fix"),
                        "applying an AI-proposed fix requires an approval ticket",
                    )],
                    &["reversible_workspace_filesystem_mutation"],
                    "the changed files in the current workspace only",
                ))
                .expect("add apply-fix");
            preview
                .add_action(action(
                    "step:stage-fix",
                    "vcs.stage_changed",
                    "Stages the applied edits; staging is reversible with unstage.",
                    PredictedWrite,
                    Idempotent,
                    true,
                    vec![write(
                        StageVcs,
                        "path:vcs-index",
                        true,
                        "stages the changed files into the VCS index",
                    )],
                    vec![dest(
                        WorkspaceFile,
                        "path:vcs-index",
                        "the workspace VCS index",
                    )],
                    vec![],
                    &["reversible_workspace_filesystem_mutation"],
                    "the workspace VCS index only",
                ))
                .expect("add stage-fix");
            preview
        }
    }
}

/// Builds the canonical stable first-consumers input.
pub fn current_dry_run_explain_first_consumers_input() -> DryRunExplainFirstConsumersInput {
    let consumer_bindings = RecipeBuilderEntrypoint::ALL
        .into_iter()
        .map(|entrypoint| {
            DryRunExplainConsumerBinding::from_preview(&seeded_consumer_preview(entrypoint))
        })
        .collect();
    DryRunExplainFirstConsumersInput {
        packet_id: DRY_RUN_EXPLAIN_FIRST_CONSUMERS_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        consumer_bindings,
        reused_contract_refs: canonical_reused_contract_refs(),
        invariants: DryRunExplainInvariantsBlock::frozen(),
    }
}

/// Materializes the canonical stable first-consumers packet.
pub fn seeded_dry_run_explain_first_consumers_packet() -> DryRunExplainFirstConsumersPacket {
    DryRunExplainFirstConsumersPacket::materialize(current_dry_run_explain_first_consumers_input())
}

/// Validates a packet, returning `Ok(())` or the findings.
pub fn validate_dry_run_explain_first_consumers_packet(
    packet: &DryRunExplainFirstConsumersPacket,
) -> Result<(), Vec<DryRunExplainFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Worked example: the notebook preview exported for round-trip review.
///
/// The notebook preview mixes a read-only inspection with a reversible workspace
/// write, so the round-trip proves the side-effect truth survives export.
pub fn seeded_dry_run_explain_export_roundtrip() -> DryRunExplainExport {
    seeded_consumer_preview(RecipeBuilderEntrypoint::Notebook)
        .export("export:notebook-run-and-export:v1", "2026-06-18T00:01:00Z")
}

/// Worked example: the incident preview whose remote action is denied at a gate.
pub fn seeded_blocked_preview() -> DryRunExplainPreview {
    seeded_consumer_preview(RecipeBuilderEntrypoint::Incident)
}
