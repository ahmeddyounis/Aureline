# Extension UI: design-system foundations

Extensions that render UI consume Aureline's frozen **design-system contract
matrix** rather than inventing their own tokens, layouts, or state semantics.
This page is the entry point for the foundations, reference-layout,
state-semantic, demo-fixture, and proof-packet objects an extender reads; the
launch-critical component contracts have their
[own guidance][component-guidance].

## What you get from the contract

The [contract matrix][matrix] publishes one governed object per design-system
concept. For UI extensions the relevant objects are:

- **`foundation`** — the canonical token families (surface/text/space colors and
  scales), plus the theme, density, and motion-posture vocabularies. Render from
  the published semantic token references; never hard-code raw values.
  Canonical artifact: [`foundations.json`][foundations].
- **`reference_layout`** — the shell slots an extension can place into, their
  landmark roles, and the placeholder behavior each slot must honor (an empty
  slot names the useful next route; a loading slot reserves layout). Canonical
  artifact: [`reference-layout.json`][layout].
- **`state_semantic_family`** — the canonical state classes (`empty`, `loading`,
  `pending`, `degraded`, `blocked`, `error`, `completed`) and the non-color cues
  each requires, so an extension's states read the same as the shell's.
- **`demo_fixture`** — the checked-in [component gallery][gallery] you can render
  against in tests.
- **`proof_packet`** — the visual/token proof lanes that block drift.

## Staying inside the contract

Each object names its owner, first consumer, canonical artifact, the release
packet that keeps it current, and the proof lane that blocks drift. An extension
that maps a contract object id inherits that object's proof. An extension surface
that references a contract object which is unmapped (not published) or whose proof
is stale is gated exactly like a first-party surface: a missing mapping blocks
Stable promotion; stale proof auto-narrows the claim. See the
[contract matrix doc][matrix] for the full gate.

## Where the truth lives

- Contract matrix doc: [`docs/design-system/m5-design-system-contract-matrix.md`][matrix]
- Foundations schema: [`schemas/design-system/m5-foundations.schema.json`][foundations-schema]
- Reference-layout schema: [`schemas/design-system/m5-reference-layout.schema.json`][layout-schema]
- Component gallery: [`fixtures/ui/m5-component-gallery/`][gallery]

[component-guidance]: extension-ui-component-contracts.md
[matrix]: ../design-system/m5-design-system-contract-matrix.md
[foundations]: ../../fixtures/ui/m5-component-gallery/foundations.json
[layout]: ../../fixtures/ui/m5-component-gallery/reference-layout.json
[gallery]: ../../fixtures/ui/m5-component-gallery/
[foundations-schema]: ../../schemas/design-system/m5-foundations.schema.json
[layout-schema]: ../../schemas/design-system/m5-reference-layout.schema.json
