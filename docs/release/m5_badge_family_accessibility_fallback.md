# M5 Badge-Family Accessibility & Auto-Narrowing (M05-946)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5
badge-family matrix
(`schemas/ui/m5-badge-family-matrix.schema.json`, module
`freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`).
Where the freeze matrix defines the six controlled badge families — **support class,
evidence freshness, lifecycle, channel, deployment scope, and compatibility state** —
with their explanation drawers, downgrade triggers, and consumer surfaces, and the
941–944 implementation lanes resolve each family's per-surface truth, this lane
certifies — per family — that badge claims stay **keyboard-complete,
assistive-tech-reachable, CLI/export-safe, and self-narrowing** across every claimed M5
surface.

- **Schema:** [`schemas/ui/m5-badge-family-accessibility-fallback.schema.json`](../../schemas/ui/m5-badge-family-accessibility-fallback.schema.json)
- **Module:** `crates/aureline-release/src/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_badge_freshness_lifecycle_deployment_support_or_compatibility_posture_is_stale_limited_imported_or_policy_blocked_across_claimed_m5_surfaces`
- **Proof bundle:** `artifacts/release/m5-badge-family-accessibility-fallback-proof/`
- **Fixtures:** `fixtures/ui/m5-badge-family-accessibility-fallback/`

## What is certified

Each `BadgeAccessibilityRow` keys on exactly one frozen `M5BadgeFamily` and reuses the
frozen `M5BadgeRequiredLabel`, `M5BadgeDowngradeTrigger`, and `M5BadgeConsumerSurface`
vocabulary — no parallel synonyms — so the certified labels stay byte-identical to the
matrix and the sibling badge lanes.

### AC1 — assistive / headless / export parity

Every family exposes a keyboard-complete, screen-reader-reachable, and CLI/headless
path into the same badge **identity, axis name, current value, explanation drawer,
evidence source, and any downgrade reason** the rich surface shows. No badge truth is
hover-only, pointer-only, or color-encoded alone. Hierarchy-heavy families (the
compatibility-state badge's nested reconciliation detail — gap class, residual
capability, repair action) additionally bind their tree to a flat list / textual path.
The support/release/evaluation export reconstructs each badge's meaning from stable
typed enums plus explanation/downgrade fields **without a screenshot**.

### AC2 — honest auto-narrowing

When a family's support, freshness, lifecycle, deployment, or compatibility truth
becomes **stale, limited, imported, or policy-blocked**, the badge claim auto-narrows
from its full claim to the permitted ceiling and discloses the narrowing with a precise
frozen trigger and binding dimension while preserving the canonical badge identity/axis.

| Condition state | Permitted ceiling |
| --------------- | ----------------- |
| `current`       | `full_claim`      |
| `limited`       | `limited`         |
| `stale`         | `provisional`     |
| `imported`      | `imported`        |
| `policy_blocked`| `policy_blocked`  |

Each badge dimension maps 1:1 to a frozen downgrade trigger, so the certified narrowing
reason stays byte-identical to the matrix. The axes stay separate: **Certified never
implies Fresh** — the support-class badge can be `full_claim` while the evidence-freshness
badge on the same object is independently `provisional`. A family with every dimension
intact carries **no** spurious narrowing.

### AC3 — cross-surface disclosure

The same narrowed state surfaces in the marketplace, help/about, settings, onboarding,
diagnostics, docs, evaluation pack, support/admin exports, CLI inspect, and product UI.
Every narrower rendering surface discloses its reduced interactivity and keeps its
labels, so badge truth stays aligned wherever it is viewed — a badge claim can never
outrun the proof it is being viewed away from.

## Boundary

The packet is metadata-only: raw evidence, signing keys, and credentials never cross
this boundary. It carries only typed class tokens, opaque summary/evidence refs,
booleans, and redacted labels. `validate()` rejects any packet whose serialized form
contains forbidden material.

## Regenerating the proof bundle

```
GEN_BADGE_A11Y_ARTIFACTS=1 cargo test -p aureline-release generate_artifacts
```

The seeded packet in `seeded_m5_badge_a11y_fallback_packet()` is the single source of
truth shared by the tests, the checked-in support export, the CSV, the Markdown report,
and the mirrored fixtures.
