# Notebook Save, Repair, and Round-Trip Safety — Artifact Packet

## Purpose

This artifact packet is the canonical, checked-in evidence for the M5 notebook save, repair, and round-trip safety lane. It is consumed by:

- `aureline-notebook` crate tests (embedded JSON, parsed at compile time)
- Downstream docs, help, support, and CI surfaces
- Release certification and interoperability reviews

## Packet Contents

The packet enumerates:

1. **Closed vocabularies** — every variant of every class used by save, repair, and round-trip records.
2. **Example save operations** — worked examples showing full save, auto-save, checkpoint save, and export-derived-format with explicit preservation/loss postures.
3. **Example repair actions** — worked examples showing minted cell IDs, restored attachments, removed corrupt cells, and raw-JSON fallback.
4. **Example round-trip assertions** — worked examples showing pass, partial, and blocked-by-format-boundary results.

## Key Invariants

- `export_derived_format` saves must not claim `round_trip_safe=true`.
- Any non-preserved preservation class requires a `loss_summary`.
- `lossy_with_silent_fallback` is non-conforming and must be rejected.
- Non-pass round-trip results require a `loss_summary`.

## Schema Version

Current schema version: `1`

## Path

```
artifacts/notebook/m5/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces.json
```
