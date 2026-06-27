# Extension UI: host-rendered primitives

The repeated shell primitives — placeholder cards, state blocks, review sheets,
durable job rows, boundary/origin bars, form controls, and dense collections —
ship a single **host-rendered implementation** per family in the
[host-primitive library][primitive-doc]. Every M5 surface routes through that one
implementation instead of forking its own variant, so state, boundary, and review
patterns render, behave, and prove the same across the product.

An extension or embedded surface has exactly two honest ways to present one of
these primitives.

## 1. Inherit the host-rendered primitive

Route your surface through the host primitive for its family and render it
verbatim. This is the `inherited_host_rendered` posture and is the path to full
first-party parity. You inherit the primitive's render plan for every controlled
state, its keyboard chords, its accessibility role, and its foundation token
references — you do not re-style or re-implement them. A consumer that inherits
carries **no** partial badge, because it *is* the host primitive.

Prefer this path. It is the only way an extension surface earns first-party
parity.

## 2. Declare a reduced posture behind a partial badge

If your surface genuinely cannot inherit the host primitive — because it crosses
a remote/provider boundary or renders across the extension boundary — declare the
`reduced_with_partial_badge` posture. You **must** carry a
`partial_badge_message_id` so the surface never reads as first-party parity. Only
`provider_backed` and `extension_contributed` consumers may declare a reduced
posture; a first-party surface cannot.

## The masquerade guard

There is no third option. An extension or embedded consumer cannot claim
first-party parity without either routing through the host primitive
(`inherited_host_rendered`) or carrying an explicit partial badge
(`reduced_with_partial_badge`). The
[schema][primitive-schema] and the Rust validator both reject a reduced consumer
with no badge, a first-party consumer claiming a reduced posture, and an inherited
consumer that carries a badge it must not. Every claimed M5 family surface must
route through exactly one primitive; a surface that is absent, or served by two
primitives, is a parallel implementation and fails validation.

## What you inherit, per state

Each primitive declares a render plan for every controlled state (`empty`,
`loading`, `pending`, `degraded`, `blocked`, `error`, `completed`). When you
inherit, you inherit:

- the anatomy parts the state renders and whether the state is interactive,
- the **non-color cues** the state carries — every plan includes `label_text`, so
  state meaning is never color-only (`blocked` adds a lock/shield glyph,
  `completed` a check marker),
- the appearance guarantees: the full density vocabulary, the standard / reduced
  / power-saver motion postures, both high-contrast theme classes, and preserved
  focus order and keyboard model.

Build against the per-primitive fixtures
(`fixtures/ui/m5-component-gallery/host-primitive-<kind>.json`) in your tests, and
pin the `library_version` you built against.

## Where the truth lives

- Host-primitive doc: [`docs/design-system/m5-host-primitive.md`][primitive-doc]
- Host-primitive schema: [`schemas/design-system/m5-host-primitive.schema.json`][primitive-schema]
- Component-manifest guidance: [`extension-ui-component-contracts.md`][contracts-guidance]
- Foundations guidance: [`extension-ui-design-system.md`][foundations-guidance]
- Component gallery: [`fixtures/ui/m5-component-gallery/`][gallery]

[primitive-doc]: ../design-system/m5-host-primitive.md
[primitive-schema]: ../../schemas/design-system/m5-host-primitive.schema.json
[contracts-guidance]: extension-ui-component-contracts.md
[foundations-guidance]: extension-ui-design-system.md
[gallery]: ../../fixtures/ui/m5-component-gallery/
