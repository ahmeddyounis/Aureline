# Reactive-state troubleshooting runbook

This runbook turns reactive-state failures into reproducible, diagnosable contract
failures. Each entry names the symptom, the exact reason code and finding code, the
narrow safe next step, and the checked-in fixture that reproduces it. Use the
finding code to cross-reference the Doctor pane, CLI/headless rows, and the
support-export envelope — they all carry the same vocabulary.

- Packet: [`artifacts/support/reactive_diagnostics.json`](./reactive_diagnostics.json)
- Schema: [`schemas/support/reactive_diagnostics.schema.json`](../../schemas/support/reactive_diagnostics.schema.json)
- Fixtures: [`fixtures/support/reactive_diagnostics/`](../../fixtures/support/reactive_diagnostics/)

## How to read a diagnosis

1. Open Project Doctor (or `aureline doctor` headless) and find the reactive
   finding by its `reactive.*` finding code.
2. The finding names the **reason code**, the **severity**, and the **safe next
   step**. Apply the safe next step — it is always the narrowest sufficient
   recovery, never a factory reset or a silent retry.
3. To share with support, export the metadata-first envelope. It is reviewable
   before it leaves the machine and carries no raw payloads, credentials, paths,
   or traces.

## Runbook entries

### Consumer reading a stale epoch — `reactive.consumer_stale`

- **Symptom:** A derived view (e.g. the AI context panel) is pinned to a known
  stale epoch after an invalidation gap broke causality. Severity: **blocking**.
- **Safe next step:** Request a fresh consistent snapshot; keep the view read-only
  as last-known until it applies. Do not act on the stale epoch as current truth.
- **Reproduce:** `fixtures/support/reactive_diagnostics/consumer_stale.json`
  (slow consumer `ai_inspector`, lag condition `invalidation_gap`).

### Consumer trailing the live epoch — `reactive.consumer_coalescing`

- **Symptom:** A consumer (e.g. CLI/headless output) lags and applies coalesced
  frames behind its producer. Severity: **degraded**.
- **Safe next step:** Wait for the coalesced frame to apply; revalidate any pending
  action against the producer before it commits.
- **Reproduce:** `fixtures/support/reactive_diagnostics/lagging_consumer.json`.

### Subscription needs to resubscribe — `reactive.resubscribe_required`

- **Symptom:** A dropped subscription is reconnecting and must resubscribe on a new
  snapshot epoch before acting. Severity: **blocking**.
- **Safe next step:** Run the visible resubscribe to re-establish the subscription;
  exact-truth actions stay withheld until the new snapshot applies.
- **Reproduce:** `fixtures/support/reactive_diagnostics/resubscribe_required.json`.

### Provider overlay unavailable — `reactive.provider_overlay_unavailable`

- **Symptom:** A derived surface (e.g. the review workspace) is reading a provider
  overlay whose backing producer is unavailable. Severity: **blocking**.
- **Safe next step:** Reconnect the provider overlay; keep the view read-only as
  last-known truth and block exact-truth actions until it returns.
- **Reproduce:** `fixtures/support/reactive_diagnostics/provider_overlay_unavailable.json`.

### Consumer epoch drifted from authority — `reactive.epoch_drift`

- **Symptom:** A consumer's snapshot epoch has drifted behind its authority epoch
  (visible in the active-subscriptions section). Severity: **degraded**.
- **Safe next step:** Request a fresh snapshot to realign the consumer; narrow to
  last-known read-only meanwhile.
- **Reproduce:** `fixtures/support/reactive_diagnostics/epoch_drift.json`.

### Invalidation storm saturating a consumer — `reactive.invalidation_storm`

- **Symptom:** A burst of invalidations is arriving faster than a consumer can
  apply, saturating its delta queue. Severity: **degraded**.
- **Safe next step:** Let the storm coalesce into one consistent frame; the surface
  stays visible but does not claim the burst already settled.
- **Reproduce:** `fixtures/support/reactive_diagnostics/invalidation_storm.json`.

### Backpressure queue overflowed — `reactive.backpressure_overflow`

- **Symptom:** A bounded delta queue overflowed and dropped intermediate frames,
  breaking causality. Severity: **degraded**.
- **Safe next step:** Drop the lossy stream and request a fresh consistent snapshot;
  narrow to last-known read-only until it applies.
- **Reproduce:** `fixtures/support/reactive_diagnostics/backpressure_overflow.json`.

### Partial scope serving a stale view — `reactive.partial_scope_stale`

- **Symptom:** A partially loaded scope is serving a stale or incomplete view
  (e.g. the companion panel). Severity: **degraded**.
- **Safe next step:** Hold the last consistent projection read-only and request a
  fresh snapshot to reconcile the partial scope.
- **Reproduce:** `fixtures/support/reactive_diagnostics/partial_scope_stale.json`.

## Export-before-share checklist

- The export envelope is `reviewable_before_share` and excludes raw payloads and
  ambient authority by construction.
- Each row carries only an evidence-reference **count**, the finding code, the
  reason code, the severity, and the recovery recommendation.
- `is_export_safe()` must hold before any share step; it verifies the redaction
  posture and that the doc/schema/report/runbook refs match the canonical packet.
