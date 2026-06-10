# Experiment Run Identities, Environment Fingerprints, Dataset Cards, and Artifact Lineage

## Purpose

This packet is the canonical checked-in artifact for the M05-028 task:
**Implement experiment run identities, environment fingerprints, dataset cards, and artifact lineage**.

It defines the closed vocabularies, worked examples, and schema contracts that
downstream docs, help, support, CI, and product surfaces ingest instead of
cloning status text.

## Scope

- **Experiment run identities** — stable run IDs, titles, source references,
  timestamps, outcome classes, commit/revision provenance, execution origin,
  and environment fingerprint references.
- **Environment fingerprints** — human-readable environment identity,
  interpreter/kernel labels, package/toolchain summaries, target origin,
  policy epochs, and freshness state.
- **Dataset cards** — dataset identity, source class, version/snapshot notes,
  size estimates, sensitivity/redaction class, and location class.
- **Artifact lineage** — artifact identity, producing run reference,
  generator step label, environment fingerprint reference, save location,
  and lineage state (current, stale, diverged, orphaned, imported).

## Local-first experiment traceability

These records enforce local-first experiment traceability: run identity,
dataset provenance, artifact lineage, and environment fingerprint stay
inspectable and portable without requiring a hosted tracking product to become
the hidden source of truth.

## Downstream consumers

- `crates/aureline-notebook` — module
  `implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage`
- `schemas/notebook/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage.schema.json`
- `fixtures/notebook/m5/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage/`
- `docs/notebook/m5/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage.md`

## Freshness

- Packet version: `1`
- Current as of: `2026-06-09T00:00:00Z`
- Schema breaking changes bump `schema_version`; additive-optional fields do not.
