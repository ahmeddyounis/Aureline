# M3 proof-consumption walkthrough

This walkthrough is the consumer-facing readout for the M3 public-proof
dry run. It exists so partner reviewers, docs writers, and support
readers can resolve a public-proof claim end-to-end without private
tribal knowledge: pick a row, read the canonical packet, follow the
freshness contract, and inspect the rebuild artifacts.

The walkthrough cites stable ids from the public-proof index and the
benchmark / clean-room rebuild rehearsal packets; it does not duplicate
their prose. If a paragraph here disagrees with the canonical block of
one of those packets, the canonical block wins.

## Audience and reading order

This document is written for three reader classes; each one can stop
at the section it needs.

| Reader class | What you need | Stop here |
|---|---|---|
| Partner reviewer | Confirm the beta train resolves to one canonical packet per claim family, with a quoteable methodology and known-limits set. | [§Partner reviewer path](#partner-reviewer-path) |
| Docs writer | Identify which public claim wording is admitted, which is narrowed, and which is currently `evidence_stale`. | [§Docs writer path](#docs-writer-path) |
| Support reader | Resolve a customer question to one stable packet id, the freshness window, and the rerun-trigger set that expires the packet. | [§Support reader path](#support-reader-path) |

## Canonical entrypoints

Every reader path resolves through these entrypoints. Treat the ids
below as the addressable surface of the M3 public-proof lane; downstream
docs, partner letters, and support exports MUST cite these ids rather
than restate them.

| Entrypoint | Ref |
|---|---|
| Public-proof index (M3) | `artifacts/milestones/m3/public_proof_index.md` |
| Publication shelf-life policy | `docs/governance/m3/publication_shelf_life_policy.md` |
| Review-packet template | `artifacts/milestones/m3/review_packet_template.md` |
| Claim manifest (M3 beta rows) | `artifacts/release/m3/claim_manifest.md` |
| Public benchmark beta packet | `artifacts/benchmarks/m3/publication_packet/packet.md` |
| Benchmark council notes | `artifacts/benchmarks/m3/benchmark_council_notes.md` |
| Benchmark publication dry-run packet | `artifacts/benchmarks/m3/publication_dry_run/packet.md` |
| Clean-room rebuild rehearsal packet | `artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md` |
| Shared build identity | `artifacts/build/build_identity.json` |
| Cross-milestone freshness SLOs | `artifacts/governance/evidence_freshness_slos.yaml` |
| Cross-milestone rerun triggers | `artifacts/governance/evidence_rerun_triggers.yaml` |

## Partner reviewer path

1. Open the public-proof index. The canonical YAML block (delimited
   by `BEGIN canonical:public_proof_index` /
   `END canonical:public_proof_index`) is the authoritative row set.
2. Pick a row by `row_id` (for example
   `m3_public_proof:benchmark_publication`).
3. Resolve `canonical_packet_ref` to the packet that speaks for that
   claim family. For the benchmark lane, the beta packet is:
   - public benchmark beta packet →
     `artifacts/benchmarks/m3/publication_packet/packet.md`
   The earlier rehearsal packets remain linked as supporting evidence:
   - benchmark publication →
     `artifacts/benchmarks/m3/publication_dry_run/packet.md`
   - clean-room rebuild rehearsal →
     `artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md`
4. In each packet, read the canonical block. Confirm:
   - `result_status` is `methodology_only` or `rehearsal_only` — the
     dry run does NOT admit public head-to-head or release-grade
     wording.
   - `freshness.proof_class_id` and `freshness.stale_after` match the
     M3 publication shelf-life policy ceilings.
   - `artifact_links.exact_build_identity_refs` points at the same
     build identity the M3 claim manifest cites.
5. Inspect the named known-limit refs to see what is explicitly NOT
   claimed.
6. (Optional) Re-run the validator yourself:
   ```sh
   python3 ci/check_m3_public_benchmark_beta.py --repo-root .
   python3 ci/check_m3_publication_rehearsal.py --repo-root .
   ```
   A passing run means every row, ref, and freshness window in the
   two new packets is consistent with the index, the shelf-life
   policy, the SLO catalog, and the rerun-trigger catalog.

A partner who can complete steps 1–5 above without contacting the
maintainer team has confirmed the lane is inspectable outside the core
team. That outcome is the acceptance state for this milestone task.

## Docs writer path

1. Read the claim manifest summary table in
   `artifacts/release/m3/claim_manifest.md`. The `Effective posture`
   column is the wording cap docs may use.
2. For each claim family, the matching public-proof index row names
   the canonical packet, the freshness window, and the stale
   propagation profile. Docs copy MUST resolve through these ids.
3. The publication shelf-life policy lists the per-family freshness
   ceiling and the per-family rerun-trigger set. A docs page that
   quotes a public-proof row MUST quote it by id and MUST NOT widen
   the freshness window or omit a rerun trigger.
4. The beta public benchmark packet and the dry-run packets do NOT
   widen any docs wording. They add:
   - the methodology + known-limits set for the benchmark publication
     lane, governed by
     `artifacts/benchmarks/m3/publication_packet/packet.md` and
     `artifacts/benchmarks/m3/benchmark_council_notes.md`; and
   - the rebuild artifact-family map for the exact-build identity
     lane.
   Both are sourced from already-published rows (the protected-fitness
   dashboard, the alpha clean-room rebuild dry run, and the shared
   build identity).
5. When a dashboard tile or compatibility row reads
   `evidence_stale`, docs MUST render the matching downgrade copy from
   the stale-propagation profile in
   `artifacts/governance/evidence_freshness_slos.yaml` and MUST NOT
   render certified, supported, or replacement-grade wording for that
   row.

## Support reader path

1. Take the customer question (e.g. "is the rollback claim certified
   in beta?"). Find the claim family by reading the row matrix in the
   compatibility report or the claim manifest summary.
2. Map the claim family to the public-proof index row:
   - `boundary_truth`, `exact_build_identity`, `docs_freshness` →
     resolve through the claim manifest and the clean-room rebuild
     rehearsal packet.
   - `benchmark_publication`, `launch_wedge` → resolve through the
     public benchmark beta packet, dashboard snapshot, protected-fitness
     catalog, and benchmark publication dry-run packet.
   - `version_skew_truth` → resolve through the compatibility report.
3. Quote the row's `canonical_packet_ref` and the row's freshness
   window to the customer. Do NOT paraphrase the row prose; cite the
   ref and let the customer read the canonical block.
4. If a customer reports that a tile or row appears stale, run:
   ```sh
   python3 ci/check_m3_public_benchmark_beta.py --repo-root .
   python3 ci/check_m3_publication_rehearsal.py --repo-root .
   python3 ci/check_m3_public_proof_index.py --repo-root .
   ```
   The validators write capture JSON under
   `artifacts/benchmarks/m3/publication_dry_run/captures/` and
   `artifacts/milestones/m3/captures/`. The capture is the support
   answer; do not invent one out of band.
5. When a rerun trigger fires (hardware image, fixture revision,
   protected-metrics change, build-identity chain change), the packet
   expires immediately. Support exports MUST render the
   stale-propagation profile downgrade and MUST cite the matching
   trigger id from
   `artifacts/governance/evidence_rerun_triggers.yaml`.

## Acceptance evidence

This walkthrough is acceptable as the M3 proof-consumption exercise
only while these statements remain true:

- A partner reviewer can complete the partner-reviewer path using only
  the entrypoints listed above and the validator command.
- A docs writer can identify the admitted public wording without
  re-reading prose outside the index, the policy, and the two new
  packets.
- A support reader can resolve a customer question to one stable
  packet id and one freshness window without contacting the core team.
- The validator (`ci/check_m3_publication_rehearsal.py`) passes on the
  current tree.
- The public benchmark copy gate
  (`ci/check_m3_public_benchmark_beta.py`) passes on the current tree.

## Refresh trigger

Refresh this walkthrough when any of these change:

- `artifacts/milestones/m3/public_proof_index.md`
- `artifacts/benchmarks/m3/publication_packet/packet.md`
- `artifacts/benchmarks/m3/benchmark_council_notes.md`
- `artifacts/benchmarks/m3/publication_dry_run/packet.md`
- `artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md`
- `docs/governance/m3/publication_shelf_life_policy.md`
- `artifacts/release/m3/claim_manifest.md`

## Failure drill

To confirm the walkthrough stays inspectable:

1. Temporarily remove or rename one canonical entrypoint above (for
   example, the benchmark dry-run packet).
2. Re-run `python3 ci/check_m3_publication_rehearsal.py --repo-root .`;
   the validator MUST fail and name the missing entrypoint.
3. Restore the entrypoint and re-run; it MUST pass.
