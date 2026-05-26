//! Stable Django Advanced workflow pack truth packet.
//!
//! This module is the language-owned contract that pins how the
//! Django Advanced workflow pack stays one boundary truth across the
//! editor framework pack panel, workflow companion, framework
//! settings/help, CLI/headless inspector, support export, release
//! proof index, Help/About proof card, and the conformance dashboard.
//! Every row binds a closed `workflow_pack_class`,
//! `workflow_pack_row_class`, `support_class`, `workflow_loop_class`,
//! `evidence_class`, `known_limit_class`, `downgrade_automation_class`,
//! and `workflow_pack_confidence_class` plus an `evidence_refs` array
//! and a `disclosure_ref` whenever the row is narrowed below
//! `expert_grade`, declares a non-`none_declared` known limit, or
//! binds a non-`none` downgrade automation.
//!
//! The packet pins the Django Advanced workflow pack across the
//! create, open, run, test, debug, rename, and review workflow
//! loops, the Django project model (`settings.py` configuration,
//! `urls.py` URL conf, app boundary inside `INSTALLED_APPS`,
//! `manage.py` and `wsgi.py`/`asgi.py` entry points, plus
//! `requirements*.txt`/`pyproject.toml` dependency closure), the
//! `manage.py` target parity surface (`runserver`, `test`,
//! `makemigrations`, `migrate`, `shell`, `createsuperuser`,
//! `collectstatic`, custom management commands, and the
//! `pytest`/`pytest-django` plus `python -m pdb` /
//! `manage.py runserver --noreload` debug entry points), and the
//! Django template awareness surface (Django Template Language tags
//! `{% %}`, variable interpolation `{{ }}`, `{% url %}`, `{% block %}`,
//! `{% extends %}`, `{% include %}`, `{% csrf_token %}`, custom
//! template tags / filters, and the editor's awareness of
//! `app/templates/` vs project `TEMPLATES['DIRS']` resolution). A row
//! that claims `expert_grade` while leaving its known limit or
//! downgrade automation unbound is refused; the validator narrows
//! below expert grade instead of inheriting an adjacent certified
//! row, and the downgrade-automation vocabulary is preserved verbatim
//! so a beta-grade capability sample cannot masquerade as a
//! replacement-grade workflow pack.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! source bodies, secrets, ambient credentials, or provider payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`DjangoAdvancedWorkflowPackTruthPacket`].
pub const DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_PACKET_RECORD_KIND: &str =
    "django_advanced_workflow_pack_truth_stable_packet";

/// Stable record-kind tag for [`DjangoAdvancedWorkflowPackTruthSupportExport`].
pub const DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "django_advanced_workflow_pack_truth_support_export";

/// Integer schema version for the workflow-pack truth packet.
pub const DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_REF: &str =
    "schemas/language/django_advanced_workflow_pack_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_DOC_REF: &str =
    "docs/languages/m4/stabilize-the-django-advanced-workflow-pack-with-manage.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m4/stabilize-the-django-advanced-workflow-pack-with-manage.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m4/django_advanced_workflow_pack_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m4/django_advanced_workflow_pack_truth_packet.json";

/// Closed workflow-pack vocabulary. Every required pack MUST have at
/// least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowPackClass {
    /// Django Advanced replacement-grade workflow pack.
    DjangoAdvancedWorkflowPack,
}

impl WorkflowPackClass {
    /// Every required workflow pack, in declaration order.
    pub const REQUIRED: [Self; 1] = [Self::DjangoAdvancedWorkflowPack];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DjangoAdvancedWorkflowPack => "django_advanced_workflow_pack",
        }
    }
}

