# Environment-artifact diagnostics runbook

This runbook is the operator-facing companion to the environment-artifact
diagnostics lane. It explains how to read a diagnostics report, what each
finding means for an online, mirror, or offline profile, and the exact next
step for every blocking and non-blocking finding. The canonical engine is
[`crates/aureline-env/src/env_diagnostics/mod.rs`](../../crates/aureline-env/src/env_diagnostics/mod.rs);
the reviewer doc is [`docs/env/env-diagnostics.md`](../../docs/env/env-diagnostics.md);
the corpus is [`fixtures/env/env-diagnostics/`](../../fixtures/env/env-diagnostics/).
Every recovery path below is computed from the same `FindingCode::recovery_path`
the report and the Project-Doctor probes carry, so this runbook cannot disagree
with the product.

## Reading a report

A report folds one bundle into per-artifact diagnostics plus four roll-ups
(`trusted`, `degraded`, `unreusable`, `untrusted`) and a `review_state`.

- **`pending_review`** — no artifact is untrusted. The bundle is shareable
  *after* a human review; nothing blocks it.
- **`blocked`** — at least one artifact is untrusted. The `blocking_artifact_tokens`
  list names every `kind:id` that must be repaired before the bundle is shared.

The `source_channel` is carried on the report and on every diagnostic, so an
online, mirror, and offline capture all read in the same vocabulary; the only
difference is provenance, never the finding model.

## Finding catalog

### Trusted

| Finding | Meaning | Action |
| --- | --- | --- |
| `trusted` | Every governing dimension is current. | None; export or hydrate as-is. |

### Degraded — usable, visibly narrowed (not blocking)

| Finding | Meaning | Action |
| --- | --- | --- |
| `maturity_narrowed` | Partial or stale source evidence narrowed the claim. | Refresh the evidence in the reason tokens, then re-export. |
| `warm_start_downgraded` | The prebuild fingerprint outran its source digest. | Rebuild the snapshot against the current digest, or accept the colder posture. |
| `prebuild_partial_reuse` | Only part of the snapshot is reusable. | Let the affected layer rebuild; the rest stays warm. |
| `materialization_degraded` | A service, mount, port, or secret projection is pending. | Wait for the pending facet, or open the runtime inspector to repair it. |
| `mirror_source_unverified` | A community-mirror or unsupported source. | Confirm the mirror is acceptable, or switch to a first-party origin. |

### Unreusable — warm reuse failed, environment still rebuilds (not blocking)

| Finding | Meaning | Action |
| --- | --- | --- |
| `prebuild_cold_rebuild` | No reuse is trustworthy for current content. | Let the cold rebuild complete; rebuild the snapshot to restore warm start. |
| `prebuild_invalidated` | The snapshot is incompatible or untrusted and is evicted. | Discard the snapshot; investigate the platform/policy/critical-artifact drift. |

### Untrusted — blocks share and hydration

| Finding | Meaning | Action |
| --- | --- | --- |
| `claim_withheld` | A required dimension — e.g. an ungated lifecycle hook — cannot be proven. | Review and gate the hook (or missing dimension) before hydrating. |
| `materialization_mismatch` | Code ran on a different target or namespace than declared. | Stop the wrong-target run and re-materialize on the declared target. |
| `schema_version_unsupported` | The artifact's env schema version is unreadable. | Update the reader, or re-export from a compatible producer. |
| `redaction_violation` | A non-metadata redaction class crossed the boundary. | Re-export through the metadata-first projection. |

## Mirror and offline profiles

Mirror and offline captures use the **same** finding catalog above. The only
extra requirements are provenance:

- A `mirror` bundle MUST name its `mirror_origin_ref`; `import_env_bundle`
  rejects a mirror bundle that does not (`bundle.provenance.mirror_origin_ref`).
- An `offline` bundle carries `source_truth` describing the sealed import and
  reaches no network; its artifacts are diagnosed exactly as an online bundle's
  would be.

This is the guardrail the lane exists to hold: mirror and offline users never
fall back to a separate, undocumented environment-artifact format.

## Worked scenarios

These mirror the checked-in corpus.

### `local_online_trusted` (online)

Capsule, template, prebuild, and runtime are all `trusted`. The report is
`pending_review`, `share_blocked: false`. Action: review, then share or hydrate.

### `remote_mirror_degraded` (mirror)

A stale capsule fingerprint reports `warm_start_downgraded`, a community
template reports `mirror_source_unverified`, a partial prebuild reports
`prebuild_partial_reuse`, and a degraded runtime reports
`materialization_degraded`. Nothing is untrusted, so the report is
`pending_review` and shareable — the downgrades are visible, not hidden.

### `offline_sealed_blocked` (offline)

An ungated capsule hook reports `claim_withheld` and a wrong-target runtime
reports `materialization_mismatch` — both `untrusted`, so the report is
`blocked`. An invalidated prebuild reports `prebuild_invalidated` (`unreusable`,
surfaced as a notice). Action: gate the hook and re-materialize on the declared
target before the bundle can be shared.
