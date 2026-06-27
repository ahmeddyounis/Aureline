# M5 host-rendered primitives

The **host-primitive library** is the versioned, machine-readable set of
host-rendered implementations the launch-critical M5 component families render
through. Where the
[component manifests](m5-component-manifest.md) declare *what* each family is —
anatomy, states, keyboard, accessibility, and token dependencies — the
host-primitive library ships the single shared *implementation* every M5 surface
routes through, so the same state, boundary, and review patterns render
equivalently instead of as parallel per-feature variants.

- Schema: [`schemas/design-system/m5-host-primitive.schema.json`](../../schemas/design-system/m5-host-primitive.schema.json)
- Canonical library: [`fixtures/ui/m5-component-gallery/host-primitive-library.json`](../../fixtures/ui/m5-component-gallery/host-primitive-library.json)
- Per-primitive fixtures: `fixtures/ui/m5-component-gallery/host-primitive-<kind>.json`
- Release packet: [`artifacts/release/m5-design-system-proof/host-primitive-release.json`](../../artifacts/release/m5-design-system-proof/host-primitive-release.json)
- Producer: `cargo run -p aureline-design-system --bin aureline_design_system_m5_host_primitive`
- Extension guidance: [`docs/sdk/extension-ui-host-primitives.md`](../sdk/extension-ui-host-primitives.md)

## Primitives

The library publishes one host-rendered primitive per launch-critical family,
keyed by the same `component_kind` as the manifest:

| Component kind | Host-rendered primitive |
| -------------- | ----------------------- |
| `placeholder_card` | Empty-but-ready card with a next route |
| `state_block` | Controlled-state block with title, detail, and recovery |
| `review_sheet` | Modal batch-review sheet that stages a decision |
| `job_row` | Durable job / activity row |
| `boundary_bar` | Embedded-surface boundary / origin bar |
| `form_control` | Labelled input with validation semantics |
| `dense_collection` | Virtualizable collection primitive |

## What a primitive declares

Each primitive is the cite-able, host-rendered implementation for one family. It
records:

- **manifest binding** — the `component_id`, `accessibility_role`,
  `keyboard_chords`, and `token_references` it inherits from its
  [component manifest](m5-component-manifest.md). The seed builder copies these
  straight from the manifest, so the primitive is wired to the shared contract
  rather than feature-local styling, and the
  `audit_primitive_manifest_alignment` audit proves the binding holds.
- **state render plans** — one plan per
  [`CanonicalStateClass`](../../crates/aureline-design-system/src/lib.rs)
  (`empty`, `loading`, `pending`, `degraded`, `blocked`, `error`, `completed`).
  Each plan names the anatomy parts it renders, the **non-color cues** it carries
  (always including `label_text`, so state meaning is never carried by color
  alone — `blocked` adds a lock/shield glyph, `completed` a check marker, and so
  on), the status message id it announces, and whether the state is interactive.
  The `mandatory` flag mirrors the manifest's mandatory states.
- **appearance binding** — the density classes, motion postures, and contrast
  classes it preserves from the
  [foundation package](m5-foundation-package.md), plus explicit
  `honors_focus_order`, `honors_keyboard_model`, `honors_high_contrast`, and
  `honors_reduced_motion` guarantees. Every primitive honors the full density
  vocabulary, the standard/reduced/power-saver motion postures, and both
  high-contrast theme classes.
- **consumer routing** — the M5 family surfaces that route through the primitive,
  each with a **conformance posture**.

## Conformance posture and the masquerade guard

Every consumer surface declares one of two postures:

- `inherited_host_rendered` — the consumer renders the host primitive verbatim,
  for full first-party parity. It carries no partial badge.
- `reduced_with_partial_badge` — the consumer cannot inherit fully and renders a
  reduced posture. It **must** carry a `partial_badge_message_id`, and only
  `provider_backed` or `extension_contributed` consumers may declare it.

This is the load-bearing rule of the lane: an embedded or extension-backed
consumer either inherits the host-rendered primitive or declares a reduced
posture behind an explicit partial badge. It can never read as first-party parity
by any third route. The schema and the Rust validator both reject a reduced
consumer with no badge, a first-party consumer claiming a reduced posture, and an
inherited consumer that carries a badge it should not.

The library also enforces that every required M5 family surface routes through
exactly one primitive (`REQUIRED_CONSUMER_SURFACES`): a claimed surface that is
absent, or one served by two primitives, is a parallel implementation and fails
validation.

## Release-packet inclusion

`release_packet` projects a `m5_design_system_host_primitive_release` packet with
one shape-and-conformance summary per primitive (state-plan count, mandatory
count, token-reference count, and the inherited / reduced consumer counts), plus
the library totals. The release center and support exports consume this
projection.

## Privacy and boundary

Host primitives are metadata-only truth packets. They carry semantic token
*references* and message *ids* — never raw color values, credential bodies, or
provider payloads. The validator scans the serialized export for forbidden
boundary material as defense in depth.

## Drift control

The seed builder in `aureline-design-system` is the single producer of the
checked-in library fixture, the per-primitive fixtures, and the release packet,
and the inline tests assert the checked-in artifacts match the seed, validate,
align with the component manifests, and reference only foundation tokens the
foundation package publishes, so any drift fails
`cargo test -p aureline-design-system m5_host_primitive`.
