//! Scaffold-safety beta truth.
//!
//! The three boundary records in this module
//! ([`TemplateGeneratorDescriptor`], [`ScaffoldPlanRecord`],
//! [`ScaffoldRunRecord`]) name **what signed template / generator is being
//! used**, **what the scaffold plan is allowed to do before any file is
//! written**, and **what one scaffold run actually created, modified, and
//! invoked**. The [`beta`] submodule binds those records into one
//! cross-surface [`ScaffoldSafetyBetaProjection`] that Start Center starter
//! rows, the command palette, the generator-preview sheet, AI-assisted
//! generation, extension-provided generation, and the CLI / headless
//! creation path all read so they agree, before generation runs, on the
//! template provenance, the file / dependency / task impact, the declared
//! hook / network / registry / remote-image / dependency side effects, the
//! create-empty / set-up-later / rollback handoffs, and the guardrails that
//! keep generated projects plain, attributable workspace content rather
//! than hidden IDE-owned state.
//!
//! Boundary schemas:
//!
//! - `schemas/workspace/template_generator_descriptor.schema.json`
//! - `schemas/workspace/scaffold_plan.schema.json`
//! - `schemas/workspace/scaffold_run.schema.json`
//!
//! Projection schema:
//!
//! - `schemas/workspace/scaffold_safety.schema.json`
//!
//! The beta contract lives in
//! `docs/workspace/m3/scaffold_safety_beta.md`. Worked fixtures live under
//! `fixtures/workspace/m3/scaffold_preflight_and_generation/`.

pub mod beta;
pub mod descriptors;
pub mod shared;

pub use shared::{FixtureMetadata as ScaffoldFixtureMetadata, ScaffoldSurface};

pub use descriptors::{
    DeclaredHook, DeclaredValidationTask, DependencyActionClass, DependencyPlanEntry,
    DescriptorParameter, DescriptorProvenance, DescriptorSignatureState, EgressPostureClass,
    FileImpactSummary, GenerationKindClass, GenerationVerb, HookExecutionClass, HookTriggerClass,
    ParameterKind, ParameterSourceClass, PolicyConstraintClass, RemoteImplication,
    RemoteImplicationClass, ResolvedParameter, RollbackBoundary, RollbackBoundaryClass,
    RollbackStateClass, ScaffoldActor, ScaffoldActorClass, ScaffoldOutcomeClass,
    ScaffoldPlanRecord, ScaffoldPlanRecordKind, ScaffoldReviewState, ScaffoldRollbackState,
    ScaffoldRunRecord, ScaffoldRunRecordKind, ScaffoldScopeClass, ScaffoldSideEffectClass,
    ScaffoldTarget, ScaffoldTaskExecutionClass, SetupChoiceClass, SideEffectDeclaration,
    SourceDistributionClass, TaskPlanEntry, TemplateGeneratorDescriptor,
    TemplateGeneratorDescriptorRecordKind, TemplateProviderClass, TrustExpectationClass,
    ValidationTaskClass, SCAFFOLD_PLAN_RECORD_KIND, SCAFFOLD_PLAN_SCHEMA_VERSION,
    SCAFFOLD_RUN_RECORD_KIND, SCAFFOLD_RUN_SCHEMA_VERSION,
    TEMPLATE_GENERATOR_DESCRIPTOR_RECORD_KIND, TEMPLATE_GENERATOR_DESCRIPTOR_SCHEMA_VERSION,
};

pub use beta::{
    DeclaredSideEffectSummary, ScaffoldHonestyLabel, ScaffoldRunSummary, ScaffoldSafetyBetaError,
    ScaffoldSafetyBetaInputs, ScaffoldSafetyBetaProjection, ScaffoldSafetyGuardrails,
    ScaffoldSafetyRecordKind, SetupHandoffSummary, SCAFFOLD_SAFETY_RECORD_KIND,
    SCAFFOLD_SAFETY_SCHEMA_VERSION,
};
