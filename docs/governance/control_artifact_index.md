# Control-artifact index

This document is the human-readable overview of the control-artifact
index. It exists so that every control asset the foundations milestone
produces has one canonical home, one named owner, and one review path
— and so that engineering, design, QE, docs, support, and release can
all find that home from a single page.

Companion artifacts:

- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — machine-readable register. Tooling reads this file; the narrative
  below describes the same rows.
- [`./source_appendix_completion_audit.md`](./source_appendix_completion_audit.md)
  and
  [`/artifacts/governance/source_seed_completion_matrix.yaml`](../../artifacts/governance/source_seed_completion_matrix.yaml)
  — appendix-seed completion audit and machine-readable crosswalk that
  keeps source-document appendix promises traceable to concrete repo
  artifacts (or explicit waivers), plus a merge gate at
  [`/ci/check_source_seed_completion.py`](../../ci/check_source_seed_completion.py).
- [`/docs/network/transport_permission_matrix.md`](../network/transport_permission_matrix.md),
  [`/artifacts/network/permission_classes.yaml`](../../artifacts/network/permission_classes.yaml),
  and
  [`/artifacts/network/mirror_offline_matrix.yaml`](../../artifacts/network/mirror_offline_matrix.yaml)
  — shared network permission-class, audit-requirement, and
  mirror/offline matrix. Worked examples live under
  [`/fixtures/network/audit_event_examples/`](../../fixtures/network/audit_event_examples/).
- [`/docs/network/transport_explainability_surface_contract.md`](../network/transport_explainability_surface_contract.md)
  and
  [`/fixtures/network/transport_explainability_cases/`](../../fixtures/network/transport_explainability_cases/)
  — transport summary strip, endpoint row, certificate/detail card,
  denied-attempt history, repair action, and no-bypass projection
  contract and worked cases.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — canonical public/private issue-and-RFC routing matrix, disclosure
  transitions, and escalation paths. Normative narrative lives in
  [`./issue_routing_matrix.md`](./issue_routing_matrix.md); worked
  examples live under
  [`/fixtures/governance/issue_routes/`](../../fixtures/governance/issue_routes/).
- [`./feature_flag_policy.md`](./feature_flag_policy.md)
  — normative policy for experiments, feature flags, Labs inventory,
  rollouts, policy disables, and kill switches.
- [`./experiment_expiry_and_schema_review_contract.md`](./experiment_expiry_and_schema_review_contract.md)
  — contract for experiment expiry/graduation and schema-governance
  mismatch cues across import, sync, downgrade/open, restore, and
  support export.
- [`../policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  — local admin-policy artifact and signed bundle-cache contract,
  including precedence, safe-default, last-known-good, emergency-disable,
  and explain/export rules.
- [`/artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml)
  — canonical register for every control row, including hidden
  developer toggles and reserved control-stack bindings.
- [`/artifacts/governance/labs_register.yaml`](../../artifacts/governance/labs_register.yaml)
  — contributor-visible Labs / prototype / preview inventory
  projected from the canonical register.
- [`/artifacts/governance/experiment_graduation_matrix.yaml`](../../artifacts/governance/experiment_graduation_matrix.yaml)
  and [`/fixtures/governance/experiment_edge_cases/`](../../fixtures/governance/experiment_edge_cases/)
  — lifecycle action packet requirements plus worked edge cases for
  stale/expired experiment handling and copy-safe schema mismatch cues.
- [`/artifacts/governance/requirement_register_seed.yaml`](../../artifacts/governance/requirement_register_seed.yaml)
  — machine-readable canonical requirement register consumed by
  scorecards, packets, waivers, and release evidence.
- [`./normative_requirement_policy.md`](./normative_requirement_policy.md)
  — interpretation policy for requirement IDs, BCP 14 keyword language,
  advisory prose, illustrative examples, document precedence, waiver
  expiry, stale verified requirements, and mandatory review artifacts.
- [`/artifacts/governance/requirement_lifecycle_states.yaml`](../../artifacts/governance/requirement_lifecycle_states.yaml)
  — machine-readable requirement-family and lifecycle-state vocabulary
  consumed by requirement rows, waivers, verification packets, and
  signoff automation.
- [`/artifacts/governance/verification_classes.yaml`](../../artifacts/governance/verification_classes.yaml)
  — canonical verification-class ids for evidence packets, scorecards,
  benchmark reports, compatibility reports, and release review.
- [`/artifacts/governance/mandatory_review_artifacts.yaml`](../../artifacts/governance/mandatory_review_artifacts.yaml)
  — when ADRs, RFCs, design packets, verification packets,
  compatibility reports, benchmark reports, and waiver packets are
  mandatory, plus their minimum contents.
- [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml)
  — machine-readable stable-surface and future stable-surface
  inventory. Compatibility, docs, migration, and deprecation work cite
  row refs here.
- [`/artifacts/governance/package_inventory.yaml`](../../artifacts/governance/package_inventory.yaml)
  — machine-readable workspace package inventory with protected/off-cone
  posture and allowed internal dependency edges. Pairs with
  [`/docs/repo/dependency_rules.md`](../repo/dependency_rules.md).
