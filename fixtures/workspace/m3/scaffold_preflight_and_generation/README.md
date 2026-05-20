# Scaffold-safety beta fixtures

Scenario fixtures for the M3 scaffold-safety beta lane. Each fixture is a
self-contained scenario that binds one
`template_generator_descriptor_record`, one `scaffold_plan_record`, and
zero-or-one `scaffold_run_record`, then asserts the derived
`ScaffoldSafetyBetaProjection` truth via an `expect` block.

The beta contract lives at
[`/docs/workspace/m3/scaffold_safety_beta.md`](../../../../docs/workspace/m3/scaffold_safety_beta.md).

The inner records validate against the three boundary schemas:

- [`/schemas/workspace/template_generator_descriptor.schema.json`](../../../../schemas/workspace/template_generator_descriptor.schema.json)
- [`/schemas/workspace/scaffold_plan.schema.json`](../../../../schemas/workspace/scaffold_plan.schema.json)
- [`/schemas/workspace/scaffold_run.schema.json`](../../../../schemas/workspace/scaffold_run.schema.json)

and the derived projection record is described by:

- [`/schemas/workspace/scaffold_safety.schema.json`](../../../../schemas/workspace/scaffold_safety.schema.json)

The integration test
[`crates/aureline-workspace/tests/scaffold_safety_beta.rs`](../../../../crates/aureline-workspace/tests/scaffold_safety_beta.rs)
replays every fixture, asserts the closed acceptance truth, and
round-trips each record through the Rust descriptors.

## Fixture shape

```jsonc
{
  "__fixture__": { "name": "...", "scenario": "...", "doc_sections": ["..."] },
  "surface": "start_center",                  // ScaffoldSurface token
  "descriptor": { /* template_generator_descriptor_record */ },
  "plan": { /* scaffold_plan_record */ },
  "run": { /* scaffold_run_record, optional */ },
  "expect": {
    "provider_class": "...",
    "signature_state": "...",
    "generation_verb": "...",
    "egress_posture": "...",
    "declared_side_effect_classes": ["..."],  // compared sorted
    "create_empty_available": false,
    "set_up_later_available": true,
    "rollback_boundary": "...",
    "rollback_automatic": true,
    "has_run": true,
    "run_outcome": "...",                      // null when no run
    "honesty_labels": ["..."],                 // compared sorted
    "guardrails_all_hold": true,
    "surface_must_disclose": true
  }
}
```

## Index

| Scenario | Verb | Highlights |
|----------|------|-----------|
| `create_project_first_party_signed` | create_project | First-party signed project template; deferred registry restore; checkpoint rollback; run succeeds. |
| `generate_component_into_existing` | generate_into_existing | Preflight-only component generator into an existing project; no egress; delete-generated rollback. |
| `ai_assisted_scaffold_governed` | create_project | AI-assisted, unsigned; AI-suggested params; governed surface; run blocks undeclared hooks. |
| `extension_provided_generator` | generate_into_existing | Extension-provided, signed-unverified; registry side effect; extension actor run. |
| `update_regenerate_migration` | update_regenerate | Template update/migration; partially-applied run; plain replay-safe lineage. |
| `offline_signed_template_no_egress` | create_project | Offline signed bundle; no egress; create-empty / set-up-later visible. |
| `remote_image_devcontainer_scaffold` | create_project | Devcontainer template; remote-image pull + devcontainer bootstrap declared before execution. |
| `failed_rolled_back_run` | create_project | Run fails mid-apply and rolls back to checkpoint; partial output stays attributable. |
| `policy_constrained_registry_allowlist` | create_project | Fleet-pinned version; registry allowlist; policy constraints surfaced in preflight. |

Removing any of these scenarios without a replacement fixture is a
breaking contract change.
