# M5 badge-family surface certification contract (M05-947)

This is the closing capstone for batch B111. It certifies that the six frozen M5
badge families — **support class, evidence freshness, lifecycle, channel, deployment
scope, and compatibility state** — hold the *same* controlled truth on every claimed
M5 badge-bearing surface, rather than letting each surface reinterpret the labels
locally.

- Freeze matrix: `crates/aureline-release/src/freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix/`
- Primitive lanes (M05-941..944): support-class/freshness, lifecycle/channel,
  deployment-scope, compatibility-state resolvers.
- Consumer adoption (M05-945): `add_shared_marketplace_help_settings_onboarding_diagnostics_export_runtime_and_workspace_consumers_…`
- Accessibility + auto-narrowing (M05-946): `implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_…`
- This capstone (M05-947): `certify_support_class_evidence_freshness_lifecycle_channel_deployment_scope_and_compatibility_badge_truth_on_every_claimed_m5_surface/`

Schema: [`schemas/ui/m5-badge-family-certification.schema.json`](../../schemas/ui/m5-badge-family-certification.schema.json).

## What it certifies

The packet is keyed on the **claimed surface** a user reads a badge on, not on the
badge family or the primitive lane. The eight certified surfaces are:

| Surface | Token |
| --- | --- |
| Marketplace | `marketplace` |
| Help / About | `help_about` |
| Settings | `settings` |
| Onboarding | `onboarding` |
| Diagnostics | `diagnostics` |
| Runtime / deployment | `runtime_deployment` |
| Support / export | `support_export` |
| CLI / headless | `cli_headless` |

Each row certifies its surface across six truth axes:

1. **Visual** — the badge's typed value, axis name, and explanation-drawer affordance
   are shown on-surface.
2. **Keyboard** — the same badge truth and its explanation drawer are pointer-free.
3. **Screen-reader** — the axis name and typed value are announced non-visually, never
   color- or glyph-only.
4. **CLI/export** *(always-on)* — the badge state exports as text / JSON / Markdown so
   support and automation can reconstruct it from the same badge identity the user saw.
5. **Degraded-state** — a stale, limited, imported, or policy-blocked dimension honestly
   narrows the `full_claim` / `supported` claim rather than presenting last-known
   posture as current.
6. **Axis-separation** — the six badge cues stay distinct: no badge merges into,
   implies, or stands in for another, and **Certified never implies Fresh**.

## The invariants

- **A degraded axis must produce a visible claim narrowing.** A surface that keeps a
  `full_claim` / `supported` claim while a truth axis is not current is over-claiming
  and blocks (red). A surface that discloses the reduction by narrowing its
  badge-support claim — with a bound, non-generic reason and a frozen downgrade trigger
  — is honestly yellow.
- **CLI/export parity is always-on** and must stay certified on every row.
- **Badge-meaning preservation (M05-947 delta).** Each badge's axis meaning, plus its
  explanation drawer, downgrade rule, and filter key, must never be collapsed or dropped
  between the marketplace, help, diagnostics, and exported evidence. A row that loses
  badge meaning blocks (`badge_meaning_preserved = false` → red / `BadgeMeaningDropped`).
- **Certification only narrows, never strengthens** a claim.
- **One canonical bundle.** Every row cites
  `artifacts/release/m5-badge-family-proof/support_export.json` (the frozen badge-family
  matrix release proof) rather than cloning per-surface evidence.
- **Metadata-only.** Raw badge material, signing keys, and evidence cursors never cross
  the boundary; `RawBadgeMaterialInExport` rejects them.

The badge-support claim ceiling (`support_claim`) is reused directly from the M05-946
accessibility lane: `full_claim` > `supported` > `limited` > `provisional` > `imported`
> `policy_blocked`.

## Verdict derivation

`derived_status` is never authored — it is recomputed from the axis outcomes and the
claim narrowing (`BadgeSurfaceCertificationRow::derive_status`) and re-checked on
validation (`status_is_fresh`). The seeded, checked-in packet certifies all eight
surfaces: **four green** (marketplace, help/about, settings, support/export) and **four
yellow** (onboarding, diagnostics, runtime/deployment, CLI/headless), with **no red**.
Every frozen badge family is certified on at least one surface.

## Artifacts

- Support export (canonical, `include_str!`): `artifacts/release/m5-badge-family-certification-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-badge-family-certification-proof/matrix.csv`
- Markdown report: `artifacts/release/m5-badge-family-certification-proof/report.md`
- Fixture mirror: `fixtures/ui/m5-badge-family-certification/`

Regenerate after any change to the seed builder:

```
GEN_BADGE_CERT_ARTIFACTS=1 cargo test -p aureline-release --lib \
  certify_support_class_evidence_freshness -- generate_artifacts
```

then re-run the suite so the `include_str!` canonical is picked up:

```
cargo test -p aureline-release --lib certify_support_class_evidence_freshness
```
