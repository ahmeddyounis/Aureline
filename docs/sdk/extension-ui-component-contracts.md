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
- Component gallery: [`fixtures/ui/m5-component-gallery/`][gallery]
- Foundations guidance: [`extension-ui-design-system.md`][foundations-guidance]

[matrix]: ../design-system/m5-design-system-contract-matrix.md
[component-schema]: ../../schemas/design-system/m5-component-contract.schema.json
[gallery]: ../../fixtures/ui/m5-component-gallery/
[foundations-guidance]: extension-ui-design-system.md
