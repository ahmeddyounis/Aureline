# M5 ecosystem certification, qualification automation, and downgrade paths

This document describes the `m5_ecosystem_certification` packet — the qualification
layer that decides which marketed M5 ecosystem row may publish a support claim.

Where the install-governance matrix freezes one governance row per marketed M5 artifact
family, and the conformance-and-validators packet proves each family's *support claim* is
backed by a current, owned, evidence-linked conformance scorecard, this packet is the
aggregator that rolls every ecosystem drill into one decision per row. It is the canonical
qualification source for the M5 ecosystem-trust and install-governance lane: marketed
rows, docs badges, and support exports narrow from this packet rather than from parallel
spreadsheets.

- Typed model: `aureline-ecosystem` crate, `m5_ecosystem_certification`.
- Canonical packet: `artifacts/ecosystem/m5/m5-ecosystem-certification.json`.
- Schema: `schemas/ecosystem/m5-ecosystem-certification.schema.json`.
- Fixtures: `fixtures/ecosystem/m5/m5-ecosystem-certification/`.

## What a certification entry aggregates

Each entry covers one marketed ecosystem row and carries:

- the package kind, distribution **source class**, and runtime origin;
- the linked **conformance scorecard** and whether that scorecard is certified;
- the **owner**, **evidence freshness**, and **supported deployment and runtime
  profiles** a support claim rides on;
- one **lane evidence** record for every ecosystem drill — marketplace information,
  install review, lifecycle state, compatibility label, permission manifest, activation
  budget, mirror/private-registry parity, and rollback/quarantine — each naming the
  drill's evidence state and an opaque evidence ref; and
- the recomputed **qualification signals**, **disposition**, **effective support class**,
  and **downgrade path**.

A row cannot be certified by running a subset of the drills: validation fails unless every
lane appears exactly once.

## How the decision is recomputed

The disposition an entry publishes is not stored by hand. It is recomputed as the widest
minimum disposition over every detected signal, and the stored signals, disposition,
effective support class, and downgrade path must equal the recomputation or validation
fails.

| Signal | Forces |
| --- | --- |
| `source_class_capped` | (transparency only; no disposition change) |
| `lane_conditional` | `conditionally_qualified` |
| `lane_narrowed`, `lane_stale`, `evidence_not_current`, `supported_profiles_missing` | `downgraded` |
| `owner_missing`, `lane_missing`, `lane_failed`, `conformance_not_certified` | `disqualified` |

The effective support class is the weakest of the claimed class, the disposition ceiling,
the source-class ceiling, and the evidence-freshness ceiling, and is forced to
`unsupported` when the entry is `disqualified`.

### Source classes never inherit a broader claim

The source class is a structural cap, not an evidence-driven downgrade. A
`mirrored_registry`, `private_registry`, or `bridge_backed` row is capped at
`best_effort_supported`, a `community` row at `community_supported`, and a `side_loaded`
row at `unsupported`, regardless of how current its evidence is. A private-registry
framework pack can clear every drill and still publishes only best-effort support; it
cannot inherit the public-registry full claim. The `source_class_capped` signal makes that
cap legible without lowering the qualification disposition.

### The downgrade path is always explicit

Every entry carries a downgrade path: whether a narrowing was applied, the support class
the claim started from and landed on, the signals that explain it, and the opaque
requalification ref an owner follows to restore the claim. A narrowed entry that names no
requalification path fails validation, so a withdrawn claim always names how to recover.

## Consumer surfaces

`M5EcosystemCertification::export_projection` produces a certification index and a flat
downgrade report. Release evidence, marketplace badges, docs/help, and support exports
render the projection instead of restating qualification, support, and downgrade status by
hand, so stale ecosystem evidence cannot stay green by inertia.

## Validation

The typed model
(`m5_ecosystem_certification::M5EcosystemCertification::validate`) is canonical: it parses
the embedded packet, recomputes every entry's signals, disposition, effective support
class, and downgrade path, asserts each entry runs every drill lane, and asserts the
summary counts. A test cross-checks each entry's `conformance_certified` flag against the
linked conformance scorecard's actual disposition, so the aggregation can never drift from
its source. The JSON Schema validates the artifact's shape and closed vocabularies.
