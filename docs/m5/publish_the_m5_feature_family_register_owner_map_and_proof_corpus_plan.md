# Publish the M5 feature-family register, owner map, and proof-corpus plan

This document is the human-readable companion to the canonical M5 feature-family register checked in at `artifacts/release/m5/publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan.json`.

## Purpose

The M5 feature-family register publishes the canonical control surface for every M5 depth lane's owner map and proof-corpus plan. It ensures that no notebook, AI, data, framework, review, companion, or managed-depth family may widen without a named owner, a complete proof-corpus plan, and current evidence.

## Structure

The register contains:

- **Family rows** — one per M5 family (`notebook`, `data_rich`, `ai_adjacent`, `framework`, `review`, `companion`, `managed_depth`).
- **Owner map** — each family binds to a canonical owner with a sign-off record.
- **Proof-corpus plan** — per-family list of required proof artifacts (`proof_packet`, `compatibility_report`, `admin_policy`, `rollback_path`, `scorecard`, `owner_attestation`), each with its state and artifact ref.
- **Stop rules** — closed conditions that gate publication. Every gap reason has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed from firing stop rules.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the publication verdict against the stable claim manifest and the M5 feature-train matrix, and fail publication if the register is stale or underqualified.
