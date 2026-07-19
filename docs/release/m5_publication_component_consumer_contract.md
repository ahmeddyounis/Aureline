# M5 Publication-Component Consumer Contract

**Record kind:** `add_shared_release_center_update_center_about_help_docs_evaluation_and_support_publication_component_consumers`
**Module:** `crates/aureline-release/src/add_shared_release_center_update_center_about_help_docs_evaluation_and_support_publication_component_consumers/`
**Boundary schema:** [`schemas/ui/m5-publication-component-consumer.schema.json`](../../schemas/ui/m5-publication-component-consumer.schema.json)
**Support export:** [`artifacts/release/m5-publication-component-consumer-proof/support_export.json`](../../artifacts/release/m5-publication-component-consumer-proof/support_export.json)

## Purpose

Aureline narrows four of the six frozen release-center component families into
working primitives, each with its own canonical schema, contract doc, and
support-export artifact:

| Component family | Canonical primitive schema |
| --- | --- |
| Release-candidate card | `schemas/ui/m5-release-candidate-card.schema.json` |
| Version-bump / publish-target | `schemas/ui/m5-publish-target-review-sheet.schema.json` |
| Artifact provenance bundle | `schemas/ui/m5-artifact-provenance-bundle-card.schema.json` |
| Promotion / rollback history | `schemas/ui/m5-promotion-timeline-step.schema.json` |

This lane is the **adoption** layer over those primitives. It proves the four
families are reusable *components* — not one release pipeline plus a few
admin-only pages — by binding every claimed M5 publication-component consumer to
the same canonical schemas and the same descriptor vocabulary, so
release/publication facts stop diverging between the product UI, the docs, the
evaluation packet, and the support artifact.

## Consumers

The six claimed publication-component consumers, each keyed by
`M5PublicationComponentConsumer`:

- `release_center` — the authoritative shiproom rendering.
- `update_center` — the in-product update surface.
- `about_help` — the About/help surface.
- `docs_portal` — the docs portal.
- `enterprise_evaluation` — the enterprise-evaluation packet.
- `support_export` — the support export.

Every consumer adopts at least two of the four canonical component families, and
every family is adopted by at least two consumers, so reuse across surfaces is
proven rather than asserted.

## Shared descriptor vocabulary

Consumers reuse one descriptor vocabulary (`M5PublicationDescriptor`) instead of
inventing new badges or stale wording. All four descriptors are required on every
binding — the track invariant that these facts stay explicit everywhere Aureline
proposes, publishes, promotes, mirrors, evaluates, or exports a release:

- `provenance` — signature, attestation, digest lineage.
- `freshness` — evidence / proof freshness.
- `qualification` — stable / beta / preview class.
- `client_scope` — full / narrowed / mirror / handoff scope.

## Client-scope modes and reduced-scope banners

Each binding declares the client-scope mode it renders under
(`M5ClientScopeMode`). Full scope preserves the descriptor vocabulary as-is; any
reduced scope keeps the same vocabulary but discloses a self-contained
`M5ReducedScopeBanner` naming the exact reason and next action rather than a
generic "reduced" note:

| Client-scope mode | Reduced-scope reason | Next action |
| --- | --- | --- |
| `full_client_scope` | *(none — descriptors preserved)* | — |
| `narrowed_client_scope` | `client_narrowed` | `widen_client_scope` |
| `mirror_offline_scope` | `mirror_offline` | `refresh_from_canonical_source` |
| `browser_companion_handoff` | `browser_companion_handoff` | `open_authoritative_release_center` |

Mirror/offline and browser/companion handoff caveats (`M5HandoffCaveat`) are
preserved whenever a component appears outside the main release center.

## Invariants enforced by `validate()`

- Every consumer is present; every canonical family is reused across ≥2 consumers.
- Every component binding points at its family's canonical schema and artifact
  ref (`references_canonical_not_local_prose` is `true`), never a local
  re-description. Docs/help consumers reference the canonical schema for every
  family they adopt.
- Every binding keeps all four required descriptors.
- At least one worked binding proves a narrowed rendering with a self-contained
  banner; at least one proves a full-scope rendering with preserved parity and no
  banner.
- Row hard invariants must all be `false`: `rewords_descriptors_per_surface`,
  `invents_new_badge_vocabulary`, `drops_provenance_or_freshness_when_narrowed`,
  `hides_mirror_or_offline_handoff_caveat`.
- The export carries no raw URLs, keys, tokens, credentials, or secrets.

## Regenerating the artifacts

The headless emitter is the only mint-from-truth path:

```sh
BIN=aureline_release_add_shared_release_center_update
cargo run -q -p aureline-release --bin $BIN -- support-export > artifacts/release/m5-publication-component-consumer-proof/support_export.json
cargo run -q -p aureline-release --bin $BIN -- csv           > artifacts/release/m5-publication-component-consumer-proof/matrix.csv
cargo run -q -p aureline-release --bin $BIN -- report        > artifacts/components/m5-publication-component-consumer.md
cargo run -q -p aureline-release --bin $BIN -- fixture-about-help-handoff-narrowed  > fixtures/ui/m5-publication-component-consumers/about_help_handoff_narrowed.json
cargo run -q -p aureline-release --bin $BIN -- fixture-docs-mirror-offline-narrowed > fixtures/ui/m5-publication-component-consumers/docs_mirror_offline_narrowed.json
```

The inline test `checked_support_export_validates_and_matches_seed` asserts the
checked-in export is byte-aligned with the seed builder.
