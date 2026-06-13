# Depth-surface schema registry

This document freezes the support-side registry packet for M5 depth surfaces:
notebook kernels, provider-run sessions, profiler/replay sessions, pipeline
viewers, preview dev servers, and data/API connectors.

The packet exists so support, diagnostics, CLI/headless, and release surfaces
all quote the same truth for:

- declared crash, performance, feature-usage, and support-export schemas;
- the consent-ledger row each signal inherits;
- the active endpoint or local-only posture shown to users;
- retention and redaction defaults; and
- redaction-default packet classes used for export and handoff.

## Canonical files

- Schema:
  `schemas/support/depth-surface-schema-registry.schema.json`
- Support packet:
  `crates/aureline-support/src/schema_registry/mod.rs`
- Review artifact:
  `artifacts/support/m5/depth-surface-schema-registry.md`
- Fixtures:
  `fixtures/support/m5/depth_surface_schema_registry/`

## Rules

1. Every covered signal must have a declared `schema_id`, `schema_version`,
   purpose, allowed-field list, prohibited-content list, retention class,
   redaction profile, owner, reviewers, and evidence refs.
2. Open-source and local builds stay local-first and opt-in for crash,
   performance, and feature-usage signals.
3. Support export remains explicit preview or export flow only. It may reuse the
   same registry vocabulary, but it is not ambient telemetry.
4. Managed builds may narrow what is collected, retained, or exported, but they
   may not silently broaden fields beyond the shipped registry vocabulary.
5. Ordinary diagnostic and telemetry rows forbid source code, filenames and
   paths, prompt bodies, terminal contents, secrets, and clipboard contents by
   default.

## Inspectability

The packet is designed to be projected into:

- product diagnostics;
- CLI/headless summaries;
- support-bundle preview and manifest review; and
- release or readiness evidence.

Every surface row therefore keeps signal-class coverage, disabled-by-default
signals, export-only signals, packet-class refs, and retention classes visible
without requiring raw payload capture.
