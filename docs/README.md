# Docs index

Aureline is an open-source next-generation IDE (working name). The
repository is in its pre-implementation stage; these documents describe
the governance, ownership, and build discipline that precede source
code.

## Governance

- [`governance/dri_map.md`](./governance/dri_map.md) — DRI, backup
  owners, blocker aging, and narrowing authority.
- [`governance/control_artifact_index.md`](./governance/control_artifact_index.md)
  — overview of the control-artifact index: one home, one owner,
  and one review path for every control asset.
- [`governance/interface_inventory.md`](./governance/interface_inventory.md)
  — outline of interface-inventory categories and owning lanes.
- [`governance/benchmark_council_charter.md`](./governance/benchmark_council_charter.md)
  — seed charter for the benchmark council (roles, scope, cadence,
  quorum placeholder, escalation).
- [`governance/decision_backlog.md`](./governance/decision_backlog.md)
  — seeded architecture decisions with freeze dates and default
  narrowing postures.
- [`governance/decision_workflow.md`](./governance/decision_workflow.md)
  — how decisions open, close, supersede, and narrow.
- [`governance/templates/`](./governance/templates/) — waiver,
  verification packet, and freeze-exception templates.
- [`governance/provenance_and_compliance_baseline.md`](./governance/provenance_and_compliance_baseline.md)
  — IP, provenance, and supply-chain baseline that pairs with
  [`/CONTRIBUTING.md`](../CONTRIBUTING.md).
- [`governance/record_state_and_policy_simulation_models.md`](./governance/record_state_and_policy_simulation_models.md)
  — governed-record state model (local_only / managed_copy / held /
  delete_requested / delete_complete / export_available), orthogonal
  copy / hold / delete-request / export axes, chronology packet
  (utc_instant, civil-time rendering, monotonic duration, skew /
  partial-order flags, export and rendering representation rules),
  policy-simulation vocabulary, and waiver / remembered-decision
  expiry envelope. Boundary schemas in
  [`/schemas/governance/record_state.schema.json`](../schemas/governance/record_state.schema.json)
  and
  [`/schemas/governance/waiver_expiry.schema.json`](../schemas/governance/waiver_expiry.schema.json);
  worked fixtures (including the required mixed-chronology admin
  timeline) in
  [`/fixtures/governance/record_state_examples/`](../fixtures/governance/record_state_examples/).

## Decision records

- [`adr/README.md`](./adr/README.md) — Architecture Decision Records
  (how and when to write one).
- [`adr/0000-template.md`](./adr/0000-template.md) — ADR template.
- [`rfc/README.md`](./rfc/README.md) — Requests for Comment (how and
  when to open one).
- [`rfc/0000-template.md`](./rfc/0000-template.md) — RFC template.

## Repository topology and build

- [`repo/topology.md`](./repo/topology.md) — package topology.
- [`repo/dependency_rules.md`](./repo/dependency_rules.md) — allowed
  dependency directions between packages.
- [`build/reproducible_build_baseline.md`](./build/reproducible_build_baseline.md)
  — pinned toolchain, bootstrap command, and build-identity record.

## Product boundary

- [`product/boundary_manifest_strawman.md`](./product/boundary_manifest_strawman.md)
  — strawman that classifies every product capability against the
  open-source core versus managed / commercial / service-plane
  boundary. Reserves deployment-profile, residual-dependency,
  data-boundary, portability, and local-core-continuity slots per
  row so later claim packets do not retrofit them inconsistently.
  Conforms to
  [`/schemas/product/boundary_manifest.schema.json`](../schemas/product/boundary_manifest.schema.json).

## Frozen vocabularies

- [`search/search_readiness_vocabulary.md`](./search/search_readiness_vocabulary.md)
  — copy-guidance companion to the search readiness, ranking-reason,
  hidden-scope, result-truth, and deep-link drift ADR. Frozen sentence
  corpus every palette, full-search, symbol-jump, docs-search,
  graph-overlay, AI-explanation-overlay, and support-export surface
  quotes. Machine-readable corpus in
  [`/artifacts/search/result_truth_labels.yaml`](../artifacts/search/result_truth_labels.yaml);
  boundary schema in
  [`/schemas/search/search_result_truth.schema.json`](../schemas/search/search_result_truth.schema.json);
  worked fixtures in
  [`/fixtures/search/result_truth_examples/`](../fixtures/search/result_truth_examples/).
- [`ux/shell_interaction_safety_contract.md`](./ux/shell_interaction_safety_contract.md)
  — shell-level interaction-safety contract covering focus return,
  batch-scope review, preview / apply / revert, typed permission
  prompts, safe preview of high-risk content classes,
  representation-labeled copy / export, and responsive fallback on
  protected shell surfaces. Boundary schema in
  [`/schemas/ux/interaction_safety.schema.json`](../schemas/ux/interaction_safety.schema.json);
  worked fixtures (destructive core path, publish-capable /
  externally-mutating path, responsive-fallback denial) in
  [`/fixtures/ux/interaction_safety_cases/`](../fixtures/ux/interaction_safety_cases/).
