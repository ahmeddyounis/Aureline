//! Cross-surface beta projection of the three scaffold records.
//!
//! [`ScaffoldSafetyBetaProjection`] is the single record Start Center
//! starter rows, the command palette, the generator-preview sheet,
//! AI-assisted generation, extension-provided generation, the CLI /
//! headless creation path, and support exporters read so they all agree,
//! **before any file is written**, on:
//!
//! - **what is being generated** (the signed descriptor identity,
//!   provider, signature state, generation kind, supported ecosystems /
//!   archetypes, and generation verb quoted from the
//!   [`TemplateGeneratorDescriptor`] and [`ScaffoldPlanRecord`]),
//! - **what the scaffold plan will do** (the target scope, resolved
//!   parameter sources, file / directory impact, dependency and task
//!   plan, and remote / bootstrap implications),
//! - **which side effects are declared before execution** (the closed
//!   hook / network / registry / remote-image / dependency side-effect
//!   set, each declared before execution and attributable after rollback),
//! - **which create-empty / set-up-later / rollback handoff paths are
//!   visible** (the [`SetupChoiceClass`] list and the rollback boundary),
//!   and
//! - **whether the run kept generated output as plain workspace content**
//!   (the optional run outcome plus the guardrails that block undeclared
//!   hooks and hidden project databases).
//!
//! The projection never invents new vocabulary; it only quotes the three
//! boundary records and adds typed predicates the surface contract
//! requires.

use serde::{Deserialize, Serialize};

use super::descriptors::{
    DescriptorSignatureState, EgressPostureClass, GenerationKindClass, GenerationVerb,
    RollbackBoundaryClass, ScaffoldOutcomeClass, ScaffoldPlanRecord, ScaffoldRunRecord,
    ScaffoldSideEffectClass, SetupChoiceClass, TemplateGeneratorDescriptor, TemplateProviderClass,
    TrustExpectationClass,
};
use super::shared::ScaffoldSurface;

/// Schema version for [`ScaffoldSafetyBetaProjection`].
pub const SCAFFOLD_SAFETY_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for the projection.
pub const SCAFFOLD_SAFETY_RECORD_KIND: &str = "scaffold_safety_record";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldSafetyRecordKind {
    ScaffoldSafetyRecord,
}

/// Closed honesty-label set the surface renders verbatim alongside a
/// scaffold / generation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldHonestyLabel {
    UnsignedTemplate,
    SignatureMismatch,
    AiAssistedGeneration,
    ExtensionProvidedGeneration,
    HooksDeclared,
    NetworkEgressDeclared,
    RegistryAccessDeclared,
    RemoteImagePullDeclared,
    DependencyRestoreDeclared,
    WritesIntoExistingProject,
    SetUpLaterAvailable,
    CreateEmptyAvailable,
    PolicyConstrained,
    PartialRollback,
}

impl ScaffoldHonestyLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsignedTemplate => "unsigned_template",
            Self::SignatureMismatch => "signature_mismatch",
            Self::AiAssistedGeneration => "ai_assisted_generation",
            Self::ExtensionProvidedGeneration => "extension_provided_generation",
            Self::HooksDeclared => "hooks_declared",
            Self::NetworkEgressDeclared => "network_egress_declared",
            Self::RegistryAccessDeclared => "registry_access_declared",
            Self::RemoteImagePullDeclared => "remote_image_pull_declared",
            Self::DependencyRestoreDeclared => "dependency_restore_declared",
            Self::WritesIntoExistingProject => "writes_into_existing_project",
            Self::SetUpLaterAvailable => "set_up_later_available",
            Self::CreateEmptyAvailable => "create_empty_available",
            Self::PolicyConstrained => "policy_constrained",
            Self::PartialRollback => "partial_rollback",
        }
    }
}

/// Side-effect summary the projection discloses before execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredSideEffectSummary {
    /// Union of declared side-effect classes (hook / network / registry /
    /// remote-image / dependency).
    pub classes: Vec<ScaffoldSideEffectClass>,
    /// True when every declared side effect is declared before execution.
    pub all_declared_before_execution: bool,
    /// True when every declared side effect remains attributable after a
    /// failure or rollback.
    pub all_attributable_after_rollback: bool,
}

/// Setup-handoff summary the projection discloses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupHandoffSummary {
    /// The explicit setup choices the plan exposes.
    pub choices: Vec<SetupChoiceClass>,
    /// True when a create-empty choice is offered.
    pub create_empty_available: bool,
    /// True when a set-up-later choice is offered.
    pub set_up_later_available: bool,
    /// The rollback boundary class the plan plants before any write.
    pub rollback_boundary: RollbackBoundaryClass,
    /// True when the rollback boundary is automatic (no manual file delete).
    pub rollback_automatic: bool,
}

