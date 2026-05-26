# Django Advanced workflow pack truth packet — reviewer artifact

This is the reviewer-facing artifact for the M4 stable Django
Advanced workflow pack truth packet covering the create, open, run,
test, debug, rename, and review loops with expert-grade support,
workflow loop coverage, Django project-model evidence (`settings.py`,
`urls.py`, `INSTALLED_APPS`, `manage.py`/`wsgi.py`/`asgi.py` entry
points, dependency closure), `manage.py` target parity evidence
(`runserver`, `test`, `makemigrations`, `migrate`, `shell`,
`createsuperuser`, `collectstatic`, custom management commands,
`pytest`/`pytest-django`, and `python -m pdb` /
`manage.py runserver --noreload` debug entry points), Django template
awareness evidence (Django Template Language tags `{% %}`, variable
interpolation `{{ }}`, `{% url %}`/`{% block %}`/`{% extends %}`/
`{% include %}`/`{% csrf_token %}`, custom template tags / filters,
and `app/templates/` vs project `TEMPLATES['DIRS']` resolution),
framework-migration evidence, known limits, downgrade automation, and
evidence binding. The contract lives at
`docs/languages/m4/stabilize-the-django-advanced-workflow-pack-with-manage.md`
and is replayed by
`crates/aureline-language/tests/django_advanced_workflow_pack_truth_packet.rs`.

## Stable claim

For the governed workflow pack class
(`django_advanced_workflow_pack`) the packet binds:

- at least one `pack_qualification` row (the pack's headline
  workflow-pack qualification),
- a `workflow_loop` row per certified step (create, open, run, test,
  debug, rename, review) when the pack claims `expert_grade`,
- at least one `framework_migration_row` certifying the Django 4.2
  LTS → Django 5.x migration archetype,
- at least one `project_model_row` certifying the Django project
  structure: `settings.py` configuration, `urls.py` URL conf, app
  boundary inside `INSTALLED_APPS`, `manage.py`/`wsgi.py`/`asgi.py`
  entry points, and `requirements*.txt`/`pyproject.toml` dependency
  closure,
- at least one `manage_py_target_parity_row` certifying the `manage.py`
  target surface: `runserver`, `test`, `makemigrations`, `migrate`,
  `shell`, `createsuperuser`, `collectstatic`, custom management
  commands plus `pytest`/`pytest-django` runner surface and
  `python -m pdb` / `manage.py runserver --noreload` debug entry
  points,
- at least one `template_awareness_row` certifying the Django Template
  Language surface: tag `{% %}`, variable `{{ }}`, `{% url %}`,
  `{% block %}`, `{% extends %}`, `{% include %}`, `{% csrf_token %}`,
  custom template tags / filters, and `app/templates/` vs project
  `TEMPLATES['DIRS']` resolution,
- a closed `support_class` (no surface pretends `expert_grade` while a
  binding is unbound),
- a closed `workflow_loop_class` (every expert-grade pack covers the
  full workflow loop; non-loop rows bind `not_applicable`),
- a closed `evidence_class` (archetype-repo, framework-migration,
  design-partner, fixture-repo, conformance-suite, benchmark,
  project-model, manage.py target parity, template awareness, or
  docs-disclosure),
- a closed `known_limit_class` (framework subset, language subset,
  archetype subset, migration subset, project-model subset, manage.py
  target subset, template awareness subset, unsupported runtime
  target, beta capability sample, or `none_declared`),
- a closed `downgrade_automation_class` (auto-narrow on missing
  fixture/archetype, auto-narrow on failed migration / framework gap /
  unproven project model / unproven manage.py target / unproven
  template awareness, auto-demote on low confidence, auto-block on
  missing evidence, manual-only, or `none`),
- a closed `workflow_pack_confidence_class`, and
- at least one `evidence_refs` entry plus a `disclosure_ref` whenever
  the row is not `expert_grade`, declares a non-`none_declared` known
  limit, or binds a non-`none` downgrade automation.

## Companion artifacts

- Schema: `schemas/language/django_advanced_workflow_pack_truth.schema.json`
- Checked-in packet:
  `artifacts/language/m4/django_advanced_workflow_pack_truth_packet.json`
- Fixture corpus:
  `fixtures/language/m4/django_advanced_workflow_pack_truth_packet/`
- Rust contract:
  `crates/aureline-language/src/django_advanced_workflow_pack_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-language/tests/django_advanced_workflow_pack_truth_packet.rs`
- Reviewer doc:
  `docs/languages/m4/stabilize-the-django-advanced-workflow-pack-with-manage.md`

## Required consumer projections

The packet is preserved verbatim across eight consumer projections:

| Projection                    | Surface                              |
| ----------------------------- | ------------------------------------ |
| `editor_framework_pack_panel` | Editor framework pack panel          |
| `workflow_companion`          | Workflow companion / runner panel    |
| `framework_settings`          | Framework settings / help surface    |
| `cli_headless`                | CLI/headless inspector               |
| `support_export`              | Support export bundle                |
| `release_proof_index`         | Release proof index entry            |
| `help_about`                  | Help/About proof card                |
| `conformance_dashboard`       | Conformance dashboard row            |

A projection that collapses any closed vocabulary, drops the packet
id, drops the pack class, row class, support class, workflow-loop,
known-limit, downgrade-automation, or evidence-class vocabulary, or
leaks raw private material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `expert_grade` while its support, known-limit,
  downgrade-automation, or evidence class is unbound,
- a pack that claims `expert_grade` workflow-pack support is missing a
  certified `workflow_loop` row for any of the seven required steps
  (create, open, run, test, debug, rename, review),
- a `workflow_loop` row drops its workflow-loop step binding,
- a non-`workflow_loop` row binds a workflow-loop step it cannot
  certify,
- a row narrowed below `expert_grade` or with a non-default known
  limit / non-`none` downgrade automation drops its disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies,
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`DjangoAdvancedWorkflowPackTruthPacket::materialize` and then read the
projection that matches their surface. The packet is metadata-only
and suitable for inclusion in any support export or release proof
bundle.

## Where the packet lives

- Schema: [`schemas/language/django_advanced_workflow_pack_truth.schema.json`](../../../schemas/language/django_advanced_workflow_pack_truth.schema.json)
- Reviewer doc: [`docs/languages/m4/stabilize-the-django-advanced-workflow-pack-with-manage.md`](../../../docs/languages/m4/stabilize-the-django-advanced-workflow-pack-with-manage.md)
- Fixture corpus: [`fixtures/language/m4/django_advanced_workflow_pack_truth_packet/`](../../../fixtures/language/m4/django_advanced_workflow_pack_truth_packet/)
- Rust module: [`crates/aureline-language/src/django_advanced_workflow_pack_truth_packet/mod.rs`](../../../crates/aureline-language/src/django_advanced_workflow_pack_truth_packet/mod.rs)