- [`ai/context_assembly_contract.md`](./ai/context_assembly_contract.md)
  — AI context-assembly contract covering included / omitted /
  pinned / redacted / policy-blocked / tainted context segments
  with stable segment ids and provenance refs; route / spend
  truth (provider / model / path / cost visibility) with planned
  and receipted records; reserved typed prompt-composer session,
  mention, attachment, and turn-draft slots; evidence-packet
  seed with tainted-content fence rules that survive background
  branch-agent and review-handoff dispatches without silent
  downgrade. Boundary schemas in
  [`/schemas/ai/context_assembly.schema.json`](../schemas/ai/context_assembly.schema.json)
  and
  [`/schemas/ai/evidence_packet.schema.json`](../schemas/ai/evidence_packet.schema.json);
  worked fixtures (inline composer turn with tainted retrieved
  context, background branch-agent dispatch that preserves
  tainted-usage constraints across the handoff) in
  [`/fixtures/ai/context_assembly_cases/`](../fixtures/ai/context_assembly_cases/).
- [`workspace/entry_restore_object_model.md`](./workspace/entry_restore_object_model.md)
  — first-run, open, clone, import, add-root, restore, resume, and
  start-from-snapshot vocabulary covering project-entry action
  records, recent-work rows, restore prompts, and
  migration-result records. Freezes the seven-entry-verb set,
  the target-kind set, the resulting-mode set, restore-level
  controlled terms (`exact_restore`, `compatible_restore`,
  `layout_only`, `recovered_drafts`, `evidence_only`,
  `no_restore`), the missing-target-state set, the
  session-scoped execution-posture set, the checkpoint-linked
  recovery-class set, and the migration-item outcome set
  (`exact`, `translated`, `approximated`, `skipped`, `blocked`,
  `needs_review`, `rollback_available`). Reserves typed slots
  for category-specific parity scores, equivalence-map rows,
  post-import validation refs, and the four migration-handoff
  next-step decisions (`roll_back_import`, `keep_imported_state`,
  `adopt_recommended_bundle`, `review_unsupported_items`).
  Boundary schema in
  [`/schemas/workspace/entry_and_restore_result.schema.json`](../schemas/workspace/entry_and_restore_result.schema.json);
  worked fixtures (open local folder, clone remote repo, VS Code
  settings import + migration result, restore last session,
  resume managed workspace, start from prebuild, recent-work row
  with missing target) in
  [`/fixtures/workspace/entry_restore_examples/`](../fixtures/workspace/entry_restore_examples/).
- [`runtime/target_discovery_and_install_review_taxonomy.md`](./runtime/target_discovery_and_install_review_taxonomy.md)
  — target-discovery confidence, host-boundary cue, managed-
  workspace lifecycle, notebook-trust ladder, structured round-
  trip risk, and install-review summary-slot vocabulary every
  launch-review, managed-workspace control plane, notebook-trust
  gate, and install / update review surface projects against.
  Reserves notebook-trust rung, structured round-trip preview
  state, irreversibility / disclosure flag, and install-review
  slot fields so mixed-trust notebooks and rich-document previews
  stay reviewable once those lanes land. Machine-readable
  managed-workspace lifecycle matrix (per-state minimum fields,
  admissible transitions, activation-budget slices, conformance
  tests, worked transition scenarios) in
  [`/artifacts/runtime/managed_workspace_lifecycle.yaml`](../artifacts/runtime/managed_workspace_lifecycle.yaml);
  example compatibility-label and activation-budget-summary
  packet shapes are embedded in the taxonomy doc.

## Machine-readable registers

These live outside `docs/` because tooling reads them; the narrative
above is paired with a YAML form that is authoritative for automation:

- [`/artifacts/governance/ownership_matrix.yaml`](../artifacts/governance/ownership_matrix.yaml)
  — DRI, backup owners, and waivers.
- [`/artifacts/governance/control_artifact_index.yaml`](../artifacts/governance/control_artifact_index.yaml)
  — canonical location, owner, review cadence, visibility class,
  and next-milestone target for every control asset.
- [`/artifacts/governance/issue_routing.yaml`](../artifacts/governance/issue_routing.yaml)
  — public / private routing, privacy class, disclosure class,
  public-summary expectation, and owning forum per issue class.
- [`/artifacts/governance/decision_index.yaml`](../artifacts/governance/decision_index.yaml)
  — decision rows with freeze dates and default-if-unresolved postures.
- [`/artifacts/governance/package_inventory.yaml`](../artifacts/governance/package_inventory.yaml)
  — package topology and protected-path posture.
- [`/artifacts/governance/compliance_checklist.yaml`](../artifacts/governance/compliance_checklist.yaml)
  — register of dependencies, vendored sources, generators, and
  pending notice rows.
- [`/artifacts/governance/milestone_scorecard_template.yaml`](../artifacts/governance/milestone_scorecard_template.yaml)
  — per-milestone lane status.
- [`/artifacts/governance/governance_packet_template.yaml`](../artifacts/governance/governance_packet_template.yaml)
  — verification, benchmark-report, compatibility-report, claim-manifest,
  shiproom, and waiver-register packet families.
- [`/schemas/governance/`](../schemas/governance/) — schemas the YAML
  registers conform to.
- [`/schemas/product/boundary_manifest.schema.json`](../schemas/product/boundary_manifest.schema.json)
  — contract for the product boundary manifest strawman.
