# Generated-artifact governance

This document describes the generated-artifact provenance, regeneration,
writable-boundary, and reversible-checkpoint governance lane for claimed M5
artifact classes. The canonical packet is implemented in
[`crates/aureline-generated/src/m5_generated_governance/mod.rs`](../../crates/aureline-generated/src/m5_generated_governance/mod.rs)
and serialized to
[`artifacts/generated/m5-generated-proof-packet.json`](../../artifacts/generated/m5-generated-proof-packet.json).

It composes the generated-artifact-relevant packets already frozen on the M5
line:

- the template-manifest scaffold lineage at
  [`artifacts/scaffold/stabilize-template-manifest-scaffold-lineage.md`](../../artifacts/scaffold/stabilize-template-manifest-scaffold-lineage.md),
- the template-health states at
  [`artifacts/scaffolding/template_health_states.yaml`](../../artifacts/scaffolding/template_health_states.yaml),
- the experiment-provenance and result-comparison packet at
  [`artifacts/data/qualify-experiment-provenance-and-result-comparison.json`](../../artifacts/data/qualify-experiment-provenance-and-result-comparison.json),
- the coverage/profile/notebook evidence-handoff artifact lineage at
  [`artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json`](../../artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json),
- the save/review choice matrix at
  [`artifacts/fs/save_review_choice_matrix.yaml`](../../artifacts/fs/save_review_choice_matrix.yaml),
- the mutation classes at
  [`artifacts/change/mutation_classes.yaml`](../../artifacts/change/mutation_classes.yaml),
- the reversible rollback-checkpoint example at
  [`artifacts/migration/rollback_checkpoint_examples/checkpoint_created_pre_apply.yaml`](../../artifacts/migration/rollback_checkpoint_examples/checkpoint_created_pre_apply.yaml),
- the restore-provenance packet at
  [`artifacts/migration/m3/restore_provenance_packet.md`](../../artifacts/migration/m3/restore_provenance_packet.md).

## Why this exists

The M5 line already covers generated-artifact lineage surfaces, template lineage,
refactor transaction policy, mutation journals, and restore provenance. What it
leaves implicit is the actual *generated-artifact contract*: the canonical source
a derived file points back to, the generator that produced it, the provenance
class it carries, the writable-boundary policy that decides whether a direct edit
is safe, the regeneration route that rebuilds it, the drift state that says
whether the bytes still match their source, and the reversible-checkpoint lineage
that captured the change. Without one governed matrix, a scaffolded project, a
notebook output, a preview/runtime derivative, an API/request artifact, framework
codegen, an AI-assisted edit, or an exportable support packet can each guess
differently about what is authoritative, what may be written directly, what must
be regenerated, and what local history actually captured.

This lane closes that loophole. It turns generated-artifact truth into a
promotion-grade claim per claimed class and narrows both the claim and the
writable-boundary posture automatically when the backing evidence goes partial,
stale, or missing.

## The certified dimensions

Every claimed class must prove seven provenance dimensions. A surface may not
present a generated artifact as authoritative unless all seven are canonical and
testable:

- **`canonical_source`** — the artifact declares the canonical source it derives
  from, so a derived file is never mistaken for its own source of truth.
- **`generator_identity`** — the artifact declares the generator that produced it.
- **`provenance_class`** — the artifact carries a typed provenance class instead of
  leaving its authority implicit.
- **`writable_boundary`** — the artifact declares the writable-boundary policy that
  decides whether a direct edit is allowed, reviewed, or blocked.
- **`regeneration_route`** — the artifact declares the regeneration route that
  rebuilds it from its canonical source.
- **`drift_state`** — the artifact declares whether the derived bytes still match
  their canonical source.
- **`checkpoint_lineage`** — the artifact declares the reversible-checkpoint lineage
  that captured the change, including what was captured, omitted, or rederived.

## The narrowing engine

Each dimension carries an `evidence_state`. One engine —
`certify_artifact_outcome` — folds the per-dimension evidence into a single
verdict, an effective maturity floor, **and** a narrowed writable-boundary
posture. It is the only place the downgrade rule lives; the rows, the drills, the
fixtures, the freshness rules, and the edit-boundary rules all read it.

