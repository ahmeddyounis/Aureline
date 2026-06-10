# AI Memory Deletion and Export Posture Fixtures

This directory contains fixture files for the AI memory deletion/export posture
lane, which binds the per-class delete and export fan-out over the frozen AI
memory classes (with disclosed retention holds for evidence-governed copies),
accountable, consented, revocable explicit saved memory, and workspace- or
tenant-scoped delete and export operations whose claimed-complete runs address
every class and carry a verified receipt.

## Files

- `valid_packet.json` — A fully valid posture packet that passes all validation
  invariants. Mirrors the checked-in support export.
- `delete_fan_out_incomplete.json` — A packet whose `turn_state` class carries no
  retention hold yet is not covered by the delete fan-out, triggering
  `delete_fan_out_incomplete`.
