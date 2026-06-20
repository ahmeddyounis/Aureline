# Generated-artifact Project Doctor

This is the reviewer and help surface for the generated-artifact Project Doctor
lane. The Support Center Doctor module, the headless `doctor` output, the
in-product help, the About diagnostics excerpt, and the support export all bind
to **one** findings packet and reuse the vocabulary, severity, resolution
ordering, and actions defined here. They do not re-describe generated-file
behavior in prose.

The packet is implemented in
[`crates/aureline-support/src/generated_doctor.rs`](../../crates/aureline-support/src/generated_doctor.rs)
and serialized to
[`artifacts/generated/generated-doctor-packet.json`](../../artifacts/generated/generated-doctor-packet.json).
It is a **read-only projection**: it folds the canonical write-boundary
decision packet from `aureline-generated` into Doctor findings. It never mutates
an artifact, applies a repair, or regenerates anything — the actions it surfaces
are links back to the owning descriptor, write-boundary, and regeneration-plan
objects.

## What the Doctor explains

A generated artifact is not ordinary authoritative source merely because it
looks like a file on disk. When its bytes drift from their canonical source,
when that source goes missing, when the generator that rebuilds it is
unavailable, when policy blocks regeneration, or when a direct edit is denied,
the Doctor turns the situation into one named, inspectable finding instead of a
generic save or index failure.

Every finding carries:

- the **canonical source**, **generator identity**, and **last reversible
  checkpoint** the issue traces back to, so an operator can follow it to its
  origin without raw log spelunking;
- the **boundary state**, **attempt outcome**, **effective edit gate**, and
  **why-blocked tokens** quoted verbatim from the write-boundary decision;
- the **regeneration route** and whether regeneration can run right now;
- a **controlled summary and next action** (below) plus the runtime guidance
  line;
- four **actions** — open details, compare, regenerate, and docs/help — each
  linked to the same object the runtime surfaces use.

## Resolution ordering

A single case can show more than one symptom at once: a drifted artifact also
has its direct edit held. The Doctor classifies each case into exactly one
**root-cause** finding using this frozen ordering, most-blocking first:

1. `source_missing`
2. `generator_unavailable`
3. `regeneration_blocked`
4. `drift_detected`
5. `direct_edit_denied`

An in-sync artifact whose direct edit is admitted — or admitted through a
recorded reviewed override — is healthy and produces no finding.

## Actions

Every finding offers the same four actions, in order. An action that cannot run
right now keeps its target link and a stable reason token instead of
disappearing:

| Action | Opens | Notes |
|--------|-------|-------|
| `open_details` | the [generated-artifact descriptor](./generated-artifact-descriptor.md) | always available |
| `compare` | the write-boundary three-way compare over source, current bytes, and regenerated candidate | unavailable when too few legs can be produced (e.g. a missing source) |
| `regenerate` | the [regeneration plan](./regeneration-plan.md) | unavailable when the source is missing, the generator is unavailable, or policy blocks it |
| `open_docs` | this page, anchored to the finding class | always available |

## Finding classes

Each class below is named by its stable token, its controlled summary, and its
controlled next action — the exact strings every surface renders.

<a id="source_missing"></a>

### Source missing — `source_missing`

Severity: **blocking**.

> The canonical source for this generated artifact is missing.

Next action:

> Restore the canonical source, then regenerate; the artifact cannot be compared or rebuilt without it.

Without the canonical source the artifact cannot be compared or regenerated, so
both the compare and regenerate actions are unavailable, each with a reason
token. The finding still preserves the generator identity and the last
checkpoint so the source can be recovered and the artifact rebuilt.

<a id="generator_unavailable"></a>

### Generator unavailable — `generator_unavailable`

Severity: **blocking**.

> The generator that rebuilds this artifact is unavailable.

Next action:

> Restore the generator or its runtime, then regenerate from the canonical source.

The canonical source is present, so the compare still works; regeneration is
blocked until the generator or its runtime is restored.

<a id="regeneration_blocked"></a>

### Regeneration blocked — `regeneration_blocked`

Severity: **blocking**.

> Regeneration of this artifact is blocked by policy.

Next action:

> Resolve the policy that blocks regeneration before rebuilding the artifact.

The generator and source both exist; a policy forbids the rebuild. The Doctor
surfaces the policy block rather than a generic save failure.

<a id="drift_detected"></a>

### Drift detected — `drift_detected`

Severity: **warning**.

> The generated artifact has drifted from its canonical source.

Next action:

> Compare against the canonical source, then regenerate to discard local bytes or reconcile the change into the source.

The derived bytes have diverged from their source. Both compare and regenerate
are available; the direct edit is also held, but drift is the root cause the
Doctor reports.

<a id="direct_edit_denied"></a>

### Direct-edit denied — `direct_edit_denied`

Severity: **notice**.

> A direct edit to this generated artifact was denied.

Next action:

> Regenerate from the canonical source, or escalate the edit through a reviewed override.

The artifact is in sync, but it is not its own canonical source, so a direct
edit is blocked in favor of regeneration or held for a reviewed override. The
artifact is intact; this is the mildest finding.

## Support export and redaction

The [support export](../../artifacts/generated/generated-doctor-findings.md) is
metadata-safe by construction. It carries the finding vocabulary, the
canonical-source references, the generator identity, and the checkpoint lineage,
and it excludes raw generated bytes, raw diffs, private source material, and
ambient authority or credentials. The export asserts that every finding
preserves its canonical-source lineage (a source reference, or an explicit
`source_missing` state) and its checkpoint lineage before it is considered safe
to share.

## Schema, fixtures, and replay

- Boundary schema:
  [`schemas/generated/generated-doctor.schema.json`](../../schemas/generated/generated-doctor.schema.json)
- Proof packet:
  [`artifacts/generated/generated-doctor-packet.json`](../../artifacts/generated/generated-doctor-packet.json)
- Findings report:
  [`artifacts/generated/generated-doctor-findings.md`](../../artifacts/generated/generated-doctor-findings.md)
- Fixture corpus:
  [`fixtures/generated/doctor/`](../../fixtures/generated/doctor/)
- Replay gate:
  [`crates/aureline-support/tests/generated_doctor.rs`](../../crates/aureline-support/tests/generated_doctor.rs)

Regenerate the packet and fixtures from the seeded projection with:

```bash
cargo run -p aureline-support --example dump_generated_doctor -- write
```
