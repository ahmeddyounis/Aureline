# M5 claim narrowing

**Claim narrowing** is the layer that binds descriptor freshness and client-scope truth to the
surfaces that publish user-visible support or capability claims. It takes an underlying evidence
condition — a [descriptor object](m5-descriptor-object.md) — and projects the one controlled
degraded-claim state that condition implies onto every public-truth consumer, so a stale or
narrowed supporting descriptor cannot leave a release card, marketplace row, docs badge,
evaluation-pack summary, or companion handoff summary green by accident.

- Registry schema: `schemas/provenance/m5-claim-narrowing.schema.json`
- Published registry: `artifacts/public-truth/m5-claim-narrowing.json`
- Release parity proof: `artifacts/release/m5-descriptor-parity-proof/claim-narrowing.json`
- Runtime: `crates/aureline-release/src/m5_claim_narrowing/`
- Emitter: `cargo run -q -p aureline-release --bin aureline_release_m5_claim_narrowing -- registry`

## Controlled degraded-claim states

Every consumer publishes one state from a single frozen vocabulary, derived from the descriptor's
narrowings rather than hand-authored per surface:

| State | Meaning | Effective qualification |
|-------|---------|-------------------------|
| `fully_supported` | No supporting descriptor narrowed the claim; it stands at its ceiling. | claimed class (e.g. `stable`) |
| `limited` | Evidence or provenance is present but limited in scope. | `beta` |
| `retest_pending` | A retest is pending before the claim can be relied on. | `beta` |
| `evidence_stale` | Backing evidence fell out of its freshness window. | `beta` |
| `unsupported_client` | The client scope cannot carry the claimed capability or authority. | `beta` |
| `unsupported` | A blocking condition holds the claim from public truth entirely. | `unavailable` |

Declaration order is least → most degraded. When a condition carries more than one narrowing the
most severe applicable state wins, and every narrowing still surfaces as its own reason — a
weaker mirror, offline, side-loaded, or `not_provided` origin can never disappear into omission.

## The state is derived, the surfaces converge

Each `ClaimNarrowingCase` embeds the descriptor condition, derives the `canonical_claim_state`
and `canonical_effective_qualification` from that descriptor's own narrowings, and then projects
the same state onto every consumer:

- `release_center` — release/help provenance card
- `help_about` — Help/About support row
- `marketplace` — marketplace listing row
- `docs_help` — docs/help reference badge
- `certification` — certification claim row
- `evaluation_packs` — evaluation-pack claim summary
- `support_export` — support-export claim line
- `companion_handoff` — companion handoff summary

Because every projection is derived from one descriptor through one function, the surfaces
**converge**: the same evidence condition yields the same degraded state everywhere. Each
projection records `converges_with_canonical`, and the registry conformance review rejects any
projection that diverges or that reads `fully_supported` while its descriptor carries a narrowing.

## Why it narrowed, and what would restore it

The downgrade is never a silent relabel. Each narrowing produces an inspectable
`ClaimNarrowingReason` (the `facet`, the value `token`, the `effect`, and the `implied_state`) and
a paired `RestorationStep` naming the action that would restore the claim:

| Restoration action | Restores | Triggered by |
|--------------------|----------|--------------|
| `refresh_evidence` | Re-run or refresh the backing evidence. | stale / expired / missing freshness, retest pending |
| `complete_evidence` | Supply the missing or partial evidence. | limited / partial / not-provided evidence |
| `provide_provenance` | Attach first-party signed and attested provenance. | weaker origin or signature |
| `use_desktop_client` | Perform the action on the full desktop client or via its handoff target. | narrowed client kind, authority, or required handoff |

A user or support engineer can therefore read both why a claim narrowed and what would lift it.

## One runtime, every consumer

`M5ClaimNarrowingRegistry` is the single inspectable, serde-serializable truth packet every
public-truth consumer reads. It carries metadata and refs only — no credential bodies or raw
provider payloads — and a redaction scan runs as part of validation.
