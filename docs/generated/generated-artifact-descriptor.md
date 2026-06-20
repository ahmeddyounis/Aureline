# Generated-artifact descriptor

This document describes the typed generated-artifact *descriptor* — the
per-file object the M5 file-tree, search, review, AI-context, and
support/export surfaces render. The canonical packet is implemented in
[`crates/aureline-generated/src/descriptor/mod.rs`](../../crates/aureline-generated/src/descriptor/mod.rs)
and serialized to
[`artifacts/generated/generated-artifact-descriptor-packet.json`](../../artifacts/generated/generated-artifact-descriptor-packet.json).

The sibling
[`generated-artifact governance`](./m5-generated-governance.md) matrix
certifies generated-artifact truth one row per *class*. This lane models the
per-*artifact* object those surfaces actually render, so the file tree, a
search result, a diff/review view, an AI context line, and a support export
all read the same identity instead of inferring it from how a file looks on
disk.

## Why this exists

A generated file looks like any other file on disk. Without one typed
descriptor, each surface can guess differently about whether a file is
authoritative or derived, who generated it, whether its bytes still match
their source, and whether a direct edit is safe. This lane makes that
identity a first-class object: every surface projects the same fields, and a
derived file is never presented as ordinary authoritative source merely
because it sits in the file tree.

## What a descriptor carries

Each [`GeneratedArtifactDescriptor`] carries:

- **`artifact_class`** — the generated-artifact class (scaffolded project,
  notebook output, preview derivative, request artifact, framework codegen,
  AI-assisted edit, or support packet).
- **`authority_class`** — the provenance class of the bytes relative to the
  source: `canonical_authoritative`, `derived_editable`, or
  `derived_readonly`.
- **`generator`** — the generator identity *with version*
  (`kind/name@version`). A generator without a version cannot prove the
  artifact came from a known, reproducible run.
- **`canonical_source`** — the canonical source the artifact derives from,
  and whether that source is `linked`, `hidden`, or `missing`.
- **`regeneration_route`** — the route that rebuilds the artifact from its
  canonical source.
- **`drift_state`** — whether the derived bytes are `in_sync`, `drifting`,
  `source_missing`, or `unknown`.
- **`declared_edit_posture`** — the writable-boundary posture declared for
  the artifact before narrowing.
- **`checkpoint_lineage_ref`** — the reversible-checkpoint lineage that
  captured the change.

## The presentation engine

One engine — `derive_descriptor_presentation` — folds those fields into a
single [`DescriptorPresentation`]:

- **`presented_authority`** — how a surface may present the artifact:
  `ordinary_source`, `derived_annotated`, or `provenance_withheld`.
- **`ordinary_source_claim_allowed`** — whether the artifact may be shown as
  ordinary authoritative source.
- **`effective_edit_posture`** — the writable-boundary posture after
  narrowing.
- **`block_reason_tokens`** — stable tokens explaining any block or
  downgrade.
- **`copy_line`** — the one stable copy/export form.

### Presented authority

| Authority class | Canonical source | Drift | Presented authority |
| --- | --- | --- | --- |
| `canonical_authoritative` | `linked` | `in_sync` | `ordinary_source` |
| `canonical_authoritative` | `linked` | not in sync | `provenance_withheld` |
| `canonical_authoritative` | `hidden`/`missing` | any | `provenance_withheld` |
| `derived_*` | `linked` | `in_sync`/`drifting` | `derived_annotated` |
| `derived_*` | `hidden`/`missing` or uncertain drift | — | `provenance_withheld` |

The marquee guardrail: **hidden or missing canonical-source information
blocks any ordinary-source claim**. An artifact is `ordinary_source` only
when it is canonical-authoritative, its source is `linked`, and it is
`in_sync`.

### Edit-posture narrowing

The effective writable-boundary posture starts at the declared posture and is
floored by the canonical-source state and the drift state; the strictest
result wins and the posture is never widened.

| Input | Edit-posture floor |
| --- | --- |
| canonical source `hidden` | `reviewed_override_required` |
| canonical source `missing` | `regenerate_only` |
| drift `drifting` | `reviewed_override_required` |
| drift `source_missing` | `regenerate_only` |
| drift `unknown` | `reviewed_override_required` |

## One identity for every surface

Each descriptor projects onto every surface through
`GeneratedArtifactDescriptor::project`. The projection embeds the shared
[`IdentityFields`] — artifact class, authority class, generator, canonical
source state, drift state, presented authority, effective edit posture, and
the ordinary-source claim — so the file tree, search, review, AI context, and
support export cannot disagree. The same
`GeneratedArtifactDescriptor::copy_line` is the single copy/export form
diagnostics and docs cite, so support exports carry the descriptor rather
than a lossy text-only summary.

Real consumers bind to the packet:

- `file_tree` — `crates/aureline-workspace/src/generated_artifacts/mod.rs`
- `search_result` — `crates/aureline-search/src/results/mod.rs`
- `review_view` — `crates/aureline-review/src/change_inspector/mod.rs`
- `ai_context` — `crates/aureline-ai/src/context_inspector/mod.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`

## Regeneration

The proof packet and fixtures are projections of the seeded packet:

```bash
cargo run -q -p aureline-generated --example dump_generated_artifact_descriptor -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/generated/generated-artifact-descriptor-packet.json
```

The fixture corpus under
[`fixtures/generated/generated-artifact-descriptor/`](../../fixtures/generated/generated-artifact-descriptor/)
is generated the same way from the `fixtures` mode and split one file per
fixture. The replay gate in
[`crates/aureline-generated/tests/generated_artifact_descriptor.rs`](../../crates/aureline-generated/tests/generated_artifact_descriptor.rs)
fails CI if the artifact or fixtures drift from the seeded packet.
