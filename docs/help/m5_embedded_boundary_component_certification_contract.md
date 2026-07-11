# M5 embedded-boundary component surface certification contract (M05-1075)

This is the closing **surface-certification capstone** for the B127 embedded-boundary
component lane. Where the freeze matrix defines the eight reusable components, the
implement lanes narrow each one, the consumer lane adopts them, and the accessibility
lane proves keyboard / screen-reader / reduced-motion / high-contrast / CLI-export
parity, this capstone *certifies* that the shared component truth holds on every claimed
M5 embedded / browser-handoff surface — and auto-narrows any surface that cannot sustain
it.

- **Schema:** [`schemas/ui/m5-embedded-boundary-component-certification.schema.json`](../../schemas/ui/m5-embedded-boundary-component-certification.schema.json)
- **Canonical proof bundle:** `artifacts/release/m5-embedded-boundary-component-certification-proof/support_export.json`
- **Cited embedded-boundary proof bundle (one, shared):** `artifacts/release/m5-embedded-boundary-proof/support_export.json`
- **Accessibility evidence:** `artifacts/release/m5-embedded-boundary-component-accessibility-proof/support_export.json`

## What is certified

The packet is keyed on the **claimed surface** a user, operator, or support engineer
reads embedded-boundary truth through, not on the reusable component family it renders.
The eight certified surfaces are:

| Surface | Reads |
| --- | --- |
| `docs_help_pane` | docs-pane header, boundary-fact grid |
| `marketplace_pane` | marketplace-account-boundary card, open-in-browser-handoff row |
| `account_pane` | marketplace-account-boundary card, embedded-origin bar |
| `remote_service_dashboard` | remote-service-dashboard header, embedded-state panel |
| `embedded_webview` | embedded-origin bar, embedded-state panel |
| `auth_handoff` | auth-handoff card, open-in-browser-handoff row |
| `support_export` | replayable support / export bundle |
| `cli_headless` | text / JSON / Markdown automation output |

Every frozen component family — docs-pane header, embedded-origin bar, boundary-fact
grid, marketplace-account-boundary card, auth-handoff card, remote-service-dashboard
header, open-in-browser-handoff row, and embedded-state panel — is certified on at least
one surface.

## The six truth axes

Each surface is scored on exactly six axes, each appearing once:

1. **`visual`** — owner/origin, data boundary, source/version/last-updated,
   network/offline state, browser fallback, account scope, freshness, and capability
   limits are shown on-surface.
2. **`keyboard`** — the same inspect / reload / open-in-browser actions are
   keyboard-reachable.
3. **`screen_reader`** — the same boundary truth is announced non-visually, never
   color/glyph-only.
4. **`cli_export`** (always-on) — the surface state is reconstructable as text / JSON /
   Markdown for support and automation. This axis must always certify.
5. **`degraded_state`** — a stale, offline, or provider-blocked reading honestly
   downgrades a `full_truth` / `resolved_truth` claim rather than reading as fresh
   first-party local truth.
6. **`boundary_truth`** — owner/origin, data boundary, browser fallback, capability
   limits, account scope, and freshness stay explicit and never collapse into generic
   chrome wording, imitate native permission or irreversible approval UI, hide the
   browser fallback behind menus only, or render a stale / blocked pane as fresh.

## The invariant: a degraded axis must produce a visible claim narrowing

The boundary-support claim ceiling ranks, strongest first: `full_truth` (5),
`resolved_truth` (4), `degraded` (3), `stale` (2), `offline` (1), `provider_blocked` (0).

- **Green** — every axis certified, the claimed ceiling delivered.
- **Yellow** — an axis is `disclosed_narrowed` with a bound reason and a frozen downgrade
  trigger, and the surface visibly narrows its claim from `claimed_claim` to a weaker
  `certified_claim` via `claim_auto_narrow` (bound to the narrowed, non-always-on axis,
  with a non-generic label).
- **Red** — a degraded axis is hidden behind a fresh first-party full claim
  (`undisclosed_drift`, or a disclosed narrowing with no claim reduction), the CLI/export
  axis drops, the copy / export parity is incomplete, the certified claim exceeds the
  claimed one, or the narrowing is inconsistent. Red surfaces are not publishable.

A stale, offline, or provider-blocked pane can never keep a fresh first-party
`full_truth` claim, and no embedded surface may imitate native permission or irreversible
approval chrome. The stored `derived_status` is always recomputed and compared on
validation, so the verdict can never be hand-asserted.

## Metadata-only boundary

Raw credentials, session tokens, and provider secrets never cross this boundary. The
validator rejects any export carrying obviously forbidden material.

## Regenerating the proof

```
GEN_EMBEDDED_BOUNDARY_CERT_ARTIFACTS=1 cargo test -p aureline-shell \
  certify_docs_pane_header...::tests::generate_artifacts
```

This writes the canonical `support_export.json`, `matrix.csv`, and `report.md` under
`artifacts/release/m5-embedded-boundary-component-certification-proof/` and mirrors them
into `fixtures/ui/m5-embedded-boundary-component-certification/`. The
`checked_in_export_matches_seeded_builder` test byte-locks the on-disk export to the
seeded builder.
