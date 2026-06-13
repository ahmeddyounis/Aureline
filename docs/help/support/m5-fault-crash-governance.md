# M5 Fault / Crash Governance

## Overview

This packet freezes the M5 host-failure and crash-forensics contract used by
notebook kernels, data/API connectors, preview dev servers, provider-run
sessions, profiler/replay sessions, pipeline viewers, docs/browser bridges,
and infrastructure helpers.

It gives support, release, and field-readiness surfaces one shared vocabulary
for:

- fault-domain class and restart class;
- checkpoint or rehydrate source;
- quarantine triggers and minimum diagnostic exports;
- crash artifacts and exact-build symbolication posture; and
- diagnostics-schema, consent, retention, and redaction rules for crash,
  performance, usage, and support signals.

## What the packet freezes

### Fault domains

The packet quotes the seven canonical fault-domain classes from the
architecture:

- `shell_interaction_core`
- `workspace_knowledge_group`
- `session_execution_host`
- `extension_or_external_tool_host`
- `ai_tool_broker`
- `remote_connector`
- `policy_verifier_helper`

Each row binds one isolation unit, one restart class, one checkpoint source,
typed quarantine or hard-stop triggers, and the minimum metadata-safe
diagnostic exports required for the domain.

### Restart classes

The restart classes are frozen and inspectable:

- `stateless_helper`
- `workspace_knowledge`
- `session_scoped`
- `privileged_externally_mutating`
- `authority_verifier`

Every row carries the default strike window, automatic restart budget, and
escalation posture. No retry path may silently widen authority.

### Crash artifacts

The crash-artifact vocabulary is frozen across support, debugger, and release
surfaces:

- `crash_envelope`
- `minidump_or_core_artifact`
- `symbol_or_source_map_manifest`
- `local_symbolication_report`
- `mirrored_symbol_service`

The packet requires local-first capture, exact-build identity, no automatic raw
upload, and explicit mirror/access policy when a symbol service is claimed.

### Diagnostic schema rows

The packet freezes four governed signal families:

- `diagnostics.crash_payload`
- `performance.trace_support_bundle`
- `usage.metering_export_packet`
- `support.bundle_manifest`

Each row names:

- the schema id and schema ref;
- purpose;
- data class;
- opt-in or consent scope;
- prohibited content classes;
- retention class; and
- redaction profile.

## Host families covered

The seed packet covers at least these required M5 consumers:

- notebook kernel host
- data/API connector host
- preview dev server host
- provider-run session host
- profiler/replay session host
- pipeline viewer host

It also carries rows for query runtimes, docs/browser bridges, registry or
database connectors, and infrastructure helpers so new M5 hosts inherit typed
failure semantics rather than generic process supervision.

## Downgrade behavior

Rows may not stay greener than their proof. The packet freezes downgrade rules
for:

- stale restart or quarantine evidence;
- stale crash-artifact proof;
- symbolication that is absent or not exact-build; and
- stale diagnostics-schema or consent/redaction review proof.

When one of those triggers applies, the row narrows to `narrowed_preview`,
`narrowed_local_only`, or `blocked_unverified` instead of inheriting adjacent
stability.

## Related contracts

- [Supervised-Restart Evidence Pipeline](./supervised-restart-evidence-pipeline.md)
- [Crash-loop recovery center](../m3/crash_loop_recovery_beta.md)
- [Field readiness metrics](../m3/field_readiness_metrics.md)