/// Optional run summary present once a scaffold run has executed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldRunSummary {
    pub scaffold_run_ref: String,
    pub outcome_class: ScaffoldOutcomeClass,
    pub created_artifact_count: u64,
    pub modified_artifact_count: u64,
    /// True when the run only invoked hooks / tasks declared on the
    /// descriptor and blocked undeclared actions.
    pub undeclared_actions_blocked: bool,
    /// True when generated output stayed plain workspace content (no hidden
    /// project database is authoritative).
    pub plain_file_authority: bool,
    pub no_hidden_project_database: bool,
    /// Opaque ref to the plain-file generated-project lineage metadata.
    pub generated_lineage_ref: String,
}

/// Typed guardrail predicates the projection guarantees. Each maps to a
/// guardrail or acceptance criterion in the scaffold-safety spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldSafetyGuardrails {
    /// No write happens before the user can review a scaffold plan or its
    /// exported preflight summary.
    pub no_writes_before_review: bool,
    /// Every hook / network / registry / remote-image / dependency side
    /// effect is declared before execution.
    pub side_effects_declared_before_execution: bool,
    /// Side effects remain attributable to the plan / run after failure or
    /// rollback.
    pub side_effects_attributable_after_rollback: bool,
    /// AI-assisted or extension-provided generation cannot invent
    /// undeclared hooks or hidden bootstrap steps; the run invokes only
    /// declared hooks / tasks and blocks undeclared actions.
    pub no_undeclared_hooks_or_bootstrap: bool,
    /// Generated output remains plain, attributable workspace content: no
    /// hidden project database is the authoritative result of generation.
    pub generated_output_is_plain_workspace_content: bool,
    /// A rollback / delete-generated-files boundary is visible and the run's
    /// rollback state is attributable.
    pub rollback_boundary_visible: bool,
    /// AI / extension generation reuses the same governed scaffold-plan /
    /// diff-review surface rather than IDE-only authority.
    pub ai_extension_uses_governed_surface: bool,
}

impl ScaffoldSafetyGuardrails {
    /// True when every guardrail holds.
    pub const fn all_hold(self) -> bool {
        self.no_writes_before_review
            && self.side_effects_declared_before_execution
            && self.side_effects_attributable_after_rollback
            && self.no_undeclared_hooks_or_bootstrap
            && self.generated_output_is_plain_workspace_content
            && self.rollback_boundary_visible
            && self.ai_extension_uses_governed_surface
    }
}

/// Inputs used to assemble a beta projection.
#[derive(Debug, Clone)]
pub struct ScaffoldSafetyBetaInputs<'a> {
    pub descriptor: &'a TemplateGeneratorDescriptor,
    pub plan: &'a ScaffoldPlanRecord,
    /// The run is `None` before generation (preflight only) and `Some`
    /// after a scaffold run has executed.
    pub run: Option<&'a ScaffoldRunRecord>,
    pub surface: ScaffoldSurface,
}

/// Errors returned while assembling a beta projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldSafetyBetaError {
    /// The plan does not reference the supplied descriptor.
    PlanDescriptorMismatch {
        expected_descriptor_id: String,
        observed: String,
    },
    /// The run does not reference the supplied plan.
    RunPlanMismatch {
        expected_scaffold_plan_id: String,
        observed: String,
    },
    /// The run does not reference the supplied descriptor.
    RunDescriptorMismatch {
        expected_descriptor_id: String,
        observed: String,
    },
    /// The plan plans a task it claims is declared in the descriptor, but no
    /// such hook / task exists there.
    UndeclaredTaskPlanned { task_id: String },
    /// The run invoked a hook the descriptor never declared.
    UndeclaredHookInvoked { hook_id: String },
    /// The run invoked a validation task the descriptor never declared.
    UndeclaredTaskInvoked { task_id: String },
}

impl std::fmt::Display for ScaffoldSafetyBetaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlanDescriptorMismatch {
                expected_descriptor_id,
                observed,
            } => write!(
                f,
                "scaffold plan references descriptor {observed}, expected {expected_descriptor_id}"
            ),
            Self::RunPlanMismatch {
                expected_scaffold_plan_id,
                observed,
            } => write!(
                f,
                "scaffold run references plan {observed}, expected {expected_scaffold_plan_id}"
            ),
            Self::RunDescriptorMismatch {
                expected_descriptor_id,
                observed,
            } => write!(
                f,
                "scaffold run references descriptor {observed}, expected {expected_descriptor_id}"
            ),
            Self::UndeclaredTaskPlanned { task_id } => write!(
                f,
                "scaffold plan plans task {task_id} as declared, but it is not on the descriptor"
            ),
            Self::UndeclaredHookInvoked { hook_id } => {
                write!(f, "scaffold run invoked undeclared hook {hook_id}")
            }
            Self::UndeclaredTaskInvoked { task_id } => {
                write!(f, "scaffold run invoked undeclared task {task_id}")
            }
        }
    }
}

