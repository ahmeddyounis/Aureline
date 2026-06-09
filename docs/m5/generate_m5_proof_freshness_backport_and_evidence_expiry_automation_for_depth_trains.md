# Generate M5 proof-freshness, backport, and evidence-expiry automation for depth trains

This document is the human-readable companion to the canonical M5 depth-train automation register checked in at `artifacts/release/m5/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains.json`.

## Purpose

The M5 depth-train automation register governs proof-freshness SLO enforcement, backport eligibility tracking, and evidence expiry automation for every M5 depth lane. It ensures that no notebook, AI, data, framework, review, companion, or managed-depth row may widen without current proof, open backport windows, unexpired evidence, and owner sign-off.

## Structure

The register contains:

- **Lane rows** — one per M5 lane (`notebook`, `data_rich`, `ai_adjacent`, `framework`, `review`, `companion`, `managed_depth`).
- **Proof packets** — per-lane proof packet with freshness SLO and state.
- **Backport eligibility** — per-lane backport kind, window, and policy refs.
- **Evidence expiry records** — per-lane evidence kind, capture date, expiry date, and expired flag.
- **Stop rules** — closed conditions that gate promotion. Every gap reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from firing stop rules.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
