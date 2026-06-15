# M5 runtime inspector cards

This document describes the canonical packet that freezes the **M5 runtime inspector
cards** — one first-class runtime inspector per claimed M5 ecosystem family. It is the
user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-runtime-inspector.json` and the typed model in the
`aureline-ecosystem` crate (`m5_runtime_inspector`).

Where the [`M5 install/update review sheets`](m5-install-review.md) and the
[`M5 sideload review sheets`](m5-sideload-review.md) review a package *before* it loads,
this packet freezes the inspector an author or operator opens to read the **running**
truth of one installed or locally-built family — its activation time, current host,
granted capabilities, recent logs, recent failures, hot-reload posture, and the
quarantine/disable/re-enable actions — without scraping raw supervisor traces. The
inspector must stay useful when the package is failing or quarantined, so each card
keeps a last-known-good state visible whenever the current load failed or the source
path disappeared.

## What each card makes explicit

- **Identity** — the package kind / artifact family, the source class, the namespaced
  `publisher/extension` identity, and the version.
- **Current runtime** — the `runtime_class` and the `current_host` the artifact runs on
  right now.
- **Activation profile** — the cold/warm `bucket`, the measured `activation_millis`,
  the `memory_pressure` relative to budget, and the `peak_memory_mib`.
- **Granted capabilities** — each granted capability with its class, redacted target,
  rationale, and a declared-versus-exercised state (`declared_exercised`,
  `declared_unused` over-grant, or `undeclared_exercised` policy violation).
- **Logs** — recent redacted log entries, each carrying a level, stable code, redacted
  message label, and sequence number. No raw log body or supervisor trace.
- **Recent failures** — each failure with its class, occurrence count, redacted
  last-seen label, and redacted detail. Crash history is never hidden.
- **Load state** — `loaded_healthy`, `loaded_degraded`, `load_failed`, `source_missing`,
  `quarantine_held`, or `operator_disabled`.
- **Last-known-good** — the revision, runtime class, host, captured label, and rendered
  badge of the last good state, kept visible whenever the current load failed or the
  source disappeared.
- **Actions** — the quarantine, disable-for-workspace, disable-globally, re-enable,
  restart, reload-source, view-logs, and request-fresh-review controls.

## Three recomputed values

The card is honest by construction. Three published values are **recomputed** from the
card's facts, and the stored values must equal the recomputation or validation fails:

- **`rendered_trust_tier`** is the weaker of the claimed tier and the signing-state
  ceiling. An unsigned local-dev build, an unsigned side-load, or a revoked signature
  caps at `unsigned_local_only`, so a local or side-loaded artifact can never inherit a
  `verified_publisher` or `enterprise_approved` badge just because it was built on a
  machine that holds a trusted key. A genuinely signed-and-verified package still
  carries its real badge.
- **`review_triggers`** are computed from the hot-reload posture and the
  granted-capability state: a hot reload that widens the runtime class, expands
  permissions, or adds an external executable — or an undeclared exercised capability —
  each appears in the set, and any trigger forces a fresh review.
- **`disposition`** is the strongest of the load-state base, the fresh-review gate, and
  a hard `quarantined` for an anti-abuse hold. A clean running card is
  `running_healthy`; recent failures make it `running_degraded`; a failed load or a
  missing source is `showing_last_known_good`; a widening reload is
  `fresh_review_required`; an operator hold is `operator_disabled`; an anti-abuse hold
  is `quarantined`.

## Guardrails

- **No inherited trust.** A local-dev, side-loaded, or revoked artifact never renders a
  trusted-publisher badge; the validator flags any rendered tier above the recomputed
  cap.
- **No silent widening hot reload.** A card whose disposition is `fresh_review_required`
  must offer the request-fresh-review action and must not expose an enabled restart or
  reload that would apply the widening silently.
- **Useful when failing.** A `load_failed` or `source_missing` card must keep a
  last-known-good state visible, and that state can never render a stronger badge than
  the family's current cap.
- **Nothing hidden.** Every card exposes its logs, and crash history and granted
  capabilities (including over-grants and undeclared exercises) stay on the card even
  when the package is disabled or quarantined.
- **Not extension-manager metadata only.** The inspector carries live activation,
  capability, log, and failure truth, not just listing metadata.
- **Export-safe.** Every field is a typed state, a redacted label, or an opaque ref —
  no absolute paths, raw log bodies, supervisor traces, signing secrets, or payloads.

## Consuming surfaces

`M5RuntimeInspector::export_projection()` produces a redaction-safe row set that support
exports, service-health, docs/help, and release/public-truth surfaces render instead of
restating runtime, trust, and disposition text by hand.