impl std::error::Error for ScaffoldSafetyBetaError {}

/// Beta truth one scaffold surface reads so it agrees with every other
/// surface about template provenance, scaffold-plan impact, declared side
/// effects, setup / rollback handoffs, and generated-project health before
/// any file is written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldSafetyBetaProjection {
    pub record_kind: ScaffoldSafetyRecordKind,
    pub scaffold_safety_schema_version: u32,
    pub surface: ScaffoldSurface,

    pub descriptor_ref: String,
    pub scaffold_plan_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaffold_run_ref: Option<String>,

    pub template_or_generator_id: String,
    pub version: String,
    pub provider_class: TemplateProviderClass,
    pub signature_state: DescriptorSignatureState,
    pub generation_kind: GenerationKindClass,
    pub generation_verb: GenerationVerb,
    pub trust_expectation: TrustExpectationClass,
    pub egress_posture: EgressPostureClass,

    pub supported_ecosystems: Vec<String>,
    pub supported_archetypes: Vec<String>,

    pub required_parameter_count: u64,
    pub resolved_parameter_count: u64,
    pub declared_hook_count: u64,

    pub file_impact: super::descriptors::FileImpactSummary,

    pub declared_side_effects: DeclaredSideEffectSummary,
    pub remote_implication_count: u64,
    pub setup_handoff: SetupHandoffSummary,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_summary: Option<ScaffoldRunSummary>,

    pub honesty_labels: Vec<ScaffoldHonestyLabel>,
    pub guardrails: ScaffoldSafetyGuardrails,
}

impl ScaffoldSafetyBetaProjection {
    /// Builds a projection from a descriptor, plan, and optional run.
    /// Validates that the plan binds the supplied descriptor, that the run
    /// (when present) binds the supplied plan and descriptor, and that no
    /// planned-as-declared task and no invoked hook / task escapes the
    /// descriptor's declared set.
    pub fn project(inputs: ScaffoldSafetyBetaInputs<'_>) -> Result<Self, ScaffoldSafetyBetaError> {
        let ScaffoldSafetyBetaInputs {
            descriptor,
            plan,
            run,
            surface,
        } = inputs;

        let descriptor_id = descriptor.descriptor_id.as_str();
        let plan_id = plan.scaffold_plan_id.as_str();

        if plan.descriptor_ref != descriptor_id {
            return Err(ScaffoldSafetyBetaError::PlanDescriptorMismatch {
                expected_descriptor_id: descriptor_id.to_owned(),
                observed: plan.descriptor_ref.clone(),
            });
        }

        let declared_hook_ids: Vec<&str> = descriptor.declared_hook_ids().collect();
        let declared_task_ids: Vec<&str> = descriptor.declared_task_ids().collect();
        let declared_action_id = |id: &str| -> bool {
            declared_hook_ids.contains(&id) || declared_task_ids.contains(&id)
        };

        for task in &plan.task_plan {
            if task.declared_in_descriptor && !declared_action_id(task.task_id.as_str()) {
                return Err(ScaffoldSafetyBetaError::UndeclaredTaskPlanned {
                    task_id: task.task_id.clone(),
                });
            }
        }

        if let Some(run) = run {
            if run.scaffold_plan_ref != plan_id {
                return Err(ScaffoldSafetyBetaError::RunPlanMismatch {
                    expected_scaffold_plan_id: plan_id.to_owned(),
                    observed: run.scaffold_plan_ref.clone(),
                });
            }
            if run.descriptor_ref != descriptor_id {
                return Err(ScaffoldSafetyBetaError::RunDescriptorMismatch {
                    expected_descriptor_id: descriptor_id.to_owned(),
                    observed: run.descriptor_ref.clone(),
                });
            }
            for hook_id in &run.invoked_declared_hook_ids {
                if !declared_hook_ids.contains(&hook_id.as_str()) {
                    return Err(ScaffoldSafetyBetaError::UndeclaredHookInvoked {
                        hook_id: hook_id.clone(),
                    });
                }
            }
            for task_id in &run.invoked_declared_task_ids {
                if !declared_task_ids.contains(&task_id.as_str()) {
                    return Err(ScaffoldSafetyBetaError::UndeclaredTaskInvoked {
                        task_id: task_id.clone(),
                    });
                }
            }
        }

        let declared_side_effects = resolve_side_effects(plan);
        let setup_handoff = resolve_setup_handoff(plan);
        let run_summary = run.map(resolve_run_summary);
        let honesty_labels = resolve_honesty_labels(descriptor, plan, run);
        let guardrails = resolve_guardrails(descriptor, plan, run, &declared_side_effects);

        Ok(Self {
            record_kind: ScaffoldSafetyRecordKind::ScaffoldSafetyRecord,
            scaffold_safety_schema_version: SCAFFOLD_SAFETY_SCHEMA_VERSION,
            surface,
            descriptor_ref: descriptor.descriptor_id.clone(),
            scaffold_plan_ref: plan.scaffold_plan_id.clone(),
            scaffold_run_ref: run.map(|r| r.scaffold_run_id.clone()),
            template_or_generator_id: descriptor.template_or_generator_id.clone(),
            version: descriptor.version.clone(),
            provider_class: descriptor.provider_class,
            signature_state: descriptor.signature_state,
            generation_kind: descriptor.generation_kind,
            generation_verb: plan.generation_verb,
            trust_expectation: descriptor.trust_expectation,
            egress_posture: descriptor.egress_posture,
            supported_ecosystems: descriptor.supported_ecosystems.clone(),
            supported_archetypes: descriptor.supported_archetypes.clone(),
            required_parameter_count: descriptor.required_parameters.len() as u64,
            resolved_parameter_count: plan.resolved_parameters.len() as u64,
            declared_hook_count: descriptor.declared_hooks.len() as u64,
            file_impact: plan.file_impact,
            declared_side_effects,
            remote_implication_count: plan.remote_bootstrap_implications.len() as u64,
            setup_handoff,
            run_summary,
            honesty_labels,
            guardrails,
        })
    }

