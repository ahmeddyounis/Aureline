//! Beta coverage for the support / audit **export** workset / sparse-slice /
//! policy-limited-view scope boundaries.
//!
//! Named-workset, sparse-slice, and policy-limited-view scope truth is already
//! beta on the workspace, search, graph, refactor, and AI surfaces. This drill
//! promotes it on the support / audit export surface: it replays a frozen corpus
//! through the workspace scope-truth projection and proves a support/audit export
//! honours the declared scope the same way the sibling surfaces do —
//!
//! 1. the export **declares** the active named workset / sparse scope /
//!    policy-limited view it was produced under (an embedded
//!    [`WorksetScopeBetaSupportExport`] that itself validates and quotes the
//!    active scope class, workset name, and scope lineage);
//! 2. out-of-scope roots and policy-hidden members are **disclosed** through the
//!    scope truth's `excluded_roots` rather than silently dropped;
//! 3. policy-limited content is **redacted / labeled** with the support redaction
//!    vocabulary (`policy_locked` / `omitted_policy_locked` / `policy_denied`)
//!    rather than silently embedded.
//!
//! The corpus reuses the `aureline-workspace` `ScopeClass` vocabulary verbatim
//! and the `aureline-support` redaction vocabulary, so the export surface shares
//! one scope-truth and one redaction vocabulary instead of a divergent label set.
//! `aureline-support` already depends on `aureline-workspace`, so the test builds
//! the canonical scope artifact and projects the same scope truth the product
//! surfaces consume. `ci/check_beta_support_workset_scope.py` re-derives both
//! vocabularies from crate source and fails closed if the shared maps or the
//! required scope-class coverage drift.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_support::bundle::{
    DiagnosticDataClass, ExcludedReasonClass, HighRiskContentClass, LocalFirstDefaults,
    RedactionState, ReviewDecisionClass,
};
use aureline_workspace::{
    BetaConsumerSurface, BroadActionClass, ExcludedRootReason, ScopeClass, ScopeObservationInputs,
    WorksetArtifactRecord, WorksetScopeBetaSupportExport, WorksetScopeBetaTruth,
};

/// Canonical `aureline-workspace` scope-class vocabulary, read directly from the
/// dependency. `ci/check_beta_support_workset_scope.py` re-derives the same set
/// from crate source and fails closed if the corpus mirror drifts from it.
const WORKSPACE_SCOPE_CLASSES: [ScopeClass; 5] = [
    ScopeClass::CurrentRepo,
    ScopeClass::SelectedWorkset,
    ScopeClass::SparseSlice,
    ScopeClass::FullWorkspace,
    ScopeClass::PolicyLimitedView,
];

#[derive(Debug, Clone, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    scope_class_vocabulary_map: BTreeMap<String, String>,
    support_only_scope_classes: Vec<String>,
    required_scope_classes: Vec<String>,
    workspace_scope_class_vocabulary: Vec<String>,
    cases: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Case {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    support_scope_class: String,
    workspace_scope_class: String,
    workspace: WorkspaceBlock,
    artifact: WorksetArtifactRecord,
    export_rows: Vec<CaseExportRow>,
    expect: CaseExpect,
}

