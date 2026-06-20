//! Replay and coverage gate for the declarative workspace-template corpus.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_env::{
    diff_templates, export_template_metadata, inspect_template, plan_template_change,
    seeded_workspace_template_fixtures, seeded_workspace_templates, validate_workspace_template,
    validate_workspace_template_fixture, RedactionClass, RowVerdict, TemplateLifecycleOp,
    TemplateSourceClass, WorkspaceTemplateFixture, WORKSPACE_TEMPLATE_DOC_REF,
    WORKSPACE_TEMPLATE_FIXTURE_DIR, WORKSPACE_TEMPLATE_FIXTURE_MANIFEST_REF,
    WORKSPACE_TEMPLATE_PROOF_REF, WORKSPACE_TEMPLATE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_fixtures() -> Vec<WorkspaceTemplateFixture> {
    let dir = repo_root().join(WORKSPACE_TEMPLATE_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: WorkspaceTemplateFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push(fixture);
    }
    out.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert!(!out.is_empty(), "expected at least one fixture");
    out
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_workspace_template_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_workspace_template_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        WORKSPACE_TEMPLATE_SCHEMA_REF,
        WORKSPACE_TEMPLATE_DOC_REF,
        WORKSPACE_TEMPLATE_PROOF_REF,
        WORKSPACE_TEMPLATE_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(WORKSPACE_TEMPLATE_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn seeded_templates_validate_and_their_composition_refs_exist() {
    let root = repo_root();
    for template in seeded_workspace_templates() {
        validate_workspace_template(&template).unwrap_or_else(|err| {
            panic!(
                "template {} must validate: {err}",
                template.identity.template_id
            )
        });
        // Composition references that point at checked-in artifacts or docs
        // must exist on disk.
        let refs = template
            .workflow_bundle_refs
            .iter()
            .map(|b| b.reference.clone())
            .chain(
                template
                    .archetype_defaults
                    .iter()
                    .map(|a| a.reference.clone()),
            )
            .chain(template.docs_refs.iter().map(|d| d.reference.clone()));
        for reference in refs {
            if reference.starts_with("artifacts/") || reference.starts_with("docs/") {
                assert!(
                    root.join(&reference).exists(),
                    "template {} composition ref must exist on disk: {}",
                    template.identity.template_id,
                    reference
                );
            }
        }
    }
}

#[test]
fn corpus_covers_every_source_class() {
    let fixtures = load_fixtures();
    let mut classes = BTreeSet::new();
    for fixture in &fixtures {
        classes.insert(fixture.source_class);
    }
    for required in TemplateSourceClass::ALL {
        assert!(
            classes.contains(&required),
            "corpus must cover source class {}",
            required.as_str()
        );
    }
}

#[test]
fn inspection_is_metadata_first_on_every_template() {
    for template in seeded_workspace_templates() {
        let inspection = inspect_template(&template);
        assert_eq!(inspection.redaction_class, RedactionClass::MetadataOnly);
        let export = export_template_metadata(&template);
        assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
        assert_eq!(
            export.inspection, inspection,
            "support export must wrap the canonical inspection"
        );
    }
}

#[test]
fn template_does_not_fork_its_capsule_inspection() {
    // Acceptance: hydration reuses the same environment model. The template
    // inspection embeds the exact capsule inspection a direct run produces.
    for template in seeded_workspace_templates() {
        let inspection = inspect_template(&template);
        let capsule = aureline_env::inspect_environment(&template.environment_capsule);
        assert_eq!(
            inspection.capsule_inspection, capsule,
            "template {} must not fork its capsule inspection",
            template.identity.template_id
        );
    }
}

#[test]
fn corpus_is_diffable_across_two_source_classes() {
    let templates = seeded_workspace_templates();
    let first = templates
        .iter()
        .find(|t| t.identity.template_id == "env.template.first_party")
        .expect("first-party template");
    let managed = templates
        .iter()
        .find(|t| t.identity.template_id == "env.template.managed_approved")
        .expect("managed template");
    let diff = diff_templates(first, managed);
    assert!(!diff.identical, "two source classes must differ");
    assert!(
        diff.changes.iter().any(|c| c.path == "trust.source_class"),
        "diff must surface the source-class change"
    );
}

#[test]
fn lifecycle_plans_are_reviewable_and_rollback_aware() {
    let templates = seeded_workspace_templates();
    let base = templates
        .iter()
        .find(|t| t.identity.template_id == "env.template.first_party")
        .expect("first-party template");
    let mut updated = base.clone();
    updated.identity.template_version = 2;

    let install = plan_template_change(TemplateLifecycleOp::Install, None, Some(base));
    let update = plan_template_change(TemplateLifecycleOp::Update, Some(base), Some(&updated));
    let remove = plan_template_change(TemplateLifecycleOp::Remove, Some(base), None);

    for plan in [&install, &update, &remove] {
        assert!(plan.reviewable, "every lifecycle plan must be reviewable");
        assert!(
            !plan.rollback_summary.trim().is_empty(),
            "every lifecycle plan must explain rollback"
        );
    }
    assert!(install.diff.is_none());
    assert!(update.diff.is_some(), "update must carry a diff for review");
    assert!(remove.composed_layers.is_empty());
}

#[test]
fn corpus_exercises_certified_narrowed_and_withheld_verdicts() {
    let fixtures = load_fixtures();
    let verdicts: BTreeSet<RowVerdict> = fixtures.iter().map(|f| f.expected_verdict).collect();
    for required in [
        RowVerdict::Certified,
        RowVerdict::Narrowed,
        RowVerdict::Withheld,
    ] {
        assert!(
            verdicts.contains(&required),
            "corpus must exercise {required:?}"
        );
    }
    let saw_warm_downgrade = fixtures
        .iter()
        .any(|f| !f.expected_warm_start_downgrade_tokens.is_empty());
    assert!(
        saw_warm_downgrade,
        "corpus must exercise a warm-start downgrade"
    );
    let saw_composition = fixtures
        .iter()
        .any(|f| !f.expected_composition_narrow_tokens.is_empty());
    assert!(
        saw_composition,
        "corpus must exercise composition narrowing"
    );
}