    /// True when a surface MUST disclose this generation as something other
    /// than a plain trusted local create: it is unsigned / mismatched,
    /// AI / extension provided, network-bearing, writes into an existing
    /// project, or its run failed and left attributable artifacts.
    pub fn surface_must_disclose_generation(&self) -> bool {
        self.signature_state.is_not_verified()
            || self.provider_class.is_ai_or_extension()
            || self.egress_posture.is_network_bearing()
            || self.generation_verb.writes_into_existing()
            || !self.declared_side_effects.classes.is_empty()
            || self
                .run_summary
                .as_ref()
                .is_some_and(|run| run.outcome_class.is_failure())
    }
}

fn resolve_side_effects(plan: &ScaffoldPlanRecord) -> DeclaredSideEffectSummary {
    let mut classes: Vec<ScaffoldSideEffectClass> = Vec::new();
    let mut all_declared_before_execution = true;
    let mut all_attributable_after_rollback = true;
    for declaration in &plan.side_effect_declarations {
        push_unique(&mut classes, declaration.class);
        if !declaration.declared_before_execution {
            all_declared_before_execution = false;
        }
        if !declaration.attributable_after_rollback {
            all_attributable_after_rollback = false;
        }
    }
    DeclaredSideEffectSummary {
        classes,
        all_declared_before_execution,
        all_attributable_after_rollback,
    }
}

fn resolve_setup_handoff(plan: &ScaffoldPlanRecord) -> SetupHandoffSummary {
    let create_empty_available = plan.setup_choices.contains(&SetupChoiceClass::CreateEmpty);
    let set_up_later_available = plan.setup_choices.contains(&SetupChoiceClass::SetUpLater);
    SetupHandoffSummary {
        choices: plan.setup_choices.clone(),
        create_empty_available,
        set_up_later_available,
        rollback_boundary: plan.rollback_boundary.class,
        rollback_automatic: plan.rollback_boundary.class.is_automatic(),
    }
}

fn resolve_run_summary(run: &ScaffoldRunRecord) -> ScaffoldRunSummary {
    ScaffoldRunSummary {
        scaffold_run_ref: run.scaffold_run_id.clone(),
        outcome_class: run.outcome_class,
        created_artifact_count: run.created_artifact_refs.len() as u64,
        modified_artifact_count: run.modified_artifact_refs.len() as u64,
        undeclared_actions_blocked: run.undeclared_actions_blocked,
        plain_file_authority: run.plain_file_authority,
        no_hidden_project_database: run.no_hidden_project_database,
        generated_lineage_ref: run.generated_lineage_ref.clone(),
    }
}

