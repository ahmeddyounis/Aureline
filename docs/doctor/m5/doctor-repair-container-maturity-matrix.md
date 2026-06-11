# Project Doctor, guided-repair, and container/devcontainer maturity matrix

This document describes the canonical packet that certifies blocked-user recovery
and container/devcontainer boundary maturity across every deployment profile, and
that automatically narrows any underqualified cell before publication. It is the
user-facing companion to the governed artifact at
`artifacts/doctor/m5/doctor-repair-container-maturity-matrix.json` and the typed
model in the `aureline-doctor` crate
(`freeze_the_m5_project_doctor_guided_repair_and_container_or_devcontainer_maturity_matrix`).

## What this packet covers

The packet is a **maturity matrix**: one row for every (capability, profile) cell.
The claimed recovery capabilities are:

1. **`project_doctor`** — Project Doctor probe families, finding codes, and
   explainability.
2. **`guided_repair`** — guided-repair / repair-transaction classes and reversal
   behavior.
3. **`container_boundary`** — container and devcontainer boundary depth (engine,
   mount, port, tunnel scope).

Each capability is certified independently against every deployment profile:
`local_workspace`, `remote_ssh`, `container`, and `devcontainer`.

Each row answers, for its cell:

- **What does the lane claim?** A `declared_maturity` of `certified`,
  `provisional`, `underqualified`, or `unsupported`.
- **How fresh is the proof?** An `evidence_freshness` of `current`, `stale`,
  `expired`, or `unknown`.
- **How safe is the repair?** A `reversal_class` of `reversible`, `checkpointed`,
  `irreversible`, or `not_applicable` (read-only lanes).
- **Which surfaces carry it?** A `support_parity` of `full`, `desktop_cli`,
  `desktop_only`, or `unavailable`.
- **What is backing it?** A `scorecard_ref`, a `latency_corpus_ref` to the cell's
  diagnosis-latency proof corpus, a `rollback_ref` to its durable rollback path, a
  `compatibility_ref` to its compatibility/downgrade story, and an optional
  `admin_policy_ref` where one is relevant.
- **What blocks promotion?** Zero or more `blocking_reasons`.
- **What does the gate publish?** A `published_maturity` and a `narrowing_action`
  derived from the inputs above.

## The promotion gate narrows automatically

The maturity a row may publish is **not** copied from `declared_maturity`. It is
recomputed from the row's freshness, blocking reasons, and evidence, and the
`published_maturity` and `narrowing_action` fields must equal that recomputation
or validation fails. The gate lowers the published maturity to the weakest of:

- the **declared maturity**;
- the **freshness ceiling** — `current` permits `certified`, `stale`/`unknown`
  cap at `provisional`, and `expired` caps at `underqualified`;
- each **blocking-reason ceiling** — `stale`, `engine_unavailable`, and
  `latency_slo_breached` cap at `provisional`; `missing_proof_corpus`,
  `missing_rollback_path`, and `boundary_unverified` cap at `underqualified`;
- an **evidence ceiling** — a row missing its scorecard, latency corpus, rollback
  path, or flagged with a missing-evidence blocking reason caps at
  `underqualified`.

The `narrowing_action` then names the result: `none` for a published `certified`,
`narrow_to_provisional`, `narrow_to_underqualified`, or `withhold_from_publication`
for `unsupported`.

This is what lets release/public-truth tooling **prove** that stale or
underqualified cells narrow before publication: a row that is stale,
engine-unavailable, latency-breached, or missing rollback/boundary/corpus evidence
simply cannot carry a `certified` published claim, because the recomputed gate
decision overrides the stored one.

## Recovery and repair truth stays cell-specific

A strong local lane must never imply maturity on a remote or containerized one.
The packet enforces this several ways:

- Every claimed (capability, profile) cell must carry exactly one row
  (`MissingMatrixCell` / `DuplicateMatrixCell` otherwise), so no cell inherits
  trust from an adjacent one.
- Every row must carry its own non-empty `scorecard_ref`, `latency_corpus_ref`,
  `rollback_ref`, and `compatibility_ref`, and a row may not cover a capability
  outside the claimed set (`UnclaimedCapabilityRow`).
- Every `guided_repair` cell must carry a concrete reversal class — never
  `not_applicable` (`RepairLaneMissingReversalClass`) — so repair safety cannot
  widen through support folklore.

A promotable row — one that publishes `certified` — must additionally be clean:
current freshness and no blocking reason (`PromotedRowNotClean` otherwise).

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with the capability,
profile, declared and published maturity, freshness, reversal class, support
parity, narrowing action, and blocking-reason tokens, plus `promotable_count`,
`narrowed_count`, and `withheld_count`. Help/About, docs/help, support exports, and
release/public-truth packets — including the M5 recovery/container release freeze
matrix — should ingest this projection directly rather than restating recovery or
container status by hand, so the public and internal claim surfaces use the same
lifecycle, freshness, reversal, and downgrade vocabulary as the underlying packet.

## Validation

`DoctorRepairContainerMaturityMatrix::validate()` reports every violation,
including unsupported schema version or record kind, non-canonical closed
vocabularies, empty required fields, duplicate row ids, duplicate or missing matrix
cells, unclaimed-capability rows, duplicate blocking reasons, a guided-repair cell
missing its reversal class, an overstated published maturity, a narrowing action
that disagrees with the gate, a promotable row that is not clean, and a summary
block that disagrees with the rows.
