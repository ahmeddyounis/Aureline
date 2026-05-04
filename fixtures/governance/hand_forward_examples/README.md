# Hand-forward worked examples

Cross-functional fixtures for
[`/docs/governance/cross_functional_deliverable_ledger.md`](../../../docs/governance/cross_functional_deliverable_ledger.md),
[`/artifacts/governance/milestone_deliverable_ledger.yaml`](../../../artifacts/governance/milestone_deliverable_ledger.yaml),
and
[`/artifacts/governance/externalization_overlay.yaml`](../../../artifacts/governance/externalization_overlay.yaml).

Each fixture walks one milestone-close hand-forward end-to-end across
the canonical owner-lane categories — program/product, UX/design,
engineering/architecture, quality/security/accessibility/release, and
docs/DevRel/support — demonstrating how design evidence ids,
benchmark corpora, support packs, docs truth, and release packets flow
together. Fixtures cite ledger row ids and overlay row ids verbatim;
they do not invent parallel identifiers.

Cases:

- `foundations_close_hand_forward.yaml` — foundations milestone close
  with the architecture pack, design-evidence index, ADR set, surface
  contracts, benchmark-lab seed, public-truth seed, and support-packet
  index handed forward to prototype.
- `prototype_close_hand_forward.yaml` — prototype milestone close with
  cutline, dogfood design packets, editor-truth packet, dogfood
  verification corpus, dogfood runbook, and help/About truth prototype
  handed forward to alpha.
- `alpha_close_hand_forward.yaml` — alpha milestone close with the
  launch-wedge claim manifest, launch-wedge design pack, launch-wedge
  contracts, launch-wedge benchmark publication, certified-archetype
  alpha report, alpha release packet, and alpha support-window
  statement handed forward to first_beta.

Fixtures are descriptive, not prescriptive: they reuse canonical ledger
row ids and overlay row ids already present in the seeded artifacts,
and they do not introduce new requirement ids, ADR ordinals, surface
ids, or claim row ids.