fn resolve_honesty_labels(
    descriptor: &TemplateGeneratorDescriptor,
    plan: &ScaffoldPlanRecord,
    run: Option<&ScaffoldRunRecord>,
) -> Vec<ScaffoldHonestyLabel> {
    let mut labels: Vec<ScaffoldHonestyLabel> = Vec::new();

    match descriptor.signature_state {
        DescriptorSignatureState::SignatureMismatch => {
            push_unique(&mut labels, ScaffoldHonestyLabel::SignatureMismatch)
        }
        DescriptorSignatureState::Unsigned
        | DescriptorSignatureState::SignatureMissing
        | DescriptorSignatureState::SignedUnverified => {
            push_unique(&mut labels, ScaffoldHonestyLabel::UnsignedTemplate)
        }
        DescriptorSignatureState::SignedVerified => {}
    }

    match descriptor.provider_class {
        TemplateProviderClass::AiAssisted => {
            push_unique(&mut labels, ScaffoldHonestyLabel::AiAssistedGeneration)
        }
        TemplateProviderClass::ExtensionProvided => push_unique(
            &mut labels,
            ScaffoldHonestyLabel::ExtensionProvidedGeneration,
        ),
        _ => {}
    }

    if !descriptor.declared_hooks.is_empty() {
        push_unique(&mut labels, ScaffoldHonestyLabel::HooksDeclared);
    }

    match descriptor.egress_posture {
        EgressPostureClass::RegistryFetch => {
            push_unique(&mut labels, ScaffoldHonestyLabel::RegistryAccessDeclared)
        }
        EgressPostureClass::RemoteImagePull => {
            push_unique(&mut labels, ScaffoldHonestyLabel::RemoteImagePullDeclared)
        }
        EgressPostureClass::GitFetch | EgressPostureClass::ArbitraryNetwork => {
            push_unique(&mut labels, ScaffoldHonestyLabel::NetworkEgressDeclared)
        }
        EgressPostureClass::NoEgress => {}
    }

    for declaration in &plan.side_effect_declarations {
        match declaration.class {
            ScaffoldSideEffectClass::Network => {
                push_unique(&mut labels, ScaffoldHonestyLabel::NetworkEgressDeclared)
            }
            ScaffoldSideEffectClass::Registry => {
                push_unique(&mut labels, ScaffoldHonestyLabel::RegistryAccessDeclared)
            }
            ScaffoldSideEffectClass::RemoteImage => {
                push_unique(&mut labels, ScaffoldHonestyLabel::RemoteImagePullDeclared)
            }
            ScaffoldSideEffectClass::Dependency => {
                push_unique(&mut labels, ScaffoldHonestyLabel::DependencyRestoreDeclared)
            }
            ScaffoldSideEffectClass::Hook => {
                push_unique(&mut labels, ScaffoldHonestyLabel::HooksDeclared)
            }
        }
    }

    if plan
        .dependency_plan
        .iter()
        .any(|entry| entry.action_class.restores_packages())
    {
        push_unique(&mut labels, ScaffoldHonestyLabel::DependencyRestoreDeclared);
    }

    if plan.generation_verb.writes_into_existing() || plan.target.into_existing {
        push_unique(&mut labels, ScaffoldHonestyLabel::WritesIntoExistingProject);
    }
    if plan.setup_choices.contains(&SetupChoiceClass::SetUpLater) {
        push_unique(&mut labels, ScaffoldHonestyLabel::SetUpLaterAvailable);
    }
    if plan.setup_choices.contains(&SetupChoiceClass::CreateEmpty) {
        push_unique(&mut labels, ScaffoldHonestyLabel::CreateEmptyAvailable);
    }
    if !descriptor.policy_constraints.is_empty() {
        push_unique(&mut labels, ScaffoldHonestyLabel::PolicyConstrained);
    }
    if run.is_some_and(|r| matches!(r.outcome_class, ScaffoldOutcomeClass::PartiallyApplied)) {
        push_unique(&mut labels, ScaffoldHonestyLabel::PartialRollback);
    }

    labels
}