#[derive(Debug, Clone, Deserialize)]
struct WorkspaceBlock {
    workspace_root_refs: Vec<String>,
    #[serde(default)]
    workspace_root_labels: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RowMembership {
    InScope,
    OutOfScope,
    PolicyLimited,
}

#[derive(Debug, Clone, Deserialize)]
struct CaseExportRow {
    row_id: String,
    membership: RowMembership,
    diagnostic_data_class: DiagnosticDataClass,
    high_risk_content_class: HighRiskContentClass,
    redaction_state: RedactionState,
    review_decision_class: ReviewDecisionClass,
    #[serde(default)]
    excluded_reason_class: Option<ExcludedReasonClass>,
}

#[derive(Debug, Clone, Deserialize)]
struct CaseExpect {
    support_scope_label: String,
    scope_declaration_present: bool,
    in_scope_count: usize,
    out_of_scope_count: usize,
    policy_limited_count: usize,
    #[serde(default)]
    excluded_root_refs: Vec<String>,
    policy_hidden_disclosed: bool,
    export_artifact_decision: String,
    support_archive_decision: String,
}

// --------------------------------------------------------------------------- //
// The contract under test: a support / audit export that carries a scope
// declaration plus redacted content rows.
// --------------------------------------------------------------------------- //

/// One content row staged for a support/audit export, already carrying its
/// resolved redaction posture (drawn from the `aureline-support` redaction
/// vocabulary).
#[derive(Debug, Clone)]
struct RedactedExportRow {
    row_id: String,
    membership: RowMembership,
    redaction_state: RedactionState,
    decision_class: ReviewDecisionClass,
    excluded_reason: Option<ExcludedReasonClass>,
}

/// A support / audit export: an embedded scope declaration plus the redacted
/// content rows the export would carry. The scope declaration is the
/// workspace support-export projection, so the export quotes the same scope
/// truth the in-product surfaces consume.
#[derive(Debug)]
struct ScopeDeclaredSupportExport {
    scope_declaration: Option<WorksetScopeBetaSupportExport>,
    rows: Vec<RedactedExportRow>,
}

/// Typed failures a support/audit export can carry on the scope surface.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ExportScopeViolation {
    /// The export carries no embedded scope declaration at all.
    ScopeDeclarationMissing,
    /// The embedded scope declaration failed the workspace contract.
    ScopeDeclarationInvalid(String),
    /// The export does not declare a required consumer surface.
    ScopeDeclarationMissingSurface(&'static str),
    /// A policy-limited content row was embedded without a policy label.
    PolicyLimitedContentLeaked(String),
    /// A policy-limited content row carries no policy redaction label.
    PolicyLimitedRowMissingPolicyLabel(String),
    /// An in-scope content row was not actually included.
    InScopeRowNotIncluded(String),
    /// An out-of-scope content row was silently embedded in the export.
    OutOfScopeRowSilentlyIncluded(String),
}

/// Returns true when the decision embeds the row's content into the export.
fn is_included_decision(decision: ReviewDecisionClass) -> bool {
    matches!(
        decision,
        ReviewDecisionClass::IncludedDefault | ReviewDecisionClass::IncludedAfterOptIn
    )
}

/// Returns true when the redaction state embeds row content (raw or summary)
/// into the export, as opposed to keeping it out / local-only / locked.
fn redaction_state_embeds_content(state: RedactionState) -> bool {
    matches!(
        state,
        RedactionState::NotRequiredMetadata
            | RedactionState::RedactedSummary
            | RedactionState::SanitizedSnapshot
    )
}

/// True when the row's content would actually enter the export bundle.
fn is_visibly_embedded(row: &RedactedExportRow) -> bool {
    is_included_decision(row.decision_class) || redaction_state_embeds_content(row.redaction_state)
}

/// True when the row carries an explicit policy label (the support redaction
/// vocabulary's policy-locked tokens).
fn carries_policy_label(row: &RedactedExportRow) -> bool {
    row.redaction_state == RedactionState::PolicyLocked
        || row.decision_class == ReviewDecisionClass::OmittedPolicyLocked
        || row.excluded_reason == Some(ExcludedReasonClass::PolicyDenied)
}

impl ScopeDeclaredSupportExport {
    fn validate(&self) -> Vec<ExportScopeViolation> {
        let mut violations = Vec::new();
        match &self.scope_declaration {
            None => violations.push(ExportScopeViolation::ScopeDeclarationMissing),
            Some(declaration) => {
                for truth in &declaration.truths {
                    if let Err(err) = truth.validate() {
                        violations
                            .push(ExportScopeViolation::ScopeDeclarationInvalid(err.to_string()));
                    }
                }
                // A support/audit export must declare both the export-writer and
                // the support-packet surfaces it was produced under.
                if declaration.truth_for(BetaConsumerSurface::Export).is_none() {
                    violations.push(ExportScopeViolation::ScopeDeclarationMissingSurface("export"));
                }
                if declaration.truth_for(BetaConsumerSurface::SupportPacket).is_none() {
                    violations.push(ExportScopeViolation::ScopeDeclarationMissingSurface(
                        "support_packet",
                    ));
                }
            }
        }

        for row in &self.rows {
            match row.membership {
                RowMembership::InScope => {
                    if !is_included_decision(row.decision_class) {
                        violations
                            .push(ExportScopeViolation::InScopeRowNotIncluded(row.row_id.clone()));
                    }
                }
                RowMembership::PolicyLimited => {
                    if !carries_policy_label(row) {
                        violations.push(ExportScopeViolation::PolicyLimitedRowMissingPolicyLabel(
                            row.row_id.clone(),
                        ));
                    }
                    if is_visibly_embedded(row) && !carries_policy_label(row) {
                        violations.push(ExportScopeViolation::PolicyLimitedContentLeaked(
                            row.row_id.clone(),
                        ));
                    }
                }
                RowMembership::OutOfScope => {
                    if is_visibly_embedded(row) {
                        violations.push(ExportScopeViolation::OutOfScopeRowSilentlyIncluded(
                            row.row_id.clone(),
                        ));
                    }
                }
            }
        }
        violations
    }
}

// --------------------------------------------------------------------------- //
// Corpus loading
// --------------------------------------------------------------------------- //

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/support/workset_scope_export_beta")
}

