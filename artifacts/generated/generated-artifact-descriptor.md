# Generated-artifact descriptor proof packet

The canonical generated-artifact descriptor packet is implemented in
[`crates/aureline-generated/src/descriptor/mod.rs`](../../crates/aureline-generated/src/descriptor/mod.rs)
and serialized to
[`artifacts/generated/generated-artifact-descriptor-packet.json`](./generated-artifact-descriptor-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/generated/generated-artifact-descriptor.md`](../../docs/generated/generated-artifact-descriptor.md)
- the boundary schema at
  [`schemas/generated/generated-artifact-descriptor.schema.json`](../../schemas/generated/generated-artifact-descriptor.schema.json)
- fixture replay in
  [`crates/aureline-generated/tests/generated_artifact_descriptor.rs`](../../crates/aureline-generated/tests/generated_artifact_descriptor.rs)
- the fixture corpus under
  [`fixtures/generated/generated-artifact-descriptor/`](../../fixtures/generated/generated-artifact-descriptor/)

## What the packet models

For each generated-artifact class — scaffolded project, notebook output,
preview/runtime derivative, API/request artifact, framework codegen,
AI-assisted edit, and exportable support packet — the packet carries one
typed descriptor and stamps the presentation the engine reaches.

## Described descriptors

| Descriptor | Class | Authority | Source | Drift | Presented | Edit posture | Ordinary source |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `generated.descriptor.scaffolded_project` | scaffolded_project | `canonical_authoritative` | `linked` | `in_sync` | `ordinary_source` | `direct_edit_allowed` | yes |
| `generated.descriptor.notebook_output` | notebook_output | `derived_readonly` | `linked` | `in_sync` | `derived_annotated` | `regenerate_only` | no |
| `generated.descriptor.preview_derivative` | preview_derivative | `derived_readonly` | `linked` | `in_sync` | `derived_annotated` | `regenerate_only` | no |
| `generated.descriptor.request_artifact` | request_artifact | `derived_editable` | `linked` | `in_sync` | `derived_annotated` | `reviewed_override_required` | no |
| `generated.descriptor.framework_codegen` | framework_codegen | `derived_editable` | `linked` | `in_sync` | `derived_annotated` | `reviewed_override_required` | no |
| `generated.descriptor.ai_assisted_edit` | ai_assisted_edit | `canonical_authoritative` | `linked` | `in_sync` | `ordinary_source` | `direct_edit_allowed` | yes |
| `generated.descriptor.support_packet` | support_packet | `derived_readonly` | `linked` | `in_sync` | `derived_annotated` | `regenerate_only` | no |

## The frozen guardrail

Hidden or missing canonical-source information blocks any ordinary-source
claim. The fixture corpus exercises a hidden canonical source (scaffolded
project → provenance withheld and a reviewed-override boundary), a missing
canonical source (AI-assisted edit → provenance withheld and a
regenerate-only boundary), drifting bytes (AI-assisted edit → provenance
withheld and a reviewed-override boundary), and uncomputed drift (framework
codegen → provenance withheld).

## Surface bindings

Every binding ingests the same packet id
(`generated.generated_artifact_descriptor.v1`) and preserves the descriptor
identity fields verbatim:

- `file_tree` — `crates/aureline-workspace/src/generated_artifacts/mod.rs`
- `search_result` — `crates/aureline-search/src/results/mod.rs`
- `review_view` — `crates/aureline-review/src/change_inspector/mod.rs`
- `ai_context` — `crates/aureline-ai/src/context_inspector/mod.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`
