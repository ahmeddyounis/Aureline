# Generated-artifact governance proof packet

The canonical generated-artifact governance packet is implemented in
[`crates/aureline-generated/src/m5_generated_governance/mod.rs`](../../crates/aureline-generated/src/m5_generated_governance/mod.rs)
and serialized to
[`artifacts/generated/m5-generated-proof-packet.json`](./m5-generated-proof-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/generated/m5-generated-governance.md`](../../docs/generated/m5-generated-governance.md)
- the boundary schema at
  [`schemas/generated/m5-generated-governance.schema.json`](../../schemas/generated/m5-generated-governance.schema.json)
- fixture replay in
  [`crates/aureline-generated/tests/m5_generated_governance.rs`](../../crates/aureline-generated/tests/m5_generated_governance.rs)
- the fixture corpus under
  [`fixtures/generated/m5-generated-governance/`](../../fixtures/generated/m5-generated-governance/)

## What the packet certifies

For each claimed M5 generated-artifact class — scaffolded project, notebook
output, preview/runtime derivative, API/request artifact, framework codegen,
AI-assisted edit, and exportable support packet — the packet proves the seven
required provenance dimensions (canonical source, generator identity, provenance
class, writable boundary, regeneration route, drift state, and checkpoint lineage)
and stamps the verdict and writable-boundary posture the narrowing engine reaches.

A class is `certified` only when every dimension is `current`. Partial evidence
narrows the claim to `beta`; stale evidence narrows it to `preview`; missing
evidence withholds the claim. Stale or partial canonical-source / writable-boundary
evidence additionally narrows the writable-boundary posture. The certification only
narrows — it never widens a claim, and a class absent from the packet is
uncertified rather than implicitly authoritative.

## Certified rows

| Row | Class | Claimed | Effective | Verdict | Edit posture |
| --- | --- | --- | --- | --- | --- |
| `generated.artifact.scaffolded_project` | scaffolded_project | `stable` | `stable` | `certified` | `direct_edit_allowed` |
| `generated.artifact.notebook_output` | notebook_output | `beta` | `beta` | `certified` | `regenerate_only` |
| `generated.artifact.preview_derivative` | preview_derivative | `beta` | `beta` | `certified` | `regenerate_only` |
| `generated.artifact.request_artifact` | request_artifact | `beta` | `beta` | `certified` | `reviewed_override_required` |
| `generated.artifact.framework_codegen` | framework_codegen | `beta` | `beta` | `certified` | `reviewed_override_required` |
| `generated.artifact.ai_assisted_edit` | ai_assisted_edit | `stable` | `stable` | `certified` | `direct_edit_allowed` |
| `generated.artifact.support_packet` | support_packet | `stable` | `stable` | `certified` | `regenerate_only` |

## Automatic narrowing rules

| Trigger evidence | Maturity floor | Edit-posture floor (canonical-source/writable-boundary only) |
| --- | --- | --- |
| `partial` | `beta` | `reviewed_override_required` |
| `stale` | `preview` | `regenerate_only` |
| `missing` | `withdrawn` | `regenerate_only` |

## Failure and recovery drills

One drill per class injects a failure into a backing dimension, narrows or
withholds the claim, then recovers to `certified` after the evidence is refreshed.
The drills cover partial canonical-source coverage (scaffolded project → beta and a
reviewed override), a stale writable boundary (AI-assisted edit → preview and a
forced regenerate-only boundary), a missing regeneration route (framework codegen →
withheld), a stale provenance class (request artifact → preview), undetected drift
(notebook output → preview), a stale generator identity (preview derivative →
preview), and a broken checkpoint lineage (support packet → preview).

## Publication bindings

Every binding ingests the same packet id (`generated.m5_generated_governance.v1`)
and preserves the per-row verdict, effective maturity, writable-boundary posture,
and narrowing tokens verbatim:

- `release_shiproom` — holds promotion for any narrowed or withheld release-scope
  class.
- `support_export` — re-exports the verdict, writable-boundary posture, and
  narrowing tokens with no raw paths, credentials, or generator payloads.
- `docs` — quotes the certified dimensions, freshness and edit-boundary rules, and
  verdicts.
- `help` — reuses the same vocabulary in the why-this-artifact inspector.
