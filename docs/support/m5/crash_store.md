# M5 local crash-store viewer

This document defines the local-first crash-store viewer packet for M5 runtime
families.

## What this row owns

- A typed crash-store viewer packet that binds crash-envelope refs, dump/core
  refs, exact-build identity, fault-domain attribution, trace ids, and
  session-type identity into one local review surface.
- Visible preservation classes so users can tell whether the raw dump is still
  retained locally, requires explicit opt-in, or has already expired while
  metadata remains inspectable.
- Visible redaction posture and action rows for preview, local export, raw-dump
  attach opt-in, reviewed upload, and restart-lineage inspection.
- The boundary schema at
  [`/schemas/support/crash_store_viewer.schema.json`](../../../schemas/support/crash_store_viewer.schema.json).
- The protected fixtures at
  [`/fixtures/support/m5/crash_store/`](../../../fixtures/support/m5/crash_store/).

## Acceptance and how this row meets it

- Every claimed M5 crash-heavy host family has a local crash-store row with a
  stable `crash_id`, `build_id`, `fault_domain_id`, `session_type_id`, trace
  ids, sandbox profile, policy fingerprint, and bounded crash window.
- Dump/core metadata stays attributable with architecture, signal/exception
  class, dump-format identity, module ids, and module build ids.
- Local preview and local metadata export remain visible before any upload path.
- Raw dump attachment is always explicit opt-in, and silent upload is forbidden.

## Out of scope for this row

- Symbolication execution itself.
- Hosted crash intake implementation.
- Platform-specific dump capture code.
