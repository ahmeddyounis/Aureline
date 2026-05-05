# Signal-class matrix (telemetry, diagnostics, and support transfers)

This document publishes the shared **signal-class matrix** used to keep
observability, privacy, and support capture rules consistent across:

- the telemetry/support schema registry,
- privacy-history events and export/delete summaries,
- support-bundle preview and support-pack inclusion matrices, and
- release-evidence or parity packets that cite observational evidence.

Signal classes name **intent + risk**, not transport. A support bundle
may reference a crash payload without turning it into telemetry; a
telemetry counter may remain local-only without changing its class.

Authoritative design anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix AM.1
  (seed signal-class matrix).
- `.t2/docs/Aureline_PRD.md` §5.19 (observability and diagnostics
  architecture; local bounded retention; disableable diagnostics).

Companion artifacts:

- [`/artifacts/observability/signal_classes.yaml`](../../artifacts/observability/signal_classes.yaml)
  — machine-readable matrix rows.
- [`/fixtures/observability/signal_class_examples/`](../../fixtures/observability/signal_class_examples/)
  — worked examples explaining why counters, crash artifacts, and
  support uploads do or do not require explicit review and redaction.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  and [`/artifacts/governance/consent_ledger_seed.yaml`](../../artifacts/governance/consent_ledger_seed.yaml)
  — payload-family registry that each signal family MUST resolve
  through before implementation starts.
- [`/docs/governance/privacy_history_and_lifecycle_contract.md`](../governance/privacy_history_and_lifecycle_contract.md)
  — consent-history linkage and required per-event signal-class
  resolutions.
- [`/docs/support/support_bundle_preview_contract.md`](../support/support_bundle_preview_contract.md)
  and [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  — preview-before-export rules and the support-bundle packet boundary.
- [`/docs/support/diagnostic_artifact_matrix.md`](../support/diagnostic_artifact_matrix.md)
  and [`/artifacts/support/support_evidence_pack_matrix.yaml`](../../artifacts/support/support_evidence_pack_matrix.yaml)
  — item-level inclusion rules that bind support evidence to signal
  classes without widening capture scope.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — retention/export ceilings for governed record classes referenced by
  the schema registry and support bundles.

Out of scope: implementing telemetry backends, crash upload services, or
support submission pipelines. This is a policy and vocabulary matrix.

## Matrix (class-level defaults)

The matrix below describes defaults for **collection posture**, allowed
content, forbidden-by-default content, retention expectations, the
override path, and whether a **preview** is required before bytes leave
the device.

| Signal class id | Default collection posture | Upload/export preview | Allowed content (examples) | Forbidden by default (examples) |
|---|---|---|---|---|
| `install_update_active_version_counts` | local aggregation + **explicit opt-in** upload | not required (schema-bound) | coarse counts, version/channel, device class | code bodies, file paths, prompts, secrets, terminal/clipboard |
| `feature_usage_counters` | local aggregation + **explicit opt-in** upload | not required (schema-bound) | stable event ids, coarse counters, capability classes | free text, raw payloads, identifiers not declared in schema |
| `performance_metrics` | local capture + **explicit opt-in** upload | required for raw captures | coarse timings, memory buckets, workflow ids, digests | code bodies, file paths, prompts, secrets, raw traces by default |
| `crash_panic_reports` | local capture; upload only on explicit submit | required before any upload | redacted stack, build id, extension inventory digest | raw workspace content, raw memory dumps, raw secrets |
| `support_bundle_transfer_events` | **manual user/admin action** only | required before export/upload | bundle manifest metadata, inclusion/omission reasons | any hidden always-on support capture path |
| `managed_analytics_aggregates` | **admin policy gated** (managed lanes) | required for user-visible exports | aggregated usage/quota/compliance summaries | code bodies, prompts, secrets, raw identifiers outside schema |
| `local_diagnostic_signal` | local only (bounded retention) | required if promoted into support export | redacted logs/traces summaries, error codes, digests | raw secrets, raw code bodies, full terminal scrollback |
| `manual_support_evidence` | excluded until user selects | required per-item | user-selected bounded excerpts + hashes | automatic capture, unbounded workspace export, secret material |
| `forbidden_secret_bearing_artifact` | not collected | n/a (forbidden) | omission markers only | any secret-bearing bytes or raw credential material |

## Linkage and audit rules

Signal classes prevent “instrumentation drift” by forcing each signal to
declare intent and risk **before** shared plumbing (exporters, upload
queues, retention stores) makes different capture paths indistinguishable.

Rules:

1. **Every payload family resolves through the schema registry.** Any
   telemetry/crash/support/export family MUST have a schema-registry
   entry (`/artifacts/governance/consent_ledger_seed.yaml`) before it
   can ship, and that entry MUST cite at least one signal class above.
2. **Consent and policy changes are recorded as privacy-history events.**
   Whenever a user/admin action changes collection posture, the product
   MUST emit a `privacy_history_event_record` that includes
   `signal_class_resolutions[]` (not vague prose) so later review can
   reconstruct what was enabled, disabled, exported, held, or denied.
3. **Support export is never hidden telemetry.** Anything leaving the
   device via support flows MUST be represented as either
   `support_bundle_transfer_events` (the transfer/receipt) or
   `manual_support_evidence` (user-selected attachments). No other
   class is allowed to introduce a background “always-on support
   capture” path.
4. **Preview and redaction are part of the boundary.** When preview is
   required, the preview is not a courtesy UI: it is the policy gate
   that binds what can embed, what is reference-only, what is omitted,
   and what requires high-friction opt-in.
5. **Release evidence cites, it does not smuggle.** If a release,
   benchmark, regression, or parity claim depends on observational data
   (crash, perf capture, support bundle), the release-evidence packet
   MUST cite governed refs (record class + schema-registry entry + proof
   refs) instead of embedding raw, policy-ambiguous payloads.

## Examples

Worked examples live under
[`/fixtures/observability/signal_class_examples/`](../../fixtures/observability/signal_class_examples/).
They are intentionally “why” focused:

- coarse counters can be uploaded without per-event review because the
  schema forbids raw content classes and the payload is aggregate-only;
- crash artifacts are captured locally by default, but upload requires a
  preview because even redacted crash payloads are high-risk; and
- support uploads are always explicit and previewed, with item-level
  selection and omission markers to prevent accidental content sharing.

