# Generated-artifact certification proof packet

The canonical generated-artifact certification packet is implemented in
[`crates/aureline-generated/src/m5_generated_certification/mod.rs`](../../crates/aureline-generated/src/m5_generated_certification/mod.rs)
and serialized to
[`artifacts/generated/m5-generated-certification-packet.json`](./m5-generated-certification-packet.json).

It is the promotion-grade capstone over the
[generated-artifact governance proof packet](./m5-generated-governance.md) and
the checked-in truth source for:

- the reviewer contract in
  [`docs/generated/m5-generated-certification.md`](../../docs/generated/m5-generated-certification.md)
- the boundary schema at
  [`schemas/generated/m5-generated-certification.schema.json`](../../schemas/generated/m5-generated-certification.schema.json)
- fixture replay in
  [`crates/aureline-generated/tests/m5_generated_certification.rs`](../../crates/aureline-generated/tests/m5_generated_certification.rs)
- the fixture corpus under
  [`fixtures/generated/m5-generated-certification/`](../../fixtures/generated/m5-generated-certification/)

## What the packet certifies

For each claimed M5 publishable profile — scaffolded project, notebook output,
preview/runtime derivative, API/request artifact, and framework codegen — the
packet binds the profile's upstream claim-publication object and its backing
generated-artifact class to the four generated-artifact domains (canonical-source
visibility, writable-boundary truth, regeneration path, and restore/export
honesty) and stamps the verdict, certified maturity, and promotion decision the
narrowing engine reaches.

A profile is `certified` and promotes only when every domain is `current`.
Partial evidence narrows the claim to `beta`; stale evidence narrows it to
`preview`; missing evidence withholds the claim and holds promotion. The
certified maturity is never wider than the published claim, and never wider than
the governance lane's claim for the backing class. A profile absent from the
packet is uncertified rather than implicitly promotable.

## Certified rows

| Row | Profile | Published | Certified | Verdict | Promotion |
| --- | --- | --- | --- | --- | --- |
| `generated.certification.scaffolded_project` | scaffolded_project | `stable` | `stable` | `certified` | `promote` |
| `generated.certification.notebook_output` | notebook_output | `beta` | `beta` | `certified` | `promote` |
| `generated.certification.preview_derivative` | preview_derivative | `beta` | `beta` | `certified` | `promote` |
| `generated.certification.request_artifact` | request_artifact | `beta` | `beta` | `certified` | `promote` |
| `generated.certification.framework_codegen` | framework_codegen | `beta` | `beta` | `certified` | `promote` |

## Automatic narrowing rules

| Trigger evidence | Maturity floor | Promotion |
| --- | --- | --- |
| `partial` | `beta` | `promote_narrowed` |
| `stale` | `preview` | `promote_narrowed` |
| `missing` | `withdrawn` | `hold` |

## Failure and recovery drills

One drill per profile injects a failure into a backing domain, narrows or
withholds the claim, then recovers to `certified` after the evidence is
refreshed. The drills cover partial canonical-source visibility (scaffolded
project → beta), stale restore/export honesty (notebook output → preview), a
missing regeneration route (preview derivative → withheld / hold), a stale
writable boundary (request artifact → preview), and a missing canonical-source
linkage (framework codegen → withheld / hold).

## Publication bindings

Every binding ingests the same packet id
(`generated.m5_generated_certification.v1`) and preserves the per-row verdict,
certified maturity, promotion decision, and narrowing tokens verbatim:

- `release_shiproom` — holds promotion for any profile whose promotion decision
  is `hold` and publishes the narrowed maturity for any narrowed profile.
- `support_export` — re-exports the verdict, certified maturity, promotion
  decision, and narrowing tokens with no raw paths, credentials, or generator
  payloads.
- `docs` — quotes the certified domains, freshness rules, and verdicts.
- `help` — reuses the same vocabulary in the why-this-certified inspector.
