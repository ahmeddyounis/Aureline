# Finalize Experiments/Labs Inventory, Kill-Switch Visibility, and Release-Claim Alignment

**Artifact ref:** `artifacts/release/m4/finalize-experiments-labs-inventory.md`  
**Contract ref:** `release:finalize_experiments_labs_inventory:v1`  
**Schema version:** 1  
**As of:** 2026-05-14

## Purpose

This artifact certifies that every experiment or Labs flag affecting a claimed
surface is visible, exportable, and bounded by owner, cohort, expiry, rollout
ring, and kill-switch metadata. Stable claims do not silently depend on hidden
experiment state.

## Certification scope

The certification page (`FinalizeExperimentsLabsInventoryPage`) binds, for every
capability row in the experiments inventory:

1. **Inventory completeness** — `capability_id`, `owner`, `declared_lifecycle_state`,
   `effective_lifecycle_state`, `review_or_expiry_date`, enrollment scope,
   cohort/ring, and public label.
2. **Kill-switch visibility** — every `DisabledByPolicy` row exposes the winning
   disable-source class, reason, preserved-data scope, and fallback path.
3. **Dependency-marker truth** — every saved artifact, bundle, sync packet, and
   migration export that depends on a non-stable capability carries a visible
   marker.
4. **Release-claim alignment** — every experiment row maps to the stable-claim
   manifest entry whose lifecycle label it backs.
5. **Controlled vocabulary** — the lifecycle vocabulary is exactly `Labs`,
   `Preview`, `Beta`, `Stable`, `Deprecated`, `DisabledByPolicy`, `Retired`.

## Acceptance criteria

- Every experiment or Labs flag affecting a claimed surface has owner, cohort,
  expiry, kill switch, public label class, and dependency marker visible in UI,
  CLI, export, and support packets.
- Stable claims do not silently depend on hidden experiment state.
- Kill-switch drills preserve durable user data and emit export-safe explanation.
- Hidden experiment dependencies on marketed stable rows are zero.

## Canonical paths

- **Doc:** `docs/release/m4/finalize-experiments-labs-inventory.md`
- **Fixture:** `fixtures/release/m4/finalize-experiments-labs-inventory/`
- **Source:** `crates/aureline-release/src/finalize_experiments_labs_inventory/mod.rs`
- **Bin:** `cargo run -q -p aureline-release --bin aureline_release_finalize_experiments_labs_inventory -- page`
