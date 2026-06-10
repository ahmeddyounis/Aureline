# AI Explain/Debug/Test Flow Fixtures

This directory contains fixture files for the AI explain, debug, and test flow
lane, which binds a read-only flow, the evidence links it consumed into logs,
traces, runbooks, and profiles, and the findings it produced — each citing the
evidence link ids that back it.

## Files

- `valid_packet.json` — A fully valid flow evidence packet that passes all
  validation invariants. Mirrors the checked-in support export.
- `uncited_finding_not_flagged.json` — A packet whose uncited `caveat` finding
  leaves `requires_human_confirmation` false, triggering
  `uncited_finding_not_flagged`.