- [`./benchmark_council_charter.md`](./benchmark_council_charter.md)
  — charter seed for the benchmark council (roles, scope, cadence).
- [`./forum_charters.md`](./forum_charters.md) and
  [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
  — standing decision-forum charter pack and machine-readable forum
  matrix.
- [`./forum_packet_templates.md`](./forum_packet_templates.md)
  — required input-packet profiles and output-landing rules for the
  standing forums.
- [`./decision_rights_and_signoff_matrix.md`](./decision_rights_and_signoff_matrix.md),
  [`/artifacts/governance/signoff_matrix.yaml`](../../artifacts/governance/signoff_matrix.yaml),
  and
  [`/artifacts/governance/promotion_decision_rows.yaml`](../../artifacts/governance/promotion_decision_rows.yaml)
  — role-based decision-rights, concurrence, packet, evidence-bundle,
  degraded-ownership, and reconstruction rows for launch-bearing
  decisions.
- [`./requirement_alias_crosswalk.md`](./requirement_alias_crosswalk.md)
  — human-readable crosswalk for the canonical requirement register and
  the local labels that must resolve back to it.
- [`./interface_inventory.md`](./interface_inventory.md) — narrative
  companion to the stable-surface inventory and category outline for
  surfaces that have not yet earned a row.
- [`./compatibility_surface_inventory.md`](./compatibility_surface_inventory.md)
  and
  [`/artifacts/governance/compatibility_surfaces.yaml`](../../artifacts/governance/compatibility_surfaces.yaml)
  — wider machine-readable inventory for every compatibility-bearing
  public surface (settings and profile JSON, workspace manifests and
  Project Doctor outputs, extension manifests, CLI structured output,
  WIT host interfaces, optional service APIs, evidence / support
  bundles, task-event envelopes, plus specialized surfaces). Future
  public or beta surfaces register there before they may claim
  stability.
- [`./interface_lifecycle_policy.md`](./interface_lifecycle_policy.md)
  — shared lifecycle and deprecation metadata policy for stable ids,
  aliases, schema families, replacement chains, support windows, and
  notice-surface requirements once an interface leaves experimental
  state.
- [`./interface_freeze_matrix.md`](./interface_freeze_matrix.md) and
  [`./interface_freeze_guide.md`](./interface_freeze_guide.md) —
  canonical implementation-broadening freeze matrix and the short
  downstream citation guide that points task specs, ADRs, and packet
  updates at stable row ids instead of repeating contract prose.
- [`/docs/architecture/subsystem_contract_cards.md`](../architecture/subsystem_contract_cards.md)
  and
  [`/artifacts/architecture/subsystem_contract_cards/`](../../artifacts/architecture/subsystem_contract_cards/)
  — reviewer guide and machine-readable cards for launch-critical
  subsystem contracts. Cards summarize owned objects, allowed
  dependencies, budgets, failure modes, proof packets, owners, freeze
  status, and explicit gaps; ADRs, schemas, scorecards, and proof
  packets remain authoritative for detail.
- [`./frozen_surface_ci_policy.md`](./frozen_surface_ci_policy.md) and
  [`/artifacts/contracts/frozen_surface_manifest.yaml`](../../artifacts/contracts/frozen_surface_manifest.yaml)
  — manifest-driven CI policy and machine-readable row set for the M0
  frozen surfaces that already require same-train diff metadata and
  companion updates.
- [`./merge_control_policy.md`](./merge_control_policy.md),
  [`/artifacts/governance/protected_merge_classes.yaml`](../../artifacts/governance/protected_merge_classes.yaml),
  [`/artifacts/governance/public_surface_change_controls.yaml`](../../artifacts/governance/public_surface_change_controls.yaml),
  and
  [`/artifacts/governance/branch_protection_seed.yaml`](../../artifacts/governance/branch_protection_seed.yaml)
  — merge-control, public-surface change, branch-protection, bypass,
  and emergency reconstruction policy that projects CODEOWNERS,
  ownership, subsystem cards, compatibility rows, required review
  artifacts, and signing quorum into one review vocabulary.
- [`./contract_packet_template.md`](./contract_packet_template.md) —
  surface-contract packet template backed by
  `schemas/governance/contract_packet.schema.json`.
- [`./verification_packet_template.md`](./verification_packet_template.md)
  — canonical verification-packet template for shared claim-row,
  evidence-id, freshness, and signoff structure.
- [`./change_budget_workflow.md`](./change_budget_workflow.md)
  — protected change-budget matrix, review thresholds, exception-packet
  routing, and debt-dashboard field definitions for freeze-era
  decisions.
- [`./descoping_policy.md`](./descoping_policy.md)
  — canonical scope-control ladder, never-cut bars, milestone-at-risk
  defaults, and repeated-miss routing for milestone descopes.
- [`/artifacts/milestones/cut_classes.yaml`](../../artifacts/milestones/cut_classes.yaml)
  and [`/artifacts/milestones/kill_criteria.yaml`](../../artifacts/milestones/kill_criteria.yaml)
  — machine-readable ledgers for backlog/requirement cut classes and
  protected quality kill rows.
- [`/artifacts/governance/evidence_id_conventions.md`](../../artifacts/governance/evidence_id_conventions.md)
  — stable evidence-id grammar plus artifact-linking rules across
  design, benchmark, verification, support, and signoff packets.
- [`/schemas/governance/evidence_packet_header.schema.json`](../../schemas/governance/evidence_packet_header.schema.json)
  — shared header contract for packet identity, ownership, freshness,
  visibility, and artifact-link fields.
- [`/docs/ci/control_artifact_validation.md`](../ci/control_artifact_validation.md)
  — local and CI invocation guide for the shared contract-artifact
  validation lane.
- [`./artifact_hierarchy_and_packet_classes.md`](./artifact_hierarchy_and_packet_classes.md)
  and
  [`/artifacts/governance/packet_class_registry.yaml`](../../artifacts/governance/packet_class_registry.yaml)
  — narrative overview and machine-readable registry that freeze the
  canonical control-artifact stack (requirement register and alias
  crosswalk, ADR/RFC, design packet, contract/schema/interface packet,
  verification corpus and benchmark plan, compatibility or certified-
  archetype report, claim manifest and docs/migration pack,
  runbook/support/shiproom packet, public-proof publication bundle)
  with owner lane, canonical id namespace, source-of-truth location,
  visibility class, freshness class, minimum backlinks, release uses,
  and a per-milestone release-use matrix. Worked cross-class fixtures
  live under
  [`/fixtures/governance/artifact_class_examples/`](../../fixtures/governance/artifact_class_examples/).

**One home, one owner, one review path.** Every control asset — the
control-artifact graph, the interface inventory, the
compatibility-surface inventory, benchmark governance,
the benchmark change-control register, the protected-metrics file, the
benchmark-publication pack, the public-comparison rules, the
qualification cadence, the
compatibility qualification seed, the canonical requirement register,
the decision dependency register, the decision-rights signoff matrix,
the promotion decision row registry, the build-vs-buy register, the fork-review policy, the supply-chain
dependency and import registers, the critical-upstream health
scorecard, the maintainer-coverage policy, the signing-quorum matrix,
the merge-control policy, protected merge class catalog, branch-protection
seed, public-surface change-control matrix,
the experiment/Labs control registers, the release-notice seed, the
provenance-badge contract, the post-install notice and provenance
disclosure contract, the
release-artifact graph, release evidence, docs and help truth, route
and build-truth artifacts, count/scope/freshness microcopy grammar,
subsystem contract cards,
accessibility review packets,
surface-traceability artifacts, and frozen-surface manifests —
appears as exactly one row in the index file. If two documents describe
the same asset, the index names only the canonical one and the other is
either merged in or retired.

The index does not restate the content of the assets it points at. The
detailed governance workflow lives in
[`decision_workflow.md`](./decision_workflow.md); contract schemas live
in [`/schemas/`](../../schemas/); claim-manifest rules live in the
governance-packet template; and shared verification/evidence-link rules
live in the verification-packet template, the evidence-id conventions,
and the shared evidence-packet header. The index only names, routes,
and scopes.

## How each role uses the index

### Engineering

- Before introducing a new control asset — a register, manifest,
  review packet, or machine-readable surface definition — look for an
  existing row in the index. If the asset already has a canonical
  home, extend that home; do not mint a parallel document.
- Stable-facing or future stable-facing interfaces now route through
  `stable_surface_inventory.yaml` and the surface-contract packet
  template. Do not keep owner, versioning, support-window, or
  downgrade posture only in ADR prose.
- Workspace crate membership, off-cone prototype posture, and allowed
  internal dependency edges now route through `package_inventory.yaml`
  and `docs/repo/dependency_rules.md`. Do not add or promote a crate,
  or widen an internal edge, in `Cargo.toml` alone.
- Implementation-broadening contract reuse now routes through
  `interface_freeze_matrix.yaml` and the downstream citation guide. Do
  not restate renderer, buffer, command, settings, restore, token,
  attention, embedded-boundary, AI/provider, collaboration, review, or
  companion contract prose in per-task handoffs once a freeze row
  already exists.
- Subsystem handoff now routes through
  `docs/architecture/subsystem_contract_cards.md` and the card YAML
  directory. Use a card for owned objects, allowed dependencies,
  failure/degraded modes, proof packets, and explicit gaps before
  broadening implementation. Do not treat the card as a replacement for
  the ADR, schema, benchmark packet, scorecard, or frozen-surface row it
  references.
- Stable ids, alias ids, schema families, and record-class ids that
  leave experimental state now route through
  `interface_lifecycle_policy.md` and
  `deprecation_metadata.schema.json`. Do not repurpose a
  non-experimental id, hide a deprecated alias in prose only, or keep
  replacement-chain history only in release notes.
- Requirement ids now route through the canonical requirement register
  and alias crosswalk. Do not let scorecard calls, packet labels,
  fitness rows, or spec-local ids become de facto requirement ids in a
  new change.
- Freeze-era protected-path changes now route through
  `protected_change_budget.yaml`, `change_budget_workflow.md`, and the
  canonical exception-packet schema. Do not bury a repeated exception in
  PR narrative, milestone comments, or review-thread folklore.
- Verification packets and any other packet family that needs stable
  proof joins now route through `verification_packet_template.md`,
  `evidence_id_conventions.md`, and
  `evidence_packet_header.schema.json`. Do not mint packet-local
  ownership, freshness, or evidence-id fields when the shared header
  already covers them.
- Launch-bearing architecture, waiver, cutline, claim-publication,
  release-promotion, LTS-line, and workflow-bundle decisions now
  route through `signoff_matrix.yaml` and
  `promotion_decision_rows.yaml`. Do not write approval state as
  free-form scorecard or shiproom text when a decision row id, packet
  id field, closed status value, and evidence-bundle join already
  exist.
- Package, boundary, claim, source-anchor, and decision-reference drift
  now route through the shared contract-artifact validation lane. Frozen
  surface changes also route through the frozen-surface manifest and CI
  policy. Do not broaden a governed artifact family without keeping that
  lane passing or updating the expected checks in the same change.
- Experiments, feature flags, rollout rows, and Labs inventory updates
  now route through `feature_flag_policy`, `experiments_register`, and
  `labs_register`. Do not hide a new prototype mode or developer
  toggle in script help text alone.
- Protected-path dependency choices, deliberate upstream divergence,
  reviewer-depth policy, and emergency approval rules now route through
  `build_vs_buy_register`, `dependency_review_policy`,
  `critical_dependency_register`, `critical_upstream_health_scorecard`,
  `maintainer_coverage_policy`, `signing_quorum`, and
  `fork_review_policy`. Do not keep build-vs-buy posture, bus-factor
  risk, or quorum expectations only in an ADR or PR comment.
- Merge-control and protected-ref policy now routes through
  `merge_control_policy`, `protected_merge_class_catalog`,
  `public_surface_change_control_matrix`, and `branch_protection_seed`.
  Do not treat repository-host green checks as sufficient for
  protected, public-surface, release-bearing, or emergency changes when
  the required packet or concurrence is missing.
- Program blocker ids, blocker-aging SLAs, and forced correction
  responses now route through `program_dependency_ledger`,
  `blocker_aging_sla_table`, and `correction_trigger_table`. Do not
  keep a critical-path blocker only in a scorecard note, risk-summary
  paragraph, or meeting memory.
- Milestone descopes now route through `descoping_policy.md`,
  `cut_classes.yaml`, and `kill_criteria.yaml`. Do not invent new
  labels such as "probably optional" or "soft blocker" in a scorecard,
  backlog note, or shiproom packet when the governed ledgers already
  classify the row.
- When a pull request changes anything under a canonical-location path
  named in the index, update the corresponding row's `status` or
  `notes` if the change moves the artifact between `outline_only`,
  `seeded`, and `not_yet_seeded`.
- Review cadence `each_change` means the asset must be re-consulted
  on every pull request that could affect it. Review cadence
  `per_milestone` means on milestone boundaries. Review cadence
  `each_release` means during release-evidence assembly.

### Design

- Use the `design_system_seeds` and `accessibility_review_packets`
  rows as the only canonical locations for design-system snapshots,
  token sources, component references, accessibility audits, input-
  method review packets, and reduced-motion / contrast artifacts.
- When proposing a new design-system artifact, extend the canonical
  location under `artifacts/ux/`.
- When proposing a new accessibility or input-review artifact, extend
  the seeded family under `docs/accessibility/`,
  `artifacts/accessibility/`, or `fixtures/accessibility/` according to
  whether the change is a packet template, machine-readable matrix, or
  review corpus. Do not create parallel homes in docs, UX snapshots, or
  external tools.

### Quality engineering

- The `benchmark_governance`, `benchmark_change_control`,
  `external_proof_program_packet`,
  `design_partner_intake_checklist`,
  `benchmark_reference_hardware_manifest`,
  `benchmark_lab_image_manifest`, `benchmark_self_capture_parity`,
  `protected_metrics_file`, `benchmark_publication_pack`,
  `benchmark_publication_rehearsal_checklist`,
  `benchmark_public_comparison_rules`, `fixture_privacy_clearance_cases`,
  `fitness_function_catalog`,
  `benchmark_corpus_manifest`, `journey_trace_schema`,
  `journey_harness_tool`, `journey_trace_seed`,
  `protected_path_ledger`, `latency_budget_ledger`,
  `protected_path_evidence_linkage_seed`,
  `compatibility_qualification_seed`,
  `qualification_cadence`, `qualification_schedule`,
  `qualification_evidence_ownership_map`,
  `canonical_requirement_register`,
  `decision_dependency_register`, `program_dependency_ledger`,
  `correction_trigger_table`,
  `build_vs_buy_register`, `critical_dependency_register`,
  `critical_upstream_health_scorecard`, `maintainer_coverage_policy`,
  `fork_review_policy`, `merge_control_policy`,
  `protected_merge_class_catalog`, `public_surface_change_control_matrix`,
  `branch_protection_seed`, `feature_flag_policy`, and
  `experiments_register` rows are the anchor points for quality work.
  New fitness functions, benchmark corpora, and qualification gates
  land under the lanes named by those rows. Protected speed and
  safety claims MUST cite a row in the fitness-function catalog via
  the `fitness_function_snapshot` packet shape rather than invent a
  parallel metric.
- Public benchmark claims do not ship straight from the raw dashboard
  or a slide deck. They now route through
  [`docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  and
  [`docs/benchmarks/public_comparison_rules.md`](../benchmarks/public_comparison_rules.md)
  so exact command line, corpus revision, protected-metrics revision,
  comparability, docs applicability, known limits, and competitor
  settings are frozen in one packet.
- Design-partner and partner-derived fixture inputs now route through
  [`docs/program/design_partner_and_public_proof_packet.md`](../program/design_partner_and_public_proof_packet.md)
  and
  [`artifacts/program/design_partner_intake_checklist.yaml`](../../artifacts/program/design_partner_intake_checklist.yaml).
  Privacy clearance cases live in
  [`fixtures/bench/privacy_clearance_cases/`](../../fixtures/bench/privacy_clearance_cases/),
  and benchmark-publication dry runs route through
  [`artifacts/bench/publication_rehearsal_checklist.yaml`](../../artifacts/bench/publication_rehearsal_checklist.yaml).
  Do not admit raw partner bytes, raw traces, support packets, or
  license-restricted corpora into public proof without a recorded
  clearance decision, bundle ref, owner coverage, and rehearsal result.
- Benchmark-publication packs now share the same packet-header and
  evidence-id join rules as verification and signoff packets. Reuse the
  same `evidence_id` when a benchmark run also appears in a design,
  verification, or release packet.
- Evidence freshness, rerun-trigger handling, and stale-propagation now
  route through
  [`docs/governance/evidence_freshness_policy.md`](./evidence_freshness_policy.md),
  [`artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml),
  and
  [`artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml).
  Scorecards, shiproom packets, and claim rows now read one freshness
  matrix instead of treating `stale_after` as packet-local prose.
- Disputes about benchmark results route through the benchmark-council
  charter (see `benchmark_governance`); do not open ad-hoc comparison
  threads outside that forum. The `fitness_function_catalog` row is
  the source of truth for which protected fitness functions exist at
  a given catalog revision and which waiver authority covers each.

### Docs

- The `docs_public_truth` and `route_build_truth` rows are the
  canonical homes for every public-facing document, including the
  known-limits matrix, the support-window statement, and migration
  guides.
- When docs, scorecards, or packets need to name a governed obligation,
  resolve it through the canonical requirement register rather than by
  copying a local label out of a packet or a spec appendix.
- The clean-room rebuild lane now lives beside the reproducible-build
  baseline under `route_build_truth`; build truth is no longer only a
  narrative baseline plus an implicit CI script.
- The Labs inventory is not free-form copy. Contributor-visible
  prototype or preview inventory must resolve back to
  `labs_register.yaml`, and policy/process guidance must resolve back
  to `feature_flag_policy.md`.
- Any claim made in docs that a downstream consumer might rely on must
  cite an evidence owner via the claim-manifest packet family
  (governance-packet template). The index does not duplicate the
  manifest itself — it only names where the manifest lives.
- The public-truth claim-manifest contract now lives in
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  with the seeded packet at
  [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml),
  the parity matrix at
  [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml),
  and the narrative overview at
  [`/docs/governance/claim_manifest_contract.md`](./claim_manifest_contract.md).
  Docs/help, support-export, release-note, CLI/help, evaluation, and
  public-proof channels are all projections of those claim rows rather
  than separate truth sources.
- The `public_surface_truth_map` row is the canonical home for the
  owner-routing map that says which artifact owns lifecycle, policy,
  install, channel/provenance, compatibility, support-window, and
  known-limit truth. If the owner changes, the map and its drift rules
  update in the same change.
- Known-limit notes, migration guidance, and public-proof docs should
  preserve the same `evidence_id` and packet refs that the upstream
  verification packet used; docs are not allowed to invent a second
  proof namespace.

### Support

- The `support_export` and `accessibility_review_packets` rows are the
  canonical homes for supportability artifacts. Field runbooks, the
  crash-diagnostics corpus, export-safe packet schemas, and the
  support-packet family index all live there.
- The `record_class_registry` row is the canonical home for class-level
  retention, export, hold, delete, and offboarding posture that
  support bundles, issue handoff packets, and later managed support
  claims must quote instead of re-labelling privately.
- The records-governance indicator scoreboard contract now lives at
  [`/docs/governance/records_indicator_contract.md`](./records_indicator_contract.md)
  with the seeded register at
  [`/artifacts/governance/records_governance_indicator_scoreboard.yaml`](../../artifacts/governance/records_governance_indicator_scoreboard.yaml).
  It pins monthly boundary-manifest coverage, deletion/hold visibility,
  export/offboarding completeness, record-class freshness, dependency-marker
  debt, and public-proof link coverage to named record classes and claim rows
  so shiproom and release reviews do not treat records governance as anecdotal.
- Hidden compatibility aliases and retired schema families remain
  support-visible through the interface-lifecycle metadata rows even
  when general docs/help discovery hides them. Support/export surfaces
  should treat "not listed in help" as a visibility choice, not as an
  absence of governance.
- The support-packet index contract now lives at
  [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json).
  Shiproom, support-export, and field-handoff flows cite its canonical
  family ids for exact-build support, route/origin reconstruction,
  known-limit correlation, security triage, and rollback review instead
  of inventing case-local packet names.
- The crash-artifact retention seed and exact-build symbolication smoke
  path now live with the same support family under
  [`/artifacts/support/`](../../artifacts/support/) and
  [`/docs/support/`](../support/). Crash envelopes, dump/core manifests,
  symbolication reports, and support-bundle refs therefore reuse the
  same redaction and retention seed instead of creating case-local
  labels.
- The deployment continuity drill catalog seed lives under
  [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  with narrative guidance in
  [`/docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md).
  Release and boundary lanes cite that catalog rather than minting
  separate control-plane/data-plane outage vocabulary.
- Outage and maintenance notices use
  [`/docs/ux/control_data_plane_status_contract.md`](../ux/control_data_plane_status_contract.md),
  [`/schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json),
  and
  [`/fixtures/ops/outage_notices/`](../../fixtures/ops/outage_notices/)
  for the user-visible distinction between control-plane effects,
  data-plane effects, retained local-safe work, blocked writes, and
  boundary-change follow-up.
- Private partner and support cases follow the private routes in
  [`issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml);
  public supportability defects route to the OSS lane. Both cases
  preserve the owning forum.

### Release

- The `release_artifact_graph`, `release_evidence`,
  `shiproom_runbook`, `shiproom_dashboard_seed`,
  `release_posture_adr`, `release_artifact_family_map`,
  `packaging_installation_matrix`,
  `install_artifact_family_matrix`,
  `channel_identity_state_root_contract`,
  `release_promotion_gate_map`,
  `compatibility_qualification_seed`,
  `release_notice_seed`, `maintainer_coverage_policy`,
  `signing_quorum`, `merge_control_policy`,
  `protected_merge_class_catalog`, `branch_protection_seed`,
  `public_surface_change_control_matrix`, `frozen_surface_manifests`, `route_build_truth`,
  and `cleanroom_rebuild_lane` rows are the anchor points for release
  assembly.
  `review_cadence: each_release` means the release-engineer DRI MUST
  re-consult the artifact's rules before cutting a release.
- Shiproom review is no longer meeting-local folklore. The runbook now
  fixes the review order, go/no-go vocabulary, exception logging, and
  owner handoffs, while the dashboard seed rolls scorecards, waivers,
  stale proof, dependency reds, claim-manifest drift, and public-proof
  coverage into one canonical release-control surface.
- The blocker-aging SLA table and correction-trigger table now sit
  between the scorecard, risk register, and shiproom view. Release and
  milestone reviewers should use those ids instead of inferring whether
  a slip means descoping, rebaseline, or an exception packet.
- The desktop-platform conformance matrix and claimed-desktop-profile
  registry are now the canonical home for named macOS, Windows, and
  Linux claims, platform-owned primitives, deployment-path narrowings,
  validation methods, and explicitly unclaimed lanes. Release and
  support wording should cite those rows rather than saying
  "desktop-supported" generically.
- The desktop-management contract, deployment-pattern matrix, managed-
  controls matrix, and scriptable-install fixtures are now the canonical
  IT-facing home for silent install/uninstall, channel pinning, update
  controls, extension mirrors, proxy posture, policy bundle location,
  and documented uninstall behavior. Enterprise rollout guidance should
  cite those rows rather than relying on installer folklore.
- The deployment and unsupported-path disclosure matrix now sits between
  desktop conformance, desktop management, and install-profile cards.
  Package-manager, fleet-tool, helper/agent, desktop-environment,
  display-stack, and secret-store limitations should cite
  [`/docs/platform/deployment_and_unsupported_path_matrix.md`](../platform/deployment_and_unsupported_path_matrix.md)
  and its machine-readable companions rather than relying on hidden
  support notes.
- The window display behavior contract now lives in
  [`/docs/ux/window_display_contract.md`](../ux/window_display_contract.md)
  with boundary schema
  [`/schemas/platform/window_state.schema.json`](../../schemas/platform/window_state.schema.json).
  Platform adapters and support/export readers should use those
  fullscreen, snapped/tiled, native-control, display-topology,
  restore-history, focus-return, owned-prompt, secondary-window, and
  presentation fallback fields rather than minting OS-local state names.
- The window/display verification matrix seed lives under
  [`/artifacts/qa/window_display_matrix.yaml`](../../artifacts/qa/window_display_matrix.yaml)
  with narrative guidance in
  [`/docs/qa/multi_window_verification.md`](../qa/multi_window_verification.md).
  Claimed window/session continuity evidence should cite those scenario
  and drill ids rather than shipping a per-release topology checklist.
- The source-fidelity and undo-honesty verification seed now lives
  under
  [`/fixtures/io/source_fidelity_corpus_manifest.yaml`](../../fixtures/io/source_fidelity_corpus_manifest.yaml)
  with narrative guidance in
  [`/docs/verification/source_fidelity_and_undo_packet.md`](../verification/source_fidelity_and_undo_packet.md)
  and the rewrite vocabulary in
  [`/artifacts/io/save_rewrite_classes.yaml`](../../artifacts/io/save_rewrite_classes.yaml).
  Save-truth, whole-file-rewrite, and recovery-label evidence should
  cite those ids rather than inventing per-surface save banners.
- The focus-return and batch-scope verification seed now lives under
  [`/fixtures/ux/selection_and_virtualization_manifest.yaml`](../../fixtures/ux/selection_and_virtualization_manifest.yaml)
  with narrative guidance in
  [`/docs/verification/focus_and_batch_scope_packet.md`](../verification/focus_and_batch_scope_packet.md),
  reviewer-facing focus-return examples in
  [`/artifacts/ux/focus_return_examples/`](../../artifacts/ux/focus_return_examples/),
  and assistive-tech range-selection cases in
  [`/artifacts/accessibility/range_selection_at_cases/`](../../artifacts/accessibility/range_selection_at_cases/).
  Dense-collection scope truth, focus return, and range-selection
  accessibility evidence should cite those ids rather than inventing
  per-surface selection vocabulary.
- Release completeness is no longer implicit. The canonical graph and
  machine-readable rules now live in
  [`docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  and
  [`artifacts/release/artifact_graph_rules.yaml`](../../artifacts/release/artifact_graph_rules.yaml),
  which bind binaries, debug manifests, docs/help truth, benchmark
  proof packets, advisories/emergency actions/revocations, and
  promotion evidence into one non-overlapping release graph.
- Emergency actions and revocations now have one shared contract at
  [`docs/security/emergency_action_model.md`](../security/emergency_action_model.md)
  and
  [`schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json).
  Channel freezes, kill switches, trust-root rotations, mirror/manual-
  import freshness, signer continuity, local continuity, and
  post-incident reconciliation therefore travel as one governed object
  instead of surface-local warning fields.
- Threat claims and audit exports now share one vocabulary at
  [`docs/security/threat_model_and_audit_stream_contract.md`](../security/threat_model_and_audit_stream_contract.md),
  [`artifacts/security/threat_classes.yaml`](../../artifacts/security/threat_classes.yaml),
  [`schemas/security/audit_stream_record.schema.json`](../../schemas/security/audit_stream_record.schema.json),
  and
  [`schemas/security/evidence_window.schema.json`](../../schemas/security/evidence_window.schema.json).
  Advisories, incidents, approval tickets, support exports, admin
  exports, collaboration-control grants, remote join/leave rows, and
  managed-tenant/key reviews cite the same threat ids, audit-stream
  fields, evidence-window states, and omission-disposition rules.
- Release posture is no longer hidden inside packet prose. The
  governing ADR and machine-readable maps now live in
  [`docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md`](../adr/0017-release-posture-artifact-families-and-promotion-gates.md),
  [`artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml),
  and
  [`artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml).
  Those artifacts freeze channel posture, RC-as-stage, rollback-atom
  membership, same-change-set release bundles, waiver and late-proof
  policy, emergency mirror/manual-import transport, and the gates that
  still block stable-facing promotion after a successful build.
- Packaging and installation artifact families are now explicit
  release-bearing rows rather than installer folklore. The narrative
  matrix and machine-readable companions live in
  [`docs/release/packaging_installation_matrix.md`](../release/packaging_installation_matrix.md),
  [`artifacts/release/install_artifact_families.yaml`](../../artifacts/release/install_artifact_families.yaml),
  and
  [`artifacts/release/channel_identity_and_state_roots.yaml`](../../artifacts/release/channel_identity_and_state_roots.yaml).
  They bind MSI, MSIX, portable ZIP, PKG, DMG, app ZIP, DEB, RPM,
  AppImage, tarball, remote-helper tarball, and image-layer bundle rows
  to channel identity, state roots, update markers, recent items, file
  associations, protocol handlers, mirror posture, and rollback rules.
- Validation-ring widening is no longer implied by rollout folklore.
  The canonical policy, machine-readable matrix, and ring-history packet
  contract now live in
  [`docs/release/ring_progression_policy.md`](../release/ring_progression_policy.md),
  [`artifacts/release/ring_matrix.yaml`](../../artifacts/release/ring_matrix.yaml),
  and
  [`schemas/release/ring_history_packet.schema.json`](../../schemas/release/ring_history_packet.schema.json).
  Those artifacts separate validation widening from install-topology
  rollout rings and preserve the exact evidence snapshot behind every
  widening, hold, reset, or rollback-stop decision.
- The release-evidence family is now seeded with a narrative packet
  template in [`docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md),
  a filled seed example, a release-waiver schema under
  [`schemas/release/`](../../schemas/release/), and a shared evidence
  metadata catalog under
  [`artifacts/evidence/`](../../artifacts/evidence/). Concrete release
  packets still land under `artifacts/release/` when a real candidate is
  assembled.
- Release, benchmark, verification, support, and signoff packets now
  share one header contract and one evidence-id grammar, so release
  assembly can join upstream proof artifacts mechanically instead of by
  free-text packet prose.
- Shiproom and milestone-close freshness checks now read the same SLO
  matrix and rerun-trigger catalog the packet authors read. Release,
  support, docs, and benchmark lanes no longer keep packet shelf-life in
  separate review notes.
- Maintainer coverage and emergency approval are no longer implicit
  release lore. Release packets now cite
  [`docs/governance/maintainer_coverage_policy.md`](./maintainer_coverage_policy.md)
  for reviewer-depth and waiver posture and
  [`artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
  for the action ids that governed promotion, freeze, revocation, or
  break-glass handling.
- Decision-rights and signoff are likewise explicit. Release packets,
  shiproom packets, LTS-line decisions, and workflow-bundle
  certification or downgrade rows cite
  [`signoff_matrix.yaml`](../../artifacts/governance/signoff_matrix.yaml)
  and
  [`promotion_decision_rows.yaml`](../../artifacts/governance/promotion_decision_rows.yaml)
  so packet ids, evidence bundles, owner roles, concurrence roles, and
  degraded ownership states remain reconstructable.
- Merge-control and branch-protection evidence are likewise explicit.
  Protected and release-bearing merges now cite
  [`docs/governance/merge_control_policy.md`](./merge_control_policy.md),
  the protected merge-class catalog, public-surface change-control
  matrix, and branch-protection seed for required approvers, packets,
  status checks, bypass authority, and reconstruction obligations.
- The clean-room rebuild lane now has a canonical command
  ([`ci/cleanroom_rebuild.sh`](../../ci/cleanroom_rebuild.sh)), a public
  contract document
  ([`docs/build/cleanroom_rebuild_lane.md`](../build/cleanroom_rebuild_lane.md)),
  and a CI wrapper
  ([`/.github/workflows/cleanroom_rebuild.yml`](../../.github/workflows/cleanroom_rebuild.yml)).
  Its current limitations are intentionally named in emitted artifacts
  rather than being left as tribal CI knowledge.
- Frozen-surface manifests remain out of scope at this milestone.
  Shared stable-surface contract metadata now lives in the surface
  inventory and surface-contract packet template.

## Review cadence semantics

- **`each_change`** — the artifact is consulted on every pull request
  that could affect it. Decision-register rows, public-truth copy,
  build-truth artifacts, and the stable-surface inventory use this
  cadence.
- **`per_milestone`** — the artifact is reviewed at milestone
  boundaries, alongside the scorecard. Governance packets and
  benchmark-council outputs use this cadence.
- **`each_release`** — the artifact is consulted during release-
  evidence assembly. Release packets and frozen-surface manifests
  use this cadence.

## Visibility classes

- **`internal`** — the artifact is a repository-internal control
  record. It may still live in a public repository, but it is not a
  published surface and no downstream consumer is expected to
  integrate against it.
- **`public`** — the artifact is part of the project's public truth:
  it is linked from docs intended for downstream consumers and is
  governed by the `docs_public_truth` lane.

### Product

- The `boundary_manifest_strawman` row is the canonical home for the
  open-source core versus managed / commercial / service-plane
  boundary. The strawman narrative lives under `docs/product/` and
  conforms to `schemas/product/boundary_manifest.schema.json`. Every
  new product capability or managed service must either map to an
  existing row or land a new row in the same change; adding a
  managed service without a row is a governance error.
- The `open_paid_boundary_and_antilockin_matrix` row is the canonical
  home for the open-vs-paid boundary and anti-lock-in publication
  controls. It provides a machine-readable row register
  (`artifacts/governance/open_paid_boundary_rows.yaml`) plus worked
  publication-control examples under
  `fixtures/governance/publication_control_examples/` so packaging and
  docs/help surfaces cannot quietly make local-core workflows depend on
  hidden paid services or vendor-console-only control surfaces.
- Capability rows that introduce managed copies, support exports, AI
  evidence, usage exports, exit packets, or destruction receipts also
  resolve through the `record_class_registry` row so the managed claim
  does not hide record behavior.

## What this index is not

- It is **not** the detailed governance workflow. That lives in
  [`decision_workflow.md`](./decision_workflow.md).
- It is **not** the contract schema for any artifact. Schemas live
  under [`/schemas/`](../../schemas/).
- It is **not** the claim manifest. The claim-manifest family is
  defined by
  [`/artifacts/governance/governance_packet_template.yaml`](../../artifacts/governance/governance_packet_template.yaml),
  the row contract at
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json),
  and the narrative overview in
  [`/docs/governance/claim_manifest_contract.md`](./claim_manifest_contract.md).
  Concrete release-time instances extend the seeded row corpus rather
  than inventing channel-local claim shapes.
- It is **not** a substitute for the ownership matrix. The matrix
  defines who owns each lane; this index defines which artifacts sit
  inside those lanes. Lane IDs in the index always resolve back to
  `ownership_matrix.scorecard_lane_index`.

## Change discipline

- Adding a new control asset requires: a row in
  [`control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml),
  a canonical location, a named owner DRI, a lane from the ownership
  matrix, a review cadence, a visibility class, and a next-milestone
  target. All in the same change.
- Retiring a control asset requires: setting its `status` to reflect
  the removal and leaving the row in place with a note in `notes`.
  Rows are not deleted, so the audit trail of "this home existed and
  was retired" survives.
- When this document and the YAML index disagree, the YAML index wins
  for tooling and this document must be updated in the same change.
