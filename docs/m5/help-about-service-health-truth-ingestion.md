# M5 truth-surface evidence ingestion

This page describes how the user- and operator-facing truth surfaces for the M5
feature families — **Help/About**, **service health**, the **release center**,
**support exports**, and **public-truth packs** — stay honest. It belongs to the
M5 learnability and boundary truth lane: every M5 depth lane must remain
teachable and boundary-honest, and no marketed surface may imply a stable or
fully qualified posture that its backing evidence no longer supports.

## The problem this removes: drift

The M5 depth lanes ship behind a set of canonical, checked-in evidence packets.
The single source of truth for *what each family currently claims* and *how far
each family is narrowed below stable* is the M5 feature family register. The
risk is **drift**: a page that once read "stable" and keeps reading "stable"
after the backing evidence aged out, narrowed, hit a policy block, or never
qualified.

The ingestion register fixes the direction of truth flow. Instead of each
surface cloning status text that can rot, every surface **ingests** the
canonical packets and shows the family's current state directly.

## What the register contains

For every M5 feature family the register records one **ingestion row** per truth
surface (7 families × 5 surfaces = 35 rows). Each row:

- ingests the canonical claim **ceiling** and the canonical **effective label**
  straight from the feature family register — never retyped per surface;
- publishes a label that **must equal** the canonical effective label, so a
  surface can never advertise a wider claim than the packet backing it;
- exposes a typed **ingest state** — `current`, `stale`, `narrowed`,
  `policy_blocked`, `preview_only`, or `underqualified` — so a reader can tell
  *why* a lane is below stable without reading internal notes;
- always discloses **posture** — `local_only`, `mirrored`, `managed`, or
  `browser_handoff` — because boundary posture is never inferred; and
- on service-health rows, additionally carries an operational
  **service-contract state** (`ready`, `degraded`, `local_only`, `stale`,
  `policy_blocked`, …).

The register also declares a small set of **contradiction rules** (no widening,
no optimistic state when the source narrowed, mandatory posture disclosure,
mandatory service-health contract state, and source-drift detection) and a
`proceed`/`hold` publication verdict.

## How contradictions are caught

Two layers keep the surfaces honest, and a contradiction fails review in either:

1. **The typed model** (`aureline-release::m5_truth_surface_evidence_ingestion`)
   enforces the clock-independent invariants — the no-widening ceiling,
   state/label coherence, posture disclosure, service-contract wiring, family
   and surface coverage, and the publication verdict. These run as Rust tests.
2. **The CI gate** (`tools/ci/m5/truth_surface_ingestion_check.py`) re-reads the
   canonical feature family register and fails if any ingested ceiling or
   effective label has drifted from the live source — this is the load-bearing
   contradiction check that catches stale cloned copy. It also validates the
   artifact against the boundary schema and confirms the checked-in artifact
   matches the regenerator output.

## Regenerating

The artifact is derived, not hand-authored. To regenerate after the source
family register changes:

```bash
python3 tools/regenerate_m5_truth_surface_ingestion.py
```

Use `--check` to verify the checked-in copy is current without rewriting it
(this is what CI runs).

## Canonical files

- Schema: `schemas/governance/m5_truth_surface_evidence_ingestion.schema.json`
- Artifact: `artifacts/release/m5/m5_truth_surface_evidence_ingestion.json`
- Validation capture:
  `artifacts/release/m5/captures/m5_truth_surface_evidence_ingestion_validation_capture.json`
- Typed model: `crates/aureline-release/src/m5_truth_surface_evidence_ingestion/`
- Regenerator: `tools/regenerate_m5_truth_surface_ingestion.py`
- CI gate: `tools/ci/m5/truth_surface_ingestion_check.py`

The register is canonical for this lane: later dashboards, docs/help surfaces,
release-center views, and support exports should ingest it instead of cloning
status text.
