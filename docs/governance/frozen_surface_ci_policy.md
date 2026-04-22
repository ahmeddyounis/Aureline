# Frozen-surface CI policy

This policy turns Aureline's M0 frozen-surface claims into a CI rule
instead of a review suggestion. A surface is not treated as truly
frozen unless:

- it has a row in
  [`/artifacts/contracts/frozen_surface_manifest.yaml`](../../artifacts/contracts/frozen_surface_manifest.yaml),
- the row names its monitored paths, validation hooks, and same-train
  follow-up obligations, and
- the shared validation lane can fail when those obligations are absent.

Machine-checkable companions:

- [`/artifacts/contracts/frozen_surface_manifest.yaml`](../../artifacts/contracts/frozen_surface_manifest.yaml)
  — canonical row set for the seeded M0 frozen surfaces.
- [`/tools/check_frozen_surfaces.py`](../../tools/check_frozen_surfaces.py)
  — direct validator entry point for the manifest and changed-surface
  obligations.
- [`/tools/ci/validate_contract_artifacts.py`](../../tools/ci/validate_contract_artifacts.py)
  — shared CI lane that embeds the frozen-surface checks alongside the
  other control-artifact checks.
- [`/fixtures/ci/contract_validation/missing_frozen_surface_metadata.json`](../../fixtures/ci/contract_validation/missing_frozen_surface_metadata.json)
  — checked-in failing scenario proving a changed frozen surface fails
  when the same change does not carry diff metadata or companion
  updates.

## Scope

This seed covers the M0 surfaces that already act like contracts even
before the runtime exists:

- command descriptors and invocation sessions,
- docs-pack manifest publication,
- exact-build identity fields,
- route taxonomy,
- object-handoff packet contract, and
- protected-path dependency and ownership rules.

The manifest may carry `provisional` rows later, but the M0 seed is
intentionally narrow and change-bearing.

## What CI enforces now

When a changed file falls under a manifest row's `monitored_paths`, CI
requires all of the following:

1. The change must carry explicit diff metadata.
   M0 satisfies this by touching the manifest row itself or a row-local
   `diff_report_ref`. The manifest touch is an explicit change-control
   acknowledgement until dedicated per-surface diff packets exist.
2. The same change must also touch at least one row-local same-train
   follow-up artifact, or it must touch a waiver/exception packet path.
3. The row's listed validation hooks must still exist and remain
   runnable through the local CI toolchain.

The current seed does not attempt semantic diff classification. It
applies a conservative rule: if you changed a file the manifest says is
part of a frozen surface, carry one explicit diff artifact and one
companion artifact, or carry a waiver/exception packet update.

## Same-train obligation classes

Rows can require one or more of these obligation classes:

- `claim_or_public_truth`
  claim rows, public-truth parity, release notices, or similar public
  truth bindings must move with the change.
- `docs_help_or_release_copy`
  docs/help route copy or release-copy governance must move with the
  change.
- `migration_or_compatibility`
  compatibility rows, lifecycle metadata, or migration guidance must
  move with the change.
- `evidence_or_release_packet`
  verification packets, release-evidence templates, artifact-graph
  rules, or similar proof-bearing packet refs must move with the
  change.
- `support_or_handoff`
  support-bundle, handoff-packet, or support-center artifacts must move
  with the change.
- `ownership_or_topology`
  topology, ownership, CODEOWNERS, or similar protected-path metadata
  must move with the change.
- `waiver_or_exception`
  exception-packet or waiver paths that document why the stronger same-
  train obligations are intentionally not met.

Rows do not need every class. They only need the classes that make the
changed surface reviewable.

## How to use the policy

When you change a monitored file:

1. Find the row in the manifest.
2. Touch one `diff_report_ref`.
3. Touch at least one same-train obligation ref for that row, or touch
   a waiver/exception path.
4. Keep the listed validation hooks passing.

Examples:

- If a command schema or contract changes, update the command diff
  narrative, then carry the same train into claim/release notes,
  compatibility/lifecycle guidance, or the support/handoff packet.
- If the docs-pack manifest changes, carry the same train into the
  reviewed-pack or late-copy policy, claim/public-truth rows, or
  support/handoff paths.
- If exact-build fields change, carry the same train into release
  evidence, claim rows, or support bundle/handoff surfaces.
- If route taxonomy changes, carry the same train into docs/help route
  copy, support/handoff surfaces, or the claim/public-truth lane.
- If the protected-path rule changes, carry the same train into
  topology/ownership metadata or an exception workflow artifact.

## Waivers and exceptions

Waivers do not suppress the need for explicit reviewable artifacts. They
replace the normal companion update requirement only when the same
change touches one of the row's `waiver_packet_refs`.

The preferred paths are:

- [`/docs/governance/templates/exception_packet_template.md`](./templates/exception_packet_template.md)
- [`/schemas/governance/exception_packet.schema.json`](../../schemas/governance/exception_packet.schema.json)
- [`/docs/governance/templates/waiver_template.md`](./templates/waiver_template.md)
- [`/schemas/release/waiver_packet.schema.json`](../../schemas/release/waiver_packet.schema.json)

If a surface changes repeatedly under waivers, the protected change-
budget workflow applies:

- [`/artifacts/governance/protected_change_budget.yaml`](../../artifacts/governance/protected_change_budget.yaml)
- [`/docs/governance/change_budget_workflow.md`](./change_budget_workflow.md)

## Local verification

Run the direct frozen-surface validator:

```bash
python3 tools/check_frozen_surfaces.py \
  --repo-root . \
  --report target/contract-validation/frozen_surface_report.json
```

Run the shared contract-validation lane, which now includes the same
frozen-surface checks:

```bash
./ci/contract_validation.sh --out-dir target/contract-validation
```

Run the checked-in failing scenario:

```bash
python3 tools/check_frozen_surfaces.py \
  --repo-root . \
  --scenario fixtures/ci/contract_validation/missing_frozen_surface_metadata.json \
  --report target/contract-validation/missing_frozen_surface_metadata_report.json
```

The scenario is expected to exit non-zero and report
`frozen_surface_manifest.diff_metadata_required`.
