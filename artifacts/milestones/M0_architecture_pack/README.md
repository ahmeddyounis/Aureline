# M0 architecture pack

This packet is the Milestone 0 closure binder for the pre-implementation
foundations review. It synthesizes the accepted ADR set, the renderer spike,
the benchmark lab seed, ownership and package topology, open carry-forward
decisions, and the M0 to M1 cutline into one review tree.

Review intent

- Decision requested: conditional M0 close for architecture freeze and
  proceed-to-M1 planning with explicit carry-forward exceptions.
- Prepared on: 2026-04-21.
- Review owner: `@ahmeddyounis`.
- Packet status: review-ready with named yellow/red carry-forwards.

Explicit red/yellow/green calls

| Area | Call | Why |
|---|---|---|
| Renderer viability | Yellow | ADR 0002 is accepted and the spike proves hooks and ownership, but the committed shell spike still runs headless and native window binding is deferred. |
| Benchmark governance | Yellow | The nightly lane, corpus-governance policy, protected-metrics file, and dashboard seed exist, but the council-approved hardware baseline is not seeded yet. |
| Ownership | Yellow | Every protected lane has an owner, but all protected backup coverage still rides the `single-maintainer-backup` waiver through 2026-10-19. |
| Public-truth seeds | Yellow | Docs/help/service-health truth, docs-pack contract, and support-center concept are seeded, but exact-build joins and stale-example enforcement are still open. |
| Unresolved narrowing decisions | Red | Shell home, keyboard model, accessibility packet home, exact-build identity, and several M1-facing contract families remain narrowed or explicitly deferred. |

Packet tree

- `packet_index.yaml` - review index, accepted ADR set, renderer and benchmark summaries, freeze calendar, open-issue table, requirement/assumption/decision/dependency slices, and M0 to M1 cutline.
- `canonical_matrices.yaml` - architecture-driver matrix, principle-enforcement matrix, scheduling and worker-class contract, language/router contract set, late-M0 class matrices, and reserved attach points for later release-control work.
- `coverage_and_freeze_exceptions.yaml` - explicit cross-pack family coverage and every named freeze exception required to keep M1 from inventing missing contracts in code.
- `../M0_scorecard.yaml` - lane-by-lane review surface keyed to the ownership matrix.
- `../M0_risk_register.yaml` - named risks, explicit mitigations, and carry-forward owners.
- `../M0_design_evidence_index.yaml` - stable evidence IDs with owner, freshness, artifact ref, and source task reference.
- `../M0_signoff_packet.md` - shared M0 signoff packet covering the checklist, reviewer matrix, mandatory signed sections, and contract-family coverage cross-checks.

Top-down review order

1. Read `packet_index.yaml` for the review posture, accepted ADR set, and the M0 to M1 cutline.
2. Read `canonical_matrices.yaml` to confirm the driver, principle, scheduler, route, and truth matrices that M1 must inherit.
3. Read `coverage_and_freeze_exceptions.yaml` before approving any M1 breadth so missing contract families stay explicit.
4. Read `../M0_scorecard.yaml` and `../M0_risk_register.yaml` together; the scorecard names lane health and the risk register names why specific yellows and reds still exist.
5. Read `../M0_signoff_packet.md` for the reviewer-facing exit checklist, required signoffs, freshness blockers, and shared packet sections.
6. Use `../M0_design_evidence_index.yaml` as the evidence lookup table instead of reconstructing provenance from handoffs or git history.

M0 to M1 cutline

- M1 may build immediately on the accepted renderer, buffer, VFS, RPC,
  subscription, settings, execution-context, docs-truth, route-taxonomy,
  preview-trust, portability, benchmark, and release-skeleton artifacts.
- M1 must treat shell home, keyboard completeness, accessibility packet
  ownership, exact-build joins, council-approved benchmark baselines, and the
  extension/publication policy family as carry-forward constraints.
- Collaboration, hosted review merge policy, diagnostics/code-action
  convergence, completion/signature-help/snippet truth, localization/locale
  governance, and voice-surface contracts remain intentionally deferred and are
  frozen only as explicit exceptions in this packet.