/// Closed workflow-pack row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowPackRowClass {
    /// The pack's headline qualification row.
    PackQualification,
    /// One step of the certified workflow loop on a pack.
    WorkflowLoop,
    /// Framework-migration evidence row on a pack.
    FrameworkMigrationRow,
    /// Archetype-repo / fixture-repo evidence row on a pack.
    ArchetypeRepoRow,
    /// Django project-model evidence row (`settings.py`, `urls.py`,
    /// `INSTALLED_APPS`, `manage.py`/`wsgi.py`/`asgi.py` entry points,
    /// dependency closure).
    ProjectModelRow,
    /// `manage.py` target parity row covering `runserver`, `test`,
    /// `makemigrations`, `migrate`, `shell`, `createsuperuser`,
    /// `collectstatic`, custom management commands, and the
    /// `pytest`/`pytest-django` plus `python -m pdb` debug entry
    /// points.
    ManagePyTargetParityRow,
    /// Django template awareness row covering Django Template
    /// Language tags `{% %}`, variable interpolation `{{ }}`,
    /// `{% url %}`/`{% block %}`/`{% extends %}`/`{% include %}`/
    /// `{% csrf_token %}`, custom template tags / filters, and
    /// `app/templates/` vs project `TEMPLATES['DIRS']` resolution.
    TemplateAwarenessRow,
    /// Design-partner evidence row on a pack.
    DesignPartnerRow,
    /// Precisely labeled unsupported-gap row on a pack.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a pack.
    KnownLimit,
    /// Downgrade-automation rule row attached to a pack.
    DowngradeAutomation,
}

impl WorkflowPackRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackQualification => "pack_qualification",
            Self::WorkflowLoop => "workflow_loop",
            Self::FrameworkMigrationRow => "framework_migration_row",
            Self::ArchetypeRepoRow => "archetype_repo_row",
            Self::ProjectModelRow => "project_model_row",
            Self::ManagePyTargetParityRow => "manage_py_target_parity_row",
            Self::TemplateAwarenessRow => "template_awareness_row",
            Self::DesignPartnerRow => "design_partner_row",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound workflow-loop step.
    pub const fn requires_workflow_loop(self) -> bool {
        matches!(self, Self::WorkflowLoop)
    }
}

/// Closed support-class vocabulary applied to a row. A row is never
/// `expert_grade` while its known limit or downgrade automation is
/// unbound; the validator demotes it instead of inheriting an adjacent
/// expert-grade row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims expert-grade workflow-pack support (replacement grade for the pack).
    ExpertGrade,
    /// Row is intentionally narrowed below expert grade; the narrowing is disclosed.
    StableBelowExpert,
    /// Row is at beta-grade only (capability sample, not workflow pack).
    BetaGradeOnly,
    /// Row is at preview only (under-review wedge).
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl SupportClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpertGrade => "expert_grade",
            Self::StableBelowExpert => "stable_below_expert",
            Self::BetaGradeOnly => "beta_grade_only",
            Self::PreviewOnly => "preview_only",
            Self::Unsupported => "unsupported",
            Self::SupportUnbound => "support_unbound",
        }
    }

    /// True when this support class satisfies the support-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    /// True when the support class must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::ExpertGrade)
    }
}

/// Closed workflow-loop vocabulary. The full workflow loop MUST be
/// covered for each pack that claims `expert_grade`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowLoopClass {
    /// The row certifies the create step of the workflow loop.
    Create,
    /// The row certifies the open step of the workflow loop.
    Open,
    /// The row certifies the run step of the workflow loop.
    Run,
    /// The row certifies the test step of the workflow loop.
    Test,
    /// The row certifies the debug step of the workflow loop.
    Debug,
    /// The row certifies the rename step of the workflow loop.
    Rename,
    /// The row certifies the review step of the workflow loop.
    Review,
    /// The row is not bound to a workflow-loop step (non-loop row classes).
    NotApplicable,
}

