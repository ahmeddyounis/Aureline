# Template Registry, Framework-Pack, and Support-Class Matrix

This document is the contract for the frozen matrix that qualifies four template
and framework lanes — a signed template registry, a scaffold planner, framework
packs, and archetype health bundles. The matrix is the canonical control source
for this lane: galleries, diff/run/recovery surfaces, docs, Help/About surfaces,
and support exports ingest the checked-in packet rather than cloning status text.

- Record kind: `freeze_template_registry_framework_pack_and_support_class_matrix`
- Schema: [`schemas/templates/freeze-the-m5-template-registry-framework-pack-and-support-class-matrix.schema.json`](../../../schemas/templates/freeze-the-m5-template-registry-framework-pack-and-support-class-matrix.schema.json)
- Canonical support export: [`artifacts/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/support_export.json`](../../../artifacts/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/support_export.json)
- Summary artifact: [`artifacts/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix.md`](../../../artifacts/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix.md)
- Fixtures: [`fixtures/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/`](../../../fixtures/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/)
- Producer: `aureline_scaffold::freeze_the_m5_template_registry_framework_pack_and_support_class_matrix::current_stable_template_framework_matrix_export`

## Lanes

| Lane | Qualification | Support class | Generation truth | Source contract |
| --- | --- | --- | --- | --- |
| `signed_template_registry` | Stable | Officially supported | Authored | [`schemas/templates/template_registry_entry.schema.json`](../../../schemas/templates/template_registry_entry.schema.json) |
| `scaffold_planner` | Stable | Officially supported | Mixed authored/generated | [`schemas/templates/scaffold_run_alpha.schema.json`](../../../schemas/templates/scaffold_run_alpha.schema.json) |
| `framework_pack` | Beta | Community supported | Runtime only | [`schemas/language/framework_pack_descriptor.schema.json`](../../../schemas/language/framework_pack_descriptor.schema.json) |
| `archetype_health_bundle` | Stable | Officially supported | Generated | [`schemas/governance/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.schema.json`](../../../schemas/governance/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.schema.json) |

Each lane row binds a qualification class to its support class, its
authored-versus-generated-versus-runtime-only truth, its evidence requirement,
required evidence packet refs, downgrade triggers, rollback posture, source
contracts, and the consumer surfaces that must project the lane's qualification
truth from gallery through diff, run, and recovery.

## Track invariant

Template source, generator version, pack version, authored/generated/runtime-only
truth, and archetype health state stay inspectable from gallery through diff, run,
and recovery. The `trust_review` block encodes these as hard invariants — all must
hold for the matrix to validate:

- `template_source_provenance_inspectable`, `generator_and_pack_versions_inspectable`,
  and `signed_registry_signatures_verified` — provenance, versions, and signature
  verification are visible before a template is offered.
- `scaffold_diff_preview_before_write` and `rollback_boundary_visible` — the
  scaffold planner previews its file and directory impact and shows its rollback
  boundary before any write.
- `authored_generated_runtime_truth_explicit` and
  `support_class_and_downgrade_cues_current` — authored, generated, and runtime-only
  content stay separated, and support-class and downgrade cues stay current.
- `heuristic_never_presented_as_exact` — framework packs never present heuristic or
  bridge behavior as exact first-party truth.
- `archetype_health_partitioned`, `no_credential_bodies_in_export`,
  `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected lane.
`apply_downgrade_automation` consumes per-lane observations: invalid evidence
holds a lane (qualification `held`, support class `narrowed_below_stable`, no
Stable evidence obligation), while stale proof or a narrowed upstream narrows a
Stable lane to Beta with a `narrowed_below_stable` support class. The supported
downgrade triggers are `proof_stale`, `policy_blocked`, `signature_unverified`,
`template_revision_unavailable`, `scaffold_preview_unavailable`,
`support_class_narrowed`, `heuristic_presented_as_exact`, `archetype_health_stale`,
`lineage_truth_missing`, `scope_expansion_unqualified`, and
`upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/)
show a framework pack narrowed on a dropped support class and a held archetype
health bundle; both remain valid packets because narrowing is explicit, not
hidden.

## Boundary

Template source bodies, raw generator output, raw provider payloads, credentials,
and secret values never cross this boundary. The packet carries only metadata,
qualification truth, support-class truth, generation truth, and contract
references. Starter convenience never outruns provenance, preview, or rollback.
