# Reactive-state Project Doctor and support diagnostics

Reactive-state failures must be diagnosable as first-class contract failures, not
as generic "UI weirdness" or "cache corruption" anecdotes. This packet folds the
canonical reactive-state objects into one Project Doctor and support-export model
so support and Doctor flows name the failure directly and recommend the right
**narrow** recovery path.

- Boundary schema: [`schemas/support/reactive_diagnostics.schema.json`](../../schemas/support/reactive_diagnostics.schema.json)
- Machine-readable packet: [`artifacts/support/reactive_diagnostics.json`](../../artifacts/support/reactive_diagnostics.json)
- Reviewer report: [`artifacts/support/reactive_diagnostics.md`](../../artifacts/support/reactive_diagnostics.md)
- Troubleshooting runbook: [`artifacts/support/reactive_diagnostics_runbook.md`](../../artifacts/support/reactive_diagnostics_runbook.md)
- Fixture corpus: [`fixtures/support/reactive_diagnostics/`](../../fixtures/support/reactive_diagnostics/)
- Source module: `crates/aureline-support/src/reactive_diagnostics/`

## One vocabulary, not two

This packet does **not** invent a second diagnostics-only state model. Every
freshness, completeness, backpressure, epoch, invalidation-reason, view-class,
lag-condition, recovery-strategy, and truth-claim token is the same one the
product surfaces show, re-exported from `aureline-reactive-state`. Every finding
severity, confidence class, and repair-availability class is the same one Project
Doctor already uses, re-exported from `aureline-doctor`. Support exports and the
Doctor pane therefore preserve the exact stale/partial/cached wording a user sees
in the product.

## Sections

The packet carries six diagnostic sections:

| Section | What it answers |
| --- | --- |
| `active_subscriptions` | Which `(binding, scope)` subscriptions are live, the authority epoch vs. the consumer's snapshot epoch, and whether they have drifted. |
| `invalidation_history` | The ordered invalidation history, each entry naming the exact reason and the epoch transition it drove. |
| `stale_materializations` | Which materialized-view classes are stale and the narrow rebuild path for each. |
| `slow_consumers` | Which consumers lag, their backpressure mode, and the recommended recovery strategy and safe next step. |
| `doctor_probes` | One Project Doctor probe per condition: a stable finding code, reason code, severity, confidence, recovery recommendation, and repair availability. |
| `invariants` | The properties the packet enforces. |

## Reason codes and safe next steps

Each diagnosis names an exact reason code and the narrow next step it recommends:

| Reason code | Finding code | Severity | Safe next step |
| --- | --- | --- | --- |
| `consumer_stale` | `reactive.consumer_stale` | blocking | request a fresh snapshot |
| `consumer_coalescing` | `reactive.consumer_coalescing` | degraded | wait for coalesced catch-up |
| `resubscribe_required` | `reactive.resubscribe_required` | blocking | run the visible resubscribe |
| `provider_overlay_unavailable` | `reactive.provider_overlay_unavailable` | blocking | reconnect the provider |
| `epoch_drift` | `reactive.epoch_drift` | degraded | request a fresh snapshot |
| `invalidation_storm` | `reactive.invalidation_storm` | degraded | wait for coalesced catch-up |
| `backpressure_overflow` | `reactive.backpressure_overflow` | degraded | request a fresh snapshot |
| `partial_scope_stale` | `reactive.partial_scope_stale` | degraded | hold last-known read-only |

The first four — stale, coalescing, resubscribe-required, and provider-unavailable
— are the conditions the contract requires a diagnosis to name directly, and
every one carries a Doctor probe.

## Invariants

- Every diagnosis names an exact reason code and a narrow safe next step.
- A consumer that is not on the live epoch never offers an exact-truth action or a
  silent retry.
- A derived surface never presents exact current truth; a provider-unavailable
  overlay blocks exact-truth actions and holds last-known read-only.
- The invalidation history is strictly ordered and never rolls an epoch backward.
- A stale materialization is never marked authoritative.

## Metadata-first support export

`compile_support_export_envelope` projects the Doctor probes into a
`ReactiveDiagnosticsSupportExportEnvelope` that is reviewable before it leaves the
machine. It carries finding identity, reason codes, severities, and recovery
recommendations — but only an evidence-reference **count**, never raw payloads,
credentials, raw provider responses, raw paths, or raw traces. The envelope is
marked `reviewable_before_share`, and `is_export_safe()` proves the redaction
posture and the canonical refs before any share step.

## Reproducing reactive-state bugs

The fixture corpus reproduces the failure shapes that used to be tribal knowledge:
epoch drift, invalidation storms, lagging consumers, partial-scope stale views,
plus the provider-unavailable, resubscribe, stale, and backpressure conditions.
Each fixture pins the scenario, the expected reason code, finding code, severity,
and safe next step. See the runbook for the step-by-step reproduction.