impl WorkflowLoopClass {
    /// Every certified workflow-loop step in declaration order. An
    /// `expert_grade` pack MUST cover every step.
    pub const REQUIRED_FOR_EXPERT: [Self; 7] = [
        Self::Create,
        Self::Open,
        Self::Run,
        Self::Test,
        Self::Debug,
        Self::Rename,
        Self::Review,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Open => "open",
            Self::Run => "run",
            Self::Test => "test",
            Self::Debug => "debug",
            Self::Rename => "rename",
            Self::Review => "review",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// The row is backed by certified archetype-repo evidence.
    ArchetypeRepoEvidence,
    /// The row is backed by framework-migration evidence.
    FrameworkMigrationEvidence,
    /// The row is backed by design-partner evidence.
    DesignPartnerEvidence,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a conformance suite run.
    ConformanceSuiteEvidence,
    /// The row is backed by a benchmark / fitness function capture.
    BenchmarkEvidence,
    /// The row is backed by a Django project-model capture
    /// (`settings.py`/`urls.py`/`INSTALLED_APPS`/`manage.py` boundary).
    ProjectModelEvidence,
    /// The row is backed by a `manage.py` target parity capture.
    ManagePyTargetParityEvidence,
    /// The row is backed by a Django template awareness capture.
    TemplateAwarenessEvidence,
    /// The row is backed by a docs/help disclosure (gap label only).
    DocsDisclosureEvidence,
    /// The row has no bound evidence class; this never qualifies stable.
    EvidenceUnbound,
}

impl EvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchetypeRepoEvidence => "archetype_repo_evidence",
            Self::FrameworkMigrationEvidence => "framework_migration_evidence",
            Self::DesignPartnerEvidence => "design_partner_evidence",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
            Self::BenchmarkEvidence => "benchmark_evidence",
            Self::ProjectModelEvidence => "project_model_evidence",
            Self::ManagePyTargetParityEvidence => "manage_py_target_parity_evidence",
            Self::TemplateAwarenessEvidence => "template_awareness_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this evidence class satisfies the evidence-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary attached to a workflow-pack row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies a framework subset.
    FrameworkSubsetOnly,
    /// The row only certifies a language subset (e.g., one Python version).
    LanguageSubsetOnly,
    /// The row only certifies an archetype subset (specific repos).
    ArchetypeSubsetOnly,
    /// The row only certifies a migration subset (e.g., Django 4 → Django 5 LTS only).
    MigrationSubsetOnly,
    /// The row only certifies a subset of the Django project model.
    ProjectModelSubsetOnly,
    /// The row only certifies a subset of the `manage.py` target parity surface.
    ManagePyTargetSubsetOnly,
    /// The row only certifies a subset of the Django template awareness surface.
    TemplateAwarenessSubsetOnly,
    /// The row certifies an unsupported runtime target gap.
    UnsupportedRuntimeTarget,
    /// The row certifies a beta-grade-only capability gap.
    BetaCapabilitySampleOnly,
    /// The row has no bound known-limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::FrameworkSubsetOnly => "framework_subset_only",
            Self::LanguageSubsetOnly => "language_subset_only",
            Self::ArchetypeSubsetOnly => "archetype_subset_only",
            Self::MigrationSubsetOnly => "migration_subset_only",
            Self::ProjectModelSubsetOnly => "project_model_subset_only",
            Self::ManagePyTargetSubsetOnly => "manage_py_target_subset_only",
            Self::TemplateAwarenessSubsetOnly => "template_awareness_subset_only",
            Self::UnsupportedRuntimeTarget => "unsupported_runtime_target",
            Self::BetaCapabilitySampleOnly => "beta_capability_sample_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// True when this known-limit class satisfies the limit-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// True when this known-limit class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}

/// Closed downgrade-automation vocabulary attached to a workflow-pack row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a certified fixture is missing or stale.
    AutoNarrowOnMissingFixture,
    /// Automatically narrow when a certified archetype repo is missing.
    AutoNarrowOnMissingArchetype,
    /// Automatically narrow when a migration probe fails.
    AutoNarrowOnFailedMigration,
    /// Automatically narrow when the framework pack drops below depth.
    AutoNarrowOnFrameworkGap,
    /// Automatically narrow when the Django project model is unproven.
    AutoNarrowOnUnprovenProjectModel,
    /// Automatically narrow when the `manage.py` target parity surface is unproven.
    AutoNarrowOnUnprovenManagePyTarget,
    /// Automatically narrow when the Django template awareness surface is unproven.
    AutoNarrowOnUnprovenTemplateAwareness,
    /// Automatically demote when confidence drops below the certified bar.
    AutoDemoteOnLowConfidence,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnMissingFixture => "auto_narrow_on_missing_fixture",
            Self::AutoNarrowOnMissingArchetype => "auto_narrow_on_missing_archetype",
            Self::AutoNarrowOnFailedMigration => "auto_narrow_on_failed_migration",
            Self::AutoNarrowOnFrameworkGap => "auto_narrow_on_framework_gap",
            Self::AutoNarrowOnUnprovenProjectModel => "auto_narrow_on_unproven_project_model",
            Self::AutoNarrowOnUnprovenManagePyTarget => "auto_narrow_on_unproven_manage_py_target",
            Self::AutoNarrowOnUnprovenTemplateAwareness => {
                "auto_narrow_on_unproven_template_awareness"
            }
            Self::AutoDemoteOnLowConfidence => "auto_demote_on_low_confidence",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for a workflow-pack row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowPackConfidenceClass {
    /// High confidence — the pack can certify expert grade.
    HighConfidence,
    /// Medium confidence — the pack narrows below expert grade.
    MediumConfidence,
    /// Low confidence — the pack narrows below expert grade until evidence grows.
    LowConfidence,
}

impl WorkflowPackConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
        }
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the workflow-pack packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required workflow pack has no row.
    MissingWorkflowPackCoverage,
    /// A pack claiming expert_grade is missing a certified workflow-loop step.
    MissingWorkflowLoopCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row claims expert_grade while one or more bindings is unbound.
    ExpertGradeWithUnboundBinding,
    /// A row narrowed below expert grade drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A workflow-loop row drops its workflow-loop step binding.
    WorkflowLoopNotApplicable,
    /// A non-workflow-loop row binds a workflow-loop step it cannot certify.
    WorkflowLoopNotPermittedOnRowClass,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops workflow-pack truth.
    ConsumerProjectionDrift,
    /// A projection collapses the pack vocabulary.
    PackVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the workflow-loop vocabulary.
    WorkflowLoopVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingWorkflowPackCoverage => "missing_workflow_pack_coverage",
            Self::MissingWorkflowLoopCoverage => "missing_workflow_loop_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::ExpertGradeWithUnboundBinding => "expert_grade_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::WorkflowLoopNotApplicable => "workflow_loop_not_applicable",
            Self::WorkflowLoopNotPermittedOnRowClass => "workflow_loop_not_permitted_on_row_class",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::PackVocabularyCollapsed => "pack_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WorkflowLoopVocabularyCollapsed => "workflow_loop_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the workflow-pack packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Editor framework pack panel surface.
    EditorFrameworkPackPanel,
    /// Workflow companion / runner panel surface.
    WorkflowCompanion,
    /// Framework settings / help surface.
    FrameworkSettings,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Help/About proof card surface.
    HelpAbout,
    /// Conformance dashboard surface.
    ConformanceDashboard,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::EditorFrameworkPackPanel,
        Self::WorkflowCompanion,
        Self::FrameworkSettings,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorFrameworkPackPanel => "editor_framework_pack_panel",
            Self::WorkflowCompanion => "workflow_companion",
            Self::FrameworkSettings => "framework_settings",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One workflow-pack row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowPackRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Workflow pack this row certifies.
    pub pack_class: WorkflowPackClass,
    /// Workflow-pack row class.
    pub row_class: WorkflowPackRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Workflow-loop step certified by the row (or `not_applicable`).
    pub workflow_loop_class: WorkflowLoopClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: WorkflowPackConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `expert_grade`, declares a non-`none_declared` known limit, or
    /// binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl WorkflowPackRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DjangoAdvancedWorkflowPackConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Workflow-pack packet id consumed by the projection.
    pub django_advanced_workflow_pack_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the pack vocabulary is preserved verbatim.
    pub preserves_pack_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the workflow-loop vocabulary is preserved verbatim.
    pub preserves_workflow_loop_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl DjangoAdvancedWorkflowPackConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.django_advanced_workflow_pack_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_pack_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_workflow_loop_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`DjangoAdvancedWorkflowPackTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DjangoAdvancedWorkflowPackTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Workflow packs the packet covers.
    #[serde(default)]
    pub covered_packs: Vec<WorkflowPackClass>,
    /// Workflow-pack rows.
    #[serde(default)]
    pub rows: Vec<WorkflowPackRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DjangoAdvancedWorkflowPackConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying the Django Advanced workflow
/// pack on the stable lane across the create, open, run, test, debug,
/// rename, and review loops, with explicit Django project-model,
/// `manage.py` target parity, and template awareness rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DjangoAdvancedWorkflowPackTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Workflow packs the packet covers.
    #[serde(default)]
    pub covered_packs: Vec<WorkflowPackClass>,
    /// Workflow-pack rows.
    #[serde(default)]
    pub rows: Vec<WorkflowPackRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DjangoAdvancedWorkflowPackConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl DjangoAdvancedWorkflowPackTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: DjangoAdvancedWorkflowPackTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_packs: input.covered_packs,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable workflow-pack invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique pack tokens observed across rows.
    pub fn pack_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.pack_class);
        }
        set.into_iter().map(WorkflowPackClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(WorkflowPackRowClass::as_str).collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(SupportClass::as_str).collect()
    }

    /// Returns the unique workflow-loop tokens observed across rows.
    pub fn workflow_loop_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.workflow_loop_class);
        }
        set.into_iter().map(WorkflowLoopClass::as_str).collect()
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter().map(EvidenceClass::as_str).collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter().map(KnownLimitClass::as_str).collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DjangoAdvancedWorkflowPackTruthSupportExport {
        DjangoAdvancedWorkflowPackTruthSupportExport {
            record_kind: DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            django_advanced_workflow_pack_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            django_advanced_workflow_pack_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "django advanced workflow pack packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "django advanced workflow pack packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_packs.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingWorkflowPackCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered workflow pack",
            ));
        }

        for pack in &self.covered_packs {
            let present = self.rows.iter().any(|row| row.pack_class == *pack);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingWorkflowPackCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers workflow pack {}", pack.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_source_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawSourceMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw source bodies past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority/credentials past the boundary",
                        row.row_id
                    ),
                ));
            }

            if !row.support_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSupportClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound support class", row.row_id),
                ));
            }
            if !row.known_limit_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingKnownLimit,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound known-limit class", row.row_id),
                ));
            }
            if !row.downgrade_automation_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDowngradeAutomation,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound downgrade-automation class", row.row_id),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }

            if matches!(row.support_class, SupportClass::ExpertGrade)
                && !row.all_bindings_satisfied()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ExpertGradeWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims expert_grade while a binding (support, known limit, downgrade automation, or evidence) is unbound",
                        row.row_id
                    ),
                ));
            }

            if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
                findings.push(ValidationFinding::new(
                    FindingKind::NarrowedRowMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has support class {} without a disclosure ref",
                        row.row_id,
                        row.support_class.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} discloses known limit {} without a disclosure ref",
                        row.row_id,
                        row.known_limit_class.as_str()
                    ),
                ));
            }
            if row
                .downgrade_automation_class
                .requires_explicit_disclosure()
                && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade automation {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_automation_class.as_str()
                    ),
                ));
            }

            if row.evidence_refs.is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            if row.row_class.requires_workflow_loop()
                && matches!(row.workflow_loop_class, WorkflowLoopClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WorkflowLoopNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a workflow_loop but has no bound workflow-loop step",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_workflow_loop()
                && !matches!(row.workflow_loop_class, WorkflowLoopClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WorkflowLoopNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds workflow-loop step {}; only workflow_loop rows may bind a step",
                        row.row_id,
                        row.row_class.as_str(),
                        row.workflow_loop_class.as_str()
                    ),
                ));
            }

            if matches!(
                row.confidence_class,
                WorkflowPackConfidenceClass::LowConfidence
            ) && matches!(row.support_class, SupportClass::ExpertGrade)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ExpertGradeWithUnboundBinding,
                    FindingSeverity::Warning,
                    format!(
                        "row {} claims expert_grade at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
            }
        }

        for pack in &self.covered_packs {
            let pack_claims_expert = self.rows.iter().any(|row| {
                row.pack_class == *pack
                    && matches!(row.row_class, WorkflowPackRowClass::PackQualification)
                    && matches!(row.support_class, SupportClass::ExpertGrade)
            });
            if pack_claims_expert {
                for step in WorkflowLoopClass::REQUIRED_FOR_EXPERT {
                    let covered = self.rows.iter().any(|row| {
                        row.pack_class == *pack
                            && matches!(row.row_class, WorkflowPackRowClass::WorkflowLoop)
                            && row.workflow_loop_class == step
                    });
                    if !covered {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingWorkflowLoopCoverage,
                            FindingSeverity::Blocker,
                            format!(
                                "pack {} claims expert_grade but has no workflow_loop row for {}",
                                pack.as_str(),
                                step.as_str()
                            ),
                        ));
                    }
                }
            }
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve workflow-pack truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_pack_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::PackVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the pack vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_support_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SupportClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the support-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_workflow_loop_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::WorkflowLoopVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the workflow-loop vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_known_limit_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the known-limit vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_automation_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-automation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_evidence_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::EvidenceClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the evidence-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DjangoAdvancedWorkflowPackTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub django_advanced_workflow_pack_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub django_advanced_workflow_pack_packet: DjangoAdvancedWorkflowPackTruthPacket,
}

impl DjangoAdvancedWorkflowPackTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_VERSION
            && self.django_advanced_workflow_pack_packet_id_ref
                == self.django_advanced_workflow_pack_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .django_advanced_workflow_pack_packet
                .validate()
                .is_empty()
    }
}

/// Errors emitted when reading the checked-in stable workflow-pack packet.
#[derive(Debug)]
pub enum DjangoAdvancedWorkflowPackTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for DjangoAdvancedWorkflowPackTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "django advanced workflow pack packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "django advanced workflow pack packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DjangoAdvancedWorkflowPackTruthArtifactError {}

/// Returns the checked-in stable Django Advanced workflow pack truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_django_advanced_workflow_pack_truth_packet() -> Result<
    DjangoAdvancedWorkflowPackTruthPacket,
    DjangoAdvancedWorkflowPackTruthArtifactError,
> {
    let packet: DjangoAdvancedWorkflowPackTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m4/django_advanced_workflow_pack_truth_packet.json"
    )))
    .map_err(DjangoAdvancedWorkflowPackTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DjangoAdvancedWorkflowPackTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack_evidence_ref() -> String {
        DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_DOC_REF.to_owned()
    }

    fn qualification_row(row_id: &str, pack: WorkflowPackClass) -> WorkflowPackRow {
        WorkflowPackRow {
            row_id: row_id.to_owned(),
            pack_class: pack,
            row_class: WorkflowPackRowClass::PackQualification,
            support_class: SupportClass::ExpertGrade,
            workflow_loop_class: WorkflowLoopClass::NotApplicable,
            evidence_class: EvidenceClass::ArchetypeRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: WorkflowPackConfidenceClass::HighConfidence,
            evidence_refs: vec![pack_evidence_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_block_on_missing_evidence",
                DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_DOC_REF
            )),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn loop_row(
        row_id: &str,
        pack: WorkflowPackClass,
        step: WorkflowLoopClass,
    ) -> WorkflowPackRow {
        WorkflowPackRow {
            row_id: row_id.to_owned(),
            pack_class: pack,
            row_class: WorkflowPackRowClass::WorkflowLoop,
            support_class: SupportClass::ExpertGrade,
            workflow_loop_class: step,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
            confidence_class: WorkflowPackConfidenceClass::HighConfidence,
            evidence_refs: vec![pack_evidence_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_missing_fixture",
                DJANGO_ADVANCED_WORKFLOW_PACK_TRUTH_DOC_REF
            )),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> DjangoAdvancedWorkflowPackConsumerProjection {
        DjangoAdvancedWorkflowPackConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            django_advanced_workflow_pack_packet_id_ref:
                "packet:m4:django_advanced_workflow_pack".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_pack_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_workflow_loop_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn pack_rows(pack: WorkflowPackClass, prefix: &str) -> Vec<WorkflowPackRow> {
        let mut out = vec![qualification_row(&format!("row:{prefix}:quality"), pack)];
        for step in WorkflowLoopClass::REQUIRED_FOR_EXPERT {
            out.push(loop_row(
                &format!("row:{prefix}:loop:{}", step.as_str()),
                pack,
                step,
            ));
        }
        out
    }

    fn sample_input() -> DjangoAdvancedWorkflowPackTruthPacketInput {
        let rows = pack_rows(
            WorkflowPackClass::DjangoAdvancedWorkflowPack,
            "django_advanced",
        );
        DjangoAdvancedWorkflowPackTruthPacketInput {
            packet_id: "packet:m4:django_advanced_workflow_pack".to_owned(),
            workflow_or_surface_id: "workflow.language.django_advanced_workflow_pack".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_packs: WorkflowPackClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![pack_evidence_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            WorkflowPackClass::DjangoAdvancedWorkflowPack.as_str(),
            "django_advanced_workflow_pack"
        );
        assert_eq!(
            WorkflowPackRowClass::ProjectModelRow.as_str(),
            "project_model_row"
        );
        assert_eq!(
            WorkflowPackRowClass::ManagePyTargetParityRow.as_str(),
            "manage_py_target_parity_row"
        );
        assert_eq!(
            WorkflowPackRowClass::TemplateAwarenessRow.as_str(),
            "template_awareness_row"
        );
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(WorkflowLoopClass::Review.as_str(), "review");
        assert_eq!(
            EvidenceClass::ProjectModelEvidence.as_str(),
            "project_model_evidence"
        );
        assert_eq!(
            EvidenceClass::ManagePyTargetParityEvidence.as_str(),
            "manage_py_target_parity_evidence"
        );
        assert_eq!(
            EvidenceClass::TemplateAwarenessEvidence.as_str(),
            "template_awareness_evidence"
        );
        assert_eq!(EvidenceClass::EvidenceUnbound.as_str(), "evidence_unbound");
        assert_eq!(
            KnownLimitClass::ProjectModelSubsetOnly.as_str(),
            "project_model_subset_only"
        );
        assert_eq!(
            KnownLimitClass::ManagePyTargetSubsetOnly.as_str(),
            "manage_py_target_subset_only"
        );
        assert_eq!(
            KnownLimitClass::TemplateAwarenessSubsetOnly.as_str(),
            "template_awareness_subset_only"
        );
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            DowngradeAutomationClass::AutoNarrowOnUnprovenProjectModel.as_str(),
            "auto_narrow_on_unproven_project_model"
        );
        assert_eq!(
            DowngradeAutomationClass::AutoNarrowOnUnprovenManagePyTarget.as_str(),
            "auto_narrow_on_unproven_manage_py_target"
        );
        assert_eq!(
            DowngradeAutomationClass::AutoNarrowOnUnprovenTemplateAwareness.as_str(),
            "auto_narrow_on_unproven_template_awareness"
        );
        assert_eq!(
            DowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
        );
        assert_eq!(
            ConsumerSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::EvidenceClassVocabularyCollapsed.as_str(),
            "evidence_class_vocabulary_collapsed"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = DjangoAdvancedWorkflowPackTruthPacket::materialize(sample_input());
        assert_eq!(
            packet.promotion_state,
            PromotionState::Stable,
            "expected stable but got findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|f| f.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export(
                "support:m4:django_advanced_workflow_pack",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn expert_grade_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = DjangoAdvancedWorkflowPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::ExpertGradeWithUnboundBinding));
    }

    #[test]
    fn missing_workflow_loop_for_expert_grade_blocks() {
        let mut input = sample_input();
        input
            .rows
            .retain(|row| row.workflow_loop_class != WorkflowLoopClass::Review);
        let packet = DjangoAdvancedWorkflowPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingWorkflowLoopCoverage));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::StableBelowExpert;
        input.rows[0].disclosure_ref = None;
        let packet = DjangoAdvancedWorkflowPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn workflow_loop_not_applicable_on_loop_row_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, WorkflowPackRowClass::WorkflowLoop)
                && row.workflow_loop_class == WorkflowLoopClass::Create
            {
                row.workflow_loop_class = WorkflowLoopClass::NotApplicable;
                break;
            }
        }
        let packet = DjangoAdvancedWorkflowPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::WorkflowLoopNotApplicable));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != ConsumerSurface::ConformanceDashboard
        });
        let packet = DjangoAdvancedWorkflowPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_workflow_loop_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_workflow_loop_vocabulary = false;
            }
        }
        let packet = DjangoAdvancedWorkflowPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::WorkflowLoopVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = DjangoAdvancedWorkflowPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
