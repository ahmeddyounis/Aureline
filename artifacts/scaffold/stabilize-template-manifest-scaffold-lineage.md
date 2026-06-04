# Stable Scaffold Lineage Proof Packet

This packet records the stable scaffold truth source introduced for template
manifests, scaffold preflights, scaffold runs, template-health reports, and
generated-project lineage.

## Canonical Artifacts

| Artifact | Path |
|---|---|
| Rust contract | `crates/aureline-scaffold/src/stabilize_template_manifest_scaffold_lineage/mod.rs` |
| Schema | `schemas/scaffold/template-manifest-and-run.schema.json` |
| Fixture corpus | `fixtures/scaffold/stabilize-template-manifest-scaffold-lineage/` |
| User-facing contract | `docs/scaffold/stabilize-template-manifest-scaffold-lineage.md` |

## Evidence Summary

- Template manifests declare source/support class, publisher/signature refs,
  archetype, ecosystems, platforms, required parameters, file classes,
  hooks/tasks, and trust/egress posture.
- Scaffold plans must be reviewable or exportable before write and must carry
  target scope, parameter sources, file impact, planned action ids, rollback
  boundary, and create-empty parity.
- Scaffold runs bind to plan and manifest refs, preserve actor/workset
  identity, created/modified artifact refs, invoked action ids, checkpoint
  ref, and outcome.
- Template-health reports preserve blockers, warnings, optional
  optimizations, skipped checks, fix guidance, runtime/platform scope, and
  `live` / `cached` / `policy-evaluated` / `unchecked` freshness states.
- Generated-project lineage binds run, manifest, workset, divergence state,
  update/rebase compatibility, and latest health report ref in plain
  reviewable metadata.

## Stable-Lane Decision

A scaffold-capable surface may claim stable generation only when
`StableScaffoldPacket::validate_stable()` succeeds for the packet backing the
card, preflight, run, health report, and lineage projection. Any failure
narrows the lane below stable until the missing proof is supplied.