| Evidence state | Maturity floor | Edit-posture floor (canonical-source/writable-boundary only) |
| --- | --- | --- |
| `current` | none | none |
| `partial` | `beta` | `reviewed_override_required` |
| `stale` | `preview` | `regenerate_only` |
| `missing` | `withdrawn` | `regenerate_only` |
| `not_applicable` | none | none |

The effective maturity is the worst (narrowest) of the claimed maturity and every
triggered floor. The writable-boundary posture is narrowed the same way, but only
the canonical-source and writable-boundary dimensions govern it — a direct edit is
trustworthy only while the artifact's canonical-source linkage and its boundary
policy are current. The verdict follows:

- **`certified`** — the effective maturity equals the claimed maturity.
- **`narrowed`** — the effective maturity is below the claimed maturity but the
  claim still holds (beta or preview).
- **`withheld`** — a required dimension is missing, so the claim is withdrawn.

The certification only ever narrows. It never promotes an artifact above its
claimed maturity or writable-boundary posture, and a class absent from the packet
is uncertified rather than implicitly authoritative.

## Certified classes

| Class | Authority | Claimed maturity | Claimed edit posture |
| --- | --- | --- | --- |
| `scaffolded_project` | `canonical_authoritative` | `stable` | `direct_edit_allowed` |
| `notebook_output` | `derived_readonly` | `beta` | `regenerate_only` |
| `preview_derivative` | `derived_readonly` | `beta` | `regenerate_only` |
| `request_artifact` | `derived_editable` | `beta` | `reviewed_override_required` |
| `framework_codegen` | `derived_editable` | `beta` | `reviewed_override_required` |
| `ai_assisted_edit` | `canonical_authoritative` | `stable` | `direct_edit_allowed` |
| `support_packet` | `derived_readonly` | `stable` | `regenerate_only` |

In the checked-in packet every dimension is `current`, so every class is
`certified` at its claimed maturity and writable-boundary posture.

## Derived bytes are not the source

The marquee guardrail: a `direct_edit_allowed` claim drops to a reviewed override
or a regenerate-only boundary whenever the canonical-source linkage or
writable-boundary policy outruns current truth. A stale writable boundary narrows
the claim to `preview` **and** forces a `regenerate_only` boundary, so a generated
artifact is never presented as ordinary authoritative source merely because it
looks like a file on disk.

## Failure and recovery drills

Each class carries one failure / recovery drill. A drill injects a failure into
one dimension, observes the degraded evidence, watches the claim narrow or
withhold (and the writable-boundary posture downgrade where applicable), refreshes
the evidence, and recovers to `certified`. The degraded posture is computed from
the same engine the rows use, so a drill can never disagree with the
certification. The drill set covers partial canonical-source coverage (scaffolded
project → beta + reviewed override), a stale writable boundary (AI-assisted edit →
preview + regenerate-only), a missing regeneration route (framework codegen →
withheld), a stale provenance class (request artifact → preview), undetected drift
(notebook output → preview), a stale generator identity (preview derivative →
preview), and a broken checkpoint lineage (support packet → preview).

## One packet for every surface

Release/shiproom, support export, docs, and help all bind to this packet rather
than re-deriving generated-artifact staleness. Each binding preserves the per-row
verdict, effective maturity, writable-boundary posture, and narrowing tokens
verbatim, and narrows in lockstep with the packet, so the product tells one
consistent story about what is authoritative, what may be written directly, and
what must be regenerated.

## Regeneration

The proof packet and fixtures are projections of the seeded packet:

```bash
cargo run -q -p aureline-generated --example dump_m5_generated_governance -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/generated/m5-generated-proof-packet.json
```

The fixture corpus under
[`fixtures/generated/m5-generated-governance/`](../../fixtures/generated/m5-generated-governance/)
is generated the same way from the `fixtures` mode and split one file per fixture.
The replay gate in
[`crates/aureline-generated/tests/m5_generated_governance.rs`](../../crates/aureline-generated/tests/m5_generated_governance.rs)
fails CI if the artifact or fixtures drift from the seeded packet.
