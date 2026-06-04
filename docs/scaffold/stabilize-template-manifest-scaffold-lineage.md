# Stable Template Manifest, Scaffold Run, Health, and Lineage Contract

This contract promotes scaffold-capable lanes from starter-entry disclosure
to stable generation truth. A lane is stable only when it can produce one
inspectable packet with:

- a versioned template manifest;
- a scaffold plan that can be reviewed or exported before any write;
- an attributable scaffold run record after execution;
- a template-health report that separates blocked prerequisites, warnings,
  and optional optimizations;
- a generated-project lineage record that supports three-way update or
  rebase review.

Canonical schema:

- `schemas/scaffold/template-manifest-and-run.schema.json`

Canonical Rust API:

- `crates/aureline-scaffold::stabilize_template_manifest_scaffold_lineage`

Canonical fixtures:

- `fixtures/scaffold/stabilize-template-manifest-scaffold-lineage/`

## Stable Admission Rules

1. The template manifest declares template identity, version, publisher and
   signature refs, source class, support class, archetype, ecosystems,
   platforms, parameters, file classes, hooks/tasks, and trust/egress notes.
   A scaffold runner rejects any hook, setup task, dependency install, or
   remote action that is not declared.
2. The scaffold plan binds to one manifest revision, resolves parameter
   sources, summarizes file and directory impact, lists planned declared
   actions, names the rollback boundary, and exposes create-empty parity
   before writes.
3. Secret-bearing parameters resolve through broker handles. Raw secrets are
   not stored in manifests, preflights, histories, support exports, or
   generated-project lineage by default.
4. Template health is row-based. Rows preserve `live`, `cached`,
   `policy-evaluated`, and `unchecked` freshness states and classify each row
   as a blocked prerequisite, warning, or optional optimization.
5. Generated-project lineage is a plain reviewable metadata file in the
   generated workspace. Update and rebase review uses lineage-aware three-way
   comparison against the bound template revision and the candidate template
   revision.
6. If any of the above proof is missing, the lane is narrowed below stable
   even when the starter gallery can honestly describe the entry choice.

## Surface Mapping

Template cards read manifest source class, support class, target
ecosystems/platforms, publisher signature refs, trust/egress notes, and the
create-empty alternative. Preflight and diff surfaces read the scaffold plan
for target scope, resolved parameter sources, file impact, dependency/task
impact, rollback boundary, and planned action ids. Support and migration
flows read the run and lineage records to answer what template created a
project, what changed since generation, and whether update or rebase remains
available.

## Narrowing Rules

The stable API returns a validation error for undeclared actions, writes
before review, missing create-empty parity, raw or missing secret handles,
unbound run or lineage records, hidden lineage authority, collapsed health
freshness, or missing three-way update/rebase truth. Product surfaces should
render the narrower support class instead of inventing generic create-project
copy.