fn resolve_guardrails(
    descriptor: &TemplateGeneratorDescriptor,
    plan: &ScaffoldPlanRecord,
    run: Option<&ScaffoldRunRecord>,
    side_effects: &DeclaredSideEffectSummary,
) -> ScaffoldSafetyGuardrails {
    // No write happens before review: the plan guarantees it and exposes an
    // exportable preflight summary, and any run agrees it wrote nothing
    // before review.
    let plan_no_writes =
        plan.review_state.no_writes_before_review && plan.review_state.preflight_export_available;
    let run_no_writes = run.map_or(true, |r| r.no_writes_before_review);
    let no_writes_before_review = plan_no_writes && run_no_writes;

    // Every side effect is declared before execution.
    let side_effects_declared_before_execution = side_effects.all_declared_before_execution
        && plan
            .remote_bootstrap_implications
            .iter()
            .all(|implication| implication.declared_before_execution);

    let side_effects_attributable_after_rollback = side_effects.all_attributable_after_rollback;

    // AI / extension generation cannot invent undeclared hooks or hidden
    // bootstrap: the run invokes only declared hooks / tasks (already
    // validated in `project`) and blocks undeclared actions.
    let declared_hook_ids: Vec<&str> = descriptor.declared_hook_ids().collect();
    let declared_task_ids: Vec<&str> = descriptor.declared_task_ids().collect();
    let no_undeclared_hooks_or_bootstrap = run.map_or(true, |r| {
        r.undeclared_actions_blocked
            && r.invoked_declared_hook_ids
                .iter()
                .all(|id| declared_hook_ids.contains(&id.as_str()))
            && r.invoked_declared_task_ids
                .iter()
                .all(|id| declared_task_ids.contains(&id.as_str()))
    });

    // Generated output remains plain workspace content: no hidden project
    // database is the authoritative result of generation.
    let generated_output_is_plain_workspace_content = run.map_or(true, |r| {
        r.plain_file_authority && r.no_hidden_project_database
    });

    // A rollback / delete-generated boundary is visible, and any run carries
    // an attributable rollback state.
    let plan_rollback_visible = !matches!(
        plan.rollback_boundary.class,
        RollbackBoundaryClass::ManualOnly
    ) || plan.rollback_boundary.checkpoint_ref.is_some();
    let run_rollback_attributable = run.map_or(true, |r| {
        // A failure that left artifacts in place must name a manual rollback
        // path; a clean run may report not-needed.
        !r.outcome_class.is_failure()
            || !matches!(
                r.rollback_state.class,
                super::descriptors::RollbackStateClass::NotNeeded
            )
    });
    let rollback_boundary_visible = plan_rollback_visible && run_rollback_attributable;

    // AI / extension generation reuses the governed surface: when the
    // descriptor provider or the run actor is AI / extension, the plan still
    // enforces no-writes-before-review and the run still blocks undeclared
    // actions.
    let provider_ai_extension = descriptor.provider_class.is_ai_or_extension();
    let actor_ai_extension = run.is_some_and(|r| r.actor.class.is_ai_or_extension());
    let ai_extension_uses_governed_surface = if provider_ai_extension || actor_ai_extension {
        plan.review_state.no_writes_before_review
            && run.map_or(true, |r| {
                r.undeclared_actions_blocked && r.no_writes_before_review
            })
    } else {
        true
    };

    ScaffoldSafetyGuardrails {
        no_writes_before_review,
        side_effects_declared_before_execution,
        side_effects_attributable_after_rollback,
        no_undeclared_hooks_or_bootstrap,
        generated_output_is_plain_workspace_content,
        rollback_boundary_visible,
        ai_extension_uses_governed_surface,
    }
}

