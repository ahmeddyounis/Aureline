# Publish the M5 local-model, provider-graduation, and spend-governance control packet

This document is the human-readable companion to the canonical M5 control-packet register checked in at `artifacts/release/m5/publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet.json`.

## Purpose

The M5 control-packet register publishes the canonical control surface for every M5 depth lane's local-model, provider-graduation, and spend-governance posture. It ensures that no local-model, provider-graduation, or spend-governance lane may widen without a complete control-packet story, a named owner, and current evidence.

## Structure

The register contains:

- **Lane rows** — one per M5 control-packet lane (`local_model`, `provider_graduation`, `spend_governance`).
- **Control-packet story** — per-lane set of required story items (`local_model_capability`, `provider_graduation_path`, `spend_governance_policy`, `privacy_trust_posture`, `rollback_downgrade_path`, `compatibility_interop`), each with its state and artifact ref.
- **Owner map** — each lane binds to a canonical owner with a sign-off record.
- **Stop rules** — closed conditions that gate publication. Every gap reason has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed from firing stop rules.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the publication verdict against the stable claim manifest and the M5 feature-train matrix, and fail publication if the register is stale or underqualified.
