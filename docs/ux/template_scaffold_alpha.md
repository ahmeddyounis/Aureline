# Template Scaffold Alpha

This document describes the first alpha template/scaffold slice consumed by the
Start Center and workspace lineage code. It binds one TypeScript launch-wedge
starter to checked-in manifest, preflight, health, run, and generated-project
lineage records so project creation remains inspectable before any file write.

## Canonical Artifacts

- `schemas/templates/template_manifest_alpha.schema.json` defines the signed
  alpha manifest boundary: template id/version, signer/source class, support
  class, ecosystems/platforms, parameters, generated artifacts, declared hooks,
  setup tasks, trust notes, egress notes, and create-without-starter routes.
- `schemas/templates/scaffold_run_alpha.schema.json` defines the reviewed
  preflight/run boundary: target scope, file impact, dependency/setup plan,
  rollback checkpoint, review export, created/modified artifacts, invoked
  declared actions, outcome, actor, and generated-lineage reference.
- `schemas/templates/generated_project_lineage_alpha.schema.json` defines the
  plain-file lineage boundary: manifest ref, template version, generated root,
  scaffold run, checkpoint, created/modified artifacts, divergence,
  manual-edit detection, update/rebase compatibility, and last health report.
- `artifacts/compat/template_scaffold_alpha_packet.json` is the canonical
  product packet for the TypeScript Vite React starter.
- `artifacts/compat/template_health_alpha_sample.json` is the standalone health
  report sample used by release evidence and support review.

## Surface Contract

Start Center reads the packet through
`aureline_workspace::project_template_scaffold_alpha_packet` and projects it via
`aureline_shell::start_center::templates`. The row must show:

- source class, signature state, signer, support class, ecosystems, and
  supported platforms;
- required parameter count, declared hook count, and declared setup-task count;
- target scope, file/directory impact, dependency/setup-task plan, checkpoint,
  and review-export reference;
- same-weight `Create empty` and `Continue without starter` bypass paths;
- health state with separate `live`, `cached`, `policy-evaluated`, and
  `unchecked` freshness sources;
- scaffold-run id, generated-project lineage id, divergence state,
  manual-edit detection state, update/rebase compatibility, and lineage file
  reference.

## Guardrails

- No generated file write may occur before review/export.
- No undeclared hook, setup task, package install, lifecycle script, or remote
  bootstrap action may run.
- Package install remains a declared deferred action, not part of the primary
  create action.
- Generated-project lineage is a plain project file and support-export metadata;
  no hidden product database is authoritative.
- Mirror/offline source classes use the same vocabulary as public sources and
  cannot broaden trust or egress posture.
