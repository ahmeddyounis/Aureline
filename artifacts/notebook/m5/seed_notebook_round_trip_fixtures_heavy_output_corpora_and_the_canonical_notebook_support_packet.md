# Notebook Round-Trip Fixtures, Heavy-Output Corpora, and Canonical Notebook Support Packet — Artifact Packet

## Purpose

This artifact packet is the canonical, checked-in evidence for the M5 notebook round-trip fixtures, heavy-output corpora, and support packet lane. It is consumed by:

- `aureline-notebook` crate tests (embedded JSON, parsed at compile time)
- Downstream docs, help, support, and CI surfaces
- Release certification and interoperability reviews

## Packet Contents

The packet enumerates:

1. **Closed vocabularies** — every variant of every class used by round-trip fixtures, heavy-output corpus entries, and the support packet.
2. **Example round-trip fixtures** — worked examples showing clean canonical, attachment-heavy, metadata-rich, unknown-namespace-dense, corrupted-then-repaired, export-only, no-kernel-editable, and cell-id-stress scenarios.
3. **Example heavy-output corpus entries** — worked examples showing small, medium, large, and very-large notebooks with varying trust implications and virtualization strategies.

## Key Invariants

- Non-pass expected results on round-trip fixtures require a `loss_summary`.
- Pass expected results must not carry a `loss_summary`.
- Heavy-output corpus entries must have `output_count > 0`.
- Small size-bucket entries must use `none` virtualization.
- The support packet must list every variant of every closed vocabulary.

## Schema Version

Current schema version: `1`

## Path

```
artifacts/notebook/m5/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet.json
```
