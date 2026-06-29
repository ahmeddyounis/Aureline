# M5 client-scope card

The **client-scope card** turns one client-scope descriptor into the discovery, deep-link, handoff,
and companion disclosures a surface actually shows, so a narrowed client states its scope and
authority *before* a user discovers a limit by failing into it — and can never imply capability or
authority parity it does not hold. The [descriptor object](m5-descriptor-object.md) lane freezes the
typed client-scope truth (client kind, authority class, handoff requirement), the
[claim-narrowing](m5-claim-narrowing.md) lane derives the one claim state a narrowed client implies,
the [descriptor join](m5-descriptor-join.md) lane proves that truth survives copy/export, and the
[omission guard](m5-omission-guard.md) lane proves it never disappears. This lane is the
client-facing consumer: it renders that truth where browser, headless, and unsupported surfaces are
most tempted to imply desktop parity.

- Registry schema: `schemas/provenance/m5-client-scope-card.schema.json`
- Published registry: `artifacts/public-truth/m5-client-scope-card.json`
- Release parity proof: `artifacts/release/m5-descriptor-parity-proof/client-scope-card.json`
- Runtime: `crates/aureline-release/src/m5_client_scope_card/`
- Emitter: `cargo run -q -p aureline-release --bin aureline_release_m5_client_scope_card -- registry`

## Four surface classes, one authority ceiling

A card is built for one of four surface classes, and only the desktop surface may carry full
authority. Every narrower surface class binds a client-scope descriptor whose authority class is not
full, so a companion, headless, or unsupported surface can never claim desktop parity.

| Surface class | Label | Full authority |
|---------------|-------|----------------|
| `desktop` | Desktop | yes — the only full-authority surface |
| `browser_companion` | Browser companion | no |
| `headless` | Headless client | no |
| `unsupported` | Unsupported surface | no |

## Capabilities, blocked actions, and parity caveats are derived

Each card derives — never hand-authors — every visible fact from the bound client-scope descriptor:

- **Granted capabilities.** The authority class grants a strict prefix of the desktop's capability
  ladder (`observe` → `mutate_in_place` → `approve` → `administer`), so the card can state what the
  client *can* do.
- **Blocked actions.** Every desktop capability the scope does not grant becomes a blocked action,
  each carrying an attributable reason and the handoff that recovers it (`desktop_handoff_required`
  or `console_handoff_required`). A reference-only browser surface blocks `mutate_in_place`,
  `approve`, and `administer`; a handoff-only or `not_provided` surface blocks every capability.
- **Parity caveats.** One caveat per weaker client-scope facet — a narrowed `client_kind`, a narrowed
  `authority_class`, or a required `handoff_requirement` — so a narrowed client never reads at desktop
  parity by omission.
- **Claim state.** The controlled [claim state](m5-claim-narrowing.md) is read from the shared
  claim-narrowing runtime over a clean baseline whose only narrowing is this client scope: a desktop
  card resolves to `fully_supported`, every narrowed card to `unsupported_client`.

A weaker authority or handoff value survives as explicit state — `not_provided` authority and
`not_provided` handoff stay first-class — rather than disappearing into omission.

## Deep-link and handoff disclosures preserve the truth

Each card projects onto every disclosure surface — `discovery`, `deep_link`, `handoff`, and
`companion` — and every projection re-states the surface class, the authority class, the handoff
requirement, and the full blocked-action and parity-caveat counts. The per-card `guard` block is the
rule:

- **scope and authority stated** — every disclosure carries the surface class and authority class, so
  a user meets the limit on the discovery surface before any failure;
- **no disclosure implies broader authority** — a deep link or a handoff summary can never read as
  full authority when the card is narrowed;
- **deep link and handoff preserve truth** — the two surfaces most likely to over-claim carry the
  card's exact authority, handoff, blocked-action count, and parity-caveat count;
- **handoff disclosed when required** — a required handoff is surfaced on every disclosure;
- **only desktop carries full authority** — a non-desktop card never carries full authority or claims
  parity it lacks.

The `M5ClientScopeCardRegistry` is the one inspectable, serde-serializable truth packet every
consumer reads; its conformance block proves all four surface classes are covered, narrowed
companion / browser / headless cards can never imply desktop parity by omission, the claim state
matches the shared runtime, the controlled enums are frozen, and the export carries no raw provider
material.

## Consumers

The registry binds the same eight public-truth consumers the sibling descriptor lanes bind — release
center, Help/About, marketplace, docs/help, certification, evaluation packs, support exports, and
companion handoffs — so discovery, deep-link, and handoff surfaces render the same client-scope truth
that release, marketplace, and support already export, rather than each hand-authoring an equivalent
state that could imply parity a narrowed client does not have.
