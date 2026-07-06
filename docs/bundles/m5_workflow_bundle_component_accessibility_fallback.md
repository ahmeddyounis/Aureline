# M5 workflow-bundle component accessibility & auto-narrowing (M05-850)

This lane is the accessibility-and-auto-narrowing capstone over the frozen
[M5 workflow-bundle component matrix](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json).
Where the freeze matrix defines the reusable start-center bundle card,
certified-archetype badge group, bundle detail page, install / update review sheet,
drift banner, local-override row, rollback / remove card, class-disclosure card, and
claim-narrowing row primitives, and the 845–849 implementation lanes resolve their
per-surface truth, this lane certifies — **per component family** — that
workflow-bundle claims stay keyboard-complete, screen-reader-reachable,
CLI/export-safe, and honestly self-narrowing.

- Boundary schema: [`schemas/ui/m5-workflow-bundle-component-accessibility-fallback.schema.json`](../../schemas/ui/m5-workflow-bundle-component-accessibility-fallback.schema.json)
- Release proof: [`artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/`](../../artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/)
- Fixtures: [`fixtures/ui/m5-workflow-bundle-component-accessibility-fallback/`](../../fixtures/ui/m5-workflow-bundle-component-accessibility-fallback/)
- Implementation: `crates/aureline-workspace/src/implement_keyboard_screen_reader_cli_export_parity_and_bundle_claim_auto_narrowing/`

## What it certifies

Each row keys on one frozen `M5WorkflowBundleComponentFamily` and reuses that frozen
family vocabulary plus the frozen `M5BundleRequiredLabel` and
`M5BundleComponentDowngradeTrigger` and the shared `M5BundleDisclosureSurfaceFamily`
consumer surfaces, so the certified labels stay byte-identical to the matrix and the
sibling primitive packets.

### Keyboard / screen-reader / CLI reach (AC2)

Every family exposes a keyboard-complete, screen-reader-reachable, and
CLI/headless-reachable path into the same bundle identity, signer / source class,
certification freshness, drift state, and rollback path the rich surface shows —
never a view-only card that strands assistive-tech or headless users. The
hierarchy-heavy family (the bundle detail page's component / dependency inventory)
additionally binds its tree to a flat list / textual path.

### Export parity (AC2)

The support / release export reconstructs each component's meaning from typed tokens
and opaque refs **without a screenshot**, preserving the same bundle IDs, source
classes, evidence ages, and drift states shown in-product. Copy / export is offered
as text, JSON, and Markdown.

### Honest auto-narrowing (AC1)

When a bundle dimension — bundle freshness, certification evidence, source
provenance, artifact availability, or dependency posture — is partial, stale,
imported, mirror-only, offline, or policy-blocked, the component's **bundle-support
claim** auto-narrows to the permitted ceiling and names the binding dimension and
frozen trigger while preserving canonical identity. A component with every dimension
intact carries **no** spurious narrowing.

| Condition state | Permitted support ceiling | Frozen trigger |
| --- | --- | --- |
| `intact` | `certified` | — |
| `partial` | `limited` | `local_override_drift` |
| `stale` | `retest_pending` | `stale_certification` |
| `imported` | `imported` | `imported_not_native` |
| `mirror_stale` | `mirror_only` | `mirror_stale` |
| `offline_only` | `offline_cache_only` | `offline_cache_only` |
| `policy_blocked` | `policy_blocked` | `entitlement_dependency_unmet` |

The effective support claim is the weakest ceiling across all modeled dimensions,
capped at the family's full claim. A stale or partial bundle can therefore no longer
present as fully certified or fully self-sufficient.

### Cross-surface disclosure (AC3)

The same narrowed state surfaces in UI, docs/help, migration packets, diagnostics,
and support/admin exports (`M5BundleDisclosureSurfaceFamily`), so claim publication
and field triage stay aligned on workflow-bundle downgrade behavior. Every narrower
rendering surface discloses its reduced interactivity and preserves its labels;
nothing is silently dropped.

## Certified rows

Nine families, one row each: **2 green / 7 yellow / 0 red**.

| Row | Family | Effective claim | Binding dimension |
| --- | --- | --- | --- |
| `a11y:start-center-bundle-card` | start-center bundle card | `certified` (green) | — |
| `a11y:certified-archetype-badge-group` | certified-archetype badge group | `retest_pending` | certification evidence (stale) |
| `a11y:bundle-detail-page` | bundle detail page | `policy_blocked` | dependency posture (policy-blocked) |
| `a11y:bundle-install-update-review-sheet` | install / update review sheet | `supported` (green) | — |
| `a11y:bundle-drift-banner` | drift banner | `limited` | bundle freshness (partial) |
| `a11y:bundle-local-override-row` | local-override row | `mirror_only` | artifact availability (mirror-stale) |
| `a11y:bundle-rollback-remove-card` | rollback / remove card | `offline_cache_only` | artifact availability (offline) |
| `a11y:bundle-class-disclosure-card` | class-disclosure card | `imported` | source provenance (imported) |
| `a11y:bundle-claim-narrowing-row` | claim-narrowing row | `retest_pending` | certification evidence (stale) |

## Metadata-only boundary

Raw manifests, credentials, entitlement tokens, mirror URLs, and provider cursors
never cross this boundary. The packet carries only typed class tokens, opaque
summary / evidence refs, booleans, and redacted labels.

## Regenerating the proof

The checked-in `support_export.json`, `matrix.csv`, and `report.md` are the one
source of truth, byte-aligned with the `seeded_m5_bundle_a11y_fallback_packet()`
builder. Regenerate with:

```
GEN_BUNDLE_A11Y_ARTIFACTS=1 cargo test -p aureline-workspace generate_artifacts
```