fn push_unique<T: PartialEq>(slot: &mut Vec<T>, value: T) {
    if !slot.contains(&value) {
        slot.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::super::descriptors::{
        DeclaredHook, DeclaredValidationTask, DependencyActionClass, DependencyPlanEntry,
        DescriptorParameter, DescriptorProvenance, FileImpactSummary, HookExecutionClass,
        HookTriggerClass, ParameterKind, RemoteImplication, RemoteImplicationClass,
        RollbackBoundary, RollbackStateClass, ScaffoldActor, ScaffoldActorClass,
        ScaffoldPlanRecordKind, ScaffoldReviewState, ScaffoldRollbackState, ScaffoldRunRecordKind,
        ScaffoldScopeClass, ScaffoldTarget, ScaffoldTaskExecutionClass, SideEffectDeclaration,
        SourceDistributionClass, TaskPlanEntry, TemplateGeneratorDescriptorRecordKind,
        ValidationTaskClass, SCAFFOLD_PLAN_SCHEMA_VERSION, SCAFFOLD_RUN_SCHEMA_VERSION,
        TEMPLATE_GENERATOR_DESCRIPTOR_SCHEMA_VERSION,
    };
    use super::*;

    fn now() -> String {
        "mono:1:00:00:00.0000".to_string()
    }

    fn descriptor() -> TemplateGeneratorDescriptor {
        TemplateGeneratorDescriptor {
            schema: None,
            fixture: None,
            record_kind: TemplateGeneratorDescriptorRecordKind::TemplateGeneratorDescriptorRecord,
            template_generator_descriptor_schema_version:
                TEMPLATE_GENERATOR_DESCRIPTOR_SCHEMA_VERSION,
            descriptor_id: "descriptor:web".to_string(),
            template_or_generator_id: "template.web.vite_react".to_string(),
            version: "1.4.0".to_string(),
            display_name: "Vite + React".to_string(),
            provider_class: TemplateProviderClass::FirstParty,
            signature_state: DescriptorSignatureState::SignedVerified,
            generation_kind: GenerationKindClass::ProjectTemplate,
            supported_ecosystems: vec!["typescript".to_string()],
            supported_archetypes: vec!["web_frontend".to_string()],
            required_parameters: vec![DescriptorParameter {
                parameter_key: "project_name".to_string(),
                parameter_kind: ParameterKind::Identifier,
                required: true,
                prompt_label: None,
            }],
            declared_hooks: vec![DeclaredHook {
                hook_id: "hook.post_create.git_init".to_string(),
                trigger_class: HookTriggerClass::PostCreate,
                execution_class: HookExecutionClass::LocalSideEffect,
                network_egress: false,
                presentation_label: None,
            }],
            declared_validation_tasks: vec![DeclaredValidationTask {
                task_id: "task.build".to_string(),
                task_class: ValidationTaskClass::Build,
                presentation_label: None,
            }],
            trust_expectation: TrustExpectationClass::WorkspaceTrustRequired,
            egress_posture: EgressPostureClass::RegistryFetch,
            policy_constraints: Vec::new(),
            provenance: DescriptorProvenance {
                source_distribution_class: SourceDistributionClass::PublicRegistry,
                signer_label: Some("Aureline".to_string()),
                provenance_attestation_ref: Some("attestation:web".to_string()),
                producer_build: None,
            },
            presentation_subtitle: None,
            observed_at: now(),
        }
    }

    fn plan() -> ScaffoldPlanRecord {
        ScaffoldPlanRecord {
            schema: None,
            fixture: None,
            record_kind: ScaffoldPlanRecordKind::ScaffoldPlanRecord,
            scaffold_plan_schema_version: SCAFFOLD_PLAN_SCHEMA_VERSION,
            scaffold_plan_id: "plan:web".to_string(),
            descriptor_ref: "descriptor:web".to_string(),
            generation_verb: GenerationVerb::CreateProject,
            target: ScaffoldTarget {
                scope: ScaffoldScopeClass::NewProjectRoot,
                target_path_ref: "path:web".to_string(),
                into_existing: false,
            },
            resolved_parameters: Vec::new(),
            file_impact: FileImpactSummary {
                create_count: 12,
                modify_count: 0,
                delete_count: 0,
                directory_count: 4,
            },
            dependency_plan: vec![DependencyPlanEntry {
                summary: "add react".to_string(),
                action_class: DependencyActionClass::RestoreDeferred,
                registry_class: Some("npm".to_string()),
            }],
            task_plan: vec![TaskPlanEntry {
                task_id: "task.build".to_string(),
                execution_class: ScaffoldTaskExecutionClass::Deferred,
                declared_in_descriptor: true,
                summary: None,
            }],
            remote_bootstrap_implications: vec![RemoteImplication {
                class: RemoteImplicationClass::RegistryAccess,
                declared_before_execution: true,
                presentation_label: None,
            }],
            side_effect_declarations: vec![
                SideEffectDeclaration {
                    class: ScaffoldSideEffectClass::Hook,
                    declared_before_execution: true,
                    attributable_after_rollback: true,
                    presentation_label: None,
                },
                SideEffectDeclaration {
                    class: ScaffoldSideEffectClass::Dependency,
                    declared_before_execution: true,
                    attributable_after_rollback: true,
                    presentation_label: None,
                },
            ],
            setup_choices: vec![SetupChoiceClass::FullScaffold, SetupChoiceClass::SetUpLater],
            rollback_boundary: RollbackBoundary {
                class: RollbackBoundaryClass::Checkpoint,
                checkpoint_ref: Some("checkpoint:web".to_string()),
                presentation_label: None,
            },
            review_state: ScaffoldReviewState {
                no_writes_before_review: true,
                preflight_export_available: true,
                preflight_export_ref: Some("export:web".to_string()),
            },
            presentation_label: None,
            emitted_at: now(),
        }
    }

    fn run() -> ScaffoldRunRecord {
        ScaffoldRunRecord {
            schema: None,
            fixture: None,
            record_kind: ScaffoldRunRecordKind::ScaffoldRunRecord,
            scaffold_run_schema_version: SCAFFOLD_RUN_SCHEMA_VERSION,
            scaffold_run_id: "run:web".to_string(),
            scaffold_plan_ref: "plan:web".to_string(),
            descriptor_ref: "descriptor:web".to_string(),
            actor: ScaffoldActor {
                class: ScaffoldActorClass::User,
                actor_label: None,
                approval_ref: None,
            },
            outcome_class: ScaffoldOutcomeClass::Succeeded,
            created_artifact_refs: vec!["artifact:1".to_string(), "artifact:2".to_string()],
            modified_artifact_refs: Vec::new(),
            invoked_declared_hook_ids: vec!["hook.post_create.git_init".to_string()],
            invoked_declared_task_ids: Vec::new(),
            checkpoint_ref: Some("checkpoint:web".to_string()),
            rollback_state: ScaffoldRollbackState {
                class: RollbackStateClass::Available,
                rollback_ref: None,
            },
            no_writes_before_review: true,
            undeclared_actions_blocked: true,
            generated_lineage_ref: "lineage:web".to_string(),
            plain_file_authority: true,
            no_hidden_project_database: true,
            emitted_at: now(),
        }
    }

    #[test]
    fn preflight_only_projection_holds_all_guardrails() {
        let descriptor = descriptor();
        let plan = plan();
        let projection = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
            descriptor: &descriptor,
            plan: &plan,
            run: None,
            surface: ScaffoldSurface::GeneratorPreview,
        })
        .expect("preflight projects");

        assert!(projection.guardrails.all_hold());
        assert_eq!(projection.scaffold_run_ref, None);
        assert!(projection.run_summary.is_none());
        assert!(projection
            .declared_side_effects
            .classes
            .contains(&ScaffoldSideEffectClass::Dependency));
        assert!(projection.setup_handoff.set_up_later_available);
    }

    #[test]
    fn run_projection_binds_lineage_and_holds_guardrails() {
        let descriptor = descriptor();
        let plan = plan();
        let run = run();
        let projection = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
            descriptor: &descriptor,
            plan: &plan,
            run: Some(&run),
            surface: ScaffoldSurface::StartCenter,
        })
        .expect("run projects");

        assert!(projection.guardrails.all_hold());
        let summary = projection.run_summary.as_ref().expect("run summary");
        assert_eq!(summary.created_artifact_count, 2);
        assert!(summary.plain_file_authority);
        assert_eq!(summary.generated_lineage_ref, "lineage:web");
    }

    #[test]
    fn undeclared_invoked_hook_is_rejected() {
        let descriptor = descriptor();
        let plan = plan();
        let mut run = run();
        run.invoked_declared_hook_ids = vec!["hook.invented".to_string()];
        let err = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
            descriptor: &descriptor,
            plan: &plan,
            run: Some(&run),
            surface: ScaffoldSurface::AiAssist,
        })
        .expect_err("undeclared hook rejected");
        assert!(matches!(
            err,
            ScaffoldSafetyBetaError::UndeclaredHookInvoked { .. }
        ));
    }

    #[test]
    fn plan_descriptor_mismatch_is_rejected() {
        let descriptor = descriptor();
        let mut plan = plan();
        plan.descriptor_ref = "descriptor:other".to_string();
        let err = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
            descriptor: &descriptor,
            plan: &plan,
            run: None,
            surface: ScaffoldSurface::CommandPalette,
        })
        .expect_err("mismatch rejected");
        assert!(matches!(
            err,
            ScaffoldSafetyBetaError::PlanDescriptorMismatch { .. }
        ));
    }

    #[test]
    fn writes_before_review_breaks_guardrail() {
        let descriptor = descriptor();
        let mut plan = plan();
        plan.review_state.no_writes_before_review = false;
        let projection = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
            descriptor: &descriptor,
            plan: &plan,
            run: None,
            surface: ScaffoldSurface::GeneratorPreview,
        })
        .expect("projects");
        assert!(!projection.guardrails.no_writes_before_review);
        assert!(!projection.guardrails.all_hold());
    }

    #[test]
    fn ai_provider_surfaces_disclosure_and_governed_surface() {
        let mut descriptor = descriptor();
        descriptor.provider_class = TemplateProviderClass::AiAssisted;
        descriptor.signature_state = DescriptorSignatureState::Unsigned;
        let plan = plan();
        let projection = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
            descriptor: &descriptor,
            plan: &plan,
            run: None,
            surface: ScaffoldSurface::AiAssist,
        })
        .expect("projects");
        assert!(projection.surface_must_disclose_generation());
        assert!(projection
            .honesty_labels
            .contains(&ScaffoldHonestyLabel::AiAssistedGeneration));
        assert!(projection.guardrails.ai_extension_uses_governed_surface);
    }
}
