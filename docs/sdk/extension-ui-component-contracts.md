# Extension UI: component contracts

Launch-critical components are governed objects in Aureline's frozen
**design-system contract matrix**. An extension that ships or extends one of these
components declares the same contract the first-party component does, so the
component reads, behaves, and proves the same across the product.

## The component contract

Each `component_contract` object in the [contract matrix][matrix] points at a
canonical artifact in the [component gallery][gallery] that declares the full
contract:

- **anatomy** — the named parts of the component and their roles.
- **states** — the state-semantic families the component renders (`empty`,
  `loading`, `pending`, `degraded`, `blocked`, `error`, `completed`).
- **keyboard model** — the key chords and the actions they trigger.
- **accessibility** — the component role, the screen-reader label rule, and the
  focus-order rule (focus follows visual order and returns to the invoker on
  dismissal).
- **token dependencies** — the semantic token references the component renders
  from.
- **extension guidance** — this page.

The shape is governed by
[`m5-component-contract.schema.json`][component-schema]; the gallery fixtures are
checked-in instances you can render against in tests.

## Launch-critical component families: the manifest package

The reused M5 component families — placeholder cards, state blocks, review
sheets, job rows, boundary bars, form controls, and dense collection primitives —
ship a richer, versioned **component-manifest package** governed by
[`m5-component-manifest.schema.json`][manifest-schema]. Each manifest is the
single, cite-able contract for one family and adds, on top of the fields above:

- **mandatory vs. optional states** — the manifest splits the canonical state set
  into the states a component MUST render and those it MAY render, so you do not
  have to guess which states are required.
- **labels and commands** — governed label message ids and the commands the
  component offers (each with its key chord).
- **versioned lifecycle and ownership** — an owner role, a lifecycle state, and a
  monotonic manifest version, so your extension can pin the contract revision it
  was built against.
- **extension consumption rules** — explicit rules per family that you must honor
  to reuse or extend the component.

Build your extension's component against the manifest for its family rather than
copying shell behavior into a separate doc: read the manifest's anatomy, render
its mandatory states, reuse its label and token references, and honor its
consumption rules. The per-family fixtures
(`fixtures/ui/m5-component-gallery/component-manifest-<kind>.json`) are
checked-in instances you can render against in tests. See the
[component-manifest doc][manifest-doc] for the full shape.

## Declaring a component

To extend a launch-critical component, declare a component-contract artifact that
conforms to the schema and reuses the published foundation token references and
canonical state classes. Map the component-contract object id from your
surface's coverage row. An extension surface that references a component contract
which is unmapped (not published) or whose proof is stale is gated exactly like a
first-party surface — a missing mapping blocks Stable promotion, stale proof
auto-narrows the claim.

## Where the truth lives

- Contract matrix doc: [`docs/design-system/m5-design-system-contract-matrix.md`][matrix]
- Component-contract schema: [`schemas/design-system/m5-component-contract.schema.json`][component-schema]
- Component-manifest schema: [`schemas/design-system/m5-component-manifest.schema.json`][manifest-schema]
- Component-manifest doc: [`docs/design-system/m5-component-manifest.md`][manifest-doc]
- Component gallery: [`fixtures/ui/m5-component-gallery/`][gallery]
- Foundations guidance: [`extension-ui-design-system.md`][foundations-guidance]

[matrix]: ../design-system/m5-design-system-contract-matrix.md
[component-schema]: ../../schemas/design-system/m5-component-contract.schema.json
[manifest-schema]: ../../schemas/design-system/m5-component-manifest.schema.json
[manifest-doc]: ../design-system/m5-component-manifest.md
[gallery]: ../../fixtures/ui/m5-component-gallery/
[foundations-guidance]: extension-ui-design-system.md
