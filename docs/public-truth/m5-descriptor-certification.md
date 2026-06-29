# M5 descriptor / badge certification

The **descriptor certification** is the qualification packet *over* the shared M5 public-truth
runtime. The sibling lanes each own one slice of that runtime — the
[descriptor object](m5-descriptor-object.md) and the
[descriptor / badge matrix](m5-descriptor-badge-matrix.md), the
[badge vocabulary](m5-badge-vocabulary.md), the [claim narrowing](m5-claim-narrowing.md) state, the
[descriptor join](m5-descriptor-join.md) export carriers, the [omission guard](m5-omission-guard.md),
and the [client-scope card](m5-client-scope-card.md). This packet certifies all of them at once: it
maps every claimed M5 consumer to the runtime lanes it reads, the descriptor schemas and badge
families those lanes expose, the frozen downgrade rules that govern them, and the release-grade
parity-proof fixtures that keep them current — and auto-narrows a consumer's claim the moment any
lane it reads goes stale or failing.

- Registry schema: `schemas/provenance/m5-descriptor-certification.schema.json`
- Published certification: `artifacts/public-truth/m5-descriptor-certification.json`
- Release parity proof: `artifacts/release/m5-descriptor-parity-proof/descriptor-certification.json`
- Runtime: `crates/aureline-release/src/m5_descriptor_certification/`
- Emitter: `cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_certification -- registry`

## Seven certified runtime lanes, three dimensions

Every lane is certified with its schema, its published registry, and its release-grade parity-proof
fixture, and belongs to one of three certification dimensions — so a drift report names *which*
dimension aged out rather than collapsing the cause into one flag.

| Dimension | Runtime lanes |
|-----------|---------------|
| `descriptor_parity` | `descriptor_object`, `descriptor_badge_matrix`, `client_scope_card` |
| `badge_runtime` | `badge_vocabulary`, `descriptor_join` |
| `freshness_integration` | `claim_narrowing`, `omission_guard` |

## Mapping is derived, narrowing is deterministic

Each claimed consumer — release center, Help/About, marketplace, docs/help, certification, evaluation
packs, support exports, companion handoffs — declares the descriptor families it binds and the
runtime lanes it reads. The certification then *derives* — never hand-authors — its mapping and
verdict from that declaration and the lanes' parity-proof freshness:

- the **descriptor schemas** resolve from the bound families, and the **badge families** from those
  same families, so a consumer always maps to the current descriptor and badge vocabulary;
- the **downgrade rules** are the shared canonical rule set, filtered to the families the consumer
  binds — the certification never loosens the matrix's downgrade coverage;
- the **proof fixtures** resolve from the lanes the consumer reads, so the certification names the
  exact parity proofs backing each claim;
- the **gate** uses the matrix's semantics exactly: a lane whose parity proof is `stale` narrows
  every consumer that reads it to at most Beta; a lane whose proof is `expired`, `missing`, or
  uncertified blocks every consumer that reads it from Stable promotion, with the gap named per
  consumer (lane, dimension, and cause).

So descriptor or badge-runtime drift narrows claims deterministically instead of remaining hidden
behind local copy, and a stale or failing certification can never read fully certified.

## One certification output

The `M5DescriptorCertification` packet is the one inspectable, serde-serializable certification
truth release, support, docs, and evaluation surfaces consume rather than maintaining parallel truth
inventories. Its conformance block proves every lane is certified with a proof fixture, every
dimension is covered, every consumer maps to descriptors and proof, a stale lane narrows
deterministically while a missing lane blocks, the gaps are named per consumer, the downgrade rules
still cover every weaker descriptor value, and the export carries no raw provider material. The
checked-in `stale` and `missing` drill fixtures under `fixtures/public-truth/m5-badge-consumers/`
exercise the auto-narrowing: perturbing one lane's parity proof narrows or blocks exactly the
consumers that read it and leaves the rest certified.