fn load_manifest() -> Manifest {
    let path = corpus_dir().join("manifest.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let manifest: Manifest =
        serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
    assert_eq!(manifest.record_kind, "support_workset_scope_export_beta_manifest");
    assert_eq!(manifest.schema_version, 1);
    manifest
}

fn load_case(name: &str) -> Case {
    let path = corpus_dir().join(name);
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let case: Case =
        serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
    assert_eq!(
        case.record_kind, "support_workset_scope_export_beta_case",
        "unexpected record_kind in {name}"
    );
    assert_eq!(case.schema_version, 1, "unexpected schema_version in {name}");
    case
}

fn load_cases(manifest: &Manifest) -> Vec<Case> {
    manifest.cases.iter().map(|name| load_case(name)).collect()
}

fn scope_class_from_token(token: &str) -> ScopeClass {
    WORKSPACE_SCOPE_CLASSES
        .iter()
        .copied()
        .find(|class| class.as_str() == token)
        .unwrap_or_else(|| panic!("unknown workspace scope class token {token}"))
}

/// Derives the support export's scope label from the shared `ScopeClass`
/// chip-label vocabulary, so the export never mints a private scope label.
fn derive_scope_label(scope_class: ScopeClass, workset_name: &str) -> String {
    match scope_class {
        ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => {
            scope_class.chip_label_family().to_owned()
        }
        ScopeClass::SelectedWorkset | ScopeClass::SparseSlice | ScopeClass::PolicyLimitedView => {
            format!("{} · {}", scope_class.chip_label_family(), workset_name)
        }
    }
}

/// Projects the support/audit export's scope declaration: the workspace
/// support-export projection bundling the `Export` and `SupportPacket` consumer
/// surface truths for the case's artifact.
fn project_scope_declaration(case: &Case) -> WorksetScopeBetaSupportExport {
    let observation = || ScopeObservationInputs {
        workspace_root_refs: &case.workspace.workspace_root_refs,
        workspace_root_labels: &case.workspace.workspace_root_labels,
        parent_artifact: None,
    };
    let truths = vec![
        case.artifact
            .project_beta_truth(BetaConsumerSurface::Export, observation(), "mono:support:beta:export"),
        case.artifact.project_beta_truth(
            BetaConsumerSurface::SupportPacket,
            observation(),
            "mono:support:beta:support",
        ),
    ];
    WorksetScopeBetaSupportExport::from_truths(truths, "mono:support:beta:declaration")
        .unwrap_or_else(|err| panic!("scope declaration must bundle in {}: {err}", case.case_id))
}

fn build_rows(case: &Case) -> Vec<RedactedExportRow> {
    case.export_rows
        .iter()
        .map(|row| RedactedExportRow {
            row_id: row.row_id.clone(),
            membership: row.membership,
            redaction_state: row.redaction_state,
            decision_class: row.review_decision_class,
            excluded_reason: row.excluded_reason_class,
        })
        .collect()
}

fn export_truth(case: &Case) -> WorksetScopeBetaTruth {
    let observation = ScopeObservationInputs {
        workspace_root_refs: &case.workspace.workspace_root_refs,
        workspace_root_labels: &case.workspace.workspace_root_labels,
        parent_artifact: None,
    };
    case.artifact
        .project_beta_truth(BetaConsumerSurface::Export, observation, "mono:support:beta:export")
}

// --------------------------------------------------------------------------- //
// Tests
// --------------------------------------------------------------------------- //

#[test]
#[allow(clippy::too_many_lines)]
fn every_export_declares_its_scope_and_labels_policy_limited_content() {
    let manifest = load_manifest();
    let cases = load_cases(&manifest);
    assert!(!cases.is_empty(), "corpus must declare at least one case");

    for case in &cases {
        let scope_class = scope_class_from_token(&case.workspace_scope_class);
        assert_eq!(
            case.artifact.scope_class, scope_class,
            "artifact scope_class disagrees with declared workspace_scope_class in {}",
            case.case_id
        );

        // The export scope label is derived from the shared workspace vocabulary.
        let scope_label = derive_scope_label(scope_class, &case.artifact.workset_name);
        assert_eq!(
            scope_label, case.expect.support_scope_label,
            "scope label mismatch in {}",
            case.case_id
        );

        // The canonical scope artifact validates.
        case.artifact
            .validate()
            .unwrap_or_else(|err| panic!("artifact in {} must validate: {err}", case.case_id));

        // The export's scope declaration is the workspace support-export
        // projection; it declares the active scope class, workset name, and
        // lineage the export was produced under.
        let declaration = project_scope_declaration(case);
        assert_eq!(
            declaration.artifact_scope_class, scope_class,
            "scope declaration scope_class mismatch in {}",
            case.case_id
        );
        assert_eq!(
            declaration.artifact_workset_name, case.artifact.workset_name,
            "scope declaration workset_name mismatch in {}",
            case.case_id
        );
        assert_eq!(
            declaration.artifact_workset_ref, case.artifact.workset_id,
            "scope declaration workset_ref mismatch in {}",
            case.case_id
        );
        assert!(
            !declaration.lineage.is_empty(),
            "scope declaration must carry a lineage in {}",
            case.case_id
        );

        // The export truth carries the export_artifact / support_archive scope
        // decisions for the active scope class.
        let truth = export_truth(case);
        truth
            .validate()
            .unwrap_or_else(|err| panic!("export scope truth must validate in {}: {err}", case.case_id));
        let export_admission = truth
            .admission_for(BroadActionClass::ExportArtifact)
            .expect("export_artifact admission must exist");
        assert_eq!(
            export_admission.decision.as_str(),
            case.expect.export_artifact_decision.as_str(),
            "export_artifact decision mismatch in {}",
            case.case_id
        );
        let support_admission = truth
            .admission_for(BroadActionClass::SupportArchive)
            .expect("support_archive admission must exist");
        assert_eq!(
            support_admission.decision.as_str(),
            case.expect.support_archive_decision.as_str(),
            "support_archive decision mismatch in {}",
            case.case_id
        );

        // Out-of-scope roots are disclosed in the export truth, never silently
        // dropped.
        for excluded in &case.expect.excluded_root_refs {
            assert!(
                truth.excluded_roots.iter().any(|entry| {
                    &entry.root_ref == excluded
                        && entry.reason == ExcludedRootReason::NotInWorksetRootList
                }),
                "out-of-scope root {excluded} must be disclosed in the export truth in {}",
                case.case_id
            );
        }
        if case.expect.policy_hidden_disclosed {
            assert!(
                truth
                    .excluded_roots
                    .iter()
                    .any(|entry| entry.reason == ExcludedRootReason::PolicyHidden),
                "policy-hidden members must be disclosed in the export truth in {}",
                case.case_id
            );
        }

        // The assembled support/audit export carries the declaration plus the
        // redacted content rows, and validates clean: scope declared, in-scope
        // content included, policy-limited content labeled.
        let export = ScopeDeclaredSupportExport {
            scope_declaration: case
                .expect
                .scope_declaration_present
                .then(|| project_scope_declaration(case)),
            rows: build_rows(case),
        };
        let violations = export.validate();
        assert!(
            violations.is_empty(),
            "support/audit export must validate clean in {}: {violations:?}",
            case.case_id
        );

        // Membership tallies agree with the declared counts.
        let mut in_scope = 0_usize;
        let mut out_of_scope = 0_usize;
        let mut policy_limited = 0_usize;
        for row in &case.export_rows {
            match row.membership {
                RowMembership::InScope => {
                    // In-scope metadata rows reuse the canonical local-first
                    // default redaction posture rather than a private posture.
                    if row.diagnostic_data_class == DiagnosticDataClass::MetadataOnly {
                        let posture = LocalFirstDefaults::posture_for(
                            row.diagnostic_data_class,
                            row.high_risk_content_class,
                        );
                        assert_eq!(
                            posture.redaction_state, row.redaction_state,
                            "in-scope metadata row {} must use the default redaction state in {}",
                            row.row_id, case.case_id
                        );
                        assert_eq!(
                            posture.decision_class, row.review_decision_class,
                            "in-scope metadata row {} must use the default review decision in {}",
                            row.row_id, case.case_id
                        );
                    }
                    in_scope += 1;
                }
                RowMembership::OutOfScope => out_of_scope += 1,
                RowMembership::PolicyLimited => {
                    // Every policy-limited row carries the policy redaction label.
                    let labeled = row.redaction_state == RedactionState::PolicyLocked
                        || row.review_decision_class == ReviewDecisionClass::OmittedPolicyLocked
                        || row.excluded_reason_class == Some(ExcludedReasonClass::PolicyDenied);
                    assert!(
                        labeled,
                        "policy-limited row {} must carry a policy redaction label in {}",
                        row.row_id, case.case_id
                    );
                    policy_limited += 1;
                }
            }
        }
        assert_eq!(
            in_scope, case.expect.in_scope_count,
            "in-scope row count mismatch in {}",
            case.case_id
        );
        assert_eq!(
            out_of_scope, case.expect.out_of_scope_count,
            "out-of-scope row count mismatch in {}",
            case.case_id
        );
        assert_eq!(
            policy_limited, case.expect.policy_limited_count,
            "policy-limited row count mismatch in {}",
            case.case_id
        );

        // The scope declaration round-trips through serde, so the export is
        // genuinely exportable with the scope truth attached.
        let payload = serde_json::to_string(&declaration).expect("declaration must serialize");
        let parsed: WorksetScopeBetaSupportExport =
            serde_json::from_str(&payload).expect("declaration must round-trip");
        assert_eq!(parsed, declaration, "declaration round-trip mismatch in {}", case.case_id);
    }
}

#[test]
fn export_omitting_the_scope_declaration_fails() {
    let case = load_case("policy_limited_view_export.json");
    // An export that drops the scope declaration can never pass: a support/audit
    // export must declare the scope it was produced under.
    let export = ScopeDeclaredSupportExport {
        scope_declaration: None,
        rows: build_rows(&case),
    };
    let violations = export.validate();
    assert!(
        violations.contains(&ExportScopeViolation::ScopeDeclarationMissing),
        "export without a scope declaration must fail; got {violations:?}"
    );
}

#[test]
fn export_leaking_policy_limited_content_without_a_label_fails() {
    let case = load_case("policy_limited_view_export.json");
    let mut rows = build_rows(&case);
    // Find the policy-limited row and embed it into the export *without* a policy
    // label — exactly the silent-leak the export surface must refuse.
    let leaked = rows
        .iter_mut()
        .find(|row| row.membership == RowMembership::PolicyLimited)
        .expect("policy-limited row must exist");
    leaked.redaction_state = RedactionState::NotRequiredMetadata;
    leaked.decision_class = ReviewDecisionClass::IncludedDefault;
    leaked.excluded_reason = None;
    let leaked_id = leaked.row_id.clone();

    let export = ScopeDeclaredSupportExport {
        scope_declaration: Some(project_scope_declaration(&case)),
        rows,
    };
    let violations = export.validate();
    assert!(
        violations.contains(&ExportScopeViolation::PolicyLimitedContentLeaked(leaked_id.clone())),
        "leaking policy-limited content without a label must fail; got {violations:?}"
    );
    assert!(
        violations
            .contains(&ExportScopeViolation::PolicyLimitedRowMissingPolicyLabel(leaked_id)),
        "the leaked policy-limited row must also be flagged as unlabeled; got {violations:?}"
    );
}

#[test]
fn corpus_covers_named_workset_sparse_slice_and_policy_limited_view() {
    let manifest = load_manifest();
    let cases = load_cases(&manifest);
    let covered: BTreeSet<&str> = cases
        .iter()
        .map(|case| case.workspace_scope_class.as_str())
        .collect();
    for required in &manifest.required_scope_classes {
        assert!(
            covered.contains(required.as_str()),
            "corpus is missing a case for required scope class {required}"
        );
    }
    for required in ["selected_workset", "sparse_slice", "policy_limited_view"] {
        assert!(
            covered.contains(required),
            "corpus must cover scope class {required}"
        );
    }
}

#[test]
fn support_scope_classes_reuse_the_workspace_scope_vocabulary() {
    let manifest = load_manifest();

    // The canonical workspace vocabulary, read directly from the dependency.
    let workspace_vocab: BTreeSet<String> = WORKSPACE_SCOPE_CLASSES
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();

    // The manifest mirror of the workspace vocabulary agrees with crate source.
    let mirror: BTreeSet<String> = manifest
        .workspace_scope_class_vocabulary
        .iter()
        .cloned()
        .collect();
    assert_eq!(
        mirror, workspace_vocab,
        "manifest workspace_scope_class_vocabulary drifted from aureline-workspace ScopeClass"
    );

    // Map keys and support-only classes must be disjoint.
    for token in &manifest.support_only_scope_classes {
        assert!(
            !manifest.scope_class_vocabulary_map.contains_key(token),
            "support-only class {token} must not also appear in the mapping"
        );
    }

    // The mapping is injective and surjective over the shared vocabulary.
    let mapped: BTreeSet<String> = manifest
        .scope_class_vocabulary_map
        .values()
        .cloned()
        .collect();
    assert_eq!(
        manifest.scope_class_vocabulary_map.len(),
        mapped.len(),
        "scope-class mapping must be injective"
    );
    assert_eq!(
        mapped, workspace_vocab,
        "mapped scope classes must cover the aureline-workspace ScopeClass vocabulary exactly"
    );

    // Every mapped value resolves to a real ScopeClass and round-trips.
    for value in manifest.scope_class_vocabulary_map.values() {
        assert_eq!(
            scope_class_from_token(value).as_str(),
            value,
            "mapped workspace class {value} must round-trip through ScopeClass"
        );
    }

    // Each case's declared workspace scope class agrees with the shared map.
    for case in load_cases(&manifest) {
        let mapped = manifest
            .scope_class_vocabulary_map
            .get(&case.support_scope_class)
            .unwrap_or_else(|| {
                panic!(
                    "case {} uses unmapped support scope class {}",
                    case.case_id, case.support_scope_class
                )
            });
        assert_eq!(
            mapped, &case.workspace_scope_class,
            "case {} workspace_scope_class disagrees with the vocabulary map",
            case.case_id
        );
    }
}
