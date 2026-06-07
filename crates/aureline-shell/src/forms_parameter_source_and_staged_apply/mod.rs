//! Stable form-source, validation, and staged-apply truth contract.
//!
//! A [`FormTruthPacketRecord`] is the single governed record that settings,
//! setup, scaffold, provider, policy, publish, and recovery surfaces read when
//! they render launch-critical structured input. It keeps field rows, parameter
//! source precedence, async validation, apply timing, secret/path/reference
//! truth, wizard resume posture, and support-export redaction in one canonical
//! shape.
//!
//! The contract narrative is
//! `docs/m4/forms-parameter-source-and-staged-apply.md`; release evidence is
//! `artifacts/release/m4/forms-parameter-source-and-staged-apply.md`; and the
//! boundary schema is
//! `schemas/release/forms-parameter-source-and-staged-apply.schema.json`.

pub mod corpus;
pub mod model;

pub use corpus::{forms_parameter_source_and_staged_apply_corpus, FormTruthScenario, CORPUS_AS_OF};
pub use model::{
    validate_form_truth_packet, AccessibilityReview, ApplyTiming, BuildImpactClass,
    ClientLimitation, ClientScope, CodeBackedFieldTruth, FieldAction, FieldActionClass, FieldKind,
    FieldRowContract, FormSurfaceClass, FormTruthPacketRecord, FormTruthValidationError, PathBasis,
    PathFieldTruth, PathLocationClass, PrecedenceCandidate, ReferenceFieldTruth, RequirementState,
    SecretExportBehavior, SecretFieldTruth, SecretStorageMode, SideEffectClass, SourcePrecedence,
    SourcePrecedenceAudit, StagedApplyPacket, ValidationClass, ValidationResult, ValidationState,
    WizardStepState, FORM_TRUTH_NOTICE, FORM_TRUTH_RECORD_KIND, FORM_TRUTH_SCHEMA_VERSION,
    FORM_TRUTH_SHARED_CONTRACT_REF,
};
