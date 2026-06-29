# M5 descriptor joins

A **descriptor join** carries the public-truth a [descriptor object](m5-descriptor-object.md) holds
into the copy-safe carrier shapes the support, admin, and reporting paths actually emit. The
descriptor-object and [claim-narrowing](m5-claim-narrowing.md) lanes make that truth *interactive*;
this lane makes it *portable*. It takes a descriptor object and joins it into an export packet, a
support bundle, an admin report, and a plain copy-safe summary — preserving the descriptor
identity, the typed artifact binding, and the inspectable downgrade reasons so they survive
copy/export instead of collapsing to flat text.

- Registry schema: `schemas/provenance/m5-descriptor-join.schema.json`
- Published registry: `artifacts/public-truth/m5-descriptor-join.json`
- Release parity proof: `artifacts/release/m5-descriptor-parity-proof/descriptor-join.json`
- Runtime: `crates/aureline-release/src/m5_descriptor_join/`
- Emitter: `cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_join -- registry`

## Four copy-safe carriers

Every join renders into one frozen set of carrier shapes, all carrying the same descriptor
identity, artifact binding, and downgrade-reason count:

| Carrier | Where it lands |
|---------|----------------|
| `export_packet` | A copy-safe export packet (desktop export or CLI/headless structured output). |
| `support_bundle` | A support-bundle attachment. |
| `admin_report` | An admin / fleet report row. |
| `copy_safe_summary` | A plain, copy-safe one-line summary. |

A carrier rendering records `preserves_identity`, `preserves_binding`, and
`preserves_downgrade_reasons` — all three must hold, so a narrowed claim can never read fully
supported and a binding can never be dropped on the way to an export.

## The join is derived, the carriers converge

Each `DescriptorJoin` embeds the descriptor condition and derives — never hand-authors — its
carrier truth from that descriptor's own state:

- the **claim state** is read from the shared claim-narrowing runtime, so an export packet narrows
  exactly as the interactive release card does;
- the **downgrade reasons** pair the descriptor's named narrowings with the claim-narrowing
  reasons, so every weaker mirror / offline / side-loaded / `not_provided` origin stays
  attributable (facet, value token, effect, implied state, floor) rather than disappearing into
  omission;
- the **supporting evidence references** name the artifact schema, the artifact content-digest
  ref, the descriptor schema, and the published descriptor proof packet — references only, never a
  raw payload;
- the **copy-safe summary** is a deterministic one-line restatement of the same truth.

Because the join carries no channel-specific state, the desktop UI, the CLI/headless path, and
offline / mirror-safe packet generation emit byte-identical output. The `M5DescriptorJoinRegistry`
is the one inspectable, serde-serializable truth packet every export path reads; its conformance
block proves identity and binding survive every carrier, downgrade reasons stay attributable, the
full descriptor truth is reconstructable without ad hoc translation, and the export carries no raw
provider material.

## Consumers

The registry binds the same eight public-truth consumers the sibling descriptor lanes bind —
release center, Help/About, marketplace, docs/help, certification, evaluation packs, support
exports, and companion handoffs — so support, admin, and reporting flows reconstruct one shared
truth rather than each hand-authoring an equivalent state.
