# Depth-surface schema registry

Status: active support-governance packet for M5 depth surfaces.

## Canonical refs

| Kind | Ref |
|---|---|
| Schema | `schemas/support/depth-surface-schema-registry.schema.json` |
| Help doc | `docs/help/support/depth-surface-schema-registry.md` |
| Source module | `crates/aureline-support/src/schema_registry/mod.rs` |
| Fixtures | `fixtures/support/m5/depth_surface_schema_registry/` |

## Scope

The packet covers:

- notebook kernel;
- provider run session;
- profiler/replay session;
- pipeline viewer;
- preview dev server; and
- data/API connector surfaces.

Each surface carries four declared signal families:

- crash diagnostics;
- performance diagnostics;
- feature-usage telemetry; and
- support export.

## Review points

- Crash, performance, and feature-usage signals stay local-first and opt-in on
  open-source or local builds.
- Support export remains manual and explicit even when it reuses the same
  schema-registry vocabulary as other support signals.
- Redaction-default packet classes forbid raw code, filenames and paths, prompt
  bodies, terminal contents, secrets, and clipboard contents by default.
- Managed builds may narrow fields or destinations, but they may not silently
  broaden beyond the declared rows in the packet.
