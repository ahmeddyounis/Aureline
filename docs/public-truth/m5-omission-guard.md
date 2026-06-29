# M5 no-silent-omission guard

The **omission guard** fixes the last failure mode the descriptor lanes leave open: a consumer that
quietly *drops* a weaker state — a Mirrored / Offline / Side-loaded origin, a `not_provided` value,
or partial / stale evidence — so a narrowed surface reads cleaner than it is. The
[descriptor object](m5-descriptor-object.md) lane freezes the typed truth, the
[claim-narrowing](m5-claim-narrowing.md) lane derives the one degraded-claim state it implies, and
the [descriptor join](m5-descriptor-join.md) lane proves the truth survives copy/export. This lane
proves the truth can never *disappear*: it freezes one weaker-evidence-state vocabulary and a rule
that every public-truth consumer must render the same present states, the same way.

- Registry schema: `schemas/provenance/m5-omission-guard.schema.json`
- Published registry: `artifacts/public-truth/m5-omission-guard.json`
- Release parity proof: `artifacts/release/m5-descriptor-parity-proof/omission-guard.json`
- Runtime: `crates/aureline-release/src/m5_omission_guard/`
- Emitter: `cargo run -q -p aureline-release --bin aureline_release_m5_omission_guard -- registry`

## One weaker-evidence-state vocabulary

Every negative, partial, or non-authoritative origin/evidence condition resolves to exactly one
stable token, one user-facing label, and one explanation message id, so the state reads identically
wherever it is surfaced. The authoritative `official` origin is part of the same vocabulary: the
absence of weakening is *stated*, never left blank.

| State | Label | Weakening |
|-------|-------|-----------|
| `official` | Official | anchor |
| `vendor` | Vendor | yes |
| `community` | Community | yes |
| `mirrored` | Mirrored | yes |
| `offline` | Offline | yes |
| `side_loaded` | Side-loaded | yes |
| `unverified` | Unverified | yes |
| `partial` | Partial | yes |
| `retest_pending` | Retest pending | yes |
| `stale` | Stale | yes |
| `expired` | Expired | yes |
| `missing` | Missing | yes |
| `scoped_client` | Scoped client | yes |
| `handoff_required` | Handoff required | yes |
| `not_provided` | Not provided | yes |

Each present state names the descriptor `facet:token` pairs that put it in the set, so the
derivation is auditable rather than asserted.

## The present set is derived, and never empty

Each `OmissionGuardCase` reads a descriptor condition and derives — never hand-authors — the set of
present states from that descriptor's own facets. Because the origin always sources exactly one
state, the present set is **never empty**: a clean surface still states `official`. Weakening is
present on a surface exactly when the shared claim-narrowing runtime narrows the claim, so a stale,
mirrored, side-loaded, or not-provided condition can never read fully supported.

## The no-silent-omission rule

The present set is projected onto every public-truth consumer, and the per-case `guard` block is the
rule:

- **no consumer omits a present state** — a consumer's rendered set must contain every present
  state; dropping one is a `silent_omission` violation;
- **no consumer invents an absent state** — the rendered set may not exceed the present set;
- **one vocabulary across consumers** — every consumer renders the same label and resolves the same
  explanation message id for a given state;
- **weakening aligns with the shared claim** — weakening is present exactly when the claim is
  narrowed.

The `M5OmissionGuardRegistry` is the one inspectable, serde-serializable truth packet every consumer
reads; its conformance block proves the mirror / offline / side-loaded origins are first-class, a
`not_provided` value is never hidden, partial states are surfaced, the official anchor stays
explicit, and the export carries no raw provider material.

## Consumers

The registry binds the same eight public-truth consumers the sibling descriptor lanes bind — release
center, Help/About, marketplace, docs/help, certification, evaluation packs, support exports, and
companion handoffs — so support and docs surfaces stay aligned with release and marketplace truth
under degraded evidence conditions, rather than each hand-authoring an equivalent state that could
silently drop a weaker one.
